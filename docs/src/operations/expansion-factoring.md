# Expansion and Factoring

> 📍 **You are here:** Operations > Expansion & Factoring
>
> **Related Topics:** [Simplification](simplification.md) | [Differentiation](differentiation.md) | [Solving Equations](solving.md)
>
> **Skill Level:** ⭐ Beginner (basics) | ⭐⭐ Intermediate (noncommutative) | ⭐⭐⭐ Advanced (binomial theorem)

Transform expressions between expanded and factored forms for easier manipulation and analysis.

## Quick Start (⭐ Start here if you're new)

Expand a simple product in 3 lines:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Expand (x + 1)(x - 1)
let factored = expr!((x + 1) * (x - 1));
let expanded = factored.expand();
// Result: x^2 - 1

println!("{}", expanded); // x² - 1
```

## Table of Contents

- [Understanding Expansion & Factoring](#understanding-expansion--factoring)
- [Basic Expansion (⭐ Beginner)](#basic-expansion--beginner)
- [Power Expansion](#power-expansion)
- [Trigonometric & Logarithmic Expansion](#trigonometric--logarithmic-expansion)
- [Noncommutative Expansion (⭐⭐ Intermediate)](#noncommutative-expansion--intermediate)
- [Factoring (Coming Soon)](#factoring-coming-soon)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)

## Understanding Expansion & Factoring

### What is Expansion? (Plain English)

**Expansion** multiplies out products and distributes operations to write expressions as sums of terms.

**Examples:**
- `(x + 1)(x - 1)` → `x^2 - 1` (difference of squares)
- `(x + y)^2` → `x^2 + 2xy + y^2` (perfect square)
- `2(x + 3)` → `2x + 6` (distribution)

### Mathematical Background

**Distributive Law:**
$$a(b + c) = ab + ac$$

**Binomial Expansion:**
$$(x + y)^n = \sum_{k=0}^{n} \binom{n}{k} x^{n-k} y^k$$

For small powers:
- $$(x + y)^2 = x^2 + 2xy + y^2$$
- $$(x + y)^3 = x^3 + 3x^2y + 3xy^2 + y^3$$
- $$(x - y)^2 = x^2 - 2xy + y^2$$ (difference of squares when factored)

**Special Products:**
- **Difference of Squares:** $(x + y)(x - y) = x^2 - y^2$
- **Perfect Square Trinomial:** $(x + y)^2 = x^2 + 2xy + y^2$
- **Sum/Difference of Cubes:** $(x + y)^3$, $(x - y)^3$

**Reference:** Stewart, *Calculus* 8th ed., Appendix A (Algebra Review)

### When to Use Expansion

**Expand when:**
1. **Simplifying derivatives:** Expanded form often easier to differentiate
2. **Integrating:** Polynomials easier to integrate than products
3. **Solving equations:** Collect like terms to identify coefficients
4. **Circuit analysis:** Distribute impedances across parallel components
5. **Signal processing:** Expand transfer functions for frequency response

**Don't expand when:**
- Factored form reveals structure (zeros, poles)
- Factored form is simpler (fewer terms)
- Need to identify common factors

## Basic Expansion (⭐ Beginner)

### Simple Products

Expand products of sums:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Expand 2(x + 3)
let expr1 = expr!(2 * (x + 3));
let expanded1 = expr1.expand();
// Result: 2x + 6

// Expand (x + 1)(x + 2)
let expr2 = expr!((x + 1) * (x + 2));
let expanded2 = expr2.expand();
// Result: x² + 3x + 2

// Expand (x + y)(x - y) - difference of squares
let y = symbol!(y);
let expr3 = expr!((x + y) * (x - y));
let expanded3 = expr3.expand();
// Result: x² - y²
```

### Multiple Variables

Expansion works with any number of variables:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// Expand (x + y)(x + z)
let expr = expr!((x + y) * (x + z));
let expanded = expr.expand();
// Result: x² + xz + xy + yz

// Expand (x + y + z)^2 (use power expansion)
let expr2 = expr!((x + y + z) ^ 2);
let expanded2 = expr2.expand();
// Result: x² + y² + z² + 2xy + 2xz + 2yz
```

### Nested Products

Expansion handles nested structures:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Expand ((x + 1)(x + 2))(x + 3)
let expr = expr!(((x + 1) * (x + 2)) * (x + 3));
let expanded = expr.expand();
// Result: x³ + 6x² + 11x + 6

// Expand (2(x + 1) + 3)(x - 2)
let expr2 = expr!((2 * (x + 1) + 3) * (x - 2));
let expanded2 = expr2.expand();
// Result: 2x² + x - 10
```

## Power Expansion

### Integer Powers

Expand expressions raised to small integer powers:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Expand (x + 1)^2
let expr1 = expr!((x + 1) ^ 2);
let expanded1 = expr1.expand();
// Result: x² + 2x + 1

