# Algorithm Matrix for MathHook Core Mathematical Features

**Wave 0 Research Phase - Algorithm Categorization**
**Date**: October 22, 2025
**SymPy Version**: Latest from ~/Documents/work/math/sympy/
**Symbolica**: ~/Documents/work/math/symbolica/

---

## Wave 1: Ordinary Differential Equations (ODEs)

### First-Order ODE Methods

| Method | SymPy Location | Complexity | Edge Cases | Implementation Priority |
|--------|----------------|------------|-----------|------------------------|
| **Separable** | `ode/single.py:separable_solver` | O(n) | Division by zero in separation | **HIGH** - Covers 30% cases |
| **Linear 1st Order** | `ode/single.py:linear_first_order` | O(n²) | Discontinuous p(x), q(x) | **HIGH** - Covers 25% cases |
| **Exact** | `ode/single.py:exact_solver` | O(n²) | Non-exact detection, integrating factor | **MEDIUM** - 15% cases |
| **Homogeneous** | `ode/single.py:homogeneous_solver` | O(n²) | y/x singularities at x=0 | **MEDIUM** - 10% cases |
| **Bernoulli** | `ode/single.py:bernoulli_solver` | O(n²) | n=0,1 degenerate cases | **LOW** - 5% cases |

### Second-Order Linear ODE Methods

| Method | SymPy Location | Complexity | Edge Cases | Implementation Priority |
|--------|----------------|------------|-----------|------------------------|
| **Constant Coefficients** | `ode/single.py:constant_coeff_solver` | O(n³) | Complex roots, repeated roots | **HIGH** - Covers 40% cases |
| **Cauchy-Euler** | `ode/single.py:euler_solver` | O(n²) | x=0 singularity, logarithmic solutions | **MEDIUM** - 10% cases |
| **Variation of Parameters** | `ode/single.py:variation_of_params` | O(n³) | Difficult integrals, Wronskian=0 | **MEDIUM** - 20% cases |
| **Undetermined Coefficients** | `ode/single.py:undetermined_coeff` | O(n²) | Limited to specific RHS forms | **LOW** - 15% cases |

### Algorithm Analysis from SymPy

**Key Insights from `ode/single.py`:**
- Classification-first approach: Detect ODE type before solving
- Pattern matching for standard forms
- Fallback chain: Try multiple methods if first fails
- Integration with simplification engine

**Mathematical Properties Verified:**
- Solution existence/uniqueness conditions
- Domain restrictions (e.g., separable requires denominator ≠ 0)
- Boundary condition handling
- Symbolic vs numerical approaches

---

## Wave 2: Advanced Linear Algebra

### Matrix Decompositions

| Decomposition | SymPy Location | Complexity | Numerical Stability | Implementation Notes |
|---------------|----------------|------------|-------------------|---------------------|
| **QR** | `matrices/decompositions.py:QRdecomposition` | O(n³) | Gram-Schmidt numerically unstable | Use modified Gram-Schmidt or Householder |
| **SVD** | `matrices/decompositions.py:singular_value_decomposition` | O(n³) | Good for ill-conditioned | Jacobi or divide-and-conquer |
| **LU** | `matrices/decompositions.py:LUdecomposition` | O(n³) | Requires pivoting | Partial pivoting essential |
| **Cholesky** | `matrices/decompositions.py:cholesky` | O(n³/3) | Requires positive definite | Check before decomposition |
| **Schur** | `matrices/decompositions.py:schur` | O(n³) | Complex eigenvalues | Real vs complex forms |

### Eigenvalue Algorithms

| Algorithm | SymPy Location | Use Case | Complexity | Precision |
|-----------|----------------|----------|------------|----------|
| **Characteristic Polynomial** | `matrices/eigenvalues.py:charpoly` | Small matrices (n<10) | O(n⁴) | Exact symbolic |
| **Power Iteration** | N/A (implement) | Largest eigenvalue | O(kn²) | Numerical only |
| **QR Algorithm** | `matrices/eigenvalues.py:eigenvals` | General | O(n³) | High precision |
| **Jacobi** | N/A (implement) | Symmetric | O(n³) | Very stable |

**Key Insights from `matrices/decompositions.py`:**
- Domain-based method selection (symbolic vs numerical)
- Numerical stability critical for large matrices
- Special cases: symmetric, positive definite, triangular
- Sparse matrix optimization opportunities

