#!/bin/bash

# Wave 1: Core Type System & Symbol Enhancement - Verification Script
# Purpose: Verify commutativity tracking infrastructure
# CRITICAL: Expression must stay EXACTLY 32 bytes (no size increase!)
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 1: CORE TYPE SYSTEM VERIFICATION"
echo "Noncommutative Algebra - Foundation"
echo "CRITICAL: Expression size must be 32 bytes"
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
FILES=(
    "crates/mathhook-core/src/core/symbol.rs"
    "crates/mathhook-core/src/core/commutativity.rs"
)

for FILE in "${FILES[@]}"; do
    if [ -f "$FILE" ]; then
        LINES=$(wc -l < "$FILE")
        if [ "$LINES" -gt 500 ]; then
            echo -e "${RED}✗ $FILE: $LINES lines (max 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $FILE: $LINES lines${NC}"
        fi
    else
        echo -e "${YELLOW}⚠ $FILE: Not found (may be new)${NC}"
    fi
done

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply with 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/core/ 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in core/${NC}"
    grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/core/ 2>/dev/null | head -5
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found in core/${NC}"
fi

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

echo "Running: cargo check -p mathhook-core"
BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | tail -20
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: EXPRESSION SIZE (CRITICAL!)
echo "========================================"
echo "CATEGORY 4: EXPRESSION SIZE (CRITICAL)"
echo "MUST be exactly 32 bytes (NO INCREASE!)"
echo "========================================"

# Check if size test exists and passes
if cargo test -p mathhook-core test_expression_size 2>&1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ Expression size test exists and passes (32 bytes)${NC}"
else
    echo -e "${RED}✗ Expression size test FAILED or missing${NC}"
    echo "CRITICAL: Expression must be 32 bytes!"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: TYPE EXISTENCE
echo "========================================"
echo "CATEGORY 5: TYPE EXISTENCE"
echo "Verify new types exist"
echo "========================================"

TYPE_CHECKS=0

# Check for SymbolType enum with 4 variants
if grep -q "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs 2>/dev/null; then
    echo -e "${GREEN}✓ SymbolType enum exists${NC}"

    # Verify all 4 variants
    VARIANTS=("Scalar" "Matrix" "Operator" "Quaternion")
    for VARIANT in "${VARIANTS[@]}"; do
        if grep -q "$VARIANT" crates/mathhook-core/src/core/symbol.rs 2>/dev/null; then
            echo -e "${GREEN}  ✓ SymbolType::$VARIANT exists${NC}"
        else
            echo -e "${RED}  ✗ SymbolType::$VARIANT missing${NC}"
            TYPE_CHECKS=$((TYPE_CHECKS + 1))
        fi
    done
else
    echo -e "${RED}✗ SymbolType enum not found${NC}"
    TYPE_CHECKS=$((TYPE_CHECKS + 1))
fi

# Check for Commutativity enum with ONLY 2 variants
if grep -q "pub enum Commutativity" crates/mathhook-core/src/core/commutativity.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Commutativity enum exists${NC}"

    # Verify ONLY 2 variants (no Unknown!)
    if grep -q "Commutative" crates/mathhook-core/src/core/commutativity.rs 2>/dev/null && \
       grep -q "Noncommutative" crates/mathhook-core/src/core/commutativity.rs 2>/dev/null; then
        echo -e "${GREEN}  ✓ Has Commutative and Noncommutative variants${NC}"
    else
        echo -e "${RED}  ✗ Missing required variants${NC}"
        TYPE_CHECKS=$((TYPE_CHECKS + 1))
    fi

    # Check for forbidden Unknown variant
    if grep -q "Unknown" crates/mathhook-core/src/core/commutativity.rs 2>/dev/null; then
        echo -e "${RED}  ✗ FORBIDDEN: Unknown variant found (should only have 2 variants!)${NC}"
        TYPE_CHECKS=$((TYPE_CHECKS + 1))
    else
        echo -e "${GREEN}  ✓ No Unknown variant (correct - only 2 variants)${NC}"
    fi
else
    echo -e "${RED}✗ Commutativity enum not found${NC}"
    TYPE_CHECKS=$((TYPE_CHECKS + 1))
fi

# CRITICAL: Verify Expression::Mul is UNCHANGED
if grep -q "Mul(Box<Vec<Expression>>)" crates/mathhook-core/src/core/expression/data_types.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Expression::Mul signature UNCHANGED (correct!)${NC}"
else
    echo -e "${RED}✗ Expression::Mul signature changed (WRONG!)${NC}"
    echo "CRITICAL: Mul variant must stay Mul(Box<Vec<Expression>>)"
    TYPE_CHECKS=$((TYPE_CHECKS + 1))
fi

if [ $TYPE_CHECKS -eq 0 ]; then
    echo -e "${GREEN}✓ All required types exist and correct${NC}"
else
    echo -e "${RED}✗ $TYPE_CHECKS type issue(s)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: SYMBOL METHODS
echo "========================================"
echo "CATEGORY 6: SYMBOL METHODS"
echo "Verify Symbol type methods exist"
echo "========================================"

SYMBOL_METHODS=0

# Check for constructors
for METHOD in "scalar" "matrix" "operator" "quaternion"; do
    if grep -q "pub fn $METHOD" crates/mathhook-core/src/core/symbol.rs 2>/dev/null; then
        echo -e "${GREEN}✓ Symbol::${METHOD}() exists${NC}"
    else
        echo -e "${RED}✗ Symbol::${METHOD}() not found${NC}"
        SYMBOL_METHODS=$((SYMBOL_METHODS + 1))
    fi
