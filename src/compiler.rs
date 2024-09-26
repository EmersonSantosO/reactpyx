// core_reactpyx/src/compiler.rs

use crate::component_parser::ComponentParser;
use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use inflector::cases::snakecase::to_snake_case;
use log::{error, info};
use notify::{watcher, RecursiveMode, Watcher};
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use syn::{parse_file, Item};
use tree_sitter::{Node, Parser};
use tree_sitter_python::language as python_language;
/// Compila todos los archivos `.pyx` en el proyecto.
/// Retorna una tupla con los archivos compilados y una lista de errores.
pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
) -> Result<(Vec<String>, Vec<(String, String)>)> {
    let components_dir = Path::new(project_root).join("src").join("components");
    let mut parser = ComponentParser::new()?;
    let components = parser
        .detect_components_in_directory(components_dir.to_str().unwrap())
        .await?;

    info!(
        "Iniciando la compilación con los componentes: {:?}",
        components
    );

    let mut compiled_files = Vec::new();
    let mut errors = Vec::new();

    for component in components {
        let file_path = Path::new(project_root)
            .join("src")
            .join("components")
            .join(format!("{}.pyx", component));

        if file_path.exists() {
            match compile_pyx_file_to_python(&file_path) {
                Ok(python_code) => {
                    compiled_files.push(file_path.to_str().unwrap().to_string());
                    info!("Compilado exitosamente: {:?}", file_path);
                    let output_path = Path::new(project_root)
                        .join("build")
                        .join(format!("{}.py", component));
                    fs::write(output_path, python_code)?;
                }
                Err(e) => {
                    errors.push((file_path.to_str().unwrap().to_string(), e.to_string()));
                    error!("Error compilando {:?}: {}", file_path, e);
                }
            }
        } else {
            errors.push((
                file_path.to_str().unwrap().to_string(),
                "Archivo no encontrado.".to_string(),
            ));
            error!("Error compilando {:?}: Archivo no encontrado.", file_path);
        }
    }

    Ok((compiled_files, errors))
}
pub fn compile_pyx_file_to_python(file_path: &Path) -> Result<String> {
    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("Error al leer el archivo {:?}", file_path))?;

    let syntax_tree =
        parse_file(&source_code).with_context(|| "Error al parsear el código fuente")?;

    let mut transformed_items = Vec::new();

    for item in syntax_tree.items {
        match item {
            Item::Fn(mut func) => {
                // Realiza las transformaciones necesarias en la función
                // Por ejemplo, ajusta los atributos o modifica el cuerpo según sea necesario
                let transformed_code = quote! { #func };
                transformed_items.push(transformed_code.to_string());
            }
            _ => {
                let item_code = quote! { #item };
                transformed_items.push(item_code.to_string());
            }
        }
    }

    Ok(transformed_items.join("\n"))
}

/// Compila PyX a JavaScript y guarda el resultado en `output_dir`.
pub async fn compile_pyx_to_js(
    entry_file: &str,
    config_path: &str,
    output_dir: &str,
) -> Result<()> {
    let entry_path = Path::new(entry_file);
    if !entry_path.exists() {
        return Err(anyhow::anyhow!(
            "El archivo de entrada {} no existe",
            entry_file
        ));
    }

    let source_code = fs::read_to_string(entry_path)
        .with_context(|| format!("Error al leer el archivo {:?}", entry_file))?;
    let transformed_js = transform_jsx_to_js(&source_code)?;

    let output_file = Path::new(output_dir).join("output.js");
    fs::write(output_file, transformed_js)
        .with_context(|| format!("Error al escribir el archivo {:?}", output_file))?;

    info!("Archivo JavaScript generado: {:?}", output_file);
    Ok(())
}

/// Transforma código JSX a JavaScript.
fn transform_jsx_to_js(jsx_code: &str) -> Result<String> {
    // Implementa la transformación de JSX a JS aquí.
    // Esta es una implementación simplificada.
    let transformed_code = jsx_code
        .replace("<", "React.createElement(")
        .replace("/>", ")")
        .replace(">", ", null, ")
        .replace("</", "");

    Ok(transformed_code)
}

/// Monitorea los archivos en el directorio `src/components` para recompilarlos automáticamente cuando cambien.
pub fn watch_for_changes(project_root: &str, config_path: &str) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = notify::watcher(tx, Duration::from_secs(2))
        .with_context(|| "Error al crear el watcher de archivos")?;
    let components_dir = Path::new(project_root).join("src").join("components");

    watcher
        .watch(&components_dir, notify::RecursiveMode::Recursive)
        .with_context(|| format!("Error al observar el directorio {:?}", components_dir))?;

    loop {
        match rx.recv() {
            Ok(event) => {
                info!("Cambio detectado: {:?}", event);
                let _ = compile_all_pyx(project_root, config_path);
            }
            Err(e) => error!("Error al recibir evento de cambio: {:?}", e),
        }
    }
}
