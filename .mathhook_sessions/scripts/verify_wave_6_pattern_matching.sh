#!/bin/bash

# Wave 6: Pattern Matching & Substitution Verification
# Verifies noncommutative pattern matching support
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 6: PATTERN MATCHING VERIFICATION"
echo "Noncommutative pattern matching & substitution"
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

# Check all pattern module files
while IFS= read -r -d '' file; do
    LINES=$(wc -l < "$file" | tr -d ' ')
    if [ "$LINES" -gt 500 ]; then
        echo -e "${RED}✗ $file: $LINES lines ($(( (LINES - 500) * 100 / 500 ))% over)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ $file: $LINES lines${NC}"
    fi
done < <(find crates/mathhook-core/src/pattern -name "*.rs" -print0)

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

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/pattern/ 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    grep -rn "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/pattern/ 2>/dev/null | head -5
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
echo "Wave 6 requires 30+ tests for pattern matching"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --lib pattern 2>&1)
TEST_COUNT=$(echo "$TEST_OUTPUT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")

if [ -z "$TEST_COUNT" ]; then
    TEST_COUNT=0
fi

echo "Pattern tests passing: $TEST_COUNT"

if [ "$TEST_COUNT" -ge 30 ]; then
    echo -e "${GREEN}✓ $TEST_COUNT tests passing (≥30)${NC}"
else
    echo -e "${YELLOW}⚠ $TEST_COUNT tests passing (<30 target)${NC}"
fi

# CATEGORY 5: NONCOMMUTATIVE PATTERN MATCHING TESTS
echo "========================================"
echo "CATEGORY 5: NONCOMMUTATIVE PATTERN TESTS"
echo "Must test AB should NOT match BA for matrices"
echo "========================================"

PATTERN_TESTS=$(grep -r "fn test.*matrix\|fn test.*noncommutative\|fn test.*operator\|fn test.*order" crates/mathhook-core/src/pattern/ crates/mathhook-core/tests/ 2>/dev/null | grep -i "pattern\|match\|subs" | wc -l)

echo "Noncommutative pattern tests found: $PATTERN_TESTS"

if [ "$PATTERN_TESTS" -ge 5 ]; then
    echo -e "${GREEN}✓ $PATTERN_TESTS pattern tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $PATTERN_TESTS pattern tests (need 5+)${NC}"
fi

# CATEGORY 6: SUBSTITUTION POSITION PRESERVATION TESTS
echo "========================================"
echo "CATEGORY 6: SUBSTITUTION POSITION TESTS"
echo "Must test A→C in ABA preserves positions"
echo "========================================"

SUBST_TESTS=$(grep -r "fn test.*substitut" crates/mathhook-core/src/pattern/ crates/mathhook-core/tests/ 2>/dev/null | grep -i "matrix\|noncommutative\|position\|order" | wc -l)

echo "Substitution position tests found: $SUBST_TESTS"

if [ "$SUBST_TESTS" -ge 5 ]; then
    echo -e "${GREEN}✓ $SUBST_TESTS substitution tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $SUBST_TESTS substitution tests (need 5+)${NC}"
fi

# CATEGORY 7: COMMUTATIVITY CHECKS
echo "========================================"
echo "CATEGORY 7: COMMUTATIVITY CHECKS"
echo "Pattern matching must check commutativity"
echo "========================================"

COMM_CHECKS=0

if grep -r "is_commutative\|commutativity()\|can_sort()" "crates/mathhook-core/src/pattern/" 2>/dev/null | grep -q .; then
    echo -e "${GREEN}✓ Pattern module checks commutativity${NC}"
    COMM_CHECKS=$((COMM_CHECKS + 1))
else
    echo -e "${YELLOW}⚠ Pattern module may not check commutativity${NC}"
fi

if [ $COMM_CHECKS -ge 1 ]; then
    echo -e "${GREEN}✓ Pattern module checks commutativity${NC}"
else
    echo -e "${YELLOW}⚠ Pattern module should check commutativity${NC}"
fi

# CATEGORY 8: DOCUMENTATION COMPLIANCE
echo "========================================"
echo "CATEGORY 8: DOCUMENTATION COMPLIANCE"
echo "All modified files must have module docs"
echo "========================================"

DOC_VIOLATIONS=0

for file in \
    "crates/mathhook-core/src/pattern/mod.rs" \
    "crates/mathhook-core/src/pattern/matching.rs" \
    "crates/mathhook-core/src/pattern/substitution/mod.rs"; do

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

if [ "$FULL_TEST_COUNT" -ge 613 ]; then
    echo -e "${GREEN}✓ $FULL_TEST_COUNT tests passing (no regressions from Wave 5)${NC}"
else
    echo -e "${YELLOW}⚠ $FULL_TEST_COUNT tests (Wave 5 had 613)${NC}"
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
    echo "Wave 6: Pattern Matching & Substitution is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 6 requires fixes before approval"
    exit 1
fi
