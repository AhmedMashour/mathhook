# PDE Classification

## Why Classification Matters

Different PDE types require completely different solution methods:
- **Elliptic**: Boundary value problems, steady-state
- **Parabolic**: Initial value + boundary, diffusion
- **Hyperbolic**: Initial value + boundary, wave propagation

Using the wrong method **WILL FAIL** or produce nonsense results.

## Mathematical Classification Theory

### The Discriminant Formula

For a general second-order PDE:

$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + \text{lower order terms} = 0$$

The **discriminant** is:

$$\Delta = B^2 - 4AC$$

### Classification Categories

| Discriminant | Type | Canonical Form | Prototype |
|--------------|------|----------------|-----------|
| $\Delta < 0$ | **Elliptic** | $u_{xx} + u_{yy} = 0$ | Laplace |
| $\Delta = 0$ | **Parabolic** | $u_t = u_{xx}$ | Heat |
| $\Delta > 0$ | **Hyperbolic** | $u_{tt} = c^2 u_{xx}$ | Wave |

### Physical Interpretation

#### Elliptic ($\Delta < 0$)

**Characteristics**: No real characteristics (complex)

**Physical Meaning**: Equilibrium states, no time evolution

**Properties**:
- Smooth solutions (infinitely differentiable if coefficients are smooth)
- Maximum principle: solution maximum on boundary
- Propagation speed: **infinite** (disturbance felt everywhere instantly)

**Examples**:
- Laplace's equation: $\nabla^2 u = 0$ (electrostatics, steady heat)
- Poisson's equation: $\nabla^2 u = f$ (gravity, charged regions)
- Minimal surface equation: $(1 + u_y^2) u_{xx} - 2u_x u_y u_{xy} + (1 + u_x^2) u_{yy} = 0$

#### Parabolic ($\Delta = 0$)

**Characteristics**: One family of real characteristics

**Physical Meaning**: Diffusion processes, irreversible evolution

**Properties**:
- Smoothing effect (rough initial data becomes smooth)
- Infinite propagation speed (finite but small amplitude)
- Irreversible in time (cannot reverse diffusion without external forcing)

**Examples**:
- Heat equation: $u_t = \alpha u_{xx}$ (thermal diffusion)
- Black-Scholes equation: $\frac{\partial V}{\partial t} + \frac{1}{2}\sigma^2 S^2 \frac{\partial^2 V}{\partial S^2} + rS\frac{\partial V}{\partial S} - rV = 0$ (option pricing)
- Fokker-Planck equation: $\frac{\partial p}{\partial t} = \frac{\partial^2}{\partial x^2}(D p) - \frac{\partial}{\partial x}(\mu p)$ (stochastic processes)

#### Hyperbolic ($\Delta > 0$)

**Characteristics**: Two families of real characteristics

**Physical Meaning**: Wave propagation, reversible evolution

**Properties**:
- Finite propagation speed $c$ (disturbances travel along characteristics)
- Preservation of discontinuities (shocks can form)
- Reversible in time (wave equation is time-symmetric)

**Examples**:
- Wave equation: $u_{tt} = c^2 u_{xx}$ (vibrations, sound)
- Telegraph equation: $u_{tt} + 2\alpha u_t = c^2 u_{xx}$ (damped waves)
- Beam equation: $u_{tt} + \gamma u_{xxxx} = 0$ (elastic beam vibrations)

## Discriminant Computation in MathHook

### Wave Equation Example

For the wave equation $\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$:

Rewrite in standard form:

$$\frac{\partial^2 u}{\partial t^2} - c^2 \frac{\partial^2 u}{\partial x^2} = 0$$

Identify coefficients (with $x$ as first variable, $t$ as second):
- $A = -c^2$ (coefficient of $u_{xx}$)
- $B = 0$ (no mixed derivative $u_{xt}$)
- $C = 1$ (coefficient of $u_{tt}$)

Discriminant:

$$\Delta = B^2 - 4AC = 0 - 4(-c^2)(1) = 4c^2 > 0$$

**Classification**: Hyperbolic ✓

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Wave equation structure
let equation = expr!(mul: x, t);  // Pattern: mixed variables
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Hyperbolic));

// Discriminant computation
let disc = pde.compute_discriminant();
assert!(disc > 0.0);  // Positive discriminant
assert_eq!(disc, 4.0);  // Wave equation discriminant
```

### Heat Equation Example

For the heat equation $\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$:

This is a **first-order in time** PDE (parabolic by convention), not second-order:
- Only one second derivative: $u_{xx}$
- No $u_{tt}$ term

By convention, parabolic PDEs have discriminant $\Delta = 0$.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Heat equation structure
let equation = expr!(add: x, t);  // Pattern: additive variables
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Parabolic));

// Discriminant
let disc = pde.compute_discriminant();
assert_eq!(disc.abs(), 0.0);  // Zero discriminant
```

### Laplace Equation Example

For Laplace's equation $\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$:

Identify coefficients:
- $A = 1$ (coefficient of $u_{xx}$)
- $B = 0$ (no mixed derivative $u_{xy}$)
- $C = 1$ (coefficient of $u_{yy}$)

Discriminant:

$$\Delta = B^2 - 4AC = 0 - 4(1)(1) = -4 < 0$$

**Classification**: Elliptic ✓

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);

// Laplace equation structure
let equation = expr!(add: x, y);  // Pattern: additive spatial variables
let pde = Pde::new(equation, u, vec![x, y]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Elliptic));

