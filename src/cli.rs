use anyhow::Context;
use clap::Parser;
use clap_derive::{Parser, Subcommand};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

mod cli_build_project;
mod cli_create_project;
mod cli_init_project;
mod cli_install_library;
mod cli_run_server;

use cli_build_project::build_project;
use cli_create_project::create_project;
use cli_init_project::init_project;
use cli_install_library::install_library;
use cli_run_server::run_server;
use log::{error, info};

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Error creating Tokio runtime")
});

const ENV_OPTIONS: &[&str] = &["node", "python"];
const ALLOWED_LIBRARIES: &[&str] = &["tailwind", "bootstrap"];

#[derive(Parser)]
#[command(name = "reactpyx")]
#[command(about = "ReactPyx CLI built in Rust.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new ReactPyx project
    CreateProject {
        /// Project name
        project_name: String,
    },
    /// Initialize project (install dependencies)
    Init {
        /// Specify the environment (development or production)
        #[arg(short, long, default_value = "development")]
        env: String,
    },
    /// Run the development server
    Run,
    /// Build the project for production (Node.js or Python)
    Build {
        /// Deployment environment (node or python)
        #[arg(short, long)]
        env: String,
        /// Output directory for compiled files
        #[arg(short, long, default_value = "dist")]
        output: String,
    },
    /// Install a style library (e.g., tailwind, bootstrap)
    Install {
        /// Name of the library (e.g., tailwind)
        library: String,
    },
}

pub fn run_cli() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateProject { project_name } => {
            info!("Creando proyecto: {}", project_name);
            create_project(&project_name).context("Error al crear el proyecto")?;
        }
        Commands::Init { env } => {
            info!("Inicializando proyecto en modo {}", env);
            if !["development", "production"].contains(&env.as_str()) {
                error!(
                    "Ambiente inválido: {}. Usa 'development' o 'production'.",
                    env
                );
                std::process::exit(1);
            }
            init_project(&env).context("Error al inicializar el proyecto")?;
        }
        Commands::Run => {
            info!("Ejecutando servidor de desarrollo");
            TOKIO_RUNTIME
                .block_on(run_server())
                .context("Error al ejecutar el servidor")?;
        }
        Commands::Build { env, output } => {
            info!("Construyendo proyecto para ambiente {}", env);
            if !ENV_OPTIONS.contains(&env.as_str()) {
                error!("Ambiente no reconocido: {}. Usa 'node' o 'python'.", env);
                std::process::exit(1);
            }
            TOKIO_RUNTIME
                .block_on(build_project(&output, &env))
                .context("Error al construir el proyecto")?;
        }
        Commands::Install { library } => {
            info!("Instalando librería: {}", library);
            if !ALLOWED_LIBRARIES.contains(&library.as_str()) {
                error!(
                    "Librería no reconocida: {}. Las librerías permitidas son: {:?}",
                    library, ALLOWED_LIBRARIES
                );
                std::process::exit(1);
            }
            install_library(&library).context("Error al instalar la librería")?;
        }
    }

    info!("Comando ejecutado exitosamente.");
    Ok(())
}
