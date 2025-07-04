# Arcadis Game Engine Setup

[![Engine CI](https://github.com/aryan/od-wave/arcadis/actions/workflows/engine-ci.yml/badge.svg)](https://github.com/aryan/od-wave/arcadis/actions/workflows/engine-ci.yml)

A no_std WebAssembly game engine built on the Stellar blockchain with Soroban smart contracts.

## Features

- Written in Rust with `no_std` support
- WebAssembly target for cross-platform compatibility
- Soroban smart contract integration

## Development

### Prerequisites

- Rust toolchain with wasm32 target: `rustup target add wasm32-unknown-unknown`
- Soroban CLI: `cargo install --locked soroban-cli`

### Building

```sh
# Build with native target
cargo build

# Build with WebAssembly target
cargo build --target wasm32-unknown-unknown --release

# Build Soroban contract
soroban contract build
```

### Testing

```sh
cargo test
```

## CI/CD

The project uses GitHub Actions for continuous integration and deployment. The pipeline:

- Checks code formatting with `cargo fmt`
- Builds the engine for both native and wasm32 targets
- Builds the Stellar contract
- Runs all tests

Pull requests must pass all CI checks before being merged.
