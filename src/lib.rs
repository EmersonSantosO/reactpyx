use crate::compiler::{
    compile_all_pyx, compile_pyx_file_to_python, compile_pyx_to_js, update_application,
};
use crate::hooks::{Dispatch, SetState};
use log::info;
use once_cell::sync::Lazy;
use pyo3::{prelude::*, wrap_pyfunction};
use pyo3_asyncio_0_21::tokio::{future_into_py_with_locals, get_current_locals};
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Error al crear el runtime de Tokio"));

#[pymodule]
fn core_reactpyx(py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::init();

    add_hooks_to_module(m)?;
    add_minifiers_to_module(m)?;
    add_jsx_transformers_to_module(m)?;
    add_compiler_to_module(m)?;

    info!("CLI y lógica de compilación migradas correctamente.");

    Ok(())
}

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

fn add_minifiers_to_module(m: &PyModule) -> PyResult<()> {
    use crate::css_minifier::minify_css_code;
    use crate::html_minifier::minify_html_code;
    use crate::js_minifier::minify_js_code;

    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_css_code, m)?)?;
    m.add_function(wrap_pyfunction!(minify_html_code, m)?)?;
    Ok(())
}

fn add_jsx_transformers_to_module(m: &PyModule) -> PyResult<()> {
    use crate::jsx_transformer::{incremental_jsx_transform, parse_jsx};

    m.add_function(wrap_pyfunction!(parse_jsx, m)?)?;
    m.add_function(wrap_pyfunction!(incremental_jsx_transform, m)?)?;
    Ok(())
}

fn add_compiler_to_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compile_all_pyx_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_file_to_python_py, m)?)?;
    m.add_function(wrap_pyfunction!(update_application_py, m)?)?;
    m.add_function(wrap_pyfunction!(compile_pyx_to_js_py, m)?)?;
    Ok(())
}

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
) -> PyResult<Bound<'a, PyAny>> {
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
}

#[pyfunction]
fn compile_pyx_file_to_python_py(
    file_path: &str,
    config_path: &str,
    py: Python<'_>,
) -> PyResult<Bound<'_, PyAny>> {
    validate_path(file_path)?;
    validate_path(config_path)?;

    let path = std::path::PathBuf::from(file_path);
    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        compile_pyx_file_to_python(&path, config_path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
}

#[pyfunction]
fn update_application_py(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: &str,
    py: Python<'_>,
) -> PyResult<Bound<'_, PyAny>> {
    validate_path(module_name)?;
    validate_path(entry_function)?;
    validate_path(project_root)?;

    let project_root_path = std::path::PathBuf::from(project_root);
    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        update_application(module_name, code, entry_function, project_root_path)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
}

#[pyfunction]
fn compile_pyx_to_js_py(
    entry_file: &str,
    config_path: &str,
    output_dir: &str,
    py: Python<'_>,
) -> PyResult<Bound<'_, PyAny>> {
    validate_path(entry_file)?;
    validate_path(config_path)?;
    validate_path(output_dir)?;

    let locals = get_current_locals(py)?;

    future_into_py_with_locals(py, locals, async move {
        compile_pyx_to_js(entry_file, config_path, output_dir)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    })
}
