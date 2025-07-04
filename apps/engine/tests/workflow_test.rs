// Integration test for the CI workflow
// Since this is an integration test, it has access to std
use std::process::Command;

/// This test simulates the CI workflow locally
/// Run with: cargo test --test workflow_test -- --ignored
#[test]
#[ignore] // Ignored by default as it's a longer-running test
fn test_ci_workflow() {
    // Get the project root directory
    let project_dir = std::env::current_dir().expect("Failed to get current directory");

    // Print info
    println!("Running workflow test from: {}", project_dir.display());
    println!("This test simulates the CI workflow steps locally");

    // Check formatting
    println!("Step 1: Checking code formatting...");
    let fmt_status = Command::new("cargo")
        .args(["fmt", "--", "--check"])
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run cargo fmt");
    assert!(fmt_status.success(), "Code formatting check failed");
    println!("✅ Code formatting check passed");

    // Build with native target
    println!("Step 2: Building with native target...");
    let build_status = Command::new("cargo")
        .args(["build"])
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run cargo build");
    assert!(build_status.success(), "Native build failed");
    println!("✅ Native build passed");

    // We can't easily test wasm32 build in an integration test as it would require
    // the correct target to be installed, so we'll check if it's available
    println!("Step 3: Checking if wasm32 target is available...");
    let wasm_check = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .current_dir(&project_dir)
        .output()
        .expect("Failed to check wasm32 target");
    
    let output = String::from_utf8_lossy(&wasm_check.stdout);
    if output.contains("wasm32-unknown-unknown") {
        println!("wasm32-unknown-unknown target is available, attempting build...");
        let wasm_status = Command::new("cargo")
            .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
            .current_dir(&project_dir)
            .status()
            .expect("Failed to run wasm32 build");
        assert!(wasm_status.success(), "Wasm32 build failed");
        println!("✅ Wasm32 build passed");
    } else {
        println!("⚠️ wasm32-unknown-unknown target not installed, skipping wasm build test");
        println!("  To install: rustup target add wasm32-unknown-unknown");
    }

    // Check if soroban CLI is available
    println!("Step 4: Checking if Soroban CLI is available...");
    let soroban_check = Command::new("which")
        .arg("soroban")
        .status();
    
    // Build Stellar contract if soroban is available
    if soroban_check.is_ok() && soroban_check.unwrap().success() {
        println!("Soroban CLI found, attempting to build contract...");
        let soroban_status = Command::new("soroban")
            .args(["contract", "build"])
            .current_dir(&project_dir)
            .status()
            .expect("Failed to run soroban contract build");
        assert!(soroban_status.success(), "Soroban contract build failed");
        println!("✅ Soroban contract build passed");
    } else {
        println!("⚠️ Soroban CLI not found, skipping contract build test");
        println!("  To install: cargo install --locked soroban-cli");
    }

    // Run tests
    println!("Step 5: Running unit tests...");
    let test_status = Command::new("cargo")
        .args(["test", "--lib"]) // Only run lib tests, not this integration test again
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run cargo test");
    assert!(test_status.success(), "Tests failed");
    println!("✅ Unit tests passed");

    println!("\n🎉 Workflow test completed successfully!");
    println!("All CI steps passed locally. Your code should pass the CI pipeline.");
}
