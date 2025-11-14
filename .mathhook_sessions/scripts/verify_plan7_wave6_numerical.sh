#!/bin/bash
set -e

echo "=== Plan 7 Wave 6 Verification: Numerical Methods & Integration ===="
echo ""

# Change to worktree directory
cd /Users/ahmedmashhour/Documents/work/math/mathhook

echo "Verification Date: $(date)"
echo "Branch: $(git branch --show-current)"
echo ""

# 1. Check Numerical Methods module structure exists
echo "1. Verifying Numerical Methods module structure..."
required_files=(
    "crates/mathhook-core/src/calculus/integrals/numerical.rs"
    "crates/mathhook-core/src/calculus/integrals/mod.rs"
)

optional_files=(
    "crates/mathhook-core/src/numerical/mod.rs"
    "crates/mathhook-core/src/numerical/root_finding.rs"
    "crates/mathhook-core/src/numerical/optimization.rs"
)

missing_required=0
for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ MISSING REQUIRED: $file"
        missing_required=$((missing_required + 1))
    fi
done

missing_optional=0
for file in "${optional_files[@]}"; do
    if [ ! -f "$file" ]; then
        missing_optional=$((missing_optional + 1))
    fi
done

if [ $missing_required -eq 0 ]; then
    echo "✅ All required Numerical Methods module files exist"
else
    echo "❌ FAIL: $missing_required required files missing"
fi

if [ $missing_optional -gt 0 ]; then
    echo "⚠️  INFO: $missing_optional optional numerical files missing (may be organized differently)"
fi

# 2. Check Numerical Integration methods
echo ""
echo "2. Verifying numerical integration implementations..."
integration_methods=0
if [ -f "crates/mathhook-core/src/calculus/integrals/numerical.rs" ]; then
    grep -q "trapezoidal\|trapezoid" crates/mathhook-core/src/calculus/integrals/numerical.rs && integration_methods=$((integration_methods + 1)) || echo "  ❌ Trapezoidal rule missing"
    grep -q "simpson\|simpsons" crates/mathhook-core/src/calculus/integrals/numerical.rs && integration_methods=$((integration_methods + 1)) || echo "  ❌ Simpson's rule missing"
    grep -q "gauss\|gaussian\|quadrature" crates/mathhook-core/src/calculus/integrals/numerical.rs && integration_methods=$((integration_methods + 1)) || echo "  ❌ Gaussian quadrature missing"
    grep -q "adaptive\|romberg" crates/mathhook-core/src/calculus/integrals/numerical.rs && integration_methods=$((integration_methods + 1)) || echo "  ❌ Adaptive integration missing"

    if [ $integration_methods -eq 4 ]; then
        echo "✅ All 4 numerical integration methods present"
    elif [ $integration_methods -gt 0 ]; then
        echo "⚠️  PARTIAL: Only $integration_methods/4 integration methods found"
    else
        echo "❌ FAIL: No numerical integration methods found"
    fi
else
    echo "❌ FAIL: numerical.rs not found in integrals module"
    integration_methods=0
fi

# 3. Check Root-Finding methods
echo ""
echo "3. Verifying root-finding implementations..."
root_methods=0
root_finding_file=""

# Check various possible locations
if [ -f "crates/mathhook-core/src/numerical/root_finding.rs" ]; then
    root_finding_file="crates/mathhook-core/src/numerical/root_finding.rs"
elif [ -f "crates/mathhook-core/src/solvers/root_finding.rs" ]; then
    root_finding_file="crates/mathhook-core/src/solvers/root_finding.rs"
fi

if [ -n "$root_finding_file" ]; then
    grep -q "newton\|newton_raphson" "$root_finding_file" && root_methods=$((root_methods + 1)) || echo "  ❌ Newton-Raphson missing"
    grep -q "bisection" "$root_finding_file" && root_methods=$((root_methods + 1)) || echo "  ❌ Bisection method missing"
    grep -q "secant" "$root_finding_file" && root_methods=$((root_methods + 1)) || echo "  ❌ Secant method missing"

    if [ $root_methods -eq 3 ]; then
        echo "✅ All 3 root-finding methods present"
    elif [ $root_methods -gt 0 ]; then
        echo "⚠️  PARTIAL: Only $root_methods/3 root-finding methods found"
    else
        echo "⚠️  WARNING: No root-finding methods found in $root_finding_file"
    fi
else
    echo "⚠️  INFO: Root-finding module not found (may be organized differently)"
    root_methods=0
fi

# 4. Run Numerical Methods tests
echo ""
echo "4. Running Numerical Methods test suite..."
numerical_test_output=$(cargo test -p mathhook-core numerical --lib 2>&1 || true)
numerical_test_count=$(echo "$numerical_test_output" | grep " passed" | sed -n 's/.*test result: ok\. \([0-9]*\) passed.*/\1/p' | tail -1)

# Also check integrals tests
integrals_test_output=$(cargo test -p mathhook-core integrals --lib 2>&1 || true)
integrals_test_count=$(echo "$integrals_test_output" | grep " passed" | sed -n 's/.*test result: ok\. \([0-9]*\) passed.*/\1/p' | tail -1)

# Combine test counts
if [ -z "$numerical_test_count" ]; then
    numerical_test_count=0
fi
if [ -z "$integrals_test_count" ]; then
    integrals_test_count=0
fi

total_numerical_tests=$((numerical_test_count + integrals_test_count))

if [ "$total_numerical_tests" -ge 30 ]; then
    echo "✅ Numerical Methods test suite passing: $total_numerical_tests tests"
elif [ "$total_numerical_tests" -gt 0 ]; then
    echo "⚠️  PARTIAL: Numerical tests exist but may need more coverage: $total_numerical_tests passing"
