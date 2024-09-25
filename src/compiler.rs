// core_reactpyx/src/compiler.rs

use crate::component_parser::ComponentParser;
use anyhow::Result;
use convert_case::{Case, Casing};
use inflector::cases::snakecase::to_snake_case;
use log::{error, info};
use notify::{watcher, RecursiveMode, Watcher};
use quote::quote;
use std::error::Error;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use syn::{parse_str, ItemFn};
use tree_sitter::{Node, Parser};
use tree_sitter_python::language as python_language;
/// Compila todos los archivos `.pyx` en el proyecto.
/// Retorna una tupla con los archivos compilados y una lista de errores.
pub async fn compile_all_pyx(
    project_root: &str,
    config_path: &str,
) -> Result<(Vec<String>, Vec<(String, String)>), Box<dyn Error>> {
    let components_dir = Path::new(project_root).join("src").join("components");
    let mut parser = ComponentParser::new()?;
    let components = parser.detect_components_in_directory(components_dir.to_str().unwrap())?;

    info!(
        "Iniciando la compilación con los componentes: {:?}",
        components
    );

    let mut compiled_files = Vec::new();
    let mut errors = Vec::new();

    for component in components {
        let file_path = Path::new(project_root)
            .join("src")
            .join("components")
            .join(format!("{}.pyx", component));

        if file_path.exists() {
            match compile_pyx_file_to_python(&file_path) {
                Ok(python_code) => {
                    compiled_files.push(file_path.to_str().unwrap().to_string());
                    info!("Compilado exitosamente: {:?}", file_path);
                    let output_path = Path::new(project_root)
                        .join("build")
                        .join(format!("{}.py", component));
                    fs::write(output_path, python_code)?;
                }
                Err(e) => {
                    errors.push((file_path.to_str().unwrap().to_string(), e.to_string()));
                    error!("Error compilando {:?}: {}", file_path, e);
                }
            }
        } else {
            errors.push((
                file_path.to_str().unwrap().to_string(),
                "Archivo no encontrado.".to_string(),
            ));
            error!("Error compilando {:?}: Archivo no encontrado.", file_path);
        }
    }

    Ok((compiled_files, errors))
}

/// Función auxiliar para compilar un archivo `.pyx` a Python.
pub fn compile_pyx_file_to_python(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let source_code = fs::read_to_string(file_path)?;

    // Ejemplo: parsear funciones y renombrarlas
    let syntax_tree = syn::parse_file(&source_code)?;

    let mut transformed_functions = Vec::new();

    for item in syntax_tree.items {
        if let syn::Item::Fn(mut func) = item {
            // Renombrar la función
            func.sig.ident =
                syn::Ident::new(&format!("{}_py", func.sig.ident), func.sig.ident.span());

            // Generar el código de la función transformada
            let transformed_code = quote! {
                #func
            };

            transformed_functions.push(transformed_code.to_string());
        }
    }

    // Combinar todas las funciones transformadas
    Ok(transformed_functions.join("\n"))
}

/// Compila PyX a JavaScript y guarda el resultado en `output_dir`.
pub async fn compile_pyx_to_js(
    entry_file: &str,
    config_path: &str,
    output_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let entry_path = Path::new(entry_file);
    if !entry_path.exists() {
        return Err(format!("El archivo de entrada {} no existe", entry_file).into());
    }

    let source_code = fs::read_to_string(entry_path)?;
    let transformed_js = source_code
        .replace("<", "createElement(")
        .replace("/>", ");")
        .replace(">", ", []")
        .replace("</", ");");

    let output_file = Path::new(output_dir).join("output.js");
    fs::write(output_file, transformed_js)?;

    info!("Archivo JavaScript generado: {:?}", output_file);
    Ok(())
}

/// Monitorea los archivos en el directorio `src/components` para recompilarlos automáticamente cuando cambien.
pub fn watch_for_changes(project_root: &str, config_path: &str) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2))?;
    let components_dir = Path::new(project_root).join("src").join("components");

    watcher.watch(components_dir, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(_) => {
                // Recompilar los archivos cuando haya cambios
                let _ = compile_all_pyx(project_root, config_path);
            }
            Err(e) => error!("Error al observar los archivos: {:?}", e),
        }
    }
}

