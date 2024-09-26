// core_reactpyx/src/lib.rs

mod async_task;
mod compiler;
mod component_parser;
mod concurrent_rendering;
mod css_minifier;
mod event_handler;
mod hooks;
mod html_minifier;
mod js_minifier;
mod jsx_transformer;
mod lazy_component;
mod precompiler;
mod suspense;
mod virtual_dom;

use crate::compiler::{
    compile_all_pyx, compile_pyx_to_js, compile_pyx_to_python, update_application,
    watch_for_changes,
};
use crate::css_minifier::minify_css_code;
use crate::html_minifier::minify_html_code;
use crate::js_minifier::minify_js_code;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::path::PathBuf;

#[pymodule]
fn core_reactpyx(py: Python, m: &PyModule) -> PyResult<()> {
    // Inicializar el logger
    env_logger::init();

    // Exponer el AsyncTaskManager
    m.add_class::<async_task::AsyncTaskManager>()?;

    // Exponer ConcurrentRenderer
    m.add_class::<concurrent_rendering::ConcurrentRenderer>()?;

    // Exponer hooks
    m.add_function(wrap_pyfunction!(hooks::use_state, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::set_state, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_lazy_state, m)?)?;

    // Exponer transformador JSX
    m.add_function(wrap_pyfunction!(jsx_transformer::parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(
        jsx_transformer::incremental_jsx_transform,
        m
    )?)?;

    // Exponer minificadores
    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_html_code, m)?)?;

    // Exponer Suspense y ErrorBoundary
    m.add_class::<suspense::SuspenseComponent>()?;
    m.add_class::<suspense::ErrorBoundary>()?;

    // Exponer Event Handler
    m.add_class::<event_handler::EventHandler>()?;

    // Exponer Lazy Component
    m.add_class::<lazy_component::LazyComponent>()?;

    // Exponer Virtual DOM
    m.add_class::<virtual_dom::VNode>()?;
    m.add_function(wrap_pyfunction!(virtual_dom::render_vnode, m)?)?;

    // Exponer Precompiler
    m.add_class::<precompiler::JSXPrecompiler>()?;

    // Exponer funciones del compilador
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    m.add_function(wrap_pyfunction!(watch_for_changes_py, m)?)?;

    Ok(())
}

#[pyfunction]
fn compile_all_pyx_py(
    project_root: &str,
    config_path: &str,
) -> PyResult<(Vec<String>, Vec<(String, String)>)> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(compile_all_pyx(project_root, config_path));
    match result {
        Ok(tuple) => Ok(tuple),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error compilando PyX: {}",
            e
        ))),
    }
}

#[pyfunction]
fn compile_pyx_to_python_py(entry_file: &str, config_path: &str) -> PyResult<String> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(compile_pyx_to_python(entry_file, config_path));
    match result {
        Ok(code) => Ok(code),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error compilando PyX a Python: {}",
            e
        ))),
    }
}

#[pyfunction]
fn update_application_py(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: &str,
) -> PyResult<()> {
    let project_root_path = PathBuf::from(project_root);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(update_application(
        module_name,
        code,
        entry_function,
        project_root_path,
    ));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error actualizando la aplicación: {}",
            e
        ))),
    }
}

#[pyfunction]
fn compile_pyx_to_js_py(entry_file: &str, config_path: &str, output_dir: &str) -> PyResult<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(compile_pyx_to_js(entry_file, config_path, output_dir));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error compilando PyX a JS: {}",
            e
        ))),
    }
}

#[pyfunction]
fn watch_for_changes_py(project_root: &str, config_path: &str) -> PyResult<()> {
    match watch_for_changes(project_root, config_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error observando cambios: {}",
            e
        ))),
    }
}
