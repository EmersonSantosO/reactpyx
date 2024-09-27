use crate::compiler::sanitize_html;
use pyo3::prelude::*;

#[pyfunction]
pub fn minify_html_code(html_code: &str) -> PyResult<String> {
    // Sanear el HTML usando html5ever
    let sanitized_html = sanitize_html(html_code)?;

    // Minificar el HTML (puedes usar la lógica de minificación original o una biblioteca dedicada)
    let minified = minify_code(&sanitized_html);

    Ok(minified)
}

fn remove_html_comments(html_code: &str) -> String {
    let comment_re = Regex::new(r"<!--[\s\S]*?-->")
        .expect("Error compilando la expresión regular para comentarios de HTML");
    comment_re.replace_all(html_code, "").to_string()
}

fn minify_code(code: &str) -> String {
    let mut minified = String::new();
    let mut inside_tag = false;
    let mut prev_char = '\0';

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
            ' ' | '\n' | '\t' if !inside_tag => {
                if prev_char != ' ' && prev_char != '\n' && prev_char != '\t' {
                    minified.push(' ');
                }
            }
            _ => minified.push(c),
        }
        prev_char = c;
    }

    minified.trim().to_string()
}
