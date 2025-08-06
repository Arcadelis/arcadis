# Testing Documentation for Arcadis Game Engine

## Overview

This document provides comprehensive guidelines for testing the Arcadis game engine, including unit tests, integration tests, and smart contract testing using the Soroban SDK. Our testing strategy ensures code quality, reliability, and proper functionality across all engine components.

## Testing Framework

The Arcadis game engine uses a multi-layered testing approach:

- **Unit Tests**: Test individual functions and modules in isolation
- **Integration Tests**: Test component interactions and workflows  
- **Contract Tests**: Verify Soroban smart contract functionality
- **CI/CD Tests**: Automated testing in the continuous integration pipeline

### Core Testing Tools

- **Rust Test Framework**: Built-in `cargo test` for unit and integration tests
- **Soroban SDK Testing**: Specialized testing utilities for smart contracts
- **Stellar CLI**: Contract deployment and interaction testing
- **GitHub Actions**: Automated CI/CD testing pipeline

## Project Testing Structure

```plaintext
apps/engine/
├── src/
│   └── lib.rs                 # Unit tests alongside source code
├── tests/
│   ├── integration/           # Integration tests
│   ├── workflow_test.rs       # CI workflow simulation
│   └── game.rs               # Game logic tests (when implemented)
├── docs/
│   └── testing.md            # This documentation
└── scripts/
    └── test.sh               # Test execution script
```

## Writing Unit Tests

### Basic Unit Test Structure

Unit tests in Rust are typically written in the same file as the code they test, within a `tests` module:

```rust
// src/lib.rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_function() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_overflow_protection() {
        let result = add(u64::MAX - 1, 1);
        assert_eq!(result, u64::MAX);
    }
}
