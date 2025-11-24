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

    // --- Create directories ---
    fs::create_dir_all(project_root.join("src").join("components"))
        .context("Failed to create src/components directory")?;
    fs::create_dir_all(project_root.join("src").join("styles"))
        .context("Failed to create src/styles directory")?;
    fs::create_dir_all(project_root.join("public").join("static"))
        .context("Failed to create public/static directory")?;
    fs::create_dir_all(project_root.join("templates"))
        .context("Failed to create templates directory")?; // <-- VERIFICADO

    // --- Create .pyx files ---
    let main_pyx_content = r#"# src/main.pyx - Entry point
from App import App

def MainApp():
    # This function should return the root component instance
    return App()
"#;
    fs::write(project_root.join("src/main.pyx"), main_pyx_content)
        .context("Failed to write src/main.pyx")?;

    let app_pyx_content = r#"# src/App.pyx
from reactpyx import use_state, use_effect
# Asegúrate que la ruta de importación sea correcta según tu estructura
from components.Header import Header
from components.StyledButton import StyledButton

def App():
    count, set_count = use_state("app", "count", 0)

    def increment():
        set_count(count + 1)

    # Example effect hook
    use_effect(lambda: print("App component rendered or updated"))

    return (
        <div className="container">
            <Header title="Welcome to ReactPyx" subtitle="Built with Python, Rust, and JSX-like syntax!" />
            <main>
                <h2>Interactive Counter</h2>
                <p>Current count: {count}</p>
                <StyledButton onClick={increment} text="Click Me!" variant="primary" />
            </main>
            <footer className="footer">
                <p>Powered by ReactPyx</p>
            </footer>
            {/* Example of scoped styles within a component */}
            <style>
              .container {{ max-width: 960px; margin: 2rem auto; padding: 0 1rem; font-family: sans-serif; }}
              main {{ background-color: #fff; padding: 1.5rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }}
              h2 {{ color: #333; margin-top: 0; }}
              .footer {{ margin-top: 2rem; border-top: 1px solid #e2e8f0; padding-top: 1rem; color: #a0aec0; text-align: center; font-size: 0.9em; }}
            </style>
        </div>
    )
"#;
    fs::write(project_root.join("src/App.pyx"), app_pyx_content)
        .context("Failed to write src/App.pyx")?;

    let header_pyx_content = r#"# src/components/Header.pyx

def Header(props):
    title = props.get('title', 'Default Title')
    subtitle = props.get('subtitle', '')

    return (
        <header className="header">
            <h1>{title}</h1>
            {subtitle and <p>{subtitle}</p>}
            {/* Scoped styles for the header */}
            <style>
              .header {{ margin-bottom: 2rem; padding-bottom: 1rem; border-bottom: 1px solid #e2e8f0; }}
              .header h1 {{ margin: 0 0 0.25rem 0; color: #2d3748; font-size: 2em; }}
              .header p {{ margin: 0; color: #718096; font-size: 1.1em; }}
            </style>
        </header>
    )
"#;
    fs::write(
        project_root.join("src/components/Header.pyx"),
        header_pyx_content,
    )
    .context("Failed to write src/components/Header.pyx")?;

    let button_pyx_content = r#"# src/components/StyledButton.pyx
from reactpyx import use_state

# Simple helper if css_helper.py is not generated/imported
def combine_classes(*args):
    return " ".join(filter(None, args))

def StyledButton(props):
    # State to track hover (optional, for visual feedback)
    hover, set_hover = use_state("styled_button", "hover", False)

    # Get props with defaults
    text = props.get('text', 'Default Button')
    variant = props.get('variant', 'secondary') # Default to secondary
    onClick = props.get('onClick', lambda *args: None) # Default no-op function

    # Event handlers need to potentially accept an event argument from JS frontend
    def handle_mouse_enter(event=None):
        set_hover(True)

    def handle_mouse_leave(event=None):
        set_hover(False)

    # Combine CSS classes dynamically
    button_class = combine_classes(
        "button", # Base class
        f"button-{variant}", # Variant class (e.g., button-primary)
        hover and "button-hover" # Hover class
    )

    return (
        # Assign classes and event handlers
        <button
            className={button_class}
            onClick={onClick}
            onMouseEnter={handle_mouse_enter}
            onMouseLeave={handle_mouse_leave}
        >
            {text}
        </button>
        # Note: Scoped styles for the button itself are better placed in src/styles/main.css
        # or a dedicated button.css for better organization, but could be added here too.
    )
"#;
    fs::write(
        project_root.join("src/components/StyledButton.pyx"),
        button_pyx_content,
    )
    .context("Failed to write src/components/StyledButton.pyx")?;

    // --- Create base CSS file ---
    let css_content = r#"/* src/styles/main.css - Base styles for the project */
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
  margin: 0;
  padding: 0;
  background-color: #f8fafc; /* Light grey background */
  color: #1a202c; /* Dark text */
  line-height: 1.6;
}

/* Basic button styles - applied via className="button button-primary" etc */
.button {
  display: inline-block;
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  border: none;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 500;
  text-align: center;
  text-decoration: none;
  transition: background-color 0.2s ease, transform 0.1s ease, box-shadow 0.2s ease;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.button-primary {
  background-color: #4299e1; /* Blue */
  color: white;
}

.button-secondary {
  background-color: #e2e8f0; /* Light Gray */
  color: #4a5568; /* Dark Gray */
  border: 1px solid #cbd5e0;
}

/* Hover effects */
.button:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.button-primary:hover {
  background-color: #3182ce; /* Darker Blue */
}

.button-secondary:hover {
  background-color: #cbd5e0; /* Darker Gray */
}

/* Add other global styles or utility classes here */
.container { /* Example */
  max-width: 960px; margin: 2rem auto; padding: 0 1rem;
}

```

"#;
    fs::write(project_root.join("src/styles/main.css"), css_content)
        .context("Failed to write src/styles/main.css")?;

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

    // --- Create main.py (using Jinja2) ---
    // (Content verified from previous step)
    let server_content = r#"# main.py - Servidor FastAPI para la aplicación ReactPyx
from fastapi import FastAPI, Request, HTTPException
from fastapi.responses import HTMLResponse, FileResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
import os
import sys

app = FastAPI(title="ReactPyxApp", version="0.1.0")

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
# sys.path.append(BASE_DIR) # Solo si necesitas importar desde aquí

templates_dir = os.path.join(BASE_DIR, "templates")
if not os.path.isdir(templates_dir):
    script_dir = os.path.dirname(os.path.realpath(__file__))
    templates_dir = os.path.join(script_dir, "templates")
    if not os.path.isdir(templates_dir):
        print(f"FATAL: Directory 'templates' not found near {script_dir}", file=sys.stderr)
        sys.exit(1)
templates = Jinja2Templates(directory=templates_dir)

static_dir = os.path.join(BASE_DIR, "public", "static")
os.makedirs(static_dir, exist_ok=True)
app.mount("/static", StaticFiles(directory=static_dir), name="static")

favicon_path = os.path.join(BASE_DIR, "public", "favicon.ico")
@app.get("/favicon.ico", include_in_schema=False)
async def favicon():
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
        # Note: This assumes src/main.pyx compiles to build/main.py
        import main as app_main
        
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
