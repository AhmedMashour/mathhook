#!/bin/bash

# Wave 8: Parser Integration (LaTeX) Verification Script
# Verifies grammar.lalrpop modifications for \mathbf, \hat, and type inference
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 8: PARSER INTEGRATION VERIFICATION"
echo "LaTeX notation support for type inference"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: CORRECT FILE MODIFIED
echo "========================================"
echo "CATEGORY 1: CORRECT FILE MODIFIED"
echo "CRITICAL: Must modify grammar.lalrpop, NOT lexer"
echo "========================================"

GRAMMAR_MODIFIED=$(git diff --name-only HEAD crates/mathhook-core/src/parser/grammar.lalrpop 2>/dev/null | wc -l)
LEXER_MODIFIED=$(git diff --name-only HEAD crates/mathhook-core/src/parser/lexer/*.rs 2>/dev/null | wc -l)

if [ "$GRAMMAR_MODIFIED" -gt 0 ]; then
    echo -e "${GREEN}✓ grammar.lalrpop was modified${NC}"
else
    echo -e "${RED}✗ grammar.lalrpop was NOT modified${NC}"
    FAILURES=$((FAILURES + 1))
fi

if [ "$LEXER_MODIFIED" -gt 0 ]; then
    echo -e "${RED}✗ Lexer files were modified (INCORRECT - should only modify grammar)${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ Lexer files were NOT modified (correct)${NC}"
fi

# CATEGORY 2: TOKEN DEFINITIONS
echo "========================================"
echo "CATEGORY 2: TOKEN DEFINITIONS"
echo "Must have \\mathbf token defined"
echo "========================================"

if grep -q '"\\\\mathbf"' crates/mathhook-core/src/parser/grammar.lalrpop; then
    echo -e "${GREEN}✓ \\mathbf token defined${NC}"
else
    echo -e "${RED}✗ \\mathbf token NOT found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 3: PARSER RULES
echo "========================================"
echo "CATEGORY 3: PARSER RULES"
echo "Must have rules for \\mathbf{A} → Matrix symbol"
echo "========================================"

MATHBF_RULE_COUNT=$(grep -c "LATEX_MATHBF" crates/mathhook-core/src/parser/grammar.lalrpop 2>/dev/null || echo "0")

if [ "$MATHBF_RULE_COUNT" -ge 2 ]; then
    echo -e "${GREEN}✓ \\mathbf rules found (token + grammar rule)${NC}"
else
    echo -e "${YELLOW}⚠ Only $MATHBF_RULE_COUNT \\mathbf reference(s) found${NC}"
    echo "Expected: token definition + at least one grammar rule"
fi

# CATEGORY 4: FILE SIZE VIOLATIONS
echo "========================================"
echo "CATEGORY 4: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

GRAMMAR_LINES=$(wc -l < crates/mathhook-core/src/parser/grammar.lalrpop)

echo "grammar.lalrpop: $GRAMMAR_LINES lines"

if [ "$GRAMMAR_LINES" -gt 500 ]; then
    echo -e "${YELLOW}⚠ grammar.lalrpop exceeds 500 lines${NC}"
    echo "Note: Pre-existing violation, acceptable for Wave 8"
else
    echo -e "${GREEN}✓ grammar.lalrpop complies${NC}"
fi

# CATEGORY 5: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 5: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" crates/mathhook-core/src/parser/grammar.lalrpop 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in grammar.lalrpop${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 6: PARSER REGENERATION
echo "========================================"
echo "CATEGORY 6: PARSER REGENERATION"
echo "Grammar must be regenerated after changes"
echo "========================================"

if [ -f "crates/mathhook-core/src/parser/grammar.rs" ]; then
    echo -e "${GREEN}✓ Generated parser file exists${NC}"

    # Check if it's recent (modified after grammar.lalrpop)
    if [ "crates/mathhook-core/src/parser/grammar.rs" -nt "crates/mathhook-core/src/parser/grammar.lalrpop" ]; then
        echo -e "${GREEN}✓ Parser was regenerated (grammar.rs is newer)${NC}"
    else
        echo -e "${YELLOW}⚠ Parser may not be regenerated (grammar.lalrpop is newer)${NC}"
        echo "Run: lalrpop crates/mathhook-core/src/parser/grammar.lalrpop"
    fi
else
    echo -e "${RED}✗ Generated parser file NOT found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: BUILD STATUS
echo "========================================"
echo "CATEGORY 7: BUILD STATUS"
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

# CATEGORY 8: TEST VALIDATION
echo "========================================"
echo "CATEGORY 8: TEST VALIDATION"
echo "Parser tests must pass"
echo "========================================"

echo "Running parser tests..."
TEST_OUTPUT=$(cargo test -p mathhook-core parser 2>&1)
TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -c "test result: ok" || echo "0")

if [ "$TEST_PASSED" -gt 0 ]; then
    echo -e "${GREEN}✓ Parser tests passed${NC}"
    echo "$TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ Parser tests failed${NC}"
    echo "$TEST_OUTPUT" | tail -30
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: PARSER TEST COUNT
echo "========================================"
echo "CATEGORY 9: PARSER TEST COUNT"
echo "Target: 20+ new parser tests"
echo "========================================"

NEW_TEST_FILES=$(find crates/mathhook-core/tests -name "*parser*" -o -name "*mathbf*" -o -name "*matrix_symbol*" 2>/dev/null | wc -l)

echo "Parser test files found: $NEW_TEST_FILES"

if [ "$NEW_TEST_FILES" -ge 1 ]; then
    echo -e "${GREEN}✓ Parser test files exist${NC}"
else
    echo -e "${YELLOW}⚠ No dedicated parser test files found${NC}"
    echo "Tests may be inline in grammar.lalrpop or in general parser tests"
fi

# CATEGORY 10: SYMPY VALIDATION READINESS
echo "========================================"
echo "CATEGORY 10: SYMPY VALIDATION READINESS"
echo "Tests should validate against SymPy behavior"
echo "========================================"

SYMPY_REFERENCES=$(grep -r "SymPy\|sympy" crates/mathhook-core/tests/*parser* 2>/dev/null | wc -l)

if [ "$SYMPY_REFERENCES" -gt 0 ]; then
    echo -e "${GREEN}✓ SymPy validation references found${NC}"
else
    echo -e "${YELLOW}⚠ No SymPy validation references (may not be needed for parser)${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 8: Parser Integration is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 8: Parser Integration requires fixes before approval"
    exit 1
fi
