// core_reactpyx/src/hooks.rs

use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static GLOBAL_STATE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

#[pyfunction]
pub fn use_state(key: &str, initial_value: String) -> PyResult<String> {
    let state_map = GLOBAL_STATE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;

    Ok(state_map.get(key).cloned().unwrap_or_else(|| {
        state_map.insert(key.to_string(), initial_value.clone());
        initial_value
    }))
}

#[pyfunction]
pub fn set_state(key: &str, new_value: String) -> PyResult<()> {
    let mut state_map = GLOBAL_STATE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;

    state_map.insert(key.to_string(), new_value);
    Ok(())
}

#[pyfunction]
pub fn use_reducer(
    key: &str,
    reducer: PyObject,
    initial_state: String,
    py: Python,
) -> PyResult<String> {
    let mut state_map = GLOBAL_STATE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;
    let state = state_map
        .entry(key.to_string())
        .or_insert_with(|| initial_state.clone());

    let action = PyDict::new(py);
    let new_state: String = reducer.call1(py, (state.clone(), action))?.extract(py)?;

    *state = new_state.clone();
    Ok(new_state)
}

#[pyfunction]
pub fn use_lazy_state(key: &str, initial_value: Option<String>) -> PyResult<String> {
    let state_map = GLOBAL_STATE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;

    Ok(state_map.get(key).cloned().unwrap_or_else(|| {
        let value = initial_value.unwrap_or_default();
        state_map.insert(key.to_string(), value.clone());
        value
    }))
}
