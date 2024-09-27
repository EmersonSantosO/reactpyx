use anyhow::Result;
use clap::{Parser, Subcommand};
use log::error;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

const CONFIG_PATH: &str = "pyx.config.json";
const ENTRY_FILE: &str = "./src/main.pyx";

#[derive(Parser)]
#[command(name = "reactpyx")]
#[command(about = "Empaquetador rápido para ReactPyx construido en Rust.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crear un nuevo proyecto ReactPyx
    CreateProject {
        /// Nombre del proyecto
        project_name: String,
    },
    /// Inicializar el proyecto (instalar dependencias)
    Init,
    /// Ejecutar el servidor de desarrollo
    Run,
    /// Construir el proyecto para producción
    Build {
        /// Directorio de salida para los archivos construidos
        #[arg(short, long, default_value = "dist")]
        output: String,
    },
}

/// Función principal que ejecuta la CLI.
pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateProject { project_name } => {
            if let Err(e) = create_project(&project_name) {
                error!("Error al crear el proyecto: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Init => {
            if let Err(e) = init_project() {
                error!("Error al inicializar el proyecto: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Run => {
            let rt = Runtime::new().expect("Error creando el runtime de Tokio");
            if let Err(e) = rt.block_on(run_server()) {
                error!("Error al ejecutar el servidor: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Build { output } => {
            let rt = Runtime::new().expect("Error creando el runtime de Tokio");
            if let Err(e) = rt.block_on(build_project(&output)) {
                error!("Error al construir el proyecto: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn create_project(project_name: &str) -> Result<()> {
    // Lógica para crear un proyecto
    println!("Creando proyecto: {}", project_name);
    Ok(())
}

fn init_project() -> Result<()> {
    // Lógica para inicializar el proyecto
    println!("Inicializando el proyecto");
    Ok(())
}

async fn run_server() -> Result<()> {
    // Lógica para ejecutar el servidor
    println!("Ejecutando el servidor de desarrollo");
    Ok(())
}

async fn build_project(output: &str) -> Result<()> {
    // Lógica para construir el proyecto
    println!(
        "Construyendo el proyecto para producción en el directorio: {}",
        output
    );
    Ok(())
}

// Exponer funciones a Python usando PyO3.
#[pyfunction]
pub fn run_cli_py() -> PyResult<()> {
    run_cli().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn create_project_py(project_name: &str) -> PyResult<()> {
    create_project(project_name)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn init_project_py() -> PyResult<()> {
    init_project().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn run_server_py() -> PyResult<()> {
    let rt = Runtime::new().expect("Error creando el runtime de Tokio");
    rt.block_on(run_server())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
pub fn build_project_py(output: &str) -> PyResult<()> {
    let rt = Runtime::new().expect("Error creando el runtime de Tokio");
    rt.block_on(build_project(output))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
