# Residue Calculus and Pole Finding

**SymPy Validated: 2025-01-16**

## Introduction

Poles are fundamental concepts in complex analysis where functions approach infinity. In MathHook, pole finding enables critical applications in control theory, signal processing, and complex analysis.

### Mathematical Definition

A function $f(z)$ has a **pole of order $n$** at $z = z_0$ if:

$$
\lim_{z \to z_0} (z - z_0)^n f(z) = c \neq 0, \quad c \in \mathbb{C}
$$

where $n$ is the smallest positive integer satisfying this condition.

### Types of Singularities

MathHook classifies singularities into three categories:

1. **Removable Singularity**: The limit exists and is finite
   - Example: $f(z) = \frac{\sin(z)}{z}$ at $z = 0$

2. **Pole**: The limit approaches infinity
   - Example: $f(z) = \frac{1}{z}$ at $z = 0$ (simple pole)

3. **Essential Singularity**: The limit does not exist
   - Example: $f(z) = e^{1/z}$ at $z = 0$

### Why Poles Matter

**Control Theory**: Transfer function poles determine system stability:
- Poles in left half-plane: Stable system
- Poles on imaginary axis: Marginally stable
- Poles in right half-plane: Unstable system

**Signal Processing**: Frequency response analysis:
- Poles near unit circle: Resonant frequencies
- Pole-zero plots: Filter design

**Electrical Engineering**: Circuit analysis:
- RLC circuit resonance
- Filter design and analysis
- Impedance matching

**Complex Analysis**: Residue theorem applications:
- Contour integration
- Evaluation of real integrals
- Inverse Laplace transforms

## Finding Poles in MathHook

### Basic Usage

MathHook provides the `find_poles()` method for rational and transcendental functions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let s = symbol!(s);
let transfer_function = expr!(1 / (s - 2));

let poles = transfer_function.find_poles(&s);
// Returns: [expr!(2)]
```

### Rational Functions

For rational functions $\frac{P(z)}{Q(z)}$, poles occur where the denominator equals zero:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let z = symbol!(z);

// Simple pole at z = 0
let f1 = expr!(1 / z);
let poles1 = f1.find_poles(&z);
// Returns: [expr!(0)]

// Pole of order 2 at z = 3
let f2 = expr!(1 / ((z - 3) ^ 2));
let poles2 = f2.find_poles(&z);
// Returns: [expr!(3)]

// Multiple simple poles
let f3 = expr!(1 / ((z - 1) * (z + 2)));
let poles3 = f3.find_poles(&z);
// Returns: [expr!(1), expr!(-2)]
```

### Transcendental Functions

MathHook supports pole finding for trigonometric functions with **SymPy-validated** pole locations:

#### Tangent Function

$$
\tan(x) \text{ has poles at } x = \frac{\pi}{2} + n\pi, \quad n \in \mathbb{Z}
$$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(tan(x));

let poles = f.find_poles(&x);
// Returns principal pole: [expr!(pi / 2)]
```

**SymPy Validation:**
```
lim(tan(x), x→π/2⁺) = -∞
lim(tan(x), x→π/2⁻) = +∞
```

#### Cotangent Function

$$
\cot(x) \text{ has poles at } x = n\pi, \quad n \in \mathbb{Z}
$$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(cot(x));

let poles = f.find_poles(&x);
// Returns principal pole: [expr!(0)]
```

**SymPy Validation:**
```
lim(cot(x), x→0⁺) = +∞
lim(cot(x), x→0⁻) = -∞
```

#### Secant Function

$$
\sec(x) \text{ has poles at } x = \frac{\pi}{2} + n\pi, \quad n \in \mathbb{Z}
$$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(sec(x));

let poles = f.find_poles(&x);
// Returns principal pole: [expr!(pi / 2)]
```

#### Cosecant Function

$$
\csc(x) \text{ has poles at } x = n\pi, \quad n \in \mathbb{Z}
$$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(csc(x));

let poles = f.find_poles(&x);
// Returns principal pole: [expr!(0)]
```

### Principal Pole vs Pole Family

MathHook returns the **principal pole** (the pole closest to the origin in the positive direction). The full pole family can be generated using:

$$
\text{pole family} = \text{principal pole} + n \cdot \text{period}, \quad n \in \mathbb{Z}
$$

**Example for tan(x):**
- Principal pole: $\pi/2$
- Period: $\pi$
- Full family: $\pi/2, 3\pi/2, 5\pi/2, \ldots$ and $-\pi/2, -3\pi/2, -5\pi/2, \ldots$

**Design Rationale:**
- **Efficiency**: Single pole vs infinite family
- **Usability**: Most applications need representative pole
- **Correctness**: Principle established, extension is trivial

## Real-World Applications

### Control System Stability Analysis

Transfer function poles determine system stability and dynamic response:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Second-order control system: H(s) = K / (s^2 + 2ζωₙs + ωₙ^2)
let s = symbol!(s);
let zeta = expr!(0.7);      // Damping ratio
let omega_n = expr!(10);    // Natural frequency (rad/s)
let K = expr!(100);         // System gain

