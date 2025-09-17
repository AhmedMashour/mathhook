# Finite Field Arithmetic

**Last Updated:** 2025-11-29T12:30:00

The finite field module provides arithmetic in Z_p (integers modulo a prime p), essential for modular GCD algorithms.

## Overview

Finite field arithmetic is the backbone of Zippel's modular GCD algorithm:

1. Reduce polynomials to Z_p[x]
2. Compute GCD in Z_p[x] (efficient)
3. Reconstruct integer coefficients via CRT

## Field Elements (Z_p)

```rust
use mathhook_core::core::polynomial::finite_field::FieldElement;

// Create elements in Z_7
let a = FieldElement::new(3, 7);  // 3 mod 7
let b = FieldElement::new(5, 7);  // 5 mod 7

// Arithmetic
let sum = a + b;       // 8 mod 7 = 1
let diff = a - b;      // -2 mod 7 = 5
let prod = a * b;      // 15 mod 7 = 1
let quot = a / b;      // 3 * 5^(-1) mod 7 = 3 * 3 = 9 mod 7 = 2

// Inverse
let inv = b.inverse(); // 5^(-1) mod 7 = 3 (since 5*3 = 15 = 1 mod 7)
```

### Properties

- All non-zero elements have multiplicative inverses
- Division is well-defined for non-zero divisors
- Modulus must be prime for a field

## Polynomials over Z_p

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

// Create polynomial x^2 + 2x + 1 in Z_5[x]
let p = PolyZp::from_coeffs(vec![1, 2, 1], 5);  // coefficients: [a_0, a_1, a_2]

// Polynomial properties
let deg = p.degree();           // Some(2)
let coeffs = p.coefficients();  // [1, 2, 1]

// Create from integer coefficients (auto-reduce mod p)
let q = PolyZp::from_coeffs(vec![7, -3, 6], 5);  // becomes [2, 2, 1]
```

### Polynomial Operations

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

let f = PolyZp::from_coeffs(vec![1, 0, 1], 5);  // x^2 + 1
let g = PolyZp::from_coeffs(vec![1, 1], 5);     // x + 1

// Addition
let sum = f.add(&g);

// Multiplication
let prod = f.mul(&g);

// Division (quotient and remainder)
let (quotient, remainder) = f.div_rem(&g);

// Scalar multiplication
let scaled = f.scalar_mul(3);  // 3(x^2 + 1) = 3x^2 + 3
```

## GCD in Z_p[x]

The Euclidean algorithm works efficiently in Z_p[x]:

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

let f = PolyZp::from_coeffs(vec![4, 0, 0, 1], 5);  // x^3 + 4 in Z_5[x]
let g = PolyZp::from_coeffs(vec![1, 1], 5);         // x + 1 in Z_5[x]

// Compute GCD
let gcd = f.gcd(&g).unwrap();
```

### Extended GCD

Get GCD along with Bezout coefficients:

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

let f = PolyZp::from_coeffs(vec![1, 0, 1], 5);  // x^2 + 1
let g = PolyZp::from_coeffs(vec![1, 1], 5);     // x + 1

// Extended GCD: gcd = s*f + t*g
let (gcd, s, t) = f.extended_gcd(&g);
```

## Modular Reduction

Convert integer polynomials to Z_p:

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

// Integer coefficients reduced mod p
let int_coeffs: Vec<i64> = vec![15, -7, 23];
let p = 7u64;

let poly = PolyZp::from_coeffs(
    int_coeffs.iter().map(|&c| ((c % p as i64 + p as i64) % p as i64) as u64).collect(),
    p
);
```

## Bridge to Zippel GCD

The finite field module integrates with Zippel's algorithm:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::modular_gcd_univariate;
use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::mod_positive;

// mod_positive ensures positive representatives
let val = mod_positive(-3, 7);  // 4 (not -3)
```

### Symmetric Representation

For CRT reconstruction, use symmetric representatives:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::symmetric_mod;

// Symmetric mod: result in (-p/2, p/2]
let sym = symmetric_mod(6, 7);  // -1 (not 6)
let sym2 = symmetric_mod(2, 7); // 2
```

## Prime Selection

The algorithm uses carefully selected primes:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::LARGE_PRIMES;

// Large primes for CRT reconstruction
// Chosen to avoid common coefficient divisors
for p in LARGE_PRIMES.iter().take(5) {
    println!("Prime: {}", p);
}
```

### Selection Criteria

1. **Large enough**: Avoid overflow in reconstruction
2. **Coprime to leading coefficients**: Skip primes dividing LC(f) or LC(g)
3. **Well-distributed**: Different primes give independent information

## CRT Reconstruction

Combine results from multiple primes:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::crt_combine_u128;

// Combine results from two primes
let coef1 = 3;    // result mod p1
let mod1 = 7u128; // first prime
let coef2 = 5;    // result mod p2
let mod2 = 11u128; // second prime

let combined = crt_combine_u128(coef1, mod1, coef2, mod2);
// combined is the unique value in [0, 77) that is 3 mod 7 and 5 mod 11
```

### CRT Formula

For coprime moduli m1, m2:
```
x = a1 * m2 * (m2^(-1) mod m1) + a2 * m1 * (m1^(-1) mod m2) (mod m1*m2)
```

## Performance Considerations

- **Field size**: Larger primes reduce CRT iterations but increase per-operation cost
- **Polynomial degree**: Operations are O(d) to O(d^2) depending on operation
- **Sparsity**: Sparse polynomials may benefit from sparse representation

## Mathematical Background

### Finite Field Properties

Z_p is a field when p is prime:
- Every non-zero element has a multiplicative inverse
- Fermat's little theorem: a^(p-1) = 1 for a != 0
- Inverse via extended Euclidean algorithm or a^(p-2) mod p

### Polynomial Rings

Z_p[x] is a Euclidean domain:
- Division algorithm holds
- GCD can be computed via Euclidean algorithm
- Unique factorization (up to units)

### Chinese Remainder Theorem

If m1, m2 are coprime, then:
```
Z/(m1*m2) ≅ Z/m1 × Z/m2
```

This isomorphism is the basis for CRT reconstruction.

## See Also

- [GCD Algorithms](./gcd.md) - Uses finite field arithmetic
- [Polynomial Overview](./overview.md) - Module introduction
