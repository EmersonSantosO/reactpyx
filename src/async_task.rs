// core_reactpyx/src/async_task.rs

use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task;

#[pyclass]
pub struct AsyncTaskManager {
    pub is_complete: Arc<Mutex<bool>>,
}

#[pymethods]
impl AsyncTaskManager {
    #[new]
    pub fn new() -> Self {
        AsyncTaskManager {
            is_complete: Arc::new(Mutex::new(false)),
        }
    }

    pub fn run_async_task(&self, delay_secs: u64) -> PyResult<()> {
        let is_complete_clone = Arc::clone(&self.is_complete);
        task::spawn(async move {
            tokio::time::sleep(Duration::from_secs(delay_secs)).await;
            let mut is_complete = is_complete_clone.lock().unwrap();
            *is_complete = true;
        });
        Ok(())
    }

    pub fn is_task_complete(&self) -> PyResult<bool> {
        let is_complete = self.is_complete.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Error al adquirir el lock: {}",
                e
            ))
        })?;
        Ok(*is_complete)
    }
}
