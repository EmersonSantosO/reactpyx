use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use pyo3::prelude::*;

/// Minifies CSS code using `lightningcss`.
#[pyfunction]
pub fn minify_css_code(css_code: &str) -> PyResult<String> {
    // Parse CSS using `lightningcss`
    let options = ParserOptions::default();
    let stylesheet = StyleSheet::parse(css_code, options).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing CSS: {}", e))
    })?;

    // Minify CSS using `lightningcss`
    let mut printer_options = PrinterOptions::default();
    printer_options.minify = true;

    let minified = stylesheet.to_css(printer_options).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error minifying CSS: {}", e))
    })?;

    Ok(minified.code)
}
