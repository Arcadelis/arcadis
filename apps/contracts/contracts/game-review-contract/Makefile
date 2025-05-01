TARGET_DIR := ../../../target/
WASM_DIR := $(TARGET_DIR)wasm32-unknown-unknown/release/
WASM_BINDGEN_DIR := $(TARGET_DIR)wasm-bindgen/

WASM_FILE := $(WASM_DIR)game_review_contract.wasm
BINDGEN_WASM_FILE := $(WASM_BINDGEN_DIR)game_review_contract_bg.wasm

.PHONY: all
all: build test

.PHONY: build
build:
	cargo build --release --target wasm32-unknown-unknown

.PHONY: test
test:
	cargo test --no-default-features

clean:
	cargo clean
	rm -rf .soroban 