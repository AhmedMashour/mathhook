# Limits

> 📍 **You are here:** Operations > Limits
>
> **Related Topics:** [Differentiation](differentiation.md) | [Series](series.md) | [Integration](integration.md)
>
> **Skill Level:** ⭐ Beginner (finite limits) | ⭐⭐ Intermediate (infinity) | ⭐⭐⭐ Advanced (L'Hôpital's rule)

Compute limits of expressions as variables approach values, infinity, or points of discontinuity.

## Quick Start (⭐ Start here if you're new)

Compute a simple limit in 3 lines:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→0) sin(x)/x = 1
let expr = expr!(sin(x) / x);
let limit = expr.limit(&x, &expr!(0));
// Result: 1

println!("{}", limit); // 1
```

## Table of Contents

- [Understanding Limits](#understanding-limits)
- [Basic Limits (⭐ Beginner)](#basic-limits--beginner)
- [One-Sided Limits](#one-sided-limits)
- [Limits at Infinity (⭐⭐ Intermediate)](#limits-at-infinity--intermediate)
- [Indeterminate Forms (⭐⭐⭐ Advanced)](#indeterminate-forms--advanced)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)

## Understanding Limits

### What are Limits? (Plain English)

A **limit** describes the value a function approaches as the input approaches a particular point, without necessarily reaching that point.

**Examples:**
- `lim(x→0) sin(x)/x = 1` (approaches 1 as x approaches 0, even though 0/0 is undefined)
- `lim(x→∞) 1/x = 0` (approaches 0 as x grows infinitely large)
- `lim(x→2) x² = 4` (approaches 4 as x approaches 2)

### Mathematical Background

**Epsilon-Delta Definition (ε-δ):**

$$\lim_{x \to a} f(x) = L$$

means: For every $\varepsilon > 0$, there exists $\delta > 0$ such that:

$$0 < |x - a| < \delta \implies |f(x) - L| < \varepsilon$$

**Intuitive Meaning:** We can make $f(x)$ arbitrarily close to $L$ by making $x$ sufficiently close to $a$.

**Limit Laws:**

1. **Sum/Difference:** $\lim_{x \to a} [f(x) \pm g(x)] = \lim_{x \to a} f(x) \pm \lim_{x \to a} g(x)$
2. **Product:** $\lim_{x \to a} [f(x) \cdot g(x)] = \lim_{x \to a} f(x) \cdot \lim_{x \to a} g(x)$
3. **Quotient:** $\lim_{x \to a} \frac{f(x)}{g(x)} = \frac{\lim_{x \to a} f(x)}{\lim_{x \to a} g(x)}$ (if denominator $\neq 0$)
4. **Constant Multiple:** $\lim_{x \to a} [c \cdot f(x)] = c \cdot \lim_{x \to a} f(x)$
5. **Power:** $\lim_{x \to a} [f(x)]^n = [\lim_{x \to a} f(x)]^n$

**Reference:** Stewart, *Calculus* 8th ed., Chapter 2 (Limits and Derivatives)

### When to Use Limits

**Use limits for:**
1. **Continuity testing:** Check if function is continuous at a point
2. **Derivative definition:** $f'(a) = \lim_{h \to 0} \frac{f(a+h) - f(a)}{h}$
3. **Asymptotic analysis:** Behavior as $x \to \infty$ or $x \to -\infty$
4. **Indeterminate forms:** Resolve $\frac{0}{0}$, $\frac{\infty}{\infty}$, etc.
5. **Convergence testing:** Determine if sequences or series converge

**Don't use limits when:**
- Simple substitution works (function is continuous)
- Exact evaluation is needed (limits give behavior, not exact values at discontinuities)

## Basic Limits (⭐ Beginner)

### Direct Substitution

For continuous functions, substitute directly:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→2) x² = 4
let expr1 = expr!(x ^ 2);
let limit1 = expr1.limit(&x, &expr!(2));
// Result: 4

// Limit: lim(x→π) sin(x) = 0
let expr2 = expr!(sin(x));
let limit2 = expr2.limit(&x, &Expression::pi());
// Result: 0

// Limit: lim(x→1) (3x + 2) = 5
let expr3 = expr!(3 * x + 2);
let limit3 = expr3.limit(&x, &expr!(1));
// Result: 5
```

### Factorization Method

