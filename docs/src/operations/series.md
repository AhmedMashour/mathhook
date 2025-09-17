# Series Expansions

> 📍 **You are here:** Operations > Series
>
> **Related Topics:** [Limits](limits.md) | [Differentiation](differentiation.md) | [Integration](integration.md)
>
> **Skill Level:** ⭐ Beginner (Maclaurin) | ⭐⭐ Intermediate (Taylor) | ⭐⭐⭐ Advanced (Laurent, Fourier)

Expand functions as infinite series for numerical approximation and analysis.

## Quick Start (⭐ Start here if you're new)

Expand a function as Taylor series in 3 lines:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// Taylor series for exp(x) at x=0: 1 + x + x²/2 + x³/6 + ...
let f = expr!(exp(x));
let series = f.taylor_series(&x, &expr!(0), 5);
// Result: 1 + x + x²/2! + x³/3! + x⁴/4! + x⁵/5!

println!("{}", series);
```

## Table of Contents

- [Understanding Series](#understanding-series)
- [Maclaurin Series (⭐ Beginner)](#maclaurin-series--beginner)
- [Taylor Series (⭐⭐ Intermediate)](#taylor-series--intermediate)
- [Laurent Series (⭐⭐⭐ Advanced)](#laurent-series--advanced)
- [Fourier Series](#fourier-series)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)

## Understanding Series

### What are Series? (Plain English)

A **series** represents a function as an infinite sum of simpler terms (usually powers).

**Examples:**
- `exp(x) = 1 + x + x²/2! + x³/3! + ...` (exponential function)
- `sin(x) = x - x³/3! + x⁵/5! - ...` (sine function)
- `1/(1-x) = 1 + x + x² + x³ + ...` (geometric series)

### Mathematical Background

**Taylor's Theorem:**

If $f(x)$ is infinitely differentiable at $x = a$, then:

$$f(x) = \sum_{n=0}^{\infty} \frac{f^{(n)}(a)}{n!} (x - a)^n$$

Expanded form:

$$f(x) = f(a) + f'(a)(x-a) + \frac{f''(a)}{2!}(x-a)^2 + \frac{f'''(a)}{3!}(x-a)^3 + \cdots$$

**Maclaurin Series (Special Case: a = 0):**

$$f(x) = \sum_{n=0}^{\infty} \frac{f^{(n)}(0)}{n!} x^n$$

**Convergence:**

A series converges if:

$$\lim_{N \to \infty} \left| f(x) - \sum_{n=0}^{N} \frac{f^{(n)}(a)}{n!} (x-a)^n \right| = 0$$

**Radius of Convergence ($R$):**

The series converges for $|x - a| < R$ and may diverge for $|x - a| > R$.

**Reference:** Stewart, *Calculus* 8th ed., Chapter 11 (Infinite Sequences and Series)

### When to Use Series

**Use series for:**
1. **Numerical approximation:** Approximate transcendental functions
2. **Limit evaluation:** Use series to resolve indeterminate forms
3. **Integration:** Integrate functions without closed-form antiderivatives
4. **Differential equations:** Series solutions for ODEs
5. **Signal analysis:** Fourier series for periodic functions

**Don't use series when:**
- Closed-form expression is available and simpler
- Series converges slowly (poor truncation error)
- Outside radius of convergence (series diverges)

## Maclaurin Series (⭐ Beginner)

### Common Maclaurin Series

Standard functions at `x = 0`:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// exp(x) = 1 + x + x²/2! + x³/3! + ...
let exp_series = expr!(exp(x)).taylor_series(&x, &expr!(0), 5);
// Result: 1 + x + x²/2 + x³/6 + x⁴/24 + x⁵/120

// sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 7);
// Result: x - x³/6 + x⁵/120 - x⁷/5040

// cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
let cos_series = expr!(cos(x)).taylor_series(&x, &expr!(0), 6);
// Result: 1 - x²/2 + x⁴/24 - x⁶/720

// ln(1+x) = x - x²/2 + x³/3 - x⁴/4 + ... (|x| < 1)
let log_series = expr!(log(1 + x)).taylor_series(&x, &expr!(0), 5);
// Result: x - x²/2 + x³/3 - x⁴/4 + x⁵/5

// (1+x)^n = 1 + nx + n(n-1)x²/2! + ... (binomial series)
let binomial = expr!((1 + x) ^ 3);
let binomial_series = binomial.taylor_series(&x, &expr!(0), 4);
// Result: 1 + 3x + 3x² + x³
```

