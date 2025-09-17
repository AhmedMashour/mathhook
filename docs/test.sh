#!/usr/bin/env bash
# MathHook Documentation Test Script
#
# This script builds the mathhook-book crate and runs mdbook test with the
# correct library paths, allowing code examples to access mathhook dependencies.

set -e

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building mathhook-book crate..."
cargo build --quiet

echo "Running mdbook tests with library dependencies..."
mdbook test -L target/debug/deps

echo "✅ Documentation tests complete!"
