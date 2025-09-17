# Partial Differential Equations (PDEs)

## What Are PDEs?

Partial Differential Equations (PDEs) describe relationships involving functions of multiple variables and their partial derivatives. Unlike Ordinary Differential Equations (ODEs) which involve functions of a single variable, PDEs govern phenomena that vary in space **and** time.

### Mathematical Definition

A **second-order linear PDE** in two independent variables has the general form:

$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + D \frac{\partial u}{\partial x} + E \frac{\partial u}{\partial y} + Fu = G$$

where:
- $u(x,y)$ is the unknown function
- $A, B, C, D, E, F, G$ are coefficients (may depend on $x$, $y$, or $u$)
- $x, y$ are independent variables (typically spatial coordinates or time)

### Why PDEs Matter

PDEs are the mathematical language of:

1. **Physics**: Heat conduction, wave propagation, electromagnetic fields, quantum mechanics
2. **Engineering**: Structural analysis, fluid dynamics, signal processing, control systems
3. **Finance**: Option pricing (Black-Scholes), risk modeling
4. **Biology**: Population dynamics, pattern formation, reaction-diffusion systems
5. **Computer Graphics**: Image processing, surface modeling, fluid simulation

### Real-World Examples

#### Heat Equation (Parabolic)
**Describes**: How temperature distributes through materials over time

$$\frac{\partial u}{\partial t} = \alpha \nabla^2 u$$

**Applications**:
- Cooling of electronic components
- Thermal insulation design
- Metallurgical heat treatment
- Climate modeling

#### Wave Equation (Hyperbolic)
**Describes**: Oscillatory phenomena and wave propagation

$$\frac{\partial^2 u}{\partial t^2} = c^2 \nabla^2 u$$

**Applications**:
- Vibrating strings and membranes (musical instruments)
- Electromagnetic wave propagation
- Seismic wave analysis (earthquake prediction)
- Acoustics and sound engineering

#### Laplace's Equation (Elliptic)
**Describes**: Steady-state distributions (equilibrium states)

$$\nabla^2 u = 0$$

**Applications**:
- Electrostatic potential distribution
- Steady-state heat conduction
- Fluid flow around obstacles (potential flow)
- Gravitational fields

## MathHook PDE Module Capabilities

### What MathHook Provides (Version 7.5/10)

✅ **Core Functionality**:
- PDE classification via discriminant ($B^2 - 4AC$)
- Heat equation solver (1D, Dirichlet boundary conditions)
- Wave equation solver (1D, Dirichlet boundary conditions)
- Laplace equation solver (2D rectangular domains, Dirichlet boundary conditions)
- Eigenvalue computation for standard boundary conditions
- Registry-based solver dispatch (O(1) lookup)
- Symbolic solution representation

✅ **Mathematical Correctness**:
- Verified against SymPy reference implementation
- Correct eigenvalue formulas
- Proper separation of variables structure
- Accurate boundary condition handling

### Current Limitations (Honestly Documented)

⚠️ **Symbolic Fourier Coefficients**:
- Solutions contain symbolic coefficients ($A_1, A_2, A_3, \ldots$)
- Numerical evaluation requires symbolic integration (Phase 2)
- Example: Heat equation returns $u(x,t) = \sum A_n \sin(\lambda_n x) e^{-\lambda_n \alpha t}$ with $A_n$ symbolic

⚠️ **Limited Boundary Conditions**:
- Only Dirichlet (fixed value) boundary conditions fully supported
- Neumann (derivative) and Robin (mixed) BCs planned for Phase 2

⚠️ **Standard Equations Only**:
- Supports heat, wave, and Laplace equations
- General nonlinear PDEs not yet supported

⚠️ **Experimental Separation of Variables**:
- `separation_of_variables` module exists but is non-functional
- DO NOT use in production code

### When to Use MathHook for PDEs

✅ **Good Use Cases**:
- Educational demonstrations of PDE solution structure
- Prototyping analytical solutions
- Understanding eigenvalue behavior
- Symbolic manipulation of PDE solutions
- Learning separation of variables technique

❌ **Not Suitable For**:
- Numerical PDE solving (use FEniCS, deal.II, or finite difference methods)
- Nonlinear PDEs (requires specialized solvers)
- Complex geometries (limited to rectangles currently)
- Production systems requiring numerical coefficients

## Module Architecture

### Registry-Based Design

MathHook uses a **registry pattern** for O(1) solver lookup:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Create registry (auto-registers all solvers)
let registry = PDESolverRegistry::new();

// Define PDE
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let equation = expr!(add: x, t);  // Heat equation pattern
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification and solving
let solution = registry.solve(&pde)?;

