#!/bin/bash

# Wave 1: Number Theory Bug Fix & Verification Script
# Purpose: Verify LCM fix, MOD/is_prime status, test coverage, SymPy validation
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 1: NUMBER THEORY VERIFICATION"
echo "Fix LCM Bug & Verify Number Theory Functions"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: LCM BUG FIX VERIFICATION
echo ""
echo "========================================"
echo "CATEGORY 1: LCM BUG FIX"
echo "CRITICAL: LCM must not return a*b"
echo "========================================"

# Check if LCM implementation is fixed in gcd.rs
LCM_IMPL=$(grep -A 10 "fn lcm" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null | grep -c "Expression::div")

if [ "$LCM_IMPL" -gt 0 ]; then
    echo -e "${GREEN}✓ LCM implementation uses division (likely fixed)${NC}"
else
    echo -e "${RED}✗ LCM implementation may still return product only${NC}"
    FAILURES=$((FAILURES + 1))
fi

# Check that old broken pattern is gone
BAD_LCM=$(grep -A 5 "fn lcm" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null | grep -c "^[[:space:]]*product[[:space:]]*$")

if [ "$BAD_LCM" -eq 0 ]; then
    echo -e "${GREEN}✓ Old broken 'return product' pattern not found${NC}"
else
    echo -e "${RED}✗ Still has 'return product' without division${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: MOD OPERATION STATUS
echo ""
echo "========================================"
echo "CATEGORY 2: MOD OPERATION VERIFICATION"
echo "Check if MOD implementation exists"
echo "========================================"

# Search for modulo/mod implementation
MOD_IMPL=$(grep -r "fn modulo\|fn mod\|pub fn.*mod" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/ 2>/dev/null | wc -l)

if [ "$MOD_IMPL" -gt 0 ]; then
    echo -e "${GREEN}✓ MOD implementation found: $MOD_IMPL locations${NC}"
    echo "Status: IMPLEMENTED"
else
    echo -e "${YELLOW}⚠ MOD implementation not found in Expression methods${NC}"
    echo "Status: NOT IMPLEMENTED (acceptable if documented)"
fi

# CATEGORY 3: IS_PRIME STATUS
echo ""
echo "========================================"
echo "CATEGORY 3: IS_PRIME VERIFICATION"
echo "Check if primality testing exists"
echo "========================================"

# Search for is_prime implementation
PRIME_IMPL=$(grep -r "fn is_prime\|fn primality" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/ 2>/dev/null | wc -l)

if [ "$PRIME_IMPL" -gt 0 ]; then
    echo -e "${GREEN}✓ is_prime implementation found: $PRIME_IMPL locations${NC}"
    echo "Status: IMPLEMENTED"
else
    echo -e "${YELLOW}⚠ is_prime implementation not found${NC}"
    echo "Status: NOT IMPLEMENTED (acceptable if documented)"
fi

# CATEGORY 4: TEST COVERAGE
echo ""
echo "========================================"
echo "CATEGORY 4: NUMBER THEORY TEST COVERAGE"
echo "Target: 15+ new tests with SymPy validation"
echo "========================================"

# Count tests in number theory test files
GCD_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*gcd*.rs 2>/dev/null || echo "0")
LCM_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*lcm*.rs 2>/dev/null || echo "0")
NUMBER_THEORY_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*number_theory*.rs 2>/dev/null || echo "0")

