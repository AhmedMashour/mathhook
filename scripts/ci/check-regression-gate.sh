#!/bin/bash
# Check regression gate and generate summary
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

REGRESSION_DETECTED="${1:-false}"
REGRESSION_REPORT="${2:-}"

gh_summary "## Benchmark Regression Analysis"
gh_summary ""

FAILED=0
THRESHOLD="${REGRESSION_THRESHOLD:-10}"

if [[ "$REGRESSION_DETECTED" == "true" ]]; then
    gh_summary "⚠️ **Regression detected in Rust benchmarks**"
    gh_summary ""
    gh_summary "$REGRESSION_REPORT"
    FAILED=1
else
    gh_summary "✅ No significant regressions detected"
fi

echo "failed=$FAILED" >> "$GITHUB_OUTPUT"
