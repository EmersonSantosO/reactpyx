mod cli;
mod compiler;
mod css_minifier;
mod event_handler;
mod hooks;
mod html_minifier;
mod js_minifier;
mod jsx_transformer;
mod lazy_component;
mod logger;
mod plugin_system;
mod precompiler;
mod suspense;
mod virtual_dom;

use crate::compiler::{compile_all_pyx, compile_pyx_file_to_python, update_application};
use crate::hooks::{Dispatch, SetState};
use crate::virtual_dom::Patch;
use log::{error, info};
use once_cell::sync::Lazy;
use pyo3::{prelude::*, wrap_pyfunction};
use std::ffi::CString;
use std::sync::Once;
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error creating Tokio runtime")
});

static LOGGER_INIT: Once = Once::new();

/// Main ReactPyx module in Rust for Python
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    LOGGER_INIT.call_once(|| env_logger::init());

    m.add_class::<Patch>()?;
    m.add_class::<crate::lazy_component::LazyComponent>()?;
    m.add_class::<crate::suspense::SuspenseComponent>()?;
    m.add_function(wrap_pyfunction!(run_cli_py, m)?)?;

    add_jsx_transformers_to_module(m)?;
    add_hooks_to_module(m)?;
    add_minifiers_to_module(m)?;
    add_compiler_to_module(m)?;
    add_event_handlers_to_module(m)?;
    add_virtual_dom_to_module(m)?;
    add_css_compiler_to_module(m)?; // Add the CSS compiler module

    info!("ReactPyx core and CLI successfully initialized.");
    Ok(())
}

/// Add hooks to PyO3 module
fn add_hooks_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::hooks::{
        use_context, use_effect, use_effect_with_deps, use_lazy_state, use_reducer, use_state,
    };

    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect, m)?)?;
    Ok(())
}

/// Add minifiers to PyO3 module
fn add_minifiers_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::css_minifier::minify_css_code;
    use crate::html_minifier::minify_html_code;
    use crate::js_minifier::minify_js_code;

    m.add_function(wrap_pyfunction!(minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_html_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    Ok(())
}

/// Add JSX transformers to PyO3 module
fn add_jsx_transformers_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::jsx_transformer::{incremental_jsx_transform, parse_jsx};

    m.add_function(wrap_pyfunction!(parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(incremental_jsx_transform, m)?)?;
    Ok(())
}

fn add_compiler_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_file_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    Ok(())
}

/// Add event handlers to PyO3 module
fn add_event_handlers_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::event_handler::EventHandler;

    m.add_class::<EventHandler>()?;
    Ok(())
}

/// Add Virtual DOM and related functionalities to PyO3 module
fn add_virtual_dom_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::virtual_dom::VNode;

    m.add_class::<VNode>()?;
    Ok(())
}

/// Validate paths to avoid empty paths
fn validate_path(path: &str) -> PyResult<()> {
    if path.trim().is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Path cannot be empty.",
        ));
    } else if path.contains(&['*', '?', '"', '<', '>', '|'][..]) {
        // Fixed to allow directory separators '/' and '\'
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Path contains invalid characters.",
        ));
    } else {
        Ok(())
    }
}

/// Compile all `.pyx` files in a project
#[pyfunction]
fn compile_all_pyx_py(
    project_root: &str,
    _config_path: &str,
    _target_env: &str,
) -> PyResult<(Vec<String>, Vec<(String, String)>)> {
    validate_path(project_root)?;
    // validate_path(_config_path)?;

    TOKIO_RUNTIME.block_on(async move {
        compile_all_pyx(project_root, _config_path, _target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    })
}

/// Compile a `.pyx` file to Python
#[pyfunction]
fn compile_pyx_file_to_python_py(
    file_path: &str,
    config_path: &str,
    target_env: &str,
) -> PyResult<(String, String, String)> {
    validate_path(file_path)?;
    validate_path(config_path)?;

    let path = std::path::PathBuf::from(file_path);

    TOKIO_RUNTIME.block_on(async move {
        compile_pyx_file_to_python(&path, config_path, target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    })
}

/// Update the application when the source code changes
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

    let project_root_path = project_root.to_string();

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

    TOKIO_RUNTIME.block_on(async move {
        compile_pyx_to_js(entry_file, config_path, output_dir, target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    })
}

/// Compile a `.pyx` file to JavaScript - Underlying implementation
async fn compile_pyx_to_js(
    entry_file: &str,
    _config_path: &str,
    output_dir: &str,
    _target_env: &str,
) -> anyhow::Result<()> {
    use std::path::Path;

    let entry_path = Path::new(entry_file);
    if !entry_path.exists() {
        return Err(anyhow::anyhow!("Input file does not exist: {}", entry_file));
    }

    let code = tokio::fs::read_to_string(entry_file).await?;
    let js_code = js_minifier::minify_js_code(&code)?;

    let output_path = Path::new(output_dir).join("bundle.js");
    tokio::fs::create_dir_all(output_dir).await?;
    tokio::fs::write(&output_path, js_code).await?;

    info!("Successfully compiled to JavaScript: {}", entry_file);
    Ok(())
}

/// Add Python functions to start the CLI from Python
#[pyfunction]
fn run_cli_py() -> PyResult<()> {
    use crate::cli::run_cli_with_args;

    let args: Vec<String> = Python::with_gil(|py| {
        let sys = py.import("sys")?;
        sys.getattr("argv")?.extract()
    })?;

    if let Err(e) = run_cli_with_args(args) {
        error!("Error in CLI: {}", e);
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            e.to_string(),
        ));
    }

    Ok(())
}

/// Import and register the CSS compiler Python module
fn import_css_compiler(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let module_code = include_str!("python/css_compiler.py");
    let code = CString::new(module_code).unwrap();
    let file_name = CString::new("reactpyx.css_compiler").unwrap();
    let module_name = CString::new("reactpyx.css_compiler").unwrap();

    let module = PyModule::from_code(py, &code, &file_name, &module_name)?;
    Ok(module)
}

/// Add Python CSS compiler functions to the module
fn add_css_compiler_to_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let css_compiler = import_css_compiler(m.py())?;

    // Import functions from css_compiler module
    let extract_css = css_compiler.getattr("extract_css_from_pyx")?;
    let compile_css = css_compiler.getattr("compile_css_modules")?;
    let process_tailwind = css_compiler.getattr("process_tailwind_classes")?;
    let integrate_framework = css_compiler.getattr("integrate_framework_styles")?;

    // Add functions to the main module
    m.add("extract_css_from_pyx", extract_css)?;
    m.add("compile_css_modules", compile_css)?;
    m.add("process_tailwind_classes", process_tailwind)?;
    m.add("integrate_framework_styles", integrate_framework)?;

    Ok(())
}
