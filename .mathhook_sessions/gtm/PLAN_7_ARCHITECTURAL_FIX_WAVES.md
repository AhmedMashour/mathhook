# Plan 7 Architectural Fix Waves

**Date**: 2025-10-22
**Purpose**: Define refactoring waves to integrate Plan 7 modules with MathHook's architecture
**Prerequisites**: PLAN_7_FULL_DIFF_ANALYSIS.md

---

## Overview

These waves refactor existing Plan 7 code to integrate with MathHook's unified architecture while preserving mathematical correctness.

**Critical Principle**: Fix → Verify Correctness → Refactor → Verify Correctness Maintained

**Architecture Goals**:
1. Unified equation solving through `SmartEquationSolver` + `EquationAnalyzer`
2. Expression-centric API + Solver object orchestration (hybrid design)
3. Function intelligence through `UniversalFunctionRegistry`
4. Single point of entry: `MathSolver::solve()`

---

## Wave 0A: Build Restoration (PREREQUISITE)

**Duration**: 2-4 hours
**Priority**: CRITICAL (blocks everything else)
**Goal**: Restore green build state

### Tasks

#### Task 1: Fix ODE Educational Demo Compilation (1-2 hours)

**File**: `crates/mathhook-core/examples/ode_educational_demo.rs`
**Error**: Missing `use std::iter::repeat;` import (15 errors)

**Actions**:
1. Add missing import
2. Verify example compiles: `cargo build --example ode_educational_demo`
3. Test example runs: `cargo run --example ode_educational_demo`

#### Task 2: Fix Gröbner Basis Compilation (1-2 hours)

**Files**: `crates/mathhook-core/src/algebra/groebner/*.rs`
**Status**: Commented out in algebra.rs due to compilation errors

**Actions**:
1. Uncomment `pub mod groebner;` in algebra.rs
2. Fix compilation errors in groebner module
3. Verify: `cargo build -p mathhook-core`
4. Run groebner tests: `cargo test -p mathhook-core groebner`

### Success Criteria

```bash
cargo build  # ✅ Success
cargo test   # ✅ All tests pass (get baseline count)
```

### Deliverables

- [ ] Green build state
- [ ] Test baseline count documented
- [ ] All examples compile and run

---

## Wave 0B: Mathematical Correctness Baseline (PREREQUISITE)

**Duration**: 4-6 hours
**Priority**: CRITICAL (before refactoring)
**Goal**: Verify Plan 7 implementations are mathematically correct

### Tasks

#### Task 1: ODE Correctness Verification (2-3 hours)

**Reference**: SymPy ODE solvers (`~/Documents/work/math/sympy/sympy/solvers/ode/`)

**Actions**:
1. Create verification script comparing MathHook vs SymPy ODE solutions
2. Test cases:
   - Separable ODEs: `dy/dx = x*y`
   - Linear first-order: `dy/dx + P(x)*y = Q(x)`
   - Second-order constant coefficients: `y'' + a*y' + b*y = 0`
3. Document any discrepancies
4. Fix mathematical errors BEFORE refactoring

**Script**: `.mathhook_sessions/verify_plan7_ode_correctness.py`

#### Task 2: PDE Correctness Verification (2-3 hours)

**Reference**: SymPy PDE solvers

**Actions**:
1. Verify separation of variables solutions
2. Verify method of characteristics solutions
3. Compare against SymPy for standard PDEs (heat, wave, Laplace)
4. Document correctness

**Script**: `.mathhook_sessions/verify_plan7_pde_correctness.py`

### Success Criteria

```bash
python .mathhook_sessions/verify_plan7_ode_correctness.py  # ✅ 100% match
python .mathhook_sessions/verify_plan7_pde_correctness.py  # ✅ 100% match
```

### Deliverables

- [ ] ODE correctness verified against SymPy
- [ ] PDE correctness verified against SymPy
- [ ] Correctness baseline documented
- [ ] Any bugs fixed BEFORE architectural changes

