#!/bin/bash

# Wave 8: Final Completion - Verification Script
# Comprehensive quality audit, documentation, and release readiness
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 8: FINAL COMPLETION & RELEASE"
echo "Documentation + Audit + Testing"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: DOCUMENTATION DELIVERABLES
echo "========================================"
echo "CATEGORY 1: DOCUMENTATION DELIVERABLES"
echo "Wave 8 must deliver comprehensive docs"
echo "========================================"

REQUIRED_DOCS=(
    "docs/INTEGRATION_GUIDE.md"
    "docs/RISCH_ALGORITHM.md"
    ".mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md"
)

MISSING_DOCS=0
for doc in "${REQUIRED_DOCS[@]}"; do
    if [ -f "$doc" ]; then
        LINES=$(wc -l < "$doc" 2>/dev/null || echo 0)
        echo -e "${GREEN}✓ Found: $doc ($LINES lines)${NC}"
    else
        echo -e "${RED}✗ Missing: $doc${NC}"
        MISSING_DOCS=$((MISSING_DOCS + 1))
    fi
done

if [ $MISSING_DOCS -eq 0 ]; then
    echo -e "${GREEN}✓ All documentation deliverables present${NC}"
else
    echo -e "${RED}✗ $MISSING_DOCS documentation files missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: COMPREHENSIVE TEST PASS RATE
echo "========================================"
echo "CATEGORY 2: COMPREHENSIVE TEST PASS RATE"
echo "All integration tests must pass"
echo "========================================"

# Run all integration tests
TEST_OUTPUT=$(cargo test -p mathhook-core integration 2>&1)

# Count total tests
TOTAL_TESTS=$(echo "$TEST_OUTPUT" | grep -o "running [0-9]* test" | awk '{sum+=$2} END {print sum}')
PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep -o "[0-9]* passed" | awk '{sum+=$1} END {print sum}')
FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep -o "[0-9]* failed" | awk '{sum+=$1} END {print sum}')

echo "Total integration tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: ${FAILED_TESTS:-0}"

if [ "${FAILED_TESTS:-0}" -eq 0 ]; then
    echo -e "${GREEN}✓ All integration tests passing${NC}"
else
    echo -e "${RED}✗ $FAILED_TESTS tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 3: WAVE 6 TEST FIXES
echo "========================================"
echo "CATEGORY 3: WAVE 6 TEST FIXES"
echo "integration_comprehensive.rs must pass"
echo "========================================"

COMPREHENSIVE_OUTPUT=$(cargo test -p mathhook-core --test integration_comprehensive 2>&1)

if echo "$COMPREHENSIVE_OUTPUT" | grep -q "test result: ok"; then
    COMP_PASSED=$(echo "$COMPREHENSIVE_OUTPUT" | grep "test result:" | grep -o "[0-9]* passed" | awk '{print $1}')
    echo -e "${GREEN}✓ Comprehensive tests passing: $COMP_PASSED tests${NC}"
else
    echo -e "${RED}✗ Comprehensive tests have failures${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: PERFORMANCE BENCHMARKS
echo "========================================"
echo "CATEGORY 4: PERFORMANCE BENCHMARKS"
echo "Must document performance characteristics"
echo "========================================"

# Check if performance report exists
if [ -f ".mathhook_sessions/INTEGRATION_PERFORMANCE_REPORT.md" ]; then
    echo -e "${GREEN}✓ Performance report exists${NC}"
elif grep -r "performance\|benchmark" .mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md >/dev/null 2>&1; then
    echo -e "${GREEN}✓ Performance benchmarks documented in audit${NC}"
else
    echo -e "${YELLOW}⚠ No performance benchmarks documented${NC}"
fi

# CATEGORY 5: COVERAGE METRICS
echo "========================================"
echo "CATEGORY 5: COVERAGE METRICS"
echo "Must document 75% → 93-95% coverage improvement"
echo "========================================"

if [ -f ".mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md" ]; then
    if grep -q "75%.*93\|75%.*95\|coverage" .mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md; then
        echo -e "${GREEN}✓ Coverage metrics documented${NC}"
    else
        echo -e "${YELLOW}⚠ Coverage metrics not found in audit${NC}"
    fi
else
    echo -e "${YELLOW}⚠ Quality audit not yet created${NC}"
fi

# CATEGORY 6: BUILD STATUS (FINAL)
echo "========================================"
echo "CATEGORY 6: BUILD STATUS (FINAL)"
echo "Must compile with 0 errors"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: CLAUDE.MD COMPLIANCE (ALL WAVES)
echo "========================================"
echo "CATEGORY 7: CLAUDE.MD COMPLIANCE (ALL WAVES)"
echo "All integration files must be compliant"
echo "========================================"

