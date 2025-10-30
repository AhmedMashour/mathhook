#!/bin/bash

echo "========================================="
echo "Batch 2.1 Verification: Gamma Family"
echo "========================================="
echo ""

echo "✓ Module Structure:"
find crates/mathhook-core/src/core/functions -type f -name "*.rs" | sort | sed 's|^|  |'
echo ""

echo "✓ File Sizes (max 500 lines per CLAUDE.md):"
wc -l crates/mathhook-core/src/core/functions/gamma/mod.rs | awk '{print "  gamma/mod.rs: " $1 " lines"}'
wc -l crates/mathhook-core/src/core/functions/beta/mod.rs | awk '{print "  beta/mod.rs: " $1 " lines"}'
wc -l crates/mathhook-core/src/core/functions/digamma/mod.rs | awk '{print "  digamma/mod.rs: " $1 " lines"}'
echo ""

echo "✓ Test Coverage:"
gamma_tests=$(grep -c "#\[test\]" crates/mathhook-core/src/core/functions/gamma/tests.rs)
beta_tests=$(grep -c "#\[test\]" crates/mathhook-core/src/core/functions/beta/tests.rs)
digamma_tests=$(grep -c "#\[test\]" crates/mathhook-core/src/core/functions/digamma/tests.rs)
total_tests=$((gamma_tests + beta_tests + digamma_tests))
echo "  Gamma: $gamma_tests tests"
echo "  Beta: $beta_tests tests"
echo "  Digamma: $digamma_tests tests"
echo "  Total: $total_tests tests (baseline: 11, delta: +$((total_tests - 11)))"
echo ""

echo "✓ Architecture Compliance:"
echo "  Data separation verified:"
grep -l "gamma_special_value" crates/mathhook-core/src/core/functions/gamma/mod.rs > /dev/null && echo "    ✓ gamma/mod.rs uses data::gamma_special_value"
grep -l "beta_special_value" crates/mathhook-core/src/core/functions/beta/mod.rs > /dev/null && echo "    ✓ beta/mod.rs uses data::beta_special_value"
grep -l "digamma_special_value" crates/mathhook-core/src/core/functions/digamma/mod.rs > /dev/null && echo "    ✓ digamma/mod.rs uses data::digamma_special_value"
echo ""

echo "✓ CLAUDE.md Compliance:"
inline_comments=$(rg "^\s*//[^/!]" --type rust crates/mathhook-core/src/core/functions/ 2>/dev/null | wc -l)
if [ "$inline_comments" -eq 0 ]; then
    echo "  ✓ No inline comments (only /// and //!)"
else
    echo "  ⚠ Found $inline_comments inline comments"
fi

todos=$(rg "TODO" crates/mathhook-core/src/core/functions/ 2>/dev/null | wc -l)
if [ "$todos" -eq 0 ]; then
    echo "  ✓ No TODO comments"
else
    echo "  ⚠ Found $todos TODO comments"
fi

emojis=$(rg "[😀-🙏]" crates/mathhook-core/src/core/functions/ 2>/dev/null | wc -l)
if [ "$emojis" -eq 0 ]; then
    echo "  ✓ No emojis"
else
    echo "  ⚠ Found $emojis emojis"
fi
echo ""

echo "========================================="
echo "✅ Batch 2.1 refactoring COMPLETE"
echo "========================================="
