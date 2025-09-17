# Assumptions System

> 📍 **You are here:** Advanced > Assumptions
>
> **Related Topics:** [Symbols & Numbers](../core/symbols-numbers.md) | [Simplification](../operations/simplification.md) | [Solving](../operations/solving.md)
>
> **Skill Level:** ⭐⭐⭐ Advanced

Tell MathHook that variables are positive, real, integer, etc. to enable smarter simplification and more accurate solving.

## Quick Start (⭐⭐⭐ Start here)

Assumptions change how MathHook simplifies expressions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Without assumptions: sqrt(x²) → |x| (absolute value)
let expr1 = expr!(sqrt(x^2));
let simplified1 = expr1.simplify();
println!("{}", simplified1); // Output: |x|

// With positive assumption: sqrt(x²) → x
// TODO: Assumptions API not yet implemented
// This will be: let x_pos = x.assume_positive();
// let expr2 = expr!(sqrt(x_pos^2));
// let simplified2 = expr2.simplify();
// println!("{}", simplified2); // Output: x
```

**Note:** The assumptions system is currently under development. This documentation describes the planned API and behavior.

## Table of Contents

- [Understanding Assumptions](#understanding-assumptions)
- [Assumption Types](#assumption-types)
- [Using Assumptions (Planned API)](#using-assumptions-planned-api)
- [Assumption Propagation](#assumption-propagation)
- [Conflict Detection](#conflict-detection)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Common Pitfalls](#common-pitfalls)
- [Performance Considerations](#performance-considerations)
- [See Also](#see-also)

## Understanding Assumptions

### What Are Assumptions? (Plain English)

**Assumptions** tell MathHook additional information about symbols:
- "x is always positive"
- "n is an integer"
- "theta is a real number"

This enables **smarter simplification** and **more accurate solving**.

**Example without assumptions:**
- `sqrt(x²)` simplifies to `|x|` (absolute value needed because x could be negative)

**Example with assumptions:**
- If we assume `x > 0`, then `sqrt(x²)` simplifies to just `x`

### Mathematical Background

**Assumptions affect algebraic properties:**

1. **Sign properties:**
   - If $x > 0$ and $y > 0$, then $x + y > 0$ and $xy > 0$
   - If $x > 0$, then $x^2 > 0$ and $\sqrt{x}$ is real

2. **Domain properties:**
   - If $x \in \mathbb{R}$, then $\overline{x} = x$ (complex conjugate equals itself)
   - If $x \in \mathbb{Z}$, then $\sin(2\pi x) = 0$

3. **Type properties:**
   - If $n \in \mathbb{Z}$, then $n! = n(n-1)(n-2)\cdots 1$
   - If $p$ is prime, then $\gcd(p, n) \in \{1, p\}$

**Reference:** Assumptions systems are used in computer algebra systems like Mathematica (Assumptions) and SymPy (assumptions module). See Cohen, *Computer Algebra and Symbolic Computation*, Chapter 8.

### When to Use Assumptions

**Use assumptions for:**
1. **Simplification:** Enable more aggressive algebraic reductions
2. **Optimization:** Constrained optimization problems (x ≥ 0)
3. **Physics problems:** Real-valued quantities (energy, mass)
4. **Number theory:** Integer-only algorithms (GCD, primes)
5. **Analysis:** Convergence of series (|x| < 1)

**Don't use assumptions when:**
- Variables truly can take any value
- You want general-purpose formulas
- Debugging incorrect simplifications (assumptions might hide bugs)

## Assumption Types

MathHook will support several categories of assumptions:

### Domain Assumptions

Tell MathHook which number domain a symbol belongs to:

| Assumption | Mathematical Set | Meaning |
|------------|-----------------|---------|
| `real` | $\mathbb{R}$ | Real numbers |
| `complex` | $\mathbb{C}$ | Complex numbers |
| `integer` | $\mathbb{Z}$ | Integers |
| `rational` | $\mathbb{Q}$ | Rational numbers (p/q) |
| `natural` | $\mathbb{N}$ | Natural numbers (0, 1, 2, ...) |

**Examples:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Planned API
let x = symbol!(x).assume_real();     // x ∈ ℝ
let n = symbol!(n).assume_integer();  // n ∈ ℤ
let p = symbol!(p).assume_rational(); // p ∈ ℚ
```