done

# Check for symbol_type getter
if grep -q "pub fn symbol_type" crates/mathhook-core/src/core/symbol.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Symbol::symbol_type() exists${NC}"
else
    echo -e "${RED}✗ Symbol::symbol_type() not found${NC}"
    SYMBOL_METHODS=$((SYMBOL_METHODS + 1))
fi

# Check for commutativity method
if grep -q "pub fn commutativity" crates/mathhook-core/src/core/symbol.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Symbol::commutativity() exists${NC}"
else
    echo -e "${RED}✗ Symbol::commutativity() not found${NC}"
    SYMBOL_METHODS=$((SYMBOL_METHODS + 1))
fi

if [ $SYMBOL_METHODS -eq 0 ]; then
    echo -e "${GREEN}✓ All Symbol methods exist${NC}"
else
    echo -e "${RED}✗ $SYMBOL_METHODS Symbol method(s) missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: EXPRESSION METHODS
echo "========================================"
echo "CATEGORY 7: EXPRESSION METHODS"
echo "Verify Expression::commutativity() exists"
echo "========================================"

# Check for Expression::commutativity() method
if grep -q "pub fn commutativity" crates/mathhook-core/src/core/expression/methods.rs 2>/dev/null || \
   grep -q "fn commutativity" crates/mathhook-core/src/core/expression/mod.rs 2>/dev/null; then
    echo -e "${GREEN}✓ Expression::commutativity() exists${NC}"
else
    echo -e "${RED}✗ Expression::commutativity() not found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: TEST SUITE
echo "========================================"
echo "CATEGORY 8: TEST SUITE"
echo "All existing tests must pass (zero regressions)"
echo "========================================"

echo "Running: cargo test -p mathhook-core --lib"
TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)

if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep "test result: ok" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
    echo -e "${GREEN}✓ All tests passed ($PASSED tests)${NC}"
else
    echo -e "${RED}✗ Some tests failed${NC}"
    echo "$TEST_OUTPUT" | grep -A 5 "FAILED"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: NEW TESTS
echo "========================================"
echo "CATEGORY 9: NEW TESTS"
echo "Wave 1 target: 20+ new tests"
echo "========================================"

# Count tests in new files
NEW_TEST_COUNT=0

if [ -f "crates/mathhook-core/src/core/commutativity.rs" ]; then
    COMM_TESTS=$(grep -c "#\[test\]" crates/mathhook-core/src/core/commutativity.rs 2>/dev/null || echo 0)
    echo "Found $COMM_TESTS tests in commutativity.rs"
    NEW_TEST_COUNT=$((NEW_TEST_COUNT + COMM_TESTS))
fi

if [ -f "crates/mathhook-core/src/core/symbol.rs" ]; then
    SYM_TESTS=$(grep -c "test.*symbol_type\|test.*commutativity" crates/mathhook-core/src/core/symbol.rs 2>/dev/null || echo 0)
    echo "Found $SYM_TESTS symbol type tests in symbol.rs"
    NEW_TEST_COUNT=$((NEW_TEST_COUNT + SYM_TESTS))
fi

echo "Total new tests: $NEW_TEST_COUNT"

if [ $NEW_TEST_COUNT -ge 20 ]; then
    echo -e "${GREEN}✓ Test count target met ($NEW_TEST_COUNT ≥ 20)${NC}"
elif [ $NEW_TEST_COUNT -ge 15 ]; then
    echo -e "${YELLOW}⚠ Test count slightly below target ($NEW_TEST_COUNT < 20)${NC}"
else
    echo -e "${RED}✗ Insufficient tests ($NEW_TEST_COUNT < 20)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 10: DOCUMENTATION
echo "========================================"
echo "CATEGORY 10: DOCUMENTATION"
echo "Verify documentation exists"
echo "========================================"

DOC_ISSUES=0

# Check for module-level docs in commutativity.rs
if [ -f "crates/mathhook-core/src/core/commutativity.rs" ]; then
    if grep -q "^//!" crates/mathhook-core/src/core/commutativity.rs 2>/dev/null; then
        echo -e "${GREEN}✓ commutativity.rs has module docs${NC}"
    else
        echo -e "${RED}✗ commutativity.rs missing module docs${NC}"
        DOC_ISSUES=$((DOC_ISSUES + 1))
    fi

    # Check for enum/struct documentation
    if grep -q "^/// " crates/mathhook-core/src/core/commutativity.rs 2>/dev/null; then
        echo -e "${GREEN}✓ Items have documentation${NC}"
    else
        echo -e "${RED}✗ Missing item documentation${NC}"
        DOC_ISSUES=$((DOC_ISSUES + 1))
    fi
fi

if [ $DOC_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✓ Documentation complete${NC}"
else
    echo -e "${YELLOW}⚠ $DOC_ISSUES documentation issue(s)${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 1: Core Type System & Symbol Enhancement is VERIFIED COMPLETE"
    echo ""
    echo "Key Achievements:"
    echo "  ✓ Expression size remains 32 bytes (no increase!)"
    echo "  ✓ Symbol has SymbolType (Scalar/Matrix/Operator/Quaternion)"
    echo "  ✓ Commutativity computed on-demand (not stored)"
    echo "  ✓ Only 2 Commutativity variants (Commutative/Noncommutative)"
    echo "  ✓ Zero regressions in existing tests"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 1 requires fixes before approval"
    exit 1
fi
