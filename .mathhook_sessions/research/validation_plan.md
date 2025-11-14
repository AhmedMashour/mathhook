# Mathematical Correctness Validation Plan
**Wave 0 Research Phase**
**Date**: October 22, 2025

---

## Executive Summary

This document defines the strategy for ensuring mathematical correctness of MathHook's core features. **Mathematical correctness is the highest priority** - zero tolerance for mathematical errors.

---

## Validation Hierarchy

```
Level 1: Unit Test Correctness
    ├── Each function tested independently
    ├── Edge cases identified and tested
    └── Domain restrictions validated

Level 2: Property-Based Testing
    ├── Mathematical properties verified (commutativity, associativity, etc.)
    ├── Idempotence checked (simplify(simplify(x)) = simplify(x))
    └── Algebraic identities validated

Level 3: Oracle Validation (SymPy Comparison)
    ├── Test cases extracted from SymPy test suite
    ├── Results compared for mathematical equivalence
    └── 100% pass rate required

Level 4: Cross-Reference Validation
    ├── Compare against Symbolica where available
    ├── Validate against published algorithms
    └── Reference standard mathematical texts

Level 5: Educational Validation
    ├── Explanations mathematically accurate
    ├── Domain restrictions clearly communicated
    └── Edge cases explained
```

---

## Level 1: Unit Test Correctness

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mathhook_core::{symbol, expr};

    #[test]
    fn test_ode_separable_simple() {
        // Arrange
        let x = symbol!(x);
        let y = symbol!(y);
        let ode = expr!(y.diff(x) = x); // dy/dx = x

        // Act
        let solution = solve_separable(&ode, &y, &x).unwrap();

        // Assert - check solution form
        // Expected: y = x²/2 + C
        assert_contains_term(&solution, &expr!(x^2 / 2));
        assert_contains_integration_constant(&solution);

        // Verify solution by substituting back into ODE
        assert_satisfies_ode(&solution, &ode, &y, &x);
    }

    #[test]
    fn test_ode_separable_complex() {
        // Test: dy/dx = x*y
        // Expected: y = C*exp(x²/2)
        let x = symbol!(x);
        let y = symbol!(y);
        let ode = expr!(y.diff(x) = x * y);

        let solution = solve_separable(&ode, &y, &x).unwrap();

        // Verify exponential form
        assert_has_exponential_form(&solution);
        assert_contains_integration_constant(&solution);
        assert_satisfies_ode(&solution, &ode, &y, &x);
    }

    #[test]
    fn test_ode_separable_edge_case_division_by_zero() {
        // Test: dy/dx = x/y (requires y ≠ 0)
        let x = symbol!(x);
        let y = symbol!(y);
        let ode = expr!(y.diff(x) = x / y);

        let result = solve_separable(&ode, &y, &x);

        // Should either:
        // 1. Return solution with domain restriction y ≠ 0
        // 2. Include y=0 as separate solution
        assert!(result.is_ok());
        let solution = result.unwrap();

        // Verify domain restriction is documented
        assert!(solution.has_domain_restriction());
    }

    #[test]
    fn test_ode_not_separable() {
        // Test: dy/dx = x + y (linear, not separable)
        let x = symbol!(x);
        let y = symbol!(y);
        let ode = expr!(y.diff(x) = x + y);

        let result = solve_separable(&ode, &y, &x);

        // Should fail gracefully with clear error
        assert!(matches!(result, Err(ODEError::NotSeparable)));
    }
}
```

### Edge Case Catalog

For each algorithm, test:

1. **Boundary Conditions**:
   - Zero inputs
   - Infinite limits
   - Undefined points

2. **Domain Restrictions**:
   - Division by zero
   - Square root of negatives
   - Logarithm of non-positives
   - Singular points

3. **Special Cases**:
   - Trivial solutions (dy/dx = 0 → y = C)
   - Identity transformations
   - Degenerate cases

4. **Type Boundaries**:
   - Integer overflow
   - Float precision limits
   - Rational reduction

---

## Level 2: Property-Based Testing

### Mathematical Properties to Verify

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_ode_solution_satisfies_equation(
        a in -10.0..10.0f64,
        b in -10.0..10.0f64
    ) {
        // Generate random ODE: dy/dx = a*x + b
        let x = symbol!(x);
        let y = symbol!(y);
        let ode = expr!(y.diff(x) = a*x + b);

        let solution = solve_linear_first_order(&ode, &y, &x).unwrap();

        // Property: Solution must satisfy the ODE
        assert_satisfies_ode(&solution, &ode, &y, &x);
    }

    #[test]
    fn test_matrix_qr_decomposition_correctness(
        matrix_elements in prop::collection::vec(-100.0..100.0f64, 9)
    ) {
        // Create 3x3 matrix
        let matrix = Matrix::from_vec(3, 3, matrix_elements);

        if let Ok((Q, R)) = qr_decomposition(&matrix) {
            // Property 1: Q is orthogonal (Q^T * Q = I)
            let qt_q = Q.transpose() * Q;
            assert_matrix_near_identity(&qt_q, 1e-10);

            // Property 2: R is upper triangular
            assert!(R.is_upper_triangular());

            // Property 3: A = Q * R
            let reconstructed = Q * R;
            assert_matrix_near_equal(&matrix, &reconstructed, 1e-10);
        }
    }

    #[test]
    fn test_polynomial_factorization_completeness(
        degree in 1..=5usize,
        coeffs in prop::collection::vec(-10..=10i64, 1..=6)
    ) {
        // Generate random polynomial
        let x = symbol!(x);
        let poly = polynomial_from_coefficients(&x, &coeffs);

        if let Ok(factors) = factor_polynomial(&poly, &x) {
            // Property: Product of factors equals original polynomial
            let reconstructed = multiply_factors(&factors);
            let simplified = reconstructed.simplify();
            assert_expressions_equivalent(&poly, &simplified);
        }
    }
}
```

