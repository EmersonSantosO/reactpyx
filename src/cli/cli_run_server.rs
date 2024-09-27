use anyhow::Result;
use colored::Colorize;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::runtime::Runtime;

pub async fn run_server() -> Result<()> {
    println!(
        "{} {}",
        "Ejecutando el servidor de desarrollo...".yellow(),
        "http://localhost:8000".blue()
    );

    // ----> Inicia el servidor FastAPI en un proceso separado
    let mut child = Command::new("uvicorn") // Asumiendo que usas Uvicorn
        .arg("main:app") // Reemplaza con la ruta a tu archivo principal y la instancia de la aplicación FastAPI
        .arg("--reload") // Habilita la recarga automática en Uvicorn
        .spawn()?;

    // ----> Observa cambios en los archivos
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;
    watcher.watch("src", RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event {
                Event::Write(path) => {
                    println!("{} {}", "Archivo modificado:".green(), path.display());
                    // ----> Aquí puedes agregar lógica para recompilar el archivo modificado
                }
                _ => {}
            },
            Err(e) => println!("{} {:?}", "Error al observar:".red(), e),
        }
    }

    // ----> Espera a que el proceso del servidor termine
    child.wait()?;

    Ok(())
}
