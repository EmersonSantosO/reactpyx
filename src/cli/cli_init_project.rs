use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::time::Duration;

pub fn init_project(env: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Inicializando proyecto en modo {msg}...")?,
    );
    pb.set_message(env.to_string());

    // Instalar dependencias específicas para el entorno
    match env {
        "development" => {
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi", "uvicorn"])
                .spawn()?
                .wait()?;
        }
        "production" => {
            // Solo instala dependencias necesarias para producción
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi"])
                .spawn()?
                .wait()?;
        }
        _ => {
            return Err(anyhow::anyhow!("Entorno no reconocido: {}", env));
        }
    }

    pb.finish_with_message(format!(
        "{} {}",
        "Proyecto".green(),
        "inicializado exitosamente!".green()
    ));
    Ok(())
}
