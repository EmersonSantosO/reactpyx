use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::time::Duration;

pub fn create_project(project_name: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["-", "\\", "|", "/"])
            .template("{spinner:.blue} Creating project: {msg}")?,
    );
    pb.set_message(project_name.to_string());

    // Create directories and initial files
    fs::create_dir_all(format!("{}/src/components", project_name))?;
    fs::create_dir_all(format!("{}/src/styles", project_name))?;
    fs::create_dir_all(format!("{}/public/static", project_name))?;

    // Create main file `main.pyx`
    let main_content = r#"from App import App

def MainApp():
    return App()
"#;
    fs::write(format!("{}/src/main.pyx", project_name), main_content)?;

    // Create App.pyx with CSS integration example
    let app_content = r#"from reactpyx import use_state, use_effect
from components.Header import Header
from components.StyledButton import StyledButton

def App():
    count, set_count = use_state("app", "count", 0)
    
    def increment():
        set_count(count + 1)
    
    use_effect(lambda: print("App rendered!"))
    
    return (
        <div className="container">
            <Header title="Welcome to ReactPyx" subtitle="A Python framework for building reactive UIs" />
            <main>
                <h2>Counter Example: {count}</h2>
                <StyledButton onClick={increment} text="Increment" />
            </main>
            <footer className="footer">
                <p>Created with ReactPyx</p>
            </footer>
        </div>
    )
"#;
    fs::write(format!("{}/src/App.pyx", project_name), app_content)?;

    // Create a Header component
    let header_content = r#"def Header(props):
    title = props.get('title', 'ReactPyx App')
    subtitle = props.get('subtitle', '')
    
    return (
        <header className="header">
            <h1>{title}</h1>
            {subtitle and <p>{subtitle}</p>}
        </header>
    )
"#;
    fs::write(
        format!("{}/src/components/Header.pyx", project_name),
        header_content,
    )?;

    // Create a StyledButton component that uses CSS
    let button_content = r#"from reactpyx import use_state
from src.css_helper import combine_classes

def StyledButton(props):
    hover, set_hover = use_state("styled_button", "hover", False)
    
    text = props.get('text', 'Button')
    variant = props.get('variant', 'primary')
    onClick = props.get('onClick', lambda: None)
    
    def handle_mouse_enter(event):
        set_hover(True)
    
    def handle_mouse_leave(event):
        set_hover(False)
    
    # Dynamic class names based on props and state
    button_class = combine_classes(
        "button",
        f"button-{variant}",
        hover and "button-hover"
    )
    
    return (
        <button 
            className={button_class}
            onClick={onClick}
            onMouseEnter={handle_mouse_enter}
            onMouseLeave={handle_mouse_leave}
        >
            {text}
        </button>
    )
"#;
    fs::write(
        format!("{}/src/components/StyledButton.pyx", project_name),
        button_content,
    )?;

    // Create a basic CSS file
    let css_content = r#"/* Main styles */
body {
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  margin: 0;
  padding: 0;
  color: #333;
  background-color: #f8fafc;
}

.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.header {
  border-bottom: 1px solid #e2e8f0;
  padding-bottom: 20px;
  margin-bottom: 20px;
}

.header h1 {
  margin-bottom: 10px;
  color: #2d3748;
}

.header p {
  color: #718096;
  margin-top: 0;
}

/* Button styles */
.button {
  padding: 10px 20px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s ease;
}

.button-primary {
  background-color: #4299e1;
  color: white;
}

.button-secondary {
  background-color: #edf2f7;
  color: #4a5568;
}

.button-hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.footer {
  margin-top: 40px;
  padding-top: 20px;
  border-top: 1px solid #e2e8f0;
  color: #718096;
  font-size: 14px;
}
"#;
    fs::write(format!("{}/src/styles/main.css", project_name), css_content)?;

    // Create config file
    let config_content = r#"{
    "entry": "./src/main.pyx",
    "entryFunction": "MainApp",
    "publicPath": "./public",
    "compilerOptions": {
        "minify": true,
        "sourceMaps": true
    }
}"#;
    fs::write(format!("{}/pyx.config.json", project_name), config_content)?;

    // Create basic HTML file
    let html_content = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>ReactPyx Application</title>
        <link rel="stylesheet" href="/static/styles.css">
        <meta name="description"
            content="A web application built with ReactPyx, a Python framework similar to JSX.">
        
        <!-- You can add CSS framework CDNs here -->
        <!-- For example, uncomment to use Tailwind CSS: -->
        <!-- <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet"> -->
    </head>
    <body>
        <div id="app">
            <!-- Content rendered by ReactPyx will be inserted here -->
        </div>
        <script src="/static/app.js" async></script>
    </body>
</html>"#;
    fs::write(format!("{}/public/index.html", project_name), html_content)?;

    // Create Python main.py server file
    let server_content = r#"from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse, FileResponse
from fastapi.staticfiles import StaticFiles
import os

app = FastAPI()

# Mount static files directory
app.mount("/static", StaticFiles(directory="public/static"), name="static")

@app.get("/favicon.ico")
async def favicon():
    return FileResponse("public/favicon.ico") if os.path.exists("public/favicon.ico") else ""

@app.get("/{full_path:path}")
async def serve_app(request: Request, full_path: str):
    # Serve the index.html for all routes (Single Page App pattern)
    index_path = os.path.join("public", "index.html")
    with open(index_path, "r", encoding="utf-8") as f:
        content = f.read()
    return HTMLResponse(content=content)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
"#;
    fs::write(format!("{}/main.py", project_name), server_content)?;

    // Create a basic css_helper.py
    let css_helper_content = r#"""CSS helper module for ReactPyx"""

def use_styles(styles_dict):
    """Helper function to use inline styles"""
    return {k: {"style": v} for k, v in styles_dict.items()}

def combine_classes(*args):
    """Helper function to combine multiple class names"""
    return " ".join([cls for cls in args if cls])
"#;
    fs::write(
        format!("{}/src/css_helper.py", project_name),
        css_helper_content,
    )?;

    pb.finish_with_message(format!(
        "{} {}",
        "Project".green(),
        "created successfully!".green()
    ));

    println!(
        "\n{} {}",
        "Next steps:".cyan().bold(),
        "To get started with your new project:".cyan()
    );
    println!("  1. {}", format!("cd {}", project_name).yellow());
    println!("  2. {}", "reactpyx Init --env development".yellow());
    println!("  3. {}", "reactpyx Run".yellow());
    println!("\n{}", "To add CSS frameworks:".cyan());
    println!("  - {}", "reactpyx Install tailwind".yellow());
    println!("  - {}", "reactpyx Install bootstrap".yellow());

    Ok(())
}
