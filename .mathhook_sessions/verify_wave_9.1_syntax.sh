#!/bin/bash

# Wave 9.1: Enhanced symbols!() Syntax Verification Script
# Verifies new comma+arrow syntax and hybrid type specification
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 9.1: ENHANCED symbols!() SYNTAX VERIFICATION"
echo "Comma-separated identifiers with hybrid types"
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

SYMBOLS_RS_LINES=$(wc -l < crates/mathhook-core/src/macros/symbols.rs)
echo "macros/symbols.rs: $SYMBOLS_RS_LINES lines"

if [ "$SYMBOLS_RS_LINES" -gt 500 ]; then
    echo -e "${RED}✗ symbols.rs exceeds 500 lines${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ symbols.rs complies${NC}"
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/macros/symbols.rs 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 3: NEW SYNTAX VERIFICATION
echo "========================================"
echo "CATEGORY 3: NEW SYNTAX VERIFICATION"
echo "Must support comma-separated identifiers"
echo "========================================"

# Check for new syntax patterns in macro
COMMA_SYNTAX=$(grep -c '\$name:ident),+' crates/mathhook-core/src/macros/symbols.rs 2>/dev/null || echo "0")
ARROW_SYNTAX=$(grep -c '=> \$type:ident' crates/mathhook-core/src/macros/symbols.rs 2>/dev/null || echo "0")

echo "Syntax patterns found:"
echo "  - Comma-separated: $COMMA_SYNTAX references"
echo "  - Arrow (=>): $ARROW_SYNTAX references"

if [ "$COMMA_SYNTAX" -ge 1 ] && [ "$ARROW_SYNTAX" -ge 1 ]; then
    echo -e "${GREEN}✓ New syntax patterns present${NC}"
else
    echo -e "${RED}✗ New syntax patterns missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: BUILD STATUS
echo "========================================"
echo "CATEGORY 4: BUILD STATUS"
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

# CATEGORY 5: TEST VALIDATION
echo "========================================"
echo "CATEGORY 5: TEST VALIDATION"
echo "symbols!() macro tests must pass"
echo "========================================"

echo "Running macro enhancement tests..."
TEST_OUTPUT=$(cargo test --test macro_enhancement_tests 2>&1)
TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -c "test result: ok" || echo "0")

if [ "$TEST_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Macro tests passed${NC}"
    echo "$TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Macro tests failed${NC}"
    echo "$TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: TEST COUNT
echo "========================================"
echo "CATEGORY 6: TEST COUNT"
echo "Target: 35+ tests (25 existing + 10 new)"
echo "========================================"

TEST_COUNT=$(grep -c "fn test_" crates/mathhook-core/tests/macro_enhancement_tests.rs 2>/dev/null || echo "0")

echo "Test functions found: $TEST_COUNT"

if [ "$TEST_COUNT" -ge 35 ]; then
    echo -e "${GREEN}✓ Test count meets target (35+)${NC}"
elif [ "$TEST_COUNT" -ge 25 ]; then
    echo -e "${YELLOW}⚠ Test count: $TEST_COUNT (target 35+)${NC}"
else
    echo -e "${RED}✗ Test count below minimum: $TEST_COUNT${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: CLAUDE.MD UPDATED
echo "========================================"
echo "CATEGORY 7: CLAUDE.MD UPDATED"
echo "Must show new syntax examples"
echo "========================================"

NEW_SYNTAX_EXAMPLES=$(grep -c 'symbols!\[' CLAUDE.md 2>/dev/null || echo "0")

echo "CLAUDE.md new syntax references: $NEW_SYNTAX_EXAMPLES"

if [ "$NEW_SYNTAX_EXAMPLES" -ge 5 ]; then
    echo -e "${GREEN}✓ CLAUDE.md updated with new syntax${NC}"
else
    echo -e "${RED}✗ CLAUDE.md needs updating${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 8: DOCUMENTATION QUALITY"
echo "Must have examples for new syntax"
echo "========================================"

DOCTEST_COUNT=$(grep -c "/// \`\`\`rust" crates/mathhook-core/src/macros/symbols.rs 2>/dev/null || echo "0")

echo "Doctests: $DOCTEST_COUNT"

if [ "$DOCTEST_COUNT" -ge 3 ]; then
    echo -e "${GREEN}✓ Documentation quality is good${NC}"
else
    echo -e "${YELLOW}⚠ Could use more documentation${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 9.1: Enhanced symbols!() Syntax is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 9.1 requires fixes before approval"
    exit 1
fi
