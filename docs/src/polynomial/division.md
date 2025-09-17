# Polynomial Division and Factorization

**Last Updated:** 2025-11-29T12:30:00

This chapter covers polynomial division algorithms and factorization capabilities in MathHook.

## Polynomial Division

### Long Division

The standard polynomial long division algorithm computes quotient and remainder:

```rust
use mathhook_core::core::polynomial::algorithms::polynomial_long_division;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Divide (x^2 - 1) by (x - 1)
let dividend = expr!((x ^ 2) - 1);
let divisor = expr!(x - 1);

let (quotient, remainder) = polynomial_long_division(&dividend, &divisor, &x).unwrap();

// quotient = x + 1
// remainder = 0
// Verify: dividend = divisor * quotient + remainder
```

### Exact Division

When you expect the division to be exact (zero remainder):

```rust
use mathhook_core::core::polynomial::algorithms::exact_division;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// x^2 / x = x (exact)
let dividend = expr!(x ^ 2);
let divisor = expr!(x);

match exact_division(&dividend, &divisor, &x) {
    Ok(quotient) => println!("Exact quotient: {}", quotient),
    Err(e) => println!("Division not exact: {:?}", e),
}
```

### Using the Trait API

```rust
use mathhook_core::core::polynomial::PolynomialArithmetic;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

let f = expr!((x ^ 3) - 1);
let g = expr!(x - 1);

// Returns (quotient, remainder)
let (q, r) = f.poly_div(&g, &x).unwrap();
// q = x^2 + x + 1
// r = 0
```

## Factorization

### Common Factor Extraction

Extract the greatest common factor from all terms:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::algorithms::extract_common_factor;

let x = symbol!(x);

// 2x + 4 has common factor 2
let poly = expr!(2 * x + 4);

let common = extract_common_factor(&poly);
// common = 2
```

### Numeric Coefficient Factoring

Separate the numeric coefficient from the polynomial:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::algorithms::factor_numeric;

let x = symbol!(x);

// 6x
let poly = expr!(6 * x);

let (coeff, remaining) = factor_numeric(&poly);
// coeff = 6
// remaining = x
```

### Square-Free Factorization

Decompose a polynomial into square-free factors:

```rust
use mathhook_core::core::polynomial::algorithms::square_free_factorization;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let poly = expr!(x ^ 2);

let factors = square_free_factorization(&poly, &x).unwrap();
// Returns Vec<(factor, multiplicity)>
```

## Resultant and Discriminant

### Resultant

The resultant of two polynomials is zero if and only if they share a common root:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::algorithms::polynomial_resultant;

let x = symbol!(x);

// p1 = x - 1
let p1 = expr!(x - 1);
// p2 = x - 2
let p2 = expr!(x - 2);

let res = polynomial_resultant(&p1, &p2, &x).unwrap();
// Resultant is non-zero (distinct roots)
```

### Discriminant

The discriminant indicates whether a polynomial has repeated roots:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::algorithms::polynomial_discriminant;

let x = symbol!(x);

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
let poly = expr!(x ^ 2 - 2 * x + 1);

let disc = polynomial_discriminant(&poly, &x).unwrap();
// Discriminant = 0 (repeated root)
```

### Coprimality Test

Check if two polynomials are coprime (GCD is constant):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::algorithms::are_coprime;

let x = symbol!(x);

let p1 = expr!(x);      // x
let p2 = expr!(x + 1);  // x + 1

if are_coprime(&p1, &p2, &x).unwrap() {
    println!("Polynomials are coprime");
}
```

## Mathematical Background

### Polynomial Long Division

For polynomials `f(x)` and `g(x)` with `deg(g) <= deg(f)`:

```
f(x) = g(x) * q(x) + r(x)
```

where:
- `q(x)` is the quotient
- `r(x)` is the remainder with `deg(r) < deg(g)`

### Resultant Definition

The resultant `Res(f, g)` of two polynomials is the determinant of their Sylvester matrix. Key properties:

1. `Res(f, g) = 0` if and only if `f` and `g` share a common root
2. `Res(f, g) = a_n^m * b_m^n * prod((alpha_i - beta_j))` where `alpha_i`, `beta_j` are roots

### Discriminant Definition

For polynomial `f(x)` of degree `n` with leading coefficient `a_n`:

```
disc(f) = (-1)^(n(n-1)/2) * Res(f, f') / a_n
```

Properties:
- `disc(f) = 0` if and only if `f` has a repeated root
- For quadratic `ax^2 + bx + c`: `disc = b^2 - 4ac`

## Error Handling

```rust
use mathhook_core::core::polynomial::PolynomialError;

// Common errors
match result {
    Err(PolynomialError::DivisionByZero) => {
        println!("Cannot divide by zero polynomial");
    }
    Err(PolynomialError::DivisionNotExact { dividend, divisor }) => {
        println!("{} does not exactly divide {}", divisor, dividend);
    }
    Err(PolynomialError::NotPolynomial { expr }) => {
        println!("{} is not a polynomial", expr);
    }
    _ => {}
}
```

## See Also

- [GCD Algorithms](./gcd.md) - Greatest common divisor computation
- [Polynomial Overview](./overview.md) - Module introduction
- [Groebner Bases](./groebner.md) - Polynomial ideals