---

## Wave 1-INT: ODE Integration Refactoring

**Duration**: 12-16 hours
**Priority**: HIGH (user experience)
**Goal**: Integrate ODE module with SmartEquationSolver/EquationAnalyzer

**Dependencies**: Wave 0A (build), Wave 0B (correctness verified)

### Phase 1: Extend EquationAnalyzer (4-5 hours)

#### Task 1: Extend EquationType Enum

**File**: `crates/mathhook-core/src/algebra/equation_analyzer.rs`

**Changes**:
```rust
// Add ODE cases to EquationType:
pub enum EquationType {
    // Existing:
    Linear,
    Quadratic,
    Polynomial { degree: usize },
    MatrixEquation,

    // NEW - Add these:
    OrdinaryDifferential { order: usize, ode_subtype: ODESubtype },
    PartialDifferential { order: usize, pde_subtype: PDESubtype },
}

// Import from ODE module:
use crate::ode::classifier::ODEType as ODESubtype;
use crate::pde::classification::PDEType as PDESubtype;
```

#### Task 2: Integrate ODEClassifier into EquationAnalyzer

**File**: `crates/mathhook-core/src/algebra/equation_analyzer.rs`

**Changes**:
```rust
impl EquationAnalyzer {
    pub fn classify_equation(expr: &Expression, var: &Symbol) -> EquationType {
        // First check if it's a differential equation
        if Self::is_differential_equation(expr) {
            return Self::classify_differential(expr, var);
        }

        // Existing algebraic classification...
    }

    fn is_differential_equation(expr: &Expression) -> bool {
        // Check for derivative notation: d/dx, ∂/∂x, etc.
        // Detect Expression::Derivative or Function("diff", ...)
    }

    fn classify_differential(expr: &Expression, var: &Symbol) -> EquationType {
        let order = Self::differential_order(expr);

        if Self::is_partial_differential(expr) {
            // Delegate to PDE classifier
            let pde_type = crate::pde::classification::PDEClassifier::classify_pde(...);
            EquationType::PartialDifferential { order, pde_subtype: pde_type }
        } else {
            // Delegate to ODE classifier
            let ode_type = crate::ode::classifier::ODEClassifier::classify_first_order(...);
            EquationType::OrdinaryDifferential { order, ode_subtype: ode_type }
        }
    }
}
```

**Tests**:
```rust
#[test]
fn test_classify_separable_ode() {
    let x = symbol!(x);
    let y = symbol!(y);
    let ode = parse_latex(r"\frac{dy}{dx} = x \cdot y").unwrap();

    let eq_type = EquationAnalyzer::classify_equation(&ode, &y);
    assert!(matches!(eq_type, EquationType::OrdinaryDifferential {
        order: 1,
        ode_subtype: ODESubtype::Separable
    }));
}
```

### Phase 2: Integrate with SmartEquationSolver (4-5 hours)

#### Task 1: Add ODE Routing

**File**: `crates/mathhook-core/src/algebra/equation_analyzer.rs`

