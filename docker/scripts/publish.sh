#!/bin/bash
# MathHook Publisher
# Publishes to crates.io, PyPI, and npm with proper security handling
set -eo pipefail

log_info() { echo ">>> $1"; }
log_error() { echo "ERROR: $1" >&2; }
log_warn() { echo "WARNING: $1" >&2; }

# Cleanup function for sensitive files and logs
cleanup() {
    log_info "Cleaning up sensitive files..."
    rm -f .npmrc npm/*/.npmrc /tmp/publish_*.log /tmp/npm_*.log /tmp/napi_*.log 2>/dev/null || true
}
trap cleanup EXIT

echo "=== Publishing to all registries ==="

# Validate required environment variables
missing_tokens=""
[[ -z "$CARGO_REGISTRY_TOKEN" ]] && missing_tokens="$missing_tokens CARGO_REGISTRY_TOKEN"
[[ -z "$PYPI_API_TOKEN" ]] && missing_tokens="$missing_tokens PYPI_API_TOKEN"
[[ -z "$NPM_TOKEN" ]] && missing_tokens="$missing_tokens NPM_TOKEN"

if [[ -n "$missing_tokens" ]]; then
    log_error "Missing required tokens:$missing_tokens"
    exit 1
fi

# Publish to crates.io with proper error handling
publish_crate() {
    local crate="$1"
    local version

    cd "/build/crates/$crate"
    version=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')

    log_info "Publishing $crate@$version to crates.io..."

    # Use environment variable instead of command line for security
    if CARGO_REGISTRY_TOKEN="$CARGO_REGISTRY_TOKEN" cargo publish --allow-dirty 2>&1 | tee /tmp/publish_$crate.log; then
        log_info "$crate: published successfully"
        return 0
    else
        if grep -q "already exists" /tmp/publish_$crate.log; then
            log_info "$crate@$version: already published (skipping)"
            return 0
        else
            log_error "$crate: publish failed"
            cat /tmp/publish_$crate.log
            return 1
        fi
    fi
}

# Wait for crates.io index with verification
wait_for_crate() {
    local crate="$1"
    local version="$2"
    local max_wait=120
    local elapsed=0

    log_info "Waiting for $crate@$version to appear in crates.io index..."
    while [[ $elapsed -lt $max_wait ]]; do
        if cargo search "$crate" 2>/dev/null | grep -q "^$crate = \"$version\""; then
            log_info "$crate@$version indexed"
            return 0
        fi
        sleep 10
        elapsed=$((elapsed + 10))
        log_info "Waiting... ($elapsed/$max_wait seconds)"
    done
    log_warn "Timeout waiting for $crate@$version, continuing anyway"
}

log_info "Publishing to crates.io..."
MACROS_VERSION=$(grep '^version' /build/crates/mathhook-macros/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')

if publish_crate mathhook-macros; then
    wait_for_crate mathhook-macros "$MACROS_VERSION"
fi

CORE_VERSION=$(grep '^version' /build/crates/mathhook-core/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')

if publish_crate mathhook-core; then
    wait_for_crate mathhook-core "$CORE_VERSION"
fi

publish_crate mathhook || log_warn "mathhook publish failed, continuing with other registries"

# Publish to PyPI
log_info "Publishing to PyPI..."
if MATURIN_PYPI_TOKEN="$PYPI_API_TOKEN" maturin upload --skip-existing /build/target/wheels/*.whl; then
    log_info "PyPI: published successfully"
else
    log_warn "PyPI: publish failed or already exists"
fi

# Publish to npm with secure token handling
log_info "Publishing to npm..."
cd /build/crates/mathhook-node

# Configure npm authentication
npm config set //registry.npmjs.org/:_authToken "$NPM_TOKEN"

# Prepare npm packages (creates dirs + copies .node files)
log_info "Preparing npm packages..."
npx napi prepublish -t npm

# Publish platform packages
for dir in npm/*/; do
    if [[ -d "$dir" ]]; then
        pkg_name=$(basename "$dir")
        log_info "Publishing npm platform package: $pkg_name"
        if npm publish --access public -C "$dir" 2>&1 | tee /tmp/npm_$pkg_name.log; then
            log_info "$pkg_name: published"
        else
            if grep -q "already exists" /tmp/npm_$pkg_name.log; then
                log_info "$pkg_name: already published (skipping)"
            else
                log_warn "$pkg_name: publish failed"
            fi
        fi
    fi
done

sleep 15

# Publish main package
log_info "Publishing main npm package..."
if npm publish --access public 2>&1 | tee /tmp/npm_main.log; then
    log_info "mathhook-node: published"
else
    if grep -q "already exists" /tmp/npm_main.log; then
        log_info "mathhook-node: already published (skipping)"
    else
        log_warn "mathhook-node: publish failed"
    fi
fi

echo "=== Publishing complete ==="
