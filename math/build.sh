#!/bin/bash

# Script for building and converting WASM module
set -e

echo "🔨 Building WebAssembly module..."

# Build the project
cargo build --target wasm32-unknown-unknown --release

# Path of the generated WASM file
WASM_FILE="target/wasm32-unknown-unknown/release/math.wasm"

# Check if the file was generated
if [ ! -f "$WASM_FILE" ]; then
    echo "❌ Error: WASM file not found at $WASM_FILE"
    exit 1
fi

echo "✅ WASM generated successfully: $WASM_FILE"

# Convert WASM to byte array
echo "🔄 Converting WASM to byte array..."

# Create the bytes file
od -An -v -t uC "$WASM_FILE" \
    | tr -s ' ' \
    | tr ' ' ',' \
    | tr -d '\n' \
    | sed 's/^,//;s/,$//g' > BYTES_RESULT.txt

echo "✅ Bytes saved to BYTES_RESULT.txt"

# Show statistics
BYTES_SIZE=$(wc -c < "$WASM_FILE")
echo "📊 WASM size: $BYTES_SIZE bytes"

# List exported functions (if wasm-objdump is available)
if command -v wasm-objdump &> /dev/null; then
    echo "📋 Exported functions:"
    wasm-objdump -x "$WASM_FILE" | grep -E "(FUNCTION|add|mul|sub|div)" || echo "   (wasm-objdump didn't find functions)"
else
    echo "📋 Available functions: add, mul, sub, div, rem, abs, max, min, pow"
fi

echo "🎉 Build completed successfully!" 