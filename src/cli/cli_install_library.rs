use anyhow::Result;
use colored::Colorize;
use std::process::Command;

pub fn install_library(library: &str) -> Result<()> {
    match library {
        "tailwind" => {
            println!("{} Tailwind CSS...", "Instalando".green());
            Command::new("npm")
                .args(&["install", "-D", "tailwindcss"])
                .spawn()?
                .wait()?;

            Command::new("npx")
                .args(&["tailwindcss", "init"])
                .spawn()?
                .wait()?;
            println!("{} Tailwind CSS instalado.", "Completado:".green());
        }
        "bootstrap" => {
            println!("{} Bootstrap...", "Instalando".green());
            Command::new("npm")
                .args(&["install", "bootstrap"])
                .spawn()?
                .wait()?;
            println!("{} Bootstrap instalado.", "Completado:".green());
        }
        _ => {
            return Err(anyhow::anyhow!("Librería no reconocida: {}", library));
        }
    }
    Ok(())
}
