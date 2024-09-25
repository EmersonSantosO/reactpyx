// core_reactpyx/src/component_parser.rs

use log::{error, info};
use std::error::Error;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, TreeCursor};
use tree_sitter_python::language as python_language;

/// Estructura para manejar la detección de componentes.
pub struct ComponentParser {
    parser: Parser,
}

impl ComponentParser {
    /// Crea una nueva instancia del parser de componentes.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut parser = Parser::new();
        parser.set_language(python_language())?; // Usando el lenguaje Python con Tree-sitter
        Ok(Self { parser })
    }

    /// Detecta componentes en un archivo `.pyx`.
    ///
    /// Un componente se define como una función que comienza con una letra mayúscula
    /// y retorna una estructura similar a JSX.
    pub fn detect_components_in_file(
        &mut self,
        file_path: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let source = fs::read_to_string(file_path)?;
        let tree = self
            .parser
            .parse(&source, None)
            .ok_or_else(|| format!("Fallo al parsear el archivo: {}", file_path))?;
        let root_node = tree.root_node();

        let mut components = Vec::new();
        let mut cursor = root_node.walk();

        // Recorremos el árbol de sintaxis en busca de definiciones de funciones
        for child in root_node.children(&mut cursor) {
            if child.kind() == "function_definition" {
                if let Some(name) = Self::get_function_name(&child, &source) {
                    // Consideramos que los componentes comienzan con una letra mayúscula
                    if name
                        .chars()
                        .next()
                        .map(|c| c.is_uppercase())
                        .unwrap_or(false)
                    {
                        // Opcional: Verificar si la función retorna JSX
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

    /// Obtiene el nombre de una función dada su nodo en el AST.
    fn get_function_name(node: &Node, source: &str) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "identifier" {
                let name = child.utf8_text(source.as_bytes()).ok()?.to_string();
                return Some(name);
            }
        }
        None
    }

    /// Verifica si una función retorna JSX.
    fn function_returns_jsx(node: &Node, source: &str) -> bool {
        // Implementa la lógica para verificar si la función retorna JSX.
        // Esto puede involucrar analizar el cuerpo de la función en busca de retornos de JSX.
        // Como Tree-sitter para Python no tiene conocimiento específico de JSX,
        // podrías buscar patrones específicos en el código retornado.

        // Ejemplo simplificado: verificar si hay 'return <'
        let return_node = node
            .children(&mut node.walk())
            .find(|child| child.kind() == "return_statement");
        if let Some(return_stmt) = return_node {
            let return_expr = return_stmt.child_by_field_name("argument");
            if let Some(expr) = return_expr {
                let text = expr.utf8_text(source.as_bytes()).unwrap_or("");
                return text.trim_start().starts_with("<");
            }
        }
        false
    }

    /// Detecta componentes en todo el directorio `src/components`.
    pub fn detect_components_in_directory(
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

        // Recorremos todos los archivos en el directorio `src/components`
        for entry in fs::read_dir(components_path)? {
            let entry = entry?;
            let path = entry.path();

            // Verificamos si es un archivo `.pyx`
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pyx") {
                match self.detect_components_in_file(path.to_str().unwrap()) {
                    Ok(mut detected) => components.append(&mut detected),
                    Err(e) => error!("Error al parsear {:?}: {}", path, e),
                }
            }
        }

        info!("Componentes detectados: {:?}", components);
        Ok(components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_detect_components_in_file() {
        let mut parser = ComponentParser::new().unwrap();
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_component.pyx");
        let mut file = File::create(&file_path).unwrap();

        let code = r#"
def not_a_component():
    pass

def MyComponent():
    return <div>Hello, World!</div>

def AnotherComponent():
    return some_function()
"#;

        file.write_all(code.as_bytes()).unwrap();

        let components = parser
            .detect_components_in_file(file_path.to_str().unwrap())
            .unwrap();

        assert_eq!(components, vec!["MyComponent"]);
    }

    #[test]
    fn test_detect_components_in_directory() {
        let mut parser = ComponentParser::new().unwrap();
        let dir = tempdir().unwrap();
        let components_dir = dir.path().join("components");
        fs::create_dir(&components_dir).unwrap();

        let file1_path = components_dir.join("component1.pyx");
        let mut file1 = File::create(&file1_path).unwrap();
        let code1 = r#"
def FirstComponent():
    return <span>First</span>
"#;
        file1.write_all(code1.as_bytes()).unwrap();

        let file2_path = components_dir.join("component2.pyx");
        let mut file2 = File::create(&file2_path).unwrap();
        let code2 = r#"
def second_component():
    return <div>Second</div>

def SecondComponent():
    return <div>Second</div>
"#;
        file2.write_all(code2.as_bytes()).unwrap();

        let components = parser
            .detect_components_in_directory(components_dir.to_str().unwrap())
            .unwrap();

        assert_eq!(components, vec!["FirstComponent", "SecondComponent"]);
    }
}
