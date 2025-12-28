#!/bin/bash
# Run Rust benchmarks for CI
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

RESULTS_FILE="${1:-benchmark-results/rust.json}"
RESULTS_DIR=$(dirname "$RESULTS_FILE")
ensure_dir "$RESULTS_DIR"

log_info "Running Rust benchmarks..."

if [[ ! -f "crates/mathhook-benchmarks/Cargo.toml" ]]; then
    log_warn "Benchmark crate not found"
    echo '{"platform":"rust","status":"not_found"}' > "$RESULTS_FILE"
    exit 0
fi

OUTPUT_FILE=$(make_temp_file)
trap 'rm -f "$OUTPUT_FILE"' EXIT

if cargo bench -p mathhook-benchmarks --benches 2>&1 | tee "$OUTPUT_FILE"; then
    log_success "Rust benchmarks completed successfully"
    echo '{"platform":"rust","status":"success"}' > "$RESULTS_FILE"
else
    log_warn "Some Rust benchmarks had errors (may be partial success)"
    echo '{"platform":"rust","status":"partial","note":"Some benchmarks may have failed"}' > "$RESULTS_FILE"
fi
