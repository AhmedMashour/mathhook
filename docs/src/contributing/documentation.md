# Documentation Standards

Documentation must be accurate, complete, and match the actual code.

## Required Documentation

### Public API

Every public function, struct, trait, and enum needs documentation:

```rust
/// Compute the derivative of an expression.
///
/// # Arguments
///
/// * `expr` - The expression to differentiate
/// * `var` - The variable to differentiate with respect to
///
/// # Returns
///
/// * `Ok(Expression)` - The derivative
/// * `Err(MathError)` - If differentiation fails
///
/// # Examples
///
/// ```rust
/// use mathhook::prelude::*;
///
/// let x = symbol!(x);
/// let f = expr!(x ^ 2);
/// let df = differentiate(&f, &x)?;
/// assert_eq!(df, expr!(2 * x));
/// ```
pub fn differentiate(expr: &Expression, var: &Symbol) -> Result<Expression, MathError>
```

### Module Level

Each module needs a `//!` header:

```rust
//! Polynomial operations for symbolic computation.
//!
//! This module provides:
//! - GCD computation via Euclidean algorithm
//! - Polynomial division with remainder
//! - Factorization over integers and finite fields
```

## Documentation Structure

### Example Pattern

```rust
/// Brief one-line description.
///
/// Longer explanation if needed, describing behavior,
/// mathematical background, or implementation notes.
///
/// # Arguments
///
/// * `param1` - Description
/// * `param2` - Description
///
/// # Returns
///
/// Description of return value or Result variants.
///
/// # Errors
///
/// - `MathError::DomainError` - When input is outside valid domain
/// - `MathError::Undefined` - When result is mathematically undefined
///
/// # Panics
///
/// This function does not panic. (Or list panic conditions if any)
///
/// # Examples
///
/// ```rust
/// // Working example with assertions
/// ```
///
/// # Mathematical Background
///
/// Optional section for complex algorithms.
pub fn function_name(...) { ... }
```

## Doctests

### Requirements

1. **Every example must compile and run**
2. **Include assertions to verify behavior**
3. **Use `use mathhook::prelude::*;`** for common imports

### Patterns

```rust
/// # Examples
///
/// Basic usage:
/// ```rust
/// use mathhook::prelude::*;
///
/// let x = symbol!(x);
/// let result = sin(&expr!(0)).unwrap();
/// assert_eq!(result, expr!(0));
/// ```
///
/// Error handling:
/// ```rust
/// use mathhook::prelude::*;
///
/// let result = log(&expr!(0));
/// assert!(result.is_err());
/// ```
```

### Hidden Lines

Use `#` to hide setup code:

```rust
/// ```rust
/// # use mathhook::prelude::*;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let x = symbol!(x);
/// let f = expr!(x ^ 2);
/// # Ok(())
/// # }
/// ```
```

## mdbook Documentation

### Location

All user-facing documentation lives in `docs/src/`.

### Building

```bash
cd docs
mdbook serve --open  # Preview at localhost:3000
mdbook build         # Build static site
```

### LaTeX

Use `$...$` for inline math and `$$...$$` for display math:

```markdown
The quadratic formula is $x = \frac{-b \pm \sqrt{b^2-4ac}}{2a}$.

For the heat equation:

$$\frac{\partial u}{\partial t} = \alpha \nabla^2 u$$
```

**Note:** We use mdbook-katex preprocessor. Avoid characters that conflict with Markdown (escape underscores in text mode).

## Code Examples in Docs

### Always Use Macros

```markdown
✅ Correct:
​```rust
let x = symbol!(x);
let f = expr!(x ^ 2 + 2 * x + 1);
​```

❌ Wrong:
​```rust
let x = Symbol::new("x");  // Never in docs
​```
```

### Show Practical Usage

```markdown
// Complete working example
let x = symbol!(x);
let f = expr!(sin(x) ^ 2 + cos(x) ^ 2);
let simplified = simplify(&f);
assert_eq!(simplified, expr!(1));
```

## API Reference Accuracy

### Verify Before Publishing

1. **Check function signatures** match actual code
2. **Verify return types** (especially `Result` vs direct return)
3. **Test all examples** with `cargo test --doc`
4. **Check method names** match (`.to_latex()` parameters, etc.)

### Common Errors

| Error | Example | Fix |
|-------|---------|-----|
| Wrong method signature | `.to_latex(ctx)` | Check actual API |
| Outdated macro | `Symbol::matrix("A")` | Use `symbol!(A; matrix)` |
| Missing Result handling | `function(&x)` | Add `.unwrap()` or `?` |
| Wrong import | `use mathhook::*` | `use mathhook::prelude::*` |

## Keeping Docs Updated

### When Code Changes

1. Update corresponding documentation
2. Run `cargo test --doc` to verify examples
3. Check mdbook builds without errors

### Audit Process

```bash
# Find potentially outdated patterns
grep -r "Symbol::new" docs/src/
grep -r "Symbol::matrix" docs/src/
grep -r "LaTeXContext::default" docs/src/

# Verify mdbook builds
cd docs && mdbook build
```

## What NOT to Document

1. **Private implementation details** - They can change
2. **Obvious behavior** - `add` adds things
3. **Internal helper functions** - Unless exposed publicly
4. **Deprecated patterns** - Remove, don't document
