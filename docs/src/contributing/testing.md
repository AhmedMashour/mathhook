# Testing Strategy

Testing ensures mathematical correctness—the highest priority in MathHook.

## Commands

```bash
# All tests (with 10-min timeout)
gtimeout 600s cargo test --no-fail-fast

# Doctests only
gtimeout 600s cargo test --no-fail-fast --doc

# Specific crate
cargo test -p mathhook-core

# Specific module
cargo test -p mathhook-core parser

# Single test
cargo test test_sin_special_values
```

## Test Categories

### Unit Tests

Test individual functions in isolation.

```rust
#[test]
fn test_sin_zero() {
    let result = sin(&expr!(0)).unwrap();
    assert_eq!(result, expr!(0));
}
```

### Integration Tests

Test through the public API—not just internals.

```rust
#[test]
fn test_public_simplify_api() {
    // ✅ Tests actual user-facing behavior
    let result = mathhook::simplify("x + x");
    assert_eq!(result, "2*x");
}

// ❌ Don't only test internal functions
// Internal tests can pass while public API is broken
```

### Doctest Requirements

All public functions need working examples:

```rust
/// Compute sine of an expression.
///
/// # Examples
///
/// ```rust
/// use mathhook::prelude::*;
///
/// let x = symbol!(x);
/// let result = sin(&expr!(0)).unwrap();
/// assert_eq!(result, expr!(0));
/// ```
pub fn sin(arg: &Expression) -> Result<Expression, MathError> { ... }
```

## Mathematical Test Requirements

### Edge Cases (ALWAYS Test)

Every mathematical operation must test:

| Case | Example |
|------|---------|
| Zero | `f(0)` |
| Negative | `f(-1)`, `f(-x)` |
| Complex | `f(i)`, `f(1+2i)` |
| Infinity | `f(∞)`, `f(-∞)` |
| Undefined | Poles, domain violations |
| Symbolic | `f(x)` where x is unknown |

### Special Values

Test all special values from `data.rs`:

```rust
#[test]
fn test_gamma_special_values() {
    // Integer values: Γ(n) = (n-1)!
    assert_eq!(gamma(&expr!(1)).unwrap(), expr!(1));
    assert_eq!(gamma(&expr!(5)).unwrap(), expr!(24));

    // Half-integer: Γ(1/2) = √π
    assert_eq!(gamma(&expr!(1/2)).unwrap(), expr!(sqrt(pi)));

    // Poles (undefined at non-positive integers)
    assert!(gamma(&expr!(0)).is_err());
    assert!(gamma(&expr!(-1)).is_err());
}
```

### Domain Errors

```rust
#[test]
fn test_log_domain_error() {
    // log(0) is undefined
    let result = log(&expr!(0));
    assert!(matches!(result, Err(MathError::DomainError { .. })));
}
```

### Mathematical Properties

```rust
#[test]
fn test_sin_odd_function() {
    // sin(-x) = -sin(x)
    let x = symbol!(x);
    let neg_x = expr!(-x);

    let sin_neg = sin(&neg_x).unwrap();
    let neg_sin = expr!(-sin(x));

    assert_eq!(sin_neg, neg_sin);
}

#[test]
fn test_sin_periodicity() {
    // sin(x + 2π) = sin(x)
    let x = symbol!(x);
    let shifted = expr!(x + 2 * pi);

    // After simplification, should be equivalent
    assert_eq!(simplify(&sin(&shifted)), simplify(&sin(&x)));
}
```

## SymPy Validation

**Requirement:** Verify against SymPy before implementing math algorithms.

```bash
# Validation scripts
./scripts/validate.sh               # All validations
./scripts/validate.sh simplify      # Simplification only
./scripts/validate.sh ode           # ODE solver only
```

### Manual Verification

```python
# In SymPy
from sympy import *

x = Symbol('x')
print(simplify(sin(x)**2 + cos(x)**2))  # Should be 1
print(integrate(x**2, x))                # Should be x**3/3
```

Compare MathHook output against SymPy results.

## Numerical Accuracy

### Float Comparison

```rust
// ❌ NEVER use ==
assert_eq!(result, 1.0);

// ✅ ALWAYS use epsilon
const EPSILON: f64 = 1e-10;
assert!((result - 1.0).abs() < EPSILON);
```

### Numerical vs Symbolic

```rust
#[test]
fn test_sin_pi_exact() {
    // sin(π) must be exactly 0, not 1.2246e-16
    let result = sin(&Expression::pi()).unwrap();
    assert_eq!(result, expr!(0));  // Exact symbolic zero
}
```

## Test Organization

```
crates/mathhook-core/src/
├── functions/
│   └── sin/
│       ├── mod.rs       # Implementation
│       ├── data.rs      # Special values
│       └── tests.rs     # Unit tests
└── tests/               # Integration tests
```

**Rule:** Tests go in `tests.rs` submodule, not scattered in main code.

## Test Naming

```rust
// Pattern: test_{function}_{scenario}
#[test] fn test_sin_zero() { ... }
#[test] fn test_sin_pi() { ... }
#[test] fn test_sin_negative() { ... }
#[test] fn test_sin_complex() { ... }
#[test] fn test_sin_domain_error() { ... }
```

## Regression Prevention

```bash
# Before any change
cargo test  # Must pass

# After changes
cargo test  # Must still pass

# For performance changes
./scripts/bench.sh save before-change
# Make changes
./scripts/bench.sh compare before-change
```

## What NOT to Do

| ❌ Wrong | ✅ Right |
|----------|----------|
| Skip tests to make build pass | Fix the underlying issue |
| Comment out failing tests | Investigate root cause |
| Test only happy path | Test edge cases and errors |
| Only unit tests | Include integration tests |
| Hardcode expected values | Derive from mathematical properties |
