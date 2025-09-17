# Function System

MathHook provides a comprehensive mathematical function system with intelligent evaluation, symbolic manipulation, and educational explanations. Functions are first-class expressions that support both exact symbolic computation and high-performance numerical evaluation.

## Function Architecture

### Universal Function Intelligence System

MathHook uses a **modular intelligence architecture** where each function family has dedicated intelligence:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Each function knows its own properties and behavior
let sin_x = expr!(sin(x));

// Automatic property lookup: sin is periodic with period 2π
// Automatic simplification: sin(0) → 0, sin(π/2) → 1
// Automatic derivative: d/dx sin(x) → cos(x)
```

**Key Benefits:**
- **O(1) Lookup:** Function properties retrieved via registry (HashMap)
- **Extensible:** Add new functions without modifying core system
- **Intelligent:** Functions know derivatives, integrals, special values
- **Educational:** Built-in step-by-step explanations

## Creating Functions with Macros

The `function!` macro provides ergonomic function creation with compile-time validation.

### Basic Usage

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Single argument functions
let sine = function!(sin, x);        // sin(x)
let cosine = function!(cos, x);      // cos(x)

// Multi-argument functions
let log_base = function!(log, x, base);   // log(x, base)
let power = function!(pow, base, exp);    // pow(base, exp)

// Zero-argument functions (constants represented as functions)
let gamma_val = function!(gamma);    // gamma()
```

### Comparison with Expression API

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// ❌ Verbose (still works, but not recommended):
let f = Expression::function("sin", vec![x.clone()]);

// ✅ Preferred (cleaner, compile-time checked):
let f = function!(sin, x);
```

### Common Mathematical Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let base = symbol!(base);
let n = symbol!(n);

// Trigonometric
let f1 = function!(sin, x);
let f2 = function!(cos, x);
let f3 = function!(tan, x);
let f4 = function!(asin, x);
let f5 = function!(acos, x);
let f6 = function!(atan, x);

// Hyperbolic
let f7 = function!(sinh, x);
let f8 = function!(cosh, x);
let f9 = function!(tanh, x);

// Exponential/Logarithmic
let f10 = function!(exp, x);
let f11 = function!(ln, x);
let f12 = function!(log, x, base);

// Special Functions
let f13 = function!(gamma, x);
let f14 = function!(factorial, n);
let f15 = function!(abs, x);
let f16 = function!(sqrt, x);
```

### Using with expr! Macro

Functions can also be called directly in `expr!`:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// These are equivalent:
let f1 = function!(sin, x);
let f2 = expr!(sin(x));

// Complex expressions
let complex = expr!(sin(x)^2 + cos(x)^2);  // Should simplify to 1
```

**When to use `function!` vs `expr!`:**
- **`function!`**: When you need just a function call expression
- **`expr!`**: When building larger expressions with operators and multiple terms

## Function Categories

### Elementary Functions

#### Trigonometric Functions

**Basic Trigonometry:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Basic trig functions
let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let tan_x = expr!(tan(x));

// Extended trig functions
let sec_x = expr!(sec(x));   // secant: 1/cos(x)
let csc_x = expr!(csc(x));   // cosecant: 1/sin(x)
let cot_x = expr!(cot(x));   // cotangent: 1/tan(x)
```

**Mathematical Properties:**

- **Periodicity:** $$\sin(x + 2\pi) = \sin(x)$$, $$\cos(x + 2\pi) = \cos(x)$$
- **Pythagorean Identity:** $$\sin^2(x) + \cos^2(x) = 1$$
- **Angle Sum:** $$\sin(a \pm b) = \sin(a)\cos(b) \pm \cos(a)\sin(b)$$

**Exact Values:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// MathHook recognizes exact trigonometric values
assert_eq!(expr!(sin(0)), expr!(0));
assert_eq!(expr!(sin(pi/6)), expr!(1/2));
assert_eq!(expr!(sin(pi/4)), expr!(sqrt(2)/2));
assert_eq!(expr!(sin(pi/3)), expr!(sqrt(3)/2));
assert_eq!(expr!(sin(pi/2)), expr!(1));