**Changes**:
```rust
impl SmartEquationSolver {
    pub fn solve_with_equation(
        &mut self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (AlgebraSolverResult, Option<String>) {
        let eq_type = EquationAnalyzer::classify_equation(equation, variable);

        match eq_type {
            // Existing cases:
            EquationType::Linear => { /* ... */ }
            EquationType::Quadratic => { /* ... */ }

            // NEW - Add ODE routing:
            EquationType::OrdinaryDifferential { order, ode_subtype } => {
                self.solve_ode(equation, variable, order, ode_subtype)
            }

            // NEW - Add PDE routing:
            EquationType::PartialDifferential { order, pde_subtype } => {
                self.solve_pde(equation, variable, order, pde_subtype)
            }
        }
    }

    fn solve_ode(
        &mut self,
        equation: &Expression,
        variable: &Symbol,
        order: usize,
        ode_type: ODESubtype,
    ) -> (AlgebraSolverResult, Option<String>) {
        // Extract RHS from equation (dy/dx = rhs)
        let rhs = Self::extract_ode_rhs(equation);

        // Delegate to ODESolver (internal implementation)
        match order {
            1 => {
                let solution = crate::ode::solver::ODESolver::solve_first_order(
                    &rhs, variable, &Self::get_independent_var(equation)
                );

                match solution {
                    Ok(ode_solution) => {
                        let explanation = format!(
                            "Detected {} ODE, solved using {}",
                            ode_solution.metadata.ode_type,
                            ode_solution.metadata.method
                        );
                        (
                            AlgebraSolverResult::SingleSolution(ode_solution.solution),
                            Some(explanation)
                        )
                    }
                    Err(e) => (AlgebraSolverResult::NoSolution, Some(format!("ODE error: {}", e)))
                }
            }
            _ => (
                AlgebraSolverResult::NoSolution,
                Some(format!("ODEs of order {} not yet supported", order))
            )
        }
    }
}
```

#### Task 2: Integration Tests Through MathSolver

**File**: `crates/mathhook-core/tests/integration_ode_solver.rs`

**Tests**:
```rust
use mathhook_core::{MathSolver, symbol, parser::latex::parse_latex};

#[test]
fn test_solve_separable_ode_through_mathsolver() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Parse ODE equation
    let ode = parse_latex(r"\frac{dy}{dx} = x \cdot y").unwrap();

    // Solve through unified MathSolver API
    let mut solver = MathSolver::new();
    let solution = solver.solve(&ode, &y).unwrap();

    // Verify solution is correct
    // Expected: y = C*exp(x^2/2)
    assert!(solution.contains("exp"));
}

#[test]
fn test_solve_linear_first_order_ode_through_mathsolver() {
    let x = symbol!(x);
    let y = symbol!(y);

    let ode = parse_latex(r"\frac{dy}{dx} + y = x").unwrap();

    let mut solver = MathSolver::new();
    let solution = solver.solve(&ode, &y).unwrap();

    // Verify solution
    assert!(solution.is_ok());
}
```

### Phase 3: Preserve Internal API (2-3 hours)

**Goal**: Keep `ODESolver` as internal implementation detail, but allow direct use if needed

**File**: `crates/mathhook-core/src/ode/mod.rs`

**Documentation**:
```rust
//! ODE solving module
//!
//! # Public API (RECOMMENDED)
//!
//! Use the unified `MathSolver` API:
//! ```rust
//! use mathhook_core::{MathSolver, symbol, parser::latex::parse_latex};
//!
//! let y = symbol!(y);
//! let ode = parse_latex(r"\frac{dy}{dx} = x*y").unwrap();
//! let mut solver = MathSolver::new();
//! let solution = solver.solve(&ode, &y).unwrap();
//! ```
//!
//! # Direct API (ADVANCED)
//!
//! For advanced use cases, you can use `ODESolver` directly:
//! ```rust
//! use mathhook_core::ode::solver::ODESolver;
//! use mathhook_core::{symbol, expr};
//!
//! let x = symbol!(x);
//! let y = symbol!(y);
//! let rhs = expr!(x * y);
//! let solution = ODESolver::solve_first_order(&rhs, &y, &x).unwrap();
//! ```
```

### Phase 4: Verification (2-3 hours)

#### Regression Testing

```bash
# All existing tests must pass:
cargo test -p mathhook-core

# ODE-specific tests:
cargo test -p mathhook-core ode

# Integration tests:
cargo test -p mathhook-core integration_ode
```

#### Mathematical Correctness Verification

```bash
# Re-run correctness verification from Wave 0B:
python .mathhook_sessions/verify_plan7_ode_correctness.py

