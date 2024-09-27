use dashmap::DashMap;
use once_cell::sync::Lazy;

use pyo3::types::PyTuple;
use pyo3::{prelude::*, wrap_pyfunction};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

// Estado global organizado por componente y estado
static GLOBAL_STATE: Lazy<DashMap<String, DashMap<String, Arc<Mutex<PyObject>>>>> =
    Lazy::new(DashMap::new);

thread_local! {
    // Cache para dependencias de efectos, separadas por identificador de efecto
    static LAST_EFFECT_DEPS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

#[pyclass]
#[derive(Clone)]
pub struct SetState {
    pub key: String,
    pub component_id: String,
}

#[pymethods]
impl SetState {
    #[new]
    fn new(component_id: String, key: String) -> Self {
        SetState { component_id, key }
    }

    fn set(&self, py: Python, new_value: PyObject) -> PyResult<()> {
        // Acceder al estado del componente específico
        let component_state = GLOBAL_STATE
            .entry(self.component_id.clone())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(self.key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(py.None())));

        let mut state = lock.lock().unwrap(); // Acceso seguro
        *state = new_value.into_py(py);
        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Dispatch {
    pub key: String,
    pub component_id: String,
    reducer: Py<PyAny>,
}

#[pymethods]
impl Dispatch {
    #[new]
    fn new(component_id: String, key: String, reducer: Py<PyAny>) -> Self {
        Dispatch {
            component_id,
            key,
            reducer,
        }
    }

    fn dispatch(&self, py: Python, action: PyObject) -> PyResult<()> {
        // Acceder al estado del componente específico
        let component_state = GLOBAL_STATE
            .entry(self.component_id.clone())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(self.key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(py.None())));

        let mut state = lock.lock().unwrap();

        let reducer = self.reducer.bind(py);

        let args = PyTuple::new(py, &[state.clone(), action]);
        let new_state = reducer.call1(args)?;

        *state = new_state.into_py(py);
        Ok(())
    }
}

// Hook para `useState`
#[pyfunction]
pub fn use_state(
    component_id: &str,
    key: &str,
    initial_value: PyObject,
) -> PyResult<(PyObject, Py<SetState>)> {
    Python::with_gil(|py| {
        let component_state = GLOBAL_STATE
            .entry(component_id.to_string())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(initial_value.clone())));

        let state = lock.lock().unwrap();
        let set_state: Py<SetState> =
            Py::new(py, SetState::new(component_id.to_string(), key.to_string()))?;
        Ok((state.clone(), set_state))
    })
}

// Hook para `useLazyState`
#[pyfunction]
pub fn use_lazy_state(
    component_id: &str,
    key: &str,
    initial_value: Option<PyObject>,
) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let component_state = GLOBAL_STATE
            .entry(component_id.to_string())
            .or_insert_with(DashMap::new);

        let lock = component_state.entry(key.to_string()).or_insert_with(|| {
            Arc::new(Mutex::new(
                initial_value.clone().unwrap_or_else(|| py.None()),
            ))
        });

        let state = lock.lock().unwrap();
        Ok(state.clone())
    })
}

// Hook para `useContext`
#[pyfunction]
pub fn use_context(component_id: &str, key: &str) -> PyResult<Option<PyObject>> {
    Python::with_gil(|_| {
        Ok(GLOBAL_STATE.get(component_id).and_then(|component_state| {
            component_state.get(key).map(|lock| {
                // Bloquear el Mutex en un contexto síncrono
                let state = lock.lock().unwrap();
                state.clone()
            })
        }))
    })
}

// Hook para `useEffect` con dependencias
#[pyfunction]
pub fn use_effect_with_deps(
    effect_id: &str,
    py: Python,
    effect_function: &PyAny,
    dependencies: Vec<PyObject>,
) -> PyResult<()> {
    let dependencies_hash = hash_dependencies(&dependencies);

    let should_run_effect = LAST_EFFECT_DEPS.with(|deps_map| {
        let mut deps_map = deps_map.borrow_mut();
        let last_deps_hash = deps_map.get(effect_id);

        // Comparar el hash de las dependencias previas con las nuevas
        let has_changed = match last_deps_hash {
            Some(prev_hash) => prev_hash != &dependencies_hash,
            None => true,
        };

        // Actualizar las dependencias guardadas
        deps_map.insert(effect_id.to_string(), dependencies_hash);
        has_changed
    });

    if should_run_effect {
        let effect = effect_function.call1(PyTuple::new(py, &dependencies))?;

        // Ejecutar `cleanup` si existe
        if let Ok(cleanup) = effect.getattr("cleanup") {
            cleanup.call0()?;
        }
    }

    Ok(())
}

// Hook para `useReducer`
#[pyfunction]
pub fn use_reducer(
    component_id: &str,
    key: &str,
    reducer: &PyAny,
    initial_state: PyObject,
) -> PyResult<(PyObject, Py<Dispatch>)> {
    Python::with_gil(|py| {
        let component_state = GLOBAL_STATE
            .entry(component_id.to_string())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(initial_state.clone())));

        let state = lock.lock().unwrap();
        let reducer_py = reducer.to_object(py).into();
        let dispatch: Py<Dispatch> = Py::new(
            py,
            Dispatch::new(component_id.to_string(), key.to_string(), reducer_py),
        )?;
        Ok((state.clone(), dispatch))
    })
}

// Calcular el hash de dependencias de efectos
fn hash_dependencies(dependencies: &[PyObject]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();

    Python::with_gil(|py| {
        for dep in dependencies {
            // Llamar a `__hash__()` del objeto Python
            let id = dep.call_method0(py, "__hash__").unwrap();
            let id_value: isize = id.extract(py).unwrap();
            id_value.hash(&mut hasher);
        }
    });

    hasher.finish()
}

// Módulo de PyO3 para exponer hooks
#[pymodule]
fn hooks(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    Ok(())
}

fn add_hooks_to_module(m: &PyModule) -> PyResult<()> {
    use crate::hooks::{
        use_context, use_effect_with_deps, use_lazy_state, use_reducer, use_state, Dispatch,
        SetState,
    };

    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    Ok(())
}

static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Error al crear el runtime de Tokio"));