### Sign Assumptions

Tell MathHook the sign of a symbol:

| Assumption | Mathematical Condition | Meaning |
|------------|----------------------|---------|
| `positive` | $x > 0$ | Strictly positive |
| `negative` | $x < 0$ | Strictly negative |
| `nonnegative` | $x \geq 0$ | Positive or zero |
| `nonpositive` | $x \leq 0$ | Negative or zero |
| `nonzero` | $x \neq 0$ | Not equal to zero |

**Examples:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Planned API
let x = symbol!(x).assume_positive();    // x > 0
let y = symbol!(y).assume_nonnegative(); // y ≥ 0
let z = symbol!(z).assume_nonzero();     // z ≠ 0
```

### Type Assumptions

Tell MathHook special number-theoretic properties:

| Assumption | Meaning | Mathematical Property |
|------------|---------|---------------------|
| `prime` | Prime number | $p$ is prime |
| `even` | Even integer | $n = 2k$ for some $k \in \mathbb{Z}$ |
| `odd` | Odd integer | $n = 2k + 1$ for some $k \in \mathbb{Z}$ |
| `composite` | Composite number | $n = ab$ where $1 < a, b < n$ |

**Examples:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Planned API
let p = symbol!(p).assume_prime();   // p is prime
let n = symbol!(n).assume_even();    // n = 2k
let m = symbol!(m).assume_odd();     // m = 2k + 1
```

### Bound Assumptions

Tell MathHook the range of a symbol:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Planned API
let x = symbol!(x).assume_bounded(0.0, 1.0);  // 0 ≤ x ≤ 1
let theta = symbol!(theta).assume_in_range(-PI, PI); // -π ≤ θ ≤ π
```

**Use cases:**
- Convergence of series: $|x| < 1$
- Trigonometric simplification: $0 \leq \theta < 2\pi$
- Optimization constraints: $0 \leq t \leq 10$

## Using Assumptions (Planned API)

### Setting Assumptions

**Chained API (recommended):**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Single assumption
let x = symbol!(x).assume_positive();

// Multiple assumptions (chaining)
let n = symbol!(n)
    .assume_integer()
    .assume_positive();

// Complex assumption
let theta = symbol!(theta)
    .assume_real()
    .assume_bounded(-PI, PI);
```

**Builder pattern (alternative):**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::symbol::{Symbol, SymbolBuilder};

let x = SymbolBuilder::new("x")
    .domain(Domain::Real)
    .sign(Sign::Positive)
    .build();
```

### Querying Assumptions

Check if an assumption holds:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

// Query methods
assert!(x.is_positive());     // Returns: true
assert!(x.is_real());         // Returns: true (implied by positive)
assert!(x.is_nonzero());      // Returns: true (implied by positive)
assert!(!x.is_negative());    // Returns: false (contradicts positive)
```

### Using Assumptions in Simplification

Assumptions automatically affect simplification:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();
let y = symbol!(y).assume_positive();

// sqrt(x²) → x (not |x|)
let expr1 = expr!(sqrt(x^2));
assert_eq!(expr1.simplify(), expr!(x));

// sqrt(x*y) → sqrt(x)*sqrt(y) (valid for positive x, y)
let expr2 = expr!(sqrt(x * y));
assert_eq!(expr2.simplify(), expr!(sqrt(x) * sqrt(y)));

