// core_reactpyx/src/compiler.rs

use crate::component_parser::ComponentParser;
use anyhow::{Context, Result};
use log::{error, info};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command as TokioCommand;

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
        let file_path = components_dir.join(format!("{}.pyx", component));

        if file_path.exists() {
            match compile_pyx_file_to_python(&file_path).await {
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

pub async fn compile_pyx_file_to_python(file_path: &Path) -> Result<String> {
    let source_code = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error al leer el archivo {:?}", file_path))?;

    // Implementar la lógica de transformación de PyX a Python aquí.

    let python_code = source_code; // Placeholder

    Ok(python_code)
}

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
    // Implementar la transformación de JSX a JS aquí.

    let transformed_code = jsx_code
        .replace("<", "React.createElement('")
        .replace("/>", "')")
        .replace(">", "', null, ")
        .replace("</", "");

    Ok(transformed_code)
}

pub fn watch_for_changes(project_root: &str, config_path: &str) -> Result<()> {
    // Implementación de observador de archivos.
    // Se puede utilizar una biblioteca como `notify` con funciones asíncronas.
    Ok(())
}

pub async fn update_application(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: PathBuf,
) -> Result<()> {
    let output_file = project_root.join(format!("{}.py", module_name));
    fs::write(&output_file, code)
        .await
        .with_context(|| format!("Error al escribir el archivo {:?}", output_file))?;

    info!("Aplicación actualizada en {:?}", output_file);

    Ok(())
}
