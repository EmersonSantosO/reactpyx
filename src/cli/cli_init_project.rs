use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
pub fn init_project() -> Result<()> {
    // Barra de progreso para la inicialización
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Inicializando proyecto...")?,
    );

    // Lógica para inicializar el proyecto
    // ...
    std::thread::sleep(Duration::from_secs(2)); // Simula la inicialización

    pb.finish_with_message(format!(
        "{} {}",
        "Proyecto".green(),
        "inicializado exitosamente!".green()
    ));
    Ok(())
}
