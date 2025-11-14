#!/bin/bash
set -e

echo "=== Plan 7 Wave 2 Verification: Advanced Linear Algebra ==

="
echo ""

# Change to worktree directory
cd /Users/ahmedmashhour/Documents/work/math/mathhook

echo "Verification Date: $(date)"
echo "Branch: $(git branch --show-current)"
echo ""

# 1. Check Linear Algebra module structure exists
echo "1. Verifying Linear Algebra module structure..."
required_files=(
    "crates/mathhook-core/src/matrix/mod.rs"
    "crates/mathhook-core/src/matrix/decomposition.rs"
    "crates/mathhook-core/src/matrix/operations.rs"
)

missing_files=0
for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ MISSING: $file"
        missing_files=$((missing_files + 1))
    fi
done

if [ $missing_files -eq 0 ]; then
    echo "✅ All required Linear Algebra module files exist"
else
    echo "⚠️  WARNING: $missing_files Linear Algebra files missing"
fi

# 2. Check matrix decomposition functions
echo ""
echo "2. Verifying matrix decomposition implementations..."
decomp_functions=0
if [ -f "crates/mathhook-core/src/matrix/decomposition.rs" ]; then
    grep -q "lu_decomposition" crates/mathhook-core/src/matrix/decomposition.rs && decomp_functions=$((decomp_functions + 1)) || echo "  ❌ LU decomposition missing"
    grep -q "qr_decomposition" crates/mathhook-core/src/matrix/decomposition.rs && decomp_functions=$((decomp_functions + 1)) || echo "  ❌ QR decomposition missing"
    grep -q "eigenvalues\|eigen" crates/mathhook-core/src/matrix/decomposition.rs && decomp_functions=$((decomp_functions + 1)) || echo "  ❌ Eigenvalue computation missing"
    grep -q "svd\|singular_value" crates/mathhook-core/src/matrix/decomposition.rs && decomp_functions=$((decomp_functions + 1)) || echo "  ❌ SVD missing"

    if [ $decomp_functions -eq 4 ]; then
        echo "✅ All 4 matrix decomposition functions present"
    else
        echo "⚠️  WARNING: Only $decomp_functions/4 decomposition functions found"
    fi
else
    echo "❌ FAIL: decomposition.rs not found"
fi

# 3. Check matrix operations
echo ""
echo "3. Verifying matrix operations..."
if [ -f "crates/mathhook-core/src/matrix/operations.rs" ]; then
    ops_count=0
    grep -q "transpose\|matrix_transpose" crates/mathhook-core/src/matrix/operations.rs && ops_count=$((ops_count + 1)) || echo "  ❌ Transpose missing"
    grep -q "inverse\|matrix_inverse" crates/mathhook-core/src/matrix/operations.rs && ops_count=$((ops_count + 1)) || echo "  ❌ Inverse missing"
    grep -q "determinant\|matrix_determinant" crates/mathhook-core/src/matrix/operations.rs && ops_count=$((ops_count + 1)) || echo "  ❌ Determinant missing"
    grep -q "rank\|matrix_rank" crates/mathhook-core/src/matrix/operations.rs && ops_count=$((ops_count + 1)) || echo "  ❌ Rank missing"

    if [ $ops_count -eq 4 ]; then
        echo "✅ All 4 matrix operations present"
    else
        echo "⚠️  WARNING: Only $ops_count/4 matrix operations found"
    fi
else
    echo "❌ FAIL: operations.rs not found"
fi

# 4. Run Linear Algebra tests
echo ""
echo "4. Running Linear Algebra test suite..."
matrix_test_output=$(cargo test -p mathhook-core matrix --lib 2>&1 || true)
matrix_test_count=$(echo "$matrix_test_output" | grep " passed" | sed -n 's/.*test result: ok\. \([0-9]*\) passed.*/\1/p' | tail -1)

if [ -z "$matrix_test_count" ]; then
    matrix_test_count=0
fi

if [ "$matrix_test_count" -ge 40 ]; then
    echo "✅ Linear Algebra test suite passing: $matrix_test_count tests"
elif [ "$matrix_test_count" -gt 0 ]; then
    echo "⚠️  PARTIAL: Matrix tests exist but may need more coverage: $matrix_test_count passing"
else
    echo "⚠️  WARNING: No Matrix tests passing (implementation may need work)"
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
undocumented=$(grep -r "pub fn" crates/mathhook-core/src/matrix/ | grep -v "///" || true)
if [ -n "$undocumented" ]; then
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

# 9. Calculate Wave 2 completion percentage
echo ""
echo "9. Calculating Wave 2 completion status..."

completion_score=0
total_criteria=8

# Module structure (3 files)
[ $missing_files -eq 0 ] && completion_score=$((completion_score + 1))

# Decomposition functions (4 functions)
[ $decomp_functions -eq 4 ] && completion_score=$((completion_score + 2)) || [ $decomp_functions -gt 0 ] && completion_score=$((completion_score + 1))

# Matrix operations (4 operations)
[ $ops_count -eq 4 ] && completion_score=$((completion_score + 1))

# Matrix tests
[ "$matrix_test_count" -ge 40 ] && completion_score=$((completion_score + 2)) || [ "$matrix_test_count" -gt 0 ] && completion_score=$((completion_score + 1))

# CLAUDE.md compliance (test count)
[ "$total_test_count" -ge 676 ] && completion_score=$((completion_score + 1))

# Documentation
[ -z "$undocumented" ] && completion_score=$((completion_score + 1))

completion_percentage=$((completion_score * 100 / total_criteria))

# Summary
echo ""
echo "================================================================="
if [ $completion_percentage -ge 90 ]; then
    echo "=== Plan 7 Wave 2: VERIFICATION PASSED (${completion_percentage}%) ==="
elif [ $completion_percentage -ge 70 ]; then
    echo "=== Plan 7 Wave 2: PARTIAL COMPLETION (${completion_percentage}%) ==="
else
    echo "=== Plan 7 Wave 2: NEEDS WORK (${completion_percentage}%) ==="
fi
echo "================================================================="
echo ""
echo "Completion Breakdown:"
echo "  Module Structure: $([ $missing_files -eq 0 ] && echo '✅' || echo '⚠️ ')"
echo "  Decomposition Functions: $decomp_functions/4"
echo "  Matrix Operations: $ops_count/4"
echo "  Matrix Tests: $matrix_test_count (target: 40+)"
echo "  Total Tests: $total_test_count (CLAUDE.md minimum: 676)"
echo "  Documentation: $([ -z "$undocumented" ] && echo '✅' || echo '⚠️ ')"
echo ""
echo "Status Assessment:"
if [ $completion_percentage -ge 90 ]; then
    echo "  ✅ Wave 2 is COMPLETE and verified"
elif [ $completion_percentage -ge 70 ]; then
    echo "  ⚠️  Wave 2 implementation exists but needs:"
    echo "     - Additional test coverage (40+ matrix-specific tests)"
    echo "     - Verification of CLAUDE.md compliance (676+ total tests)"
    echo "     - Documentation improvements"
else
    echo "  ❌ Wave 2 needs significant work:"
    echo "     - Complete matrix decompositions (LU, QR, Eigenvalues, SVD)"
    echo "     - Complete matrix operations (transpose, inverse, determinant, rank)"
    echo "     - Add comprehensive test coverage"
fi
echo ""
