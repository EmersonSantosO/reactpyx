use colored::Colorize;
use log::{error, info, warn};

pub fn log_info(message: &str) {
    info!("{}", message.blue());
    println!("{}", message.blue());
}

pub fn log_warning(message: &str) {
    warn!("{}", message.yellow());
    println!("{}", message.yellow());
}

pub fn log_error(message: &str) {
    error!("{}", message.red());
    println!("{}", message.red());
}
