// core_reactpyx/src/css_minifier.rs

use pyo3::prelude::*;
use regex::Regex;
use std::collections::HashSet;

/// Minifica el código CSS, eliminando estilos no referenciados en HTML o JS.
#[pyfunction]
pub fn minify_css_code(css_code: &str, html_code: &str, js_code: &str) -> PyResult<String> {
    let no_comments = remove_css_comments(css_code);
    let mut used_selectors = HashSet::new();
    extract_used_selectors(html_code, &mut used_selectors);
    extract_used_selectors(js_code, &mut used_selectors);
    let minified = minify_css_with_used_selectors(&no_comments, &used_selectors);
    Ok(minified)
}

/// Elimina los comentarios de un código CSS.
fn remove_css_comments(css_code: &str) -> String {
    let comment_re = Regex::new(r"/\*[\s\S]*?\*/")
        .expect("Error compilando la expresión regular para comentarios de CSS");
    comment_re.replace_all(css_code, "").to_string()
}

/// Minifica el código CSS, conservando solo los selectores usados.
fn minify_css_with_used_selectors(css_code: &str, used_selectors: &HashSet<String>) -> String {
    let selector_re = Regex::new(r"(?m)^([\w\.\#\-]+)\s*\{([^}]*)\}")
        .expect("Error compilando la expresión regular para selectores de CSS");

    let mut minified = String::new();

    for cap in selector_re.captures_iter(css_code) {
        let selector = cap[1].trim();
        let declarations = cap[2].trim();

        if is_selector_used(selector, used_selectors) {
            minified.push_str(selector);
            minified.push('{');
            minified.push_str(&declarations.replace('\n', "").replace("  ", ""));
            minified.push('}');
        }
    }

    minified
}

/// Extrae los selectores CSS usados en un código HTML o JavaScript.
fn extract_used_selectors(code: &str, selectors: &mut HashSet<String>) {
    let class_re = Regex::new(r#"(?:class|className)=["']([^"']+)["']"#).unwrap();
    let id_re = Regex::new(r#"id=["']([^"']+)["']"#).unwrap();

    for cap in class_re.captures_iter(code) {
        for cls in cap[1].split_whitespace() {
            selectors.insert(format!(".{}", &cls.to_string()));
        }
    }

    for cap in id_re.captures_iter(code) {
        selectors.insert(format!("#{}", &cap[1].to_string()));
    }
}

/// Comprueba si un selector CSS está siendo utilizado.
fn is_selector_used(selector: &str, used_selectors: &HashSet<String>) -> bool {
    for sel in selector.split(',') {
        let sel = sel.trim();
        if used_selectors.contains(sel) {
            return true;
        }
    }
    false
}
