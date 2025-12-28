#!/bin/bash
# MathHook Python Wheel Builder
# Builds wheels for all supported platforms with type stub generation
set -eo pipefail

PROJECT_ROOT="${PROJECT_ROOT:-/build}"
cd "$PROJECT_ROOT/crates/mathhook-python"

log_info() { echo ">>> $1"; }
log_error() { echo "ERROR: $1" >&2; }
log_warn() { echo "WARNING: $1" >&2; }

echo "=== Building Python wheels (abi3) for all platforms ==="

# Generate binding code (MUST run before building)
log_info "Generating Python bindings..."
cd "$PROJECT_ROOT"
cargo run -p mathhook-binding-codegen -- generate --target python
cd "$PROJECT_ROOT/crates/mathhook-python"

# Generate Python type stubs
log_info "Generating type stubs..."
if cargo build --release --bin stub_gen -p mathhook-python 2>&1 | tee /tmp/stub_gen_build.log; then
    if [[ -f "$PROJECT_ROOT/target/release/stub_gen" ]]; then
        "$PROJECT_ROOT/target/release/stub_gen" || log_warn "Stub generation returned non-zero"
    else
        log_warn "stub_gen binary not found after build"
    fi
else
    log_warn "stub_gen build failed, skipping stub generation"
fi

# Build function with error handling
build_target() {
    local target="$1"
    shift
    local extra_args=("$@")

    log_info "Building for $target..."
    if maturin build --release --target "$target" "${extra_args[@]}"; then
        log_info "$target: success"
        return 0
    else
        log_error "$target: failed"
        return 1
    fi
}

# Track failures
FAILURES=0

# Linux builds (native in container)
build_target "x86_64-unknown-linux-gnu" --compatibility manylinux2014 || FAILURES=$((FAILURES + 1))
build_target "aarch64-unknown-linux-gnu" --zig --compatibility manylinux2014 || FAILURES=$((FAILURES + 1))

# macOS builds (cross-compile with zig)
if command -v zig &>/dev/null; then
    build_target "x86_64-apple-darwin" --zig || FAILURES=$((FAILURES + 1))
    build_target "aarch64-apple-darwin" --zig || FAILURES=$((FAILURES + 1))
else
    log_warn "Zig not found, skipping macOS cross-compilation"
fi

# Windows build (requires xwin for MSVC)
if [[ -d "/opt/xwin" ]] || command -v xwin &>/dev/null; then
    build_target "x86_64-pc-windows-msvc" || FAILURES=$((FAILURES + 1))
else
    log_warn "xwin not found, skipping Windows cross-compilation"
fi

echo "=== Python build complete ==="
log_info "Wheels in: $PROJECT_ROOT/target/wheels/"
ls -la "$PROJECT_ROOT/target/wheels/"*.whl 2>/dev/null || log_warn "No wheel files found"

if [[ $FAILURES -gt 0 ]]; then
    log_warn "$FAILURES target(s) failed to build"
    exit 1
fi
