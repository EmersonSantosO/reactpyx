use dashmap::DashMap;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyTuple};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Estado global gestionado por componentes
static GLOBAL_STATE: Lazy<DashMap<String, DashMap<String, Arc<Mutex<PyObject>>>>> =
    Lazy::new(DashMap::new);

thread_local! {
    // Caché para dependencias de efectos, organizado por ID de efecto
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

    /// Método para actualizar el estado del componente
    fn set(&self, py: Python<'_>, new_value: PyObject) -> PyResult<()> {
        let component_state = GLOBAL_STATE
            .entry(self.component_id.clone())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(self.key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(py.None())));

        let mut state = lock.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al bloquear estado: {:?}",
                e
            ))
        })?;
        *state = new_value;
        Ok(())
    }
}

#[pyclass]
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

    /// Método para despachar una acción y actualizar el estado
    fn dispatch(&self, py: Python<'_>, action: PyObject) -> PyResult<()> {
        let component_state = GLOBAL_STATE
            .entry(self.component_id.clone())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(self.key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(py.None())));

        let mut state = lock.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al bloquear estado: {:?}",
                e
            ))
        })?;

        let args = PyTuple::new(py, &[state.clone_ref(py), action]);
        let new_state = self.reducer.call1(py, args)?;
        *state = new_state;
        Ok(())
    }
}

/// Hook para gestionar el estado del componente
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
            .or_insert_with(|| Arc::new(Mutex::new(initial_value.clone_ref(py))));

        // Uso de try_lock + espera para evitar deadlocks permanentes
        let mut retry_count = 0;
        let max_retries = 5;

        let state = loop {
            match lock.try_lock() {
                Ok(guard) => break guard.clone_ref(py),
                Err(_) => {
                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Error al acceder al estado: máximo de reintentos alcanzado",
                        ));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        };

        let set_state: Py<SetState> =
            Py::new(py, SetState::new(component_id.to_string(), key.to_string()))?;
        Ok((state, set_state))
    })
}

/// Inicialización perezosa del estado
#[pyfunction(signature = (component_id, key, initial_value=None))]
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
                initial_value
                    .as_ref()
                    .map_or_else(|| py.None(), |v| v.clone_ref(py)),
            ))
        });

        let state = lock
            .lock()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Error al bloquear estado: {:?}",
                    e
                ))
            })?
            .clone_ref(py);
        Ok(state)
    })
}

/// Acceso al contexto compartido dentro de un componente
#[pyfunction]
pub fn use_context(component_id: &str, key: &str) -> PyResult<Option<PyObject>> {
    Python::with_gil(|py| {
        Ok(GLOBAL_STATE.get(component_id).and_then(|component_state| {
            component_state.get(key).map(|lock| {
                let state = lock.lock().expect("Error locking state").clone_ref(py);
                state
            })
        }))
    })
}

/// Hook para efectos con dependencias
#[pyfunction]
pub fn use_effect_with_deps(
    effect_id: &str,
    effect_function: Py<PyAny>,
    dependencies: Vec<PyObject>,
) -> PyResult<()> {
    Python::with_gil(|py| {
        let dependencies_hash = hash_dependencies(&dependencies, py)?;

        let should_run_effect = LAST_EFFECT_DEPS.with(|deps_map| {
            let mut deps_map = deps_map.borrow_mut();
            let last_deps_hash = deps_map.get(effect_id);

            let has_changed = match last_deps_hash {
                Some(prev_hash) => prev_hash != &dependencies_hash,
                None => true,
            };

            deps_map.insert(effect_id.to_string(), dependencies_hash);
            has_changed
        });

        if should_run_effect {
            effect_function.call1(py, (dependencies,))?;
        }

        Ok(())
    })
}

/// Hook simplificado para efectos sin dependencias (ejecuta siempre)
#[pyfunction]
pub fn use_effect(effect_function: Py<PyAny>) -> PyResult<()> {
    Python::with_gil(|py| {
        effect_function.call0(py)?;
        Ok(())
    })
}

/// Calcular hash para dependencias
fn hash_dependencies(dependencies: &[PyObject], py: Python<'_>) -> PyResult<u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();

    for dep in dependencies {
        let id = dep.call_method0(py, "__hash__").map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Error al calcular el hash de la dependencia",
            )
        })?;
        let id_value: isize = id.extract(py).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Error al extraer valor del hash")
        })?;
        id_value.hash(&mut hasher);
    }

    Ok(hasher.finish())
}

/// Hook para manejar el estado con un reducer
#[pyfunction]
pub fn use_reducer(
    component_id: &str,
    key: &str,
    reducer: Py<PyAny>,
    initial_state: PyObject,
) -> PyResult<(PyObject, Py<Dispatch>)> {
    Python::with_gil(|py| {
        let component_state = GLOBAL_STATE
            .entry(component_id.to_string())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(key.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(initial_state.clone_ref(py))));

        let state = lock
            .lock()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Error al bloquear estado: {:?}",
                    e
                ))
            })?
            .clone_ref(py);
        let dispatch: Py<Dispatch> = Py::new(
            py,
            Dispatch::new(component_id.to_string(), key.to_string(), reducer),
        )?;
        Ok((state, dispatch))
    })
}

/// Agregar hooks al módulo PyO3
#[pymodule]
fn hooks(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect, m)?)?; // Agregamos nuevo hook
    Ok(())
}