// Discriminant
let disc = pde.compute_discriminant();
assert!(disc < 0.0);  // Negative discriminant
assert_eq!(disc, -4.0);  // Laplace equation discriminant
```

## Current Implementation Limitations

### Pattern-Based Classification

MathHook currently uses **pattern matching** instead of symbolic differentiation for coefficient extraction:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Current approach (type_detection.rs):
pub(super) fn compute_discriminant(&self) -> f64 {
    if self.independent_vars.len() < 2 {
        return 0.0;
    }

    if self.looks_like_laplace_equation() {
        -4.0  // Discriminant for Laplace equation
    } else if self.looks_like_heat_equation() {
        0.0   // Discriminant for heat equation
    } else if self.looks_like_wave_equation() {
        4.0   // Discriminant for wave equation
    } else {
        0.0   // Default
    }
}
```

**Why Pattern Matching?**:
- Symbolic differentiation of coefficients requires extracting $A$, $B$, $C$ from PDE
- This requires: $A = \frac{\partial^2}{\partial x^2}(\text{equation})$ evaluated symbolically
- MathHook's differentiation module focuses on expressions, not PDE coefficient extraction
- Pattern matching works for standard equations (heat, wave, Laplace)

**Future Enhancement**:
Phase 2 will implement full symbolic coefficient extraction:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Future approach:
let A = extract_coefficient(pde, &x, &x);  // ∂²/∂x² coefficient
let B = extract_coefficient(pde, &x, &y);  // ∂²/∂x∂y coefficient
let C = extract_coefficient(pde, &y, &y);  // ∂²/∂y² coefficient

let discriminant = B*B - 4*A*C;  // True symbolic discriminant
```

## Variable Naming Heuristics

MathHook infers PDE type from variable names:

### Time-Space PDEs

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Variables named 't' or 'time' → considered temporal
// Variables named 'x', 'y', 'z', or 'space' → considered spatial

let t = symbol!(t);      // Temporal variable
let x = symbol!(x);      // Spatial variable
let time = symbol!(time); // Also temporal
let space = symbol!(space); // Also spatial
```

**Heat/Wave Equation Detection**:
- Requires exactly 2 variables
- One temporal (`t` or `time`)
- One spatial (`x` or `space`)
- Heat: Additive structure (first-order time derivative)
- Wave: Multiplicative structure (second-order time derivative)

### Spatial-Only PDEs

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Variables named 'x' and 'y' → spatial coordinates
let x = symbol!(x);
let y = symbol!(y);
```

**Laplace Equation Detection**:
- Requires 2+ spatial variables
- No temporal variable
- Additive structure

### Custom Variable Names

**⚠️ Warning**: Non-standard variable names may not classify correctly:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ May not classify:
let r = symbol!(r);      // Radial coordinate (not recognized as spatial)
let theta = symbol!(theta); // Angular coordinate (not recognized)

// ✅ Use standard names or wait for Phase 2:
let x = symbol!(x);
let y = symbol!(y);
```

## Classification Edge Cases

### Mixed-Type PDEs

Some PDEs change type based on region:

**Tricomi Equation**: $y u_{xx} + u_{yy} = 0$

- $y > 0$: Elliptic ($\Delta = -4y < 0$)
- $y = 0$: Parabolic ($\Delta = 0$)
- $y < 0$: Hyperbolic ($\Delta = -4y > 0$)

**⚠️ MathHook does NOT handle mixed-type PDEs currently**.

### Degenerate Cases

**Equation**: $u_{xx} = 0$

This is technically a **degenerate elliptic** PDE (only one second derivative):
- Discriminant: $\Delta = 0 - 4(1)(0) = 0$ (parabolic by formula)
- But no time evolution (elliptic by behavior)

**MathHook Classification**: Depends on variable names. Use with caution.

## Validation Examples

### Verifying Classification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
#[test]
fn test_classification_examples() {
    // Wave equation (hyperbolic)
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);
    let wave = Pde::new(expr!(mul: x, t), u.clone(), vec![x.clone(), t.clone()]);
    assert_eq!(wave.pde_type(), Some(PdeType::Hyperbolic));
    assert!(wave.compute_discriminant() > 0.0);

    // Heat equation (parabolic)
    let heat = Pde::new(expr!(add: x, t), u.clone(), vec![x.clone(), t.clone()]);
    assert_eq!(heat.pde_type(), Some(PdeType::Parabolic));
    assert_eq!(heat.compute_discriminant().abs(), 0.0);

    // Laplace equation (elliptic)
    let y = symbol!(y);
    let laplace = Pde::new(expr!(add: x, y), u, vec![x, y]);
    assert_eq!(laplace.pde_type(), Some(PdeType::Elliptic));
    assert!(laplace.compute_discriminant() < 0.0);
}
```

## Mathematical References

1. **Strauss, Walter A.** *Partial Differential Equations: An Introduction*, Chapter 1
   - Classification theory for second-order PDEs
   - Canonical forms and characteristics

2. **Evans, Lawrence C.** *Partial Differential Equations*, Chapter 2
   - Rigorous treatment of characteristic theory
   - Geometric interpretation

3. **John, Fritz** *Partial Differential Equations*, 4th ed., Chapter 2
   - Classical classification approach
   - Transformation to canonical forms

## Summary

**Key Takeaways**:

1. ✅ Discriminant formula: $\Delta = B^2 - 4AC$
2. ✅ Three types: Elliptic ($\Delta < 0$), Parabolic ($\Delta = 0$), Hyperbolic ($\Delta > 0$)
3. ✅ MathHook correctly classifies standard equations (heat, wave, Laplace)
4. ⚠️ Current implementation uses pattern matching (not symbolic differentiation)
5. ⚠️ Variable naming matters: use `t`, `x`, `y` for reliable classification
6. ⚠️ Mixed-type and degenerate PDEs not fully supported

**Next**: See solver-specific documentation for heat, wave, and Laplace equations.
