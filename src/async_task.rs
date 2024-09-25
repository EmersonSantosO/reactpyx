// core_reactpyx/src/async_task.rs

use pyo3::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
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
            tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
            let mut is_complete = is_complete_clone.lock().await;
            *is_complete = true;
        });
        Ok(())
    }

    pub async fn is_task_complete(&self) -> PyResult<bool> {
        let is_complete = self.is_complete.lock().await;
        Ok(*is_complete)
    }
}
// core_reactpyx/src/async_task.rs

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_async_task_manager() {
        let task_manager = AsyncTaskManager::new();
        let delay = 2;
        task_manager.run_async_task(delay).unwrap();

        // Esperar un poco más que el retraso para que la tarea se complete
        tokio::time::sleep(tokio::time::Duration::from_secs(delay + 1)).await;

        assert!(task_manager.is_task_complete().await.unwrap());
    }
}
