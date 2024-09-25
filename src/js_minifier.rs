// core_reactpyx/src/js_minifier.rs

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use regex::Regex;

/// Minifica el código JavaScript compatible con ES2023.
///
/// Args:
///     js_code (str): El código JavaScript a minificar.
///
/// Returns:
///     str: El código JavaScript minificado.
#[pyfunction]
#[pyo3(text_signature = "(js_code)")]
pub fn minify_js_code(js_code: &str) -> PyResult<String> {
    // Eliminar comentarios
    let no_comments = remove_js_comments(js_code).map_err(|e| {
        PyValueError::new_err(format!("Error al eliminar comentarios de JS: {}", e))
    })?;

    // Eliminar espacios en blanco innecesarios
    let no_whitespace = minify_code(&no_comments, true).map_err(|e| {
        PyValueError::new_err(format!(
            "Error al minificar espacios en blanco de JS: {}",
            e
        ))
    })?;

    // Optimizar el código JavaScript
    let optimized_code = optimize_js(&no_whitespace)
        .map_err(|e| PyValueError::new_err(format!("Error al optimizar el código JS: {}", e)))?;

    Ok(optimized_code)
}

/// Elimina los comentarios de un código JavaScript.
fn remove_js_comments(js_code: &str) -> Result<String, regex::Error> {
    let comment_re = Regex::new(r"//[^\n\r]*|/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")?;
    Ok(comment_re.replace_all(js_code, "").to_string())
}

/// Minifica un código genérico eliminando espacios en blanco innecesarios.
///
/// Esta función preserva un espacio entre tokens alfanuméricos y símbolos,
/// pero elimina los espacios al principio y al final de la cadena.
fn minify_code(code: &str, preserve_single_spaces: bool) -> Result<String, regex::Error> {
    // Esta función actualmente no puede fallar, pero se deja como Result para futuras mejoras
    let mut minified = String::new();
    let mut inside_string = false;
    let mut inside_regex = false;
    let mut prev_char = '\0';

    for c in code.chars() {
        match c {
            '"' | '\'' | '`' => {
                inside_string = !inside_string;
                minified.push(c);
            }
            '/' if !inside_string && prev_char == '=' => {
                inside_regex = true;
                minified.push(c);
            }
            '/' if inside_regex => {
                inside_regex = false;
                minified.push(c);
            }
            ' ' | '\n' | '\t' if !inside_string && !inside_regex => {
                if preserve_single_spaces && prev_char.is_alphanumeric() {
                    minified.push(' ');
                }
            }
            _ => minified.push(c),
        }
        prev_char = c;
    }

    Ok(minified.trim().to_string())
}

/// Optimiza el código JavaScript eliminando declaraciones redundantes y simplificando expresiones.
///
/// Esta función elimina las llamadas a `console.log`, los bloques vacíos y los puntos y coma redundantes.
fn optimize_js(code: &str) -> Result<String, regex::Error> {
    let no_console = remove_console_logs(code)?;
    let no_empty_blocks = remove_empty_blocks(&no_console)?;
    let optimized_code = remove_redundant_semicolons(&no_empty_blocks)?;
    Ok(optimized_code)
}

/// Elimina las llamadas a `console.log` del código JavaScript.
fn remove_console_logs(code: &str) -> Result<String, regex::Error> {
    let console_re = Regex::new(r"console\.log\([^\)]*\);?")?;
    Ok(console_re.replace_all(code, "").to_string())
}

/// Elimina los bloques vacíos del código JavaScript.
fn remove_empty_blocks(code: &str) -> Result<String, regex::Error> {
    let empty_block_re = Regex::new(r"\{\s*\}")?;
    Ok(empty_block_re.replace_all(code, "{}").to_string())
}

/// Elimina los puntos y coma redundantes del código JavaScript.
fn remove_redundant_semicolons(code: &str) -> Result<String, regex::Error> {
    let semicolon_re = Regex::new(r";+\s*;")?;
    Ok(semicolon_re.replace_all(code, ";").to_string())
}

/// Módulo de Python para la minificación.
#[pymodule]
fn my_minifier(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(minify_js_code, m)?)?;
    Ok(())
}