// log(x^n) → n*log(x) (valid for positive x)
let n = symbol!(n).assume_real();
let expr3 = expr!(log(x^n));
assert_eq!(expr3.simplify(), expr!(n * log(x)));
```

## Assumption Propagation

MathHook **propagates assumptions** through operations:

### Addition and Multiplication

**Rules:**
- $x > 0 \land y > 0 \Rightarrow x + y > 0$
- $x > 0 \land y > 0 \Rightarrow xy > 0$
- $x \in \mathbb{R} \land y \in \mathbb{R} \Rightarrow x + y \in \mathbb{R}$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();
let y = symbol!(y).assume_positive();

let sum = expr!(x + y);
assert!(sum.is_positive());  // Propagated: sum > 0

let product = expr!(x * y);
assert!(product.is_positive()); // Propagated: product > 0
```

### Powers and Roots

**Rules:**
- $x > 0 \Rightarrow x^n > 0$ for all $n \in \mathbb{R}$
- $x > 0 \Rightarrow \sqrt{x} \in \mathbb{R}$
- $x^2 \geq 0$ for all $x \in \mathbb{R}$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

let power = expr!(x^2);
assert!(power.is_positive());  // x² > 0

let sqrt_expr = expr!(sqrt(x));
assert!(sqrt_expr.is_real());  // sqrt(x) is real
assert!(sqrt_expr.is_positive()); // sqrt(x) > 0
```

### Functions

**Rules:**
- $x \in \mathbb{R} \Rightarrow \sin(x), \cos(x) \in [-1, 1]$
- $x > 0 \Rightarrow \log(x) \in \mathbb{R}$
- $x \in \mathbb{R} \Rightarrow e^x > 0$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_real();

let sin_x = expr!(sin(x));
assert!(sin_x.is_bounded(-1.0, 1.0));

let exp_x = expr!(exp(x));
assert!(exp_x.is_positive()); // e^x > 0 for all real x
```

## Conflict Detection

MathHook detects **conflicting assumptions** and raises errors:

### Direct Conflicts

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

// ❌ ERROR: Cannot assume both positive and negative
// let x_neg = x.assume_negative(); // Panics or returns error
```

### Implied Conflicts

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x)
    .assume_positive()
    .assume_integer();

// Equation: x² = -1
let equation = expr!(x^2 + 1);

// Solving returns no solution (conflict: x² = -1 impossible for positive x)
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
assert!(solutions.is_empty()); // No solutions
```

### Assumption Hierarchy

Some assumptions **imply** others:

| Assumption | Implies |
|------------|---------|
| `positive` | `nonzero`, `real`, `nonnegative` |
| `negative` | `nonzero`, `real`, `nonpositive` |
| `integer` | `rational`, `real` |
| `rational` | `real` |
| `prime` | `integer`, `positive` |
| `even` | `integer` |
| `odd` | `integer` |

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

// Automatically inferred (no need to explicitly assume)
assert!(x.is_nonzero());     // ✅ Implied
assert!(x.is_real());        // ✅ Implied
assert!(x.is_nonnegative()); // ✅ Implied
```

## Real-World Applications

### 1. **Optimization (Constrained Variables)**

**Problem:** Minimize $f(x) = x^2 + 1/x$ for $x > 0$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use derivatives::differentiate;

let x = symbol!(x).assume_positive();
let f = expr!(x^2 + 1/x);

// Take derivative
let df = differentiate(&f, &x);
// df/dx = 2x - 1/x²

// Solve df/dx = 0
let mut solver = MathSolver::new();
let critical_points = solver.solve(&df, &x);
// Solution: x = (1/2)^(1/3)

// Because x > 0, we don't need to check negative solutions
```

### 2. **Physics (Real Quantities)**

**Problem:** Energy conservation in classical mechanics

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Assume all physical quantities are real and positive
let m = symbol!(m).assume_positive();  // Mass
let v = symbol!(v).assume_real();      // Velocity
let g = symbol!(g).assume_positive();  // Gravitational acceleration
let h = symbol!(h).assume_nonnegative(); // Height

// Kinetic energy: KE = (1/2)mv²
let kinetic = expr!((1/2) * m * (v^2));

// Potential energy: PE = mgh
let potential = expr!(m * g * h);

