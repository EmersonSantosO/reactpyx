use anyhow::Result;
use pyo3::prelude::*;
use std::path::Path;

// Function to test Python bindings compatibility
pub fn test_python_bindings() -> Result<()> {
    println!("Testing Python bindings...");

    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        println!("Python version: {}", version);

        // Verify that we're using a compatible version
        let ver_info = sys.getattr("version_info")?;
        let major: u8 = ver_info.getattr("major")?.extract()?;
        let minor: u8 = ver_info.getattr("minor")?.extract()?;

        if major == 3 && (8..=13).contains(&minor) {
            println!("✓ Compatible Python version (3.8-3.13)");
        } else {
            println!(
                "⚠️ Python version not extensively tested: {}.{}",
                major, minor
            );
        }

        println!("✓ Python bindings working correctly");

        // Test hooks API compatibility
        let code = r#"
        from reactpyx import use_state, use_effect, use_effect_with_deps
        
        # Basic test (not actually running code, just importing)
        print("✓ All modules imported correctly")
        "#;

        py.run(code, None, None)?;
        Ok::<(), PyErr>(())
    })?;

    println!("✓ All tests passed");
    Ok(())
}

// Verify that component directories exist
pub fn ensure_component_dirs(project_root: &str) -> Result<()> {
    let components_dir = Path::new(project_root).join("src").join("components");

    if !components_dir.exists() {
        std::fs::create_dir_all(&components_dir)?;
        println!("✓ Components directory created: {:?}", components_dir);
    }

    Ok(())
}
