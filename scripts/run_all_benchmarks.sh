#!/bin/bash

# Wave 3: Run all benchmarks and capture baseline
# This script runs all 11 benchmark suites systematically

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OUTPUT_DIR="$PROJECT_ROOT/.mathhook_sessions/gtm/wave3"

mkdir -p "$OUTPUT_DIR"

cd "$PROJECT_ROOT"

echo "========================================="
echo "Wave 3: Baseline Performance Capture"
echo "========================================="
echo "Output directory: $OUTPUT_DIR"
echo ""

# List of all benchmarks
benchmarks=(
    "core_performance"
    "realistic_cas_benchmarks"
    "comprehensive_performance_suite"
    "performance_consistency"
    "simd_performance_analysis"
    "calculus_benchmarks"
    "solving_benchmarks"
    "simplification_benchmarks"
    "function_evaluation_benchmarks"
    "educational_benchmarks"
    "parsing_benchmarks"
)

# Run each benchmark
for bench in "${benchmarks[@]}"; do
    echo "----------------------------------------"
    echo "Running: $bench"
    echo "----------------------------------------"

    output_file="$OUTPUT_DIR/baseline_${bench}.txt"

    if [ -f "$output_file" ] && grep -q "Benchmarking.*: Analyzing" "$output_file"; then
        echo "✓ Already completed: $bench"
        echo "  Results: $output_file"
    else
        echo "▶ Running benchmark..."
        cargo bench -p mathhook-benchmarks --bench "$bench" 2>&1 | tee "$output_file"
        echo "✓ Completed: $bench"
        echo "  Results saved: $output_file"
    fi

    echo ""
done

echo "========================================="
echo "All benchmarks completed!"
echo "========================================="
echo ""
echo "Results saved in: $OUTPUT_DIR/"
echo ""
echo "Next steps:"
echo "1. Analyze baseline results"
echo "2. Run: scripts/analyze_baseline.sh"
echo ""
