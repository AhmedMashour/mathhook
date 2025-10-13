# WAVE 2 VERIFICATION CHECKERS
## Objective Truth System - Zero False Positives (P1 Tasks)

**Purpose**: Automated, repeatable verification of Wave 2 (P1 High Priority) completion
**Date Created**: 2025-10-13
**Current Wave 2 Status**: COMPUTED BELOW

---

## VERIFICATION METHODOLOGY

### Core Principles (Same as Wave 1):
1. **Only automated test results count** - No subjective assessment
2. **Exact numeric criteria** - Clear pass/fail thresholds
3. **Repeatable commands** - Same command always gives same result
4. **No agent reports without verification** - Trust but verify
5. **Ground truth only** - Run actual commands, capture actual output
6. **Mathematical correctness** - Validate against SymPy/Symbolica when possible

---

## P1-1: REFACTOR HARDCODED FUNCTIONS TO REGISTRY

### Completion Criteria:
- ✅ Zero hardcoded function matches in `simplify/functions.rs`
- ✅ Zero hardcoded function matches in `calculus/derivatives/chain_rule.rs`
- ✅ All functions use `UniversalFunctionRegistry`
- ✅ Adding new function requires only registry registration
- ✅ All existing tests still pass (no regressions)

### Verification Commands:
```bash
# Check for hardcoded function name matching in simplify
rg 'match\s+(name|func_name)\s*\{' crates/mathhook-core/src/simplify/functions.rs

# Check for hardcoded function name matching in derivatives
rg 'match\s+name\s*\{' crates/mathhook-core/src/calculus/derivatives/chain_rule.rs

# Verify test suite health
cargo test -p mathhook-core --quiet 2>&1 | grep "test result:"
```

### Success Threshold:
- **COMPLETE**:
  - 0 hardcoded matches in simplify/functions.rs
  - 0 hardcoded matches in calculus/derivatives/chain_rule.rs
  - All tests passing (>= baseline test count)
- **IN_PROGRESS**: Hardcoded matches still exist
- **BLOCKED**: Tests failing after refactoring

### Current Actual Status:
```
Last Verified: NEVER
Status: NOT_STARTED
Hardcoded matches in simplify/functions.rs: Unknown (need to check)
Hardcoded matches in chain_rule.rs: Unknown (need to check)
```

---

## P1-2: COMPLETE COMPLEX NUMBER ARITHMETIC

### Completion Criteria:
- ✅ All complex arithmetic operations implemented (add, mul, div)
- ✅ `real()`, `imag()`, `conjugate()` methods working
- ✅ `abs()` and `arg()` methods working
- ✅ Polar conversions (`to_polar()`, `from_polar()`) working
- ✅ All complex number tests passing

### Verification Commands:
```bash
# Check if complex arithmetic tests exist and pass
cargo test -p mathhook-core complex --quiet 2>&1 | grep "test result:"

# Check specific methods exist
rg "pub fn (real|imag|conjugate|abs|arg|to_polar|from_polar)" crates/mathhook-core/src/algebra/complex.rs
```

### Success Threshold:
- **COMPLETE**:
  - All complex arithmetic tests passing (20+ tests)
  - All 7 required methods implemented
  - Doctest in algebra/complex.rs passing
- **IN_PROGRESS**: Some methods missing or tests failing
- **BLOCKED**: Compilation fails

### Current Actual Status:
```
Last Verified: NEVER
Status: NOT_STARTED
Complex tests: Unknown
Methods implemented: Unknown
```

---

## P1-3: COMPLETE INTEGRATION TABLE

### Completion Criteria:
- ✅ All elementary function integrals implemented
- ✅ Integration by parts working for simple cases
- ✅ Derivative of integral returns original expression
- ✅ All integration tests passing (30+ tests)
- ✅ Cross-validated against SymPy

### Verification Commands:
```bash
# Check integration tests
cargo test -p mathhook-core integration --quiet 2>&1 | grep "test result:"

# Check for elementary integral implementations
rg "// ∫" crates/mathhook-core/src/calculus/integrals/basic.rs | wc -l

# Verify integration by parts exists
test -f crates/mathhook-core/src/calculus/integrals/by_parts.rs && echo "EXISTS" || echo "MISSING"
```

### Success Threshold:
- **COMPLETE**:
  - 30+ integration tests passing
  - All elementary integrals (sin, cos, exp, 1/x, x^n, etc.) working
  - Integration by parts module exists and functional
- **IN_PROGRESS**: Some integrals missing or tests failing
- **BLOCKED**: Compilation fails

### Current Actual Status:
```
Last Verified: NEVER
Status: NOT_STARTED
Integration tests: Unknown
Elementary integrals: Unknown
By parts module: Unknown
```

---

## P1-4: SYSTEM EQUATION SOLVER

### Completion Criteria:
- ✅ Can solve 2x2, 3x3, NxN linear systems
- ✅ Detects inconsistent systems (no solution)
- ✅ Detects underdetermined systems (infinite solutions)
- ✅ All system solver tests passing (15+ tests)
- ✅ Integration with Matrix module

