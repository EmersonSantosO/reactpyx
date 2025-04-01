use anyhow::Result;
use pyo3::prelude::*;
use std::path::Path;

// Función para testear la compatibilidad de bindings con Python
pub fn test_python_bindings() -> Result<()> {
    println!("Testeando bindings de Python...");
    
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;
        
        println!("Python version: {}", version);
        
        // Verificar que estamos en una versión compatible
        let ver_info = sys.getattr("version_info")?;
        let major: u8 = ver_info.getattr("major")?.extract()?;
        let minor: u8 = ver_info.getattr("minor")?.extract()?;
        
        if major == 3 && (8..=13).contains(&minor) {
            println!("✓ Versión de Python compatible (3.8-3.13)");
        } else {
            println!("⚠️ Versión de Python no probada extensivamente: {}.{}", major, minor);
        }
        
        println!("✓ Python bindings funcionando correctamente");
        
        // Probar la compatibilidad con la API de hooks
        let code = r#"
        from core_reactpyx import use_state, use_effect, use_effect_with_deps
        
        # Test básico (no ejecuta realmente el código, solo importa)
        print("✓ Todos los módulos importados correctamente")
        "#;
        
        py.run(code, None, None)?;
        Ok::<(), PyErr>(())
    })?;
    
    println!("✓ Todos los tests pasaron");
    Ok(())
}

// Verificar que los directorios de componentes existan
pub fn ensure_component_dirs(project_root: &str) -> Result<()> {
    let components_dir = Path::new(project_root).join("src").join("components");
    
    if !components_dir.exists() {
        std::fs::create_dir_all(&components_dir)?;
        println!("✓ Directorio de componentes creado: {:?}", components_dir);
    }
    
    Ok(())
}
