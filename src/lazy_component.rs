use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tokio::task::JoinHandle;

#[pyclass]
#[derive(Clone)]
pub struct LazyComponent {
    pub is_loading: Arc<AsyncMutex<bool>>,
    pub result: Arc<AsyncMutex<Option<String>>>,
    pub task_handle: Arc<AsyncMutex<Option<JoinHandle<()>>>>,
}

#[pymethods]
impl LazyComponent {
    #[new]
    pub fn new() -> Self {
        LazyComponent {
            is_loading: Arc::new(AsyncMutex::new(true)),
            result: Arc::new(AsyncMutex::new(None)),
            task_handle: Arc::new(AsyncMutex::new(None)),
        }
    }

    /// Carga recurso de forma asíncrona y establece `result` al completar.
    pub async fn load_resource_async(&self, delay: u64) -> PyResult<()> {
        let is_loading_clone = Arc::clone(&self.is_loading);
        let result_clone = Arc::clone(&self.result);

        // Cancelar tarea anterior si existe
        let mut task_handle = self.task_handle.lock().await;
        if let Some(handle) = task_handle.take() {
            handle.abort();
        }

        // Crear nueva tarea
        *task_handle = Some(tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;

            if let Ok(mut is_loading) = is_loading_clone.lock().await {
                *is_loading = false;
            }

            if let Ok(mut result) = result_clone.lock().await {
                *result = Some("Recurso cargado exitosamente".to_string());
            }
        }));

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

    pub async fn cancel(&self) -> PyResult<()> {
        let mut task_handle = self.task_handle.lock().await;
        if let Some(handle) = task_handle.take() {
            handle.abort();
            let mut is_loading = self.is_loading.lock().await;
            *is_loading = false;
        }
        Ok(())
    }
}

impl Drop for LazyComponent {
    fn drop(&mut self) {
        // Intentamos cancelar la tarea si el componente es destruido
        let task_handle = self.task_handle.clone();
        tokio::spawn(async move {
            if let Ok(mut handle) = task_handle.lock().await {
                if let Some(task) = handle.take() {
                    task.abort();
                }
            }
        });
    }
}
