# Architecture Design for MathHook Core Mathematical Features
**Wave 0 Research Phase**
**Date**: October 22, 2025

---

## Executive Summary

This document defines the module architecture, API design, and implementation strategy for completing MathHook's core mathematical features across Waves 1-6. The design prioritizes:

1. **Mathematical Correctness**: All algorithms verified against SymPy
2. **Performance**: 10-100x faster than SymPy (Rust vs Python)
3. **Maintainability**: Clear module boundaries, comprehensive tests
4. **Educational Integration**: Step-by-step explanations for all operations

---

## Overall Architecture

```
crates/mathhook-core/src/
├── ode/                    # Wave 1: Ordinary Differential Equations
│   ├── mod.rs             # Public API, classifier
│   ├── first_order/       # First-order ODE solvers
│   ├── second_order/      # Second-order ODE solvers
│   └── systems/           # Systems of ODEs (future)
│
├── linalg_advanced/       # Wave 2: Advanced Linear Algebra
│   ├── mod.rs             # Public API
│   ├── decompositions/    # Matrix decompositions
│   ├── eigenvalues/       # Eigenvalue algorithms
│   └── properties/        # Matrix properties (rank, nullspace, etc.)
│
├── ntheory/               # Wave 3: Number Theory
│   ├── mod.rs             # Public API
│   ├── factorization/     # Integer factorization algorithms
│   ├── primes/            # Prime number functions
│   └── modular/           # Modular arithmetic
│
├── polys_advanced/        # Wave 3: Advanced Polynomial Algorithms
│   ├── mod.rs             # Public API
│   ├── factorization/     # Multivariate factorization
│   ├── groebner/          # Gröbner basis algorithms
│   └── gcd/               # Polynomial GCD algorithms
│
├── series/                # Wave 4: Series Expansions
│   ├── mod.rs             # Public API
│   ├── taylor.rs          # Taylor series
│   ├── laurent.rs         # Laurent series
│   ├── fourier.rs         # Fourier series
│   └── asymptotic.rs      # Asymptotic expansions
│
├── special_functions/     # Wave 4: Special Functions
│   ├── mod.rs             # Public API, registry
│   ├── gamma.rs           # Gamma, Beta functions
│   ├── error_functions.rs # erf, erfc
│   ├── bessel.rs          # Bessel functions
│   └── hypergeometric.rs  # Hypergeometric functions
│
├── pde/                   # Wave 5: Partial Differential Equations
│   ├── mod.rs             # Public API, classifier
│   ├── separation.rs      # Separation of variables
│   ├── characteristics.rs # Method of characteristics
│   └── standard.rs        # Heat, wave, Laplace equations
│
└── numerical/             # Wave 6: Numerical Methods
    ├── mod.rs             # Public API
    ├── integration/       # Numerical integration
    ├── ode_solvers/       # Numerical ODE solvers
    └── root_finding/      # Numerical equation solving
```

---

## Wave 1: ODE Module Architecture

### Module Structure

```rust
// crates/mathhook-core/src/ode/mod.rs

/// Ordinary Differential Equation solving
pub mod first_order;
pub mod second_order;
pub mod classifier;
pub mod utils;

pub use first_order::{
    solve_separable,
    solve_linear_first_order,
    solve_exact,
    solve_homogeneous,
};

pub use second_order::{
    solve_constant_coefficients,
    solve_cauchy_euler,
    solve_variation_of_parameters,
};

/// Main ODE solver with automatic classification
pub fn solve_ode(
    ode: &Expression,
    dependent: &Symbol,
    independent: &Symbol,
    initial_conditions: Option<Vec<(Expression, Expression)>>
) -> Result<Expression, ODEError> {
    // Auto-classify and route to appropriate solver
    let ode_type = classifier::classify_ode(ode, dependent, independent)?;

    match ode_type {
        ODEType::SeparableFirst => first_order::solve_separable(ode, dependent, independent),
        ODEType::LinearFirst => first_order::solve_linear_first_order(ode, dependent, independent),
        ODEType::ConstantCoeffSecond => second_order::solve_constant_coefficients(ode, dependent, independent),
        // ... other types
        _ => Err(ODEError::UnsupportedType(ode_type))
    }
}
```

### First-Order Solver Design

