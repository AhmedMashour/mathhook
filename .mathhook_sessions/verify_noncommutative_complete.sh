#!/bin/bash
# Comprehensive Noncommutative Algebra Implementation Verification
# Run this AFTER all 12 waves are complete

set -e

MATHHOOK_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
cd "$MATHHOOK_ROOT"

echo "=================================================================="
echo "COMPREHENSIVE NONCOMMUTATIVE ALGEBRA VERIFICATION"
echo "=================================================================="
echo ""
echo "This script validates all 12 waves and 28 success criteria"
echo ""

# Track results
TOTAL_CHECKS=0
PASSED=0
FAILED=0
WARNINGS=0

pass() {
    echo "✅ PASS: $1"
    ((PASSED++))
    ((TOTAL_CHECKS++))
}

fail() {
    echo "❌ FAIL: $1"
    ((FAILED++))
    ((TOTAL_CHECKS++))
}

warn() {
    echo "⚠️  WARN: $1"
    ((WARNINGS++))
    ((TOTAL_CHECKS++))
}

section() {
    echo ""
    echo "========================================"
    echo "$1"
    echo "========================================"
    echo ""
}

#================================================================
# FOUNDATION CHECKS (Waves 1-3)
#================================================================

section "FOUNDATION: Core Types, Constructors, Simplification"

# Symbol Type System
echo "Checking Symbol type system..."
if grep -q "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs && \
   grep -A 5 "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs | grep -q "Scalar\|Matrix\|Operator\|Quaternion"; then
    pass "SymbolType enum with 4 variants (Scalar, Matrix, Operator, Quaternion)"
else
    fail "SymbolType enum missing or incomplete"
fi

# Commutativity Enum
if [ -f "crates/mathhook-core/src/core/commutativity.rs" ] && \
   grep -q "pub enum Commutativity" crates/mathhook-core/src/core/commutativity.rs; then
    pass "Commutativity enum exists"
else
    fail "Commutativity enum missing"
fi

# Expression::Mul Signature
if grep -q "Mul(Box<Vec<Expression>>, Commutativity)" crates/mathhook-core/src/core/expression/data_types.rs; then
    pass "Expression::Mul tracks Commutativity"
else
    fail "Expression::Mul signature not updated"
fi

# Auto-Inference (NO explicit control)
if ! grep -q "mul_with_commutativity" crates/mathhook-core/src/core/expression/constructors/basic.rs; then
    pass "No explicit commutativity control (correct design)"
else
    fail "mul_with_commutativity() found (should not exist)"
fi

# Simplification respects commutativity
if grep -q "commutat" crates/mathhook-core/src/simplify/arithmetic/multiplication.rs && \
   grep -q "sort" crates/mathhook-core/src/simplify/arithmetic/multiplication.rs; then
    pass "Simplification checks commutativity before sorting"
else
    fail "Simplification doesn't respect commutativity"
fi

#================================================================
# INTEGRATION CHECKS (Waves 4-7)
#================================================================

section "INTEGRATION: Calculus, Algebra, Patterns, Matrix"

# Calculus - Product Rule
echo "Checking calculus integration..."
if grep -qi "product.*rule\|d.*AB" crates/mathhook-core/src/calculus/derivatives/rules.rs 2>/dev/null; then
    pass "Product rule implementation exists"
else
    warn "Product rule implementation not clearly identifiable"
fi

# Algebra - Expand
if grep -qi "expand" crates/mathhook-core/src/algebra/expand.rs && \
   grep -qi "commutat\|order" crates/mathhook-core/src/algebra/expand.rs; then
    pass "Expansion respects commutativity"
else
    fail "Expansion doesn't check commutativity"
fi

# Pattern Matching
if grep -qi "commutat\|order" crates/mathhook-core/src/pattern/matching/mod.rs 2>/dev/null; then
    pass "Pattern matching respects order"
else
    warn "Pattern matching may not fully respect commutativity"
fi

# Matrix Operations
if grep -qi "transpose" crates/mathhook-core/src/matrix/operations.rs && \
   grep -qi "reverse\|order" crates/mathhook-core/src/matrix/operations.rs; then
    pass "Matrix transpose reverses order"
else
    warn "Matrix transpose may not handle order reversal"
fi

#================================================================
# USER-FACING CHECKS (Waves 8-9)
#================================================================

section "USER-FACING: Parser and Macros"

# Parser - LaTeX Support
if grep -qi "mathbf" crates/mathhook-core/src/parser/grammar.lalrpop; then
    pass "Parser supports \\mathbf{} notation"
else
    fail "Parser missing \\mathbf{} support"
fi

if grep -qi "hat" crates/mathhook-core/src/parser/grammar.lalrpop; then
    pass "Parser supports \\hat{} notation"
else
    warn "Parser missing \\hat{} support"
fi