INTEGRATION_FILES=(
    "crates/mathhook-core/src/calculus/integrals.rs"
    "crates/mathhook-core/src/calculus/integrals/strategy.rs"
    "crates/mathhook-core/src/calculus/integrals/rational.rs"
    "crates/mathhook-core/src/calculus/integrals/table.rs"
    "crates/mathhook-core/src/calculus/integrals/substitution.rs"
    "crates/mathhook-core/src/calculus/integrals/trigonometric.rs"
    "crates/mathhook-core/src/calculus/integrals/educational.rs"
)

EMOJI_COUNT=0
for file in "${INTEGRATION_FILES[@]}"; do
    if [ -f "$file" ]; then
        COUNT=$(grep -c "✅\|❌\|⚠️\|🚀\|✨" "$file" 2>/dev/null || echo 0)
        EMOJI_COUNT=$((EMOJI_COUNT + COUNT))
    fi
done

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis in integration files${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis in integration files${NC}"
fi

# CATEGORY 8: QUALITY AUDIT COMPLETENESS
echo "========================================"
echo "CATEGORY 8: QUALITY AUDIT COMPLETENESS"
echo "Quality audit must cover all waves"
echo "========================================"

if [ -f ".mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md" ]; then
    AUDIT_SIZE=$(wc -l < ".mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md")
    if [ "$AUDIT_SIZE" -gt 200 ]; then
        echo -e "${GREEN}✓ Quality audit comprehensive ($AUDIT_SIZE lines)${NC}"
    else
        echo -e "${YELLOW}⚠ Quality audit brief ($AUDIT_SIZE lines)${NC}"
    fi

    # Check for wave coverage
    for wave in {1..7}; do
        if grep -q "Wave $wave\|wave $wave" .mathhook_sessions/INTEGRATION_QUALITY_AUDIT.md; then
            echo -e "${GREEN}✓ Wave $wave covered in audit${NC}"
        else
            echo -e "${YELLOW}⚠ Wave $wave not explicitly mentioned${NC}"
        fi
    done
else
    echo -e "${RED}✗ Quality audit not created${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 9: INTEGRATION GUIDE QUALITY
echo "========================================"
echo "CATEGORY 9: INTEGRATION GUIDE QUALITY"
echo "User guide must be comprehensive"
echo "========================================"

if [ -f "docs/INTEGRATION_GUIDE.md" ]; then
    GUIDE_SIZE=$(wc -l < "docs/INTEGRATION_GUIDE.md")
    if [ "$GUIDE_SIZE" -gt 300 ]; then
        echo -e "${GREEN}✓ Integration guide comprehensive ($GUIDE_SIZE lines)${NC}"
    else
        echo -e "${YELLOW}⚠ Integration guide brief ($GUIDE_SIZE lines)${NC}"
    fi

    # Check for technique coverage
    TECHNIQUES=("rational" "substitution" "trigonometric" "risch" "table" "by.parts")
    for tech in "${TECHNIQUES[@]}"; do
        if grep -qi "$tech" docs/INTEGRATION_GUIDE.md; then
            echo -e "${GREEN}✓ Covers: $tech${NC}"
        else
            echo -e "${YELLOW}⚠ May not cover: $tech${NC}"
        fi
    done
fi

# CATEGORY 10: RISCH ALGORITHM DOCUMENTATION
echo "========================================"
echo "CATEGORY 10: RISCH ALGORITHM DOCUMENTATION"
echo "Risch guide must explain algorithm"
echo "========================================"

if [ -f "docs/RISCH_ALGORITHM.md" ]; then
    RISCH_SIZE=$(wc -l < "docs/RISCH_ALGORITHM.md")
    if [ "$RISCH_SIZE" -gt 200 ]; then
        echo -e "${GREEN}✓ Risch documentation comprehensive ($RISCH_SIZE lines)${NC}"
    else
        echo -e "${YELLOW}⚠ Risch documentation brief ($RISCH_SIZE lines)${NC}"
    fi

    # Check for key concepts
    CONCEPTS=("differential.extension\|tower" "hermite" "rde\|differential.equation" "exponential\|logarithmic")
    for concept in "${CONCEPTS[@]}"; do
        if grep -qi "$concept" docs/RISCH_ALGORITHM.md; then
            echo -e "${GREEN}✓ Covers: $concept${NC}"
        else
            echo -e "${YELLOW}⚠ May not cover: $concept${NC}"
        fi
    done
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 8: Final Completion is VERIFIED COMPLETE"
    echo "Symbolic Integration Enhancement PROJECT READY FOR RELEASE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 8 requires fixes before approval"
    exit 1
fi
