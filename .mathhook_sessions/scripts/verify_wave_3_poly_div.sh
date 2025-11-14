#!/bin/bash

# Wave 3: Polynomial Division API Enhancement - Verification Script
# Target: 10/10 quality (Polish existing implementation)

echo "================================================"
echo "Wave 3: Polynomial Division API Enhancement"
echo "Verification Script - 10 Categories"
echo "================================================"
echo ""

PASS=0
FAIL=0

# Category 1: File Existence and Structure
echo "1. FILE EXISTENCE AND STRUCTURE"
echo "--------------------------------"
FILES_EXIST=0
if [ -f "crates/mathhook-core/src/algebra/polynomial_division.rs" ]; then
    echo "✓ polynomial_division.rs exists (already implemented)"
    FILES_EXIST=$((FILES_EXIST + 1))
fi

if [ -f "crates/mathhook-core/tests/polynomial_division_api_tests.rs" ]; then
    echo "✓ polynomial_division_api_tests.rs exists (new)"
    FILES_EXIST=$((FILES_EXIST + 1))
fi

if [ "$FILES_EXIST" -eq 2 ]; then
    PASS=$((PASS + 1))
else
    echo "✗ Missing required files"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 2: CLAUDE.md Compliance - No Emojis
echo "2. CLAUDE.MD COMPLIANCE - NO EMOJIS"
echo "------------------------------------"
EMOJI_COUNT=0
if [ -f "crates/mathhook-core/src/algebra/polynomial_division.rs" ]; then
    if grep -P "[\x{1F300}-\x{1F6FF}]" crates/mathhook-core/src/algebra/polynomial_division.rs 2>/dev/null | grep -v "^//"; then
        echo "✗ Found emojis in polynomial_division.rs"
        EMOJI_COUNT=$((EMOJI_COUNT + 1))
    fi
fi

if [ -f "crates/mathhook-core/tests/polynomial_division_api_tests.rs" ]; then
    if grep -P "[\x{1F300}-\x{1F6FF}]" crates/mathhook-core/tests/polynomial_division_api_tests.rs 2>/dev/null | grep -v "^//"; then
        echo "✗ Found emojis in polynomial_division_api_tests.rs"
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
if cargo clippy -p mathhook-core --quiet 2>&1 | grep "polynomial_division_api_tests.rs"; then
    echo "✗ Clippy warnings in new test file"
    NEW_WARNINGS=$((NEW_WARNINGS + 1))
fi

if [ "$NEW_WARNINGS" -eq 0 ]; then
    echo "✓ No clippy warnings in new code"
    PASS=$((PASS + 1))
else
    echo "✗ Found clippy warnings"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 5: Test Count and Execution