println!("Solution: {}", solution.solution);
println!("Eigenvalues: {:?}", solution.get_eigenvalues());
```

**Benefits**:
- No hardcoded `match` statements for equation types
- Easy addition of custom solvers
- Priority-based solver selection
- Clean separation of concerns

### Solution Structure

All solvers return a unified `PDESolution` type:

```rust
pub struct PDESolution {
    /// Symbolic solution expression
    pub solution: Expression,

    /// PDE type (Elliptic/Parabolic/Hyperbolic)
    pub pde_type: PdeType,

    /// Solver-specific metadata
    pub metadata: SolutionMetadata,
}

pub enum SolutionMetadata {
    Heat {
        alpha: Expression,           // Thermal diffusivity
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>, // Symbolic: A_1, A_2, ...
    },
    Wave {
        wave_speed: Expression,      // Propagation speed c
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>, // Symbolic: A_1, B_1, A_2, B_2, ...
    },
    Laplace {
        x_eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>, // Symbolic: C_1, C_2, ...
    },
}
```

## Solution Methodology: Separation of Variables

All MathHook PDE solvers use the **separation of variables** technique:

### Step 1: Assume Product Solution

For example, heat equation:

$$u(x,t) = X(x) T(t)$$

### Step 2: Separate Variables

Substitute into PDE:

$$X(x) T'(t) = \alpha X''(x) T(t)$$

Divide by $X(x) T(t)$:

$$\frac{T'(t)}{\alpha T(t)} = \frac{X''(x)}{X(x)} = -\lambda$$

where $\lambda$ is the **separation constant** (eigenvalue).

### Step 3: Solve ODEs

This creates two ODEs:
- Spatial: $X''(x) + \lambda X(x) = 0$
- Temporal: $T'(t) + \lambda \alpha T(t) = 0$

### Step 4: Apply Boundary Conditions

Boundary conditions determine eigenvalues $\lambda_n$:

For Dirichlet BCs $u(0,t) = u(L,t) = 0$:

$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 1, 2, 3, \ldots$$

### Step 5: Construct General Solution

Superposition of eigensolutions:

$$u(x,t) = \sum_{n=1}^{\infty} A_n \sin\left(\frac{n\pi x}{L}\right) \exp\left(-\lambda_n \alpha t\right)$$

### Step 6: Match Initial Conditions (Phase 2)

Fourier coefficients from initial condition $u(x,0) = f(x)$:

$$A_n = \frac{2}{L} \int_0^L f(x) \sin\left(\frac{n\pi x}{L}\right) dx$$

**⚠️ MathHook currently returns symbolic $A_n$**. Numerical evaluation requires symbolic integration.

## Validation Against SymPy

MathHook solutions are validated against SymPy's `pdsolve()`:

```python
# SymPy validation example
from sympy import Function, symbols, Eq, pdsolve
from sympy.abc import x, t

u = Function('u')
alpha = symbols('alpha', positive=True)

# Heat equation
heat_eq = Eq(u(x, t).diff(t), alpha * u(x, t).diff(x, 2))
sympy_solution = pdsolve(heat_eq, u(x, t))

# Compare with MathHook output structure
```

**Validation Criteria**:
1. ✅ Eigenvalue formulas match SymPy
2. ✅ Solution structure matches SymPy
3. ✅ Boundary condition satisfaction verified
4. ⚠️ Fourier coefficients symbolic (both implementations)

## Mathematical References

MathHook PDE implementations follow standard textbook approaches:

1. **Strauss, Walter A.** *Partial Differential Equations: An Introduction*, 2nd ed. (2007)
   - Chapter 4: Heat Equation
   - Chapter 5: Wave Equation
   - Chapter 6: Laplace's Equation

2. **Evans, Lawrence C.** *Partial Differential Equations*, 2nd ed. (2010)
   - Authoritative graduate-level reference
   - Theoretical foundations

3. **Haberman, Richard** *Applied Partial Differential Equations*, 5th ed. (2012)
   - Engineering-focused treatment
   - Fourier series methods

4. **Farlow, S. J.** *Partial Differential Equations for Scientists and Engineers* (1993)
   - Accessible introduction
   - Many worked examples

## Next Steps

Continue to specific solver documentation:

- [PDE Classification](./classification.md) - Discriminant formula and equation types
- [Heat Equation](./heat-equation.md) - Thermal diffusion solver
- [Wave Equation](./wave-equation.md) - Wave propagation solver
- [Laplace Equation](./laplace-equation.md) - Equilibrium state solver
- [Fourier Coefficients](./fourier-coefficients.md) - Why they're symbolic
- [Boundary Conditions](./boundary-conditions.md) - Types and implementation
- [Registry System](./registry.md) - Solver architecture
- [SymPy Validation](./sympy-validation.md) - Verification workflow
- [Complete Examples](./examples.md) - Real-world applications
