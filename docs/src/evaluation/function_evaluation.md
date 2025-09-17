# Function Evaluation

MathHook provides a unified, intelligent function evaluation system that handles both symbolic and numerical computation. The system uses the **Universal Function Registry** architecture to dispatch function calls to specialized implementations while maintaining mathematical correctness.

## Overview

Function evaluation in MathHook supports:

- **Elementary functions**: sin, cos, tan, exp, log, sqrt, abs, and their inverses
- **Hyperbolic functions**: sinh, cosh, tanh, and their inverses
- **Special functions**: gamma, zeta, bessel functions
- **Number theory functions**: factorial, binomial coefficients
- **Symbolic evaluation**: Returns exact symbolic results when possible
- **Numerical evaluation**: High-performance numerical approximations
- **Special value recognition**: Automatically simplifies known exact values

## Basic Usage

### Evaluating Elementary Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Trigonometric functions
let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let tan_x = expr!(tan(x));

// Exponential and logarithmic
let exp_x = expr!(exp(x));
let log_x = expr!(log(x));

// Square root and absolute value
let sqrt_x = expr!(sqrt(x));
let abs_x = expr!(abs(x));
```

### Special Value Evaluation

MathHook recognizes and simplifies special mathematical values automatically:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Trigonometric special values
let sin_0 = expr!(sin(0));
assert_eq!(sin_0.simplify(), expr!(0));

let cos_0 = expr!(cos(0));
assert_eq!(cos_0.simplify(), expr!(1));

// Exponential and logarithmic
let exp_0 = expr!(exp(0));
assert_eq!(exp_0.simplify(), expr!(1));

let log_1 = expr!(log(1));
assert_eq!(log_1.simplify(), expr!(0));

// Factorial
let fact_5 = expr!(factorial(5));
assert_eq!(fact_5.simplify(), expr!(120));
```

## Evaluation Architecture

### Function Intelligence System

Every function in MathHook has associated **intelligence properties** that define:

1. **Domain and Range**: Where the function is defined and what values it can produce
2. **Special Values**: Known exact values (e.g., sin(0) = 0, gamma(1) = 1)
3. **Evaluation Strategy**: How to compute the function symbolically and numerically
4. **Mathematical Properties**: Symmetry, periodicity, derivative rules, etc.

### Evaluation Flow

```
User Expression
      ↓
Function Name + Arguments
      ↓
Universal Registry Lookup
      ↓
Function Properties Dispatch
      ↓
┌─────────────────┬──────────────────┐
│ Special Value?  │ Symbolic Input?  │ Numerical Input?
│ → Exact Result  │ → Keep Symbolic  │ → Numerical Eval
└─────────────────┴──────────────────┘
```

## Special Values Table

### Trigonometric Functions

| Function | Input | Output |
|----------|-------|--------|
| sin(x)   | 0     | 0      |
| sin(x)   | π/2   | 1      |
| cos(x)   | 0     | 1      |
| cos(x)   | π     | -1     |
| tan(x)   | 0     | 0      |

### Exponential and Logarithmic

| Function | Input | Output |
|----------|-------|--------|
| exp(x)   | 0     | 1      |
| log(x)   | 1     | 0      |
| log(x)   | e     | 1      |

### Special Functions

| Function | Input | Output |
|----------|-------|--------|
| gamma(x) | 1     | 1      |
| gamma(x) | 2     | 1      |
| gamma(x) | n     | (n-1)! |

## Function Composition

Functions can be nested and composed:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// sin(cos(x))
let nested = expr!(sin(cos(x)));

// exp(log(x)) simplifies to x
let exp_log = expr!(exp(log(x)));
let simplified = exp_log.simplify();
// Result: x (identity simplification)
```

## Composite Expression Evaluation

MathHook intelligently handles composite expressions that mix symbolic variables with evaluable constants, preserving symbolic parts while evaluating numeric parts.

### Mixed Symbolic and Numeric Evaluation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Composite expression: sin(x^2 + 1) * cos(y) - sqrt(4)
// sqrt(4) will evaluate to 2, but sin and cos remain symbolic
let composite = expr!((sin((x^2) + 1) * cos(y)) - sqrt(4));

let result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2
// Note: sqrt(4) evaluated to 2, symbolic parts preserved
```

### Evaluation Propagation in Complex Expressions

The evaluation system recursively evaluates subexpressions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Expression with multiple evaluable constants
let complex_expr = expr!(2 * sin(0) * exp(x) * sqrt(16));

let result = complex_expr.simplify();
// Result: 0 (because sin(0) = 0, and anything times 0 is 0)
```

### Special Value Recognition in Nested Expressions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Nested expression: sin(π/2) * cos(0) + sqrt(9)
let nested_special = expr!((sin(pi() / 2) * cos(0)) + sqrt(9));

let result = nested_special.simplify();
// Result: 4
// Breakdown: sin(π/2) = 1, cos(0) = 1, sqrt(9) = 3
//            1 * 1 + 3 = 4
```

