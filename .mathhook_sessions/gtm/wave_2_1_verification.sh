#!/bin/bash
# Wave 2.1 Verification Script: Symbol Constructor Migration - Part 1
# Target: Reduce Symbol::new() uses from 287 to 144 (50% reduction)
# Priority: MANDATORY (CLAUDE.md Priority 1)

set -e

WAVE="2.1"
WAVE_NAME="Symbol Constructor Migration - Part 1"
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
REPORT_FILE=".mathhook_sessions/gtm/wave_2_1_verification_report.md"

echo "============================================"
echo "Wave $WAVE Verification: $WAVE_NAME"
echo "Started: $TIMESTAMP"
echo "============================================"
echo ""

# Initialize counters
PASS_COUNT=0
FAIL_COUNT=0
TOTAL_CHECKS=0

# Helper functions
check_pass() {
    echo "✅ PASS: $1"
    PASS_COUNT=$((PASS_COUNT + 1))
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
}

check_fail() {
    echo "❌ FAIL: $1"
    echo "   Details: $2"
    FAIL_COUNT=$((FAIL_COUNT + 1))
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
}

check_info() {
    echo "ℹ️  INFO: $1"
}

# Navigate to project root
cd "$(dirname "$0")/../.."

echo "=== Category 1: Symbol Constructor Usage Reduction ==="
echo ""

# C1.1: Count Symbol::new() uses
SYMBOL_NEW_COUNT=$(rg "Symbol::new\(" --type rust -c | awk -F: '{sum+=$2} END {print sum}')
echo "Symbol::new() uses found: $SYMBOL_NEW_COUNT"

if [ "$SYMBOL_NEW_COUNT" -le 144 ]; then
    check_pass "C1.1: Symbol::new() uses reduced to ≤144 (found: $SYMBOL_NEW_COUNT)"
else
    check_fail "C1.1: Symbol::new() uses still >144 (found: $SYMBOL_NEW_COUNT)" "Target is ≤144 uses"
fi

# C1.2: Count symbol!() macro uses
SYMBOL_MACRO_COUNT=$(rg "symbol!\(" --type rust -c | awk -F: '{sum+=$2} END {print sum}')
echo "symbol!() macro uses found: $SYMBOL_MACRO_COUNT"

# Expect at least 143 migrations (287 - 144 = 143)
if [ "$SYMBOL_MACRO_COUNT" -ge 143 ]; then
    check_pass "C1.2: symbol!() macro usage increased (found: $SYMBOL_MACRO_COUNT, expected ≥143)"
else
    check_fail "C1.2: Insufficient symbol!() macro usage (found: $SYMBOL_MACRO_COUNT, expected ≥143)" "Not enough migrations completed"
fi

# C1.3: Check for remaining Symbol::scalar/matrix/operator/quaternion uses
SYMBOL_TYPED_COUNT=$(rg "Symbol::(scalar|matrix|operator|quaternion)\(" --type rust -c | awk -F: '{sum+=$2} END {print sum}')
echo "Symbol::{typed}() uses found: $SYMBOL_TYPED_COUNT"

if [ "$SYMBOL_TYPED_COUNT" -eq 0 ]; then
    check_pass "C1.3: No Symbol::{typed}() direct constructors remain"
else
    check_info "C1.3: Found $SYMBOL_TYPED_COUNT Symbol::{typed}() uses (should migrate to symbol!(x; type) syntax)"
fi

echo ""
echo "=== Category 2: Mathematical Correctness (No Regressions) ==="
echo ""

# C2.1: Run full test suite
echo "Running test suite..."
if cargo test --all --quiet 2>&1 | tee /tmp/test_output.txt | grep -q "test result: ok"; then
    FAIL_COUNT_TESTS=$(grep "test result:" /tmp/test_output.txt | grep -oP '\d+ failed' | grep -oP '\d+' || echo "0")
    if [ "$FAIL_COUNT_TESTS" -eq 0 ]; then
        check_pass "C2.1: All tests pass (no regressions from symbol migration)"
    else
        check_fail "C2.1: Test failures detected ($FAIL_COUNT_TESTS tests failed)" "See /tmp/test_output.txt"
    fi
else
    check_fail "C2.1: Test suite execution failed" "See /tmp/test_output.txt"
fi

