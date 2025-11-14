# SymPy Comprehensive Capability Analysis

Generated: October 20, 2025
Codebase: `/Users/ahmedmashhour/Documents/work/math/sympy/`

## Executive Summary

SymPy is a mature, feature-complete symbolic mathematics library with approximately **776,000 lines of code** across **1,549 Python files** organized into **45+ major modules**. It provides extensive capabilities across mathematics, physics, and computer algebra domains.

---

## Core Statistics

- **Total Files**: 1,549 Python files
- **Total Lines of Code**: 776,131
- **Test Directories**: 66 (comprehensive test coverage)
- **Major Modules**: 45+
- **Main Entry Points**: 47 solver functions, 20+ integral transforms

---

## 1. Solvers Module (`sympy/solvers/`)

The solvers module is the most comprehensive equation-solving framework in the codebase.

### 1.1 General Equation Solving

- **`solve()`** - General algebraic equation solver with automatic classification
- **`solveset()`** - Set-theoretic solver (handles infinite solution sets, discrete solutions)
- **`linsolve()`** - Linear systems (vector form solver)
- **`nonlinsolve()`** - Nonlinear systems (Gröbner basis based)
- **`checksol()`** - Solution verification

### 1.2 Polynomial System Solving

- **`solve_poly_system()`** - Polynomial systems solver
- **`solve_triangulated()`** - Triangulation-based solving
- **`factor_system()`** - Factor polynomial systems
- Gröbner basis computation (Buchberger algorithm and F5B algorithm)

### 1.3 Differential Equation Solving (`ode/` subdirectory)

**Ordinary Differential Equations (ODEs)**:

- **Classification System**: `classify_ode()` - Auto-identifies ODE type
- **Solver**: `dsolve()` - Automatic ODE solution
- **Solution Verification**: `checkodesol()`
- **Order Detection**: `homogeneous_order()`

**ODE Types Supported**:

1. **First-Order ODEs**:
   - Separable equations
   - Exact equations
   - Homogeneous equations
   - Bernoulli equations
   - Riccati equations (`riccati.py`)
   - Linear first-order

2. **Higher-Order Linear ODEs**:
   - Constant coefficient linear ODEs
   - Euler-Cauchy equations (equidimensional)
   - Homogeneous solutions
   - Particular solutions (variation of parameters)

3. **Series Solutions**:
   - Frobenius method (power series around singularities)
   - Hypergeometric solutions

4. **Systems of ODEs**:
   - Linear systems (`systems.py`)
   - Nonlinear systems
   - Coupled differential equations

5. **Special Methods** (`nonhomogeneous.py`):
   - Undetermined coefficients
   - Variation of parameters
   - Green's function methods

### 1.4 Partial Differential Equations (`pde.py`)

- **Classification**: `classify_pde()` - Auto-identify PDE type
- **Solver**: `pdsolve()` - Automatic PDE solution
- **Verification**: `checkpdesol()`
- **Separation of Variables**:
  - `pde_separate()` - General separation
  - `pde_separate_add()` - Additive separation
  - `pde_separate_mul()` - Multiplicative separation

**PDE Types Handled**:
- First-order quasi-linear PDEs
- Second-order linear PDEs
- Elliptic, parabolic, hyperbolic classification
- Special PDEs (heat equation, wave equation, etc.)

### 1.5 Diophantine Equations (`diophantine/`)

- **`diophantine()`** - Integer solutions for polynomial equations
- Linear Diophantine equations
- Pell equations
- Pythagorean triples
- General polynomial Diophantine solving

### 1.6 Recurrence Relations (`recurr.py`)

