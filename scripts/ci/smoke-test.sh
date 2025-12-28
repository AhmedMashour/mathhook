#!/bin/bash
# Post-publish smoke tests
# Verifies packages are installable from registries
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

VERSION="${1:-}"
TARGETS="${2:-all}"

if [[ -z "$VERSION" ]]; then
    log_error "Usage: $0 <version> [all|rust|python|node]"
    exit 1
fi

FAILED=0

smoke_test_rust() {
    log_info "Testing Rust crate installation..."

    local temp_dir
    temp_dir=$(make_temp_dir)
    trap "rm -rf '$temp_dir'" RETURN

    cd "$temp_dir"

    cat > Cargo.toml << EOF
[package]
name = "smoke-test"
version = "0.1.0"
edition = "2021"

[dependencies]
mathhook = "=$VERSION"
EOF

    cat > src/main.rs << 'EOF'
fn main() {
    println!("Smoke test passed!");
}
EOF

    mkdir -p src

    if poll_until 120 10 check_crate mathhook "$VERSION"; then
        if cargo check --quiet 2>/dev/null; then
            log_success "Rust crate mathhook@$VERSION is installable"
        else
            gh_error "Rust crate mathhook@$VERSION failed to compile"
            FAILED=1
        fi
    else
        gh_error "Rust crate mathhook@$VERSION not found on crates.io after 2 minutes"
        FAILED=1
    fi
}

smoke_test_python() {
    log_info "Testing Python package installation..."

    local temp_dir
    temp_dir=$(make_temp_dir)
    trap "rm -rf '$temp_dir'" RETURN

    cd "$temp_dir"
    python3 -m venv venv
    source venv/bin/activate

    if poll_until 120 10 check_pypi_package mathhook "$VERSION"; then
        if pip install "mathhook==$VERSION" --quiet 2>/dev/null; then
            if python -c "import mathhook; print('OK')" 2>/dev/null; then
                log_success "Python package mathhook==$VERSION is installable and importable"
            else
                gh_error "Python package mathhook==$VERSION installed but failed to import"
                FAILED=1
            fi
        else
            gh_error "Python package mathhook==$VERSION failed to install"
            FAILED=1
        fi
    else
        gh_error "Python package mathhook==$VERSION not found on PyPI after 2 minutes"
        FAILED=1
    fi

    deactivate
}

smoke_test_node() {
    log_info "Testing Node.js package installation..."

    local temp_dir
    temp_dir=$(make_temp_dir)
    trap "rm -rf '$temp_dir'" RETURN

    cd "$temp_dir"
    npm init -y --quiet 2>/dev/null

    if poll_until 120 10 check_npm_package mathhook-node "$VERSION"; then
        if npm install "mathhook-node@$VERSION" --quiet 2>/dev/null; then
            if node -e "require('mathhook-node'); console.log('OK')" 2>/dev/null; then
                log_success "Node.js package mathhook-node@$VERSION is installable and requireable"
            else
                gh_error "Node.js package mathhook-node@$VERSION installed but failed to require"
                FAILED=1
            fi
        else
            gh_error "Node.js package mathhook-node@$VERSION failed to install"
            FAILED=1
        fi
    else
        gh_error "Node.js package mathhook-node@$VERSION not found on npm after 2 minutes"
        FAILED=1
    fi
}

case "$TARGETS" in
    all)
        smoke_test_rust
        smoke_test_python
        smoke_test_node
        ;;
    rust)
        smoke_test_rust
        ;;
    python)
        smoke_test_python
        ;;
    node)
        smoke_test_node
        ;;
    *)
        log_error "Unknown target: $TARGETS"
        exit 1
        ;;
esac

gh_summary "## Smoke Test Results"
if [[ $FAILED -eq 0 ]]; then
    gh_summary "All smoke tests passed for version $VERSION"
    log_success "All smoke tests passed!"
else
    gh_summary "Some smoke tests failed for version $VERSION"
    gh_error "Some smoke tests failed!"
    exit 1
fi
