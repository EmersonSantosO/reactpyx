use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use lightningcss::targets::{Browsers, Targets};
use pyo3::prelude::*;

/// Minifies CSS code using `lightningcss`.
#[pyfunction]
pub fn minify_css_code(css_code: &str) -> PyResult<String> {
    // Parse CSS using `lightningcss`
    let options = ParserOptions::default();
    let mut stylesheet = StyleSheet::parse(css_code, options).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error parsing CSS: {}", e))
    })?;

    // Define targets for autoprefixing (targeting recent modern browsers)
    // This ensures compatibility with Chrome 90+, Firefox 88+, Safari 14+
    let targets = Targets {
        browsers: Some(Browsers {
            chrome: Some(90 << 16),
            firefox: Some(88 << 16),
            safari: Some(14 << 16),
            ..Browsers::default()
        }),
        ..Targets::default()
    };

    // Minify and optimize CSS based on targets (adds vendor prefixes)
    let minify_options = MinifyOptions {
        targets,
        ..MinifyOptions::default()
    };

    stylesheet.minify(minify_options).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error optimizing CSS: {}", e))
    })?;

    // Generate final CSS
    let printer_options = PrinterOptions {
        minify: true,
        targets, // Use targets for printer optimizations too
        ..PrinterOptions::default()
    };

    let minified = stylesheet.to_css(printer_options).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Error generating CSS: {}", e))
    })?;

    Ok(minified.code)
}
