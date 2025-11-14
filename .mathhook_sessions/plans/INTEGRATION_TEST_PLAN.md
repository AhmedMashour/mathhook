# Integration Enhancement Test Plan: Comprehensive Validation Strategy

## Executive Summary

This document defines the comprehensive testing strategy for validating MathHook's integration enhancement across Waves 2-6. The plan includes 200+ test cases organized into 7 categories, SymPy validation methodology, performance benchmarks, and regression testing protocols. The goal is to verify 93-95 percent coverage while maintaining mathematical correctness and performance targets.

## Test Organization

### Test Categories Overview

| Category | Wave | Test Count | Purpose | Priority |
|----------|------|------------|---------|----------|
| Rational Functions | 2 | 35 | Partial fractions, P(x)/Q(x) | HIGH |
| Trigonometric | 4 | 35 | sin^m·cos^n patterns | MEDIUM |
| Substitution | 3 | 30 | U-substitution, chain rule | HIGH |
| Table Lookups | 3 | 30 | Common pattern O(1) lookup | HIGH |
| Risch Algorithm | 5 | 30 | Hard cases, non-elementary | MEDIUM |
| Combined Techniques | 6 | 20 | Multi-step integrals | HIGH |
| Edge Cases | All | 20 | Domain, special values | HIGH |
| **Total** | - | **200** | - | - |

### Test Execution Strategy

**Phase 1: Unit Tests** - Each wave implements technique-specific tests
- Test individual algorithms in isolation
- Fast execution (< 1 second per test)
- Run continuously during development

**Phase 2: Integration Tests** - Verify technique composition
- Test strategy dispatcher routing
- Verify fallback behavior
- Validate technique precedence

**Phase 3: SymPy Validation** - Cross-validate all results
- Compare 150+ integrals against SymPy
- Verify mathematical equivalence (not just string equality)
- Detect regressions early

**Phase 4: Performance Benchmarks** - Measure and optimize
- Technique hit rates
- Average execution time per technique
- Memory usage profiling

**Phase 5: Regression Suite** - Protect existing functionality
- 18 existing tests must continue passing
- Add new tests for each bug fix
- Automated CI/CD validation

## Category 1: Rational Functions (35 Tests)

**Purpose**: Validate partial fraction decomposition and rational function integration.

### Proper Fractions (deg(P) < deg(Q)) - 15 Tests

**Linear Factors (Single)**:
1. `∫1/(x-1) dx` → `ln|x-1| + C`
2. `∫1/(x+2) dx` → `ln|x+2| + C`
3. `∫3/(2x-5) dx` → `(3/2)ln|2x-5| + C`

**Linear Factors (Multiple Distinct)**:
4. `∫1/((x-1)(x-2)) dx` → Partial fractions: `A/(x-1) + B/(x-2)`
   - Result: `ln|x-1| - ln|x-2| + C`
5. `∫1/((x+1)(x+3)) dx` → `(1/2)ln|x+1| - (1/2)ln|x+3| + C`
6. `∫(2x+3)/((x-1)(x+2)) dx` → Linear numerator with linear factors

**Linear Factors (Repeated)**:
7. `∫1/(x-1)² dx` → `-1/(x-1) + C`
8. `∫1/(x-1)³ dx` → `-1/(2(x-1)²) + C`
9. `∫x/(x-1)² dx` → `ln|x-1| + 1/(x-1) + C`
10. `∫1/((x-1)²(x+1)) dx` → Mixed repeated and simple factors

**Irreducible Quadratic Factors**:
11. `∫1/(x²+1) dx` → `arctan(x) + C`
12. `∫1/(x²+4) dx` → `(1/2)arctan(x/2) + C`
13. `∫x/(x²+1) dx` → `(1/2)ln(x²+1) + C` (logarithmic derivative)
14. `∫(2x+3)/(x²+4) dx` → `ln(x²+4) + (3/2)arctan(x/2) + C`
15. `∫1/(x²-1) dx` → `(1/2)ln|(x-1)/(x+1)| + C` (difference of logs)

### Improper Fractions (deg(P) ≥ deg(Q)) - 10 Tests

