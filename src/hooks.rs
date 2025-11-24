use dashmap::DashMap;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Global state managed by components
// SessionID -> ComponentID -> Key -> Value
static GLOBAL_STATE: Lazy<
    DashMap<String, DashMap<String, DashMap<String, Arc<Mutex<Py<PyAny>>>>>>,
> = Lazy::new(DashMap::new);

fn get_session_id(py: Python<'_>) -> PyResult<String> {
    let context_module = py.import("reactpyx.context")?;
    let session_id: Option<String> = context_module
        .call_method0("get_current_session_id")?
        .extract()?;
    Ok(session_id.unwrap_or_else(|| "global".to_string()))
}

thread_local! {
    // Cache for effect dependencies, organized by effect ID
    static LAST_EFFECT_DEPS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

#[pyclass]
#[derive(Clone)]
pub struct SetState {
    pub key: String,
    pub component_id: String,
    pub session_id: String,
}

#[pymethods]
impl SetState {
    #[new]
    fn new(component_id: String, key: String, session_id: String) -> Self {
        SetState {
            component_id,
            key,
            session_id,
        }
    }

    /// Method to update component state
    fn set(&self, py: Python<'_>, new_value: Py<PyAny>) -> PyResult<()> {
        let session_state = GLOBAL_STATE
            .entry(self.session_id.clone())
            .or_insert_with(DashMap::new);

        let component_state = session_state
            .entry(self.component_id.clone())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(self.key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(py.None())));

        let mut state = lock.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error locking state: {:?}",
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
    pub session_id: String,
    reducer: Py<PyAny>,
}

#[pymethods]
impl Dispatch {
    #[new]
    fn new(component_id: String, key: String, session_id: String, reducer: Py<PyAny>) -> Self {
        Dispatch {
            component_id,
            key,
            session_id,
            reducer,
        }
    }

    /// Method to dispatch an action and update state
    fn dispatch(&self, py: Python<'_>, action: Py<PyAny>) -> PyResult<()> {
        let session_state = GLOBAL_STATE
            .entry(self.session_id.clone())
            .or_insert_with(DashMap::new);

        let component_state = session_state
            .entry(self.component_id.clone())
            .or_insert_with(DashMap::new);

        let lock = component_state
            .entry(self.key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(py.None())));

        let mut state = lock.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error locking state: {:?}",
                e
            ))
        })?;

        let args = (state.clone_ref(py), action);
        let new_state = self.reducer.call1(py, args)?;
        *state = new_state;
        Ok(())
    }
}

/// Hook to manage component state
#[pyfunction]
pub fn use_state(
    py: Python<'_>,
    component_id: &str,
    key: &str,
    initial_value: Py<PyAny>,
) -> PyResult<(Py<PyAny>, Py<SetState>)> {
    let session_id = get_session_id(py)?;

    let session_state = GLOBAL_STATE
        .entry(session_id.clone())
        .or_insert_with(DashMap::new);

    let component_state = session_state
        .entry(component_id.to_string())
        .or_insert_with(DashMap::new);

    let lock = component_state
        .entry(key.to_string())
        .or_insert_with(|| Arc::new(Mutex::new(initial_value.clone_ref(py))));

    // Using try_lock + wait to avoid permanent deadlocks
    let mut retry_count = 0;
    let max_retries = 5;

    let state = loop {
        match lock.try_lock() {
            Ok(guard) => break guard.clone_ref(py),
            Err(_) => {
                retry_count += 1;
                if retry_count >= max_retries {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                        "Error accessing state: maximum retries reached",
                    ));
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    };

    let set_state: Py<SetState> = Py::new(
        py,
        SetState::new(component_id.to_string(), key.to_string(), session_id),
    )?;
    Ok((state, set_state))
}

/// Lazy state initialization
#[pyfunction(signature = (component_id, key, initial_value=None))]
pub fn use_lazy_state(
    py: Python<'_>,
    component_id: &str,
    key: &str,
    initial_value: Option<Py<PyAny>>,
) -> PyResult<Py<PyAny>> {
    let session_id = get_session_id(py)?;

    let session_state = GLOBAL_STATE.entry(session_id).or_insert_with(DashMap::new);

    let component_state = session_state
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
                "Error locking state: {:?}",
                e
            ))
        })?
        .clone_ref(py);
    Ok(state)
}