/// Actualiza la aplicación FastAPI con el nuevo código.
pub async fn update_application(
    module_name: &str,
    code: &str,
    entry_function: &str,
    project_root: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let app_py_path = project_root.join("app.py");

    // Genera el código de la aplicación FastAPI
    let app_code = format!(
        r#"
import sys
import os
sys.path.append(os.path.abspath("."))
from {} import {}

app = {}()
    "#,
        module_name, entry_function, entry_function
    );

    // Escribe el código en app.py
    fs::write(app_py_path, app_code)?;

    // Recarga el módulo en Python (si es necesario)
    // ...

    Ok(())
}
fn transform_jsx(jsx_code: &str) -> Result<String, Box<dyn Error>> {
    let mut parser = Parser::new();
    parser
        .set_language(python_language())
        .expect("Error al configurar el lenguaje Python para Tree-sitter");
    let tree = parser
        .parse(jsx_code, None)
        .ok_or("Error al parsear el código JSX")?;
    let mut cursor = tree.root_node().walk();

    let mut output = String::new();
    let mut indent_level = 0;

    for child in tree.root_node().children(&mut cursor) {
        match child.kind() {
            "expression_statement" => {
                let expression = child.child_by_field_name("expression").unwrap();
                if expression.kind() == "parenthesized_expression" {
                    let inner_expression = expression.child(1).unwrap();
                    if inner_expression.kind() == "jsx_element" {
                        output.push_str(&" ".repeat(indent_level * 4));
                        output.push_str(&transform_jsx_element(
                            inner_expression,
                            jsx_code,
                            &mut indent_level,
                        )?);
                        output.push('\n');
                    } else {
                        output.push_str(&" ".repeat(indent_level * 4));
                        output.push_str(expression.utf8_text(jsx_code.as_bytes()).unwrap());
                        output.push('\n');
                    }
                }
            }
            "function_definition" => {
                indent_level += 1;
                output.push_str(&" ".repeat(indent_level * 4));
                output.push_str(child.utf8_text(jsx_code.as_bytes()).unwrap());
                output.push('\n');
            }
            "block" => {
                indent_level += 1;
                output.push_str(&" ".repeat(indent_level * 4));
                output.push('{');
                output.push('\n');
            }
            "}" => {
                indent_level -= 1;
                output.push_str(&" ".repeat(indent_level * 4));
                output.push('}');
                output.push('\n');
            }
            _ => {}
        }
    }

    Ok(output)
}

fn transform_jsx_element(
    node: Node,
    source_code: &str,
    indent_level: &mut usize,
) -> Result<String, Box<dyn Error>> {
    let tag_name = node
        .child_by_field_name("opening_element")
        .unwrap()
        .child(1)
        .unwrap()
        .utf8_text(source_code.as_bytes())
        .unwrap();

    let mut props = String::new();
    if let Some(attributes) = node
        .child_by_field_name("opening_element")
        .unwrap()
        .child_by_field_name("attributes")
    {
        props.push('{');
        for attribute in attributes.children(&mut attributes.walk()) {
            if attribute.kind() == "jsx_attribute" {
                let name = attribute
                    .child_by_field_name("name")
                    .unwrap()
                    .utf8_text(source_code.as_bytes())
                    .unwrap();

                // Convertir a snake_case para eventos usando to_snake_case
                let name = if name.starts_with("on") {
                    to_snake_case(&name[2..]) // Convertimos a snake_case
                } else {
                    name
                };

                let value_node = attribute.child_by_field_name("value").unwrap();
                let value = match value_node.kind() {
                    "jsx_expression" => {
                        let expression = value_node.child(1).unwrap();
                        format!(
                            "{}", // No convertimos a str directamente
                            expression.utf8_text(source_code.as_bytes()).unwrap()
                        )
                    }
                    "true" => "True".to_string(),
                    "false" => "False".to_string(),
                    _ => format!(
                        "\"{}\"",
                        value_node.utf8_text(source_code.as_bytes()).unwrap()
                    ),
                };
                props.push_str(&format!("\"{}\": {}, ", name, value));
            }
        }
        if props.len() > 2 {
            // Eliminar la coma y el espacio al final si hay propiedades
            props.pop();
            props.pop();
        }
        props.push('}');
    } else {
        props.push_str("{}"); // Agregar llaves vacías si no hay propiedades
    }

    let mut children = Vec::new();
    for child in node.children(&mut node.walk()) {
        if child.kind() == "jsx_child" {
            children.push(transform_jsx_child(child, source_code, indent_level)?);
        }
    }

    let children_str = if !children.is_empty() {
        format!(", {}", children.join(", "))
    } else {
        String::new()
    };

    Ok(format!(
        "createElement(\"{}\", {}{})",
        tag_name, props, children_str
    ))
}

