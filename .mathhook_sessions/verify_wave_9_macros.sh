#!/bin/bash

# Wave 9: symbol! and symbols! Macro Enhancement Verification Script
# Verifies macro implementations for all four symbol types
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 9: MACRO ENHANCEMENT VERIFICATION"
echo "symbol!() and symbols!() macros for all four types"
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

MACROS_LINES=$(wc -l < crates/mathhook-core/src/macros/expressions.rs)
SPECIALIZED_LINES=$(wc -l < crates/mathhook-core/src/core/expression/constructors/specialized.rs 2>/dev/null || echo "0")

echo "macros/expressions.rs: $MACROS_LINES lines"
echo "constructors/specialized.rs: $SPECIALIZED_LINES lines"

if [ "$MACROS_LINES" -gt 500 ]; then
    echo -e "${RED}✗ macros/expressions.rs exceeds 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ macros/expressions.rs complies${NC}"
fi

if [ "$SPECIALIZED_LINES" -gt 500 ]; then
    echo -e "${RED}✗ specialized.rs exceeds 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ specialized.rs complies${NC}"
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/macros/ crates/mathhook-core/src/core/expression/constructors/ 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/macros/ crates/mathhook-core/src/core/expression/constructors/ 2>/dev/null
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 3: symbol!() MACRO VERIFICATION
echo "========================================"
echo "CATEGORY 3: symbol!() MACRO VERIFICATION"
echo "Must support all four types: scalar, matrix, operator, quaternion"
echo "========================================"

