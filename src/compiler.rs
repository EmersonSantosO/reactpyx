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

/// Compile all `.pyx` files in the project asynchronously and in parallel
pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
    target_env: &str, // "node" or "python"
) -> Result<(Vec<String>, Vec<(String, String)>)> {
    let components_dir = Path::new(project_root).join("src").join("components");

    let components = detect_components_in_directory(&components_dir).await?;
    info!("Starting compilation with components: {:?}", components);

    let compiled_files = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::new()));

    let concurrency_level = 8; // Concurrency level to control parallel processing

    // Process files in parallel
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
                                info!("Successfully compiled: {:?}", file_path);

                                // Write the transformed code to appropriate build directories
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
                                        "Error writing transformed files for {:?}: {}",
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
                                error!("Error compiling {:?}: {}", file_path, e);
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

/// Write transformed Python, CSS, and JS files to build directories
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
        .context("Failed to create output directory")?;

    fs::write(&output_path, python_code)
        .await
        .context("Failed to write transformed Python code")?;

    // Minify and write CSS
    let css_output_path = Path::new(project_root).join("build").join("styles.css");
    let minified_css = minify_css_code(css_code).context("CSS minification failed")?;
    fs::write(css_output_path, minified_css)
        .await
        .context("Failed to write minified CSS")?;

    // Minify and write JS
    let js_output_path = Path::new(project_root).join("build").join("bundle.js");
    let minified_js = minify_js_code(js_code).context("JS minification failed")?;
    fs::write(js_output_path, minified_js)
        .await
        .context("Failed to write minified JS")?;

    Ok(())
}

/// Compile a `.pyx` file to Python, CSS, and JavaScript
pub async fn compile_pyx_file_to_python(
    file_path: &Path,
    _config_path: &str,
    target_env: &str,
) -> Result<(String, String, String)> {
    if !["node", "python"].contains(&target_env) {
        return Err(anyhow::anyhow!(
            "Unsupported target environment: {}",
            target_env
        ));
    }

    let source_code = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error reading the file: {:?}", file_path))?;

    if source_code.trim().is_empty() {
        return Err(anyhow::anyhow!("Source file is empty: {:?}", file_path));
    }

    // Transform `.pyx` code to Python
    let python_code: String = transform_pyx_to_python(&source_code).await?.to_string();

    // Transform Python styles to CSS and logic to JavaScript
    let (css_code, js_code) = transform_styles_and_js(&python_code, target_env)?;

    Ok((python_code, css_code, js_code))
}

/// Transform styles and logic from Python to CSS and JavaScript
fn transform_styles_and_js(python_code: &str, target_env: &str) -> Result<(String, String)> {
    // Placeholder for the logic to transform styles to CSS
    let css_code = format!(
        "/* CSS generated from styles in Python */\n{}",
        python_code // Replace with actual CSS transformation logic
    );

    // Transform styles and animations to JS
    let js_code = match target_env {
        "node" => format!(
            "// JS logic generated for Node.js environment\n{}",
            python_code // Replace with Node.js-specific transformation logic
        ),
        "python" => format!(
            "// JS logic generated for Python environment\n{}",
            python_code // Replace with Python-specific transformation logic
        ),
        _ => unreachable!(),
    };

    Ok((css_code, js_code))
}

/// Detect components in the components directory
async fn detect_components_in_directory(components_dir: &Path) -> Result<Vec<String>> {
    let mut components = Vec::new();
    let mut dir_entries = ReadDirStream::new(
        fs::read_dir(components_dir)
            .await
            .with_context(|| format!("Error reading the directory {:?}", components_dir))?,
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
        warn!("No components found in directory: {:?}", components_dir);
    }

    Ok(components)
}

/// Detect components within a file
async fn detect_components_in_file(file_path: &Path) -> Result<Vec<String>> {
    let source = fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Error reading the file {:?}", file_path))?;

    if !source.is_ascii() {
        return Err(anyhow::anyhow!(
            "File contains non-ASCII characters: {:?}",
            file_path
        ));
    }

    let tree = syn::parse_file(&source)
        .with_context(|| format!("Error parsing the file: {:?}", file_path))?;

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
                info!("Detected component: {}", name);
            }
        }
    }

    Ok(components)
}

/// Transform `.pyx` code to Python
#[cached(
    type = "cached::TimedCache<String, String>",
    create = "{ cached::TimedCache::with_lifespan_and_capacity(60, 1000) }",
    convert = r#"{ blake3::hash(pyx_code.as_bytes()).to_hex().to_string() }"#
)]

pub async fn transform_pyx_to_python(pyx_code: &str) -> Result<String> {
    // Process transformation from `.pyx` to Python
    let pyx_code_cloned = pyx_code.to_string();
    let python_code = spawn_blocking(move || {
        let syntax_tree =
            syn::parse_file(&pyx_code_cloned).with_context(|| "Error parsing PyX code")?;
        Ok::<String, anyhow::Error>(prettyplease::unparse(&syntax_tree))
    })
    .await??;

    Ok(python_code)
}

/// Updates the application by recompiling components and applying any necessary changes.
pub async fn update_application(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: String,
) -> Result<()> {
    // Example: Perform recompilation or refresh application state
    info!("Updating application for module: {}", module_name);

    // Optionally, call compile_all_pyx to recompile all components
    compile_all_pyx(&project_root, "config.json", "python").await?;

    // Example logging for successful update
    info!(
        "Application updated successfully for module: {}, entry function: {}",
        module_name, entry_function
    );

    Ok(())
}
