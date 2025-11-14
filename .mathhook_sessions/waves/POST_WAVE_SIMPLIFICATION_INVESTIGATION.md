# Post-Wave Cleanup: Algebraic Simplification Investigation

**Priority**: 🔴 **CRITICAL** - Blocks Tests 1, 6, and potentially Test 8
**Estimated Effort**: 5-8 hours
**Impact**: HIGH - Enables 2-3 additional tests

---

## Executive Summary

Wave 3.2 correctly identified that **algebraic simplification failure** is the root cause preventing Tests 1 and 6 from passing. This document provides a comprehensive investigation plan to diagnose and fix the simplification issues.

### The Core Problem

During integration by parts for `∫x²·ln(x) dx`:
1. First iteration produces: `(x²/2)·ln(x) - ∫(x²/2)·(1/x) dx`
2. The second integral should simplify: `(x²/2) * (1/x) = x²/(2x) = x/2`
3. Then integrate: `∫x/2 dx = x²/4`
4. **ACTUAL BEHAVIOR**: `(x²/2) * (1/x)` does NOT simplify to `x/2`
5. Result: Recursive integration call returns symbolic integral

**Mathematical Failure**: `x² * x^(-1)` should reduce to `x^(2-1) = x`, but doesn't.

---

## Investigation Phases

### Phase 1: Diagnostic Testing (2-3 hours)

#### Objective
Create comprehensive test suite to identify exactly which simplification rules are missing or broken.

#### Task 1.1: Create Algebraic Simplification Test Suite

**File**: `crates/mathhook-core/tests/simplification_algebraic_tests.rs`

**Test Cases**:

