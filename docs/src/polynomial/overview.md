# Polynomial Module Overview

**Last Updated:** 2025-11-29T12:30:00

The polynomial module provides comprehensive symbolic polynomial manipulation capabilities in MathHook. It implements a trait-based architecture for automatic classification, property computation, arithmetic operations, and GCD algorithms.

## Architecture

The module uses **decomposed traits** for clean separation of concerns:

| Trait | Purpose |
|-------|---------|
| `PolynomialClassification` | Type detection and variable extraction |
| `PolynomialProperties` | Degree, leading coefficient, content, primitive part |
| `PolynomialArithmetic` | Division, multiplication, addition |
| `PolynomialGcdOps` | GCD, LCM, cofactors |
| `PolynomialEducational` | Step-by-step explanations (opt-in) |

### Design Philosophy

1. **Automatic Classification**: Users don't need to manually wrap expressions - the system detects polynomial structure automatically
2. **Trait Composition**: Functionality is split into focused traits rather than one monolithic interface
3. **Performance First**: Thread-local LRU caching for expensive operations like degree computation
4. **Educational Support**: Optional step-by-step explanations for learning

## Module Structure

```
polynomial/
├── classification.rs   # PolynomialClassification trait
├── properties.rs       # PolynomialProperties trait (cached)
├── arithmetic.rs       # PolynomialArithmetic trait
├── gcd_ops.rs         # PolynomialGcdOps trait
├── educational.rs     # PolynomialEducational trait
├── cache.rs           # Thread-local LRU caching
├── error.rs           # PolynomialError types
├── algorithms/
│   ├── gcd.rs         # High-level GCD entry points
│   ├── division.rs    # Polynomial long division
│   ├── factorization.rs # Factorization algorithms
│   ├── resultant.rs   # Resultant and discriminant
│   └── zippel_gcd/    # Modular GCD implementation
│       ├── univariate.rs
│       ├── multivariate.rs
│       ├── content.rs
│       ├── sparse.rs
│       └── educational.rs
├── finite_field/      # Z_p arithmetic for modular GCD
│   ├── element.rs     # Field elements
│   ├── poly.rs        # Polynomials over Z_p
│   └── gcd.rs         # GCD in Z_p[x]
├── groebner.rs        # Groebner basis computation
└── special_families.rs # Orthogonal polynomials
```

## Quick Start

### Basic Usage

```rust
use mathhook_core::core::polynomial::{
    PolynomialClassification,
    PolynomialProperties,
    PolynomialGcdOps
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Create polynomials using expr! macro
let f = expr!((x ^ 2) + (2 * x) + 1);  // x^2 + 2x + 1
let g = expr!((x ^ 2) - 1);             // x^2 - 1

// Properties
assert_eq!(f.degree(&x), Some(2));
assert!(f.is_polynomial_in(&[x.clone()]));

// GCD computation
let gcd = f.polynomial_gcd(&g).unwrap();
// gcd = x + 1 (since f = (x+1)^2 and g = (x+1)(x-1))
```

### Polynomial Classification

```rust
use mathhook_core::core::polynomial::PolynomialClassification;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// Automatic detection
let poly = expr!((x ^ 2) + (y * x) + 1);
assert!(poly.is_polynomial());
assert!(poly.is_polynomial_in(&[x.clone(), y.clone()]));

// Variable extraction
let vars = poly.polynomial_variables();
// vars contains x and y
```

## Key Concepts

### Univariate vs Multivariate

| Type | Description | Example |
|------|-------------|---------|
| **Univariate** | Single variable | `x^2 + 2x + 1` |
| **Multivariate** | Multiple variables | `x^2 + xy + y^2` |

The module automatically detects and routes to appropriate algorithms.

### Content and Primitive Part

For a polynomial `f(x) = a_n x^n + ... + a_1 x + a_0`:

- **Content**: `gcd(a_n, ..., a_1, a_0)` - the GCD of all coefficients
- **Primitive Part**: `f(x) / content(f)` - the polynomial with content factored out

```rust
use mathhook_core::core::polynomial::PolynomialProperties;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let poly = expr!((6 * (x ^ 2)) + (9 * x) + 3);  // 6x^2 + 9x + 3

let content = poly.content();           // 3
let primitive = poly.primitive_part();  // 2x^2 + 3x + 1
```

### Caching Strategy

Property computations are cached using thread-local LRU caching:

- **Automatic**: No user intervention required
- **Thread-safe**: Each thread has its own cache
- **Size-limited**: LRU eviction prevents memory bloat
- **Hash-based**: Uses pointer address + discriminant for fast lookup

## See Also

- [GCD Algorithms](./gcd.md) - Detailed guide to GCD computation
- [Division and Factorization](./division.md) - Polynomial division algorithms
- [Groebner Bases](./groebner.md) - Polynomial ideal computation
- [Special Families](./special-families.md) - Orthogonal polynomials
- [Finite Field Arithmetic](./finite-field.md) - Modular arithmetic for GCD