/// Access to shared context inside a component
#[pyfunction]
pub fn use_context(py: Python<'_>, component_id: &str, key: &str) -> PyResult<Option<Py<PyAny>>> {
    let session_id = get_session_id(py)?;

    Ok(GLOBAL_STATE.get(&session_id).and_then(|session_state| {
        session_state.get(component_id).and_then(|component_state| {
            component_state.get(key).map(|lock| {
                let state = lock.lock().expect("Error locking state").clone_ref(py);
                state
            })
        })
    }))
}

/// Hook for effects with dependencies
#[pyfunction]
pub fn use_effect_with_deps(
    py: Python<'_>,
    effect_id: &str,
    effect_function: Py<PyAny>,
    dependencies: Vec<Py<PyAny>>,
) -> PyResult<()> {
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
}

/// Simplified hook for effects without dependencies (runs every time)
#[pyfunction]
pub fn use_effect(py: Python<'_>, effect_function: Py<PyAny>) -> PyResult<()> {
    effect_function.call0(py)?;
    Ok(())
}

/// Calculate hash for dependencies
fn hash_dependencies(dependencies: &[Py<PyAny>], py: Python<'_>) -> PyResult<u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();

    for dep in dependencies {
        let id = dep.call_method0(py, "__hash__").map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Error calculating dependency hash")
        })?;
        let id_value: isize = id.extract(py).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Error extracting hash value")
        })?;
        id_value.hash(&mut hasher);
    }

    Ok(hasher.finish())
}

/// Hook to manage state with a reducer
#[pyfunction]
pub fn use_reducer(
    py: Python<'_>,
    component_id: &str,
    key: &str,
    reducer: Py<PyAny>,
    initial_state: Py<PyAny>,
) -> PyResult<(Py<PyAny>, Py<Dispatch>)> {
    let session_id = get_session_id(py)?;

    let session_state = GLOBAL_STATE
        .entry(session_id.clone())
        .or_insert_with(DashMap::new);

    let component_state = session_state
        .entry(component_id.to_string())
        .or_insert_with(DashMap::new);

    let lock = component_state
        .entry(key.to_string())
        .or_insert_with(|| Arc::new(Mutex::new(initial_state.clone_ref(py))));

    let state = lock
        .lock()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error locking state: {:?}",
                e
            ))
        })?
        .clone_ref(py);
    let dispatch: Py<Dispatch> = Py::new(
        py,
        Dispatch::new(
            component_id.to_string(),
            key.to_string(),
            session_id,
            reducer,
        ),
    )?;
    Ok((state, dispatch))
}

/// Check if running on Python 3.13 or higher
#[pyfunction]
pub fn is_python_313_plus(py: Python) -> PyResult<bool> {
    let sys = py.import("sys")?;
    let version_info = sys.getattr("version_info")?;
    let major: u8 = version_info.getattr("major")?.extract()?;
    let minor: u8 = version_info.getattr("minor")?.extract()?;

    Ok(major >= 3 && minor >= 13)
}

/// Adds Python 3.13 compatibility for ReactPyx components
#[pyfunction]
pub fn add_py313_features(component: &Bound<'_, PyAny>) -> PyResult<()> {
    let py = component.py();
    // Check if running on Python 3.13
    if is_python_313_plus(py)? {
        // Add Python 3.13 specific attributes
        component.setattr("__py313_enhanced", true)?;

        // Add extra information that Python 3.13 supports
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        component.setattr("__python_version", version)?;
    }
    Ok(())
}

/// Add hooks to PyO3 module
#[pymodule]
fn hooks(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect, m)?)?;
    m.add_function(wrap_pyfunction!(is_python_313_plus, m)?)?;
    m.add_function(wrap_pyfunction!(add_py313_features, m)?)?;
    Ok(())
}
