# PDE Quick Reference Card

**One-page cheat sheet for Method of Characteristics**

---

## When to Use Method of Characteristics

| PDE Type | Method Applies? | Alternative |
|----------|----------------|-------------|
| First-order quasi-linear | ✅ YES | - |
| Second-order (heat, wave, Laplace) | ❌ NO | Separation of variables, Fourier |
| Fully nonlinear | ❌ NO | Specialized techniques |
| Two independent variables | ✅ YES | - |
| Three+ independent variables | ⚠️ COMPLEX | Requires generalization |

---

## Standard Forms

### Transport Equation
$$\frac{\partial u}{\partial t} + c \frac{\partial u}{\partial x} = 0$$

**Solution:** $u(x,t) = f(x - ct)$ where $f$ is initial condition

**Physical meaning:** Wave propagates right at speed $c$

---

### Burgers' Equation (Nonlinear)
$$\frac{\partial u}{\partial t} + u \frac{\partial u}{\partial x} = 0$$

**Solution:** $u = f(\xi)$ where $x = \xi + f(\xi)t$ (implicit)

**Warning:** Shocks can form! Use Rankine-Hugoniot for shock speed.

---

### General Quasi-Linear Form
$$a(x,y,u) \frac{\partial u}{\partial x} + b(x,y,u) \frac{\partial u}{\partial y} = c(x,y,u)$$

**Characteristic equations:**
$$\frac{dx}{ds} = a, \quad \frac{dy}{ds} = b, \quad \frac{du}{ds} = c$$

---

## 5-Step Solution Template

```
1. EXTRACT coefficients: a, b, c from PDE

2. BUILD characteristic system:
   dx/ds = a(x,y,u)
   dy/ds = b(x,y,u)
   du/ds = c(x,y,u)

3. SOLVE ODEs with IC: (x₀, y₀, u₀) = (ξ, 0, g(ξ))

4. ELIMINATE parameter s: solve for u(x,y)

5. VERIFY: Check PDE and IC satisfaction
```

---

## Quick Code Template

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Define symbols
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Build PDE
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let result = method_of_characteristics(&pde).unwrap();

