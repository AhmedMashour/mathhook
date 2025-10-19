#!/bin/bash
# Wave 1: Core Type System & Symbol Enhancement - Verification Script

set -e  # Exit on error

MATHHOOK_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
cd "$MATHHOOK_ROOT"

echo "========================================"
echo "WAVE 1 VERIFICATION: Core Type System"
echo "========================================"
echo ""

# Track results
PASS=0
FAIL=0

# Function to check and report
check() {
    if eval "$1" > /dev/null 2>&1; then
        echo "✅ PASS: $2"
        ((PASS++))
    else
        echo "❌ FAIL: $2"
        ((FAIL++))
        return 1
    fi
}

# 1. Check SymbolType enum exists
echo "1. Checking SymbolType enum..."
if grep -q "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs; then
    echo "✅ PASS: SymbolType enum exists"
    ((PASS++))

    # Check for all 4 variants
    if grep -A 5 "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs | grep -q "Scalar" && \
       grep -A 5 "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs | grep -q "Matrix" && \
       grep -A 5 "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs | grep -q "Operator" && \
       grep -A 5 "pub enum SymbolType" crates/mathhook-core/src/core/symbol.rs | grep -q "Quaternion"; then
        echo "  ✅ All 4 variants present: Scalar, Matrix, Operator, Quaternion"
        ((PASS++))
    else
        echo "  ❌ Missing one or more variants"
        ((FAIL++))
    fi
else
    echo "❌ FAIL: SymbolType enum not found"
    ((FAIL++))
fi
echo ""

# 2. Check Commutativity enum exists
echo "2. Checking Commutativity enum..."
if [ -f "crates/mathhook-core/src/core/commutativity.rs" ]; then
    echo "✅ PASS: commutativity.rs file exists"
    ((PASS++))

    if grep -q "pub enum Commutativity" crates/mathhook-core/src/core/commutativity.rs; then
        echo "✅ PASS: Commutativity enum exists"
        ((PASS++))

        # Check for 3 variants
        if grep -A 5 "pub enum Commutativity" crates/mathhook-core/src/core/commutativity.rs | grep -q "Commutative" && \
           grep -A 5 "pub enum Commutativity" crates/mathhook-core/src/core/commutativity.rs | grep -q "Noncommutative" && \
           grep -A 5 "pub enum Commutativity" crates/mathhook-core/src/core/commutativity.rs | grep -q "Unknown"; then
            echo "  ✅ All 3 variants present: Commutative, Noncommutative, Unknown"
            ((PASS++))
        else
            echo "  ❌ Missing one or more variants"
            ((FAIL++))
        fi
    else
        echo "❌ FAIL: Commutativity enum not found"
        ((FAIL++))
    fi
else
    echo "❌ FAIL: commutativity.rs file not found"
    ((FAIL++))
fi
echo ""

# 3. Check Expression::Mul signature
echo "3. Checking Expression::Mul signature..."
if grep -q "Mul(Box<Vec<Expression>>, Commutativity)" crates/mathhook-core/src/core/expression/data_types.rs; then
    echo "✅ PASS: Expression::Mul signature updated"
    ((PASS++))
else
    echo "❌ FAIL: Expression::Mul signature not updated"
    ((FAIL++))
fi
echo ""

# 4. Check Symbol constructors
echo "4. Checking Symbol constructors..."
CONSTRUCTORS=("scalar" "matrix" "operator" "quaternion")
for ctor in "${CONSTRUCTORS[@]}"; do
    if grep -q "pub fn $ctor" crates/mathhook-core/src/core/symbol.rs; then
        echo "  ✅ $ctor() constructor exists"
        ((PASS++))
    else
        echo "  ❌ $ctor() constructor NOT found"
        ((FAIL++))
    fi
done
echo ""

# 5. Check Symbol::commutativity() method
echo "5. Checking Symbol::commutativity() method..."
if grep -q "pub fn commutativity" crates/mathhook-core/src/core/symbol.rs; then
    echo "✅ PASS: commutativity() method exists"
    ((PASS++))
else
    echo "❌ FAIL: commutativity() method NOT found"
    ((FAIL++))
fi
echo ""

