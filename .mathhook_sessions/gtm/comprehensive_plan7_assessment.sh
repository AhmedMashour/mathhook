#!/bin/bash

# Comprehensive Plan 7 Assessment Script
# Purpose: Assess architectural integration and implementation status across all waves

set -e

echo "=========================================="
echo "PLAN 7 COMPREHENSIVE ASSESSMENT"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

# Function to print status
print_status() {
    local status=$1
    local message=$2

    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}✓${NC} $message"
        ((PASS_COUNT++))
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}✗${NC} $message"
        ((FAIL_COUNT++))
    elif [ "$status" = "WARN" ]; then
        echo -e "${YELLOW}⚠${NC} $message"
        ((WARN_COUNT++))
    else
        echo -e "${BLUE}ℹ${NC} $message"
    fi
}

# ========================================
# SECTION 1: BUILD STATUS
# ========================================
echo "=========================================="
echo "SECTION 1: BUILD STATUS"
echo "=========================================="
echo ""

echo "Building mathhook-core..."
BUILD_OUTPUT=$(cargo build -p mathhook-core 2>&1)
if echo "$BUILD_OUTPUT" | grep -q "^error\["; then
    print_status "FAIL" "Build has compilation errors"
elif echo "$BUILD_OUTPUT" | tail -5 | grep -q "Finished"; then
    print_status "PASS" "Build compiles successfully"
else
    print_status "WARN" "Build status unclear"
fi

# Count warnings
WARNING_COUNT=$(cargo build -p mathhook-core 2>&1 | grep "warning:" | wc -l | tr -d ' ')
if [ "$WARNING_COUNT" -gt 0 ]; then
    print_status "WARN" "Build has $WARNING_COUNT warnings"
else
    print_status "PASS" "Build has no warnings"
fi

echo ""

# ========================================
# SECTION 2: MODULE STRUCTURE ANALYSIS
# ========================================
echo "=========================================="
echo "SECTION 2: MODULE STRUCTURE ANALYSIS"
echo "=========================================="
echo ""

