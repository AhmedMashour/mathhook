#!/bin/bash
# Wait for a crate to appear in the crates.io index
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

CRATE_NAME="${1:-}"
VERSION="${2:-}"
MAX_WAIT="${3:-120}"
INTERVAL=10

if [[ -z "$CRATE_NAME" || -z "$VERSION" ]]; then
    log_error "Usage: $0 <crate-name> <version> [max-wait-seconds]"
    exit 1
fi

log_info "Waiting for $CRATE_NAME@$VERSION in crates.io index..."

ELAPSED=0

while [[ $ELAPSED -lt $MAX_WAIT ]]; do
    # Use proper error handling for network failures
    SEARCH_OUTPUT=""
    if SEARCH_OUTPUT=$(cargo search "$CRATE_NAME" 2>&1); then
        if echo "$SEARCH_OUTPUT" | grep -q "^$CRATE_NAME = \"$VERSION\""; then
            log_success "$CRATE_NAME@$VERSION is now indexed"
            exit 0
        fi
    else
        log_warn "cargo search failed (network issue?), retrying..."
    fi

    log_info "Waiting... ($ELAPSED/$MAX_WAIT seconds)"
    sleep "$INTERVAL"
    ELAPSED=$((ELAPSED + INTERVAL))
done

gh_warning "Timeout waiting for $CRATE_NAME@$VERSION after ${MAX_WAIT}s, continuing anyway"
exit 0
