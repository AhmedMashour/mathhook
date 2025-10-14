#!/bin/bash

# Educational Wave 2 Verification Script
# Enforces CLAUDE.md compliance and validates agent work quality

echo "========================================"
echo "EDUCATIONAL WAVE 2 VERIFICATION"
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

echo "Checking Wave 2 modified files..."

# Agent 2A files - polynomial is now a module
if [ -d "crates/mathhook-core/src/algebra/solvers/polynomial" ]; then
    echo "Polynomial is a module (split into sub-files):"
    find crates/mathhook-core/src/algebra/solvers/polynomial -name "*.rs" -exec sh -c 'LINES=$(wc -l < "$1"); FILE=$(basename "$1"); if [ $LINES -gt 500 ]; then echo -e "${RED}✗ polynomial/$FILE: $LINES lines${NC}"; exit 1; else echo -e "${GREEN}✓ polynomial/$FILE: $LINES lines${NC}"; fi' sh {} \;
else
    POLYNOMIAL_LINES=$(wc -l < crates/mathhook-core/src/algebra/solvers/polynomial.rs 2>/dev/null || echo "0")
    if [ "$POLYNOMIAL_LINES" -gt 500 ]; then
        OVER=$((POLYNOMIAL_LINES - 500))
        PERCENT=$(( (OVER * 100) / 500 ))
        echo -e "${RED}✗ polynomial.rs: $POLYNOMIAL_LINES lines (+$PERCENT% over limit)${NC}"
        FAILURES=$((FAILURES + 1))
    else
        echo -e "${GREEN}✓ polynomial.rs: $POLYNOMIAL_LINES lines${NC}"
    fi
fi

SYSTEMS_LINES=$(wc -l < crates/mathhook-core/src/algebra/solvers/systems.rs)

if [ "$SYSTEMS_LINES" -gt 500 ]; then
    OVER=$((SYSTEMS_LINES - 500))
    PERCENT=$(( (OVER * 100) / 500 ))
    echo -e "${RED}✗ systems.rs: $SYSTEMS_LINES lines (+$PERCENT% over limit)${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ systems.rs: $SYSTEMS_LINES lines${NC}"
fi

# Agent 2B files
STEP_BY_STEP_LINES=$(wc -l < crates/mathhook-core/src/educational/step_by_step.rs)

if [ "$STEP_BY_STEP_LINES" -gt 500 ]; then
    OVER=$((STEP_BY_STEP_LINES - 500))
    PERCENT=$(( (OVER * 100) / 500 ))
    echo -e "${YELLOW}⚠ step_by_step.rs: $STEP_BY_STEP_LINES lines (+$PERCENT% over limit) [PRE-EXISTING]${NC}"
fi

echo ""
echo "========================================
CATEGORY 2: EMOJI COMPLIANCE
CLAUDE.md: No emojis in code
========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/algebra/solvers/ crates/mathhook-core/src/educational/step_by_step.rs 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in Wave 2 files${NC}"
    grep -rn "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/algebra/solvers/ crates/mathhook-core/src/educational/step_by_step.rs 2>/dev/null | head -5
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found in Wave 2 files${NC}"
fi

echo ""
echo "========================================
CATEGORY 3: TEST VALIDATION
Must have content validation tests (NO false positives)
========================================"

# Check for polynomial/system test file
if [ -f "crates/mathhook-core/tests/equation_solver_education_test.rs" ]; then
    echo -e "${GREEN}✓ equation_solver_education_test.rs exists${NC}"

    # Run tests
    echo "Running equation solver education tests..."
    TEST_OUTPUT=$(cargo test -p mathhook-core --test equation_solver_education_test 2>&1)

    if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
        PASSED=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo -e "${GREEN}✓ Equation solver tests: $PASSED passed${NC}"

        # Check for minimum 8 tests (Agent 2A requirement)
        if [ "$PASSED" -lt 8 ]; then
            echo -e "${RED}✗ Only $PASSED tests (requirement: 8+)${NC}"
            FAILURES=$((FAILURES + 1))
        fi
    else
        echo -e "${RED}✗ Equation solver tests FAILED${NC}"
        echo "$TEST_OUTPUT" | grep -E "error|FAILED"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ equation_solver_education_test.rs NOT FOUND${NC}"
    echo "  Agent 2A did not create required test file"
    FAILURES=$((FAILURES + 1))