# Check for type support in symbol! macro
SYMBOL_MACRO_SCALAR=$(grep -c "Symbol::scalar" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")
SYMBOL_MACRO_MATRIX=$(grep -c "Symbol::matrix" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")
SYMBOL_MACRO_OPERATOR=$(grep -c "Symbol::operator" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")
SYMBOL_MACRO_QUATERNION=$(grep -c "Symbol::quaternion" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")

echo "symbol!() macro references:"
echo "  - Scalar: $SYMBOL_MACRO_SCALAR"
echo "  - Matrix: $SYMBOL_MACRO_MATRIX"
echo "  - Operator: $SYMBOL_MACRO_OPERATOR"
echo "  - Quaternion: $SYMBOL_MACRO_QUATERNION"

if [ "$SYMBOL_MACRO_SCALAR" -ge 1 ] && [ "$SYMBOL_MACRO_MATRIX" -ge 1 ] && [ "$SYMBOL_MACRO_OPERATOR" -ge 1 ] && [ "$SYMBOL_MACRO_QUATERNION" -ge 1 ]; then
    echo -e "${GREEN}✓ symbol!() supports all four types${NC}"
else
    echo -e "${RED}✗ symbol!() missing type support${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: symbols!() MACRO VERIFICATION
echo "========================================"
echo "CATEGORY 4: symbols!() MACRO VERIFICATION"
echo "Must exist and support bulk creation for all four types"
echo "========================================"

SYMBOLS_MACRO_EXISTS=$(grep -c "macro_rules! symbols" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")

if [ "$SYMBOLS_MACRO_EXISTS" -ge 1 ]; then
    echo -e "${GREEN}✓ symbols!() macro exists${NC}"

    # Check for type support in symbols! macro
    SYMBOLS_SCALAR=$(grep -A 50 "macro_rules! symbols" crates/mathhook-core/src/macros/expressions.rs | grep -c "scalar" || echo "0")
    SYMBOLS_MATRIX=$(grep -A 50 "macro_rules! symbols" crates/mathhook-core/src/macros/expressions.rs | grep -c "matrix" || echo "0")
    SYMBOLS_OPERATOR=$(grep -A 50 "macro_rules! symbols" crates/mathhook-core/src/macros/expressions.rs | grep -c "operator" || echo "0")
    SYMBOLS_QUATERNION=$(grep -A 50 "macro_rules! symbols" crates/mathhook-core/src/macros/expressions.rs | grep -c "quaternion" || echo "0")

    echo "symbols!() macro type support:"
    echo "  - Scalar: $SYMBOLS_SCALAR references"
    echo "  - Matrix: $SYMBOLS_MATRIX references"
    echo "  - Operator: $SYMBOLS_OPERATOR references"
    echo "  - Quaternion: $SYMBOLS_QUATERNION references"

    if [ "$SYMBOLS_SCALAR" -ge 1 ] && [ "$SYMBOLS_MATRIX" -ge 1 ] && [ "$SYMBOLS_OPERATOR" -ge 1 ] && [ "$SYMBOLS_QUATERNION" -ge 1 ]; then
        echo -e "${GREEN}✓ symbols!() supports all four types${NC}"
    else
        echo -e "${YELLOW}⚠ symbols!() may have incomplete type support${NC}"
    fi
else
    echo -e "${RED}✗ symbols!() macro NOT found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: COMMUTATOR/ANTICOMMUTATOR FUNCTIONS
echo "========================================"
echo "CATEGORY 5: COMMUTATOR/ANTICOMMUTATOR FUNCTIONS"
echo "Must have commutator [A,B] and anticommutator {A,B} support"
echo "========================================"

COMMUTATOR_EXISTS=$(grep -r "commutator\|Commutator" crates/mathhook-core/src/core/expression/constructors/ 2>/dev/null | wc -l)
ANTICOMMUTATOR_EXISTS=$(grep -r "anticommutator\|Anticommutator" crates/mathhook-core/src/core/expression/constructors/ 2>/dev/null | wc -l)

echo "Commutator references: $COMMUTATOR_EXISTS"
echo "Anticommutator references: $ANTICOMMUTATOR_EXISTS"

if [ "$COMMUTATOR_EXISTS" -ge 1 ]; then
    echo -e "${GREEN}✓ Commutator function exists${NC}"
else
    echo -e "${RED}✗ Commutator function NOT found${NC}"
    FAILURES=$((FAILURES + 1))
fi

if [ "$ANTICOMMUTATOR_EXISTS" -ge 1 ]; then
    echo -e "${GREEN}✓ Anticommutator function exists${NC}"
else
    echo -e "${RED}✗ Anticommutator function NOT found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: BUILD STATUS
echo "========================================"
echo "CATEGORY 6: BUILD STATUS"
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

# CATEGORY 7: TEST VALIDATION
echo "========================================"
echo "CATEGORY 7: TEST VALIDATION"
echo "Macro tests must pass"
echo "========================================"

echo "Running macro tests..."
TEST_OUTPUT=$(cargo test -p mathhook-core macros 2>&1)
TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -c "test result: ok" || echo "0")

if [ "$TEST_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Macro tests passed${NC}"
    echo "$TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Macro tests failed${NC}"
    echo "$TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: TEST COUNT
echo "========================================"
echo "CATEGORY 8: TEST COUNT"
echo "Target: 25+ new macro tests"
echo "========================================"

# Count test functions in macro test files
MACRO_TEST_COUNT=$(grep -r "fn test_" crates/mathhook-core/src/macros/ crates/mathhook-core/tests/macro_*.rs 2>/dev/null | wc -l)

echo "Macro test functions found: $MACRO_TEST_COUNT"

if [ "$MACRO_TEST_COUNT" -ge 25 ]; then
    echo -e "${GREEN}✓ Test count meets target (25+)${NC}"
elif [ "$MACRO_TEST_COUNT" -ge 20 ]; then
    echo -e "${YELLOW}⚠ Test count close to target: $MACRO_TEST_COUNT (target 25+)${NC}"
else
    echo -e "${RED}✗ Test count below target: $MACRO_TEST_COUNT (target 25+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION QUALITY"
echo "Macros must have comprehensive examples and doctests"
echo "========================================"

DOCTEST_COUNT=$(grep -c "/// ```rust" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")
EXAMPLE_COUNT=$(grep -c "# Examples" crates/mathhook-core/src/macros/expressions.rs 2>/dev/null || echo "0")

echo "Documentation examples: $EXAMPLE_COUNT"
echo "Doctests: $DOCTEST_COUNT"

if [ "$EXAMPLE_COUNT" -ge 2 ] && [ "$DOCTEST_COUNT" -ge 2 ]; then
    echo -e "${GREEN}✓ Documentation quality is good${NC}"
else
    echo -e "${YELLOW}⚠ Could use more documentation (examples: $EXAMPLE_COUNT, doctests: $DOCTEST_COUNT)${NC}"
fi

# CATEGORY 10: BACKWARD COMPATIBILITY
echo "========================================"
echo "CATEGORY 10: BACKWARD COMPATIBILITY"
echo "symbol!(x) must still default to scalar (backward compatible)"
echo "========================================"

# Run specific test for backward compatibility
COMPAT_TEST=$(cargo test -p mathhook-core test_symbol_macro_scalar_default 2>&1)

if echo "$COMPAT_TEST" | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ Backward compatibility verified${NC}"
else
    echo -e "${RED}✗ Backward compatibility broken${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 9: Macro Enhancement is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 9: Macro Enhancement requires fixes before approval"
    exit 1
fi
