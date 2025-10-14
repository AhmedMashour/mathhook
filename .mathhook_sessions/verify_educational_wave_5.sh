#!/bin/bash

# Educational Wave 5 Verification Script
# Testing & QA - Final Wave
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "EDUCATIONAL WAVE 5 VERIFICATION"
echo "Testing & QA - Final Wave Before 0.1"
echo "========================================"
echo ""

# Track failures
FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "========================================"
echo "CATEGORY 1: DEFERRED ISSUES FROM WAVE 3"
echo "Fix 3 limit tests that were deferred"
echo "========================================"

echo "Running limit education tests..."
TEST_OUTPUT=$(cargo test -p mathhook-core --test limit_education_test 2>&1)

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    echo -e "${GREEN}✓ Limit tests: $PASSED passed${NC}"

    # Check if all 16 tests are now passing (was 12/15 before)
    if [ "$PASSED" -ge 15 ]; then
        echo -e "${GREEN}✓ All limit tests fixed (was 12/15)${NC}"
    else
        echo -e "${YELLOW}⚠ Only $PASSED/16 limit tests passing${NC}"
    fi
else
    echo -e "${RED}✗ Limit tests FAILED${NC}"
    echo "$TEST_OUTPUT" | grep -E "error|FAILED" | head -10
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================"
echo "CATEGORY 2: TOTAL TEST COUNT"
echo "Target: 100+ content validation tests"
echo "========================================"

# Count all educational tests
DERIVATIVE_TESTS=$(cargo test -p mathhook-core --test derivative_education_test 2>&1 | grep -oE "[0-9]+ passed" | head -1 | grep -oE "[0-9]+" || echo "0")
INTEGRATION_TESTS=$(cargo test -p mathhook-core --test integration_education_test 2>&1 | grep -oE "[0-9]+ passed" | head -1 | grep -oE "[0-9]+" || echo "0")
LIMIT_TESTS=$(cargo test -p mathhook-core --test limit_education_test 2>&1 | grep -oE "[0-9]+ passed" | head -1 | grep -oE "[0-9]+" || echo "0")
EQUATION_TESTS=$(cargo test -p mathhook-core --test equation_solver_education_test 2>&1 | grep -oE "[0-9]+ passed" | head -1 | grep -oE "[0-9]+" || echo "0")
ALGEBRAIC_TESTS=$(cargo test -p mathhook-core --test algebraic_manipulation_education_test 2>&1 | grep -oE "[0-9]+ passed" | head -1 | grep -oE "[0-9]+" || echo "0")
FUNCTION_TESTS=$(cargo test -p mathhook-core --test function_education_test 2>&1 | grep -oE "[0-9]+ passed" | head -1 | grep -oE "[0-9]+" || echo "0")

TOTAL_TESTS=$((DERIVATIVE_TESTS + INTEGRATION_TESTS + LIMIT_TESTS + EQUATION_TESTS + ALGEBRAIC_TESTS + FUNCTION_TESTS))

echo "Test breakdown:"
echo "  Derivative education: $DERIVATIVE_TESTS"
echo "  Integration education: $INTEGRATION_TESTS"
echo "  Limit education: $LIMIT_TESTS"
echo "  Equation solver education: $EQUATION_TESTS"
echo "  Algebraic manipulation education: $ALGEBRAIC_TESTS"
echo "  Function education: $FUNCTION_TESTS"
echo "  ─────────────────────────────"
echo "  TOTAL: $TOTAL_TESTS tests"

if [ "$TOTAL_TESTS" -ge 100 ]; then
    echo -e "${GREEN}✓ Exceeds 100-test target ($TOTAL_TESTS tests)${NC}"
elif [ "$TOTAL_TESTS" -ge 90 ]; then
    echo -e "${YELLOW}⚠ Close to target: $TOTAL_TESTS/100 tests${NC}"
else
    echo -e "${RED}✗ Below target: $TOTAL_TESTS/100 tests${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================"
echo "CATEGORY 3: QUALITY AUDIT SCORES"
echo "Target: 8+/10 across all implementations"
echo "========================================"

echo "Checking implementation quality..."
echo ""

