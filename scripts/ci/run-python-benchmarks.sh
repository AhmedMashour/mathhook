#!/bin/bash
# Run Python benchmarks for CI
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

RESULTS_FILE="${1:-benchmark-results/python.json}"
RESULTS_DIR=$(dirname "$RESULTS_FILE")
PROJECT_ROOT=$(pwd)

ensure_dir "$RESULTS_DIR"

# Convert to absolute path
RESULTS_FILE="$PROJECT_ROOT/$RESULTS_FILE"

log_info "Running Python benchmarks..."

if [[ ! -f "crates/mathhook-python/Cargo.toml" ]]; then
    log_warn "Python bindings not found"
    echo '{"platform":"python","status":"not_found"}' > "$RESULTS_FILE"
    exit 0
fi

# Build Python bindings
log_info "Building Python bindings..."
pip install maturin || {
    log_error "Failed to install maturin"
    echo '{"platform":"python","status":"build_error"}' > "$RESULTS_FILE"
    exit 0
}

cd crates/mathhook-python
maturin develop --release || {
    log_error "Failed to build Python bindings"
    echo '{"platform":"python","status":"build_error"}' > "$RESULTS_FILE"
    exit 0
}
cd "$PROJECT_ROOT"

# Run benchmarks
BENCH_SCRIPT="crates/mathhook-benchmarks/public/python/bench_mathhook.py"
if [[ ! -f "$BENCH_SCRIPT" ]]; then
    log_warn "Python benchmark script not found"
    echo '{"platform":"python","status":"script_not_found"}' > "$RESULTS_FILE"
    exit 0
fi

cd crates/mathhook-benchmarks/public/python

TEMP_OUTPUT=$(make_temp_file)
trap 'rm -f "$TEMP_OUTPUT"' EXIT

if python3 bench_mathhook.py --json --iterations 50 > "$TEMP_OUTPUT" 2>&1; then
    # Validate JSON
    if python3 -c "import json; json.load(open('$TEMP_OUTPUT'))" 2>/dev/null; then
        log_success "Python benchmarks completed successfully"
        cp "$TEMP_OUTPUT" "$RESULTS_FILE"
    else
        log_error "Python benchmarks produced invalid JSON"
        echo '{"platform":"python","status":"json_error"}' > "$RESULTS_FILE"
    fi
else
    log_error "Python benchmarks failed to run"
    echo '{"platform":"python","status":"run_error"}' > "$RESULTS_FILE"
fi
