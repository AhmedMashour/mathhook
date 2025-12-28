#!/bin/bash
# MathHook Node.js Binding Builder
# Builds native bindings for all supported platforms
set -eo pipefail

PROJECT_ROOT="${PROJECT_ROOT:-/build}"
cd "$PROJECT_ROOT/crates/mathhook-node"

log_info() { echo ">>> $1"; }
log_error() { echo "ERROR: $1" >&2; }
log_warn() { echo "WARNING: $1" >&2; }

echo "=== Building Node.js native bindings ==="

# Generate binding code (MUST run before building)
log_info "Generating Node.js bindings..."
cd "$PROJECT_ROOT"
cargo run -p mathhook-binding-codegen -- generate --target node
cd "$PROJECT_ROOT/crates/mathhook-node"

log_info "Installing dependencies..."
npm install

# Build function with error handling
build_target() {
    local target="$1"
    shift
    local extra_args=("$@")

    log_info "Building for $target..."
    if npx napi build --platform --release --target "$target" "${extra_args[@]}"; then
        log_info "$target: success"
        return 0
    else
        log_error "$target: failed"
        return 1
    fi
}

# Track failures
FAILURES=0

# Linux builds
build_target "x86_64-unknown-linux-gnu" || FAILURES=$((FAILURES + 1))
build_target "x86_64-unknown-linux-musl" || FAILURES=$((FAILURES + 1))
build_target "aarch64-unknown-linux-gnu" --use-napi-cross || FAILURES=$((FAILURES + 1))

# macOS builds (cross-compile with zig)
if command -v zig &>/dev/null; then
    build_target "x86_64-apple-darwin" --zig || FAILURES=$((FAILURES + 1))
    build_target "aarch64-apple-darwin" --zig || FAILURES=$((FAILURES + 1))
else
    log_warn "Zig not found, skipping macOS cross-compilation"
fi

# Windows build
if [[ -d "/opt/xwin" ]] || command -v xwin &>/dev/null; then
    build_target "x86_64-pc-windows-msvc" || FAILURES=$((FAILURES + 1))
else
    log_warn "xwin not found, skipping Windows cross-compilation"
fi

echo "=== Node.js build complete ==="

# Verify outputs
if ls -la *.node index.js index.d.ts 2>/dev/null; then
    log_info "Build artifacts found"
else
    log_warn "Expected build artifacts not found"
fi

if [[ $FAILURES -gt 0 ]]; then
    log_warn "$FAILURES target(s) failed to build"
    exit 1
fi
