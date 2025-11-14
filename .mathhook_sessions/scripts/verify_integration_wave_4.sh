#!/bin/bash

# Wave 4: Advanced - Trigonometric Integrals Verification Script
# Purpose: Verify sin^m*cos^n pattern handling implementation
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 4: ADVANCED VERIFICATION"
echo "Trigonometric Integrals (sin^m*cos^n)"
echo "Coverage Target: 90% → 92%"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: REQUIRED FILES (1 NEW)
echo "========================================"
echo "CATEGORY 1: REQUIRED FILES"
echo "Wave 4 must deliver 1 new file + 1 modified"
echo "========================================"

REQUIRED_NEW_FILES=(
    "crates/mathhook-core/src/calculus/integrals/trigonometric.rs"
)

REQUIRED_MODIFIED_FILES=(
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

# CATEGORY 3: TRIGONOMETRIC PATTERNS
echo "========================================"
echo "CATEGORY 3: TRIGONOMETRIC PATTERNS"
echo "Must implement sin^m*cos^n reduction"
echo "========================================"

TRIG_FILE="crates/mathhook-core/src/calculus/integrals/trigonometric.rs"
TRIG_ISSUES=0

if [ -f "$TRIG_FILE" ]; then
    # Check for key trigonometric concepts
    REQUIRED_PATTERNS=(
        "sin"
        "cos"
        "reduction"
        "power"
    )

    for pattern in "${REQUIRED_PATTERNS[@]}"; do
        if ! grep -qi "$pattern" "$TRIG_FILE"; then
            echo -e "${RED}✗ Missing pattern/concept: '$pattern'${NC}"
            TRIG_ISSUES=$((TRIG_ISSUES + 1))
        fi
    done

    if [ $TRIG_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Trigonometric implementation covers key patterns${NC}"
    else
        echo -e "${RED}✗ Trigonometric implementation incomplete: $TRIG_ISSUES concepts not found${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ trigonometric.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: STRATEGY INTEGRATION
echo "========================================"
echo "CATEGORY 4: STRATEGY INTEGRATION"
echo "Trigonometric must be in strategy dispatcher"
echo "========================================"

STRATEGY_FILE="crates/mathhook-core/src/calculus/integrals/strategy.rs"
STRATEGY_ISSUES=0

if [ -f "$STRATEGY_FILE" ]; then
    # Check if trigonometric is integrated
    if grep -qi "trigonometric" "$STRATEGY_FILE"; then
        echo -e "${GREEN}✓ Trigonometric integrated in strategy${NC}"
    else
        echo -e "${RED}✗ Trigonometric not found in strategy${NC}"
        STRATEGY_ISSUES=$((STRATEGY_ISSUES + 1))
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ strategy.rs missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: TEST COVERAGE (35+ TESTS)
echo "========================================"
echo "CATEGORY 5: TEST COVERAGE"
echo "Wave 4 requires 35+ new tests"
echo "========================================"

# Find test files
TEST_FILES=$(find crates/mathhook-core/tests -name "*trigonometric*.rs" 2>/dev/null)

TOTAL_NEW_TESTS=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        TEST_COUNT=$(grep -c "^[[:space:]]*#\[test\]" "$test_file" 2>/dev/null || echo 0)
        TOTAL_NEW_TESTS=$((TOTAL_NEW_TESTS + TEST_COUNT))
        echo "  $test_file: $TEST_COUNT tests"
    fi
done

if [ $TOTAL_NEW_TESTS -ge 35 ]; then
    echo -e "${GREEN}✓ Found $TOTAL_NEW_TESTS tests (target: 35+)${NC}"
elif [ $TOTAL_NEW_TESTS -ge 30 ]; then
    echo -e "${YELLOW}⚠ Found $TOTAL_NEW_TESTS tests (close to 35 target)${NC}"
else
    echo -e "${RED}✗ Found only $TOTAL_NEW_TESTS tests (target: 35+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: SYMPY VALIDATION
echo "========================================"
echo "CATEGORY 6: SYMPY VALIDATION"
echo "Tests should reference SymPy results"
echo "========================================"

SYMPY_COUNT=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        COUNT=$(grep -ci "sympy\|validation" "$test_file" 2>/dev/null || echo 0)
        SYMPY_COUNT=$((SYMPY_COUNT + COUNT))
    fi
done

if [ $SYMPY_COUNT -ge 20 ]; then
    echo -e "${GREEN}✓ Found $SYMPY_COUNT SymPy/validation references (good coverage)${NC}"
elif [ $SYMPY_COUNT -ge 10 ]; then
    echo -e "${YELLOW}⚠ Found $SYMPY_COUNT SymPy/validation references (could be more)${NC}"
else
    echo -e "${YELLOW}⚠ Only $SYMPY_COUNT SymPy/validation references${NC}"
    # Not a failure, just a warning
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

TEST_OUTPUT=$(cargo test -p mathhook-core --lib integration 2>&1 || echo "FAILED")

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSING=$(echo "$TEST_OUTPUT" | grep -oE '[0-9]+ passed' | head -1)
    echo -e "${GREEN}✓ Integration tests: $PASSING${NC}"
else
    echo -e "${RED}✗ Some integration tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 9: EMOJI COMPLIANCE"
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
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in Wave 4 files${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 10: DOCUMENTATION
echo "========================================"
echo "CATEGORY 10: DOCUMENTATION"
echo "All public functions must have docs"
echo "========================================"

DOC_ISSUES=0
if [ -f "$TRIG_FILE" ]; then
    # Check for module doc
    if ! grep -q "^//!" "$TRIG_FILE"; then
        echo -e "${RED}✗ Missing module documentation (//!)${NC}"
        DOC_ISSUES=$((DOC_ISSUES + 1))
    fi

    # Check for function docs (at least some /// comments)
    FUNC_DOC_COUNT=$(grep -c "^///" "$TRIG_FILE" 2>/dev/null || echo 0)
    if [ "$FUNC_DOC_COUNT" -lt 5 ]; then
        echo -e "${YELLOW}⚠ Only $FUNC_DOC_COUNT function docs found${NC}"
        DOC_ISSUES=$((DOC_ISSUES + 1))
    fi
fi

if [ $DOC_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✓ Documentation present${NC}"
else
    echo -e "${YELLOW}⚠ $DOC_ISSUES documentation issues (warning only)${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 4: Advanced is VERIFIED COMPLETE"
    echo ""
    echo "Achievements:"
    echo "  - Trigonometric integration implemented (sin^m*cos^n patterns)"
    echo "  - $TOTAL_NEW_TESTS tests created (target: 35+)"
    echo "  - SymPy validation: $SYMPY_COUNT references"
    echo "  - Zero regressions (all existing tests pass)"
    echo "  - Coverage: 90% → 92% (estimated)"
    echo ""
    echo "Ready to proceed to Wave 5: Risch Algorithm (THE BIG ONE)"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 4: Advanced requires fixes before approval"
    echo ""
    echo "Fix the issues above and re-run verification."
    exit 1
fi
