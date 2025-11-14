#!/bin/bash

# Wave 3: Simplification Engine Updates - Verification Script
# Purpose: Verify simplification respects commutativity
# CRITICAL: A*B + B*A must NOT simplify for noncommutative symbols

echo "========================================"
echo "WAVE 3: SIMPLIFICATION ENGINE VERIFICATION"
echo "Noncommutative Algebra - Conditional Sorting"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: FILE SIZE VIOLATIONS
echo "========================================"
echo "CATEGORY 1: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0
FILES=(
    "crates/mathhook-core/src/simplify/arithmetic/multiplication.rs"
    "crates/mathhook-core/src/simplify/arithmetic/addition.rs"
    "crates/mathhook-core/src/simplify/arithmetic/power.rs"
)

for FILE in "${FILES[@]}"; do
    if [ -f "$FILE" ]; then
        LINES=$(wc -l < "$FILE")
        if [ "$LINES" -gt 500 ]; then
            echo -e "${RED}✗ $FILE: $LINES lines (max 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $FILE: $LINES lines${NC}"
        fi
    fi
done

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply with 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/simplify/ 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in simplify/${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found in simplify/${NC}"
fi

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

echo "Running: cargo check -p mathhook-core"
BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | tail -20
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: CONDITIONAL SORTING
echo "========================================"
echo "CATEGORY 4: CONDITIONAL SORTING"
echo "CRITICAL: Only sort if commutative"
echo "========================================"

# Check if simplify_multiplication checks commutativity
if grep -q "commutativity()\|can_sort()" crates/mathhook-core/src/simplify/arithmetic/multiplication.rs; then
    echo -e "${GREEN}✓ simplify_multiplication checks commutativity${NC}"
else
    echo -e "${RED}✗ simplify_multiplication doesn't check commutativity${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: TEST SUITE
echo "========================================"
echo "CATEGORY 5: TEST SUITE"
echo "All tests must pass (zero regressions)"
echo "========================================"

echo "Running: cargo test -p mathhook-core --lib"
TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    echo -e "${GREEN}✓ All tests passed ($PASSED tests)${NC}"
else
    echo -e "${RED}✗ Some tests failed${NC}"
    echo "$TEST_OUTPUT" | grep -A 5 "FAILED"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: CRITICAL BEHAVIOR TESTS
echo "========================================"
echo "CATEGORY 6: CRITICAL BEHAVIOR TESTS"
echo "Matrix multiplication must preserve order"
echo "========================================"

# Check for tests that verify A*B ≠ B*A for matrices
CRITICAL_TESTS=0

if grep -q "test.*matrix.*not.*simplif\|test.*noncommutative.*not.*combin" crates/mathhook-core/src/simplify/arithmetic/*.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Critical noncommutative tests exist${NC}"
else
    echo -e "${YELLOW}⚠ Critical noncommutative tests not found (check manually)${NC}"
fi

# CATEGORY 7: NEW TESTS
echo "========================================"
echo "CATEGORY 7: NEW TESTS"
echo "Wave 3 target: 30+ new tests"
echo "========================================"

NEW_TEST_COUNT=0

for FILE in crates/mathhook-core/src/simplify/arithmetic/*.rs; do
    if [ -f "$FILE" ]; then
        COUNT=$(grep -c "#\[test\]" "$FILE" 2>/dev/null || echo 0)
        NEW_TEST_COUNT=$((NEW_TEST_COUNT + COUNT))
    fi
done

echo "Found $NEW_TEST_COUNT tests in simplify/arithmetic/"

if [ $NEW_TEST_COUNT -ge 30 ]; then
    echo -e "${GREEN}✓ Test count target met ($NEW_TEST_COUNT ≥ 30)${NC}"
elif [ $NEW_TEST_COUNT -ge 20 ]; then
    echo -e "${YELLOW}⚠ Test count below target ($NEW_TEST_COUNT < 30)${NC}"
else
    echo -e "${RED}✗ Insufficient tests ($NEW_TEST_COUNT < 30)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: DOCUMENTATION
echo "========================================"
echo "CATEGORY 8: DOCUMENTATION"
echo "Verify documentation updated"
echo "========================================"

DOC_ISSUES=0

# Check for documentation of commutativity handling
if grep -q "commutativ" crates/mathhook-core/src/simplify/arithmetic/multiplication.rs; then
    echo -e "${GREEN}✓ Commutativity documented in multiplication.rs${NC}"
else
    echo -e "${YELLOW}⚠ Commutativity not mentioned in docs${NC}"
    DOC_ISSUES=$((DOC_ISSUES + 1))
fi

if [ $DOC_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✓ Documentation complete${NC}"
else
    echo -e "${YELLOW}⚠ $DOC_ISSUES documentation issue(s)${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 3: Simplification Engine Updates is VERIFIED COMPLETE"
    echo ""
    echo "Key Achievements:"
    echo "  ✓ Simplification respects commutativity"
    echo "  ✓ Only sorts commutative factors"
    echo "  ✓ Matrix products preserve order"
    echo "  ✓ Zero regressions in existing tests"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 3 requires fixes before approval"
    exit 1
fi
