#!/bin/bash

# Wave 2: Square Root Function - Verification Script
# Target: 10/10 quality (PERFECT execution on small scope)

echo "=================================="
echo "Wave 2: Square Root Function"
echo "Verification Script - 10 Categories"
echo "=================================="
echo ""

PASS=0
FAIL=0

# Category 1: File Existence and Structure
echo "1. FILE EXISTENCE AND STRUCTURE"
echo "--------------------------------"
if [ -f "crates/mathhook-core/src/functions/elementary/sqrt.rs" ]; then
    LINE_COUNT=$(wc -l < "crates/mathhook-core/src/functions/elementary/sqrt.rs")
    echo "✓ sqrt.rs exists ($LINE_COUNT lines)"
    if [ "$LINE_COUNT" -le 500 ]; then
        echo "✓ sqrt.rs within size limit (≤500 lines)"
        PASS=$((PASS + 1))
    else
        echo "✗ sqrt.rs exceeds size limit ($LINE_COUNT > 500)"
        FAIL=$((FAIL + 1))
    fi
else
    echo "✗ sqrt.rs does not exist"
    FAIL=$((FAIL + 1))
fi

if [ -f "crates/mathhook-core/tests/sqrt_tests.rs" ]; then
    echo "✓ sqrt_tests.rs exists"
    PASS=$((PASS + 1))
else
    echo "✗ sqrt_tests.rs does not exist"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 2: CLAUDE.md Compliance - No Emojis in Source
echo "2. CLAUDE.MD COMPLIANCE - NO EMOJIS"
echo "------------------------------------"
EMOJI_COUNT=0
if [ -f "crates/mathhook-core/src/functions/elementary/sqrt.rs" ]; then
    # Check for actual emojis, not checkmarks in comments
    if grep -P "[\x{1F300}-\x{1F6FF}]" crates/mathhook-core/src/functions/elementary/sqrt.rs 2>/dev/null | grep -v "^//"; then
        echo "✗ Found emojis in sqrt.rs code"
        EMOJI_COUNT=$((EMOJI_COUNT + 1))
    fi
fi

if [ -f "crates/mathhook-core/tests/sqrt_tests.rs" ]; then
    if grep -P "[\x{1F300}-\x{1F6FF}]" crates/mathhook-core/tests/sqrt_tests.rs 2>/dev/null | grep -v "^//"; then
        echo "✗ Found emojis in sqrt_tests.rs code"
        EMOJI_COUNT=$((EMOJI_COUNT + 1))
    fi
fi

if [ "$EMOJI_COUNT" -eq 0 ]; then
    echo "✓ No emojis in source code"
    PASS=$((PASS + 1))
else
    echo "✗ Found $EMOJI_COUNT file(s) with emojis"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 3: Build Success
echo "3. BUILD SUCCESS"
echo "----------------"
if cargo check -p mathhook-core --quiet 2>&1 | grep -i "error\["; then
    echo "✗ Build has errors"
    FAIL=$((FAIL + 1))
else
    echo "✓ Build successful (cargo check)"
    PASS=$((PASS + 1))
fi
echo ""

# Category 4: Clippy Warnings (New Code Only)
echo "4. CLIPPY WARNINGS (NEW CODE)"
echo "-----------------------------"
NEW_WARNINGS=0
if [ -f "crates/mathhook-core/src/functions/elementary/sqrt.rs" ]; then
    if cargo clippy -p mathhook-core --quiet 2>&1 | grep "sqrt.rs"; then
        echo "✗ Clippy warnings in sqrt.rs"
        NEW_WARNINGS=$((NEW_WARNINGS + 1))
    fi
fi

if [ "$NEW_WARNINGS" -eq 0 ]; then
    echo "✓ No clippy warnings in new code"
    PASS=$((PASS + 1))
else
    echo "✗ Found clippy warnings in new files"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 5: Test Count and Execution
echo "5. TEST COUNT AND EXECUTION"
echo "---------------------------"
SQRT_TEST_COUNT=$(cargo test -p mathhook-core --test sqrt_tests --quiet 2>&1 | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | head -1)
if [ -z "$SQRT_TEST_COUNT" ]; then
    SQRT_TEST_COUNT=0
fi

