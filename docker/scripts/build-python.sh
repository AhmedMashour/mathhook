#!/bin/bash
# MathHook Python Wheel Builder
# Builds wheels for all supported platforms with type stub generation
#
# Usage:
#   ./build-python.sh           # Full rebuild (default, for publishing)
#   ./build-python.sh --resume  # Skip existing wheels (for dev/debugging)
set -eo pipefail

PROJECT_ROOT="${PROJECT_ROOT:-/build}"
WHEEL_DIR="$PROJECT_ROOT/target/wheels"
RESUME_MODE=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --resume|-r) RESUME_MODE=true ;;
    esac
done

cd "$PROJECT_ROOT/crates/mathhook-python"

log_info() { echo ">>> $1"; }
log_error() { echo "ERROR: $1" >&2; }
log_warn() { echo "WARNING: $1" >&2; }
log_skip() { echo "⏭️  $1"; }

# Map rust target to wheel platform pattern
target_to_wheel_pattern() {
    local target="$1"
    case "$target" in
        x86_64-unknown-linux-gnu)   echo "*manylinux*_x86_64*.whl" ;;
        aarch64-unknown-linux-gnu)  echo "*manylinux*_aarch64*.whl" ;;
        x86_64-unknown-linux-musl)  echo "*musllinux*_x86_64*.whl" ;;
        aarch64-unknown-linux-musl) echo "*musllinux*_aarch64*.whl" ;;
        x86_64-apple-darwin)        echo "*macosx*_x86_64*.whl" ;;
        aarch64-apple-darwin)       echo "*macosx*_arm64*.whl" ;;
        x86_64-pc-windows-msvc)     echo "*win_amd64*.whl" ;;
        *) echo "" ;;
    esac
}

# Check if wheel already exists for target
wheel_exists() {
    local target="$1"
    local pattern=$(target_to_wheel_pattern "$target")
    [[ -n "$pattern" ]] && ls $WHEEL_DIR/$pattern &>/dev/null 2>&1
}

echo "=== Building Python wheels (abi3) for all platforms ==="
[[ "$RESUME_MODE" == "true" ]] && log_info "Resume mode (skipping existing wheels)" || log_info "Full rebuild (use --resume to skip existing)"

# Detect host architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  HOST_TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) HOST_TARGET="aarch64-unknown-linux-gnu" ;;
    arm64)   HOST_TARGET="aarch64-unknown-linux-gnu" ;;  # macOS arm64
    *)       log_error "Unsupported architecture: $ARCH"; exit 1 ;;
esac
log_info "Detected host architecture: $ARCH -> $HOST_TARGET"

# Generate binding code (MUST run before building)
log_info "Generating Python bindings..."
cd "$PROJECT_ROOT"
cargo run -p mathhook-binding-codegen -- generate --target python
cd "$PROJECT_ROOT/crates/mathhook-python"

# Generate Python type stubs
# NOTE: stub_gen must be built for the HOST without cross-compilation flags.
# It needs to link against the host's Python libraries.
log_info "Generating type stubs..."

# Build stub_gen without target flag to use native toolchain (avoids cross-linker issues)
STUB_GEN_PATH="$PROJECT_ROOT/target/release/stub_gen"

if cargo build --release --bin stub_gen -p mathhook-python 2>&1 | tee /tmp/stub_gen_build.log; then
    if [[ -f "$STUB_GEN_PATH" ]]; then
        "$STUB_GEN_PATH" || log_warn "Stub generation returned non-zero"
    else
        log_warn "stub_gen binary not found at $STUB_GEN_PATH after build"
    fi
else
    log_warn "stub_gen build failed, skipping stub generation"
fi

# Build function with error handling and resume support
build_target() {
    local target="$1"
    shift
    local extra_args=("$@")

    # Skip if wheel exists and in resume mode
    if [[ "$RESUME_MODE" == "true" ]] && wheel_exists "$target"; then
        log_skip "Skipping $target (wheel exists)"
        return 0
    fi

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

# Linux glibc builds - native for host arch, cross-compile with zig for other
if [[ "$HOST_TARGET" == "x86_64-unknown-linux-gnu" ]]; then
    # x86_64 host: native x86_64, cross-compile aarch64
    build_target "x86_64-unknown-linux-gnu" --compatibility manylinux2014 || FAILURES=$((FAILURES + 1))
    build_target "aarch64-unknown-linux-gnu" --zig --compatibility manylinux2014 || FAILURES=$((FAILURES + 1))
else
    # aarch64 host: native aarch64, cross-compile x86_64
    build_target "aarch64-unknown-linux-gnu" --compatibility manylinux2014 || FAILURES=$((FAILURES + 1))
    build_target "x86_64-unknown-linux-gnu" --zig --compatibility manylinux2014 || FAILURES=$((FAILURES + 1))
fi

# Linux musl builds (Alpine Linux, lightweight containers)
build_target "x86_64-unknown-linux-musl" --zig --compatibility musllinux_1_2 || FAILURES=$((FAILURES + 1))
build_target "aarch64-unknown-linux-musl" --zig --compatibility musllinux_1_2 || FAILURES=$((FAILURES + 1))

# macOS builds (cross-compile with zig + macOS SDK)
if [[ -d "/opt/MacOSX.sdk" ]] && command -v zig &>/dev/null; then
    # Pass framework paths via RUSTFLAGS (cargo-zigbuild doesn't respect CARGO_TARGET_*_RUSTFLAGS)
    MACOS_LINK_FLAGS="-C link-arg=-F/opt/MacOSX.sdk/System/Library/Frameworks -C link-arg=-L/opt/MacOSX.sdk/usr/lib"

    RUSTFLAGS="$MACOS_LINK_FLAGS" build_target "x86_64-apple-darwin" --zig || FAILURES=$((FAILURES + 1))
    RUSTFLAGS="$MACOS_LINK_FLAGS" build_target "aarch64-apple-darwin" --zig || FAILURES=$((FAILURES + 1))
else
    log_warn "macOS SDK or zig not found, skipping macOS builds"
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
