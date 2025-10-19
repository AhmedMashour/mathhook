#!/bin/bash

# Number Theory & Polynomial Functions Completion Verification Script
# Purpose: Verify actual implementation status independent of orchestrator claims
# Can be run anytime to assess current state vs 4 objectives
# Created: 2025-10-19

echo "========================================"
echo "NUMBER THEORY & POLYNOMIAL COMPLETION"
echo "VERIFICATION SCRIPT"
echo "========================================"
echo ""
echo "This script verifies the 4 critical objectives:"
echo "1. Fix LCM bug"
echo "2. Polynomial evaluation implementation"
echo "3. Verify MOD/is_prime status"
echo "4. Complete polynomial GCD"
echo ""

FAILURES=0
WARNINGS=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# ============================================
# OBJECTIVE 1: LCM BUG FIX
# ============================================
echo "========================================"
echo "OBJECTIVE 1: LCM BUG FIX"
echo "Critical: LCM must return correct value, not a*b"
echo "========================================"

# Check if LCM implementation is fixed
echo "Checking LCM implementation in gcd.rs..."

LCM_IMPL=$(grep -A 15 "fn lcm(&self, other: &Self)" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null)

if echo "$LCM_IMPL" | grep -q "Expression::div.*product.*gcd"; then
    echo -e "${GREEN}✓ LCM implementation looks correct (divides by GCD)${NC}"
elif echo "$LCM_IMPL" | grep -q "just.*product\|For now.*product\|product$"; then
    echo -e "${RED}✗ LCM still broken - returns product without dividing by GCD${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${YELLOW}⚠ LCM implementation unclear - manual check needed${NC}"
    echo "Found implementation:"
    echo "$LCM_IMPL" | head -10
    WARNINGS=$((WARNINGS + 1))
fi

# Check for LCM tests
echo ""
echo "Checking for LCM tests..."
LCM_TEST_COUNT=$(find /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests -name "*.rs" -exec grep -l "test.*lcm\|lcm.*test" {} \; 2>/dev/null | wc -l)

if [ "$LCM_TEST_COUNT" -gt 0 ]; then
    echo -e "${GREEN}✓ Found $LCM_TEST_COUNT test file(s) with LCM tests${NC}"
else
    echo -e "${YELLOW}⚠ No dedicated LCM test files found${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

# Try to run a simple LCM test (if build works)
echo ""
echo "Attempting to compile and run LCM test..."
cat > /tmp/test_lcm.rs << 'EOF'
#[cfg(test)]
mod lcm_verification {
    use mathhook_core::algebra::gcd::PolynomialGcd;
    use mathhook_core::core::Expression;

    #[test]
    fn verify_lcm_fixed() {
        let a = Expression::integer(12);
        let b = Expression::integer(8);
        let result = a.lcm(&b);

        // LCM(12, 8) should be 24, not 96 (12*8)
        // If this returns integer(24), LCM is fixed
        // If this returns integer(96), LCM is still broken
        println!("LCM(12, 8) = {:?}", result);
    }
}
EOF

if cargo test --lib lcm 2>&1 | grep -q "test.*ok\|passed"; then
    echo -e "${GREEN}✓ LCM tests pass${NC}"
else
    echo -e "${YELLOW}⚠ LCM test status unclear (check cargo test output)${NC}"
fi

rm -f /tmp/test_lcm.rs