### Geometric Series

The simplest series:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// 1/(1-x) = 1 + x + x² + x³ + ... (|x| < 1)
let geometric = expr!(1 / (1 - x));
let series = geometric.taylor_series(&x, &expr!(0), 10);
// Result: 1 + x + x² + x³ + ... + x¹⁰

// Application: Sum of geometric series
// S = a + ar + ar² + ar³ + ... = a/(1-r) for |r| < 1
let a = symbol!(a);
let r = symbol!(r);
let sum = expr!(a / (1 - r));
// Represents: a + ar + ar² + ar³ + ...
```

### Rational Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// 1/(1+x²) = 1 - x² + x⁴ - x⁶ + ... (|x| < 1)
let series1 = expr!(1 / (1 + (x ^ 2))).taylor_series(&x, &expr!(0), 10);

// arctan(x) = ∫ 1/(1+x²) dx = x - x³/3 + x⁵/5 - x⁷/7 + ...
let arctan_series = expr!(atan(x)).taylor_series(&x, &expr!(0), 7);
// Result: x - x³/3 + x⁵/5 - x⁷/7
```

## Taylor Series (⭐⭐ Intermediate)

### Taylor Series at Arbitrary Points

Expand around any point `a`:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// sin(x) at x = π/2:
// sin(x) = 1 - (x-π/2)²/2! + (x-π/2)⁴/4! - ...
let sin_at_pi_2 = expr!(sin(x)).taylor_series(&x, &Expression::pi_over_2(), 5);

// exp(x) at x = 1:
// exp(x) = e + e(x-1) + e(x-1)²/2! + e(x-1)³/3! + ...
let exp_at_1 = expr!(exp(x)).taylor_series(&x, &expr!(1), 5);

// ln(x) at x = 1:
// ln(x) = (x-1) - (x-1)²/2 + (x-1)³/3 - (x-1)⁴/4 + ...
let log_at_1 = expr!(log(x)).taylor_series(&x, &expr!(1), 5);
// Result: (x-1) - (x-1)²/2 + (x-1)³/3 - (x-1)⁴/4 + (x-1)⁵/5
```

### Radius of Convergence

Determine where series converges:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// 1/(1-x) has radius R = 1 (converges for |x| < 1)
let geometric = expr!(1 / (1 - x));
// Converges: x = 0.5 (inside radius)
// Diverges: x = 2 (outside radius)

// exp(x) has radius R = ∞ (converges everywhere)
let exponential = expr!(exp(x));
// Converges for all x

// ln(1+x) has radius R = 1 (converges for |x| < 1)
let logarithm = expr!(log(1 + x));
// Converges: x = 0.5
// Diverges: x = 2
```

### Composition and Operations

Combine series:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// exp(sin(x)) = series(exp(series(sin(x))))
// 1. Compute sin(x) series
let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 5);
// 2. Substitute into exp series
let composed = expr!(exp(sin_series)).taylor_series(&x, &expr!(0), 5);

// exp(x)·cos(x) = product of series
let exp_series = expr!(exp(x)).taylor_series(&x, &expr!(0), 5);
let cos_series = expr!(cos(x)).taylor_series(&x, &expr!(0), 5);
let product = expr!(exp_series * cos_series).taylor_series(&x, &expr!(0), 5);
```

## Laurent Series (⭐⭐⭐ Advanced)

### Series with Negative Powers

For functions with singularities:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// 1/x near x = 0 (pole of order 1)
// Laurent series: ... + a₋₂x⁻² + a₋₁x⁻¹ + a₀ + a₁x + a₂x² + ...
let pole = expr!(1 / x);
let laurent = pole.laurent_series(&x, &expr!(0), -1, 5);
// Result: x⁻¹ (principal part only)

// exp(1/x) at x = 0:
// exp(1/x) = 1 + 1/x + 1/(2!x²) + 1/(3!x³) + ...
let exp_pole = expr!(exp(1 / x));
let laurent2 = exp_pole.laurent_series(&x, &expr!(0), -10, 0);
// Result: 1 + x⁻¹ + x⁻²/2 + x⁻³/6 + ... + x⁻¹⁰/3628800

// sin(x)/x at x = 0 (removable singularity)
// sin(x)/x = 1 - x²/3! + x⁴/5! - ...
let sinc = expr!(sin(x) / x);
let laurent3 = sinc.laurent_series(&x, &expr!(0), 0, 5);
// Result: 1 - x²/6 + x⁴/120 (no negative powers)
```

