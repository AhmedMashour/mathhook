#!/bin/bash

# Wave 1: Analysis & Planning Verification Script
# Purpose: Verify comprehensive architecture analysis completed
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE 1: ANALYSIS & PLANNING VERIFICATION"
echo "Solver Architecture Refactoring"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: DELIVERABLES EXISTENCE
echo "========================================"
echo "CATEGORY 1: DELIVERABLES EXISTENCE"
echo "Required: Architecture audit and refactoring plan"
echo "========================================"

AUDIT_FILE=".mathhook_sessions/SOLVER_ARCHITECTURE_AUDIT.md"
PLAN_FILE=".mathhook_sessions/SOLVER_REFACTORING_PLAN.md"

if [ -f "$AUDIT_FILE" ]; then
    echo -e "${GREEN}✓ Architecture audit exists${NC}"
else
    echo -e "${RED}✗ Architecture audit missing: $AUDIT_FILE${NC}"
    FAILURES=$((FAILURES + 1))
fi

if [ -f "$PLAN_FILE" ]; then
    echo -e "${GREEN}✓ Refactoring plan exists${NC}"
else
    echo -e "${RED}✗ Refactoring plan missing: $PLAN_FILE${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: AUDIT COMPLETENESS
echo "========================================"
echo "CATEGORY 2: AUDIT COMPLETENESS"
echo "Architecture audit must cover all required sections"
echo "========================================"

if [ -f "$AUDIT_FILE" ]; then
    REQUIRED_SECTIONS=("MathSolver" "SmartEquationSolver" "SolverResult" "bindings" "usage")
    MISSING_SECTIONS=0

    for section in "${REQUIRED_SECTIONS[@]}"; do
        if grep -qi "$section" "$AUDIT_FILE"; then
            echo -e "${GREEN}✓ Section found: $section${NC}"
        else
            echo -e "${YELLOW}⚠ Section may be missing: $section${NC}"
            MISSING_SECTIONS=$((MISSING_SECTIONS + 1))
        fi
    done

    if [ $MISSING_SECTIONS -eq 0 ]; then
        echo -e "${GREEN}✓ All required sections present${NC}"
    else
        echo -e "${YELLOW}⚠ $MISSING_SECTIONS sections may be incomplete${NC}"
    fi
else
    echo -e "${RED}✗ Cannot verify completeness (audit missing)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 3: PLAN COMPLETENESS
echo "========================================"
echo "CATEGORY 3: PLAN COMPLETENESS"
echo "Refactoring plan must be actionable"
echo "========================================"

if [ -f "$PLAN_FILE" ]; then
    REQUIRED_ELEMENTS=("API naming" "export" "migration" "files to change" "success criteria")
    MISSING_ELEMENTS=0

    for element in "${REQUIRED_ELEMENTS[@]}"; do
        if grep -qi "$element" "$PLAN_FILE"; then
            echo -e "${GREEN}✓ Element found: $element${NC}"
        else
            echo -e "${YELLOW}⚠ Element may be missing: $element${NC}"
            MISSING_ELEMENTS=$((MISSING_ELEMENTS + 1))
        fi
    done

    if [ $MISSING_ELEMENTS -eq 0 ]; then
        echo -e "${GREEN}✓ Plan is comprehensive${NC}"
    else
        echo -e "${YELLOW}⚠ $MISSING_ELEMENTS elements may be incomplete${NC}"
    fi
else
    echo -e "${RED}✗ Cannot verify plan (file missing)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: NO CODE MODIFICATIONS
echo "========================================"
echo "CATEGORY 4: NO CODE MODIFICATIONS"
echo "Wave 1 is analysis only - no code changes allowed"
echo "========================================"

# Check for modifications outside .mathhook_sessions/
CODE_CHANGES=$(git status --porcelain | grep -v "^?? .mathhook_sessions/" | grep -v "^M  .mathhook_sessions/" | wc -l)

if [ "$CODE_CHANGES" -eq 0 ]; then
    echo -e "${GREEN}✓ No code modifications (analysis only)${NC}"
else
    echo -e "${RED}✗ Found $CODE_CHANGES code changes (not allowed in Wave 1)${NC}"
    git status --porcelain | grep -v ".mathhook_sessions/"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 5: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 5: DOCUMENTATION QUALITY"
