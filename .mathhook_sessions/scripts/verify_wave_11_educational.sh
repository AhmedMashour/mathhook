#!/bin/bash

# Wave 11: Educational, Message Registry & Formatter Verification Script
# Verifies noncommutative algebra educational features
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 11: EDUCATIONAL FEATURES VERIFICATION"
echo "Message registry & LaTeX formatter for noncommutative algebra"
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

# Find all potentially modified educational files
EDUCATIONAL_FILES=$(find crates/mathhook-core/src/educational -name "*.rs" 2>/dev/null)
FORMATTER_FILES=$(find crates/mathhook-core/src/formatter -name "*.rs" 2>/dev/null)

echo "Checking educational files..."
for file in $EDUCATIONAL_FILES; do
    LINES=$(wc -l < "$file")
    if [ "$LINES" -gt 500 ]; then
        echo -e "${RED}✗ $file exceeds 500 lines ($LINES)${NC}"
        FAILURES=$((FAILURES + 1))
    fi
done

echo "Checking formatter files..."
for file in $FORMATTER_FILES; do
    LINES=$(wc -l < "$file")
    if [ "$LINES" -gt 500 ]; then
        echo -e "${RED}✗ $file exceeds 500 lines ($LINES)${NC}"
        FAILURES=$((FAILURES + 1))
    fi
done

echo -e "${GREEN}✓ File size check complete${NC}"

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/educational/ crates/mathhook-core/src/formatter/ 2>/dev/null | wc -l)

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

# CATEGORY 4: MESSAGE REGISTRY UPDATES
echo "========================================"
echo "CATEGORY 4: MESSAGE REGISTRY UPDATES"
echo "Must have left/right division messages"
echo "========================================"

LEFT_MSG=$(grep -r "left.*multiply\|multiply.*left" crates/mathhook-core/src/educational/ 2>/dev/null | wc -l)
RIGHT_MSG=$(grep -r "right.*multiply\|multiply.*right" crates/mathhook-core/src/educational/ 2>/dev/null | wc -l)

echo "Left division message references: $LEFT_MSG"
echo "Right division message references: $RIGHT_MSG"

if [ "$LEFT_MSG" -ge 1 ] && [ "$RIGHT_MSG" -ge 1 ]; then
    echo -e "${GREEN}✓ Message registry has left/right division messages${NC}"
else
    echo -e "${YELLOW}⚠ Message registry may need left/right division messages${NC}"
fi

# CATEGORY 5: LATEX FORMATTER UPDATES
echo "========================================"
echo "CATEGORY 5: LATEX FORMATTER UPDATES"
echo "Must have type-aware symbol formatting"
echo "========================================"

MATHBF_COUNT=$(grep -r "mathbf" crates/mathhook-core/src/formatter/ 2>/dev/null | wc -l)
HAT_COUNT=$(grep -r "hat" crates/mathhook-core/src/formatter/ 2>/dev/null | wc -l)

echo "\\mathbf (matrix) references: $MATHBF_COUNT"
echo "\\hat (operator) references: $HAT_COUNT"

if [ "$MATHBF_COUNT" -ge 1 ] && [ "$HAT_COUNT" -ge 1 ]; then
    echo -e "${GREEN}✓ LaTeX formatter has type-aware formatting${NC}"
else
    echo -e "${YELLOW}⚠ LaTeX formatter may need type-aware formatting${NC}"
fi

# CATEGORY 6: STEP-BY-STEP INTEGRATION
echo "========================================"
echo "CATEGORY 6: STEP-BY-STEP INTEGRATION"
echo "Must use educational messages in explanations"
echo "========================================"

STEP_MSG_INTEGRATION=$(grep -r "message\|Message\|registry\|Registry" crates/mathhook-core/src/educational/step_by_step/ 2>/dev/null | wc -l)

echo "Message/registry integration references: $STEP_MSG_INTEGRATION"

if [ "$STEP_MSG_INTEGRATION" -ge 1 ]; then
    echo -e "${GREEN}✓ Step-by-step uses message registry${NC}"
else
    echo -e "${YELLOW}⚠ Step-by-step may need message registry integration${NC}"
fi

# CATEGORY 7: TEST COUNT
echo "========================================"
echo "CATEGORY 7: TEST COUNT"
echo "Target: 25+ tests for educational features"
echo "========================================"

TEST_COUNT=$(grep -c "fn test_" crates/mathhook-core/tests/educational_noncommutative_tests.rs 2>/dev/null || echo "0")

echo "Educational noncommutative tests found: $TEST_COUNT"

if [ "$TEST_COUNT" -ge 25 ]; then
    echo -e "${GREEN}✓ Test count meets target (25+)${NC}"
elif [ "$TEST_COUNT" -ge 20 ]; then
    echo -e "${YELLOW}⚠ Test count close to target: $TEST_COUNT (target 25+)${NC}"
else
    echo -e "${RED}✗ Test count below target: $TEST_COUNT (target 25+)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: TEST VALIDATION
echo "========================================"
echo "CATEGORY 8: TEST VALIDATION"
echo "Educational tests must pass"
echo "========================================"

echo "Running educational noncommutative tests..."
TEST_OUTPUT=$(cargo test --test educational_noncommutative_tests 2>&1)
TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -c "test result: ok" || echo "0")

if [ "$TEST_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Educational tests passed${NC}"
    echo "$TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Educational tests failed or not found${NC}"
    echo "$TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION QUALITY"
echo "New educational features must have documentation"
echo "========================================"

EDUCATIONAL_DOCS=$(grep -c "/// " crates/mathhook-core/src/educational/ -r 2>/dev/null || echo "0")
FORMATTER_DOCS=$(grep -c "/// " crates/mathhook-core/src/formatter/ -r 2>/dev/null || echo "0")

echo "Educational documentation lines: $EDUCATIONAL_DOCS"
echo "Formatter documentation lines: $FORMATTER_DOCS"

TOTAL_DOCS=$((EDUCATIONAL_DOCS + FORMATTER_DOCS))

if [ "$TOTAL_DOCS" -ge 50 ]; then
    echo -e "${GREEN}✓ Documentation quality is good${NC}"
else
    echo -e "${YELLOW}⚠ Could use more documentation${NC}"
fi

# CATEGORY 10: ZERO REGRESSIONS
echo "========================================"
echo "CATEGORY 10: ZERO REGRESSIONS"
echo "All existing educational tests must pass"
echo "========================================"

echo "Running all educational tests..."
EDUCATIONAL_TEST_OUTPUT=$(cargo test -p mathhook-core educational 2>&1)
EDUCATIONAL_PASSED=$(echo "$EDUCATIONAL_TEST_OUTPUT" | grep "test result: ok" | wc -l)

if [ "$EDUCATIONAL_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Educational tests passed (zero regressions)${NC}"
    echo "$EDUCATIONAL_TEST_OUTPUT" | grep "test result:" | head -10
else
    echo -e "${RED}✗ Educational tests failed (regressions detected)${NC}"
    echo "$EDUCATIONAL_TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 11: Educational Features Integration is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 11: Educational Features requires fixes before approval"
    exit 1
fi