### Algebraic Properties

Test these properties for all applicable operations:

1. **Commutativity**: `a + b = b + a`, `a * b = b * a`
2. **Associativity**: `(a + b) + c = a + (b + c)`
3. **Identity**: `a + 0 = a`, `a * 1 = a`, `a^1 = a`
4. **Inverse**: `a - a = 0`, `a / a = 1` (when a ≠ 0)
5. **Distributivity**: `a * (b + c) = a*b + a*c`
6. **Idempotence**: `simplify(simplify(x)) = simplify(x)`

---

## Level 3: Oracle Validation (SymPy Comparison)

### Test Oracle Structure

```json
{
  "metadata": {
    "generated_at": "2025-10-22T04:00:00Z",
    "sympy_version": "1.13.3",
    "total_test_cases": 547
  },
  "test_cases": {
    "ode_first_order_separable": [
      {
        "type": "ode_first_order_separable",
        "description": "dy/dx = x",
        "input": {
          "ode": "Eq(Derivative(y(x), x), x)",
          "dependent_var": "y",
          "independent_var": "x"
        },
        "expected_output": "Eq(y(x), C1 + x**2/2)",
        "sympy_version": "1.13.3",
        "difficulty": "simple"
      }
    ]
  }
}
```

### Oracle Validation Tests

```rust
#[test]
fn test_wave1_oracle_validation() {
    // Load test oracle
    let oracle_path = ".research/test_oracle.json";
    let oracle: TestOracle = load_oracle(oracle_path).expect("Failed to load oracle");

    let mut passed = 0;
    let mut failed = 0;
    let mut failures = Vec::new();

    // Test all ODE cases
    for category in ["ode_first_order_separable", "ode_first_order_linear", "ode_second_order_const_coeff"] {
        if let Some(test_cases) = oracle.test_cases.get(category) {
            for (i, case) in test_cases.iter().enumerate() {
                // Parse input
                let ode = parse_expression(&case.input.ode).unwrap();
                let y = Symbol::new(&case.input.dependent_var);
                let x = Symbol::new(&case.input.independent_var);

                // Solve with MathHook
                let mathhook_result = solve_ode(&ode, &y, &x);

                match mathhook_result {
                    Ok(mathhook_solution) => {
                        // Parse SymPy expected output
                        let sympy_solution = parse_expression(&case.expected_output).unwrap();

                        // Check mathematical equivalence
                        if are_solutions_equivalent(&mathhook_solution, &sympy_solution, &y, &x) {
                            passed += 1;
                        } else {
                            failed += 1;
                            failures.push(OracleFailure {
                                category: category.to_string(),
                                test_index: i,
                                description: case.description.clone(),
                                mathhook_output: mathhook_solution.to_string(),
                                expected_output: case.expected_output.clone(),
                                reason: "Mathematical equivalence check failed".to_string(),
                            });
                        }
                    },
                    Err(e) => {
                        failed += 1;
                        failures.push(OracleFailure {
                            category: category.to_string(),
                            test_index: i,
                            description: case.description.clone(),
                            mathhook_output: "ERROR".to_string(),
                            expected_output: case.expected_output.clone(),
                            reason: format!("Solver error: {}", e),
                        });
                    }
                }
            }
        }
    }

    // Report results
    println!("\nOracle Validation Results:");
    println!("  Passed: {}/{}", passed, passed + failed);
    println!("  Failed: {}/{}", failed, passed + failed);
    println!("  Pass Rate: {:.2}%", (passed as f64 / (passed + failed) as f64) * 100.0);

    if !failures.is_empty() {
        println!("\nFailures:");
        for failure in &failures {
            println!("  [{}] Test {}: {}", failure.category, failure.test_index, failure.description);
            println!("    Reason: {}", failure.reason);
            println!("    Expected: {}", failure.expected_output);
            println!("    Got: {}", failure.mathhook_output);
        }
    }

    // CRITICAL: 100% pass rate required
    assert_eq!(failed, 0, "Oracle validation failed: {} test cases failed", failed);
}
```

