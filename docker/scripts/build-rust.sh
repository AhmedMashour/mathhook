#!/bin/bash
# MathHook Rust Builder
# Builds and validates the core Rust libraries
set -eo pipefail

echo "=== Building Rust libraries ==="
cargo build --release --workspace
cargo clippy --release --workspace -- -D warnings
echo "=== Rust build complete ==="
