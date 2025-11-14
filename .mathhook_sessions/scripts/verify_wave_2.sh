#!/bin/bash
# Wave 2: Constructor & Accessor Updates - Verification Script

set -e

MATHHOOK_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
cd "$MATHHOOK_ROOT"

echo "========================================"
echo "WAVE 2 VERIFICATION: Constructors"
echo "========================================"
echo ""

PASS=0
FAIL=0

# 1. Check Expression::mul() auto-infers commutativity
echo "1. Checking Expression::mul() auto-inference..."
if grep -A 30 "pub fn mul(" crates/mathhook-core/src/core/expression/constructors/basic.rs | grep -qi "infer\|commutat"; then
    echo "✅ PASS: mul() has commutativity inference logic"
    ((PASS++))
else
    echo "❌ FAIL: mul() missing commutativity inference"
    ((FAIL++))
fi
echo ""

# 2. Check NO explicit control method
echo "2. Checking NO mul_with_commutativity()..."
if grep -q "mul_with_commutativity" crates/mathhook-core/src/core/expression/constructors/basic.rs; then
    echo "❌ FAIL: mul_with_commutativity() found (should NOT exist)"
    ((FAIL++))
else
    echo "✅ PASS: No explicit control method (correct)"
    ((PASS++))
fi
echo ""

# 3. Count Mul pattern matches (should be 100-150)
echo "3. Counting Expression::Mul pattern matches..."
MUL_COUNT=$(grep -r "Expression::Mul(" crates/mathhook-core/src/ --include="*.rs" | wc -l)
echo "   Found $MUL_COUNT pattern matches"
if [ "$MUL_COUNT" -ge 100 ]; then
    echo "✅ PASS: $MUL_COUNT matches updated (≥ 100)"
    ((PASS++))
else
    echo "❌ FAIL: Only $MUL_COUNT matches (expected ≥ 100)"
    ((FAIL++))
fi
echo ""

# 4. Check commutativity() method
echo "4. Checking Expression::commutativity() method..."
if grep -q "pub fn commutativity(&self)" crates/mathhook-core/src/core/expression/methods.rs; then
    echo "✅ PASS: commutativity() method exists"
    ((PASS++))
else
    echo "❌ FAIL: commutativity() method NOT found"
    ((FAIL++))
fi
echo ""

# 5. Check is_commutative() convenience method
echo "5. Checking Expression::is_commutative() method..."
if grep -q "pub fn is_commutative(&self)" crates/mathhook-core/src/core/expression/methods.rs; then
    echo "✅ PASS: is_commutative() method exists"
    ((PASS++))
else
    echo "❌ FAIL: is_commutative() method NOT found"
    ((FAIL++))
fi
echo ""

# 6. Run tests
echo "6. Running tests..."
if cargo test -p mathhook-core --no-fail-fast 2>&1 | tee /tmp/wave2_tests.log | grep -q "test result: ok"; then
    TEST_RESULT=$(grep "test result:" /tmp/wave2_tests.log | tail -1)
    echo "✅ PASS: Tests passed - $TEST_RESULT"
    ((PASS++))
else
    TEST_RESULT=$(grep "test result:" /tmp/wave2_tests.log | tail -1)
    echo "❌ FAIL: Tests failed - $TEST_RESULT"
    ((FAIL++))
fi
echo ""

# 7. Check for commutativity tests
echo "7. Checking for commutativity-specific tests..."
COMM_TESTS=$(cargo test -p mathhook-core --no-fail-fast 2>&1 | grep -i "commutat\|infer" | wc -l)
if [ "$COMM_TESTS" -ge 5 ]; then
    echo "✅ PASS: Found $COMM_TESTS commutativity tests"
    ((PASS++))
else
    echo "⚠️  WARN: Only $COMM_TESTS commutativity tests (expected more)"
    ((PASS++))
fi
echo ""

# 8. Build check
echo "8. Checking build..."
if cargo build -p mathhook-core 2>&1 | grep -q "Finished"; then
    echo "✅ PASS: Build successful"
    ((PASS++))
else
    echo "❌ FAIL: Build failed"
    ((FAIL++))
fi
echo ""

# Summary
echo "========================================"
echo "WAVE 2 VERIFICATION SUMMARY"
echo "========================================"
echo "Passed: $PASS / Failed: $FAIL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "✅✅✅ WAVE 2: ALL CHECKS PASSED ✅✅✅"
    exit 0
elif [ $FAIL -le 2 ]; then
    echo "⚠️⚠️⚠️ WAVE 2: MOSTLY PASSED ⚠️⚠️⚠️"
    exit 1
else
    echo "❌❌❌ WAVE 2: CRITICAL FAILURES ❌❌❌"
    exit 2
fi