### Solution Equivalence Checking

Critical: Two solutions may look different but be mathematically equivalent.

```rust
/// Check if two solutions are mathematically equivalent
fn are_solutions_equivalent(
    sol1: &Expression,
    sol2: &Expression,
    var: &Symbol,
    indep_var: &Symbol
) -> bool {
    // Strategy 1: Simplify difference
    let diff = Expression::sub(sol1.clone(), sol2.clone()).simplify();
    if is_constant_wrt_variable(&diff, var) {
        // Difference is constant - solutions equivalent up to integration constant
        return true;
    }

    // Strategy 2: Verify both satisfy same ODE
    // Extract ODE from solution form
    // Check both satisfy it

    // Strategy 3: Numerical sampling
    // Sample at multiple points and compare
    for _ in 0..100 {
        let test_point = random_value();
        let val1 = sol1.substitute(indep_var, &test_point).evaluate_numerical();
        let val2 = sol2.substitute(indep_var, &test_point).evaluate_numerical();

        if !values_near_equal(val1, val2, 1e-10) {
            return false;
        }
    }

    true
}
```

---

## Level 4: Cross-Reference Validation

### Reference Sources

1. **SymPy** (primary): Algorithm implementations and test cases
2. **Symbolica** (Rust reference): Rust-specific patterns
3. **Wolfram MathWorld**: Mathematical definitions and properties
4. **DLMF**: Digital Library of Mathematical Functions (special functions)
5. **Numerical Recipes**: Numerical algorithm correctness

### Validation Against Published Algorithms

For each algorithm:
1. Document source (textbook, paper, reference implementation)
2. Verify algorithm matches published version
3. Test against published example cases
4. Check edge cases mentioned in literature

Example:
```markdown
## ODE Separable Algorithm Validation

### Reference
- **Source**: "Elementary Differential Equations" by Boyce & DiPrima, 11th Ed., Section 2.2
- **Algorithm**: Separation of Variables Method
- **Page**: 42-48

### Algorithm Steps (from reference):
1. Write equation in form: M(x) + N(y)dy/dx = 0
2. Separate: M(x)dx = -N(y)dy
3. Integrate both sides
4. Solve for y if possible

### MathHook Implementation:
- Location: `crates/mathhook-core/src/ode/first_order/separable.rs`
- Matches reference: ✓
- Handles edge cases (division by zero): ✓

### Test Cases from Reference:
- Example 1 (page 43): dy/dx = x/y → Verified ✓
- Example 2 (page 45): dy/dx = -x/y → Verified ✓
- Example 3 (page 46): dy/dx = x²y → Verified ✓
```

---

## Level 5: Educational Validation

### Explanation Correctness

Educational explanations must be:
1. **Mathematically accurate**: No simplifications that change meaning
2. **Complete**: All assumptions stated
3. **Clear domain restrictions**: User understands when solution is valid
4. **Edge case handling**: Explains special cases

### Validation Process

```rust
#[test]
fn test_educational_explanation_correctness() {
    let x = symbol!(x);
    let y = symbol!(y);
    let ode = expr!(y.diff(x) = x * y);

    let (solution, steps) = solve_separable_with_steps(&ode, &y, &x).unwrap();

    // Verify explanation steps
    assert!(steps.len() > 0, "No explanation steps generated");

    // Check key steps are present
    assert_contains_step(&steps, "Identify ODE type");
    assert_contains_step(&steps, "Separate variables");
    assert_contains_step(&steps, "Integrate both sides");
    assert_contains_step(&steps, "Solve for y");

    // Verify each step is mathematically sound
    for step in &steps {
        // Verify transformation is valid
        assert_valid_transformation(&step.expression_before, &step.expression_after);

        // Verify reasoning makes sense
        assert!(!step.reasoning.is_empty());
    }

    // Verify domain restrictions mentioned
    assert_mentions_domain_restrictions(&steps);
}
```