**Requires Polynomial Division**:
16. `∫x²/(x-1) dx` → `x²/2 + x + ln|x-1| + C`
17. `∫(x²+1)/(x-1) dx` → Division: `x+1 + 2/(x-1)`
18. `∫x³/(x²-1) dx` → `x²/2 + (1/2)ln|x²-1| + C`
19. `∫(x⁴+x²+1)/(x²+1) dx` → Polynomial + arctan term
20. `∫(2x³-x²+3x-1)/(x²-x-2) dx` → Complex division

**Mixed Complexity**:
21. `∫(3x²+2x+1)/(x³-x) dx` → Factor denominator: `x(x-1)(x+1)`
22. `∫(x³+2x²-x+3)/(x²-4) dx` → Division + partial fractions
23. `∫(x⁴-2x²+1)/(x³-x) dx` → High-degree numerator
24. `∫(5x²+20x+6)/(x³-2x²) dx` → Repeated root in denominator
25. `∫(x³+1)/(x⁴+x²) dx` → Factorizable denominator

### Complex Cases - 10 Tests

**Quadratic Factors (Repeated)**:
26. `∫1/(x²+1)² dx` → Reduction formula
27. `∫x/(x²+1)² dx` → `-(1/2)/(x²+1) + C`
28. `∫(2x+1)/(x²+1)² dx` → Mix of techniques

**Mixed Linear and Quadratic**:
29. `∫1/((x-1)(x²+1)) dx` → `A/(x-1) + (Bx+C)/(x²+1)`
30. `∫x/((x+2)(x²+4)) dx` → Three distinct factors

**Edge Cases**:
31. `∫1/(x²-4) dx` → Factorable quadratic: `(x-2)(x+2)`
32. `∫1/(x⁴-1) dx` → Factor: `(x-1)(x+1)(x²+1)`
33. `∫(x²+x+1)/(x³+1) dx` → Complex factorization
34. `∫1/(x⁵+x) dx` → `∫1/(x(x⁴+1)) dx`
35. `∫(x+1)/(x³-x²-x+1) dx` → Fully factorizable cubic

### Validation Requirements

**For Each Test**:
1. Verify result differentiates back to integrand: `d/dx(result) == integrand`
2. Compare against SymPy result
3. Check domain restrictions (e.g., ln requires x > 0 or absolute value)
4. Verify simplification of final result

## Category 2: Trigonometric Integrals (35 Tests)

**Purpose**: Validate sin^m·cos^n pattern recognition and integration.

### Odd Sin Power - 7 Tests

1. `∫sin(x) dx` → `-cos(x) + C`
2. `∫sin³(x) dx` → `-cos(x) + (1/3)cos³(x) + C`
3. `∫sin⁵(x) dx` → Reduction formula
4. `∫sin(x)·cos²(x) dx` → Odd sin, even cos
5. `∫sin³(x)·cos²(x) dx` → `u = cos(x)` substitution
6. `∫sin⁵(x)·cos⁴(x) dx` → High powers
7. `∫sin(2x)·cos²(2x) dx` → Non-unit coefficient

### Odd Cos Power - 7 Tests

8. `∫cos(x) dx` → `sin(x) + C`
9. `∫cos³(x) dx` → `sin(x) - (1/3)sin³(x) + C`
10. `∫cos⁵(x) dx` → Reduction formula
11. `∫sin²(x)·cos(x) dx` → Even sin, odd cos
12. `∫sin²(x)·cos³(x) dx` → `u = sin(x)` substitution
13. `∫sin⁴(x)·cos⁵(x) dx` → High powers
14. `∫sin²(3x)·cos(3x) dx` → Non-unit coefficient

### Even Powers (Both) - 7 Tests

15. `∫sin²(x) dx` → `x/2 - sin(2x)/4 + C`
16. `∫cos²(x) dx` → `x/2 + sin(2x)/4 + C`
17. `∫sin²(x)·cos²(x) dx` → `x/8 - sin(4x)/32 + C`
18. `∫sin⁴(x) dx` → Multiple half-angle applications
19. `∫cos⁴(x) dx` → Multiple half-angle applications
20. `∫sin²(x)·cos⁴(x) dx` → High even powers
21. `∫sin⁶(x)·cos²(x) dx` → Very high powers

