use crate::precompiler::JSXPrecompiler;
use anyhow::{Context, Result};
use log::{error, info};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin; // Importar Pin correctamente
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;
use tree_sitter::{Node, Parser};
use tree_sitter_python::language as python_language;

pub struct ComponentDetector {
    project_root: PathBuf,
    parser: Parser,
    precompiler: JSXPrecompiler,
}

impl ComponentDetector {
    pub fn new(project_root: &str) -> Result<Self> {
        let mut parser = Parser::new();
        parser
            .set_language(python_language())
            .context("Error al configurar el lenguaje Python para Tree-sitter")?;
        let precompiler = JSXPrecompiler::new();

        Ok(Self {
            project_root: PathBuf::from(project_root),
            parser,
            precompiler,
        })
    }

    /// Detecta componentes de forma asíncrona.
    pub async fn detect_components(&mut self) -> Result<Vec<String>> {
        info!("Iniciando la detección de componentes...");

        let components_dir = self.project_root.join("src").join("components");
        if !components_dir.exists() {
            error!(
                "Directorio de componentes no encontrado: {:?}",
                components_dir
            );
            return Err(anyhow::anyhow!("Directorio de componentes no encontrado."));
        }

        let components = self.detect_components_in_directory(&components_dir).await?;

        info!("Componentes detectados: {:?}", components);

        Ok(components)
    }

    fn detect_components_in_directory_boxed(
        &mut self,
        components_dir: &Path,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>>> + '_>> {
        Box::pin(self.detect_components_in_directory(components_dir))
    }

    async fn detect_components_in_directory(
        &mut self,
        components_dir: &Path,
    ) -> Result<Vec<String>> {
        let mut components = Vec::new();

        let mut dir_entries = ReadDirStream::new(
            fs::read_dir(components_dir)
                .await
                .with_context(|| format!("Error al leer el directorio {:?}", components_dir))?,
        );

        while let Some(entry) = dir_entries.next().await {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                match path.extension().and_then(|s| s.to_str()) {
                    Some("pyx") => {
                        let detected_components = self.detect_components_in_file(&path).await?;
                        components.extend(detected_components);
                    }
                    Some("jsx") => {
                        info!("Precompilando archivo JSX: {:?}", path);
                        self.precompiler
                            .precompile_jsx(path.to_str().unwrap())
                            .with_context(|| {
                                format!("Error en la precompilación JSX: {:?}", path)
                            })?;
                        info!("Precompilación JSX exitosa: {:?}", path);
                    }
                    _ => {}
                }
            } else if path.is_dir() {
                // Recursivamente detecta componentes en subdirectorios usando la función boxeada
                let sub_components = self.detect_components_in_directory_boxed(&path).await?;
                components.extend(sub_components);
            }
        }

        Ok(components)
    }

    async fn detect_components_in_file(&mut self, file_path: &Path) -> Result<Vec<String>> {
        let source = fs::read_to_string(file_path)
            .await
            .with_context(|| format!("Error al leer el archivo {:?}", file_path))?;
        let tree = self
            .parser
            .parse(&source, None)
            .ok_or_else(|| anyhow::anyhow!("Fallo al parsear el archivo: {:?}", file_path))?;
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
                        // Verificar si la función retorna JSX
                        if Self::function_returns_jsx(&child, &source) {
                            components.push(name.clone());
                            info!("Componente detectado: {}", name);
                        }
                    }
                } else {
                    error!(
                        "No se pudo obtener el nombre de la función en el archivo: {:?}",
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
                return Some(child.utf8_text(source.as_bytes()).ok()?.to_string());
            }
        }
        None
    }

    fn function_returns_jsx(node: &Node, source: &str) -> bool {
        if let Some(expr) = node.child_by_field_name("expression") {
            if let Ok(text) = expr.utf8_text(source.as_bytes()) {
                return text.trim().starts_with('<');
            }
        }
        false
    }
}