# C2.2: Check for build errors
echo "Checking build status..."
if cargo build --all --quiet 2>&1 | tee /tmp/build_output.txt; then
    check_pass "C2.2: Project builds successfully"
else
    check_fail "C2.2: Build errors detected" "See /tmp/build_output.txt"
fi

echo ""
echo "=== Category 3: CLAUDE.md Compliance ==="
echo ""

# C3.1: Check for forbidden Symbol::new() in application code (excluding macro implementations)
SYMBOL_NEW_IN_APP=$(rg "Symbol::new\(" --type rust --glob '!**/macros/*.rs' --glob '!**/tests/*.rs' -c | awk -F: '{sum+=$2} END {print sum}')
echo "Symbol::new() in application code (excluding macros/tests): $SYMBOL_NEW_IN_APP"

if [ "$SYMBOL_NEW_IN_APP" -le 72 ]; then
    check_pass "C3.1: Symbol::new() in application code ≤72 (found: $SYMBOL_NEW_IN_APP, 50% of 144 target)"
else
    check_info "C3.1: Symbol::new() in application code still high (found: $SYMBOL_NEW_IN_APP)"
fi

# C3.2: Check for proper symbol!() usage patterns
SYMBOL_MACRO_PROPER=$(rg "let \w+ = symbol!\(\w+\);" --type rust -c | awk -F: '{sum+=$2} END {print sum}')
echo "Proper symbol!() usage patterns found: $SYMBOL_MACRO_PROPER"

if [ "$SYMBOL_MACRO_PROPER" -ge 100 ]; then
    check_pass "C3.2: Proper symbol!() usage patterns found (≥100)"
else
    check_info "C3.2: Limited proper symbol!() usage patterns (found: $SYMBOL_MACRO_PROPER)"
fi

echo ""
echo "=== Category 4: Code Quality ==="
echo ""

# C4.1: Check for clippy warnings in modified modules
echo "Running clippy on modified modules..."
CLIPPY_WARNINGS=$(cargo clippy --all --quiet -- -W clippy::all 2>&1 | grep -c "warning:" || echo "0")
echo "Clippy warnings: $CLIPPY_WARNINGS"

if [ "$CLIPPY_WARNINGS" -le 300 ]; then
    check_pass "C4.1: Clippy warnings ≤300 (found: $CLIPPY_WARNINGS)"
else
    check_fail "C4.1: Clippy warnings increased (found: $CLIPPY_WARNINGS, limit: 300)" "Review clippy output"
fi

# C4.2: Check for proper documentation
UNDOCUMENTED_FUNCTIONS=$(rg "pub fn \w+\(" --type rust -A1 | grep -v "///" | grep -c "pub fn" || echo "0")
echo "Undocumented public functions: $UNDOCUMENTED_FUNCTIONS"

if [ "$UNDOCUMENTED_FUNCTIONS" -le 50 ]; then
    check_pass "C4.2: Undocumented public functions ≤50 (found: $UNDOCUMENTED_FUNCTIONS)"
else
    check_info "C4.2: Found $UNDOCUMENTED_FUNCTIONS undocumented public functions"
fi

echo ""
echo "=== Category 5: Migration Pattern Validation ==="
echo ""

# C5.1: Check for mixed usage (both Symbol::new and symbol! in same file)
FILES_WITH_BOTH=$(rg "Symbol::new\(" --type rust --files-with-matches | xargs -I {} sh -c 'rg -q "symbol!\(" {} && echo {}' | wc -l)
echo "Files with both Symbol::new() and symbol!(): $FILES_WITH_BOTH"

if [ "$FILES_WITH_BOTH" -le 20 ]; then
    check_pass "C5.1: Limited mixed usage files (≤20 files with both patterns)"
else
    check_info "C5.1: Many files with mixed usage (found: $FILES_WITH_BOTH)"
fi

# C5.2: Verify no runtime variable misuse in macros
MACRO_RUNTIME_VARS=$(rg "for .* in.*symbol!\(" --type rust -c | awk -F: '{sum+=$2} END {print sum}')
echo "Potential runtime variable misuse in symbol!(): $MACRO_RUNTIME_VARS"

if [ "$MACRO_RUNTIME_VARS" -eq 0 ]; then
    check_pass "C5.2: No runtime variables in symbol!() macros"
else
    check_fail "C5.2: Runtime variables detected in symbol!() macros" "Review CLAUDE.md macro pitfalls"
