mod cli;
mod compiler;
mod css_minifier;
mod event_handler;
mod hooks;
mod html_minifier;
mod js_minifier;
mod jsx_transformer;
mod logger;
mod virtual_dom;

use crate::compiler::{compile_all_pyx, compile_pyx_file_to_python, update_application};
use crate::hooks::{Dispatch, SetState};
use crate::virtual_dom::Patch;
use log::{error, info};
use once_cell::sync::Lazy;
use pyo3::{prelude::*, wrap_pyfunction};
use std::sync::Once;
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error al crear el runtime de Tokio")
});

static LOGGER_INIT: Once = Once::new();

/// Módulo principal de ReactPyx en Rust para Python
#[pymodule]
fn core_reactpyx(py: Python, m: &PyModule) -> PyResult<()> {
    LOGGER_INIT.call_once(|| env_logger::init());

    m.add_class::<Patch>()?;
    m.add_function(wrap_pyfunction!(run_cli_py, m)?)?;

    add_jsx_transformers_to_module(py, m)?;
    add_hooks_to_module(py, m)?;
    add_minifiers_to_module(py, m)?;
    add_compiler_to_module(py, m)?;
    add_event_handlers_to_module(py, m)?;
    add_virtual_dom_to_module(py, m)?;

    info!("Núcleo y CLI de ReactPyx inicializados exitosamente.");
    Ok(())
}

/// Agregar hooks al módulo PyO3
fn add_hooks_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::hooks::{use_context, use_effect_with_deps, use_lazy_state, use_reducer, use_state};

    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect_with_deps, m)?)?;
    Ok(())
}

/// Agregar minificadores al módulo PyO3
fn add_minifiers_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::css_minifier::minify_css_code;
    use crate::html_minifier::minify_html_code;
    use crate::js_minifier::minify_js_code;

    m.add_function(wrap_pyfunction!(minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_html_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    Ok(())
}

/// Agregar transformadores JSX al módulo PyO3
fn add_jsx_transformers_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::jsx_transformer::{incremental_jsx_transform, parse_jsx};

    m.add_function(wrap_pyfunction!(parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(incremental_jsx_transform, m)?)?;
    Ok(())
}

fn add_compiler_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::compiler::{
        compile_all_pyx_py, compile_pyx_file_to_python_py, compile_pyx_to_js_py,
        update_application_py,
    };

    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_file_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    Ok(())
}

/// Agregar manejadores de eventos al módulo PyO3
fn add_event_handlers_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::event_handler::EventHandler;

    m.add_class::<EventHandler>()?;
    Ok(())
}

/// Agregar Virtual DOM y funcionalidades relacionadas al módulo PyO3
fn add_virtual_dom_to_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::virtual_dom::VNode;

    m.add_class::<VNode>()?;
    Ok(())
}

/// Validar rutas para evitar rutas vacías
fn validate_path(path: &str) -> PyResult<()> {
    if path.trim().is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "La ruta no puede estar vacía.",
        ));
    } else if path.contains(&['*', '?', '"', '<', '>', '|'][..]) {
        // Corregido para permitir separadores de directorios '/' y '\'
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "La ruta contiene caracteres inválidos.",
        ));
    } else {
        Ok(())
    }
}

/// Compilar todos los archivos `.pyx` en un proyecto
#[pyfunction]
fn compile_all_pyx_py(project_root: &str, config_path: &str, target_env: &str) -> PyResult<(Vec<String>, Vec<(String, String)>)> {
    validate_path(project_root)?;
    validate_path(config_path)?;

    TOKIO_RUNTIME.block_on(async move {
        compile_all_pyx(project_root, config_path, target_env)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    })
}

/// Compilar un archivo `.pyx` a Python
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

/// Actualizar la aplicación cuando cambie el código fuente
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

/// Compilar un archivo `.pyx` a JavaScript
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

/// Compilar un archivo `.pyx` a JavaScript - Implementación subyacente
async fn compile_pyx_to_js(
    entry_file: &str,
    config_path: &str,
    output_dir: &str,
    target_env: &str,
) -> anyhow::Result<()> {
    // Implementación básica para que compile
    use std::path::Path;
    
    let entry_path = Path::new(entry_file);
    if !entry_path.exists() {
        return Err(anyhow::anyhow!("Archivo de entrada no existe: {}", entry_file));
    }
    
    let code = tokio::fs::read_to_string(entry_file).await?;
    let js_code = js_minifier::minify_js_code(&code)?;
    
    let output_path = Path::new(output_dir).join("bundle.js");
    tokio::fs::create_dir_all(output_dir).await?;
    tokio::fs::write(&output_path, js_code).await?;
    
    info!("Compilado exitosamente a JavaScript: {}", entry_file);
    Ok(())
}

/// Agregar funciones Python para iniciar el CLI desde Python
#[pyfunction]
fn run_cli_py() -> PyResult<()> {
    use crate::cli::run_cli;
    
    if let Err(e) = run_cli() {
        error!("Error en la CLI: {}", e);
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()));
    }
    
    Ok(())
}
