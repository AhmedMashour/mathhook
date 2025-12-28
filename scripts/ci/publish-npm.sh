#!/bin/bash
# Publish Node.js packages to npm
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

NODE_PKG_DIR="${1:-crates/mathhook-node}"
DRY_RUN="${DRY_RUN:-false}"

if [[ ! -d "$NODE_PKG_DIR" ]]; then
    gh_error "Node package directory not found: $NODE_PKG_DIR"
    exit 1
fi

require_env NODE_AUTH_TOKEN

cd "$NODE_PKG_DIR"

VERSION=$(get_npm_version package.json)
log_info "Preparing npm packages for mathhook-node@$VERSION..."

# Create platform package directories
if ! npx napi create-npm-dirs --npm-dir npm 2>&1; then
    log_warn "npm dirs creation failed (may already exist)"
fi

# Copy binaries to platform packages
for node_file in *.node; do
    if [[ -f "$node_file" ]]; then
        platform=$(echo "$node_file" | sed 's/mathhook-node\.\(.*\)\.node/\1/')
        if [[ -d "npm/$platform" ]]; then
            cp "$node_file" "npm/$platform/"
            log_info "Copied $node_file to npm/$platform/"
        fi
    fi
done

if [[ "$DRY_RUN" == "true" ]]; then
    log_info "DRY RUN: Would publish the following packages:"
    npm pack --dry-run
    exit 0
fi

FAILED=0
LOG_FILE=$(make_temp_file)
trap 'rm -f "$LOG_FILE"' EXIT

# Publish platform packages
log_info "Publishing platform packages..."
for pkg_dir in npm/*/; do
    if [[ -d "$pkg_dir" ]]; then
        pkg_name=$(basename "$pkg_dir")
        log_info "Publishing $pkg_name..."

        if npm publish --access public -C "$pkg_dir" 2>&1 | tee "$LOG_FILE"; then
            log_success "$pkg_name published"
        elif grep -q "already exists" "$LOG_FILE"; then
            gh_notice "$pkg_name already published"
        else
            gh_error "$pkg_name publish failed"
            FAILED=1
        fi
    fi
done

# Wait for npm registry to index platform packages
log_info "Waiting for npm registry to index platform packages..."
sleep 15

# Publish main package
log_info "Publishing main package: mathhook-node..."
if npm publish --access public 2>&1 | tee "$LOG_FILE"; then
    log_success "mathhook-node@$VERSION published"
elif grep -q "already exists" "$LOG_FILE"; then
    gh_notice "mathhook-node@$VERSION already published"
else
    gh_error "mathhook-node publish failed"
    cat "$LOG_FILE" >&2
    FAILED=1
fi

if [[ $FAILED -ne 0 ]]; then
    log_error "Some npm packages failed to publish"
    exit 1
fi

log_success "npm publishing complete"
gh_summary "## npm Publish"
gh_summary "Published mathhook-node@$VERSION to npm"
