#!/usr/bin/env bash
# Generate security scan summary
# Usage: security-summary.sh <cargo_audit_result> <cargo_deny_result> <trivy_result> <npm_audit_result>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "${SCRIPT_DIR}/lib.sh"

CARGO_AUDIT="${1:-skipped}"
CARGO_DENY="${2:-skipped}"
TRIVY="${3:-skipped}"
NPM_AUDIT="${4:-skipped}"

status_icon() {
    [[ "$1" == "success" ]] && echo "✅" || echo "❌"
}

gh_summary "## Security Scan Summary"
gh_summary ""
gh_summary "| Check | Status |"
gh_summary "|-------|--------|"
gh_summary "| Cargo Audit | $(status_icon "$CARGO_AUDIT") |"
gh_summary "| Cargo Deny | $(status_icon "$CARGO_DENY") |"
gh_summary "| Trivy | $(status_icon "$TRIVY") |"
gh_summary "| npm Audit | $(status_icon "$NPM_AUDIT") |"
