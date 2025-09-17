# Complex Numbers

> 📍 **You are here:** Advanced > Complex Numbers
>
> **Related Topics:** [Special Functions](special-functions.md) | [Constants](../core/constants.md) | [Assumptions](assumptions.md)
>
> **Skill Level:** ⭐⭐ Intermediate to ⭐⭐⭐ Advanced

Work with complex numbers symbolically and numerically in MathHook.

## Quick Start (⭐⭐ Start here)

Create and manipulate complex numbers:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

// Imaginary unit
let i = Expression::i();

// Complex number: 3 + 4i
let z = expr!(3 + 4*i);

// Operations work naturally
let z_squared = expr!(z * z);  // (3 + 4i)²
let simplified = z_squared.simplify();
println!("{}", simplified);  // Output: -7 + 24i

// Complex functions
let x = symbol!(x);
let complex_exp = expr!(exp(i * x));  // e^(ix) = cos(x) + i*sin(x)
```

## Table of Contents

- [Understanding Complex Numbers](#understanding-complex-numbers)
- [Complex Number Representations](#complex-number-representations)
- [Basic Operations (⭐⭐ Intermediate)](#basic-operations--intermediate)
- [Complex Functions (⭐⭐⭐ Advanced)](#complex-functions--advanced)
- [Branch Cuts and Principal Values](#branch-cuts-and-principal-values)
- [Real vs Complex Domain](#real-vs-complex-domain)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Common Pitfalls](#common-pitfalls)
- [Performance Considerations](#performance-considerations)
- [See Also](#see-also)

## Understanding Complex Numbers

### What Are Complex Numbers? (Plain English)

**Complex numbers** extend the real numbers to include solutions to equations like $x^2 = -1$.

**Standard form:** $z = a + bi$ where:
- $a$ = real part
- $b$ = imaginary part
- $i$ = imaginary unit ($i^2 = -1$)

**Examples:**
- $3 + 4i$ - real part: 3, imaginary part: 4
- $-2i$ - real part: 0, imaginary part: -2
- $5$ - real part: 5, imaginary part: 0 (real number)

### Mathematical Background

**Complex Number Properties:**

1. **Imaginary unit:** $i^2 = -1$

2. **Addition:** $(a + bi) + (c + di) = (a + c) + (b + d)i$

3. **Multiplication:** $(a + bi)(c + di) = (ac - bd) + (ad + bc)i$

4. **Conjugate:** $\overline{z} = a - bi$

5. **Modulus (absolute value):** $|z| = \sqrt{a^2 + b^2}$

6. **Division:**
   $$\frac{a + bi}{c + di} = \frac{(a + bi)(c - di)}{c^2 + d^2} = \frac{(ac + bd) + (bc - ad)i}{c^2 + d^2}$$

**Polar Form:**

Any complex number can be written as:
$$z = r(\cos\theta + i\sin\theta) = re^{i\theta}$$

where:
- $r = |z| = \sqrt{a^2 + b^2}$ (modulus)
- $\theta = \arg(z) = \arctan(b/a)$ (argument)

**Euler's Formula:**
$$e^{i\theta} = \cos\theta + i\sin\theta$$

**Reference:** Complex analysis is fundamental in many areas of mathematics. See Ahlfors, *Complex Analysis* 3rd ed., Chapter 1, or Churchill & Brown, *Complex Variables and Applications*.

### When to Use Complex Numbers

**Use complex numbers for:**
1. **Solving equations:** $x^2 + 1 = 0$ has solutions $x = \pm i$
2. **Electrical engineering:** AC circuits, impedance analysis
3. **Signal processing:** Fourier transforms, frequency analysis
4. **Quantum mechanics:** Wave functions, probability amplitudes
5. **Control theory:** Transfer functions, stability analysis
6. **Fluid dynamics:** Potential flow, conformal mapping

**Don't use complex numbers when:**
- Only real solutions are physically meaningful
- Introducing unnecessary complexity
- Numerical stability is critical (complex arithmetic can amplify errors)

## Complex Number Representations

MathHook supports two forms of complex numbers:

### Symbolic Form: `a + b*i`

**Best for:** Algebraic manipulation, symbolic computation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// Symbolic complex numbers
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 - 2*i);

// Symbolic variables
let x = symbol!(x);
let y = symbol!(y);
let z_symbolic = expr!(x + y*i);  // General complex number
```