Factor to cancel common terms:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→2) (x² - 4)/(x - 2)
// Factor numerator: (x - 2)(x + 2)/(x - 2) = x + 2
let expr = expr!((x ^ 2 - 4) / (x - 2));
let limit = expr.limit(&x, &expr!(2));
// Result: 4 (substitute x = 2 into x + 2)

// Limit: lim(x→3) (x² - 9)/(x - 3)
let expr2 = expr!((x ^ 2 - 9) / (x - 3));
let limit2 = expr2.limit(&x, &expr!(3));
// Result: 6
```

### Rationalization

Multiply by conjugate:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let h = symbol!(h);

// Limit: lim(h→0) (√(x+h) - √x)/h
// Multiply by conjugate: (√(x+h) + √x)/(√(x+h) + √x)
// Result: 1/(2√x)
let expr = expr!((sqrt(x + h) - sqrt(x)) / h);
let limit = expr.limit(&h, &expr!(0));
// Result: 1/(2√x)
```

## One-Sided Limits

### Left-Hand Limit (x → a⁻)

Approach from the left (values less than a):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→0⁻) 1/x = -∞
let expr = expr!(1 / x);
let left_limit = expr.limit_with_direction(&x, &expr!(0), Direction::Left);
// Result: -∞

// Limit: lim(x→2⁻) floor(x) = 1
let expr2 = expr!(floor(x));
let left_limit2 = expr2.limit_with_direction(&x, &expr!(2), Direction::Left);
// Result: 1
```

### Right-Hand Limit (x → a⁺)

Approach from the right (values greater than a):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→0⁺) 1/x = +∞
let expr = expr!(1 / x);
let right_limit = expr.limit_with_direction(&x, &expr!(0), Direction::Right);
// Result: +∞

// Limit: lim(x→2⁺) floor(x) = 2
let expr2 = expr!(floor(x));
let right_limit2 = expr2.limit_with_direction(&x, &expr!(2), Direction::Right);
// Result: 2
```

### Two-Sided Limit

Limit exists only if left and right limits agree:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→0) |x|/x does not exist
// Left: lim(x→0⁻) |x|/x = -1
// Right: lim(x→0⁺) |x|/x = +1
let expr = expr!(abs(x) / x);

let left = expr.limit_with_direction(&x, &expr!(0), Direction::Left);
// Result: -1

let right = expr.limit_with_direction(&x, &expr!(0), Direction::Right);
// Result: +1

// Two-sided limit does not exist (left ≠ right)
let two_sided = expr.limit(&x, &expr!(0));
// Result: Undefined or error
```

## Limits at Infinity (⭐⭐ Intermediate)

### Polynomial Limits

Dominated by highest-degree term:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) (3x² + 2x + 1) = ∞
let expr1 = expr!(3 * (x ^ 2) + 2 * x + 1);
let limit1 = expr1.limit(&x, &Expression::infinity());
// Result: ∞

// Limit: lim(x→∞) (-2x³ + x²) = -∞
let expr2 = expr!(-2 * (x ^ 3) + (x ^ 2));
let limit2 = expr2.limit(&x, &Expression::infinity());
// Result: -∞

// Limit: lim(x→-∞) (x⁴ + 1) = ∞ (even power always positive)
let expr3 = expr!(x ^ 4 + 1);
let limit3 = expr3.limit(&x, &Expression::neg_infinity());
// Result: ∞
```

### Rational Function Limits

Compare degrees of numerator and denominator:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) (2x² + 1)/(x² + 3) = 2
// (degrees equal → ratio of leading coefficients)
let expr1 = expr!((2 * (x ^ 2) + 1) / ((x ^ 2) + 3));
let limit1 = expr1.limit(&x, &Expression::infinity());
// Result: 2

// Limit: lim(x→∞) (x + 1)/(x² + 1) = 0
// (numerator degree < denominator degree → 0)
let expr2 = expr!((x + 1) / ((x ^ 2) + 1));
let limit2 = expr2.limit(&x, &Expression::infinity());
// Result: 0

// Limit: lim(x→∞) (x³)/(x² + 1) = ∞
// (numerator degree > denominator degree → ∞)
let expr3 = expr!((x ^ 3) / ((x ^ 2) + 1));
let limit3 = expr3.limit(&x, &Expression::infinity());
// Result: ∞
```

### Exponential Limits

Exponentials dominate polynomials:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) e^x/x^n = ∞ (exponential > polynomial)
let expr1 = expr!(exp(x) / (x ^ 10));
let limit1 = expr1.limit(&x, &Expression::infinity());
// Result: ∞

// Limit: lim(x→∞) x^n/e^x = 0 (exponential dominates)
let expr2 = expr!((x ^ 10) / exp(x));
let limit2 = expr2.limit(&x, &Expression::infinity());
// Result: 0

// Limit: lim(x→-∞) e^x = 0
let expr3 = expr!(exp(x));
let limit3 = expr3.limit(&x, &Expression::neg_infinity());
// Result: 0
```

