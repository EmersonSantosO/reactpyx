use pyo3::prelude::*;
use regex::Regex;

/// Incremental transformation of JSX code to a Python-compatible format
#[pyfunction]
pub fn incremental_jsx_transform(js_code: &str) -> PyResult<String> {
    let transformed: Vec<String> = js_code
        .lines()
        .map(|part| transform_jsx_line(part))
        .collect();

    Ok(transformed.join("\n"))
}

/// Complete transformation of JSX code to Python-compatible code
#[pyfunction]
pub fn parse_jsx(js_code: &str) -> PyResult<String> {
    let base_transformed = transform_jsx_base(js_code);

    // Replace `{}` expressions with string interpolation
    let re = Regex::new(r"\{(.+?)\}").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error compiling Regex: {}", e))
    })?;
    let parsed_code = re.replace_all(&base_transformed, "str($1)").to_string();

    Ok(parsed_code)
}

// Helper function for basic JSX transformation
pub fn transform_jsx_base(jsx_code: &str) -> String {
    jsx_code
        .replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);")
}

// Helper function to transform individual lines
pub fn transform_jsx_line(line: &str) -> String {
    line.replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);")
}