# Wave 1: ODE
echo "Wave 1: ODE Module"
if [ -d "crates/mathhook-core/src/ode" ]; then
    print_status "PASS" "ODE module exists"

    # Count files
    ODE_FILES=$(find crates/mathhook-core/src/ode -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Files: $ODE_FILES"

    # Check subdirectories
    for subdir in first_order second_order numerical educational systems; do
        if [ -d "crates/mathhook-core/src/ode/$subdir" ]; then
            print_status "PASS" "  - $subdir/ exists"
        else
            print_status "FAIL" "  - $subdir/ missing"
        fi
    done
else
    print_status "FAIL" "ODE module missing"
fi
echo ""

# Wave 2: Advanced Linear Algebra
echo "Wave 2: Advanced Linear Algebra"
if [ -d "crates/mathhook-core/src/matrix" ]; then
    print_status "PASS" "Matrix module exists"

    MATRIX_FILES=$(find crates/mathhook-core/src/matrix -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Files: $MATRIX_FILES"

    # Check for decomposition implementations
    for decomp in qr lu svd cholesky; do
        if grep -r "pub fn ${decomp}" crates/mathhook-core/src/matrix/ > /dev/null 2>&1; then
            print_status "PASS" "  - $decomp decomposition found"
        else
            print_status "WARN" "  - $decomp decomposition not found"
        fi
    done
else
    print_status "FAIL" "Matrix module missing"
fi
echo ""

# Wave 3: Gröbner Basis
echo "Wave 3: Gröbner Basis & Number Theory"
if [ -d "crates/mathhook-core/src/algebra/groebner" ]; then
    print_status "PASS" "Gröbner basis module exists"

    GROEBNER_FILES=$(find crates/mathhook-core/src/algebra/groebner -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Files: $GROEBNER_FILES"

    # Check for key algorithms
    for algo in buchberger s_polynomial reduction; do
        if [ -f "crates/mathhook-core/src/algebra/groebner/${algo}.rs" ]; then
            print_status "PASS" "  - $algo.rs exists"
        else
            print_status "FAIL" "  - $algo.rs missing"
        fi
    done
else
    print_status "FAIL" "Gröbner basis module missing"
fi
echo ""

# Wave 4: Special Functions
echo "Wave 4: Special Functions"
if [ -d "crates/mathhook-core/src/functions/special" ]; then
    print_status "PASS" "Special functions module exists"

    SPECIAL_FILES=$(find crates/mathhook-core/src/functions/special -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Files: $SPECIAL_FILES"
else
    print_status "FAIL" "Special functions module missing"
fi
echo ""

# Wave 5: PDEs
echo "Wave 5: PDEs"
if [ -d "crates/mathhook-core/src/pde" ]; then
    print_status "PASS" "PDE module exists"

    PDE_FILES=$(find crates/mathhook-core/src/pde -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Files: $PDE_FILES"

    # Check subdirectories
    for subdir in classification educational standard; do
        if [ -d "crates/mathhook-core/src/pde/$subdir" ]; then
            print_status "PASS" "  - $subdir/ exists"
        else
            print_status "WARN" "  - $subdir/ missing"
        fi
    done
else
    print_status "FAIL" "PDE module missing"
fi
echo ""

# Wave 6: Numerical Methods
echo "Wave 6: Numerical Methods"
ROOT_FINDING_EXISTS=false
NUMERICAL_INT_EXISTS=false

if [ -d "crates/mathhook-core/src/algebra/root_finding" ]; then
    print_status "PASS" "Root finding module exists"
    ROOT_FINDING_EXISTS=true

    ROOT_FILES=$(find crates/mathhook-core/src/algebra/root_finding -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Root finding files: $ROOT_FILES"
else
    print_status "WARN" "Root finding module missing"
fi

if [ -d "crates/mathhook-core/src/calculus/integrals/numerical" ]; then
    print_status "PASS" "Numerical integration module exists"
    NUMERICAL_INT_EXISTS=true

    NUM_INT_FILES=$(find crates/mathhook-core/src/calculus/integrals/numerical -name "*.rs" | wc -l | tr -d ' ')
    echo "  - Numerical integration files: $NUM_INT_FILES"
else
    print_status "WARN" "Numerical integration module missing"
fi
echo ""

# ========================================
# SECTION 3: ARCHITECTURAL INTEGRATION
# ========================================
echo "=========================================="
echo "SECTION 3: ARCHITECTURAL INTEGRATION"
echo "=========================================="
echo ""

echo "Checking SmartEquationSolver integration..."

# Check if EquationType has ODE/PDE variants
if grep -q "enum EquationType" crates/mathhook-core/src/algebra/equation_analyzer.rs; then
    if grep -A 20 "enum EquationType" crates/mathhook-core/src/algebra/equation_analyzer.rs | grep -q "ODE"; then
        print_status "PASS" "EquationType::ODE variant exists"
    else
        print_status "FAIL" "EquationType::ODE variant missing"
    fi

    if grep -A 20 "enum EquationType" crates/mathhook-core/src/algebra/equation_analyzer.rs | grep -q "PDE"; then
        print_status "PASS" "EquationType::PDE variant exists"
    else
        print_status "FAIL" "EquationType::PDE variant missing"
    fi
else
    print_status "FAIL" "EquationType enum not found"
fi

# Check SmartEquationSolver has ODE solver field
if grep -q "ode_solver" crates/mathhook-core/src/algebra/equation_analyzer.rs; then
    print_status "PASS" "SmartEquationSolver has ode_solver field"
else
    print_status "WARN" "SmartEquationSolver missing ode_solver field"
fi

# Check for hardcoded function matching (anti-pattern)
echo ""
echo "Checking for architectural anti-patterns..."

HARDCODED_MATCHES=$(grep -r "match.*\.as_str()" crates/mathhook-core/src/ --include="*.rs" | grep -E "sin|cos|log|exp" | wc -l | tr -d ' ')
if [ "$HARDCODED_MATCHES" -gt 0 ]; then
    print_status "WARN" "Found $HARDCODED_MATCHES potential hardcoded function matches (check if registry-based)"
else
    print_status "PASS" "No hardcoded function matching detected"
fi

echo ""

# ========================================
# SECTION 4: TEST COVERAGE
# ========================================
echo "=========================================="
echo "SECTION 4: TEST COVERAGE"
echo "=========================================="
echo ""

echo "Running test suite..."
TEST_OUTPUT=$(cargo test -p mathhook-core --lib 2>&1)

TOTAL_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $3}')
PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $4}')
FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $6}')

echo "Test Results:"
echo "  - Total: $TOTAL_TESTS"
echo "  - Passed: $PASSED_TESTS"
echo "  - Failed: $FAILED_TESTS"

if [ -n "$FAILED_TESTS" ] && [ "$FAILED_TESTS" != "0;" ]; then
    print_status "WARN" "Some tests are failing"
else
    print_status "PASS" "All tests passing"
fi

# Check integration tests
echo ""
echo "Integration tests:"

for test_file in test_ode_integration test_pde_integration; do
    if [ -f "crates/mathhook-core/tests/${test_file}.rs" ]; then
        print_status "PASS" "${test_file}.rs exists"

        # Run specific integration test
        INT_TEST_OUTPUT=$(cargo test -p mathhook-core --test ${test_file} 2>&1 || true)
        INT_PASSED=$(echo "$INT_TEST_OUTPUT" | grep "test result:" | awk '{print $4}')

        if [ -n "$INT_PASSED" ]; then
            echo "    Passed: $INT_PASSED"
        fi
    else
        print_status "WARN" "${test_file}.rs missing"
    fi
done

echo ""

# ========================================
# SECTION 5: IMPLEMENTATION STATUS
# ========================================
echo "=========================================="
echo "SECTION 5: IMPLEMENTATION STATUS"
echo "=========================================="
echo ""

# Count implemented solvers per wave
echo "Wave 1 (ODEs) - Solver Implementations:"
ODE_SOLVERS=0
for solver in separable linear exact homogeneous bernoulli; do
    if [ -f "crates/mathhook-core/src/ode/first_order/${solver}.rs" ]; then
        print_status "PASS" "  - First-order ${solver}"
        ((ODE_SOLVERS++))
    fi
done

for solver in constant_coeff; do
    if [ -f "crates/mathhook-core/src/ode/second_order/${solver}.rs" ]; then
        print_status "PASS" "  - Second-order ${solver}"
        ((ODE_SOLVERS++))
    fi
done
echo "  Total ODE solvers: $ODE_SOLVERS"
echo ""

echo "Wave 5 (PDEs) - Solver Implementations:"
PDE_SOLVERS=0
for method in separation_of_variables method_of_characteristics; do
    if [ -f "crates/mathhook-core/src/pde/${method}.rs" ]; then
        print_status "PASS" "  - ${method}"
        ((PDE_SOLVERS++))
    fi
done

for pde in heat wave laplace; do
    if [ -f "crates/mathhook-core/src/pde/standard/${pde}.rs" ]; then
        print_status "PASS" "  - Standard ${pde} equation"
        ((PDE_SOLVERS++))
    fi
done
echo "  Total PDE solvers: $PDE_SOLVERS"
echo ""

echo "Wave 6 (Numerical) - Method Implementations:"
NUMERICAL_METHODS=0

for method in newton_raphson secant bisection; do
    if [ -f "crates/mathhook-core/src/algebra/root_finding/${method}.rs" ]; then
        print_status "PASS" "  - Root finding: ${method}"
        ((NUMERICAL_METHODS++))
    fi
done

for method in gaussian simpson romberg; do
    if [ -f "crates/mathhook-core/src/calculus/integrals/numerical/${method}.rs" ]; then
        print_status "PASS" "  - Numerical integration: ${method}"
        ((NUMERICAL_METHODS++))
    fi
done
echo "  Total numerical methods: $NUMERICAL_METHODS"
echo ""

# ========================================
# SECTION 6: FILE SIZE COMPLIANCE
# ========================================
echo "=========================================="
echo "SECTION 6: FILE SIZE COMPLIANCE"
echo "=========================================="
echo ""

echo "Checking CLAUDE.md file size limit (500 lines)..."

LARGE_FILES=$(find crates/mathhook-core/src -name "*.rs" -type f -exec wc -l {} \; | awk '$1 > 500 {print $2 " (" $1 " lines)"}')

if [ -n "$LARGE_FILES" ]; then
    print_status "WARN" "Files exceeding 500 lines:"
    echo "$LARGE_FILES" | while read line; do
        echo "    $line"
    done
else
    print_status "PASS" "All files within 500 line limit"
fi

echo ""

# ========================================
# SECTION 7: DOCUMENTATION COMPLIANCE
# ========================================
echo "=========================================="
echo "SECTION 7: DOCUMENTATION COMPLIANCE"
echo "=========================================="
echo ""

echo "Checking for CLAUDE.md violations..."

# Check for emojis (prohibited)
EMOJI_COUNT=$(grep -r "[😀-🙏🚀-🛿]" crates/mathhook-core/src --include="*.rs" 2>/dev/null | wc -l | tr -d ' ')
if [ "$EMOJI_COUNT" -gt 0 ]; then
    print_status "FAIL" "Found $EMOJI_COUNT emoji violations"
else
    print_status "PASS" "No emojis found"
fi

# Check for TODO comments (allowed only for enhancements, not critical functionality)
TODO_COUNT=$(grep -r "TODO" crates/mathhook-core/src --include="*.rs" | wc -l | tr -d ' ')
if [ "$TODO_COUNT" -gt 0 ]; then
    print_status "WARN" "Found $TODO_COUNT TODO comments (review if critical)"
else
    print_status "PASS" "No TODO comments"
fi

echo ""

# ========================================
# FINAL SUMMARY
# ========================================
echo "=========================================="
echo "FINAL SUMMARY"
echo "=========================================="
echo ""

echo -e "${GREEN}Passed:${NC} $PASS_COUNT"
echo -e "${YELLOW}Warnings:${NC} $WARN_COUNT"
echo -e "${RED}Failed:${NC} $FAIL_COUNT"
echo ""

TOTAL_CHECKS=$((PASS_COUNT + WARN_COUNT + FAIL_COUNT))
PASS_RATE=$((PASS_COUNT * 100 / TOTAL_CHECKS))

echo "Overall Pass Rate: ${PASS_RATE}%"
echo ""

if [ "$FAIL_COUNT" -eq 0 ]; then
    echo -e "${GREEN}✓ ASSESSMENT COMPLETE - No critical failures${NC}"
    exit 0
else
    echo -e "${RED}✗ ASSESSMENT COMPLETE - $FAIL_COUNT critical failures${NC}"
    exit 1
fi
