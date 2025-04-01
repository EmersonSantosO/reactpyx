use anyhow::Result;

// This test simply verifies that the project compiles correctly
#[test]
fn test_compilation() -> Result<()> {
    // If the test runs, it means the project compiles
    println!("✅ Project compiles correctly");
    Ok(())
}
