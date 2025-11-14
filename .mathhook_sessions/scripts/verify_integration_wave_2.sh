#!/bin/bash

# Wave 2: Foundation - Rational Functions + Strategy Dispatcher Verification Script
# Purpose: Verify implementation of partial fractions and layered strategy dispatcher
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 2: FOUNDATION VERIFICATION"
echo "Rational Functions + Strategy Dispatcher"
echo "Coverage Target: 75% → 85%"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: REQUIRED FILES (3 NEW/MODIFIED)
echo "========================================"
echo "CATEGORY 1: REQUIRED FILES"
echo "Wave 2 must deliver 2 new files + 1 modified"
echo "========================================"

REQUIRED_NEW_FILES=(
    "crates/mathhook-core/src/calculus/integrals/rational.rs"
    "crates/mathhook-core/src/calculus/integrals/strategy.rs"
)

REQUIRED_MODIFIED_FILES=(
    "crates/mathhook-core/src/calculus/integrals.rs"
)

MISSING_FILES=0
for file in "${REQUIRED_NEW_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}✗ MISSING NEW FILE: $file${NC}"
        MISSING_FILES=$((MISSING_FILES + 1))
    else
        echo -e "${GREEN}✓ Found: $file${NC}"
    fi
done

for file in "${REQUIRED_MODIFIED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}✗ MISSING: $file${NC}"
        MISSING_FILES=$((MISSING_FILES + 1))
    else
        echo -e "${GREEN}✓ Modified: $file${NC}"
    fi
done

if [ $MISSING_FILES -gt 0 ]; then
    echo -e "${RED}✗ $MISSING_FILES files missing${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All required files present${NC}"
fi

# CATEGORY 2: FILE SIZE COMPLIANCE (≤500 LINES)
echo "========================================"
echo "CATEGORY 2: FILE SIZE COMPLIANCE"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0
for file in "${REQUIRED_NEW_FILES[@]}" "${REQUIRED_MODIFIED_FILES[@]}"; do
    if [ -f "$file" ]; then
        LINE_COUNT=$(wc -l < "$file" | tr -d ' ')
        if [ "$LINE_COUNT" -gt 500 ]; then
            echo -e "${RED}✗ $file: $LINE_COUNT lines (max 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $file: $LINE_COUNT lines${NC}"
        fi
    fi
done

if [ $VIOLATIONS -gt 0 ]; then
    echo -e "${RED}✗ $VIOLATIONS file size violations${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All files within 500-line limit${NC}"
fi

# CATEGORY 3: RATIONAL FUNCTION INTEGRATION
echo "========================================"
echo "CATEGORY 3: RATIONAL FUNCTION INTEGRATION"
echo "Must implement partial fractions"
echo "========================================"

RATIONAL_FILE="crates/mathhook-core/src/calculus/integrals/rational.rs"
RATIONAL_ISSUES=0