---

## Validation Metrics

### Coverage Requirements

| Category | Target | Measured By |
|----------|--------|-------------|
| Unit test coverage | 100% | Line coverage |
| Property tests | All mathematical properties | Property test suite |
| Oracle validation | 100% pass rate | SymPy comparison |
| Edge cases | All identified cases | Edge case catalog |
| Educational accuracy | 100% | Manual review |

### Quality Gates

**Before Wave Completion**:
- [ ] All unit tests passing
- [ ] Property tests passing (no failures in 1000 iterations)
- [ ] 100% oracle validation pass rate
- [ ] All edge cases documented and tested
- [ ] Educational explanations reviewed and accurate
- [ ] No mathematical errors found in code review

**Before Merging**:
- [ ] No test regressions
- [ ] Performance targets met
- [ ] Documentation complete
- [ ] Code review approved
- [ ] CI passing

---

## Validation Tools

### Assertion Helpers

```rust
/// Assert two expressions are mathematically equivalent
pub fn assert_expressions_equivalent(e1: &Expression, e2: &Expression) {
    let diff = (e1.clone() - e2.clone()).simplify();
    assert!(diff.is_zero(), "Expressions not equivalent:\n  Left: {}\n  Right: {}\n  Difference: {}", e1, e2, diff);
}

/// Assert solution satisfies ODE
pub fn assert_satisfies_ode(
    solution: &Expression,
    ode: &Expression,
    dependent: &Symbol,
    independent: &Symbol
) {
    // Substitute solution into ODE
    let lhs = ode.lhs().substitute(dependent, solution);
    let rhs = ode.rhs().substitute(dependent, solution);

    // Compute derivatives if needed
    let lhs_simplified = lhs.simplify();
    let rhs_simplified = rhs.simplify();

    assert_expressions_equivalent(&lhs_simplified, &rhs_simplified);
}

/// Assert matrix is near identity
pub fn assert_matrix_near_identity(matrix: &Matrix, tolerance: f64) {
    assert!(matrix.is_square());
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            let expected = if i == j { 1.0 } else { 0.0 };
            let actual = matrix.get(i, j).evaluate_numerical().unwrap();
            assert!(
                (actual - expected).abs() < tolerance,
                "Matrix not near identity at ({}, {}): expected {}, got {}",
                i, j, expected, actual
            );
        }
    }
}
```

---

## Continuous Validation

### CI Integration

```yaml
name: Mathematical Correctness

on: [push, pull_request]

jobs:
  validation:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Unit Tests
        run: cargo test --all

      - name: Property Tests
        run: cargo test --features proptest

      - name: Oracle Validation
        run: cargo test oracle_validation

      - name: Educational Validation
        run: cargo test educational

      - name: Check Coverage
        run: |
          cargo tarpaulin --out Xml
          # Fail if coverage < 100% for core math modules

      - name: Validate No Regressions
        run: |
          cargo test --all -- --test-threads=1
          # Check test count didn't decrease
```

---

## Error Reporting

When validation fails, provide actionable information:

```rust
#[derive(Debug)]
struct ValidationFailure {
    test_name: String,
    category: String,
    input: String,
    expected: String,
    actual: String,
    diff: String,
    suggestions: Vec<String>,
}

impl std::fmt::Display for ValidationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "
Validation Failure: {}
Category: {}

Input:
  {}

Expected:
  {}

Got:
  {}

Difference:
  {}

Suggestions:
{}
", self.test_name, self.category, self.input, self.expected, self.actual, self.diff,
   self.suggestions.iter().map(|s| format!("  - {}", s)).collect::<Vec<_>>().join("\n"))
    }
}
```

---

## Success Criteria

### Wave Completion Checklist

- [ ] 100% unit test pass rate
- [ ] 100% oracle validation pass rate
- [ ] 100% property test pass rate (1000+ iterations)
- [ ] All edge cases tested and passing
- [ ] Educational explanations validated
- [ ] No known mathematical errors
- [ ] Code review approved
- [ ] Documentation complete

### Quality Metrics

- **Zero tolerance for mathematical errors**
- **100% correctness validation** before wave completion
- **Comprehensive edge case coverage**
- **Educational accuracy** verified

---

## Conclusion

This validation plan ensures:
1. **Mathematical correctness** through multi-level validation
2. **Comprehensive testing** of all edge cases
3. **Oracle-based validation** against SymPy
4. **Property-based testing** for mathematical properties
5. **Educational accuracy** for teaching purposes

**Remember**: Mathematical correctness is the highest priority. Performance is secondary to correctness.
