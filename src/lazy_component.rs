use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use std::sync::{Arc, RwLock};

#[pyclass]
#[derive(Clone)]
pub struct LazyComponent {
    pub is_loading: Arc<RwLock<bool>>,
    pub result: Arc<RwLock<Option<String>>>,
}

#[pymethods]
impl LazyComponent {
    #[new]
    pub fn new() -> Self {
        LazyComponent {
            is_loading: Arc::new(RwLock::new(true)),
            result: Arc::new(RwLock::new(None)),
        }
    }

    pub fn load_resource_async<'py>(&self, delay: u64, py: Python<'py>) -> PyResult<&'py PyAny> {
        let is_loading_clone = Arc::clone(&self.is_loading);
        let result_clone = Arc::clone(&self.result);

        future_into_py(py, async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            let mut is_loading = is_loading_clone.write().unwrap();
            let mut result = result_clone.write().unwrap();
            *is_loading = false;
            *result = Some("Recurso cargado exitosamente".to_string());
            Ok(())
        })
    }
}
