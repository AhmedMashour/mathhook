#!/usr/bin/env bash
# Run ShellCheck on all shell scripts
# Usage: run-shellcheck.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

cd "$PROJECT_ROOT"

find scripts docker/scripts -name '*.sh' -type f -print0 2>/dev/null | \
    xargs -0 shellcheck --severity=warning --shell=bash