assert_eq!(expr!(cos(0)), expr!(1));
assert_eq!(expr!(cos(pi/3)), expr!(1/2));
assert_eq!(expr!(cos(pi/2)), expr!(0));
assert_eq!(expr!(cos(pi)), expr!(-1));

assert_eq!(expr!(tan(0)), expr!(0));
assert_eq!(expr!(tan(pi/4)), expr!(1));
```

**Inverse Trigonometric Functions:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Inverse trig functions
let arcsin_x = expr!(arcsin(x));  // asin(x) also works
let arccos_x = expr!(arccos(x));  // acos(x) also works
let arctan_x = expr!(arctan(x));  // atan(x) also works
let arcsec_x = expr!(arcsec(x));
let arccsc_x = expr!(arccsc(x));
let arccot_x = expr!(arccot(x));

// Inverse identities
assert_eq!(expr!(sin(arcsin(x))).simplify(), x);
assert_eq!(expr!(arcsin(sin(x))).simplify(), x);
```

**Hyperbolic Functions:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Hyperbolic functions
let sinh_x = expr!(sinh(x));  // (e^x - e^(-x))/2
let cosh_x = expr!(cosh(x));  // (e^x + e^(-x))/2
let tanh_x = expr!(tanh(x));  // sinh(x)/cosh(x)
let sech_x = expr!(sech(x));  // 1/cosh(x)
let csch_x = expr!(csch(x));  // 1/sinh(x)
let coth_x = expr!(coth(x));  // 1/tanh(x)

// Hyperbolic identity
assert_eq!(expr!(cosh(x)^2 - sinh(x)^2).simplify(), expr!(1));
```

#### Exponential and Logarithmic Functions

**Exponential Function:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Natural exponential
let exp_x = expr!(exp(x));    // e^x
let e_to_x = expr!(e^x);      // Same as exp(x)

// Derivative property: derivative of e^x is itself
assert_eq!(exp_x.derivative(&x, 1), exp_x);

// Exponential growth
let growth = expr!(exp(r*t));  // Population, radioactive decay
```

**Logarithmic Functions:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Natural logarithm (base e)
let ln_x = expr!(ln(x));

// Common logarithms
let log_x = expr!(log(x, 10));    // Base 10
let log2_x = expr!(log(x, 2));    // Base 2
let log_base_b = expr!(log(x, b)); // Arbitrary base

// Logarithm identities
assert_eq!(expr!(ln(e)).simplify(), expr!(1));
assert_eq!(expr!(ln(1)).simplify(), expr!(0));
assert_eq!(expr!(ln(e^x)).simplify(), x);
assert_eq!(expr!(e^(ln(x))).simplify(), x);

// Logarithm laws
assert_eq!(expr!(ln(x*y)).simplify(), expr!(ln(x) + ln(y)));
assert_eq!(expr!(ln(x/y)).simplify(), expr!(ln(x) - ln(y)));
assert_eq!(expr!(ln(x^n)).simplify(), expr!(n*ln(x)));
```

**Change of Base:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// log_b(x) = ln(x) / ln(b)
let log_b_x = expr!(ln(x) / ln(b));
```

#### Power and Root Functions

**Square Root:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Square root
let sqrt_x = expr!(sqrt(x));
let sqrt_alt = expr!(x^(1/2));  // Equivalent

// Properties
assert_eq!(expr!(sqrt(4)), expr!(2));
assert_eq!(expr!(sqrt(9)), expr!(3));
assert_eq!(expr!(sqrt(x^2)).simplify(), expr!(abs(x)));  // For real x
```

**nth Roots:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let n = symbol!(n);

// nth root
let nth_root = expr!(x^(1/n));

// Cube root
let cbrt = expr!(x^(1/3));
```

### Special Functions

#### Gamma Function

The Gamma function generalizes factorials to complex numbers.

**Mathematical Definition:**

