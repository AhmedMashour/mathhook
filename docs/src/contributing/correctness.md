# Mathematical Correctness

**Mathematical correctness is Priority Level 1.** Nothing else matters if the math is wrong.

## Why This Is Non-Negotiable

MathHook is used by:
- **Students** learning mathematics
- **Engineers** making calculations
- **Researchers** validating results

A wrong answer doesn't just fail—it **actively harms** users who trust it.

If `sin(π)` returns `1.2246e-16` instead of `0`:
- A student learns wrong
- An engineer's calculation is off
- A researcher's proof is invalid

**There is no acceptable rate of mathematical errors. Zero is the only target.**

## Core Principles

1. **Never ship incorrect math** to fix a bug elsewhere
2. **Never sacrifice correctness for performance**
3. **Never assume—verify** against SymPy
4. **Test edge cases obsessively**
5. **Domain errors must be caught**, not silently wrong

## Validation Process

### Before Implementation

```bash
# Verify against SymPy
python3 -c "
from sympy import *
x = Symbol('x')
print(simplify(sin(x)**2 + cos(x)**2))
"
```

### After Implementation

```bash
./scripts/validate.sh           # All validations
./scripts/validate.sh simplify  # Specific module
```

## Edge Cases (ALWAYS Test)

| Case | Example | Why It Matters |
|------|---------|----------------|
| Zero | `f(0)` | Identity behavior |
| Negative | `f(-1)` | Sign handling |
| Complex | `f(i)` | Branch cuts |
| Infinity | `f(∞)` | Limit behavior |
| Undefined | `tan(π/2)` | Domain restrictions |

## Symbol Assumptions

Symbols carry assumptions that affect correctness:

```rust
// sqrt(x^2) depends on assumptions about x
let x = symbol!(x);  // Unknown sign

// sqrt(x^2) = |x|, not x (unless x is positive)
let x_positive = symbol!(x).with_assumption(Assumption::Positive);
// Now sqrt(x^2) = x is correct
```

### Assumption Categories

| Category | Assumptions |
|----------|-------------|
| Domain | `real`, `complex`, `positive`, `negative`, `nonzero` |
| Type | `integer`, `rational`, `prime`, `even`, `odd` |
| Bounds | `bounded(a, b)` |

## Numerical Precision

### Float Comparison

```rust
// ❌ NEVER - Floating point equality is unreliable
if result == 0.0 { ... }

// ✅ ALWAYS - Use epsilon comparison
const EPSILON: f64 = 1e-10;
if result.abs() < EPSILON { ... }
```

### Exact vs Approximate

```rust
// Symbolic π must give exact results
sin(Expression::pi())  // Must return exactly 0, not 1.2246e-16

// Only use floats for numerical approximation
expr!(3.14159)  // Approximate - loses exactness
Expression::pi()  // Symbolic - maintains exactness
```

## Complex Numbers

### Branch Cuts

| Function | Branch Cut | Principal Value |
|----------|------------|-----------------|
| `sqrt(z)` | Negative real axis | `Re(sqrt(z)) ≥ 0` |
| `log(z)` | Negative real axis | `-π < arg(z) ≤ π` |
| `z^w` | Uses principal `log` | `z^w = exp(w * log(z))` |

### Default Behavior

```rust
// Complex-safe by default
sqrt(&expr!(-1))  // Returns i, not NaN or error
```

## Domain Restrictions

| Function | Valid Domain | Error Outside |
|----------|--------------|---------------|
| `sqrt(x)` | x ≥ 0 (real) | Complex or `DomainError` |
| `log(x)` | x > 0 | Complex or `DomainError` |
| `1/x` | x ≠ 0 | `DivisionByZero` |
| `tan(x)` | x ≠ π/2 + nπ | `Undefined` |

## Error Handling for Domain Violations

```rust
pub fn log(arg: &Expression) -> Result<Expression, MathError> {
    // Check for zero
    if arg.is_zero() {
        return Err(MathError::DomainError {
            operation: "log".into(),
            value: arg.clone(),
            reason: "logarithm of zero is undefined".into(),
        });
    }

    // Check for negative (if real mode)
    if arg.is_negative_real() {
        return Err(MathError::DomainError {
            operation: "log".into(),
            value: arg.clone(),
            reason: "logarithm of negative real requires complex mode".into(),
        });
    }

    // Continue with evaluation...
}
```

## Simplification Rules

### Order of Operations

1. **Canonicalize** - Flatten, sort, remove identity elements
2. **Apply identities** - Combine like terms, power rules, trig
3. **Numerical evaluation** - Only if explicitly requested

### Idempotence Requirement

```rust
// Simplification must be idempotent
let once = simplify(&expr);
let twice = simplify(&once);
assert_eq!(once, twice);  // Must be equal
```

## Canonical Forms

| Operation | Canonical Form |
|-----------|---------------|
| Commutative | `y + x` → `x + y` |
| Associative | `(a + b) + c` → `Add(a, b, c)` |
| Identity | `x + 0` → `x` |
| Rationals | `6/4` → `3/2` |
| Subtraction | `a - b` → `a + (-1 * b)` |
| Division | `a / b` → `a * b^(-1)` |

## Common Errors to Avoid

| Error | Example | Consequence |
|-------|---------|-------------|
| Sign error in chain rule | `d/dx[f(g(x))] = f'(x)*g'(x)` | Wrong derivatives |
| Ignoring assumptions | `sqrt(x^2) = x` | Wrong for negative x |
| Float equality | `sin(π) == 0.0` | Fails due to precision |
| Wrong branch cut | `log(-1) = -πi` | Should be `πi` |

## Reference Materials

For complex cases, consult:

1. **Abramowitz & Stegun** - Handbook of Mathematical Functions
2. **DLMF** - Digital Library of Mathematical Functions
3. **SymPy source** - For implementation details
