# Groebner Bases

**Last Updated:** 2025-11-29T12:30:00

Groebner bases are a fundamental tool in computational algebraic geometry for working with polynomial ideals.

## Overview

A Groebner basis is a special generating set for a polynomial ideal that has many useful computational properties:

- **Ideal Membership Testing**: Determine if a polynomial belongs to an ideal
- **Polynomial System Solving**: Find common solutions to systems of polynomial equations
- **Variable Elimination**: Eliminate variables from polynomial systems
- **Geometric Theorem Proving**: Prove geometric theorems algebraically

## Basic Usage

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::groebner::{GroebnerBasis, MonomialOrder};

let x = symbol!(x);
let y = symbol!(y);

// Define polynomials: f1 = x - y, f2 = y^2 - 1
let f1 = expr!(x - y);
let f2 = expr!(y ^ 2 - 1);

// Create Groebner basis
let mut gb = GroebnerBasis::new(
    vec![f1, f2],
    vec![x.clone(), y.clone()],
    MonomialOrder::Lex
);

// Compute the basis
gb.compute();

println!("Basis has {} polynomials", gb.basis.len());
```

## Monomial Orders

The choice of monomial order affects the structure of the Groebner basis:

| Order | Description | Use Case |
|-------|-------------|----------|
| `Lex` | Lexicographic | Variable elimination |
| `Grlex` | Graded lexicographic | Balanced computation |
| `Grevlex` | Graded reverse lexicographic | Efficient computation |

```rust
use mathhook_core::core::polynomial::groebner::MonomialOrder;

let lex = MonomialOrder::Lex;       // x > y > z
let grlex = MonomialOrder::Grlex;   // Total degree first, then lex
let grevlex = MonomialOrder::Grevlex; // Total degree first, then reverse lex
```

### Lexicographic Order (Lex)

Compares exponents from left to right:
- `x^2y > xy^2` (2 > 1 in first position)
- `xy^3 > xz^5` (y > z in second position)

Best for: Variable elimination, solving systems

### Graded Lexicographic (Grlex)

Compares total degree first, then lexicographic:
- `xy^2 > x^2` (degree 3 > 2)
- `x^2 > xy` (same degree, x^2 > xy lexicographically)

Best for: Balanced trade-off between structure and efficiency

### Graded Reverse Lexicographic (Grevlex)

Compares total degree first, then reverse lexicographic from right:
- `xy^2 > x^2y` (same degree, compare from right)

Best for: Efficient computation (often produces smaller bases)

## Sparse Polynomial Representation

Internally, the module uses sparse polynomial representation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::groebner::{Monomial, SparsePolynomial};

// Create a monomial x^2 * y (exponents [2, 1])
let mono = Monomial::new(vec![2, 1]);
assert_eq!(mono.degree(), 3);

// Sparse polynomial operations
use mathhook_core::core::polynomial::groebner::expression_to_sparse_polynomial;

let x = symbol!(x);
let y = symbol!(y);
let poly = expr!(x ^ 2 + y);

let vars = vec![x, y];
let sparse = expression_to_sparse_polynomial(&poly, &vars);
```

## Buchberger's Algorithm

The classic algorithm for computing Groebner bases:

```rust
use mathhook_core::core::polynomial::groebner::{
    buchberger_algorithm,
    s_polynomial,
    poly_reduce
};

// The s_polynomial function computes the S-polynomial of two polynomials
// This is the key operation in Buchberger's algorithm

// Efficient version with pair selection strategies
use mathhook_core::core::polynomial::groebner::efficient_buchberger_algorithm;
```

### Algorithm Steps

1. **Initialize**: Start with the input polynomials
2. **S-pairs**: For each pair of polynomials, compute the S-polynomial
3. **Reduce**: Reduce each S-polynomial by the current basis
4. **Add**: If reduction is non-zero, add to basis
5. **Repeat**: Continue until no new polynomials are added

## Polynomial Reduction

Reduce a polynomial by a set of polynomials:

```rust
use mathhook_core::core::polynomial::groebner::{
    poly_reduce,
    poly_reduce_completely
};

// Single-step reduction
let reduced = poly_reduce(&poly, &basis, &order);

// Complete reduction (until no further reduction possible)
let fully_reduced = poly_reduce_completely(&poly, &basis, &order);
```

## Conversion Functions

Convert between Expression and sparse polynomial representation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::core::polynomial::groebner::{
    expression_to_sparse_polynomial,
    sparse_polynomial_to_expression
};

let x = symbol!(x);
let y = symbol!(y);
let vars = vec![x.clone(), y.clone()];

// Expression to sparse
let expr = expr!(x ^ 2 + y);
let sparse = expression_to_sparse_polynomial(&expr, &vars).unwrap();

// Sparse back to expression
let back = sparse_polynomial_to_expression(&sparse, &vars);
```

## Applications

### Solving Polynomial Systems

Compute a Groebner basis with Lex order to get elimination ideals:

```rust
// System: x + y = 2, xy = 1
// Groebner basis with Lex: {y^2 - 2y + 1, x + y - 2}
// The first polynomial only involves y, giving solutions
```

### Ideal Membership

Test if a polynomial belongs to an ideal by checking if its remainder under the Groebner basis is zero.

### Elimination

With Lex order x > y > z, the Groebner basis contains polynomials in:
- Only z (elimination of x and y)
- y and z (elimination of x)
- x, y, and z

## Performance Considerations

- **Monomial Order**: Grevlex is typically fastest to compute
- **Variable Order**: Ordering variables by expected degree can improve performance
- **Sparsity**: The algorithm is more efficient for sparse polynomials
- **Degree Bounds**: Groebner bases can be exponentially large in worst case

## Mathematical Background

### Definition

A set G = {g1, ..., gm} is a Groebner basis for ideal I if:

```
<LT(g1), ..., LT(gm)> = <LT(I)>
```

where LT denotes the leading term and <> denotes the ideal generated.

### S-Polynomial

The S-polynomial of f and g is:

```
S(f,g) = (lcm(LT(f), LT(g)) / LT(f)) * f - (lcm(LT(f), LT(g)) / LT(g)) * g
```

### Buchberger's Criterion

G is a Groebner basis if and only if for all pairs gi, gj:

```
S(gi, gj) reduces to 0 modulo G
```

## See Also

- [Polynomial Overview](./overview.md)
- [GCD Algorithms](./gcd.md) - Related polynomial algorithms
- [Division](./division.md) - Polynomial division and reduction