$$\Gamma(n) = \int_0^{\infty} t^{n-1} e^{-t} dt$$

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Factorial relationship: Γ(n) = (n-1)!
assert_eq!(expr!(gamma(1)), expr!(1));   // Γ(1) = 0! = 1
assert_eq!(expr!(gamma(2)), expr!(1));   // Γ(2) = 1! = 1
assert_eq!(expr!(gamma(3)), expr!(2));   // Γ(3) = 2! = 2
assert_eq!(expr!(gamma(4)), expr!(6));   // Γ(4) = 3! = 6
assert_eq!(expr!(gamma(5)), expr!(24));  // Γ(5) = 4! = 24

// Half-integer values
assert_eq!(expr!(gamma(1/2)), expr!(sqrt(pi)));
```

#### Bessel Functions

Bessel functions are solutions to Bessel's differential equation, appearing in wave propagation, heat conduction, and vibrations.

**Mathematical Definition:**

$$x^2 \frac{d^2y}{dx^2} + x\frac{dy}{dx} + (x^2 - n^2)y = 0$$

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let n = symbol!(n);

// Bessel functions of the first kind
let J_n = expr!(bessel_j(n, x));

// Bessel functions of the second kind
let Y_n = expr!(bessel_y(n, x));

// Modified Bessel functions
let I_n = expr!(bessel_i(n, x));
let K_n = expr!(bessel_k(n, x));

// Common orders
let J_0 = expr!(bessel_j(0, x));
let J_1 = expr!(bessel_j(1, x));
```

**Real-World Applications:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Circular membrane vibration (drumhead)
let r = symbol!(r);
let theta = symbol!(theta);
let omega = symbol!(omega);
let t = symbol!(t);

// Displacement: u(r,θ,t) = J_n(k*r) * (A*cos(nθ) + B*sin(nθ)) * cos(ωt)
let displacement = expr!(bessel_j(n, k*r) * cos(n*theta) * cos(omega*t));
```

#### Orthogonal Polynomials

**Legendre Polynomials:**

Solutions to Legendre's differential equation, used in physics and engineering.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let n = symbol!(n);

// Legendre polynomial P_n(x)
let P_n = expr!(legendre_p(n, x));

// Specific orders
let P_0 = expr!(legendre_p(0, x));  // P_0(x) = 1
let P_1 = expr!(legendre_p(1, x));  // P_1(x) = x
let P_2 = expr!(legendre_p(2, x));  // P_2(x) = (3x² - 1)/2
```

**Hermite Polynomials:**

Used in quantum mechanics (harmonic oscillator) and probability theory.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let n = symbol!(n);

// Hermite polynomials (probabilist's convention)
let H_n = expr!(hermite(n, x));

// Physicist's Hermite polynomials
let He_n = expr!(hermite_physicist(n, x));
```

**Laguerre Polynomials:**

Appear in quantum mechanics (hydrogen atom) and numerical analysis.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let n = symbol!(n);

// Laguerre polynomial L_n(x)
let L_n = expr!(laguerre(n, x));

// Associated Laguerre polynomial L_n^k(x)
let L_n_k = expr!(laguerre_associated(n, k, x));
```

**Chebyshev Polynomials:**

Fundamental in approximation theory and numerical integration.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let n = symbol!(n);

// Chebyshev polynomials of the first kind T_n(x)
let T_n = expr!(chebyshev_first(n, x));

// Chebyshev polynomials of the second kind U_n(x)
let U_n = expr!(chebyshev_second(n, x));
```

### Utility Functions

#### Absolute Value

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Absolute value
let abs_x = expr!(abs(x));

// Properties
assert_eq!(expr!(abs(5)), expr!(5));
assert_eq!(expr!(abs(-5)), expr!(5));
assert_eq!(expr!(abs(0)), expr!(0));

// Complex magnitude
let z = expr!(3 + 4*i);
assert_eq!(expr!(abs(z)).simplify(), expr!(5));
```

#### Sign Function

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Sign function: -1, 0, or 1
let sign_x = expr!(sign(x));

