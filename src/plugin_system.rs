use anyhow::Result;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct Plugin {
    pub name: String,
    pub execute: PyObject,
}

impl Plugin {
    pub fn new(name: String, execute: PyObject) -> Self {
        Plugin { name, execute }
    }
}

#[pymodule]
fn plugin_system(m: &PyModule) -> PyResult<()> {
    let plugin_registry: HashMap<String, Plugin> = HashMap::new();

    #[pyfunction]
    pub fn register_plugin(name: String, execute: PyObject) -> PyResult<()> {
        Python::with_gil(|py| {
            plugin_registry.insert(name.clone(), Plugin::new(name, execute));
            println!("Plugin registered: {}", name);
        });
        Ok(())
    }

    #[pyfunction]
    pub fn run_plugin(name: String) -> PyResult<()> {
        if let Some(plugin) = plugin_registry.get(&name) {
            Python::with_gil(|py| {
                let _ = plugin
                    .execute
                    .call1(py, ())
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()));
            });
        } else {
            println!("Plugin not found: {}", name);
        }
        Ok(())
    }

    m.add_function(wrap_pyfunction_bound!(register_plugin, m)?)?;
    m.add_function(wrap_pyfunction_bound!(run_plugin, m)?)?;
    Ok(())
}
