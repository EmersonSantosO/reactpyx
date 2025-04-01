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
            .template("{spinner:.blue} Construyendo proyecto...")?,
    );

    match env {
        "development" => {
            info!("Modo Desarrollo Activado");
            transform_styles_to_js().await?;
            build_development_assets(output).await?;
        }
        "node" | "python" => {
            info!("Modo Producción Activado");
            transform_styles_to_js().await?;
            minify_and_optimize_assets(output).await?;
            generate_server_files(output, env).await?;
        }
        _ => {
            error!("Ambiente de despliegue inválido: {}", env);
            return Err(anyhow::anyhow!("Ambiente de despliegue inválido: {}", env));
        }
    }

    pb.finish_with_message(format!(
        "{} en {}",
        "Proyecto construido exitosamente!".green(),
        output
    ));
    Ok(())
}

async fn transform_styles_to_js() -> Result<()> {
    println!("Transformando estilos a JavaScript...");
    Ok(())
}

async fn build_development_assets(output: &str) -> Result<()> {
    println!("Construyendo recursos para desarrollo...");
    Ok(())
}

async fn minify_and_optimize_assets(output: &str) -> Result<()> {
    println!("Minificando y optimizando recursos...");
    Ok(())
}

async fn generate_server_files(output: &str, env: &str) -> Result<()> {
    match env {
        "node" => generate_node_server_files(output).await,
        "python" => generate_fastapi_files(output).await,
        _ => unreachable!("Ambiente de despliegue inválido"),
    }
}

async fn generate_node_server_files(output: &str) -> Result<()> {
    fs::write(format!("{}/server.js", output), "/* Código para Node.js */").await?;
    Ok(())
}

async fn generate_fastapi_files(output: &str) -> Result<()> {
    fs::write(format!("{}/main.py", output), "/* Código para FastAPI */").await?;
    Ok(())
}