assert_eq!(expr!(sign(5)), expr!(1));
assert_eq!(expr!(sign(-5)), expr!(-1));
assert_eq!(expr!(sign(0)), expr!(0));
```

#### Floor, Ceiling, Round

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Floor function (greatest integer ≤ x)
let floor_x = expr!(floor(x));
assert_eq!(expr!(floor(3.7)), expr!(3));
assert_eq!(expr!(floor(-2.3)), expr!(-3));

// Ceiling function (smallest integer ≥ x)
let ceil_x = expr!(ceiling(x));
assert_eq!(expr!(ceiling(3.2)), expr!(4));
assert_eq!(expr!(ceiling(-2.8)), expr!(-2));

// Rounding function
let round_x = expr!(round(x));
assert_eq!(expr!(round(3.4)), expr!(3));
assert_eq!(expr!(round(3.6)), expr!(4));
```

#### Max and Min

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Maximum and minimum
assert_eq!(expr!(max(3, 5)), expr!(5));
assert_eq!(expr!(min(3, 5)), expr!(3));

// Multiple arguments
let max_of_three = expr!(max(2, 7, 4));
assert_eq!(max_of_three, expr!(7));
```

### Complex Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let z = expr!(3 + 4*i);

// Real and imaginary parts
let real_part = expr!(real(z));
assert_eq!(real_part, expr!(3));

let imag_part = expr!(imag(z));
assert_eq!(imag_part, expr!(4));

// Complex conjugate
let conjugate = expr!(conjugate(z));
assert_eq!(conjugate, expr!(3 - 4*i));

// Argument (phase angle)
let arg_z = expr!(arg(z));
// Returns atan2(4, 3)
```

## Function Evaluation

### Symbolic Evaluation

Functions preserve symbolic form when possible:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Symbolic - no evaluation
let symbolic = expr!(sin(x));

// Exact special values
let exact = expr!(sin(pi/2));
assert_eq!(exact, expr!(1));

// Unevaluated for non-special values
let unevaluated = expr!(sin(0.5));  // Kept symbolic
```

### Numerical Evaluation

Convert to numerical approximation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Substitute and evaluate
let sin_half = expr!(sin(1/2));
let numerical = sin_half.evaluate();
// Returns approximate float value

// High-performance SIMD evaluation
// For arrays of values (performance-critical)
let values = vec![0.0, 0.5, 1.0, 1.5, 2.0];
let results = Expression::evaluate_array(&expr!(sin(x)), &values);
```

### Multi-Strategy Evaluation

Functions choose the best evaluation strategy:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Strategy 1: Exact symbolic (special values)
let exact = expr!(sin(pi/2));  // → 1

// Strategy 2: Simplification
let simplified = expr!(sin(pi - x));  // → sin(x)

// Strategy 3: Numerical approximation
let approx = expr!(sin(0.123));  // → ~0.122715...

// Strategy 4: SIMD vectorized (for arrays)
// Automatic dispatch to fastest implementation
```

## Function Derivatives

Functions know their own derivatives:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Trigonometric
assert_eq!(expr!(sin(x)).derivative(&x, 1), expr!(cos(x)));
assert_eq!(expr!(cos(x)).derivative(&x, 1), expr!(-sin(x)));
assert_eq!(expr!(tan(x)).derivative(&x, 1), expr!(sec(x)^2));

// Exponential and logarithmic
assert_eq!(expr!(exp(x)).derivative(&x, 1), expr!(exp(x)));
assert_eq!(expr!(ln(x)).derivative(&x, 1), expr!(1/x));

// Power function
let n = symbol!(n);
assert_eq!(expr!(x^n).derivative(&x, 1), expr!(n * x^(n-1)));

// Chain rule automatic
let f = expr!(sin(x^2));
assert_eq!(f.derivative(&x, 1), expr!(2*x*cos(x^2)));
```

## Real-World Examples

### Physics: Damped Harmonic Oscillator

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Damped harmonic motion: x(t) = A*e^(-γt)*cos(ωt + φ)
let t = symbol!(t);
let A = expr!(1);       // Amplitude
let gamma = expr!(0.1); // Damping coefficient
let omega = expr!(2*pi); // Angular frequency
let phi = expr!(0);     // Phase

let position = expr!(A * e^(-gamma*t) * cos(omega*t + phi));
let velocity = position.derivative(&t, 1);
let acceleration = velocity.derivative(&t, 1);

