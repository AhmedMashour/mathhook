#!/bin/bash

# Wave 1: Analysis & Research Verification Script
# Purpose: Verify pure research phase - NO CODE CHANGES, only documentation
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 1: ANALYSIS & RESEARCH VERIFICATION"
echo "Deep dive into MathHook + SymPy architecture (INCLUDING Risch)"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: REQUIRED DOCUMENTS (5 DELIVERABLES)
echo "========================================"
echo "CATEGORY 1: REQUIRED DOCUMENTS"
echo "Wave 1 must deliver 5 analysis documents"
echo "========================================"

REQUIRED_DOCS=(
    ".mathhook_sessions/INTEGRATION_AUDIT.md"
    ".mathhook_sessions/SYMPY_INTEGRATION_ARCHITECTURE.md"
    ".mathhook_sessions/RISCH_ALGORITHM_DESIGN.md"
    ".mathhook_sessions/INTEGRATION_ENHANCEMENT_DESIGN.md"
    ".mathhook_sessions/INTEGRATION_TEST_PLAN.md"
)

MISSING_DOCS=0
for doc in "${REQUIRED_DOCS[@]}"; do
    if [ ! -f "$doc" ]; then
        echo -e "${RED}✗ MISSING: $doc${NC}"
        MISSING_DOCS=$((MISSING_DOCS + 1))
    else
        echo -e "${GREEN}✓ Found: $doc${NC}"
    fi
done

if [ $MISSING_DOCS -gt 0 ]; then
    echo -e "${RED}✗ $MISSING_DOCS documents missing${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All 5 required documents present${NC}"
fi

# CATEGORY 2: DOCUMENT COMPLETENESS
echo "========================================"
echo "CATEGORY 2: DOCUMENT COMPLETENESS"
echo "Each document must be substantive (not stubs)"
echo "========================================"

MIN_WORDS=500  # Each doc should have at least 500 words

INCOMPLETE_DOCS=0
for doc in "${REQUIRED_DOCS[@]}"; do
    if [ -f "$doc" ]; then
        WORD_COUNT=$(wc -w < "$doc")
        if [ "$WORD_COUNT" -lt $MIN_WORDS ]; then
            echo -e "${RED}✗ $doc: Only $WORD_COUNT words (minimum $MIN_WORDS)${NC}"
            INCOMPLETE_DOCS=$((INCOMPLETE_DOCS + 1))
        else
            echo -e "${GREEN}✓ $doc: $WORD_COUNT words${NC}"
        fi
    fi
done

if [ $INCOMPLETE_DOCS -gt 0 ]; then
    echo -e "${RED}✗ $INCOMPLETE_DOCS documents are too short${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All documents are substantive${NC}"
fi

# CATEGORY 3: RISCH ALGORITHM STUDY (CRITICAL)
echo "========================================"
echo "CATEGORY 3: RISCH ALGORITHM STUDY"
echo "Wave 5 depends on deep Risch understanding"
echo "========================================"

RISCH_DOC=".mathhook_sessions/RISCH_ALGORITHM_DESIGN.md"
RISCH_ISSUES=0

