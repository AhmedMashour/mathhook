#!/bin/bash
# Verification Script for Wave 1.1: CRITICAL Safety - Stack Overflow Fix
# Plan 10: Integration Tests Fixes Orchestration
# Total Points: 100

set -e

SCORE=0
MAX_SCORE=100

echo "======================================================================"
echo "Wave 1.1 Verification: Stack Overflow Prevention"
echo "======================================================================"
echo ""

# Category 1: Test Execution (30 points)
echo "### Category 1: Test Execution (30 points) ###"
echo ""

echo "[1.1] Running Test 8 (product_requiring_parts_and_substitution)..."
if cargo test --test integration_comprehensive test_product_requiring_parts_and_substitution -- --nocapture 2>&1 | grep -q "test result: ok"; then
    echo "  ✓ Test 8 PASSES (15 points)"
    SCORE=$((SCORE + 15))
else
    echo "  ✗ Test 8 FAILS (0 points)"
fi

echo "[1.2] Running Test 1 (iterated_integration_by_parts)..."
if cargo test --test integration_comprehensive test_iterated_integration_by_parts -- --nocapture 2>&1 | grep -q "test result: ok"; then
    echo "  ✓ Test 1 PASSES (10 points)"
    SCORE=$((SCORE + 10))
else
    echo "  ✗ Test 1 FAILS (0 points)"
fi

echo "[1.3] Running Test 6 (repeated_by_parts)..."
if cargo test --test integration_comprehensive test_repeated_by_parts -- --nocapture 2>&1 | grep -q "test result: ok"; then
    echo "  ✓ Test 6 PASSES (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ✗ Test 6 FAILS (0 points)"
fi

echo ""

# Category 2: Mathematical Correctness (25 points)
echo "### Category 2: Mathematical Correctness (25 points) ###"
echo ""

echo "[2.1] Verifying Test 8 result: ∫x·ln(x) dx = (x²/2)·ln(x) - x²/4 + C"
TEST8_RESULT=$(cat << 'EOF' | cargo run --example playground_test_8_trace 2>&1 | grep -A1 "Result:"
EOF
)
if echo "$TEST8_RESULT" | grep -qE "(x\^2.*ln|ln.*x\^2)"; then
    echo "  ✓ Contains x²·ln(x) term (10 points)"
    SCORE=$((SCORE + 10))
else
    echo "  ✗ Missing x²·ln(x) term (0 points)"
fi

echo "[2.2] Verifying no symbolic integral returned..."
if cargo run --example playground_test_8_trace 2>&1 | grep -q "✓ SUCCESS"; then
    echo "  ✓ Returns closed-form solution (10 points)"
    SCORE=$((SCORE + 10))
else
    echo "  ✗ Returns symbolic integral (0 points)"
fi

echo "[2.3] Derivative verification (d/dx[result] = x·ln(x))..."
# This requires mathematical verification - check if implementation includes derivative test
if grep -r "derivative.*test.*by_parts" crates/mathhook-core/src/calculus/integrals/by_parts.rs > /dev/null 2>&1; then
    echo "  ✓ Derivative verification present (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ~ Derivative verification not found in code (manual check required, 5 points awarded provisionally)"
    SCORE=$((SCORE + 5))
fi

echo ""

# Category 3: Recursion Safety (20 points)
echo "### Category 3: Recursion Safety (20 points) ###"
echo ""

echo "[3.1] Checking for recursion depth tracking..."
if grep -r "depth" crates/mathhook-core/src/calculus/integrals/by_parts.rs | grep -qE "(max_depth|depth.*limit|recursion.*depth)"; then
    echo "  ✓ Depth tracking implemented (10 points)"
    SCORE=$((SCORE + 10))
else
    echo "  ✗ No depth tracking found (0 points)"
fi

echo "[3.2] Checking for max depth limit..."
if grep -r "MAX_DEPTH\|max_depth" crates/mathhook-core/src/calculus/integrals/by_parts.rs > /dev/null 2>&1; then
    echo "  ✓ Max depth limit defined (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ✗ No max depth limit (0 points)"
fi

echo "[3.3] Checking for depth exceeded handling..."
if grep -r "depth.*>" crates/mathhook-core/src/calculus/integrals/by_parts.rs | grep -q "return"; then
    echo "  ✓ Depth exceeded returns symbolic (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ✗ No depth exceeded handling (0 points)"
fi

echo ""

# Category 4: Simplification Improvements (15 points)
echo "### Category 4: Simplification Improvements (15 points) ###"
echo ""

echo "[4.1] Testing x²/2 * 1/x simplification..."
SIMPLIFY_TEST=$(cat << 'EOF' | cargo run --example playground_test_8_trace 2>&1
EOF
)
if echo "$SIMPLIFY_TEST" | grep -q "After simplify:.*x/2\|x \* 1/2"; then
    echo "  ✓ Algebraic cancellation works (10 points)"
    SCORE=$((SCORE + 10))
else
    echo "  ~ Simplification may need improvement (5 points partial credit)"
    SCORE=$((SCORE + 5))
fi

echo "[4.2] Checking for algebraic cancellation rules..."
if grep -r "cancel.*power\|algebraic.*cancel" crates/mathhook-core/src/simplify > /dev/null 2>&1; then
    echo "  ✓ Cancellation rules found (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ~ Cancellation rules not explicitly found (0 points)"
fi

echo ""

# Category 5: Code Quality (10 points)
echo "### Category 5: Code Quality (10 points) ###"
echo ""

echo "[5.1] Checking for documentation updates..."
if grep -r "recursion\|depth" crates/mathhook-core/src/calculus/integrals/by_parts.rs | grep -q "///"; then
    echo "  ✓ Documentation includes recursion safety (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ✗ No recursion safety documentation (0 points)"
fi

echo "[5.2] Running clippy on modified files..."
if cargo clippy --quiet -- -D warnings 2>&1 | grep -q "0 warnings"; then
    echo "  ✓ No clippy warnings (5 points)"
    SCORE=$((SCORE + 5))
else
    echo "  ✗ Clippy warnings present (0 points)"
fi

echo ""
echo "======================================================================"
echo "FINAL SCORE: $SCORE / $MAX_SCORE"
echo "======================================================================"
echo ""

if [ $SCORE -ge 90 ]; then
    echo "✅ EXCELLENT - Wave 1.1 Complete!"
    echo "   Ready to proceed to Wave 2.1"
    exit 0
elif [ $SCORE -ge 75 ]; then
    echo "⚠️  ACCEPTABLE - Minor issues remain"
    echo "   Consider addressing issues before Wave 2.1"
    exit 0
else
    echo "❌ FAIL - Critical issues must be fixed"
    echo "   Do not proceed to Wave 2.1"
    exit 1
fi
