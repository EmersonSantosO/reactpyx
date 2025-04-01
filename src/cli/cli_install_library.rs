use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

pub fn install_library(library: &str) -> Result<()> {
    println!("{} {}...", "Instalando".green(), library.blue());
    
    match library {
        "tailwind" => {
            Command::new("npm")
                .args(&["install", "-D", "tailwindcss"])
                .spawn()?
                .wait()
                .context("Error al ejecutar npm install tailwindcss")?;

            Command::new("npx")
                .args(&["tailwindcss", "init"])
                .spawn()?
                .wait()
                .context("Error al inicializar tailwindcss")?;
                
            println!("{} {}", "✓".green(), "Tailwind CSS instalado correctamente");
            
            // Crear archivos de configuración para Tailwind
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
            .context("Error al crear tailwind.config.js")?;
        }
        "bootstrap" => {
            Command::new("npm")
                .args(&["install", "bootstrap"])
                .spawn()?
                .wait()
                .context("Error al ejecutar npm install bootstrap")?;
                
            println!("{} {}", "✓".green(), "Bootstrap instalado correctamente");
            
            // Crear archivo de importación para Bootstrap
            std::fs::write(
                "src/styles/bootstrap.css",
                r#"/* Archivo de importación de Bootstrap */
@import 'bootstrap/dist/css/bootstrap.min.css';
"#,
            )
            .context("Error al crear archivo de importación bootstrap.css")?;
        }
        _ => {
            return Err(anyhow::anyhow!("Librería no reconocida: {}", library));
        }
    }
    
    println!("{} {}", "Completado:".green(), format!("{} instalado y configurado.", library).green());
    Ok(())
}
