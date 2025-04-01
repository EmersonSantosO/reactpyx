use anyhow::Result;

// Este test simplemente comprueba que el proyecto compila correctamente
#[test]
fn test_compilation() -> Result<()> {
    // Si el test se ejecuta, significa que el proyecto compila
    println!("✅ El proyecto compila correctamente");
    Ok(())
}