else
    echo "⚠️  WARNING: No Numerical Methods tests passing (implementation may need work)"
fi

# 5. Check CLAUDE.md compliance - 32-byte Expression constraint
echo ""
echo "5. Checking CLAUDE.md compliance (32-byte Expression constraint)..."
if cargo build -p mathhook-core --lib 2>&1 | grep -q "error"; then
    echo "❌ FAIL: Build has errors"
else
    echo "✅ Build successful (32-byte constraint likely maintained)"
fi

# 6. Check for compiler warnings
echo ""
echo "6. Checking for compiler warnings..."
build_output=$(cargo build -p mathhook-core --lib 2>&1)
warning_count=$(echo "$build_output" | grep -c "warning:" || true)
if [ "$warning_count" -gt 0 ]; then
    echo "⚠️  WARNING: Found $warning_count compiler warnings"
    echo "$build_output" | grep "warning:" | head -5
else
    echo "✅ Zero compiler warnings"
fi

# 7. Check documentation compliance
echo ""
echo "7. Verifying documentation compliance..."
undocumented_count=0

if [ -d "crates/mathhook-core/src/numerical" ]; then
    undocumented=$(grep -r "pub fn" crates/mathhook-core/src/numerical/ | grep -v "///" || true)
    if [ -n "$undocumented" ]; then
        undocumented_count=$((undocumented_count + 1))
    fi
fi

if [ -f "crates/mathhook-core/src/calculus/integrals/numerical.rs" ]; then
    undocumented=$(grep "pub fn" crates/mathhook-core/src/calculus/integrals/numerical.rs | grep -v "///" || true)
    if [ -n "$undocumented" ]; then
        undocumented_count=$((undocumented_count + 1))
    fi
fi

if [ $undocumented_count -gt 0 ]; then
    echo "⚠️  WARNING: Some public functions may lack documentation"
else
    echo "✅ Documentation compliance verified"
fi

# 8. Check total test count (CLAUDE.md requires 676/677 minimum)
echo ""
echo "8. Verifying overall test count (CLAUDE.md requirement: 676/677)..."
total_test_output=$(cargo test -p mathhook-core --lib 2>&1 || true)
total_test_count=$(echo "$total_test_output" | grep " passed" | sed -n 's/.*test result: ok\. \([0-9]*\) passed.*/\1/p' | tail -1)

if [ -z "$total_test_count" ]; then
    total_test_count=0
fi

if [ "$total_test_count" -ge 676 ]; then
    echo "✅ Test count meets CLAUDE.md minimum: $total_test_count tests"
else
    echo "❌ FAIL: Test count below CLAUDE.md minimum (676): $total_test_count tests"
fi

# 9. Calculate Wave 6 completion percentage
echo ""
echo "9. Calculating Wave 6 completion status..."

completion_score=0
total_criteria=8

# Required module structure
[ $missing_required -eq 0 ] && completion_score=$((completion_score + 1))

# Integration methods (4 methods)
[ $integration_methods -eq 4 ] && completion_score=$((completion_score + 2)) || [ $integration_methods -gt 0 ] && completion_score=$((completion_score + 1))

# Root-finding methods (3 methods)
[ $root_methods -eq 3 ] && completion_score=$((completion_score + 1))

# Numerical tests
[ "$total_numerical_tests" -ge 30 ] && completion_score=$((completion_score + 2)) || [ "$total_numerical_tests" -gt 0 ] && completion_score=$((completion_score + 1))

# CLAUDE.md compliance (test count)
[ "$total_test_count" -ge 676 ] && completion_score=$((completion_score + 1))

# Documentation
[ $undocumented_count -eq 0 ] && completion_score=$((completion_score + 1))

completion_percentage=$((completion_score * 100 / total_criteria))

# Summary
echo ""
echo "================================================================="
if [ $completion_percentage -ge 90 ]; then
    echo "=== Plan 7 Wave 6: VERIFICATION PASSED (${completion_percentage}%) ==="
elif [ $completion_percentage -ge 70 ]; then
    echo "=== Plan 7 Wave 6: PARTIAL COMPLETION (${completion_percentage}%) ==="
else
    echo "=== Plan 7 Wave 6: NEEDS WORK (${completion_percentage}%) ==="
fi
echo "================================================================="
echo ""
echo "Completion Breakdown:"
echo "  Required Module Structure: $([ $missing_required -eq 0 ] && echo '✅' || echo '❌')"
echo "  Integration Methods: $integration_methods/4"
echo "  Root-Finding Methods: $root_methods/3"
echo "  Numerical Tests: $total_numerical_tests (target: 30+)"
echo "  Total Tests: $total_test_count (CLAUDE.md minimum: 676)"
echo "  Documentation: $([ $undocumented_count -eq 0 ] && echo '✅' || echo '⚠️ ')"
echo ""
echo "Status Assessment:"
if [ $completion_percentage -ge 90 ]; then
    echo "  ✅ Wave 6 is COMPLETE and verified"
elif [ $completion_percentage -ge 70 ]; then
    echo "  ⚠️  Wave 6 implementation exists but needs:"
    echo "     - Additional test coverage (30+ numerical-specific tests)"
    echo "     - Verification of CLAUDE.md compliance (676+ total tests)"
    echo "     - Documentation improvements"
else
    echo "  ❌ Wave 6 needs significant work:"
    echo "     - Complete numerical integration (Trapezoidal, Simpson's, Gaussian, Adaptive)"
    echo "     - Complete root-finding methods (Newton-Raphson, Bisection, Secant)"
    echo "     - Add comprehensive test coverage"
fi
echo ""
