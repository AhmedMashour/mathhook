#!/usr/bin/env bash
# Display PyPI rollback information (PyPI doesn't support yanking)
# Usage: pypi-rollback-note.sh <version> <reason>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "${SCRIPT_DIR}/lib.sh"

VERSION="${1:-}"
REASON="${2:-}"

gh_summary "## PyPI Rollback"
gh_summary ""
gh_summary "⚠️ **PyPI does not support yanking releases.**"
gh_summary ""
gh_summary "Options:"
gh_summary "1. Publish a new patch version with the fix"
gh_summary "2. Contact PyPI support for critical security issues"
gh_summary "3. Add a warning to the package description"
gh_summary ""
gh_summary "Version: ${VERSION}"
gh_summary "Reason: ${REASON}"
