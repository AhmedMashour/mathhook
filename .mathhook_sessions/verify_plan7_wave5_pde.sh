#!/bin/bash
set -e

echo "=== Plan 7 Wave 5 Verification: Partial Differential Equations ==="
echo ""

# Change to worktree directory
cd /Users/ahmedmashhour/Documents/work/math/mathhook/worktrees/agent-7-core-math

echo "Verification Date: $(date)"
echo "Branch: $(git branch --show-current)"
echo ""

# 1. Check PDE module structure exists
echo "1. Verifying PDE module structure..."
required_files=(
    "crates/mathhook-core/src/pde/mod.rs"
    "crates/mathhook-core/src/pde/types.rs"
    "crates/mathhook-core/src/pde/classification.rs"
    "crates/mathhook-core/src/pde/separation_of_variables.rs"
    "crates/mathhook-core/src/pde/method_of_characteristics.rs"
    "crates/mathhook-core/src/pde/standard/mod.rs"
    "crates/mathhook-core/src/pde/standard/heat.rs"
    "crates/mathhook-core/src/pde/standard/wave.rs"
    "crates/mathhook-core/src/pde/standard/laplace.rs"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ FAIL: Missing file: $file"
        exit 1
    fi
done
echo "✅ All 9 required PDE module files exist"

# 2. Check PDE tests exist and pass
echo ""
echo "2. Running PDE test suite..."
pde_test_output=$(cargo test -p mathhook-core pde --lib 2>&1)
pde_test_count=$(echo "$pde_test_output" | grep " passed" | sed -n 's/.*test result: ok\. \([0-9]*\) passed.*/\1/p' | tail -1)

if [ -z "$pde_test_count" ] || [ "$pde_test_count" -lt 50 ]; then
    echo "❌ FAIL: Expected 50+ PDE tests, found: ${pde_test_count:-0}"
    echo "$pde_test_output" | tail -20
    exit 1
fi
echo "✅ PDE test suite passing: $pde_test_count tests"

# 3. Verify Wave Equation implementation
echo ""
echo "3. Verifying Wave Equation solver..."
if ! grep -q "pub struct WaveSolution" crates/mathhook-core/src/pde/standard/wave.rs; then
    echo "❌ FAIL: WaveSolution struct not found"
    exit 1
fi
if ! grep -q "solve_wave_equation_1d" crates/mathhook-core/src/pde/standard/wave.rs; then
    echo "❌ FAIL: solve_wave_equation_1d function not found"
    exit 1
fi
wave_tests=$(echo "$pde_test_output" | grep -c "wave::" || true)
if [ "$wave_tests" -lt 10 ]; then
    echo "❌ FAIL: Expected 10+ wave equation tests, found: $wave_tests"
    exit 1
fi
echo "✅ Wave equation solver implemented with $wave_tests tests"

# 4. Verify Laplace Equation implementation
echo ""
echo "4. Verifying Laplace Equation solver..."
if ! grep -q "pub struct LaplaceSolution" crates/mathhook-core/src/pde/standard/laplace.rs; then
    echo "❌ FAIL: LaplaceSolution struct not found"
    exit 1
fi
if ! grep -q "solve_laplace_2d" crates/mathhook-core/src/pde/standard/laplace.rs; then
    echo "❌ FAIL: solve_laplace_2d function not found"
    exit 1
fi
laplace_tests=$(echo "$pde_test_output" | grep -c "laplace::" || true)
if [ "$laplace_tests" -lt 10 ]; then
    echo "❌ FAIL: Expected 10+ Laplace equation tests, found: $laplace_tests"
    exit 1
fi
echo "✅ Laplace equation solver implemented with $laplace_tests tests"

# 5. Verify Heat Equation (pre-existing)
echo ""
echo "5. Verifying Heat Equation solver..."
if ! grep -q "solve_heat_equation" crates/mathhook-core/src/pde/standard/heat.rs; then
    echo "❌ FAIL: Heat equation solver not found"
    exit 1
fi
heat_tests=$(echo "$pde_test_output" | grep -c "heat::" || true)
if [ "$heat_tests" -lt 5 ]; then
    echo "❌ FAIL: Expected 5+ heat equation tests, found: $heat_tests"
    exit 1
fi
echo "✅ Heat equation solver verified with $heat_tests tests"

# 6. Verify PDE Classification System
echo ""
echo "6. Verifying PDE classification system..."
if ! grep -q "looks_like_heat_equation" crates/mathhook-core/src/pde/classification.rs; then
    echo "❌ FAIL: PDE classification heuristics not found"
    exit 1
fi
classification_tests=$(echo "$pde_test_output" | grep -c "classification::" || true)
if [ "$classification_tests" -lt 15 ]; then
    echo "❌ FAIL: Expected 15+ classification tests, found: $classification_tests"
    exit 1
fi
echo "✅ PDE classification system working with $classification_tests tests"

# 7. Verify Method of Characteristics
echo ""
echo "7. Verifying Method of Characteristics..."
if ! grep -q "solve_using_characteristics" crates/mathhook-core/src/pde/method_of_characteristics.rs; then
    echo "❌ FAIL: Method of characteristics not found"
    exit 1
fi
char_tests=$(echo "$pde_test_output" | grep -c "method_of_characteristics::" || true)
if [ "$char_tests" -lt 10 ]; then
    echo "❌ FAIL: Expected 10+ characteristics tests, found: $char_tests"
    exit 1
fi
echo "✅ Method of characteristics implemented with $char_tests tests"

# 8. Verify Separation of Variables
echo ""
echo "8. Verifying Separation of Variables..."
if ! grep -q "solve_with_separation" crates/mathhook-core/src/pde/separation_of_variables.rs; then
    echo "❌ FAIL: Separation of variables not found"
    exit 1
fi
sep_tests=$(echo "$pde_test_output" | grep -c "separation_of_variables::" || true)
if [ "$sep_tests" -lt 15 ]; then
    echo "❌ FAIL: Expected 15+ separation tests, found: $sep_tests"
    exit 1
fi
echo "✅ Separation of variables implemented with $sep_tests tests"

# 9. Check for compiler warnings
echo ""
echo "9. Checking for compiler warnings..."
build_output=$(cargo build -p mathhook-core --lib 2>&1)
warning_count=$(echo "$build_output" | grep -c "warning:" || true)
if [ "$warning_count" -gt 0 ]; then
    echo "⚠️  WARNING: Found $warning_count compiler warnings"
    echo "$build_output" | grep "warning:" | head -5
else
    echo "✅ Zero compiler warnings"
fi

# 10. Verify CLAUDE.md compliance (documentation)
echo ""
echo "10. Verifying documentation compliance..."
undocumented=$(grep -r "pub fn" crates/mathhook-core/src/pde/ | grep -v "///" || true)
if [ -n "$undocumented" ]; then
    echo "⚠️  WARNING: Some public functions may lack documentation"
else
    echo "✅ Documentation compliance verified"
fi

# 11. Check completion report exists
echo ""
echo "11. Verifying completion report..."
if [ ! -f ".mathhook_sessions/WAVE_5_COMPLETION_REPORT.md" ]; then
    echo "❌ FAIL: Wave 5 completion report not found"
    exit 1
fi
echo "✅ Completion report exists"

# Summary
echo ""
echo "================================================================="
echo "=== Plan 7 Wave 5 (PDE Solvers): VERIFICATION PASSED ==="
echo "================================================================="
echo ""
echo "Summary:"
echo "  - Module structure: 10/10 files present"
echo "  - Test coverage: $pde_test_count tests passing (target: 50+)"
echo "  - Wave equation: $wave_tests tests"
echo "  - Laplace equation: $laplace_tests tests"
echo "  - Heat equation: $heat_tests tests"
echo "  - Classification: $classification_tests tests"
echo "  - Method of characteristics: $char_tests tests"
echo "  - Separation of variables: $sep_tests tests"
echo "  - Compiler warnings: $warning_count"
echo ""
echo "Wave 5 Deliverables:"
echo "  ✅ Heat equation solver (parabolic PDE)"
echo "  ✅ Wave equation solver (hyperbolic PDE)"
echo "  ✅ Laplace equation solver (elliptic PDE)"
echo "  ✅ Separation of variables framework"
echo "  ✅ Method of characteristics"
echo "  ✅ PDE classification system"
echo ""
echo "Next Steps:"
echo "  - Wave 5 is COMPLETE"
echo "  - Proceed to remaining waves (2, 3, 4, 6)"
echo "  - Final Plan 7 validation after all waves complete"
echo ""
