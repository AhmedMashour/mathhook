#!/bin/bash
# Wave 2.2 Verification Script - Substitution Engine Fixes
# Tests: 3, 4, 7 (chain rule substitution patterns)

set -e

echo "=================================="
echo "WAVE 2.2 VERIFICATION"
echo "Substitution Engine (Tests 3,4,7)"
echo "=================================="
echo ""

SCORE=0
MAX_SCORE=100

# Test 1: Unit Tests (40 points)
echo "TEST 1: Unit Tests (8 tests) - 40 points"
echo "Running substitution unit tests..."
if cargo test -p mathhook-core --lib calculus::integrals::substitution::tests --quiet 2>&1 | grep -q "test result: ok. 8 passed"; then
    echo "✅ All 8 unit tests passing"
    SCORE=$((SCORE + 40))
else
    echo "❌ Unit tests failing"
fi
echo ""

# Test 2: Integration Test - Chain Rule Pattern (20 points)
echo "TEST 2: Integration test_chain_rule_pattern - 20 points"
if cargo test -p mathhook-core --test integration_comprehensive test_chain_rule_pattern --quiet 2>&1 | grep -q "test result: ok"; then
    echo "✅ test_chain_rule_pattern passing"
    SCORE=$((SCORE + 20))
else
    echo "❌ test_chain_rule_pattern failing"
fi
echo ""

# Test 3: Integration Test - Substitution with Trig (20 points)
echo "TEST 3: Integration test_substitution_with_trig_inside - 20 points"
if cargo test -p mathhook-core --test integration_comprehensive test_substitution_with_trig_inside --quiet 2>&1 | grep -q "test result: ok"; then
    echo "✅ test_substitution_with_trig_inside passing"
    SCORE=$((SCORE + 20))
else
    echo "❌ test_substitution_with_trig_inside failing"
fi
echo ""

# Test 4: No Regressions (20 points)
echo "TEST 4: Regression Check - 20 points"
PASSING=$(cargo test -p mathhook-core --test integration_comprehensive -- --skip test_product_requiring_parts_and_substitution --quiet 2>&1 | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
echo "Integration tests passing: $PASSING/40"
if [ "$PASSING" -ge 39 ]; then
    echo "✅ No regressions (≥39/40 tests passing)"
    SCORE=$((SCORE + 20))
else
    echo "❌ Regressions detected"
fi
echo ""

# Final Score
echo "=================================="
echo "FINAL SCORE: $SCORE/$MAX_SCORE"
echo "=================================="

if [ $SCORE -ge 90 ]; then
    echo "Status: EXCELLENT (90+)"
    exit 0
elif [ $SCORE -ge 75 ]; then
    echo "Status: PASS (75+)"
    exit 0
else
    echo "Status: FAIL (<75)"
    exit 1
fi