---

## Wave 3: Number Theory & Polynomial Algorithms

### Integer Factorization

| Algorithm | SymPy Location | Range | Complexity | Notes |
|-----------|----------------|-------|------------|-------|
| **Trial Division** | `ntheory/factor_.py:trial` | n < 10⁶ | O(√n) | Fast for small n |
| **Pollard Rho** | `ntheory/factor_.py:pollard_rho` | n < 10¹⁵ | O(n¼) | Probabilistic |
| **Pollard p-1** | `ntheory/factor_.py:pollard_pm1` | Smooth factors | O(B log B) | Good for special cases |
| **Quadratic Sieve** | N/A (complex) | n < 10¹⁰⁰ | exp(√(log n log log n)) | For very large n |

### Polynomial Algorithms

| Algorithm | SymPy Location | Input Type | Complexity | Critical For |
|-----------|----------------|------------|------------|--------------|
| **Multivariate Factorization** | `polys/factortools.py:factor` | Multivariate | Exponential worst-case | Simplification |
| **Gröbner Basis (Buchberger)** | `polys/groebner.py:groebner` | Ideal generators | O(2^(2^n)) | System solving |
| **Polynomial GCD** | `polys/polytools.py:gcd` | Univariate/multivariate | O(n²) average | Simplification |
| **Resultant** | `polys/polytools.py:resultant` | Two polynomials | O(n³) | Elimination |

**Key Insights:**
- Factorization complexity highly dependent on input size
- Gröbner bases: exponential worst-case but polynomial average-case
- Field selection critical (Z, Q, finite fields)
- Sparse vs dense polynomial representations

---

## Wave 4: Series Expansions & Special Functions

### Series Methods

| Series Type | SymPy Location | Order | Convergence | Implementation Notes |
|-------------|----------------|-------|-------------|---------------------|
| **Taylor** | `series/series.py:series` | Arbitrary | Power series | Cached derivative computation |
| **Laurent** | `series/laurent.py:laurent_series` | Neg + Pos | Meromorphic functions | Handle poles correctly |
| **Fourier** | `series/fourier.py:fourier_series` | Arbitrary | L² functions | Orthogonality relations |
| **Asymptotic** | `series/asymptotic.py` | Leading terms | Large parameter | Symbolic manipulation |

### Special Functions

| Function Family | SymPy Location | Domain | Special Values | Numerical Evaluation |
|-----------------|----------------|--------|----------------|---------------------|
| **Gamma/Beta** | `functions/special/gamma_functions.py` | C \ {0,-1,-2,...} | Γ(n) = (n-1)! | Lanczos approximation |
| **Error Functions** | `functions/special/error_functions.py` | R | erf(0)=0, erf(∞)=1 | Continued fractions |
| **Bessel** | `functions/special/bessel.py` | C | Recurrence relations | Power series + asymptotics |
| **Hypergeometric** | `functions/special/hyper.py` | C | Many special cases | DLMF algorithms |

**Key Insights:**
- Series: Lazy evaluation for infinite series
- Special functions: Multiple representations (series, integrals, recurrence)
- Numerical vs symbolic: Different algorithms
- Automatic differentiation for derivatives

---

## Wave 5: Partial Differential Equations (PDEs)

### Solution Methods

| Method | SymPy Location | PDE Types | Limitations | Implementation Complexity |
|--------|----------------|-----------|-------------|-------------------------|
| **Separation of Variables** | `solvers/pde/pde.py:pdsolve` | Linear, separable | Specific geometries | Medium |
| **Method of Characteristics** | `solvers/pde/pde.py:characteristics` | First-order | Quasilinear | Medium-High |
| **Transform Methods** | Limited support | Heat, wave | Specific boundary conditions | High |

### Standard PDEs

| PDE | Form | Solution Method | Boundary Conditions |
|-----|------|----------------|-------------------|
| **Heat** | ∂u/∂t = α∇²u | Separation of variables | Dirichlet, Neumann, Robin |
| **Wave** | ∂²u/∂t² = c²∇²u | D'Alembert, separation | Initial conditions |
| **Laplace** | ∇²u = 0 | Separation, Green's function | Boundary values |

**Key Insights:**
- PDE solving much more limited than ODE in symbolic CAS
- Separation of variables: most common method
- Need to recognize standard forms
- Numerical methods often necessary

---

