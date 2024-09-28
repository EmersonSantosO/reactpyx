use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
#[derive(Clone)]
pub struct EventHandler {
    handlers: DashMap<String, Vec<PyObject>>, // Usa `Mutex` para mejorar concurrencia de lectura
}

#[pymethods]
impl EventHandler {
    #[new]
    pub fn new() -> Self {
        EventHandler {
            handlers: Arc::new(Mutex::new(vec![])), // Inicializa con `Mutex`
        }
    }

    pub fn add_event_listener(&self, event: String, callback: PyObject) -> PyResult<()> {
        if event.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Evento no puede ser vacío",
            ));
        }
        let mut handlers = self
            .handlers
            .lock()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Lock poisoned"))?;
        handlers.push(callback);
        Ok(())
    }

    pub fn trigger_event(&self, event: String, py: Python) -> PyResult<()> {
        println!("Evento '{}' activado", event);
        let handlers = self
            .handlers
            .lock()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Lock poisoned"))?;
        for handler in handlers.iter() {
            handler.call0(py)?;
        }
        Ok(())
    }
}
