use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
#[derive(Clone)]
pub struct EventHandler {
    handlers: Arc<Mutex<Vec<PyObject>>>,
}

#[pymethods]
impl EventHandler {
    #[new]
    pub fn new() -> Self {
        EventHandler {
            handlers: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn add_event_listener(&self, event: String, callback: PyObject) -> PyResult<()> {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.push(callback);
        println!("Evento '{}' registrado", event);
        Ok(())
    }

    pub fn trigger_event(&self, event: String, py: Python) -> PyResult<()> {
        println!("Evento '{}' activado", event);
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.call0(py)?;
        }
        Ok(())
    }
}
