# Special Polynomial Families

**Last Updated:** 2025-11-29T12:30:00

MathHook provides access to classical orthogonal polynomial families with both symbolic expansion and numerical evaluation.

## Supported Families

| Family | Symbol | Interval | Weight Function |
|--------|--------|----------|-----------------|
| Legendre | P_n(x) | [-1, 1] | w(x) = 1 |
| Chebyshev (1st) | T_n(x) | [-1, 1] | w(x) = 1/sqrt(1-x^2) |
| Chebyshev (2nd) | U_n(x) | [-1, 1] | w(x) = sqrt(1-x^2) |
| Hermite | H_n(x) | (-inf, inf) | w(x) = exp(-x^2) |
| Laguerre | L_n(x) | [0, inf) | w(x) = exp(-x) |

## The OrthogonalPolynomial Trait

All families implement a unified interface:

```rust
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;

pub trait OrthogonalPolynomial {
    /// Generate the n-th polynomial as an Expression
    fn polynomial(n: usize, var: &Symbol) -> Expression;

    /// Evaluate numerically at a specific point
    fn evaluate(n: usize, x: f64) -> f64;

    /// Get recurrence relation coefficients
    fn recurrence_coefficients(n: usize) -> (f64, f64, f64);
}
```

## Legendre Polynomials

Solutions to Legendre's differential equation: `(1-x^2)P_n'' - 2xP_n' + n(n+1)P_n = 0`

```rust
use mathhook_core::core::polynomial::special_families::Legendre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic expansion
let p0 = Legendre::polynomial(0, &x);  // 1
let p1 = Legendre::polynomial(1, &x);  // x
let p2 = Legendre::polynomial(2, &x);  // (3x^2 - 1)/2

// Numerical evaluation
let val = Legendre::evaluate(2, 0.5);  // P_2(0.5) = -0.125

// Recurrence: P_{n+1} = ((2n+1)x*P_n - n*P_{n-1}) / (n+1)
let (a, b, c) = Legendre::recurrence_coefficients(2);
```

### Properties

- P_n(1) = 1 for all n
- P_n(-1) = (-1)^n
- P_n has n distinct real roots in (-1, 1)

## Chebyshev Polynomials

### First Kind (T_n)

Defined by: `T_n(cos(theta)) = cos(n*theta)`

```rust
use mathhook_core::core::polynomial::special_families::ChebyshevT;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let t0 = ChebyshevT::polynomial(0, &x);  // 1
let t1 = ChebyshevT::polynomial(1, &x);  // x
let t2 = ChebyshevT::polynomial(2, &x);  // 2x^2 - 1

// Numerical
let val = ChebyshevT::evaluate(2, 0.5);  // T_2(0.5) = -0.5

// Recurrence: T_{n+1} = 2x*T_n - T_{n-1}
```

### Second Kind (U_n)

Defined by: `U_n(cos(theta)) = sin((n+1)*theta) / sin(theta)`

```rust
use mathhook_core::core::polynomial::special_families::ChebyshevU;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

let u0 = ChebyshevU::polynomial(0, &x);  // 1
let u1 = ChebyshevU::polynomial(1, &x);  // 2x

// Numerical
let val = ChebyshevU::evaluate(1, 0.5);  // U_1(0.5) = 1
```

### Properties

- |T_n(x)| <= 1 for x in [-1, 1]
- T_n has optimal minimax properties for polynomial approximation
- Used in spectral methods and numerical analysis

## Hermite Polynomials

Solutions to Hermite's equation: `H_n'' - 2xH_n' + 2nH_n = 0`

Uses physicist's convention: `H_n(x) = (-1)^n * exp(x^2) * d^n/dx^n(exp(-x^2))`

```rust
use mathhook_core::core::polynomial::special_families::Hermite;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let h0 = Hermite::polynomial(0, &x);  // 1
let h1 = Hermite::polynomial(1, &x);  // 2x
let h2 = Hermite::polynomial(2, &x);  // 4x^2 - 2

// Numerical
let val = Hermite::evaluate(1, 0.5);  // H_1(0.5) = 1

// Recurrence: H_{n+1} = 2x*H_n - 2n*H_{n-1}
```

### Properties

- H_n(-x) = (-1)^n * H_n(x) (even/odd symmetry)
- Related to quantum harmonic oscillator wave functions
- H_n has n distinct real roots

## Laguerre Polynomials

Solutions to Laguerre's equation: `xL_n'' + (1-x)L_n' + nL_n = 0`

```rust
use mathhook_core::core::polynomial::special_families::Laguerre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let l0 = Laguerre::polynomial(0, &x);  // 1
let l1 = Laguerre::polynomial(1, &x);  // 1 - x
let l2 = Laguerre::polynomial(2, &x);  // (x^2 - 4x + 2)/2

// Numerical
let val = Laguerre::evaluate(1, 0.5);  // L_1(0.5) = 0.5

// Recurrence: L_{n+1} = ((2n+1-x)*L_n - n*L_{n-1}) / (n+1)
```

### Properties

- L_n(0) = 1 for all n
- Related to hydrogen atom wave functions
- L_n has n distinct positive real roots

## Variable Substitution

All polynomial families support variable substitution:

```rust
use mathhook_core::core::polynomial::special_families::Legendre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

// Use variable t instead of x
let t = symbol!(t);
let p2_t = Legendre::polynomial(2, &t);
// Result uses t: (3t^2 - 1)/2
```

## Direct Function Access

For lower-level access, use the evaluation and symbolic modules directly:

```rust
use mathhook_core::core::polynomial::special_families::evaluation::{
    evaluate_legendre_numerical,
    evaluate_chebyshev_first_numerical,
    evaluate_hermite_numerical,
    evaluate_laguerre_numerical,
};

use mathhook_core::core::polynomial::special_families::symbolic::{
    expand_legendre_symbolic,
    expand_chebyshev_first_symbolic,
    expand_hermite_symbolic,
    expand_laguerre_symbolic,
};

// Direct numerical evaluation
let vals = evaluate_legendre_numerical(&[2.0, 0.5]);

// Direct symbolic expansion
let expr = expand_legendre_symbolic(2);
```

## Recurrence Relations

All orthogonal polynomials satisfy a three-term recurrence:

```
P_{n+1}(x) = (a_n * x + b_n) * P_n(x) - c_n * P_{n-1}(x)
```

Get coefficients via `recurrence_coefficients(n)`:

| Family | a_n | b_n | c_n |
|--------|-----|-----|-----|
| Legendre | (2n+1)/(n+1) | 0 | n/(n+1) |
| Chebyshev T | 2 | 0 | 1 |
| Chebyshev U | 2 | 0 | 1 |
| Hermite | 2 | 0 | 2n |
| Laguerre | -1/(n+1) | (2n+1)/(n+1) | n/(n+1) |

## Applications

- **Numerical Integration**: Gaussian quadrature rules
- **Spectral Methods**: Approximation and PDE solving
- **Physics**: Quantum mechanics, electrostatics
- **Signal Processing**: Filter design, approximation

## See Also

- [Polynomial Overview](./overview.md)
- [Function Evaluation](../evaluation/function_evaluation.md)
