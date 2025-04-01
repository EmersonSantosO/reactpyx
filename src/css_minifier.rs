use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use pyo3::prelude::*;

/// Minifica código CSS usando `lightningcss`.
#[pyfunction]
pub fn minify_css_code(css_code: &str) -> PyResult<String> {
    // Analiza el CSS usando `lightningcss`
    let options = ParserOptions::default();
    let stylesheet = StyleSheet::parse(css_code, options).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error al analizar CSS: {}", e))
    })?;

    // Minifica el CSS usando `lightningcss`
    let minified = stylesheet.to_css(PrinterOptions::default()).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error al minificar CSS: {}", e))
    })?;

    Ok(minified.code)
}
