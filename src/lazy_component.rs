use pyo3::{prelude::*, wrap_pyfunction};
use pyo3_asyncio_0_21::tokio::future_into_py;
use std::sync::Arc;
use tokio::sync::Mutex;
#[pyclass]
#[derive(Clone)]
pub struct LazyComponent {
    pub is_loading: Arc<Mutex<bool>>, // Cambiado a `tokio::sync::Mutex`
    pub result: Arc<Mutex<Option<String>>>, // Cambiado a `tokio::sync::Mutex`
}

#[pymethods]
impl LazyComponent {
    #[new]
    pub fn new() -> Self {
        LazyComponent {
            is_loading: Arc::new(Mutex::new(true)),
            result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn load_resource_async<'py>(
        &self,
        delay: u64,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let is_loading_clone = Arc::clone(&self.is_loading);
        let result_clone = Arc::clone(&self.result);

        future_into_py(py, async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            let mut is_loading = is_loading_clone.lock().await; // Usa `await` para `tokio::sync::Mutex`
            let mut result = result_clone.lock().await; // Usa `await` para `tokio::sync::Mutex`
            *is_loading = false;
            *result = Some("Recurso cargado exitosamente".to_string());
            Ok(())
        })
    }
}