// Expand (x + y)^3
let expr2 = expr!((x + y) ^ 3);
let expanded2 = expr2.expand();
// Result: x³ + 3x²y + 3xy² + y³

// Expand (x - 2)^4
let expr3 = expr!((x - 2) ^ 4);
let expanded3 = expr3.expand();
// Result: x⁴ - 8x³ + 24x² - 32x + 16
```

**Current Limitations:**
- Automatically expands powers 0-10
- Powers >10 remain symbolic (performance optimization)
- To expand larger powers, manually apply binomial theorem

### Binomial Theorem Application

For explicit binomial expansion:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use expand::Expand;

let x = symbol!(x);
let y = symbol!(y);

// Expand (x + y)^5 using binomial theorem
let base = expr!(x + y);

// expand_binomial computes: Σ C(n,k) * x^(n-k) * y^k
let expanded = base.expand_binomial(&expr!(x), &expr!(y), 5);

// Result: x⁵ + 5x⁴y + 10x³y² + 10x²y³ + 5xy⁴ + y⁵
```

**Binomial Coefficients:**
- $C(5,0) = 1$
- $C(5,1) = 5$
- $C(5,2) = 10$
- $C(5,3) = 10$
- $C(5,4) = 5$
- $C(5,5) = 1$

## Trigonometric & Logarithmic Expansion

### Trigonometric Identities

Expand trigonometric functions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Expand sin(x + y) using sum formula
// sin(x + y) = sin(x)cos(y) + cos(x)sin(y)
let sin_sum = expr!(sin(x + y));
let expanded = sin_sum.expand();

// Expand cos(x - y) using difference formula
// cos(x - y) = cos(x)cos(y) + sin(x)sin(y)
let cos_diff = expr!(cos(x - y));
let expanded2 = cos_diff.expand();

// Expand tan(x + y) using tangent sum formula
// tan(x + y) = (tan(x) + tan(y)) / (1 - tan(x)tan(y))
let tan_sum = expr!(tan(x + y));
let expanded3 = tan_sum.expand();
```

**Trigonometric Sum/Difference Formulas:**
- $$\sin(a \pm b) = \sin a \cos b \pm \cos a \sin b$$
- $$\cos(a \pm b) = \cos a \cos b \mp \sin a \sin b$$
- $$\tan(a \pm b) = \frac{\tan a \pm \tan b}{1 \mp \tan a \tan b}$$

### Logarithmic Expansion

Expand logarithms of products, quotients, and powers:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let a = symbol!(a);
let b = symbol!(b);
let c = symbol!(c);

// Expand log(ab) = log(a) + log(b)
let log_product = expr!(log(a * b));
let expanded1 = log_product.expand();

// Expand log(a/b) = log(a) - log(b)
let log_quotient = expr!(log(a / b));
let expanded2 = log_quotient.expand();

// Expand log(a^n) = n*log(a)
let log_power = expr!(log(a ^ 3));
let expanded3 = log_power.expand();

// Complex expansion: log((a*b)/c^2) = log(a) + log(b) - 2*log(c)
let complex_log = expr!(log((a * b) / (c ^ 2)));
let expanded4 = complex_log.expand();
```

**Logarithm Laws:**
- $$\log(ab) = \log a + \log b$$
- $$\log(a/b) = \log a - \log b$$
- $$\log(a^n) = n \log a$$

## Noncommutative Expansion (⭐⭐ Intermediate)

For matrices, operators, and quaternions, **order matters**. MathHook correctly preserves order.

### Matrix Expansion

When expanding matrix expressions, $AB \neq BA$ in general:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Create matrix symbols
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Expand (A + B)^2 - preserves noncommutativity
let expr = expr!((A + B) ^ 2);
let expanded = expr.expand();
// Result: A² + AB + BA + B²   (4 terms, NOT A² + 2AB + B²)

// Expand (A + B)(C)
let expr2 = expr!((A + B) * C);
let expanded2 = expr2.expand();
// Result: AC + BC   (order preserved: A*C first, then B*C)

// Expand (A + B)(A - B)
let expr3 = expr!((A + B) * (A - B));
let expanded3 = expr3.expand();
// Result: A² + BA - AB - B²   (4 terms, NOT A² - B²)
```

**Why 4 terms instead of 3?**

For scalars (commutative):
$$(x + y)^2 = x^2 + xy + yx + y^2 = x^2 + 2xy + y^2 \quad \text{(since } xy = yx\text{)}$$

For matrices (noncommutative):
$$(A + B)^2 = A^2 + AB + BA + B^2 \quad \text{(cannot combine } AB \text{ and } BA\text{)}$$

### Quantum Operator Expansion

In quantum mechanics, operators don't commute:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Position and momentum operators
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Expand (x + p)^2 - canonical commutation
let expr = expr!((x + p) ^ 2);
let expanded = expr.expand();
// Result: x² + xp + px + p²

// Note: [x,p] = xp - px = iℏ (canonical commutation relation)
// So: (x + p)² = x² + p² + 2xp - iℏ (when commutator applied)
```

