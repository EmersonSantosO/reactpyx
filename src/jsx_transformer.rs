use pyo3::prelude::*;
use regex::Regex;

/// Transformación incremental de código JSX a un formato compatible con Python
#[pyfunction]
pub fn incremental_jsx_transform(js_code: &str) -> PyResult<String> {
    let transformed: Vec<String> = js_code
        .lines()
        .map(|part| transform_jsx_line(part))
        .collect();

    Ok(transformed.join("\n"))
}

/// Transformación completa de código JSX a código compatible con Python
#[pyfunction]
pub fn parse_jsx(js_code: &str) -> PyResult<String> {
    let base_transformed = transform_jsx_base(js_code);

    // Reemplazar expresiones `{}` con interpolación de cadenas
    let re = Regex::new(r"\{(.+?)\}").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error al compilar Regex: {}", e))
    })?;
    let parsed_code = re.replace_all(&base_transformed, "str($1)").to_string();

    Ok(parsed_code)
}

// Función auxiliar para transformación básica de JSX
pub fn transform_jsx_base(jsx_code: &str) -> String {
    jsx_code
        .replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);")
}

// Función auxiliar para transformar líneas individuales
pub fn transform_jsx_line(line: &str) -> String {
    line.replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);")
}