fi

echo ""
echo "=== Category 6: File-by-File Migration Progress ==="
echo ""

# C6.1: Identify top 5 files needing migration
echo "Top 5 files with most Symbol::new() uses:"
rg "Symbol::new\(" --type rust -c | sort -t: -k2 -nr | head -5 | while IFS=: read -r file count; do
    echo "  - $file: $count uses"
done

# C6.2: Identify successfully migrated files (0 Symbol::new() uses)
FULLY_MIGRATED=$(rg "Symbol::new\(" --type rust --files-with-matches | wc -l)
TOTAL_RUST_FILES=$(find crates -name "*.rs" | wc -l)
CLEAN_FILES=$((TOTAL_RUST_FILES - FULLY_MIGRATED))
echo ""
echo "Files fully migrated (0 Symbol::new()): $CLEAN_FILES / $TOTAL_RUST_FILES"

if [ "$CLEAN_FILES" -ge 50 ]; then
    check_pass "C6.1: ≥50 files fully migrated to symbol!() macro"
else
    check_info "C6.1: $CLEAN_FILES files fully migrated (target: ≥50)"
fi

echo ""
echo "============================================"
echo "Verification Summary"
echo "============================================"
echo "Total Checks: $TOTAL_CHECKS"
echo "Passed: $PASS_COUNT"
echo "Failed: $FAIL_COUNT"
echo ""

# Calculate success percentage
if [ "$TOTAL_CHECKS" -gt 0 ]; then
    SUCCESS_RATE=$((PASS_COUNT * 100 / TOTAL_CHECKS))
    echo "Success Rate: $SUCCESS_RATE%"
    echo ""
fi

# Generate detailed report
cat > "$REPORT_FILE" << EOF
# Wave 2.1 Verification Report: Symbol Constructor Migration - Part 1

**Timestamp**: $TIMESTAMP
**Wave**: $WAVE - $WAVE_NAME
**Status**: $([ "$FAIL_COUNT" -eq 0 ] && echo "✅ PASS" || echo "❌ FAIL")

## Executive Summary

- **Total Checks**: $TOTAL_CHECKS
- **Passed**: $PASS_COUNT
- **Failed**: $FAIL_COUNT
- **Success Rate**: $SUCCESS_RATE%

## Verification Results

### Symbol Constructor Usage Reduction
- Symbol::new() uses: $SYMBOL_NEW_COUNT (target: ≤144)
- symbol!() macro uses: $SYMBOL_MACRO_COUNT (expected: ≥143)
- Symbol::{typed}() uses: $SYMBOL_TYPED_COUNT (target: 0)

### Mathematical Correctness
- Test suite: $([ "$FAIL_COUNT_TESTS" -eq 0 ] && echo "✅ All tests pass" || echo "❌ $FAIL_COUNT_TESTS tests failed")
- Build status: $(cargo build --all --quiet 2>&1 > /dev/null && echo "✅ Success" || echo "❌ Failed")

### Code Quality
- Clippy warnings: $CLIPPY_WARNINGS (limit: 300)
- Undocumented functions: $UNDOCUMENTED_FUNCTIONS

### Migration Progress
- Files with both patterns: $FILES_WITH_BOTH
- Files fully migrated: $CLEAN_FILES / $TOTAL_RUST_FILES

## Detailed Results

EOF

# Append detailed check results to report
if [ "$FAIL_COUNT" -eq 0 ]; then
    echo "✅ **All verification checks passed**" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "## Next Steps" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "- Proceed to Wave 2.2: Symbol Constructor Migration - Part 2" >> "$REPORT_FILE"
    echo "- Target: Complete migration to 0 Symbol::new() uses" >> "$REPORT_FILE"
else
    echo "❌ **Verification failed with $FAIL_COUNT failed checks**" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "## Required Fixes" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    echo "Review failed checks above and address before proceeding to Wave 2.2." >> "$REPORT_FILE"
fi

echo "Verification report saved to: $REPORT_FILE"
echo ""

# Exit with appropriate code
if [ "$FAIL_COUNT" -eq 0 ]; then
    echo "✅ Wave 2.1 verification PASSED"
    exit 0
else
    echo "❌ Wave 2.1 verification FAILED ($FAIL_COUNT checks failed)"
    exit 1
fi
