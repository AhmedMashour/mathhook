#!/bin/bash

# Wave 3: Symbolic Polynomial Expansion Verification Script
# Purpose: Verify symbolic expansion implementation using function intelligence
# Enforces CLAUDE.md compliance + SymPy symbolic validation strictly

echo "========================================"
echo "WAVE 3: SYMBOLIC EXPANSION VERIFICATION"
echo "Symbolic Polynomial Expansion for All 5 Families"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: SYMBOLIC EXPANSION IMPLEMENTATION
echo ""
echo "========================================"
echo "CATEGORY 1: SYMBOLIC EXPANSION METHODS"
echo "CRITICAL: Must use function intelligence, integrate with Expression system"
echo "========================================"

# Check for expand_symbolic methods
LEGENDRE_EXPAND=$(grep -c "fn expand_symbolic\|pub fn expand_symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/legendre.rs 2>/dev/null || echo "0")
HERMITE_EXPAND=$(grep -c "fn expand_symbolic\|pub fn expand_symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/hermite.rs 2>/dev/null || echo "0")
LAGUERRE_EXPAND=$(grep -c "fn expand_symbolic\|pub fn expand_symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/laguerre.rs 2>/dev/null || echo "0")
CHEBYSHEV_EXPAND=$(grep -c "fn expand_symbolic\|pub fn expand_symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/chebyshev.rs 2>/dev/null || echo "0")

# Check evaluation module
EVAL_EXPAND=$(grep -c "fn expand_symbolic\|pub fn expand_symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/evaluation.rs 2>/dev/null || echo "0")

TOTAL_EXPAND_METHODS=$((LEGENDRE_EXPAND + HERMITE_EXPAND + LAGUERRE_EXPAND + CHEBYSHEV_EXPAND + EVAL_EXPAND))

if [ "$TOTAL_EXPAND_METHODS" -ge 5 ]; then
    echo -e "${GREEN}✓ Symbolic expansion methods found: $TOTAL_EXPAND_METHODS${NC}"
    echo "  Legendre: $LEGENDRE_EXPAND"
    echo "  Hermite: $HERMITE_EXPAND"
    echo "  Laguerre: $LAGUERRE_EXPAND"
    echo "  Chebyshev: $CHEBYSHEV_EXPAND"
    echo "  Evaluation module: $EVAL_EXPAND"
else
    echo -e "${RED}✗ Insufficient expansion methods found: $TOTAL_EXPAND_METHODS (need 5+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EXPRESSION SYSTEM INTEGRATION
echo ""
echo "========================================"
echo "CATEGORY 2: EXPRESSION SYSTEM INTEGRATION"
echo "Must return Expression, not hardcoded strings"
echo "========================================"

# Check that expansion returns Expression type
EXPRESSION_RETURNS=$(grep -A 5 "fn expand_symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs 2>/dev/null | grep -c "-> Expression")

if [ "$EXPRESSION_RETURNS" -ge 5 ]; then
    echo -e "${GREEN}✓ Symbolic expansion returns Expression: $EXPRESSION_RETURNS methods${NC}"
else
    echo -e "${YELLOW}⚠ Check return types (expected Expression, found $EXPRESSION_RETURNS)${NC}"
    echo "Note: All expand_symbolic() should return Expression, not String"
fi

# CATEGORY 3: RECURRENCE USAGE (NOT HARDCODED FORMULAS)
echo ""
echo "========================================"
echo "CATEGORY 3: RECURRENCE-BASED EXPANSION"
echo "Must build symbolically using recurrence, not hardcoded formulas"
echo "========================================"

# Check for recurrence-based construction
RECURRENCE_REFS=$(grep -r "recurrence\|p_prev\|p_curr" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs 2>/dev/null | grep -c "expand_symbolic")

if [ "$RECURRENCE_REFS" -gt 0 ]; then
    echo -e "${GREEN}✓ Expansion uses recurrence-based construction: $RECURRENCE_REFS references${NC}"
else
    echo -e "${YELLOW}⚠ May not be using recurrence (check implementation)${NC}"
    echo "Note: Should build P_n symbolically using recurrence, not hardcoded coefficients"
fi

# CATEGORY 4: TEST COVERAGE
echo ""
echo "========================================"
echo "CATEGORY 4: SYMBOLIC EXPANSION TESTS"
echo "Target: 15+ tests with symbolic/numerical consistency"
echo "========================================"

# Count symbolic expansion tests
SYMBOLIC_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*symbolic*.rs 2>/dev/null || echo "0")
POLY_SYMBOLIC_TESTS=$(grep -c "expand_symbolic\|symbolic" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial*.rs 2>/dev/null || echo "0")

TOTAL_SYMBOLIC_TESTS=$((SYMBOLIC_TESTS + POLY_SYMBOLIC_TESTS))

echo "Symbolic expansion tests: $SYMBOLIC_TESTS"
echo "Polynomial tests with symbolic: $POLY_SYMBOLIC_TESTS"
echo "Total: $TOTAL_SYMBOLIC_TESTS"

if [ "$TOTAL_SYMBOLIC_TESTS" -ge 15 ]; then
    echo -e "${GREEN}✓ Test coverage adequate ($TOTAL_SYMBOLIC_TESTS >= 15)${NC}"
else
    echo -e "${RED}✗ Test coverage below target ($TOTAL_SYMBOLIC_TESTS < 15)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: SYMBOLIC VS NUMERICAL CONSISTENCY
echo ""
echo "========================================"
echo "CATEGORY 5: SYMBOLIC/NUMERICAL CONSISTENCY"
echo "Tests must verify symbolic.evaluate() == numerical()"
echo "========================================"

# Check for consistency validation in tests
CONSISTENCY_CHECKS=$(grep -r "evaluate\|consistency" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*symbolic*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial*.rs 2>/dev/null | wc -l)

if [ "$CONSISTENCY_CHECKS" -gt 10 ]; then
    echo -e "${GREEN}✓ Symbolic/numerical consistency checks found: $CONSISTENCY_CHECKS${NC}"
else
    echo -e "${YELLOW}⚠ Limited consistency checks: $CONSISTENCY_CHECKS${NC}"
    echo "Note: Tests should verify expand_symbolic(n).evaluate(x) == numerical(n, x)"
fi

# CATEGORY 6: SYMPY SYMBOLIC VALIDATION
echo ""
echo "========================================"
echo "CATEGORY 6: SYMPY SYMBOLIC VALIDATION"
echo "Must validate symbolic forms against SymPy"
echo "========================================"

# Check for SymPy symbolic references
SYMPY_SYMBOLIC=$(grep -r "sympy\|SymPy" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*symbolic*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/polynomial*.rs 2>/dev/null | grep -i "legendre\|hermite\|laguerre\|chebyshev" | wc -l)

if [ "$SYMPY_SYMBOLIC" -gt 5 ]; then
    echo -e "${GREEN}✓ SymPy symbolic validation references: $SYMPY_SYMBOLIC${NC}"
else
    echo -e "${YELLOW}⚠ Limited SymPy symbolic validation: $SYMPY_SYMBOLIC${NC}"
    echo "Note: Should validate symbolic forms against SymPy polynomial expansions"
fi

# CATEGORY 7: FILE SIZE COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 7: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0

# Check all polynomial files (may have grown)
for file in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs; do
    if [ -f "$file" ]; then
        SIZE=$(wc -l < "$file" 2>/dev/null || echo "0")
        FILENAME=$(basename "$file")
        if [ "$SIZE" -gt 500 ]; then
            echo -e "${RED}✗ $FILENAME: $SIZE lines (exceeds 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $FILENAME: $SIZE lines${NC}"
        fi
    fi
done

# Check test files
for file in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*symbolic*.rs; do
    if [ -f "$file" ]; then
        SIZE=$(wc -l < "$file" 2>/dev/null || echo "0")
        FILENAME=$(basename "$file")
        if [ "$SIZE" -gt 500 ]; then
            echo -e "${RED}✗ $FILENAME: $SIZE lines (exceeds 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $FILENAME: $SIZE lines${NC}"
        fi
    fi
done

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply with 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500-line limit${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: EMOJI COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 8: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/ /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*symbolic*.rs 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in polynomial/symbolic code${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 9: BUILD STATUS
echo ""
echo "========================================"
echo "CATEGORY 9: BUILD STATUS"
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

# CATEGORY 10: SYMBOLIC TEST SUITE
echo ""
echo "========================================"
echo "CATEGORY 10: SYMBOLIC TEST SUITE"
echo "All symbolic expansion tests must pass"
echo "========================================"

# Run symbolic tests
SYMBOLIC_TEST_OUTPUT=$(cargo test -p mathhook-core symbolic polynomial 2>&1)
SYMBOLIC_PASS=$(echo "$SYMBOLIC_TEST_OUTPUT" | grep -c "test result: ok")

if [ "$SYMBOLIC_PASS" -gt 0 ]; then
    echo -e "${GREEN}✓ Symbolic tests passing${NC}"
    echo "$SYMBOLIC_TEST_OUTPUT" | grep "test result:" | head -5
else
    echo -e "${RED}✗ Symbolic tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo ""
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 3: Symbolic Polynomial Expansion is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 3 requires fixes before approval"
    exit 1
fi
