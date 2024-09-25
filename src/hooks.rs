// core_reactpyx/src/hooks.rs

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static GLOBAL_STATE: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[pyfunction]
pub fn use_state(key: &str, initial_value: String) -> PyResult<String> {
    let mut state_map = GLOBAL_STATE.lock().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Error al adquirir el lock: {}",
            e
        ))
    })?;
    let value = state_map
        .entry(key.to_string())
        .or_insert(initial_value.clone());
    Ok(value.clone())
}

#[pyfunction]
pub fn set_state(key: &str, new_value: String) -> PyResult<()> {
    let mut state_map = GLOBAL_STATE.lock().map_err(|e| {
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
    let mut state_map = GLOBAL_STATE.lock().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Error al adquirir el lock: {}",
            e
        ))
    })?;
    let state = state_map
        .entry(key.to_string())
        .or_insert(initial_state.clone());

    let action = PyDict::new(py);
    let new_state: String = reducer.call1(py, (state.clone(), action))?.extract(py)?;

    *state = new_state.clone();
    Ok(new_state)
}

// src/hooks.rs

#[pyfunction]
pub fn use_lazy_state(key: &str, initial_value: Option<String>) -> PyResult<String> {
    let mut state_map = GLOBAL_STATE.lock().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Error al adquirir el lock: {}",
            e
        ))
    })?;

    let value = state_map.entry(key.to_string()).or_insert_with(|| {
        println!("Inicializando estado perezoso para: {}", key);
        initial_value.clone().unwrap_or_default()
    });

    Ok(value.clone())
}
