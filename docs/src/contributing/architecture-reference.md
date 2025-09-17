# Quick Reference for Core Math Functions

## Module Structure Pattern

```
src/core/functions/FUNCNAME/
├── mod.rs       # Main implementation with evaluate(), properties(), simplify()
├── data.rs      # Special values HashMap (LazyLock<SpecialValuesMap>)
└── tests.rs     # Unit tests
```

## Implementation Checklist (Per Function)

- [ ] Create module directory: `src/core/functions/FUNCNAME/`
- [ ] Implement `mod.rs` with:
  - [ ] `pub fn FUNCNAME(arg: &Expression) -> Result<Expression, MathError>`
  - [ ] Special value lookups from data.rs
  - [ ] Computed special values (general patterns)
  - [ ] Mathematical identities (symmetry, periodicity)
  - [ ] Numerical evaluation
  - [ ] Symbolic return for unevaluated forms
- [ ] Create `data.rs` with:
  - [ ] `pub static FUNCNAME_SPECIAL_VALUES: LazyLock<SpecialValuesMap>`
  - [ ] Exact values with LaTeX explanations
  - [ ] Undefined/error cases (poles, domain violations)
- [ ] Migrate/create `tests.rs`:
  - [ ] Test all special values
  - [ ] Test domain boundaries
  - [ ] Test mathematical properties (symmetry, periodicity, identities)
  - [ ] Test numerical evaluation
  - [ ] Test error cases
- [ ] Register in UniversalFunctionRegistry (if needed)
- [ ] Test: `cargo test -p mathhook-core functions::FUNCNAME`
- [ ] Verify no regressions: `cargo test`

## Pattern Templates

### data.rs Template (Elementary Functions)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
//! Special values for FUNCNAME function

use crate::core::Expression;
use std::sync::LazyLock;
use std::collections::HashMap;

pub type SpecialValuesMap = HashMap<Expression, SpecialValueResult>;

pub enum SpecialValueResult {
    Exact {
        output: Expression,
        latex: String,
    },
    Error {
        error: MathError,
        latex: String,
    },
}

pub static FUNCNAME_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Exact zero
    map.insert(
        Expression::integer(0),
        SpecialValueResult::Exact {
            output: Expression::integer(0),
            latex: "\\FUNCNAME(0) = 0".to_string(),
        },
    );

    // Common special values
    // ...

    // Undefined cases (poles)
    map.insert(
        Expression::div(Expression::pi(), Expression::integer(2)),
        SpecialValueResult::Error {
            error: MathError::Undefined {
                expression: Expression::function("FUNCNAME", vec![
                    Expression::div(Expression::pi(), Expression::integer(2))
                ]),
            },
            latex: "\\FUNCNAME(\\frac{\\pi}{2}) = \\text{undefined}".to_string(),
        },
    );

    map
});
```

### mod.rs Template (Elementary Functions)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
//! FUNCNAME function implementation
//!
//! Provides exact symbolic evaluation, special values, and numerical computation.

use crate::core::Expression;
use crate::error::MathError;

mod data;
use data::{FUNCNAME_SPECIAL_VALUES, SpecialValueResult};

#[cfg(test)]
mod tests;

/// Evaluate FUNCNAME function
///
/// # Arguments
///
/// * `arg` - The input expression
///
/// # Returns
///
/// * `Ok(Expression)` - The evaluated result
/// * `Err(MathError)` - Domain error or undefined
///
/// # Examples
///
/// ```rust
/// use mathhook::functions::elementary::FUNCNAME::FUNCNAME;
/// use mathhook::{expr, symbol};
///
/// let x = symbol!(x);
/// let result = FUNCNAME(&expr!(0)).unwrap();
/// assert_eq!(result, expr!(0));
/// ```
pub fn FUNCNAME(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values (exact or error)
    if let Some(result) = FUNCNAME_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }

    // 2. Computed special values (general patterns)
    // Example: For trig functions, handle multiples of π
    // if let Some(pi_mult) = arg.as_pi_multiple() {
    //     return Ok(eval_FUNCNAME_at_pi_multiple(&pi_mult));
    // }

    // 3. Mathematical identities
    // Example: sin(-x) = -sin(x) (odd function)
    // if let Some(neg_arg) = arg.as_negation() {
    //     return FUNCNAME(&neg_arg).map(|result| Expression::neg(result));
    // }

    // 4. Numerical evaluation
    if let Some(val) = arg.try_to_f64() {
        return Ok(Expression::float(val.FUNCNAME()));  // Use appropriate std method
    }

    // 5. Unevaluated (symbolic)
    Ok(Expression::function("FUNCNAME", vec![arg.clone()]))
}

