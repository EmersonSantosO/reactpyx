use dashmap::DashMap;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct EventHandler {
    handlers: Arc<DashMap<String, Vec<PyObject>>>,
}

#[pymethods]
impl EventHandler {
    #[new]
    pub fn new() -> Self {
        EventHandler {
            handlers: Arc::new(DashMap::new()),
        }
    }

    /// Añadir un nuevo listener para un evento específico
    pub fn add_event_listener(&self, event: &str, callback: PyObject) -> PyResult<()> {
        if event.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "El nombre del evento no puede estar vacío.",
            ));
        }

        let event = event.to_string();
        let mut handlers = self.handlers.entry(event.clone()).or_default();
        handlers.push(callback);
        Ok(())
    }

    /// Disparar un evento y ejecutar todos sus callbacks
    pub fn trigger_event(&self, event: &str, args: Vec<PyObject>, py: Python) -> PyResult<()> {
        if let Some(handlers) = self.handlers.get(event) {
            for handler in handlers.value().iter() {
                let args_tuple = PyTuple::new(py, &args);
                handler.call1(py, args_tuple)?;
            }
        }
        Ok(())
    }

    /// Remover listeners de un evento específico
    pub fn remove_event_listeners(&self, event: &str) -> PyResult<()> {
        self.handlers.remove(event);
        Ok(())
    }

    /// Remover un listener específico basado en un callback
    pub fn remove_listener_by_callback(&self, event: &str, callback: PyObject) -> PyResult<()> {
        if let Some(mut handlers) = self.handlers.get_mut(event) {
            handlers.retain(|handler| !handler.eq(&callback));
        }
        Ok(())
    }
}
