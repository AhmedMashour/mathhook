# Method of Characteristics

The **Method of Characteristics** is the primary technique for solving first-order partial differential equations (PDEs). It transforms the PDE into a system of ordinary differential equations (ODEs) that can be solved along special curves called **characteristic curves**.

## Quick Overview

**Applies to:** First-order quasi-linear PDEs
**Equation form:** $$a(x,y,u) \frac{\partial u}{\partial x} + b(x,y,u) \frac{\partial u}{\partial y} = c(x,y,u)$$
**Key idea:** PDE becomes ODE along characteristic curves
**MathHook implementation:** `method_of_characteristics()` function

## Mathematical Foundation

### Geometric Interpretation

A first-order PDE defines a **direction field** in the $(x,y,u)$ space. The solution surface $u(x,y)$ must be tangent to this direction field everywhere.

**Characteristic curves** are integral curves of this direction field. Along each characteristic, the PDE reduces to an ODE that can be solved.

### The Characteristic System

For the quasi-linear PDE:
$$a(x,y,u) \frac{\partial u}{\partial x} + b(x,y,u) \frac{\partial u}{\partial y} = c(x,y,u)$$

The **characteristic equations** are:
$$\frac{dx}{ds} = a(x,y,u), \quad \frac{dy}{ds} = b(x,y,u), \quad \frac{du}{ds} = c(x,y,u)$$

where $s$ is a parameter along the characteristic curve.

### Solution Strategy

1. **Solve the characteristic system** of ODEs
2. **Obtain parametric solution** $(x(s), y(s), u(s))$
3. **Eliminate parameter $s$** to get implicit solution $u = u(x,y)$
4. **Apply initial/boundary conditions** to determine integration constants

## Complete Examples

### Example 1: Transport Equation

**PDE:** $\frac{\partial u}{\partial t} + c \cdot \frac{\partial u}{\partial x} = 0$

**Physical meaning:** Wave traveling at constant speed $c$

**Characteristic equations:**
$$\frac{dt}{ds} = 1, \quad \frac{dx}{ds} = c, \quad \frac{du}{ds} = 0$$

**Solution:**
- From first two: $x - ct = \text{constant}$
- From third: $u = \text{constant}$ along characteristics
- **General solution:** $u(x,t) = f(x - ct)$ where $f$ is arbitrary

**Initial condition:** If $u(x,0) = g(x)$, then $u(x,t) = g(x - ct)$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Transport equation PDE structure
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let solution = method_of_characteristics(&pde).unwrap();
println!("Solution: u(x,t) = F(x - ct)");

// With initial condition u(x,0) = sin(x):
println!("Specific solution: u(x,t) = sin(x - ct)");
```

### Example 2: Burgers' Equation (Nonlinear)

**PDE:** $\frac{\partial u}{\partial t} + u \frac{\partial u}{\partial x} = 0$

**Physical meaning:** Nonlinear wave with speed depending on amplitude

**Characteristic equations:**
$$\frac{dt}{ds} = 1, \quad \frac{dx}{ds} = u, \quad \frac{du}{ds} = 0$$

**Key insight:** $u$ is constant along each characteristic, but different characteristics have different speeds!

**Solution process:**
- From $\frac{du}{ds} = 0$: $u = u_0$ (constant along characteristic)
- From $\frac{dx}{ds} = u_0$: $x = u_0 t + x_0$
- **Implicit solution:** $u(x,t) = u_0$ where $x_0 = x - u_0 t$

**Initial condition:** $u(x,0) = f(x)$ gives $u(x,t) = f(x - u(x,t) \cdot t)$ (implicit!)

**Shock formation:** Characteristics can intersect, leading to discontinuities (shocks)

### Example 3: General Linear Case

**PDE:** $\frac{\partial u}{\partial x} + 2\frac{\partial u}{\partial y} = 3u$

**Characteristic equations:**
$$\frac{dx}{ds} = 1, \quad \frac{dy}{ds} = 2, \quad \frac{du}{ds} = 3u$$

**Solution:**
- From first two: $y = 2x + C_1$ (characteristic curves are straight lines)
- From third: $u = C_2 e^{3s}$
- Eliminating $s$: $u = C_2 e^{3x}$
- **General solution:** $u(x,y) = g(y - 2x) e^{3x}$ where $g$ is arbitrary

## MathHook Implementation

### Function Signature

```rust
pub fn method_of_characteristics(pde: &Pde) -> Result<PDESolution, PDEError>
```

### Current Capabilities

✅ **Supports:**
- Linear first-order PDEs
- Quasi-linear first-order PDEs
- Transport equations
- Constant coefficient PDEs

⚠️ **Limitations:**
- Returns general solution structure (may be implicit)
- Shock formation not automatically detected
- Some nonlinear PDEs require manual interpretation

### Usage Pattern

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Define PDE
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

let equation = /* build PDE expression */;
let pde = Pde::new(equation, u, vec![x, t]);

// Solve
match method_of_characteristics(&pde) {
    Ok(solution) => {
        println!("Solution: {}", solution.solution);
        // Apply initial conditions as needed
    }
    Err(e) => println!("Error: {:?}", e),
}
```

## When to Use Method of Characteristics

**✅ Use when:**
- PDE is first-order
- Need exact analytical solution
- Understanding wave propagation
- Educational demonstrations

**❌ Don't use when:**
- PDE is second-order (use Separation of Variables)
- Need numerical approximation (use finite differences)
- Complex nonlinear PDEs (may require specialized methods)

## Common Pitfalls

### 1. Implicit Solutions
Some PDEs yield **implicit solutions** that cannot be solved for $u$ explicitly.

**Example:** Burgers' equation gives $u(x,t) = f(x - u(x,t) \cdot t)$

**What to do:** Accept implicit form or use numerical methods

### 2. Shock Formation
**Characteristics can intersect** in nonlinear PDEs, causing **discontinuities (shocks)**.

**Example:** Burgers' equation with $u(x,0) = -x$ develops shock at $t = 1$

**What to do:** Use weak solutions or shock-capturing numerics

### 3. Parameter Elimination
Eliminating parameter $s$ can be non-trivial for complex characteristic systems.

**Strategy:** Look for first integrals or invariant combinations

## Further Reading

For comprehensive treatment:
1. **Strauss (2008)** - *Partial Differential Equations: An Introduction*, Chapter 1
2. **Evans (2010)** - *Partial Differential Equations*, Chapter 3
3. **John (1982)** - *Partial Differential Equations*, Chapters 2-3

For detailed examples and exercises:
- [User Guide](./user-guide.md) - Worked examples
- [Technical Guide](./technical-guide.md) - Mathematical rigor
- [Quick Start](./quick-start.md) - Code examples

## See Also

- [PDE Classification](./classification.md) - When to use this method
- [Separation of Variables](./separation-of-variables.md) - For second-order PDEs
- [Registry System](./registry.md) - How solvers are dispatched
- [Complete Examples](./examples.md) - Real-world applications