fi

# Check for algebraic manipulation test file
if [ -f "crates/mathhook-core/tests/algebraic_manipulation_education_test.rs" ]; then
    echo -e "${GREEN}✓ algebraic_manipulation_education_test.rs exists${NC}"

    # Run tests
    echo "Running algebraic manipulation education tests..."
    TEST_OUTPUT=$(cargo test -p mathhook-core --test algebraic_manipulation_education_test 2>&1)

    if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
        PASSED=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo -e "${GREEN}✓ Algebraic manipulation tests: $PASSED passed${NC}"

        # Check for minimum 12 tests (Agent 2B requirement)
        if [ "$PASSED" -lt 12 ]; then
            echo -e "${RED}✗ Only $PASSED tests (requirement: 12+)${NC}"
            FAILURES=$((FAILURES + 1))
        fi
    else
        echo -e "${RED}✗ Algebraic manipulation tests FAILED${NC}"
        echo "$TEST_OUTPUT" | grep -E "error|FAILED"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ algebraic_manipulation_education_test.rs NOT FOUND${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================
CATEGORY 4: CONTENT VALIDATION (Anti-False-Positive)
Tests must validate actual math content
========================================"

echo "Checking for false-positive patterns..."

# Check algebraic manipulation tests for content validation
if [ -f "crates/mathhook-core/tests/algebraic_manipulation_education_test.rs" ]; then
    FALSE_POSITIVE_COUNT=$(grep -E "assert.*steps\.len\(\) > 0|assert.*!.*is_empty\(\)" crates/mathhook-core/tests/algebraic_manipulation_education_test.rs | wc -l)
    CONTENT_VALIDATION_COUNT=$(grep -E "has_step_containing|assert.*contains" crates/mathhook-core/tests/algebraic_manipulation_education_test.rs | wc -l)

    echo "Algebraic manipulation tests:"
    echo "  Structure-only checks: $FALSE_POSITIVE_COUNT"
    echo "  Content validation checks: $CONTENT_VALIDATION_COUNT"

    if [ "$CONTENT_VALIDATION_COUNT" -lt 10 ]; then
        echo -e "${YELLOW}⚠ Low content validation count (expected 10+)${NC}"
    else
        echo -e "${GREEN}✓ Good content validation coverage${NC}"
    fi
fi

# Check equation solver tests if they exist
if [ -f "crates/mathhook-core/tests/equation_solver_education_test.rs" ]; then
    FALSE_POSITIVE_COUNT=$(grep -E "assert.*steps\.len\(\) > 0|assert.*!.*is_empty\(\)" crates/mathhook-core/tests/equation_solver_education_test.rs | wc -l)
    CONTENT_VALIDATION_COUNT=$(grep -E "has_step_containing|assert.*contains" crates/mathhook-core/tests/equation_solver_education_test.rs | wc -l)

    echo "Equation solver tests:"
    echo "  Structure-only checks: $FALSE_POSITIVE_COUNT"
    echo "  Content validation checks: $CONTENT_VALIDATION_COUNT"

    if [ "$CONTENT_VALIDATION_COUNT" -lt 8 ]; then
        echo -e "${YELLOW}⚠ Low content validation count (expected 8+)${NC}"
    else
        echo -e "${GREEN}✓ Good content validation coverage${NC}"
    fi
fi

echo ""
echo "========================================
CATEGORY 5: IMPLEMENTATION COMPLETENESS
Verify all required operations implemented
========================================"

echo "Checking polynomial solver implementation..."
if grep -rq "Rational Root Theorem" crates/mathhook-core/src/algebra/solvers/polynomial*; then
    echo -e "${GREEN}✓ Polynomial solver has Rational Root Theorem${NC}"
else
    echo -e "${RED}✗ Polynomial solver missing Rational Root Theorem${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo "Checking system solver implementation..."
if grep -q "Substitution Method\|substitution method" crates/mathhook-core/src/algebra/solvers/systems.rs; then
    echo -e "${GREEN}✓ System solver has substitution method${NC}"
else
    echo -e "${RED}✗ System solver missing substitution method${NC}"
    FAILURES=$((FAILURES + 1))
fi

if grep -q "Elimination Method\|elimination method" crates/mathhook-core/src/algebra/solvers/systems.rs; then
    echo -e "${GREEN}✓ System solver has elimination method${NC}"
else
    echo -e "${RED}✗ System solver missing elimination method${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo "Checking algebraic manipulation implementations..."
if grep -q "combine like terms\|like terms" crates/mathhook-core/src/educational/step_by_step.rs; then
    echo -e "${GREEN}✓ Simplification implemented${NC}"
else
    echo -e "${RED}✗ Simplification not fully implemented${NC}"
    FAILURES=$((FAILURES + 1))
fi

if grep -q "FOIL\|distributive" crates/mathhook-core/src/educational/step_by_step.rs; then
    echo -e "${GREEN}✓ Expansion implemented${NC}"
else
    echo -e "${RED}✗ Expansion not fully implemented${NC}"
    FAILURES=$((FAILURES + 1))
fi

if grep -q "GCF\|greatest common factor" crates/mathhook-core/src/educational/step_by_step.rs; then
    echo -e "${GREEN}✓ Factorization implemented${NC}"
else
    echo -e "${RED}✗ Factorization not fully implemented${NC}"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================
CATEGORY 6: GLOBAL FORMATTER USAGE
CLAUDE.md: Must use global formatter, not custom
========================================"

echo "Checking for custom educational formatters..."
CUSTOM_FORMATTER=$(grep -rn "impl.*Formatter.*for.*Expression" crates/mathhook-core/src/educational/ 2>/dev/null | grep -v "LaTeXFormatter" | wc -l)

if [ "$CUSTOM_FORMATTER" -gt 0 ]; then
    echo -e "${RED}✗ Found custom educational formatters (violation)${NC}"
    grep -rn "impl.*Formatter.*for.*Expression" crates/mathhook-core/src/educational/ | grep -v "LaTeXFormatter"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No custom educational formatters (using global formatter)${NC}"
fi

echo "Verifying global formatter usage..."
LATEX_USAGE=$(grep -rn "\.to_latex\(\)" crates/mathhook-core/src/algebra/solvers/ crates/mathhook-core/src/educational/step_by_step.rs 2>/dev/null | wc -l)

if [ "$LATEX_USAGE" -gt 0 ]; then
    echo -e "${GREEN}✓ Using .to_latex() (global formatter) - $LATEX_USAGE occurrences${NC}"
else
    echo -e "${YELLOW}⚠ No .to_latex() usage found (may be using message registry)${NC}"
fi

echo ""
echo "========================================
CATEGORY 7: FULL TEST SUITE REGRESSION
Ensure no tests broken by Wave 2 changes
========================================"

echo "Running full test suite..."
FULL_TEST_OUTPUT=$(cargo test -p mathhook-core 2>&1)

if echo "$FULL_TEST_OUTPUT" | grep -q "test result: ok"; then
    TOTAL_PASSED=$(echo "$FULL_TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | awk '{sum+=$1} END {print sum}')
    echo -e "${GREEN}✓ All tests passing: $TOTAL_PASSED total${NC}"
else
    echo -e "${RED}✗ Some tests FAILED${NC}"
    echo "$FULL_TEST_OUTPUT" | grep -E "FAILED|error"
    FAILURES=$((FAILURES + 1))
fi

echo ""
echo "========================================
VERIFICATION SUMMARY
========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Educational Wave 2 is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s) found${NC}"
    echo "Wave 2 requires fixes before approval"
    exit 1
fi
