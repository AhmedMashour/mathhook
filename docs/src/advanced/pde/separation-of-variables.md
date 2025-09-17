# Separation of Variables

**Separation of Variables** is the fundamental technique for solving linear second-order partial differential equations (PDEs) with boundary conditions. It transforms the PDE into multiple ordinary differential equations (ODEs) that can be solved independently.

## Quick Overview

**Applies to:** Linear second-order PDEs with separable boundary conditions
**Equation types:** Heat equation, wave equation, Laplace equation
**Key idea:** Assume solution is a product of functions, each depending on only one variable
**MathHook implementation:** Used internally by Heat, Wave, and Laplace solvers

## Mathematical Foundation

### Core Assumption

For a PDE with two independent variables $x$ and $t$, assume:
$$u(x,t) = X(x) \cdot T(t)$$

For three variables (e.g., Laplace in 2D):
$$u(x,y) = X(x) \cdot Y(y)$$

**Crucial requirement:** The PDE must be **linear** (otherwise product form doesn't work)

### The Separation Process

**Step 1:** Substitute product form into PDE

**Step 2:** Separate variables (all $x$-terms on one side, all $t$-terms on other)

**Step 3:** Both sides must equal same constant (the **separation constant** $\lambda$)

**Step 4:** Solve resulting ODEs

**Step 5:** Apply boundary conditions to find eigenvalues

**Step 6:** Construct general solution as superposition

## Complete Examples

### Example 1: Heat Equation (1D)

**Problem:** Solve heat diffusion in a rod of length $L$

**PDE:** $\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$

**Boundary conditions:** $u(0,t) = 0$, $u(L,t) = 0$ (fixed temperature at ends)

**Initial condition:** $u(x,0) = f(x)$ (initial temperature distribution)

---

**Solution Steps:**

**1. Assume product solution:**
$$u(x,t) = X(x) T(t)$$

**2. Substitute into PDE:**
$$X(x) T'(t) = \alpha X''(x) T(t)$$

**3. Separate variables:**
$$\frac{T'(t)}{\alpha T(t)} = \frac{X''(x)}{X(x)}$$

Left side depends only on $t$, right side only on $x$. Both must equal constant $-\lambda$:

**4. Get two ODEs:**
- **Spatial ODE:** $X''(x) + \lambda X(x) = 0$
- **Temporal ODE:** $T'(t) + \lambda \alpha T(t) = 0$

**5. Apply boundary conditions to spatial ODE:**

$X(0) = 0$ and $X(L) = 0$ give eigenvalues:
$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 1, 2, 3, \ldots$$

Eigenfunctions:
$$X_n(x) = \sin\left(\frac{n\pi x}{L}\right)$$

**6. Solve temporal ODE:**
$$T_n(t) = e^{-\lambda_n \alpha t}$$

**7. General solution (superposition):**
$$u(x,t) = \sum_{n=1}^{\infty} A_n \sin\left(\frac{n\pi x}{L}\right) e^{-\lambda_n \alpha t}$$

**8. Match initial condition:**
$$u(x,0) = f(x) = \sum_{n=1}^{\infty} A_n \sin\left(\frac{n\pi x}{L}\right)$$

Fourier coefficients:
$$A_n = \frac{2}{L} \int_0^L f(x) \sin\left(\frac{n\pi x}{L}\right) dx$$

---

**MathHook Code:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Heat equation
let registry = PDESolverRegistry::new();
let pde = /* build heat equation */;

let solution = registry.solve(&pde).unwrap();
println!("Solution: {}", solution.solution);
// Output: Σ A_n sin(n*π*x/L) * exp(-λ_n*α*t)
```

---

### Example 2: Wave Equation (1D)

**Problem:** Vibrating string of length $L$

**PDE:** $\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$

**Boundary conditions:** $u(0,t) = 0$, $u(L,t) = 0$ (fixed ends)

**Initial conditions:** $u(x,0) = f(x)$ (initial displacement), $\frac{\partial u}{\partial t}(x,0) = g(x)$ (initial velocity)

---

**Solution Steps:**

**1. Assume:** $u(x,t) = X(x) T(t)$

**2. Substitute and separate:**
$$\frac{T''(t)}{c^2 T(t)} = \frac{X''(x)}{X(x)} = -\lambda$$

**3. Spatial ODE (same as heat equation):**
$$X''(x) + \lambda X(x) = 0$$

Eigenvalues: $\lambda_n = (n\pi/L)^2$

Eigenfunctions: $X_n(x) = \sin(n\pi x/L)$

**4. Temporal ODE:**
$$T''(t) + \lambda c^2 T(t) = 0$$

Solution (oscillatory):
$$T_n(t) = A_n \cos(\omega_n t) + B_n \sin(\omega_n t)$$

where $\omega_n = c\sqrt{\lambda_n} = cn\pi/L$

**5. General solution:**
$$u(x,t) = \sum_{n=1}^{\infty} \left[A_n \cos(\omega_n t) + B_n \sin(\omega_n t)\right] \sin\left(\frac{n\pi x}{L}\right)$$

**6. Match initial conditions:**
- $u(x,0) = f(x)$ determines $A_n$
- $\frac{\partial u}{\partial t}(x,0) = g(x)$ determines $B_n$

---

### Example 3: Laplace Equation (2D Rectangle)

**Problem:** Steady-state temperature in rectangular plate

**PDE:** $\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$

**Domain:** $0 \le x \le a$, $0 \le y \le b$

**Boundary conditions:**
- $u(0,y) = 0$, $u(a,y) = 0$ (left/right edges cold)
- $u(x,0) = 0$, $u(x,b) = f(x)$ (bottom cold, top has temperature profile)

---

**Solution Steps:**

**1. Assume:** $u(x,y) = X(x) Y(y)$

**2. Substitute and separate:**
$$\frac{X''(x)}{X(x)} + \frac{Y''(y)}{Y(y)} = 0$$

$$\frac{X''(x)}{X(x)} = -\frac{Y''(y)}{Y(y)} = -\lambda$$

**3. Two ODEs:**
- $X''(x) + \lambda X(x) = 0$
- $Y''(y) - \lambda Y(y) = 0$

**4. Apply boundary conditions in $x$:**

$X(0) = 0$, $X(a) = 0$ give:
$$\lambda_n = \left(\frac{n\pi}{a}\right)^2, \quad X_n(x) = \sin\left(\frac{n\pi x}{a}\right)$$

**5. Solve $Y$ equation:**
$$Y_n(y) = C_n \sinh\left(\frac{n\pi y}{a}\right)$$

(Using $Y(0) = 0$)

**6. General solution:**
$$u(x,y) = \sum_{n=1}^{\infty} C_n \sin\left(\frac{n\pi x}{a}\right) \sinh\left(\frac{n\pi y}{a}\right)$$

**7. Match boundary condition at $y = b$:**
$$u(x,b) = f(x) = \sum_{n=1}^{\infty} C_n \sinh\left(\frac{n\pi b}{a}\right) \sin\left(\frac{n\pi x}{a}\right)$$

Coefficients:
$$C_n = \frac{2}{a \sinh(n\pi b/a)} \int_0^a f(x) \sin\left(\frac{n\pi x}{a}\right) dx$$

---

## Eigenvalue Problems

**Central concept:** Separation of variables converts PDEs to **eigenvalue problems**

**Sturm-Liouville form:**
$$\frac{d}{dx}\left[p(x)\frac{dX}{dx}\right] + q(x)X + \lambda w(x)X = 0$$

with boundary conditions.

**Key properties:**
1. **Eigenvalues are real** and can be ordered: $\lambda_1 < \lambda_2 < \lambda_3 < \cdots$
2. **Eigenfunctions are orthogonal** (with weight function $w(x)$)
3. **Eigenfunctions form complete basis** (any function can be expanded in them)

**Example (Dirichlet BCs on [0,L]):**
- Eigenvalues: $\lambda_n = (n\pi/L)^2$
- Eigenfunctions: $\sin(n\pi x/L)$
- Orthogonality: $\int_0^L \sin(m\pi x/L) \sin(n\pi x/L) dx = 0$ if $m \ne n$

## Fourier Series Expansion

**The general solution is a Fourier series:**

$$u(x,t) = \sum_{n=1}^{\infty} a_n(t) X_n(x)$$

where $X_n(x)$ are eigenfunctions.

**Coefficients from initial conditions:**
$$a_n(0) = \frac{\langle f, X_n \rangle}{\langle X_n, X_n \rangle}$$

where $\langle \cdot, \cdot \rangle$ is inner product with weight function.

For $X_n = \sin(n\pi x/L)$ on $[0,L]$:
$$a_n(0) = \frac{2}{L} \int_0^L f(x) \sin(n\pi x/L) dx$$

**⚠️ MathHook Limitation:** Currently returns **symbolic coefficients** ($A_1, A_2, \ldots$). Numerical evaluation requires symbolic integration (planned for Phase 2).

## When Separation of Variables Works

**✅ Requirements:**
1. **Linear PDE** (otherwise product form invalid)
2. **Constant coefficients** or separable coefficients
3. **Rectangular domain** or simple geometry
4. **Separable boundary conditions** (each BC involves only one variable)

**✅ Works for:**
- Heat equation
- Wave equation
- Laplace equation
- Schrödinger equation (quantum mechanics)
- Many diffusion/wave problems

**❌ Doesn't work for:**
- Nonlinear PDEs
- Complex geometries (use finite elements)
- Non-separable boundary conditions
- Time-dependent coefficients

## Common Pitfalls

### 1. Wrong Sign for Separation Constant

**Common mistake:** Using $+\lambda$ instead of $-\lambda$

**Result:** Exponential growth instead of sinusoidal eigenfunctions

**Fix:** Check boundary conditions - usually need sinusoidal solutions

### 2. Forgetting Homogeneous Boundary Conditions

Separation of variables requires **homogeneous BCs** ($u = 0$ or $\frac{\partial u}{\partial n} = 0$ at boundaries).

**Non-homogeneous BCs:** Use **change of variables** to make homogeneous first.

### 3. Incomplete Superposition

**Mistake:** Using only one eigenfunction

**Fix:** General solution is **infinite series** (superposition of all eigenfunctions)

## MathHook Implementation

MathHook uses separation of variables internally for:
- `HeatEquationSolver`
- `WaveEquationSolver`
- `LaplaceEquationSolver`

**User doesn't call separation of variables directly** - it's applied automatically based on PDE classification.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let registry = PDESolverRegistry::new();
let solution = registry.solve(&pde).unwrap();
// Automatically uses separation of variables if applicable
```

## Further Reading

**Textbooks:**
1. **Strauss (2008)** - Chapters 4-6 (Heat, Wave, Laplace)
2. **Haberman (2012)** - Chapters 2-4 (Detailed worked examples)
3. **Farlow (1993)** - Chapters 5-7 (Engineering applications)

**Related documentation:**
- [Heat Equation](./heat-equation.md) - Parabolic PDEs
- [Wave Equation](./wave-equation.md) - Hyperbolic PDEs
- [Laplace Equation](./laplace-equation.md) - Elliptic PDEs
- [Fourier Coefficients](./fourier-coefficients.md) - Computing coefficients
- [Boundary Conditions](./boundary-conditions.md) - Types and handling

## See Also

- [Method of Characteristics](./method-of-characteristics.md) - For first-order PDEs
- [User Guide](./user-guide.md) - Comprehensive tutorials
- [Technical Guide](./technical-guide.md) - Mathematical rigor
- [Complete Examples](./examples.md) - Real-world applications
