#!/bin/bash
set -e

echo "=== Wave 2 Verification: Comprehensive Core Functionality Benchmarks ==="
echo ""

# Define benchmark files to check
benchmarks=(
    "calculus_benchmarks"
    "solving_benchmarks"
    "simplification_benchmarks"
    "function_evaluation_benchmarks"
    "educational_benchmarks"
    "parsing_benchmarks"
)

# 1. Check all benchmark files exist
echo "Step 1: Checking benchmark files exist..."
for bench in "${benchmarks[@]}"; do
    if [ ! -f "crates/mathhook-benchmarks/benches/${bench}.rs" ]; then
        echo "❌ FAIL: ${bench}.rs not found"
        exit 1
    fi
    echo "  ✓ ${bench}.rs exists"
done
echo "✅ All 6 benchmark files exist"
echo ""

# 2. Check all registered in Cargo.toml
echo "Step 2: Checking registration in Cargo.toml..."
for bench in "${benchmarks[@]}"; do
    if ! grep -q "name = \"$bench\"" crates/mathhook-benchmarks/Cargo.toml; then
        echo "❌ FAIL: $bench not registered in Cargo.toml"
        exit 1
    fi
    echo "  ✓ $bench registered"
done
echo "✅ All benchmarks registered in Cargo.toml"
echo ""

# 3. Compile all benchmarks (without running)
echo "Step 3: Compiling benchmarks..."
if cargo bench --no-run -p mathhook-benchmarks 2>&1 | grep -q "error"; then
    echo "❌ FAIL: Benchmarks don't compile"
    exit 1
fi
echo "✅ All benchmarks compile successfully"
echo ""

# 4. Count benchmark functions
echo "Step 4: Analyzing benchmark coverage..."
total_benches=0
for bench in "${benchmarks[@]}"; do
    count=$(grep -c "fn bench_" "crates/mathhook-benchmarks/benches/${bench}.rs" || echo 0)
    echo "  - $bench: $count benchmarks"
    total_benches=$((total_benches + count))
done
echo "✅ Total benchmark functions: $total_benches"
echo ""

# 5. Summary
echo "=== Wave 2 Verification: PASSED ==="
echo ""
echo "Summary:"
echo "- ✅ 6 benchmark files created"
echo "- ✅ All benchmarks registered in Cargo.toml"
echo "- ✅ All benchmarks compile"
echo "- ✅ Total benchmark functions: $total_benches"
echo ""
echo "Benchmarks created:"
for bench in "${benchmarks[@]}"; do
    echo "  - $bench"
done
echo ""
echo "Priority coverage achieved:"
echo "  Priority 1 (CRITICAL): calculus_benchmarks, solving_benchmarks"
echo "  Priority 2 (HIGH):     simplification_benchmarks"
echo "  Priority 3 (MEDIUM):   function_evaluation_benchmarks, educational_benchmarks"
echo "  Priority 4 (LOW):      parsing_benchmarks"
echo ""
echo "Next steps:"
echo "  1. Run full benchmarks: cargo bench -p mathhook-benchmarks"
echo "  2. Analyze baseline performance"
echo "  3. Proceed to Wave 3 (Regression Fixes)"