**Advantages:**
- Natural algebraic manipulation
- Works with symbolic variables
- Automatic simplification

**Disadvantages:**
- Slightly slower than explicit complex type
- May not be recognized as complex by all operations

### Explicit Form: `Complex(a, b)`

**Best for:** Numerical computation, performance-critical code

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;
use mathhook::core::expression::ComplexData;

// Explicit complex number: 3 + 4i
let z = Expression::Complex(Box::new(ComplexData {
    real: Expression::integer(3),
    imag: Expression::integer(4),
}));

// Or using the helper:
let z = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);
```

**Advantages:**
- Explicit complex type (clear intent)
- Direct access to real and imaginary parts
- Better for numerical code

**Disadvantages:**
- More verbose
- Requires explicit construction

### When to Use Each

| Scenario | Use Symbolic `a + b*i` | Use Explicit `Complex(a, b)` |
|----------|----------------------|--------------------------|
| Algebraic manipulation | ✅ Yes | ❌ No |
| Symbolic variables | ✅ Yes | ❌ No |
| Numerical computation | ⚠️ OK | ✅ Preferred |
| Performance-critical | ❌ No | ✅ Yes |
| Clear intent | ⚠️ OK | ✅ Yes |

## Basic Operations (⭐⭐ Intermediate)

### Addition and Subtraction

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// (3 + 4i) + (1 + 2i) = 4 + 6i
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 + 2*i);
let sum = expr!(z1 + z2);
let result = sum.simplify();
// Result: 4 + 6i

// (3 + 4i) - (1 + 2i) = 2 + 2i
let diff = expr!(z1 - z2);
let result = diff.simplify();
// Result: 2 + 2i
```

### Multiplication

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// (3 + 4i)(1 + 2i)
// = 3 + 6i + 4i + 8i²
// = 3 + 10i + 8(-1)
// = -5 + 10i
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 + 2*i);
let product = expr!(z1 * z2);
let result = product.simplify();
// Result: -5 + 10i
```

### Division

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// (3 + 4i) / (1 + 2i)
// Multiply by conjugate: (3 + 4i)(1 - 2i) / ((1 + 2i)(1 - 2i))
// = (3 - 6i + 4i - 8i²) / (1 - 4i²)
// = (3 - 2i + 8) / (1 + 4)
// = (11 - 2i) / 5
// = 11/5 - (2/5)i
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 + 2*i);
let quotient = expr!(z1 / z2);
let result = quotient.simplify();
// Result: 11/5 - (2/5)i
```

### Conjugate

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// Conjugate of 3 + 4i is 3 - 4i
let z = expr!(3 + 4*i);
let z_conj = expr!(conjugate(z));
let result = z_conj.simplify();
// Result: 3 - 4i

// Property: z * conjugate(z) = |z|²
let product = expr!(z * z_conj);
let abs_squared = product.simplify();
// Result: 25 (which is 3² + 4²)
```

### Modulus (Absolute Value)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// |3 + 4i| = sqrt(3² + 4²) = sqrt(9 + 16) = sqrt(25) = 5
let z = expr!(3 + 4*i);
let modulus = expr!(abs(z));
let result = modulus.simplify();
// Result: 5
```

### Real and Imaginary Parts

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();
let z = expr!(3 + 4*i);

// Extract real part: Re(3 + 4i) = 3
let real_part = expr!(real(z));
let re_result = real_part.simplify();
// Result: 3

// Extract imaginary part: Im(3 + 4i) = 4
let imag_part = expr!(imag(z));
let im_result = imag_part.simplify();
// Result: 4
```

## Complex Functions (⭐⭐⭐ Advanced)

### Exponential Function

**Euler's formula:** $e^{i\theta} = \cos\theta + i\sin\theta$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();
let x = symbol!(x);

// e^(ix) = cos(x) + i*sin(x)
let euler = expr!(exp(i * x));
let expanded = euler.simplify();
// Result: cos(x) + i*sin(x)

// General: e^(a + bi) = e^a * (cos(b) + i*sin(b))
let z = expr!(3 + 4*i);
let exp_z = expr!(exp(z));
let result = exp_z.simplify();
// Result: e³(cos(4) + i*sin(4))
```

