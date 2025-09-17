# Fourier Coefficients: Why They're Symbolic

## The Coefficient Problem

All PDE solutions via separation of variables involve **Fourier series coefficients** that must be computed from initial/boundary conditions.

### General Form

For any PDE solution:

$$u(x,t) = \sum_{n=1}^{\infty} A_n X_n(x) T_n(t)$$

The coefficients $A_n$ come from matching initial conditions:

$$u(x,0) = f(x) = \sum_{n=1}^{\infty} A_n X_n(x)$$

**Orthogonality** of eigenfunctions $X_n(x)$ gives:

$$A_n = \frac{\langle f, X_n \rangle}{\langle X_n, X_n \rangle} = \frac{\int_0^L f(x) X_n(x) \, dx}{\int_0^L X_n^2(x) \, dx}$$

**This requires symbolic integration**.

## Heat Equation Example

For heat equation with Dirichlet BCs on $[0,L]$:

$$X_n(x) = \sin\left(\frac{n\pi x}{L}\right)$$

Orthogonality:

$$\int_0^L \sin\left(\frac{n\pi x}{L}\right) \sin\left(\frac{m\pi x}{L}\right) dx = \begin{cases}
L/2 & n = m \\
0 & n \neq m
\end{cases}$$

Fourier coefficients:

$$A_n = \frac{2}{L} \int_0^L f(x) \sin\left(\frac{n\pi x}{L}\right) dx$$

**Requires symbolic integration** of $f(x) \sin(n\pi x/L)$.

### Simple Case: Constant Initial Condition

$$f(x) = c \quad \text{(constant)}$$

$$A_n = \frac{2c}{L} \int_0^L \sin\left(\frac{n\pi x}{L}\right) dx = \frac{2c}{L} \left[-\frac{L}{n\pi} \cos\left(\frac{n\pi x}{L}\right)\right]_0^L$$

$$= \frac{2c}{n\pi} [1 - \cos(n\pi)] = \frac{2c}{n\pi} [1 - (-1)^n]$$

$$= \begin{cases}
\frac{4c}{n\pi} & n \text{ odd} \\
0 & n \text{ even}
\end{cases}$$

**MathHook can compute this** (Phase 2) with symbolic integration.

### Complex Case: Arbitrary Function

$$f(x) = x(L-x) \quad \text{(parabola)}$$

$$A_n = \frac{2}{L} \int_0^L x(L-x) \sin\left(\frac{n\pi x}{L}\right) dx$$

**Requires integration by parts twice**:

$$= \frac{2}{L} \left[\frac{4L^3}{n^3\pi^3}[1 - (-1)^n]\right] = \frac{8L^2}{n^3\pi^3}[1 - (-1)^n]$$

**MathHook needs symbolic integration** for this.

## Why MathHook Returns Symbolic Coefficients

### Current Implementation

MathHook solvers return:

```rust
pub struct HeatSolution {
    pub solution: Expression,     // Σ A_n sin(λₙx) exp(-λₙαt)
    pub eigenvalues: Vec<Expression>,  // [λ₁, λ₂, λ₃, ...] ✅ COMPUTED
    pub coefficients: Vec<Expression>, // [A_1, A_2, A_3, ...] ⚠️ SYMBOLIC
}
```

The `coefficients` are **symbolic symbols** $A_1, A_2, A_3, \ldots$ (not numerical values).

**Why?**

Computing numerical $A_n$ requires:

$$A_n = \frac{2}{L} \int_0^L f(x) \sin\left(\frac{n\pi x}{L}\right) dx$$

This is **symbolic integration** of a **user-provided function** $f(x)$.

**MathHook's integration module** (Phase 1) focuses on:
- Standard integrals ($\int x^n dx$, $\int \sin(x) dx$, etc.)
- Integration by substitution
- Integration by parts

**NOT YET IMPLEMENTED**:
- Definite integral evaluation with symbolic limits
- Fourier sine/cosine integral tables
- Automated integration strategy selection

### Phase 2 Roadmap

**Goal**: Automatically compute Fourier coefficients for common initial conditions.

**Requirements**:

1. **Symbolic definite integration**:
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   let integral = expr!(integrate: f(x)*sin(n*pi*x/L), x, 0, L);
   let evaluated = integral.evaluate()?;  // Compute definite integral
   ```

2. **Fourier integral table**:
   - $\int_0^L 1 \cdot \sin(n\pi x/L) dx$
   - $\int_0^L x \cdot \sin(n\pi x/L) dx$
   - $\int_0^L x^2 \cdot \sin(n\pi x/L) dx$
   - $\int_0^L e^{ax} \cdot \sin(n\pi x/L) dx$
   - ...

3. **Pattern matching** for common forms:
   - Polynomial × trig
   - Exponential × trig
   - Piecewise functions

## Workaround: Manual Coefficient Computation

For simple initial conditions, manually compute coefficients:

```rust
use mathhook_core::pde::standard::heat::HeatEquationSolver;
use mathhook_core::{symbol, expr};

