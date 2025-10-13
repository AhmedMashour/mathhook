# Quick Start

This guide will get you up and running with MathHook in 5 minutes.

## Your First Expression (Rust)

```rust
use mathhook_core::prelude::*;

fn main() {
    // Create a symbol
    let x = symbol!(x);

    // Build an expression: x^2 + 2x + 1
    let expr = expr!((x ^ 2) + (2 * x) + 1);

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
use mathhook_core::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(r"\frac{x^2 + 1}{2}").unwrap();

println!("{}", expr);
```

### Computing Derivatives

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

// First derivative: 3x^2
let derivative = expr.derivative(&x, 1);

// Second derivative: 6x
let second_derivative = expr.derivative(&x, 2);

println!("f(x) = {}", expr);
println!("f'(x) = {}", derivative);
println!("f''(x) = {}", second_derivative);
```

### Solving Equations

```rust
use mathhook_core::prelude::*;

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
use mathhook_core::Expression;

let matrix = Expression::matrix(vec![
    vec![Expression::integer(1), Expression::integer(2)],
    vec![Expression::integer(3), Expression::integer(4)],
]);

let det = matrix.determinant();
println!("Determinant: {}", det);
// Output: -2
```

## Step-by-Step Explanations

```rust
use mathhook_core::prelude::*;
use mathhook_core::educational::*;

let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

// Get educational explanation
let explanation = expr.explain_simplification();

for step in explanation.steps() {
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
use mathhook_core::Expression;

// For known values at compile time
let expr = expr!((x ^ 2) + 3);

// For runtime/dynamic expressions
let mut terms = Vec::new();
for i in 0..5 {
    terms.push(Expression::mul(vec![
        Expression::integer(i),
        Expression::pow(symbol!(x), Expression::integer(i))
    ]));
}
let polynomial = Expression::add(terms);
```

### Substitution

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let expr = expr!((x ^ 2) + (2 * x) + 1);

// Substitute x = 3
let result = expr.substitute(&x, &Expression::integer(3));
println!("Result: {}", result);
// Output: 16
```

### Formatting Output

```rust
use mathhook_core::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 2);

// Standard notation
println!("Standard: {}", expr);

// LaTeX notation
println!("LaTeX: {}", expr.to_latex());

// Wolfram notation
println!("Wolfram: {}", expr.to_wolfram());
```

## Tips

1. **Use macros** (`symbol!`, `expr!`) for cleaner code
2. **Explicit grouping** with parentheses in `expr!` macro avoids precedence issues
3. **Check errors** when parsing - use `.unwrap()` only in examples
4. **Simplify often** - many operations work better on simplified expressions
5. **Profile first** - don't optimize prematurely

## Common Mistakes

### Runtime Variables in Macros

```rust
// DON'T do this - 'i' is seen as a symbol name, not the value
for i in 0..10 {
    let expr = expr!(i);  // Creates Symbol("i"), not integer i!
}

// DO this instead
for i in 0..10 {
    let expr = Expression::integer(i);
}
```

### Precedence Without Parentheses

```rust
// Unclear precedence
let expr = expr!(2*x + 3);

// Better - explicit grouping
let expr = expr!((2*x) + 3);
```

### Floating Point Comparison

```rust
// DON'T use == for floats
if expr.evaluate() == 3.14159 { }  // BAD

// DO use epsilon comparison or exact rationals
if (expr.evaluate() - 3.14159).abs() < 1e-10 { }  // GOOD
```

## Performance Tips

For performance-critical code:

1. Reuse expressions when possible (they're immutable and cheap to clone)
2. Use SIMD for bulk operations (automatic for arrays > 100 elements)
3. Enable parallel processing for large collections
4. Cache frequently computed results

```rust
use mathhook_core::core::performance::config::{
    PerformanceConfig,
    BindingContext,
    set_global_config
};

// Configure for optimal performance
let config = PerformanceConfig::for_binding(BindingContext::Native);
set_global_config(config);
```

## Next Steps

- [Basic Usage](./basic-usage.md) - Comprehensive usage guide
- [Core Concepts](../core/expressions.md) - Deep dive into the type system
- [API Reference](../api/core.md) - Complete API documentation