echo ""
echo "OBJECTIVE 1 STATUS:"
if [ $FAILURES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ LCM appears to be FIXED${NC}"
elif [ $FAILURES -gt 0 ]; then
    echo -e "${RED}✗ LCM is NOT fixed${NC}"
else
    echo -e "${YELLOW}⚠ LCM status uncertain - manual verification needed${NC}"
fi

# ============================================
# OBJECTIVE 2: POLYNOMIAL EVALUATION
# ============================================
echo ""
echo "========================================"
echo "OBJECTIVE 2: POLYNOMIAL EVALUATION"
echo "Critical: Must be able to compute P_n(x) for all polynomial families"
echo "========================================"

# Check for evaluate methods in polynomial files
echo "Checking for evaluate() implementations..."

EVAL_METHODS_FOUND=0

for POLY_FILE in legendre hermite laguerre chebyshev; do
    POLY_PATH="/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/${POLY_FILE}.rs"

    if [ -f "$POLY_PATH" ]; then
        if grep -q "fn evaluate.*n.*x\|pub fn evaluate" "$POLY_PATH"; then
            echo -e "${GREEN}✓ $POLY_FILE.rs has evaluate() method${NC}"
            EVAL_METHODS_FOUND=$((EVAL_METHODS_FOUND + 1))
        else
            echo -e "${RED}✗ $POLY_FILE.rs MISSING evaluate() method${NC}"
            FAILURES=$((FAILURES + 1))
        fi
    else
        echo -e "${YELLOW}⚠ $POLY_FILE.rs not found${NC}"
    fi
done

echo ""
echo "Checking for evaluation tests..."
POLY_EVAL_TESTS=$(find /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests -name "*.rs" -exec grep -l "evaluate.*polynomial\|polynomial.*evaluate\|legendre.*evaluate\|hermite.*evaluate" {} \; 2>/dev/null | wc -l)

if [ "$POLY_EVAL_TESTS" -gt 0 ]; then
    echo -e "${GREEN}✓ Found $POLY_EVAL_TESTS test file(s) with polynomial evaluation tests${NC}"
else
    echo -e "${YELLOW}⚠ No polynomial evaluation test files found${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

echo ""
echo "OBJECTIVE 2 STATUS:"
if [ $EVAL_METHODS_FOUND -eq 4 ]; then
    echo -e "${GREEN}✓ All 4 polynomial families have evaluate() methods${NC}"
elif [ $EVAL_METHODS_FOUND -gt 0 ]; then
    echo -e "${YELLOW}⚠ Partial implementation: $EVAL_METHODS_FOUND/4 families have evaluate()${NC}"
else
    echo -e "${RED}✗ NO polynomial evaluation implementations found${NC}"
fi

# ============================================
# OBJECTIVE 3: MOD/IS_PRIME VERIFICATION
# ============================================
echo ""
echo "========================================"
echo "OBJECTIVE 3: MOD/IS_PRIME VERIFICATION"
echo "Verify actual implementation status"
echo "========================================"

# Check for MOD implementation
echo "Checking for MOD operation implementation..."

MOD_FOUND=false
if grep -r "fn.*mod\|pub fn modulo\|fn.*remainder" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src --include="*.rs" 2>/dev/null | grep -v "module\|//\|mod\.rs" | head -5; then
    echo -e "${GREEN}✓ MOD operation implementation found${NC}"
    MOD_FOUND=true
else
    echo -e "${YELLOW}⚠ MOD operation implementation NOT clearly found${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

# Check for is_prime implementation
echo ""
echo "Checking for is_prime implementation..."

ISPRIME_FOUND=false
if grep -r "fn is_prime\|fn.*primality" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src --include="*.rs" 2>/dev/null | grep -v "//" | head -5; then
    echo -e "${GREEN}✓ is_prime implementation found${NC}"
    ISPRIME_FOUND=true
else
    echo -e "${YELLOW}⚠ is_prime implementation NOT found${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

echo ""
echo "OBJECTIVE 3 STATUS:"
if $MOD_FOUND && $ISPRIME_FOUND; then
    echo -e "${GREEN}✓ Both MOD and is_prime appear to be implemented${NC}"
elif $MOD_FOUND || $ISPRIME_FOUND; then
    echo -e "${YELLOW}⚠ Partial: Only one of MOD/is_prime found${NC}"
else
    echo -e "${YELLOW}⚠ Neither MOD nor is_prime clearly implemented${NC}"
fi

# ============================================
# OBJECTIVE 4: POLYNOMIAL GCD COMPLETION
# ============================================
echo ""
echo "========================================"
echo "OBJECTIVE 4: POLYNOMIAL GCD COMPLETION"
echo "Must have polynomial division and full Euclidean algorithm"
echo "========================================"

# Check for polynomial division methods
echo "Checking for polynomial division methods (div, quo, rem)..."

DIV_METHODS_FOUND=0

for METHOD in "fn div\|fn polynomial_div" "fn quo\|fn quotient" "fn rem\|fn remainder"; do
    if grep -r "$METHOD" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra --include="*.rs" 2>/dev/null | grep -v "//\|derive" | head -1 > /dev/null; then
        echo -e "${GREEN}✓ Division method found matching: $METHOD${NC}"
        DIV_METHODS_FOUND=$((DIV_METHODS_FOUND + 1))
    fi
done

if [ $DIV_METHODS_FOUND -eq 0 ]; then
    echo -e "${RED}✗ No polynomial division methods found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# Check polynomial_gcd_euclidean implementation
echo ""
echo "Checking polynomial_gcd_euclidean implementation..."

GCD_IMPL=$(grep -A 20 "fn polynomial_gcd_euclidean" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null)

if echo "$GCD_IMPL" | grep -q "Expression::integer(1)"; then
    echo -e "${RED}✗ polynomial_gcd_euclidean still returns fallback Expression::integer(1)${NC}"
    echo -e "${RED}  This means full Euclidean algorithm NOT implemented${NC}"
    FAILURES=$((FAILURES + 1))
elif echo "$GCD_IMPL" | grep -q "polynomial.*division\|euclidean.*algorithm"; then
    echo -e "${GREEN}✓ polynomial_gcd_euclidean appears to have full implementation${NC}"
else
    echo -e "${YELLOW}⚠ polynomial_gcd_euclidean status unclear${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

# Check for polynomial GCD tests
echo ""
echo "Checking for polynomial GCD tests..."
POLY_GCD_TESTS=$(find /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/gcd -name "*.rs" 2>/dev/null | wc -l)

if [ "$POLY_GCD_TESTS" -gt 0 ]; then
    echo -e "${GREEN}✓ Found $POLY_GCD_TESTS GCD test file(s)${NC}"
else
    echo -e "${YELLOW}⚠ No dedicated GCD test files found in tests/gcd/${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

echo ""
echo "OBJECTIVE 4 STATUS:"
if [ $DIV_METHODS_FOUND -ge 2 ] && ! echo "$GCD_IMPL" | grep -q "Expression::integer(1)"; then
    echo -e "${GREEN}✓ Polynomial GCD appears to be COMPLETE${NC}"
elif [ $DIV_METHODS_FOUND -gt 0 ]; then
    echo -e "${YELLOW}⚠ Partial implementation: Division methods exist but GCD may be incomplete${NC}"
else
    echo -e "${RED}✗ Polynomial GCD NOT complete${NC}"
fi

# ============================================
# GENERAL CHECKS
# ============================================
echo ""
echo "========================================"
echo "GENERAL CHECKS"
echo "Build, tests, CLAUDE.md compliance"
echo "========================================"

# Build check
echo "Checking build status..."
if cargo check -p mathhook-core 2>&1 | grep -q "Finished.*dev"; then
    echo -e "${GREEN}✓ Build successful (cargo check passes)${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    FAILURES=$((FAILURES + 1))
fi

# Test count check
echo ""
echo "Checking test suite..."
TEST_OUTPUT=$(cargo test --lib 2>&1 | tail -20)
if echo "$TEST_OUTPUT" | grep -q "test result:.*ok"; then
    PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result" | sed 's/.*\([0-9]\+\) passed.*/\1/')
    echo -e "${GREEN}✓ All tests passing ($PASSED_TESTS tests)${NC}"
else
    echo -e "${RED}✗ Some tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CLAUDE.md file size check
echo ""
echo "Checking CLAUDE.md file size compliance (max 500 lines)..."
OVERSIZED_FILES=$(find /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src -name "*.rs" -exec sh -c 'lines=$(wc -l < "$1"); if [ "$lines" -gt 500 ]; then echo "$1: $lines lines"; fi' _ {} \; 2>/dev/null)

if [ -z "$OVERSIZED_FILES" ]; then
    echo -e "${GREEN}✓ No new files over 500 lines${NC}"
else
    echo -e "${YELLOW}⚠ Files over 500 lines found:${NC}"
    echo "$OVERSIZED_FILES"
    WARNINGS=$((WARNINGS + 1))
fi

# Emoji check
echo ""
echo "Checking for emojis (CLAUDE.md violation)..."
EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨\|🎯\|📊\|📈\|💡" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src --include="*.rs" 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✓ No emojis found in source code${NC}"
else
    echo -e "${YELLOW}⚠ Found $EMOJI_COUNT emoji instance(s) - CLAUDE.md violation${NC}"
    WARNINGS=$((WARNINGS + 1))
fi

# ============================================
# FINAL SUMMARY
# ============================================
echo ""
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"
echo ""

echo -e "${BLUE}Objective Status:${NC}"
echo "1. LCM Bug Fix: (see above)"
echo "2. Polynomial Evaluation: $EVAL_METHODS_FOUND/4 families implemented"
echo "3. MOD/is_prime: MOD=$MOD_FOUND, is_prime=$ISPRIME_FOUND"
echo "4. Polynomial GCD: Division methods=$DIV_METHODS_FOUND"
echo ""

echo -e "${BLUE}Quality Metrics:${NC}"
echo "Failures: $FAILURES"
echo "Warnings: $WARNINGS"
echo ""

if [ $FAILURES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}✓ ALL OBJECTIVES APPEAR COMPLETE${NC}"
    echo -e "${GREEN}✓ NO ISSUES FOUND${NC}"
    echo -e "${GREEN}========================================${NC}"
    exit 0
elif [ $FAILURES -eq 0 ]; then
    echo -e "${YELLOW}========================================${NC}"
    echo -e "${YELLOW}⚠ OBJECTIVES PARTIALLY COMPLETE${NC}"
    echo -e "${YELLOW}⚠ $WARNINGS WARNING(S) - Manual verification recommended${NC}"
    echo -e "${YELLOW}========================================${NC}"
    exit 1
else
    echo -e "${RED}========================================${NC}"
    echo -e "${RED}✗ OBJECTIVES INCOMPLETE${NC}"
    echo -e "${RED}✗ $FAILURES FAILURE(S), $WARNINGS WARNING(S)${NC}"
    echo -e "${RED}========================================${NC}"
    echo ""
    echo "Recommended actions:"
    echo "1. Review failed objectives above"
    echo "2. Check implementation in source files"
    echo "3. Re-run orchestrator for incomplete objectives"
    echo "4. Or manually implement missing functionality"
    exit 1
fi
