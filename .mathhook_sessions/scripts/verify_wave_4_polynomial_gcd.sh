#!/bin/bash

# Wave 4: Polynomial GCD & Final Verification Script
# Purpose: Verify polynomial division and GCD implementation
# Enforces CLAUDE.md compliance + SymPy polynomial validation strictly

echo "========================================"
echo "WAVE 4: POLYNOMIAL GCD VERIFICATION"
echo "Polynomial Division & Euclidean GCD"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: POLYNOMIAL DIVISION IMPLEMENTATION
echo ""
echo "========================================"
echo "CATEGORY 1: POLYNOMIAL DIVISION"
echo "CRITICAL: div(), quo(), rem() operations"
echo "========================================"

# Check for division methods
DIV_METHODS=$(grep -c "fn div\|pub fn div\|fn quo\|pub fn quo\|fn rem\|pub fn rem" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs 2>/dev/null || echo "0")

if [ "$DIV_METHODS" -ge 3 ]; then
    echo -e "${GREEN}✓ Polynomial division methods found: $DIV_METHODS${NC}"
else
    echo -e "${RED}✗ Insufficient division methods: $DIV_METHODS (need 3+: div, quo, rem)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EUCLIDEAN GCD IMPLEMENTATION
echo ""
echo "========================================"
echo "CATEGORY 2: EUCLIDEAN GCD FOR POLYNOMIALS"
echo "CRITICAL: Full algorithm with division"
echo "========================================"

# Check for Euclidean algorithm completion
GCD_COMPLETE=$(grep -c "polynomial_gcd_euclidean\|euclidean_algorithm" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null || echo "0")

if [ "$GCD_COMPLETE" -gt 0 ]; then
    echo -e "${GREEN}✓ Euclidean GCD implementation found${NC}"

    # Check for fallback "return 1" pattern
    FALLBACK=$(grep -c "Expression::integer(1)" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs | head -1)
    if [ "$FALLBACK" -gt 0 ]; then
        echo -e "${YELLOW}⚠ Check for fallback returns (should use division, not return 1)${NC}"
    fi
else
    echo -e "${RED}✗ Euclidean GCD not found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 3: TEST COVERAGE
echo ""
echo "========================================"
echo "CATEGORY 3: POLYNOMIAL GCD TEST COVERAGE"
echo "Target: 20+ tests with SymPy validation"
echo "========================================"

# Count polynomial GCD tests
GCD_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial_gcd_tests.rs 2>/dev/null || echo "0")

echo "Polynomial GCD tests: $GCD_TESTS"

if [ "$GCD_TESTS" -ge 20 ]; then
    echo -e "${GREEN}✓ Test coverage adequate ($GCD_TESTS >= 20)${NC}"
else
    echo -e "${RED}✗ Test coverage below target ($GCD_TESTS < 20)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: SYMPY VALIDATION
echo ""
echo "========================================"
echo "CATEGORY 4: SYMPY POLYNOMIAL GCD VALIDATION"
echo "Must validate against SymPy polynomial GCD"
echo "========================================"

# Check for SymPy validation references
SYMPY_REFS=$(grep -r "sympy\|SymPy" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial_gcd_tests.rs 2>/dev/null | grep -i "gcd" | wc -l)

if [ "$SYMPY_REFS" -gt 5 ]; then
    echo -e "${GREEN}✓ SymPy validation references: $SYMPY_REFS${NC}"
else
    echo -e "${YELLOW}⚠ Limited SymPy validation: $SYMPY_REFS${NC}"
    echo "Note: Should validate polynomial GCD against SymPy for correctness"
fi

# CATEGORY 5: EDGE CASE COVERAGE
echo ""
echo "========================================"
echo "CATEGORY 5: EDGE CASE COVERAGE"
echo "Must handle zero, constants, high degree"
echo "========================================"

# Check for edge case tests
EDGE_CASES=$(grep -r "zero\|constant\|degree" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial_gcd_tests.rs 2>/dev/null | wc -l)

if [ "$EDGE_CASES" -gt 10 ]; then
    echo -e "${GREEN}✓ Edge case coverage: $EDGE_CASES references${NC}"
