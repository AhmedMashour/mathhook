#!/bin/bash

# Wave 1-INT: ODE Integration Verification Script
# Enforces CLAUDE.md compliance and architectural correctness
# Verifies ODE module integration with SmartEquationSolver

echo "========================================"
echo "WAVE 1-INT VERIFICATION"
echo "ODE Integration with SmartEquationSolver"
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
done << EOF
crates/mathhook-core/src/algebra/equation_analyzer.rs
crates/mathhook-core/src/ode/educational/wrapper.rs
crates/mathhook-core/tests/test_ode_integration.rs
EOF

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
    "crates/mathhook-core/src/ode/educational/wrapper.rs" \
    "crates/mathhook-core/tests/test_ode_integration.rs"; do
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

# CATEGORY 4: INTEGRATION TEST VALIDATION
echo "========================================"
echo "CATEGORY 4: INTEGRATION TEST VALIDATION"
echo "All 7 integration tests must pass"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --test test_ode_integration 2>&1)

if echo "$TEST_OUTPUT" | grep -q "7 passed"; then
    echo -e "${GREEN}✓ All 7 integration tests passed${NC}"
else
    echo -e "${RED}✗ Integration tests failed:${NC}"
    echo "$TEST_OUTPUT" | grep -A 5 "test result"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: NO REGRESSIONS
echo "========================================"
echo "CATEGORY 5: NO REGRESSIONS"
echo "Existing tests must still pass (901 baseline)"
echo "========================================"

FULL_TEST_OUTPUT=$(cargo test -p mathhook-core 2>&1)
# macOS grep doesn't support -P, use -E instead
PASSED_COUNT=$(echo "$FULL_TEST_OUTPUT" | grep -E '[0-9]+ passed' | tail -1 | grep -Eo '[0-9]+' | head -1)

if [ -n "$PASSED_COUNT" ] && [ "$PASSED_COUNT" -ge 901 ]; then
    echo -e "${GREEN}✓ No regressions: $PASSED_COUNT tests passing (≥901)${NC}"
else
    echo -e "${RED}✗ Regression detected: Only $PASSED_COUNT tests passing (expected ≥901)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: ARCHITECTURAL PATTERN COMPLIANCE
echo "========================================"
echo "CATEGORY 6: ARCHITECTURAL PATTERN"
echo "Verify registry-based pattern (not hardcoded)"
echo "========================================"

# Check for hardcoded ODE matching (anti-pattern)
HARDCODED_MATCHES=0
for file in "crates/mathhook-core/src/algebra/equation_analyzer.rs"; do
    if [ -f "$file" ]; then
        # Look for hardcoded function name matches in analyze() method
        # This is BAD: match func_name.as_str() { "derivative" => ...
        if grep -A 20 "pub fn analyze" "$file" | grep -q 'match.*as_str.*"derivative"'; then
            echo -e "${RED}  Found hardcoded derivative matching in $file${NC}"
            HARDCODED_MATCHES=$((HARDCODED_MATCHES + 1))
        fi
    fi
done

# Check for helper methods (good pattern)
HELPER_METHODS=0
if grep -q "fn has_derivatives" "crates/mathhook-core/src/algebra/equation_analyzer.rs"; then
    HELPER_METHODS=$((HELPER_METHODS + 1))
fi
if grep -q "fn has_partial_derivatives" "crates/mathhook-core/src/algebra/equation_analyzer.rs"; then
    HELPER_METHODS=$((HELPER_METHODS + 1))
fi

if [ "$HARDCODED_MATCHES" -eq 0 ] && [ "$HELPER_METHODS" -eq 2 ]; then
    echo -e "${GREEN}✓ Uses helper methods (registry pattern), no hardcoded matching${NC}"
else
    echo -e "${RED}✗ Architectural pattern violation${NC}"
    echo "  Hardcoded matches: $HARDCODED_MATCHES (should be 0)"
    echo "  Helper methods: $HELPER_METHODS (should be 2)"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: NO STUB IMPLEMENTATIONS IN CRITICAL PATH
echo "========================================"
echo "CATEGORY 7: NO STUB IMPLEMENTATIONS"
echo "SmartEquationSolver must have actual ODE solver field"
echo "========================================"

STUB_VIOLATIONS=0

# Check SmartEquationSolver has ode_solver field
if ! grep -q "ode_solver: EducationalODESolver" "crates/mathhook-core/src/algebra/equation_analyzer.rs"; then
    echo -e "${RED}  SmartEquationSolver missing ode_solver field${NC}"
    STUB_VIOLATIONS=$((STUB_VIOLATIONS + 1))
fi

# Check ODE routing in solve_with_equation
# Need to check across multiple lines: EquationType::ODE => self\n.ode_solver
if ! grep -A 2 "EquationType::ODE" "crates/mathhook-core/src/algebra/equation_analyzer.rs" | grep -q "ode_solver"; then
    echo -e "${RED}  Missing ODE routing in solve_with_equation${NC}"
    STUB_VIOLATIONS=$((STUB_VIOLATIONS + 1))
