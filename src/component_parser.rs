// core_reactpyx/src/component_parser.rs

use log::{error, info};
use std::error::Error;
use std::fs;
use std::path::Path;

use tree_sitter::{Node, Parser};
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
            .ok_or("Fallo al parsear el archivo.")?;
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
                        components.push(name);
                    }
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
            return Err("Directorio de componentes no encontrado.".into());
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