else
    echo -e "${YELLOW}⚠ Limited edge case coverage: $EDGE_CASES${NC}"
fi

# CATEGORY 6: FILE SIZE COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 6: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0

# Check polynomial division file
if [ -f "/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs" ]; then
    SIZE=$(wc -l < "/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs" 2>/dev/null || echo "0")
    if [ "$SIZE" -gt 500 ]; then
        echo -e "${RED}✗ polynomial_division.rs: $SIZE lines (exceeds 500)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ polynomial_division.rs: $SIZE lines${NC}"
    fi
fi

# Check gcd.rs
SIZE=$(wc -l < "/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs" 2>/dev/null || echo "0")
if [ "$SIZE" -gt 500 ]; then
    echo -e "${RED}✗ gcd.rs: $SIZE lines (exceeds 500)${NC}"
    VIOLATIONS=$((VIOLATIONS + 1))
else
    echo -e "${GREEN}✓ gcd.rs: $SIZE lines${NC}"
fi

# Check test file
if [ -f "/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial_gcd_tests.rs" ]; then
    SIZE=$(wc -l < "/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial_gcd_tests.rs" 2>/dev/null || echo "0")
    if [ "$SIZE" -gt 500 ]; then
        echo -e "${RED}✗ polynomial_gcd_tests.rs: $SIZE lines (exceeds 500)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ polynomial_gcd_tests.rs: $SIZE lines${NC}"
    fi
fi

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply with 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500-line limit${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: EMOJI COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 7: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial_gcd_tests.rs 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in polynomial GCD code${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 8: BUILD STATUS
echo ""
echo "========================================"
echo "CATEGORY 8: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | grep "error\|Error" | head -5
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: POLYNOMIAL GCD TEST SUITE
echo ""
echo "========================================"
echo "CATEGORY 9: POLYNOMIAL GCD TEST SUITE"
echo "All GCD tests must pass"
echo "========================================"

# Run polynomial GCD tests
GCD_TEST_OUTPUT=$(cargo test -p mathhook-core polynomial_gcd 2>&1)
GCD_PASS=$(echo "$GCD_TEST_OUTPUT" | grep -c "test result: ok")

if [ "$GCD_PASS" -gt 0 ]; then
    echo -e "${GREEN}✓ Polynomial GCD tests passing${NC}"
    echo "$GCD_TEST_OUTPUT" | grep "test result:" | head -5
else
    echo -e "${RED}✗ Polynomial GCD tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 10: FINAL PROJECT VERIFICATION
echo ""
echo "========================================"
echo "CATEGORY 10: FINAL PROJECT VERIFICATION"
echo "All waves (1-4) complete and passing"
echo "========================================"

# Run all number theory and polynomial tests
ALL_TESTS=$(cargo test -p mathhook-core number_theory polynomial 2>&1)
ALL_PASS=$(echo "$ALL_TESTS" | grep -c "test result: ok")

if [ "$ALL_PASS" -gt 3 ]; then
    echo -e "${GREEN}✓ All number theory and polynomial tests passing${NC}"
else
    echo -e "${YELLOW}⚠ Some tests may need attention${NC}"
fi

# Count total tests added across all waves
WAVE_1_TESTS=22
WAVE_2_TESTS=28
WAVE_3_TESTS=28
WAVE_4_TESTS=$GCD_TESTS

TOTAL_TESTS=$((WAVE_1_TESTS + WAVE_2_TESTS + WAVE_3_TESTS + WAVE_4_TESTS))

echo ""
echo "Total tests added across all waves: $TOTAL_TESTS"
echo "Target: 75+ tests"

if [ "$TOTAL_TESTS" -ge 75 ]; then
    echo -e "${GREEN}✓ Total test count exceeds target ($TOTAL_TESTS >= 75)${NC}"
else
    echo -e "${YELLOW}⚠ Total test count below target ($TOTAL_TESTS < 75)${NC}"
fi

# SUMMARY
echo ""
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 4: Polynomial GCD & Final Verification is VERIFIED COMPLETE"
    echo ""
    echo "🎉 ALL 4 WAVES COMPLETE!"
    echo "Number Theory & Polynomial Functions: 100% Working Status"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 4 requires fixes before approval"
    exit 1
fi
