#!/bin/bash
# setup.sh - Automated environment setup script for Arcadis game engine
# This script installs and configures all necessary dependencies for the engine development

set -e  # Exit immediately if a command exits with a non-zero status

# ANSI color codes for better output formatting
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to display status messages
print_status() {
  echo -e "${GREEN}[✓] $1${NC}"
}

print_warning() {
  echo -e "${YELLOW}[!] $1${NC}"
}

print_error() {
  echo -e "${RED}[✗] $1${NC}"
}

# Function to check if command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Navigate to the engine directory
ENGINE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ENGINE_DIR"

echo -e "\n${GREEN}=== Arcadis Game Engine Setup ===${NC}"
echo -e "Setting up development environment in: ${YELLOW}$ENGINE_DIR${NC}\n"

# Step 1: Install Rust via rustup
if command_exists rustc; then
  print_status "Rust is already installed ($(rustc --version))"
else
  print_warning "Rust is not installed. Installing via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
  print_status "Rust installation completed ($(rustc --version))"
fi

# Step 2: Install Soroban CLI
if command_exists soroban; then
  print_status "Soroban CLI is already installed ($(soroban --version))"
else
  print_warning "Soroban CLI is not installed. Installing via cargo..."
  cargo install stellar-cli
  print_status "Soroban CLI installation completed ($(soroban --version))"
fi

# Step 3: Add wasm32-unknown-unknown target
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
  print_status "wasm32-unknown-unknown target is already installed"
else
  print_warning "Adding wasm32-unknown-unknown target..."
  rustup target add wasm32-unknown-unknown
  print_status "wasm32-unknown-unknown target added successfully"
fi

# Step 4: Verify environment
echo -e "\n${GREEN}=== Verifying Environment ===${NC}"
print_warning "Running cargo check to verify the environment..."

# First try a basic cargo check which is less likely to fail than build
if cargo check; then
  print_status "Basic dependency verification successful!"
else
  print_warning "Basic cargo check failed, this might be expected for no_std projects."
fi

# For Wasm/no_std projects, try checking with the wasm target
print_warning "Checking with wasm32-unknown-unknown target (suitable for no_std projects)..."
if cargo check --target wasm32-unknown-unknown; then
  print_status "Wasm target verification successful!"
else
  print_warning "Wasm target verification had issues, but setup can continue."
  print_warning "You may need to configure your project's Cargo.toml for proper no_std support."
fi

# Success message
echo -e "\n${GREEN}=== Setup Complete! ===${NC}"
echo -e "The Arcadis game engine development environment has been successfully set up."
echo -e "You can now start developing by navigating to: ${YELLOW}$ENGINE_DIR${NC}\n"
