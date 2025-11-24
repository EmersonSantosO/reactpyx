use dashmap::DashMap;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::sync::Arc;

#[pyclass]
#[derive(Clone)]
pub struct EventHandler {
    handlers: Arc<DashMap<String, Vec<Py<PyAny>>>>,
}

#[pymethods]
impl EventHandler {
    #[new]
    pub fn new() -> Self {
        EventHandler {
            handlers: Arc::new(DashMap::new()),
        }
    }

    /// Add a new listener for a specific event
    pub fn add_event_listener(&self, event: &str, callback: Py<PyAny>) -> PyResult<()> {
        if event.trim().is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Event name cannot be empty.",
            ));
        }

        let event = event.to_string();
        let mut handlers = self.handlers.entry(event.clone()).or_default();
        handlers.push(callback);
        Ok(())
    }

    /// Trigger an event and execute all its callbacks
    pub fn trigger_event(&self, event: &str, args: Vec<Py<PyAny>>, py: Python) -> PyResult<()> {
        if let Some(handlers) = self.handlers.get(event) {
            for handler in handlers.value().iter() {
                let args_tuple = PyTuple::new(py, &args)?;
                handler.call1(py, args_tuple)?;
            }
        }
        Ok(())
    }

    /// Remove listeners for a specific event
    pub fn remove_event_listeners(&self, event: &str) -> PyResult<()> {
        self.handlers.remove(event);
        Ok(())
    }

    /// Remove a specific listener based on a callback
    pub fn remove_listener_by_callback(&self, event: &str, callback: Py<PyAny>) -> PyResult<()> {
        if let Some(mut handlers) = self.handlers.get_mut(event) {
            handlers.retain(|handler| handler.as_ptr() != callback.as_ptr());
        }
        Ok(())
    }
}
