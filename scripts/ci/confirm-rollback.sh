#!/usr/bin/env bash
# Confirm rollback action and output summary
# Usage: confirm-rollback.sh <version> <reason> <targets>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "${SCRIPT_DIR}/lib.sh"

VERSION="${1:-}"
REASON="${2:-}"
TARGETS="${3:-}"

gh_summary "## ⚠️ Rollback Confirmation"
gh_summary ""
gh_summary "**Version:** ${VERSION}"
gh_summary "**Reason:** ${REASON}"
gh_summary "**Targets:** ${TARGETS}"
gh_summary ""
gh_summary "Proceeding with rollback..."

gh_output "confirmed" "true"