```rust
//! Algebraic Simplification Tests
//!
//! Tests basic algebraic manipulation rules that should work during integration.
//! Failures here indicate missing or broken simplification rules.

use mathhook_core::core::Expression;
use mathhook_core::core::Symbol;
use mathhook_core::simplify::Simplify;

fn symbol(name: &str) -> Symbol {
    Symbol::scalar(name)
}

fn x() -> Symbol {
    symbol("x")
}

// CRITICAL TEST CASES FOR INTEGRATION

#[test]
fn test_power_combination_basic() {
    // x² * x^(-1) should simplify to x
    let x_sym = x();
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x_sym), Expression::integer(-1)),
    ]);

    let simplified = expr.simplify();

    // Expected: x (i.e., x^1)
    assert_eq!(
        simplified,
        Expression::symbol(x()),
        "x² * x^(-1) should simplify to x"
    );
}

#[test]
fn test_power_combination_general() {
    // x^a * x^b should simplify to x^(a+b)
    let x_sym = x();

    // Test: x³ * x^(-2) → x
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x_sym), Expression::integer(-2)),
    ]);

    let simplified = expr.simplify();

    assert_eq!(
        simplified,
        Expression::symbol(x()),
        "x³ * x^(-2) should simplify to x"
    );
}

#[test]
fn test_rational_coefficient_multiplication() {
    // (x²/2) * (1/x) should simplify to x/2
    let x_sym = x();

    // Build: (x²/2) * (1/x) = (x² * 1) / (2 * x) = x²/(2x)
    let numerator = Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(2));
    let denominator = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(x_sym.clone()),
    ]);

    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let simplified = expr.simplify();

    // Expected: x/2
    let expected = Expression::mul(vec![
        Expression::symbol(x_sym),
        Expression::rational(1, 2),
    ]);

    assert_eq!(
        simplified,
        expected,
        "(x²/2) * (1/x) should simplify to x/2"
    );
}

#[test]
fn test_division_cancellation() {
    // x² / x should simplify to x
    let x_sym = x();

    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x_sym), Expression::integer(-1)),
    ]);

    let simplified = expr.simplify();

    assert_eq!(
        simplified,
        Expression::symbol(x()),
        "x²/x should simplify to x"
    );
}

#[test]
fn test_nested_rational_simplification() {
    // (1/2) * x should simplify to x/2
    let x_sym = x();

    let expr = Expression::mul(vec![
        Expression::rational(1, 2),
        Expression::symbol(x_sym.clone()),
    ]);

    let simplified = expr.simplify();

    // Should maintain: (1/2) * x or x/2 (both acceptable)
    // Just verify it doesn't break
    assert!(
        simplified.to_string().contains("x"),
        "Expression should contain x after simplification"
    );
}

#[test]
fn test_constant_factor_extraction() {
    // (2x) * (3y) should simplify to 6xy
    let x_sym = x();
    let y_sym = symbol("y");

    let expr = Expression::mul(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x_sym)]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y_sym)]),
    ]);

    let simplified = expr.simplify();

    // Expected: 6 * x * y (order may vary)
    let simplified_str = simplified.to_string();
    assert!(
        simplified_str.contains("6") && simplified_str.contains("x") && simplified_str.contains("y"),
        "(2x)(3y) should simplify to 6xy, got: {}",
        simplified_str
    );
}

// INTEGRATION-SPECIFIC FAILURE CASES

#[test]
fn test_by_parts_intermediate_simplification() {
    // Exact case from Test 1: ∫x²·ln(x) dx
    // After first by-parts: (x²/2)·ln(x) - ∫(x²/2)·(1/x) dx
    // The integrand (x²/2)·(1/x) MUST simplify to x/2

    let x_sym = x();

    // Build: (x²/2) * (1/x)
    let x_squared_over_2 = Expression::mul(vec![
        Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(2)),
        Expression::rational(1, 2),
    ]);

    let one_over_x = Expression::pow(
        Expression::symbol(x_sym.clone()),
        Expression::integer(-1),
    );

    let expr = Expression::mul(vec![x_squared_over_2, one_over_x]);

    let simplified = expr.simplify();

    // Expected: x/2 or (1/2)*x
    let expected = Expression::mul(vec![
        Expression::rational(1, 2),
        Expression::symbol(x_sym),
    ]);

    assert_eq!(
        simplified,
        expected,
        "By-parts intermediate (x²/2)·(1/x) should simplify to x/2"
    );
}

#[test]
fn test_power_rule_with_coefficients() {
    // (3x²) * (1/x) should simplify to 3x
    let x_sym = x();

    let expr = Expression::mul(vec![
        Expression::integer(3),
        Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x_sym.clone()), Expression::integer(-1)),
    ]);

    let simplified = expr.simplify();

    // Expected: 3x
    let expected = Expression::mul(vec![
        Expression::integer(3),
        Expression::symbol(x_sym),
    ]);

    assert_eq!(
        simplified,
        expected,
        "3x² * (1/x) should simplify to 3x"
    );
}
```

**Expected Outcomes**:
- ❌ Many tests will FAIL - this identifies the missing simplification rules
- ✅ Passing tests show what DOES work
- 📊 Failure patterns reveal which simplification categories are broken

---

#### Task 1.2: Trace Simplification Execution

**Objective**: Understand what simplification rules ARE being applied and which are missing.

**File**: `crates/mathhook-core/examples/trace_simplification.rs`

```rust
//! Simplification Execution Trace
//!
//! Debug example to see exactly what happens during simplification.

use mathhook_core::core::{Expression, Symbol};
use mathhook_core::simplify::Simplify;

fn main() {
    let x = Symbol::scalar("x");

    println!("=== CRITICAL TEST CASE ===");
    println!("Expression: (x²/2) * (1/x)");
    println!("Expected: x/2");
    println!("");

    // Build expression
    let x_squared_over_2 = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::rational(1, 2),
    ]);

    let one_over_x = Expression::pow(
        Expression::symbol(x.clone()),
        Expression::integer(-1),
    );

    let expr = Expression::mul(vec![x_squared_over_2, one_over_x]);

    println!("Before simplification:");
    println!("  Expression tree: {:?}", expr);
    println!("  String form: {}", expr);
    println!("");

    // Simplify
    let simplified = expr.simplify();

    println!("After simplification:");
    println!("  Expression tree: {:?}", simplified);
    println!("  String form: {}", simplified);
    println!("");

    // Check if it matches expected
    let expected = Expression::mul(vec![
        Expression::rational(1, 2),
        Expression::symbol(x),
    ]);

    if simplified == expected {
        println!("✅ SUCCESS: Simplification worked correctly!");
    } else {
        println!("❌ FAILURE: Simplification did not produce expected result");
        println!("  Expected: {}", expected);
        println!("  Got: {}", simplified);
    }
}
```