# Also check embedded tests in gcd.rs
EMBEDDED_GCD_TESTS=$(grep -c "#\[test\]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null || echo "0")

TOTAL_NT_TESTS=$((GCD_TESTS + LCM_TESTS + NUMBER_THEORY_TESTS + EMBEDDED_GCD_TESTS))

echo "GCD tests: $GCD_TESTS"
echo "LCM tests: $LCM_TESTS"
echo "Number theory tests: $NUMBER_THEORY_TESTS"
echo "Embedded GCD tests: $EMBEDDED_GCD_TESTS"
echo "Total: $TOTAL_NT_TESTS"

if [ "$TOTAL_NT_TESTS" -ge 15 ]; then
    echo -e "${GREEN}✓ Test coverage adequate ($TOTAL_NT_TESTS >= 15)${NC}"
else
    echo -e "${YELLOW}⚠ Test coverage below target ($TOTAL_NT_TESTS < 15)${NC}"
    echo "Note: Acceptable if new tests added meet 15+ target"
fi

# CATEGORY 5: SYMPY VALIDATION MENTIONS
echo ""
echo "========================================"
echo "CATEGORY 5: SYMPY VALIDATION"
echo "Check for SymPy comparison/validation"
echo "========================================"

# Check for SymPy references in tests
SYMPY_MENTIONS=$(grep -r "sympy\|SymPy" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/ 2>/dev/null | wc -l)
SYMPY_COMMENTS=$(grep -r "// SymPy:\|# Validated against SymPy" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/ 2>/dev/null | wc -l)

if [ "$SYMPY_MENTIONS" -gt 0 ] || [ "$SYMPY_COMMENTS" -gt 0 ]; then
    echo -e "${GREEN}✓ SymPy validation references found: $((SYMPY_MENTIONS + SYMPY_COMMENTS)) locations${NC}"
else
    echo -e "${YELLOW}⚠ No explicit SymPy validation references found${NC}"
    echo "Note: Tests should reference SymPy validation in comments"
fi

# CATEGORY 6: FILE SIZE COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 6: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0

# Check gcd.rs (likely modified)
GCD_SIZE=$(wc -l < /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null || echo "0")
if [ "$GCD_SIZE" -gt 500 ]; then
    echo -e "${RED}✗ gcd.rs: $GCD_SIZE lines (exceeds 500)${NC}"
    VIOLATIONS=$((VIOLATIONS + 1))
else
    echo -e "${GREEN}✓ gcd.rs: $GCD_SIZE lines${NC}"
fi

# Check any new number theory files
if [ -f "/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/number_theory.rs" ]; then
    NT_SIZE=$(wc -l < /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/number_theory.rs)
    if [ "$NT_SIZE" -gt 500 ]; then
        echo -e "${RED}✗ number_theory.rs: $NT_SIZE lines (exceeds 500)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    else
        echo -e "${GREEN}✓ number_theory.rs: $NT_SIZE lines${NC}"
    fi
fi

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply with 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed 500-line limit${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: EMOJI COMPLIANCE
echo ""
echo "========================================"
echo "CATEGORY 7: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/ 2>/dev/null | wc -l)
TEST_EMOJI=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*number_theory*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*gcd*.rs /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/*lcm*.rs 2>/dev/null | wc -l)

TOTAL_EMOJI=$((EMOJI_COUNT + TEST_EMOJI))

if [ "$TOTAL_EMOJI" -gt 0 ]; then
    echo -e "${RED}✗ Found $TOTAL_EMOJI emojis in number theory code/tests${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 8: BUILD STATUS
echo ""
echo "========================================"
echo "CATEGORY 8: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    echo "$BUILD_OUTPUT" | grep "error\|Error" | head -5
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: NUMBER THEORY TESTS PASS
echo ""
echo "========================================"
echo "CATEGORY 9: NUMBER THEORY TEST SUITE"
echo "All number theory tests must pass"
echo "========================================"

# Run GCD tests
GCD_TEST_OUTPUT=$(cargo test -p mathhook-core gcd 2>&1)
GCD_PASS=$(echo "$GCD_TEST_OUTPUT" | grep -c "test result: ok")

if [ "$GCD_PASS" -gt 0 ]; then
    echo -e "${GREEN}✓ GCD tests passing${NC}"
    echo "$GCD_TEST_OUTPUT" | grep "test result:"
else
    echo -e "${RED}✗ GCD tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# Run LCM tests if they exist
if cargo test -p mathhook-core lcm --list 2>&1 | grep -q "test"; then
    LCM_TEST_OUTPUT=$(cargo test -p mathhook-core lcm 2>&1)
    LCM_PASS=$(echo "$LCM_TEST_OUTPUT" | grep -c "test result: ok")

    if [ "$LCM_PASS" -gt 0 ]; then
        echo -e "${GREEN}✓ LCM tests passing${NC}"
        echo "$LCM_TEST_OUTPUT" | grep "test result:"
    else
        echo -e "${RED}✗ LCM tests failing${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${YELLOW}⚠ No LCM-specific tests found${NC}"
fi

# CATEGORY 10: DOCUMENTATION STATUS
echo ""
echo "========================================"
echo "CATEGORY 10: DOCUMENTATION STATUS"
echo "Status report must document findings"
echo "========================================"

# Check if verification report exists
if [ -f "/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/WAVE_1_NUMBER_THEORY_STATUS.md" ]; then
    echo -e "${GREEN}✓ Number theory status report exists${NC}"
    REPORT_SIZE=$(wc -l < /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/WAVE_1_NUMBER_THEORY_STATUS.md)
    echo "Report size: $REPORT_SIZE lines"

    # Check report has key sections
    if grep -q "LCM" /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/WAVE_1_NUMBER_THEORY_STATUS.md && \
       grep -q "MOD\|is_prime" /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/WAVE_1_NUMBER_THEORY_STATUS.md; then
        echo -e "${GREEN}✓ Report documents LCM, MOD, is_prime status${NC}"
    else
        echo -e "${YELLOW}⚠ Report may be incomplete${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Status report not yet created${NC}"
    echo "Note: Agent should create WAVE_1_NUMBER_THEORY_STATUS.md"
fi

# SUMMARY
echo ""
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 1: Number Theory Bug Fix & Verification is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 1 requires fixes before approval"
    exit 1
fi
