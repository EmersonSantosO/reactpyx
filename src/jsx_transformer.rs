use pyo3::prelude::*;
use regex::Regex;

/// Incremental transformation of JSX code to a format compatible with Python
#[pyfunction]
pub fn incremental_jsx_transform(js_code: &str) -> PyResult<String> {
    let transformed: Vec<String> = js_code
        .lines()
        .map(|part| {
            part.replace("<", "create_element(")
                .replace("/>", ");")
                .replace(">", ", [")
                .replace("</", "]);")
        })
        .collect();

    Ok(transformed.join("\n"))
}

/// Full transformation of JSX code to Python-compatible code
#[pyfunction]
pub fn parse_jsx(js_code: &str) -> PyResult<String> {
    let mut parsed_code = js_code
        .replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);");

    // Replace `{}` expressions with string interpolation
    let re = Regex::new(r"\{(.+?)\}").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error compiling Regex: {}", e))
    })?;
    parsed_code = re.replace_all(&parsed_code, "str($1)").to_string();

    Ok(parsed_code)
}
