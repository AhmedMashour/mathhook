#!/bin/bash
# Analyze benchmark results for regressions
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

RESULTS_FILE="${1:-}"

if [[ -z "$RESULTS_FILE" ]]; then
    log_error "Usage: $0 <results-file>"
    exit 1
fi

if [[ -f "$RESULTS_FILE" ]]; then
    # Check for regressions in benchmark results
    REGRESSION=$(jq -r '.regressions // empty' "$RESULTS_FILE" 2>/dev/null || echo "")
    if [[ -n "$REGRESSION" ]]; then
        echo "regression_detected=true" >> "$GITHUB_OUTPUT"
        echo "report=$REGRESSION" >> "$GITHUB_OUTPUT"
        log_warn "Regression detected in benchmark results"
    else
        echo "regression_detected=false" >> "$GITHUB_OUTPUT"
        log_info "No regressions detected"
    fi
else
    echo "regression_detected=false" >> "$GITHUB_OUTPUT"
    log_warn "Results file not found: $RESULTS_FILE"
fi
