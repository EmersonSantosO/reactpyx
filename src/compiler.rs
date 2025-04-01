use crate::css_minifier::minify_css_code;
use crate::js_minifier::minify_js_code;
use anyhow::{Context, Result};
use cached::proc_macro::cached;
use futures::StreamExt;
use log::{error, info, warn};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::fs;
use tokio_stream::wrappers::ReadDirStream;

/// Compila todos los archivos `.pyx` en el proyecto de forma asíncrona y en paralelo
pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
    target_env: &str, // "node" o "python"
) -> Result<(Vec<String>, Vec<(String, String)>)> {
    let components_dir = Path::new(project_root).join("src").join("components");

    let components = detect_components_in_directory(&components_dir).await?;
    info!("Iniciando compilación con componentes: {:?}", components);

    let compiled_files = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::new()));

    let concurrency_level = 8; // Nivel de concurrencia para controlar el procesamiento paralelo

    // Procesa archivos en paralelo
    let stream = ReadDirStream::new(fs::read_dir(&components_dir).await?);
    stream
        .for_each_concurrent(Some(concurrency_level), |entry| {
            let compiled_files = Arc::clone(&compiled_files);
            let errors = Arc::clone(&errors);
            async move {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.exists()
                        && file_path.extension().and_then(|s| s.to_str()) == Some("pyx")
                    {
                        match compile_pyx_file_to_python(&file_path, config_path, target_env).await
                        {
                            Ok((python_code, css_code, js_code)) => {
                                let mut compiled_files = compiled_files.lock().unwrap();
                                compiled_files.push(file_path.to_string_lossy().to_string());
                                info!("Compilado exitosamente: {:?}", file_path);

                                // Escribir el código transformado en los directorios de compilación apropiados
                                if let Err(e) = write_transformed_files(
                                    project_root,
                                    &file_path,
                                    &python_code,
                                    &css_code,
                                    &js_code,
                                )
                                .await
                                {
                                    error!(
                                        "Error al escribir archivos transformados para {:?}: {}",
                                        file_path, e
                                    );
                                    let mut errors = errors.lock().unwrap();
                                    errors.push((
                                        file_path.to_string_lossy().to_string(),
                                        e.to_string(),
                                    ));
                                }
                            }
                            Err(e) => {
                                let mut errors = errors.lock().unwrap();
                                errors
                                    .push((file_path.to_string_lossy().to_string(), e.to_string()));
                                error!("Error compilando {:?}: {}", file_path, e);
                            }
                        }
                    }
                }
            }
        })
        .await;

    Ok((
        Arc::try_unwrap(compiled_files)
            .unwrap()
            .into_inner()
            .unwrap(),
        Arc::try_unwrap(errors).unwrap().into_inner().unwrap(),
    ))
}

/// Escribe archivos Python, CSS y JS transformados en los directorios de compilación
async fn write_transformed_files(
    project_root: &str,
    file_path: &Path,
    python_code: &str,
    css_code: &str,
    js_code: &str,
) -> Result<()> {
    let output_path = Path::new(project_root)
        .join("build")
        .join("components")
        .join(format!(
            "{}.py",
            file_path.file_stem().unwrap().to_str().unwrap()
        ));
    fs::create_dir_all(output_path.parent().unwrap())
        .await
        .context("Error al crear directorio de salida")?;

    fs::write(&output_path, python_code)
        .await
        .context("Error al escribir código Python transformado")?;

    // Minificar y escribir CSS
    let css_output_path = Path::new(project_root).join("build").join("styles.css");
    let minified_css = minify_css_code(css_code).context("Falló la minificación CSS")?;
    fs::write(css_output_path, minified_css)
        .await
        .context("Error al escribir CSS minificado")?;

    // Minificar y escribir JS
    let js_output_path = Path::new(project_root).join("build").join("bundle.js");
    let minified_js = minify_js_code(js_code).context("Falló la minificación JS")?;
    fs::write(js_output_path, minified_js)
        .await
        .context("Error al escribir JS minificado")?;

    Ok(())
}

