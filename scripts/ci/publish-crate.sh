#!/bin/bash
# Publish a crate to crates.io with proper error handling
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

CRATE_PATH="${1:-}"
DRY_RUN="${DRY_RUN:-false}"

if [[ -z "$CRATE_PATH" ]]; then
    log_error "Usage: $0 <crate-path>"
    log_error "Example: $0 crates/mathhook-core"
    exit 1
fi

if [[ ! -d "$CRATE_PATH" ]]; then
    gh_error "Crate path not found: $CRATE_PATH"
    exit 1
fi

require_env CARGO_REGISTRY_TOKEN

cd "$CRATE_PATH"
CRATE_NAME=$(basename "$CRATE_PATH")
VERSION=$(get_cargo_version Cargo.toml)

log_info "Publishing $CRATE_NAME@$VERSION to crates.io..."

if [[ "$DRY_RUN" == "true" ]]; then
    log_info "DRY RUN: cargo publish --dry-run"
    cargo publish --dry-run
    echo "status=dry_run" >> "${GITHUB_OUTPUT:-/dev/null}"
    exit 0
fi

LOG_FILE=$(make_temp_file)
trap 'rm -f "$LOG_FILE"' EXIT

if cargo publish 2>&1 | tee "$LOG_FILE"; then
    log_success "$CRATE_NAME@$VERSION published successfully"
    echo "status=success" >> "${GITHUB_OUTPUT:-/dev/null}"
else
    if grep -q "already exists" "$LOG_FILE"; then
        gh_notice "$CRATE_NAME@$VERSION already published"
        echo "status=skipped" >> "${GITHUB_OUTPUT:-/dev/null}"
    else
        gh_error "Failed to publish $CRATE_NAME"
        cat "$LOG_FILE" >&2
        echo "status=failed" >> "${GITHUB_OUTPUT:-/dev/null}"
        exit 1
    fi
fi
