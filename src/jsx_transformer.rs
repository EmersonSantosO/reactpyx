use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn incremental_jsx_transform(js_code: &str) -> PyResult<String> {
    let parts: Vec<&str> = js_code.split('\n').collect();
    let transformed: Vec<String> = parts
        .iter()
        .map(|part| {
            part.replace("<", "create_element(")
                .replace("/>", ");")
                .replace(">", ", [")
                .replace("</", "]);")
        })
        .collect();

    Ok(transformed.join(""))
}

#[pyfunction]
pub fn parse_jsx(js_code: &str) -> PyResult<String> {
    let mut parsed_code = js_code
        .replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);");

    let re = Regex::new(r"\{(.+?)\}").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error al compilar Regex: {}", e))
    })?;
    parsed_code = re.replace_all(&parsed_code, "str($1)").to_string();

    Ok(parsed_code)
}