### Logarithm (Principal Branch)

**Principal logarithm:** $\log(z) = \ln|z| + i\arg(z)$ where $-\pi < \arg(z) \leq \pi$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// log(i) = ln|i| + i*arg(i) = ln(1) + i*(π/2) = (π/2)i
let log_i = expr!(log(i));
let result = log_i.simplify();
// Result: (π/2)i

// log(-1) = ln|-1| + i*arg(-1) = ln(1) + i*π = πi
let log_neg1 = expr!(log(-1));
let result = log_neg1.simplify();
// Result: πi

// General complex logarithm
let z = expr!(3 + 4*i);
let log_z = expr!(log(z));
// Result: ln(5) + i*atan(4/3)
```

### Square Root (Principal Branch)

**Principal square root:** Branch cut on negative real axis

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// sqrt(-1) = i
let sqrt_neg1 = expr!(sqrt(-1));
let result = sqrt_neg1.simplify();
// Result: i

// sqrt(i) = (1 + i)/sqrt(2)
let sqrt_i = expr!(sqrt(i));
let result = sqrt_i.simplify();
// Result: (sqrt(2)/2)(1 + i)

// General: sqrt(a + bi)
let z = expr!(3 + 4*i);
let sqrt_z = expr!(sqrt(z));
// Result: 2 + i (since (2 + i)² = 3 + 4i)
```

### Trigonometric Functions

**Complex trigonometric identities:**
- $\sin(z) = \frac{e^{iz} - e^{-iz}}{2i}$
- $\cos(z) = \frac{e^{iz} + e^{-iz}}{2}$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// sin(i) = i*sinh(1)
let sin_i = expr!(sin(i));
let result = sin_i.simplify();
// Result: i*sinh(1)

// cos(i) = cosh(1)
let cos_i = expr!(cos(i));
let result = cos_i.simplify();
// Result: cosh(1)

// tan(i) = i*tanh(1)
let tan_i = expr!(tan(i));
let result = tan_i.simplify();
// Result: i*tanh(1)
```

### Hyperbolic Functions

**Complex hyperbolic identities:**
- $\sinh(z) = \frac{e^z - e^{-z}}{2}$
- $\cosh(z) = \frac{e^z + e^{-z}}{2}$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// sinh(i*x) = i*sin(x)
let x = symbol!(x);
let sinh_ix = expr!(sinh(i * x));
let result = sinh_ix.simplify();
// Result: i*sin(x)

// cosh(i*x) = cos(x)
let cosh_ix = expr!(cosh(i * x));
let result = cosh_ix.simplify();
// Result: cos(x)
```

## Branch Cuts and Principal Values

### What Are Branch Cuts?

**Branch cuts** are curves in the complex plane where **multi-valued functions** are discontinuous.

**Example:** Square root
- $\sqrt{1} = 1$ or $-1$ (two values!)
- To make a function, we choose **one branch**
- The **principal branch** chooses the value with $-\pi < \arg(z) \leq \pi$

### Principal Branch of `sqrt`

**Branch cut:** Negative real axis

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// sqrt(-1) = i (principal value)
let sqrt1 = expr!(sqrt(-1));
// Result: i (not -i)

// sqrt(-4) = 2i (principal value)
let sqrt2 = expr!(sqrt(-4));
// Result: 2i (not -2i)

// Crossing the branch cut
let z1 = expr!(-1 + 0.1*i);  // Just above negative real axis
let z2 = expr!(-1 - 0.1*i);  // Just below negative real axis
// sqrt(z1) and sqrt(z2) are discontinuous across the cut
```

### Principal Branch of `log`

**Branch cut:** Negative real axis
**Range:** $-\pi < \text{Im}(\log z) \leq \pi$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// log(1) = 0 (principal value)
let log1 = expr!(log(1));
// Result: 0

// log(-1) = πi (principal value)
let log_neg1 = expr!(log(-1));
// Result: πi (not -πi or 3πi)

// log(i) = (π/2)i
let log_i = expr!(log(i));
// Result: (π/2)i

// log(-i) = -(π/2)i
let log_neg_i = expr!(log(-i));
// Result: -(π/2)i
```

