use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct SuspenseComponent {
    is_loading: Arc<Mutex<bool>>,      // Para estado de carga
    error: Arc<Mutex<Option<String>>>, // Para manejo de errores
}

#[pymethods]
impl SuspenseComponent {
    #[new]
    pub fn new() -> Self {
        SuspenseComponent {
            is_loading: Arc::new(Mutex::new(true)),
            error: Arc::new(Mutex::new(None)),
        }
    }

    pub fn load_data(&self) {
        let is_loading_clone = Arc::clone(&self.is_loading);
        let error_clone = Arc::clone(&self.error);

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(3));
            let mut is_loading = is_loading_clone.lock().unwrap();
            *is_loading = false;

            let mut error = error_clone.lock().unwrap();
            *error = Some("Error de carga de datos".to_string());
        });
    }

    pub fn is_loading(&self) -> bool {
        *self.is_loading.lock().unwrap()
    }

    pub fn has_error(&self) -> bool {
        self.error.lock().unwrap().is_some()
    }

    pub fn get_error_message(&self) -> Option<String> {
        self.error.lock().unwrap().clone()
    }
}