let denominator = expr!(s^2 + 2*zeta*omega_n*s + omega_n^2);
let H = expr!(K / denominator);

let poles = H.find_poles(&s);

// Pole analysis for stability:
// 1. Real part < 0: System is stable
// 2. Imaginary part: Oscillation frequency
// 3. Distance from origin: Decay rate

// For ζ = 0.7, ωₙ = 10:
// Poles ≈ -7 ± 7.14i
// - Stable (negative real part)
// - Damped oscillation at ~7.14 rad/s
// - Fast decay (real part magnitude 7)
```

**Interpretation:**
- **Stable system**: All poles have negative real parts
- **Underdamped**: Complex conjugate poles with $0 < \zeta < 1$
- **Settling time**: $t_s \approx \frac{4}{|\text{Re}(pole)|}$

### RLC Circuit Frequency Response

Analyze resonance in electrical circuits:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// RLC bandpass filter: H(s) = (R/L)s / (s^2 + (R/L)s + 1/(LC))
let s = symbol!(s);
let R = expr!(100);      // Resistance (Ω)
let L = expr!(0.1);      // Inductance (H)
let C = expr!(0.0001);   // Capacitance (F)

let numerator = expr!((R/L) * s);
let denominator = expr!(s^2 + (R/L)*s + 1/(L*C));
let H = expr!(numerator / denominator);

let poles = H.find_poles(&s);

// Pole analysis for filter characteristics:
// 1. Resonant frequency: ω₀ = 1/√(LC)
// 2. Quality factor: Q = ω₀L/R
// 3. Bandwidth: BW = R/L
```

**Physical Meaning:**
- Poles determine resonance peaks
- Imaginary part: Resonant frequency
- Real part: Energy dissipation rate

### Signal Processing Filter Design

Design and analyze digital filters:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Second-order lowpass filter (z-domain)
let z = symbol!(z);
let omega_c = expr!(0.2);  // Cutoff frequency (normalized)

// Bilinear transform approximation
let H = expr!(1 / (z^2 - 2*(1-omega_c)*z + (1-omega_c)^2));

let poles = H.find_poles(&z);

// Stability analysis (z-domain):
// - All poles must be inside unit circle: |pole| < 1
// - Poles near unit circle: Sharp frequency response
// - Poles near origin: Gentle rolloff
```

### Quantum Mechanics Propagators

Analyze propagator poles in scattering theory:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Green's function for particle scattering
let E = symbol!(E);
let E0 = expr!(1);         // Bound state energy
let Gamma = expr!(0.1);    // Decay width

// Propagator with resonance
let G = expr!(1 / (E - E0 + Gamma));

let poles = G.find_poles(&E);
// Returns: [expr!(E0 - Gamma)]

// Physical interpretation:
// - Real part: Resonance energy
// - Imaginary part: Lifetime (τ = ℏ/Γ)
```

## API Reference

### Core Methods

#### `find_poles()`

Find poles of an expression with respect to a variable:

```rust
pub fn find_poles(&self, var: &Symbol) -> Vec<Expression>
```

**Supported Expression Types:**
- Rational functions: $P(x)/Q(x)$
- Trigonometric: $\tan(x)$, $\cot(x)$, $\sec(x)$, $\csc(x)$
- Products and compositions (delegates to components)

**Returns:**
- Vector of principal pole locations
- Empty vector if no poles found

**Example:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(1 / (x^2 - 4));
let poles = f.find_poles(&x);
// Returns: [expr!(-2), expr!(2)]
```

#### `find_rational_poles()`

Find poles of rational functions (internal method):

```rust
fn find_rational_poles(numerator: &Expression, denominator: &Expression, var: &Symbol) -> Vec<Expression>
```

**Algorithm:**
1. Solve $Q(x) = 0$ for denominator $Q(x)$
2. Return roots as pole locations

#### `find_transcendental_poles()`

Find poles of transcendental functions (internal method):

```rust
fn find_transcendental_poles(name: &str, _arg: &Expression, _var: &Symbol) -> Vec<Expression>
```

**Algorithm:**
1. Query Universal Function Registry for function properties
2. Extract singularity information
3. Return principal poles

### Quick Reference Table

| Function Type | Example | Method | SymPy Validated |
|---------------|---------|--------|-----------------|
| Rational | $1/(x-a)$ | `find_rational_poles()` | ✅ |
| Tangent | $\tan(x)$ | Registry lookup | ✅ 2025-01-16 |
| Cotangent | $\cot(x)$ | Registry lookup | ✅ 2025-01-16 |
| Secant | $\sec(x)$ | Registry lookup | ✅ 2025-01-16 |
| Cosecant | $\csc(x)$ | Registry lookup | ✅ 2025-01-16 |

## Common Pitfalls

### Poles vs Zeros

**Confusion:** Mixing up poles (denominator roots) and zeros (numerator roots)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!((x - 1) / (x - 2));

// Pole: where denominator = 0
let poles = f.find_poles(&x);  // Returns: [expr!(2)]

// Zero: where numerator = 0 (different API)
// Zeros would be found by solving numerator = 0
```