**Run**:
```bash
cargo run --example trace_simplification
```

**Analysis Questions**:
1. What does the expression tree look like before/after simplification?
2. Are multiplication factors being flattened?
3. Are powers being combined?
4. What's the canonical form?

---

#### Task 1.3: Compare with SymPy Behavior

**File**: `playground_simplification_sympy_comparison.py`

```python
#!/usr/bin/env python3
"""
Simplification Comparison with SymPy

Verify expected behavior for all algebraic simplification cases.
This establishes the "ground truth" for what SHOULD happen.
"""

from sympy import symbols, simplify, expand, powsimp, ratsimp

def test_case(description, expr_lambda, expected_lambda):
    x, y = symbols('x y')
    expr = expr_lambda(x, y)
    expected = expected_lambda(x, y)

    result = simplify(expr)

    print(f"\n{'='*60}")
    print(f"Test: {description}")
    print(f"  Expression: {expr}")
    print(f"  Expected:   {expected}")
    print(f"  SymPy:      {result}")
    print(f"  Match:      {result == expected or expand(result - expected) == 0}")

    return result

def main():
    print("ALGEBRAIC SIMPLIFICATION - SymPy Ground Truth")
    print("="*60)

    # Power combination
    test_case(
        "x² * x^(-1) → x",
        lambda x, y: x**2 * x**(-1),
        lambda x, y: x
    )

    test_case(
        "x³ * x^(-2) → x",
        lambda x, y: x**3 * x**(-2),
        lambda x, y: x
    )

    # Critical integration case
    test_case(
        "(x²/2) * (1/x) → x/2",
        lambda x, y: (x**2 / 2) * (1/x),
        lambda x, y: x/2
    )

    test_case(
        "x²/x → x",
        lambda x, y: x**2 / x,
        lambda x, y: x
    )

    # Coefficient handling
    test_case(
        "(1/2) * x → x/2",
        lambda x, y: (1/2) * x,
        lambda x, y: x/2
    )

    test_case(
        "(2x) * (3y) → 6xy",
        lambda x, y: (2*x) * (3*y),
        lambda x, y: 6*x*y
    )

    test_case(
        "3x² * (1/x) → 3x",
        lambda x, y: 3*x**2 * (1/x),
        lambda x, y: 3*x
    )

if __name__ == "__main__":
    main()
```

**Run**:
```bash
python playground_simplification_sympy_comparison.py
```

**Expected Output**: ALL cases should show `Match: True` in SymPy.

---

### Phase 2: Root Cause Analysis (1-2 hours)

#### Objective
Identify exactly which simplification rules are missing and where to implement them.

#### Task 2.1: Examine Current Simplification Architecture

**Files to Review**:
1. `crates/mathhook-core/src/simplify/mod.rs` - Simplify trait definition
2. `crates/mathhook-core/src/simplify/*.rs` - Individual simplification modules
3. `crates/mathhook-core/src/core/expression.rs` - Expression::simplify() implementation

**Analysis Checklist**:

