use colored::Colorize;
use log::{error, info, warn};

pub fn log_info(mensaje: &str) {
    info!("{}", mensaje.blue());
    println!("{}", mensaje.blue());
}

pub fn log_warning(mensaje: &str) {
    warn!("{}", mensaje.yellow());
    println!("{}", mensaje.yellow());
}

pub fn log_error(mensaje: &str) {
    error!("{}", mensaje.red());
    println!("{}", mensaje.red());
}
