use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

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

    // Devolvemos un Py<JoinHandle>
    pub fn render_concurrently(
        &self,
        delay_secs: u64,
        py: Python<'_>,
    ) -> PyResult<Py<task::JoinHandle<()>>> {
        let is_complete_clone = Arc::clone(&self.is_complete);

        let join_handle = task::spawn(async move {
            if let Err(e) = tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await {
                error!("Error en la tarea de renderizado concurrente: {}", e);
            }
            let mut is_complete = is_complete_clone.lock().await;
            *is_complete = true;
        });

        Ok(Py::new(py, join_handle)?)
    }

    pub async fn is_render_complete(&self) -> PyResult<bool> {
        let is_complete = self.is_complete.lock().await;
        Ok(*is_complete)
    }

    pub async fn reset_render(&self) -> PyResult<()> {
        let mut is_complete = self.is_complete.lock().await;
        *is_complete = false;
        Ok(())
    }
}
