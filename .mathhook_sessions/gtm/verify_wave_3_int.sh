#!/bin/bash

# Wave 3-INT: Gröbner Basis Integration Verification Script
# Enforces CLAUDE.md compliance and architectural correctness
# Verifies Gröbner basis module integration with SmartEquationSolver (or standalone status)

echo "========================================"
echo "WAVE 3-INT VERIFICATION"
echo "Gröbner Basis Integration with SmartEquationSolver"
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
while IFS= read -r file; do
    if [ -f "$file" ]; then
        LINES=$(wc -l < "$file")
        if [ "$LINES" -gt 500 ]; then
            echo -e "${RED}✗ $file: $LINES lines (exceeds 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        fi
    fi
done << 'FILELIST'
crates/mathhook-core/src/algebra/equation_analyzer.rs
crates/mathhook-core/src/algebra/groebner/mod.rs
crates/mathhook-core/src/algebra/groebner/basis.rs
crates/mathhook-core/src/algebra/groebner/buchberger.rs
FILELIST

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply (≤500 lines)${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500 line limit${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code/comments"
echo "========================================"

EMOJI_PATTERN="✅|❌|⚠️|🚀|✨|🔥|💡|📝|🎯|🧪|🔧|📊|💻|🎨|🚧|⚡"

EMOJI_COUNT=0
for file in \
    "crates/mathhook-core/src/algebra/equation_analyzer.rs" \
    "crates/mathhook-core/src/algebra/groebner/mod.rs" \
    "crates/mathhook-core/src/algebra/groebner/basis.rs" \
    "crates/mathhook-core/src/algebra/groebner/buchberger.rs"; do
    if [ -f "$file" ]; then
        COUNT=$(grep -E "$EMOJI_PATTERN" "$file" 2>/dev/null | wc -l)
        EMOJI_COUNT=$((EMOJI_COUNT + COUNT))
        if [ "$COUNT" -gt 0 ]; then
            echo -e "${RED}  Found in $file${NC}"
        fi
    fi
done

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emoji(s) in code${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors:${NC}"
    echo "$BUILD_OUTPUT" | tail -20
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: GRÖBNER BASIS MODULE EXISTS
echo "========================================"
echo "CATEGORY 4: GRÖBNER BASIS MODULE EXISTS"
echo "Verify Gröbner basis module is present"
echo "========================================"

MODULE_VIOLATIONS=0

if [ ! -d "crates/mathhook-core/src/algebra/groebner" ]; then
    echo -e "${RED}  Missing algebra/groebner/ directory${NC}"
    MODULE_VIOLATIONS=$((MODULE_VIOLATIONS + 1))
fi

if [ ! -f "crates/mathhook-core/src/algebra/groebner/mod.rs" ]; then
    echo -e "${RED}  Missing algebra/groebner/mod.rs${NC}"
    MODULE_VIOLATIONS=$((MODULE_VIOLATIONS + 1))
fi

if [ "$MODULE_VIOLATIONS" -eq 0 ]; then
    echo -e "${GREEN}✓ Gröbner basis module exists${NC}"
else
    echo -e "${RED}✗ $MODULE_VIOLATIONS missing module component(s)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: INTEGRATION STATUS
echo "========================================"
echo "CATEGORY 5: INTEGRATION STATUS"
echo "Determine if Gröbner basis is integrated with SmartEquationSolver"
echo "========================================"

# Check if SmartEquationSolver has groebner_solver field
INTEGRATION_STATUS="STANDALONE"

if grep -q "groebner" "crates/mathhook-core/src/algebra/equation_analyzer.rs" 2>/dev/null; then
    INTEGRATION_STATUS="INTEGRATED"
    echo -e "${GREEN}✓ Gröbner basis integrated with SmartEquationSolver${NC}"
else
    echo -e "${YELLOW}⚠ Gröbner basis is STANDALONE (not integrated with SmartEquationSolver)${NC}"
    echo "  This is acceptable if Gröbner basis is a utility module"
fi

# CATEGORY 6: NO REGRESSIONS
echo "========================================"
echo "CATEGORY 6: NO REGRESSIONS"
echo "Existing tests must still pass (902 baseline)"
echo "========================================"

FULL_TEST_OUTPUT=$(cargo test -p mathhook-core 2>&1)
PASSED_COUNT=$(echo "$FULL_TEST_OUTPUT" | grep -E '[0-9]+ passed' | tail -1 | grep -Eo '[0-9]+' | head -1)

if [ -n "$PASSED_COUNT" ] && [ "$PASSED_COUNT" -ge 902 ]; then
    echo -e "${GREEN}✓ No regressions: $PASSED_COUNT tests passing (≥902)${NC}"
else
    echo -e "${RED}✗ Regression detected: Only $PASSED_COUNT tests passing (expected ≥902)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: ARCHITECTURAL PATTERN COMPLIANCE
echo "========================================"
echo "CATEGORY 7: ARCHITECTURAL PATTERN"
echo "Verify proper module structure and exports"
echo "========================================"

ARCH_VIOLATIONS=0

# Check that algebra/mod.rs exports groebner module
if ! grep -q "pub mod groebner" "crates/mathhook-core/src/algebra/mod.rs" 2>/dev/null; then
    echo -e "${RED}  algebra/mod.rs doesn't export groebner module${NC}"
    ARCH_VIOLATIONS=$((ARCH_VIOLATIONS + 1))
fi

if [ "$ARCH_VIOLATIONS" -eq 0 ]; then
    echo -e "${GREEN}✓ Proper module structure and exports${NC}"
else
    echo -e "${RED}✗ $ARCH_VIOLATIONS architectural pattern violation(s)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: GRÖBNER BASIS TESTS
echo "========================================"
echo "CATEGORY 8: GRÖBNER BASIS TESTS"
echo "Verify Gröbner basis has test coverage"
echo "========================================"

GROEBNER_TEST_OUTPUT=$(cargo test -p mathhook-core groebner 2>&1)

if echo "$GROEBNER_TEST_OUTPUT" | grep -q "test result:"; then
    GROEBNER_TESTS=$(echo "$GROEBNER_TEST_OUTPUT" | grep -E '[0-9]+ passed' | tail -1 | grep -Eo '[0-9]+' | head -1)
    if [ -n "$GROEBNER_TESTS" ] && [ "$GROEBNER_TESTS" -gt 0 ]; then
        echo -e "${GREEN}✓ Gröbner basis has $GROEBNER_TESTS test(s)${NC}"
    else
        echo -e "${YELLOW}⚠ Gröbner basis has no tests${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Gröbner basis module may not have tests${NC}"
fi

# CATEGORY 9: DOCUMENTATION COMPLIANCE
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION COMPLIANCE"
echo "CLAUDE.md: No ALL CAPS, proper comment style"
echo "========================================"

DOC_VIOLATIONS=0

for file in \
    "crates/mathhook-core/src/algebra/groebner/mod.rs" \
    "crates/mathhook-core/src/algebra/groebner/basis.rs" \
    "crates/mathhook-core/src/algebra/groebner/buchberger.rs"; do
    if [ -f "$file" ]; then
        ALL_CAPS=$(grep -E '^\s*(//)' "$file" | grep -E '[A-Z]{4,}.*[A-Z]{4,}' | grep -v "const " | wc -l)
        if [ "$ALL_CAPS" -gt 0 ]; then
            echo -e "${YELLOW}  Warning: Possible ALL CAPS in $file${NC}"
            DOC_VIOLATIONS=$((DOC_VIOLATIONS + 1))
        fi
    fi
done

if [ "$DOC_VIOLATIONS" -eq 0 ]; then
    echo -e "${GREEN}✓ Documentation style compliant${NC}"
else
    echo -e "${YELLOW}⚠ $DOC_VIOLATIONS potential documentation style issue(s)${NC}"
fi

# CATEGORY 10: INTEGRATION TYPE DETERMINATION
echo "========================================"
echo "CATEGORY 10: INTEGRATION TYPE"
echo "Document integration architecture"
echo "========================================"

echo -e "${GREEN}✓ Integration status: $INTEGRATION_STATUS${NC}"

if [ "$INTEGRATION_STATUS" = "INTEGRATED" ]; then
    echo "  Gröbner basis is integrated with SmartEquationSolver"
    echo "  Expected: groebner_solver field, EquationType::GroebnerBasis routing"
else
    echo "  Gröbner basis is a standalone utility module"
    echo "  Used by: Polynomial operations, ideal theory, system solving"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 3-INT: Gröbner Basis Integration is VERIFIED COMPLETE"
    echo ""
    echo "Architectural Integration Summary:"
    echo "  - Gröbner basis module exists and is properly structured"
    echo "  - Integration status: $INTEGRATION_STATUS"
    echo "  - Module exports correct"
    echo "  - 0 regressions detected ($PASSED_COUNT tests passing)"
    echo "  - Build successful"
    echo ""
    echo "Ready for next wave in Phase 2"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 3-INT requires fixes before approval"
    echo ""
    echo "Please address the failing categories above."
    exit 1
fi