/// Compila un archivo `.pyx` a Python, CSS y JavaScript
pub async fn compile_pyx_file_to_python(
    file_path: &Path,
    _config_path: &str,
    target_env: &str,
) -> Result<(String, String, String)> {
    if !["node", "python"].contains(&target_env) {
        return Err(anyhow::anyhow!(
            "Entorno de destino no soportado: {}",
            target_env
        ));
    }

    let source_code = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error al leer el archivo: {:?}", file_path))?;

    if source_code.trim().is_empty() {
        return Err(anyhow::anyhow!("Archivo fuente vacío: {:?}", file_path));
    }

    // Transformar código `.pyx` a Python
    let python_code: String = transform_pyx_to_python(&source_code).await?.to_string();

    // Transformar estilos Python a CSS y lógica a JavaScript
    let (css_code, js_code) = transform_styles_and_js(&python_code, target_env)?;

    Ok((python_code, css_code, js_code))
}

/// Transformar estilos y lógica de Python a CSS y JavaScript
fn transform_styles_and_js(python_code: &str, target_env: &str) -> Result<(String, String)> {
    // Marcador de posición para la lógica de transformación de estilos a CSS
    let css_code = format!(
        "/* CSS generado desde estilos en Python */\n{}",
        python_code // Reemplazar con lógica real de transformación CSS
    );

    // Transformar estilos y animaciones a JS
    let js_code = match target_env {
        "node" => format!(
            "// Lógica JS generada para entorno Node.js\n{}",
            python_code // Reemplazar con lógica de transformación específica para Node.js
        ),
        "python" => format!(
            "// Lógica JS generada para entorno Python\n{}",
            python_code // Reemplazar con lógica de transformación específica para Python
        ),
        _ => unreachable!(),
    };

    Ok((css_code, js_code))
}

/// Detectar componentes en el directorio de componentes
async fn detect_components_in_directory(components_dir: &Path) -> Result<Vec<String>> {
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
                    let detected_components = detect_components_in_file(&path).await?;
                    components.extend(detected_components);
                }
                _ => {}
            }
        } else if path.is_dir() {
            let sub_components = detect_components_in_directory(&path).await?;
            components.extend(sub_components);
        }
    }

    if components.is_empty() {
        warn!(
            "No se encontraron componentes en el directorio: {:?}",
            components_dir
        );
    }

    Ok(components)
}

/// Detectar componentes dentro de un archivo
async fn detect_components_in_file(file_path: &Path) -> Result<Vec<String>> {
    let source = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error al leer el archivo {:?}", file_path))?;

    if !source.is_ascii() {
        return Err(anyhow::anyhow!(
            "El archivo contiene caracteres no ASCII: {:?}",
            file_path
        ));
    }

    let tree = syn::parse_file(&source)
        .with_context(|| format!("Error al analizar el archivo: {:?}", file_path))?;

    let mut components = Vec::new();
    for item in &tree.items {
        if let syn::Item::Fn(func) = item {
            let name = func.sig.ident.to_string();
            if name
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                components.push(name.clone());
                info!("Componente detectado: {}", name);
            }
        }
    }

    Ok(components)
}

/// Transformar código `.pyx` a Python
#[cached(
    type = "cached::TimedCache<String, String>",
    create = "{ cached::TimedCache::with_lifespan_and_capacity(60, 1000) }",
    convert = r#"{ blake3::hash(pyx_code.as_bytes()).to_hex().to_string() }"#
)]
pub async fn transform_pyx_to_python(pyx_code: &str) -> Result<String> {
    // Procesa la transformación de `.pyx` a Python
    let pyx_code_cloned = pyx_code.to_string();
    let python_code = spawn_blocking(move || {
        let syntax_tree =
            syn::parse_file(&pyx_code_cloned).with_context(|| "Error al analizar código PyX")?;
        Ok::<String, anyhow::Error>(prettyplease::unparse(&syntax_tree))
    })
    .await??;

    Ok(python_code)
}

/// Actualiza la aplicación recompilando componentes y aplicando los cambios necesarios.
pub async fn update_application(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: String,
) -> Result<()> {
    // Ejemplo: Realizar recompilación o actualizar estado de la aplicación
    info!("Actualizando aplicación para módulo: {}", module_name);

    // Opcionalmente, llamar a compile_all_pyx para recompilar todos los componentes
    compile_all_pyx(&project_root, "config.json", "python").await?;

    // Ejemplo de registro para actualización exitosa
    info!(
        "Aplicación actualizada exitosamente para módulo: {}, función de entrada: {}",
        module_name, entry_function
    );

    Ok(())
}
