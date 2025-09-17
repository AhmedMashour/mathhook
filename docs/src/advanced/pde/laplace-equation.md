# Laplace Equation Solver

## Mathematical Model

**Laplace's equation** describes steady-state (equilibrium) distributions:

$$\nabla^2 u = 0$$

In 2D:

$$\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$$

**Key property**: No time dependence → equilibrium state.

## Physical Interpretation

Laplace's equation governs:
1. **Electrostatics**: $\nabla^2 \phi = 0$ (electric potential in charge-free regions)
2. **Steady-state heat**: $\nabla^2 T = 0$ (temperature at equilibrium)
3. **Potential flow**: $\nabla^2 \psi = 0$ (stream function for irrotational, incompressible flow)
4. **Gravity**: $\nabla^2 U = 0$ (gravitational potential in vacuum)

## Real-World Example: Electrostatic Potential in Rectangular Plate

### Problem Setup

A thin rectangular conducting plate (dimensions $a \times b$ = 10 cm × 5 cm) has:
- **Bottom edge** ($y=0$): Grounded (0 V)
- **Top edge** ($y=b$): Fixed potential 100 V
- **Left/Right edges** ($x=0$, $x=a$): Grounded (0 V)

Find the electrostatic potential $u(x,y)$ inside the plate.

### Mathematical Formulation

**PDE**:
$$\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0, \quad 0 < x < a, \quad 0 < y < b$$

**Boundary Conditions**:
$$u(0, y) = 0, \quad u(a, y) = 0$$
$$u(x, 0) = 0, \quad u(x, b) = 100$$

### Analytical Solution

**Separation of variables**: $u(x,y) = X(x)Y(y)$

$$\frac{X''(x)}{X(x)} + \frac{Y''(y)}{Y(y)} = 0$$

$$\frac{X''(x)}{X(x)} = -\lambda, \quad \frac{Y''(y)}{Y(y)} = \lambda$$

**Eigenvalue problem from x-direction BCs**:

$$X''(x) + \lambda X(x) = 0, \quad X(0) = 0, \quad X(a) = 0$$

**Eigenvalues and eigenfunctions**:

$$\lambda_n = \left(\frac{n\pi}{a}\right)^2, \quad X_n(x) = \sin\left(\frac{n\pi x}{a}\right), \quad n = 1, 2, 3, \ldots$$

**Y equation**:

$$Y''(y) - \lambda_n Y(y) = 0$$

$$Y_n(y) = A_n e^{\sqrt{\lambda_n} y} + B_n e^{-\sqrt{\lambda_n} y} = C_n \sinh\left(\frac{n\pi y}{a}\right) + D_n \cosh\left(\frac{n\pi y}{a}\right)$$

**Apply $Y(0) = 0$**: $D_n = 0$ (cosh term vanishes)

**General solution**:

$$u(x,y) = \sum_{n=1}^{\infty} C_n \sin\left(\frac{n\pi x}{a}\right) \sinh\left(\frac{n\pi y}{a}\right)$$

**Apply top BC**: $u(x,b) = 100$

$$100 = \sum_{n=1}^{\infty} C_n \sinh\left(\frac{n\pi b}{a}\right) \sin\left(\frac{n\pi x}{a}\right)$$

**Fourier sine series**:

$$C_n \sinh\left(\frac{n\pi b}{a}\right) = \frac{2}{a} \int_0^a 100 \sin\left(\frac{n\pi x}{a}\right) dx = \frac{200}{n\pi}[1 - (-1)^n]$$

$$C_n = \frac{200}{n\pi \sinh(n\pi b/a)} [1 - (-1)^n] = \begin{cases}
\frac{400}{n\pi \sinh(n\pi b/a)} & n \text{ odd} \\
0 & n \text{ even}
\end{cases}$$

**Final solution**:

$$u(x,y) = \frac{400}{\pi} \sum_{k=0}^{\infty} \frac{1}{(2k+1) \sinh((2k+1)\pi b/a)} \sin\left(\frac{(2k+1)\pi x}{a}\right) \sinh\left(\frac{(2k+1)\pi y}{a}\right)$$

### MathHook Implementation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);

let equation = expr!(u);
let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

// Boundary conditions
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);
let bc_right = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0.1) },  // a=10cm
);
let bc_bottom = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: y.clone(), value: expr!(0) },
);
let bc_top = BoundaryCondition::dirichlet(
    expr!(100),
    BoundaryLocation::Simple { variable: y, value: expr!(0.05) },  // b=5cm
);

// Solve
let solver = LaplaceEquationSolver::new();
let result = solver.solve_laplace_equation_2d(&pde, &[bc_left, bc_right, bc_bottom, bc_top])?;

