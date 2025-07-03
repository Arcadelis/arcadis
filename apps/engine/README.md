# Arcadis Game Engine

The Arcadis Game Engine is built on Rust and integrates with Stellar's Soroban smart contract platform.

## 🛠️ Setup

The setup process is automated through a script that handles all dependency installation and environment configuration.

### Requirements

- Linux, macOS, or WSL (Windows Subsystem for Linux) for Windows users
- An internet connection to download dependencies
- Basic terminal/command-line knowledge

### Quick Start

To set up the development environment:

```bash
# Navigate to the engine directory
cd apps/engine

# Run the setup script
./scripts/setup.sh
```

The script will automatically:

1. Install Rust via rustup (if not already installed)
2. Install Soroban CLI via cargo (if not already installed)
3. Add the wasm32-unknown-unknown target for WebAssembly compilation
4. Verify the environment with a test build

### Manual Setup

If you prefer to set up manually, you'll need to:

1. Install Rust:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Soroban CLI:

   ```bash
   cargo install stellar-cli
   ```

3. Add the WebAssembly target:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. Verify your setup:

   ```bash
   cargo build
   ```

## 📚 Development Resources

- [Rust Documentation](https://www.rust-lang.org/learn)
- [Stellar Development Documentation](https://developers.stellar.org/docs)
- [Soroban Documentation](https://soroban.stellar.org/)

## 🧪 Testing

Tests can be run with:

```bash
cargo test
```

## 🔄 Build

To build the project:

```bash
cargo build
```

For release builds:

```bash
cargo build --release
```
