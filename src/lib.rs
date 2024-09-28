mod cli;
mod compiler;
mod css_minifier;
mod hooks;
mod html_minifier;
mod js_minifier;
mod jsx_transformer;
mod virtual_dom;

use crate::cli::{build_project_py, create_project_py, init_project_py, run_cli_py, run_server_py};
use crate::compiler::{
    compile_all_pyx, compile_pyx_file_to_python, compile_pyx_to_js, update_application,
};
use crate::hooks::{Dispatch, SetState};
use crate::virtual_dom::Patch;
use log::info;
use once_cell::sync::Lazy;
use pyo3::{prelude::*, wrap_pyfunction};
use pyo3_asyncio_0_21::tokio::{future_into_py_with_locals, get_current_locals};
use std::sync::Once;
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error al crear el runtime de Tokio")
});

static LOGGER_INIT: Once = Once::new();

#[pymodule]
fn core_reactpyx(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Asegura que `env_logger` solo se inicialice una vez
    LOGGER_INIT.call_once(|| env_logger::init());

    // Exponer clases y funciones
    m.add_class::<Patch>()?;
    add_hooks_to_module(m)?;
    add_minifiers_to_module(m)?;
    add_jsx_transformers_to_module(m)?;
    add_compiler_to_module(m)?;
    add_cli_to_module(m)?;

    info!("CLI y lógica de compilación migradas correctamente.");

    Ok(())
}

/// Función para agregar hooks al módulo de PyO3
fn add_hooks_to_module(m: &PyModule) -> PyResult<()> {
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

/// Función para agregar minificadores al módulo de PyO3
fn add_minifiers_to_module(m: &PyModule) -> PyResult<()> {
    use crate::css_minifier::minify_css_code;
    use crate::html_minifier::minify_html_code;
    use crate::js_minifier::minify_js_code;

    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_html_code, m)?)?;
    Ok(())
}

/// Función para agregar transformadores de JSX al módulo de PyO3
fn add_jsx_transformers_to_module(m: &PyModule) -> PyResult<()> {
    use crate::jsx_transformer::{incremental_jsx_transform, parse_jsx};

    m.add_function(wrap_pyfunction!(parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(incremental_jsx_transform, m)?)?;
    Ok(())
}

/// Función para agregar funcionalidades del compilador al módulo de PyO3
fn add_compiler_to_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_file_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    Ok(())
}

/// Función para agregar funcionalidades del CLI al módulo de PyO3
fn add_cli_to_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli_py, m)?)?;
    m.add_function(wrap_pyfunction!(create_project_py, m)?)?;
    m.add_function(wrap_pyfunction!(init_project_py, m)?)?;
    m.add_function(wrap_pyfunction!(run_server_py, m)?)?;
    m.add_function(wrap_pyfunction!(build_project_py, m)?)?;
    Ok(())
}

/// Validar rutas para asegurar que no estén vacías
fn validate_path(path: &str) -> PyResult<()> {
    if path.trim().is_empty() {
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "El path no puede estar vacío",
        ))
    } else {
        Ok(())
    }
}
#[pyfunction]
fn compile_all_pyx_py<'a>(
    project_root: &'a str,
    config_path: &'a str,
    py: Python<'a>,
) -> PyResult<Py<PyAny>> {
    validate_path(project_root)?;
    validate_path(config_path)?;

    let project_root = project_root.to_string();
    let config_path = config_path.to_string();

    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        compile_all_pyx(&project_root, &config_path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
    .map(|res| res.into())
}

#[pyfunction]
fn compile_pyx_file_to_python_py<'a>(
    file_path: &'a str,
    config_path: &'a str,
    py: Python<'a>,
) -> PyResult<Py<PyAny>> {
    validate_path(file_path)?;
    validate_path(config_path)?;

    let path = std::path::PathBuf::from(file_path);
    let config_path = config_path.to_string();
    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        compile_pyx_file_to_python(&path, &config_path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
    .map(|res| res.into())
}

#[pyfunction]
fn update_application_py<'a>(
    module_name: &'a str,
    code: &'a str,
    entry_function: &'a str,
    project_root: &'a str,
    py: Python<'a>,
) -> PyResult<Py<PyAny>> {
    let module_name = module_name.to_string();
    let code = code.to_string();
    let entry_function = entry_function.to_string();
    let project_root = project_root.to_string();

    let project_root_path = std::path::PathBuf::from(project_root);
    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        update_application(&module_name, &code, &entry_function, project_root_path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
    .map(|res| res.into())
}

#[pyfunction]
fn compile_pyx_to_js_py<'a>(
    entry_file: &'a str,
    config_path: &'a str,
    output_dir: &'a str,
    py: Python<'a>,
) -> PyResult<Py<PyAny>> {
    let entry_file = entry_file.to_string();
    let config_path = config_path.to_string();
    let output_dir = output_dir.to_string();

    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        compile_pyx_to_js(&entry_file, &config_path, &output_dir)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
    .map(|res| res.into())
}
