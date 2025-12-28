#!/usr/bin/env bash
# Verify TypeScript source matches compiled JavaScript
# Usage: verify-typescript.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib.sh
source "${SCRIPT_DIR}/lib.sh"

cd "${SCRIPT_DIR}/ts"

# Type check first
log_info "Running TypeScript type check..."
tsc --noEmit

# Compile
log_info "Compiling TypeScript..."
tsc

# Check for differences
log_info "Verifying compiled JS matches source..."
if git diff --exit-code ../js/; then
    log_success "Compiled JS matches TypeScript source"
else
    log_error "Compiled JS differs from TypeScript source"
    log_error "Run 'cd scripts/ci/ts && tsc' to recompile"
    exit 1
fi
