use anyhow::Result;
use std::process::Command;

pub fn install_plugin(plugin_name: &str) -> Result<()> {
    match plugin_name {
        "tailwind" => {
            println!("Instalando plugin: Tailwind CSS...");
            Command::new("npm")
                .args(&["install", "-D", "tailwindcss"])
                .spawn()?
                .wait()?;
            Command::new("npx")
                .args(&["tailwindcss", "init"])
                .spawn()?
                .wait()?;
            println!("Tailwind CSS instalado con éxito.");
        }
        "bootstrap" => {
            println!("Instalando plugin: Bootstrap...");
            Command::new("npm")
                .args(&["install", "bootstrap"])
                .spawn()?
                .wait()?;
            println!("Bootstrap instalado con éxito.");
        }
        _ => {
            println!("Plugin no reconocido: {}", plugin_name);
        }
    }
    Ok(())
}
