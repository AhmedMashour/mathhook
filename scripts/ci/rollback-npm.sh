#!/bin/bash
# Deprecate npm package
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

VERSION="${1:-}"
REASON="${2:-deprecated}"

if [[ -z "$VERSION" ]]; then
    log_error "Usage: $0 <version> [reason]"
    exit 1
fi

require_env NODE_AUTH_TOKEN

log_info "Deprecating mathhook-node@$VERSION..."

if npm deprecate "mathhook-node@$VERSION" "$REASON - use newer version" 2>&1; then
    log_success "mathhook-node@$VERSION deprecated"
else
    gh_error "Failed to deprecate mathhook-node@$VERSION"
    exit 1
fi

gh_summary "## npm Rollback"
gh_summary "Deprecated mathhook-node@$VERSION on npm"
gh_summary "Reason: $REASON"