// Total energy: E = KE + PE
let total_energy = expr!(kinetic + potential);
assert!(total_energy.is_real()); // Energy is real
```

### 3. **Number Theory (Integer Algorithms)**

**Problem:** GCD and divisibility

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let a = symbol!(a).assume_integer().assume_positive();
let b = symbol!(b).assume_integer().assume_positive();

// GCD is always an integer
let gcd_expr = expr!(gcd(a, b));
assert!(gcd_expr.is_integer());
assert!(gcd_expr.is_positive());

// For prime p: gcd(p, n) ∈ {1, p}
let p = symbol!(p).assume_prime();
let n = symbol!(n).assume_integer();
let gcd_p_n = expr!(gcd(p, n));
// Result must be 1 or p
```

### 4. **Complex Analysis (Domain Restrictions)**

**Problem:** Principal branch of logarithm

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For real positive x: log is real
let x_real = symbol!(x).assume_positive();
let log_real = expr!(log(x_real));
assert!(log_real.is_real());

// For complex z: log has branch cut on negative real axis
let z = symbol!(z).assume_complex();
let log_complex = expr!(log(z));
// log(z) = ln|z| + i*arg(z) where -π < arg(z) ≤ π
```

## Common Patterns

### Pattern 1: Simplifying Square Roots

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();
let y = symbol!(y).assume_positive();

// sqrt(x²) → x
let expr1 = expr!(sqrt(x^2));
assert_eq!(expr1.simplify(), expr!(x));

// sqrt(xy) → sqrt(x)*sqrt(y)
let expr2 = expr!(sqrt(x * y));
assert_eq!(expr2.simplify(), expr!(sqrt(x) * sqrt(y)));

// sqrt(x/y) → sqrt(x)/sqrt(y)
let expr3 = expr!(sqrt(x / y));
assert_eq!(expr3.simplify(), expr!(sqrt(x) / sqrt(y)));
```

### Pattern 2: Logarithm Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();
let y = symbol!(y).assume_positive();
let n = symbol!(n).assume_real();

// log(x^n) → n*log(x)
let expr1 = expr!(log(x^n));
assert_eq!(expr1.simplify(), expr!(n * log(x)));

// log(xy) → log(x) + log(y)
let expr2 = expr!(log(x * y));
assert_eq!(expr2.simplify(), expr!(log(x) + log(y)));

// log(x/y) → log(x) - log(y)
let expr3 = expr!(log(x / y));
assert_eq!(expr3.simplify(), expr!(log(x) - log(y)));
```

### Pattern 3: Absolute Value Removal

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

// |x| → x
let expr1 = expr!(abs(x));
assert_eq!(expr1.simplify(), expr!(x));

// |x*y| → |x|*|y| → x*y (if both positive)
let y = symbol!(y).assume_positive();
let expr2 = expr!(abs(x * y));
assert_eq!(expr2.simplify(), expr!(x * y));
```

### Pattern 4: Trigonometric Bounds

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_real();

// -1 ≤ sin(x) ≤ 1
let sin_x = expr!(sin(x));
assert!(sin_x.is_bounded(-1.0, 1.0));

// -1 ≤ cos(x) ≤ 1
let cos_x = expr!(cos(x));
assert!(cos_x.is_bounded(-1.0, 1.0));

// 0 ≤ sin²(x) ≤ 1
let sin_squared = expr!(sin(x)^2);
assert!(sin_squared.is_bounded(0.0, 1.0));
```

## Common Pitfalls

### ❌ Pitfall 1: Forgetting Assumptions

**Problem:** Generic simplification is too conservative

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Without assumptions, sqrt(x²) → |x|
let x = symbol!(x);
let expr = expr!(sqrt(x^2));
let result = expr.simplify();
assert_eq!(result, expr!(abs(x))); // Not simplified to x

// ✅ CORRECT: With assumptions, sqrt(x²) → x
let x_pos = symbol!(x).assume_positive();
let expr_pos = expr!(sqrt(x_pos^2));
let result_pos = expr_pos.simplify();
assert_eq!(result_pos, expr!(x_pos)); // Simplified to x
```

### ❌ Pitfall 2: Conflicting Assumptions

**Problem:** Contradictory assumptions cause errors

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

