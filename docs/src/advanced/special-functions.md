# Special Functions

MathHook provides comprehensive support for special mathematical functions including the Gamma function family (Gamma, Digamma, Polygamma), Beta function, and other advanced special functions.

## Gamma Function Family

### Gamma Function (Γ)

The gamma function extends the factorial to real and complex numbers:

$$\Gamma(n) = (n-1)!$$

for positive integers n.

More generally, for complex numbers with positive real part:

$$\Gamma(z) = \int_0^\infty t^{z-1} e^{-t} \, dt$$

#### Properties

- **Functional equation**: $\Gamma(z+1) = z \cdot \Gamma(z)$
- **Special values**:
  - $\Gamma(1) = 1$
  - $\Gamma(1/2) = \sqrt{\pi}$
  - $\Gamma(n) = (n-1)!$ for positive integers
- **Half-integers**: $\Gamma(n + 1/2) = \frac{(2n-1)!! \sqrt{\pi}}{2^n}$

#### Examples

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

// Integer factorial: Γ(5) = 4!
let result = gamma(&expr!(5));
// Result: 24

// Half-integer (exact symbolic): Γ(1/2) = √π
let result = gamma(&Expression::Number(Number::Float(0.5)));
// Result: sqrt(pi)

// Half-integer: Γ(3/2) = √π/2
let result = gamma(&Expression::Number(Number::Float(1.5)));
// Result: sqrt(pi)/2

// Numerical evaluation for arbitrary values
let result = gamma(&Expression::Number(Number::Float(3.7)));
// Result: numerical float value
```

#### Implementation Details

- **Integer values**: Computed exactly using factorial
- **Half-integers**: Exact symbolic form using $\sqrt{\pi}$
- **General values**: Lanczos approximation with 14-digit precision
- **Performance**: Optimized for both symbolic and numerical paths

### Digamma Function (ψ)

The digamma function is the logarithmic derivative of the gamma function:

$$\psi(z) = \frac{d}{dz} \ln(\Gamma(z)) = \frac{\Gamma'(z)}{\Gamma(z)}$$

#### Special Values

- $\psi(1) = -\gamma$ (Euler-Mascheroni constant ≈ -0.5772156649)
- $\psi(1/2) = -\gamma - \ln(4)$
- $\psi(n) = -\gamma + \sum_{k=1}^{n-1} \frac{1}{k}$ for positive integers $n > 1$

#### Recurrence Relation

$$\psi(z+1) = \psi(z) + \frac{1}{z}$$

This allows computing digamma at consecutive integers efficiently.

#### Examples

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

// Special value: ψ(1) = -γ
let result = digamma(&expr!(1));
// Result: -EulerGamma (symbolic)

// Using recurrence: ψ(5) = ψ(1) + 1 + 1/2 + 1/3 + 1/4
let result = digamma(&expr!(5));
// Result: -EulerGamma + 25/12 (exact symbolic)

// Numerical evaluation
let result = digamma(&Expression::Number(Number::Float(2.5)));
// Result: numerical float value

// Half-integer
let result = digamma(&Expression::Number(Number::Float(0.5)));
// Result: -γ - ln(4) ≈ -1.9635100260
```

#### Mathematical Context

The digamma function appears in:
- Statistical distributions (log-likelihood derivatives)
- Asymptotic analysis
- Number theory (harmonic series)
- Regularization in quantum field theory

### Polygamma Function (ψ^(n))

The polygamma function is the (n+1)-th derivative of ln(Γ(z)):

$$\psi^{(n)}(z) = \frac{d^{n+1}}{dz^{n+1}} \ln(\Gamma(z)) = \frac{d^n}{dz^n} \psi(z)$$

#### Special Cases

- $\psi^{(0)}(z) = \psi(z)$ (digamma function)
- $\psi^{(1)}(z)$ (trigamma function)
- $\psi^{(2)}(z)$ (tetragamma function)
- $\psi^{(3)}(z)$ (pentagamma function)

#### Special Values

