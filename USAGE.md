# MathHook Usage Guide

This guide provides comprehensive examples and patterns for using MathHook in Rust.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Expression Creation](#expression-creation)
3. [Algebraic Operations](#algebraic-operations)
4. [Calculus Operations](#calculus-operations)
5. [Equation Solving](#equation-solving)
6. [Matrix Operations](#matrix-operations)
7. [Parsing](#parsing)
8. [Performance Optimization](#performance-optimization)
9. [Educational Features](#educational-features)

## Getting Started

Add MathHook to your `Cargo.toml`:

```toml
[dependencies]
mathhook-core = "0.1"
```

Import the prelude for common operations:

```rust
use mathhook_core::prelude::*;
```

## Expression Creation

### Using Macros (Recommended)

MathHook provides ergonomic macros for creating expressions:

```rust
use mathhook_core::prelude::*;

// Create symbols
let x = symbol!(x);
let y = symbol!(y);
let theta = symbol!(theta);

// Simple expressions
let expr1 = expr!(x + y);
let expr2 = expr!(2 * x);
let expr3 = expr!(x ^ 2);

// Complex expressions with explicit grouping
let expr4 = expr!((x + 1) * (x - 1));
let expr5 = expr!((2*x) + (3*y));

// Functions
let expr6 = expr!(sin(x));
let expr7 = expr!(log(x, 10));

// Multi-term operations
let expr8 = expr!(add: x, y, z, w);
let expr9 = expr!(mul: 2, x, y, z);
```

### Using Explicit Constructors

For programmatic construction or when macros aren't suitable:

```rust
use mathhook_core::Expression;

// Numbers
let int = Expression::integer(42);
let float = Expression::float(3.14159);
let rational = Expression::rational(3, 4); // 3/4

// Symbols
let x = Expression::symbol(symbol!(x));

// Operations
let sum = Expression::add(vec![
    Expression::integer(2),
    Expression::integer(3),
]);

let product = Expression::mul(vec![
    Expression::integer(2),
    Expression::symbol(symbol!(x)),
]);

let power = Expression::pow(
    Expression::symbol(symbol!(x)),
    Expression::integer(2),
);

// Functions
let sin_x = Expression::function("sin", vec![
    Expression::symbol(symbol!(x))
]);
```

### Constants

```rust
use mathhook_core::Expression;

let pi = Expression::pi();
let e = Expression::e();
let i = Expression::i();              // Imaginary unit
let inf = Expression::infinity();
let phi = Expression::golden_ratio();
let gamma = Expression::euler_gamma();
```

### Complex Numbers

```rust
use mathhook_core::Expression;

// 3 + 4i
let complex = Expression::complex(
    Expression::integer(3),
    Expression::integer(4),
);

// Using addition
let complex2 = Expression::add(vec![
    Expression::integer(3),
    Expression::mul(vec![
        Expression::integer(4),
        Expression::i(),
    ]),
]);
```

## Algebraic Operations

### Simplification

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// Basic simplification
let expr = expr!(2 + 3);
assert_eq!(expr.simplify(), expr!(5));

// Algebraic simplification
let expr = expr!(x + x);
assert_eq!(expr.simplify(), expr!(2 * x));

// Trigonometric identities
let expr = expr!(sin(0));
assert_eq!(expr.simplify(), expr!(0));

// Power rules
let expr = expr!(x ^ 0);
assert_eq!(expr.simplify(), expr!(1));

let expr = expr!(x ^ 1);
assert_eq!(expr.simplify(), expr!(x));
```

### Expansion

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Expand (x + 1)(x - 1)
let expr = expr!((x + 1) * (x - 1));
let expanded = expr.expand();
// Result: x^2 - 1
```

### Factorization

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// Factor x^2 - 1
let expr = Expression::add(vec![
    Expression::pow(expr!(x), Expression::integer(2)),
    Expression::integer(-1),
]);
let factored = expr.factor();
// Result: (x + 1)(x - 1)
```

### Substitution

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Substitute x = 2 into x^2 + 3x + 1
let expr = Expression::add(vec![
    Expression::pow(expr!(x), Expression::integer(2)),
    Expression::mul(vec![Expression::integer(3), expr!(x)]),
    Expression::integer(1),
]);

let result = expr.substitute(&x, &Expression::integer(2));
// Result: 11
```

## Calculus Operations

### Derivatives

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// First derivative of x^2
let expr = expr!(x ^ 2);
let derivative = expr.derivative(&x, 1);
// Result: 2*x

// Second derivative
let second_deriv = expr.derivative(&x, 2);
// Result: 2

// Derivative of sin(x)
let expr = Expression::function("sin", vec![expr!(x)]);
let deriv = expr.derivative(&x, 1);
// Result: cos(x)

// Chain rule: d/dx sin(x^2)
let expr = Expression::function("sin", vec![expr!(x ^ 2)]);
let deriv = expr.derivative(&x, 1);
// Result: 2*x*cos(x^2)
```

### Integrals

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// Indefinite integral
let expr = expr!(x ^ 2);
let integral = expr.integrate(&x);
// Result: x^3/3

// Definite integral from 0 to 1
let expr = expr!(x ^ 2);
let definite = Expression::definite_integral(
    expr,
    x.clone(),
    Expression::integer(0),
    Expression::integer(1),
);
// Result: 1/3
```

### Limits

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// lim(x->0) sin(x)/x = 1
let expr = Expression::mul(vec![
    Expression::function("sin", vec![expr!(x)]),
    Expression::pow(expr!(x), Expression::integer(-1)),
]);
let limit = expr.limit(&x, &Expression::integer(0));
// Result: 1

// Limit at infinity
let expr = Expression::mul(vec![
    Expression::integer(1),
    Expression::pow(expr!(x), Expression::integer(-1)),
]);
let limit = expr.limit(&x, &Expression::infinity());
// Result: 0
```

### Series Expansions

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// Taylor series of sin(x) around 0
let expr = Expression::function("sin", vec![expr!(x)]);
let series = expr.taylor_series(&x, &Expression::integer(0), 5);
// Result: x - x^3/6 + x^5/120 + ...

// Laurent series (for functions with poles)
let expr = Expression::pow(expr!(x), Expression::integer(-1));
let laurent = expr.laurent_series(&x, &Expression::integer(0), 5);
```

## Equation Solving

### Linear Equations

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// Solve: 2x + 3 = 7
let mut solver = MathSolver::new();
let equation = Expression::equation(
    Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), expr!(x)]),
        Expression::integer(3),
    ]),
    Expression::integer(7),
);

let solutions = solver.solve(&equation, &x);
// Result: x = 2
```

### Quadratic Equations

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);

// Solve: x^2 - 5x + 6 = 0
let mut solver = MathSolver::new();
let equation = Expression::equation(
    Expression::add(vec![
        Expression::pow(expr!(x), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-5), expr!(x)]),
        Expression::integer(6),
    ]),
    Expression::integer(0),
);

let solutions = solver.solve(&equation, &x);
// Result: x = 2, x = 3
```

### Systems of Equations

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Solve system:
// x + y = 5
// x - y = 1

let mut solver = MathSolver::new();

let eq1 = Expression::equation(
    Expression::add(vec![expr!(x), expr!(y)]),
    Expression::integer(5),
);

let eq2 = Expression::equation(
    Expression::add(vec![
        expr!(x),
        Expression::mul(vec![Expression::integer(-1), expr!(y)]),
    ]),
    Expression::integer(1),
);

let system = vec![eq1, eq2];
let solutions = solver.solve_system(&system, &[x, y]);
// Result: x = 3, y = 2
```

## Matrix Operations

### Creating Matrices

```rust
use mathhook_core::Expression;

// From rows
let matrix = Expression::matrix(vec![
    vec![Expression::integer(1), Expression::integer(2)],
    vec![Expression::integer(3), Expression::integer(4)],
]);

// Identity matrix
let identity = Expression::identity_matrix(3);

// Zero matrix
let zero = Expression::zero_matrix(2, 3);

// Diagonal matrix
let diag = Expression::diagonal_matrix(vec![
    Expression::integer(1),
    Expression::integer(2),
    Expression::integer(3),
]);

// From arrays (convenience)
let matrix = Expression::matrix_from_arrays([
    [1, 2, 3],
    [4, 5, 6],
]);
```

### Matrix Operations

```rust
use mathhook_core::prelude::*;

let a = Expression::matrix(vec![
    vec![Expression::integer(1), Expression::integer(2)],
    vec![Expression::integer(3), Expression::integer(4)],
]);

let b = Expression::matrix(vec![
    vec![Expression::integer(5), Expression::integer(6)],
    vec![Expression::integer(7), Expression::integer(8)],
]);

// Addition
let sum = a.add_matrix(&b);

// Multiplication
let product = a.multiply_matrix(&b);

// Transposition
let transpose = a.transpose();

// Determinant
let det = a.determinant();

// Inverse
let inverse = a.inverse();

// Trace
let trace = a.trace();
```

### Matrix Decomposition

```rust
use mathhook_core::prelude::*;

let matrix = Expression::matrix(vec![
    vec![Expression::integer(4), Expression::integer(2)],
    vec![Expression::integer(2), Expression::integer(3)],
]);

// LU decomposition
let (l, u) = matrix.lu_decomposition();

// QR decomposition
let (q, r) = matrix.qr_decomposition();

// Eigenvalues and eigenvectors
let eigen = matrix.eigenvalues();
let eigenvectors = matrix.eigenvectors();

// Cholesky decomposition (for positive definite matrices)
let cholesky = matrix.cholesky_decomposition();
```

## Parsing

### Multi-Format Parser

```rust
use mathhook_core::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());

// Standard mathematical notation
let expr = parser.parse("2*x + sin(y)").unwrap();

// LaTeX
let expr = parser.parse(r"\frac{x}{2} + y^2").unwrap();
let expr = parser.parse(r"\sin(x) + \cos(y)").unwrap();
let expr = parser.parse(r"\int_{0}^{1} x^2 \, dx").unwrap();

// Wolfram Language
let expr = parser.parse("Sin[x] + Cos[y]").unwrap();
let expr = parser.parse("D[x^2, x]").unwrap();
```

### Explicit Language Parsing

```rust
use mathhook_core::parser::{Parser, ParserConfig, Language};

let mut config = ParserConfig::default();
config.language = Language::LaTeX;

let parser = Parser::new(config);
let expr = parser.parse(r"\sin(x)").unwrap();
```

### Output Formatting

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let expr = Expression::pow(expr!(x), Expression::integer(2));

// Standard format
println!("{}", expr); // x^2

// LaTeX format
let latex = expr.to_latex();
println!("{}", latex); // x^{2}

// Wolfram format
let wolfram = expr.to_wolfram();
println!("{}", wolfram); // Power[x, 2]
```

## Performance Optimization

### Global Configuration

```rust
use mathhook_core::core::performance::strategy::{PerformanceConfig, BindingContext};
use mathhook_core::core::performance::config::set_global_config;

// Use Python-optimized configuration
let config = PerformanceConfig::for_binding(BindingContext::Python);
set_global_config(config);

// Or customize
let mut config = PerformanceConfig::default();
config.simd_enabled = true;
config.simd_threshold = 100;
config.parallel_enabled = false;
set_global_config(config);
```

### Bulk Operations

```rust
use mathhook_core::core::performance::config::{
    parallel_bulk_simplify,
    simd_bulk_add_numeric,
};

// Parallel simplification
let expressions = vec![/* many expressions */];
let simplified = parallel_bulk_simplify(&expressions);

// SIMD arithmetic
let values = vec![1.0, 2.0, 3.0, /* ... many values */];
let sum = simd_bulk_add_numeric(&values);
```

### Caching

```rust
use mathhook_core::core::performance::config::{
    cache_result,
    get_cached_result,
    compute_expr_hash,
};

let expr = expr!(x ^ 2);
let hash = compute_expr_hash(&expr);

// Check cache
if let Some(cached) = get_cached_result(hash) {
    // Use cached result
} else {
    // Compute and cache
    let result = expr.simplify();
    cache_result(hash, result.clone());
}
```

## Educational Features

### Step-by-Step Explanations

```rust
use mathhook_core::prelude::*;
use mathhook_core::educational::*;

let x = symbol!(x);
let expr = Expression::add(vec![
    Expression::pow(expr!(x), Expression::integer(2)),
    Expression::mul(vec![Expression::integer(2), expr!(x)]),
    Expression::integer(1),
]);

// Get step-by-step simplification
let explanation = expr.explain_simplification();

for step in explanation.steps() {
    println!("Step: {}", step.title);
    println!("Description: {}", step.description);
    println!("Expression: {}", step.expression);
    println!();
}

// LaTeX formatted explanation
let latex_explanation = explanation.to_latex();
println!("{}", latex_explanation);
```

### Derivative Explanation

```rust
use mathhook_core::prelude::*;
use mathhook_core::educational::*;

let x = symbol!(x);
let expr = Expression::function("sin", vec![
    Expression::pow(expr!(x), Expression::integer(2))
]);

// Get step-by-step derivative
let explanation = expr.explain_derivative(&x);

for step in explanation.steps() {
    println!("{}: {}", step.title, step.description);
}
```

## Advanced Patterns

### Custom Functions

```rust
use mathhook_core::Expression;

// Define a custom function
let my_func = Expression::function("my_custom_func", vec![
    Expression::symbol(symbol!(x)),
    Expression::integer(2),
]);
```

### Piecewise Functions

```rust
use mathhook_core::Expression;

let x = symbol!(x);

// f(x) = { x^2  if x > 0
//        { 0    otherwise
let piecewise = Expression::piecewise(
    vec![(
        Expression::pow(expr!(x), Expression::integer(2)),
        Expression::relation(
            expr!(x),
            Expression::integer(0),
            RelationType::Greater,
        ),
    )],
    Some(Expression::integer(0)),
);
```

### Assumptions

```rust
use mathhook_core::prelude::*;

// Create symbol with assumptions
let x = symbol!(x);
// Note: Assumption system is under development
// Future API: x.assume_positive();
//             x.assume_real();
//             x.assume_integer();
```

## Error Handling

```rust
use mathhook_core::prelude::*;

let parser = Parser::new(ParserConfig::default());

// Parsing errors
match parser.parse("invalid expression") {
    Ok(expr) => println!("Parsed: {}", expr),
    Err(e) => eprintln!("Parse error: {}", e),
}

// Solver errors
let mut solver = MathSolver::new();
match solver.solve(&equation, &x) {
    SolverResult::Single(solution) => println!("Solution: {}", solution),
    SolverResult::Multiple(solutions) => {
        println!("Solutions: {:?}", solutions);
    }
    SolverResult::NoSolution => println!("No solution exists"),
    SolverResult::InfiniteSolutions => println!("Infinite solutions"),
}
```

## Performance Tips

1. **Use macros for expression creation** - They're zero-cost and more readable
2. **Enable SIMD for bulk operations** - 2-4x speedup for array arithmetic
3. **Use parallel simplification** for large collections
4. **Cache frequently used expressions** - Avoid recomputation
5. **Profile before optimizing** - Use `cargo bench` to identify bottlenecks

## Best Practices

1. **Prefer `expr!` macro** over explicit constructors for readability
2. **Use `symbol!` consistently** for variable creation
3. **Group complex expressions** with parentheses for clarity
4. **Document mathematical assumptions** in comments
5. **Test edge cases** (zero, infinity, undefined)
6. **Use step-by-step explanations** for debugging
7. **Check parser language** when parsing ambiguous notation

## Common Pitfalls

1. **Runtime variables in macros**: `expr!(i)` creates symbol "i", not the variable's value
2. **Precedence in macros**: Use explicit parentheses: `expr!((2*x) + 3)`
3. **Nested macro calls**: Don't nest `expr!()` inside `expr!()`
4. **Float precision**: Use rationals for exact arithmetic
5. **Symbol comparison**: Symbols with same name are equal
6. **Parser language detection**: LaTeX vs Wolfram can be ambiguous

## Further Reading

- [API Documentation](https://docs.rs/mathhook-core)
- [Python Usage Guide](crates/mathhook-python/USAGE.md)
- [Node.js Usage Guide](crates/mathhook-node/USAGE.md)
- [CLAUDE.md](CLAUDE.md) - Developer documentation
- [Examples](https://github.com/ahmedmashhour/mathhook/tree/main/examples)