```
Current Simplification Rules (check if implemented):

Arithmetic:
[ ] Constant folding: 2 + 3 → 5
[ ] Identity elements: x + 0 → x, x * 1 → x
[ ] Associativity flattening: (a + b) + c → a + b + c

Power Rules:
[ ] x^a * x^b → x^(a+b)  ⭐ CRITICAL FOR INTEGRATION
[ ] (x^a)^b → x^(a*b)
[ ] x^0 → 1 (except 0^0)
[ ] x^1 → x

Multiplication:
[ ] Commutative sorting: y * x → x * y
[ ] Constant extraction: 2 * 3 * x → 6 * x
[ ] Power combination in products
[ ] Rational coefficient simplification

Division:
[ ] x/x → 1
[ ] (a*b)/c → a * (b/c)
[ ] a / (b/c) → (a*c) / b

Rational Numbers:
[ ] Reduction to lowest terms: 6/4 → 3/2
[ ] Rational arithmetic: (1/2) * (1/3) → 1/6
[ ] Mixed rational/symbol: (1/2) * x → x/2
```

**Investigation Questions**:
1. Does `Expression::Mul` automatically flatten nested multiplications?
2. Is there a power combination pass that runs during simplification?
3. Where is canonical form enforced?
4. Is simplification idempotent? (simplify(simplify(x)) == simplify(x))

---

#### Task 2.2: Identify Missing Rules

Based on Phase 1 test failures, create a priority list of missing rules:

**Priority 1 (CRITICAL - Blocks Integration)**:
- [ ] Power combination in products: `x^a * x^b → x^(a+b)`
- [ ] Rational coefficient multiplication: `(p/q) * x → px/q`

**Priority 2 (Important)**:
- [ ] Division simplification: `x^n / x^m → x^(n-m)`
- [ ] Constant extraction and folding in products

**Priority 3 (Nice to Have)**:
- [ ] Advanced rational simplification
- [ ] Nested power reduction

---

#### Task 2.3: Check Integration Call Sites

**Question**: Does integration call `simplify()` before recursive calls?

**Files to Check**:
1. `crates/mathhook-core/src/calculus/integrals/by_parts.rs:130-140`
   ```rust
   // After computing v * du, is simplify() called before integrating?
   let v_du = Expression::mul(vec![v.clone(), du]);

   // SHOULD BE:
   let v_du = Expression::mul(vec![v.clone(), du]).simplify();

   // Before:
   let integral_v_du = v_du.integrate(variable.clone(), depth + 1);
   ```

2. `crates/mathhook-core/src/calculus/integrals/strategy.rs`
   ```rust
   // Is each strategy result simplified before returning?
   ```

**Create Checklist**:
```
Integration Simplification Call Sites:
[ ] by_parts.rs:130 - Before recursive integrate()
[ ] strategy.rs - After each technique
[ ] substitution.rs - After u-substitution replacement
[ ] basic.rs - After power rule application
```

---

### Phase 3: Solution Design and Implementation (2-3 hours)

#### Objective
Implement missing simplification rules and integrate into existing pipeline.

#### Task 3.1: Implement Power Combination Rule

