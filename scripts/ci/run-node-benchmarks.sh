#!/bin/bash
# Run Node.js benchmarks for CI
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

RESULTS_FILE="${1:-benchmark-results/node.json}"
RESULTS_DIR=$(dirname "$RESULTS_FILE")
PROJECT_ROOT=$(pwd)

ensure_dir "$RESULTS_DIR"

# Convert to absolute path
RESULTS_FILE="$PROJECT_ROOT/$RESULTS_FILE"

log_info "Running Node.js benchmarks..."

if [[ ! -f "crates/mathhook-node/package.json" ]]; then
    log_warn "Node bindings not found"
    echo '{"platform":"node","status":"not_found"}' > "$RESULTS_FILE"
    exit 0
fi

# Build Node bindings
log_info "Building Node bindings..."
cd crates/mathhook-node
npm install || {
    log_error "Failed to install Node dependencies"
    echo '{"platform":"node","status":"install_error"}' > "$RESULTS_FILE"
    exit 0
}
npm run build || {
    log_error "Failed to build Node bindings"
    echo '{"platform":"node","status":"build_error"}' > "$RESULTS_FILE"
    exit 0
}
cd "$PROJECT_ROOT"

# Run benchmarks
BENCH_SCRIPT="crates/mathhook-benchmarks/public/node/bench_mathhook.js"
if [[ ! -f "$BENCH_SCRIPT" ]]; then
    log_warn "Node benchmark script not found"
    echo '{"platform":"node","status":"script_not_found"}' > "$RESULTS_FILE"
    exit 0
fi

cd crates/mathhook-benchmarks/public/node

if ! npm install; then
    gh_warning "npm install failed for Node.js benchmarks"
    echo '{"platform":"node","status":"install_error"}' > "$RESULTS_FILE"
    exit 0
fi

TEMP_OUTPUT=$(make_temp_file)
trap 'rm -f "$TEMP_OUTPUT"' EXIT

if node bench_mathhook.js --json --iterations 50 > "$TEMP_OUTPUT" 2>&1; then
    # Validate JSON
    if python3 -c "import json; json.load(open('$TEMP_OUTPUT'))" 2>/dev/null; then
        log_success "Node.js benchmarks completed successfully"
        cp "$TEMP_OUTPUT" "$RESULTS_FILE"
    else
        log_error "Node.js benchmarks produced invalid JSON"
        echo '{"platform":"node","status":"json_error"}' > "$RESULTS_FILE"
    fi
else
    log_error "Node.js benchmarks failed to run"
    echo '{"platform":"node","status":"run_error"}' > "$RESULTS_FILE"
fi
