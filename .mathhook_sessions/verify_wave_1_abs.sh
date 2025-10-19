#!/bin/bash

# Wave 1: Absolute Value Function - Verification Script
# Target: 10/10 quality (PERFECT execution on small scope)

echo "=================================="
echo "Wave 1: Absolute Value Function"
echo "Verification Script - 10 Categories"
echo "=================================="
echo ""

PASS=0
FAIL=0

# Category 1: File Existence and Structure
echo "1. FILE EXISTENCE AND STRUCTURE"
echo "--------------------------------"
if [ -f "crates/mathhook-core/src/functions/elementary/abs.rs" ]; then
    LINE_COUNT=$(wc -l < "crates/mathhook-core/src/functions/elementary/abs.rs")
    echo "✓ abs.rs exists ($LINE_COUNT lines)"
    if [ "$LINE_COUNT" -le 250 ]; then
        echo "✓ abs.rs within target size (≤250 lines)"
        PASS=$((PASS + 1))
    else
        echo "✗ abs.rs exceeds target size ($LINE_COUNT > 250)"
        FAIL=$((FAIL + 1))
    fi
else
    echo "✗ abs.rs does not exist"
    FAIL=$((FAIL + 1))
fi

if [ -f "crates/mathhook-core/tests/abs_tests.rs" ]; then
    echo "✓ abs_tests.rs exists"
    PASS=$((PASS + 1))
else
    echo "✗ abs_tests.rs does not exist"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 2: CLAUDE.md Compliance - No Emojis
echo "2. CLAUDE.MD COMPLIANCE - NO EMOJIS"
echo "------------------------------------"
if grep -r "[\x{1F600}-\x{1F64F}\x{1F300}-\x{1F5FF}\x{1F680}-\x{1F6FF}]" crates/mathhook-core/src/functions/elementary/abs.rs 2>/dev/null; then
    echo "✗ Found emojis in abs.rs"
    FAIL=$((FAIL + 1))
else
    echo "✓ No emojis in abs.rs"
    PASS=$((PASS + 1))
fi

if [ -f "crates/mathhook-core/tests/abs_tests.rs" ]; then
    if grep -r "[\x{1F600}-\x{1F64F}\x{1F300}-\x{1F5FF}\x{1F680}-\x{1F6FF}]" crates/mathhook-core/tests/abs_tests.rs 2>/dev/null; then
        echo "✗ Found emojis in abs_tests.rs"
        FAIL=$((FAIL + 1))
    else
        echo "✓ No emojis in abs_tests.rs"
        PASS=$((PASS + 1))
    fi
fi
echo ""

# Category 3: Build Success
echo "3. BUILD SUCCESS"
echo "----------------"
if cargo check -p mathhook-core 2>&1 | tee /tmp/abs_build.log | grep -q "error"; then
    echo "✗ Build has errors"
    FAIL=$((FAIL + 1))
else
    echo "✓ Build successful (cargo check)"
    PASS=$((PASS + 1))
fi
echo ""

# Category 4: Clippy Warnings
echo "4. CLIPPY WARNINGS"
echo "------------------"
if cargo clippy -p mathhook-core 2>&1 | grep -q "warning:"; then
    echo "✗ Clippy warnings found"
    FAIL=$((FAIL + 1))
else
    echo "✓ No clippy warnings"
    PASS=$((PASS + 1))
fi
echo ""

# Category 5: Test Count and Execution
echo "5. TEST COUNT AND EXECUTION"
echo "---------------------------"
ABS_TEST_COUNT=$(cargo test -p mathhook-core abs_tests 2>&1 | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | head -1)
if [ -z "$ABS_TEST_COUNT" ]; then
    ABS_TEST_COUNT=0
fi

echo "Absolute value tests found: $ABS_TEST_COUNT"
if [ "$ABS_TEST_COUNT" -ge 10 ]; then
    echo "✓ Sufficient test count (≥10)"
    PASS=$((PASS + 1))
else
    echo "✗ Insufficient tests ($ABS_TEST_COUNT < 10)"
    FAIL=$((FAIL + 1))
fi

if cargo test -p mathhook-core abs_tests 2>&1 | grep -q "test result: ok"; then
    echo "✓ All abs tests pass"
    PASS=$((PASS + 1))
else
    echo "✗ Some abs tests fail"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 6: Zero Regressions
echo "6. ZERO REGRESSIONS"
echo "-------------------"
TOTAL_TESTS=$(cargo test -p mathhook-core 2>&1 | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | head -1)
if [ -z "$TOTAL_TESTS" ]; then
    TOTAL_TESTS=0
fi

echo "Total tests passing: $TOTAL_TESTS"
if [ "$TOTAL_TESTS" -ge 514 ]; then
    echo "✓ No regressions (≥514 tests passing)"
    PASS=$((PASS + 1))
else
    echo "✗ Regressions detected ($TOTAL_TESTS < 514)"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 7: Documentation Quality
echo "7. DOCUMENTATION QUALITY"
echo "------------------------"
if grep -q "///" crates/mathhook-core/src/functions/elementary/abs.rs; then
    echo "✓ Documentation comments present"
    PASS=$((PASS + 1))
else
    echo "✗ Missing documentation comments"
    FAIL=$((FAIL + 1))
fi

if grep -q "# Examples" crates/mathhook-core/src/functions/elementary/abs.rs; then
    echo "✓ Examples section present"
    PASS=$((PASS + 1))
else
    echo "✗ Missing examples section"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 8: Doctests
echo "8. DOCTESTS"
echo "-----------"
if cargo test -p mathhook-core --doc 2>&1 | grep -q "test result: ok"; then
    echo "✓ All doctests pass"
    PASS=$((PASS + 1))
else
    echo "✗ Some doctests fail"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 9: API Integration
echo "9. API INTEGRATION"
echo "------------------"
if grep -q "pub fn abs" crates/mathhook-core/src/core/expression.rs 2>/dev/null; then
    echo "✓ .abs() method added to Expression API"
    PASS=$((PASS + 1))
else
    echo "✗ .abs() method not found in Expression API"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 10: Registry Integration
echo "10. REGISTRY INTEGRATION"
echo "------------------------"
if grep -q "abs" crates/mathhook-core/src/functions/elementary/mod.rs 2>/dev/null; then
    echo "✓ abs module registered in elementary/mod.rs"
    PASS=$((PASS + 1))
else
    echo "✗ abs module not registered"
    FAIL=$((FAIL + 1))
fi
echo ""

# Summary
echo "=================================="
echo "VERIFICATION SUMMARY"
echo "=================================="
echo "Categories Passed: $PASS"
echo "Categories Failed: $FAIL"
echo "Total Categories: $((PASS + FAIL))"
echo ""

if [ "$FAIL" -eq 0 ]; then
    echo "✓ WAVE 1 VERIFICATION: PERFECT (10/10)"
    echo "Status: READY FOR WAVE 2"
    exit 0
else
    QUALITY_SCORE=$(awk "BEGIN {printf \"%.1f\", ($PASS * 10.0) / ($PASS + $FAIL)}")
    echo "✗ WAVE 1 VERIFICATION: $QUALITY_SCORE/10"
    echo "Status: REQUIRES FIXES"
    exit 1
fi
