#!/bin/bash
#
# Run All Benchmarks
#
# Executes benchmarks across all platforms (Rust, Python, Node.js).
# Used in CI/CD pipelines and local testing.
#
# Usage:
#     ./run_benchmarks.sh [--quick] [--platform PLATFORM]
#
# Options:
#     --quick      Use reduced iterations for faster execution
#     --platform   Run only specific platform (rust|python|node)
#
# Last Updated: 2025-11-29T2000

set -e

# Configuration
QUICK_MODE=false
PLATFORM="all"
ITERATIONS=100

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            ITERATIONS=50
            shift
            ;;
        --platform)
            PLATFORM="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Colors
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo "================================================================================"
echo "MathHook Cross-Platform Benchmark Suite"
echo "================================================================================"
echo "Platform: $PLATFORM"
echo "Quick mode: $QUICK_MODE"
echo "Iterations: $ITERATIONS"
echo ""

# Navigate to benchmark root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARK_ROOT="$SCRIPT_DIR/.."
cd "$BENCHMARK_ROOT"

# Create results directory
RESULTS_DIR="$BENCHMARK_ROOT/results"
mkdir -p "$RESULTS_DIR"

TIMESTAMP=$(date -u +"%Y-%m-%dT%H%M%S")

# Run Rust benchmarks
if [[ "$PLATFORM" == "all" || "$PLATFORM" == "rust" ]]; then
    echo -e "${GREEN}Running Rust benchmarks...${NC}"
    if [ "$QUICK_MODE" = true ]; then
        cargo bench --benches -- --quick 2>&1 | tee "$RESULTS_DIR/rust_${TIMESTAMP}.log"
    else
        cargo bench --benches 2>&1 | tee "$RESULTS_DIR/rust_${TIMESTAMP}.log"
    fi
    echo ""
fi

# Run Python benchmarks
if [[ "$PLATFORM" == "all" || "$PLATFORM" == "python" ]]; then
    echo -e "${GREEN}Running Python benchmarks...${NC}"
    if command -v python3 &> /dev/null; then
        python3 public/python/bench_mathhook.py --json --iterations $ITERATIONS > "$RESULTS_DIR/python_${TIMESTAMP}.json"
        echo "Results: $RESULTS_DIR/python_${TIMESTAMP}.json"
    else
        echo -e "${YELLOW}WARNING: Python not found, skipping Python benchmarks${NC}"
    fi
    echo ""
fi

# Run Node.js benchmarks
if [[ "$PLATFORM" == "all" || "$PLATFORM" == "node" ]]; then
    echo -e "${GREEN}Running Node.js benchmarks...${NC}"
    if command -v node &> /dev/null; then
        node public/node/bench_mathhook.js --json --iterations $ITERATIONS > "$RESULTS_DIR/node_${TIMESTAMP}.json"
        echo "Results: $RESULTS_DIR/node_${TIMESTAMP}.json"
    else
        echo -e "${YELLOW}WARNING: Node.js not found, skipping Node.js benchmarks${NC}"
    fi
    echo ""
fi

echo "================================================================================"
echo "Benchmark suite completed!"
echo "Results stored in: $RESULTS_DIR"
echo "================================================================================"