// Verify differential equation: ẍ + 2γẋ + ω²x = 0
let lhs = expr!(acceleration + 2*gamma*velocity + (omega^2)*position);
// lhs should simplify to 0
```

### Engineering: Transfer Function

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// First-order low-pass filter transfer function
let s = symbol!(s);      // Laplace variable
let omega_c = expr!(100); // Cutoff frequency (rad/s)

let H_s = expr!(omega_c / (s + omega_c));

// Frequency response: H(jω)
let omega = symbol!(omega);
let H_jw = H_s.substitute(&s, &expr!(i*omega));

// Magnitude and phase
let magnitude = expr!(abs(H_jw));
let phase = expr!(arg(H_jw));
```

### Signal Processing: Fourier Transform

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Gaussian function and its Fourier transform
let x = symbol!(x);
let sigma = expr!(1);

let gaussian = expr!(e^(-(x^2)/(2*sigma^2)));

// Fourier transform of Gaussian is also Gaussian
let k = symbol!(k);
let fourier = expr!(sqrt(2*pi*sigma^2) * e^(-(k^2)*(sigma^2)/2));
```

## Common Pitfalls

### ❌ Pitfall 1: Function Name Confusion

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: log() is NOT always base-10
let wrong = expr!(log(100));
// MathHook's log() can be base-e or base-10 depending on context

// ✅ CORRECT: Be explicit
let base_10 = expr!(log(100, 10));  // Explicit base-10: 2
let natural = expr!(ln(100));        // Natural log (base-e)
```

### ❌ Pitfall 2: Domain Violations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Ignoring domain restrictions
let bad = expr!(sqrt(-1));
// In real domain: error
// In complex domain: i

// ✅ CORRECT: Handle domain explicitly
let complex_result = expr!(sqrt(-1));  // Returns i in complex mode
let real_check = expr!(sqrt(x));       // Requires x ≥ 0 in real domain
```

### ❌ Pitfall 3: Numerical Precision

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Expecting exact equality for transcendental functions
let sin_pi = expr!(sin(pi)).evaluate();
assert_eq!(sin_pi, 0.0);  // May fail due to floating-point error

// ✅ CORRECT: Use symbolic simplification
assert_eq!(expr!(sin(pi)).simplify(), expr!(0));

// ✅ CORRECT: Use epsilon comparison for numerical
let sin_pi_num = expr!(sin(pi)).evaluate();
assert!((sin_pi_num - 0.0).abs() < 1e-10);
```

## Limitations

### Function Coverage

**Not all mathematical functions are built-in:**

- ✅ Implemented: Elementary functions, common special functions
- ⚠️ Missing: Some advanced special functions (Whittaker, Mathieu, etc.)
- **Workaround:** Define custom functions for missing functionality

### Numerical Stability

**Some functions require careful numerical handling:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Bessel functions for large arguments may lose precision
// Logarithms near 1: ln(1 + ε) loses precision for small ε
// Use log1p(ε) = ln(1 + ε) for better accuracy (future enhancement)
```

### Branch Cuts

**Complex functions have branch cuts:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// log(z) has branch cut on negative real axis
// sqrt(z) has branch cut on negative real axis
// Principal branch is used by default
```

## See Also

- **[Constants](./constants.md)** - Mathematical constants
- **[Expressions](./expressions.md)** - Expression structure
- **[Differentiation](../operations/differentiation.md)** - Symbolic derivatives
- **[Integration](../operations/integration.md)** - Symbolic integrals
- **[Special Functions](../advanced/special-functions.md)** - Advanced special functions
- **[Complex Numbers](../advanced/complex-numbers.md)** - Complex function evaluation
- **API Reference:** `Expression::function()`, function evaluation methods

## Mathematical References

- **Elementary Functions:** Abramowitz & Stegun, *Handbook of Mathematical Functions*
- **Special Functions:** NIST Digital Library of Mathematical Functions (DLMF)
- **Bessel Functions:** Watson, *A Treatise on the Theory of Bessel Functions*
- **Orthogonal Polynomials:** Szegö, *Orthogonal Polynomials*
- **Gamma Function:** Whittaker & Watson, *A Course of Modern Analysis*
- **Complex Analysis:** Ahlfors, *Complex Analysis* (3rd ed.)
