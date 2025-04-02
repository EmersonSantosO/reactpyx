use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: Option<u16>,
    pub entry: Option<String>,
    #[serde(rename = "entryFunction")]
    pub entry_function: Option<String>,
    #[serde(rename = "publicPath")]
    pub public_path: Option<String>,
    #[serde(rename = "compilerOptions")]
    pub compiler_options: Option<serde_json::Value>,
}

impl Config {
    /// Loads and parses the configuration file
    pub fn load(config_path: &str) -> Result<Self> {
        let config_content = fs::read_to_string(config_path).with_context(|| {
            format!(
                "Error reading configuration file '{}'",
                config_path
            )
        })?;

        let config: Config = serde_json::from_str(&config_content)
            .with_context(|| "Error parsing JSON configuration file")?;

        Ok(config)
    }
}
