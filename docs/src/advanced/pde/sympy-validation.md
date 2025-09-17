# SymPy Validation Workflow

## Why SymPy is the Reference Implementation

**SymPy** (`~/Documents/work/math/sympy/`) is the **authoritative reference** for validating MathHook PDE solvers for the following reasons:

1. **Mature and Battle-Tested**: SymPy's PDE solving has been developed and refined over 15+ years
2. **Extensive Test Suite**: Thousands of test cases covering edge cases
3. **Academic Validation**: Used in research and education worldwide
4. **Well-Documented**: Clear mathematical foundations and algorithms
5. **Python MCP Available**: Can be queried programmatically for validation

**Important**: SymPy is used **internally** for validation only. Public documentation cites textbooks and papers, NOT SymPy.

## Validation Workflow

### Step 1: Define Problem in Both Systems

**MathHook**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let pde = Pde::new(expr!(u), u, vec![x.clone(), t]);
let alpha = expr!(1);

let solver = HeatEquationSolver::new();
let result = solver.solve_heat_equation_1d(&pde, &alpha, &bcs, &ic)?;
```

**SymPy**:
```python
from sympy import symbols, Function, Eq, pdsolve
from sympy.abc import x, t

u = Function('u')
alpha = symbols('alpha', positive=True)

heat_eq = Eq(u(x,t).diff(t), alpha * u(x,t).diff(x,2))
sympy_solution = pdsolve(heat_eq, u(x,t))
```

### Step 2: Compare Solution Structure

**MathHook output**:
```
Solution: A_1*sin(π*x)*exp(-π²*α*t) + A_2*sin(2π*x)*exp(-4π²*α*t) + ...
Eigenvalues: [π², 4π², 9π², ...]
Coefficients: [A_1, A_2, A_3, ...] (symbolic)
```

**SymPy output**:
```python
u(x,t) = Sum(C_n * sin(n*pi*x/L) * exp(-n²*pi²*alpha*t/L²), (n, 1, oo))
```

**Validation**:
- ✅ Structure matches (sine modes, exponential decay)
- ✅ Eigenvalue formula matches: $\lambda_n = (n\pi/L)^2$
- ✅ Both use symbolic coefficients
- ✅ Temporal behavior matches: $\exp(-\lambda_n \alpha t)$

### Step 3: Verify Eigenvalues Numerically

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// MathHook eigenvalues
let lambda_1 = result.eigenvalues[0].evaluate()?;  // π²
let lambda_2 = result.eigenvalues[1].evaluate()?;  // 4π²
let lambda_3 = result.eigenvalues[2].evaluate()?;  // 9π²
```

```python
# SymPy eigenvalues
import math
L = 1.0
lambda_1 = (math.pi / L)**2      # π²
lambda_2 = (2 * math.pi / L)**2  # 4π²
lambda_3 = (3 * math.pi / L)**2  # 9π²
```

**Comparison**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
assert!((lambda_1 - 9.8696).abs() < 1e-4);  // π² ≈ 9.8696
assert!((lambda_2 - 39.478).abs() < 1e-3);  // 4π² ≈ 39.478
```

### Step 4: Validate Boundary Condition Satisfaction

```python
# SymPy can verify BC satisfaction
solution = sympy_solution.rhs  # Right-hand side of equation

# Check u(0,t) = 0
bc_left = solution.subs(x, 0)
assert bc_left == 0

# Check u(L,t) = 0
bc_right = solution.subs(x, L)
assert bc_right == 0
```

MathHook returns symbolic solution → substitute and verify manually or programmatically.

## Validation Test Cases

### Heat Equation

**Test 1: Dirichlet BCs, constant IC**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
#[test]
fn test_heat_vs_sympy_dirichlet() {
    // MathHook solution
    let result = solve_heat_1d(...)?;

    // SymPy reference (computed offline)
    let expected_lambda_1 = 9.8696;  // π²

    // Validate
    let lambda_1 = result.eigenvalues[0].evaluate()?;
    assert!((lambda_1 - expected_lambda_1).abs() < 1e-4);
}
```

