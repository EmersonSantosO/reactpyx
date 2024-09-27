use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::Duration;

pub fn create_project(project_name: &str) -> Result<()> {
    // Barra de progreso para la creación del proyecto
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Creando proyecto: {msg}"),
    );
    pb.set_message(project_name.to_string());

    // Lógica para crear un proyecto
    // ...
    std::thread::sleep(Duration::from_secs(2)); // Simula la creación

    pb.finish_with_message(format!(
        "{} {}",
        "Proyecto".green(),
        "creado exitosamente!".green()
    ));
    Ok(())
}