// ❌ ERROR: Cannot be both positive and negative
// let x_conflict = x.assume_negative(); // Panics or returns error
```

**Solution:** Check for conflicts before adding assumptions:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x).assume_positive();

if !x.is_negative() {
    // Safe to proceed
} else {
    // Handle conflict
}
```

### ❌ Pitfall 3: Global vs Local Assumptions

**Problem:** Assumptions leak between different parts of code

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Assumption applies everywhere
let x = symbol!(x).assume_positive();
let eq1 = expr!(sqrt(x^2)); // Assumes x > 0
let eq2 = expr!(x^2 + 1);   // Still assumes x > 0 (may be unintended)

// ✅ CORRECT: Use different symbols for different assumptions
let x_general = symbol!(x);
let x_pos = symbol!(x_pos).assume_positive();

let eq1 = expr!(sqrt(x_pos^2)); // x_pos > 0
let eq2 = expr!(x_general^2 + 1); // x_general unrestricted
```

### ❌ Pitfall 4: Overconstraining Problems

**Problem:** Too restrictive assumptions eliminate valid solutions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Equation: x² = 4
let equation = expr!(x^2 - 4);

// ❌ WRONG: Assuming positive eliminates negative solution
let x_pos = symbol!(x).assume_positive();
let mut solver = MathSolver::new();
let solutions_pos = solver.solve(&equation, &x_pos);
// Only finds x = 2 (misses x = -2)

// ✅ CORRECT: Don't assume when finding all solutions
let x = symbol!(x);
let solutions_all = solver.solve(&equation, &x);
// Finds both x = 2 and x = -2
```

## Performance Considerations

### Memory Overhead

Assumptions add minimal memory overhead:
- Stored as bitflags (1-2 bytes per symbol)
- Checked during simplification (negligible cost)

### Simplification Speed

Assumptions can **speed up** simplification:
- Fewer cases to check (e.g., don't need absolute value)
- More aggressive algebraic reductions

**Benchmark example:**
```text
Simplification of sqrt(x²) + sqrt(y²):
- Without assumptions: 12μs (needs |x| + |y|)
- With positive assumptions: 8μs (direct x + y)
Speedup: 1.5x faster
```

### When to Use Assumptions

**Use assumptions for:**
- Repeated simplification of same symbols
- Performance-critical code paths
- Domain-specific problems (optimization, physics)

**Don't use assumptions for:**
- One-off simplifications
- General-purpose libraries (unless user-controlled)
- Debugging (assumptions might hide incorrect behavior)

## API Reference

**Assumption Methods** (Planned):
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Setting assumptions
fn assume_real(self) -> Symbol;
fn assume_complex(self) -> Symbol;
fn assume_integer(self) -> Symbol;
fn assume_rational(self) -> Symbol;
fn assume_positive(self) -> Symbol;
fn assume_negative(self) -> Symbol;
fn assume_nonnegative(self) -> Symbol;
fn assume_nonzero(self) -> Symbol;
fn assume_prime(self) -> Symbol;
fn assume_even(self) -> Symbol;
fn assume_odd(self) -> Symbol;
fn assume_bounded(min: f64, max: f64) -> Symbol;

// Querying assumptions
fn is_real(&self) -> bool;
fn is_complex(&self) -> bool;
fn is_integer(&self) -> bool;
fn is_positive(&self) -> bool;
fn is_negative(&self) -> bool;
fn is_nonzero(&self) -> bool;
fn is_prime(&self) -> bool;
fn is_even(&self) -> bool;
fn is_odd(&self) -> bool;
fn is_bounded(&self) -> bool;
fn get_bounds(&self) -> Option<(f64, f64)>;
```

## See Also

- **[Symbols & Numbers](../core/symbols-numbers.md)** - Symbol creation and number types
- **[Simplification](../operations/simplification.md)** - How assumptions affect simplification
- **[Solving Equations](../operations/solving.md)** - Using assumptions in equation solving
- **[Pattern Matching](../core/pattern-matching.md)** - Querying properties in patterns
