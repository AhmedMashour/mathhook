# Mathematical Constants

MathHook provides built-in mathematical constants with exact symbolic representation and high-precision numerical evaluation. Constants are treated as special expressions that preserve mathematical exactness throughout computations.

## Available Constants

### Fundamental Constants

#### Pi (π)

The ratio of a circle's circumference to its diameter.

**Mathematical Properties:**
- **Value:** $$\pi = 3.14159265358979323846\ldots$$
- **Transcendental:** Not a root of any non-zero polynomial with rational coefficients
- **Irrational:** Cannot be expressed as ratio of integers
- **Ubiquitous:** Appears in trigonometry, calculus, number theory, and physics

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Symbolic representation
let pi_expr = Expression::pi();

// In expressions
let circle_area = expr!(pi * r^2);  // Area of circle
let euler_identity = expr!(e^(i*pi) + 1);  // Euler's identity

// Numerical evaluation
let pi_value = Expression::pi().evaluate();  // High-precision float
```

**Common Operations:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Trigonometric exact values
assert_eq!(expr!(sin(pi)), expr!(0));
assert_eq!(expr!(cos(pi)), expr!(-1));
assert_eq!(expr!(sin(pi/2)), expr!(1));

// Calculus
let periodic = expr!(sin(2*pi*x));
let derivative = periodic.derivative(&x, 1);
assert_eq!(derivative, expr!(2*pi*cos(2*pi*x)));
```

#### Euler's Number (e)

The base of natural logarithms, fundamental to exponential growth and calculus.

**Mathematical Properties:**
- **Value:** $$e = 2.71828182845904523536\ldots$$
- **Definition:** $$e = \lim_{n \to \infty} \left(1 + \frac{1}{n}\right)^n$$
- **Series:** $$e = \sum_{n=0}^{\infty} \frac{1}{n!} = 1 + 1 + \frac{1}{2} + \frac{1}{6} + \frac{1}{24} + \cdots$$
- **Transcendental:** Like π, not algebraic

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Exponential function
let exponential = Expression::e();

// Natural logarithm base
let x = symbol!(x);
let ln_identity = expr!(ln(e));
assert_eq!(ln_identity.simplify(), expr!(1));

// Exponential growth
let growth = expr!(e^(r*t));  // Population growth, radioactive decay
```

**Common Identities:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Exponential-logarithm identities
assert_eq!(expr!(ln(e^x)).simplify(), x);
assert_eq!(expr!(e^(ln(x))).simplify(), x);

// Derivative property: derivative of e^x is itself
let exp_x = expr!(e^x);
assert_eq!(exp_x.derivative(&x, 1), exp_x);
```

#### Imaginary Unit (i)

The square root of -1, foundation of complex numbers.

**Mathematical Properties:**
- **Definition:** $$i^2 = -1$$
- **Powers:** $$i^0 = 1,\ i^1 = i,\ i^2 = -1,\ i^3 = -i,\ i^4 = 1,\ \ldots$$
- **Complex plane:** Rotates vectors by 90°

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Imaginary unit
let i = Expression::i();

// Complex numbers
let z = expr!(3 + 4*i);
let magnitude = expr!(sqrt((3^2) + (4^2)));  // |z| = 5

// Euler's formula: e^(ix) = cos(x) + i*sin(x)
let x = symbol!(x);
let euler_formula = expr!(e^(i*x));
// Evaluates to cos(x) + i*sin(x) when simplified
```

**Euler's Identity (Most Beautiful Equation):**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// e^(iπ) + 1 = 0
let euler_identity = expr!(e^(i*pi) + 1);
assert_eq!(euler_identity.simplify(), expr!(0));
```

### Special Constants

#### Golden Ratio (φ)

The golden ratio appears in geometry, art, and nature.

**Mathematical Properties:**
- **Value:** $$\varphi = \frac{1 + \sqrt{5}}{2} = 1.618033988749895\ldots$$
- **Definition:** $$\varphi^2 = \varphi + 1$$
- **Fibonacci:** $$\lim_{n \to \infty} \frac{F_{n+1}}{F_n} = \varphi$$

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let phi = Expression::golden_ratio();

// Geometric properties
// Golden rectangle: sides in ratio φ:1
let golden_rect_ratio = phi;

