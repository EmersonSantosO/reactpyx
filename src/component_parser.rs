use log::{error, info};
use pyo3::prelude::*;
use std::error::Error;
use tokio::fs;
use tree_sitter::{Node, Parser};
use tree_sitter_python::language as python_language;

#[pyclass]
pub struct ComponentParser {
    parser: Parser,
}

#[pymethods]
impl ComponentParser {
    #[new]
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(python_language())
            .expect("Failed to set language");
        Self { parser }
    }

    #[pyo3(text_signature = "($self, file_path)")]
    pub fn detect_components_in_file(&mut self, file_path: &str) -> PyResult<Vec<String>> {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        let result = rt.block_on(async {
            let source = fs::read_to_string(file_path).await?;
            let tree = self
                .parser
                .parse(&source, None)
                .ok_or_else(|| format!("Failed to parse file: {}", file_path))?;
            let root_node = tree.root_node();

            let mut components = Vec::new();

            for child in root_node.children(&mut root_node.walk()) {
                if child.kind() == "function_definition" {
                    if let Some(name) = get_function_name(&child, &source) {
                        if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                            if function_returns_jsx(&child, &source) {
                                components.push(name);
                            } else {
                                info!(
                                    "Function '{}' does not return JSX. It is not considered a component.",
                                    name
                                );
                            }
                        }
                    } else {
                        error!(
                            "Could not get function name in file: {}",
                            file_path
                        );
                    }
                }
            }

            Ok::<_, Box<dyn Error>>(components)
        });

        result.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }
}

// Separar la lógica específica de `tree-sitter` fuera de `ComponentParser`
fn get_function_name(node: &Node, source: &str) -> Option<String> {
    node.children(&mut node.walk())
        .find(|child| child.kind() == "identifier")
        .and_then(|identifier_node| {
            identifier_node
                .utf8_text(source.as_bytes())
                .ok()
                .map(String::from)
        })
}

fn function_returns_jsx(node: &Node, source: &str) -> bool {
    let mut nodes_to_visit = vec![node.clone()];

    while let Some(current_node) = nodes_to_visit.pop() {
        if current_node.kind() == "return_statement" {
            if let Some(expr) = current_node.child_by_field_name("expression") {
                if let Ok(text) = expr.utf8_text(source.as_bytes()) {
                    // Si el texto tiene la forma de un elemento JSX
                    return text.trim().starts_with('<');
                }
            }
        }

        // Agregar todos los hijos del nodo actual a la lista de nodos a visitar
        nodes_to_visit.extend(current_node.children(&mut current_node.walk()));
    }
    false
}

#[pymodule]
fn component_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ComponentParser>()?;
    Ok(())
}
