#!/bin/bash

# Wave 10: Equation Solvers Integration for Noncommutative Algebra Verification Script
# Verifies equation solver updates for matrix/operator/quaternion equations
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 10: EQUATION SOLVERS VERIFICATION"
echo "Noncommutative algebra equation solving"
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

LINEAR_LINES=$(wc -l < crates/mathhook-core/src/algebra/solvers/linear.rs 2>/dev/null || echo "0")
MATRIX_EQ_LINES=$(wc -l < crates/mathhook-core/src/algebra/solvers/matrix_equations.rs 2>/dev/null || echo "0")

echo "algebra/solvers/linear.rs: $LINEAR_LINES lines"
echo "algebra/solvers/matrix_equations.rs: $MATRIX_EQ_LINES lines"

if [ "$LINEAR_LINES" -gt 500 ]; then
    echo -e "${RED}✗ linear.rs exceeds 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ linear.rs complies${NC}"
fi

if [ "$MATRIX_EQ_LINES" -gt 500 ]; then
    echo -e "${RED}✗ matrix_equations.rs exceeds 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
elif [ "$MATRIX_EQ_LINES" -eq 0 ]; then
    echo -e "${RED}✗ matrix_equations.rs not found${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ matrix_equations.rs complies${NC}"
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/algebra/solvers/ 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

echo "Running: cargo check -p mathhook-core..."
BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | tail -20
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: LINEAR SOLVER UPDATES
echo "========================================"
echo "CATEGORY 4: LINEAR SOLVER UPDATES"
echo "Must have left/right division support"
echo "========================================"

LEFT_DIV=$(grep -c "solve_left\|left_division\|DivisionSide::Left" crates/mathhook-core/src/algebra/solvers/linear.rs 2>/dev/null || echo "0")
RIGHT_DIV=$(grep -c "solve_right\|right_division\|DivisionSide::Right" crates/mathhook-core/src/algebra/solvers/linear.rs 2>/dev/null || echo "0")

echo "Left division references: $LEFT_DIV"
echo "Right division references: $RIGHT_DIV"

if [ "$LEFT_DIV" -ge 1 ] && [ "$RIGHT_DIV" -ge 1 ]; then
    echo -e "${GREEN}✓ Linear solver has left/right division${NC}"
else
    echo -e "${RED}✗ Linear solver missing division support${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: MATRIX EQUATION SOLVER EXISTS
echo "========================================"
echo "CATEGORY 5: MATRIX EQUATION SOLVER"
echo "New module must exist and be functional"
echo "========================================"

if [ -f "crates/mathhook-core/src/algebra/solvers/matrix_equations.rs" ]; then
    echo -e "${GREEN}✓ matrix_equations.rs exists${NC}"

    MATRIX_SOLVE_COUNT=$(grep -c "pub fn solve_matrix\|pub fn solve_left\|pub fn solve_right" crates/mathhook-core/src/algebra/solvers/matrix_equations.rs 2>/dev/null || echo "0")
    echo "Matrix solve functions: $MATRIX_SOLVE_COUNT"

    if [ "$MATRIX_SOLVE_COUNT" -ge 2 ]; then
        echo -e "${GREEN}✓ Matrix equation solver has solve functions${NC}"
    else
        echo -e "${YELLOW}⚠ Limited matrix solve functions${NC}"
    fi
else
    echo -e "${RED}✗ matrix_equations.rs NOT found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: COMMUTATIVITY DETECTION
echo "========================================"
echo "CATEGORY 6: COMMUTATIVITY DETECTION"
echo "Must check commutativity before solving"
echo "========================================"

COMM_CHECKS=$(grep -c "is_commutative\|commutativity\|Commutativity\|check_commutativity" crates/mathhook-core/src/algebra/solvers/linear.rs 2>/dev/null || echo "0")

echo "Commutativity checks: $COMM_CHECKS"

if [ "$COMM_CHECKS" -ge 1 ]; then
    echo -e "${GREEN}✓ Commutativity detection present${NC}"
else
    echo -e "${RED}✗ Commutativity detection missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: TEST COUNT
echo "========================================"
echo "CATEGORY 7: TEST COUNT"
echo "Target: 35+ tests for matrix equations"
echo "========================================"

TEST_COUNT=$(grep -c "fn test_" crates/mathhook-core/tests/matrix_equation_solver_tests.rs 2>/dev/null || echo "0")

echo "Matrix equation tests found: $TEST_COUNT"

if [ "$TEST_COUNT" -ge 35 ]; then
    echo -e "${GREEN}✓ Test count meets target (35+)${NC}"
elif [ "$TEST_COUNT" -ge 30 ]; then
    echo -e "${YELLOW}⚠ Test count close to target: $TEST_COUNT (target 35+)${NC}"
else
    echo -e "${RED}✗ Test count below target: $TEST_COUNT (target 35+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: TEST VALIDATION
echo "========================================"
echo "CATEGORY 8: TEST VALIDATION"
echo "Equation solver tests must pass"
echo "========================================"

echo "Running matrix equation solver tests..."
TEST_OUTPUT=$(cargo test --test matrix_equation_solver_tests 2>&1)
TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -c "test result: ok" || echo "0")

if [ "$TEST_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Matrix equation solver tests passed${NC}"
    echo "$TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Matrix equation solver tests failed${NC}"
    echo "$TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION QUALITY"
echo "New functions must have documentation"
echo "========================================"

DOCTEST_COUNT=$(grep -c "/// " crates/mathhook-core/src/algebra/solvers/matrix_equations.rs 2>/dev/null || echo "0")
EXAMPLE_COUNT=$(grep -c "# Examples" crates/mathhook-core/src/algebra/solvers/matrix_equations.rs 2>/dev/null || echo "0")

echo "Documentation lines: $DOCTEST_COUNT"
echo "Example blocks: $EXAMPLE_COUNT"

if [ "$EXAMPLE_COUNT" -ge 2 ]; then
    echo -e "${GREEN}✓ Documentation quality is good${NC}"
else
    echo -e "${YELLOW}⚠ Could use more examples${NC}"
fi

# CATEGORY 10: ZERO REGRESSIONS
echo "========================================"
echo "CATEGORY 10: ZERO REGRESSIONS"
echo "All existing solver tests must pass"
echo "========================================"

echo "Running all solver tests..."
SOLVER_TEST_OUTPUT=$(cargo test -p mathhook-core solver 2>&1)
SOLVER_PASSED=$(echo "$SOLVER_TEST_OUTPUT" | grep "test result: ok" | wc -l)

if [ "$SOLVER_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Solver tests passed (zero regressions)${NC}"
    echo "$SOLVER_TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Solver tests failed (regressions detected)${NC}"
    echo "$SOLVER_TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 10: Equation Solvers Integration is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 10: Equation Solvers requires fixes before approval"
    exit 1
fi
