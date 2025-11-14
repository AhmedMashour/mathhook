# MathHook Development Roadmap: Next Priorities

**Date**: 2025-10-19
**Status**: Post-completion of number theory & polynomial functions (9.25/10 quality, 103 tests)
**Current Coverage**: 70-75% of SymPy (up from 60-65%)

---

## Executive Summary: What to Build Next

Based on the updated analysis, here are the **highest-impact** features to work on:

**Tier 1 (Critical)**: Symbolic Integration + ODEs (closes biggest gaps)
**Tier 2 (Important)**: Gamma function + Quick wins (abs, sqrt)
**Tier 3 (Nice-to-have)**: Cubic/Quartic formulas, Bessel functions

---

## TIER 1: CRITICAL FEATURES (Next 6 Months)

### 1. Symbolic Integration - Risch-Norman Algorithm

**Priority**: **HIGHEST** (Biggest gap vs SymPy)
**Effort**: Very High (complex algorithm)
**Timeline**: 3-6 months
**Impact**: Calculus coverage from 75% → 95%

**Current State**:
- ✅ Pattern-based integration (power rule, trig, exp, log)
- ✅ Integration by parts
- ❌ Cannot integrate: `∫ 1/(x³+1) dx`, `∫ e^(x²) dx`, `∫ sin(x)/x dx`

**Recommended Phased Approach**:

**Phase 1: Basic Risch (1-2 months)**
```rust
// Target: Rational functions and polynomials
∫ (3x² + 2x + 1)/(x³ + x) dx  // ✅
∫ 1/(x² - 1) dx  // ✅ → (1/2)ln|x-1| - (1/2)ln|x+1|
∫ (2x + 3)/(x² + 3x + 2) dx  // ✅
```

**Files to create**:
- `calculus/integrals/risch/mod.rs` - Main Risch algorithm coordinator
- `calculus/integrals/risch/rational.rs` - Rational function integration
- `calculus/integrals/risch/polynomial.rs` - Polynomial integration
- `calculus/integrals/risch/logarithmic.rs` - Logarithmic part computation
- `tests/risch_rational_tests.rs` - 50+ tests vs SymPy

**Phase 2: Elementary Functions (2-3 months)**
```rust
// Target: exp, log, trig combinations
∫ e^x * sin(x) dx  // ✅
∫ ln(x) dx  // ✅
∫ x * e^(x²) dx  // ✅ (with substitution detection)
```

**Phase 3: Extended Risch (1-2 months)**
```rust
// Target: Nested and complex cases
∫ e^(x²) dx  // Recognize as non-elementary (return symbolic)
∫ sin(x)/x dx  // Si(x) - special function
∫ sqrt(1 + x²) dx  // Hyperbolic functions
```

**Success Metrics**:
- 80% of SymPy's integration test suite passes
- Educational step-by-step for all integration methods
- Graceful fallback for non-elementary integrals

**Estimated Effort**: 400-600 hours (3-6 months)

---

### 2. Differential Equation Solver (ODEs)

**Priority**: **HIGHEST** (Essential for applied math)
**Effort**: High (multiple methods)
**Timeline**: 2-4 months
**Impact**: New capability (0% → 80%)

**Current State**:
- ❌ No ODE solver exists
- ❌ Cannot solve: `dy/dx = f(x)`, `y'' + y = 0`

**Recommended Phased Approach**:

**Phase 1: Separable ODEs (2-3 weeks)**
```rust
// dy/dx = f(x) * g(y)
dy/dx = x * y  // ✅ → y = C*e^(x²/2)
dy/dx = y²  // ✅ → y = -1/(x + C)
```

**Files to create**:
- `calculus/differential_equations/mod.rs` - ODE solver coordinator
- `calculus/differential_equations/separable.rs` - Separable ODE solver
- `calculus/differential_equations/classification.rs` - ODE type detection
- `tests/ode_separable_tests.rs` - 30+ tests

**Phase 2: First-Order Linear (3-4 weeks)**
```rust
// dy/dx + P(x)y = Q(x)
dy/dx + 2xy = x  // ✅ → y = (1/2) + C*e^(-x²)
dy/dx + y/x = 1  // ✅
```

**Files to create**:
- `calculus/differential_equations/linear_first_order.rs`
- `calculus/differential_equations/integrating_factor.rs`
- `tests/ode_linear_first_order_tests.rs` - 30+ tests

**Phase 3: Second-Order Constant Coefficients (4-5 weeks)**
```rust
// ay'' + by' + cy = 0
y'' + y = 0  // ✅ → y = C₁*cos(x) + C₂*sin(x)
y'' - 4y' + 4y = 0  // ✅ → y = (C₁ + C₂*x)*e^(2x)
y'' + 2y' + 5y = 0  // ✅ → y = e^(-x)(C₁*cos(2x) + C₂*sin(2x))
```

**Files to create**:
- `calculus/differential_equations/second_order_constant.rs`
- `calculus/differential_equations/characteristic_equation.rs`
- `tests/ode_second_order_tests.rs` - 40+ tests

