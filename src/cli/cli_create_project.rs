// src/cli/cli_create_project.rs
use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::{fs, path::Path};

pub fn create_project(project_name: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Creating project: {msg}")?,
    );
    pb.set_message(project_name.to_string());

    let project_root = Path::new(project_name);

    // --- Create minimal directories (Vite-like template) ---
    fs::create_dir_all(project_root.join("src"))
        .context("Failed to create src directory")?;
    fs::create_dir_all(project_root.join("public"))
        .context("Failed to create public directory")?;
    fs::create_dir_all(project_root.join("templates"))
        .context("Failed to create templates directory")?;

    // --- Create minimal .pyx entry file ---
    let main_pyx_content = r#"# src/main.pyx - Entry point
from App import App


def MainApp():
    return App()
"#;
    fs::write(project_root.join("src/main.pyx"), main_pyx_content)
        .context("Failed to write src/main.pyx")?;

    // --- Create minimal App component ---
    let app_pyx_content = r#"# src/App.pyx
from reactpyx import use_state


def App():
    count, set_count = use_state("app", "count", 0)

    def increment():
        set_count.set(count + 1)

    return (
        <main style="font-family: system-ui; padding: 2rem;">
            <h1>ReactPyx + Python</h1>
            <p>Vite-like minimal starter.</p>
            <button onClick={increment}>
                count is {count}
            </button>
        </main>
    )
"#;
    fs::write(project_root.join("src/App.pyx"), app_pyx_content)
        .context("Failed to write src/App.pyx")?;

    // --- Create config file ---
    let config_content = r#"{
    "entry": "./src/main.pyx",
    "entryFunction": "MainApp",
    "publicPath": "./public",
    "compilerOptions": {
        "minify": true,
        "sourceMaps": true
    }
}"#;
    fs::write(project_root.join("pyx.config.json"), config_content)
        .context("Failed to write pyx.config.json")?;

    // --- Create minimal main.py (FastAPI + ReactPyx runtime) ---
    let server_content = r#"# main.py - Minimal FastAPI server for ReactPyx app
from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
import os

import reactpyx


app = FastAPI()

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

templates_dir = os.path.join(BASE_DIR, "templates")
templates = Jinja2Templates(directory=templates_dir)

static_dir = os.path.join(BASE_DIR, "public", "static")
os.makedirs(static_dir, exist_ok=True)
app.mount("/static", StaticFiles(directory=static_dir), name="static")


@app.get("/", response_class=HTMLResponse)
async def index(request: Request):
    return templates.TemplateResponse("index.jinja2", {"request": request})


@app.websocket("/_reactpyx/ws")
async def reactpyx_ws(websocket):
    await reactpyx.server.handle_websocket(websocket)


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
"#;
    return FileResponse(favicon_path) if os.path.exists(favicon_path) else HTMLResponse(status_code=204)

@app.get("/{full_path:path}", response_class=HTMLResponse)
async def serve_spa(request: Request, full_path: str):
    # SSR Implementation
    try:
        # Add build directory to path to import compiled components
        build_dir = os.path.join(BASE_DIR, "build")
        if build_dir not in sys.path:
            sys.path.append(build_dir)
            
        # Import the entry point (compiled from src/main.pyx)
        # src/main.pyx compiles to build/components/main.py
        from components import main as app_main
        
        # Reload module in development to pick up changes
        import importlib
        importlib.reload(app_main)
        
        # Render the app
        app_instance = app_main.MainApp()
        app_html = app_instance.render()
        
        # Serialize state for hydration
        import json
        app_state = app_instance.to_dict()
        app_state_json = json.dumps(app_state)
    except Exception as e:
        print(f"SSR Error: {e}")
        app_html = f"<!-- SSR Error: {e} -->"
        app_state_json = "{}"

    context = {
        "request": request,
        "page_title": "ReactPyx Application",
        "content": app_html,
        "initial_state": app_state_json
    }
    return templates.TemplateResponse("index.jinja2", context)

@app.exception_handler(404)
async def custom_404_handler(request: Request, exc: HTTPException):
    context = {"request": request}
    template_404_path = os.path.join(templates_dir, "404.jinja2")
    if os.path.exists(template_404_path):
        return templates.TemplateResponse("404.jinja2", context, status_code=404)
    else:
        return HTMLResponse(content="<h1>404 - Not Found</h1>", status_code=404)

if __name__ == "__main__":
    import uvicorn
    port = int(os.environ.get("PORT", 8000))
    host = os.environ.get("HOST", "0.0.0.0")
    uvicorn.run("main:app", host=host, port=port, reload=True)
"#;
    fs::write(project_root.join("main.py"), server_content).context("Failed to write main.py")?;

    // --- Create Jinja2 templates ---
    // (base.jinja2 content verified from previous step)
    let base_template_content = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ page_title | default('ReactPyx App') }}</title>
    {% block head_meta %}
        <meta name="description" content="Una aplicación web construida con ReactPyx.">
    {% endblock head_meta %}
    <link rel="stylesheet" href="{{ url_for('static', path='styles.css') }}">
    {% block head_extra %}{% endblock head_extra %}
    <link rel="icon" href="/favicon.ico" type="image/x-icon">