```rust
// crates/mathhook-core/src/ode/first_order/separable.rs

/// Solves separable ODE: dy/dx = g(x)h(y)
///
/// Algorithm:
/// 1. Separate variables: (1/h(y))dy = g(x)dx
/// 2. Integrate both sides: ∫(1/h(y))dy = ∫g(x)dx + C
/// 3. Solve for y if possible
///
/// # Arguments
/// * `ode` - The ODE expression (dy/dx = ...)
/// * `y` - Dependent variable symbol
/// * `x` - Independent variable symbol
///
/// # Returns
/// Solution expression or error
///
/// # Examples
/// ```rust
/// use mathhook_core::ode::first_order::solve_separable;
/// use mathhook_core::{symbol, expr};
///
/// let x = symbol!(x);
/// let y = symbol!(y);
/// let ode = expr!(y.diff(x) = x * y); // dy/dx = x*y
/// let solution = solve_separable(&ode, &y, &x)?;
/// // Returns: y = C*exp(x^2/2)
/// ```
pub fn solve_separable(
    ode: &Expression,
    y: &Symbol,
    x: &Symbol
) -> Result<Expression, ODEError> {
    // Implementation:
    // 1. Pattern match to extract g(x) and h(y)
    // 2. Separate: (1/h(y)) and g(x)
    // 3. Integrate both sides
    // 4. Solve for y
    // 5. Add integration constant C

    // Check if separable
    let (g_x, h_y) = extract_separable_parts(ode, x, y)?;

    // Integrate both sides
    let left_integral = integrate(&Expression::div(Expression::integer(1), h_y.clone()), y)?;
    let right_integral = integrate(&g_x, x)?;

    // Build solution: left_integral = right_integral + C
    let c = Symbol::new("C1");
    let equation = Expression::eq(
        left_integral,
        Expression::add(vec![right_integral, Expression::symbol(c)])
    );

    // Solve for y
    solve_for_variable(&equation, y)
}
```

### Second-Order Constant Coefficients Solver

```rust
// crates/mathhook-core/src/ode/second_order/constant_coeff.rs

/// Solves second-order linear ODE with constant coefficients
/// Form: a*y'' + b*y' + c*y = r(x)
///
/// Algorithm:
/// 1. Solve characteristic equation: a*λ² + b*λ + c = 0
/// 2. Find homogeneous solution based on roots:
///    - Real distinct: y_h = C1*exp(λ1*x) + C2*exp(λ2*x)
///    - Real repeated: y_h = (C1 + C2*x)*exp(λ*x)
///    - Complex: y_h = exp(α*x)*(C1*cos(β*x) + C2*sin(β*x))
/// 3. If non-homogeneous (r(x) ≠ 0), find particular solution
/// 4. General solution: y = y_h + y_p
pub fn solve_constant_coefficients(
    ode: &Expression,
    y: &Symbol,
    x: &Symbol
) -> Result<Expression, ODEError> {
    // Extract coefficients a, b, c
    let (a, b, c, r_x) = extract_constant_coefficients(ode, y, x)?;

    // Solve characteristic equation: a*λ² + b*λ + c = 0
    let lambda = Symbol::new("λ");
    let char_eq = Expression::add(vec![
        Expression::mul(vec![a.clone(), Expression::pow(Expression::symbol(lambda.clone()), Expression::integer(2))]),
        Expression::mul(vec![b.clone(), Expression::symbol(lambda.clone())]),
        c.clone()
    ]);

    let roots = solve_quadratic(&char_eq, &lambda)?;

    // Build homogeneous solution based on root type
    let y_h = match classify_roots(&roots) {
        RootType::RealDistinct(lambda1, lambda2) => {
            // y_h = C1*exp(λ1*x) + C2*exp(λ2*x)
            build_real_distinct_solution(lambda1, lambda2, x)
        },
        RootType::RealRepeated(lambda) => {
            // y_h = (C1 + C2*x)*exp(λ*x)
            build_real_repeated_solution(lambda, x)
        },
        RootType::ComplexConjugate(alpha, beta) => {
            // y_h = exp(α*x)*(C1*cos(β*x) + C2*sin(β*x))
            build_complex_solution(alpha, beta, x)
        }
    };

    // If non-homogeneous, find particular solution
    if !r_x.is_zero() {
        let y_p = find_particular_solution(&r_x, &roots, x)?;
        Ok(Expression::add(vec![y_h, y_p]))
    } else {
        Ok(y_h)
    }
}
```

---

## Wave 2: Advanced Linear Algebra Architecture

### Decomposition Module

```rust
// crates/mathhook-core/src/linalg_advanced/decompositions/qr.rs