**Phase 4: Advanced Methods (4-6 weeks)**
```rust
// Bernoulli, exact, substitution methods
dy/dx + P(x)y = Q(x)y^n  // Bernoulli
M(x,y)dx + N(x,y)dy = 0  // Exact (if ∂M/∂y = ∂N/∂x)
```

**Success Metrics**:
- Solves 70%+ of first-year ODE problems
- Educational step-by-step for all methods
- Proper classification of ODE types

**Estimated Effort**: 300-500 hours (2-4 months)

---

## TIER 2: IMPORTANT FEATURES (Next 3 Months)

### 3. Gamma Function Γ(z)

**Priority**: **MEDIUM-HIGH**
**Effort**: Medium
**Timeline**: 1-2 weeks
**Impact**: Unlocks many special functions

**Current State**:
- ✅ Factorial n! exists (integers only)
- ❌ No Γ(z) for general complex z

**Implementation Plan**:

```rust
// File: functions/special/gamma.rs

pub fn gamma(z: &Expression) -> Expression {
    // Special cases
    if z is positive integer {
        return factorial(z - 1);
    }

    if z == 1/2 {
        return sqrt(π);
    }

    // Recurrence: Γ(z+1) = z*Γ(z)
    // Use for reduction to [1, 2)

    // For large |z|: Stirling's approximation
    // Γ(z) ≈ √(2π/z) * (z/e)^z

    // For general case: Use series expansion or numerical methods
}

// Function intelligence
FunctionProperties::Special(Box::new(SpecialProperties {
    differential_equation: "z*Γ'(z) = Γ(z+1) - Γ(z)",
    recurrence_relations: vec![
        "Γ(z+1) = z*Γ(z)",
        "Γ(1-z)*Γ(z) = π/sin(πz)",  // Reflection formula
    ],
    special_values: vec![
        (1, 1),
        (1/2, sqrt(π)),
        (2, 1),
        (3, 2),
    ],
}))
```

**What this enables**:
- Beta function: `B(a,b) = Γ(a)*Γ(b)/Γ(a+b)`
- Incomplete gamma functions
- Digamma and polygamma functions
- Better combinatorics (non-integer factorials)

**Success Metrics**:
- Γ(n) = (n-1)! for positive integers
- Γ(1/2) = √π exactly
- Recurrence relation works
- Complex plane support (with branch cuts documented)

**Estimated Effort**: 40-60 hours (1-2 weeks)

---

### 4. Quick Wins Bundle

**Priority**: **MEDIUM-HIGH** (Low effort, high value)
**Effort**: Very Low (2-4 hours each)
**Timeline**: 1 week total

#### 4a. Absolute Value Function |x|

```rust
// File: functions/elementary/abs.rs

impl Expression {
    pub fn abs(self) -> Expression {
        Expression::function("abs", vec![self])
    }
}

// Function intelligence
FunctionProperties::Elementary(Box::new(ElementaryProperties {
    derivative_rule: DerivativeRule::Custom(|x| {
        // d/dx|x| = sgn(x) = x/|x| for x ≠ 0
        Expression::div(x.clone(), Expression::abs(x.clone()))
    }),

    antiderivative_rule: AntiderivativeRule::Custom(|x, var| {
        // ∫|x|dx = x|x|/2 + C
        Expression::div(
            Expression::mul(vec![x.clone(), Expression::abs(x.clone())]),
            Expression::integer(2)
        )
    }),

    special_values: vec![
        SpecialValue { input: "0", output: Expression::integer(0) },
    ],

    domain: Domain::Real,
    range: Range::NonNegative,

    simplification_rules: vec![
        // |a*b| = |a|*|b|
        // |x²| = x²
        // |-x| = |x|
    ],
}))
```

**Estimated Effort**: 3-4 hours

#### 4b. Square Root Function sqrt(x)

```rust
// File: functions/elementary/sqrt.rs

impl Expression {
    pub fn sqrt(self) -> Expression {
        Expression::function("sqrt", vec![self])
    }
}

// Currently: x^(1/2)
// After: Dedicated sqrt with better:
// - Domain checking (sqrt(-1) → error in real, i in complex)
// - Simplification (sqrt(4) = 2, sqrt(x²) = |x|)
// - LaTeX output (\sqrt{x} instead of x^{1/2})
```

**Estimated Effort**: 3-4 hours

#### 4c. Polynomial Division Public API

```rust
// File: algebra/polynomial_division.rs (already exists!)

// Just make public and document:
pub use crate::algebra::polynomial_division::{
    polynomial_div,  // Returns (quotient, remainder)
    polynomial_quo,  // Returns quotient only
    polynomial_rem,  // Returns remainder only
};

// Add to PolynomialGcd trait for convenience:
impl PolynomialGcd for Expression {
    fn div_polynomial(&self, other: &Self, var: &Symbol)
        -> (Expression, Expression)
    {
        polynomial_div(self, other, var)
    }
}
```

**Estimated Effort**: 2-3 hours

**Total for Quick Wins**: 8-11 hours (~1 week)

---

## TIER 3: NICE-TO-HAVE FEATURES

### 5. Cubic and Quartic Formulas

**Priority**: **MEDIUM**
**Effort**: Medium
**Timeline**: 2-3 weeks
**Impact**: Equation solving from 30% → 50%

