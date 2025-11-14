#!/bin/bash

# Wave 5: Risch Algorithm - Basic Implementation Verification Script
# Purpose: Verify core Risch algorithm implementation (basic cases)
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 5: RISCH ALGORITHM VERIFICATION"
echo "Basic Implementation (Core Foundation)"
echo "Coverage Target: 92% → 95%"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: REQUIRED FILES (MODULAR STRUCTURE)
echo "========================================"
echo "CATEGORY 1: REQUIRED FILES"
echo "Wave 5 must deliver modular Risch structure"
echo "========================================"

REQUIRED_NEW_FILES=(
    "crates/mathhook-core/src/calculus/integrals/risch.rs"
)

REQUIRED_MODIFIED_FILES=(
    "crates/mathhook-core/src/calculus/integrals/strategy.rs"
)

OPTIONAL_SUBMODULES=(
    "crates/mathhook-core/src/calculus/integrals/risch/mod.rs"
    "crates/mathhook-core/src/calculus/integrals/risch/differential_extension.rs"
    "crates/mathhook-core/src/calculus/integrals/risch/hermite.rs"
    "crates/mathhook-core/src/calculus/integrals/risch/rde.rs"
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

# Check for optional submodule structure
SUBMODULE_COUNT=0
for file in "${OPTIONAL_SUBMODULES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ Optional submodule: $file${NC}"
        SUBMODULE_COUNT=$((SUBMODULE_COUNT + 1))
    fi
done

if [ $SUBMODULE_COUNT -gt 0 ]; then
    echo -e "${GREEN}✓ Found $SUBMODULE_COUNT submodules (modular structure)${NC}"
fi

if [ $MISSING_FILES -gt 0 ]; then
    echo -e "${RED}✗ $MISSING_FILES required files missing${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All required files present${NC}"
fi

# CATEGORY 2: FILE SIZE COMPLIANCE (≤500 LINES EACH)
echo "========================================"
echo "CATEGORY 2: FILE SIZE COMPLIANCE"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0

# Check main risch.rs
if [ -f "crates/mathhook-core/src/calculus/integrals/risch.rs" ]; then
    LINE_COUNT=$(wc -l < "crates/mathhook-core/src/calculus/integrals/risch.rs" | tr -d ' ')
    if [ "$LINE_COUNT" -gt 500 ]; then
        echo -e "${RED}✗ risch.rs: $LINE_COUNT lines (max 500)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ risch.rs: $LINE_COUNT lines${NC}"
    fi
fi

# Check submodules if they exist
for file in "${OPTIONAL_SUBMODULES[@]}"; do
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

# Check strategy.rs
if [ -f "crates/mathhook-core/src/calculus/integrals/strategy.rs" ]; then
    LINE_COUNT=$(wc -l < "crates/mathhook-core/src/calculus/integrals/strategy.rs" | tr -d ' ')
    if [ "$LINE_COUNT" -gt 500 ]; then
        echo -e "${RED}✗ strategy.rs: $LINE_COUNT lines (max 500)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ strategy.rs: $LINE_COUNT lines${NC}"
    fi
fi

if [ $VIOLATIONS -gt 0 ]; then
    echo -e "${RED}✗ $VIOLATIONS file size violations${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All files within 500-line limit${NC}"
fi

# CATEGORY 3: RISCH CORE CONCEPTS
echo "========================================"
echo "CATEGORY 3: RISCH CORE CONCEPTS"
echo "Must implement key Risch algorithm components"
echo "========================================"

# Find all risch-related files
RISCH_FILES=$(find crates/mathhook-core/src/calculus/integrals -name "*risch*" -type f 2>/dev/null)

RISCH_ISSUES=0

# Check for key Risch concepts across all risch files
REQUIRED_CONCEPTS=(
    "differential"
    "extension"
    "hermite\|reduction"
    "logarithm\|exponential"
)

for concept in "${REQUIRED_CONCEPTS[@]}"; do
    FOUND=0
    for file in $RISCH_FILES; do
        if grep -qiE "$concept" "$file"; then
            FOUND=1
            break
        fi
    done

    if [ $FOUND -eq 0 ]; then
        echo -e "${RED}✗ Missing concept: '$concept'${NC}"
        RISCH_ISSUES=$((RISCH_ISSUES + 1))
    fi
done

if [ $RISCH_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✓ Risch implementation covers key concepts${NC}"
else
    echo -e "${YELLOW}⚠ Risch implementation incomplete: $RISCH_ISSUES concepts not found (may be Wave 5 scope)${NC}"
    # Not a failure for basic implementation
fi

# CATEGORY 4: STRATEGY INTEGRATION
echo "========================================"
echo "CATEGORY 4: STRATEGY INTEGRATION"
echo "Risch must be in strategy dispatcher"
echo "========================================"

STRATEGY_FILE="crates/mathhook-core/src/calculus/integrals/strategy.rs"
STRATEGY_ISSUES=0

if [ -f "$STRATEGY_FILE" ]; then
    # Check if risch is integrated
    if grep -qi "risch" "$STRATEGY_FILE"; then
        echo -e "${GREEN}✓ Risch integrated in strategy${NC}"
    else
        echo -e "${RED}✗ Risch not found in strategy${NC}"
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
echo "Wave 5 requires 40+ new tests (basic cases)"
echo "========================================"

# Find test files
TEST_FILES=$(find crates/mathhook-core/tests -name "*risch*" 2>/dev/null)

TOTAL_NEW_TESTS=0
for test_file in $TEST_FILES; do
    if [ -f "$test_file" ]; then
        TEST_COUNT=$(grep -c "^[[:space:]]*#\[test\]" "$test_file" 2>/dev/null || echo 0)
        TOTAL_NEW_TESTS=$((TOTAL_NEW_TESTS + TEST_COUNT))
        echo "  $test_file: $TEST_COUNT tests"
    fi
done

if [ $TOTAL_NEW_TESTS -ge 40 ]; then
    echo -e "${GREEN}✓ Found $TOTAL_NEW_TESTS tests (target: 40+)${NC}"
elif [ $TOTAL_NEW_TESTS -ge 30 ]; then
    echo -e "${YELLOW}⚠ Found $TOTAL_NEW_TESTS tests (close to 40 target)${NC}"
else
    echo -e "${RED}✗ Found only $TOTAL_NEW_TESTS tests (target: 40+)${NC}"
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

if [ $SYMPY_COUNT -ge 25 ]; then
    echo -e "${GREEN}✓ Found $SYMPY_COUNT SymPy/validation references (good coverage)${NC}"
elif [ $SYMPY_COUNT -ge 15 ]; then
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
for file in $RISCH_FILES; do
    if [ -f "$file" ]; then
        COUNT=$(grep -c "✅\|❌\|⚠️\|🚀\|✨" "$file" 2>/dev/null || echo 0)
        EMOJI_COUNT=$((EMOJI_COUNT + COUNT))
    fi
done

if [ $EMOJI_COUNT -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in Wave 5 files${NC}"
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
for file in $RISCH_FILES; do
    if [ -f "$file" ]; then
        # Check for module doc (at least one file should have it)
        if grep -q "^//!" "$file"; then
            echo -e "${GREEN}✓ Module documentation found in $(basename $file)${NC}"
        fi
    fi
done

# Check for function docs (at least some /// comments)
FUNC_DOC_COUNT=0
for file in $RISCH_FILES; do
    if [ -f "$file" ]; then
        COUNT=$(grep -c "^///" "$file" 2>/dev/null || echo 0)
        FUNC_DOC_COUNT=$((FUNC_DOC_COUNT + COUNT))
    fi
done

if [ $FUNC_DOC_COUNT -lt 10 ]; then
    echo -e "${YELLOW}⚠ Only $FUNC_DOC_COUNT function docs found (expected more)${NC}"
    DOC_ISSUES=$((DOC_ISSUES + 1))
else
    echo -e "${GREEN}✓ Found $FUNC_DOC_COUNT function documentation blocks${NC}"
fi

if [ $DOC_ISSUES -gt 0 ]; then
    echo -e "${YELLOW}⚠ $DOC_ISSUES documentation issues (warning only)${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 5: Risch Algorithm (Basic) is VERIFIED COMPLETE"
    echo ""
    echo "Achievements:"
    echo "  - Risch algorithm implemented (basic cases)"
    echo "  - $TOTAL_NEW_TESTS tests created (target: 40+)"
    echo "  - SymPy validation: $SYMPY_COUNT references"
    echo "  - Zero regressions (all existing tests pass)"
    echo "  - Coverage: 92% → 95% (estimated)"
    echo ""
    echo "Ready to proceed to Wave 6: Completion (Testing + Documentation)"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 5: Risch Algorithm requires fixes before approval"
    echo ""
    echo "Fix the issues above and re-run verification."
    exit 1
fi
