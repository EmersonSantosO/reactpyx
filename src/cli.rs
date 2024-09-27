use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use log::error;
use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use tokio::runtime::Runtime;

// Importa los módulos para la CLI
mod cli_build_project;
mod cli_create_project;
mod cli_init_project;
mod cli_install_library;
mod cli_run_server;

// Usa las funciones de los módulos
use cli_build_project::build_project;
use cli_create_project::create_project;
use cli_init_project::init_project;
use cli_install_library::install_library;
use cli_run_server::run_server;

// Runtime de Tokio
static TOKIO_RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Error al crear el runtime de Tokio"));

// Definición de la CLI
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

// Función principal de la CLI
pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    // Manejo de los subcomandos
    match cli.command {
        Commands::CreateProject { project_name } => {
            if let Err(e) = create_project(&project_name) {
                error!("{} {}", "Error al crear el proyecto:".red(), e);
                std::process::exit(1);
            }
        }
        Commands::Init => {
            if let Err(e) = init_project() {
                error!("{} {}", "Error al inicializar el proyecto:".red(), e);
                std::process::exit(1);
            }
        }
        Commands::Run => {
            if let Err(e) = TOKIO_RUNTIME.block_on(run_server()) {
                error!("{} {}", "Error al ejecutar el servidor:".red(), e);
                std::process::exit(1);
            }
        }
        Commands::Build { output } => {
            if let Err(e) = TOKIO_RUNTIME.block_on(build_project(&output)) {
                error!("{} {}", "Error al construir el proyecto:".red(), e);
                std::process::exit(1);
            }
        }
        Commands::Install { library } => {
            if let Err(e) = install_library(&library) {
                error!("{} {}", "Error al instalar la librería:".red(), e);
                std::process::exit(1);
            }
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
