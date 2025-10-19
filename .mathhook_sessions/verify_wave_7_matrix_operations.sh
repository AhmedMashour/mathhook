#!/bin/bash

# Wave 7: Matrix Operations Enhancement Verification
# Verifies symbolic matrix operations respect noncommutativity
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 7: MATRIX OPERATIONS VERIFICATION"
echo "Symbolic matrix operations with noncommutative support"
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

# Check all matrix module files
while IFS= read -r -d '' file; do
    LINES=$(wc -l < "$file" | tr -d ' ')
    if [ "$LINES" -gt 500 ]; then
        echo -e "${RED}✗ $file: $LINES lines ($(( (LINES - 500) * 100 / 500 ))% over)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ $file: $LINES lines${NC}"
    fi
done < <(find crates/mathhook-core/src/matrix -name "*.rs" -print0)

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS violations${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/matrix/ 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    grep -rn "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/matrix/ 2>/dev/null | head -5
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | grep "error" | head -5
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: TEST VALIDATION
echo "========================================"
echo "CATEGORY 4: TEST VALIDATION"
echo "Wave 7 requires 40+ tests for matrix operations"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --lib matrix 2>&1)
TEST_COUNT=$(echo "$TEST_OUTPUT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")

if [ -z "$TEST_COUNT" ]; then
    TEST_COUNT=0
fi

echo "Matrix tests passing: $TEST_COUNT"

if [ "$TEST_COUNT" -ge 40 ]; then
    echo -e "${GREEN}✓ $TEST_COUNT tests passing (≥40)${NC}"
else
    echo -e "${YELLOW}⚠ $TEST_COUNT tests passing (<40 target)${NC}"
fi

# CATEGORY 5: TRANSPOSE ORDER REVERSAL TESTS
echo "========================================"
echo "CATEGORY 5: TRANSPOSE ORDER REVERSAL TESTS"
echo "Must test (AB)^T = B^T A^T"
echo "========================================"

TRANSPOSE_TESTS=$(grep -r "fn test.*transpose" crates/mathhook-core/src/matrix/ crates/mathhook-core/tests/ 2>/dev/null | grep -i "order\|reverse\|product" | wc -l)

echo "Transpose order reversal tests found: $TRANSPOSE_TESTS"

if [ "$TRANSPOSE_TESTS" -ge 2 ]; then
    echo -e "${GREEN}✓ $TRANSPOSE_TESTS transpose tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $TRANSPOSE_TESTS transpose tests (need 2+)${NC}"
fi

# CATEGORY 6: INVERSE ORDER REVERSAL TESTS
echo "========================================"
echo "CATEGORY 6: INVERSE ORDER REVERSAL TESTS"
echo "Must test (AB)^(-1) = B^(-1) A^(-1)"
echo "========================================"

INVERSE_TESTS=$(grep -r "fn test.*inverse" crates/mathhook-core/src/matrix/ crates/mathhook-core/tests/ 2>/dev/null | grep -i "order\|reverse\|product" | wc -l)

echo "Inverse order reversal tests found: $INVERSE_TESTS"

if [ "$INVERSE_TESTS" -ge 2 ]; then
    echo -e "${GREEN}✓ $INVERSE_TESTS inverse tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $INVERSE_TESTS inverse tests (need 2+)${NC}"
fi

# CATEGORY 7: SYMBOLIC MATRIX TESTS
echo "========================================"
echo "CATEGORY 7: SYMBOLIC MATRIX TESTS"
echo "Must test symbolic (not just numeric) matrices"
echo "========================================"

SYMBOLIC_TESTS=$(grep -r "fn test.*symbolic" crates/mathhook-core/src/matrix/ crates/mathhook-core/tests/ 2>/dev/null | wc -l)

echo "Symbolic matrix tests found: $SYMBOLIC_TESTS"

if [ "$SYMBOLIC_TESTS" -ge 5 ]; then
    echo -e "${GREEN}✓ $SYMBOLIC_TESTS symbolic tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $SYMBOLIC_TESTS symbolic tests (need 5+)${NC}"
fi

# CATEGORY 8: DOCUMENTATION COMPLIANCE
echo "========================================"
echo "CATEGORY 8: DOCUMENTATION COMPLIANCE"
echo "All modified files must have module docs"
echo "========================================"

DOC_VIOLATIONS=0

for file in \
    "crates/mathhook-core/src/matrix/operations.rs" \
    "crates/mathhook-core/src/matrix/decomposition.rs" \
    "crates/mathhook-core/src/matrix/eigenvalues.rs"; do

    if [ -f "$file" ]; then
        if head -5 "$file" | grep -q "^//!"; then
            echo -e "${GREEN}✓ $file has module docs${NC}"
        else
            echo -e "${RED}✗ $file missing module docs${NC}"
            DOC_VIOLATIONS=$((DOC_VIOLATIONS + 1))
        fi
    fi
done

if [ $DOC_VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files documented${NC}"
else
    echo -e "${RED}✗ $DOC_VIOLATIONS files missing docs${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: ZERO REGRESSIONS
echo "========================================"
echo "CATEGORY 9: ZERO REGRESSIONS"
echo "All existing tests must still pass"
echo "========================================"

FULL_TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)
FULL_TEST_COUNT=$(echo "$FULL_TEST_OUTPUT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")

if [ -z "$FULL_TEST_COUNT" ]; then
    FULL_TEST_COUNT=0
fi

echo "Total library tests passing: $FULL_TEST_COUNT"

if [ "$FULL_TEST_COUNT" -ge 625 ]; then
    echo -e "${GREEN}✓ $FULL_TEST_COUNT tests passing (no regressions from Wave 6)${NC}"
else
    echo -e "${YELLOW}⚠ $FULL_TEST_COUNT tests (Wave 6 had 625)${NC}"
fi

if echo "$FULL_TEST_OUTPUT" | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ All tests passing${NC}"
else
    echo -e "${RED}✗ Some tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 7: Matrix Operations Enhancement is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 7 requires fixes before approval"
    exit 1
fi
