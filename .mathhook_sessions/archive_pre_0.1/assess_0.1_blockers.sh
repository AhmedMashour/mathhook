#!/bin/bash
# Comprehensive 0.1 Release Blocker Assessment Script
# Created: 2025-10-13
# Purpose: Identify ALL CLAUDE.md violations before 0.1 release

set -e

WORKSPACE="/Users/ahmedmashhour/Documents/work/math/mathhook"
cd "$WORKSPACE"

echo "========================================"
echo "0.1 RELEASE BLOCKER ASSESSMENT"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
TOTAL_VIOLATIONS=0
CRITICAL_COUNT=0
HIGH_COUNT=0
MEDIUM_COUNT=0

echo "========================================"
echo "CATEGORY 1: MODULE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"
echo ""

MODULE_SIZE_VIOLATIONS=0
echo "Scanning all .rs files in mathhook-core..."
echo ""

# Find all .rs files and check line counts
while IFS= read -r file; do
    lines=$(wc -l < "$file")
    if [ "$lines" -gt 500 ]; then
        # Calculate percentage over limit
        percent_over=$(( (lines - 500) * 100 / 500 ))

        # Determine severity
        if [ "$lines" -gt 1000 ]; then
            severity="${RED}CRITICAL${NC}"
            ((CRITICAL_COUNT++))
        elif [ "$lines" -gt 750 ]; then
            severity="${YELLOW}HIGH${NC}"
            ((HIGH_COUNT++))
        else
            severity="MEDIUM"
            ((MEDIUM_COUNT++))
        fi

        # Get relative path
        rel_path=${file#$WORKSPACE/crates/mathhook-core/src/}

        echo -e "[$severity] $rel_path: $lines lines (+$percent_over% over limit)"
        ((MODULE_SIZE_VIOLATIONS++))
        ((TOTAL_VIOLATIONS++))
    fi
done < <(find crates/mathhook-core/src -name "*.rs" -type f | grep -v "grammar.rs")

echo ""
if [ $MODULE_SIZE_VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ No module size violations found${NC}"
else
    echo -e "${RED}✗ Found $MODULE_SIZE_VIOLATIONS files exceeding 500 lines${NC}"
fi
echo ""

echo "========================================"
echo "CATEGORY 2: PLACEHOLDER CODE"
echo "CLAUDE.md: No TODO/FIXME/placeholder in production code"
echo "========================================"
echo ""

PLACEHOLDER_FILES=0
PLACEHOLDER_COUNT=0

echo "Scanning for placeholder comments..."
echo ""

# Search for various placeholder patterns
PATTERNS=(
    "TODO"
    "FIXME"
    "XXX"
    "HACK"
    "not implemented yet"
    "not yet implemented"
    "placeholder"
    "Complex.*not.*implement"
    "Integration.*not.*implement"
)

for pattern in "${PATTERNS[@]}"; do
    while IFS= read -r line; do
        file=$(echo "$line" | cut -d: -f1)
        line_num=$(echo "$line" | cut -d: -f2)
        content=$(echo "$line" | cut -d: -f3-)

        rel_path=${file#$WORKSPACE/crates/mathhook-core/src/}

        echo "  $rel_path:$line_num"
        echo "    → $content"
        ((PLACEHOLDER_COUNT++))
    done < <(grep -rn -i "$pattern" crates/mathhook-core/src --include="*.rs" 2>/dev/null || true)
done

# Count unique files with placeholders
PLACEHOLDER_FILES=$(grep -rl -i -E "TODO|FIXME|XXX|HACK|not.*implement|placeholder" crates/mathhook-core/src --include="*.rs" 2>/dev/null | wc -l)

TOTAL_VIOLATIONS=$((TOTAL_VIOLATIONS + PLACEHOLDER_COUNT))

echo ""
if [ $PLACEHOLDER_COUNT -eq 0 ]; then
    echo -e "${GREEN}✓ No placeholder code found${NC}"
else
    echo -e "${RED}✗ Found $PLACEHOLDER_COUNT placeholder comments in $PLACEHOLDER_FILES files${NC}"
fi
echo ""

echo "========================================"
echo "CATEGORY 3: DOMAIN ERROR INTEGRATION"
echo "CLAUDE.md: Use Result<Expression, MathError> for fallible operations"
echo "========================================"
echo ""

# Check if MathError is defined
MATH_ERROR_EXISTS=$(grep -r "pub enum MathError" crates/mathhook-core/src --include="*.rs" | wc -l)

if [ $MATH_ERROR_EXISTS -gt 0 ]; then
    echo -e "${GREEN}✓ MathError enum defined${NC}"

    # Count uses of Result<Expression, MathError>
    RESULT_USES=$(grep -r "Result<Expression, MathError>" crates/mathhook-core/src --include="*.rs" | wc -l)
    echo "  Found $RESULT_USES uses of Result<Expression, MathError>"

    # Check for unwrap() calls (dangerous)
    UNWRAP_COUNT=$(grep -rn "\.unwrap()" crates/mathhook-core/src --include="*.rs" | wc -l)
    if [ $UNWRAP_COUNT -gt 50 ]; then
        echo -e "  ${YELLOW}⚠ Found $UNWRAP_COUNT .unwrap() calls (potential panics)${NC}"
    fi

    # Check key operations for Result returns
    echo ""
    echo "  Checking key operations for error handling:"

    # Check division operations
    DIV_TOTAL=$(grep -rn "fn.*div\|fn.*divide" crates/mathhook-core/src --include="*.rs" | wc -l)
    DIV_WITH_RESULT=$(grep -rn "fn.*div.*Result" crates/mathhook-core/src --include="*.rs" | wc -l)
    echo "    Division operations: $DIV_WITH_RESULT/$DIV_TOTAL use Result"

    # Check sqrt operations
    SQRT_TOTAL=$(grep -rn "fn.*sqrt" crates/mathhook-core/src --include="*.rs" | wc -l)
    SQRT_WITH_RESULT=$(grep -rn "fn.*sqrt.*Result" crates/mathhook-core/src --include="*.rs" | wc -l)
    echo "    Square root operations: $SQRT_WITH_RESULT/$SQRT_TOTAL use Result"

    # Check logarithm operations
    LOG_TOTAL=$(grep -rn "fn.*log\|fn.*ln" crates/mathhook-core/src --include="*.rs" | wc -l)
    LOG_WITH_RESULT=$(grep -rn "fn.*log.*Result\|fn.*ln.*Result" crates/mathhook-core/src --include="*.rs" | wc -l)
    echo "    Logarithm operations: $LOG_WITH_RESULT/$LOG_TOTAL use Result"

    if [ $DIV_WITH_RESULT -eq 0 ] && [ $SQRT_WITH_RESULT -eq 0 ] && [ $LOG_WITH_RESULT -eq 0 ]; then
        echo ""
        echo -e "  ${RED}✗ MathError defined but NOT integrated into operations${NC}"
        TOTAL_VIOLATIONS=$((TOTAL_VIOLATIONS + 1))
    fi
else
    echo -e "${RED}✗ MathError enum not found${NC}"
    TOTAL_VIOLATIONS=$((TOTAL_VIOLATIONS + 1))
fi

echo ""

echo "========================================"
echo "CATEGORY 4: NUMBER OVERFLOW HANDLING"
echo "CLAUDE.md: Use checked arithmetic for Number type"
echo "========================================"
echo ""

# Check for checked arithmetic in number.rs
if [ -f "crates/mathhook-core/src/core/number.rs" ]; then
    CHECKED_OPS=$(grep -c "checked_add\|checked_mul\|checked_sub\|checked_div" crates/mathhook-core/src/core/number.rs || echo "0")
    UNCHECKED_OPS=$(grep -c "^\s*\(+\|-\|\*\|/\)" crates/mathhook-core/src/core/number.rs || echo "0")

    echo "  Checked arithmetic operations: $CHECKED_OPS"

    if [ $CHECKED_OPS -lt 10 ]; then
        echo -e "  ${YELLOW}⚠ Low usage of checked arithmetic (found $CHECKED_OPS)${NC}"
        echo "  CLAUDE.md recommends comprehensive checked_add/checked_mul usage"
    else
        echo -e "  ${GREEN}✓ Good coverage of checked arithmetic${NC}"
    fi
else
    echo -e "${RED}✗ number.rs not found${NC}"
fi

echo ""

echo "========================================"
echo "CATEGORY 5: TEST COVERAGE"
echo "========================================"
echo ""

# Run tests and capture results
echo "Running test suite..."
TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1 | tail -3)
UNIT_PASSED=$(echo "$TEST_OUTPUT" | grep -oP '\d+(?= passed)' || echo "0")
UNIT_FAILED=$(echo "$TEST_OUTPUT" | grep -oP '\d+(?= failed)' || echo "0")

echo "  Unit tests: $UNIT_PASSED passed, $UNIT_FAILED failed"

DOCTEST_OUTPUT=$(cargo test --doc -p mathhook-core 2>&1 | tail -3)
DOC_PASSED=$(echo "$DOCTEST_OUTPUT" | grep -oP '\d+(?= passed)' || echo "0")
DOC_FAILED=$(echo "$DOCTEST_OUTPUT" | grep -oP '\d+(?= failed)' || echo "0")

echo "  Doctests: $DOC_PASSED passed, $DOC_FAILED failed"

TOTAL_TESTS=$((UNIT_PASSED + DOC_PASSED))
TOTAL_FAILED=$((UNIT_FAILED + DOC_FAILED))

if [ $TOTAL_FAILED -eq 0 ]; then
    echo -e "  ${GREEN}✓ All tests passing (${TOTAL_TESTS} tests)${NC}"
else
    echo -e "  ${RED}✗ $TOTAL_FAILED tests failing${NC}"
    TOTAL_VIOLATIONS=$((TOTAL_VIOLATIONS + TOTAL_FAILED))
fi

echo ""

echo "========================================"
echo "SUMMARY"
echo "========================================"
echo ""

echo "Violations by Severity:"
echo -e "  ${RED}CRITICAL (>1000 lines):${NC} $CRITICAL_COUNT files"
echo -e "  ${YELLOW}HIGH (751-1000 lines):${NC} $HIGH_COUNT files"
echo "  MEDIUM (501-750 lines): $MEDIUM_COUNT files"
echo ""

echo "Violations by Category:"
echo "  Module size violations: $MODULE_SIZE_VIOLATIONS files"
echo "  Placeholder code: $PLACEHOLDER_COUNT occurrences in $PLACEHOLDER_FILES files"
echo "  Domain error integration: Incomplete (MathError defined but not integrated)"
echo "  Test failures: $TOTAL_FAILED"
echo ""

echo "Total Violations: $TOTAL_VIOLATIONS"
echo ""

if [ $TOTAL_VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}════════════════════════════════════════${NC}"
    echo -e "${GREEN}✓ ALL CHECKS PASSED - READY FOR 0.1${NC}"
    echo -e "${GREEN}════════════════════════════════════════${NC}"
    exit 0
else
    echo -e "${RED}════════════════════════════════════════${NC}"
    echo -e "${RED}✗ BLOCKERS FOUND - NOT READY FOR 0.1${NC}"
    echo -e "${RED}════════════════════════════════════════${NC}"
    echo ""
    echo "Next Steps:"
    echo "1. Review orchestration plan in PHASE_7_ORCHESTRATION_PLAN.md"
    echo "2. Execute wave-by-wave agent orchestration"
    echo "3. Verify each wave with verification scripts"
    echo "4. Re-run this assessment after each wave"
    exit 1
fi