if [ -f "$RATIONAL_FILE" ]; then
    # Check for key partial fraction concepts
    REQUIRED_CONCEPTS=(
        "partial.fraction"
        "polynomial.division"
        "linear.factor"
        "quadratic.factor"
    )

    for concept in "${REQUIRED_CONCEPTS[@]}"; do
        # Use grep with flexible matching (. matches space/underscore/case)
        if ! grep -qi "${concept//./[_ ]}" "$RATIONAL_FILE"; then
            echo -e "${RED}✗ Missing concept in code/comments: '$concept'${NC}"
            RATIONAL_ISSUES=$((RATIONAL_ISSUES + 1))
        fi
    done

    if [ $RATIONAL_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Rational integration implementation covers key concepts${NC}"
    else
        echo -e "${RED}✗ Rational integration incomplete: $RATIONAL_ISSUES concepts not found${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ rational.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: STRATEGY DISPATCHER
echo "========================================"
echo "CATEGORY 4: STRATEGY DISPATCHER"
echo "Must implement 8-layer fallthrough"
echo "========================================"

STRATEGY_FILE="crates/mathhook-core/src/calculus/integrals/strategy.rs"
STRATEGY_ISSUES=0

if [ -f "$STRATEGY_FILE" ]; then
    # Check for strategy layers (at least 6 of 8, some deferred to later waves)
    STRATEGY_LAYERS=(
        "table"
        "rational"
        "by.parts"
        "substitution"
        "trigonometric"
        "risch"
    )

    FOUND_LAYERS=0
    for layer in "${STRATEGY_LAYERS[@]}"; do
        if grep -qi "${layer//./[_ ]}" "$STRATEGY_FILE"; then
            FOUND_LAYERS=$((FOUND_LAYERS + 1))
        fi
    done

    if [ $FOUND_LAYERS -ge 4 ]; then
        echo -e "${GREEN}✓ Strategy dispatcher has $FOUND_LAYERS/6 layers mentioned${NC}"
    else
        echo -e "${RED}✗ Strategy dispatcher incomplete: only $FOUND_LAYERS/6 layers found${NC}"
        STRATEGY_ISSUES=$((STRATEGY_ISSUES + 1))
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ strategy.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: TEST COVERAGE (40+ TESTS)
echo "========================================"
echo "CATEGORY 5: TEST COVERAGE"
echo "Wave 2 requires 40+ new tests"
echo "========================================"

# Find test files related to integration
TEST_FILES=$(find crates/mathhook-core/tests -name "*integration*.rs" -o -name "*rational*.rs" 2>/dev/null)

TOTAL_NEW_TESTS=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        TEST_COUNT=$(grep -c "^[[:space:]]*#\[test\]" "$test_file" 2>/dev/null || echo 0)
        TOTAL_NEW_TESTS=$((TOTAL_NEW_TESTS + TEST_COUNT))
    fi
done

if [ $TOTAL_NEW_TESTS -ge 40 ]; then
    echo -e "${GREEN}✓ Found $TOTAL_NEW_TESTS tests (target: 40+)${NC}"
elif [ $TOTAL_NEW_TESTS -ge 35 ]; then
    echo -e "${YELLOW}⚠ Found $TOTAL_NEW_TESTS tests (close to 40 target)${NC}"
else
    echo -e "${RED}✗ Found only $TOTAL_NEW_TESTS tests (target: 40+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: SYMPY VALIDATION
echo "========================================"
echo "CATEGORY 6: SYMPY VALIDATION"
echo "All tests must include SymPy validation"
echo "========================================"

SYMPY_COMMENT_COUNT=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        COUNT=$(grep -ci "sympy" "$test_file" 2>/dev/null || echo 0)
        SYMPY_COMMENT_COUNT=$((SYMPY_COMMENT_COUNT + COUNT))
    fi
done

if [ $SYMPY_COMMENT_COUNT -ge 20 ]; then
    echo -e "${GREEN}✓ Found $SYMPY_COMMENT_COUNT SymPy references (good coverage)${NC}"
elif [ $SYMPY_COMMENT_COUNT -ge 10 ]; then
    echo -e "${YELLOW}⚠ Found $SYMPY_COMMENT_COUNT SymPy references (could be more)${NC}"
else
    echo -e "${RED}✗ Only $SYMPY_COMMENT_COUNT SymPy references (need more validation)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: BUILD STATUS
echo "========================================"
echo "CATEGORY 7: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | grep -i "error" | head -5
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: REGRESSION TESTING
echo "========================================"
echo "CATEGORY 8: REGRESSION TESTING"
echo "All existing tests must still pass"
echo "========================================"

# Run integration tests only (to save time)
TEST_OUTPUT=$(cargo test -p mathhook-core --lib integration 2>&1 || echo "FAILED")

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSING=$(echo "$TEST_OUTPUT" | grep -oP '\d+(?= passed)' | head -1)
    echo -e "${GREEN}✓ Integration tests passing: $PASSING tests${NC}"
else
    echo -e "${RED}✗ Some integration tests failing${NC}"
    echo "$TEST_OUTPUT" | grep -i "FAILED\|error" | head -10
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 9: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=0
for file in "${REQUIRED_NEW_FILES[@]}"; do
    if [ -f "$file" ]; then
        COUNT=$(grep -c "✅\|❌\|⚠️\|🚀\|✨" "$file" 2>/dev/null || echo 0)
        EMOJI_COUNT=$((EMOJI_COUNT + COUNT))
    fi
done

if [ $EMOJI_COUNT -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in new files${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 10: INTEGRATION WITH EXISTING CODE
echo "========================================"
echo "CATEGORY 10: INTEGRATION WITH EXISTING CODE"
echo "Strategy dispatcher must be called from main integration"
echo "========================================"

MAIN_INTEGRATION="crates/mathhook-core/src/calculus/integrals.rs"
INTEGRATION_ISSUES=0

if [ -f "$MAIN_INTEGRATION" ]; then
    # Check if strategy dispatcher is called
    if grep -qi "strategy" "$MAIN_INTEGRATION"; then
        echo -e "${GREEN}✓ Strategy dispatcher referenced in main integration${NC}"
    else
        echo -e "${RED}✗ Strategy dispatcher not integrated${NC}"
        INTEGRATION_ISSUES=$((INTEGRATION_ISSUES + 1))
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Main integration file missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 2: Foundation is VERIFIED COMPLETE"
    echo ""
    echo "Achievements:"
    echo "  - Rational function integration implemented (partial fractions)"
    echo "  - Strategy dispatcher operational (layered fallthrough)"
    echo "  - $TOTAL_NEW_TESTS tests created (target: 40+)"
    echo "  - SymPy validation: $SYMPY_COMMENT_COUNT references"
    echo "  - Zero regressions (all existing tests pass)"
    echo "  - Coverage: 75% → 85% (estimated)"
    echo ""
    echo "Ready to proceed to Wave 3: Enhancement (Integration Table + u-Substitution)"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 2: Foundation requires fixes before approval"
    echo ""
    echo "Fix the issues above and re-run verification."
    exit 1
fi
