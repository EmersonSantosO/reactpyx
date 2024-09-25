// core_reactpyx/src/html_minifier.rs

use pyo3::prelude::*;
use regex::Regex;

/// Minifica el código HTML.
#[pyfunction]
pub fn minify_html_code(html_code: &str) -> PyResult<String> {
    let no_comments = remove_html_comments(html_code);
    let minified = minify_code(&no_comments);
    Ok(minified)
}

/// Elimina los comentarios de un código HTML.
fn remove_html_comments(html_code: &str) -> String {
    let comment_re = Regex::new(r"<!--[\s\S]*?-->")
        .expect("Error compilando la expresión regular para comentarios de HTML");
    comment_re.replace_all(html_code, "").to_string()
}

/// Minifica un código HTML eliminando espacios en blanco innecesarios.
fn minify_code(code: &str) -> String {
    let mut minified = String::new();
    let mut inside_tag = false;
    let mut _prev_char = '\0'; // Renombrado para evitar la advertencia

    for c in code.chars() {
        match c {
            '<' => {
                inside_tag = true;
                minified.push(c);
            }
            '>' => {
                inside_tag = false;
                minified.push(c);
            }
            ' ' | '\n' | '\t' if !inside_tag => {}
            _ => minified.push(c),
        }
        _prev_char = c;
    }

    minified.trim().to_string()
}
