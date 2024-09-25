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
};

use crate::css_minifier::minify_css_code;
use crate::html_minifier::minify_html_code;
use crate::js_minifier::minify_js_code;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Módulo principal para core_reactpyx, que expone todas las funcionalidades mejoradas
#[pymodule]
fn core_reactpyx(py: Python, m: &PyModule) -> PyResult<()> {
    // Inicializar el logger
    env_logger::init();

    // Exponer el AsyncTaskManager
    m.add_class::<async_task::AsyncTaskManager>()?;

    // Exponer ConcurrentRenderer para el renderizado concurrente
    m.add_class::<concurrent_rendering::ConcurrentRenderer>()?;

    // Exponer hooks de estado
    m.add_function(wrap_pyfunction!(hooks::use_state, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::set_state, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(hooks::use_lazy_state, m)?)?;

    // Exponer transformador JSX y soporte incremental
    m.add_function(wrap_pyfunction!(jsx_transformer::parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(
        jsx_transformer::incremental_jsx_transform,
        m
    )?)?;

    // Exponer minificadores (JavaScript, CSS, HTML)
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

    // Exponer Virtual DOM y reconciliación optimizada
    m.add_class::<virtual_dom::VNode>()?;
    m.add_function(wrap_pyfunction!(virtual_dom::render_vnode, m)?)?;

    // Exponer Precompiler para transformar JSX a Python
    m.add_class::<precompiler::JSXPrecompiler>()?;

    // Exponer funciones del compilador
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;

    Ok(())
}

/// Función Python para compilar todos los archivos `.pyx` usando PyBolt.
/// Se expone a Python como `compile_all_pyx`.
///
/// # Arguments
///
/// * `project_root` - Ruta al directorio raíz del proyecto.
/// * `config_path` - Ruta al archivo de configuración.
///
/// # Returns
///
/// Una tupla que contiene:
///
/// * Una lista de los archivos `.pyx` compilados correctamente.
/// * Una lista de tuplas con los errores de compilación. Cada tupla contiene la ruta del archivo y el mensaje de error.
#[pyfunction]
fn compile_all_pyx_py(
    project_root: &str,
    config_path: &str,
) -> PyResult<(Vec<String>, Vec<(String, String)>)> {
    let result = futures::executor::block_on(compile_all_pyx(project_root, config_path));
    match result {
        Ok(tuple) => Ok(tuple),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error compilando PyX: {}",
            e
        ))),
    }
}

/// Función Python para compilar un archivo `.pyx` a Python.
/// Se expone a Python como `compile_pyx_to_python`.
///
/// # Arguments
///
/// * `entry_file` - Ruta al archivo `.pyx` de entrada.
/// * `config_path` - Ruta al archivo de configuración.
///
/// # Returns
///
/// El código Python compilado a partir del archivo `.pyx`.
#[pyfunction]
fn compile_pyx_to_python_py(entry_file: &str, config_path: &str) -> PyResult<String> {
    let result = futures::executor::block_on(compile_pyx_to_python(entry_file, config_path));
    match result {
        Ok(code) => Ok(code),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error compilando PyX a Python: {}",
            e
        ))),
    }
}

/// Función Python para actualizar la aplicación de FastAPI con el código optimizado.
/// Se expone a Python como `update_application`.
///
/// # Arguments
///
/// * `module_name` - Nombre del módulo Python que contiene la aplicación FastAPI.
/// * `code` - Código Python de la aplicación FastAPI.
/// * `entry_function` - Nombre de la función de entrada de la aplicación FastAPI.
/// * `project_root` - Ruta al directorio raíz del proyecto.
#[pyfunction]
fn update_application_py(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: &str,
) -> PyResult<()> {
    let project_root_path = std::path::Path::new(project_root).to_path_buf();
    let result = futures::executor::block_on(update_application(
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

/// Función Python para compilar PyX a JavaScript usando PyBolt.
/// Se expone a Python como `compile_pyx_to_js`.
///
/// # Arguments
///
/// * `entry_file` - Ruta al archivo `.pyx` de entrada.
/// * `config_path` - Ruta al archivo de configuración.
/// * `output_dir` - Ruta al directorio de salida para el código JavaScript compilado.
#[pyfunction]
fn compile_pyx_to_js_py(entry_file: &str, config_path: &str, output_dir: &str) -> PyResult<()> {
    let result =
        futures::executor::block_on(compile_pyx_to_js(entry_file, config_path, output_dir));
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PyValueError::new_err(format!(
            "Error compilando PyX a JS: {}",
            e
        ))),
    }
}
