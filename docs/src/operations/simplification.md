# Simplification

## Overview

MathHook provides comprehensive symbolic simplification for mathematical expressions, with full support for noncommutative algebra (matrices, operators, quaternions). The simplification system implements canonical forms and mathematical identities to reduce expressions to their simplest equivalent representation.

## Capabilities

### Arithmetic Simplification

MathHook simplifies expressions through three main operations:

- **Addition:** Collects like terms, flattens nested sums, removes identity (0)
- **Multiplication:** Combines factors, flattens nested products, removes identity (1), applies power rule
- **Power:** Simplifies exponents, distributes powers when safe (commutative case only)

### Power Rule Implementation

**New in this release:** The multiplication simplifier now implements the power rule:

$$x^a \cdot x^b \rightarrow x^{a+b}$$

This rule combines like powers with the same base, significantly reducing expression complexity.

**Example:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!((x^2) * (x^3));
let simplified = expr.simplify();
// Result: x^5
```

### Noncommutative Algebra

MathHook preserves order for noncommutative symbols, ensuring mathematical correctness for:

- **Matrices:** $AB \neq BA$ in general
- **Operators:** Quantum mechanics commutators $[x,p] = xp - px$
- **Quaternions:** $ij = k$, but $ji = -k$

The simplification system detects commutativity properties and only applies order-dependent rules when safe.

**Example:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Scalar symbols (commutative) - factors can be sorted
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!(y * x);
let simplified = expr.simplify();
// Result: x * y (sorted alphabetically)

// Matrix symbols (noncommutative) - order preserved
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!(B * A);
let simplified = expr.simplify();
// Result: B * A (original order preserved)
```

### Rational Arithmetic

MathHook performs exact rational arithmetic with arbitrary precision:

- **Exact representation:** Fractions like $\frac{1}{3}$ stay as rationals, not floats
- **Overflow protection:** Uses checked arithmetic, promotes to `BigInt` on overflow
- **Automatic simplification:** Reduces fractions to lowest terms

**Example:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = expr!(1/3 + 1/6);  // Rational arithmetic
let simplified = expr.simplify();
// Result: 1/2 (exact rational, not 0.5)
```

### Numerical Stability

The simplification system includes several numerical stability features:

- **Checked arithmetic:** Integer operations use `checked_mul`, `checked_add` to detect overflow
- **BigInt promotion:** On overflow, automatically promotes to arbitrary precision
- **Iterative flattening:** Avoids stack overflow for deeply nested expressions

## Examples

### Basic Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Identity elements
let expr = expr!((x + 0) * 1);
let simplified = expr.simplify();
// Result: x

// Constant folding
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5
```

### Power Rule

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Combine like powers
let expr = expr!((x^2) * (x^3));
let simplified = expr.simplify();
// Result: x^5

// Multiple powers
let expr = expr!((x^2) * (x^3) * (x^4));
let simplified = expr.simplify();
// Result: x^9
```

### Noncommutative Matrices

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Matrix multiplication does NOT commute
let expr = expr!(A * B);
// Simplification preserves order: A*B ≠ B*A
```

### Power Distribution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Scalars (commutative): distributes power
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x * y) ^ 2);
let simplified = expr.simplify();
// Result: x^2 * y^2 (distributed)

// Matrices (noncommutative): does NOT distribute
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!((A * B) ^ 2);
let simplified = expr.simplify();
// Result: (A*B)^2 (NOT distributed to A^2 * B^2)
```

## Performance Characteristics

### Targets

- **Simplification time:** <1ms for expressions with <100 nodes
- **Memory:** Minimal allocations through iterative flattening
- **Cache efficiency:** Expression type optimized for 32-byte size (2 per cache line)

### Optimization Strategies

- **Iterative flattening:** Avoids stack overflow for deeply nested expressions
- **Early exit:** Returns immediately for identity elements and trivial cases
- **Power combining:** O(n) grouping of like powers

### Benchmarks

Run benchmarks to measure performance on your hardware:

```bash
cargo bench --bench simplification_benchmarks
```

Benchmark results are hardware-dependent. Target performance:
- Simple expressions (<10 nodes): <10μs
- Medium expressions (10-100 nodes): <1ms
- Complex expressions (100-1000 nodes): <100ms

See [benchmarks documentation](../benchmarks.md) for detailed performance data.

## Limitations

Current limitations of the simplification system:

- **Factorization:** Not yet implemented (planned for future release)
- **Trigonometric identities:** Limited (basic Pythagorean identity only)
- **Advanced simplification:** Gröbner bases, polynomial division (future work)
- **Symbolic zero detection:** May not recognize all equivalent-to-zero expressions

## Mathematical Correctness

All simplification operations preserve mathematical equivalence:

- **SymPy validated:** Core algorithms cross-validated against SymPy
- **Property-based tested:** Algebraic properties verified through property tests
- **Commutativity-aware:** Respects noncommutative algebra requirements
- **Exact arithmetic:** Uses rationals for symbolic math, floats only for approximation

## API Reference

For complete API documentation, see:
- [Rust API documentation](https://docs.rs/mathhook-core/latest/mathhook_core/simplify/)
- [Simplify trait documentation](https://docs.rs/mathhook-core/latest/mathhook_core/simplify/trait.Simplify.html)

## Related Topics

- [Expression creation with macros](../api/macros.md)
- [Noncommutative algebra](../features/noncommutative.md)
- [Numerical precision and stability](../advanced/numerical-stability.md)
- [Performance optimization](../advanced/performance.md)