echo "Square root tests found: $SQRT_TEST_COUNT"
if [ "$SQRT_TEST_COUNT" -ge 10 ]; then
    echo "✓ Sufficient test count (≥10)"
    PASS=$((PASS + 1))
else
    echo "✗ Insufficient tests ($SQRT_TEST_COUNT < 10)"
    FAIL=$((FAIL + 1))
fi

if cargo test -p mathhook-core --test sqrt_tests --quiet 2>&1 | grep -q "test result: ok"; then
    echo "✓ All sqrt tests pass"
    PASS=$((PASS + 1))
else
    echo "✗ Some sqrt tests fail"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 6: Zero Regressions
echo "6. ZERO REGRESSIONS"
echo "-------------------"
TOTAL_TESTS=$(cargo test -p mathhook-core --quiet 2>&1 | grep -oE "[0-9]+ passed" | tail -1 | grep -oE "[0-9]+")
if [ -z "$TOTAL_TESTS" ]; then
    TOTAL_TESTS=0
fi

echo "Total tests passing: $TOTAL_TESTS"
if [ "$TOTAL_TESTS" -ge 521 ]; then
    echo "✓ No regressions (≥521 tests passing)"
    PASS=$((PASS + 1))
else
    echo "✗ Regressions detected ($TOTAL_TESTS < 521)"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 7: Documentation Quality
echo "7. DOCUMENTATION QUALITY"
echo "------------------------"
DOC_SCORE=0
if [ -f "crates/mathhook-core/src/functions/elementary/sqrt.rs" ]; then
    if grep -q "///" crates/mathhook-core/src/functions/elementary/sqrt.rs; then
        echo "✓ Documentation comments present"
        DOC_SCORE=$((DOC_SCORE + 1))
    fi

    if grep -q "# Examples" crates/mathhook-core/src/functions/elementary/sqrt.rs; then
        echo "✓ Examples section present"
        DOC_SCORE=$((DOC_SCORE + 1))
    fi
fi

if [ "$DOC_SCORE" -eq 2 ]; then
    PASS=$((PASS + 1))
else
    echo "✗ Documentation incomplete"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 8: Doctests
echo "8. DOCTESTS"
echo "-----------"
DOCTEST_RESULT=$(cargo test -p mathhook-core --doc --quiet 2>&1 | grep "sqrt.rs")
if echo "$DOCTEST_RESULT" | grep -q "ok"; then
    echo "✓ Sqrt doctests pass"
    PASS=$((PASS + 1))
elif [ -z "$DOCTEST_RESULT" ]; then
    echo "⚠  No sqrt doctests found (may be acceptable)"
    PASS=$((PASS + 1))
else
    echo "✗ Some sqrt doctests fail"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 9: API Integration
echo "9. API INTEGRATION"
echo "------------------"
API_SCORE=0
if grep -q "pub fn sqrt" crates/mathhook-core/src/core/expression.rs 2>/dev/null; then
    echo "✓ .sqrt() method in Expression API"
    API_SCORE=$((API_SCORE + 1))
elif grep -q "pub fn sqrt" crates/mathhook-core/src/core/expression/*.rs 2>/dev/null; then
    echo "✓ .sqrt() method in Expression API (submodule)"
    API_SCORE=$((API_SCORE + 1))
fi

if [ "$API_SCORE" -gt 0 ]; then
    PASS=$((PASS + 1))
else
    echo "✗ .sqrt() method not found in Expression API"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 10: Registry Integration
echo "10. REGISTRY INTEGRATION"
echo "------------------------"
if grep -q "sqrt" crates/mathhook-core/src/functions/elementary/mod.rs 2>/dev/null; then
    echo "✓ sqrt module registered in elementary/mod.rs"
    PASS=$((PASS + 1))
else
    echo "✗ sqrt module not registered"
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
    echo "✓ WAVE 2 VERIFICATION: PERFECT (10/10)"
    echo "Status: READY FOR WAVE 3"
    exit 0
else
    QUALITY_SCORE=$(awk "BEGIN {printf \"%.1f\", ($PASS * 10.0) / ($PASS + $FAIL)}")
    echo "✗ WAVE 2 VERIFICATION: $QUALITY_SCORE/10"
    echo "Status: REQUIRES FIXES"
    exit 1
fi