/// QR decomposition using Gram-Schmidt process
///
/// Decomposes matrix A into:
/// - Q: Orthogonal matrix (Q^T * Q = I)
/// - R: Upper triangular matrix
///
/// Such that A = Q * R
///
/// # Algorithm
/// Uses Modified Gram-Schmidt for numerical stability
///
/// # Complexity
/// O(n³) for n×n matrix
pub fn qr_decomposition(matrix: &Matrix) -> Result<(Matrix, Matrix), MatrixError> {
    let (m, n) = matrix.dimensions();

    // Modified Gram-Schmidt algorithm
    let mut Q = Matrix::zeros(m, n);
    let mut R = Matrix::zeros(n, n);

    for j in 0..n {
        let mut v = matrix.column(j);

        for i in 0..j {
            let q_i = Q.column(i);
            let r_ij = q_i.dot(&v);
            R.set(i, j, r_ij.clone());
            v = v - (q_i * r_ij);
        }

        let r_jj = v.norm();
        R.set(j, j, r_jj.clone());

        if !r_jj.is_zero() {
            Q.set_column(j, v / r_jj);
        } else {
            return Err(MatrixError::SingularMatrix);
        }
    }

    Ok((Q, R))
}
```

### Eigenvalue Module

```rust
// crates/mathhook-core/src/linalg_advanced/eigenvalues/symbolic.rs

/// Symbolic eigenvalue computation via characteristic polynomial
///
/// For matrix A, computes eigenvalues from det(A - λI) = 0
///
/// # Limitations
/// Only practical for small matrices (n ≤ 4)
/// For larger matrices, use numerical methods
pub fn eigenvalues_symbolic(matrix: &Matrix) -> Result<Vec<Expression>, MatrixError> {
    let n = matrix.rows();

    // Create λ symbol
    let lambda = symbol!(λ);

    // Compute A - λI
    let identity = Matrix::identity(n);
    let a_minus_lambda_i = matrix - identity * Expression::symbol(lambda.clone());

    // Compute characteristic polynomial det(A - λI)
    let char_poly = a_minus_lambda_i.determinant()?;

    // Solve for λ
    solve_polynomial(&char_poly, &lambda)
}
```

---

## Wave 3: Number Theory & Polynomial Algorithms

### Polynomial GCD (Critical for Simplification)

```rust
// crates/mathhook-core/src/polys_advanced/gcd/mod.rs

/// Polynomial GCD using Euclidean algorithm
///
/// For polynomials f(x), g(x), computes gcd(f, g)
///
/// # Algorithm
/// Extended Euclidean algorithm adapted for polynomials
pub fn polynomial_gcd(
    f: &Expression,
    g: &Expression,
    variable: &Symbol
) -> Result<Expression, PolynomialError> {
    // Convert to polynomial representation
    let mut a = to_polynomial(f, variable)?;
    let mut b = to_polynomial(g, variable)?;

    // Euclidean algorithm: gcd(a, b) = gcd(b, a mod b)
    while !b.is_zero() {
        let remainder = a.poly_rem(&b)?;
        a = b;
        b = remainder;
    }

    // Return monic polynomial (leading coefficient = 1)
    let leading_coeff = a.leading_coefficient()?;
    Ok(a / leading_coeff)
}
```

### Gröbner Basis (Buchberger Algorithm)

```rust
// crates/mathhook-core/src/polys_advanced/groebner/buchberger.rs

/// Computes Gröbner basis using Buchberger's algorithm
///
/// # Arguments
/// * `polynomials` - Set of polynomials generating ideal
/// * `variables` - Variables in order (monomial ordering)
/// * `ordering` - Monomial ordering (lex, grlex, grevlex)
///
/// # Complexity
/// Doubly exponential worst-case, but often practical
pub fn groebner_basis(
    polynomials: &[Expression],
    variables: &[Symbol],
    ordering: MonomialOrder
) -> Result<Vec<Expression>, GroebnerError> {
    let mut basis = polynomials.to_vec();
    let mut pairs = generate_critical_pairs(&basis);

    while let Some((i, j)) = pairs.pop() {
        let s_poly = s_polynomial(&basis[i], &basis[j], variables, ordering)?;
        let remainder = poly_reduce(s_poly, &basis, ordering)?;

        if !remainder.is_zero() {
            // Add new polynomial to basis
            let new_idx = basis.len();
            basis.push(remainder);

            // Update critical pairs
            for k in 0..new_idx {
                pairs.push((k, new_idx));
            }
        }
    }

    // Reduce basis (optional but recommended)
    Ok(reduce_basis(basis, ordering)?)
}
```

---

## Wave 4: Series & Special Functions

### Taylor Series Expansion

```rust
// crates/mathhook-core/src/series/taylor.rs

