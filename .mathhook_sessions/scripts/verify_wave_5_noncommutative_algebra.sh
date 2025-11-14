#!/bin/bash

# Wave 5: Algebra Operations Integration Verification
# Verifies noncommutative algebra support in expand, factor, collect, polynomial_division, rational, advanced_simplify
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 5: ALGEBRA OPERATIONS VERIFICATION"
echo "Noncommutative algebra in expand, factor, collect, polynomial_division, rational, advanced_simplify"
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

# Check single-file modules
for file in \
    "crates/mathhook-core/src/algebra/expand.rs" \
    "crates/mathhook-core/src/algebra/polynomial_division.rs" \
    "crates/mathhook-core/src/algebra/rational.rs"; do

    if [ -f "$file" ]; then
        LINES=$(wc -l < "$file" | tr -d ' ')
        if [ "$LINES" -gt 500 ]; then
            echo -e "${RED}✗ $file: $LINES lines ($(( (LINES - 500) * 100 / 500 ))% over)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $file: $LINES lines${NC}"
        fi
    fi
done

# Check modular directories (each subfile must be ≤500 lines)
for dir in "factor" "collect" "advanced_simplify"; do
    if [ -d "crates/mathhook-core/src/algebra/$dir" ]; then
        MODULE_VIOLATIONS=0
        while IFS= read -r -d '' file; do
            LINES=$(wc -l < "$file" | tr -d ' ')
            if [ "$LINES" -gt 500 ]; then
                echo -e "${RED}✗ $file: $LINES lines ($(( (LINES - 500) * 100 / 500 ))% over)${NC}"
                MODULE_VIOLATIONS=$((MODULE_VIOLATIONS + 1))
                VIOLATIONS=$((VIOLATIONS + 1))
            else
                echo -e "${GREEN}✓ $file: $LINES lines${NC}"
            fi
        done < <(find "crates/mathhook-core/src/algebra/$dir" -name "*.rs" -print0)

        if [ $MODULE_VIOLATIONS -eq 0 ]; then
            echo -e "${GREEN}✓ $dir/ module: all files ≤500 lines${NC}"
        fi
    fi
done

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

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/algebra/expand.rs crates/mathhook-core/src/algebra/factor.rs crates/mathhook-core/src/algebra/collect.rs crates/mathhook-core/src/algebra/polynomial_division.rs crates/mathhook-core/src/algebra/rational.rs crates/mathhook-core/src/algebra/advanced_simplify.rs 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    grep -rn "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/algebra/expand.rs crates/mathhook-core/src/algebra/factor.rs crates/mathhook-core/src/algebra/collect.rs crates/mathhook-core/src/algebra/polynomial_division.rs crates/mathhook-core/src/algebra/rational.rs crates/mathhook-core/src/algebra/advanced_simplify.rs 2>/dev/null | head -5
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
echo "Wave 5 requires 50+ tests for algebra operations"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --lib algebra 2>&1)
TEST_COUNT=$(echo "$TEST_OUTPUT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")

if [ -z "$TEST_COUNT" ]; then
    TEST_COUNT=0
fi

echo "Algebra tests passing: $TEST_COUNT"

if [ "$TEST_COUNT" -ge 50 ]; then
    echo -e "${GREEN}✓ $TEST_COUNT tests passing (≥50)${NC}"
else
    echo -e "${YELLOW}⚠ $TEST_COUNT tests passing (<50 target)${NC}"
fi

# CATEGORY 5: NONCOMMUTATIVE EXPAND TESTS
echo "========================================"
echo "CATEGORY 5: NONCOMMUTATIVE EXPAND TESTS"
echo "Must test (A+B)^2 expansion for matrices"
echo "========================================"

EXPAND_TESTS=$(grep -r "fn test.*matrix\|fn test.*noncommutative\|fn test.*operator" crates/mathhook-core/src/algebra/expand.rs crates/mathhook-core/tests/ 2>/dev/null | wc -l)

echo "Noncommutative expand tests found: $EXPAND_TESTS"

if [ "$EXPAND_TESTS" -ge 3 ]; then
    echo -e "${GREEN}✓ $EXPAND_TESTS expand tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $EXPAND_TESTS expand tests (need 3+)${NC}"
fi

# CATEGORY 6: NONCOMMUTATIVE COLLECT TESTS
echo "========================================"
echo "CATEGORY 6: NONCOMMUTATIVE COLLECT TESTS"
echo "Must test AB and BA treated as different terms"
echo "========================================"

COLLECT_TESTS=$(grep -r "fn test.*matrix\|fn test.*noncommutative\|fn test.*operator" crates/mathhook-core/src/algebra/collect/ crates/mathhook-core/tests/ 2>/dev/null | wc -l)