# ✅ MUST match baseline - no regressions
```

### Success Criteria

1. **API Integration**:
   ```rust
   // Unified API works:
   let mut solver = MathSolver::new();
   solver.solve(&ode_equation, &y).unwrap();  // ✅ Automatically routes to ODESolver
   ```

2. **EquationAnalyzer Integration**:
   ```rust
   EquationAnalyzer::classify_equation(&ode, &y)
   // ✅ Returns EquationType::OrdinaryDifferential
   ```

3. **Test Coverage**:
   - All existing ODE unit tests pass (preservation)
   - New integration tests through MathSolver pass
   - Regression tests pass (no breaking changes)

4. **Mathematical Correctness Maintained**:
   - SymPy comparison script still shows 100% match
   - No correctness regressions introduced

### Deliverables

- [ ] `EquationType` extended with ODE cases
- [ ] `EquationAnalyzer` detects and classifies ODEs
- [ ] `SmartEquationSolver` routes ODEs to `ODESolver`
- [ ] Integration tests through `MathSolver` API
- [ ] Documentation updated
- [ ] All tests pass
- [ ] Mathematical correctness verified

---

## Wave 5-INT: PDE Integration Refactoring

**Duration**: 12-16 hours
**Priority**: HIGH (completeness)
**Goal**: Integrate PDE module with SmartEquationSolver/EquationAnalyzer

**Dependencies**: Wave 1-INT (pattern established)

### Approach

**Same pattern as Wave 1-INT**, adapted for PDEs:

1. `EquationType::PartialDifferential` already added in Wave 1-INT
2. Integrate `PDEClassifier` into `EquationAnalyzer`
3. Add PDE routing in `SmartEquationSolver`
4. Integration tests through `MathSolver`
5. Verify correctness maintained

### Success Criteria

```rust
let mut solver = MathSolver::new();
let pde = parse_latex(r"\frac{\partial^2 u}{\partial x^2} = 0").unwrap();
solver.solve(&pde, &u).unwrap();  // ✅ Works
```

### Deliverables

- [ ] PDEs integrated with `SmartEquationSolver`
- [ ] Integration tests through `MathSolver`
- [ ] Mathematical correctness verified

---

## Wave 3-INT: Gröbner Basis Completion

**Duration**: 6-8 hours
**Priority**: MEDIUM (functionality gap)
**Goal**: Fix compilation and verify integration

**Dependencies**: Wave 0A (build fixed)

### Tasks

#### Task 1: Fix Compilation (2-3 hours)

**Actions**:
1. Uncomment `pub mod groebner;` in algebra.rs
2. Fix compilation errors
3. Verify module compiles

#### Task 2: Integration Verification (2-3 hours)

**Actions**:
1. Verify Gröbner basis algorithms are correct
2. Add integration tests with polynomial solvers
3. Document usage patterns

#### Task 3: Testing (2 hours)

**Actions**:
1. Run Gröbner tests
2. Compare against SymPy's Gröbner basis
3. Verify correctness

### Success Criteria

```bash
cargo build  # ✅ Gröbner compiles
cargo test groebner  # ✅ All tests pass
```

### Deliverables

- [ ] Gröbner basis compiles without errors
- [ ] Tests pass
- [ ] Correctness verified against SymPy

---

## Wave 6-INT: Root Finding Integration Assessment

**Duration**: 4-6 hours
**Priority**: LOW (assessment first)
**Goal**: Determine if root_finding duplicates existing functionality

### Tasks

#### Task 1: Analyze root_finding Module (2 hours)

**Actions**:
1. Read root_finding implementation
2. Compare with existing polynomial solvers
3. Check for duplication with `SmartEquationSolver`

#### Task 2: Integration Decision (2-4 hours)

**Options**:
A. **No Duplication**: Integrate with existing solvers
B. **Duplication Found**: Refactor to use existing solve() or remove
C. **Complementary**: Keep as numerical methods for existing solvers

**Actions Based on Analysis**:
- Document findings
- Implement chosen option
- Verify no regressions

### Deliverables

- [ ] root_finding analysis completed
- [ ] Integration decision documented
- [ ] Implementation completed based on decision

---

## Wave Summary

| Wave | Duration | Priority | Dependencies | Deliverable |
|------|----------|----------|--------------|-------------|
| 0A: Build Fix | 2-4h | CRITICAL | None | Green build |
| 0B: Correctness | 4-6h | CRITICAL | 0A | Baseline verified |
| 1-INT: ODE | 12-16h | HIGH | 0A, 0B | ODE integrated |
| 5-INT: PDE | 12-16h | HIGH | 1-INT | PDE integrated |
| 3-INT: Gröbner | 6-8h | MEDIUM | 0A | Gröbner compiles |
| 6-INT: Root Finding | 4-6h | LOW | 0A | Integration assessed |

**Total Estimated Duration**: 40-56 hours (5-7 working days)

---

## Execution Strategy

### Parallel Execution (Maximum 2 Agents)

**Phase 1** (Prerequisite - Sequential):
- Agent 1: Wave 0A (Build Fix) → Wave 0B (Correctness)

**Phase 2** (Parallel):
- Agent 1: Wave 1-INT (ODE Integration)
- Agent 2: Wave 3-INT (Gröbner Completion)

**Phase 3** (Parallel):
- Agent 1: Wave 5-INT (PDE Integration)
- Agent 2: Wave 6-INT (Root Finding Assessment)

### Quality Gates

**Before Each Wave**:
- Green build state verified
- Baseline tests passing
- Mathematical correctness documented

**After Each Wave**:
- All tests pass (no regressions)
- Integration tests added and passing
- Mathematical correctness re-verified
- Documentation updated

---

## Risk Mitigation

### Risk 1: Correctness Regressions During Refactoring

**Mitigation**:
- Wave 0B establishes correctness baseline BEFORE refactoring
- Re-run SymPy comparison after every change
- Keep ODESolver/PDESolver as internal implementation (preserve proven code)
- Only add routing layer, don't rewrite algorithms

### Risk 2: API Breaking Changes

**Mitigation**:
- Preserve direct `ODESolver::solve_first_order()` API as "advanced use"
- Add new unified API without removing old one
- Deprecation period before removing old APIs (if ever)
- Document both APIs clearly

### Risk 3: Integration Tests Breaking Existing Behavior

**Mitigation**:
- Run full test suite after every change
- Compare test count before/after (should increase, never decrease)
- Any test failure requires investigation before proceeding

---

## Success Metrics

**Wave 0 Success**:
- ✅ Build green
- ✅ Baseline test count established
- ✅ Mathematical correctness verified

**Integration Success**:
- ✅ Single unified API: `MathSolver::solve()` handles all equation types
- ✅ `EquationAnalyzer` classifies ODEs, PDEs, algebraic equations
- ✅ All tests pass (existing + new integration tests)
- ✅ Mathematical correctness maintained (SymPy comparison)

**Documentation Success**:
- ✅ CLAUDE.md updated with ODE/PDE patterns
- ✅ User-facing docs show unified API
- ✅ Examples demonstrate Expression-centric + Solver hybrid design

---

## Post-Integration Validation

**Final Verification Script**:
```bash
#!/bin/bash
# verify_plan7_integration.sh

echo "=== Build Verification ==="
cargo build || exit 1

echo "=== Test Suite ==="
cargo test || exit 1

echo "=== ODE Correctness ==="
python .mathhook_sessions/verify_plan7_ode_correctness.py || exit 1

echo "=== PDE Correctness ==="
python .mathhook_sessions/verify_plan7_pde_correctness.py || exit 1

echo "=== Integration Tests ==="
cargo test integration_ode || exit 1
cargo test integration_pde || exit 1

echo "✅ ALL VERIFICATIONS PASSED"
```

Run after all waves complete.
