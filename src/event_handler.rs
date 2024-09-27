use pyo3::prelude::*;
use std::sync::{Arc, RwLock};

#[pyclass]
#[derive(Clone)]
pub struct EventHandler {
    handlers: Arc<RwLock<Vec<PyObject>>>, // Usa `RwLock` para mejorar concurrencia de lectura
}

#[pymethods]
impl EventHandler {
    #[new]
    pub fn new() -> Self {
        EventHandler {
            handlers: Arc::new(RwLock::new(vec![])), // Inicializa con `RwLock`
        }
    }

    pub fn add_event_listener(&self, event: String, callback: PyObject) -> PyResult<()> {
        let mut handlers = self
            .handlers
            .write()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Lock poisoned"))?;
        handlers.push(callback);
        println!("Evento '{}' registrado", event);
        Ok(())
    }

    pub fn trigger_event(&self, event: String, py: Python) -> PyResult<()> {
        println!("Evento '{}' activado", event);
        let handlers = self
            .handlers
            .read()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Lock poisoned"))?;
        for handler in handlers.iter() {
            handler.call0(py)?;
        }
        Ok(())
    }
}