/// Computes Taylor series expansion
///
/// For function f(x) around point a:
/// f(x) = Σ(k=0 to n) [f^(k)(a) / k!] * (x-a)^k
///
/// # Arguments
/// * `expr` - Function to expand
/// * `variable` - Variable to expand around
/// * `point` - Point of expansion
/// * `order` - Maximum order of expansion
pub fn taylor_series(
    expr: &Expression,
    variable: &Symbol,
    point: &Expression,
    order: usize
) -> Result<Expression, SeriesError> {
    let mut terms = Vec::new();
    let mut derivative = expr.clone();

    for k in 0..=order {
        // Evaluate k-th derivative at point
        let deriv_at_point = derivative.substitute(variable, point)?;

        // Compute (x - a)^k / k!
        let x_minus_a = Expression::sub(
            Expression::symbol(variable.clone()),
            point.clone()
        );
        let power_term = Expression::pow(
            x_minus_a,
            Expression::integer(k as i64)
        );
        let factorial = Expression::integer(factorial(k) as i64);

        // Add term: [f^(k)(a) / k!] * (x-a)^k
        terms.push(Expression::mul(vec![
            Expression::div(deriv_at_point, factorial),
            power_term
        ]));

        // Compute next derivative
        if k < order {
            derivative = derivative.derivative(variable, 1)?;
        }
    }

    Ok(Expression::add(terms))
}
```

### Special Function Framework

```rust
// crates/mathhook-core/src/special_functions/mod.rs

/// Special function trait for unified interface
pub trait SpecialFunction {
    /// Symbolic evaluation
    fn evaluate_symbolic(&self, args: &[Expression]) -> Result<Expression, FunctionError>;

    /// Numerical evaluation (SIMD optimized)
    fn evaluate_numerical(&self, args: &[f64]) -> Result<f64, FunctionError>;

    /// Series representation
    fn series_expansion(&self, point: &Expression, order: usize) -> Result<Expression, FunctionError>;

    /// Derivative
    fn derivative(&self, arg_index: usize) -> Result<Box<dyn SpecialFunction>, FunctionError>;

    /// Domain restrictions
    fn domain(&self) -> Domain;
}

/// Gamma function implementation
pub struct GammaFunction;

impl SpecialFunction for GammaFunction {
    fn evaluate_symbolic(&self, args: &[Expression]) -> Result<Expression, FunctionError> {
        if args.len() != 1 {
            return Err(FunctionError::WrongArgumentCount);
        }

        let z = &args[0];

        // Check for special values
        if let Some(n) = z.as_integer() {
            if n > 0 {
                // Γ(n) = (n-1)! for positive integers
                return Ok(Expression::integer(factorial((n - 1) as usize) as i64));
            }
        }

        // Return unevaluated
        Ok(Expression::function("gamma", vec![z.clone()]))
    }

    fn evaluate_numerical(&self, args: &[f64]) -> Result<f64, FunctionError> {
        // Lanczos approximation for numerical evaluation
        lanczos_gamma(args[0])
    }

    // ... other methods
}
```

---

## API Design Philosophy

### Hybrid API: Expression-Centric vs Solver Objects

**Expression-Centric API** (functional, method chaining):
```rust
// Direct method on Expression
let derivative = expr.derivative(&x, 1)?;
let integral = expr.integrate(&x)?;
let taylor = expr.taylor_series(&x, &Expression::integer(0), 10)?;
```

**Solver Object API** (stateful, configuration-driven):
```rust
// Configure solver with options
let mut ode_solver = ODESolver::new()
    .with_tolerance(1e-10)
    .with_max_iterations(1000)
    .with_simplification(true);

let solution = ode_solver.solve(&ode, &y, &x)?;
```

**When to use which:**
- Expression methods: Quick, single operations
- Solver objects: Complex algorithms needing configuration, state

---

## Error Handling Strategy

### Error Type Hierarchy

```rust
// Top-level error type
pub enum MathHookError {
    ODE(ODEError),
    Matrix(MatrixError),
    Polynomial(PolynomialError),
    Series(SeriesError),
    Numerical(NumericalError),
    // ... other domains
}

