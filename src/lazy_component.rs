use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[pyclass]
#[derive(Clone)]
pub struct LazyComponent {
    pub is_loading: Arc<AsyncMutex<bool>>, // Using `Mutex` for async compatibility
    pub result: Arc<AsyncMutex<Option<String>>>, // To control results
}

#[pymethods]
impl LazyComponent {
    #[new]
    pub fn new() -> Self {
        LazyComponent {
            is_loading: Arc::new(AsyncMutex::new(true)),
            result: Arc::new(AsyncMutex::new(None)),
        }
    }

    /// Load resource asynchronously and set `result` upon completion.
    pub async fn load_resource_async(&self, delay: u64) -> PyResult<()> {
        let is_loading_clone = Arc::clone(&self.is_loading);
        let result_clone = Arc::clone(&self.result);

        // Spawn a new task with Tokio for async operation
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            let mut is_loading = is_loading_clone.lock().await;
            let mut result = result_clone.lock().await;
            *is_loading = false;
            *result = Some("Resource loaded successfully".to_string());
        });

        Ok(())
    }

    pub async fn is_loading(&self) -> PyResult<bool> {
        let is_loading = self.is_loading.lock().await;
        Ok(*is_loading)
    }

    pub async fn get_result(&self) -> PyResult<Option<String>> {
        let result = self.result.lock().await;
        Ok(result.clone())
    }
}
