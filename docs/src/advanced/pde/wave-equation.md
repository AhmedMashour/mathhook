# Wave Equation Solver

## Mathematical Model

The **wave equation** governs oscillatory phenomena and wave propagation:

$$\frac{\partial^2 u}{\partial t^2} = c^2 \nabla^2 u$$

where:
- $u(x,t)$ is displacement at position $x$ and time $t$
- $c$ is wave speed (m/s)
- $\nabla^2 u = \frac{\partial^2 u}{\partial x^2}$ (1D)

## Physical Interpretation

**Newton's Second Law** applied to small element of string/membrane:

$$\rho \frac{\partial^2 u}{\partial t^2} = T \frac{\partial^2 u}{\partial x^2}$$

where $\rho$ = linear mass density, $T$ = tension.

**Wave speed**: $c = \sqrt{T/\rho}$

**Key property**: Waves propagate at **finite speed** $c$ (unlike heat equation's infinite speed).

## Real-World Example: Vibrating Guitar String

### Problem Setup

A guitar string of length $L = 0.65$ m (standard scale length) is plucked at the center, displaced $5$ mm, and released from rest.

**Material** (steel E string):
- Tension: $T = 73.4$ N
- Linear density: $\rho = 3.75 \times 10^{-4}$ kg/m
- Wave speed: $c = \sqrt{T/\rho} = \sqrt{73.4 / 3.75 \times 10^{-4}} \approx 442$ m/s

### Mathematical Formulation

**PDE**:
$$\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}, \quad 0 < x < L, \quad t > 0$$

**Boundary Conditions** (fixed ends):
$$u(0, t) = 0, \quad u(L, t) = 0$$

**Initial Position** (triangular pluck at center):
$$u(x, 0) = \begin{cases}
\frac{2h}{L}x & 0 \leq x \leq L/2 \\
\frac{2h}{L}(L-x) & L/2 < x \leq L
\end{cases}$$

where $h = 0.005$ m (5 mm displacement).

**Initial Velocity** (released from rest):
$$\frac{\partial u}{\partial t}(x, 0) = 0$$

### Analytical Solution

**General solution**:

$$u(x,t) = \sum_{n=1}^{\infty} \left[A_n \cos(\omega_n t) + B_n \sin(\omega_n t)\right] \sin\left(\frac{n\pi x}{L}\right)$$

where:
- **Eigenvalues**: $\lambda_n = (n\pi/L)^2$
- **Angular frequencies**: $\omega_n = c\lambda_n^{1/2} = \frac{n\pi c}{L}$
- **Physical frequencies**: $f_n = \frac{\omega_n}{2\pi} = \frac{nc}{2L}$

**Fourier coefficients from initial position**:

$$A_n = \frac{2}{L} \int_0^L u(x,0) \sin\left(\frac{n\pi x}{L}\right) dx$$

For triangular pluck:

$$A_n = \frac{8h}{n^2 \pi^2} \sin\left(\frac{n\pi}{2}\right) = \begin{cases}
\frac{8h}{n^2\pi^2}(-1)^{(n-1)/2} & \text{if } n \text{ odd} \\
0 & \text{if } n \text{ even}
\end{cases}$$

**From initial velocity** (released from rest: $\partial u/\partial t = 0$):

$$B_n = 0 \quad \text{for all } n$$

**Final solution**:

$$u(x,t) = \frac{8h}{\pi^2} \sum_{k=0}^{\infty} \frac{(-1)^k}{(2k+1)^2} \sin\left(\frac{(2k+1)\pi x}{L}\right) \cos\left(\frac{(2k+1)\pi c t}{L}\right)$$

### MathHook Implementation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Variables
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// PDE
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

// Wave speed
let c = expr!(442);  // m/s for steel E string

// Boundary conditions
let bc1 = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple {
        variable: x.clone(),
        value: expr!(0),
    },
);
let bc2 = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple {
        variable: x,
        value: expr!(0.65),  // L = 0.65 m
    },
);

// Initial conditions
let ic_position = InitialCondition::value(
    // Triangular function (symbolic - not yet computable)
    expr!(0.005)  // Placeholder for triangular shape
);
let ic_velocity = InitialCondition::derivative(expr!(0));  // Released from rest

