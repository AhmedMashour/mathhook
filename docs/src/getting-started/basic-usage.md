# Basic Usage

This chapter provides a guide to using MathHook in your projects.

## Expression Creation

### Using Macros

The recommended way to create expressions is using the `expr!` and `symbol!` macros:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Simple arithmetic
let expr1 = expr!(x + y);
let expr2 = expr!(2 * x);
let expr3 = expr!(x ^ 2);

// Complex expressions with explicit grouping
let expr4 = expr!((x + 1) * (x - 1));

// Multi-term expressions: use add: helper
let expr5 = expr!(add: (2*x), (3*y), (-5));
```

### Using Constructors

For programmatic construction:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Numbers
let int = Expression::integer(42);
let float = Expression::float(3.14);
let rational = Expression::rational(3, 4);  // 3/4

// Operations
let sum = expr!(1 + 2);

let product = expr!(2 * x);
```

## Simplification

Simplification transforms expressions to their canonical form:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Combine like terms
let expr = expr!(x + x);
let simplified = expr.simplify();
// Result: 2*x

// Apply identities
let expr = expr!(x * 1);
let simplified = expr.simplify();
// Result: x

// Evaluate constants
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5
```

## Pattern Matching

Work with expression structure:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);
let test_expr = expr!(x + y);

match test_expr {
    Expression::Add(terms) => {
        println!("Addition with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Multiplication with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: base={}, exp={}", base, exp);
    }
    _ => {}
}
```

## Working with Symbols

Symbols represent variables in expressions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Symbols with same name are equal
assert_eq!(symbol!(x), symbol!(x));

// Different names are not equal
assert_ne!(symbol!(x), symbol!(y));
```

## Number Types

MathHook supports multiple number representations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Integers (exact, arbitrary precision supported for larger values)
let int = Expression::integer(123456789);

// Rationals (exact fractions)
let frac = Expression::rational(22, 7);  // 22/7 ≈ π

// Floats (approximate)
let float = Expression::float(3.14159265359);

// Complex numbers
let complex = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);  // 3 + 4i
```

## Constants

Mathematical constants are built-in:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let pi = Expression::pi();
let e = Expression::e();
let i = Expression::i();              // imaginary unit
let phi = Expression::golden_ratio();
let gamma = Expression::euler_gamma();
```

## Function Expressions

Create function calls:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Elementary functions using expr! macro
let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let log_x = expr!(log(x));

// Or using function! macro
let tan_x = function!(tan, x);
```

## Next Steps

- [Common Patterns](./common-patterns.md) - Idioms and best practices
- [Core Concepts](../core/expressions.md) - Deep dive into the type system
- [Mathematical Operations](../operations/simplification.md) - Learn all operations
