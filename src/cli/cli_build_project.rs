use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use std::time::Duration;
use tokio::fs;

pub async fn build_project(output: &str, env: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Building project...")?,
    );

    match env {
        "development" => {
            info!("Development Mode Activated");
            transform_styles_to_js().await?;
            build_development_assets(output).await?;
        }
        "node" | "python" => {
            info!("Production Mode Activated");
            transform_styles_to_js().await?;
            minify_and_optimize_assets(output).await?;
            generate_server_files(output, env).await?;
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

async fn transform_styles_to_js() -> Result<()> {
    println!("Transforming styles to JavaScript...");
    Ok(())
}

async fn build_development_assets(output: &str) -> Result<()> {
    println!("Building development assets...");
    Ok(())
}

async fn minify_and_optimize_assets(output: &str) -> Result<()> {
    println!("Minifying and optimizing assets...");
    Ok(())
}

async fn generate_server_files(output: &str, env: &str) -> Result<()> {
    match env {
        "node" => generate_node_server_files(output).await,
        "python" => generate_fastapi_files(output).await,
        _ => unreachable!("Invalid deployment environment"),
    }
}

async fn generate_node_server_files(output: &str) -> Result<()> {
    fs::write(format!("{}/server.js", output), "/* Node.js server code */").await?;
    Ok(())
}

async fn generate_fastapi_files(output: &str) -> Result<()> {
    fs::write(format!("{}/main.py", output), "/* FastAPI server code */").await?;
    Ok(())
}
