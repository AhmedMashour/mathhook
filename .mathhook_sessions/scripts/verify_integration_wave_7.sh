#!/bin/bash

# Wave 7: Educational Integration - Verification Script
# Ensures step-by-step explanations and message registry integration
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 7: EDUCATIONAL INTEGRATION"
echo "Step-by-Step + Message Registry"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: REQUIRED FILES
echo "========================================"
echo "CATEGORY 1: REQUIRED FILES"
echo "Educational module must exist"
echo "========================================"

REQUIRED_FILES=(
    "crates/mathhook-core/src/calculus/integrals/educational.rs"
)

MISSING=0
for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓ Found: $file${NC}"
    else
        echo -e "${RED}✗ Missing: $file${NC}"
        MISSING=$((MISSING + 1))
    fi
done

if [ $MISSING -eq 0 ]; then
    echo -e "${GREEN}✓ All required files present${NC}"
else
    echo -e "${RED}✗ $MISSING required files missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: FILE SIZE COMPLIANCE
echo "========================================"
echo "CATEGORY 2: FILE SIZE COMPLIANCE"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

VIOLATIONS=0
for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        LINES=$(wc -l < "$file")
        if [ "$LINES" -gt 500 ]; then
            echo -e "${RED}✗ $file: $LINES lines (exceeds 500)${NC}"
            VIOLATIONS=$((VIOLATIONS + 1))
        else
            echo -e "${GREEN}✓ $file: $LINES lines${NC}"
        fi
    fi
done

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files within 500-line limit${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS file(s) exceed limit${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 3: EDUCATIONAL CORE CONCEPTS
echo "========================================"
echo "CATEGORY 3: EDUCATIONAL CORE CONCEPTS"
echo "Must implement step-by-step system"
echo "========================================"

EDUCATIONAL_CONCEPTS=(
    "IntegrationExplanation"
    "explain.*rational"
    "explain.*substitution"
    "explain.*trigonometric"
    "explain.*risch"
    "explain.*table"
    "step.*by.*step"
)

MISSING_CONCEPTS=0
for concept in "${EDUCATIONAL_CONCEPTS[@]}"; do
    if grep -r "$concept" crates/mathhook-core/src/calculus/integrals/educational.rs >/dev/null 2>&1; then
        echo -e "${GREEN}✓ Found concept: $concept${NC}"
    else
        echo -e "${RED}✗ Missing concept: $concept${NC}"
        MISSING_CONCEPTS=$((MISSING_CONCEPTS + 1))
    fi
done

if [ $MISSING_CONCEPTS -eq 0 ]; then
    echo -e "${GREEN}✓ All educational concepts present${NC}"
else
    echo -e "${RED}✗ $MISSING_CONCEPTS concepts missing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: MESSAGE REGISTRY INTEGRATION
echo "========================================"
echo "CATEGORY 4: MESSAGE REGISTRY INTEGRATION"
echo "Must integrate with educational message system"
echo "========================================"

MESSAGE_REGISTRY_INTEGRATION=0
if grep -r "MessageKey\|EducationalRegistry\|EDUCATIONAL_REGISTRY" crates/mathhook-core/src/calculus/integrals/educational.rs >/dev/null 2>&1; then
    echo -e "${GREEN}✓ Message registry integration found${NC}"
else
    echo -e "${YELLOW}⚠ No message registry integration detected${NC}"
    MESSAGE_REGISTRY_INTEGRATION=1
fi

# CATEGORY 5: TEST COMPILATION
echo "========================================"
echo "CATEGORY 5: TEST COMPILATION"
echo "integration_educational.rs must compile"
echo "========================================"

if cargo check -p mathhook-core --test integration_educational >/dev/null 2>&1; then
    echo -e "${GREEN}✓ integration_educational.rs compiles${NC}"
else
    echo -e "${RED}✗ integration_educational.rs has compilation errors${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: TEST COVERAGE
echo "========================================"
echo "CATEGORY 6: TEST COVERAGE"
echo "Wave 7 requires educational tests passing"
echo "========================================"

TEST_OUTPUT=$(cargo test -p mathhook-core --test integration_educational 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    PASSED=$(echo "$TEST_OUTPUT" | grep "test result:" | grep -oP '\d+(?= passed)')
    echo -e "${GREEN}✓ Educational tests passing: $PASSED tests${NC}"
else
    echo -e "${RED}✗ Educational tests failing${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: BUILD STATUS
echo "========================================"
echo "CATEGORY 7: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 8: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_FILES=(
    "crates/mathhook-core/src/calculus/integrals/educational.rs"
)

EMOJI_COUNT=0
for file in "${EMOJI_FILES[@]}"; do
    if [ -f "$file" ]; then
        COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" "$file" 2>/dev/null | wc -l)
        EMOJI_COUNT=$((EMOJI_COUNT + COUNT))
    fi
done

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 9: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 9: DOCUMENTATION QUALITY"
echo "All public functions must have docs"
echo "========================================"

if [ -f "crates/mathhook-core/src/calculus/integrals/educational.rs" ]; then
    DOC_COUNT=$(grep -c "^/// " crates/mathhook-core/src/calculus/integrals/educational.rs 2>/dev/null || echo 0)
    if [ "$DOC_COUNT" -gt 10 ]; then
        echo -e "${GREEN}✓ Found $DOC_COUNT documentation lines${NC}"
    else
        echo -e "${YELLOW}⚠ Only $DOC_COUNT documentation lines (expected 10+)${NC}"
    fi
fi

# CATEGORY 10: TECHNIQUE COVERAGE
echo "========================================"
echo "CATEGORY 10: TECHNIQUE COVERAGE"
echo "Must explain all integration techniques"
echo "========================================"

TECHNIQUES=(
    "rational"
    "substitution"
    "trigonometric"
    "risch"
    "table"
    "by.*parts"
)

MISSING_TECHNIQUES=0
for technique in "${TECHNIQUES[@]}"; do
    if grep -ri "explain.*$technique\|$technique.*explain" crates/mathhook-core/src/calculus/integrals/educational.rs >/dev/null 2>&1; then
        echo -e "${GREEN}✓ Covers technique: $technique${NC}"
    else
        echo -e "${YELLOW}⚠ May not cover: $technique${NC}"
        MISSING_TECHNIQUES=$((MISSING_TECHNIQUES + 1))
    fi
done

if [ $MISSING_TECHNIQUES -eq 0 ]; then
    echo -e "${GREEN}✓ All techniques covered${NC}"
elif [ $MISSING_TECHNIQUES -le 2 ]; then
    echo -e "${YELLOW}⚠ $MISSING_TECHNIQUES techniques may not be covered (warning only)${NC}"
else
    echo -e "${RED}✗ $MISSING_TECHNIQUES techniques not covered${NC}"
    FAILURES=$((FAILURES + 1))
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 7: Educational Integration is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 7 requires fixes before approval"
    exit 1
fi