// Apply IC and verify
let solution = expr!(f(x - c*t));  // Example for transport
```

---

## Common Patterns

### Pattern 1: Constant Coefficients
If $a$, $b$, $c$ are constants:
- Characteristics are **straight lines**
- Solution: $u = F(a x - b y)$ where $F$ determined by IC

### Pattern 2: Linear PDEs
If coefficients don't depend on $u$:
- Characteristics **don't intersect**
- **Smooth solution** exists globally

### Pattern 3: Nonlinear PDEs
If coefficients depend on $u$:
- Characteristics **can intersect** → **shocks form**
- Use weak solutions + Rankine-Hugoniot + entropy condition

---

## Shock Formation Checklist

**When do shocks form?**

1. ✅ PDE is nonlinear (coefficients depend on $u$)
2. ✅ Initial data has compression region ($\partial u_0/\partial x < 0$)
3. ✅ Characteristics with different slopes intersect

**Shock speed (Rankine-Hugoniot):**
$$\frac{dx_{shock}}{dt} = \frac{[f(u)]}{[u]} = \frac{f(u_R) - f(u_L)}{u_R - u_L}$$

**Entropy condition (Lax):**
$$f'(u_L) > s > f'(u_R)$$
(Characteristics converge INTO shock)

---

## Troubleshooting

| Error | Cause | Fix |
|-------|-------|-----|
| `InvalidVariableCount` | Not 2 independent variables | Use exactly 2 vars (e.g., t, x) |
| `NotFirstOrder` | PDE has second derivatives | Use separation of variables |
| `SingularCoefficients` | Both $a=0$ and $b=0$ | Check PDE formulation |
| Solution multi-valued | Characteristics intersect | Use weak solution + shock theory |
| Solution not smooth | Shock forms | Apply Rankine-Hugoniot condition |

---

## Reference Formulas

### Characteristic Equations
$$\frac{dx}{ds} = a, \quad \frac{dy}{ds} = b, \quad \frac{du}{ds} = c$$

### General Solution Form
$$u(x,y) = F(I_1, I_2)$$
where $I_1$, $I_2$ are **independent integrals** of characteristic equations

### Verification (PDE satisfaction)
$$a \frac{\partial u}{\partial x} + b \frac{\partial u}{\partial y} - c \stackrel{?}{=} 0$$

### Verification (IC satisfaction)
$$u(x, 0) \stackrel{?}{=} g(x)$$

---

## Physical Applications

| Application | PDE | Key Feature |
|-------------|-----|-------------|
| **Wave propagation** | $u_t + c u_x = 0$ | Rigid translation |
| **Traffic flow** | $\rho_t + (v(\rho) \rho)_x = 0$ | Shocks (traffic jams) |
| **Gas dynamics** | $u_t + u u_x = 0$ | Nonlinear steepening |
| **Groundwater transport** | $C_t + v C_x = 0$ | Contaminant advection |
| **Acoustics** | $p_t + c p_x = 0$ | Sound waves |

---

## Key References (with page numbers)

1. **Evans** (2010), *Partial Differential Equations*, 2nd ed.
   - Section 3.2: Characteristics (pp. 91-104)

2. **Strauss** (2008), *Partial Differential Equations*, 2nd ed.
   - Chapter 2: Waves and diffusions (pp. 22-55)

3. **Lax** (1973), *Hyperbolic Systems of Conservation Laws*
   - Sections 1-2: Shocks and weak solutions (pp. 1-18)

4. **Haberman** (2013), *Applied PDEs*, 5th ed.
   - Section 12.2: Method of Characteristics (pp. 545-561)
   - Section 12.4: Traffic flow (pp. 570-585)

---

## Decision Tree: Which Method?

```
Is PDE first-order?
├─ YES → Is it quasi-linear?
│         ├─ YES → METHOD OF CHARACTERISTICS ✓
│         └─ NO → Specialized nonlinear techniques
└─ NO → What order?
          ├─ Second-order → Separation of variables, Fourier
          ├─ Higher-order → Advanced techniques
          └─ System → Vector method of characteristics
```

---

## Performance Tips

1. **Reuse PDE structures:** Create `Pde` once, solve multiple times
2. **Adjust ODE step size:** Larger step = faster but less accurate
3. **Simplify sparingly:** Only when presenting results (expensive)
4. **Parallel characteristics:** Trace multiple characteristics in parallel

---

## Common Mistakes (Avoid!)

❌ **Wrong:** Mixing up coefficient order ($a$ vs $b$)
✅ **Right:** Write PDE in standard form first

❌ **Wrong:** Forgetting to apply initial condition
✅ **Right:** General solution $u = F(\xi)$, IC determines $F$

❌ **Wrong:** Using `Symbol::new("x")` instead of macros
✅ **Right:** Always use `symbol!(x)` and `expr!(...)`

❌ **Wrong:** Ignoring shock formation in nonlinear PDEs
✅ **Right:** Check for characteristic intersection, apply shock theory

---

## Educational Resources

**MathHook Documentation:**
- `PDE_USER_GUIDE.md` - Comprehensive tutorial
- `METHOD_OF_CHARACTERISTICS_TUTORIAL.md` - Deep dive
- `PDE_TECHNICAL_GUIDE.md` - Mathematical rigor with proofs

**Online:**
- SymPy PDE docs: https://docs.sympy.org/latest/modules/solvers/pde.html
- MIT OCW 18.303: Linear Partial Differential Equations

**Books (Ranked by Accessibility):**
1. Strauss (2008) - Undergraduate, accessible
2. Haberman (2013) - Applied, engineering focus
3. Evans (2010) - Graduate, rigorous proofs

---

**Print this page and keep it handy while solving PDEs!**

**Document Status:** ✅ One-page quick reference complete