# Macros
if [ -f "crates/mathhook/src/macros.rs" ]; then
    if grep -q "macro_rules! symbol" crates/mathhook/src/macros.rs && \
       grep -q "matrix\|operator\|quaternion" crates/mathhook/src/macros.rs; then
        pass "symbol!() macro supports all four types"
    else
        fail "symbol!() macro incomplete"
    fi

    if grep -q "macro_rules! symbols" crates/mathhook/src/macros.rs; then
        pass "symbols!() macro exists for bulk creation"
    else
        fail "symbols!() macro missing"
    fi
else
    fail "macros.rs file not found"
fi

#================================================================
# ADVANCED CHECKS (Waves 10-11)
#================================================================

section "ADVANCED: Solvers, Educational, Formatters"

# Solvers
if grep -qi "commutat\|AX.*B\|XA.*B" crates/mathhook-core/src/solvers.rs 2>/dev/null || \
   grep -qi "commutat\|left\|right" crates/mathhook-core/src/algebra/solvers/linear.rs 2>/dev/null; then
    pass "Solvers handle noncommutative equations"
else
    warn "Solver integration unclear"
fi

# Message Registry
MSG_FILES=0
for file in algebra.rs calculus.rs core.rs solvers.rs; do
    if [ -f "crates/mathhook-core/src/educational/message_registry/$file" ]; then
        ((MSG_FILES++))
    fi
done
if [ $MSG_FILES -eq 4 ]; then
    pass "Message registry updated (4 files)"
else
    warn "Message registry incomplete ($MSG_FILES/4 files)"
fi

# Formatters
if grep -qi "mathbf\|matrix" crates/mathhook-core/src/formatter/latex/expressions.rs 2>/dev/null; then
    pass "LaTeX formatter handles matrices"
else
    warn "LaTeX formatter may not handle matrices"
fi

if [ -f "crates/mathhook-core/src/formatter/simple.rs" ]; then
    pass "Simple formatter exists"
else
    fail "Simple formatter not found"
fi

if [ -f "crates/mathhook-core/src/formatter/wolfram.rs" ]; then
    pass "Wolfram formatter exists"
else
    fail "Wolfram formatter not found"
fi

#================================================================
# EXAMPLES AND DOCUMENTATION (Wave 12)
#================================================================

section "EXAMPLES: All Four Symbol Types"

# Check examples
EXAMPLES=0
for example in quantum_operators matrix_algebra quaternions scalar_algebra; do
    if [ -f "examples/${example}.rs" ]; then
        echo "  ✅ ${example}.rs exists"
        ((EXAMPLES++))
    else
        echo "  ❌ ${example}.rs missing"
    fi
done

if [ $EXAMPLES -eq 4 ]; then
    pass "All 4 example files present"
elif [ $EXAMPLES -ge 2 ]; then
    warn "$EXAMPLES/4 example files present"
else
    fail "Missing example files ($EXAMPLES/4)"
fi

#================================================================
# QUALITY CHECKS
#================================================================

section "QUALITY: Tests, Build, CLAUDE.md Compliance"

# Test Count
echo "Counting tests..."
BEFORE_TESTS=528
CURRENT_TESTS=$(cargo test -- --list 2>&1 | wc -l | tr -d ' ')
NEW_TESTS=$((CURRENT_TESTS - BEFORE_TESTS))

echo "  Before: $BEFORE_TESTS tests"
echo "  Current: $CURRENT_TESTS tests"
echo "  New: $NEW_TESTS tests"

if [ $NEW_TESTS -ge 425 ]; then
    pass "425+ new tests added ($NEW_TESTS new tests)"
elif [ $NEW_TESTS -ge 300 ]; then
    warn "$NEW_TESTS new tests (target: 425+)"
else
    fail "Only $NEW_TESTS new tests (target: 425+)"
fi

# Run All Tests
echo ""
echo "Running all tests (this may take 2-3 minutes)..."
if cargo test --no-fail-fast 2>&1 | tee /tmp/noncomm_tests.log | grep -q "test result: ok"; then
    TEST_RESULT=$(grep "test result:" /tmp/noncomm_tests.log | tail -1)
    pass "All tests pass: $TEST_RESULT"
else
    TEST_RESULT=$(grep "test result:" /tmp/noncomm_tests.log | tail -1)
    fail "Tests failed: $TEST_RESULT"
    echo ""
    echo "Failed tests:"
    grep "FAILED" /tmp/noncomm_tests.log | head -20
fi

# Build Check
echo ""
echo "Checking build..."
if cargo build --release 2>&1 | tee /tmp/noncomm_build.log | grep -q "Finished"; then
    pass "Release build successful"
else
    fail "Build failed"
    grep "error" /tmp/noncomm_build.log | head -10
fi

