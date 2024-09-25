// core_reactpyx/src/jsx_transformer.rs

use pyo3::prelude::*;
use rayon::prelude::*;
use regex::Regex;

/// Transforma JSX de manera incremental utilizando paralelización.
#[pyfunction]
pub fn incremental_jsx_transform(js_code: &str) -> PyResult<String> {
    let parts: Vec<&str> = js_code.split("\n").collect();
    let transformed: Vec<String> = parts
        .par_iter()
        .map(|part| {
            // Ejemplo de transformación JSX
            part.replace("<", "create_element(")
                .replace("/>", ");")
                .replace(">", ", [")
                .replace("</", "]);")
        })
        .collect();

    Ok(transformed.join(""))
}

/// Parsea JSX y lo transforma en código Python.
#[pyfunction]
pub fn parse_jsx(js_code: &str) -> PyResult<String> {
    let mut parsed_code = js_code
        .replace("<", "create_element(")
        .replace("/>", ");")
        .replace(">", ", [")
        .replace("</", "]);");

    // Optimización de expresiones regulares con mejor manejo de errores
    let re = Regex::new(r"\{(.+?)\}").map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error al compilar Regex: {}", e))
    })?;
    parsed_code = re.replace_all(&parsed_code, "str($1)").to_string();

    // Manejo de Fragmentos
    parsed_code = parsed_code
        .replace("<Fragment>", "[]")
        .replace("</Fragment>", "");

    Ok(parsed_code)
}
