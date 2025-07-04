#![no_std]

extern crate alloc;

// Use wee_alloc as the global allocator for no_std
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /// This test simulates the CI workflow locally
    /// Run with: cargo test --test workflow_test -- --ignored
    #[test]
    #[ignore] // Ignored by default as it's a longer-running test
    fn workflow_test() {
        // Check formatting
        let fmt_status = Command::new("cargo")
            .args(["fmt", "--", "--check"])
            .status()
            .expect("Failed to run cargo fmt");
        assert!(fmt_status.success(), "Code formatting check failed");

        // Build with native target
        let build_status = Command::new("cargo")
            .args(["build"])
            .status()
            .expect("Failed to run cargo build");
        assert!(build_status.success(), "Native build failed");

        // Build with wasm32 target
        #[cfg(target_arch = "wasm32")]
        {
            let wasm_status = Command::new("cargo")
                .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
                .status()
                .expect("Failed to run wasm32 build");
            assert!(wasm_status.success(), "Wasm32 build failed");
        }

        // Check if soroban CLI is available
        let soroban_check = Command::new("which")
            .arg("soroban")
            .status();
        
        // Build Stellar contract if soroban is available
        if soroban_check.is_ok() && soroban_check.unwrap().success() {
            let soroban_status = Command::new("soroban")
                .args(["contract", "build"])
                .status()
                .expect("Failed to run soroban contract build");
            assert!(soroban_status.success(), "Soroban contract build failed");
        } else {
            println!("Skipping soroban contract build: soroban CLI not found");
        }

        println!("Workflow test passed successfully!");
    }
}
