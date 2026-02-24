#!/bin/bash
# MathHook Node.js Binding Builder
# Builds native bindings for all supported platforms
#
# Usage:
#   ./build-node.sh           # Full rebuild (default, for publishing)
#   ./build-node.sh --resume  # Skip existing bindings (for dev/debugging)
set -eo pipefail

PROJECT_ROOT="${PROJECT_ROOT:-/build}"
CRATE_DIR="$PROJECT_ROOT/crates/mathhook-node"
RESUME_MODE=false

# Parse arguments
for arg in "$@"; do
    case $arg in
        --resume|-r) RESUME_MODE=true ;;
    esac
done

cd "$PROJECT_ROOT/crates/mathhook-node"

log_info() { echo ">>> $1"; }
log_error() { echo "ERROR: $1" >&2; }
log_warn() { echo "WARNING: $1" >&2; }
log_skip() { echo "⏭️  $1"; }

# Map rust target to node binding filename pattern
target_to_binding_pattern() {
    local target="$1"
    case "$target" in
        x86_64-unknown-linux-gnu)   echo "linux-x64-gnu" ;;
        aarch64-unknown-linux-gnu)  echo "linux-arm64-gnu" ;;
        x86_64-apple-darwin)        echo "darwin-x64" ;;
        aarch64-apple-darwin)       echo "darwin-arm64" ;;
        x86_64-pc-windows-msvc)     echo "win32-x64-msvc" ;;
        *) echo "" ;;
    esac
}

# Check if binding already exists for target
# napi build outputs to crate root as: mathhook-node.<platform>.node
binding_exists() {
    local target="$1"
    local pattern=$(target_to_binding_pattern "$target")
    [[ -n "$pattern" ]] && [[ -f "$CRATE_DIR/mathhook-node.${pattern}.node" ]]
}

echo "=== Building Node.js native bindings ==="
[[ "$RESUME_MODE" == "true" ]] && log_info "Resume mode (skipping existing bindings)" || log_info "Full rebuild (use --resume to skip existing)"

# Detect host architecture
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  HOST_TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) HOST_TARGET="aarch64-unknown-linux-gnu" ;;
    arm64)   HOST_TARGET="aarch64-unknown-linux-gnu" ;;
    *)       log_error "Unsupported architecture: $ARCH"; exit 1 ;;
esac
log_info "Detected host architecture: $ARCH -> $HOST_TARGET"

# Generate binding code (MUST run before building)
log_info "Generating Node.js bindings..."
cd "$PROJECT_ROOT"
cargo run -p mathhook-binding-codegen -- generate --target node
cd "$PROJECT_ROOT/crates/mathhook-node"

log_info "Installing dependencies..."
npm install

# Build function with error handling and resume support
build_target() {
    local target="$1"
    shift
    local extra_args=("$@")

    # Skip if binding exists and in resume mode
    if [[ "$RESUME_MODE" == "true" ]] && binding_exists "$target"; then
        log_skip "Skipping $target (binding exists)"
        return 0
    fi

    log_info "Building for $target..."
    if npx napi build --platform --release --target "$target" "${extra_args[@]}"; then
        log_info "$target: success"
        return 0
    else
        log_error "$target: failed"
        return 1
    fi
}

# Build with napi-cross for glibc targets (need to avoid lld linker which their gcc doesn't support)
build_target_cross_glibc() {
    local target="$1"

    # Skip if binding exists and in resume mode
    if [[ "$RESUME_MODE" == "true" ]] && binding_exists "$target"; then
        log_skip "Skipping $target (binding exists)"
        return 0
    fi

    log_info "Building for $target (cross-glibc)..."
    # Use default linker, napi-cross gcc doesn't support -fuse-ld=lld
    if RUSTFLAGS="-C linker-flavor=gcc" npx napi build --platform --release --target "$target" --use-napi-cross; then
        log_info "$target: success"
        return 0
    else
        log_error "$target: failed"
        return 1
    fi
}



# Track failures
FAILURES=0

# Linux glibc builds - native for host, cross-compile for other
if [[ "$HOST_TARGET" == "x86_64-unknown-linux-gnu" ]]; then
    build_target "x86_64-unknown-linux-gnu" || FAILURES=$((FAILURES + 1))
    build_target_cross_glibc "aarch64-unknown-linux-gnu" || FAILURES=$((FAILURES + 1))
else
    build_target "aarch64-unknown-linux-gnu" || FAILURES=$((FAILURES + 1))
    build_target_cross_glibc "x86_64-unknown-linux-gnu" || FAILURES=$((FAILURES + 1))
fi


# macOS builds (cross-compile with cargo-zigbuild + macOS SDK)
build_target_macos_zig() {
    local target="$1"

    # Skip if binding exists and in resume mode
    if [[ "$RESUME_MODE" == "true" ]] && binding_exists "$target"; then
        log_skip "Skipping $target (binding exists)"
        return 0
    fi

    log_info "Building for $target (zigbuild)..."

    # Set macOS SDK paths for zig
    export SDKROOT="/opt/MacOSX.sdk"
    export MACOSX_DEPLOYMENT_TARGET="11.0"

    # Build using cargo-zigbuild directly
    if cargo zigbuild --release --target "$target" -p mathhook-node; then
        # Copy the built artifact to crate root (same location as napi build)
        local binding_name=$(target_to_binding_pattern "$target")
        local lib_name="libmathhook_node.dylib"
        local src_path="$PROJECT_ROOT/target/$target/release/$lib_name"
        local dest_path="$CRATE_DIR/mathhook-node.${binding_name}.node"

        if [[ -f "$src_path" ]]; then
            cp "$src_path" "$dest_path"
            log_info "$target: success"
            return 0
        else
            log_error "$target: built but artifact not found at $src_path"
            return 1
        fi
    else
        log_error "$target: failed"
        return 1
    fi
}

if [[ -d "/opt/MacOSX.sdk" ]] && command -v zig &>/dev/null && command -v cargo-zigbuild &>/dev/null; then
    build_target_macos_zig "x86_64-apple-darwin" || FAILURES=$((FAILURES + 1))
    build_target_macos_zig "aarch64-apple-darwin" || FAILURES=$((FAILURES + 1))
else
    log_warn "macOS SDK, zig, or cargo-zigbuild not found, skipping macOS builds"
fi

# Windows build
if [[ -d "/opt/xwin" ]]; then
    build_target "x86_64-pc-windows-msvc" || FAILURES=$((FAILURES + 1))
else
    log_warn "xwin not found, skipping Windows build"
fi

echo "=== Node.js build complete ==="

# Verify outputs
log_info "Build artifacts:"
ls -la "$CRATE_DIR"/mathhook-node.*.node 2>/dev/null || log_warn "No .node binaries found"

if [[ $FAILURES -gt 0 ]]; then
    log_warn "$FAILURES target(s) failed to build"
    exit 1
fi