## Indeterminate Forms (⭐⭐⭐ Advanced)

### 0/0 Form (L'Hôpital's Rule)

When direct substitution gives 0/0, use L'Hôpital's rule:

$$\lim_{x \to a} \frac{f(x)}{g(x)} = \lim_{x \to a} \frac{f'(x)}{g'(x)}$$

(if the limit on the right exists)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use derivatives::Derivative;

let x = symbol!(x);

// Limit: lim(x→0) sin(x)/x = 1 (0/0 form)
// Apply L'Hôpital: lim(x→0) cos(x)/1 = 1
let expr = expr!(sin(x) / x);
let limit = expr.limit(&x, &expr!(0));
// Result: 1

// Limit: lim(x→0) (1 - cos(x))/x² = 1/2 (0/0 form)
// Apply L'Hôpital twice:
// 1st: lim(x→0) sin(x)/(2x) (still 0/0)
// 2nd: lim(x→0) cos(x)/2 = 1/2
let expr2 = expr!((1 - cos(x)) / (x ^ 2));
let limit2 = expr2.limit(&x, &expr!(0));
// Result: 1/2

// Limit: lim(x→0) (e^x - 1)/x = 1 (0/0 form)
let expr3 = expr!((exp(x) - 1) / x);
let limit3 = expr3.limit(&x, &expr!(0));
// Result: 1
```

### ∞/∞ Form

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) (2x + 1)/(3x + 2) = 2/3 (∞/∞ form)
// Apply L'Hôpital: lim(x→∞) 2/3 = 2/3
let expr = expr!((2 * x + 1) / (3 * x + 2));
let limit = expr.limit(&x, &Expression::infinity());
// Result: 2/3

// Limit: lim(x→∞) ln(x)/x = 0 (∞/∞ form)
// Apply L'Hôpital: lim(x→∞) (1/x)/1 = 0
let expr2 = expr!(log(x) / x);
let limit2 = expr2.limit(&x, &Expression::infinity());
// Result: 0
```

### 0 · ∞ Form

Convert to 0/0 or ∞/∞:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) x·e^(-x) = 0 (∞·0 form)
// Rewrite as: lim(x→∞) x/e^x (∞/∞ form)
// Apply L'Hôpital: lim(x→∞) 1/e^x = 0
let expr = expr!(x * exp(-x));
let limit = expr.limit(&x, &Expression::infinity());
// Result: 0

// Limit: lim(x→0⁺) x·ln(x) = 0 (0·(-∞) form)
// Rewrite as: lim(x→0⁺) ln(x)/(1/x) (-∞/∞ form)
// Apply L'Hôpital: lim(x→0⁺) (1/x)/(-1/x²) = lim(x→0⁺) -x = 0
let expr2 = expr!(x * log(x));
let limit2 = expr2.limit_with_direction(&x, &expr!(0), Direction::Right);
// Result: 0
```

### ∞ - ∞ Form

Combine into single fraction:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→0⁺) (1/x - 1/sin(x)) = 0 (∞ - ∞ form)
// Combine: lim(x→0⁺) (sin(x) - x)/(x·sin(x))
// This is now 0/0 form, apply L'Hôpital
let expr = expr!(1 / x - 1 / sin(x));
let limit = expr.limit_with_direction(&x, &expr!(0), Direction::Right);
// Result: 0
```

### 1^∞, 0^0, ∞^0 Forms

Use logarithms:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) (1 + 1/x)^x = e (1^∞ form)
// Take ln: lim(x→∞) x·ln(1 + 1/x)
// This is ∞·0 form, convert to 0/0 and apply L'Hôpital
let expr = expr!((1 + 1 / x) ^ x);
let limit = expr.limit(&x, &Expression::infinity());
// Result: e