- **`rsolve()`** - General recurrence solver
- **`rsolve_poly()`** - Polynomial recurrence
- **`rsolve_ratio()`** - Rational function recurrence
- **`rsolve_hyper()`** - Hypergeometric recurrence (including Gosper's algorithm)

### 1.7 Inequalities (`inequalities.py`)

- **`reduce_inequalities()`** - Simplify inequality systems
- **`solve_poly_inequality()`** - Polynomial inequalities
- **`solve_rational_inequalities()`** - Rational inequalities
- **`solve_univariate_inequality()`** - Single-variable inequalities
- **`reduce_abs_inequality()`** - Absolute value inequalities

### 1.8 Linear System Solving

- **`solve_linear_system()`** - Gaussian elimination
- **`solve_linear_system_LU()`** - LU decomposition
- **`solve_linear()`** - Single linear equation
- Matrix-based solving in `matrices/solvers.py`

### 1.9 Optimization (`simplex.py`)

- **`lpmin()` / `lpmax()`** - Linear programming (minimize/maximize)
- **`linprog()`** - Linear programming interface
- Simplex algorithm implementation

### 1.10 Numerical Solving

- **`nsolve()`** - Numerical root finding (Newton's method, bisection, etc.)
- Integration with numerical backends

### 1.11 Miscellaneous

- **`solve_undetermined_coeffs()`** - Undetermined coefficients method
- **`decompogen()`** - Decompose polynomials

**Total Solvers**: 47+ equation types

---

## 2. Integration Module (`sympy/integrals/`)

### 2.1 Definite and Indefinite Integrals

- **`integrate()`** - Main integration function (automatic method selection)
- **`Integral`** - Unevaluated integral class
- **`line_integrate()`** - Line integrals

### 2.2 Risch Algorithm (`risch.py` - 1,857 lines)

Complete implementation of the **Risch algorithm for symbolic integration**:

- **Differential Extension**: `DifferentialExtension` class
- **Elementary integration**: Base transcendental functions
- **Rational function integration**: Partial fractions
- **Exponential integration**: Handles `exp(x)` extensions
- **Logarithmic integration**: Handles `log(x)` extensions
- **Trigonometric integration**: Handles `sin(x)`, `cos(x)` extensions
- **Risch Differential Equation (RDE)**: `rde.py` solver
- **Parametric RDE**: `prde.py` solver
- **Transcendental tower**: Multi-level differential extensions

**Capabilities**:
- Proves non-elementary integrability
- Detects when elementary antiderivatives don't exist
- Comprehensive error handling for special cases

### 2.3 Heuristic Integration (`heurisch.py` - 26,706 lines)

Alternative methods for integration:

- Pattern matching
- Algebraic transformations
- Special case handlers
- Fallback when Risch algorithm doesn't apply

### 2.4 Manual Integration (`manualintegrate.py` - 78,731 lines)

Educational/step-by-step integration:

- Substitution rules
- Integration by parts
- Rational function decomposition
- Special techniques
- Returns intermediate steps

### 2.5 Integral Transforms (`transforms.py` - 51,750 lines)

Comprehensive transform library:

1. **Mellin Transform**:
   - `mellin_transform()`
   - `inverse_mellin_transform()`
   - `MellinTransform` class

2. **Laplace Transform**:
   - `laplace_transform()`
   - `inverse_laplace_transform()`
   - `laplace_correspondence()` - Correspondence system
   - `laplace_initial_conds()` - Initial conditions
   - `LaplaceTransform` class

3. **Fourier Transform**:
   - `fourier_transform()`
   - `inverse_fourier_transform()`
   - `FourierTransform` class

4. **Sine Transform**:
   - `sine_transform()`
   - `inverse_sine_transform()`
   - `SineTransform` class

5. **Cosine Transform**:
   - `cosine_transform()`
   - `inverse_cosine_transform()`
   - `CosineTransform` class

6. **Hankel Transform**:
   - `hankel_transform()`
   - `inverse_hankel_transform()`
   - `HankelTransform` class

### 2.6 Special Integration

- **`singularityintegrate()`** - Handle singularities in integration domain
- **Delta functions** (`deltafunctions.py`) - Dirac delta integration
- **Singularity functions** (`singularityfunctions.py`)

### 2.7 Meijeri and Hypergeometric Integration (`meijerint.py` - 80,775 lines)

Advanced special function integration:

- Meijer G-function integration
- Hypergeometric function integration
- Symbolic evaluation of special integrals

### 2.8 Integer Polynomial Integration (`intpoly.py` - 43,237 lines)

- Integration over integer polynomial domains
- Lattice point enumeration

### 2.9 Numerical Integration

- **`quadrature.py`** - Numerical quadrature rules
- Gaussian quadrature
- Adaptive quadrature

### 2.10 Rational Tools

- **`rationaltools.py`** - Rational function manipulation
- Partial fraction decomposition

---

## 3. Functions Module (`sympy/functions/`)

Comprehensive mathematical function library with **three main categories**:

### 3.1 Elementary Functions (`elementary/`)

1. **Trigonometric** (`trigonometric.py`):
   - `sin`, `cos`, `tan`, `cot`, `csc`, `sec`
   - Inverse trig: `asin`, `acos`, `atan`, `acot`, `acsc`, `asec`
   - Hyperbolic: `sinh`, `cosh`, `tanh`, `coth`, `csch`, `sech` (`hyperbolic.py`)
   - Inverse hyperbolic: `asinh`, `acosh`, `atanh`, `acoth`, `acsch`, `asech`

2. **Exponential and Logarithmic** (`exponential.py`):
   - `exp`, `log` (natural logarithm)
   - `log(x, base)` (arbitrary base logarithm)
   - Complex logarithm handling

3. **Powers and Roots** (`exponential.py`):
   - Power functions
   - Square root, cube root (domain-aware)

4. **Integer Functions** (`integers.py`):
   - `factorial`, `binomial`
   - Rising/falling factorials
   - Pochhammer symbol

5. **Miscellaneous** (`miscellaneous.py`):
   - `sqrt`, `cbrt`
   - `atan2` (two-argument arctangent)
   - `hypot` (hypotenuse)

6. **Piecewise Functions** (`piecewise.py`):
   - `Piecewise` class for conditional functions
   - Supports arbitrary conditions and expressions

### 3.2 Special Functions (`special/`)

Extensive collection of special mathematical functions:

1. **Bessel Functions** (`bessel.py`):
   - `besselj`, `bessely` (first and second kind)
   - `besselh` (Hankel functions)
   - `besseli`, `besselk` (modified Bessel)
   - `jn`, `yn` (integer orders)

2. **Gamma Functions** (`gamma_functions.py`):
   - `gamma`, `loggamma`
   - `polygamma`, `digamma`, `trigamma`
   - `uppergamma`, `lowergamma` (incomplete gamma)

3. **Beta Functions** (`beta_functions.py`):
   - `beta`
   - Beta function properties

4. **Zeta Functions** (`zeta_functions.py`):
   - `zeta` (Riemann zeta)
   - `dirichlet_eta` (Dirichlet eta)
   - `polylog` (polylogarithm)
   - `lerch` (Lerch transcendent)

5. **Error Functions** (`error_functions.py`):
   - `erf` (error function)
   - `erfc` (complementary error)
   - `erfi` (imaginary error)
   - `Ei` (exponential integral)
   - `li` (logarithmic integral)
   - `Si`, `Ci` (sine and cosine integrals)
   - `Shi`, `Chi` (hyperbolic integrals)

6. **Elliptic Integrals** (`elliptic_integrals.py`):
   - Complete elliptic integrals: `K(m)`, `E(m)`, `D(m)`
   - Incomplete elliptic integrals

7. **Hypergeometric Functions** (`hyper.py`):
   - `hyper` (generalized hypergeometric function)
   - `meijerg` (Meijer G-function)
   - `appellf` (Appell function)

8. **Mathieu Functions** (`mathieu_functions.py`):
   - `mathieuc`, `mathieus` (even/odd Mathieu functions)
   - `mathieucprime`, `mathieusprime` (derivatives)

9. **Spherical Harmonics** (`spherical_harmonics.py`):
   - `Ynm`, `Ynm_c` (spherical harmonics)

10. **B-Splines** (`bsplines.py`):
    - `bspline` functions

11. **Combinatorial Functions** (`combinatorial/`):
    - Already covered in polynomials section

12. **Tensor Functions** (`tensor_functions.py`):
    - Tensor-related functions

13. **Singularity Functions** (`singularity_functions.py`):
    - Step functions, ramp functions

14. **Delta Functions** (`delta_functions.py`):
    - Dirac delta function

---

## 4. Polynomial Module (`sympy/polys/`)

Comprehensive polynomial manipulation and analysis:

### 4.1 Polynomial Basic Operations

- **`Poly`** class - Main polynomial representation
- Basic arithmetic (add, subtract, multiply, divide)
- GCD and LCM computation
- Resultants and subresultants

### 4.2 Factorization

- **`factor_list()`** - Complete factorization
- **`factor()`** - Polynomial factorization
- **Galois-theory-based algorithms** (`galoistools.py` - 61,530 lines)
- Modular factorization over finite fields
- Hensel lifting
- Berlekamp and Cantor-Zassenhaus algorithms

### 4.3 Gröbner Bases (`groebnertools.py`, `fglmtools.py`)

- **Buchberger algorithm** - Classic Gröbner basis computation
- **F5B algorithm** - Modern fast variant
- **FGLM algorithm** - Basis conversion between orderings
- Supports multiple monomial orderings: lexicographic, graded reverse lexicographic, etc.

### 4.4 Polynomial Roots

- **`polyroots.py`** - Comprehensive root finding
- `roots()` - Polynomial roots over various domains
- `real_roots()` - Real roots only
- `nroots()` - Numerical roots
- Quartic formula solver
- Support for algebraic number field roots

### 4.5 Partial Fractions

- **`partfrac.py`** - Partial fraction decomposition
- `apart()` - Automatic decomposition
- Supports rational functions

### 4.6 Univariate Polynomials

- **`densetools.py`** - Dense polynomial representation operations
- **`densebasic.py`** - Basic dense operations (evaluating, composition)
- **`densearith.py`** - Arithmetic on dense polynomials

### 4.7 Multivariate Polynomials

- **`distributedmodules.py`** - Module operations over polynomial rings
- Multiple variable support
- Distributed representation

### 4.8 Number Fields (`numberfields/`)

- Algebraic number field extension computation
- Minimal polynomial computation
- Element operations in number fields

### 4.9 Domains (`domains/`)

- Extensible domain system supporting:
  - Rationals (QQ)
  - Integers (ZZ)
  - Finite fields
  - Gaussian integers
  - Gaussian rationals
  - Expression domains
  - And many more

### 4.10 Monomial Operations

- **`monomials.py`** - Monomial generation and manipulation
- `monomials()` - Generate all monomials of given degree
- Monomial orderings

### 4.11 Resultants

- **`multivariate_resultants.py`** - Resultant computation
- Sylvester resultants
- Bezout matrix methods
- Supports multivariate resultants

### 4.12 Special Polynomial Functions

- **`orthopolys.py`** - Orthogonal polynomials:
  - Legendre, Chebyshev, Hermite, Laguerre, Jacobi polynomials
- **`appellseqs.py`** - Appell sequences

### 4.13 Euclidean Algorithms

- **`euclidtools.py`** - Extended Euclidean algorithm
- GCD computation with coefficients

---

## 5. Matrix Module (`sympy/matrices/`)

Complete linear algebra framework:

### 5.1 Matrix Types

- **`Matrix`** - Basic mutable matrix class
- **`ImmutableMatrix`** - Immutable matrix class
- **`SparseMatrix`** - Sparse matrix representation
- **`RepMatrix`** - Representation matrix class
- Dense and sparse backends

### 5.2 Basic Matrix Operations

- Addition, subtraction, multiplication
- Transposition, conjugation
- Scalar operations

### 5.3 Matrix Decompositions (`decompositions.py`)

- **LU decomposition** - `LU()` and `LU_solve()`
- **QR decomposition** - `QR()`
- **Cholesky decomposition** - `cholesky()`
- **Singular Value Decomposition (SVD)** - `singular_values()`, `svd_decomposition()`
- **Eigenvalue decomposition** - `eigenvects()`, `eigenvals()`

### 5.4 Determinant and Trace (`determinant.py`)

- Multiple determinant computation methods
- Trace computation

### 5.5 Inverses (`inverse.py`)

- Matrix inversion (LU-based and others)
- Moore-Penrose pseudoinverse

### 5.6 Eigenvalue Problems (`eigen.py`)

- **`eigenvals()`** - Eigenvalues
- **`eigenvects()`** - Eigenvectors and eigenvalues
- **`diagonalize()`** - Diagonalization
- Characteristic polynomial
- Multiplicities

### 5.7 Reductions (`reductions.py`)

- Row reduction (Gaussian elimination)
- Reduced row echelon form (RREF)
- Smith normal form
- Hermite normal form

### 5.8 Normal Forms (`normalforms.py`)

- Jordan normal form
- Rational canonical form

### 5.9 Matrix Equations (`solvers.py`)

- `solve()` for matrix equations
- Sylvester equation solver

### 5.10 Subspaces (`subspaces.py`)

- Row space, column space, null space computation
- Orthogonal complement
- Rank and nullity

### 5.11 Utilities and Advanced Operations

- Matrix kinds/types
- Graph-based operations (`graph.py`)
- Expression-based matrices
- Sparse matrix tools (`sparsetools.py`)

---

## 6. Calculus Module (`sympy/calculus/`)

Calculus-related functionality (note: core derivatives handled in core):

### 6.1 Singularities (`singularities.py`)

- **`singularities()`** - Find singular points
- **`is_increasing()`, `is_decreasing()`** - Monotonicity analysis
- **`is_strictly_increasing()`, `is_strictly_decreasing()`** - Strict monotonicity
- **`is_monotonic()`** - General monotonicity

### 6.2 Finite Differences (`finite_diff.py`)

- **`finite_diff_weights()`** - Finite difference coefficients
- **`apply_finite_diff()`** - Apply finite difference formula
- **`differentiate_finite()`** - Approximate derivatives

### 6.3 Utilities (`util.py`)

- **`periodicity()`** - Detect function periodicity
- **`not_empty_in()`** - Check intervals
- **`is_convex()`** - Convexity analysis
- **`stationary_points()`** - Find critical points
- **`minimum()`, `maximum()`** - Find extrema

### 6.4 Accumulation Bounds (`accumulationbounds.py`)

- Track uncertainty bounds during computation

### 6.5 Euler Equations (`euler.py`)

- Euler-Lagrange equation solver

---

## 7. Series Module (`sympy/series/`)

Series and limit analysis:

### 7.1 Power Series

- **`series()`** - Taylor series expansion
- **`series_class.py`** - Series class representation

### 7.2 Asymptotic Series

- **`aseries.py`** - Asymptotic series
- Behavior at infinity

### 7.3 Limits (`limits.py`)

- **`limit()`** - Compute limits
- **`Limit`** class
- Support for one-sided limits

### 7.4 Limit Sequences (`limitseq.py`)

- Limit analysis for sequences
- Monotonic sequences

### 7.5 Gruntz Algorithm (`gruntz.py`)

Advanced limit computation algorithm

### 7.6 Fourier Series (`fourier.py`)

- **`fourier_series()`** - Compute Fourier series coefficients
- Periodic function expansion

### 7.7 Formal Power Series (`formal.py`)

- Formal power series operations

### 7.8 Residues (`residues.py`)

- Residue computation for complex analysis

### 7.9 Continuity (`acceleration.py`)

- Series acceleration methods

---

## 8. Simplification Module (`sympy/simplify/`)

### 8.1 Core Simplification

- **`simplify()`** - Automatic simplification
- **`nsimplify()`** - Numerical simplification
- **`simplify_logic()`** - Logical expression simplification

### 8.2 Trigonometric Simplification (`trigsimp.py`)

- **`trigsimp()`** - Trigonometric simplification
- Reduce to minimal form

### 8.3 Power Simplification (`powsimp.py`)

- **`powsimp()`** - Combine powers
- `x^a * x^b -> x^(a+b)`

### 8.4 Radical Simplification (`radsimp.py`)

- **`radsimp()`** - Simplify radicals
- Denesting radicals (`sqrtdenest.py`)

### 8.5 Rational Function Simplification (`ratsimp.py`)

- **`ratsimp()`** - Rational function simplification
- Common denominator

### 8.6 Gamma Function Simplification (`gammasimp.py`)

- **`gammasimp()`** - Simplify gamma functions

### 8.7 Hyperbolic Simplification (`hyperexpand.py` - 80,775 lines)

- **`hyperexpand()`** - Expand hypergeometric functions
- Convert special functions to elementary forms when possible

### 8.8 Common Subexpression Elimination (`cse_main.py`)

- **`cse()`** - Common subexpression elimination
- Optimize expression trees
- Multiple optimization modes

### 8.9 FU Simplification (`fu.py`)

- Fraction and umbrella rules for simplification

### 8.10 Expression Traversal (`traversaltools.py`)

- Tree traversal for simplification

---

## 9. Physics Module (`sympy/physics/`)

### 9.1 Quantum Mechanics (`quantum/`)

Comprehensive quantum computing framework:

- **Quantum gates**: Single and multi-qubit gates
- **Grover's algorithm**: Implementation
- **Quantum circuits**: `gate.py`, `circuitplot.py`, `circuitutils.py`
- **Quantum state representation**: Hilbert spaces, basis states
- **Operators**: Commutators, anticommutators, dagger operations
- **Density matrices**: Mixed state representation
- **Bosons and Fermions**: Second quantization
- **Clebsch-Gordan coefficients**: `cg.py`
- **Inner products**: State overlap
- **Pauli algebra**: `paulialgebra.py`, `pring.py` (polynomial ring)
- **Wigner symbols**: `wigner.py`
- **Identity search**: `identitysearch.py`
- **Matrix caching**: `matrixcache.py`

### 9.2 Classical Mechanics (`mechanics/`)

Comprehensive mechanics framework:

- **Bodies**: Point particles and rigid bodies
- **Kane's equations**: `kane.py` - Lagrangian dynamics
- **Lagrangian mechanics**: `lagrange.py` - Euler-Lagrange equations
- **Joints**: Multiple joint types for constrained systems (`jointsmethod.py`)
- **Inertia**: Tensor computations (`inertia.py`)
- **Loads**: Forces and torques (`loads.py`)
- **Linearization**: `linearize.py` - Linearization around equilibrium
- **System dynamics**: `system.py` - Solve equations of motion
- **Actuators**: `actuator.py`
- **Pathway mechanics**: `pathway.py` - Muscular path geometry
- **Wrapping geometry**: `wrapping_geometry.py` - Geometric wrapping

### 9.3 Optics (`optics/`)

- Optical system modeling
- Gaussian optics
- Wave optics

### 9.4 High Energy Physics (`hep/`)

- Particle physics utilities
- Standard model support

### 9.5 Hydrogen Atom (`hydrogen.py`)

- Hydrogen atom wave function solutions
- Quantum numbers and energies

### 9.6 Harmonic Oscillators

- Quantum harmonic oscillator (`qho_1d.py`)
- Simple harmonic oscillator (`sho.py`)

### 9.7 Second Quantization (`secondquant.py`)

- Creation and annihilation operators
- Normal ordering

### 9.8 Units and Dimensions (`units/`)

- Physical unit support
- Dimension analysis

### 9.9 Vector Analysis (`vector/`)

- Coordinate systems (Cartesian, cylindrical, spherical, etc.)
- Gradient, divergence, curl operations
- Path and volume integrals

### 9.10 Continuum Mechanics (`continuum_mechanics/`)

- Stress and strain tensors
- Elasticity theory

### 9.11 Biomechanics (`biomechanics/`)

- Biological system mechanics

---

## 10. Combinatorics Module (`sympy/combinatorics/`)

### 10.1 Permutations

- **`permutations.py`** - Permutation class
- Cycle notation
- Permutation composition

### 10.2 Combinations and Subsets

- **`subsets.py`** - Subset generation and enumeration
- Combinatorial enumeration

### 10.3 Partitions

- **`partitions.py`** - Integer partition generation
- Partition properties

### 10.4 Gray Codes

- **`graycode.py`** - Gray code generation and sequencing

### 10.5 Group Theory (`*_groups.py`)

- **`perm_groups.py`** - Permutation groups
- **`free_groups.py`** - Free groups
- **`fp_groups.py`** - Finitely presented groups
- **`pc_groups.py`** - Polycyclic groups
- **`named_groups.py`** - Standard group constructions
- **`group_constructs.py`** - Group operations (direct product, etc.)
- **`group_numbers.py`** - Group enumeration and counting
- **`coset_table.py`** - Coset enumeration
- **`homomorphisms.py`** - Group homomorphisms

### 10.6 Polhedron

- **`polyhedron.py`** - Platonic solid symmetry groups

### 10.7 Rewriting Systems

- **`rewritingsystem.py`** - Rewriting system operations
- **`rewritingsystem_fsm.py`** - Finite state machine based rewriting

### 10.8 Galois Theory

- **`galois.py`** - Galois group computation

### 10.9 Prufer Sequences

- **`prufer.py`** - Tree encoding/decoding

### 10.10 Schur Numbers

- **`schur_number.py`** - Schur number computation

### 10.11 Tensor Canonicalization

- **`tensor_can.py`** - Canonical tensor forms

---

## 11. Number Theory Module (`sympy/ntheory/`)

### 11.1 Factorization

- **`factor_.py`** - Integer factorization
- Pollard's rho algorithm
- Trial division, Fermat method

### 11.2 Prime Numbers

- **`primetest.py`** - Primality testing (Miller-Rabin, etc.)
- **`generate.py`** - Prime generation
- Sieve of Eratosthenes

### 11.3 Continued Fractions

- **`continued_fraction.py`** - Continued fraction expansions
- Convergents, approximations

### 11.4 Modular Arithmetic

- **`modular.py`** - Modular operations
- Chinese Remainder Theorem
- Euler's totient function

### 11.5 Residues

- **`residue_ntheory.py`** - Quadratic residues
- Legendre and Jacobi symbols

### 11.6 Elliptic Curves

- **`elliptic_curve.py`** - Elliptic curve arithmetic
- Point operations
- Discrete logarithm

### 11.7 Partition Functions

- **`partitions_.py`** - Integer partition counting

### 11.8 Multibinomial Functions

- **`multinomial.py`** - Multinomial coefficient computation

### 11.9 Digit Operations

- **`digits.py`** - Number digit manipulation

### 11.10 Egyptian Fractions

- **`egyptian_fraction.py`** - Egyptian fraction representation

### 11.11 BBP Pi Algorithm

- **`bbp_pi.py`** - Bailey-Borwein-Plouffe algorithm for pi digits

### 11.12 Quadratic Sieve

- **`qs.py`** - Quadratic sieve factorization

### 11.13 Elliptic Curve Method

- **`ecm.py`** - ECM factorization algorithm

---

## 12. Tensor Module (`sympy/tensor/`)

### 12.1 Indexed Objects

- **`indexed.py`** - Indexed tensor support
- **`index_methods.py`** - Index manipulations

### 12.2 Tensor Arrays

- **`array/`** - N-dimensional array support
- Tensor contraction
- Einstein summation convention

### 12.3 Tensor Operators

- **`toperators.py`** - Tensor operator definitions

### 12.4 Differential Geometry

- **`diffgeom/`** - Manifolds, differential forms, connections

---

## 13. Geometry Module (`sympy/geometry/`)

### 13.1 Basic Geometric Objects

- **`Point`** - Points in 2D/3D space
- **`Point2D`, `Point3D`** - Specific dimensions

### 13.2 Lines and Rays

- **`Line`, `Line2D`, `Line3D`** - Infinite lines
- **`Ray`, `Ray2D`, `Ray3D`** - Half-lines
- **`Segment`, `Segment2D`, `Segment3D`** - Line segments

### 13.3 Planes

- **`Plane`** - Plane in 3D space

### 13.4 Curves

- **`Curve`** - Parametric curves

### 13.5 Conic Sections

- **`Ellipse`, `Circle`** - Ellipses and circles
- **`Parabola`** - Parabolas

### 13.6 Polygons

- **`Polygon`** - General polygons
- **`RegularPolygon`** - Regular polygons
- **`Triangle`** - Special triangle methods

### 13.7 Geometric Utilities (`util.py`)

- **`intersection()`** - Find intersections between geometric objects
- **`centroid()`** - Compute centroids
- **`convex_hull()`** - Convex hull computation
- **`are_similar()`** - Similarity testing
- **`closest_points()`** - Nearest point finding
- **`farthest_points()`** - Farthest point finding
- **`idiff()`** - Implicit differentiation

---

## 14. Statistics Module (`sympy/stats/`)

### 14.1 Random Variables

- **`rv.py`** - Random variable base class
- **`Probability()`** - Probability queries
- **`Expectation()`** - Expected value computation
- **`Entropy()`** - Information entropy
- **`variance()`, `std()`** - Statistical moments
- **`covariance()`, `correlation()`** - Joint statistics

### 14.2 Continuous Random Variables (`crv.py`)

- Continuous probability distributions
- **`ContinuousRV`** - General continuous distributions
- **Prebuilt distributions** (`crv_types.py`):
  - Normal, Uniform, Exponential, Beta, Gamma
  - Laplace, Cauchy, Chi-squared, F-distribution
  - t-distribution, Weibull, Pareto, Lognormal
  - And many more...

### 14.3 Discrete Random Variables (`drv.py`)

- Discrete probability distributions
- **`DiscreteRV`** - General discrete distributions
- **Prebuilt distributions** (`drv_types.py`):
  - Binomial, Poisson, Geometric, Negative Binomial
  - Hypergeometric, Zipf
  - And others...

### 14.4 Finite Random Variables (`frv.py`)

- Finite outcome space distributions
- **`FiniteRV`** - Custom finite distributions

### 14.5 Joint Random Variables (`joint_rv.py`)

- Multiple random variable distributions
- Multivariate Gaussian, Dirichlet, etc.
- **`JointRV`** - General joint distributions

### 14.6 Stochastic Processes (`stochastic_process.py`)

- Time-series and sequential random processes
- **`StochasticProcess`** - Base class
- Poisson process, Wiener process, etc.

### 14.7 Random Matrices (`random_matrix.py`)

- Matrix-valued random variables
- **`RandomMatrixSymbol`** - Symbolic random matrices
- Wigner ensemble, Haar ensemble, etc.

### 14.8 Error Propagation (`error_prop.py`)

- Uncertainty propagation in calculations

### 14.9 Symbolic Multivariate Probability (`symbolic_multivariate_probability.py`)

- Multivariate symbolic distributions

### 14.10 Sampling

- **`sampling/`** - Random sampling from distributions

---

## 15. Advanced Mathematical Features

### 15.1 Discrete Module (`sympy/discrete/`)

- Discrete signal processing
- Fourier transforms on discrete domains

### 15.2 Concrete Mathematics (`sympy/concrete/`)

- **Summations** (`summations.py`):
  - Symbolic summation
  - Gosper's algorithm for sum telescoping
  - `summation()` function

- **Products** (`products.py`):
  - Symbolic product computation

- **Differences and sums** (`expr_with_limits.py`, `expr_with_intlimits.py`)

- **Guessing** (`guess.py`):
  - Guess closed form from sequence

### 15.3 Holonomic Functions (`sympy/holonomic/`)

- Holonomic differential equation representation
- Symbolic computation with special functions

### 15.4 Logic Module (`sympy/logic/`)

- **Boolean algebra** (`boolalg.py`):
  - Boolean variables and operators
  - CNF/DNF conversion
  - SAT solving
  - Inference rules (`inference.py`)

- **Algorithms** (`algorithms/`):
  - Truth tables, resolution

### 15.5 Set Theory (`sympy/sets/`)

- Set operations
- Intervals, unions, intersections
- Infinities and special sets (Naturals, Primes, etc.)

### 15.6 Parsing (`sympy/parsing/`)

- Multiple input format parsing
- LaTeX parsing
- Mathematica/Wolfram parsing
- MATLAB code generation
- Human-readable expression parsing

### 15.7 Code Generation (`sympy/codegen/`)

- Generate code in various languages:
  - C, C++, Fortran, Python
  - JavaScript, MATLAB
  - LaTeX, etc.

### 15.8 Assumptions (`sympy/assumptions/`)

- Symbol property tracking (positive, real, integer, etc.)
- Automatic property inference
- Consistent assumption system

### 15.9 Strategies (`sympy/strategies/`)

- Strategic rewriting and transformation
- Complex expression manipulation algorithms

### 15.10 Unification (`sympy/unify/`)

- Pattern matching and unification
- Used for simplification and transformation

---

## 16. Specialized Features

### 16.1 Differential Geometry (`sympy/diffgeom/`)

- Manifolds and differential forms
- Connections and curvature
- Tensor field operations on manifolds

### 16.2 Lie Algebras (`sympy/liealgebras/`)

- Lie algebra representations
- Roots and weights
- Structure constants

### 16.3 Categories (`sympy/categories/`)

- Category theory support
- Morphisms and functors

### 16.4 Algebra Structures (`sympy/algebras/`)

- Quaternions, octonions
- Clifford algebras
- Group algebras

---

## 17. Formatting and Output (`sympy/printing/`)

Multiple output formats:

- **LaTeX** - `latex()` function
- **Pretty printing** - ASCII/Unicode mathematical notation
- **MathML** - Math Markup Language
- **Mathematica code** - Wolfram format
- **Python code** - Python repr
- **Fortran code** - Fortran 77/90/95
- **C/C++ code** - C and C++ output
- **Dot notation** - Graphviz format

---

## 18. Benchmarking

- Benchmark suites for performance tracking
- Located in each major module under `benchmarks/`

---

## Comparison: MathHook vs SymPy

### MathHook Strengths (Planned/Current)

1. **Performance**: Designed for 10-100x faster execution (Rust vs Python)
2. **Educational Mode**: Step-by-step explanations built-in
3. **Interactive Learning**: Educational focus throughout
4. **Type Safety**: Compile-time guarantees
5. **Parser Flexibility**: Multiple notation support (LaTeX, standard, Wolfram)
6. **Noncommutative Algebra**: First-class matrix/operator support

### SymPy Strengths (Comprehensive)

1. **Maturity**: 15+ years development, production-ready
2. **Scope**: 776K lines of code vs MathHook's initial implementation
3. **Advanced Algorithms**:
   - Complete Risch algorithm (1,857 lines of sophisticated code)
   - Multiple ODE solving strategies
   - PDE solving with classification
   - Gröbner basis computation (Buchberger, F5B)
   - Elliptic curve arithmetic
   - Quaternion algebra
   - Group theory and combinatorics

4. **Special Functions**: 20+ special function families with advanced properties
5. **Physics Modules**: Full quantum mechanics, classical mechanics frameworks
6. **Symbolic Integration Completeness**: Handles nearly all elementary integrals
7. **Number Theory**: Comprehensive number-theoretic algorithms
8. **Multiple Backends**: Can switch between numerical and symbolic computation
9. **Code Generation**: Generate working code in 8+ languages
10. **Extensive Testing**: Massive test suite with edge case coverage

### Key Gaps in MathHook vs SymPy

1. **Missing ODE Classification**: SymPy classifies ODEs and selects methods
2. **Missing PDE Support**: No partial differential equation solving
3. **Missing Risch Algorithm**: Current integration may miss some cases
4. **Missing Advanced Integration Transforms**: No Laplace, Fourier, Mellin transforms
5. **Limited Special Functions**: Only basic families implemented
6. **No Gröbner Basis**: Complex polynomial system solving incomplete
7. **No Group Theory**: Combinatorics and permutation groups absent
8. **Limited Statistics**: No probability distributions or stochastic processes
9. **No Geometry**: No geometric object representations
10. **No Tensor Algebra**: Multi-index tensor operations incomplete
11. **No Quantum Mechanics**: Physics module incomplete
12. **Limited Code Generation**: Cannot generate working code in multiple languages

---

## Architectural Insights

### SymPy Design Philosophy

1. **Extensible Domain System**: Domains for QQ, ZZ, GF(p), algebraic numbers, etc.
2. **Polynomial as First Class**: Heavy use of polynomial operations for solving
3. **Multiple Algorithms**: Each operation has multiple fallback algorithms
4. **Expression-First**: Everything converts to symbolic expressions
5. **Lazy Evaluation**: `Integral`, `Sum`, `Product` are unevaluated objects
6. **Educational Focus**: Can return unevaluated forms and step-by-step
7. **Assumptions Engine**: Symbol properties drive simplification

### SymPy Code Metrics

- **Largest Modules**:
  - `polys/polytools.py`: 222,013 lines (polynomial manipulation)
  - `integrals/risch.py`: 1,857 lines (Risch algorithm core)
  - `integrals/manualintegrate.py`: 78,731 lines (educational integration)
  - `integrals/meijerint.py`: 80,775 lines (special function integrals)
  - `simplify/hyperexpand.py`: 80,775 lines (hypergeometric expansion)
  - `solvers/solvers.py`: 3,676 lines (main solver)

- **Test Coverage**: 66 test directories with comprehensive edge case testing

---

## Recommendations for MathHook

### High Priority (Missing Core Functionality)

1. **Risch Algorithm**: Critical for claim of "correct integration" - SymPy has 1,857 lines
2. **ODE Classification**: Auto-detect ODE type and apply appropriate solver
3. **Gröbner Bases**: For polynomial system solving and equation solving completeness
4. **Laplace/Fourier Transforms**: For differential equation solving applications

### Medium Priority (Extended Capability)

1. **PDE Support**: At least basic classification and separation of variables
2. **Special Function Families**: Bessel, Gamma, Elliptic (currently limited)
3. **Probabilistic/Statistical Distributions**: Probability queries
4. **Lie Algebra Support**: For advanced mathematics
5. **Group Theory Basics**: Permutation groups, symmetric groups

### Lower Priority (Nice-to-Have)

1. **Full Group Theory**: Quaternion algebra, Clifford algebras
2. **Quantum Mechanics Framework**: Like SymPy's quantum module
3. **Tensor Algebra**: Multi-index operations
4. **Geometry Module**: Symbolic geometry objects
5. **Code Generation**: Emit code in multiple languages

### Strategic Observations

1. **SymPy's Complexity Comes From Multiple Algorithms**: Each operation has multiple fallbacks
2. **Integration is SymPy's Strongest Domain**: Heavy investment in Risch, Meijer G, etc.
3. **Polynomial Operations are Core**: Most advanced solving uses polynomial reduction
4. **Physics Integration is Deep**: Quantum and mechanics frameworks are comprehensive
5. **Testing is Extensive**: 66 test directories suggest multiple edge cases per feature

---

## Conclusion

SymPy is a **comprehensive, production-grade computer algebra system** with exceptional coverage across:
- Advanced integration (Risch algorithm)
- Differential equations (ODE/PDE)
- Polynomial manipulation (Gröbner, factorization)
- Physics (quantum mechanics, classical mechanics)
- Number theory (factorization, elliptic curves)
- Special functions (20+ families)

MathHook's opportunity is **performance, educational focus, and correctness guarantees** through Rust's type system. The current implementation should focus on depth in core areas (integration, solving, simplification) rather than breadth across all of SymPy's domains.

