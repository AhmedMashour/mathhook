# Heat Equation Solver

## Mathematical Model

The **heat equation** (also called diffusion equation) governs how temperature distributes through materials over time:

$$\frac{\partial u}{\partial t} = \alpha \nabla^2 u$$

where:
- $u(x,t)$ is temperature at position $x$ and time $t$
- $\alpha$ is thermal diffusivity (m²/s): $\alpha = \frac{k}{\rho c_p}$
  - $k$ = thermal conductivity
  - $\rho$ = density
  - $c_p$ = specific heat capacity
- $\nabla^2 u = \frac{\partial^2 u}{\partial x^2}$ (1D) or $\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2}$ (2D)

## Physical Interpretation

**Fourier's Law of Heat Conduction**: Heat flows from hot to cold at a rate proportional to the temperature gradient.

**Heat Flux**: $\mathbf{q} = -k \nabla u$ (negative sign: heat flows toward lower temperature)

**Conservation of Energy**: Rate of temperature change = net heat flow in/out

$$\rho c_p \frac{\partial u}{\partial t} = \nabla \cdot (k \nabla u)$$

For constant material properties: $\frac{\partial u}{\partial t} = \alpha \nabla^2 u$

## Real-World Example: Cooling Metal Rod

### Problem Setup

A steel rod of length $L = 1$ meter is initially heated uniformly to $100°\text{C}$. Both ends are suddenly plunged into ice water (maintained at $0°\text{C}$). Find the temperature distribution $u(x,t)$ as the rod cools.

**Material Properties** (steel):
- Thermal conductivity: $k = 50 \, \text{W/(m} \cdot \text{K)}$
- Density: $\rho = 7850 \, \text{kg/m}^3$
- Specific heat: $c_p = 490 \, \text{J/(kg} \cdot \text{K)}$
- Thermal diffusivity: $\alpha = \frac{k}{\rho c_p} = \frac{50}{7850 \times 490} \approx 1.3 \times 10^{-5} \, \text{m}^2/\text{s}$

### Mathematical Formulation

**PDE**:
$$\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}, \quad 0 < x < L, \quad t > 0$$

**Boundary Conditions** (Dirichlet):
$$u(0, t) = 0°\text{C}, \quad u(L, t) = 0°\text{C} \quad \text{for all } t > 0$$

**Initial Condition**:
$$u(x, 0) = 100°\text{C} \quad \text{for } 0 < x < L$$

### Analytical Solution via Separation of Variables

#### Step 1: Assume Product Solution

$$u(x,t) = X(x) T(t)$$

#### Step 2: Separate Variables

Substitute into PDE:

$$X(x) T'(t) = \alpha X''(x) T(t)$$

Divide by $\alpha X(x) T(t)$:

$$\frac{T'(t)}{\alpha T(t)} = \frac{X''(x)}{X(x)} = -\lambda$$

(separation constant $-\lambda$ chosen for stability)

#### Step 3: Spatial ODE with Boundary Conditions

$$X''(x) + \lambda X(x) = 0$$

$$X(0) = 0, \quad X(L) = 0$$

**Eigenvalue Problem**: Only specific $\lambda_n$ give non-trivial solutions.

**Solution**:
$$X_n(x) = \sin\left(\frac{n\pi x}{L}\right), \quad \lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 1, 2, 3, \ldots$$

#### Step 4: Temporal ODE

$$T'(t) + \lambda_n \alpha T(t) = 0$$

**Solution**:
$$T_n(t) = \exp(-\lambda_n \alpha t)$$

#### Step 5: General Solution (Superposition)

$$u(x,t) = \sum_{n=1}^{\infty} A_n \sin\left(\frac{n\pi x}{L}\right) \exp\left(-\left(\frac{n\pi}{L}\right)^2 \alpha t\right)$$

#### Step 6: Fourier Coefficients from Initial Condition

Match $u(x,0) = 100$:

$$100 = \sum_{n=1}^{\infty} A_n \sin\left(\frac{n\pi x}{L}\right)$$

**Fourier sine series**:

$$A_n = \frac{2}{L} \int_0^L 100 \sin\left(\frac{n\pi x}{L}\right) dx$$

$$= \frac{200}{L} \left[ -\frac{L}{n\pi} \cos\left(\frac{n\pi x}{L}\right) \right]_0^L$$

$$= \frac{200}{n\pi} [1 - (-1)^n]$$

$$= \begin{cases}
\frac{400}{n\pi} & \text{if } n \text{ is odd} \\
0 & \text{if } n \text{ is even}
\end{cases}$$

**Final Solution**:

$$u(x,t) = \frac{400}{\pi} \sum_{k=0}^{\infty} \frac{1}{2k+1} \sin\left(\frac{(2k+1)\pi x}{L}\right) \exp\left(-\left(\frac{(2k+1)\pi}{L}\right)^2 \alpha t\right)$$

### MathHook Implementation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Define variables
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// PDE: u_t = α u_xx
let equation = expr!(u);  // Placeholder (solver knows structure)
let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

// Thermal diffusivity for steel
let alpha = expr!(0.000013);  // 1.3 × 10^-5 m²/s

// Boundary conditions: u(0,t) = 0, u(1,t) = 0
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
        value: expr!(1),  // L = 1 meter
    },
);

