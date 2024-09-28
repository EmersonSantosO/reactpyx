use crate::js_minifier::minify_js_code;
use anyhow::{Context, Error, Result};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize};
use log::{error, info};
use markup5ever_rcdom::{RcDom, SerializableHandle};
use md5;
use once_cell::sync::Lazy;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use syn::{self, File};
use tokio::fs;
use tokio::sync::RwLock;
use tokio::task::spawn_blocking;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;

// Usar RwLock para manejar concurrencia en el caché de transformación
use ttl_cache::TtlCache;

static TRANSFORM_CACHE: Lazy<Arc<RwLock<TtlCache<String, String>>>> =
    Lazy::new(|| Arc::new(RwLock::new(TtlCache::new(1000, Duration::from_secs(60)))));

/// Compila todos los archivos `.pyx` en el proyecto
pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
) -> Result<(Vec<String>, Vec<(String, String)>)> {
    let components_dir = Path::new(project_root).join("src").join("components");

    // Detectar componentes en el directorio de forma asíncrona
    let components = detect_components_in_directory(&components_dir).await?;

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
                    compiled_files.push(file_path.to_string_lossy().to_string());
                    info!("Compilado exitosamente: {:?}", file_path);

                    let output_path = Path::new(project_root)
                        .join("build")
                        .join("components")
                        .join(format!("{}.py", component));
                    fs::create_dir_all(output_path.parent().unwrap()).await?;
                    fs::write(&output_path, python_code).await?;
                }
                Err(e) => {
                    errors.push((file_path.to_string_lossy().to_string(), e.to_string()));
                    error!("Error compilando {:?}: {}", file_path, e);
                }
            }
        } else {
            errors.push((
                file_path.to_string_lossy().to_string(),
                "Archivo no encontrado.".to_string(),
            ));
            error!("Error compilando {:?}: Archivo no encontrado.", file_path);
        }
    }

    Ok((compiled_files, errors))
}

/// Detección de componentes en un directorio de forma recursiva
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
            // Boxear el futuro para evitar un tamaño infinito
            let sub_components = Box::pin(detect_components_in_directory(&path)).await?;
            components.extend(sub_components);
        }
    }

    Ok(components)
}

/// Detección de componentes dentro de un archivo `.pyx`
async fn detect_components_in_file(file_path: &Path) -> Result<Vec<String>> {
    let source = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error al leer el archivo {:?}", file_path))?;
    let tree = syn::parse_file(&source)
        .map_err(|_| anyhow::anyhow!("Fallo al parsear el archivo: {:?}", file_path))?;

    let mut components = Vec::new();

    // Análisis sintáctico para extraer componentes
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

/// Compilar un archivo `.pyx` a código Python
pub async fn compile_pyx_file_to_python(file_path: &Path, _config_path: &str) -> Result<String> {
    let source_code = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error al leer el archivo: {:?}", file_path))?;

    let python_code = transform_pyx_to_python(&source_code).await?;

    Ok(python_code)
}

/// Transformación de código PyX a Python
async fn transform_pyx_to_python(pyx_code: &str) -> Result<String> {
    let cache_key = format!("{:x}", md5::compute(pyx_code));
    let cache = TRANSFORM_CACHE.clone();

    // Leer caché de forma segura usando `RwLock`
    let cached = {
        let read_guard = cache.read().await;
        read_guard.get(&cache_key).cloned()
    };

    if let Some(cached_code) = cached {
        return Ok(cached_code);
    }

    // Clona pyx_code para evitar que escape su referencia
    let pyx_code_cloned = pyx_code.to_string();
    // Ejecutar el código que usa syn::File en un hilo bloqueado
    let python_code = spawn_blocking(move || {
        // Parsear y transformar el AST del código PyX
        let syntax_tree: File =
            syn::parse_file(&pyx_code_cloned).with_context(|| "Error al parsear el código PyX")?;
        Ok::<String, Error>(prettyplease::unparse(&syntax_tree))
    })
    .await??; // Propaga los errores

    // Escribir el resultado al caché
    let mut write_guard = cache.write().await;
    write_guard.insert(cache_key, python_code.clone());

    Ok(python_code)
}

/// Compilar un archivo `.pyx` a JavaScript
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

    // Minificar el código JavaScript
    let (minified_js, _sourcemap) = minify_js_code(&transformed_js, true)?;

    let output_file = Path::new(output_dir).join("app.js");
    fs::create_dir_all(output_dir).await?;
    fs::write(&output_file, minified_js)
        .await
        .with_context(|| format!("Error al escribir el archivo {:?}", output_file))?;

    info!("Archivo JavaScript generado: {:?}", output_file);
    Ok(())
}

/// Transformación de código JSX a JavaScript (más robusta)
fn transform_jsx_to_js(jsx_code: &str) -> Result<String> {
    // Implementa una lógica de transformación más robusta para manejar la sintaxis completa de JSX.
    // Puedes usar una biblioteca de Rust como `syn` para analizar el código JSX y generar el código JavaScript equivalente.
    // ...

    // Ejemplo básico (reemplazar con lógica más robusta)
    let transformed_code = jsx_code
        .replace("<", "React.createElement('")
        .replace("/>", "')")
        .replace(">", "', null, ")
        .replace("</", "");

    Ok(transformed_code)
}

/// Actualizar una aplicación existente con el nuevo código
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

/// Saneamiento de HTML con html5ever
pub fn sanitize_html(html: &str) -> Result<String> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .map_err(|e| anyhow::anyhow!("Error al parsear HTML: {:?}", e))?;

    // Usa un Vec<u8> para la salida de serialize
    let mut sanitized_html = Vec::new();
    let serializable_handle = SerializableHandle::from(dom.document);
    serialize(
        &mut sanitized_html,
        &serializable_handle,
        Default::default(),
    )
    .map_err(|e| anyhow::anyhow!("Error al serializar HTML: {:?}", e))?;

    Ok(String::from_utf8(sanitized_html)?)
}