### Mixed Techniques - 7 Tests

22. `∫tan(x) dx` → `-ln|cos(x)| + C`
23. `∫cot(x) dx` → `ln|sin(x)| + C`
24. `∫sec²(x) dx` → `tan(x) + C`
25. `∫csc²(x) dx` → `-cot(x) + C`
26. `∫sec(x)·tan(x) dx` → `sec(x) + C`
27. `∫csc(x)·cot(x) dx` → `-csc(x) + C`
28. `∫tan²(x) dx` → `tan(x) - x + C`

### Special Cases - 7 Tests

29. `∫sin(x)·cos(x) dx` → `(1/2)sin²(x) + C` or `-(1/2)cos²(x) + C`
30. `∫sin(2x) dx` → `-(1/2)cos(2x) + C`
31. `∫cos(3x) dx` → `(1/3)sin(3x) + C`
32. `∫sin(ax)·cos(bx) dx` → Product-to-sum formulas
33. `∫sin²(x)·tan(x) dx` → Rewrite tan as sin/cos
34. `∫cos(x)/(1+sin(x)) dx` → Logarithmic derivative
35. `∫sin(x)/(1+cos(x)) dx` → Logarithmic derivative

### Validation Requirements

**For Each Test**:
1. Verify fundamental theorem: `d/dx(result) == integrand`
2. Compare against SymPy (allow equivalent forms)
3. Test reduction formula correctness recursively
4. Verify half-angle expansion accuracy

## Category 3: Substitution (U-Substitution) - 30 Tests

**Purpose**: Validate automatic u-substitution detection and application.

### Chain Rule Recognition - 10 Tests

1. `∫2x·sin(x²) dx` → `u = x²`, `du = 2x dx`, result: `-cos(x²) + C`
2. `∫3x²·e^(x³) dx` → `u = x³`, result: `e^(x³) + C`
3. `∫cos(x)·e^(sin(x)) dx` → `u = sin(x)`, result: `e^(sin(x)) + C`
4. `∫(2x)/(x²+1) dx` → `u = x²+1`, result: `ln(x²+1) + C`
5. `∫x/√(x²+1) dx` → `u = x²+1`, result: `√(x²+1) + C`
6. `∫sec²(x)·tan(x) dx` → `u = tan(x)`, result: `(1/2)tan²(x) + C`
7. `∫sin(x)·cos⁴(x) dx` → `u = cos(x)`, result: `-(1/5)cos⁵(x) + C`
8. `∫e^x/(1+e^x) dx` → `u = e^x`, result: `ln(1+e^x) + C`
9. `∫ln(x)/x dx` → `u = ln(x)`, result: `(1/2)(ln(x))² + C`
10. `∫(ln(x))²/x dx` → `u = ln(x)`, result: `(1/3)(ln(x))³ + C`

### Composite Functions - 10 Tests

11. `∫cos(ln(x))/x dx` → `u = ln(x)`, result: `sin(ln(x)) + C`
12. `∫e^√x/√x dx` → `u = √x`, result: `2e^√x + C`
13. `∫sin(e^x)·e^x dx` → `u = e^x`, result: `-cos(e^x) + C`
14. `∫(arctan(x))/(1+x²) dx` → `u = arctan(x)`, result: `(1/2)(arctan(x))² + C`
15. `∫e^(arcsin(x))/√(1-x²) dx` → `u = arcsin(x)`, result: `e^(arcsin(x)) + C`
16. `∫tan(x)·ln(cos(x)) dx` → `u = ln(cos(x))`, result: `-(1/2)(ln(cos(x)))² + C`
17. `∫x·e^(-x²) dx` → `u = -x²`, result: `-(1/2)e^(-x²) + C`
18. `∫x³·cos(x⁴) dx` → `u = x⁴`, result: `(1/4)sin(x⁴) + C`
19. `∫(e^x-e^(-x))/(e^x+e^(-x)) dx` → `u = e^x+e^(-x)`, result: `ln(e^x+e^(-x)) + C`
20. `∫sin(x)·e^(cos(x)) dx` → `u = cos(x)`, result: `-e^(cos(x)) + C`

### Non-Obvious Substitutions - 10 Tests

