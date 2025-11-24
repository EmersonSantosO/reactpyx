use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;

pub fn install_library(library: &str) -> Result<()> {
    println!("{} {}...", "Installing".green(), library.blue());

    match library {
        "tailwind" => {
            // Create necessary directories
            fs::create_dir_all("public/static")?;
            fs::create_dir_all("src/styles")?;

            // Create a basic integration using CDN instead of npm
            let cdn_integration = r#"<!-- Add to your HTML head -->
<link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
"#;
            fs::write("public/tailwind-cdn.html", cdn_integration)
                .context("Error creating tailwind CDN integration file")?;

            println!("{} {}", "✓".green(), "Tailwind CSS CDN integration created");

            // Create a sample configuration that doesn't rely on npm
            fs::write(
                "src/styles/tailwind-config.css",
                r#"/* Tailwind basic configuration */
/* Use these styles directly in your components */

.btn {
  @apply px-4 py-2 rounded;
}

.btn-primary {
  @apply bg-blue-500 text-white;
}
"#,
            )
            .context("Error creating tailwind config file")?;

            // Create a Python helper file for tailwind integration
            fs::write(
                "src/tailwind_helper.py",
                r#"""Tailwind CSS helper module for ReactPyx"""

def use_tailwind():
    """Import this function to enable Tailwind CSS in your component"""
    return {"class": "tailwind-enabled"}

def tw_classes(class_string):
    """Helper to manage Tailwind class names"""
    return {"className": class_string}
"#,
            )
            .context("Error creating tailwind helper file")?;
        }
        "bootstrap" => {
            // Create necessary directories
            fs::create_dir_all("public/static")?;
            fs::create_dir_all("src/styles")?;

            // Similar CDN approach for Bootstrap
            let cdn_integration = r#"<!-- Add to your HTML head -->
<link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
"#;
            fs::write("public/bootstrap-cdn.html", cdn_integration)
                .context("Error creating bootstrap CDN integration file")?;

            println!("{} {}", "✓".green(), "Bootstrap CDN integration created");

            // Create a Python helper file for bootstrap integration
            fs::write(
                "src/bootstrap_helper.py",
                r#"""Bootstrap helper module for ReactPyx"""

def use_bootstrap():
    """Import this function to enable Bootstrap in your component"""
    return {"class": "bootstrap-enabled"}

def bs_button(variant="primary", size="", text="Button"):
    """Create a Bootstrap button"""
    class_name = f"btn btn-{variant}"
    if size:
        class_name += f" btn-{size}"
    return {"className": class_name, "children": text}
"#,
            )
            .context("Error creating bootstrap helper file")?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unrecognized library: {}", library));
        }
    }

    // Update the main HTML template to include a note about CSS libraries
    let html_integration_note = r#"
<!-- Add this to your index.html to integrate CSS libraries -->
<!-- For example, to use Tailwind CSS: -->
<!-- <link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet"> -->
"#;

    // Check if the template directory exists
    if let Ok(metadata) = fs::metadata("templates/default/public/index.html") {
        if metadata.is_file() {
            let html_content = fs::read_to_string("templates/default/public/index.html")?;
            if !html_content.contains("Add this to your index.html to integrate CSS libraries") {
                let updated_content =
                    html_content.replace("</head>", &format!("{}\n</head>", html_integration_note));
                fs::write("templates/default/public/index.html", updated_content)
                    .context("Error updating HTML template")?;
            }
        }
    }

    println!(
        "{} {}",
        "Completed:".green(),
        format!("{} integration ready to use.", library).green()
    );
    println!(
        "{}",
        "Import the generated helper module in your components.".yellow()
    );

    // Output usage instructions
    match library {
        "tailwind" => {
            println!("\n{}", "Usage example:".cyan());
            println!("```python");
            println!("from src.tailwind_helper import use_tailwind, tw_classes");
            println!("");
            println!("def MyComponent(props):");
            println!("    # Enable Tailwind");
            println!("    use_tailwind()");
            println!("    ");
            println!("    # Now use Tailwind classes");
            println!("    return <div className=\"flex p-4 bg-blue-100 rounded\">");
            println!("        <h1 className=\"text-xl font-bold\">Hello Tailwind</h1>");
            println!("    </div>");
            println!("```");
        }
        "bootstrap" => {
            println!("\n{}", "Usage example:".cyan());
            println!("```python");
            println!("from src.bootstrap_helper import use_bootstrap, bs_button");
            println!("");
            println!("def MyComponent(props):");
            println!("    # Enable Bootstrap");
            println!("    use_bootstrap()");
            println!("    ");
            println!("    # Now use Bootstrap classes");
            println!("    return <div className=\"container mt-4\">");
            println!("        <h1 className=\"display-4\">Hello Bootstrap</h1>");
            println!("        {{bs_button(\"primary\", \"lg\", \"Click Me\")}}");
            println!("    </div>");
            println!("```");
        }
        _ => {}
    }

    Ok(())
}
