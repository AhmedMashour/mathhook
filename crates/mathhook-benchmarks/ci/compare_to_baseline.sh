#!/bin/bash
#
# Compare to Baseline
#
# Compares current benchmarks against stored baselines.
# Detects performance regressions across all platforms.
#
# Usage:
#     ./compare_to_baseline.sh [--threshold PERCENT] [--platform PLATFORM]
#
# Exit Codes:
#     0: No regressions detected
#     1: Performance regressions detected
#     2: Baseline not found or error
#
# Last Updated: 2025-11-29T2000

set -e

# Configuration
THRESHOLD=10.0
PLATFORM="all"
ITERATIONS=100

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --threshold)
            THRESHOLD="$2"
            shift 2
            ;;
        --platform)
            PLATFORM="$2"
            shift 2
            ;;
        --iterations)
            ITERATIONS="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo "================================================================================"
echo "MathHook Baseline Comparison"
echo "================================================================================"
echo "Platform: $PLATFORM"
echo "Threshold: ${THRESHOLD}%"
echo "Iterations: $ITERATIONS"
echo ""

# Navigate to benchmark root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARK_ROOT="$SCRIPT_DIR/.."
cd "$BENCHMARK_ROOT"

# Track if any regressions detected
HAS_REGRESSIONS=false

# Compare Rust benchmarks
if [[ "$PLATFORM" == "all" || "$PLATFORM" == "rust" ]]; then
    echo -e "${GREEN}Comparing Rust benchmarks...${NC}"
    # Criterion has built-in baseline comparison
    if cargo bench --benches 2>&1 | grep -i "regression"; then
        HAS_REGRESSIONS=true
        echo -e "${RED}Rust regressions detected${NC}"
    fi
    echo ""
fi

# Compare Python benchmarks
if [[ "$PLATFORM" == "all" || "$PLATFORM" == "python" ]]; then
    echo -e "${GREEN}Comparing Python benchmarks...${NC}"
    if command -v python3 &> /dev/null; then
        if python3 public/python/compare_baseline.py --iterations $ITERATIONS --threshold $THRESHOLD; then
            echo -e "${GREEN}Python: No regressions${NC}"
        else
            HAS_REGRESSIONS=true
            echo -e "${RED}Python regressions detected${NC}"
        fi
    else
        echo -e "${YELLOW}WARNING: Python not found, skipping${NC}"
    fi
    echo ""
fi

# Compare Node.js benchmarks
if [[ "$PLATFORM" == "all" || "$PLATFORM" == "node" ]]; then
    echo -e "${GREEN}Comparing Node.js benchmarks...${NC}"
    if command -v node &> /dev/null; then
        if node public/node/compare_baseline.js --iterations $ITERATIONS --threshold $THRESHOLD; then
            echo -e "${GREEN}Node.js: No regressions${NC}"
        else
            HAS_REGRESSIONS=true
            echo -e "${RED}Node.js regressions detected${NC}"
        fi
    else
        echo -e "${YELLOW}WARNING: Node.js not found, skipping${NC}"
    fi
    echo ""
fi

# Summary
echo "================================================================================"
if [ "$HAS_REGRESSIONS" = true ]; then
    echo -e "${RED}FAILURE: Performance regressions detected!${NC}"
    echo "Review the output above for details."
    exit 1
else
    echo -e "${GREEN}SUCCESS: No performance regressions detected!${NC}"
    exit 0
fi