21. `∫√(1+√x)/√x dx` → Nested radicals
22. `∫1/(x·ln(x)·ln(ln(x))) dx` → `u = ln(ln(x))`, result: `ln(ln(ln(x))) + C`
23. `∫e^(2x)/(1+e^x) dx` → `u = e^x`, simplify to rational
24. `∫sin(x)/(1+cos²(x)) dx` → `u = cos(x)`, result: `-arctan(cos(x)) + C`
25. `∫x/((1+x²)√(1+x²)) dx` → `u = 1+x²`
26. `∫(arcsin(x))²/√(1-x²) dx` → `u = arcsin(x)`, result: `(1/3)(arcsin(x))³ + C`
27. `∫tan(√x)/√x dx` → `u = √x`, result: `-2ln|cos(√x)| + C`
28. `∫e^x·sin(e^x)·cos(e^x) dx` → `u = e^x`, then trig
29. `∫(x²+1)/(x⁴+1) dx` → Clever factorization then substitution
30. `∫1/(e^x+e^(-x)) dx` → Multiply by e^x, then substitute

### Validation Requirements

**For Each Test**:
1. Verify derivative detection: `du` appears in integrand
2. Confirm transformation simplifies integral
3. Verify back-substitution correctness
4. Compare against SymPy

## Category 4: Table Lookups (30 Tests)

**Purpose**: Validate O(1) pattern matching for common integrals.

### Standard Forms - 10 Tests

1. `∫e^x dx` → `e^x + C`
2. `∫a^x dx` → `a^x/ln(a) + C` (for a > 0, a ≠ 1)
3. `∫1/x dx` → `ln|x| + C`
4. `∫1/(a²+x²) dx` → `(1/a)arctan(x/a) + C`
5. `∫1/√(a²-x²) dx` → `arcsin(x/a) + C`
6. `∫1/√(x²+a²) dx` → `ln|x + √(x²+a²)| + C` (arcsinh form)
7. `∫1/√(x²-a²) dx` → `ln|x + √(x²-a²)| + C` (arccosh form)
8. `∫1/(x√(x²-a²)) dx` → `(1/a)arcsec(x/a) + C`
9. `∫√(a²-x²) dx` → `(x/2)√(a²-x²) + (a²/2)arcsin(x/a) + C`
10. `∫√(x²+a²) dx` → `(x/2)√(x²+a²) + (a²/2)ln|x + √(x²+a²)| + C`

### Exponential and Logarithmic - 5 Tests

11. `∫x·e^x dx` → `x·e^x - e^x + C`
12. `∫x²·e^x dx` → `x²·e^x - 2x·e^x + 2e^x + C`
13. `∫ln(x) dx` → `x·ln(x) - x + C`
14. `∫x·ln(x) dx` → `(x²/2)·ln(x) - x²/4 + C`
15. `∫e^(ax)·sin(bx) dx` → `e^(ax)[a·sin(bx) - b·cos(bx)]/(a²+b²) + C`

### Trigonometric Forms - 5 Tests

16. `∫sec(x) dx` → `ln|sec(x) + tan(x)| + C`
17. `∫csc(x) dx` → `-ln|csc(x) + cot(x)| + C`
18. `∫arcsin(x) dx` → `x·arcsin(x) + √(1-x²) + C`
19. `∫arctan(x) dx` → `x·arctan(x) - (1/2)ln(1+x²) + C`
20. `∫x·sin(x) dx` → `-x·cos(x) + sin(x) + C`

### Hyperbolic Forms - 5 Tests

21. `∫sinh(x) dx` → `cosh(x) + C`
22. `∫cosh(x) dx` → `sinh(x) + C`
23. `∫tanh(x) dx` → `ln(cosh(x)) + C`
24. `∫sech²(x) dx` → `tanh(x) + C`
25. `∫csch(x)·coth(x) dx` → `-csch(x) + C`

### Special Combinations - 5 Tests

26. `∫x/√(a²+x²) dx` → `√(a²+x²) + C`
27. `∫(ax+b)^n dx` → `(ax+b)^(n+1)/(a(n+1)) + C` (n ≠ -1)
28. `∫1/(ax+b) dx` → `(1/a)ln|ax+b| + C`
29. `∫√(ax+b) dx` → `(2/3a)(ax+b)^(3/2) + C`
30. `∫x√(ax+b) dx` → `(2/15a²)(3ax-2b)(ax+b)^(3/2) + C`

