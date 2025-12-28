#!/bin/bash
# Validate versions across all package files match the git tag
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

TAG_VERSION="${1:-}"

if [[ -z "$TAG_VERSION" ]]; then
    log_error "Usage: $0 <version>"
    exit 1
fi

# Remove 'v' prefix if present
TAG_VERSION="${TAG_VERSION#v}"

log_info "Validating versions for tag: $TAG_VERSION"

# Extract versions using shared utilities
CARGO_VERSION=$("$SCRIPT_DIR/get-version.sh" workspace .)
PYTHON_VERSION=$("$SCRIPT_DIR/get-version.sh" python crates/mathhook-python) || PYTHON_VERSION=""
NODE_VERSION=$("$SCRIPT_DIR/get-version.sh" npm crates/mathhook-node) || NODE_VERSION=""

log_info "Cargo workspace version: $CARGO_VERSION"
log_info "Python version: ${PYTHON_VERSION:-N/A}"
log_info "Node version: ${NODE_VERSION:-N/A}"

FAILED=0

# Validate Cargo version (required)
if [[ "$TAG_VERSION" != "$CARGO_VERSION" ]]; then
    gh_error "Tag version ($TAG_VERSION) doesn't match Cargo.toml version ($CARGO_VERSION)"
    FAILED=1
fi

# Validate Python version if present
if [[ -n "$PYTHON_VERSION" && "$TAG_VERSION" != "$PYTHON_VERSION" ]]; then
    gh_error "Tag version ($TAG_VERSION) doesn't match pyproject.toml version ($PYTHON_VERSION)"
    FAILED=1
fi

# Validate Node version if present
if [[ -n "$NODE_VERSION" && "$TAG_VERSION" != "$NODE_VERSION" ]]; then
    gh_error "Tag version ($TAG_VERSION) doesn't match package.json version ($NODE_VERSION)"
    FAILED=1
fi

if [[ $FAILED -ne 0 ]]; then
    log_error "Version validation failed"
    exit 1
fi

log_success "All versions match: $TAG_VERSION"

# Output for GitHub Actions
echo "version=$TAG_VERSION" >> "${GITHUB_OUTPUT:-/dev/null}"

# Write summary
gh_summary "## Version Validation"
gh_summary ""
gh_summary "| Source | Version | Status |"
gh_summary "|--------|---------|--------|"
gh_summary "| Git Tag | $TAG_VERSION | ✅ |"
gh_summary "| Cargo.toml | $CARGO_VERSION | ✅ |"
gh_summary "| pyproject.toml | ${PYTHON_VERSION:-N/A} | ✅ |"
gh_summary "| package.json | ${NODE_VERSION:-N/A} | ✅ |"
