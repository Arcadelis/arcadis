#!/bin/bash
set -e  # Exit immediately if a command exits with non-zero status

# Colors for terminal output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Go to project directory
cd "$(dirname "$0")/.."
echo -e "${BLUE}Running workflow test from: $(pwd)${NC}"
echo -e "${BLUE}This script simulates the CI workflow steps locally${NC}"
echo ""

# Step 1: Check formatting
echo -e "${BLUE}Step 1: Checking code formatting...${NC}"
if cargo fmt -- --check; then
    echo -e "${GREEN}✅ Code formatting check passed${NC}"
else
    echo -e "${RED}❌ Code formatting check failed${NC}"
    exit 1
fi
echo ""

# Step 2: Build with native target
echo -e "${BLUE}Step 2: Building with native target...${NC}"
if cargo build; then
    echo -e "${GREEN}✅ Native build passed${NC}"
else
    echo -e "${RED}❌ Native build failed${NC}"
    exit 1
fi
echo ""

# Step 3: Check if wasm32 target is available
echo -e "${BLUE}Step 3: Checking if wasm32 target is available...${NC}"
if rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${BLUE}wasm32-unknown-unknown target is available, attempting build...${NC}"
    if cargo build --target wasm32-unknown-unknown --release; then
        echo -e "${GREEN}✅ Wasm32 build passed${NC}"
    else
        echo -e "${RED}❌ Wasm32 build failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️ wasm32-unknown-unknown target not installed, skipping wasm build test${NC}"
    echo -e "${YELLOW}  To install: rustup target add wasm32-unknown-unknown${NC}"
fi
echo ""

# Step 4: Check if Soroban CLI is available
echo -e "${BLUE}Step 4: Checking if Soroban CLI is available...${NC}"
if which soroban > /dev/null; then
    echo -e "${BLUE}Soroban CLI found, attempting to build contract...${NC}"
    if soroban contract build; then
        echo -e "${GREEN}✅ Soroban contract build passed${NC}"
    else
        echo -e "${RED}❌ Soroban contract build failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️ Soroban CLI not found, skipping contract build test${NC}"
    echo -e "${YELLOW}  To install: cargo install --locked soroban-cli${NC}"
fi
echo ""

# Step 5: Run tests (only lib tests, not this test again)
echo -e "${BLUE}Step 5: Running unit tests...${NC}"
if cargo test --lib; then
    echo -e "${GREEN}✅ Unit tests passed${NC}"
else
    echo -e "${RED}❌ Unit tests failed${NC}"
    exit 1
fi
echo ""

# Final success message
echo -e "${GREEN}🎉 Workflow test completed successfully!${NC}"
echo -e "${GREEN}All CI steps passed locally. Your code should pass the CI pipeline.${NC}"
