// core_reactpyx/src/event_handler.rs

use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
#[derive(Clone)]
pub struct EventHandler {
    pub handlers: Arc<Mutex<Vec<Box<dyn Fn() + Send>>>>,
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
        handlers.push(Box::new(move || {
            Python::with_gil(|py| {
                let callback = callback.clone_ref(py);
                callback.call0(py).unwrap();
            });
        }));
        println!("Evento '{}' registrado", event);
        Ok(())
    }

    pub fn trigger_event(&self, event: String) {
        println!("Evento '{}' activado", event);
        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler();
        }
    }
}
