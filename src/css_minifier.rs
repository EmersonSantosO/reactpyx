use minifier::css::minify;
use pyo3::prelude::*;

#[pyfunction]
pub fn minify_css_code(css_code: &str) -> PyResult<String> {
    match minify(css_code) {
        Ok(minified) => Ok(minified.to_string()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Error al minificar CSS: {}",
            e
        ))),
    }
}