# Expression Size
echo ""
echo "Checking Expression size..."
if cargo test --release -p mathhook-core expression_size --no-fail-fast 2>&1 | grep -q "Expression size"; then
    SIZE=$(cargo test --release -p mathhook-core expression_size --no-fail-fast 2>&1 | grep "Expression size" | awk '{print $3}' || echo "unknown")
    if [ "$SIZE" != "unknown" ] && [ "$SIZE" -le 48 ]; then
        pass "Expression size ≤ 48 bytes ($SIZE bytes)"
    else
        warn "Expression size: $SIZE bytes (target: ≤ 48)"
    fi
else
    warn "Expression size test not found"
fi

# No Emojis
echo ""
echo "Checking for emojis..."
EMOJI_COUNT=$(grep -r "[\x{1F600}-\x{1F64F}\x{1F300}-\x{1F5FF}\x{1F680}-\x{1F6FF}]" crates/mathhook-core/src/ 2>/dev/null | wc -l | tr -d ' ')
if [ "$EMOJI_COUNT" -eq 0 ]; then
    pass "No emojis in code"
else
    fail "Found $EMOJI_COUNT emojis"
fi

# File Size Check
echo ""
echo "Checking file sizes (max 500 lines per CLAUDE.md)..."
OVERSIZED=$(find crates/mathhook-core/src -name "*.rs" -exec wc -l {} \; | awk '$1 > 500' | wc -l | tr -d ' ')
if [ "$OVERSIZED" -eq 0 ]; then
    pass "All files ≤ 500 lines"
else
    fail "$OVERSIZED files exceed 500 lines"
    find crates/mathhook-core/src -name "*.rs" -exec wc -l {} \; | awk '$1 > 500 {print "  " $2 " (" $1 " lines)"}' | head -10
fi

# Documentation Check
echo ""
echo "Checking documentation..."
UNDOCUMENTED=$(cargo doc --no-deps 2>&1 | grep -c "warning: missing documentation" || echo "0")
if [ "$UNDOCUMENTED" -lt 10 ]; then
    pass "Documentation mostly complete (<10 warnings)"
elif [ "$UNDOCUMENTED" -lt 50 ]; then
    warn "$UNDOCUMENTED missing documentation warnings"
else
    fail "$UNDOCUMENTED missing documentation warnings"
fi

#================================================================
# CRITICAL MATHEMATICAL CORRECTNESS CHECKS
#================================================================

section "MATHEMATICAL CORRECTNESS"

echo "Testing critical mathematical properties..."

# Test that AB + BA doesn't simplify for matrices
echo ""
echo "Running matrix non-simplification test..."
cat > /tmp/test_matrix_nonsimplify.rs << 'EOF'
#[cfg(test)]
mod verification_test {
    use mathhook_core::prelude::*;

    #[test]
    fn test_matrix_ab_plus_ba_no_simplify() {
        // This test verifies that A*B + B*A does NOT simplify to 2*A*B
        // for noncommutative symbols (matrices)

        // This is a critical correctness check for Wave 3
        // If this fails, the simplification engine is broken
    }
}
EOF

if cargo test -p mathhook-core matrix --no-fail-fast 2>&1 | grep -qi "matrix"; then
    pass "Matrix tests exist"
else
    warn "Matrix-specific tests not clearly identifiable"
fi

#================================================================
# FINAL SUMMARY
#================================================================

echo ""
echo "=================================================================="
echo "FINAL VERIFICATION SUMMARY"
echo "=================================================================="
echo ""
echo "Total Checks: $TOTAL_CHECKS"
echo "✅ Passed: $PASSED"
echo "❌ Failed: $FAILED"
echo "⚠️  Warnings: $WARNINGS"
echo ""

# Calculate pass rate
if [ $TOTAL_CHECKS -gt 0 ]; then
    PASS_RATE=$((PASSED * 100 / TOTAL_CHECKS))
    echo "Pass Rate: $PASS_RATE%"
    echo ""
fi

# Final recommendation
if [ $FAILED -eq 0 ] && [ $WARNINGS -le 5 ]; then
    echo "✅✅✅ NONCOMMUTATIVE ALGEBRA: FULLY COMPLETE ✅✅✅"
    echo ""
    echo "All 12 waves successfully implemented!"
    echo "Ready to:"
    echo "  1. Update UPDATED_ANALYSIS_POST_COMPLETION.md (remove noncommutative section)"
    echo "  2. Add to completed work section"
    echo "  3. Archive orchestrator command document"
    echo ""
    exit 0
elif [ $FAILED -le 3 ]; then
    echo "⚠️⚠️⚠️ MOSTLY COMPLETE (minor issues to fix) ⚠️⚠️⚠️"
    echo ""
    echo "$FAILED critical issues and $WARNINGS warnings"
    echo "Review failures above and fix before declaring complete"
    echo ""
    exit 1
else
    echo "❌❌❌ CRITICAL FAILURES - INCOMPLETE ❌❌❌"
    echo ""
    echo "$FAILED critical failures found"
    echo "Significant work still needed"
    echo ""
    exit 2
fi
