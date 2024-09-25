// core_reactpyx/src/precompiler.rs

use pyo3::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

#[pyclass]
pub struct JSXPrecompiler {
    pub cache: Arc<Mutex<String>>, // Almacena el resultado de la precompilación
    pub last_modified: Arc<Mutex<u64>>, // Almacena el tiempo de la última modificación del archivo
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

    pub fn precompile_jsx(&self, file_path: &str) -> PyResult<String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                format!("El archivo {} no existe", file_path),
            ));
        }

        let metadata = fs::metadata(path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Error al leer metadatos: {}", e))
        })?;

        let modified_time = metadata
            .modified()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error al obtener tiempo de modificación: {}",
                    e
                ))
            })?
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Error con UNIX_EPOCH: {}", e))
            })?
            .as_secs();

        let mut last_modified = self.last_modified.lock().unwrap();
        if *last_modified != modified_time {
            // Solo recompilamos si el archivo ha sido modificado
            let jsx_code = fs::read_to_string(path).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Error al leer el archivo: {}",
                    e
                ))
            })?;
            let parsed_code = self.transform_jsx_to_python(&jsx_code);
            let mut cache = self.cache.lock().unwrap();
            *cache = parsed_code;
            *last_modified = modified_time;
        }

        let cache = self.cache.lock().unwrap();
        Ok(cache.clone())
    }

    fn transform_jsx_to_python(&self, jsx_code: &str) -> String {
        // Función que transforma el JSX en Python
        jsx_code
            .replace("<", "create_element(")
            .replace("/>", ");")
            .replace(">", ", [")
            .replace("</", "]);")
    }
}
