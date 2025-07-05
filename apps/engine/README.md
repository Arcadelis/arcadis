# Arcadis Game Engine

A high-performance blockchain-based game engine built with Rust and Soroban smart contracts, designed to power decentralized gaming experiences on the Stellar network.

## Overview

The Arcadis game engine combines the performance of Rust with the security and decentralization of Stellar smart contracts. This engine provides developers with the tools to create immersive blockchain games featuring:

### Key Features

- **🔗 Smart Contract Integration**: Seamless interaction with Stellar contracts for game logic
- **⚡ High Performance**: Rust-powered engine optimized for gaming workloads  
- **🌐 Decentralized Architecture**: Blockchain-based game state management
- **👨‍💻 Developer-Friendly**: Comprehensive tooling and clear APIs

## Prerequisites

Before setting up the Arcadis game engine, ensure you have the following system requirements:

### System Requirements

- **Operating System**: Linux, macOS, or Windows (with WSL2 recommended)
- **Memory**: Minimum 4GB RAM (8GB recommended)
- **Storage**: At least 2GB free space
- **Network**: Internet connection for downloading dependencies

## Installation

### 1. Install Rust

First, install Rust using rustup (the official Rust installer):

```bash
# Download and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Install Stellar CLI

Install the Stellar CLI tool for smart contract development:

```bash
# Install Stellar CLI
cargo install --locked stellar-cli

# Verify installation
stellar --version
```

### 3. Add WebAssembly Target

Add the WebAssembly compilation target required for Soroban contracts:

```bash
# Add wasm32 target for Soroban
rustup target add wasm32-unknown-unknown

# Verify target installation
rustup target list --installed | grep wasm32
```

### 4. Configure Stellar Network

Set up Stellar CLI for development with the testnet:

```bash
# Configure testnet network
stellar config network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Generate a test identity
stellar config identity generate --global alice

# Fund the test account (get test XLM)
stellar config identity fund alice --network testnet
```

## Running a Test Contract

Follow these steps to deploy and test a sample Soroban contract:

### 1. Create a Sample Contract

```bash
# Navigate to the contracts directory
cd contracts/

# Create a new contract
stellar contract init hello_world

# Navigate to the contract directory
cd hello_world/
```

### 2. Build the Contract

```bash
# Build the contract
stellar contract build

# The compiled contract will be in target/wasm32-unknown-unknown/release/
```

### 3. Deploy to Testnet

```bash
# Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet

# Note: Save the returned contract ID for later use
```

### 4. Invoke Contract Functions

```bash
# Invoke a contract function (example)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- hello --to world
```

## Directory Structure

```
apps/engine/
├── README.md                    # This file
├── Cargo.toml                   # Main workspace configuration
├── Cargo.lock                   # Dependency lock file
├── src/                         # Main engine source code
│   ├── lib.rs                   # Library entry point
│   ├── engine/                  # Core engine modules
│   │   ├── mod.rs               # Engine module declarations
│   │   ├── renderer.rs          # Rendering system
│   │   ├── physics.rs           # Physics engine
│   │   └── input.rs             # Input handling
│   ├── blockchain/              # Blockchain integration
│   │   ├── mod.rs               # Blockchain module declarations
│   │   ├── soroban_client.rs    # Soroban client wrapper
│   │   └── contract_manager.rs  # Contract management
│   └── utils/                   # Utility functions
│       ├── mod.rs               # Utility module declarations
│       └── helpers.rs           # Helper functions
├── contracts/                   # Soroban smart contracts
│   ├── game_logic/              # Core game logic contracts
│   ├── nft_items/               # NFT item contracts
│   └── marketplace/             # Marketplace contracts
├── examples/                    # Example games and demos
│   ├── basic_game/              # Simple game example
│   └── nft_showcase/            # NFT integration example
├── tests/                       # Test files
│   ├── integration/             # Integration tests
│   └── unit/                    # Unit tests
├── docs/                        # Additional documentation
│   ├── architecture.md          # Engine architecture
│   ├── api_reference.md         # API documentation
│   └── tutorials/               # Tutorial guides
└── scripts/                     # Build and deployment scripts
    ├── build.sh                 # Build script
    ├── deploy.sh                # Deployment script
    └── test.sh                  # Test runner script
```

## Stellar CLI Usage Examples

Here are common Stellar CLI commands you'll use during development:

### Network Management

```bash
# List configured networks
stellar config network ls

# Add a custom network
stellar config network add \
  --global local \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase "Standalone Network ; February 2017"
```

### Identity Management

```bash
# List identities
stellar config identity ls

# Generate new identity
stellar config identity generate --global bob

# Get identity address
stellar config identity address alice
```

### Contract Development

```bash
# Initialize new contract
stellar contract init my_contract

# Build contract
stellar contract build

# Install contract dependencies
stellar contract install --wasm target/wasm32-unknown-unknown/release/my_contract.wasm

# Deploy contract
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/my_contract.wasm --source alice --network testnet

# Invoke contract function
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- function_name --arg1 value1
```

### Testing and Debugging

```bash
# Run contract with local sandbox
stellar contract invoke --id <CONTRACT_ID> --source alice --network local -- test_function

# Inspect contract metadata
stellar contract inspect --wasm target/wasm32-unknown-unknown/release/my_contract.wasm

# Get contract events
stellar events --start-ledger 1000 --count 10 --network testnet
```

## Development Workflow

### Recommended Development Process

1. **🔧 Setup**  
   Follow the installation steps above

2. **📝 Contract Development**  
   Create and build Soroban contracts in the `contracts/` directory

3. **🎮 Engine Integration**  
   Implement game logic in the `src/` directory

4. **🧪 Testing**  
   Run tests using `cargo test` and contract-specific tests

5. **🚀 Deployment**  
   Deploy contracts to testnet/mainnet using provided scripts

## Getting Started

To start developing with the Arcadis game engine:

### Quick Start Steps

1. **Clone and Setup**  
   Clone the repository and navigate to `apps/engine/`

2. **Build the Engine**  
   Run `cargo build` to compile the engine

3. **Explore Examples**  
   Browse the `examples/` directory for sample implementations

4. **Study Documentation**  
   Review the `docs/` directory for detailed guides and API references

5. **Examine Contracts**  
   Check the `contracts/` directory for sample smart contracts

## Troubleshooting

### Common Issues

#### Rust Compilation Errors
- Ensure you have the latest stable Rust version: `rustup update`
- Verify wasm32 target is installed: `rustup target add wasm32-unknown-unknown`

#### Stellar CLI Issues
- Update to latest version: `cargo install --locked stellar-cli --force`
- Check network configuration: `stellar config network ls`

#### Contract Deployment Failures
- Verify account funding: `soroban config identity fund <identity> --network testnet`
- Check network connectivity and RPC endpoints

### Getting Help

- **📚 Documentation**: Check the `docs/` directory for detailed guides
- **💡 Examples**: Review `examples/` for working code samples
- **👥 Community**: Join the Stellar Discord for Soroban support
- **🐛 Issues**: Report bugs and feature requests on the project repository

## Contributing

We welcome contributions to the Arcadis game engine! Please review our contribution guidelines and submit pull requests for review.

## License

This project is licensed under the MIT License - see the LICENSE file for details.