# GCD Algorithms

**Last Updated:** 2025-11-29T12:30:00

MathHook provides multiple GCD (Greatest Common Divisor) algorithms for polynomials, optimized for different use cases.

## Algorithm Selection Guide

### Quick Decision Tree

```
Need GCD of two polynomials?
├─ Both are i64 integers? → integer_gcd(a, b)
├─ Don't know the structure? → polynomial_gcd(&p1, &p2)
├─ Single variable (x)? → univariate_gcd(&p1, &p2, &x)
├─ Need cofactors too? → modular_gcd_univariate(&p1, &p2, &x)
└─ Multiple variables (x, y, z)? → multivariate_gcd(&p1, &p2, &[x, y, z])
```

### Function Reference

| Function | Use Case | Returns |
|----------|----------|---------|
| `polynomial_gcd` | General-purpose entry point | `Result<Expression, PolynomialError>` |
| `integer_gcd` | Direct i64 integers only | `i64` |
| `univariate_gcd` | Single-variable polynomials | `Result<Expression, PolynomialError>` |
| `modular_gcd_univariate` | When you need cofactors | `Result<(gcd, cof1, cof2), PolynomialError>` |
| `multivariate_gcd` | Multi-variable polynomials | `Result<Expression, PolynomialError>` |

## General-Purpose API

For most use cases, use the `PolynomialGcdOps` trait:

```rust
use mathhook_core::core::polynomial::PolynomialGcdOps;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// f = x^2 - 1 = (x-1)(x+1)
let f = expr!((x ^ 2) - 1);
// g = x^2 - 2x + 1 = (x-1)^2
let g = expr!((x ^ 2) - (2 * x) + 1);

// Compute GCD
let gcd = f.polynomial_gcd(&g).unwrap();
// gcd = x - 1

// Compute LCM
let lcm = f.polynomial_lcm(&g).unwrap();
// lcm = (x-1)^2(x+1)
```

## Zippel's Modular GCD Algorithm

For performance-critical applications, the Zippel algorithm provides industrial-strength GCD computation using modular arithmetic.

### How It Works

1. **Content Extraction**: Separate integer content from primitive parts
2. **Prime Selection**: Choose primes that don't divide leading coefficients
3. **Modular GCD**: Compute GCD in Z_p[x] using Euclidean algorithm
4. **CRT Reconstruction**: Combine results from multiple primes using Chinese Remainder Theorem
5. **Trial Division**: Verify the result divides both inputs

### Univariate Modular GCD

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::modular_gcd_univariate;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let f = expr!((x ^ 2) - 1);
let g = expr!(x - 1);

// Returns (gcd, cofactor_f, cofactor_g)
let (gcd, cof_f, cof_g) = modular_gcd_univariate(&f, &g, &x).unwrap();

// Verify: f = gcd * cof_f, g = gcd * cof_g
```

### Multivariate GCD

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    multivariate_gcd_zippel,
    MultivariateGcdConfig
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// f = x*y, g = x*y + x
let f = expr!(x * y);
let g = expr!((x * y) + x);

let config = MultivariateGcdConfig::default();
let (gcd, _, _) = multivariate_gcd_zippel(&f, &g, &[x, y], config).unwrap();
// gcd = x
```

### Configuration Options

```rust
pub struct MultivariateGcdConfig {
    /// Maximum number of evaluation points per variable
    pub max_eval_points: usize,
    /// Whether to use sparse optimization
    pub use_sparse: bool,
    /// Prime index to start with
    pub starting_prime_idx: usize,
}
```

## Content and Primitive Part

The content/primitive part decomposition is fundamental to GCD computation:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    extract_content,
    primitive_part
};

let coeffs = vec![6, 12, 18];  // 6 + 12x + 18x^2

// Extract content (GCD of coefficients)
let content = extract_content(&coeffs);  // 6

// Get primitive part
let (cont, pp) = primitive_part(&coeffs);  // (6, [1, 2, 3])
```

## Sparse Polynomial Optimization

For polynomials with many zero coefficients, sparse optimization improves performance:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    is_sparse,
    analyze_sparsity
};

let sparse_coeffs = vec![1, 0, 0, 0, 0, 0, 0, 5];  // x^7 + 1

// Check sparsity (density < 0.3)
assert!(is_sparse(&sparse_coeffs));

// Get detailed sparsity info
let info = analyze_sparsity(&sparse_coeffs);
println!("Non-zero terms: {}", info.nonzero_count);
println!("Density: {}", info.density);
```

## Trial Division Verification

After CRT reconstruction, verify the candidate divides both inputs:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    verify_gcd_candidate,
    trial_divide,
    TrialDivisionResult
};

let f = vec![-1, 0, 1];      // x^2 - 1
let g = vec![-1, 1];         // x - 1
let candidate = vec![-1, 1]; // x - 1

// Verify candidate
if verify_gcd_candidate(&f, &g, &candidate) {
    println!("Candidate is valid GCD");
}

// Detailed trial division
match trial_divide(&f, &candidate) {
    TrialDivisionResult::Success { quotient, remainder: _ } => {
        println!("Division successful, quotient: {:?}", quotient);
    }
    TrialDivisionResult::NotExact => {
        println!("Division not exact");
    }
    _ => {}
}
```

## Educational Features

The Zippel module provides educational explanations:

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::educational;

// Get algorithm overview
println!("{}", educational::algorithm_overview());

// Explain specific steps
println!("{}", educational::explain_crt_step());
println!("{}", educational::explain_content_extraction());
```

## Performance Characteristics

| Algorithm | Complexity | Best For |
|-----------|------------|----------|
| Integer GCD | O(log(min(a,b))) | Small integers |
| Univariate Modular | O(d^2) | Single variable polynomials |
| Multivariate Zippel | O(d^n) | Sparse multivariate |
| Groebner-based | Doubly exponential | Ideal membership |

Where `d` is degree and `n` is number of variables.

## Error Handling

```rust
use mathhook_core::core::polynomial::PolynomialError;

match f.polynomial_gcd(&g) {
    Ok(gcd) => println!("GCD: {}", gcd),
    Err(PolynomialError::NotPolynomial { expr }) => {
        println!("Expression is not a polynomial: {}", expr);
    }
    Err(PolynomialError::MaxIterationsExceeded { operation, limit }) => {
        println!("{} exceeded {} iterations", operation, limit);
    }
    Err(e) => println!("Error: {:?}", e),
}
```

## See Also

- [Polynomial Overview](./overview.md)
- [Finite Field Arithmetic](./finite-field.md) - Underlying Z_p operations
- [Division Algorithms](./division.md) - Polynomial division
