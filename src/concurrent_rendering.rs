use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

#[pyclass]
pub struct ConcurrentRenderer {
    pub is_complete: Arc<RwLock<bool>>,
}

#[pymethods]
impl ConcurrentRenderer {
    #[new]
    pub fn new() -> Self {
        ConcurrentRenderer {
            is_complete: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn is_render_complete(&self) -> PyResult<bool> {
        let is_complete = self.is_complete.read().await;
        Ok(*is_complete)
    }

    pub async fn set_render_complete(&self, complete: bool) -> PyResult<()> {
        let mut is_complete = self.is_complete.write().await;
        *is_complete = complete;
        Ok(())
    }
}
