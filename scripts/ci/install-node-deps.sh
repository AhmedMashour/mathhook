#!/usr/bin/env bash
# Install Node.js native dependencies for cross-compilation
# Usage: install-node-deps.sh <target>
set -euo pipefail

TARGET="${1:-}"

sudo apt-get update

case "$TARGET" in
    aarch64-unknown-linux-gnu)
        sudo apt-get install -y gcc-aarch64-linux-gnu
        ;;
    *musl*)
        sudo apt-get install -y musl-tools
        ;;
esac