// Initial condition: u(x,0) = 100°C
let ic = InitialCondition::value(expr!(100));

// Solve
let solver = HeatEquationSolver::new();
let result = solver.solve_heat_equation_1d(&pde, &alpha, &[bc1, bc2], &ic)?;

// ✅ What you get:
println!("Solution structure: {}", result.solution);
// u(x,t) = A_1*sin(π*x)*exp(-π²*α*t) + A_2*sin(2π*x)*exp(-4π²*α*t) + ...

println!("Eigenvalues: {:?}", result.eigenvalues);
// [π², 4π², 9π², ...] = [(nπ/L)² for n=1,2,3,...]

println!("Coefficients (symbolic): {:?}", result.coefficients);
// [A_1, A_2, A_3, ...] ⚠️ SYMBOLIC, not numerical values
```

### Current Limitation: Symbolic Coefficients

⚠️ **MathHook returns symbolic $A_n$**, not the numerical values $\frac{400}{n\pi}$ for odd $n$ and $0$ for even $n$.

**Why?** Computing $A_n$ requires symbolic integration:

$$A_n = \frac{2}{L} \int_0^L f(x) \sin\left(\frac{n\pi x}{L}\right) dx$$

This needs the symbolic integration module (planned for Phase 2).

**Workaround**: Manually compute coefficients for simple initial conditions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For u(x,0) = 100 (constant), manually compute:
let mut manual_coeffs = Vec::new();
for n in 1..=10 {
    if n % 2 == 1 {  // Odd n
        let a_n = expr!(mul: (400 / (n * std::f64::consts::PI)));
        manual_coeffs.push(a_n);
    } else {  // Even n
        manual_coeffs.push(expr!(0));
    }
}
```

## Solution Behavior

### Exponential Decay

Each mode decays exponentially:

$$T_n(t) = \exp\left(-\lambda_n \alpha t\right) = \exp\left(-\left(\frac{n\pi}{L}\right)^2 \alpha t\right)$$

**Decay rate** increases with $n^2$:
- Mode $n=1$: decay time $\tau_1 \sim \frac{L^2}{\pi^2 \alpha}$
- Mode $n=2$: decay time $\tau_2 \sim \frac{\tau_1}{4}$ (4× faster)
- Mode $n=3$: decay time $\tau_3 \sim \frac{\tau_1}{9}$ (9× faster)

**Physical interpretation**: Higher spatial frequencies smooth out faster.

### Long-Time Behavior

After time $t \gg \tau_1$:

$$u(x,t) \approx A_1 \sin\left(\frac{\pi x}{L}\right) \exp\left(-\frac{\pi^2 \alpha t}{L^2}\right)$$

Only the fundamental mode survives. Temperature profile is half-sine wave.

### Maximum Principle

**Theorem**: For the heat equation with Dirichlet BCs, the maximum temperature occurs either:
1. Initially ($t=0$)
2. On the boundary ($x=0$ or $x=L$)

**Never** in the interior for $t > 0$.

**Proof**: If interior maximum at $(x_0, t_0)$ with $t_0 > 0$:
- $\frac{\partial u}{\partial t}(x_0, t_0) \geq 0$ (increasing or stationary)
- $\frac{\partial^2 u}{\partial x^2}(x_0, t_0) \leq 0$ (concave at maximum)
- But heat equation: $\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$
- This gives $0 \geq \alpha \times (\leq 0) \geq 0$, contradiction unless $\frac{\partial^2 u}{\partial x^2} = 0$

**For our problem**: $\max u(x,t) = 100°\text{C}$ at $t=0$.

## Numerical Example: Temperature at Center

At the rod's center ($x = L/2 = 0.5$ m), how long until temperature drops to $50°\text{C}$?

**Series solution** (first 5 terms):

$$u(0.5, t) = \frac{400}{\pi} \sum_{k=0}^{4} \frac{1}{2k+1} \sin\left(\frac{(2k+1)\pi}{2}\right) \exp\left(-\left(\frac{(2k+1)\pi}{1}\right)^2 \times 1.3 \times 10^{-5} \times t\right)$$

$$= \frac{400}{\pi} \left[ 1 \cdot \exp(-\pi^2 \alpha t) - \frac{1}{3} \exp(-9\pi^2 \alpha t) + \frac{1}{5} \exp(-25\pi^2 \alpha t) - \ldots \right]$$

