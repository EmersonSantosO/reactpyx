use crate::compiler::compile_all_pyx;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use std::env;
use std::time::Duration;

pub async fn build_project(output: &str, env: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Building project: {msg}")?,
    );
    pb.set_message("Compiling components...");

    // Get current directory as project root
    let project_root = env::current_dir()?.to_string_lossy().to_string();
    let config_path = "pyx.config.json"; // Default config path

    // Compile all components
    // This will generate Python files in build/components and styles.css
    match compile_all_pyx(&project_root, config_path, env).await {
        Ok((compiled_files, _)) => {
            pb.set_message(format!("Compiled {} files", compiled_files.len()));
        }
        Err(e) => {
            pb.finish_with_message(format!("{} {}", "Build failed:".red(), e));
            return Err(e);
        }
    }

    match env {
        "development" => {
            info!("Development build complete");
        }
        "node" | "python" => {
            info!("Production build complete");
            // Here we could add extra steps like minification of the final bundle
            // if it wasn't already handled by compile_all_pyx
        }
        _ => {
            error!("Invalid deployment environment: {}", env);
            return Err(anyhow::anyhow!("Invalid deployment environment: {}", env));
        }
    }

    pb.finish_with_message(format!(
        "{} in {}",
        "Project built successfully!".green(),
        output
    ));
    Ok(())
}