// Domain-specific errors
pub enum ODEError {
    UnsupportedType(ODEType),
    NotSeparable,
    SingularPoint { location: Expression },
    IntegrationFailed { reason: String },
    NoSolution,
}

pub enum MatrixError {
    DimensionMismatch { expected: (usize, usize), got: (usize, usize) },
    SingularMatrix,
    NotSquare,
    NotSymmetric,
    NotPositiveDefinite,
}
```

### Error Context

All errors include:
- Clear description of what went wrong
- Context about the operation being performed
- Suggestions for resolution when possible

```rust
impl std::fmt::Display for ODEError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ODEError::NotSeparable => write!(
                f,
                "ODE is not separable. Cannot express as dy/dx = g(x)h(y).\n\
                 Consider trying linear first-order or exact methods."
            ),
            ODEError::SingularPoint { location } => write!(
                f,
                "Singular point encountered at x = {}.\n\
                 Solution may not exist or may require special handling.",
                location
            ),
            // ... other cases
        }
    }
}
```

---

## Testing Strategy

### Test Organization

```
crates/mathhook-core/src/ode/
├── mod.rs
├── first_order/
│   ├── separable.rs
│   │   └── #[cfg(test)] mod tests
│   └── tests/
│       ├── separable_tests.rs       # Unit tests
│       └── oracle_validation.rs     # SymPy validation tests
└── tests/
    ├── integration_tests.rs         # Integration tests
    └── sympy_comparison.rs          # Correctness validation
```

### Test Categories

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test full workflows
3. **Oracle Validation**: Compare against SymPy test oracle
4. **Property Tests**: Verify mathematical properties
5. **Performance Tests**: Benchmark against targets

### Oracle Validation Pattern

```rust
#[test]
fn test_ode_separable_against_oracle() {
    // Load test cases from oracle
    let oracle = load_test_oracle().expect("Failed to load oracle");
    let cases = oracle.get("ode_first_order_separable").unwrap();

    for case in cases {
        let x = symbol!(x);
        let y = symbol!(y);

        // Parse input from oracle
        let ode = parse(&case["input"]["ode"]).unwrap();

        // Solve with MathHook
        let mathhook_solution = solve_separable(&ode, &y, &x).unwrap();

        // Parse expected output from oracle
        let sympy_solution = parse(&case["expected_output"]).unwrap();

        // Verify equivalence (not just string equality - mathematical equivalence!)
        assert_solutions_equivalent(&mathhook_solution, &sympy_solution);
    }
}
```

---

## Performance Optimization Strategy

### Profile-Guided Optimization

1. **Baseline**: Measure SymPy performance
2. **Target**: 10-100x faster
3. **Profile**: Identify MathHook hot paths
4. **Optimize**: Apply targeted optimizations
5. **Verify**: Ensure correctness maintained

### Optimization Techniques

**Arena Allocation** (for temporary expressions):
```rust
// Use arena for bulk operations
let arena = Arena::new();
let mut terms = Vec::new();
for i in 0..n {
    terms.push(arena.alloc(Expression::integer(i)));
}
```

**SIMD Operations** (for numerical evaluation):
```rust
// Vectorized evaluation for arrays
pub fn evaluate_array_simd(expr: &Expression, values: &[f64]) -> Vec<f64> {
    // SIMD-optimized evaluation
    #[cfg(target_feature = "avx2")]
    return evaluate_avx2(expr, values);

    #[cfg(target_feature = "sse2")]
    return evaluate_sse2(expr, values);

    // Scalar fallback
    evaluate_scalar(expr, values)
}
```

**Caching** (for expensive operations):
```rust
// Cache derivatives
pub struct CachedDerivative {
    expr: Expression,
    cache: HashMap<(Symbol, usize), Expression>,
}

impl CachedDerivative {
    pub fn derivative(&mut self, var: &Symbol, order: usize) -> &Expression {
        self.cache.entry((var.clone(), order)).or_insert_with(|| {
            self.expr.derivative(var, order)
        })
    }
}
```

---

## Educational Integration

### Step-by-Step Explanation Framework

```rust
/// Explanation step for educational output
pub struct ExplanationStep {
    pub description: String,
    pub expression_before: Expression,
    pub expression_after: Expression,
    pub reasoning: String,
}

