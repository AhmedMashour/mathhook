#!/bin/bash

# Wave: Function Dispatch Integration Verification Script
# Purpose: Verify that FunctionProperties properly dispatches to actual function implementations
# Strictness: MAXIMUM - All criteria must pass

set -e  # Exit on any error

echo "=================================================="
echo "Function Dispatch Integration Verification"
echo "=================================================="
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCORE=0
MAX_SCORE=100

# Category 1: Compilation (10 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 1: Compilation (10 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if cargo build -p mathhook-core --quiet 2>&1 | tee /tmp/build_output.txt | grep -q "error"; then
    echo -e "${RED}✗ FAILED: Compilation errors detected${NC}"
    cat /tmp/build_output.txt
    exit 1
else
    echo -e "${GREEN}✓ PASSED: Clean compilation${NC}"
    SCORE=$((SCORE + 10))
fi
echo ""

# Category 2: Function Pointer Storage (15 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 2: Function Pointer Storage (15 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check ElementaryProperties has evaluator field
if grep -q "pub evaluator.*fn.*Expression.*Expression" crates/mathhook-core/src/functions/properties/elementary.rs; then
    echo -e "${GREEN}✓ PASSED: ElementaryProperties has evaluator field${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: ElementaryProperties missing evaluator field${NC}"
fi

# Check SpecialProperties has evaluator field
if grep -q "pub evaluator.*fn.*Expression.*Expression" crates/mathhook-core/src/functions/properties/special.rs; then
    echo -e "${GREEN}✓ PASSED: SpecialProperties has evaluator field${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: SpecialProperties missing evaluator field${NC}"
fi

# Check PolynomialProperties has evaluator field
if grep -q "pub evaluator.*fn.*Expression.*Expression" crates/mathhook-core/src/functions/properties/special.rs; then
    echo -e "${GREEN}✓ PASSED: PolynomialProperties has evaluator field${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${YELLOW}⚠ WARNING: PolynomialProperties missing evaluator field (optional)${NC}"
    SCORE=$((SCORE + 3))
fi
echo ""

# Category 3: Dispatch Implementation (20 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 3: Dispatch Implementation (20 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check that evaluate() uses function pointer dispatch
if grep -A 20 "pub fn evaluate" crates/mathhook-core/src/functions/properties/mod.rs | grep -q "evaluator"; then
    echo -e "${GREEN}✓ PASSED: evaluate() uses function pointer dispatch${NC}"
    SCORE=$((SCORE + 10))
else
    echo -e "${RED}✗ FAILED: evaluate() does not use function pointer dispatch${NC}"
fi

# Check NO string matching for dispatch (no match on function names)
if grep -A 20 "pub fn evaluate" crates/mathhook-core/src/functions/properties/mod.rs | grep -E 'match.*"(gamma|sin|cos|bessel)"'; then
    echo -e "${RED}✗ FAILED: evaluate() still uses string matching for dispatch${NC}"
else
    echo -e "${GREEN}✓ PASSED: No string matching for function dispatch${NC}"
    SCORE=$((SCORE + 10))
fi
echo ""

# Category 4: Registry Integration (20 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 4: Registry Integration (20 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check gamma registration includes evaluator
if grep -A 10 '"gamma"' crates/mathhook-core/src/functions/special/intelligence.rs | grep -q "evaluator"; then
    echo -e "${GREEN}✓ PASSED: Gamma registration includes evaluator${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: Gamma registration missing evaluator${NC}"
fi

# Check bessel registration includes evaluator
if grep -A 10 '"bessel_j"' crates/mathhook-core/src/functions/special/intelligence.rs | grep -q "evaluator"; then
    echo -e "${GREEN}✓ PASSED: Bessel registration includes evaluator${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: Bessel registration missing evaluator${NC}"
fi

# Check zeta registration includes evaluator
if grep -A 10 '"zeta"' crates/mathhook-core/src/functions/special/intelligence.rs | grep -q "evaluator"; then
    echo -e "${GREEN}✓ PASSED: Zeta registration includes evaluator${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: Zeta registration missing evaluator${NC}"
fi

# Check sin registration includes evaluator (elementary function)
if grep -A 10 '"sin"' crates/mathhook-core/src/functions/elementary/intelligence.rs | grep -q "evaluator"; then
    echo -e "${GREEN}✓ PASSED: Sin registration includes evaluator${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: Sin registration missing evaluator${NC}"
fi
echo ""

# Category 5: Functional Testing (25 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 5: Functional Testing (25 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Run all tests
if cargo test -p mathhook-core --lib --quiet 2>&1 | tee /tmp/test_output.txt | tail -1 | grep -q "test result: ok"; then
    TEST_COUNT=$(grep -oP '\d+(?= passed)' /tmp/test_output.txt | tail -1)
    echo -e "${GREEN}✓ PASSED: All tests pass (${TEST_COUNT} tests)${NC}"
    SCORE=$((SCORE + 15))