// Limit: lim(x→0⁺) x^x = 1 (0^0 form)
// Take ln: lim(x→0⁺) x·ln(x) = 0
// Therefore: e^0 = 1
let expr2 = expr!(x ^ x);
let limit2 = expr2.limit_with_direction(&x, &expr!(0), Direction::Right);
// Result: 1
```

## Real-World Applications

### 1. Instantaneous Velocity (Physics)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let h = symbol!(h);

// Position: s(t) = -16t² + 64t (feet)
// Instantaneous velocity at t=2: lim(h→0) [s(2+h) - s(2)]/h
let s_t = expr!(-16 * (t ^ 2) + 64 * t);
let s_2_plus_h = s_t.substitute(&t, &expr!(2 + h));
let s_2 = s_t.substitute(&t, &expr!(2));

let velocity_expr = expr!((s_2_plus_h - s_2) / h);
let velocity = velocity_expr.limit(&h, &expr!(0));
// Result: 0 ft/s (at apex of trajectory)
```

### 2. Marginal Cost (Economics)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let h = symbol!(h);

// Cost function: C(x) = 1000 + 50x + 0.1x²
// Marginal cost at x=100: lim(h→0) [C(100+h) - C(100)]/h
let cost = expr!(1000 + 50 * x + 0.1 * (x ^ 2));
let marginal_cost_expr = expr!((
    cost.substitute(&x, &expr!(100 + h)) - cost.substitute(&x, &expr!(100))
) / h);

let marginal_cost = marginal_cost_expr.limit(&h, &expr!(0));
// Result: 70 ($/unit at production level 100)
```

### 3. Asymptotic Analysis (Computer Science)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let n = symbol!(n);

// Algorithm complexity: T(n) = 3n² + 5n + 2
// Growth rate: lim(n→∞) T(n)/n² = 3 (Θ(n²))
let time = expr!(3 * (n ^ 2) + 5 * n + 2);
let growth_rate = expr!(time / (n ^ 2));
let limit = growth_rate.limit(&n, &Expression::infinity());
// Result: 3 (dominated by n² term)
```

### 4. Continuous Compound Interest (Finance)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let n = symbol!(n);
let P = symbol!(P);
let r = symbol!(r);
let t = symbol!(t);

// Limit: lim(n→∞) P(1 + r/n)^(nt) = Pe^(rt)
// (discrete compounding → continuous compounding)
let amount = expr!(P * ((1 + r / n) ^ (n * t)));
let continuous = amount.limit(&n, &Expression::infinity());
// Result: P·e^(rt)
```

## Common Patterns (Cookbook)

### Pattern 1: Standard Limits

Memorize these fundamental limits:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// lim(x→0) sin(x)/x = 1
let limit1 = expr!(sin(x) / x).limit(&x, &expr!(0));

// lim(x→0) (1 - cos(x))/x = 0
let limit2 = expr!((1 - cos(x)) / x).limit(&x, &expr!(0));

// lim(x→0) (e^x - 1)/x = 1
let limit3 = expr!((exp(x) - 1) / x).limit(&x, &expr!(0));

// lim(x→∞) (1 + 1/x)^x = e
let limit4 = expr!((1 + 1 / x) ^ x).limit(&x, &Expression::infinity());
```

### Pattern 2: Squeeze Theorem

For $g(x) \leq f(x) \leq h(x)$ and $\lim g(x) = \lim h(x) = L$, then $\lim f(x) = L$:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→0) x²·sin(1/x) = 0
// Since: -x² ≤ x²·sin(1/x) ≤ x² (because -1 ≤ sin(1/x) ≤ 1)
// And: lim(x→0) -x² = lim(x→0) x² = 0
let expr = expr!((x ^ 2) * sin(1 / x));
let limit = expr.limit(&x, &expr!(0));
// Result: 0
```

### Pattern 3: Series Expansion for Limits

Use Taylor series:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Limit: lim(x→0) (e^x - 1 - x)/x² = 1/2
// Expand e^x ≈ 1 + x + x²/2 + x³/6 + ...
// So: (e^x - 1 - x)/x² ≈ (x²/2 + x³/6 + ...)/x² = 1/2 + x/6 + ...
// Therefore: lim(x→0) = 1/2
let expr = expr!((exp(x) - 1 - x) / (x ^ 2));
let limit = expr.limit(&x, &expr!(0));
// Result: 1/2
```

## Common Pitfalls

### Pitfall 1: Assuming Limit Equals Function Value

❌ **WRONG - Limit must equal function value:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// f(x) = (x² - 1)/(x - 1) is undefined at x = 1
let f = expr!((x ^ 2 - 1) / (x - 1));

// But limit exists:
let limit = f.limit(&x, &expr!(1));
// Result: 2

