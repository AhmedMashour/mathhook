#!/bin/bash
# File: .mathhook_sessions/verify_wave_2.sh

cd /Users/ahmedmashhour/Documents/work/math/mathhook

echo "=========================================="
echo "WAVE 2 VERIFICATION - GROUND TRUTH CHECK"
echo "Date: $(date)"
echo "=========================================="
echo ""

# P1-1: Registry Refactor
echo "P1-1: Registry Refactor"
echo "Hardcoded matches in simplify/functions.rs:"
rg 'match\s+(name|func_name)\s*\{' crates/mathhook-core/src/simplify/functions.rs 2>/dev/null | wc -l | xargs
echo "Hardcoded matches in chain_rule.rs:"
rg 'match\s+name\s*\{' crates/mathhook-core/src/calculus/derivatives/chain_rule.rs 2>/dev/null | wc -l | xargs
cargo test -p mathhook-core --quiet 2>&1 | grep "test result:" | tail -1
echo ""

# P1-2: Complex Arithmetic
echo "P1-2: Complex Arithmetic"
cargo test -p mathhook-core complex --quiet 2>&1 | grep "test result:" | tail -1
echo "Methods implemented:"
rg "pub fn (real|imag|conjugate|abs|arg|to_polar|from_polar)" crates/mathhook-core/src/algebra/complex.rs 2>/dev/null | wc -l | xargs
echo ""

# P1-3: Integration Table
echo "P1-3: Integration Table"
cargo test -p mathhook-core integration --quiet 2>&1 | grep "test result:" | tail -1
echo "By parts module exists:"
test -f crates/mathhook-core/src/calculus/integrals/by_parts.rs && echo "YES" || echo "NO"
echo ""

# P1-4: System Solver
echo "P1-4: System Solver"
cargo test -p mathhook-core system --quiet 2>&1 | grep "test result:" | tail -1
echo ""

# P1-5: SymPy Validation
echo "P1-5: SymPy Validation Suite"
cargo test -p mathhook-core sympy_validation --quiet 2>&1 | grep "test result:" | tail -1
echo "Validation test count:"
rg "#\[test\]" crates/mathhook-core/tests/sympy_validation/ 2>/dev/null | wc -l | xargs
echo ""

echo "=========================================="
echo "END VERIFICATION"
echo "=========================================="
