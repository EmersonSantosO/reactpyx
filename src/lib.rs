// Modules that make up the core of ReactPyx
mod cli;
mod compiler;
mod css_minifier;
mod event_handler; // Event handler for the framework
mod hooks;
mod html_minifier;
mod js_minifier;
mod jsx_transformer;
mod virtual_dom; // Virtual DOM for component management and updates

use crate::compiler::{compile_all_pyx, compile_pyx_file_to_python, update_application};
use crate::hooks::{Dispatch, SetState};
use crate::virtual_dom::Patch;
use log::{error, info};
use once_cell::sync::Lazy;
use pyo3::{prelude::*, wrap_pyfunction};
use std::sync::Once;
use tokio::runtime::Runtime;

// Initialize Tokio for asynchronous use
static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error creating Tokio runtime")
});

static LOGGER_INIT: Once = Once::new();

/// Main module of ReactPyx in Rust for Python
#[pymodule]
fn core_reactpyx(py: Python, m: &PyModule) -> PyResult<()> {
    // Initialize the logger only once
    LOGGER_INIT.call_once(|| env_logger::init());

    // Register all necessary classes and functions
    m.add_class::<Patch>()?; // Virtual DOM Patch

    // Register additional functions and modules
    add_jsx_transformers_to_module(py, m)?;
    add_hooks_to_module(py, m)?;
    add_minifiers_to_module(py, m)?;
    add_compiler_to_module(py, m)?;
    add_event_handlers_to_module(py, m)?;
    add_virtual_dom_to_module(py, m)?;

    info!("ReactPyx Core and CLI successfully initialized.");
    Ok(())
}

/// Add hooks to the PyO3 module
fn add_hooks_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::hooks::{use_context, use_effect_with_deps, use_lazy_state, use_reducer, use_state};

    // Add hook classes and functions
    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    Ok(())
}

/// Add minifiers to the PyO3 module
fn add_minifiers_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::css_minifier::minify_css_code;
    use crate::html_minifier::minify_html_code;
    use crate::js_minifier::minify_js_code;

    // Add CSS, HTML, and JS minification functions
    m.add_function(wrap_pyfunction!(minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_html_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    Ok(())
}

/// Add JSX transformers to the PyO3 module
fn add_jsx_transformers_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::jsx_transformer::{incremental_jsx_transform, parse_jsx};

    // Add functions to transform JSX
    m.add_function(wrap_pyfunction!(parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(incremental_jsx_transform, m)?)?;
    Ok(())
}

fn add_compiler_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::compiler::{
        compile_all_pyx_py, compile_pyx_file_to_python_py, compile_pyx_to_js_py,
        update_application_py,
    };

    // Add compilation functions
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_file_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    Ok(())
}

/// Add event handlers to the PyO3 module
fn add_event_handlers_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::event_handler::EventHandler; // Import the event handler

    // Add EventHandler class
    m.add_class::<EventHandler>()?;
    Ok(())
}

/// Add Virtual DOM and related functionalities to the PyO3 module
fn add_virtual_dom_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::virtual_dom::VNode;

    // Add VNode class for Virtual DOM
    m.add_class::<VNode>()?;
    Ok(())
}

/// Validate paths to prevent empty paths
fn validate_path(path: &str) -> PyResult<()> {
    if path.trim().is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "The path cannot be empty.",
        ));
    } else if path.contains(&['\\', '/', ':', '*', '?', '"', '<', '>', '|'][..]) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "The path contains invalid characters.",
        ));
    } else {
        Ok(())
    }
}

/// Compile all `.pyx` files in a project
#[pyfunction]
fn compile_all_pyx_py(project_root: &str, config_path: &str, target_env: &str) -> PyResult<()> {
    validate_path(project_root)?;
    validate_path(config_path)?;

    // Execute compilation in Tokio Runtime
    TOKIO_RUNTIME.block_on(async move {
        compile_all_pyx(project_root, config_path, target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    })
}

/// Compile a `.pyx` file to Python
#[pyfunction]
fn compile_pyx_file_to_python_py(
    file_path: &str,
    config_path: &str,
    target_env: &str,
) -> PyResult<()> {
    validate_path(file_path)?;
    validate_path(config_path)?;

    let path = std::path::PathBuf::from(file_path);

    // Execute compilation in Tokio Runtime
    TOKIO_RUNTIME.block_on(async move {
        compile_pyx_file_to_python(&path, config_path, target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    })
}

/// Update the application when source code changes
#[pyfunction]
fn update_application_py(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: &str,
) -> PyResult<()> {
    validate_path(module_name)?;
    validate_path(entry_function)?;
    validate_path(project_root)?;

    // Convert project_root to a String for compatibility
    let project_root_path = project_root.to_string();

    // Execute application update in Tokio Runtime
    TOKIO_RUNTIME.block_on(async move {
        update_application(module_name, code, entry_function, project_root_path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    })
}

/// Compile a `.pyx` file to JavaScript
#[pyfunction]
fn compile_pyx_to_js_py(
    entry_file: &str,
    config_path: &str,
    output_dir: &str,
    target_env: &str,
) -> PyResult<()> {
    validate_path(entry_file)?;
    validate_path(config_path)?;
    validate_path(output_dir)?;

    // Execute compilation to JS in Tokio Runtime
    TOKIO_RUNTIME.block_on(async move {
        compile_pyx_to_js(entry_file, config_path, output_dir, target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    })
}