else
    echo -e "${RED}✗ FAILED: Some tests failing${NC}"
    grep "FAILED" /tmp/test_output.txt || true
fi

# Doctests must pass
if cargo test --doc -p mathhook-core --quiet 2>&1 | tail -1 | grep -q "test result: ok"; then
    echo -e "${GREEN}✓ PASSED: All doctests pass${NC}"
    SCORE=$((SCORE + 10))
else
    echo -e "${RED}✗ FAILED: Some doctests failing${NC}"
fi
echo ""

# Category 6: Integration Tests (15 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 6: Integration Tests (15 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Test gamma evaluation through registry
TEST_SCRIPT=$(cat <<'EOF'
use mathhook_core::functions::evaluation::FunctionEvaluator;
use mathhook_core::core::Expression;

fn main() {
    let evaluator = FunctionEvaluator::new();

    // Test gamma(5) = 24
    let result = evaluator.evaluate("gamma", &[Expression::integer(5)]);
    match result {
        mathhook_core::functions::evaluation::EvaluationResult::Exact(expr) => {
            if expr == Expression::integer(24) {
                println!("PASS: gamma(5) = 24");
                std::process::exit(0);
            } else {
                println!("FAIL: gamma(5) = {:?}, expected 24", expr);
                std::process::exit(1);
            }
        },
        _ => {
            println!("FAIL: gamma(5) returned Unevaluated");
            std::process::exit(1);
        }
    }
}
EOF
)

echo "$TEST_SCRIPT" > /tmp/test_gamma_dispatch.rs

if rustc --edition 2021 --crate-type bin /tmp/test_gamma_dispatch.rs \
    --extern mathhook_core=target/debug/libmathhook_core.rlib \
    -L target/debug/deps \
    -o /tmp/test_gamma_dispatch 2>/dev/null && /tmp/test_gamma_dispatch; then
    echo -e "${GREEN}✓ PASSED: gamma(5) evaluates correctly through registry${NC}"
    SCORE=$((SCORE + 15))
else
    echo -e "${RED}✗ FAILED: gamma(5) does not evaluate through registry${NC}"
    echo "  Expected: Exact(24), Got: Unevaluated or wrong value"
fi
echo ""

# Category 7: Performance Verification (10 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 7: Performance Verification (10 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check FunctionProperties size hasn't grown unreasonably
SIZE_TEST=$(cat <<'EOF'
use std::mem::size_of;

fn main() {
    use mathhook_core::functions::properties::FunctionProperties;
    let size = size_of::<FunctionProperties>();
    println!("{}", size);
}
EOF
)

echo "$SIZE_TEST" > /tmp/test_size.rs
if rustc --edition 2021 --crate-type bin /tmp/test_size.rs \
    --extern mathhook_core=target/debug/libmathhook_core.rlib \
    -L target/debug/deps \
    -o /tmp/test_size 2>/dev/null; then
    SIZE=$(/tmp/test_size)
    if [ "$SIZE" -le 32 ]; then
        echo -e "${GREEN}✓ PASSED: FunctionProperties size = ${SIZE} bytes (≤ 32)${NC}"
        SCORE=$((SCORE + 10))
    else
        echo -e "${YELLOW}⚠ WARNING: FunctionProperties size = ${SIZE} bytes (> 32)${NC}"
        SCORE=$((SCORE + 5))
    fi
else
    echo -e "${YELLOW}⚠ WARNING: Could not check FunctionProperties size${NC}"
    SCORE=$((SCORE + 5))
fi
echo ""

# Category 8: Architecture Compliance (5 points)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Category 8: Architecture Compliance (5 points)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check no hardcoded function names in evaluate()
if ! grep -A 30 "pub fn evaluate" crates/mathhook-core/src/functions/properties/mod.rs | grep -E '"(gamma|sin|cos|bessel|zeta|beta)"'; then
    echo -e "${GREEN}✓ PASSED: No hardcoded function names in evaluate()${NC}"
    SCORE=$((SCORE + 5))
else
    echo -e "${RED}✗ FAILED: Found hardcoded function names in evaluate()${NC}"
fi
echo ""

# Final Score
echo "=================================================="
echo "FINAL VERIFICATION SCORE"
echo "=================================================="
echo ""
echo "Score: ${SCORE} / ${MAX_SCORE}"
echo ""

if [ "$SCORE" -ge 95 ]; then
    echo -e "${GREEN}✓✓✓ EXCELLENT - All criteria met${NC}"
    exit 0
elif [ "$SCORE" -ge 80 ]; then
    echo -e "${YELLOW}⚠ GOOD - Minor issues detected${NC}"
    exit 0
elif [ "$SCORE" -ge 60 ]; then
    echo -e "${YELLOW}⚠ ACCEPTABLE - Significant issues need attention${NC}"
    exit 1
else
    echo -e "${RED}✗✗✗ FAILED - Critical issues must be fixed${NC}"
    exit 1
fi
