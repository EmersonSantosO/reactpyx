use anyhow::Result;
use colored::Colorize;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::compiler;

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

    // Start FastAPI server in a separate process
    let mut child = Command::new("uvicorn")
        .arg("main:app")
        .arg("--reload")
        .spawn()?;

    println!("{} {}", "✓".green(), "Server started successfully");
    println!(
        "{} {}",
        "Watching for changes in".blue(),
        "src/**/*.pyx".bright_blue()
    );

    // Watch for changes in `src` folder files
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        tx,
        Duration::from_secs(1),
    )?;
    
    watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;

    // Handle change events
    for res in rx {
        match res {
            Ok(event) => {
                if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            if ext == "pyx" {
                                handle_file_change(&path).await?;
                            }
                        }
                    }
                }
            },
            Err(e) => println!("{} {:?}", "Error watching:".red(), e),
        }
    }

    // Wait for server process to terminate
    child.wait()?;

    Ok(())
}

async fn handle_file_change(path: &Path) -> Result<()> {
    println!(
        "{} {}",
        "File changed:".green(),
        path.display()
    );
    
    // Recompile modified file
    let project_root = std::env::current_dir()?.to_string_lossy().to_string();
    let file_path = path.to_string_lossy().to_string();
    
    println!("{} {}", "Recompiling".yellow(), file_path);

    match crate::compiler::compile_pyx_file_to_python(path, "config.json", "python").await {
        Ok(_) => println!("{} {}", "✓".green(), "Compilation successful"),
        Err(e) => println!("{} {}: {}", "✗".red(), "Compilation error".red(), e),
    }
    
    Ok(())
}
