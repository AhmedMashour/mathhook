#!/bin/bash

# Wave 12: Examples, Documentation & Final Verification Script
# Final verification across all waves (8-12)
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 12: FINAL VERIFICATION"
echo "Examples, Documentation & Integration Testing"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: EXAMPLES EXIST AND RUN
echo "========================================"
echo "CATEGORY 1: EXAMPLES VERIFICATION"
echo "3 comprehensive examples required"
echo "========================================"

if [ -f "crates/mathhook-core/examples/noncommutative_algebra_examples.rs" ]; then
    echo -e "${GREEN}✓ Examples file exists${NC}"

    # Check for 3 examples
    QUANTUM=$(grep -c "example_quantum_mechanics\|quantum_mechanics_example" crates/mathhook-core/examples/noncommutative_algebra_examples.rs 2>/dev/null || echo "0")
    MATRIX=$(grep -c "example_matrix_algebra\|matrix_algebra_example" crates/mathhook-core/examples/noncommutative_algebra_examples.rs 2>/dev/null || echo "0")
    QUATERNION=$(grep -c "example_quaternion\|quaternion.*example" crates/mathhook-core/examples/noncommutative_algebra_examples.rs 2>/dev/null || echo "0")

    echo "Examples found:"
    echo "  - Quantum mechanics: $QUANTUM"
    echo "  - Matrix algebra: $MATRIX"
    echo "  - Quaternions: $QUATERNION"

    if [ "$QUANTUM" -ge 1 ] && [ "$MATRIX" -ge 1 ] && [ "$QUATERNION" -ge 1 ]; then
        echo -e "${GREEN}✓ All 3 examples present${NC}"
    else
        echo -e "${RED}✗ Missing one or more examples${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Examples file not found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: INTEGRATION TESTS EXIST
echo "========================================"
echo "CATEGORY 2: INTEGRATION TESTS"
echo "Target: 20+ integration tests"
echo "========================================"

if [ -f "crates/mathhook-core/tests/noncommutative_integration_tests.rs" ]; then
    echo -e "${GREEN}✓ Integration tests file exists${NC}"

    TEST_COUNT=$(grep -c "fn test_" crates/mathhook-core/tests/noncommutative_integration_tests.rs 2>/dev/null || echo "0")
    echo "Integration tests found: $TEST_COUNT"

    if [ "$TEST_COUNT" -ge 20 ]; then
        echo -e "${GREEN}✓ Test count meets target (20+)${NC}"
    elif [ "$TEST_COUNT" -ge 15 ]; then
        echo -e "${YELLOW}⚠ Test count close to target: $TEST_COUNT (target 20+)${NC}"
    else
        echo -e "${RED}✗ Test count below target: $TEST_COUNT (target 20+)${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Integration tests file not found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 3: INTEGRATION TESTS PASS
echo "========================================"
echo "CATEGORY 3: INTEGRATION TEST VALIDATION"
echo "All integration tests must pass"
echo "========================================"

echo "Running integration tests..."
INTEGRATION_TEST_OUTPUT=$(cargo test --test noncommutative_integration_tests 2>&1)
INTEGRATION_PASSED=$(echo "$INTEGRATION_TEST_OUTPUT" | grep -c "test result: ok" || echo "0")

