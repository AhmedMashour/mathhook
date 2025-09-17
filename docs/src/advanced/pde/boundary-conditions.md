# Boundary Conditions

## Types of Boundary Conditions

Boundary conditions (BCs) specify constraints on the PDE solution at domain boundaries. They determine eigenvalues and influence solution behavior.

### Dirichlet Boundary Conditions (Fixed Value)

**Definition**: Function value specified on boundary

$$u = g \quad \text{on } \partial\Omega$$

**Physical examples**:
- Fixed temperature (heat equation): rod end held at 0°C
- Fixed position (wave equation): string end clamped at zero displacement
- Fixed potential (Laplace equation): conductor maintained at constant voltage

**MathHook support**: ✅ **FULLY SUPPORTED**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Homogeneous: u(0,t) = 0
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);

// Non-homogeneous: u(L,t) = 100
let bc_right = BoundaryCondition::dirichlet(
    expr!(100),
    BoundaryLocation::Simple { variable: x, value: expr!(1) },
);
```

**Eigenvalues** (1D, homogeneous Dirichlet):

$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 1, 2, 3, \ldots$$

### Neumann Boundary Conditions (Fixed Derivative)

**Definition**: Normal derivative specified on boundary

$$\frac{\partial u}{\partial n} = h \quad \text{on } \partial\Omega$$

**Physical examples**:
- Insulated boundary (heat): no heat flux ($\partial u/\partial x = 0$)
- Free end (wave): no force constraint
- Flux specified (Laplace): given charge density

**MathHook support**: ⚠️ **NOT YET IMPLEMENTED** (Phase 2)

**Eigenvalues** (1D, homogeneous Neumann $u'(0) = u'(L) = 0$):

$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 0, 1, 2, \ldots$$

Note: $n=0$ is allowed (constant mode)!

**Eigenfunctions**:

$$X_n(x) = \cos\left(\frac{n\pi x}{L}\right)$$

### Robin (Mixed) Boundary Conditions

**Definition**: Linear combination of function and derivative

$$\alpha u + \beta \frac{\partial u}{\partial n} = g \quad \text{on } \partial\Omega$$

**Physical examples**:
- Convective cooling (heat): $-k \frac{\partial u}{\partial x} = h(u - u_{\text{ambient}})$ (Newton's law)
- Elastic support (wave): restoring force proportional to displacement

**MathHook support**: ⚠️ **NOT YET IMPLEMENTED** (Phase 2)

**Eigenvalues**: Transcendental equation (no closed form for general $\alpha, \beta$)

### Periodic Boundary Conditions

**Definition**: Function and derivatives match at endpoints

$$u(0,t) = u(L,t), \quad \frac{\partial u}{\partial x}(0,t) = \frac{\partial u}{\partial x}(L,t)$$

**Physical examples**:
- Circular domain (heat on ring)
- Periodic structures (wave in periodic medium)

**MathHook support**: ⚠️ **NOT YET IMPLEMENTED** (Phase 2)

**Eigenvalues**:

$$\lambda_n = \left(\frac{2n\pi}{L}\right)^2, \quad n = 0, 1, 2, \ldots$$

**Eigenfunctions**: Both $\sin$ and $\cos$ modes

## Boundary Condition Implementation

### BoundaryCondition Type

```rust
pub enum BoundaryCondition {
    Dirichlet {
        value: Expression,
        location: BoundaryLocation,
    },
    Neumann {  // ⚠️ NOT YET FUNCTIONAL
        derivative: Expression,
        location: BoundaryLocation,
    },
    Robin {    // ⚠️ NOT YET FUNCTIONAL
        alpha: Expression,
        beta: Expression,
        value: Expression,
        location: BoundaryLocation,
    },
}

pub enum BoundaryLocation {
    Simple {
        variable: Symbol,
        value: Expression,
    },
    // Future: Curved boundaries, multi-dimensional faces
}
```

### Current Limitations

**Only Dirichlet BCs work**:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ✅ WORKS:
let bc = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x, value: expr!(0) },
);

// ❌ EXISTS BUT NON-FUNCTIONAL:
let bc = BoundaryCondition::neumann(
    expr!(0),
    BoundaryLocation::Simple { variable: x, value: expr!(0) },
);
// Will compile but solver will ignore or error
```

**Workaround for Neumann**: Transform to Dirichlet problem (if possible).

Example: Insulated ends $u_x(0,t) = u_x(L,t) = 0$

Cannot directly solve, but eigenvalues are known: use cosine modes manually.

## Eigenvalue Computation

### Dirichlet: Sine Modes

For $u(0) = u(L) = 0$:

$$X''(x) + \lambda X(x) = 0, \quad X(0) = 0, \quad X(L) = 0$$

**Solution**:

$$X_n(x) = \sin\left(\sqrt{\lambda_n} x\right), \quad \sqrt{\lambda_n} = \frac{n\pi}{L}$$

**MathHook implementation** (`common/eigenvalues.rs`):

```rust
pub fn compute_dirichlet_1d_eigenvalues(
    boundary_conditions: &[BoundaryCondition],
    spatial_var: &Symbol,
    max_terms: usize,
) -> Result<Vec<Expression>, PDEError> {
    let L = extract_domain_length(boundary_conditions, spatial_var)?;

    let eigenvalues: Vec<_> = (1..=max_terms)
        .map(|n| {
            let n_expr = Expression::integer(n as i64);
            let pi = Expression::pi();
            // λₙ = (nπ/L)²
            Expression::pow(
                expr!(n_expr * pi * (L.clone() ^ -1)),
                Expression::integer(2),
            )
        })
        .collect();

    Ok(eigenvalues)
}
```

### Wave Equation: Same Eigenvalues, Different Interpretation

For wave equation $u_{tt} = c^2 u_{xx}$:

**Eigenvalues**: $\lambda_n = (n\pi/L)^2$ (same as heat!)

**But**: Temporal part is oscillatory, not decaying:

$$T_n(t) = A_n \cos(\omega_n t) + B_n \sin(\omega_n t)$$

where $\omega_n = c\sqrt{\lambda_n} = \frac{n\pi c}{L}$

## Non-Homogeneous Boundary Conditions

### Problem

Heat equation with $u(0,t) = T_1$, $u(L,t) = T_2$ (non-zero constants).

**Cannot directly use separation of variables** (BCs not homogeneous).

### Solution Strategy

**Step 1**: Find steady-state $u_s(x)$ satisfying BCs:

$$u_s(x) = T_1 + (T_2 - T_1) \frac{x}{L}$$

**Step 2**: Define $v(x,t) = u(x,t) - u_s(x)$

**Step 3**: $v$ satisfies homogeneous BCs:

$$v(0,t) = u(0,t) - u_s(0) = T_1 - T_1 = 0$$

$$v(L,t) = u(L,t) - u_s(L) = T_2 - T_2 = 0$$

**Step 4**: Solve for $v(x,t)$ using MathHook

**Step 5**: Recover $u(x,t) = v(x,t) + u_s(x)$

**MathHook does NOT automate this** currently (manual transformation needed).

## Time-Dependent Boundary Conditions

### Problem

Heat equation with $u(0,t) = f(t)$ (time-varying).

**Cannot use simple separation of variables**.

### Solution: Duhamel's Principle

Break into sequence of instantaneous problems and superpose.

**⚠️ NOT SUPPORTED** in MathHook (Phase 2).

## Multi-Dimensional Boundary Conditions

### 2D Rectangle

For Laplace equation on $[0,a] \times [0,b]$:

**Four edges**, each with BC:
- Left ($x=0$): $u(0,y) = g_1(y)$
- Right ($x=a$): $u(a,y) = g_2(y)$
- Bottom ($y=0$): $u(x,0) = g_3(x)$
- Top ($y=b$): $u(x,b) = g_4(x)$

**Strategy**: Decompose into four sub-problems (one non-homogeneous edge each), solve each, superpose.

**MathHook currently**: Handles all four edges but doesn't automate decomposition.

## Mathematical References

1. **Strauss** *PDEs*, Chapter 3 - Boundary conditions and eigenvalue problems
2. **Haberman** *Applied PDEs*, Chapter 2 - Heat equation with various BCs
3. **Churchill & Brown** *Fourier Series and Boundary Value Problems*, 8th ed.

## Summary

**Boundary Condition Types**:

| Type | Support | Eigenvalues | Eigenfunctions |
|------|---------|-------------|----------------|
| Dirichlet | ✅ Full | $(n\pi/L)^2$, $n \geq 1$ | $\sin(n\pi x/L)$ |
| Neumann | ⚠️ Phase 2 | $(n\pi/L)^2$, $n \geq 0$ | $\cos(n\pi x/L)$ |
| Robin | ⚠️ Phase 2 | Transcendental | Mixed |
| Periodic | ⚠️ Phase 2 | $(2n\pi/L)^2$ | $\sin, \cos$ |

**Workarounds**:
- Non-homogeneous: Manual transformation to homogeneous
- Time-dependent: Phase 2 feature
- Multi-dimensional: Manual decomposition

**Next**: [Registry System](./registry.md) for solver architecture.