/// Dispatcher for registry integration
pub(crate) fn FUNCNAME_dispatch(args: &[Expression]) -> Result<Expression, MathError> {
    if args.len() != 1 {
        return Err(MathError::InvalidArgumentCount {
            expected: 1,
            got: args.len(),
        });
    }
    FUNCNAME(&args[0])
}
```

### tests.rs Template

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
//! Tests for FUNCNAME function

use super::*;
use crate::{expr, symbol};

#[test]
fn test_FUNCNAME_special_values() {
    // Test exact zero
    assert_eq!(FUNCNAME(&expr!(0)).unwrap(), expr!(0));

    // Test other special values
    // ...
}

#[test]
fn test_FUNCNAME_domain_errors() {
    // Test undefined cases (poles)
    let result = FUNCNAME(&expr!(pi / 2));
    assert!(result.is_err());
    if let Err(MathError::Undefined { .. }) = result {
        // Expected
    } else {
        panic!("Expected Undefined error");
    }
}

#[test]
fn test_FUNCNAME_identities() {
    let x = symbol!(x);

    // Test symmetry (odd/even function)
    // Example: sin(-x) = -sin(x)
    // let neg_x = expr!(-x);
    // assert_eq!(FUNCNAME(&neg_x), expr!(- FUNCNAME(x)));

    // Test periodicity
    // Example: sin(x + 2π) = sin(x)
}

#[test]
fn test_FUNCNAME_numerical() {
    // Test numerical evaluation
    use std::f64::consts::PI;

    let result = FUNCNAME(&Expression::float(0.0)).unwrap();
    assert!((result.try_to_f64().unwrap() - 0.0).abs() < 1e-10);

    // More numerical tests...
}

#[test]
fn test_FUNCNAME_symbolic() {
    let x = symbol!(x);

    // Unevaluated form
    let result = FUNCNAME(&Expression::symbol(x)).unwrap();
    assert_eq!(result, Expression::function("FUNCNAME", vec![Expression::symbol(x)]));
}
```

## Special Function Patterns (Gamma, Zeta, Bessel)

### data.rs for Special Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
pub static GAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();

    // Integer values: Γ(n) = (n-1)!
    map.insert(expr!(1), special_value!(1, "\\Gamma(1) = 1"));
    map.insert(expr!(2), special_value!(1, "\\Gamma(2) = 1"));
    map.insert(expr!(3), special_value!(2, "\\Gamma(3) = 2"));
    map.insert(expr!(4), special_value!(6, "\\Gamma(4) = 6"));

    // Half-integer values: Γ(1/2) = √π
    map.insert(expr!(1/2), special_value!(sqrt(pi), "\\Gamma(\\frac{1}{2}) = \\sqrt{\\pi}"));

    // Poles (undefined at non-positive integers)
    map.insert_undefined(expr!(0), "\\Gamma(0) = \\text{undefined}");
    map.insert_undefined(expr!(-1), "\\Gamma(-1) = \\text{undefined}");

    map
});
```

### Computed Special Values Pattern

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For gamma: Handle general factorial cases
if let Some(n) = arg.as_positive_integer() {
    if n > 0 && n <= 20 {  // Precompute up to reasonable limit
        return Ok(Expression::integer(factorial(n - 1)));
    }
}

// For zeta: Handle general even integer cases
if let Some(n) = arg.as_even_positive_integer() {
    // Use Bernoulli numbers formula
    return Ok(compute_zeta_even_integer(n));
}
```

