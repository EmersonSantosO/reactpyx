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

// Nuevos módulos relacionados con la CLI y configuración
mod cli;
mod component_detector;
mod config;

use log::info;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use tokio::runtime::Runtime; // Importa Runtime de Tokio para manejo de tareas async

#[pymodule]
fn core_reactpyx(_py: Python, m: &PyModule) -> PyResult<()> {
    // Inicializar el logger
    env_logger::init();

    // Exponer AsyncTaskManager
    m.add_class::<async_task::AsyncTaskManager>()?;

    // Exponer ConcurrentRenderer
    m.add_class::<concurrent_rendering::ConcurrentRenderer>()?;

    // Exponer hooks y sus clases auxiliares
    m.add_class::<hooks::SetState>()?;
    m.add_class::<hooks::Dispatch>()?;
    m.add_function(wrap_pyfunction!(hooks::use_state, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_context, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_effect, m)?)?;

    // Exponer transformador JSX
    m.add_function(wrap_pyfunction!(jsx_transformer::parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(
        jsx_transformer::incremental_jsx_transform,
        m
    )?)?;

    // Exponer minificadores
    m.add_function(wrap_pyfunction!(js_minifier::minify_js_code, m)?)?;
    m.add_function(wrap_pyfunction!(css_minifier::minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(html_minifier::minify_html_code, m)?)?;

    // Exponer Suspense y ErrorBoundary
    m.add_class::<suspense::SuspenseComponent>()?;
    m.add_class::<suspense::ErrorBoundary>()?;

    // Exponer EventHandler
    m.add_class::<event_handler::EventHandler>()?;

    // Exponer LazyComponent
    m.add_class::<lazy_component::LazyComponent>()?;

    // Exponer VNode del Virtual DOM
    m.add_class::<virtual_dom::VNode>()?;

    // Exponer JSXPrecompiler
    m.add_class::<precompiler::JSXPrecompiler>()?;

    // Exponer ComponentParser
    m.add_class::<component_parser::ComponentParser>()?;

    // Exponer funciones del compilador
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_file_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    m.add_function(wrap_pyfunction!(watch_for_changes_py, m)?)?;

    // Exponer funciones de la CLI migradas de `pybolt`
    m.add_function(wrap_pyfunction!(cli::run_cli_py, m)?)?;
    m.add_function(wrap_pyfunction!(cli::create_project_py, m)?)?;
    m.add_function(wrap_pyfunction!(cli::init_project_py, m)?)?;
    m.add_function(wrap_pyfunction!(cli::run_server_py, m)?)?;
    m.add_function(wrap_pyfunction!(cli::build_project_py, m)?)?;

    info!("CLI y lógica de compilación migradas correctamente.");

    Ok(())
}

// Funciones para la compilación y actualización de archivos PyX

#[pyfunction]
fn compile_all_pyx_py(
    project_root: &str,
    config_path: &str,
) -> PyResult<(Vec<String>, Vec<(String, String)>)> {
    let rt = Runtime::new().expect("Error al crear el runtime de Tokio");
    let result = rt.block_on(compiler::compile_all_pyx(project_root, config_path));
    result.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error compilando PyX: {}", e))
    })
}

#[pyfunction]
fn compile_pyx_file_to_python_py(file_path: &str, config_path: &str) -> PyResult<String> {
    let rt = Runtime::new().expect("Error al crear el runtime de Tokio");
    let path = std::path::PathBuf::from(file_path);
    let result = rt.block_on(compiler::compile_pyx_file_to_python(&path, config_path));
    result.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Error compilando PyX a Python: {}",
            e
        ))
    })
}

#[pyfunction]
fn update_application_py(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: &str,
) -> PyResult<()> {
    let rt = Runtime::new().expect("Error al crear el runtime de Tokio");
    let project_root_path = std::path::PathBuf::from(project_root);
    let result = rt.block_on(compiler::update_application(
        module_name,
        code,
        entry_function,
        project_root_path,
    ));
    result.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Error actualizando la aplicación: {}",
            e
        ))
    })
}

#[pyfunction]
fn compile_pyx_to_js_py(entry_file: &str, config_path: &str, output_dir: &str) -> PyResult<()> {
    let rt = Runtime::new().expect("Error al crear el runtime de Tokio");
    let result = rt.block_on(compiler::compile_pyx_to_js(
        entry_file,
        config_path,
        output_dir,
    ));
    result.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error compilando PyX a JS: {}", e))
    })
}

#[pyfunction]
fn watch_for_changes_py(project_root: &str, config_path: &str) -> PyResult<()> {
    compiler::watch_for_changes(project_root, config_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error observando cambios: {}", e))
    })
}
