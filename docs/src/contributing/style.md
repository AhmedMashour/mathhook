# Code Style Guide

Code style ensures maintainability and consistency across MathHook.

## Required Commands

Run before every commit:

```bash
cargo fmt                      # Format code
cargo clippy -- -D warnings    # Lint (zero warnings allowed)
```

## Macros Over Constructors

**Always use macros in application code:**

```rust
// ✅ ALWAYS - Use macros
symbol!(x)                     // Scalar symbol
symbol!(A; matrix)             // Matrix symbol
symbol!(p; operator)           // Operator symbol
symbols![x, y, z]              // Multiple symbols
function!(sin, x)              // Function call
expr!(x ^ 2 + 2 * x + 1)       // Expression

// ❌ NEVER - Direct constructors in app code
Symbol::new("x")               // Forbidden
Symbol::matrix("A")            // Forbidden
Expression::Function { ... }   // Forbidden
```

### Runtime Variables

```rust
// ❌ WRONG - Creates symbol named "i", not integer i
for i in 0..10 {
    expr!(i)  // Bug!
}

// ✅ RIGHT - Explicit API for runtime values
for i in 0..10 {
    Expression::integer(i)
}
```

## File Size Limit

**Maximum 500 lines** per file (including comments and blanks).

```bash
# Check file size
wc -l filename.rs

# If approaching 400 lines, plan the split
```

## Module Naming

```
✅ Correct:
src/
├── parser.rs            # Module file
└── parser/              # Submodules
    ├── lexer.rs
    └── grammar.rs

❌ Wrong:
src/
└── parser/
    └── mod.rs           # Never use mod.rs
```

## Comments Policy

**Default: No comments.** Code should be self-documenting.

### Allowed Comments

```rust
// Mathematical formula: x = (-b ± √(b²-4ac)) / 2a
// O(n²) but n < 10 in practice
// x must be positive for real sqrt
```

### Forbidden Comments

```rust
// ❌ Create a new expression
// ❌ Loop through items
// ❌ Return the result
// ❌ Increment counter
// ❌ Check if null
```

If the code needs a comment explaining what it does, the code needs rewriting.

## Documentation Comments

### Module Level (`//!`)

```rust
//! Trigonometric function implementations.
//!
//! Provides exact symbolic evaluation for sin, cos, tan, and their inverses.
```

### Public API (`///`)

```rust
/// Compute sine of an expression.
///
/// # Arguments
///
/// * `arg` - The input expression
///
/// # Returns
///
/// * `Ok(Expression)` - The evaluated result
/// * `Err(MathError)` - Domain error if applicable
///
/// # Examples
///
/// ```rust
/// use mathhook::prelude::*;
///
/// let result = sin(&expr!(0)).unwrap();
/// assert_eq!(result, expr!(0));
/// ```
pub fn sin(arg: &Expression) -> Result<Expression, MathError> { ... }
```

## Naming Conventions

### Functions and Variables

```rust
// snake_case for functions and variables
fn compute_derivative(expr: &Expression) -> Expression { ... }
let result_value = evaluate(&expr)?;
```

### Types and Traits

```rust
// PascalCase for types
struct Expression { ... }
trait Evaluable { ... }
enum MathError { ... }
```

### Constants

```rust
// SCREAMING_SNAKE_CASE for constants
const MAX_ITERATIONS: usize = 1000;
static PI_VALUE: LazyLock<Expression> = ...;
```

## Error Handling

### Return Types

```rust
// Infallible operations → direct return
pub fn add(terms: Vec<Expression>) -> Expression { ... }

// Fallible operations → Result
pub fn evaluate(expr: &Expression) -> Result<Expression, MathError> { ... }
```

### No Panics in Library Code

```rust
// ❌ NEVER
fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 { panic!("division by zero"); }
    a / b
}

// ✅ ALWAYS
fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 { return Err(MathError::DivisionByZero); }
    Ok(a / b)
}
```

## Imports Organization

```rust
// 1. Standard library
use std::collections::HashMap;
use std::sync::LazyLock;

// 2. External crates
use num_rational::Ratio;

// 3. Crate modules
use crate::core::Expression;
use crate::error::MathError;

// 4. Local modules
use super::data::SPECIAL_VALUES;
```

## Expression Construction

### Power Operations

```rust
// All three are equivalent - use what's clearest
expr!(x ^ 2)       // Mathematical notation
expr!(x ** 2)      // Python-style
expr!(x.pow(2))    // Method call
```

### Precedence

```rust
// ^ binds tighter than * and /
expr!(2 * x ^ 2)   // Parsed as 2 * (x^2)

// Right-associative
expr!(2 ^ 3 ^ 4)   // Parsed as 2^(3^4) = 2^81
```

## What to Avoid

| Pattern | Problem | Alternative |
|---------|---------|-------------|
| `Symbol::new("x")` | Bypasses macro | `symbol!(x)` |
| `mod.rs` | Old pattern | `modulename.rs` |
| `== 0.0` for floats | Floating point errors | `abs() < EPSILON` |
| `unwrap()` in lib | Can panic | Return `Result` |
| Comments restating code | Noise | Delete them |
| Files > 500 lines | Unmaintainable | Split module |