### Residue Theorem Application

Extract coefficient of `x⁻¹` (residue):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// f(x) = (x² + 1)/(x(x-1)(x-2))
// Laurent series at x = 0
let f = expr!((x ^ 2 + 1) / (x * (x - 1) * (x - 2)));
let laurent = f.laurent_series(&x, &expr!(0), -1, 5);

// Residue (coefficient of x⁻¹) used in contour integration
let residue = laurent.coefficient(-1);
```

## Fourier Series

> **Note:** Fourier series functionality is planned for a future release. The examples below show the intended API design.

### Periodic Function Expansion

Represent periodic functions:

```rust,ignore
// Planned API - not yet implemented

// Square wave: f(x) = 1 for 0 < x < π, f(x) = -1 for π < x < 2π
// Fourier series: (4/π)(sin(x) + sin(3x)/3 + sin(5x)/5 + ...)
let square_wave = FourierSeries::new()
    .period(2.0 * std::f64::consts::PI)
    .terms(10);

// Sawtooth wave: f(x) = x for -π < x < π
// Fourier series: (2/π)(sin(x) - sin(2x)/2 + sin(3x)/3 - ...)
let sawtooth = FourierSeries::sawtooth().terms(10);
```

### Fourier Coefficients

Compute coefficients:

```rust,ignore
// Planned API - not yet implemented

let x = symbol!(x);

// f(x) = x on [-π, π]
let f = expr!(x);

// Fourier coefficients:
// a₀ = (1/π) ∫₋ᵨᵨ f(x) dx
// aₙ = (1/π) ∫₋ᵨᵨ f(x)·cos(nx) dx
// bₙ = (1/π) ∫₋ᵨᵨ f(x)·sin(nx) dx
let fourier = FourierSeries::from_function(&f, &x)
    .interval(-std::f64::consts::PI, std::f64::consts::PI)
    .compute_coefficients(10);
```

## Real-World Applications

### 1. Numerical Approximation (Calculator Functions)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// Calculate sin(0.1) using 5 terms:
// sin(x) ≈ x - x³/6 + x⁵/120
let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 5);
let approximation = sin_series.substitute(&x, &expr!(0.1));
// Result: 0.09983341664... (accurate to 10⁻⁹)

// Compare with exact: sin(0.1) = 0.09983341664682815
```

### 2. Physics (Small Angle Approximation)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let theta = symbol!(theta);

// For small angles: sin(θ) ≈ θ, cos(θ) ≈ 1 - θ²/2
let sin_approx = expr!(sin(theta)).taylor_series(&theta, &expr!(0), 1);
// Result: θ (first-order approximation)

let cos_approx = expr!(cos(theta)).taylor_series(&theta, &expr!(0), 2);
// Result: 1 - θ²/2 (second-order approximation)

// Simple pendulum: θ''(t) + (g/L)·sin(θ) = 0
// Small angle: θ''(t) + (g/L)·θ ≈ 0 (linear ODE)
```

### 3. Signal Processing (Fourier Analysis)

> **Note:** Fourier series for signal processing is planned for a future release.

```rust,ignore
// Planned API - not yet implemented

let t = symbol!(t);

// Audio signal: periodic waveform
// Fourier series decomposes into frequency components
let signal = FourierSeries::from_samples(&samples)
    .sample_rate(44100.0)
    .compute_fft();

// Extract fundamental frequency and harmonics
let fundamental = signal.coefficient(1);
let second_harmonic = signal.coefficient(2);
```

### 4. Financial Mathematics (Interest Approximation)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let r = symbol!(r);

// Continuous compounding: e^r - 1
// For small r: e^r - 1 ≈ r + r²/2 + r³/6
let growth = expr!(exp(r) - 1);
let series = growth.taylor_series(&r, &expr!(0), 3);
// Result: r + r²/2 + r³/6

// For r = 0.05 (5% interest):
// e^0.05 - 1 ≈ 0.05 + 0.00125 + 0.0000208... ≈ 0.0512708...
```