if [ -f "$RISCH_DOC" ]; then
    # Check for key Risch concepts (must be mentioned)
    REQUIRED_CONCEPTS=(
        "differential extension"
        "tower"
        "RDE"
        "Hermite reduction"
        "exponential"
        "logarithmic"
        "Bronstein"
    )

    for concept in "${REQUIRED_CONCEPTS[@]}"; do
        if ! grep -qi "$concept" "$RISCH_DOC"; then
            echo -e "${RED}✗ Missing concept: '$concept'${NC}"
            RISCH_ISSUES=$((RISCH_ISSUES + 1))
        fi
    done

    if [ $RISCH_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Risch algorithm study covers all key concepts${NC}"
    else
        echo -e "${RED}✗ Risch algorithm study incomplete: $RISCH_ISSUES missing concepts${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Risch algorithm design document missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: SYMPY REFERENCE COVERAGE
echo "========================================"
echo "CATEGORY 4: SYMPY REFERENCE COVERAGE"
echo "Must analyze SymPy's complete integration pipeline"
echo "========================================"

SYMPY_DOC=".mathhook_sessions/SYMPY_INTEGRATION_ARCHITECTURE.md"
SYMPY_ISSUES=0

if [ -f "$SYMPY_DOC" ]; then
    # Check for analysis of key SymPy modules
    SYMPY_MODULES=(
        "manualintegrate"
        "rationaltools"
        "trigonometry"
        "heurisch"
        "risch.py"
        "rde.py"
    )

    for module in "${SYMPY_MODULES[@]}"; do
        if ! grep -qi "$module" "$SYMPY_DOC"; then
            echo -e "${RED}✗ Missing SymPy module analysis: '$module'${NC}"
            SYMPY_ISSUES=$((SYMPY_ISSUES + 1))
        fi
    done

    if [ $SYMPY_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ SymPy architecture analysis is comprehensive${NC}"
    else
        echo -e "${RED}✗ SymPy architecture analysis incomplete: $SYMPY_ISSUES modules not covered${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ SymPy architecture document missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: CURRENT STATE AUDIT
echo "========================================"
echo "CATEGORY 5: CURRENT STATE AUDIT"
echo "Must document what MathHook currently has/needs"
echo "========================================"

AUDIT_DOC=".mathhook_sessions/INTEGRATION_AUDIT.md"
AUDIT_ISSUES=0

if [ -f "$AUDIT_DOC" ]; then
    # Check for required sections
    REQUIRED_SECTIONS=(
        "Current Implementation"
        "What Works"
        "What's Missing"
        "Function Registry"
    )

    for section in "${REQUIRED_SECTIONS[@]}"; do
        if ! grep -qi "$section" "$AUDIT_DOC"; then
            echo -e "${RED}✗ Missing section: '$section'${NC}"
            AUDIT_ISSUES=$((AUDIT_ISSUES + 1))
        fi
    done

    if [ $AUDIT_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Current state audit is comprehensive${NC}"
    else
        echo -e "${RED}✗ Current state audit incomplete: $AUDIT_ISSUES sections missing${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Integration audit document missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: ARCHITECTURAL DESIGN
echo "========================================"
echo "CATEGORY 6: ARCHITECTURAL DESIGN"
echo "Must provide clear implementation plan"
echo "========================================"

DESIGN_DOC=".mathhook_sessions/INTEGRATION_ENHANCEMENT_DESIGN.md"
DESIGN_ISSUES=0

if [ -f "$DESIGN_DOC" ]; then
    # Check for required design elements
    REQUIRED_DESIGN=(
        "Module Structure"
        "Data Structures"
        "Algorithm"
        "Strategy Dispatcher"
        "Risch"
    )

    for element in "${REQUIRED_DESIGN[@]}"; do
        if ! grep -qi "$element" "$DESIGN_DOC"; then
            echo -e "${RED}✗ Missing design element: '$element'${NC}"
            DESIGN_ISSUES=$((DESIGN_ISSUES + 1))
        fi
    done

    if [ $DESIGN_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Architectural design is comprehensive${NC}"
    else
        echo -e "${RED}✗ Architectural design incomplete: $DESIGN_ISSUES elements missing${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Integration enhancement design document missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: TEST PLAN
echo "========================================"
echo "CATEGORY 7: TEST PLAN"
echo "Must define validation strategy"
echo "========================================"

TEST_PLAN=".mathhook_sessions/INTEGRATION_TEST_PLAN.md"
TEST_ISSUES=0

if [ -f "$TEST_PLAN" ]; then
    # Check for test categories
    TEST_CATEGORIES=(
        "Rational"
        "Trigonometric"
        "Substitution"
        "Risch"
        "SymPy validation"
    )

    for category in "${TEST_CATEGORIES[@]}"; do
        if ! grep -qi "$category" "$TEST_PLAN"; then
            echo -e "${RED}✗ Missing test category: '$category'${NC}"
            TEST_ISSUES=$((TEST_ISSUES + 1))
        fi
    done

    if [ $TEST_ISSUES -eq 0 ]; then
        echo -e "${GREEN}✓ Test plan covers all integration techniques${NC}"
    else
        echo -e "${RED}✗ Test plan incomplete: $TEST_ISSUES categories missing${NC}"
        FAILURES=$((FAILURES + 1))
    fi
else
    echo -e "${RED}✗ Integration test plan document missing!${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: NO CODE CHANGES (RESEARCH ONLY)
echo "========================================"
echo "CATEGORY 8: NO CODE CHANGES"
echo "Wave 1 is pure research - no implementation"
echo "========================================"

# Check if any Rust files were modified
CODE_CHANGES=$(git diff --name-only | grep -c "\.rs$" || echo 0)

if [ "$CODE_CHANGES" -gt 0 ]; then
    echo -e "${RED}✗ Found $CODE_CHANGES Rust file changes (Wave 1 should be documentation only)${NC}"
    git diff --name-only | grep "\.rs$"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No code changes (research phase only)${NC}"
fi

# CATEGORY 9: BUILD STATUS (SHOULD BE UNCHANGED)
echo "========================================"
echo "CATEGORY 9: BUILD STATUS"
echo "Build should still pass (no code changes)"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build still passes${NC}"
else
    echo -e "${RED}✗ Build has errors (should be impossible in Wave 1)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 10: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 10: DOCUMENTATION QUALITY"
echo "Documents should be well-structured markdown"
echo "========================================"

QUALITY_ISSUES=0

for doc in "${REQUIRED_DOCS[@]}"; do
    if [ -f "$doc" ]; then
        # Check for markdown headers (should have sections)
        HEADER_COUNT=$(grep -c "^#" "$doc" || echo 0)
        if [ "$HEADER_COUNT" -lt 3 ]; then
            echo -e "${YELLOW}⚠ $doc: Only $HEADER_COUNT sections (expected 3+)${NC}"
            QUALITY_ISSUES=$((QUALITY_ISSUES + 1))
        fi
    fi
done

if [ $QUALITY_ISSUES -gt 0 ]; then
    echo -e "${YELLOW}⚠ $QUALITY_ISSUES documents may lack structure${NC}"
    # Warning only, not a failure
else
    echo -e "${GREEN}✓ All documents are well-structured${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 1: Analysis & Research is VERIFIED COMPLETE"
    echo ""
    echo "Deliverables:"
    echo "  1. Integration Audit: Current state documented"
    echo "  2. SymPy Architecture: Complete pipeline analyzed (including Risch)"
    echo "  3. Risch Algorithm Design: Wave 5 foundation prepared"
    echo "  4. Enhancement Design: Implementation roadmap created"
    echo "  5. Test Plan: Validation strategy defined"
    echo ""
    echo "Ready to proceed to Wave 2: Foundation (Rational Functions + Strategy Dispatcher)"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 1: Analysis & Research requires fixes before approval"
    echo ""
    echo "Fix the issues above and re-run verification."
    exit 1
fi