### Partial Evaluation with Substitution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Expression: sin(x) + cos(y) + sqrt(4)
let expr_val = expr!(sin(x) + cos(y) + sqrt(4));

// First simplify to evaluate constants
let partially_evaluated = expr_val.simplify();
// Result: sin(x) + cos(y) + 2

// Then substitute specific values for variables
let substituted = partially_evaluated.substitute(
    &x,
    &expr!(0)
);
// Result: cos(y) + 2 (since sin(0) = 0)
```

### Integration with Other Operations

Composite function evaluation works seamlessly with differentiation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Differentiate sin(x^2) * sqrt(4)
let expr_val = expr!(sin(x^2) * sqrt(4));

// First simplify to evaluate sqrt(4) = 2
let simplified = expr_val.simplify();
// Result: 2 * sin(x^2)

// Then differentiate
let derivative = simplified.derivative(&x, 1);
// Result: 4*x*cos(x^2) (using chain rule)
```

## Performance Characteristics

The function evaluation system is designed for high performance:

- **Registry lookup**: O(1) constant time using hash maps
- **Special value detection**: <50ns for known values
- **Numerical evaluation**: <100ns for elementary functions
- **Total dispatch overhead**: <10ns
- **Bulk evaluation**: SIMD-optimized for arrays of values

### Bulk Evaluation

For numerical computation over many points, use bulk evaluation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::functions::FunctionEvaluator;

let evaluator = FunctionEvaluator::new();
let points = vec![0.0, 0.5, 1.0, 1.5, 2.0];

// Evaluates sin(x) at all points efficiently
if let Some(results) = evaluator.evaluate_bulk_f64("sin", &points) {
    println!("Results: {:?}", results);
}
```

**Requirements for bulk evaluation**:
- Minimum 4 data points
- Elementary functions only (sin, cos, exp, log, etc.)
- Numerical inputs (f64)

## Mathematical Correctness Guarantees

MathHook's function evaluation system provides:

1. **Exact symbolic computation**: Special values return exact results (not floating-point approximations)
2. **Domain checking**: Functions respect their mathematical domains (e.g., log requires positive inputs)
3. **SymPy validation**: All implementations validated against SymPy reference
4. **Numerical stability**: Algorithms chosen for numerical accuracy

## Supported Functions

### Elementary Functions

**Trigonometric**:
- `sin`, `cos`, `tan`
- `arcsin`, `arccos`, `arctan`
- `sec`, `csc`, `cot`

**Hyperbolic**:
- `sinh`, `cosh`, `tanh`
- `arcsinh`, `arccosh`, `arctanh`

**Exponential and Logarithmic**:
- `exp`: Natural exponential (e^x)
- `log`: Natural logarithm (ln)
- `log` with base: log(x, b) for logarithm base b

**Power and Root**:
- `sqrt`: Square root
- `pow`: General exponentiation (also via ^ operator)

**Other Elementary**:
- `abs`: Absolute value

### Special Functions

**Number Theory**:
- `factorial`: n!
- `binomial`: Binomial coefficient C(n, k)

**Special Functions**:
- `gamma`: Gamma function Γ(x)
- `zeta`: Riemann zeta function ζ(x)
- `bessel_j`: Bessel function of the first kind J_n(x)

## Limitations and Edge Cases

### Domain Restrictions

Functions respect mathematical domain restrictions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// log requires positive arguments
let log_negative = expr!(log(-1));
// Stays symbolic (complex result in general)

// sqrt of negative (stays symbolic in real domain)
let sqrt_negative = expr!(sqrt(-1));
// Result: i (imaginary unit) in complex mode
```

### Numerical Precision

For numerical evaluation:
- Elementary functions: ~15-16 digits precision (IEEE 754 double)
- Special functions: Varies by function and argument range
- Large arguments may have reduced precision

### Unsupported Cases

Currently unsupported or limited:
- Multi-valued functions return principal value only
- Some special function evaluations remain symbolic
- Complex domain evaluation is limited for some functions

## Examples

### Trigonometric Identities

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// sin^2(x) + cos^2(x) = 1
let identity = expr!((sin(x)^2) + (cos(x)^2));

let simplified = identity.simplify();
// Result: 1
```

### Exponential Properties

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// exp(x) * exp(y) = exp(x + y)
let product = expr!(exp(x) * exp(y));

let simplified = product.simplify();
// Result: exp(x + y)
```

### Factorial and Gamma

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// gamma(n) = (n-1)! for positive integers
let gamma_5 = expr!(gamma(5));
let simplified = gamma_5.simplify();
// Result: 24 (which is 4!)
```

## See Also

- [Function Intelligence System](../architecture/function-intelligence.md) - Architecture details
- [Special Functions](../advanced/special-functions.md) - Advanced special function usage
- [Performance Benchmarking](../performance/benchmarking.md) - Detailed performance analysis