## Common Patterns (Cookbook)

### Pattern 1: Series for Integration

Integrate using series when antiderivative is unknown:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;
use integrals::Integration;

let x = symbol!(x);

// Integrate exp(-x²) (no closed form)
// 1. Expand exp(-x²) as series
let integrand = expr!(exp(-(x ^ 2)));
let series = integrand.taylor_series(&x, &expr!(0), 10);
// Result: 1 - x² + x⁴/2 - x⁶/6 + ...

// 2. Integrate term-by-term
let integral = series.integrate(&x);
// Result: x - x³/3 + x⁵/10 - x⁷/42 + ...

// Useful for error function: erf(x) = (2/√π) ∫₀ˣ exp(-t²) dt
```

### Pattern 2: Series for Limit Evaluation

Use series to resolve indeterminate forms:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;
let x = symbol!(x);

// Limit: lim(x→0) (sin(x) - x)/x³
// Direct substitution gives 0/0
// Use series: sin(x) = x - x³/6 + x⁵/120 - ...
// So: (sin(x) - x)/x³ = -1/6 + x²/120 - ...
// Therefore: lim(x→0) = -1/6

let numerator = expr!(sin(x) - x);
let series = numerator.taylor_series(&x, &expr!(0), 5);
let limit = expr!(series / (x ^ 3)).limit(&x, &expr!(0));
// Result: -1/6
```

### Pattern 3: Error Estimation

Estimate truncation error:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// Taylor remainder theorem:
// |Rₙ(x)| ≤ M·|x-a|^(n+1)/(n+1)!
// where M = max|f^(n+1)(t)| for t in [a, x]

// Example: exp(1) ≈ 1 + 1 + 1/2 + 1/6 + ... (n terms)
let exp_series = expr!(exp(x)).taylor_series(&x, &expr!(0), 5);
let approximation = exp_series.substitute(&x, &expr!(1));
// Result: 1 + 1 + 1/2 + 1/6 + 1/24 + 1/120 = 2.71666...

// Error: |R₅(1)| ≤ e·1^6/6! ≈ 0.00378
// Actual: e - 2.71666... ≈ 0.00148 (tighter bound)
```

## Common Pitfalls

### Pitfall 1: Using Series Outside Radius of Convergence

❌ **WRONG - Using series where it diverges:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// ln(1+x) has radius R = 1 (converges for |x| < 1)
let log_series = expr!(log(1 + x)).taylor_series(&x, &expr!(0), 100);

// WRONG: Using at x = 2 (outside radius)
let wrong = log_series.substitute(&x, &expr!(2));
// Series diverges! Result is meaningless.
```

✅ **CORRECT - Check radius of convergence:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For x = 2, use different expansion point
// ln(3) = ln(1 + 2) → expand at x = 1 instead:
// ln(x) at x = 1: (x-1) - (x-1)²/2 + (x-1)³/3 - ...
let log_at_2 = expr!(log(x)).taylor_series(&x, &expr!(1), 10);
let correct = log_at_2.substitute(&x, &expr!(2));
// Now converges: ln(2) using (2-1) = 1 < radius
```

### Pitfall 2: Insufficient Terms

❌ **WRONG - Too few terms for accuracy:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// sin(5) with only 2 terms:
let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 2);
let approx = sin_series.substitute(&x, &expr!(5));
// Result: 5 - 5³/6 = -15.833... (very wrong!)
// Actual: sin(5) = -0.9589...
```

✅ **CORRECT - Use enough terms for convergence:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// sin(5) needs more terms (x = 5 is far from expansion point)
let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 15);
let approx = sin_series.substitute(&x, &expr!(5));
// Result: -0.9589... (accurate)

// Better: Reduce argument using sin(5) = sin(5 - 2π)
let reduced = 5.0 - 2.0 * std::f64::consts::PI;
// Now x ≈ -1.28, closer to 0, needs fewer terms
```

### Pitfall 3: Forgetting to Simplify

❌ **WRONG - Series with unsimplified terms:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// (1 + x)³ expanded as series
let series = expr!((1 + x) ^ 3).taylor_series(&x, &expr!(0), 5);
// Result: 1 + 3x + 3x² + x³ + 0·x⁴ + 0·x⁵

// WRONG: Keeping zero terms
```