// ✅ What you get:
println!("Solution: {}", result.solution);
// u(x,y) = C_1*sin(λ₁*x)*sinh(λ₁*y) + C_2*sin(λ₂*x)*sinh(λ₂*y) + ...

println!("X-eigenvalues: {:?}", result.x_eigenvalues);
// [π/a, 2π/a, 3π/a, ...] = [nπ/a for n=1,2,3,...]

println!("Coefficients: {:?}", result.coefficients);
// [C_1, C_2, C_3, ...] ⚠️ SYMBOLIC (not C_n = 400/(nπ sinh(nπb/a)))
```

## Properties of Harmonic Functions

**Definition**: Solutions to Laplace's equation are called **harmonic functions**.

### Maximum Principle

**Theorem**: If $u$ is harmonic on domain $\Omega$ with boundary $\partial\Omega$, then:

$$\max_{\bar{\Omega}} u = \max_{\partial\Omega} u$$

(Maximum occurs on boundary, not interior)

**Physical interpretation**: Equilibrium → no local maxima inside.

### Mean Value Property

**Theorem**: Harmonic function value at any point equals average over any circle centered at that point:

$$u(x_0, y_0) = \frac{1}{2\pi r} \oint_{|{\bf x} - {\bf x}_0| = r} u({\bf x}) \, d\ell$$

**Proof**: Green's identity + Laplace equation.

### Uniqueness

**Theorem**: Laplace equation with Dirichlet BCs has **unique** solution.

**Proof**: Suppose two solutions $u_1$ and $u_2$. Let $v = u_1 - u_2$. Then:
- $\nabla^2 v = 0$ (Laplace)
- $v = 0$ on boundary (same BCs)
- By maximum principle: $\max v = 0$
- Similarly: $\min v = 0$
- Therefore: $v = 0$ everywhere → $u_1 = u_2$

## Numerical Example: Potential at Center

For our 10 cm × 5 cm plate, what's the potential at the center $(x,y) = (5, 2.5)$ cm?

**Series solution** (first 3 odd terms):

$$u(5, 2.5) = \frac{400}{\pi} \sum_{k=0}^{2} \frac{1}{(2k+1) \sinh((2k+1)\pi \times 0.5)} \sin\left(\frac{(2k+1)\pi \times 5}{10}\right) \sinh\left(\frac{(2k+1)\pi \times 2.5}{10}\right)$$

Note: $\sin(n\pi/2) = 1, 0, -1, 0, \ldots$ for $n=1,2,3,4,\ldots$

$$u(5, 2.5) \approx \frac{400}{\pi} \left[\frac{1}{1 \times \sinh(\pi/2)} \times 1 \times \sinh(\pi/4) - \frac{1}{5 \times \sinh(5\pi/2)} \times 1 \times \sinh(5\pi/4) \right]$$

$$\approx \frac{400}{\pi} \left[\frac{0.8687}{2.301} - \frac{3.604}{5 \times 74.2}\right] \approx \frac{400}{\pi} \times 0.3774 \approx 48.0 \, \text{V}$$

**Physical check**: Center is about halfway between 0 V (bottom) and 100 V (top), so ~48 V is reasonable.

## Validation Against SymPy

```python
from sympy import symbols, Function, Eq, pdsolve
from sympy.abc import x, y

u = Function('u')
laplace_eq = Eq(u(x,y).diff(x,2) + u(x,y).diff(y,2), 0)
sympy_sol = pdsolve(laplace_eq, u(x,y))
# SymPy gives general form with arbitrary functions F, G
# Separation of variables + BCs needed for specific solution
```

## Limitations

⚠️ **Rectangular domains only**: Circular/irregular geometries require different coordinates.

⚠️ **Dirichlet BCs only**: Neumann BCs ($\partial u/\partial n = g$ on boundary) require different eigenfunction matching.

⚠️ **Symbolic coefficients**: Same limitation as heat/wave equations.

⚠️ **2D only currently**: 3D Laplace not yet implemented.

## Mathematical References

1. **Strauss** *PDEs: An Introduction*, Chapter 6 - Laplace equation on rectangles
2. **Axler, Sheldon** *Harmonic Function Theory*, 2nd ed. - Advanced properties
3. **Jackson, J.D.** *Classical Electrodynamics*, 3rd ed. - Physics applications

## Summary

**Laplace Equation**: $\nabla^2 u = 0$ (equilibrium states)

**MathHook provides**:
- ✅ 2D rectangular domains
- ✅ Correct eigenvalue formulas: $\lambda_n = (n\pi/a)^2$
- ✅ Sinh/cosh solution structure
- ⚠️ Symbolic coefficients

**Next**: [Fourier Coefficients](./fourier-coefficients.md) explains why coefficients are symbolic.
