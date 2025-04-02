use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
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

    // Create necessary directories for CSS integration
    fs::create_dir_all("src/styles").context("Failed to create styles directory")?;
    fs::create_dir_all("public/static").context("Failed to create static directory")?;

    // Create a basic styles.css file
    fs::write(
        "src/styles/main.css",
        r#"/* Main application styles */
:root {
  --primary-color: #3490dc;
  --secondary-color: #ffed4a;
  --accent-color: #f56565;
  --text-color: #333;
  --bg-color: #f8fafc;
}

body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  color: var(--text-color);
  background-color: var(--bg-color);
  line-height: 1.5;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 1rem;
}
"#,
    ).context("Failed to create default CSS file")?;

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
            
            // Create a development index.html with live reload support
            fs::write(
                "public/index.html",
                r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>ReactPyx Application - Development</title>
        <link rel="stylesheet" href="/static/styles.css">
        <meta name="description" content="A web application built with ReactPyx, a Python framework similar to JSX.">
        
        <!-- Add CDN libraries here if needed -->
        <!-- For example: -->
        <!-- <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet"> -->
    </head>
    <body>
        <div id="app">
            <!-- Content rendered by ReactPyx will be inserted here -->
        </div>
        <script src="/static/app.js" async></script>
        <script>
            // Simple live reload (for development)
            const liveReload = () => {
                const socket = new WebSocket(`ws://${location.host}/ws/live-reload`);
                socket.onmessage = () => location.reload();
                socket.onclose = () => setTimeout(liveReload, 1000);
            };
            liveReload();
        </script>
    </body>
</html>"#,
            ).context("Failed to create development index.html")?;
        }
        "production" => {
            // Only install production-necessary dependencies
            Command::new("pip")
                .args(&["install", "reactpyx", "fastapi"])
                .spawn()?
                .wait()
                .context("Error installing production dependencies")?;
            
            // Create a production index.html without development features
            fs::write(
                "public/index.html",
                r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>ReactPyx Application</title>
        <link rel="stylesheet" href="/static/styles.css">
        <meta name="description" content="A web application built with ReactPyx, a Python framework similar to JSX.">
        
        <!-- Add CDN libraries here if needed -->
    </head>
    <body>
        <div id="app">
            <!-- Content rendered by ReactPyx will be inserted here -->
        </div>
        <script src="/static/app.js" async></script>
    </body>
</html>"#,
            ).context("Failed to create production index.html")?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unrecognized environment: {}", env));
        }
    }

    // Create a basic CSS integration helper
    fs::write(
        "src/css_helper.py",
        r#"""CSS helper module for ReactPyx"""

def use_styles(styles_dict):
    """Helper function to use inline styles
    
    Example:
    styles = use_styles({
        "container": "display: flex; flex-direction: column;",
        "header": "font-size: 24px; font-weight: bold;"
    })
    
    # Then in your JSX:
    # <div style={styles.container}>
    #     <h1 style={styles.header}>Title</h1>
    # </div>
    """
    return {k: {"style": v} for k, v in styles_dict.items()}

def combine_classes(*args):
    """Helper function to combine multiple class names
    
    Example: 
    className={combine_classes("btn", "btn-primary", is_active and "active")}
    """
    return " ".join([cls for cls in args if cls])
"#,
    ).context("Failed to create CSS helper module")?;

    pb.finish_with_message(format!(
        "{} {}",
        "Project".green(),
        "successfully initialized!".green()
    ));
    
    println!("\n{}", "Quick Start:".cyan());
    println!("1. Add CSS via CDN: {}", "reactpyx Install tailwind".yellow());
    println!("2. Run the server: {}", "reactpyx Run".yellow());
    println!("3. Build for production: {}", "reactpyx Build --env python --output dist".yellow());
    
    Ok(())
}
