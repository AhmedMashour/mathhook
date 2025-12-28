#!/bin/bash
# MathHook Cross-Platform Benchmarks
# Runs Rust, Python, and Node.js benchmarks with proper error handling
set -eo pipefail

ARTIFACTS_DIR="${ARTIFACTS_DIR:-/build/artifacts}"
PROJECT_ROOT="${PROJECT_ROOT:-/build}"

log_info() { echo ">>> $1"; }
log_error() { echo "ERROR: $1" >&2; }
log_warn() { echo "WARNING: $1" >&2; }

mkdir -p "$ARTIFACTS_DIR"/{rust,python,node}-benchmark-results

echo "=== MathHook Cross-Platform Benchmarks ==="

# Rust benchmarks
log_info "Running Rust benchmarks..."
if cargo bench -p mathhook-benchmarks --benches 2>&1 | tee /tmp/rust_bench.log; then
    echo '{"status": "success"}' > "$ARTIFACTS_DIR/rust-benchmark-results/rust.json"
else
    log_warn "Rust benchmarks had errors (may be partial success)"
    echo '{"status": "partial", "error": "Some benchmarks failed"}' > "$ARTIFACTS_DIR/rust-benchmark-results/rust.json"
fi

# Python benchmarks
log_info "Running Python benchmarks..."
cd "$PROJECT_ROOT/crates/mathhook-python"

PYTHON_INSTALLED=false
if pip3 install --break-system-packages -e . 2>&1 | tee /tmp/pip_install.log; then
    log_info "Installed via pip"
    PYTHON_INSTALLED=true
elif command -v maturin &>/dev/null && maturin develop --release 2>&1 | tee /tmp/maturin_install.log; then
    log_info "Installed via maturin"
    PYTHON_INSTALLED=true
else
    log_warn "Could not install Python bindings (see /tmp/pip_install.log)"
fi

PYTHON_BENCH_DIR="$PROJECT_ROOT/crates/mathhook-benchmarks/public/python"
if [[ "$PYTHON_INSTALLED" == "true" ]]; then
    if [[ -f "$PYTHON_BENCH_DIR/run_all.py" ]]; then
        cd "$PYTHON_BENCH_DIR"
        if python3 run_all.py --output "$ARTIFACTS_DIR/python-benchmark-results/python.json"; then
            log_info "Python benchmarks completed"
        else
            log_warn "Python benchmarks failed"
            echo '{"status": "failed"}' > "$ARTIFACTS_DIR/python-benchmark-results/python.json"
        fi
    elif [[ -f "$PYTHON_BENCH_DIR/bench_mathhook.py" ]]; then
        cd "$PYTHON_BENCH_DIR"
        if python3 bench_mathhook.py > "$ARTIFACTS_DIR/python-benchmark-results/python.json"; then
            log_info "Python benchmarks completed"
        else
            log_warn "Python benchmarks failed"
            echo '{"status": "failed"}' > "$ARTIFACTS_DIR/python-benchmark-results/python.json"
        fi
    else
        log_warn "No Python benchmark script found"
        echo '{"status": "not_found"}' > "$ARTIFACTS_DIR/python-benchmark-results/python.json"
    fi
else
    echo '{"status": "install_failed"}' > "$ARTIFACTS_DIR/python-benchmark-results/python.json"
fi

# Node.js benchmarks
log_info "Running Node.js benchmarks..."
cd "$PROJECT_ROOT/crates/mathhook-node"

NODE_INSTALLED=false
if npm install 2>&1 | tee /tmp/npm_install.log; then
    if npx napi build --platform --release 2>&1 | tee /tmp/napi_build.log; then
        log_info "Node.js bindings built"
        NODE_INSTALLED=true
    else
        log_warn "napi build failed (see /tmp/napi_build.log)"
    fi
else
    log_warn "npm install failed (see /tmp/npm_install.log)"
fi

NODE_BENCH_DIR="$PROJECT_ROOT/crates/mathhook-benchmarks/public/node"
if [[ "$NODE_INSTALLED" == "true" ]]; then
    if [[ -f "$NODE_BENCH_DIR/runAll.js" ]]; then
        cd "$NODE_BENCH_DIR"
        if npm install 2>&1 | tee /tmp/npm_bench_install.log; then
            if node runAll.js --output "$ARTIFACTS_DIR/node-benchmark-results/node.json"; then
                log_info "Node.js benchmarks completed"
            else
                log_warn "Node.js benchmarks failed"
                echo '{"status": "failed"}' > "$ARTIFACTS_DIR/node-benchmark-results/node.json"
            fi
        else
            log_warn "npm install for benchmarks failed"
            echo '{"status": "install_failed"}' > "$ARTIFACTS_DIR/node-benchmark-results/node.json"
        fi
    elif [[ -f "$NODE_BENCH_DIR/bench_mathhook.js" ]]; then
        cd "$NODE_BENCH_DIR"
        if npm install 2>&1 | tee /tmp/npm_bench_install.log; then
            if node bench_mathhook.js > "$ARTIFACTS_DIR/node-benchmark-results/node.json"; then
                log_info "Node.js benchmarks completed"
            else
                log_warn "Node.js benchmarks failed"
                echo '{"status": "failed"}' > "$ARTIFACTS_DIR/node-benchmark-results/node.json"
            fi
        else
            log_warn "npm install for benchmarks failed"
            echo '{"status": "install_failed"}' > "$ARTIFACTS_DIR/node-benchmark-results/node.json"
        fi
    else
        log_warn "No Node.js benchmark script found"
        echo '{"status": "not_found"}' > "$ARTIFACTS_DIR/node-benchmark-results/node.json"
    fi
else
    echo '{"status": "build_failed"}' > "$ARTIFACTS_DIR/node-benchmark-results/node.json"
fi

# Generate dashboard
log_info "Generating benchmark dashboard..."
cd "$PROJECT_ROOT"
DASHBOARD_SCRIPT="scripts/ci/generate_dashboard.py"
if [[ -f "$DASHBOARD_SCRIPT" ]]; then
    python3 "$DASHBOARD_SCRIPT" "$ARTIFACTS_DIR" "$ARTIFACTS_DIR/dashboard" "$PROJECT_ROOT/target/criterion"
    log_info "Dashboard generated at: $ARTIFACTS_DIR/dashboard/index.html"
else
    log_warn "Dashboard generator not found: $DASHBOARD_SCRIPT"
fi

echo "=== Benchmarks complete ==="
