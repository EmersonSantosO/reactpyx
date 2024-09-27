use minifier::js::minify;
use pyo3::prelude::*;

#[pyfunction]
pub fn minify_js_code(js_code: &str, sourcemaps: bool) -> PyResult<(String, Option<String>)> {
    let minified = minify(js_code).to_string();
    let sourcemap = if sourcemaps {
        Some(generate_sourcemap(js_code))
    } else {
        None
    };
    Ok((minified, sourcemap))
}

fn generate_sourcemap(_code: &str) -> String {
    String::from("sourcemap_placeholder")
}