fn transform_jsx_child(
    node: Node,
    source_code: &str,
    indent_level: &mut usize,
) -> Result<String, Box<dyn Error>> {
    match node.child(0).unwrap().kind() {
        "jsx_element" => {
            *indent_level += 1;
            let result = format!(
                "{}createElement({}, {})",
                " ".repeat(indent_level * 4),
                transform_jsx_element(node.child(0).unwrap(), source_code, indent_level)?,
                transform_jsx_children(node.child(0).unwrap(), source_code, indent_level)?
            );
            *indent_level -= 1;
            Ok(result)
        }
        "jsx_expression" => {
            let expression = node.child(0).unwrap().child(1).unwrap();
            Ok(format!(
                "{}createElement(\"span\", {{}}, {})", // Envolver la expresión en un span
                " ".repeat(indent_level * 4),
                expression.utf8_text(source_code.as_bytes()).unwrap()
            ))
        }
        "string" => {
            let text = node
                .child(0)
                .unwrap()
                .utf8_text(source_code.as_bytes())
                .unwrap()
                .trim(); // Recortar espacios en blanco
            Ok(format!("\"{}\"", text))
        }
        _ => Err("Tipo de nodo JSX no soportado".into()),
    }
}

fn transform_jsx_children(
    node: Node,
    source_code: &str,
    indent_level: &mut usize,
) -> Result<String, Box<dyn Error>> {
    let mut children = Vec::new();
    for child in node.children(&mut node.walk()) {
        if child.kind() == "jsx_child" {
            children.push(transform_jsx_child(child, source_code, indent_level)?);
        }
    }

    if !children.is_empty() {
        Ok(format!("[{}]", children.join(", ")))
    } else {
        Ok(String::new())
    }
}
// core_reactpyx/src/compiler.rs

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_transform_jsx_element() {
        let mut indent_level = 0;
        let source_code = r#"
            <div id="my-div" class="container" data-attr={some_data} onClick={lambda: print("Clicked!")}>
                <p>Hello, world!</p>
                {some_variable}
            </div>
        "#;
        let mut parser = Parser::new();
        parser
            .set_language(python_language())
            .expect("Error al configurar el lenguaje Python para Tree-sitter");
        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();
        let jsx_element = root_node.child(0).unwrap().child(1).unwrap();

        let expected_output = r#"createElement("div", {"id": "my-div", "class": "container", "data-attr": some_data, "on_click": lambda: print("Clicked!")}, createElement("p", {}, "Hello, world!"), createElement("span", {}, some_variable))"#;
        let actual_output =
            transform_jsx_element(jsx_element, source_code, &mut indent_level).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_transform_jsx_children() {
        let mut indent_level = 0;
        let source_code = r#"
            <div>
                <p>Hola</p>
                {variable}
                <span>Mundo</span>
            </div>
        "#;
        let mut parser = Parser::new();
        parser
            .set_language(python_language())
            .expect("Error al configurar el lenguaje Python para Tree-sitter");
        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();
        let jsx_element = root_node.child(0).unwrap().child(1).unwrap();

        let expected_output = r#"[createElement("p", {}, "Hola"), createElement("span", {}, variable), createElement("span", {}, "Mundo")]"#;
        let actual_output =
            transform_jsx_children(jsx_element, source_code, &mut indent_level).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    // ... [otras pruebas para transform_jsx, compile_pyx_to_python, etc.] ...
}
