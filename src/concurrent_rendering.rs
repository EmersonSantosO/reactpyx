use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use std::sync::Arc;
use tokio::sync::Mutex;
#[pyclass]
pub struct ConcurrentRenderer {
    pub is_complete: Arc<Mutex<bool>>,
}

#[pymethods]
impl ConcurrentRenderer {
    #[new]
    pub fn new() -> Self {
        ConcurrentRenderer {
            is_complete: Arc::new(Mutex::new(false)),
        }
    }

    pub fn is_render_complete<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let is_complete = Arc::clone(&self.is_complete);
        future_into_py(py, async move {
            let is_complete = is_complete.lock().await;
            Ok(*is_complete)
        })
    }
}
