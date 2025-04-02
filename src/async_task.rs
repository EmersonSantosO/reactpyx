use pyo3::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::time::{sleep, Duration};

#[pyclass]
pub struct AsyncTaskManager {
    is_complete: Arc<AtomicBool>, // AtomicBool for state
}

#[pymethods]
impl AsyncTaskManager {
    #[new]
    pub fn new() -> Self {
        Self {
            is_complete: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Runs an asynchronous task with a specified delay.
    #[allow(dead_code)]
    pub fn run_async_task(&self, delay_secs: u64) -> PyResult<()> {
        let is_complete_clone = Arc::clone(&self.is_complete);
        tokio::spawn(async move {
            sleep(Duration::from_secs(delay_secs)).await;
            is_complete_clone.store(true, Ordering::SeqCst); // Use sequential consistency for safety
        });

        Ok(())
    }

    /// Checks if the asynchronous task has completed.
    pub fn is_task_complete(&self) -> bool {
        self.is_complete.load(Ordering::SeqCst)
    }
}