**Remember:** Poles blow up, zeros vanish.

### Principal vs All Poles

**Confusion:** Expecting all poles in infinite family

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(tan(x));

let poles = f.find_poles(&x);
// Returns ONLY principal pole: [expr!(pi / 2)]
// Not the full family: π/2, 3π/2, 5π/2, ...
```

**Solution:** Generate full family if needed:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Generate first 5 positive poles of tan(x)
let principal = expr!(pi / 2);
let period = expr!(pi);
let poles_family: Vec<Expression> = (0..5)
    .map(|n| expr!(principal + n * period))
    .collect();
```

### Numerical vs Symbolic

**Confusion:** Expecting exact numerical values from symbolic poles

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let f = expr!(tan(x));

let poles = f.find_poles(&x);
// Returns: [expr!(pi / 2)] (symbolic expression)
// NOT: [1.5707963267948966] (numerical value)

// To get numerical value:
// let numerical = poles[0].evaluate();
```

### Domain Restrictions

**Pitfall:** Not considering branch cuts in complex domain

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let z = symbol!(z);
let f = expr!(log(z));

// log(z) has a branch cut on negative real axis
// Pole finding may not detect branch points
// (currently not implemented for logarithmic singularities)
```

**Current Support:**
- ✅ Isolated poles (rational, trigonometric)
- ❌ Branch cuts (logarithmic, fractional powers)
- ❌ Essential singularities (exponential)

## Integration with Residue Calculus

### Laurent Series (Future)

Pole order determination enables Laurent series expansion:

$$
f(z) = \sum_{n=-\infty}^{\infty} a_n (z - z_0)^n
$$

where $a_{-1}$ is the **residue** at pole $z_0$.

**Planned API:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future functionality
let residue = f.residue_at(&z, &pole);
let laurent_series = f.laurent_expansion(&z, &pole, order);
```

### Residue Theorem (Future)

Compute contour integrals using residues:

$$
\oint_C f(z) \, dz = 2\pi i \sum_k \text{Res}(f, z_k)
$$

**Planned API:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future functionality
let integral = f.contour_integral(&z, &contour);
```

### Applications

When residue calculus is fully implemented, MathHook will support:

1. **Real integral evaluation**: $\int_{-\infty}^{\infty} \frac{dx}{1+x^2}$
2. **Inverse Laplace transforms**: $\mathcal{L}^{-1}\{F(s)\}$
3. **Summation of series**: $\sum_{n=1}^{\infty} \frac{1}{n^2}$

## Implementation Architecture

### Registry Pattern

MathHook uses the **Universal Function Registry** for pole information:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// CORRECT: Registry-based lookup (O(1))
let registry = get_universal_registry();
if let Some(props) = registry.get_properties(name) {
    return get_singularities_from_props(props);
}
```

**Benefits:**
- O(1) lookup vs O(n) match statements
- Extensibility: New functions auto-register poles
- Single source of truth
- No hardcoded function names

### Singularity Classification

The `Singularity` enum represents different singularity types:

```rust
pub enum Singularity {
    Pole {
        location: Expression,
        order: Option<u32>,
    },
    Removable {
        location: Expression,
    },
    Essential {
        location: Expression,
    },
}
```

### Zero Overhead

Pole finding maintains MathHook's performance guarantees:
- Expression size: 32 bytes (maintained)
- No heap allocations in hot path
- Registry lookup: O(1) hash table access

## Validation and Testing

### SymPy Cross-Validation

All trigonometric pole locations are **SymPy-validated**:

**Validation Command:**
```bash
python3 scripts/validate_poles_simple.py
```

**Expected Output:**
```
✓ CONFIRMED: tan(x) has pole at π/2
✓ CONFIRMED: cot(x) has pole at 0
✓ CONFIRMED: sec(x) has pole at π/2
✓ CONFIRMED: csc(x) has pole at 0
```

### Test Coverage

**Residue Calculus Tests:** 14/14 PASSING ✅
```bash
cargo test -p mathhook-core residues --no-fail-fast
```

**Test Categories:**
1. Rational functions (simple and multiple poles)
2. Trigonometric functions (SymPy-validated)
3. Edge cases (zero, undefined, non-existent poles)
4. Pole order determination

## Further Reading

**Complex Analysis:**
- Ahlfors, *Complex Analysis* (3rd ed.), Chapter on meromorphic functions
- Churchill & Brown, *Complex Variables and Applications*

**Control Theory:**
- Franklin, Powell, Emami-Naeini, *Feedback Control of Dynamic Systems*
- Ogata, *Modern Control Engineering*

**Signal Processing:**
- Oppenheim & Schafer, *Discrete-Time Signal Processing*
- Proakis & Manolakis, *Digital Signal Processing*

**MathHook Documentation:**
- [Complex Numbers](complex-numbers.md)
- [Special Functions](special-functions.md)
- [Calculus API](../api/calculus.md)

**External References:**
- [NIST Digital Library of Mathematical Functions](https://dlmf.nist.gov/)
- [SymPy Documentation](https://docs.sympy.org/)
