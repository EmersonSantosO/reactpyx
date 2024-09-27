use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::error;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3_asyncio_0_21::tokio::future_into_py;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;
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

fn create_project(project_name: &str) -> Result<()> {
    // Barra de progreso para la creación del proyecto
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Creando proyecto: {msg}"),
    );
    pb.set_message(project_name.to_string());

    // Lógica para crear un proyecto (misma que antes)
    // ...
    std::thread::sleep(Duration::from_secs(2)); // Simula la creación

    pb.finish_with_message(format!(
        "{} {}",
        "Proyecto".green(),
        "creado exitosamente!".green()
    ));
    Ok(())
}

fn init_project() -> Result<()> {
    // Barra de progreso para la inicialización
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Inicializando proyecto..."),
    );

    // Lógica para inicializar el proyecto (misma que antes)
    // ...
    std::thread::sleep(Duration::from_secs(2)); // Simula la inicialización

    pb.finish_with_message(format!(
        "{} {}",
        "Proyecto".green(),
        "inicializado exitosamente!".green()
    ));
    Ok(())
}

async fn run_server() -> Result<()> {
    println!(
        "{} {}",
        "Ejecutando el servidor de desarrollo...".yellow(),
        "http://localhost:8000".blue()
    );

    // Lógica para ejecutar el servidor (misma que antes)
    // ...

    // Observar cambios en los archivos
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;
    watcher.watch("src", RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event {
                Event::Write(path) => {
                    println!("{} {}", "Archivo modificado:".green(), path.display());
                    // Recompilar el archivo modificado
                    // ...
                }
                _ => {}
            },
            Err(e) => println!("{} {:?}", "Error al observar:".red(), e),
        }
    }

    Ok(())
}

async fn build_project(output: &str) -> Result<()> {
    // Barra de progreso para la construcción
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Construyendo proyecto..."),
    );

    // Lógica para construir el proyecto (misma que antes)
    // ...
    std::thread::sleep(Duration::from_secs(2)); // Simula la construcción

    pb.finish_with_message(format!(
        "{} en {}",
        "Proyecto construido exitosamente!".green(),
        output
    ));
    Ok(())
}

/// Nueva función para instalar librerías de estilos como Tailwind
fn install_library(library: &str) -> Result<()> {
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
            error!("{}: {}", "Librería no reconocida".red(), library);
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
