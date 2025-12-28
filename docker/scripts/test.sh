#!/bin/bash
# MathHook Test Suite
# Runs formatting, linting, and tests
set -eo pipefail

echo "=== Running MathHook test suite ==="

echo ">>> Checking formatting..."
cargo fmt --all -- --check

echo ">>> Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo ">>> Running tests..."
cargo test --all-features --workspace

echo "=== All tests passed ==="
