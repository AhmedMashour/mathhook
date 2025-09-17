#!/bin/bash
# Run the complete benchmark workflow locally (exactly like GitHub CI)
# Last Updated: 2025-11-30T0200
set -e

cd "$(dirname "$0")/../.."
ROOT=$(pwd)

echo "=========================================="
echo "  MathHook Local CI - Full Workflow"
echo "=========================================="

# Setup directories (like GitHub Actions)
rm -rf benchmark-results artifacts gh-pages
mkdir -p benchmark-results artifacts

# Set environment variables (like GitHub)
export GITHUB_SHA=$(git rev-parse HEAD 2>/dev/null || echo "local")
export GITHUB_REPOSITORY="mathhook/mathhook"

echo ""
echo "[1/5] Checking Rust Criterion reports..."
echo "----------------------------------------"
# Criterion reports are in target/criterion/ from previous runs
# We don't re-run benchmarks (they take too long) - just use existing reports
if [ -d "target/criterion" ]; then
    echo "Found existing Criterion reports in target/criterion/"
    ls -la target/criterion/ | head -20
    # Create a marker JSON file with status
    echo '{"platform":"rust","status":"success","note":"Using existing Criterion reports"}' > benchmark-results/rust.json
else
    echo "No Criterion reports found. Run 'cargo bench -p mathhook-benchmarks' first."
    echo '{"platform":"rust","status":"not_run","benchmarks":{}}' > benchmark-results/rust.json
fi

echo ""
echo "[2/5] Running Python benchmarks (MathHook via PyO3)..."
echo "----------------------------------------"
if [ -f "crates/mathhook-python/Cargo.toml" ]; then
    cd crates/mathhook-python
    pip install maturin -q 2>/dev/null || true
    echo "Building Python bindings..."
    maturin develop --release 2>&1 || echo "Python build failed"
    cd "$ROOT"

    if [ -f "crates/mathhook-benchmarks/public/python/bench_mathhook.py" ]; then
        cd crates/mathhook-benchmarks/public/python
        echo "Running Python benchmarks (this may take a minute)..."
        # stderr has "Running..." messages, stdout has JSON
        python3 bench_mathhook.py --json --iterations 10 2>/dev/null > "$ROOT/benchmark-results/python.json" || \
            echo '{"platform":"python","status":"run_error","benchmarks":{}}' > "$ROOT/benchmark-results/python.json"
        # Validate JSON
        if python3 -c "import json; json.load(open('$ROOT/benchmark-results/python.json'))" 2>/dev/null; then
            echo "Python benchmarks completed successfully"
        else
            echo "Python JSON validation failed"
            echo '{"platform":"python","status":"json_error","benchmarks":{}}' > "$ROOT/benchmark-results/python.json"
        fi
        cd "$ROOT"
    else
        echo '{"platform":"python","status":"script_not_found","benchmarks":{}}' > benchmark-results/python.json
    fi
else
    echo '{"platform":"python","status":"not_found","benchmarks":{}}' > benchmark-results/python.json
    echo "Python bindings not found, skipped"
fi

echo ""
echo "[3/5] Running Node.js benchmarks (MathHook via NAPI)..."
echo "----------------------------------------"
if [ -f "crates/mathhook-node/package.json" ]; then
    cd crates/mathhook-node
    npm install 2>&1 || echo "npm install failed"
    npm run build 2>&1 || echo "npm build failed"
    cd "$ROOT"

    if [ -f "crates/mathhook-benchmarks/public/node/bench_mathhook.js" ]; then
        cd crates/mathhook-benchmarks/public/node
        echo "Running Node.js benchmarks..."
        # Capture stdout directly
        node bench_mathhook.js --json --iterations 10 2>/dev/null > "$ROOT/benchmark-results/node.json" || \
            echo '{"platform":"node","status":"run_error","benchmarks":{}}' > "$ROOT/benchmark-results/node.json"
        # Validate JSON
        if python3 -c "import json; json.load(open('$ROOT/benchmark-results/node.json'))" 2>/dev/null; then
            echo "Node.js benchmarks completed successfully"
        else
            echo "Node.js JSON validation failed"
            echo '{"platform":"node","status":"json_error","benchmarks":{}}' > "$ROOT/benchmark-results/node.json"
        fi
        cd "$ROOT"
    else
        echo '{"platform":"node","status":"script_not_found","benchmarks":{}}' > benchmark-results/node.json
    fi
else
    echo '{"platform":"node","status":"not_found","benchmarks":{}}' > benchmark-results/node.json
    echo "Node bindings not found, skipped"
fi

echo ""
echo "[4/5] Preparing artifacts..."
echo "----------------------------------------"
mkdir -p artifacts/rust-benchmark-results
mkdir -p artifacts/python-benchmark-results
mkdir -p artifacts/node-benchmark-results

cp benchmark-results/rust.json artifacts/rust-benchmark-results/
cp benchmark-results/python.json artifacts/python-benchmark-results/
cp benchmark-results/node.json artifacts/node-benchmark-results/

# Show what we have
echo ""
echo "Benchmark Results Summary:"
echo "  Rust:       $(cat artifacts/rust-benchmark-results/rust.json | python3 -c 'import sys,json; d=json.load(sys.stdin); print(d.get("status","unknown"))')"
echo "  Python:     $(cat artifacts/python-benchmark-results/python.json | python3 -c 'import sys,json; d=json.load(sys.stdin); b=d.get("benchmarks",{}); print(f"{len(b)} benchmarks" if b else d.get("status","unknown"))')"
echo "  Node.js:    $(cat artifacts/node-benchmark-results/node.json | python3 -c 'import sys,json; d=json.load(sys.stdin); b=d.get("benchmarks",{}); print(f"{len(b)} benchmarks" if b else d.get("status","unknown"))')"

echo ""
echo "[5/5] Generating dashboard..."
echo "----------------------------------------"
# Pass criterion directory as third argument
python3 scripts/ci/generate_dashboard.py artifacts gh-pages target/criterion

echo ""
echo "=========================================="
echo "  Done! Dashboard ready."
echo "=========================================="
echo ""
echo "Files generated:"
ls -la gh-pages/
echo ""
echo "Criterion reports:"
ls -la gh-pages/criterion/ 2>/dev/null | head -10 || echo "No Criterion reports"
echo ""
echo "To view locally, run:"
echo "  cd gh-pages && python3 -m http.server 8080"
echo ""
echo "Then open:"
echo "  Main dashboard:      http://localhost:8080"
echo "  Comparison (hidden): http://localhost:8080/comparison.html"
echo ""

# Auto-open option
if [ "$1" == "--serve" ] || [ "$1" == "-s" ]; then
    echo "Starting local server..."
    cd gh-pages
    python3 -m http.server 8080
fi