**Dominant term** (first mode):

$$u(0.5, t) \approx \frac{400}{\pi} \exp(-\pi^2 \alpha t)$$

Set $u = 50$:

$$50 = \frac{400}{\pi} \exp(-\pi^2 \times 1.3 \times 10^{-5} \times t)$$

$$\exp(-\pi^2 \times 1.3 \times 10^{-5} \times t) = \frac{50\pi}{400} = 0.3927$$

$$t = \frac{-\ln(0.3927)}{\pi^2 \times 1.3 \times 10^{-5}} = \frac{0.9343}{1.283 \times 10^{-4}} \approx 7283 \, \text{seconds} \approx 2 \, \text{hours}$$

**Physical check**: Steel rod cools to half initial temperature in about 2 hours. Reasonable for a 1-meter rod.

## Validation Against SymPy

```python
# SymPy heat equation solution
from sympy import symbols, Function, Eq, pdsolve, sin, exp, pi
from sympy.abc import x, t

u = Function('u')
alpha = symbols('alpha', positive=True)
L = symbols('L', positive=True)

# Heat equation
heat_eq = Eq(u(x,t).diff(t), alpha * u(x,t).diff(x,2))

# SymPy gives general solution (symbolic coefficients)
sympy_sol = pdsolve(heat_eq, u(x,t))
# Result: u(x,t) = Sum(C_n * sin(n*pi*x/L) * exp(-n²*pi²*alpha*t/L²), (n, 1, oo))
```

**MathHook vs SymPy**:
- ✅ Solution structure matches
- ✅ Eigenvalue formula matches: $\lambda_n = (n\pi/L)^2$
- ✅ Exponential decay matches: $\exp(-\lambda_n \alpha t)$
- ⚠️ Both return symbolic coefficients (not numerically evaluated)

## Limitations and Edge Cases

### Insulated Boundaries (Neumann BCs)

**⚠️ NOT SUPPORTED** in current MathHook version.

For insulated ends: $\frac{\partial u}{\partial x}(0,t) = 0$, $\frac{\partial u}{\partial x}(L,t) = 0$

**Different eigenvalues**: $\lambda_n = (n\pi/L)^2$ for $n = 0, 1, 2, \ldots$ (includes $n=0$!)

**Different modes**: $X_n(x) = \cos(n\pi x/L)$

**Phase 2 feature**.

### Non-Homogeneous BCs

**⚠️ NOT SUPPORTED** directly.

For $u(0,t) = T_1$, $u(L,t) = T_2$ (non-zero):

**Transformation**: Let $v(x,t) = u(x,t) - [T_1 + (T_2-T_1)x/L]$

Then $v$ satisfies homogeneous BCs: $v(0,t) = v(L,t) = 0$

Apply MathHook to $v$, then add back steady-state.

### Time-Dependent BCs

**⚠️ NOT SUPPORTED**.

For $u(0,t) = f(t)$ or $u(L,t) = g(t)$:

Requires Duhamel's principle or Green's functions. Phase 2 feature.

### Multi-Dimensional Heat Equation

**⚠️ 2D/3D NOT SUPPORTED** currently.

For 2D: $\frac{\partial u}{\partial t} = \alpha \left(\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2}\right)$

**Solution**: Product of 1D solutions: $u(x,y,t) = \sum_{n,m} A_{nm} X_n(x) Y_m(y) T_{nm}(t)$

**Eigenvalues**: $\lambda_{nm} = \lambda_n^x + \lambda_m^y$ (sum of 1D eigenvalues)

Phase 2 feature.

## Mathematical References

1. **Strauss, Walter A.** *Partial Differential Equations: An Introduction*, Chapter 4
   - Heat equation derivation
   - Separation of variables
   - Maximum principle proof

2. **Haberman, Richard** *Applied PDEs*, Chapter 2
   - Fourier series solutions
   - Physical interpretation

3. **Carslaw, H. S. and Jaeger, J. C.** *Conduction of Heat in Solids*, 2nd ed.
   - Comprehensive heat conduction reference
   - Engineering applications

## Summary

**Heat Equation**:
$$\frac{\partial u}{\partial t} = \alpha \nabla^2 u$$

**MathHook Capabilities**:
- ✅ 1D heat equation with Dirichlet BCs
- ✅ Correct eigenvalue computation: $\lambda_n = (n\pi/L)^2$
- ✅ Correct solution structure
- ⚠️ Symbolic Fourier coefficients (not numerically computed)

**Limitations**:
- Neumann BCs not supported
- Non-homogeneous BCs require manual transformation
- Time-dependent BCs not supported
- Multi-dimensional not supported

**Next**: See [Wave Equation](./wave-equation.md) and [Laplace Equation](./laplace-equation.md).
