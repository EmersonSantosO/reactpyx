// core_reactpyx/src/lazy_component.rs

use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[pyclass]
pub struct LazyComponent {
    pub is_loading: Arc<Mutex<bool>>,
    pub result: Arc<Mutex<Option<String>>>,
}

#[pymethods]
impl LazyComponent {
    #[new]
    pub fn new() -> Self {
        LazyComponent {
            is_loading: Arc::new(Mutex::new(true)),
            result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn load_resource(&self, delay: u64) {
        let is_loading_clone = Arc::clone(&self.is_loading);
        let result_clone = Arc::clone(&self.result);

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(delay));
            let mut is_loading = is_loading_clone.lock().unwrap();
            let mut result = result_clone.lock().unwrap();
            *is_loading = false;
            *result = Some("Recurso cargado exitosamente".to_string());
        });
    }

    pub fn is_loading(&self) -> PyResult<bool> {
        let is_loading = self.is_loading.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;
        Ok(*is_loading)
    }

    pub fn get_result(&self) -> PyResult<Option<String>> {
        let result = self.result.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;
        Ok(result.clone())
    }
}
