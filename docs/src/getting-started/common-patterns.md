# Common Patterns

This chapter covers common patterns and best practices when using MathHook.

## Macro Usage Guidelines

### When to Use Macros

**ALWAYS use macros for:**

```rust
// Symbol creation
let x = symbol!(x);  // NOT Symbol::new("x")

// Simple expressions
let expr = expr!(x + y);
let expr = expr!(2 * x);
let expr = expr!(x ^ 2);

// Function calls
let expr = expr!(sin(x));
```

### When to Use Explicit API

**Use explicit API for:**

```rust
// Runtime/loop variables
for i in 0..10 {
    let term = Expression::integer(i);  // NOT expr!(i)
}

// Programmatic construction
let mut terms = Vec::new();
for i in 0..n {
    terms.push(Expression::mul(vec![
        Expression::integer(coefficients[i]),
        Expression::pow(symbol!(x), Expression::integer(i))
    ]));
}
let polynomial = Expression::add(terms);
```

## Building Polynomials

### Fixed Degree

```rust
let x = symbol!(x);

// ax^2 + bx + c
let a = Expression::integer(1);
let b = Expression::integer(2);
let c = Expression::integer(1);

let quadratic = Expression::add(vec![
    Expression::mul(vec![a, Expression::pow(expr!(x), Expression::integer(2))]),
    Expression::mul(vec![b, expr!(x)]),
    c,
]);
```

### Dynamic Degree

```rust
fn build_polynomial(coefficients: &[i64], x: &Symbol) -> Expression {
    let mut terms = Vec::new();
    for (i, &coeff) in coefficients.iter().enumerate() {
        terms.push(Expression::mul(vec![
            Expression::integer(coeff),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(i as i64))
        ]));
    }
    Expression::add(terms)
}

let x = symbol!(x);
let poly = build_polynomial(&[1, -5, 6], &x);  // x^2 - 5x + 6
```

## Substitution Patterns

### Single Substitution

```rust
let x = symbol!(x);
let expr = expr!((x ^ 2) + (2 * x) + 1);

// Substitute x = 3
let result = expr.substitute(&x, &Expression::integer(3));
```

### Multiple Substitutions

```rust
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x * y) + x + y);

// Substitute both x and y
let result = expr
    .substitute(&x, &Expression::integer(2))
    .substitute(&y, &Expression::integer(3));
```

## Working with Functions

### Creating Function Expressions

```rust
// Using expr! macro
let f = expr!(sin(x));
let g = expr!(cos(x));

// Using constructors
let h = Expression::function("tan", vec![expr!(x)]);
```

### Composing Functions

```rust
// sin(cos(x))
let inner = expr!(cos(x));
let composed = Expression::function("sin", vec![inner]);

// Or using nested expr!
let composed = expr!(sin(cos(x)));
```

## Matrix Patterns

### Creating Matrices

```rust
// From vectors
let matrix = Expression::matrix(vec![
    vec![Expression::integer(1), Expression::integer(2)],
    vec![Expression::integer(3), Expression::integer(4)],
]);

// Identity matrix
let identity = Expression::identity_matrix(3);

// Zero matrix
let zero = Expression::zero_matrix(2, 3);
```

### Matrix Operations

```rust
let a = Expression::matrix(/* ... */);
let b = Expression::matrix(/* ... */);

// Add matrices
let sum = a.add_matrix(&b);

// Multiply matrices
let product = a.multiply_matrix(&b);

// Transpose
let transpose = a.transpose();

// Determinant
let det = a.determinant();
```

## Error Handling

### Parsing Errors

```rust
use mathhook_core::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());

match parser.parse(input) {
    Ok(expr) => {
        println!("Parsed: {}", expr);
    }
    Err(e) => {
        eprintln!("Parse error: {}", e);
        // Handle error appropriately
    }
}
```

### Solver Errors

```rust
use mathhook_core::prelude::*;

let mut solver = MathSolver::new();

match solver.solve(&equation, &x) {
    SolverResult::Single(solution) => {
        println!("Solution: {}", solution);
    }
    SolverResult::Multiple(solutions) => {
        println!("Multiple solutions: {:?}", solutions);
    }
    SolverResult::NoSolution => {
        println!("No solution exists");
    }
    SolverResult::InfiniteSolutions => {
        println!("Infinite solutions");
    }
}
```

## Performance Patterns

### Bulk Operations

```rust
use mathhook_core::core::performance::config::parallel_bulk_simplify;

// Simplify many expressions in parallel
let expressions = vec![/* many expressions */];
let simplified = parallel_bulk_simplify(&expressions);
```

### Caching Results

```rust
use mathhook_core::core::performance::config::{
    cache_result,
    get_cached_result,
    compute_expr_hash,
};

let expr = expr!(x ^ 2);
let hash = compute_expr_hash(&expr);

if let Some(cached) = get_cached_result(hash) {
    // Use cached result
    println!("Using cached result");
} else {
    // Compute and cache
    let result = expr.simplify();
    cache_result(hash, result.clone());
}
```

## Educational Patterns

### Step-by-Step Explanations

```rust
use mathhook_core::educational::*;

let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

// Get explanation for simplification
let explanation = expr.explain_simplification();

for step in explanation.steps() {
    println!("Step: {}", step.title);
    println!("Description: {}", step.description);
    println!("Expression: {}", step.expression);
    println!();
}
```

### Derivative Explanations

```rust
use mathhook_core::educational::*;

let x = symbol!(x);
let expr = expr!(sin(x ^ 2));

// Get step-by-step derivative
let explanation = expr.explain_derivative(&x);

for step in explanation.steps() {
    println!("{}: {}", step.title, step.description);
}
```

## Common Pitfalls

### Avoid: Runtime Variables in Macros

```rust
// WRONG - creates Symbol("i"), not integer value
for i in 0..10 {
    let expr = expr!(i);  // BAD!
}

// CORRECT
for i in 0..10 {
    let expr = Expression::integer(i);  // GOOD!
}
```

### Avoid: Nested Macro Calls

```rust
// WRONG - nested expr!() doesn't work
let expr = expr!(add: expr!(2 * x), expr!(3));  // BAD!

// CORRECT - use intermediate variables or direct patterns
let term1 = expr!(2 * x);
let term2 = expr!(3);
let expr = expr!(add: term1, term2);  // GOOD!

// OR use direct pattern
let expr = expr!(add: (2 * x), 3);  // GOOD!
```

### Avoid: Float Equality

```rust
// WRONG - comparing floats with ==
if expr.evaluate() == 3.14 { }  // BAD!

// CORRECT - use epsilon comparison
if (expr.evaluate() - 3.14).abs() < 1e-10 { }  // GOOD!

// OR use exact rationals
let expr = Expression::rational(314, 100);  // Exact 3.14
```

## Next Steps

- [Core Concepts](../core/expressions.md) - Deep dive into the type system
- [Mathematical Operations](../operations/simplification.md) - Learn all operations
- [Performance](../performance/architecture.md) - Optimization techniques