fi

# Check EducationalODESolver implements EquationSolver trait
if ! grep -q "impl.*EquationSolver.*for EducationalODESolver" "crates/mathhook-core/src/ode/educational/wrapper.rs"; then
    echo -e "${RED}  EducationalODESolver doesn't implement EquationSolver trait${NC}"
    STUB_VIOLATIONS=$((STUB_VIOLATIONS + 1))
fi

if [ "$STUB_VIOLATIONS" -eq 0 ]; then
    echo -e "${GREEN}✓ No stub implementations in critical path${NC}"
else
    echo -e "${RED}✗ $STUB_VIOLATIONS stub implementation(s) found${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: EQUATIONTYPE ENUM EXTENSION
echo "========================================"
echo "CATEGORY 8: EQUATIONTYPE ENUM"
echo "Must include ODE and PDE variants"
echo "========================================"

ENUM_VIOLATIONS=0

if ! grep -A 15 "pub enum EquationType" "crates/mathhook-core/src/algebra/equation_analyzer.rs" | grep -q "ODE"; then
    echo -e "${RED}  EquationType enum missing ODE variant${NC}"
    ENUM_VIOLATIONS=$((ENUM_VIOLATIONS + 1))
fi

if ! grep -A 15 "pub enum EquationType" "crates/mathhook-core/src/algebra/equation_analyzer.rs" | grep -q "PDE"; then
    echo -e "${RED}  EquationType enum missing PDE variant${NC}"
    ENUM_VIOLATIONS=$((ENUM_VIOLATIONS + 1))
fi

if [ "$ENUM_VIOLATIONS" -eq 0 ]; then
    echo -e "${GREEN}✓ EquationType enum correctly extended${NC}"
else
    echo -e "${RED}✗ $ENUM_VIOLATIONS missing enum variant(s)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: DOCUMENTATION COMPLIANCE
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION COMPLIANCE"
echo "CLAUDE.md: No ALL CAPS, proper comment style"
echo "========================================"

DOC_VIOLATIONS=0

# Check for ALL CAPS in comments (except constants)
for file in \
    "crates/mathhook-core/src/algebra/equation_analyzer.rs" \
    "crates/mathhook-core/src/ode/educational/wrapper.rs" \
    "crates/mathhook-core/tests/test_ode_integration.rs"; do
    if [ -f "$file" ]; then
        # Look for comments with 3+ consecutive capital words (likely ALL CAPS violations)
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
    # Don't fail for this, just warn
fi

# CATEGORY 10: TEST ARCHITECTURAL CORRECTNESS
echo "========================================"
echo "CATEGORY 10: TEST ARCHITECTURAL CORRECTNESS"
echo "Tests must verify architecture, not just implementation"
echo "========================================"

ARCH_TEST_VIOLATIONS=0

# Check for architectural tests
if ! grep -q "test_architectural_pattern" "crates/mathhook-core/tests/test_ode_integration.rs"; then
    echo -e "${RED}  Missing architectural pattern test${NC}"
    ARCH_TEST_VIOLATIONS=$((ARCH_TEST_VIOLATIONS + 1))
fi

if ! grep -q "test_no_stub_implementations" "crates/mathhook-core/tests/test_ode_integration.rs"; then
    echo -e "${RED}  Missing stub implementation test${NC}"
    ARCH_TEST_VIOLATIONS=$((ARCH_TEST_VIOLATIONS + 1))
fi

if ! grep -q "test_smart_solver_ode_routing" "crates/mathhook-core/tests/test_ode_integration.rs"; then
    echo -e "${RED}  Missing solver routing test${NC}"
    ARCH_TEST_VIOLATIONS=$((ARCH_TEST_VIOLATIONS + 1))
fi

if [ "$ARCH_TEST_VIOLATIONS" -eq 0 ]; then
    echo -e "${GREEN}✓ Architectural tests present${NC}"
else
    echo -e "${RED}✗ $ARCH_TEST_VIOLATIONS missing architectural test(s)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 1-INT: ODE Integration is VERIFIED COMPLETE"
    echo ""
    echo "Architectural Integration Summary:"
    echo "  - EquationType enum extended with ODE/PDE"
    echo "  - Derivative detection methods implemented"
    echo "  - SmartEquationSolver integrated with ODE solver"
    echo "  - EquationSolver trait implemented for EducationalODESolver"
    echo "  - 7 integration tests passing"
    echo "  - 0 regressions detected"
    echo "  - Registry-based pattern enforced"
    echo ""
    echo "Ready for Wave 5-INT: PDE Integration"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 1-INT requires fixes before approval"
    echo ""
    echo "Please address the failing categories above."
    exit 1
fi