**File**: `crates/mathhook-core/src/simplify/power_rules.rs` (create if doesn't exist)

```rust
//! Power Simplification Rules
//!
//! Handles simplification of power expressions including combination and reduction.

use crate::core::{Expression, Number};

/// Simplify power expressions in multiplication
///
/// Combines powers with same base: x^a * x^b → x^(a+b)
///
/// # Examples
///
/// ```rust
/// // x² * x^(-1) → x
/// // x³ * x^(-2) → x
/// ```
pub fn simplify_power_products(factors: &[Expression]) -> Expression {
    // Group factors by base
    let mut base_to_exponents: HashMap<Expression, Vec<Expression>> = HashMap::new();
    let mut non_power_factors: Vec<Expression> = Vec::new();

    for factor in factors {
        match factor {
            Expression::Pow(base, exp) => {
                base_to_exponents
                    .entry((**base).clone())
                    .or_insert_with(Vec::new)
                    .push((**exp).clone());
            }
            Expression::Symbol(sym) => {
                // Symbol is equivalent to symbol^1
                base_to_exponents
                    .entry(Expression::symbol(sym.clone()))
                    .or_insert_with(Vec::new)
                    .push(Expression::integer(1));
            }
            _ => {
                non_power_factors.push(factor.clone());
            }
        }
    }

    // Combine exponents for each base
    let mut result_factors = non_power_factors;

    for (base, exponents) in base_to_exponents {
        if exponents.len() == 1 {
            // Only one exponent, no combination needed
            result_factors.push(Expression::pow(base, exponents[0].clone()));
        } else {
            // Combine: x^a * x^b → x^(a+b)
            let combined_exp = Expression::add(exponents);
            let simplified_exp = combined_exp.simplify();

            // Check if exponent is 0, 1, or other
            match simplified_exp {
                Expression::Number(Number::Integer(n)) if n == 0 => {
                    // x^0 = 1 (skip adding to factors)
                }
                Expression::Number(Number::Integer(n)) if n == 1 => {
                    // x^1 = x
                    result_factors.push(base);
                }
                _ => {
                    result_factors.push(Expression::pow(base, simplified_exp));
                }
            }
        }
    }

    // Return combined expression
    if result_factors.is_empty() {
        Expression::integer(1)
    } else if result_factors.len() == 1 {
        result_factors[0].clone()
    } else {
        Expression::mul(result_factors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_power_combination_basic() {
        let x = Symbol::scalar("x");

        // x² * x^(-1) → x
        let factors = vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        ];

        let result = simplify_power_products(&factors);

        assert_eq!(result, Expression::symbol(x));
    }
}
```

---

#### Task 3.2: Integrate Simplification into Multiplication

**File**: `crates/mathhook-core/src/core/expression.rs`

**Modification**: Enhance `Expression::mul()` to call power combination during construction.

```rust
impl Expression {
    pub fn mul(factors: Vec<Expression>) -> Expression {
        if factors.is_empty() {
            return Expression::integer(1);
        }

        // Flatten nested multiplications
        let mut flattened = Vec::new();
        for factor in factors {
            match factor {
                Expression::Mul(inner) => flattened.extend((*inner).clone()),
                _ => flattened.push(factor),
            }
        }

        // ADDITION: Apply power combination
        let simplified = crate::simplify::power_rules::simplify_power_products(&flattened);

        // Return canonical form
        simplified
    }
}
```

---

#### Task 3.3: Add Simplification Calls in Integration

**File**: `crates/mathhook-core/src/calculus/integrals/by_parts.rs`

**Modification**: Add `simplify()` before recursive integration.

```rust
// Line ~130 in try_by_parts()
let v_du = Expression::mul(vec![v.clone(), du]);

// ADD SIMPLIFICATION HERE (CRITICAL!)
let v_du_simplified = v_du.simplify();

// Then integrate the SIMPLIFIED expression
let integral_v_du = v_du_simplified.integrate(variable.clone(), depth + 1);
```

**Justification**: This ensures `(x²/2) * (1/x)` simplifies to `x/2` BEFORE attempting integration.

---

#### Task 3.4: Comprehensive Testing

**Run Test Suite**:
```bash
# Run new simplification tests
cargo test simplification_algebraic_tests

# Run integration tests to see if it fixes Tests 1 and 6
cargo test --test integration_comprehensive test_by_parts_with_log -- --exact --nocapture
cargo test --test integration_comprehensive test_repeated_by_parts -- --exact --nocapture

# Run full suite for regressions
cargo test -p mathhook-core
```

**Expected Outcomes**:
- ✅ Simplification tests should now PASS
- ✅ Test 1 (∫x²·ln(x) dx) should now PASS
- ✅ Test 6 (∫x³·e^x dx) should now PASS
- ✅ No regressions in existing 39 passing tests

---

### Phase 4: Performance and Validation (1 hour)

#### Task 4.1: Benchmark Performance Impact

**Concern**: Adding simplification to every multiplication might slow things down.

**File**: `crates/mathhook-benchmarks/benches/simplification_bench.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mathhook_core::core::Expression;
use mathhook_core::core::Symbol;
use mathhook_core::simplify::Simplify;

fn benchmark_multiplication_with_simplification(c: &mut Criterion) {
    let x = Symbol::scalar("x");

    c.bench_function("mul without simplification", |b| {
        b.iter(|| {
            Expression::mul(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
            ])
        })
    });

    c.bench_function("mul with simplification", |b| {
        b.iter(|| {
            Expression::mul(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
            ]).simplify()
        })
    });
}

