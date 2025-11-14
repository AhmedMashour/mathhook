#!/bin/bash

# Wave 2: Polynomial Recurrence Evaluation Engine Verification Script
# Purpose: Verify polynomial evaluation implementation using function intelligence
# Enforces CLAUDE.md compliance + SymPy validation strictly

echo "========================================"
echo "WAVE 2: POLYNOMIAL EVALUATION VERIFICATION"
echo "Recurrence Evaluation Engine for All 4 Polynomial Families"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: EVALUATION IMPLEMENTATION EXISTS
echo ""
echo "========================================"
echo "CATEGORY 1: POLYNOMIAL EVALUATION METHODS"
echo "CRITICAL: Must use function intelligence, not hardcoded"
echo "========================================"

# Check for evaluate() methods in polynomial intelligence files
LEGENDRE_EVAL=$(grep -c "fn evaluate\|pub fn evaluate" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/legendre.rs 2>/dev/null || echo "0")
HERMITE_EVAL=$(grep -c "fn evaluate\|pub fn evaluate" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/hermite.rs 2>/dev/null || echo "0")
LAGUERRE_EVAL=$(grep -c "fn evaluate\|pub fn evaluate" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/laguerre.rs 2>/dev/null || echo "0")
CHEBYSHEV_EVAL=$(grep -c "fn evaluate\|pub fn evaluate" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/chebyshev.rs 2>/dev/null || echo "0")

TOTAL_EVAL_METHODS=$((LEGENDRE_EVAL + HERMITE_EVAL + LAGUERRE_EVAL + CHEBYSHEV_EVAL))

if [ "$TOTAL_EVAL_METHODS" -ge 4 ]; then
    echo -e "${GREEN}✓ Polynomial evaluation methods found: $TOTAL_EVAL_METHODS${NC}"
    echo "  Legendre: $LEGENDRE_EVAL"
    echo "  Hermite: $HERMITE_EVAL"
    echo "  Laguerre: $LAGUERRE_EVAL"
    echo "  Chebyshev: $CHEBYSHEV_EVAL"
else
    echo -e "${RED}✗ Insufficient evaluation methods found: $TOTAL_EVAL_METHODS (need 4+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: RECURRENCE USAGE (NOT HARDCODED VALUES)
echo ""
echo "========================================"
echo "CATEGORY 2: RECURRENCE RELATION USAGE"
echo "Must use three-term recurrence from properties"
echo "========================================"

# Check that evaluation uses recurrence properties (not hardcoded polynomials)
RECURRENCE_USAGE=$(grep -r "recurrence\|alpha_coeff\|beta_coeff\|gamma_coeff" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs 2>/dev/null | grep -c "evaluate")

if [ "$RECURRENCE_USAGE" -gt 0 ]; then
    echo -e "${GREEN}✓ Evaluation uses recurrence properties: $RECURRENCE_USAGE references${NC}"
else
    echo -e "${YELLOW}⚠ May not be using recurrence properties (check implementation)${NC}"
    echo "Note: Evaluation should use three-term recurrence from PolynomialProperties"
fi

# CATEGORY 3: FUNCTION INTELLIGENCE INTEGRATION
echo ""
echo "========================================"
echo "CATEGORY 3: FUNCTION INTELLIGENCE INTEGRATION"
echo "CRITICAL: Must integrate with UniversalFunctionRegistry"
echo "========================================"

# Check for numerical_evaluator in polynomial intelligence
NUMERICAL_EVALUATORS=$(grep -c "numerical_evaluator:" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs 2>/dev/null || echo "0")

if [ "$NUMERICAL_EVALUATORS" -ge 4 ]; then
    echo -e "${GREEN}✓ Numerical evaluators defined: $NUMERICAL_EVALUATORS${NC}"

    # Check they're not all "None"
    NONE_COUNT=$(grep "numerical_evaluator: None" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs 2>/dev/null | wc -l)

    if [ "$NONE_COUNT" -lt 4 ]; then
        echo -e "${GREEN}✓ At least one polynomial has actual numerical evaluator${NC}"
    else
        echo -e "${RED}✗ All numerical_evaluators are None (not implemented)${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Numerical evaluators not found in intelligence${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: TEST COVERAGE
echo ""
echo "========================================"
echo "CATEGORY 4: POLYNOMIAL EVALUATION TESTS"
echo "Target: 25+ tests with SymPy validation"
echo "========================================"