### Verification Commands:
```bash
# Check system solver tests
cargo test -p mathhook-core system_solver --quiet 2>&1 | grep "test result:"
cargo test -p mathhook-core linear_system --quiet 2>&1 | grep "test result:"

# Check if SystemSolver exists
rg "pub struct SystemSolver" crates/mathhook-core/src/algebra/solvers/
```

### Success Threshold:
- **COMPLETE**:
  - 15+ system solver tests passing
  - Can solve 2x2, 3x3 systems correctly
  - Handles edge cases (no solution, infinite solutions)
- **IN_PROGRESS**: Partial implementation or tests failing
- **BLOCKED**: Compilation fails

### Current Actual Status:
```
Last Verified: NEVER
Status: NOT_STARTED
System solver tests: Unknown
Implementation: Unknown
```

---

## P1-5: SYMPY VALIDATION SUITE

### Completion Criteria:
- ✅ 100+ validation tests comparing MathHook to SymPy
- ✅ Categories covered: Simplification, Derivatives, Integration, Solving, Special Functions
- ✅ All validation tests passing (or discrepancies documented)
- ✅ Continuous expansion framework established

### Verification Commands:
```bash
# Check SymPy validation tests
cargo test -p mathhook-core sympy_validation --quiet 2>&1 | grep "test result:"

# Count validation tests
find crates/mathhook-core/tests/sympy_validation -name "*.rs" -exec grep -l "#\[test\]" {} \; | wc -l

# Get test count
rg "#\[test\]" crates/mathhook-core/tests/sympy_validation/ | wc -l
```

### Success Threshold:
- **COMPLETE**:
  - 100+ validation tests created
  - Tests cover 5 categories (20+ tests each)
  - All tests passing (or documented discrepancies)
- **IN_PROGRESS**: < 100 tests or tests failing without documentation
- **BLOCKED**: Test infrastructure not set up

### Current Actual Status:
```
Last Verified: NEVER
Status: NOT_STARTED
Validation tests: 0
Coverage: None
```

---

## OVERALL WAVE 2 CHECKER

### Completion Criteria:
- ✅ P1-1: COMPLETE (Registry refactor)
- ✅ P1-2: COMPLETE (Complex arithmetic)
- ✅ P1-3: COMPLETE (Integration table)
- ✅ P1-4: COMPLETE (System solver)
- ✅ P1-5: COMPLETE (SymPy validation - 100+ tests)

### Master Verification Script:
```bash
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
```

### Current Overall Status:
```
Last Full Verification: NEVER (Need to run master script)
Known Status:
  - P1-1: NOT_STARTED (need baseline check)
  - P1-2: NOT_STARTED (need baseline check)
  - P1-3: NOT_STARTED (need baseline check)
  - P1-4: NOT_STARTED (need baseline check)
  - P1-5: NOT_STARTED (need baseline check)

WAVE 2 OVERALL: NOT_STARTED (0% estimated)
```

---

## USAGE INSTRUCTIONS FOR ORCHESTRATOR

### Before Making ANY Completion Claims:

1. **Run the verification command** for that specific task
2. **Parse the output** using the status parser
3. **Compare against success threshold** (exact numeric match)
4. **Update this file** with actual results and timestamp
5. **Only then** report status to user

### Before Claiming "Wave 2 Complete":

1. **Run master verification script**: `.mathhook_sessions/verify_wave_2.sh`
2. **Verify ALL tasks meet success thresholds** (no exceptions)
3. **Document all output** in this file
4. **Update timestamp** of verification
5. **Only then** declare Wave 2 complete

### Zero False Positives Rule:

- ❌ **Never trust agent reports without verification**
- ❌ **Never estimate percentages without running tests**
- ❌ **Never claim complete without meeting exact threshold**
- ✅ **Always run verification command before status update**
- ✅ **Always capture actual output in documented files**
- ✅ **Always timestamp verification results**

---

## VERIFICATION COMMAND SUMMARY

```bash
# Quick verification of all tasks
cd /Users/ahmedmashhour/Documents/work/math/mathhook

# P1-1: Registry refactor
rg 'match\s+(name|func_name)\s*\{' crates/mathhook-core/src/simplify/functions.rs | wc -l
cargo test -p mathhook-core --quiet | grep "test result:" | tail -1

# P1-2: Complex arithmetic
cargo test -p mathhook-core complex --quiet | grep "test result:" | tail -1

# P1-3: Integration
cargo test -p mathhook-core integration --quiet | grep "test result:" | tail -1

# P1-4: System solver
cargo test -p mathhook-core system --quiet | grep "test result:" | tail -1

# P1-5: SymPy validation
cargo test -p mathhook-core sympy_validation --quiet | grep "test result:" | tail -1
rg "#\[test\]" crates/mathhook-core/tests/sympy_validation/ | wc -l
```

---

**GROUND TRUTH ONLY. NO ESTIMATES. NO ASSUMPTIONS.**

**Mathematical Correctness Validation**: All implementations MUST be validated against SymPy for correctness before claiming completion.