### Validation Requirements

**For Each Test**:
1. Verify O(1) lookup time (pattern matching performance)
2. Confirm result correctness via differentiation
3. Check for equivalent forms in table
4. Compare against SymPy

## Category 5: Risch Algorithm (30 Tests)

(See `RISCH_ALGORITHM_DESIGN.md` for complete test specifications)

**Purpose**: Validate exponential/logarithmic extensions and non-elementary detection.

### Test Breakdown
- 10 tests: Exponential extensions
- 10 tests: Logarithmic extensions
- 5 tests: Mixed towers
- 5 tests: Non-elementary detection

## Category 6: Combined Techniques (20 Tests)

**Purpose**: Validate multi-step integrals requiring multiple techniques.

### Two-Step Integrals - 10 Tests

1. `∫(x²+1)/(x³+3x+1) dx` → Polynomial division + partial fractions
2. `∫x²·sin(x) dx` → By parts twice
3. `∫e^x·cos(x) dx` → By parts twice (cyclic)
4. `∫(sin(x))³ dx` → Trigonometric reduction + u-substitution
5. `∫ln(x²+1) dx` → By parts + arctan
6. `∫x·arctan(x) dx` → By parts + substitution
7. `∫e^(√x) dx` → Substitution (u=√x) + exponential
8. `∫sin(ln(x)) dx` → Substitution (u=ln(x)) + trigonometric
9. `∫(x+1)·e^(x²+2x) dx` → Recognize d/dx(x²+2x) + exponential
10. `∫tan²(x)·sec²(x) dx` → Substitution (u=tan(x)) + power rule

### Three-Step Integrals - 10 Tests

11. `∫x²·e^x·sin(x) dx` → By parts multiple times + trig
12. `∫(ln(x))²/x dx` → Substitution + by parts
13. `∫x·√(x²+1) dx` → Substitution + power rule
14. `∫sin(x)·ln(cos(x)) dx` → Substitution + by parts
15. `∫e^(2x)/(1+e^x)² dx` → Substitution + partial fractions + simplification
16. `∫(x³+x)/((x²+1)²) dx` → Polynomial division + partial fractions + arctan
17. `∫sec³(x) dx` → By parts + reduction formula
18. `∫csc³(x) dx` → By parts + reduction formula
19. `∫x²·arctan(x) dx` → By parts + arctan integration
20. `∫√(x²-1)/x dx` → Trigonometric substitution + simplification

### Validation Requirements

**For Each Test**:
1. Document technique sequence used
2. Verify each intermediate step
3. Confirm final result correctness
4. Compare against SymPy (may use different technique order)

## Category 7: Edge Cases (20 Tests)

**Purpose**: Validate domain restrictions, special values, and error handling.

### Domain Restrictions - 5 Tests

1. `∫1/(x²-4) dx` at x=2 → Check discontinuity handling
2. `∫1/√x dx` at x=0 → Check domain restriction
3. `∫ln(x) dx` at x=0 → Check logarithm domain
4. `∫1/√(1-x²) dx` at x=±1 → Check arcsin domain
5. `∫tan(x) dx` at x=π/2 → Check tangent discontinuity

### Special Values - 5 Tests

6. `∫0 dx` → `C` (constant)
7. `∫1 dx` → `x + C`
8. `∫x^0 dx` → `x + C` (not ln|x|!)
9. `∫e^0 dx` → `x + C` (since e^0 = 1)
10. `∫sin(0) dx` → `0·x + C = C`

### Definite Integrals - 5 Tests

11. `∫₀¹ x dx` → `1/2`
12. `∫₀^π sin(x) dx` → `2`
13. `∫₁^e 1/x dx` → `1`
14. `∫₀^∞ e^(-x) dx` → `1` (improper integral)
15. `∫₋₁¹ x³ dx` → `0` (odd function symmetry)

### Error Cases - 5 Tests

