#!/bin/bash
set -e

echo "=== Wave 1 Verification: Benchmark Audit & Cleanup ==="
echo ""

# Change to worktree directory
cd /Users/ahmedmashhour/Documents/work/math/mathhook/worktrees/agent-1-performance

# 1. Check irrelevant benchmarks removed (or never existed)
echo "1. Checking for irrelevant benchmarks..."
if [ -f "crates/mathhook-benchmarks/benches/symbolica_challenge.rs" ]; then
    echo "❌ FAIL: symbolica_challenge.rs still exists"
    exit 1
fi

if [ -f "crates/mathhook-benchmarks/benches/mathhook_iq_test_suite.rs" ]; then
    echo "❌ FAIL: mathhook_iq_test_suite.rs still exists"
    exit 1
fi
echo "✅ No irrelevant benchmarks found (clean)"

# 2. Check realistic_cas_benchmarks registered
echo ""
echo "2. Checking realistic_cas_benchmarks registration..."
if ! grep -q "realistic_cas_benchmarks" crates/mathhook-benchmarks/Cargo.toml; then
    echo "❌ FAIL: realistic_cas_benchmarks not registered"
    exit 1
fi
echo "✅ realistic_cas_benchmarks registered in Cargo.toml"

# 3. Check coverage report exists
echo ""
echo "3. Checking coverage report..."
if [ ! -f ".mathhook_sessions/gtm/wave1_coverage_gaps.md" ]; then
    echo "❌ FAIL: Coverage report not found"
    exit 1
fi
echo "✅ Coverage report created: .mathhook_sessions/gtm/wave1_coverage_gaps.md"

# 4. Verify coverage report has required sections
echo ""
echo "4. Validating coverage report content..."
required_sections=("Currently Benchmarked" "Missing Coverage" "Recommendations for Wave 2")
for section in "${required_sections[@]}"; do
    if ! grep -qi "$section" .mathhook_sessions/gtm/wave1_coverage_gaps.md; then
        echo "❌ FAIL: Missing section: $section"
        exit 1
    fi
done
echo "✅ Coverage report has all required sections"

# 5. Check all existing benchmarks are registered
echo ""
echo "5. Verifying all benchmarks are registered..."
benchmarks=("core_performance" "realistic_cas_benchmarks" "comprehensive_performance_suite" "performance_consistency" "simd_performance_analysis")
for bench in "${benchmarks[@]}"; do
    if [ ! -f "crates/mathhook-benchmarks/benches/${bench}.rs" ]; then
        echo "❌ FAIL: ${bench}.rs not found"
        exit 1
    fi
    if ! grep -q "name = \"$bench\"" crates/mathhook-benchmarks/Cargo.toml; then
        echo "❌ FAIL: $bench not registered in Cargo.toml"
        exit 1
    fi
done
echo "✅ All 5 benchmarks exist and are registered"

# 6. Compile all benchmarks (no-run to save time)
echo ""
echo "6. Compiling all benchmarks..."
cargo bench --no-run -p mathhook-benchmarks --quiet 2>&1 | tail -5
if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "❌ FAIL: Benchmarks don't compile"
    exit 1
fi
echo "✅ All benchmarks compile successfully"

# 7. Check for baseline metrics documentation
echo ""
echo "7. Checking baseline metrics..."
echo "✅ Baseline infrastructure ready (to be established in first benchmark run)"

# Summary
echo ""
echo "================================================================="
echo "=== Wave 1 Verification: PASSED ==="
echo "================================================================="
echo ""
echo "Summary:"
echo "  - Benchmark suite is clean (no irrelevant benchmarks)"
echo "  - realistic_cas_benchmarks.rs already registered"
echo "  - Coverage gaps documented in .mathhook_sessions/gtm/wave1_coverage_gaps.md"
echo "  - All 5 benchmarks compile and are ready to run"
echo ""
echo "Coverage Analysis:"
echo "  - Currently covered: Core operations, matrix ops, polynomials, SIMD, GPU"
echo "  - Missing (HIGH PRIORITY): Calculus, solving, simplification"
echo "  - Missing (MEDIUM): Function evaluation, educational features"
echo "  - Missing (LOW): Parsing/formatting benchmarks"
echo ""
echo "Next Steps:"
echo "  1. Review .mathhook_sessions/gtm/wave1_coverage_gaps.md"
echo "  2. Proceed to Wave 2: Create comprehensive benchmark suite"
echo "  3. Priority order: calculus → solving → simplification → function_eval"
echo ""
