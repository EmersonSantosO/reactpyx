use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use std::sync::Arc;
use tokio::sync::Mutex;
#[pyclass]
pub struct AsyncTaskManager {
    is_complete: Arc<Mutex<bool>>, // Cambiado a `tokio::sync::Mutex`
}

#[pymethods]
impl AsyncTaskManager {
    #[new]
    pub fn new() -> Self {
        Self {
            is_complete: Arc::new(Mutex::new(false)),
        }
    }

    pub fn run_async_task<'a>(
        &self,
        py: Python<'a>,
        delay_secs: u64,
    ) -> PyResult<Bound<'a, PyAny>> {
        let is_complete_clone = Arc::clone(&self.is_complete);
        future_into_py(py, async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
            let mut is_complete = is_complete_clone.lock().await; // Usa `await` para `tokio::sync::Mutex`
            *is_complete = true;
            Ok(())
        })
    }

    pub fn is_task_complete<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let is_complete_clone = Arc::clone(&self.is_complete);
        future_into_py(py, async move {
            let is_complete = is_complete_clone.lock().await; // Usa `await` para `tokio::sync::Mutex`
            Ok(*is_complete)
        })
    }
}
