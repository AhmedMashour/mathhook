#!/bin/bash
# Collect all baseline benchmarks for Wave 3 analysis

set -e

WAVE_DIR="/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/wave3"
BENCHMARK_DIR="/Users/ahmedmashhour/Documents/work/math/mathhook/worktrees/agent-1-performance"

cd "$BENCHMARK_DIR"

echo "Starting baseline collection for Wave 3..."
echo "Results will be stored in: $WAVE_DIR"

# List of all benchmarks
BENCHMARKS=(
    "core_performance"
    "realistic_cas_benchmarks"
    "comprehensive_performance_suite"
    "calculus_benchmarks"
    "performance_consistency"
    "simd_performance_analysis"
    "solving_benchmarks"
    "simplification_benchmarks"
    "function_evaluation_benchmarks"
    "educational_benchmarks"
    "parsing_benchmarks"
)

# Run each benchmark and save output
for bench in "${BENCHMARKS[@]}"; do
    echo ""
    echo "========================================="
    echo "Running benchmark: $bench"
    echo "========================================="

    OUTPUT_FILE="$WAVE_DIR/baseline_${bench}.txt"

    # Remove old file if it exists and contains errors
    if [ -f "$OUTPUT_FILE" ]; then
        if grep -q "ERROR:" "$OUTPUT_FILE" || grep -q "TIMEOUT:" "$OUTPUT_FILE"; then
            echo "Removing failed baseline: $OUTPUT_FILE"
            rm "$OUTPUT_FILE"
        else
            echo "Baseline already exists: $OUTPUT_FILE (skipping)"
            continue
        fi
    fi

    echo "Saving to: $OUTPUT_FILE"

    # Run benchmark (no timeout on macOS since 'timeout' is not available)
    # Use a background job with a 15-minute limit instead
    (
        cargo bench --bench "$bench" > "$OUTPUT_FILE" 2>&1
    ) &

    BENCH_PID=$!

    # Wait up to 900 seconds (15 minutes)
    for i in {1..900}; do
        if ! ps -p $BENCH_PID > /dev/null 2>&1; then
            # Process finished
            wait $BENCH_PID
            EXIT_CODE=$?
            if [ $EXIT_CODE -ne 0 ]; then
                echo "ERROR: Benchmark $bench failed with exit code $EXIT_CODE" | tee -a "$OUTPUT_FILE"
            fi
            break
        fi

        if [ $i -eq 900 ]; then
            # Timeout reached
            kill $BENCH_PID 2>/dev/null
            echo "TIMEOUT: Benchmark $bench exceeded 15 minutes" | tee -a "$OUTPUT_FILE"
        fi

        sleep 1
    done

    echo "Completed: $bench"

    # Brief pause between benchmarks
    sleep 2
done

echo ""
echo "========================================="
echo "Baseline collection complete!"
echo "========================================="
echo ""
echo "Results stored in: $WAVE_DIR"
ls -lh "$WAVE_DIR"/baseline_*.txt
