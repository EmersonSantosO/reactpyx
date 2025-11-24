use dashmap::DashMap;
use once_cell::sync::Lazy;
use pyo3::prelude::*;

#[pyclass]
pub struct Plugin {
    pub name: String,
    pub execute: Py<PyAny>,
}

impl Plugin {
    pub fn new(name: String, execute: Py<PyAny>) -> Self {
        Plugin { name, execute }
    }
}

static PLUGIN_REGISTRY: Lazy<DashMap<String, Plugin>> = Lazy::new(|| DashMap::new());

#[pyfunction]
pub fn register_plugin(name: String, execute: Py<PyAny>) -> PyResult<()> {
    PLUGIN_REGISTRY.insert(name.clone(), Plugin::new(name.clone(), execute));
    println!("Plugin registered: {}", name);
    Ok(())
}

#[pyfunction]
pub fn run_plugin(name: String) -> PyResult<()> {
    if let Some(plugin) = PLUGIN_REGISTRY.get(&name) {
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

#[pymodule]
pub fn plugin_system(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(register_plugin, m)?)?;
    m.add_function(wrap_pyfunction!(run_plugin, m)?)?;
    Ok(())
}