// Fibonacci connection
// φ = (1 + √5) / 2
let phi_exact = expr!((1 + sqrt(5)) / 2);
```

#### Euler-Mascheroni Constant (γ)

Euler's constant, fundamental in number theory and analysis.

**Mathematical Properties:**
- **Value:** $$\gamma = 0.5772156649015329\ldots$$
- **Definition:** $$\gamma = \lim_{n \to \infty} \left(\sum_{k=1}^{n} \frac{1}{k} - \ln(n)\right)$$
- **Open Question:** Whether γ is rational or transcendental is unknown!

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let gamma = Expression::euler_gamma();

// Appears in:
// - Harmonic series analysis
// - Gamma function: Γ'(1) = -γ
// - Riemann zeta function special values
```

### Infinity and Undefined

#### Infinity (∞)

Represents unbounded limits and asymptotic behavior.

**Usage:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let inf = Expression::infinity();
let neg_inf = Expression::negative_infinity();

// Limit behavior
let x = symbol!(x);
let limit_at_infinity = expr!(lim(1/x, x, infinity));
assert_eq!(limit_at_infinity.simplify(), expr!(0));

// Asymptotes
// lim (x→∞) e^x = ∞
// lim (x→-∞) e^x = 0
```

**Arithmetic with Infinity:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Defined operations
assert_eq!(expr!(infinity + 1), expr!(infinity));
assert_eq!(expr!(infinity * 2), expr!(infinity));
assert_eq!(expr!(1 / infinity).simplify(), expr!(0));

// Undefined forms (return Undefined)
// infinity - infinity
// infinity / infinity
// 0 * infinity
```

#### Undefined

Represents indeterminate or undefined mathematical expressions.

**Common Cases:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Division by zero
let undefined_div = expr!(1 / 0);
assert!(matches!(undefined_div.simplify(), Expression::Undefined));

// Indeterminate forms
// 0/0, ∞/∞, 0*∞, ∞-∞, 0^0, 1^∞, ∞^0
```

## Constant Arithmetic

### Exact Symbolic Operations

Constants preserve exactness in symbolic operations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Exact trigonometric values
assert_eq!(expr!(sin(pi/6)), expr!(1/2));
assert_eq!(expr!(cos(pi/3)), expr!(1/2));
assert_eq!(expr!(tan(pi/4)), expr!(1));

// Exact exponential values
assert_eq!(expr!(e^0), expr!(1));
assert_eq!(expr!(ln(1)), expr!(0));
assert_eq!(expr!(ln(e)), expr!(1));

// Complex exponentials
let euler = expr!(e^(i*pi));
assert_eq!(euler.simplify(), expr!(-1));
```

### Numerical Evaluation

When numerical values are needed:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// High-precision floating point
let pi_approx = Expression::pi().to_f64();
assert!((pi_approx - 3.141592653589793).abs() < 1e-15);

let e_approx = Expression::e().to_f64();
assert!((e_approx - 2.718281828459045).abs() < 1e-15);

// Golden ratio
let phi_approx = Expression::golden_ratio().to_f64();
assert!((phi_approx - 1.618033988749895).abs() < 1e-15);
```

## Real-World Examples

### Physics: Harmonic Motion

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Simple harmonic oscillator: x(t) = A*cos(ω*t + φ)
let t = symbol!(t);
let A = expr!(2);        // Amplitude
let omega = expr!(pi);   // Angular frequency
let phi = expr!(pi/4);   // Phase

let position = expr!(A * cos(omega*t + phi));
let velocity = position.derivative(&t, 1);
let acceleration = velocity.derivative(&t, 1);

// Verify: a = -ω²x
assert_eq!(acceleration, expr!(-(omega^2) * position));
```

### Engineering: RC Circuit Time Constant

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Capacitor discharge: V(t) = V₀ * e^(-t/RC)
let t = symbol!(t);
let V0 = expr!(5);       // Initial voltage (volts)
let tau = expr!(1);      // Time constant RC (seconds)

let voltage = expr!(V0 * e^(-t/tau));

// Time to reach 1/e of initial voltage
let time_constant = expr!(tau);
let voltage_at_tau = voltage.substitute(&t, &time_constant);
assert_eq!(voltage_at_tau.simplify(), expr!(V0 / e));
```

### Mathematics: Fourier Series

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Square wave Fourier series (first 3 terms)
let x = symbol!(x);
let fourier_square = expr!(
    (4/pi) * (sin(x) + (sin(3*x)/3) + (sin(5*x)/5))
);

// Period is 2π
let period = expr!(2*pi);
```

