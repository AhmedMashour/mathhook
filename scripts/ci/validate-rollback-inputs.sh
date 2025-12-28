#!/usr/bin/env bash
# Validate and sanitize rollback workflow inputs
# Usage: validate-rollback-inputs.sh <version> <reason>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "${SCRIPT_DIR}/lib.sh"

VERSION="${1:-}"
REASON="${2:-}"

# Validate version format (semver)
if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    gh_error "Invalid version format. Expected semver (e.g., 0.1.5 or 0.1.5-beta.1)"
    exit 1
fi

# Sanitize reason (remove potentially dangerous characters)
SAFE_REASON=$(echo "$REASON" | tr -cd '[:alnum:][:space:].,!?-')
if [[ -z "$SAFE_REASON" ]]; then
    gh_error "Reason cannot be empty after sanitization"
    exit 1
fi

# Output sanitized values
gh_output "version" "$VERSION"
gh_output "reason" "$SAFE_REASON"
log_success "Inputs validated: version=$VERSION"