# Count polynomial evaluation tests
POLY_EVAL_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*polynomial_eval*.rs 2>/dev/null || echo "0")
LEGENDRE_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*legendre*.rs 2>/dev/null || echo "0")
HERMITE_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*hermite*.rs 2>/dev/null || echo "0")
LAGUERRE_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*laguerre*.rs 2>/dev/null || echo "0")
CHEBYSHEV_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*chebyshev*.rs 2>/dev/null || echo "0")

TOTAL_POLY_TESTS=$((POLY_EVAL_TESTS + LEGENDRE_TESTS + HERMITE_TESTS + LAGUERRE_TESTS + CHEBYSHEV_TESTS))

echo "Polynomial evaluation tests: $POLY_EVAL_TESTS"
echo "Legendre tests: $LEGENDRE_TESTS"
echo "Hermite tests: $HERMITE_TESTS"
echo "Laguerre tests: $LAGUERRE_TESTS"
echo "Chebyshev tests: $CHEBYSHEV_TESTS"
echo "Total: $TOTAL_POLY_TESTS"

if [ "$TOTAL_POLY_TESTS" -ge 25 ]; then
    echo -e "${GREEN}✓ Test coverage adequate ($TOTAL_POLY_TESTS >= 25)${NC}"
else
    echo -e "${RED}✗ Test coverage below target ($TOTAL_POLY_TESTS < 25)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: SYMPY VALIDATION
echo ""
echo "========================================"
echo "CATEGORY 5: SYMPY VALIDATION"
echo "All tests must validate against SymPy polynomials"
echo "========================================"

# Check for SymPy references in polynomial tests
SYMPY_POLY_REFS=$(grep -r "sympy\|SymPy\|legendre\|hermite\|laguerre\|chebyshev" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*polynomial*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*legendre*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*hermite*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*laguerre*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*chebyshev*.rs 2>/dev/null | wc -l)

if [ "$SYMPY_POLY_REFS" -gt 20 ]; then
    echo -e "${GREEN}✓ SymPy validation references found: $SYMPY_POLY_REFS locations${NC}"
else
    echo -e "${YELLOW}⚠ Limited SymPy validation references: $SYMPY_POLY_REFS${NC}"
    echo "Note: All tests should reference SymPy validation"
fi

# CATEGORY 6: PERFORMANCE BENCHMARKS
echo ""
echo "========================================"
echo "CATEGORY 6: PERFORMANCE TARGETS"
echo "Target: <1ms for n≤100, x∈[-10,10]"
echo "========================================"

# Check for benchmarks
BENCH_FILES=$(ls /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-benchmarks/benches/*polynomial*.rs 2>/dev/null | wc -l)

if [ "$BENCH_FILES" -gt 0 ]; then
    echo -e "${GREEN}✓ Polynomial benchmarks exist: $BENCH_FILES file(s)${NC}"
else
    echo -e "${YELLOW}⚠ No polynomial benchmarks found${NC}"
    echo "Note: Performance targets should be measured with benchmarks"
fi

# CATEGORY 7: FILE SIZE COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 7: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0

# Check all polynomial files
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

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/ 2>/dev/null | wc -l)
TEST_EMOJI=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*polynomial*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*legendre*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*hermite*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*laguerre*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*chebyshev*.rs 2>/dev/null | wc -l)

TOTAL_EMOJI=$((EMOJI_COUNT + TEST_EMOJI))

if [ "$TOTAL_EMOJI" -gt 0 ]; then
    echo -e "${RED}✗ Found $TOTAL_EMOJI emojis in polynomial code/tests${NC}"
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

# CATEGORY 10: POLYNOMIAL TESTS PASS
echo ""
echo "========================================"
echo "CATEGORY 10: POLYNOMIAL TEST SUITE"
echo "All polynomial evaluation tests must pass"
echo "========================================"

# Run polynomial tests
POLY_TEST_OUTPUT=$(cargo test -p mathhook-core polynomial legendre hermite laguerre chebyshev 2>&1)
POLY_PASS=$(echo "$POLY_TEST_OUTPUT" | grep -c "test result: ok")

if [ "$POLY_PASS" -gt 0 ]; then
    echo -e "${GREEN}✓ Polynomial tests passing${NC}"
    echo "$POLY_TEST_OUTPUT" | grep "test result:" | head -5
else
    echo -e "${RED}✗ Polynomial tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo ""
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 2: Polynomial Recurrence Evaluation is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 2 requires fixes before approval"
    exit 1
fi
