use anyhow::{Context, Result};
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

    // Verificar que pip esté instalado
    let pip_check = Command::new("pip")
        .arg("--version")
        .output()
        .context("No se pudo ejecutar pip. Asegúrate de que pip esté instalado y en el PATH")?;

    if !pip_check.status.success() {
        return Err(anyhow::anyhow!("Pip no está correctamente instalado"));
    }

    // Instalar dependencias específicas para el entorno
    match env {
        "development" => {
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi", "uvicorn"])
                .spawn()?
                .wait()
                .context("Error al instalar dependencias de desarrollo")?;
        }
        "production" => {
            // Solo instala dependencias necesarias para producción
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi"])
                .spawn()?
                .wait()
                .context("Error al instalar dependencias de producción")?;
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