✅ **CORRECT - Simplify to remove zero terms:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let series = expr!((1 + x) ^ 3).taylor_series(&x, &expr!(0), 5);
let simplified = series.simplify();
// Result: 1 + 3x + 3x² + x³ (cleaner)
```

### Pitfall 4: Symbolic vs Numerical Series

❌ **WRONG - Expecting symbolic series to give numbers:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Symbolic series
let series = expr!(exp(x)).taylor_series(&x, &expr!(0), 5);
// Result: 1 + x + x²/2 + x³/6 + x⁴/24 + x⁵/120 (symbolic)

// WRONG: Expecting numeric value without substitution
```

✅ **CORRECT - Substitute value for numerical result:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let series = expr!(exp(x)).taylor_series(&x, &expr!(0), 5);
let numerical = series.substitute(&x, &expr!(2));
// Result: 1 + 2 + 4/2 + 8/6 + 16/24 + 32/120 = 7.266... (numeric)
```

## Performance Considerations

### When Series are Expensive

**Series computation cost depends on:**
1. **Number of terms:** Each term requires differentiation + evaluation
2. **Derivative complexity:** Higher derivatives may be large expressions
3. **Symbolic vs numerical:** Symbolic series manipulate expressions, numerical evaluates

**Optimization Strategies:**

1. **Cache series expansions:**
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   // Compute once, reuse many times
   let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 10);

   // Use for multiple substitutions
   let val1 = sin_series.substitute(&x, &expr!(0.1));
   let val2 = sin_series.substitute(&x, &expr!(0.2));
   ```

2. **Use pre-computed series for common functions:**
   ```rust
   // MathHook caches common series (exp, sin, cos, log)
   // No need to recompute every time
   ```

3. **Reduce argument before series expansion:**
   ```rust
   // For sin(x) at large x, use sin(x) = sin(x mod 2π)
   // Closer to expansion point → fewer terms needed
   ```

## API Reference

### Methods

```rust
impl Expression {
    /// Compute Taylor series at expansion point
    pub fn series(&self, var: &Symbol, point: &Expression, order: usize) -> Expression;

    /// Compute Laurent series (includes negative powers)
    pub fn laurent_series(
        &self,
        var: &Symbol,
        point: &Expression,
        min_power: isize,
        max_power: usize
    ) -> Expression;
}
```

### Series Trait

```rust
pub trait SeriesExpansion {
    /// Compute Taylor series
    fn taylor_series(&self, variable: &Symbol, point: &Expression, order: u32) -> Expression;

    /// Compute Laurent series (includes negative powers)
    fn laurent_series(&self, variable: &Symbol, point: &Expression, order: u32) -> Expression;

    /// Compute Maclaurin series (Taylor around 0)
    fn maclaurin_series(&self, variable: &Symbol, order: u32) -> Expression;

    /// Compute power series coefficients
    fn power_series_coefficients(
        &self,
        variable: &Symbol,
        point: &Expression,
        order: u32,
    ) -> Vec<Expression>;
}

impl SeriesExpansion for Expression { /* ... */ }
```

### Fourier Series (Planned)

> **Note:** Fourier series struct is planned for a future release.

```rust,ignore
// Planned API
pub struct FourierSeries {
    /// Construct Fourier series
    pub fn new() -> Self;

    /// Set period
    pub fn period(self, period: f64) -> Self;

    /// Set number of terms
    pub fn terms(self, n: usize) -> Self;

    /// Compute coefficients
    pub fn compute_coefficients(&self, n: usize) -> Vec<(f64, f64)>;
}
```

## See Also

- **[Limits](limits.md)** - Series convergence uses limits
- **[Differentiation](differentiation.md)** - Taylor series requires derivatives
- **[Integration](integration.md)** - Integrate series term-by-term
- **[Approximation](../advanced/approximation.md)** - Numerical approximation methods
- **External:** [Taylor Series](https://en.wikipedia.org/wiki/Taylor_series) (Wikipedia)
- **External:** [Fourier Series](https://en.wikipedia.org/wiki/Fourier_series) (Wikipedia)
