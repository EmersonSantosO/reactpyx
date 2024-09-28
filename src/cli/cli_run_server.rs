use anyhow::Result;
use colored::Colorize;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

pub async fn run_server() -> Result<()> {
    println!(
        "{} {}",
        "Ejecutando el servidor de desarrollo...".yellow(),
        "http://localhost:8000".blue()
    );

    // Inicia el servidor FastAPI en un proceso separado
    let mut child = Command::new("uvicorn")
        .arg("main:app")
        .arg("--reload")
        .spawn()?;

    // Observa cambios en los archivos
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;
    watcher.watch(Path::new("src"), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Create(_) => {
                    println!("{} {}", "Archivo creado:".green(), event.paths[0].display());
                    //  Aquí puedes agregar lógica para recompilar el archivo modificado
                }
                notify::EventKind::Modify(_) => {
                    println!(
                        "{} {}",
                        "Archivo modificado:".green(),
                        event.paths[0].display()
                    );
                    //  Aquí puedes agregar lógica para recompilar el archivo modificado
                }
                _ => {}
            },
            Err(e) => println!("{} {:?}", "Error al observar:".red(), e),
        }
    }

    // Espera a que el proceso del servidor termine
    child.wait()?;

    Ok(())
}