// WRONG: Assuming f(1) = 2
// CORRECT: f(1) is undefined, but lim(x→1) f(x) = 2
```

✅ **CORRECT - Distinguish limit from function value:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Limit describes approach, not the value at the point
// Function may be undefined, but limit can still exist
```

### Pitfall 2: Ignoring One-Sided Limits

❌ **WRONG - Using two-sided limit when one-sided differ:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// f(x) = |x|/x has different left/right limits at x=0
let expr = expr!(abs(x) / x);

// WRONG: Using two-sided limit
let limit = expr.limit(&x, &expr!(0));
// This may give error or undefined

// CORRECT: Check one-sided limits first
let left = expr.limit_with_direction(&x, &expr!(0), Direction::Left);
// Result: -1
let right = expr.limit_with_direction(&x, &expr!(0), Direction::Right);
// Result: +1
// Two-sided limit does not exist!
```

✅ **CORRECT - Always check one-sided limits at discontinuities:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For piecewise functions, floor/ceiling, absolute value expressions
```

### Pitfall 3: Misapplying L'Hôpital's Rule

❌ **WRONG - Applying L'Hôpital when not indeterminate:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// lim(x→0) (x + 1)/x is NOT 0/0 (it's 1/0 = ∞)
let expr = expr!((x + 1) / x);

// WRONG: Applying L'Hôpital
// This would give: lim(x→0) 1/1 = 1 (incorrect!)

// CORRECT: Direct analysis
let limit = expr.limit(&x, &expr!(0));
// Result: ∞ (limit does not exist, approaches ∞)
```

✅ **CORRECT - Verify indeterminate form first:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Check that direct substitution gives 0/0 or ∞/∞
// before applying L'Hôpital's rule
```

### Pitfall 4: Symbolic vs Numerical Limits

❌ **WRONG - Expecting numeric answer for symbolic limit:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let a = symbol!(a);

// lim(x→a) x² is symbolic
let expr = expr!(x ^ 2);
let limit = expr.limit(&x, &expr!(a));
// Result: a² (symbolic, not numeric)
```

✅ **CORRECT - Substitute numeric values after limit:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Compute symbolic limit first, then substitute
let a = symbol!(a);
let symbolic_limit = expr.limit(&x, &expr!(a));
// Then substitute a = 3 for numeric result
```

## Performance Considerations

### When Limits are Expensive

**Limit computation cost depends on:**
1. **Indeterminate forms:** Require L'Hôpital (derivatives + recursive limits)
2. **Symbolic complexity:** Large expressions slow computation
3. **Multiple applications:** Nested L'Hôpital applications

**Optimization Strategies:**

1. **Simplify first:**
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   let expr = /* complex expression */;
   let simplified = expr.simplify();
   let limit = simplified.limit(&x, &target);
   ```

2. **Use known limits:**
   ```rust
   // Instead of computing lim(x→0) sin(x)/x every time,
   // recognize the standard form and return 1 directly
   ```

3. **Numerical approximation for complex limits:**
   ```rust
   // For very complex symbolic limits, consider numerical approach:
   // Evaluate at x = a - ε and x = a + ε for small ε
   ```

## API Reference

### Methods

```rust
impl Expression {
    /// Compute limit as variable approaches value
    pub fn limit(&self, var: &Symbol, target: &Expression) -> Expression;

    /// Compute one-sided limit
    pub fn limit_with_direction(
        &self,
        var: &Symbol,
        target: &Expression,
        direction: Direction
    ) -> Expression;
}
```

### Limit Trait

```rust
pub trait Limit {
    /// Compute limit
    fn limit(&self, var: &Symbol, target: &Expression) -> Expression;
}

pub enum Direction {
    Left,   // x → a⁻
    Right,  // x → a⁺
}

impl Limit for Expression { /* ... */ }
```

## See Also

- **[Differentiation](differentiation.md)** - Limits define derivatives
- **[Series](series.md)** - Limits determine convergence
- **[Integration](integration.md)** - Improper integrals use limits
- **[Continuity](../advanced/continuity.md)** - Limits test continuity
- **External:** [L'Hôpital's Rule](https://en.wikipedia.org/wiki/L%27H%C3%B4pital%27s_rule) (Wikipedia)
- **External:** [Epsilon-Delta Definition](https://en.wikipedia.org/wiki/(%CE%B5,_%CE%B4)-definition_of_limit) (Wikipedia)