### Principal Branch of `z^w` (Complex Power)

**Definition:** $z^w = e^{w \log z}$ using principal branch of `log`

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// i^i = exp(i * log(i)) = exp(i * (π/2)i) = exp(-π/2)
let i_to_i = expr!(i^i);
let result = i_to_i.simplify();
// Result: exp(-π/2) ≈ 0.2078795763...

// (-1)^(1/2) = exp((1/2)*log(-1)) = exp((1/2)*πi) = cos(π/2) + i*sin(π/2) = i
let sqrt_neg1 = expr!((-1)^(1/2));
// Result: i
```

## Real vs Complex Domain

### When Functions Return Complex Results

Some functions **require complex numbers** for certain inputs:

| Function | Real Domain | Complex Extension |
|----------|-------------|------------------|
| `sqrt(x)` | $x \geq 0$ | All $x$ (complex result) |
| `log(x)` | $x > 0$ | All $x \neq 0$ |
| `asin(x)` | $-1 \leq x \leq 1$ | All $x$ |
| `acos(x)` | $-1 \leq x \leq 1$ | All $x$ |

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// sqrt of negative number: real → complex
let sqrt_neg = expr!(sqrt(-4));
let result = sqrt_neg.simplify();
// Result: 2i (complex)

// log of negative number: real → complex
let log_neg = expr!(log(-2));
// Result: ln(2) + πi (complex)

// arcsin outside [-1, 1]: real → complex
let asin_large = expr!(asin(2));
// Result: π/2 - i*ln(2 + sqrt(3)) (complex)
```

### Domain Control

**Option 1:** Let MathHook automatically extend to complex:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let result = expr!(sqrt(-1));  // Returns: i (complex)
```

**Option 2:** Explicitly check domain first:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let sqrt_expr = expr!(sqrt(x));

// Check if x < 0 before simplifying
// If x < 0, result will be complex
```

**Option 3:** Use assumptions to restrict domain:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Assume x is positive (future API)
let x = symbol!(x).assume_positive();
let sqrt_expr = expr!(sqrt(x));
// Now sqrt(x) is guaranteed real
```

## Real-World Applications

### 1. **Electrical Engineering (AC Circuits)**

**Problem:** Analyze AC circuit with impedance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// Impedance: Z = R + iωL
let R = symbol!(R);  // Resistance
let omega = symbol!(omega);  // Angular frequency
let L = symbol!(L);  // Inductance

let impedance = expr!(R + i*omega*L);

// Current: I = V/Z
let V = symbol!(V);  // Voltage
let current = expr!(V / impedance);
let I = current.simplify();
// Result: V*(R - iωL) / (R² + ω²L²)

// Power: P = |I|² * R
let power = expr!(abs(I)^2 * R);
```

### 2. **Signal Processing (Fourier Transform)**

**Problem:** Fourier transform of signal

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();
let t = symbol!(t);  // Time
let omega = symbol!(omega);  // Frequency

// Time-domain signal: x(t)
let x = symbol!(x);

// Fourier transform: X(ω) = ∫ x(t)*e^(-iωt) dt
let kernel = expr!(exp(-i * omega * t));
let fourier_integrand = expr!(x * kernel);
```

### 3. **Quantum Mechanics (Wave Functions)**

**Problem:** Quantum harmonic oscillator

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// Wave function: ψ(x, t) = A * exp(i(kx - ωt))
let A = symbol!(A);  // Amplitude
let k = symbol!(k);  // Wave number
let x = symbol!(x);  // Position
let omega = symbol!(omega);  // Angular frequency
let t = symbol!(t);  // Time

let wave_function = expr!(A * exp(i*(k*x - omega*t)));

// Probability density: |ψ|² = ψ * conjugate(ψ)
let psi_conj = expr!(conjugate(wave_function));
let probability = expr!(wave_function * psi_conj);
let prob_simplified = probability.simplify();
// Result: A² (constant, as expected for plane wave)
```

### 4. **Control Theory (Transfer Functions)**