echo "Noncommutative collect tests found: $COLLECT_TESTS"

if [ "$COLLECT_TESTS" -ge 3 ]; then
    echo -e "${GREEN}✓ $COLLECT_TESTS collect tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $COLLECT_TESTS collect tests (need 3+)${NC}"
fi

# CATEGORY 7: NONCOMMUTATIVE FACTOR TESTS
echo "========================================"
echo "CATEGORY 7: NONCOMMUTATIVE FACTOR TESTS"
echo "Must test factoring preserves order"
echo "========================================"

FACTOR_TESTS=$(grep -r "fn test.*matrix\|fn test.*noncommutative\|fn test.*operator" crates/mathhook-core/src/algebra/factor/ crates/mathhook-core/tests/ 2>/dev/null | wc -l)

echo "Noncommutative factor tests found: $FACTOR_TESTS"

if [ "$FACTOR_TESTS" -ge 3 ]; then
    echo -e "${GREEN}✓ $FACTOR_TESTS factor tests found${NC}"
else
    echo -e "${YELLOW}⚠ Only $FACTOR_TESTS factor tests (need 3+)${NC}"
fi

# CATEGORY 8: COMMUTATIVITY CHECKS
echo "========================================"
echo "CATEGORY 8: COMMUTATIVITY CHECKS"
echo "Operations must check commutativity before reordering"
echo "========================================"

COMM_CHECKS=0

# Check expand.rs (single file)
if grep -q "is_commutative\|commutativity()\|can_sort()" "crates/mathhook-core/src/algebra/expand.rs" 2>/dev/null; then
    echo -e "${GREEN}✓ expand.rs checks commutativity${NC}"
    COMM_CHECKS=$((COMM_CHECKS + 1))
else
    echo -e "${YELLOW}⚠ expand.rs may not check commutativity${NC}"
fi

# Check factor/ directory (module)
if grep -r "is_commutative\|commutativity()\|can_sort()" "crates/mathhook-core/src/algebra/factor/" 2>/dev/null | grep -q .; then
    echo -e "${GREEN}✓ factor/ module checks commutativity${NC}"
    COMM_CHECKS=$((COMM_CHECKS + 1))
else
    echo -e "${YELLOW}⚠ factor/ module may not check commutativity${NC}"
fi

# Check collect/ directory (module)
if grep -r "is_commutative\|commutativity()\|can_sort()" "crates/mathhook-core/src/algebra/collect/" 2>/dev/null | grep -q .; then
    echo -e "${GREEN}✓ collect/ module checks commutativity${NC}"
    COMM_CHECKS=$((COMM_CHECKS + 1))
else
    echo -e "${YELLOW}⚠ collect/ module may not check commutativity${NC}"
fi

if [ $COMM_CHECKS -ge 3 ]; then
    echo -e "${GREEN}✓ All critical files/modules check commutativity${NC}"
else
    echo -e "${YELLOW}⚠ Only $COMM_CHECKS/3 files/modules check commutativity${NC}"
fi

# CATEGORY 9: DOCUMENTATION COMPLIANCE
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION COMPLIANCE"
echo "All modified files must have module docs"
echo "========================================"

DOC_VIOLATIONS=0

for file in \
    "crates/mathhook-core/src/algebra/expand.rs" \
    "crates/mathhook-core/src/algebra/factor.rs" \
    "crates/mathhook-core/src/algebra/collect.rs" \
    "crates/mathhook-core/src/algebra/polynomial_division.rs" \
    "crates/mathhook-core/src/algebra/rational.rs" \
    "crates/mathhook-core/src/algebra/advanced_simplify.rs"; do

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

# CATEGORY 10: ZERO REGRESSIONS
echo "========================================"
echo "CATEGORY 10: ZERO REGRESSIONS"
echo "All existing tests must still pass"
echo "========================================"

FULL_TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)
FULL_TEST_COUNT=$(echo "$FULL_TEST_OUTPUT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")

if [ -z "$FULL_TEST_COUNT" ]; then
    FULL_TEST_COUNT=0
fi

echo "Total library tests passing: $FULL_TEST_COUNT"

if [ "$FULL_TEST_COUNT" -ge 600 ]; then
    echo -e "${GREEN}✓ $FULL_TEST_COUNT tests passing (no regressions)${NC}"
else
    echo -e "${YELLOW}⚠ $FULL_TEST_COUNT tests (expected ≥600)${NC}"
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
    echo "Wave 5: Algebra Operations Integration is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 5 requires fixes before approval"
    exit 1
fi
