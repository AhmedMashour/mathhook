# Quick Start

This guide will get you up and running with MathHook in 5 minutes.

## Your First Expression (Rust)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

fn main() {
    // Create a symbol
    let x = symbol!(x);

    // Build an expression: x^2 + 2x + 1
    // Three equivalent ways to write powers:
    // expr!(x ^ 2)   - Infix ^ (natural math notation)
    // expr!(x ** 2)  - Infix ** (Python-style)
    // expr!(x.pow(2)) - Method call

    // Use explicit add: helper for multi-term expressions
    let expr = expr!(add: (x ^ 2), (2 * x), 1);

    // Simplify it
    let simplified = expr.simplify();

    println!("Original: {}", expr);
    println!("Simplified: {}", simplified);
}
```

## Your First Expression (Python)

```python
from mathhook import Expression

# Create a symbol
x = Expression.symbol('x')

# Build an expression: x^2 + 2x + 1
expr = x.pow(2).add(x.multiply(2)).add(1)

# Simplify it
simplified = expr.simplify()

print(f"Original: {expr}")
print(f"Simplified: {simplified}")
```

## Your First Expression (Node.js/TypeScript)

```typescript
import { Expression } from 'mathhook-node';

// Create a symbol
const x = Expression.symbol('x');

// Build an expression: x^2 + 2x + 1
const expr = x.pow(2).add(x.multiply(2)).add(1);

// Simplify it
const simplified = expr.simplify();

console.log(`Original: ${expr.toString()}`);
console.log(`Simplified: ${simplified.toString()}`);
```

## Common Operations

### Parsing LaTeX

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(r"\frac{x^2 + 1}{2}").unwrap();

println!("{}", expr);
```

### Computing Derivatives

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

// First derivative: 3x^2
let derivative = expr.derivative(x.clone());

// Second derivative: 6x (using nth_derivative for higher orders)
let second_derivative = expr.nth_derivative(x, 2);

println!("f(x) = {}", expr);
println!("f'(x) = {}", derivative);
println!("f''(x) = {}", second_derivative);
```

### Expression Operators

The `expr!` macro supports mathematical operators and method calls:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Comparison operators
let eq = expr!(x == y);       // Equality
let lt = expr!(x < y);        // Less than
let gt = expr!(x > y);        // Greater than
let le = expr!(x <= y);       // Less than or equal
let ge = expr!(x >= y);       // Greater than or equal

// Method calls
let abs_val = expr!(x.abs());     // Absolute value
let sqrt_val = expr!(x.sqrt());   // Square root
let simplified = expr!(x.simplify()); // Simplify expression

// Power operations - three equivalent syntaxes
let power1 = expr!(x ^ 2);        // Infix ^ (natural math notation)
let power2 = expr!(x ** 2);       // Infix ** (Python-style)
let power3 = expr!(x.pow(2));     // Method call
```

### Solving Equations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x^2 = 4
let mut solver = MathSolver::new();
let equation = Expression::equation(expr!(x ^ 2), expr!(4));
let solutions = solver.solve(&equation, &x);

println!("Solutions: {:?}", solutions);
// Output: [x = 2, x = -2]
```

### Matrix Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let matrix = Expression::matrix(vec![
    vec![Expression::integer(1), Expression::integer(2)],
    vec![Expression::integer(3), Expression::integer(4)],
]);

// Matrix operations (determinant, inverse, etc.)
println!("Matrix: {}", matrix);
// Note: determinant() is in matrix module - this example shows matrix creation
```

## Step-by-Step Explanations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

// Educational features for step-by-step learning
let explanation = expr.explain_simplification();

// Access steps field directly
for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}
```

## What's Next?

Now that you've seen the basics, dive deeper into:

- [Basic Usage](./basic-usage.md) - Detailed guide to expression building and manipulation
- [Core Concepts](../core/expressions.md) - Understanding the type system
- [Mathematical Operations](../operations/simplification.md) - Learn all the operations MathHook supports
- [Common Patterns](./common-patterns.md) - Idioms and best practices

## Common Patterns

### Creating Expressions Programmatically

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);

// For known values at compile time - use macros
let expr = expr!((x ^ 2) + 3);

// For runtime/dynamic expressions - use explicit API
let mut terms = Vec::new();
for i in 0..5 {
    // Can't use expr!(i) - 'i' is runtime variable!
    terms.push(Expression::mul(vec![
        Expression::integer(i as i64),
        Expression::pow(x.clone().into(), Expression::integer(i as i64))
    ]));
}
let polynomial = Expression::add(terms);
```

### Substitution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
// Multi-term expression: use add: helper
let expr = expr!(add: (x ^ 2), (2 * x), 1);

// Substitute x = 3
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);
println!("Result: {}", result);
// Output: 16
```

### Formatting Output

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 2);

// Standard notation (Display trait)
println!("Standard: {}", expr);

// LaTeX notation
let latex = expr.to_latex(None).unwrap();
println!("LaTeX: {}", latex);

// Wolfram notation (returns Result, use unwrap for examples)
println!("Wolfram: {}", expr);  // Use Display for simple cases
```

## Tips

1. **Use macros** (`symbol!`, `symbols!`, `function!`, `expr!`) for cleaner code
2. **Explicit grouping** with parentheses in `expr!` macro avoids precedence issues
3. **Check errors** when parsing - use `.unwrap()` only in examples
4. **Simplify often** - many operations work better on simplified expressions
5. **Profile first** - don't optimize prematurely

### Using the `function!` Macro

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Single argument function
let sin_x = function!(sin, x);

// Multi-argument function
let log_xy = function!(log, x, y);

// Zero-argument function call
let gamma_val = function!(gamma);
```

## Common Mistakes

### Runtime Variables in Macros

The `expr!` macro operates at compile-time, so runtime variables become symbol names:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ DON'T do this
for i in 0..10 {
    let expr = expr!(i);  // Creates Symbol("i"), not integer i!
}

// ✅ DO this instead
for i in 0..10 {
    let expr = Expression::integer(i);  // Correctly creates integer
}
```

**Rule:** If the value comes from a variable, loop, or conditional → use the explicit API (`Expression::integer()`, `Expression::symbol()`, etc.).

### Precedence Without Parentheses

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Unclear precedence - DON'T do this
// let bad_expr = expr!(2*x + 3);  // Won't compile!

// Better - explicit grouping
let good_expr = expr!((2*x) + 3);
```

### Floating Point Comparison

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let test_expr = expr!(x ^ 2);

// Create a context for numerical evaluation
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(2));
let result = test_expr.substitute(&vars);

// DON'T use == for floats - this is approximate
let pi_approx: f64 = 3.14159;
let tolerance: f64 = 1e-10;

// DO use epsilon comparison for numerical values
let difference = (pi_approx - 3.14159).abs();
assert!(difference < tolerance);
```

## Performance Tips

For performance-critical code:

1. Reuse expressions when possible (they're immutable and cheap to clone)
2. Use macros for cleaner, more efficient expression creation
3. Batch operations when working with multiple expressions
4. Cache frequently computed results

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Reuse symbols
let x = symbol!(x);
let y = symbol!(y);

// Efficient expression creation with macros
let expr1 = expr!(x ^ 2);
let expr2 = expr!(y ^ 3);

// Clone is cheap (expressions are immutable)
let expr1_copy = expr1.clone();
```

## Next Steps

- [Basic Usage](./basic-usage.md) - Usage guide
- [Core Concepts](../core/expressions.md) - Deep dive into the type system
- [API Reference](../api/core.md) - Complete API documentation
