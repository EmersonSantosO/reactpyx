use crate::compiler;
use anyhow::{Context, Result};
use colored::Colorize;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

pub async fn run_server() -> Result<()> {
    println!(
        "{} {}",
        "Running development server...".yellow(),
        "http://localhost:8000".blue()
    );

    // Check if uvicorn is available
    let uvicorn_check = Command::new("uvicorn")
        .arg("--version")
        .output()
        .map(|output| output.status.success());

    if uvicorn_check.is_err() || !uvicorn_check.unwrap() {
        println!(
            "{} {}",
            "Error:".red(),
            "Uvicorn is not installed or not available in PATH".red()
        );
        println!(
            "{} {}",
            "Suggestion:".yellow(),
            "Install uvicorn with 'pip install uvicorn'".yellow()
        );
        return Err(anyhow::anyhow!("Uvicorn is not available"));
    }

    // Process initial CSS files before starting the server
    println!("{}", "Processing CSS files...".blue());
    process_css_files().await?;

    // Start FastAPI server in a separate process
    let mut child = Command::new("uvicorn")
        .arg("main:app")
        .arg("--reload")
        .spawn()?;

    println!("{} {}", "✓".green(), "Server started successfully");
    println!(
        "{} {}",
        "Watching for changes in".blue(),
        "src/**/*.{pyx,css}".bright_blue()
    );

    // Watch for changes in `src` folder files
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Duration::from_secs(1))?;

    watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;

    // Handle change events
    for res in rx {
        match res {
            Ok(event) => {
                if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            let ext_str = ext.to_string_lossy().to_lowercase();
                            match ext_str.as_str() {
                                "pyx" => {
                                    handle_pyx_file_change(&path).await?;
                                }
                                "css" => {
                                    handle_css_file_change(&path).await?;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(e) => println!("{} {:?}", "Error watching:".red(), e),
        }
    }

    // Wait for server process to terminate
    child.wait()?;

    Ok(())
}

async fn handle_pyx_file_change(path: &Path) -> Result<()> {
    println!("{} {}", "PyX file changed:".green(), path.display());

    // Recompile modified file
    let project_root = std::env::current_dir()?.to_string_lossy().to_string();
    let file_path = path.to_string_lossy().to_string();

    println!("{} {}", "Recompiling".yellow(), file_path);

    match crate::compiler::compile_pyx_file_to_python(path, "config.json", "python").await {
        Ok(_) => println!("{} {}", "✓".green(), "Compilation successful"),
        Err(e) => println!("{} {}: {}", "✗".red(), "Compilation error".red(), e),
    }

    // Process CSS after PyX changes in case there are <style> tags
    process_css_files().await?;

    Ok(())
}

async fn handle_css_file_change(path: &Path) -> Result<()> {
    println!("{} {}", "CSS file changed:".green(), path.display());
    process_css_files().await?;
    Ok(())
}

async fn process_css_files() -> Result<()> {
    // Create the static directory if it doesn't exist
    let static_dir = Path::new("public/static");
    if !static_dir.exists() {
        std::fs::create_dir_all(static_dir)?;
    }

    // Collect all CSS files
    let mut all_css = String::new();

    // First add main.css if it exists
    let main_css = Path::new("src/styles/main.css");
    if main_css.exists() {
        all_css.push_str(&std::fs::read_to_string(main_css)?);
        all_css.push_str("\n\n");
    }

    // Add framework CSS files if they exist
    let tailwind_css = Path::new("public/tailwind-cdn.html");
    if tailwind_css.exists() {
        all_css.push_str("/* Tailwind CSS integration */\n");
        all_css.push_str("/* See public/tailwind-cdn.html for CDN link */\n\n");
    }

    let bootstrap_css = Path::new("public/bootstrap-cdn.html");
    if bootstrap_css.exists() {
        all_css.push_str("/* Bootstrap CSS integration */\n");
        all_css.push_str("/* See public/bootstrap-cdn.html for CDN link */\n\n");
    }

    // Add all other CSS files in src/styles
    let styles_dir = Path::new("src/styles");
    if styles_dir.exists() {
        for entry in std::fs::read_dir(styles_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "css") {
                // Skip main.css since we already added it
                if path.file_name().unwrap() != "main.css" {
                    all_css.push_str(&format!("/* {} */\n", path.display()));
                    all_css.push_str(&std::fs::read_to_string(&path)?);
                    all_css.push_str("\n\n");
                }
            }
        }
    }

    // Write the combined CSS to the static directory
    let output_path = static_dir.join("styles.css");
    std::fs::write(&output_path, all_css)?;
    println!("{} {}", "✓".green(), "CSS processed and combined");

    Ok(())
}