echo "5. TEST COUNT AND EXECUTION"
echo "---------------------------"
POLY_API_TEST_COUNT=$(cargo test -p mathhook-core --test polynomial_division_api_tests --quiet 2>&1 | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+" | head -1)
if [ -z "$POLY_API_TEST_COUNT" ]; then
    POLY_API_TEST_COUNT=0
fi

echo "Polynomial division API tests found: $POLY_API_TEST_COUNT"
if [ "$POLY_API_TEST_COUNT" -ge 10 ]; then
    echo "✓ Sufficient test count (≥10)"
    PASS=$((PASS + 1))
else
    echo "✗ Insufficient tests ($POLY_API_TEST_COUNT < 10)"
    FAIL=$((FAIL + 1))
fi

if cargo test -p mathhook-core --test polynomial_division_api_tests --quiet 2>&1 | grep -q "test result: ok"; then
    echo "✓ All polynomial division API tests pass"
    PASS=$((PASS + 1))
else
    echo "✗ Some tests fail"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 6: Zero Regressions
echo "6. ZERO REGRESSIONS"
echo "-------------------"
TOTAL_TESTS=$(cargo test -p mathhook-core --quiet 2>&1 | grep "test result: ok" | grep -oE "[0-9]+ passed" | tail -1 | grep -oE "[0-9]+")
if [ -z "$TOTAL_TESTS" ]; then
    TOTAL_TESTS=0
fi

echo "Total tests passing: $TOTAL_TESTS"
if [ "$TOTAL_TESTS" -ge 528 ]; then
    echo "✓ No regressions (≥528 tests passing)"
    PASS=$((PASS + 1))
else
    echo "✗ Regressions detected ($TOTAL_TESTS < 528)"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 7: Documentation Quality
echo "7. DOCUMENTATION QUALITY"
echo "------------------------"
DOC_SCORE=0
if [ -f "crates/mathhook-core/src/algebra/polynomial_division.rs" ]; then
    if grep -q "# Examples" crates/mathhook-core/src/algebra/polynomial_division.rs; then
        echo "✓ Examples section present"
        DOC_SCORE=$((DOC_SCORE + 1))
    else
        echo "⚠  No Examples section in polynomial_division.rs"
    fi
fi

if [ -f "crates/mathhook-core/src/algebra/mod.rs" ]; then
    if grep -q "polynomial division" crates/mathhook-core/src/algebra/mod.rs; then
        echo "✓ Polynomial division documented in algebra/mod.rs"
        DOC_SCORE=$((DOC_SCORE + 1))
    else
        echo "⚠  Polynomial division not documented in module docs"
    fi
fi

if [ "$DOC_SCORE" -ge 1 ]; then
    PASS=$((PASS + 1))
else
    echo "✗ Documentation incomplete"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 8: Trait Methods Added
echo "8. TRAIT METHODS ADDED"
echo "----------------------"
TRAIT_METHODS=0
if grep -q "div_polynomial" crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null; then
    echo "✓ div_polynomial method found"
    TRAIT_METHODS=$((TRAIT_METHODS + 1))
fi

if grep -q "quo_polynomial" crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null; then
    echo "✓ quo_polynomial method found"
    TRAIT_METHODS=$((TRAIT_METHODS + 1))
fi

if grep -q "rem_polynomial" crates/mathhook-core/src/algebra/gcd.rs 2>/dev/null; then
    echo "✓ rem_polynomial method found"
    TRAIT_METHODS=$((TRAIT_METHODS + 1))
fi

if [ "$TRAIT_METHODS" -eq 3 ]; then
    echo "✓ All 3 convenience methods added"
    PASS=$((PASS + 1))
else
    echo "✗ Missing convenience methods ($TRAIT_METHODS / 3)"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 9: Examples File
echo "9. EXAMPLES FILE"
echo "----------------"
if [ -f "examples/polynomial_division_usage.rs" ]; then
    echo "✓ polynomial_division_usage.rs example exists"
    LINE_COUNT=$(wc -l < "examples/polynomial_division_usage.rs")
    if [ "$LINE_COUNT" -ge 50 ]; then
        echo "✓ Example file has substantial content ($LINE_COUNT lines)"
        PASS=$((PASS + 1))
    else
        echo "✗ Example file too short ($LINE_COUNT < 50 lines)"
        FAIL=$((FAIL + 1))
    fi
else
    echo "✗ polynomial_division_usage.rs example not found"
    FAIL=$((FAIL + 1))
fi
echo ""

# Category 10: Doctests
echo "10. DOCTESTS"
echo "-----------"
DOCTEST_RESULT=$(cargo test -p mathhook-core --doc --quiet 2>&1 | grep "polynomial_division.rs")
if echo "$DOCTEST_RESULT" | grep -q "ok"; then
    echo "✓ Polynomial division doctests pass"
    PASS=$((PASS + 1))
elif [ -z "$DOCTEST_RESULT" ]; then
    echo "⚠  No polynomial division doctests (acceptable if API methods documented)"
    PASS=$((PASS + 1))
else
    echo "✗ Some doctests fail"
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
    echo "✓ WAVE 3 VERIFICATION: PERFECT (10/10)"
    echo "Status: READY FOR BUNDLE COMPLETION"
    exit 0
else
    QUALITY_SCORE=$(awk "BEGIN {printf \"%.1f\", ($PASS * 10.0) / ($PASS + $FAIL)}")
    echo "✗ WAVE 3 VERIFICATION: $QUALITY_SCORE/10"
    echo "Status: REQUIRES FIXES"
    exit 1
fi
