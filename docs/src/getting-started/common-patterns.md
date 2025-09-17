# Common Patterns

This chapter covers common patterns and best practices when using MathHook.

## Macro Usage Guidelines

### When to Use Macros

**ALWAYS use macros for:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
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
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Runtime/loop variables
for i in 0..10 {
    let term = Expression::integer(i);  // NOT expr!(i)
}

// Programmatic construction with runtime data
let x = symbol!(x);
let coefficients = vec![1, 2, 3];
let n = coefficients.len();
let mut terms = Vec::new();
for i in 0..n {
    // Can't use expr! with runtime array access
    let coeff = Expression::integer(coefficients[i]);
    let x_expr = Expression::from(x.clone());
    let power = Expression::integer(i as i64);
    terms.push(Expression::mul(vec![coeff, Expression::pow(x_expr, power)]));
}
let polynomial = Expression::add(terms);
```

## Building Polynomials

### Fixed Degree

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// ax^2 + bx + c
let a = Expression::integer(1);
let b = Expression::integer(2);
let c = Expression::integer(1);

// Multi-term addition: use add: helper or explicit API
let quadratic = expr!(add: (a * (x ^ 2)), (b * x), c);
```

### Dynamic Degree

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn build_polynomial(coefficients: &[i64], x: &Symbol) -> Expression {
    let mut terms = Vec::new();
    for (i, &coeff) in coefficients.iter().enumerate() {
        // Runtime variables - use explicit API
        let coeff_expr = Expression::integer(coeff);
        let x_expr = Expression::from(x.clone());
        let power = Expression::integer(i as i64);
        let term = Expression::mul(vec![coeff_expr, Expression::pow(x_expr, power)]);
        terms.push(term);
    }
    Expression::add(terms)
}

let x = symbol!(x);
let poly = build_polynomial(&[1, -5, 6], &x);  // x^2 - 5x + 6
```

## Substitution Patterns

### Single Substitution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
// Multi-term expression: use add: helper
let expr = expr!(add: (x ^ 2), (2 * x), 1);

// Substitute x = 3
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);
```

### Multiple Substitutions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);
// Multi-term expression: use add: helper
let expr = expr!(add: (x * y), x, y);

// Substitute both x and y
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(2));
vars.insert("y".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);
```

## Working with Functions

### Creating Function Expressions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Using expr! macro (preferred for expressions)
let f = expr!(sin(x));
let g = expr!(cos(x));

// Using function! macro (preferred for single function calls)
let h = function!(tan, x);

// Using Expression::function for runtime function names
let func_name = "tan";
let runtime_func = Expression::function(func_name, vec![Expression::from(x.clone())]);
```

### Composing Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// sin(cos(x)) - can use expr! directly
let composed = expr!(sin(cos(x)));

// Or build step by step
let inner = expr!(cos(x));
let composed_alt = function!(sin, inner);

// Verify composition
println!("Composed function: {}", composed);
```

## Matrix Patterns

### Creating Matrices

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// From vectors
let matrix = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);

// Identity matrix
let identity = Expression::identity_matrix(3);

// Zero matrix
let zero = Expression::zero_matrix(2, 3);
```

### Matrix Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Create matrices
let a = Expression::matrix(vec![
    vec![Expression::integer(1), Expression::integer(2)],
    vec![Expression::integer(3), Expression::integer(4)],
]);

let b = Expression::matrix(vec![
    vec![Expression::integer(5), Expression::integer(6)],
    vec![Expression::integer(7), Expression::integer(8)],
]);

// Matrix operations available through Expression API
println!("Matrix A: {}", a);
println!("Matrix B: {}", b);
```

## Error Handling

### Parsing Errors

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let parser = Parser::new(ParserConfig::default());
let input = "x + 2*y";

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
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
// Multi-operator expression needs explicit grouping
let equation = expr!(add: (x ^ 2), (-4));

let mut solver = MathSolver::new();
let result = solver.solve(&equation, &x);

// Handle solver results (SolverResult is a type alias)
println!("Solutions: {:?}", result);
```

## Performance Patterns

### Bulk Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Simplify many expressions
let expressions = vec![
    expr!(x + x),
    expr!(x * 1),
    expr!(add: (x ^ 2), (-(x ^ 2))),  // Use add: helper for subtraction
];

let simplified: Vec<_> = expressions
    .iter()
    .map(|e| e.simplify())
    .collect();
```

### Caching Results

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let mut cache: HashMap<String, Expression> = HashMap::new();

let expr = expr!(x ^ 2);
let key = format!("{}", expr);

if let Some(cached) = cache.get(&key) {
    // Use cached result
    println!("Using cached result");
} else {
    // Compute and cache
    let result = expr.simplify();
    cache.insert(key, result.clone());
}
```

## Educational Patterns

### Step-by-Step Explanations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

// Educational features show mathematical steps
// (API may vary - check current documentation)
let simplified = expr.simplify();
println!("Original: {}", expr);
println!("Simplified: {}", simplified);
```

### Derivative Explanations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
// Can use expr! directly for complex expressions
let expr = expr!(sin(x ^ 2));

// Compute derivative (chain rule applied automatically)
// Derivative trait is in prelude
let derivative = expr.derivative(x);
println!("f(x) = {}", expr);
println!("f'(x) = {}", derivative);
```

## Common Pitfalls

### Avoid: Runtime Variables in Macros

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
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
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// WRONG - nested expr!() doesn't work
// let bad_expr = expr!(add: expr!(2 * x), expr!(3));  // Won't compile!

// CORRECT - use intermediate variables or direct patterns
let term1 = expr!(2 * x);
let term2 = Expression::integer(3);
let good_expr = expr!(add: term1, term2);  // GOOD!

// OR use direct pattern
let good_expr2 = expr!(add: (2 * x), 3);  // GOOD!
```

### Avoid: Float Equality

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// WRONG - comparing floats directly with ==
let val1: f64 = 3.14;
let val2: f64 = 3.14000000001;
// if val1 == val2 { }  // BAD for approximate values!

// CORRECT - use epsilon comparison for floats
let tolerance: f64 = 1e-10;
if (val1 - val2).abs() < tolerance {
    println!("Values are approximately equal");
}

// OR use exact rationals for symbolic computation
let exact = Expression::rational(314, 100);  // Exact 3.14
```

## Next Steps

- [Core Concepts](../core/expressions.md) - Deep dive into the type system
- [Mathematical Operations](../operations/simplification.md) - Learn all operations
- [Performance](../performance/architecture.md) - Optimization techniques
