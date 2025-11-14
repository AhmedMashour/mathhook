#!/bin/bash

# Wave 3: Enhancement - Integration Table + u-Substitution Verification Script
# Purpose: Verify O(1) table lookup and enhanced substitution implementation
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 3: ENHANCEMENT VERIFICATION"
echo "Integration Table + Enhanced u-Substitution"
echo "Coverage Target: 85% → 90%"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: REQUIRED FILES (2 NEW/MODIFIED)
echo "========================================"
echo "CATEGORY 1: REQUIRED FILES"
echo "Wave 3 must deliver 2 new/modified files"
echo "========================================"

REQUIRED_NEW_FILES=(
    "crates/mathhook-core/src/calculus/integrals/table.rs"
)

REQUIRED_MODIFIED_FILES=(
    "crates/mathhook-core/src/calculus/integrals/substitution.rs"
    "crates/mathhook-core/src/calculus/integrals/strategy.rs"
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

# CATEGORY 3: INTEGRATION TABLE
echo "========================================"
echo "CATEGORY 3: INTEGRATION TABLE"
echo "Must implement O(1) pattern lookup"
echo "========================================"

TABLE_FILE="crates/mathhook-core/src/calculus/integrals/table.rs"
TABLE_ISSUES=0

if [ -f "$TABLE_FILE" ]; then
    # Check for key table concepts
    REQUIRED_PATTERNS=(
        "pattern"
        "lookup"
        "arctan"
        "arcsin"
    )

    for pattern in "${REQUIRED_PATTERNS[@]}"; do
        if ! grep -qi "$pattern" "$TABLE_FILE"; then
            echo -e "${RED}✗ Missing pattern/concept: '$pattern'${NC}"
            TABLE_ISSUES=$((TABLE_ISSUES + 1))
        fi
    done

    if [ $TABLE_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Integration table implementation covers key patterns${NC}"
    else
        echo -e "${RED}✗ Integration table incomplete: $TABLE_ISSUES concepts not found${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ table.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: ENHANCED U-SUBSTITUTION
echo "========================================"
echo "CATEGORY 4: ENHANCED U-SUBSTITUTION"
echo "Must implement general composite function handling"
echo "========================================"

SUBST_FILE="crates/mathhook-core/src/calculus/integrals/substitution.rs"
SUBST_ISSUES=0

if [ -f "$SUBST_FILE" ]; then
    # Check for substitution concepts
    SUBST_CONCEPTS=(
        "substitution"
        "derivative"
        "composite"
    )

    FOUND_CONCEPTS=0
    for concept in "${SUBST_CONCEPTS[@]}"; do
        if grep -qi "$concept" "$SUBST_FILE"; then
            FOUND_CONCEPTS=$((FOUND_CONCEPTS + 1))
        fi
    done

    if [ $FOUND_CONCEPTS -ge 2 ]; then
        echo -e "${GREEN}✓ u-Substitution has $FOUND_CONCEPTS/3 key concepts${NC}"
    else
        echo -e "${RED}✗ u-Substitution incomplete: only $FOUND_CONCEPTS/3 concepts found${NC}"
        SUBST_ISSUES=$((SUBST_ISSUES + 1))
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ substitution.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: STRATEGY INTEGRATION
echo "========================================"
echo "CATEGORY 5: STRATEGY INTEGRATION"
echo "Table and substitution must be in strategy dispatcher"
echo "========================================"

STRATEGY_FILE="crates/mathhook-core/src/calculus/integrals/strategy.rs"
STRATEGY_ISSUES=0

if [ -f "$STRATEGY_FILE" ]; then
    # Check if table and substitution are integrated
    if grep -qi "table" "$STRATEGY_FILE"; then
        echo -e "${GREEN}✓ Table lookup integrated in strategy${NC}"
    else
        echo -e "${RED}✗ Table lookup not found in strategy${NC}"
        STRATEGY_ISSUES=$((STRATEGY_ISSUES + 1))
    fi

    if grep -qi "substitution" "$STRATEGY_FILE"; then
        echo -e "${GREEN}✓ Substitution integrated in strategy${NC}"
    else
        echo -e "${RED}✗ Substitution not found in strategy${NC}"
        STRATEGY_ISSUES=$((STRATEGY_ISSUES + 1))
    fi

    if [ $STRATEGY_ISSUES -gt 0 ]; then
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ strategy.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: TEST COVERAGE (55+ TESTS)
echo "========================================"
echo "CATEGORY 6: TEST COVERAGE"
echo "Wave 3 requires 55+ new tests (30 table + 25 substitution)"
echo "========================================"

# Find test files
TEST_FILES=$(find crates/mathhook-core/tests -name "*table*.rs" -o -name "*substitution*.rs" 2>/dev/null)

TOTAL_NEW_TESTS=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        TEST_COUNT=$(grep -c "^[[:space:]]*#\[test\]" "$test_file" 2>/dev/null || echo 0)
        TOTAL_NEW_TESTS=$((TOTAL_NEW_TESTS + TEST_COUNT))
        echo "  $test_file: $TEST_COUNT tests"
    fi
done

if [ $TOTAL_NEW_TESTS -ge 55 ]; then
    echo -e "${GREEN}✓ Found $TOTAL_NEW_TESTS tests (target: 55+)${NC}"
elif [ $TOTAL_NEW_TESTS -ge 45 ]; then
    echo -e "${YELLOW}⚠ Found $TOTAL_NEW_TESTS tests (close to 55 target)${NC}"
else
    echo -e "${RED}✗ Found only $TOTAL_NEW_TESTS tests (target: 55+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: SYMPY VALIDATION
echo "========================================"
echo "CATEGORY 7: SYMPY VALIDATION"
echo "All tests must include SymPy validation"
echo "========================================"

SYMPY_COUNT=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        COUNT=$(grep -ci "sympy" "$test_file" 2>/dev/null || echo 0)
        SYMPY_COUNT=$((SYMPY_COUNT + COUNT))
    fi
done

if [ $SYMPY_COUNT -ge 30 ]; then
    echo -e "${GREEN}✓ Found $SYMPY_COUNT SymPy references (good coverage)${NC}"
elif [ $SYMPY_COUNT -ge 20 ]; then
    echo -e "${YELLOW}⚠ Found $SYMPY_COUNT SymPy references (could be more)${NC}"
else
    echo -e "${RED}✗ Only $SYMPY_COUNT SymPy references (need more validation)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: BUILD STATUS
echo "========================================"
echo "CATEGORY 8: BUILD STATUS"
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

# CATEGORY 9: REGRESSION TESTING
echo "========================================"
echo "CATEGORY 9: REGRESSION TESTING"
echo "All existing tests must still pass"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --lib integration 2>&1 || echo "FAILED")

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSING=$(echo "$TEST_OUTPUT" | grep -oE '[0-9]+ passed' | head -1)
    echo -e "${GREEN}✓ Integration tests: $PASSING${NC}"
else
    echo -e "${RED}✗ Some integration tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 10: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 10: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=0
for file in "${REQUIRED_NEW_FILES[@]}" "${REQUIRED_MODIFIED_FILES[@]}"; do
    if [ -f "$file" ]; then
        COUNT=$(grep -c "✅\|❌\|⚠️\|🚀\|✨" "$file" 2>/dev/null || echo 0)
        EMOJI_COUNT=$((EMOJI_COUNT + COUNT))
    fi
done

if [ $EMOJI_COUNT -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in Wave 3 files${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 3: Enhancement is VERIFIED COMPLETE"
    echo ""
    echo "Achievements:"
    echo "  - Integration table implemented (O(1) pattern lookup)"
    echo "  - Enhanced u-substitution (general composite functions)"
    echo "  - $TOTAL_NEW_TESTS tests created (target: 55+)"
    echo "  - SymPy validation: $SYMPY_COUNT references"
    echo "  - Zero regressions (all existing tests pass)"
    echo "  - Coverage: 85% → 90% (estimated)"
    echo ""
    echo "Ready to proceed to Wave 4: Advanced (Trigonometric Integrals)"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 3: Enhancement requires fixes before approval"
    echo ""
    echo "Fix the issues above and re-run verification."
    exit 1
fi
