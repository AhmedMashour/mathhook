#!/bin/bash

# Educational Wave 4 Verification Script
# Function Intelligence Education
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "EDUCATIONAL WAVE 4 VERIFICATION"
echo "Function Intelligence Education"
echo "========================================"
echo ""

# Track failures
FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "========================================
CATEGORY 1: FILE SIZE VIOLATIONS
CLAUDE.md: Maximum 500 lines per file
========================================"

echo "Checking Wave 4 modified files..."

# Function education files
if [ -d "crates/mathhook-core/src/functions/educational" ]; then
    echo "Functions educational module:"
    find crates/mathhook-core/src/functions/educational -name "*.rs" | while read file; do
        LINES=$(wc -l < "$file")
        FILE=$(basename "$file")
        if [ $LINES -gt 500 ]; then
            echo -e "${RED}✗ functions/educational/$FILE: $LINES lines${NC}"
            FAILURES=$((FAILURES + 1))
        else
            echo -e "${GREEN}✓ functions/educational/$FILE: $LINES lines${NC}"
        fi
    done
elif [ -f "crates/mathhook-core/src/functions/education.rs" ]; then
    EDUCATION_LINES=$(wc -l < crates/mathhook-core/src/functions/education.rs)
    if [ "$EDUCATION_LINES" -gt 500 ]; then
        OVER=$((EDUCATION_LINES - 500))
        PERCENT=$(( (OVER * 100) / 500 ))
        echo -e "${RED}✗ functions/education.rs: $EDUCATION_LINES lines (+$PERCENT% over limit)${NC}"
        FAILURES=$((FAILURES + 1))
    else
        echo -e "${GREEN}✓ functions/education.rs: $EDUCATION_LINES lines${NC}"
    fi
fi

echo ""
echo "========================================
CATEGORY 2: EMOJI COMPLIANCE
CLAUDE.md: No emojis in code
========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/functions/educational* crates/mathhook-core/src/functions/education.rs 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in Wave 4 files${NC}"
    grep -rn "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/functions/ 2>/dev/null | head -5
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found in Wave 4 files${NC}"
fi

echo ""
echo "========================================
CATEGORY 3: TEST VALIDATION
Must have content validation tests
========================================"

# Function education tests
if [ -f "crates/mathhook-core/tests/function_education_test.rs" ]; then
    echo -e "${GREEN}✓ function_education_test.rs exists${NC}"

    echo "Running function education tests..."
    TEST_OUTPUT=$(cargo test -p mathhook-core --test function_education_test 2>&1)

    if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
        PASSED=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo -e "${GREEN}✓ Function education tests: $PASSED passed${NC}"

        if [ "$PASSED" -lt 15 ]; then
            echo -e "${YELLOW}⚠ Only $PASSED tests (target: 15+)${NC}"
        fi
    else
        echo -e "${RED}✗ Function education tests FAILED${NC}"
        echo "$TEST_OUTPUT" | grep -E "error|FAILED" | head -10
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ function_education_test.rs NOT FOUND${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================
CATEGORY 4: CONTENT VALIDATION
Tests must validate actual math content
========================================"

if [ -f "crates/mathhook-core/tests/function_education_test.rs" ]; then
    CONTENT_VALIDATION=$(grep -E "has_step_containing|assert.*contains" crates/mathhook-core/tests/function_education_test.rs | wc -l)
    echo "Function education tests content validation checks: $CONTENT_VALIDATION"

    if [ "$CONTENT_VALIDATION" -lt 15 ]; then
        echo -e "${YELLOW}⚠ Low content validation count (target: 15+)${NC}"
    else
        echo -e "${GREEN}✓ Good content validation coverage${NC}"
    fi
fi

echo ""
echo "========================================
CATEGORY 5: IMPLEMENTATION COMPLETENESS
Verify all required functions implemented
========================================"

echo "Checking elementary function implementations..."
if grep -rq "sin\|cos\|exp\|log" crates/mathhook-core/src/functions/educational* crates/mathhook-core/src/functions/education.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Elementary functions implemented${NC}"
else
    echo -e "${RED}✗ Elementary functions missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo "Checking special value detection..."
if grep -rq "special value\|Special Value" crates/mathhook-core/src/functions/educational* crates/mathhook-core/src/functions/education.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Special value detection implemented${NC}"
else
    echo -e "${YELLOW}⚠ Special value detection may be missing${NC}"
fi

echo "Checking polynomial function implementations..."
if grep -rq "polynomial\|Polynomial" crates/mathhook-core/src/functions/educational* crates/mathhook-core/src/functions/education.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Polynomial functions implemented${NC}"
else
    echo -e "${YELLOW}⚠ Polynomial functions may be missing${NC}"
fi

echo ""
echo "========================================
CATEGORY 6: FUNCTION COUNT
Target: 20+ functions with education
========================================"

FUNCTION_COUNT=$(grep -c "step_generators.insert" crates/mathhook-core/src/functions/education.rs 2>/dev/null || echo "0")

if [ "$FUNCTION_COUNT" -ge 20 ]; then
    echo -e "${GREEN}✓ $FUNCTION_COUNT educational functions found${NC}"
elif [ "$FUNCTION_COUNT" -ge 15 ]; then
    echo -e "${YELLOW}⚠ $FUNCTION_COUNT educational functions (target: 20+)${NC}"
else
    echo -e "${RED}✗ Only $FUNCTION_COUNT educational functions (target: 20+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================
CATEGORY 7: BUILD STATUS
Must compile successfully
========================================"

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
echo "========================================
CATEGORY 8: MESSAGE REGISTRY USAGE
Must use existing function messages
========================================"

if grep -rq "MessageType.*Function\|FunctionEducation" crates/mathhook-core/src/functions/educational* crates/mathhook-core/src/functions/education.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Message registry used${NC}"
else
    echo -e "${YELLOW}⚠ Message registry usage unclear${NC}"
fi

echo ""
echo "========================================
CATEGORY 9: GLOBAL FORMATTER USAGE
Must use global formatter
========================================"

if grep -rq "to_latex\|LaTeXFormatter" crates/mathhook-core/src/functions/educational* crates/mathhook-core/src/functions/education.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Global formatter used${NC}"
else
    echo -e "${YELLOW}⚠ Global formatter usage unclear${NC}"
fi

echo ""
echo "========================================
VERIFICATION SUMMARY
========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Educational Wave 4 is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s) found${NC}"
    echo "Wave 4 requires fixes before approval"
    exit 1
fi