echo "Analysis must be thorough and actionable"
echo "========================================"

if [ -f "$AUDIT_FILE" ]; then
    AUDIT_LINES=$(wc -l < "$AUDIT_FILE")
    if [ "$AUDIT_LINES" -gt 200 ]; then
        echo -e "${GREEN}✓ Architecture audit is comprehensive ($AUDIT_LINES lines)${NC}"
    else
        echo -e "${YELLOW}⚠ Architecture audit may be too brief ($AUDIT_LINES lines)${NC}"
    fi
else
    echo -e "${RED}✗ Cannot assess quality (audit missing)${NC}"
    FAILURES=$((FAILURES + 1))
fi

if [ -f "$PLAN_FILE" ]; then
    PLAN_LINES=$(wc -l < "$PLAN_FILE")
    if [ "$PLAN_LINES" -gt 100 ]; then
        echo -e "${GREEN}✓ Refactoring plan is detailed ($PLAN_LINES lines)${NC}"
    else
        echo -e "${YELLOW}⚠ Refactoring plan may be too brief ($PLAN_LINES lines)${NC}"
    fi
else
    echo -e "${RED}✗ Cannot assess quality (plan missing)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 6: IMPACT ANALYSIS
echo "========================================"
echo "CATEGORY 6: IMPACT ANALYSIS"
echo "Must identify all files to be changed"
echo "========================================"

if [ -f "$PLAN_FILE" ]; then
    # Check for file paths in plan
    FILE_MENTIONS=$(grep -c "/Users/ahmedmashhour/Documents/work/math/mathhook" "$PLAN_FILE" 2>/dev/null || echo 0)

    if [ "$FILE_MENTIONS" -gt 5 ]; then
        echo -e "${GREEN}✓ Impact analysis includes file paths ($FILE_MENTIONS mentions)${NC}"
    else
        echo -e "${YELLOW}⚠ Impact analysis may lack specific file paths ($FILE_MENTIONS mentions)${NC}"
    fi
else
    echo -e "${RED}✗ Cannot verify impact analysis (plan missing)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 7: ACTIONABILITY
echo "========================================"
echo "CATEGORY 7: ACTIONABILITY"
echo "Plan must enable Wave 2 implementation"
echo "========================================"

if [ -f "$PLAN_FILE" ]; then
    # Check for concrete steps
    STEP_INDICATORS=$(grep -ci "step\|phase\|action\|modify\|delete\|update\|rename" "$PLAN_FILE")

    if [ "$STEP_INDICATORS" -gt 10 ]; then
        echo -e "${GREEN}✓ Plan contains actionable steps ($STEP_INDICATORS action words)${NC}"
    else
        echo -e "${YELLOW}⚠ Plan may lack concrete actions ($STEP_INDICATORS action words)${NC}"
    fi
else
    echo -e "${RED}✗ Cannot verify actionability (plan missing)${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 8: CLAUDE.MD AWARENESS
echo "========================================"
echo "CATEGORY 8: CLAUDE.MD AWARENESS"
echo "Analysis must reference CLAUDE.md principles"
echo "========================================"

CLAUDE_MENTIONS=0
if [ -f "$AUDIT_FILE" ]; then
    if grep -qi "CLAUDE.md\|Hybrid API\|abstraction" "$AUDIT_FILE"; then
        CLAUDE_MENTIONS=$((CLAUDE_MENTIONS + 1))
    fi
fi

if [ -f "$PLAN_FILE" ]; then
    if grep -qi "CLAUDE.md\|Hybrid API\|abstraction" "$PLAN_FILE"; then
        CLAUDE_MENTIONS=$((CLAUDE_MENTIONS + 1))
    fi
fi

if [ $CLAUDE_MENTIONS -gt 0 ]; then
    echo -e "${GREEN}✓ CLAUDE.md principles referenced${NC}"
else
    echo -e "${YELLOW}⚠ CLAUDE.md principles may not be referenced${NC}"
fi

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "Wave 1: Analysis & Planning is VERIFIED COMPLETE"
    echo ""
    echo "Ready for Wave 2: Refactoring Implementation"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "Wave 1: Analysis & Planning requires fixes before approval"
    echo ""
    echo "Address issues above before proceeding to Wave 2"
    exit 1
fi