**Test 2: Different domain lengths**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
#[test]
fn test_heat_eigenvalues_scaling() {
    // L = 1: λ₁ = π²
    // L = 2: λ₁ = (π/2)² = π²/4
    // L = 0.5: λ₁ = (π/0.5)² = 4π²

    let L = 2.0;
    let result = solve_heat_1d_with_length(L)?;
    let expected = std::f64::consts::PI.powi(2) / 4.0;
    assert!((result.eigenvalues[0].evaluate()? - expected).abs() < 1e-4);
}
```

### Wave Equation

**Test 1: Standing wave frequencies**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
#[test]
fn test_wave_frequencies_vs_sympy() {
    let c = 340.0;  // m/s (speed of sound)
    let L = 1.0;    // m

    let result = solve_wave_1d(...)?;

    // SymPy: ω_n = n*π*c/L
    let omega_1 = std::f64::consts::PI * c / L;
    let f_1 = omega_1 / (2.0 * std::f64::consts::PI);  // Frequency in Hz

    // MathHook eigenvalues: λ_n = (nπ/L)²
    // ω_n = c*√λ_n = c*nπ/L
    let lambda_1 = result.eigenvalues[0].evaluate()?;
    let omega_mathhook = c * lambda_1.sqrt();

    assert!((omega_mathhook - omega_1).abs() < 1e-6);
}
```

### Laplace Equation

**Test 1: Rectangular domain eigenvalues**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
#[test]
fn test_laplace_2d_eigenvalues() {
    let a = 1.0;  // Width
    let b = 0.5;  // Height

    let result = solve_laplace_2d(...)?;

    // SymPy: λₙ = (nπ/a)²
    let expected_lambda_1 = (std::f64::consts::PI / a).powi(2);

    let lambda_1 = result.x_eigenvalues[0].evaluate()?;
    assert!((lambda_1 - expected_lambda_1).abs() < 1e-6);
}
```

## Known Differences (Acceptable)

### 1. Coefficient Representation

**SymPy**: Uses `Sum()` with index notation
```python
Sum(C_n * sin(n*pi*x/L) * exp(-n²*pi²*alpha*t/L²), (n, 1, oo))
```

**MathHook**: Expands finite sum explicitly
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
A_1*sin(π*x)*exp(-π²*α*t) + A_2*sin(2π*x)*exp(-4π²*α*t) + ...
```

**Why acceptable**: Both representations are mathematically equivalent. MathHook finite sum is more practical for numerical evaluation.

### 2. Variable Naming

**SymPy**: Uses function notation `u(x,t)`

**MathHook**: Uses symbol `u` with independent variables as context

**Why acceptable**: Notational difference only; mathematical content identical.

### 3. Symbolic vs Numerical Coefficients

**Both return symbolic coefficients** for Fourier series. SymPy requires separate `fourier_series()` call; MathHook plans integration in Phase 2.

**Why acceptable**: Both defer coefficient computation to avoid integration challenges.

## Validation Checklist

Before claiming a PDE solver is correct:

1. ✅ **Solution structure** matches SymPy (sine/cosine modes, exp/sinh/cosh temporal)
2. ✅ **Eigenvalue formula** matches SymPy (verified numerically)
3. ✅ **Boundary conditions** satisfied when substituted
4. ✅ **Initial conditions** structure correct (even if coefficients symbolic)
5. ✅ **Edge cases** tested (different domain lengths, BCs)
6. ✅ **Known limitations** documented (Neumann BCs, non-homogeneous BCs, etc.)

## SymPy MCP Integration

**Available via MCP**: SymPy can be queried programmatically for validation.

**Example workflow**:
```
1. Agent implements new MathHook PDE solver
2. Agent queries SymPy MCP for reference solution
3. Agent compares eigenvalues, solution structure
4. Agent verifies BCs/ICs satisfied
5. Agent documents any acceptable differences
6. Agent adds regression tests
```

**Important**: SymPy MCP is for **internal validation**, NOT cited in public documentation.

## Mathematical References (For Public Documentation)

When documenting PDE solvers, cite these instead of SymPy:

1. **Strauss, Walter A.** *Partial Differential Equations: An Introduction*, 2nd ed.
2. **Evans, Lawrence C.** *Partial Differential Equations*, 2nd ed.
3. **Haberman, Richard** *Applied Partial Differential Equations*, 5th ed.

## Summary

**SymPy Validation Workflow**:
1. Implement solver in MathHook
2. Compare solution structure with SymPy
3. Verify eigenvalues numerically
4. Test BC/IC satisfaction
5. Document acceptable differences
6. Add regression tests

**Validation Criteria**:
- ✅ Structure matches
- ✅ Eigenvalues match (numerical verification)
- ✅ BCs/ICs satisfied
- ⚠️ Symbolic coefficients acceptable (both implementations)

**Next**: [Complete Examples](./examples.md) for end-to-end workflows.