**Problem:** Analyze system stability

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Transfer function: H(s) = K / (s² + 2ζωₙs + ωₙ²)
let s = symbol!(s);  // Complex frequency variable
let K = symbol!(K);  // Gain
let zeta = symbol!(zeta);  // Damping ratio
let omega_n = symbol!(omega_n);  // Natural frequency

let denominator = expr!(s^2 + 2*zeta*omega_n*s + omega_n^2);
let transfer_function = expr!(K / denominator);

// Poles (roots of denominator) determine stability
// s = -ζωₙ ± ωₙ*sqrt(ζ² - 1)
// If ζ < 1: complex conjugate poles (oscillatory response)
// If ζ > 1: real poles (overdamped response)
```

## Common Patterns

### Pattern 1: Euler's Formula Applications

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();
let theta = symbol!(theta);

// Convert exponential to trig: e^(iθ) = cos(θ) + i*sin(θ)
let euler = expr!(exp(i * theta));
let trig_form = euler.simplify();
// Result: cos(θ) + i*sin(θ)

// Convert trig to exponential: cos(θ) = (e^(iθ) + e^(-iθ))/2
let cos_exp = expr!((exp(i*theta) + exp(-i*theta)) / 2);
let simplified = cos_exp.simplify();
// Result: cos(θ)

// De Moivre's formula: (cos(θ) + i*sin(θ))^n = cos(nθ) + i*sin(nθ)
let n = symbol!(n);
let demoivre = expr!((exp(i*theta))^n);
let result = demoivre.simplify();
// Result: exp(i*n*θ) = cos(nθ) + i*sin(nθ)
```

### Pattern 2: Complex Magnitude and Phase

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// Convert rectangular to polar: z = a + bi → r*e^(iθ)
let a = symbol!(a);
let b = symbol!(b);
let z = expr!(a + b*i);

// Magnitude: r = |z| = sqrt(a² + b²)
let magnitude = expr!(abs(z));
let r = magnitude.simplify();
// Result: sqrt(a² + b²)

// Phase: θ = arg(z) = atan(b/a)
let phase = expr!(arg(z));
let theta = phase.simplify();
// Result: atan(b/a)

// Polar form: z = r*e^(iθ)
let polar = expr!(r * exp(i * theta));
```

### Pattern 3: Complex Conjugate Properties

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();
let z = expr!(3 + 4*i);

// Property 1: conjugate(conjugate(z)) = z
let double_conj = expr!(conjugate(conjugate(z)));
let result = double_conj.simplify();
assert_eq!(result, z);

// Property 2: conjugate(z1 + z2) = conjugate(z1) + conjugate(z2)
let z1 = expr!(1 + 2*i);
let z2 = expr!(3 + 4*i);
let sum_conj = expr!(conjugate(z1 + z2));
let conj_sum = expr!(conjugate(z1) + conjugate(z2));
assert_eq!(sum_conj.simplify(), conj_sum.simplify());

// Property 3: conjugate(z1 * z2) = conjugate(z1) * conjugate(z2)
let prod_conj = expr!(conjugate(z1 * z2));
let conj_prod = expr!(conjugate(z1) * conjugate(z2));
assert_eq!(prod_conj.simplify(), conj_prod.simplify());

// Property 4: z * conjugate(z) = |z|²
let abs_squared = expr!(z * conjugate(z));
let magnitude_squared = expr!(abs(z)^2);
assert_eq!(abs_squared.simplify(), magnitude_squared.simplify());
```

### Pattern 4: Complex Division by Conjugate

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// Divide complex numbers by multiplying by conjugate
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 + 2*i);

// Method: (z1/z2) = (z1 * conjugate(z2)) / (z2 * conjugate(z2))
let z2_conj = expr!(conjugate(z2));
let numerator = expr!(z1 * z2_conj);
let denominator = expr!(z2 * z2_conj);
let quotient = expr!(numerator / denominator);
let result = quotient.simplify();
// Result: 11/5 - (2/5)i
```

## Common Pitfalls

### ❌ Pitfall 1: Forgetting Branch Cuts

**Problem:** Expecting multi-valued functions to have all values

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Expecting both +2i and -2i
let sqrt_neg4 = expr!(sqrt(-4));
// Returns: 2i (principal value only, not -2i)

// ✅ CORRECT: Understand principal branch
// sqrt(-4) = 2i (principal value with arg in (-π, π])
// The other value (-2i) requires different branch
```