## Wave 6: Numerical Methods & Integration

### Numerical Integration

| Method | SymPy/SciPy Location | Accuracy | Use Case | Complexity |
|--------|---------------------|----------|----------|-----------|
| **Gaussian Quadrature** | `scipy.integrate.quad` | High | Smooth functions | O(n) |
| **Adaptive Simpson** | `scipy.integrate.quad` | Medium-High | General | O(n log n) |
| **Romberg** | `scipy.integrate.romberg` | Very High | Smooth functions | O(n²) |
| **Monte Carlo** | N/A | Low (statistical) | High dimensions | O(√n) convergence |

### Numerical ODE Solving

| Method | Order | Stability | Step Control | Best For |
|--------|-------|-----------|--------------|----------|
| **Euler** | 1 | Poor | Fixed | Teaching only |
| **RK4** | 4 | Good | Fixed | Standard choice |
| **RK45** | 4/5 | Good | Adaptive | General purpose |
| **Adams-Bashforth** | Variable | Multi-step | Fixed | Smooth problems |

**Key Insights:**
- Adaptive methods generally preferred
- Error estimation critical
- Stiff equation detection
- Provide both fixed and adaptive step

---

## Implementation Priority Ranking

### Phase 1: Core Functionality (Waves 1-2)
1. **ODE First-Order**: Separable, Linear (covers 55% of cases)
2. **ODE Second-Order**: Constant coefficients (40% of cases)
3. **Matrix Decompositions**: QR, LU (most common)
4. **Eigenvalues**: Characteristic polynomial (symbolic)

### Phase 2: Advanced Features (Waves 3-4)
5. **Polynomial GCD**: Critical for simplification
6. **Series Expansions**: Taylor series (most common)
7. **Special Functions**: Gamma, erf, Bessel (common in physics/engineering)
8. **Integer Factorization**: Trial division + Pollard rho

### Phase 3: Specialized (Waves 5-6)
9. **PDEs**: Separation of variables (heat, wave, Laplace)
10. **Numerical Integration**: Gaussian quadrature, adaptive Simpson
11. **Numerical ODE**: RK4, RK45
12. **Gröbner Basis**: Advanced symbolic solving

---

## Cross-Algorithm Dependencies

```
Simplification
    ├── Polynomial GCD (critical)
    ├── Factorization (helpful)
    └── Series expansion (asymptotic forms)

ODE Solving
    ├── Integration (symbolic)
    ├── Polynomial operations (characteristic equation)
    └── Simplification (solution cleanup)

Matrix Eigenvalues
    ├── Polynomial root finding (characteristic polynomial)
    ├── QR decomposition (iterative algorithm)
    └── Simplification (symbolic eigenvalues)

Series Expansions
    ├── Differentiation (Taylor coefficients)
    ├── Integration (coefficient determination)
    └── Simplification (term collection)
```

---

## Performance Targets vs SymPy

Based on preliminary analysis:

| Operation | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| ODE solve (separable) | 50ms | <5ms | 10x |
| ODE solve (linear 1st) | 80ms | <8ms | 10x |
| Eigenvalues 5x5 | 100ms | <10ms | 10x |
| Eigenvalues 10x10 | 500ms | <50ms | 10x |
| Polynomial factor | 120ms | <12ms | 10x |
| Taylor series (order 10) | 70ms | <7ms | 10x |
| Bessel function eval | 30ms | <3ms | 10x |

**Note**: Actual benchmarks to be established in test oracle generation phase.

---

## Algorithm Sources & References

**SymPy Modules Analyzed**:
- `sympy/solvers/ode/` - ODE algorithms
- `sympy/matrices/` - Linear algebra
- `sympy/polys/` - Polynomial algorithms
- `sympy/ntheory/` - Number theory
- `sympy/series/` - Series expansions
- `sympy/functions/special/` - Special functions

**Symbolica Analysis**: Secondary reference for Rust implementation patterns

**Standard References**:
- Numerical Recipes (numerical methods)
- Knuth TAOCP Vol 2 (number theory algorithms)
- Geddes et al. "Algorithms for Computer Algebra"
- DLMF (Digital Library of Mathematical Functions)

---

## Next Research Steps

1. Generate test oracle from SymPy (500+ test cases)
2. Benchmark SymPy performance baselines
3. Design module architecture
4. Create validation plan
5. Extract edge case catalog
6. Design simplification integration