**Canonical Commutation Relation:**
$$[\hat{x}, \hat{p}] = \hat{x}\hat{p} - \hat{p}\hat{x} = i\hbar$$

### Quaternion Expansion

Quaternions have noncommutative multiplication:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Quaternion basis: i² = j² = k² = -1, ij = k, ji = -k
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);

// Expand (i + j)^2
let expr = expr!((i + j) ^ 2);
let expanded = expr.expand();
// Result: i² + ij + ji + j²
// Simplified: -1 + k + (-k) + (-1) = -2 (when quaternion rules applied)

// Expand (1 + i)(1 + j)
let expr2 = expr!((1 + i) * (1 + j));
let expanded2 = expr2.expand();
// Result: 1 + j + i + ij = 1 + i + j + k
```

**Quaternion Multiplication Rules:**
- $$i^2 = j^2 = k^2 = -1$$
- $$ij = k, \quad jk = i, \quad ki = j$$
- $$ji = -k, \quad kj = -i, \quad ik = -j$$

## Factoring (Coming Soon)

Factoring is the reverse of expansion: find products that multiply to give an expression.

### Current Status

MathHook currently supports:
- Expansion (multiply out products)
- Simplification (combine like terms, reduce)
- Zero detection (factor out common terms in simplification)

**Planned Factoring Features:**
- Polynomial factoring (factor quadratics, cubics)
- Greatest common divisor (GCD) factoring
- Difference of squares factoring
- Perfect square trinomial factoring
- Grouping method

### Workarounds

Until full factoring is implemented:

1. **Use solving for roots:**
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   // Find roots of x² - 5x + 6 = 0
   let roots = equation.solve(&x);
   // roots = [2, 3]
   // Therefore: x² - 5x + 6 = (x - 2)(x - 3)
   ```

2. **Manual factoring:**
   ```rust
   // Manually construct factored form
   let factored = expr!((x - 2) * (x - 3));
   let expanded = factored.expand();
   // Verify: expanded = x² - 5x + 6
   ```

## Real-World Applications

### 1. Circuit Analysis (Impedance Expansion)

Expand complex impedances:

```rust
use mathhook::{expr, symbol};

let R = symbol!(R);
let L = symbol!(L);
let C = symbol!(C);
let omega = symbol!(omega);

// Total impedance: Z = R + jωL + 1/(jωC)
// Expand to separate real and imaginary parts
let impedance = expr!(R + (omega * L) + 1 / (omega * C));
let expanded = impedance.expand();

// For frequency response analysis
```

### 2. Signal Processing (Transfer Functions)

Expand transfer functions for frequency analysis:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let s = symbol!(s);
let a = symbol!(a);
let b = symbol!(b);

// Transfer function: H(s) = (s + a)(s + b) / (s² + cs + d)
let numerator = expr!((s + a) * (s + b));
let expanded_num = numerator.expand();
// Result: s² + (a+b)s + ab

// Identify coefficients for Bode plot, pole-zero analysis
```

### 3. Physics (Kinetic Energy with Combined Mass)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let m1 = symbol!(m1);
let m2 = symbol!(m2);
let v = symbol!(v);

// Kinetic energy of combined system: KE = ½(m₁ + m₂)v²
let ke = expr!((1 / 2) * (m1 + m2) * (v ^ 2));
let expanded = ke.expand();
// Result: ½m₁v² + ½m₂v² (sum of individual kinetic energies)
```

### 4. Financial Mathematics (Compound Interest)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let P = symbol!(P); // Principal
let r = symbol!(r); // Interest rate
let n = symbol!(n); // Periods

// Expand (1 + r)ⁿ for binomial approximation
let growth = expr!((1 + r) ^ 3);
let expanded = growth.expand();
// Result: 1 + 3r + 3r² + r³

// For small r, approximate: (1 + r)³ ≈ 1 + 3r (drop higher-order terms)
```

## Common Patterns (Cookbook)

### Pattern 1: Expand Before Differentiating

Expansion often simplifies differentiation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use derivatives::Derivative;

let x = symbol!(x);

// Method 1: Differentiate directly (uses product rule)
let expr = expr!((x + 1) * (x - 1));
let derivative1 = expr.derivative(x.clone());

// Method 2: Expand first, then differentiate (simpler)
let expanded = expr.expand(); // x² - 1
let derivative2 = expanded.derivative(x.clone()); // 2x

// Both give same result, but Method 2 is often faster
```