## Number Theory Functions (NO HashMap - Algorithmic)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// gcd, lcm, factor do NOT use HashMap
// They are purely algorithmic

pub fn gcd(a: &Expression, b: &Expression) -> Result<Expression, MathError> {
    // Euclidean algorithm
    if let (Some(a_int), Some(b_int)) = (a.as_integer(), b.as_integer()) {
        return Ok(Expression::integer(gcd_algorithm(a_int, b_int)));
    }

    // Symbolic GCD
    Ok(Expression::function("gcd", vec![a.clone(), b.clone()]))
}
```

## Registry Integration Pattern

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// functions/registry/elementary.rs

use crate::functions::elementary::sin::{sin_dispatch, SIN_SPECIAL_VALUES};

pub fn get_elementary_properties() -> Vec<(String, FunctionProperties)> {
    vec![
        (
            "sin".to_string(),
            FunctionProperties {
                name: "sin",
                // Derived from shared data (CANNOT drift!)
                special_values: SIN_SPECIAL_VALUES
                    .iter()
                    .map(|(input, result)| match result {
                        SpecialValueResult::Exact { output, latex } => SpecialValue {
                            input: input.to_string(),
                            output: output.clone(),
                            latex_explanation: latex.clone(),
                        },
                        SpecialValueResult::Error { .. } => SpecialValue {
                            input: input.to_string(),
                            output: Expression::undefined(),
                            latex_explanation: "undefined".to_string(),
                        },
                    })
                    .collect(),
                dispatch: sin_dispatch,
                domain: "all reals".to_string(),
                range: "[-1, 1]".to_string(),
                period: Some(Expression::mul(vec![Expression::integer(2), Expression::pi()])),
            },
        ),
        // More functions...
    ]
}
```

## Critical Patterns

### 1. Single Source of Truth
- Special values defined ONCE in data.rs
- Registry derives from data.rs (no duplication)
- Implementation uses data.rs directly (no hardcoded values)

### 2. Error Handling
- Domain violations return `Err(MathError::DomainError { ... })`
- Undefined cases (poles) use `SpecialValueResult::Error`
- Symbolic unevaluated forms for generic expressions

### 3. Mathematical Correctness
- Exact symbolic values (use rationals, not floats)
- Handle symmetry (odd/even functions)
- Handle periodicity (trig functions)
- Handle domain restrictions (log, sqrt, etc.)

### 4. Testing Requirements
- Test ALL special values from data.rs
- Test domain boundaries and error cases
- Test mathematical properties (identities, symmetry)
- Test numerical evaluation accuracy
- Test symbolic unevaluated forms

## Migration Workflow

### For Refactored Functions (7 functions)

1. **Extract current implementation** from special.rs
2. **Create module directory** `src/core/functions/FUNCNAME/`
3. **Create data.rs** with special values from hardcoded matches
4. **Migrate mod.rs** to use data.rs lookups
5. **Migrate tests** from special.rs to tests.rs
6. **Run tests** to verify no regressions
7. **Update registry** (if needed)

### For New Implementations (21 functions)

1. **Create module directory** `src/core/functions/FUNCNAME/`
2. **Create data.rs** with special values
3. **Implement mod.rs** following pattern
4. **Create tests.rs** with coverage
5. **Register in registry** (elementary.rs, polynomials.rs, etc.)
6. **Run tests** and verify correctness

## Verification Commands

```bash
# Test individual function
cargo test -p mathhook-core functions::FUNCNAME

# Test all functions
cargo test -p mathhook-core functions

# Full test suite (no regressions)
cargo test

# Doctests
cargo test --doc

# Check Expression size (must remain 32 bytes)
cargo test expression_size
```

## Success Criteria

- ✅ All functions pass individual tests
- ✅ All functions pass integration tests
- ✅ No test regressions (full suite passes)
- ✅ Expression size remains 32 bytes
- ✅ Documentation complete with examples
- ✅ No hardcoded special values in implementation (only in data.rs)
- ✅ Registry correctly derives from data.rs