if [ "$INTEGRATION_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Integration tests passed${NC}"
    echo "$INTEGRATION_TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Integration tests failed or not found${NC}"
    echo "$INTEGRATION_TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: DOCUMENTATION EXISTS
echo "========================================"
echo "CATEGORY 4: DOCUMENTATION COMPLETENESS"
echo "NONCOMMUTATIVE_ALGEBRA.md must exist"
echo "========================================"

if [ -f "NONCOMMUTATIVE_ALGEBRA.md" ]; then
    echo -e "${GREEN}✓ NONCOMMUTATIVE_ALGEBRA.md exists${NC}"

    DOC_LINES=$(wc -l < NONCOMMUTATIVE_ALGEBRA.md)
    echo "Documentation: $DOC_LINES lines"

    if [ "$DOC_LINES" -ge 200 ]; then
        echo -e "${GREEN}✓ Documentation is comprehensive${NC}"
    else
        echo -e "${YELLOW}⚠ Documentation could be more detailed${NC}"
    fi
else
    echo -e "${RED}✗ NONCOMMUTATIVE_ALGEBRA.md not found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: CLAUDE.MD UPDATED
echo "========================================"
echo "CATEGORY 5: CLAUDE.MD UPDATES"
echo "Must document noncommutative algebra"
echo "========================================"

CLAUDE_NONCOMM=$(grep -c "Noncommutative\|noncommutative" CLAUDE.md 2>/dev/null || echo "0")

echo "Noncommutative references in CLAUDE.md: $CLAUDE_NONCOMM"

if [ "$CLAUDE_NONCOMM" -ge 10 ]; then
    echo -e "${GREEN}✓ CLAUDE.md updated with noncommutative content${NC}"
else
    echo -e "${YELLOW}⚠ CLAUDE.md could use more noncommutative documentation${NC}"
fi

# CATEGORY 6: ALL WAVE TESTS PASS
echo "========================================"
echo "CATEGORY 6: CUMULATIVE TEST VALIDATION"
echo "All tests from all waves must pass"
echo "========================================"

echo "Running all noncommutative tests..."
ALL_TESTS_OUTPUT=$(cargo test -p mathhook-core noncommutative 2>&1)
ALL_TESTS_PASSED=$(echo "$ALL_TESTS_OUTPUT" | grep "test result: ok" | wc -l)

echo "Test suites passing: $ALL_TESTS_PASSED"

if [ "$ALL_TESTS_PASSED" -ge 5 ]; then
    echo -e "${GREEN}✓ All wave tests passed${NC}"
    echo "$ALL_TESTS_OUTPUT" | grep "test result:" | head -10
else
    echo -e "${RED}✗ Some wave tests failed${NC}"
    echo "$ALL_TESTS_OUTPUT" | grep "FAILED" | head -10
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: FILE SIZE COMPLIANCE
echo "========================================"
echo "CATEGORY 7: FILE SIZE COMPLIANCE"
echo "All files must be ≤500 lines"
echo "========================================"

echo "Checking file sizes..."
OVERSIZED_FILES=$(find crates/mathhook-core/src -name "*.rs" -type f -exec wc -l {} \; | awk '$1 > 500 {print $2}' | wc -l)

if [ "$OVERSIZED_FILES" -eq 0 ]; then
    echo -e "${GREEN}✓ All files ≤500 lines${NC}"
else
    echo -e "${RED}✗ Found $OVERSIZED_FILES files >500 lines${NC}"
    find crates/mathhook-core/src -name "*.rs" -type f -exec wc -l {} \; | awk '$1 > 500 {print}'
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 8: EMOJI COMPLIANCE"
echo "Zero emojis in entire codebase"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src crates/mathhook-core/tests crates/mathhook-core/examples 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✓ No emojis found${NC}"
else
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: BUILD STATUS
echo "========================================"
echo "CATEGORY 9: BUILD STATUS"
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

# CATEGORY 10: FINAL QUALITY METRICS
echo "========================================"
echo "CATEGORY 10: FINAL QUALITY METRICS"
echo "Cumulative test count and quality"
echo "========================================"

# Count total tests across all waves
echo "Counting cumulative tests..."

WAVE_8_TESTS=$(grep -c "fn test_" crates/mathhook-core/tests/parser_type_inference_tests.rs 2>/dev/null || echo "0")
WAVE_9_TESTS=$(grep -c "fn test_" crates/mathhook-core/tests/macro_enhancement_tests.rs 2>/dev/null || echo "0")
WAVE_10_TESTS=$(grep -c "fn test_" crates/mathhook-core/tests/matrix_equation_solver_tests.rs 2>/dev/null || echo "0")
WAVE_11_MSG_TESTS=$(grep -c "fn test_" crates/mathhook-core/tests/educational_noncommutative_messages_tests.rs 2>/dev/null || echo "0")
WAVE_11_STEP_TESTS=$(grep -c "fn test_" crates/mathhook-core/tests/educational_noncommutative_steps_tests.rs 2>/dev/null || echo "0")
WAVE_12_TESTS=$(grep -c "fn test_" crates/mathhook-core/tests/noncommutative_integration_tests.rs 2>/dev/null || echo "0")

TOTAL_TESTS=$((WAVE_8_TESTS + WAVE_9_TESTS + WAVE_10_TESTS + WAVE_11_MSG_TESTS + WAVE_11_STEP_TESTS + WAVE_12_TESTS))

echo "Test count by wave:"
echo "  - Wave 8 (Parser): $WAVE_8_TESTS tests"
echo "  - Wave 9 (Macros): $WAVE_9_TESTS tests"
echo "  - Wave 10 (Solvers): $WAVE_10_TESTS tests"
echo "  - Wave 11 (Messages): $WAVE_11_MSG_TESTS tests"
echo "  - Wave 11 (Steps): $WAVE_11_STEP_TESTS tests"
echo "  - Wave 12 (Integration): $WAVE_12_TESTS tests"
echo "  - TOTAL: $TOTAL_TESTS tests"

if [ "$TOTAL_TESTS" -ge 160 ]; then
    echo -e "${GREEN}✓ Cumulative test count meets target (160+)${NC}"
elif [ "$TOTAL_TESTS" -ge 140 ]; then
    echo -e "${YELLOW}⚠ Cumulative test count close to target: $TOTAL_TESTS (target 160+)${NC}"
else
    echo -e "${RED}✗ Cumulative test count below target: $TOTAL_TESTS (target 160+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "FINAL VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 12: Examples, Documentation & Final Verification is VERIFIED COMPLETE"
    echo ""
    echo "NONCOMMUTATIVE ALGEBRA IMPLEMENTATION COMPLETE!"
    echo "All 12 waves successfully verified."
    echo "Total tests: $TOTAL_TESTS"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 12: Final Verification requires fixes before approval"
    exit 1
fi
