// core_reactpyx/src/component_parser.rs

use log::{error, info};
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tree_sitter::{Node, Parser};
use tree_sitter_python::language as python_language;

pub struct ComponentParser {
    parser: Parser,
}

impl ComponentParser {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut parser = Parser::new();
        parser.set_language(python_language())?;
        Ok(Self { parser })
    }

    pub async fn detect_components_in_file(
        &mut self,
        file_path: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let source = fs::read_to_string(file_path).await?;
        let tree = self
            .parser
            .parse(&source, None)
            .ok_or_else(|| format!("Fallo al parsear el archivo: {}", file_path))?;
        let root_node = tree.root_node();

        let mut components = Vec::new();
        let mut cursor = root_node.walk();

        for child in root_node.children(&mut cursor) {
            if child.kind() == "function_definition" {
                if let Some(name) = Self::get_function_name(&child, &source) {
                    if name
                        .chars()
                        .next()
                        .map(|c| c.is_uppercase())
                        .unwrap_or(false)
                    {
                        if Self::function_returns_jsx(&child, &source) {
                            components.push(name);
                        } else {
                            info!(
                                "Función '{}' no retorna JSX. No se considera un componente.",
                                name
                            );
                        }
                    }
                } else {
                    error!(
                        "No se pudo obtener el nombre de la función en el archivo: {}",
                        file_path
                    );
                }
            }
        }

        Ok(components)
    }

    fn get_function_name(node: &Node, source: &str) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "identifier" {
                let name = child.utf8_text(source.as_bytes()).ok()?.to_string();
                return Some(name);
            }
        }
        None
    }

    fn function_returns_jsx(node: &Node, source: &str) -> bool {
        let return_node = node
            .children(&mut node.walk())
            .find(|child| child.kind() == "return_statement");
        if let Some(return_stmt) = return_node {
            let return_expr = return_stmt.child_by_field_name("expression");
            if let Some(expr) = return_expr {
                let text = expr.utf8_text(source.as_bytes()).unwrap_or("");
                return text.trim_start().starts_with('<');
            }
        }
        false
    }

    pub async fn detect_components_in_directory(
        &mut self,
        components_dir: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut components = Vec::new();
        let components_path = Path::new(components_dir);

        if !components_path.exists() || !components_path.is_dir() {
            error!(
                "Directorio de componentes no encontrado: {:?}",
                components_path
            );
            return Err(format!(
                "Directorio de componentes no encontrado: {:?}",
                components_path
            )
            .into());
        }

        let mut entries = fs::read_dir(components_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pyx") {
                match self.detect_components_in_file(path.to_str().unwrap()).await {
                    Ok(mut detected) => components.append(&mut detected),
                    Err(e) => error!("Error al parsear {:?}: {}", path, e),
                }
            }
        }

        info!("Componentes detectados: {:?}", components);
        Ok(components)
    }
}
