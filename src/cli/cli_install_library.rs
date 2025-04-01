use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

pub fn install_library(library: &str) -> Result<()> {
    println!("{} {}...", "Installing".green(), library.blue());
    
    match library {
        "tailwind" => {
            Command::new("npm")
                .args(&["install", "-D", "tailwindcss"])
                .spawn()?
                .wait()
                .context("Error running npm install tailwindcss")?;

            Command::new("npx")
                .args(&["tailwindcss", "init"])
                .spawn()?
                .wait()
                .context("Error initializing tailwindcss")?;
                
            println!("{} {}", "✓".green(), "Tailwind CSS installed successfully");
            
            // Create configuration files for Tailwind
            std::fs::write(
                "tailwind.config.js",
                r#"module.exports = {
  content: ["./src/**/*.{pyx,py,html,js}"],
  theme: {
    extend: {},
  },
  plugins: [],
}"#,
            )
            .context("Error creating tailwind.config.js")?;
        }
        "bootstrap" => {
            Command::new("npm")
                .args(&["install", "bootstrap"])
                .spawn()?
                .wait()
                .context("Error running npm install bootstrap")?;
                
            println!("{} {}", "✓".green(), "Bootstrap installed successfully");
            
            // Create import file for Bootstrap
            std::fs::write(
                "src/styles/bootstrap.css",
                r#"/* Bootstrap import file */
@import 'bootstrap/dist/css/bootstrap.min.css';
"#,
            )
            .context("Error creating bootstrap.css import file")?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unrecognized library: {}", library));
        }
    }
    
    println!("{} {}", "Completed:".green(), format!("{} installed and configured.", library).green());
    Ok(())
}
