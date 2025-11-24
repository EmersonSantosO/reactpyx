use pyo3::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

#[pyclass]
pub struct JSXPrecompiler {
    pub cache: Arc<Mutex<String>>, // Use `Mutex` for concurrency handling
    pub last_modified: Arc<Mutex<u64>>, // Use `Mutex` to handle last modification time
}

#[pymethods]
impl JSXPrecompiler {
    #[new]
    pub fn new() -> Self {
        JSXPrecompiler {
            cache: Arc::new(Mutex::new(String::new())),
            last_modified: Arc::new(Mutex::new(0)),
        }
    }

    /// Precompiles the JSX file and updates the cache if needed
    pub fn precompile_jsx(&self, file_path: &str) -> PyResult<String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                format!("The file {} does not exist", file_path),
            ));
        }

        // Get the file's last modification time
        let metadata = fs::metadata(path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Error reading metadata: {}", e))
        })?;
        let modified_time = metadata
            .modified()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error getting modification time: {}",
                    e
                ))
            })?
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Error with UNIX_EPOCH: {}", e))
            })?
            .as_secs();

        // Check if the file has changed since last time
        let mut last_modified = self.last_modified.lock().unwrap();
        if *last_modified != modified_time {
            let jsx_code = fs::read_to_string(path).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error reading the file: {}",
                    e
                ))
            })?;

            // Transform JSX to Python
            let parsed_code = self.transform_jsx_to_python(&jsx_code);
            let mut cache = self.cache.lock().unwrap();
            *cache = parsed_code;
            *last_modified = modified_time;
        }

        let cache = self.cache.lock().unwrap();
        Ok(cache.clone())
    }

    /// Logic to transform JSX to Python
    fn transform_jsx_to_python(&self, jsx_code: &str) -> String {
        // Use the shared JSX transformer logic
        crate::jsx_transformer::parse_jsx(jsx_code).unwrap_or_else(|e| {
            eprintln!("Error transforming JSX: {}", e);
            jsx_code.to_string()
        })
    }
}