/// Explanation builder for solving process
pub struct ExplanationBuilder {
    steps: Vec<ExplanationStep>,
}

impl ExplanationBuilder {
    pub fn add_step(
        &mut self,
        description: impl Into<String>,
        before: Expression,
        after: Expression,
        reasoning: impl Into<String>
    ) {
        self.steps.push(ExplanationStep {
            description: description.into(),
            expression_before: before,
            expression_after: after,
            reasoning: reasoning.into(),
        });
    }

    pub fn finish(self) -> Vec<ExplanationStep> {
        self.steps
    }
}

// Usage in solver:
pub fn solve_separable_with_steps(
    ode: &Expression,
    y: &Symbol,
    x: &Symbol
) -> Result<(Expression, Vec<ExplanationStep>), ODEError> {
    let mut explainer = ExplanationBuilder::new();

    explainer.add_step(
        "Identify ODE type",
        ode.clone(),
        ode.clone(),
        "This is a separable ODE of the form dy/dx = g(x)h(y)"
    );

    let (g_x, h_y) = extract_separable_parts(ode, x, y)?;

    explainer.add_step(
        "Separate variables",
        ode.clone(),
        Expression::eq(
            Expression::div(Expression::integer(1), h_y.clone()),
            g_x.clone()
        ),
        format!("Rewrite as (1/h(y))dy = g(x)dx where h(y) = {} and g(x) = {}", h_y, g_x)
    );

    // ... continue with more steps

    Ok((solution, explainer.finish()))
}
```

---

## Implementation Roadmap

### Phase 1: Core Infrastructure (Weeks 1-2)
- Set up module structure
- Define error types
- Create base traits
- Set up test oracle loading

### Phase 2: Wave 1 Implementation (Weeks 3-6)
- First-order ODE solvers (separable, linear)
- Second-order ODE solvers (constant coefficients)
- Classifier algorithm
- Oracle validation tests

### Phase 3: Wave 2 Implementation (Weeks 7-10)
- Matrix decompositions (QR, LU, SVD)
- Eigenvalue algorithms
- Matrix properties
- Performance benchmarks

### Phase 4: Wave 3 Implementation (Weeks 11-15)
- Polynomial GCD
- Multivariate factorization
- Gröbner basis (Buchberger)
- Number theory functions

### Phase 5: Wave 4 Implementation (Weeks 16-19)
- Series expansions (Taylor, Laurent, Fourier)
- Special functions (Gamma, Bessel, etc.)
- Integration with function registry

### Phase 6: Wave 5 Implementation (Weeks 20-22)
- PDE solver (separation of variables)
- Standard PDE solutions
- Method of characteristics

### Phase 7: Wave 6 Implementation (Weeks 23-26)
- Numerical integration
- Numerical ODE solvers
- Root finding algorithms

### Phase 8: Integration & Optimization (Weeks 27-30)
- Performance tuning
- Educational integration
- Documentation
- Final validation

---

## Success Metrics

### Correctness
- 100% oracle validation pass rate
- Zero mathematical errors
- All edge cases handled

### Performance
- 10-100x faster than SymPy
- <1ms for simple operations
- Competitive with Symbolica

### Maintainability
- <500 lines per file
- >80% test coverage
- Clear module boundaries
- Comprehensive documentation

### Educational Quality
- Step-by-step explanations for all operations
- Clear domain restriction messages
- Helpful error messages

---

## Risk Mitigation

### Mathematical Complexity Risk
**Risk**: Algorithms more complex than anticipated
**Mitigation**: Extensive SymPy source study before implementation

### Performance Risk
**Risk**: Not achieving 10x speedup target
**Mitigation**: Profile early, optimize hot paths, SIMD where applicable

### Scope Creep Risk
**Risk**: Trying to implement too much
**Mitigation**: Strict priority ordering, MVP for each wave

### Integration Risk
**Risk**: New modules break existing functionality
**Mitigation**: Regression tests, gradual rollout, feature flags

---

## Conclusion

This architecture provides:
- **Clear module boundaries** for maintainability
- **Hybrid API design** for flexibility
- **Comprehensive error handling** for reliability
- **Performance optimization strategy** for speed
- **Educational integration** for teaching
- **Test oracle validation** for correctness

The design prioritizes mathematical correctness while maintaining MathHook's performance and educational goals.