16. `∫1/x dx` from -1 to 1 → Error (discontinuity at 0)
17. `∫1/√x dx` from 0 to 1 → Improper integral (convergent)
18. `∫e^(x²) dx` → Non-elementary (return symbolic or error)
19. `∫sin(x)/x dx` → Non-elementary (Si function)
20. `∫unknown_function(x) dx` → Graceful fallback to symbolic

### Validation Requirements

**For Each Test**:
1. Verify domain checking logic
2. Confirm error messages are helpful
3. Test boundary condition handling
4. Ensure no panics on invalid input

## SymPy Validation Methodology

### Equivalence Testing

**Challenge**: Two correct antiderivatives may look different:
- `(1/2)ln|x²-1|` vs `ln|x-1| - ln|x+1|`
- `sin²(x)` vs `(1 - cos(2x))/2`

**Solution**: Derivative comparison
```rust
fn assert_equivalent(mathhook_result: &Expression, sympy_result: &Expression, variable: Symbol) {
    let mathhook_derivative = mathhook_result.derivative(variable.clone()).simplify();
    let sympy_derivative = sympy_result.derivative(variable.clone()).simplify();

    assert_eq!(mathhook_derivative, sympy_derivative,
        "Results differ: MathHook={}, SymPy={}", mathhook_result, sympy_result);
}
```

### SymPy Execution

**Method 1**: Offline comparison (pre-computed SymPy results)
```rust
const SYMPY_RESULTS: &[(&str, &str)] = &[
    ("integrate(x**2, x)", "x**3/3"),
    ("integrate(sin(x), x)", "-cos(x)"),
    // ... 150+ entries
];
```

**Method 2**: Runtime SymPy calls (requires Python)
```rust
fn run_sympy(expression: &str) -> Expression {
    let output = Command::new("python3")
        .arg("-c")
        .arg(format!("from sympy import *; x = symbols('x'); print({})", expression))
        .output()
        .expect("Failed to run SymPy");

    parse_sympy_output(&output.stdout)
}
```

### Validation Suite Execution

**Automated CI/CD**:
```bash
# Run all SymPy validation tests
cargo test --test sympy_validation -- --test-threads=1

# Generate validation report
cargo test --test sympy_validation -- --nocapture > validation_report.txt
```

**Expected Output**:
```
SymPy Validation Report
=======================
Total tests: 150
Passed: 142 (94.7%)
Failed: 5 (3.3%)
Skipped: 3 (2.0%)

Failed tests:
- test_rational_35: Result differs (numerical accuracy)
- test_trig_28: SymPy uses different form
- ...
```

## Performance Benchmarks

### Benchmark Categories

**1. Technique Performance** - Average time per technique:
```rust
#[bench]
fn bench_table_lookup(b: &mut Bencher) {
    let x = symbol!(x);
    let expr = parse("tan(x)");
    b.iter(|| table::lookup(&expr, &x));
}

#[bench]
fn bench_rational_integration(b: &mut Bencher) {
    let x = symbol!(x);
    let expr = parse("(x+1)/(x^2-1)");
    b.iter(|| rational::integrate(&expr, &x));
}

// ... benchmarks for all techniques
```

**Targets**:
- Table lookup: < 0.1ms (O(1))
- Rational: < 10ms (polynomial operations)
- Substitution: < 5ms (pattern matching + transformation)
- Trigonometric: < 10ms (reduction formulas)
- Risch: < 100ms (exponential cases), < 1s (logarithmic cases)

**2. Strategy Dispatcher Overhead**:
```rust
#[bench]
fn bench_dispatcher_overhead(b: &mut Bencher) {
    let x = symbol!(x);
    let simple_expr = parse("x^2");  // Should hit fast path immediately
    b.iter(|| strategy.integrate(&simple_expr, x.clone()));
}
```

**Target**: < 0.5ms overhead for fast path

**3. Memory Usage**:
```rust
#[test]
fn test_memory_usage() {
    let x = symbol!(x);
    let expr = parse("(x^10 + 1)/(x^10 - 1)");  // Large expression

    let start_memory = current_memory_usage();
    let result = strategy.integrate(&expr, x);
    let peak_memory = current_memory_usage();

    assert!(peak_memory - start_memory < 100_KB, "Memory usage too high");
}
```

### Technique Hit Rate Analysis

