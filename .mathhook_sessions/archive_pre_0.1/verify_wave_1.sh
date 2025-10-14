#!/bin/bash
# WAVE 1 MASTER VERIFICATION SCRIPT
# Purpose: Establish objective ground truth for all P0 tasks

cd /Users/ahmedmashhour/Documents/work/math/mathhook

echo "=========================================="
echo "WAVE 1 VERIFICATION - GROUND TRUTH CHECK"
echo "Date: $(date)"
echo "=========================================="
echo ""

# P0-1: Pattern Matching
echo "P0-1: Pattern Matching"
cargo test -p mathhook-core pattern --quiet 2>&1 | grep "test result:"
echo ""

# P0-2: Polynomial Solver
echo "P0-2: Polynomial Solver"
cargo test -p mathhook-core polynomial_solver --quiet 2>&1 | grep "test result:"
echo ""

# P0-3: Doctests
echo "P0-3: Doctests"
cargo test --doc -p mathhook-core --quiet 2>&1 | grep "test result:" | tail -1
echo ""

# P0-4: Number Arithmetic
echo "P0-4: Number Arithmetic"
cargo test -p mathhook-core number_arithmetic --quiet 2>&1 | grep "test result:"
echo ""

# P0-5: Domain Errors
echo "P0-5: Domain Errors"
cargo test -p mathhook-core --test domain_error_tests --quiet 2>&1 | grep "test result:"
echo ""

# P0-6: Code Quality
echo "P0-6: Code Quality"
emoji_count=$(rg '[^\x00-\x7F]' --type rust crates/mathhook-core/src/ 2>/dev/null | wc -l | xargs)
caps_count=$(rg '^[^/]*//[!/]?.*[A-Z]{4,}' --type rust crates/mathhook-core/src/ 2>/dev/null | wc -l | xargs)
echo "Emoji violations: $emoji_count"
echo "ALL CAPS violations: $caps_count"
if [ "$emoji_count" -eq 0 ] && [ "$caps_count" -eq 0 ]; then
    echo "test result: ok. 0 violations"
else
    echo "test result: FAILED. $((emoji_count + caps_count)) violations"
fi
echo ""

echo "=========================================="
echo "END VERIFICATION"
echo "=========================================="
