use crate::css_minifier::minify_css_code;
use crate::js_minifier::minify_js_code;
use anyhow::{Context, Result};
use futures::StreamExt;
use log::{error, info};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::fs;

/// Compiles all `.pyx` files in the project asynchronously and in parallel
pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
    target_env: &str, // "node" or "python"
) -> Result<(Vec<String>, Vec<(String, String)>)> {
    let src_dir = Path::new(project_root).join("src");

    // Recursively find all .pyx files
    let mut pyx_files = Vec::new();
    let mut dirs_to_visit = vec![src_dir];

    while let Some(dir) = dirs_to_visit.pop() {
        if let Ok(mut entries) = fs::read_dir(&dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    dirs_to_visit.push(path);
                } else if let Some(ext) = path.extension() {
                    if ext == "pyx" {
                        pyx_files.push(path);
                    }
                }
            }
        }
    }

    info!("Found {} .pyx files to compile", pyx_files.len());

    let compiled_files = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::new()));

    let concurrency_level = 8;

    // Process files in parallel
    futures::stream::iter(pyx_files)
        .for_each_concurrent(Some(concurrency_level), |file_path| {
            let compiled_files = Arc::clone(&compiled_files);
            let errors = Arc::clone(&errors);
            let project_root = project_root.to_string();
            let config_path = config_path.to_string();
            let target_env = target_env.to_string();

            async move {
                match compile_pyx_file_to_python(&file_path, &config_path, &target_env).await {
                    Ok((python_code, css_code, js_code)) => {
                        let mut compiled_files = compiled_files.lock().unwrap();
                        compiled_files.push(file_path.to_string_lossy().to_string());
                        info!("Successfully compiled: {:?}", file_path);

                        // Write transformed code to appropriate build directories
                        if let Err(e) = write_transformed_files(
                            &project_root,
                            &file_path,
                            &python_code,
                            &css_code,
                            &js_code,
                        )
                        .await
                        {
                            error!("Error writing transformed files for {:?}: {}", file_path, e);
                            let mut errors = errors.lock().unwrap();
                            errors.push((file_path.to_string_lossy().to_string(), e.to_string()));
                        }
                    }
                    Err(e) => {
                        let mut errors = errors.lock().unwrap();
                        errors.push((file_path.to_string_lossy().to_string(), e.to_string()));
                        error!("Error compiling {:?}: {}", file_path, e);
                    }
                }
            }
        })
        .await;

    // Ensure __init__.py exists in build/components
    let components_out_dir = Path::new(project_root).join("build").join("components");
    if components_out_dir.exists() {
        let init_path = components_out_dir.join("__init__.py");
        if !init_path.exists() {
            let _ = fs::write(init_path, "").await;
        }
    }

    Ok((
        Arc::try_unwrap(compiled_files)
            .unwrap()
            .into_inner()
            .unwrap(),
        Arc::try_unwrap(errors).unwrap().into_inner().unwrap(),
    ))
}

/// Writes Python, CSS and JS transformed files to build directories
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
        .context("Error creating output directory")?;

    fs::write(&output_path, python_code)
        .await
        .context("Error writing transformed Python code")?;

    // Minify and write CSS
    let css_output_path = Path::new(project_root).join("build").join("styles.css");
    let minified_css = minify_css_code(css_code).context("CSS minification failed")?;
    fs::write(css_output_path, minified_css)
        .await
        .context("Error writing minified CSS")?;

    // Minify and write JS
    let js_output_path = Path::new(project_root).join("build").join("bundle.js");
    let minified_js = minify_js_code(js_code).context("JS minification failed")?;
    fs::write(js_output_path, minified_js)
        .await
        .context("Error writing minified JS")?;

    Ok(())
}

/// Compiles a `.pyx` file to Python, CSS and JavaScript
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
        .with_context(|| format!("Error reading file: {:?}", file_path))?;

    if source_code.trim().is_empty() {
        return Err(anyhow::anyhow!("Empty source file: {:?}", file_path));
    }

    // Transform `.pyx` code to Python
    let python_code: String = transform_pyx_to_python(&source_code).await?.to_string();

    // Transform Python styles to CSS and logic to JavaScript
    let (css_code, js_code) = transform_styles_and_js(&python_code, target_env)?;

    Ok((python_code, css_code, js_code))
}

/// Transform Python styles and logic to CSS and JavaScript
fn transform_styles_and_js(python_code: &str, target_env: &str) -> Result<(String, String)> {
    // Extract CSS from <style> tags in the code
    // This is a basic implementation that looks for strings containing <style>...</style>
    // In a real implementation, we would parse the Python AST to find these strings.

    let mut css_code = String::new();
    // let mut cleaned_python = python_code.to_string();

    let style_regex = regex::Regex::new(r"(?s)<style>(.*?)</style>").unwrap();

    // Collect all styles
    for cap in style_regex.captures_iter(python_code) {
        if let Some(style_content) = cap.get(1) {
            css_code.push_str(style_content.as_str());
            css_code.push('\n');
        }
    }

    // Remove style tags from python code (optional, but cleaner)
    // For now we keep them or replace them with empty strings to avoid breaking line numbers too much
    // cleaned_python = style_regex.replace_all(python_code, "").to_string();

    // Transform styles and animations to JS
    let js_code = match target_env {
        "node" => format!(
            "// JS logic generated for Node.js environment\nconsole.log('Hydration not implemented yet');",
        ),
        "python" => format!(
            "// JS logic generated for Python environment\nconsole.log('Hydration not implemented yet');",
        ),
        _ => unreachable!(),
    };

    Ok((css_code, js_code))
}

/// Transforms `.pyx` code to Python
pub async fn transform_pyx_to_python(pyx_code: &str) -> Result<String> {
    // Process the transformation from `.pyx` to Python
    // We use the JSX transformer instead of syn/prettyplease which are for Rust

    let pyx_code_cloned = pyx_code.to_string();
    let python_code = tokio::task::spawn_blocking(move || {
        crate::jsx_transformer::parse_jsx(&pyx_code_cloned)
            .map_err(|e| anyhow::anyhow!("JSX Transformation error: {}", e))
    })
    .await??;

    Ok(python_code)
}

/// Updates the application by recompiling components and applying necessary changes.
pub async fn update_application(
    module_name: &str,
    _code: &str,
    entry_function: &str,
    project_root: String,
) -> Result<()> {
    // Example: Perform recompilation or update application state
    info!("Updating application for module: {}", module_name);

    // Optionally, call compile_all_pyx to recompile all components
    compile_all_pyx(&project_root, "config.json", "python").await?;

    // Example log for successful update
    info!(
        "Application successfully updated for module: {}, entry function: {}",
        module_name, entry_function
    );

    Ok(())
}
