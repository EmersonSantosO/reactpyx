use anyhow::Result;
use std::process::Command;

pub fn install_plugin(plugin_name: &str) -> Result<()> {
    match plugin_name {
        "tailwind" => {
            println!("Installing plugin: Tailwind CSS...");
            Command::new("npm")
                .args(&["install", "-D", "tailwindcss"])
                .spawn()?
                .wait()?;
            Command::new("npx")
                .args(&["tailwindcss", "init"])
                .spawn()?
                .wait()?;
            println!("Tailwind CSS installed successfully.");
        }
        "bootstrap" => {
            println!("Installing plugin: Bootstrap...");
            Command::new("npm")
                .args(&["install", "bootstrap"])
                .spawn()?
                .wait()?;
            println!("Bootstrap installed successfully.");
        }
        _ => {
            println!("Unrecognized plugin: {}", plugin_name);
        }
    }
    Ok(())
}