### Geometry: Circle Properties

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let r = symbol!(r);  // Radius

// Area: A = πr²
let area = expr!(pi * r^2);

// Circumference: C = 2πr
let circumference = expr!(2*pi*r);

// Verify: dA/dr = C (area derivative equals circumference)
let area_derivative = area.derivative(&r, 1);
assert_eq!(area_derivative, circumference);
```

## Common Pitfalls

### ❌ Pitfall 1: Premature Numerical Evaluation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Loses precision
let pi_approx = 3.14159;
let area_bad = expr!(pi_approx * r^2);  // Uses float, loses exactness

// ✅ CORRECT: Keep symbolic until final evaluation
let r = symbol!(r);
let area_good = expr!(pi * r^2);  // Exact symbolic representation
let area_value = area_good.substitute(&r, &expr!(5)).evaluate();
```

### ❌ Pitfall 2: Treating Constants as Symbols

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: pi is a constant, not a variable
let pi = symbol!(pi);  // This creates a SYMBOL named "pi", not the constant
let bad_expr = expr!(pi * 2);  // This is 2*<symbol pi>, not 2π

// ✅ CORRECT: Use constant constructor
let pi_const = Expression::pi();
let good_expr = expr!(pi * 2);  // Parser recognizes 'pi' as constant
```

### ❌ Pitfall 3: Infinity Arithmetic

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Indeterminate form
let indeterminate = expr!(infinity - infinity);
// Returns Undefined, not 0

// ❌ WRONG: Undefined operation
let undefined = expr!((infinity * 0));
// Returns Undefined

// ✅ CORRECT: Use limits for indeterminate forms
let x = symbol!(x);
let limit_form = expr!(lim(x - x, x, infinity));
assert_eq!(limit_form.simplify(), expr!(0));
```

## Limitations

### Arbitrary Precision

**Current Limitation:** Constants evaluate to f64 (64-bit floating point).

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Limited to ~15-17 decimal digits of precision
let pi_f64 = Expression::pi().to_f64();
// For higher precision, use external arbitrary-precision libraries
```

**Workaround:** Keep expressions symbolic as long as possible:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// ✅ Symbolic expression preserves exactness
let exact = expr!(pi * x^2);

// Only evaluate numerically at the end
let result = exact.substitute(&x, &expr!(10)).evaluate();
```

### Special Values

**Not all special values are pre-computed:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ✅ Available: Common exact values
assert_eq!(expr!(sin(pi/2)), expr!(1));
assert_eq!(expr!(cos(0)), expr!(1));

// ⚠️  Not pre-computed: Less common exact values
// sin(π/7) is exact but not pre-computed
// Falls back to numerical approximation
```

### Complex Constants

**Imaginary unit (i) requires careful handling:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ✅ Complex arithmetic works
let z = expr!(3 + 4*i);

// ⚠️  Real vs complex domain matters
// sqrt(-1) in real domain → error
// sqrt(-1) in complex domain → i
```

## See Also

- **[Expressions](./expressions.md)** - Expression structure and creation
- **[Functions](./functions.md)** - Mathematical function system
- **[Complex Numbers](../advanced/complex-numbers.md)** - Complex number support
- **[Simplification](../operations/simplification.md)** - Simplification with constants
- **API Reference:**
  - `Expression::pi()` - π constant
  - `Expression::e()` - Euler's number
  - `Expression::i()` - Imaginary unit
  - `Expression::infinity()` - Infinity
  - `Expression::golden_ratio()` - Golden ratio
  - `Expression::euler_gamma()` - Euler-Mascheroni constant

## Mathematical References

- **π (Pi):** Euler's *Introductio in analysin infinitorum* (1748)
- **e:** Euler's work on logarithms and calculus
- **i:** Gauss, *Disquisitiones Arithmeticae* (1801)
- **φ (Golden Ratio):** Euclid's *Elements*, Book VI
- **γ (Euler-Mascheroni):** Euler's work on harmonic series (1734)
- **Complex Analysis:** Ahlfors, *Complex Analysis* (3rd ed.)
- **Number Theory:** Hardy & Wright, *An Introduction to the Theory of Numbers*
