#!/bin/bash
# Yank crates from crates.io
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
    log_error "Usage: $0 <version>"
    exit 1
fi

FAILED=0

yank_crate() {
    local crate="$1"
    log_info "Yanking $crate@$VERSION..."

    # Capture output once to avoid running cargo yank twice
    local output
    if output=$(cargo yank "$crate@$VERSION" 2>&1); then
        log_success "$crate@$VERSION yanked"
    elif echo "$output" | grep -q "already yanked"; then
        gh_notice "$crate@$VERSION already yanked"
    else
        gh_error "Failed to yank $crate@$VERSION"
        echo "$output" >&2
        FAILED=1
    fi
}

yank_crate "mathhook"
yank_crate "mathhook-core"
yank_crate "mathhook-macros"

gh_summary "## crates.io Rollback"
gh_summary "Yanked version $VERSION from crates.io"

if [[ $FAILED -ne 0 ]]; then
    gh_error "Some crates failed to yank"
    exit 1
fi
