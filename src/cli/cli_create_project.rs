use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::time::Duration;

pub fn create_project(project_name: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Creando proyecto: {msg}")?,
    );
    pb.set_message(project_name.to_string());

    // Crear directorios y archivos iniciales
    fs::create_dir_all(format!("{}/src/components", project_name))?;
    fs::create_dir_all(format!("{}/public/static", project_name))?;

    // Crear archivo principal `main.pyx`
    let main_content = r#"
from App import App

def MainApp():
    return App()
"#;
    fs::write(format!("{}/src/main.pyx", project_name), main_content)?;

    pb.finish_with_message(format!(
        "{} {}",
        "Proyecto".green(),
        "creado exitosamente!".green()
    ));
    Ok(())
}
