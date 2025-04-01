use anyhow::Result;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

/// Iniciar un watcher para cambios en archivos de componentes
pub async fn start_hmr_watcher(
    project_root: &str,
    tx: mpsc::Sender<String>,
    config_tx: mpsc::Sender<String>, // Envía rutas para cambios de configuración
) -> Result<()> {
    // Canal para notificaciones de cambios en el sistema de archivos
    let (notify_tx, notify_rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher =
        Watcher::new(notify_tx, std::time::Duration::from_secs(1))?;

    // Directorios y archivos a observar
    let components_path = Path::new(project_root).join("src").join("components");
    let config_path = Path::new(project_root).join("pyx.config.json");

    // Observar directorios de componentes y archivo de configuración
    watcher.watch(&components_path, RecursiveMode::Recursive)?;
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;

    // Convertir std_mpsc::Receiver a un stream de Tokio
    let (stream_tx, mut stream_rx) = mpsc::channel(100);
    tokio::spawn(async move {
        while let Ok(event) = notify_rx.recv() {
            if let Err(_) = stream_tx.send(event).await {
                break; // Detener observación si el canal está cerrado
            }
        }
    });

    // Procesar eventos asíncronos usando Tokio Stream
    let mut stream = ReceiverStream::new(stream_rx);

    while let Some(event) = stream.next().await {
        match event {
            Event {
                kind: EventKind::Modify(_) | EventKind::Create(_),
                paths,
                ..
            } => {
                for path in paths {
                    if let Some(ext) = path.extension() {
                        // Procesar archivos `.pyx`
                        if ext == "pyx" {
                            if let Some(file_path) = path.to_str() {
                                tx.send(file_path.to_string()).await.unwrap_or_default();
                            }
                        } else if path.ends_with("pyx.config.json") {
                            // Procesar cambios en el archivo de configuración
                            if let Some(file_path) = path.to_str() {
                                config_tx
                                    .send(file_path.to_string())
                                    .await
                                    .unwrap_or_default();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
