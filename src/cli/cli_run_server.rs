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
        "Ejecutando el servidor de desarrollo...".yellow(),
        "http://localhost:8000".blue()
    );

    // Verificar si uvicorn está disponible
    let uvicorn_check = Command::new("uvicorn")
        .arg("--version")
        .output()
        .map(|output| output.status.success());

    if uvicorn_check.is_err() || !uvicorn_check.unwrap() {
        println!(
            "{} {}",
            "Error:".red(),
            "Uvicorn no está instalado o no está disponible en el PATH".red()
        );
        println!(
            "{} {}",
            "Sugerencia:".yellow(),
            "Instala uvicorn con 'pip install uvicorn'".yellow()
        );
        return Err(anyhow::anyhow!("Uvicorn no está disponible"));
    }

    // Inicia el servidor FastAPI en un proceso separado
    let mut child = Command::new("uvicorn")
        .arg("main:app")
        .arg("--reload")
        .spawn()?;

    println!("{} {}", "✓".green(), "Servidor iniciado correctamente");
    println!(
        "{} {}",
        "Observando cambios en".blue(),
        "src/**/*.pyx".bright_blue()
    );

    // Observa cambios en los archivos de la carpeta `src`
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        tx,
        Duration::from_secs(1),
    )?;
    
    watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;

    // Maneja eventos de cambio
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
            Err(e) => println!("{} {:?}", "Error al observar:".red(), e),
        }
    }

    // Espera a que el proceso del servidor termine
    child.wait()?;

    Ok(())
}

async fn handle_file_change(path: &Path) -> Result<()> {
    println!(
        "{} {}",
        "Archivo modificado:".green(),
        path.display()
    );
    
    // Recompilar el archivo modificado
    let project_root = std::env::current_dir()?.to_string_lossy().to_string();
    let file_path = path.to_string_lossy().to_string();
    
    println!("{} {}", "Recompilando".yellow(), file_path);

    match compiler::compile_pyx_file_to_python(path, "config.json", "python").await {
        Ok(_) => println!("{} {}", "✓".green(), "Compilación exitosa"),
        Err(e) => println!("{} {}: {}", "✗".red(), "Error al compilar".red(), e),
    }
    
    Ok(())
}
