use anyhow::Result;
use clap::{Parser, Subcommand};
use log::error;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use std::process::Command;
use tokio::runtime::Runtime;

static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Error al crear el runtime de Tokio"));

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
    /// Instalar una librería de estilos (por ejemplo: tailwind, bootstrap)
    Install {
        /// Nombre de la librería (por ejemplo: tailwind)
        library: String,
    },
}

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
            if let Err(e) = TOKIO_RUNTIME.block_on(run_server()) {
                error!("Error al ejecutar el servidor: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Build { output } => {
            if let Err(e) = TOKIO_RUNTIME.block_on(build_project(&output)) {
                error!("Error al construir el proyecto: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Install { library } => {
            if let Err(e) = install_library(&library) {
                error!("Error al instalar la librería: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn create_project(project_name: &str) -> Result<()> {
    println!("Creando proyecto: {}", project_name);
    // Lógica para crear un proyecto
    Ok(())
}

fn init_project() -> Result<()> {
    println!("Inicializando el proyecto");
    // Lógica para inicializar el proyecto
    Ok(())
}

async fn run_server() -> Result<()> {
    println!("Ejecutando el servidor de desarrollo");
    // Lógica para ejecutar el servidor
    Ok(())
}

async fn build_project(output: &str) -> Result<()> {
    println!(
        "Construyendo el proyecto para producción en el directorio: {}",
        output
    );
    // Lógica para construir el proyecto
    Ok(())
}

/// Nueva función para instalar librerías de estilos como Tailwind
fn install_library(library: &str) -> Result<()> {
    match library {
        "tailwind" => {
            println!("Instalando Tailwind CSS...");
            Command::new("npm")
                .args(&["install", "-D", "tailwindcss"])
                .spawn()?
                .wait()?;

            Command::new("npx")
                .args(&["tailwindcss", "init"])
                .spawn()?
                .wait()?;
            println!("Tailwind CSS instalado correctamente.");
        }
        "bootstrap" => {
            println!("Instalando Bootstrap...");
            Command::new("npm")
                .args(&["install", "bootstrap"])
                .spawn()?
                .wait()?;
            println!("Bootstrap instalado correctamente.");
        }
        _ => {
            error!("Librería no reconocida: {}", library);
            return Err(anyhow::anyhow!("Librería no reconocida: {}", library));
        }
    }
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
pub fn run_server_py(py: Python) -> PyResult<Bound<PyAny>> {
    future_into_py(py, async {
        run_server()
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    })
}

#[pyfunction]
pub fn build_project_py<'a>(output: &'a str, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
    let output = output.to_string(); // Clona la cadena para hacerla `'static`
    future_into_py(py, async move {
        build_project(&output)
            .await
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    })
}