// ... (setup PDE, BCs, IC as before)

let result = solver.solve_heat_equation_1d(&pde, &alpha, &bcs, &ic)?;

// ⚠️ Coefficients are symbolic
println!("Symbolic: {:?}", result.coefficients);  // [A_1, A_2, A_3, ...]

// ✅ Manually compute for f(x) = 100 (constant)
let mut numerical_coeffs = Vec::new();
for n in 1..=10 {
    let a_n = if n % 2 == 1 {
        // Odd n: A_n = 400/(nπ)
        expr!(mul: (400.0 / ((n as f64) * std::f64::consts::PI)))
    } else {
        // Even n: A_n = 0
        expr!(0)
    };
    numerical_coeffs.push(a_n);
}

// Now substitute into solution expression
// (Requires substitution/evaluation - also Phase 2 feature)
```

## Examples: Computing Coefficients

### Heat Equation: Constant Initial Temp

**Initial condition**: $u(x,0) = T_0$

**Analytical**:

$$A_n = \frac{2T_0}{L} \int_0^L \sin\left(\frac{n\pi x}{L}\right) dx = \frac{2T_0}{n\pi}[1 - (-1)^n]$$

**Numerical** (for $T_0 = 100$, $L = 1$):
- $A_1 = \frac{200}{\pi} \approx 63.66$
- $A_2 = 0$
- $A_3 = \frac{200}{3\pi} \approx 21.22$
- $A_4 = 0$
- $A_5 = \frac{200}{5\pi} \approx 12.73$

### Wave Equation: Triangular Pluck

**Initial position**: Triangular (plucked at center)

**Analytical**:

$$A_n = \frac{8h}{n^2\pi^2} \sin\left(\frac{n\pi}{2}\right)$$

**Numerical** (for $h = 0.005$ m):
- $A_1 = \frac{8 \times 0.005}{\pi^2} \approx 0.00405$ m
- $A_2 = 0$ (sin(π) = 0)
- $A_3 = -\frac{8 \times 0.005}{9\pi^2} \approx -0.00045$ m
- $A_4 = 0$

### Laplace Equation: Fixed Top Edge

**Boundary condition**: $u(x,b) = V_0$

**Analytical**:

$$C_n = \frac{2V_0}{n\pi \sinh(n\pi b/a)} [1 - (-1)^n]$$

**Numerical** (for $V_0 = 100$, $a=0.1$, $b=0.05$):
- $C_1 = \frac{200}{\pi \sinh(\pi/2)} \approx \frac{200}{\pi \times 2.301} \approx 27.67$
- $C_2 = 0$
- $C_3 = \frac{200}{3\pi \sinh(3\pi/2)} \approx 0.83$
- ...

## Validation: SymPy Also Returns Symbolic

**Important**: SymPy's `pdsolve()` **ALSO** returns symbolic coefficients:

```python
from sympy import symbols, Function, Sum, sin, exp, pdsolve, Eq
u = Function('u')
x, t, n = symbols('x t n', real=True)
alpha, L = symbols('alpha L', positive=True)

heat_eq = Eq(u(x,t).diff(t), alpha * u(x,t).diff(x,2))
solution = pdsolve(heat_eq, u(x,t))

# Result: u(x,t) = Sum(C_n * sin(n*pi*x/L) * exp(-n²*pi²*alpha*t/L²), (n, 1, oo))
# C_n are SYMBOLIC (not computed from ICs)
```

**Why?** SymPy separates:
1. Solving PDE → symbolic solution structure
2. Matching ICs → separate `fourier_series()` function

MathHook follows the same philosophy.

## Summary

**Fourier Coefficients**:

$$A_n = \frac{\langle f, X_n \rangle}{\langle X_n, X_n \rangle}$$

**Current MathHook Status**:
- ✅ Correct eigenfunction identification
- ✅ Correct solution structure
- ✅ Symbolic coefficient placeholders ($A_1, A_2, \ldots$)
- ⚠️ Numerical evaluation requires symbolic integration (Phase 2)

**Workarounds**:
- Manually compute for simple $f(x)$ (constant, polynomial, etc.)
- Use external tools (SymPy, Mathematica) for complex $f(x)$
- Substitute numerical values into symbolic solution

**Phase 2 Goal**: Automatic Fourier coefficient computation via symbolic integration.

**Next**: [Boundary Conditions](./boundary-conditions.md) for more on BC types and handling.
