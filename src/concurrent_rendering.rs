// core_reactpyx/src/concurrent_rendering.rs

use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

    pub fn render_concurrently(&self, delay_secs: u64) {
        let is_complete_clone = Arc::clone(&self.is_complete);

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(delay_secs)); // Simulamos un proceso costoso
            let mut is_complete = is_complete_clone.lock().expect("Error al adquirir el lock");
            *is_complete = true;
        });
    }

    pub fn is_render_complete(&self) -> bool {
        let is_complete = self.is_complete.lock().expect("Error al adquirir el lock");
        *is_complete
    }

    // Funcionalidad para cancelar renderizados futuros
    pub fn reset_render(&self) {
        let mut is_complete = self.is_complete.lock().expect("Error al adquirir el lock");
        *is_complete = false;
    }
}