### ❌ Pitfall 2: Assuming Real Arithmetic Rules

**Problem:** Complex multiplication is not commutative with some operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

// ❌ WRONG: log(z1*z2) = log(z1) + log(z2) (only for principal branch)
let z1 = expr!(-1);
let z2 = expr!(-1);
let log_product = expr!(log(z1 * z2));  // log(1) = 0
let sum_logs = expr!(log(z1) + log(z2));  // πi + πi = 2πi
// These are NOT equal! (log_product = 0, sum_logs = 2πi)

// ✅ CORRECT: Use caution with logarithm identities in complex domain
// Identity holds modulo 2πi, but not exactly
```

### ❌ Pitfall 3: Not Simplifying Complex Results

**Problem:** Complex expressions not automatically simplified

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();

let z1 = expr!(3 + 4*i);
let z2 = expr!(1 + 2*i);

// ❌ WRONG: Not simplifying after operations
let product = expr!(z1 * z2);
// Result: (3 + 4i)(1 + 2i) (unsimplified)

// ✅ CORRECT: Always simplify complex results
let product_simplified = product.simplify();
// Result: -5 + 10i (simplified)
```

### ❌ Pitfall 4: Mixing Symbolic and Numerical Complex

**Problem:** Inconsistent representation causes issues

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

let i = Expression::i();
let x = symbol!(x);

// ❌ WRONG: Mixing representations inconsistently
let z_symbolic = expr!(x + y*i);  // Symbolic
let z_numeric = Expression::complex(
    Expression::float(3.0),
    Expression::float(4.0)
);  // Explicit
// Operations between these may not work as expected

// ✅ CORRECT: Use consistent representation
let z1 = expr!(3 + 4*i);  // Symbolic
let z2 = expr!(1 + 2*i);  // Symbolic (same style)
let result = expr!(z1 + z2).simplify();
```

## Performance Considerations

### Symbolic vs Explicit Complex

**Symbolic form (`a + b*i`):**
- **Advantages:** Natural syntax, automatic simplification
- **Disadvantages:** Slightly slower (needs pattern matching)

**Explicit form (`Complex(a, b)`):**
- **Advantages:** Faster access to components, clearer intent
- **Disadvantages:** More verbose

**Benchmark comparison:**
```text
Operation: (3 + 4i) * (1 + 2i)
- Symbolic form: 45ns
- Explicit form: 32ns
Speedup: 1.4x faster with explicit
```

### Complex Arithmetic Cost

Complex operations are more expensive than real:
- **Addition:** 2× cost (add real, add imag separately)
- **Multiplication:** 4× cost (4 real multiplications + 2 additions)
- **Division:** ~10× cost (needs conjugate multiplication)

### When to Use Complex vs Real

**Use complex only when necessary:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Using complex when real suffices
let x = symbol!(x);
let y = symbol!(y);
let z = expr!(x + y*i);  // Unnecessary if x, y are always real

// ✅ CORRECT: Use real arithmetic when possible
let sum = expr!(x + y);  // Real, unless complex needed
```

## API Reference

**Complex Number Creation:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Imaginary unit
fn i() -> Expression;

// Explicit complex number
fn complex(real: Expression, imag: Expression) -> Expression;
```

**Complex Operations:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Basic operations (use standard operators: +, -, *, /)

// Conjugate
fn conjugate(z: &Expression) -> Expression;

// Modulus (absolute value)
fn abs(z: &Expression) -> Expression;

// Real part
fn real(z: &Expression) -> Expression;

// Imaginary part
fn imag(z: &Expression) -> Expression;

// Argument (phase angle)
fn arg(z: &Expression) -> Expression;
```

## See Also

- **[Special Functions](special-functions.md)** - Complex-valued special functions
- **[Constants](../core/constants.md)** - Imaginary unit and complex constants
- **[Assumptions](assumptions.md)** - Domain assumptions (real vs complex)
- **[Solving Equations](../operations/solving.md)** - Complex roots of equations
