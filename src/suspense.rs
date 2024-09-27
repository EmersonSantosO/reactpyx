use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyclass]
pub struct SuspenseComponent {
    is_loading: Arc<Mutex<bool>>,      // Cambiado a `Mutex`
    error: Arc<Mutex<Option<String>>>, // Cambiado a `Mutex`
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
            *error = Some("Error en la carga de datos".to_string());
        });
    }

    pub fn is_loading(&self) -> bool {
        let is_loading = self.is_loading.lock().unwrap();
        *is_loading
    }

    pub fn has_error(&self) -> bool {
        let error = self.error.lock().unwrap();
        error.is_some()
    }

    pub fn get_error_message(&self) -> Option<String> {
        let error = self.error.lock().unwrap();
        error.clone()
    }
}

#[pyclass]
pub struct ErrorBoundary {
    has_error: Arc<Mutex<bool>>,               // Cambiado a `Mutex`
    error_message: Arc<Mutex<Option<String>>>, // Cambiado a `Mutex`
}

#[pymethods]
impl ErrorBoundary {
    #[new]
    pub fn new() -> Self {
        ErrorBoundary {
            has_error: Arc::new(Mutex::new(false)),
            error_message: Arc::new(Mutex::new(None)),
        }
    }

    pub fn catch_error(&self, error_message: &str) {
        let mut has_error = self.has_error.lock().unwrap();
        *has_error = true;

        let mut error = self.error_message.lock().unwrap();
        *error = Some(error_message.to_string());
    }

    pub fn has_error(&self) -> bool {
        let has_error = self.has_error.lock().unwrap();
        *has_error
    }

    pub fn get_error_message(&self) -> Option<String> {
        let error_message = self.error_message.lock().unwrap();
        error_message.clone()
    }
}
