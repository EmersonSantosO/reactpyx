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
            .template("{spinner:.blue} Initializing project in {msg} mode...")?,
    );
    pb.set_message(env.to_string());

    // Check that pip is installed
    let pip_check = Command::new("pip")
        .arg("--version")
        .output()
        .context("Could not run pip. Make sure pip is installed and in your PATH")?;

    if !pip_check.status.success() {
        return Err(anyhow::anyhow!("Pip is not properly installed"));
    }

    // Install environment-specific dependencies
    match env {
        "development" => {
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi", "uvicorn"])
                .spawn()?
                .wait()
                .context("Error installing development dependencies")?;
        }
        "production" => {
            // Only install production-necessary dependencies
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi"])
                .spawn()?
                .wait()
                .context("Error installing production dependencies")?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unrecognized environment: {}", env));
        }
    }

    pb.finish_with_message(format!(
        "{} {}",
        "Project".green(),
        "successfully initialized!".green()
    ));
    Ok(())
}