**Current State**:
- ✅ Rational root theorem finds some roots
- ❌ Cannot solve cubics/quartics without rational roots

**Implementation**:

```rust
// File: algebra/solvers/cubic.rs

pub fn solve_cubic(a: i64, b: i64, c: i64, d: i64) -> Vec<Expression> {
    // ax³ + bx² + cx + d = 0

    // Cardano's formula:
    // 1. Depress: Substitute x = t - b/(3a) → t³ + pt + q = 0
    // 2. Solve using Cardano's discriminant
    // 3. Transform back
}

// File: algebra/solvers/quartic.rs

pub fn solve_quartic(a: i64, b: i64, c: i64, d: i64, e: i64) -> Vec<Expression> {
    // ax⁴ + bx³ + cx² + dx + e = 0

    // Ferrari's method:
    // 1. Depress: x = y - b/(4a)
    // 2. Solve resolvent cubic
    // 3. Solve two quadratics
}
```

**Success Metrics**:
- Solves all degree ≤4 polynomials
- Returns exact symbolic roots
- Educational step-by-step explanation

**Estimated Effort**: 80-120 hours (2-3 weeks)

---

### 6. Bessel Functions

**Priority**: **LOW-MEDIUM**
**Effort**: High
**Timeline**: 2-3 weeks

**Implementation**: Similar to existing special functions (Chebyshev, Legendre)

**Files to create**:
- `functions/special/bessel.rs` - Bessel function intelligence
- `functions/special/bessel_evaluation.rs` - Numerical evaluation
- Add to UniversalFunctionRegistry

**What this enables**:
- J_n(x) - Bessel functions of the first kind
- Y_n(x) - Bessel functions of the second kind
- Physics applications (wave equations, heat transfer, quantum mechanics)

**Estimated Effort**: 80-100 hours (2-3 weeks)

---

## Recommended 6-Month Development Plan

### Month 1: Foundation + Quick Wins
- Week 1: abs(), sqrt(), polynomial division API (quick wins)
- Week 2-4: Gamma function Γ(z) with full intelligence

**Deliverables**: 3 quick wins + Gamma function

---

### Month 2-3: Symbolic Integration (Basic Risch)
- Week 1-2: Rational function integration
- Week 3-4: Polynomial integration
- Week 5-6: Logarithmic part computation
- Week 7-8: Testing and refinement (50+ SymPy validation tests)

**Deliverables**: Basic Risch algorithm (rational + polynomial)

---

### Month 4-5: Differential Equations
- Week 1-2: Separable ODEs
- Week 3-4: First-order linear ODEs
- Week 5-6: Second-order constant coefficients
- Week 7-8: Bernoulli, exact equations
- Week 9-10: Testing and educational explanations

**Deliverables**: ODE solver (4 major types)

---

### Month 6: Polish + Extensions
- Week 1-2: Extended Risch (elementary functions)
- Week 3-4: Cubic/quartic formulas OR Bessel functions (choose one)

**Deliverables**: Enhanced integration + one bonus feature

---

## Success Metrics

**After 6 months**:
- ✅ Integration coverage: 75% → 90% (Risch algorithm)
- ✅ ODE solving: 0% → 80% (4 major types)
- ✅ Equation solving: 30% → 50% (cubic/quartic formulas)
- ✅ Overall SymPy coverage: 70-75% → 85-90%

**Quality Targets**:
- 300+ new tests (50 per month average)
- 9.0+/10 average quality (maintain current standard)
- 100% SymPy validation for all new features
- Zero regressions

---

## Features to Defer (Low Priority)

**Don't work on these yet** (low ROI for effort):

1. **Gröbner Bases** - Very high effort, niche use case
2. **Noncommutative Algebra** - Architectural change, limited users
3. **Logic/SAT Solver** - Completely different domain
4. **Diophantine Equations** - Niche number theory
5. **Permutation Groups** - Advanced group theory, limited demand

**Rationale**: Focus on **calculus completeness** first (integration + ODEs). These serve the largest user base (students, engineers, scientists) and provide the most educational value.

---

## Summary: Strategic Focus

**Core Thesis**: MathHook should **dominate calculus** before expanding into niche areas.

**Why**:
1. Largest user base (calculus is universal in STEM)
2. Complements existing strengths (differentiation, limits already excellent)
3. Biggest gap vs SymPy (integration is the weakest area)
4. Highest educational value (step-by-step integration + ODE solving)

**After achieving calculus dominance** (integration + ODEs + series):
- Then expand to advanced algebra (Gröbner bases, noncommutative)
- Then add specialized features (logic, group theory, etc.)

**Next 6 Months Focus** (in order):
1. ✅ Quick wins (abs, sqrt, public APIs) - **Week 1**
2. ✅ Gamma function - **Weeks 2-4**
3. ✅ Symbolic integration (basic Risch) - **Months 2-3**
4. ✅ Differential equations - **Months 4-5**
5. ✅ Polish + extensions - **Month 6**

---

**Created**: 2025-10-19
**Status**: Ready for execution
**First Task**: Implement abs() function (est. 3-4 hours)