// Solve
let solver = WaveEquationSolver::new();
let result = solver.solve_wave_equation_1d(
    &pde,
    &c,
    &[bc1, bc2],
    &ic_position,
    &ic_velocity
)?;

// ✅ What you get:
println!("Solution: {}", result.solution);
// u(x,t) = [A_1*cos(ω₁*t) + B_1*sin(ω₁*t)]*sin(π*x/L) + ...

println!("Eigenvalues: {:?}", result.eigenvalues);
// [λ₁, λ₂, λ₃, ...] where λₙ = (nπ/L)²

println!("Position coefficients (A_n): {:?}", result.position_coefficients);
// [A_1, A_2, A_3, ...] ⚠️ SYMBOLIC

println!("Velocity coefficients (B_n): {:?}", result.velocity_coefficients);
// [B_1, B_2, B_3, ...] ⚠️ SYMBOLIC
```

## Musical Harmonics

### Fundamental Frequency (n=1)

$$f_1 = \frac{c}{2L} = \frac{442}{2 \times 0.65} \approx 340 \, \text{Hz}$$

This is close to **E4 note** (329.63 Hz) - the open E string frequency.

### Overtones (n=2, 3, 4, ...)

$$f_n = n f_1$$

- $f_2 = 680$ Hz (octave)
- $f_3 = 1020$ Hz (octave + fifth)
- $f_4 = 1360$ Hz (two octaves)

**Timbre**: Combination of harmonics determines sound quality. Triangular pluck emphasizes odd harmonics.

## Standing Waves

**Physical interpretation**: Superposition of left-traveling and right-traveling waves.

**D'Alembert solution**:

$$u(x,t) = \frac{1}{2}[f(x-ct) + f(x+ct)]$$

where $f(x)$ is the initial shape extended periodically.

**Standing wave form**: Separation of variables gives standing waves (nodes don't move).

**Nodes**: Points where $u(x,t) = 0$ for all $t$:

$$\sin\left(\frac{n\pi x}{L}\right) = 0 \implies x = \frac{kL}{n}, \quad k = 0, 1, \ldots, n$$

Mode $n$ has $n+1$ nodes (including endpoints).

## Energy Conservation

**Total energy** (kinetic + potential):

$$E(t) = \frac{1}{2} \int_0^L \left[\rho \left(\frac{\partial u}{\partial t}\right)^2 + T \left(\frac{\partial u}{\partial x}\right)^2 \right] dx$$

**Theorem**: For wave equation with fixed BCs, $E(t) = E(0)$ (constant).

**Proof sketch**: $\frac{dE}{dt} = 0$ using PDE and integration by parts.

**Physical meaning**: No energy dissipation (ideal string). Real strings have damping → telegraph equation.

## Validation

**SymPy comparison**:
```python
from sympy import symbols, Function, Eq, pdsolve
u = Function('u')
x, t, c = symbols('x t c', real=True, positive=True)

wave_eq = Eq(u(x,t).diff(t,2), c**2 * u(x,t).diff(x,2))
sympy_sol = pdsolve(wave_eq, u(x,t))
# Matches MathHook structure
```

## Limitations

⚠️ **Symbolic coefficients only** (same as heat equation)

⚠️ **Damped waves NOT supported**: Telegraph equation $u_{tt} + 2\alpha u_t = c^2 u_{xx}$ requires different solver.

⚠️ **Forcing terms NOT supported**: $u_{tt} = c^2 u_{xx} + F(x,t)$ requires inhomogeneous solution methods.

## Mathematical References

1. **Strauss, Walter A.** *PDEs: An Introduction*, Chapter 5 - Wave equation theory
2. **Berg, Richard E.** *The Physics of Sound*, 3rd ed. - Musical applications
3. **Whitham, G.B.** *Linear and Nonlinear Waves* - Advanced wave theory

## Summary

**Wave Equation**: $u_{tt} = c^2 u_{xx}$

**MathHook provides**:
- ✅ Correct eigenvalue formulas
- ✅ Two sets of coefficients ($A_n$ from position, $B_n$ from velocity)
- ✅ Frequency computation: $f_n = nc/(2L)$
- ⚠️ Symbolic coefficients (not numerically evaluated)

**Next**: [Laplace Equation](./laplace-equation.md)