criterion_group!(benches, benchmark_multiplication_with_simplification);
criterion_main!(benches);
```

**Run**:
```bash
cargo bench --bench simplification_bench
```

**Acceptance Criteria**: <20% performance impact on multiplication

---

#### Task 4.2: Validate Against SymPy

**Rerun Python comparison**:
```bash
python playground_simplification_sympy_comparison.py
```

**Verify**: All test cases now match SymPy behavior.

---

## Deliverables

### Files Created
1. ✅ `tests/simplification_algebraic_tests.rs` - Comprehensive simplification test suite
2. ✅ `examples/trace_simplification.rs` - Debug tracer for simplification execution
3. ✅ `playground_simplification_sympy_comparison.py` - SymPy ground truth comparison
4. ✅ `src/simplify/power_rules.rs` - Power combination simplification rules

### Files Modified
1. ✅ `src/core/expression.rs` - Enhanced mul() with power combination
2. ✅ `src/calculus/integrals/by_parts.rs` - Added simplify() before recursive integration
3. ✅ `benches/simplification_bench.rs` - Performance benchmarks

### Documentation
1. ✅ `.mathhook_sessions/SIMPLIFICATION_INVESTIGATION.md` - This document
2. ✅ Investigation findings and root cause report
3. ✅ Implementation decisions and trade-offs

---

## Success Criteria

### Functional
- [ ] All simplification tests pass (simplification_algebraic_tests.rs)
- [ ] Test 1 (∫x²·ln(x) dx) passes
- [ ] Test 6 (∫x³·e^x dx) passes
- [ ] No regressions in existing 39 passing tests

### Performance
- [ ] Multiplication performance impact <20%
- [ ] Integration tests still run in reasonable time (<5 seconds each)

### Correctness
- [ ] All simplification cases match SymPy behavior
- [ ] Mathematical correctness verified with manual proofs

---

## Risks and Mitigation

### Risk 1: Performance Regression
**Mitigation**: Benchmark before/after, lazy simplification if needed

### Risk 2: Breaking Existing Code
**Mitigation**: Comprehensive regression testing, phased rollout

### Risk 3: Incomplete Rule Set
**Mitigation**: Iterative approach - implement critical rules first, expand later

---

## Timeline Estimate

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1 | Diagnostic Testing | 2-3 hours |
| Phase 2 | Root Cause Analysis | 1-2 hours |
| Phase 3 | Implementation | 2-3 hours |
| Phase 4 | Validation | 1 hour |
| **Total** | | **6-9 hours** |

**Recommended Schedule**: 2-3 dedicated sessions over 1-2 weeks

---

## Next Steps After Investigation

1. If simplification fixes work → Retry Wave 3.2 for Tests 1 and 6
2. If simplification alone insufficient → Investigate other factors
3. Document learnings in CLAUDE.md
4. Consider similar fixes for Test 8 (may need architectural change)

---

## Questions to Answer

During investigation, explicitly answer these questions:

1. **What simplification rules are currently implemented?**
2. **Which rules are missing that block integration?**
3. **Where in the integration pipeline should simplify() be called?**
4. **What's the performance impact of adding simplification?**
5. **Does the fix enable Tests 1 and 6 to pass?**
6. **Are there any regressions introduced?**
7. **What additional simplification rules might be needed in future?**

---

**Investigation Owner**: To be determined
**Review Date**: After Phase 2 completion
**Go/No-Go Decision**: After Phase 3 (implementation complete)
