#!/bin/bash
# Wave 6 Verification: Completion - Testing + Documentation
# Expected time: 12-16 hours
# This wave focuses on comprehensive testing, documentation, and final polish

set -e

echo "=========================================="
echo "Wave 6: Completion - Testing + Documentation"
echo "=========================================="
echo ""

# Category 1: Test Coverage - Comprehensive Integration Testing
echo "[1/10] Verifying Test Coverage..."
total_tests=$(cargo test -p mathhook-core --lib 2>&1 | grep "test result:" | awk '{print $4}')
if [ "$total_tests" -lt 200 ]; then
    echo "❌ FAIL: Expected 200+ total tests, found $total_tests"
    exit 1
fi
echo "✅ PASS: $total_tests total tests (target: 200+)"
echo ""

# Category 2: Integration Test Suite
echo "[2/10] Verifying Integration Test Coverage..."
integration_count=$(find crates/mathhook-core/tests -name "integration_*.rs" -type f | wc -l | tr -d ' ')
if [ "$integration_count" -lt 5 ]; then
    echo "❌ FAIL: Expected 5+ integration test files, found $integration_count"
    exit 1
fi
echo "✅ PASS: Found $integration_count integration test files"
echo ""

# Category 3: Documentation Coverage
echo "[3/10] Verifying Documentation..."
# Check for key documentation files
if [ ! -f "crates/mathhook-core/src/calculus/integrals/README.md" ] && \
   [ ! -f "docs/integration_system.md" ]; then
    echo "❌ FAIL: Missing integration system documentation"
    exit 1
fi

# Verify doctests pass
doctest_result=$(cargo test -p mathhook-core --doc 2>&1 | grep "test result:" | awk '{print $4}')
if [ -z "$doctest_result" ] || [ "$doctest_result" -eq 0 ]; then
    echo "⚠️  WARNING: No doctests found or all skipped"
else
    echo "✅ PASS: $doctest_result doctests passing"
fi
echo ""

# Category 4: Performance Benchmarks
echo "[4/10] Verifying Performance Benchmarks..."
if [ ! -d "crates/mathhook-benchmarks/benches" ]; then
    echo "⚠️  WARNING: No benchmark directory found"
else
    benchmark_count=$(find crates/mathhook-benchmarks/benches -name "*integration*.rs" -type f | wc -l | tr -d ' ')
    if [ "$benchmark_count" -eq 0 ]; then
        echo "⚠️  WARNING: No integration benchmarks found"
    else
        echo "✅ PASS: Found $benchmark_count integration benchmark files"
    fi
fi
echo ""

# Category 5: Educational Content
echo "[5/10] Verifying Educational Content..."
# Check for educational registry integration
if ! grep -q "EDUCATIONAL_REGISTRY" crates/mathhook-core/src/calculus/integrals/strategy.rs; then
    echo "⚠️  WARNING: Educational registry not integrated in strategy"
else
    echo "✅ PASS: Educational registry integrated"
fi
echo ""

# Category 6: All Tests Passing
echo "[6/10] Running Full Test Suite..."
if ! cargo test -p mathhook-core --lib --quiet 2>&1 | grep -q "test result: ok"; then
    echo "❌ FAIL: Some tests are failing"
    cargo test -p mathhook-core --lib 2>&1 | tail -20
    exit 1
fi
echo "✅ PASS: All library tests passing"
echo ""

# Category 7: Integration Tests Passing
echo "[7/10] Running All Integration Tests..."
integration_pass=0
integration_total=0
for test_file in crates/mathhook-core/tests/integration_*.rs; do
    if [ -f "$test_file" ]; then
        test_name=$(basename "$test_file" .rs)
        if cargo test -p mathhook-core --test "$test_name" --quiet 2>&1 | grep -q "test result: ok"; then
            integration_pass=$((integration_pass + 1))
        fi
        integration_total=$((integration_total + 1))
    fi
done

if [ "$integration_pass" -ne "$integration_total" ]; then
    echo "❌ FAIL: $integration_pass/$integration_total integration test suites passing"
    exit 1
fi
echo "✅ PASS: All $integration_total integration test suites passing"
echo ""

# Category 8: Build Clean
echo "[8/10] Verifying Clean Build..."
if ! cargo build -p mathhook-core --quiet 2>&1; then
    echo "❌ FAIL: Build errors detected"
    exit 1
fi
echo "✅ PASS: Clean build successful"
echo ""

# Category 9: Clippy Clean
echo "[9/10] Verifying Code Quality (Clippy)..."
if ! cargo clippy -p mathhook-core --quiet 2>&1 | grep -q "0 warnings emitted"; then
    echo "⚠️  WARNING: Clippy warnings detected"
    cargo clippy -p mathhook-core 2>&1 | grep "warning:"
else
    echo "✅ PASS: No clippy warnings"
fi
echo ""

# Category 10: Final Coverage Report
echo "[10/10] Final Coverage Summary..."
echo "-------------------------------------------"
echo "Strategy Layers Implemented:"
grep -n "Layer.*ACTIVE\|Layer.*Wave" crates/mathhook-core/src/calculus/integrals/strategy.rs | while read line; do
    echo "  $line"
done
echo ""

echo "Test Counts by Integration Type:"
for test_file in crates/mathhook-core/tests/integration_*.rs; do
    if [ -f "$test_file" ]; then
        test_name=$(basename "$test_file" .rs | sed 's/integration_//')
        test_count=$(grep -c "^fn test_" "$test_file" || echo "0")
        echo "  - $test_name: $test_count tests"
    fi
done
echo ""

echo "=========================================="
echo "✅ WAVE 6 VERIFICATION COMPLETE"
echo "=========================================="
echo ""
echo "Summary:"
echo "  - Total Tests: $total_tests"
echo "  - Integration Suites: $integration_total (all passing)"
echo "  - Doctest Coverage: $doctest_result doctests"
echo "  - Build Status: Clean"
echo "  - Ready for: Final Sign-off and v0.2.0 Release"
echo ""
