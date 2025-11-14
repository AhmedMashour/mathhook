# SymPy Quick Reference: Key Capabilities at a Glance

Generated: October 20, 2025

## What SymPy Has (MathHook Should Target)

### Solvers (47+ equation types)

**General**:
- Algebraic equations with auto-classification
- Linear systems (Gaussian, LU decomposition)
- Polynomial systems (Gröbner basis)
- Nonlinear systems

**Differential Equations**:
- **ODEs**: 10+ equation types (separable, exact, Bernoulli, Riccati, linear, higher-order, systems)
- **PDEs**: Classification system + separation of variables
- **Recurrence**: General recurrence, polynomial, rational, hypergeometric
- **Diophantine**: Integer solutions
- **Inequalities**: Polynomial, rational, univariate

**Optimization**:
- Linear programming (simplex)
- Numerical solving (Newton, bisection)

---

### Integration (Comprehensive)

**Core**:
- Risch algorithm (1,857 lines) - Complete transcendental integration
- Heuristic fallback (26,706 lines) - Pattern matching
- Manual/educational (78,731 lines) - Step-by-step

**Transforms** (20+ integral transforms):
- Laplace, Fourier, Mellin, Hankel (all with inverse)
- Sine, cosine transforms
- Complete transform correspondence system

**Special**:
- Meijer G-function integration (80,775 lines)
- Hypergeometric integrals
- Delta functions, singularity functions

---

### Functions (Elementary + 20+ Special)

**Elementary**:
- Trig (sin, cos, tan, etc.) + inverse + hyperbolic
- Exp, log (with complex support)
- Powers, factorials, binomials

**Special Functions**:
- Bessel (J, Y, Hankel, modified)
- Gamma, beta, zeta (Riemann + Dirichlet eta)
- Error functions (erf, erfc, Ei, Si, Ci)
- Elliptic integrals (complete + incomplete)
- Hypergeometric (hyper, meijerg, appell)
- Mathieu functions
- Spherical harmonics
- B-splines

---

### Polynomials (Advanced)

- Factorization (modular + Galois theory)
- Gröbner bases (Buchberger, F5B, FGLM)
- Root finding (exact + numerical)
- Partial fractions
- Resultants (univariate + multivariate)
- Orthogonal polynomials (Legendre, Chebyshev, Hermite, Laguerre, Jacobi)

---

### Linear Algebra (Complete)

**Decompositions**:
- LU, QR, Cholesky
- SVD, eigenvalue decomposition
- Jordan normal form, rational canonical form
- Smith/Hermite normal forms

**Operations**:
- Determinant, trace, inverse, pseudoinverse
- RREF, row/column space, null space
- Eigenvalue problems, diagonalization

---

### Simplification (Comprehensive)

- Generic simplification (multi-strategy)
- Trigonometric simplification
- Radical/power simplification
- Rational function simplification
- Hypergeometric expansion (80,775 lines!)
- Common subexpression elimination
- Numeric simplification

---

### Physics Modules

**Quantum**:
- Quantum gates, circuits, Grover's algorithm
- Hilbert spaces, basis states
- Commutators, anticommutators, dagger
- Density matrices, bosons/fermions
- Pauli algebra, Wigner symbols

**Classical Mechanics**:
- Kane's equations, Lagrangian mechanics
- Joints, constraints, bodies
- Linearization around equilibrium
- Actuators, biomechanics

**Other**:
- Optics, HEP, hydrogen atom
- Vector calculus, continuum mechanics

---

### Combinatorics (Advanced)

- Permutations, combinations, partitions
- Gray codes, permutation groups
- Free groups, finitely presented groups, polycyclic groups
- Galois groups, coset enumeration
- Group homomorphisms

---

### Number Theory (Sophisticated)

- Integer factorization (Pollard's rho, quadratic sieve, ECM)
- Primality testing (Miller-Rabin)
- Continued fractions
- Modular arithmetic, CRT
- Elliptic curves
- Gaussian integers/rationals

---

### Statistics & Probability

- Continuous distributions (20+)
- Discrete distributions (10+)
- Joint distributions, stochastic processes
- Random matrices (Wigner, Haar ensembles)
- Probability, expectation, variance, entropy

---

### Calculus Analysis

- Singularities, monotonicity analysis
- Convexity, critical points, extrema
- Periodicity detection
- Finite differences
- Limits (with Gruntz algorithm)
- Fourier series expansion

---

### Series Analysis

- Taylor series, asymptotic series
- Power series operations
- Formal power series
- Residue computation
- Series acceleration

---

### Advanced Features

**Sets & Logic**:
- Set operations, intervals
- Boolean algebra, SAT solving
- CNF/DNF conversion

**Parsing**:
- LaTeX input
- Mathematica/Wolfram syntax
- Standard math notation

**Code Generation**:
- C, C++, Fortran, Python
- JavaScript, MATLAB
- LaTeX output

**Tensor Operations**:
- Indexed tensors
- Array operations
- Einstein summation
- Differential geometry (manifolds, forms)

**Algebra Structures**:
- Quaternions, octonions
- Clifford algebras
- Lie algebras

---

## Key Metrics

| Metric | Value |
|--------|-------|
| Total Lines of Code | 776,131 |
| Python Files | 1,549 |
| Test Directories | 66 |
| Major Modules | 45+ |
| Equation Types (Solvers) | 47+ |
| Integral Transforms | 20+ |
| Special Function Families | 20+ |
| Probability Distributions | 30+ |
| Development Years | 15+ |

---

## Critical Missing in MathHook (As of Oct 2025)

| Feature | Status | Why Needed |
|---------|--------|-----------|
| Risch Algorithm | Not implemented | Proves elementary integrability |
| ODE Classification | Not implemented | Auto-detect equation type |
| Gröbner Bases | Not implemented | Solve polynomial systems |
| Laplace/Fourier Transforms | Not implemented | Differential equation solving |
| PDE Solver | Not implemented | Partial differential equations |
| Special Functions (20+) | Limited | Advanced integration, physics |
| Quantum Mechanics | Not implemented | Physics applications |
| Combinatorics | Not implemented | Group theory, permutations |
| Statistics | Not implemented | Probability distributions |
| Code Generation | Not implemented | Export to other languages |

---

## Strategic Insights

### What Makes SymPy Strong

1. **Multiple Algorithms**: Each operation has fallback strategies
2. **Domain System**: Extensible number domains (QQ, ZZ, GF(p), algebraic numbers)
3. **Integration Investment**: 80K+ lines devoted to special function integrals
4. **Educational Design**: Returns unevaluated forms, step-by-step solutions
5. **Comprehensive Testing**: 66 test directories with edge case coverage
6. **Long Development**: 15+ years of production maturity

### What MathHook Should Prioritize

1. **Performance First**: Rust vs Python = 10-100x faster potential
2. **Correctness Guarantees**: Type system ensures mathematical properties
3. **Educational Focus**: Step-by-step built-in (not retrofit)
4. **Risch Algorithm**: Critical for integration completeness
5. **ODE/PDE Solving**: Classification-based solver dispatch
6. **Core Depth**: Excellence in (integration, solving, simplification) > breadth

### Development Lesson

Each SymPy domain took years of development:
- Risch algorithm: 1,857 lines of sophisticated math
- Integration transforms: 200K+ lines total
- Polynomial systems: 300K+ lines (solvers + polys)

MathHook should focus on **deep correctness** in fewer domains rather than attempting SymPy's full scope.

---

## Reference Files

Full detailed analysis: `SYMPY_CAPABILITY_ANALYSIS.md`
SymPy Source: `/Users/ahmedmashhour/Documents/work/math/sympy/`