**Metric**: Percentage of integrals resolved by each technique

```rust
struct TechniqueStats {
    table_hits: usize,
    rational_hits: usize,
    registry_hits: usize,
    by_parts_hits: usize,
    substitution_hits: usize,
    trigonometric_hits: usize,
    risch_hits: usize,
    symbolic_fallback: usize,
}

#[test]
fn analyze_technique_hit_rates() {
    let mut stats = TechniqueStats::default();

    for test_case in ALL_TEST_CASES {
        let result = strategy.integrate(&test_case.expr, test_case.var);
        match result.technique {
            TechniqueUsed::TableLookup => stats.table_hits += 1,
            TechniqueUsed::Rational => stats.rational_hits += 1,
            // ... count all techniques
        }
    }

    println!("Technique Hit Rates (n=200):");
    println!("  Table: {}%", (stats.table_hits * 100) / 200);
    println!("  Rational: {}%", (stats.rational_hits * 100) / 200);
    // ... print all rates
}
```

**Expected Rates** (target for 200 tests):
- Table: 25-30% (50-60 tests)
- Rational: 15-20% (30-40 tests)
- Registry: 15-20% (30-40 tests)
- Substitution: 10-15% (20-30 tests)
- Trigonometric: 5-8% (10-16 tests)
- By Parts: 5-10% (10-20 tests)
- Risch: 3-5% (6-10 tests)
- Symbolic: ~5% (10 tests)

## Regression Testing

### Protect Existing Functionality

**Baseline**: 18 existing tests must continue passing:
- 9 tests in `basic.rs`
- 4 tests in `by_parts.rs`
- 26 tests in `integral_registry_tests.rs` (active)

**Regression Suite**:
```rust
#[test]
fn test_no_regressions() {
    // All existing tests
    run_existing_tests();

    // New tests from each wave
    run_wave_2_tests();
    run_wave_3_tests();
    run_wave_4_tests();
    run_wave_5_tests();

    // Verify no functionality broke
    assert_all_passed();
}
```

### Continuous Integration

**CI Pipeline**:
1. Run full test suite on every commit
2. Benchmark performance (fail if > 10% slower)
3. Check code coverage (target: 85%+ for integration modules)
4. Validate SymPy equivalence (150+ tests)
5. Generate coverage report

**Failure Protocol**:
- Block merge if any test fails
- Require performance justification for slowdowns
- Mandate new tests for bug fixes

## Test Execution Timeline

### Wave 2 (Rational Functions + Strategy)
- 35 rational function tests
- 10 strategy dispatcher tests
- **Total new**: 45 tests
- **Running total**: 63 tests (18 existing + 45 new)

### Wave 3 (Table + Substitution)
- 30 table lookup tests
- 30 substitution tests
- **Total new**: 60 tests
- **Running total**: 123 tests

### Wave 4 (Trigonometric)
- 35 trigonometric tests
- **Total new**: 35 tests
- **Running total**: 158 tests

### Wave 5 (Risch)
- 30 Risch algorithm tests
- **Total new**: 30 tests
- **Running total**: 188 tests

### Wave 6 (Combined + Edge Cases + Validation)
- 20 combined technique tests
- 20 edge case tests
- 150+ SymPy validation tests (may overlap with above)
- **Total new**: 40 tests
- **Running total**: 228 tests

### Final Coverage
- **Unit tests**: 228 tests
- **SymPy validation**: 150 tests (with some overlap)
- **Performance benchmarks**: 20 benchmarks
- **Regression tests**: Continuous (all tests)

## Conclusion

This comprehensive test plan ensures MathHook's integration enhancement achieves 93-95% coverage with mathematical correctness guarantees:

**Test Coverage**: 200+ unit tests + 150 SymPy validation tests
**Performance Validation**: 20 benchmarks, technique hit rate analysis
**Regression Protection**: Continuous CI/CD, all existing tests preserved
**Quality Assurance**: Derivative verification, domain checking, error handling

Each wave adds specific test suites that build upon previous waves. The SymPy validation suite provides an external correctness benchmark. Performance benchmarks ensure the layered strategy dispatcher maintains speed targets.

With this test plan, MathHook's integration system will be thoroughly validated, performant, and mathematically sound.