- $\psi^{(1)}(1) = \frac{\pi^2}{6}$ (related to Riemann zeta function: $\zeta(2)$)
- $\psi^{(2)}(1) = -2\zeta(3)$ (Apéry's constant)
- $\psi^{(n)}(1) = (-1)^{n+1} n! \zeta(n+1)$ for $n \geq 1$

#### Recurrence Relation

$$\psi^{(n)}(z+1) = \psi^{(n)}(z) + \frac{(-1)^n n!}{z^{n+1}}$$

#### Sign Pattern

The polygamma functions alternate in sign:
- $\psi^{(0)}(z)$ can be positive or negative
- $\psi^{(1)}(z) > 0$ for all $z > 0$ (trigamma is always positive)
- $\psi^{(2)}(z) < 0$ for all $z > 0$
- $\psi^{(3)}(z) > 0$ for all $z > 0$
- General pattern: $(-1)^{n+1} \psi^{(n)}(z) > 0$ for $z > 0$

#### Examples

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

// Digamma (n=0)
let result = polygamma(0, &expr!(1));
// Result: same as digamma(1) = -γ

// Trigamma (n=1) special value: ψ^(1)(1) = π²/6
let result = polygamma(1, &expr!(1));
// Result: pi^2/6 (exact symbolic)

// Tetragamma (n=2)
let result = polygamma(2, &expr!(1));
// Result: numerical value (related to ζ(3))

// Numerical evaluation
let result = polygamma(1, &Expression::Number(Number::Float(2.0)));
// Result: π²/6 - 1 ≈ 0.6449340668...

// Higher orders
let result = polygamma(3, &Expression::Number(Number::Float(1.5)));
// Result: numerical value
```

#### Applications

Polygamma functions appear in:
- Probability theory (cumulant generating functions)
- Statistical mechanics (partition functions)
- Asymptotic expansions
- Special function identities

### Beta Function

The beta function is defined in terms of the gamma function:

$$B(a, b) = \frac{\Gamma(a) \cdot \Gamma(b)}{\Gamma(a+b)}$$

Equivalently (integral representation):

$$B(a, b) = \int_0^1 t^{a-1} (1-t)^{b-1} \, dt$$

#### Properties

- **Symmetry**: $B(a, b) = B(b, a)$
- **Relation to binomial coefficient**: $B(n, m) = \frac{1}{n \binom{n+m-1}{n-1}}$ for integers
- **Recursion**: $B(a, b) = \frac{b-1}{a+b-1} B(a, b-1)$ for $b > 1$

#### Examples

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

// Integer values: B(2, 3) = Γ(2)·Γ(3)/Γ(5) = 1·2/24 = 1/12
let a = expr!(2);
let b = expr!(3);
let result = beta(&a, &b);
// Result: exact symbolic (depends on gamma implementation)

// Numerical evaluation
let a = Expression::Number(Number::Float(2.5));
let b = Expression::Number(Number::Float(3.7));
let result = beta(&a, &b);
// Result: numerical float value

// Mixed types (auto-converts to numerical)
let a = Expression::Number(Number::Float(2.5));
let b = expr!(3);
let result = beta(&a, &b);
// Result: numerical float value

// Symmetry property
let result1 = beta(&expr!(2), &expr!(5));
let result2 = beta(&expr!(5), &expr!(2));
// result1 == result2 (B(2,5) = B(5,2) = 1/30)
```

## Implementation Notes

### Numerical Accuracy

All gamma family functions use high-precision numerical methods:

- **Lanczos approximation**: 14-digit precision for general values
- **Series expansions**: Asymptotic series for large arguments
- **Reflection formulas**: For values with negative real parts
- **Special value detection**: Exact symbolic results when possible

### Performance Characteristics

- **Integer factorials**: O(n) time, exact results
- **Half-integers**: O(1) symbolic construction
- **General values**: O(1) Lanczos evaluation
- **Digamma series**: O(n) for recurrence, O(1) asymptotic
- **Polygamma**: O(k) series summation (converges rapidly)

### Input Validation

All functions validate inputs for:
- NaN or infinity (returns NaN)
- Poles (non-positive integers for gamma)
- Domain restrictions
- Numerical stability limits

### Error Handling

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Gamma at pole (non-positive integer)
let result = gamma(&expr!(0));
// Returns: infinity (mathematical pole)

let result = gamma(&Expression::Number(Number::Float(f64::NAN)));
// Returns: NaN (invalid input)

// Digamma at pole
let result = digamma(&expr!(0));
// Returns: NaN (mathematical pole)
```

## Mathematical Background

### Euler-Mascheroni Constant (γ)

The Euler-Mascheroni constant appears frequently in gamma function identities:

$$\gamma = \lim_{n \to \infty} \left( \sum_{k=1}^n \frac{1}{k} - \ln(n) \right) \approx 0.5772156649$$

It is the constant in the digamma special value $\psi(1) = -\gamma$.

### Connection to Riemann Zeta Function

The polygamma functions are closely related to the Riemann zeta function:

$$\psi^{(n)}(1) = (-1)^{n+1} n! \zeta(n+1)$$

This connects special function theory to number theory.

### Asymptotic Expansions

For large $z$, the digamma function has the asymptotic expansion:

$$\psi(z) \sim \ln(z) - \frac{1}{2z} - \sum_{k=1}^\infty \frac{B_{2k}}{2k z^{2k}}$$

where $B_{2k}$ are Bernoulli numbers.

## References

- Abramowitz & Stegun, "Handbook of Mathematical Functions" (1964)
- NIST Digital Library of Mathematical Functions (DLMF)
- SymPy documentation (gamma.py implementation reference)
- Numerical Recipes, Chapter 6 (Special Functions)

## See Also

- [Elementary Functions](../core/functions.md)
- [Complex Numbers](./complex-numbers.md)
- [Numerical Integration](../operations/integration.md)
- [Series Expansion](../operations/series.md)