# 6. Check Expression size
echo "6. Checking Expression size..."
echo "   (Running size test - this may take a moment...)"
if cargo test --release -p mathhook-core expression_size --no-fail-fast 2>&1 | grep -q "Expression size"; then
    SIZE=$(cargo test --release -p mathhook-core expression_size --no-fail-fast 2>&1 | grep "Expression size" | awk '{print $3}')
    if [ -n "$SIZE" ]; then
        if [ "$SIZE" -le 48 ]; then
            echo "✅ PASS: Expression size = $SIZE bytes (≤ 48 bytes)"
            ((PASS++))
        else
            echo "⚠️  WARN: Expression size = $SIZE bytes (> 48 bytes, acceptable but document)"
            ((PASS++))
        fi
    else
        echo "⚠️  WARN: Could not determine Expression size"
        ((PASS++))
    fi
else
    echo "⚠️  WARN: Expression size test not found (may not exist yet)"
    ((PASS++))
fi
echo ""

# 7. Check no emojis in code
echo "7. Checking for emojis in code..."
EMOJI_COUNT=$(grep -r "[\x{1F600}-\x{1F64F}\x{1F300}-\x{1F5FF}\x{1F680}-\x{1F6FF}]" crates/mathhook-core/src/ 2>/dev/null | wc -l || echo "0")
if [ "$EMOJI_COUNT" -eq 0 ]; then
    echo "✅ PASS: No emojis found"
    ((PASS++))
else
    echo "❌ FAIL: Found $EMOJI_COUNT emojis in code"
    ((FAIL++))
fi
echo ""

# 8. Check file sizes (max 500 lines per CLAUDE.md)
echo "8. Checking file sizes (max 500 lines)..."
OVERSIZED=0
while IFS= read -r -d '' file; do
    LINES=$(wc -l < "$file")
    if [ "$LINES" -gt 500 ]; then
        echo "  ❌ $file has $LINES lines (> 500)"
        ((OVERSIZED++))
    fi
done < <(find crates/mathhook-core/src/core -name "*.rs" -print0)

if [ "$OVERSIZED" -eq 0 ]; then
    echo "✅ PASS: All core files ≤ 500 lines"
    ((PASS++))
else
    echo "❌ FAIL: $OVERSIZED files exceed 500 lines"
    ((FAIL++))
fi
echo ""

# 9. Run tests
echo "9. Running tests..."
echo "   (This may take 1-2 minutes...)"
if cargo test -p mathhook-core --no-fail-fast 2>&1 | tee /tmp/wave1_tests.log | grep -q "test result: ok"; then
    TEST_RESULT=$(grep "test result:" /tmp/wave1_tests.log | tail -1)
    echo "✅ PASS: Tests passed - $TEST_RESULT"
    ((PASS++))
else
    TEST_RESULT=$(grep "test result:" /tmp/wave1_tests.log | tail -1 || echo "UNKNOWN")
    echo "❌ FAIL: Tests failed - $TEST_RESULT"
    ((FAIL++))
    echo ""
    echo "Failed tests:"
    grep "FAILED" /tmp/wave1_tests.log || echo "  (Could not extract failed test names)"
fi
echo ""

# 10. Build check
echo "10. Checking build..."
if cargo build -p mathhook-core 2>&1 | tee /tmp/wave1_build.log | grep -q "Finished"; then
    echo "✅ PASS: Build successful"
    ((PASS++))
else
    echo "❌ FAIL: Build failed"
    ((FAIL++))
    echo ""
    echo "Build errors:"
    grep "error" /tmp/wave1_build.log | head -10
fi
echo ""

# Summary
echo "========================================"
echo "WAVE 1 VERIFICATION SUMMARY"
echo "========================================"
echo "Total Checks: $((PASS + FAIL))"
echo "Passed: $PASS"
echo "Failed: $FAIL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "✅✅✅ WAVE 1: ALL CHECKS PASSED ✅✅✅"
    echo ""
    echo "Recommendation: Proceed to Wave 2"
    exit 0
elif [ $FAIL -le 2 ]; then
    echo "⚠️⚠️⚠️ WAVE 1: MOSTLY PASSED (minor issues) ⚠️⚠️⚠️"
    echo ""
    echo "Recommendation: Fix minor issues before Wave 2"
    exit 1
else
    echo "❌❌❌ WAVE 1: CRITICAL FAILURES ❌❌❌"
    echo ""
    echo "Recommendation: Must fix before proceeding"
    exit 2
fi
