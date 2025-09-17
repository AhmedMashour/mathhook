# Expression Evaluation

**Last Updated:** 2025-01-24_1730

MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** - Compute numerical values with domain checking
2. **Simplification** - Algebraic reduction while staying symbolic

Understanding when to use each operation is critical for correct mathematical computation.

## Quick Decision Guide

```text
Need a numerical value?
├─ YES → Use evaluate() or evaluate_with_context()
│   ├─ With variables? → evaluate_with_context(context)
│   └─ Constants only? → evaluate()
│
└─ NO → Need algebraic simplification?
    ├─ YES → Use simplify()
    └─ NO → Keep expression as-is
```

## Key Differences

| Operation | Purpose | Domain Checking | Substitution | Returns |
|-----------|---------|-----------------|--------------|---------|
| **`evaluate()`** | Numerical computation | ✅ Yes | ❌ No | `Result<Expression, MathError>` |
| **`evaluate_with_context()`** | Substitution + computation | ✅ Yes | ✅ Yes | `Result<Expression, MathError>` |
| **`simplify()`** | Algebraic reduction | ❌ No | ❌ No | `Expression` |

## Evaluation: Numerical Computation

`evaluate()` computes numerical values while validating mathematical domain constraints:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::MathError;

// Constants evaluate to numbers
let sum = expr!(2 + 3);
assert_eq!(sum.evaluate().unwrap(), expr!(5));

// Domain checking catches errors
let sqrt_neg = expr!(sqrt(-1));
assert!(matches!(sqrt_neg.evaluate(), Err(MathError::DomainError { .. })));
```

### Domain Constraints Checked

- `sqrt(x)`: Requires x ≥ 0 in real domain
- `log(x)`: Requires x > 0 (pole at 0)
- `tan(x)`: Has poles at π/2 + nπ
- `arcsin(x)`, `arccos(x)`: Require |x| ≤ 1 in real domain
- Division by zero: Checked in `x/y` and `x^(-n)`

## Evaluation with Context: Substitution

`evaluate_with_context()` provides variable substitution and configurable evaluation:

```rust
use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook_core::{symbol, Expression};
use std::collections::HashMap;

let x = symbol!(x);

// Substitute x = 3 and evaluate
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let ctx = EvalContext::numeric(vars);

let expr = Expression::pow(x.clone(), Expression::integer(2));
assert_eq!(expr.evaluate_with_context(&ctx).unwrap(), Expression::integer(9));
```

### Evaluation Context Options

```rust
pub struct EvalContext {
    pub variables: HashMap<String, Expression>,  // Variable substitutions
    pub precision: u32,                          // Numerical precision
    pub simplify_first: bool,                    // Simplify before evaluation?
    pub numeric: bool,                           // Perform numerical evaluation?
}
```

## Simplification: Algebraic Reduction

`simplify()` transforms expressions into equivalent but simpler symbolic forms:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::simplify::Simplify;

let x = symbol!(x);

// Combine like terms
let sum = expr!(x + x);
assert_eq!(sum.simplify(), expr!(2 * x));

// Apply identities
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(0 * x).simplify(), expr!(0));
```

### Simplification Rules Applied

1. **Canonical Form**: Flatten, sort, remove identities
2. **Algebraic Identities**: Combine like terms, power rules
3. **Constant Folding**: Evaluate constant subexpressions
4. **Special Values**: Apply function identities (sin(0) → 0)

### Important: No Domain Checking

Simplification operates purely symbolically without domain validation:

```rust
// No error - stays symbolic or simplifies to i (complex domain)
let sqrt_neg = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);
let result = sqrt_neg.simplify(); // No error! Use evaluate() for domain checking
```

## Common Patterns

### Pattern 1: Evaluate Constants

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let result = expr!(2 + 3).evaluate().unwrap();
assert_eq!(result, expr!(5));
```

### Pattern 2: Simplify Before Solving

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::simplify::Simplify;

let x = symbol!(x);

// Original equation: 2x + 3x = 10
let equation = expr!(2 * x + 3 * x);

// Simplify first: 5x = 10 (easier to solve)
let simplified = equation.simplify();
assert_eq!(simplified, expr!(5 * x));
```

### Pattern 3: Substitute Then Evaluate

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::expression::eval_numeric::EvalContext;
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);

// Formula: f(x, y) = x² + 2xy + y²
let formula = expr!(x ^ 2 + 2 * x * y + y ^ 2);

// Evaluate at (x=3, y=4)
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
vars.insert("y".to_string(), expr!(4));
let ctx = EvalContext::numeric(vars);

let result = formula.evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(49)); // (3 + 4)² = 49
```

## Common Pitfalls

### ❌ Pitfall 1: Expecting Numbers from `simplify()`

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook_core::simplify::Simplify;
let x = symbol!(x);
let result = expr!(x + x).simplify();
// Expected: 2 (numerical value)
// Actual: 2*x (still symbolic!)
```

✅ **Solution:** Use `evaluate_with_context()` for numerical results.

### ❌ Pitfall 2: Using `evaluate()` Without Variables

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let result = expr!(x + 1).evaluate().unwrap();
// Expected: Error or numerical value
// Actual: x + 1 (symbolic, simplified)
```

✅ **Solution:** Use `evaluate_with_context()` to substitute variables.

### ❌ Pitfall 3: Ignoring Domain Errors

```rust
// PANIC!
let result = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)])
    .evaluate()
    .unwrap(); // Panics on domain error!
```

✅ **Solution:** Always handle `Result`:

```rust
match sqrt_neg.evaluate() {
    Ok(result) => println!("Result: {}", result),
    Err(MathError::DomainError { operation, value, reason }) => {
        eprintln!("Domain error in {}: {} ({})", operation, value, reason);
    }
    Err(e) => eprintln!("Other error: {:?}", e),
}
```

## Performance Considerations

- **`simplify()`**: Typically faster (no domain validation)
- **`evaluate()`**: Adds domain checking overhead
- **`evaluate_with_context()`**: Additional substitution overhead

**Optimization:** Simplify once, evaluate many times:

```rust
let simplified = expr.simplify(); // Once
for value in values {
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), Expression::integer(value));
    let ctx = EvalContext::numeric(vars);
    let result = simplified.evaluate_with_context(&ctx).unwrap(); // Many times
}
```

## See Also

- [Function Evaluation](./function_evaluation.md) - Function-specific evaluation
- [Simplification](./simplification.md) - Detailed simplification strategies
- [Substitution](./substitution.md) - Variable substitution patterns
- **Comprehensive Guide**: `../advanced/evaluation_vs_simplification.md`
