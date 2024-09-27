// core_reactpyx/src/compiler.rs

use crate::component_parser::ComponentParser;
use anyhow::{Context, Result};
use log::{error, info};
use md5;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use syn::{self, File};
use tokio::fs;

static TRANSFORM_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
) -> Result<(Vec<String>, Vec<(String, String)>)> {
    let components_dir = Path::new(project_root).join("src").join("components");
    let mut parser = ComponentParser::new();

    // Aquí deberías implementar la lógica para detectar los componentes en el directorio
    let components = parser
        .detect_components_in_file(components_dir.to_str().unwrap()) // Suponiendo que esta función lea un archivo específico
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    info!(
        "Iniciando la compilación con los componentes: {:?}",
        components
    );

    let mut compiled_files = Vec::new();
    let mut errors = Vec::new();

    for component in components {
        let file_path = components_dir.join(format!("{}.pyx", component));

        if file_path.exists() {
            match compile_pyx_file_to_python(&file_path, config_path).await {
                Ok(python_code) => {
                    compiled_files.push(file_path.to_str().unwrap().to_string());
                    info!("Compilado exitosamente: {:?}", file_path);

                    let output_path = Path::new(project_root)
                        .join("build")
                        .join("components")
                        .join(format!("{}.py", component));
                    fs::create_dir_all(output_path.parent().unwrap()).await?;
                    fs::write(&output_path, python_code).await?;
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

pub async fn compile_pyx_file_to_python(file_path: &Path, _config_path: &str) -> Result<String> {
    let source_code = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error al leer el archivo {:?}", file_path))?;

    let python_code = transform_pyx_to_python(&source_code)?;

    Ok(python_code)
}

fn transform_pyx_to_python(pyx_code: &str) -> Result<String> {
    let cache_key = format!("{:x}", md5::compute(pyx_code));
    let mut cache = TRANSFORM_CACHE.lock().unwrap();

    if let Some(cached) = cache.get(&cache_key) {
        return Ok(cached.clone());
    }

    // Parsea el código PyX y transforma el AST
    let syntax_tree: File =
        syn::parse_file(pyx_code).with_context(|| "Error al parsear el código PyX")?;

    // Aquí, aplicarías las transformaciones necesarias al AST
    let python_code = prettyplease::unparse(&syntax_tree);

    cache.insert(cache_key, python_code.clone());

    Ok(python_code)
}

pub async fn compile_pyx_to_js(
    entry_file: &str,
    _config_path: &str,
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
        .await
        .with_context(|| format!("Error al leer el archivo {:?}", entry_file))?;
    let transformed_js = transform_jsx_to_js(&source_code)?;

    let output_file = Path::new(output_dir).join("app.js");
    fs::create_dir_all(output_dir).await?;
    fs::write(&output_file, transformed_js)
        .await
        .with_context(|| format!("Error al escribir el archivo {:?}", output_file))?;

    info!("Archivo JavaScript generado: {:?}", output_file);
    Ok(())
}

fn transform_jsx_to_js(jsx_code: &str) -> Result<String> {
    // Transformación básica del código JSX a JS
    let transformed_code = jsx_code
        .replace("<", "React.createElement('")
        .replace("/>", "')")
        .replace(">", "', null, ")
        .replace("</", "");

    Ok(transformed_code)
}

pub fn watch_for_changes(_project_root: &str, _config_path: &str) -> Result<()> {
    // Implementación para ver cambios en el proyecto
    Ok(())
}

pub async fn update_application(
    module_name: &str,
    code: &str,
    _entry_function: &str,
    project_root: PathBuf,
) -> Result<()> {
    let output_file = project_root.join(format!("{}.py", module_name));
    fs::write(&output_file, code)
        .await
        .with_context(|| format!("Error al escribir el archivo {:?}", output_file))?;

    info!("Aplicación actualizada en {:?}", output_file);

    Ok(())
}
