#!/bin/bash
# MathHook Rust Builder
# Builds and validates the core Rust libraries
set -eo pipefail

echo "=== Building Rust libraries ==="
cargo clippy -p mathhook-macros --release --workspace -- -D warnings
cargo clippy -p mathhook-core --release --workspace -- -D warnings
echo "=== Rust build complete ==="