</head>
<body>
    <div id="app">
        {{ content | safe }}
        {% block content %}
        {% endblock content %}
    </div>
    <script>
        window.__INITIAL_STATE__ = {{ initial_state | safe }};
    </script>
    <script src="{{ url_for('static', path='bundle.js') }}"></script>
</body>
</html>
"#;
    fs::write(
        project_root.join("templates/base.jinja2"),
        base_template_content,
    )
    .context("Failed to write templates/base.jinja2")?;

    // (index.jinja2 content verified from previous step)
    let index_template_content = r#"{% extends "base.jinja2" %}

{% block head_meta %}
    {{ super() }} {# Include base meta tags #}
    <meta name="description" content="Página principal de la aplicación ReactPyx.">
{% endblock head_meta %}

{% block content %}
    {# Just inherit the base content which includes the #app div and loader #}
    {{ super() }}
{% endblock content %}
"#;
    fs::write(
        project_root.join("templates/index.jinja2"),
        index_template_content,
    )
    .context("Failed to write templates/index.jinja2")?;

    // (404.jinja2 content verified from previous step)
    let fourohfour_template_content = r#"{% extends "base.jinja2" %}
{% block head_meta %}<meta name="description" content="Página no encontrada.">{% endblock %}
{% block head_extra %}
    <style>
        .error-container { display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 80vh; text-align: center; font-family: sans-serif; padding: 1rem; }
        .error-code { font-size: clamp(5rem, 20vw, 10rem); font-weight: bold; color: #e53e3e; margin: 0; line-height: 1; }
        .error-message { font-size: clamp(1.5rem, 5vw, 2.5rem); color: #4a5568; margin: 0.5rem 0 2rem; }
        .error-details { color: #718096; margin-bottom: 2rem; max-width: 500px; }
        .back-link { display: inline-block; color: #3182ce; text-decoration: none; font-weight: bold; padding: 0.75rem 1.5rem; border: 2px solid #3182ce; border-radius: 0.25rem; transition: all 0.2s ease; margin-top: 1rem; }
        .back-link:hover { background-color: #3182ce; color: white; }
    </style>
{% endblock %}
{% block content %}
    <div class="error-container">
        <h1 class="error-code">404</h1>
        <p class="error-message">Página No Encontrada</p>
        <p class="error-details">Lo sentimos, no pudimos encontrar la página que buscas. Puede que haya sido movida o eliminada.</p>
        <a href="/" class="back-link">Volver a la Página Principal</a>
    </div>
{% endblock %}
{# No incluir el script app.js en la página 404 #}
{% block body_scripts %}{% endblock body_scripts %}
"#;
    fs::write(
        project_root.join("templates/404.jinja2"),
        fourohfour_template_content,
    )
    .context("Failed to write templates/404.jinja2")?;

    // --- Create css_helper.py ---
    // (Content verified from previous step)
    let css_helper_content = r#"""src/css_helper.py - Utilidades para manejar CSS en ReactPyx"""

def combine_classes(*args):
    """
    Combina múltiples strings de clases CSS, ignorando valores None o vacíos.
    """
    return " ".join(filter(None, args))

def use_styles(styles_dict):
    """
    Prepara un diccionario de estilos para ser usado como prop 'style'.
    """
    return styles_dict # Devuelve el diccionario tal cual por ahora
"#;
    fs::write(project_root.join("src/css_helper.py"), css_helper_content)
        .context("Failed to write src/css_helper.py")?;

    // --- Final messages ---
    pb.finish_with_message(format!(
        "{} {}",
        "Project".green(),
        "created successfully!".green()
    ));
    println!(
        "\n{} {}",
        "Next steps:".cyan().bold(),
        "To get started:".cyan()
    );
    println!("  1. {}", format!("cd {}", project_name).yellow());
    println!(
        "  2. {}",
        "Consider creating a virtual environment: python -m venv venv && source venv/bin/activate"
            .dimmed()
    );
    println!(
        "  3. {}",
        "Install dependencies: pip install -e .".yellow().bold()
    ); // Assuming pyproject.toml is set up for editable install
       // println!("  3. {}", "reactpyx init --env development (If needed)".yellow()); // Comentado si 'init' no es necesario ahora
    println!(
        "  4. {}",
        "Run the development server: reactpyx run".yellow().bold()
    );
    println!("\n{}", "CSS Frameworks:".cyan());
    println!("  - {}", "reactpyx install tailwind".yellow());
    println!("  - {}", "reactpyx install bootstrap".yellow());
    println!(
        "{}",
        "  (Remember to add CDN links to templates/base.jinja2)".dimmed()
    );

    Ok(())
}