### Pattern 2: Expand Before Integrating

Polynomials are easier to integrate:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// Integrate (x + 1)(x - 1) dx
let expr = expr!((x + 1) * (x - 1));
let expanded = expr.expand(); // x² - 1

// Integrate term-by-term: ∫(x² - 1)dx = x³/3 - x + C
let integral = expanded.integrate(&x);
```

### Pattern 3: Expand for Coefficient Extraction

Extract coefficients after expansion:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Expand to identify coefficients
let expr = expr!((x + 2) * (x + 3));
let expanded = expr.expand(); // x² + 5x + 6

// Coefficients: [6, 5, 1] for constant, x, x² terms
// Useful for polynomial solvers, curve fitting
```

### Pattern 4: Expand Logarithms for Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let a = symbol!(a);
let b = symbol!(b);

// Simplify: d/dx[log(x(x+1))] by expanding first
let log_expr = expr!(log(a * (a + 1)));
let expanded_log = log_expr.expand(); // log(a) + log(a + 1)

// Now easier to differentiate: d/dx[log(a)] + d/dx[log(a+1)]
```

## Common Pitfalls

### Pitfall 1: Forgetting Noncommutativity

❌ **WRONG - Assuming commutativity for matrices:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

let expr = expr!((A + B) ^ 2);
let expanded = expr.expand();

// WRONG assumption: A² + 2AB + B²
// CORRECT result: A² + AB + BA + B²
```

✅ **CORRECT - Respect noncommutative algebra:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For matrices, (A + B)² has 4 terms:
// (A + B)(A + B) = A² + AB + BA + B²
// where AB ≠ BA in general
```

### Pitfall 2: Expanding Too Early

❌ **WRONG - Losing structure:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Factor form reveals zeros at x = 2, 3
let factored = expr!((x - 2) * (x - 3));

// Expanding loses this information
let expanded = factored.expand(); // x² - 5x + 6

// Now harder to see zeros
```

✅ **CORRECT - Expand only when needed:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Keep factored for root analysis
// Expand only for coefficient extraction or polynomial operations
```

### Pitfall 3: Symbolic vs Numeric Expansion

❌ **WRONG - Expecting numeric evaluation:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

let expr = expr!((x + 1) ^ 10);
let expanded = expr.expand();

// Result is symbolic: x¹⁰ + 10x⁹ + 45x⁸ + ...
// NOT a number (x is symbolic)
```

✅ **CORRECT - Use substitution for numeric values:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expanded = expr.expand();
// Substitution would be used here (see substitution.md)
// Then evaluate to: 3¹⁰ = 59049
```

## Performance Considerations

### When Expansion is Expensive

**Expansion cost grows with:**
1. **Power:** $(a + b)^n$ generates $O(2^n)$ terms for noncommutative, $O(n)$ for commutative
2. **Nesting depth:** $((a+b)(c+d))((e+f)(g+h))$ requires recursive expansion
3. **Number of terms:** $(a + b + c + d)^2$ generates more terms than $(a + b)^2$

**Optimization Strategies:**

1. **Use power limits:**
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   // MathHook automatically limits expansion for powers >10
   let expr = expr!((x + 1) ^ 100);
   // Stays symbolic (performance protection)
   ```

2. **Simplify after expansion:**
   ```rust
   let expanded = expr.expand();
   let simplified = expanded.simplify();
   // Combine like terms, reduce complexity
   ```

### Memory Usage

Large expansions can consume memory:

```rust
// (x + y)^10 generates many terms for noncommutative
// Each term needs Expression allocation

// For large powers, consider:
// 1. Streaming evaluation (compute coefficients on-demand)
// 2. Sparse representation (store only non-zero coefficients)
// 3. Symbolic powers (keep (x+y)^10 symbolic)
```

## API Reference

### Methods

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
impl Expression {
    /// Expand expression by distributing multiplication
    pub fn expand(&self) -> Self;

    /// Expand binomial (x + y)^n using binomial theorem
    pub fn expand_binomial(&self, x: &Expression, y: &Expression, n: u32) -> Expression;
}
```

### Expand Trait

```rust
pub trait Expand {
    /// Expand the expression
    fn expand(&self) -> Self;
}

impl Expand for Expression { /* ... */ }
```

## See Also

- **[Simplification](simplification.md)** - Combine like terms after expansion
- **[Differentiation](differentiation.md)** - Often easier after expansion
- **[Integration](integration.md)** - Polynomials integrate term-by-term
- **[Solving Equations](solving.md)** - Extract coefficients from expanded form
- **[Noncommutative Algebra](../advanced/noncommutative-algebra.md)** - Matrix/operator expansion rules
- **External:** [Binomial Theorem](https://en.wikipedia.org/wiki/Binomial_theorem) (Wikipedia)