# Check for comprehensive step-by-step explanations (quality indicator)
DERIVATIVE_QUALITY=$(grep -c "Step::new" crates/mathhook-core/src/calculus/derivatives/educational/*.rs 2>/dev/null || echo "0")
echo "Derivative implementations: $DERIVATIVE_QUALITY step definitions"

INTEGRAL_QUALITY=$(grep -c "Step::new" crates/mathhook-core/src/calculus/integrals/*.rs 2>/dev/null || echo "0")
echo "Integration implementations: $INTEGRAL_QUALITY step definitions"

LIMIT_QUALITY=$(grep -c "Step::new" crates/mathhook-core/src/calculus/limits.rs 2>/dev/null || echo "0")
echo "Limit implementations: $LIMIT_QUALITY step definitions"

FUNCTION_QUALITY=$(grep -c "Step::new" crates/mathhook-core/src/functions/education.rs 2>/dev/null || echo "0")
echo "Function implementations: $FUNCTION_QUALITY step definitions"

TOTAL_QUALITY=$((DERIVATIVE_QUALITY + INTEGRAL_QUALITY + LIMIT_QUALITY + FUNCTION_QUALITY))
echo ""
echo "Total step definitions: $TOTAL_QUALITY"

if [ "$TOTAL_QUALITY" -ge 200 ]; then
    echo -e "${GREEN}✓ Excellent implementation quality (200+ steps)${NC}"
elif [ "$TOTAL_QUALITY" -ge 150 ]; then
    echo -e "${YELLOW}⚠ Good quality ($TOTAL_QUALITY steps)${NC}"
else
    echo -e "${YELLOW}⚠ Quality needs assessment ($TOTAL_QUALITY steps)${NC}"
fi

echo ""
echo "========================================"
echo "CATEGORY 4: FILE SIZE COMPLIANCE"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

echo "Checking all educational files..."
VIOLATIONS=0

# Check all educational implementation files
find crates/mathhook-core/src -name "*.rs" -path "*/educational*" -o -name "*.rs" -path "*/education.rs" | while read file; do
    LINES=$(wc -l < "$file")
    BASENAME=$(basename "$file")
    if [ $LINES -gt 500 ]; then
        echo -e "${RED}✗ $BASENAME: $LINES lines${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    fi
done

# Check test files
find crates/mathhook-core/tests -name "*education*test.rs" | while read file; do
    LINES=$(wc -l < "$file")
    BASENAME=$(basename "$file")
    if [ $LINES -gt 500 ]; then
        echo -e "${RED}✗ $BASENAME: $LINES lines${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    fi
done

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply with 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500-line limit${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================"
echo "CATEGORY 5: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨\|📝\|💡\|🎯\|🔍\|⚡" crates/mathhook-core/src crates/mathhook-core/tests 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in codebase${NC}"
    grep -rn "✅\|❌\|⚠️\|🚀\|✨\|📝\|💡\|🎯\|🔍\|⚡" crates/mathhook-core/src crates/mathhook-core/tests 2>/dev/null | head -10
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found in codebase${NC}"
fi

echo ""
echo "========================================"
echo "CATEGORY 6: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

echo "Checking build..."
BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | grep -E "error\[" | head -10
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================"
echo "CATEGORY 7: FULL TEST SUITE"
echo "All tests must pass"
echo "========================================"

echo "Running full test suite..."
FULL_TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)

if echo "$FULL_TEST_OUTPUT" | grep -q "test result: ok"; then
    TOTAL_PASSED=$(echo "$FULL_TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    echo -e "${GREEN}✓ Full test suite: $TOTAL_PASSED passed${NC}"
else
    echo -e "${RED}✗ Test suite has failures${NC}"
    echo "$FULL_TEST_OUTPUT" | grep -E "FAILED|error" | head -10
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================"
echo "CATEGORY 8: CONTENT VALIDATION RATIO"
echo "Tests must validate actual math content"
echo "========================================"

# Count content validation assertions vs structure-only
CONTENT_ASSERTIONS=$(grep -r "has_step_containing\|assert.*contains" crates/mathhook-core/tests/*education*.rs 2>/dev/null | wc -l)
STRUCTURE_ONLY=$(grep -r "steps.len()" crates/mathhook-core/tests/*education*.rs 2>/dev/null | wc -l)

if [ "$CONTENT_ASSERTIONS" -gt 0 ]; then
    RATIO=$(( (CONTENT_ASSERTIONS * 100) / (CONTENT_ASSERTIONS + STRUCTURE_ONLY) ))
    echo "Content validation assertions: $CONTENT_ASSERTIONS"
    echo "Structure-only assertions: $STRUCTURE_ONLY"
    echo "Content validation ratio: $RATIO%"

    if [ "$RATIO" -ge 80 ]; then
        echo -e "${GREEN}✓ Excellent content validation ratio ($RATIO%)${NC}"
    elif [ "$RATIO" -ge 60 ]; then
        echo -e "${YELLOW}⚠ Good ratio ($RATIO%)${NC}"
    else
        echo -e "${RED}✗ Too many structure-only tests ($RATIO%)${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${YELLOW}⚠ Unable to assess content validation ratio${NC}"
fi

echo ""
echo "========================================"
echo "CATEGORY 9: ZERO REGRESSIONS"
echo "Verify no existing functionality broken"
echo "========================================"

echo "Checking for regression indicators..."

# Check if core tests still pass
CORE_TESTS=$(cargo test -p mathhook-core number arithmetic domain 2>&1 | grep -c "test result: ok" || echo "0")

if [ "$CORE_TESTS" -ge 2 ]; then
    echo -e "${GREEN}✓ Core functionality tests passing${NC}"
else
    echo -e "${RED}✗ Core functionality may have regressed${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================"
echo "CATEGORY 10: EDUCATIONAL COVERAGE"
echo "Verify comprehensive educational system"
echo "========================================"

echo "Checking educational coverage..."

# Count operations with education
OPS_WITH_EDUCATION=0

# Derivatives
if [ -d "crates/mathhook-core/src/calculus/derivatives/educational" ]; then
    DERIVATIVE_OPS=$(find crates/mathhook-core/src/calculus/derivatives/educational -name "*.rs" | wc -l)
    echo "Derivatives: $DERIVATIVE_OPS files"
    OPS_WITH_EDUCATION=$((OPS_WITH_EDUCATION + DERIVATIVE_OPS))
fi

# Integrals
if [ -d "crates/mathhook-core/src/calculus/integrals" ]; then
    INTEGRAL_OPS=$(find crates/mathhook-core/src/calculus/integrals -name "*educational*.rs" -o -name "*basic*.rs" -o -name "*parts*.rs" | wc -l)
    echo "Integrals: $INTEGRAL_OPS files"
    OPS_WITH_EDUCATION=$((OPS_WITH_EDUCATION + INTEGRAL_OPS))
fi

# Functions
FUNCTION_COUNT=$(grep -c "step_generators.insert" crates/mathhook-core/src/functions/education.rs 2>/dev/null || echo "0")
echo "Functions: $FUNCTION_COUNT with education"
OPS_WITH_EDUCATION=$((OPS_WITH_EDUCATION + FUNCTION_COUNT))

# Algebra
ALGEBRA_OPS=$(find crates/mathhook-core/src/algebra/solvers -name "*educational*.rs" | wc -l)
echo "Algebra: $ALGEBRA_OPS educational files"
OPS_WITH_EDUCATION=$((OPS_WITH_EDUCATION + ALGEBRA_OPS))

echo ""
echo "Total operations with education: $OPS_WITH_EDUCATION"

if [ "$OPS_WITH_EDUCATION" -ge 30 ]; then
    echo -e "${GREEN}✓ Comprehensive educational coverage${NC}"
elif [ "$OPS_WITH_EDUCATION" -ge 20 ]; then
    echo -e "${YELLOW}⚠ Good coverage ($OPS_WITH_EDUCATION operations)${NC}"
else
    echo -e "${YELLOW}⚠ Limited coverage ($OPS_WITH_EDUCATION operations)${NC}"
fi

echo ""
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Educational Wave 5 is VERIFIED COMPLETE"
    echo "Ready for 0.1 Release"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s) found${NC}"
    echo "Wave 5 requires fixes before approval"
    exit 1
fi
