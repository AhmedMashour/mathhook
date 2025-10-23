# Plan 7: Core Mathematical Features Completion

**Priority**: ⚡ CRITICAL
**Timeline**: 24-36 weeks (updated from original 12-16 weeks estimate)
**Waves**: 7 (added Wave 0 for algorithm research)
**Orchestrator**: `/sc:spawn`

## Executive Summary

**Current State**: MathHook has solid foundation but missing critical CAS features:
- ❌ **Differential Equations**: No ODE/PDE solvers
- ❌ **Advanced Linear Algebra**: Missing SVD, QR, eigenvalue algorithms
- ⚠️  **Number Theory**: Basic functions only (missing advanced factorization, primes)
- ⚠️  **Special Functions**: Limited coverage beyond basic trig/exp/log
- ❌ **Series Expansions**: No Taylor/Fourier series
- ❌ **Polynomial Algorithms**: Missing Gröbner bases, advanced factorization

**Goal**: Complete core CAS functionality to match/exceed SymPy capabilities

**Competitive Gap**: Without these features, MathHook cannot replace SymPy for many use cases

---

## Bootstrap Command

```bash
/sc:spawn rust-engineer "Execute Wave-Based Core Mathematical Features Completion for MathHook"
```

**Orchestrator Prompt**:

```markdown
You are the Orchestrator for **MathHook Core Mathematical Features Completion**.

**Context**: You are the `rust-engineer` agent from `.claude/agents/rust-engineer.md` - Expert Rust developer specializing in mathematical algorithm implementation for MathHook CAS.

**Your Mission**: Execute a 6-wave plan to complete missing core CAS functionality: ODEs, PDEs, advanced linear algebra, number theory, special functions, and polynomial algorithms.

**Mandatory Reading** (in this order):
1. `/Users/ahmedmashhour/.claude/agents/rust-engineer.md` - Your agent specification
2. `/Users/ahmedmashhour/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven wave-based methodology
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` - Project constraints
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_7_CORE_MATH_FEATURES.md` - This plan

**Critical References**:
- SymPy algorithms (~/Documents/work/math/sympy/): Primary reference for correctness
- Symbolica (~/Documents/work/math/symbolica/): Secondary reference

**5 Mandatory Rules**:
1. **You Are Always The Orchestrator** - Delegate to rust-engineer agents
2. **Sequential Waves, Parallel Agents** - Complete waves in order
3. **Mandatory Verification** - Each wave ends with verification against SymPy
4. **Strict CLAUDE.md Enforcement** - Maintain 32-byte Expression, test 676/677 minimum
5. **Maintain Momentum** - Report after each wave
```

---

## Wave Breakdown

### Wave 0: Algorithm Research & Architecture (2-3 weeks)

**Goal**: Deep study of SymPy/Symbolica algorithms before implementation to avoid costly rewrites

**Rationale**: Mathematical algorithms are complex and have many edge cases. Rushing to implementation without understanding the full problem space leads to:
- Incorrect algorithms (mathematical bugs)
- Incomplete implementations (missing edge cases)
- Performance bottlenecks (wrong algorithmic approach)
- Costly rewrites (discovered after implementation)

**Research Strategy**: Study existing implementations as authoritative references

**Tasks**:

1. **SymPy Algorithm Study** (Primary Reference):
   ```bash
   # Read SymPy source code systematically
   cd ~/Documents/work/math/sympy/

   # ODE algorithms
   rg "class ODESolver" sympy/solvers/ode/ --context 50 > .research/ode_classes.txt
   rg "def dsolve" sympy/solvers/ode/ --context 100 > .research/ode_dsolve.txt

   # Linear algebra algorithms
   rg "def eigenvals" sympy/matrices/ --context 50 > .research/eigenvals.txt
   rg "class QRDecomposition" sympy/matrices/ --context 100 > .research/qr_decomposition.txt

   # Number theory algorithms
   rg "def factorint" sympy/ntheory/ --context 50 > .research/factorization.txt
   rg "class GröbnerBasis" sympy/polys/ --context 100 > .research/groebner.txt
   ```

2. **Algorithm Categorization**:
   ```markdown
   # .research/algorithm_matrix.md

   ## ODE Solvers (7 methods identified)

   | Method | SymPy Implementation | Complexity | Edge Cases |
   |--------|---------------------|------------|-----------|
   | Separable | `separable.py:45` | O(n) | Division by zero |
   | Linear 1st Order | `linear.py:78` | O(n²) | Discontinuous p(x) |
   | Exact | `exact.py:123` | O(n²) | Non-exact detection |
   | Homogeneous | `homogeneous.py:56` | O(n²) | y/x singularities |
   | Constant Coeff | `linear_coeff.py:234` | O(n³) | Complex roots |
   | Cauchy-Euler | `euler.py:89` | O(n²) | x = 0 singularity |
   | Variation of Params | `variation.py:145` | O(n³) | Difficult integrals |

   **Implementation Priority**: Separable → Linear 1st → Constant Coeff (covers 80% of common cases)
   ```

3. **Mathematical Correctness Validation Strategy**:
   ```python
   # .research/validation_plan.py
   """
   For each algorithm:
   1. Extract 50+ test cases from SymPy test suite
   2. Categorize by: simple, medium, complex, edge cases
   3. Document expected behavior for each case
   4. Create correctness validation suite
   """

   validation_cases = {
       'ode_separable': {
           'simple': [
               "dy/dx = x",  # SymPy result: y = x²/2 + C
               "dy/dx = y",  # SymPy result: y = C*exp(x)
           ],
           'medium': [
               "dy/dx = x*y",  # SymPy result: y = C*exp(x²/2)
           ],
           'edge_cases': [
               "dy/dx = y/x",  # Homogeneous, not separable
               "dy/dx = 0",    # Trivial: y = C
           ]
       }
   }
   ```

4. **Performance Benchmark Design**:
   ```rust
   // .research/benchmark_plan.md

   ## Performance Targets (vs SymPy)

   | Operation | SymPy Time | MathHook Target | Speedup Goal |
   |-----------|-----------|----------------|--------------|
   | ODE solve (simple) | 50ms | <5ms | 10x |
   | Eigenvalues 10x10 | 200ms | <20ms | 10x |
   | Polynomial factor | 100ms | <10ms | 10x |
   | Taylor series (order 10) | 80ms | <8ms | 10x |

   **Measurement Strategy**:
   - Run SymPy benchmarks first (establish baseline)
   - Design Criterion benchmarks matching SymPy test cases
   - Track performance during implementation (no regression)
   ```

5. **Architecture Design Document**:
   ```markdown
   # .research/architecture_design.md

   ## Module Structure

   ```
   crates/mathhook-core/src/
   ├── ode/
   │   ├── mod.rs              # Public API
   │   ├── first_order/
   │   │   ├── separable.rs
   │   │   ├── linear.rs
   │   │   ├── exact.rs
   │   │   └── homogeneous.rs
   │   ├── second_order/
   │   │   ├── linear_const_coeff.rs
   │   │   ├── cauchy_euler.rs
   │   │   └── variation_of_params.rs
   │   └── classifier.rs       # Auto-detect ODE type
   ├── linalg_advanced/
   │   ├── decompositions/
   │   │   ├── qr.rs
   │   │   ├── svd.rs
   │   │   ├── lu.rs
   │   │   └── cholesky.rs
   │   └── eigenvalues/
   │       ├── symbolic.rs     # Characteristic polynomial method
   │       └── numerical.rs    # QR algorithm
   └── ...
   ```

   ## Algorithm Selection Framework

   ```rust
   pub enum ODEType {
       Separable,
       LinearFirstOrder,
       Exact,
       Homogeneous,
       ConstantCoefficients,
       // ...
   }

   pub fn classify_ode(ode: &Expression) -> Option<ODEType> {
       // Pattern matching to identify ODE type
       // Priority order: fastest → slowest methods
   }
   ```
   ```

6. **SymPy Comparison Test Suite**:
   ```python
   # .research/sympy_comparison_suite.py
   """
   Generate comprehensive test cases by running SymPy
   and saving expected outputs for validation.
   """

   import sympy as sp
   import json

   def generate_test_cases():
       x, y, t = sp.symbols('x y t')

       test_cases = {
           'ode_first_order': [],
           'eigenvalues': [],
           'factorization': [],
           # ... for each wave
       }

       # ODE test cases
       odes = [
           sp.Eq(y.diff(x), x),
           sp.Eq(y.diff(x), y),
           sp.Eq(y.diff(x), x*y),
           # ... 50+ cases per category
       ]

       for ode in odes:
           solution = sp.dsolve(ode, y)
           test_cases['ode_first_order'].append({
               'input': str(ode),
               'expected_output': str(solution),
               'sympy_version': sp.__version__
           })

       # Save test oracle
       with open('.research/test_oracle.json', 'w') as f:
           json.dump(test_cases, f, indent=2)
   ```

**Deliverables**:
- [ ] Algorithm research notes for all 6 implementation waves
- [ ] Test case extraction from SymPy (500+ cases)
- [ ] Performance benchmarks baseline (SymPy measurements)
- [ ] Architecture design document
- [ ] Mathematical correctness validation strategy
- [ ] Edge case catalog for each algorithm
- [ ] Implementation priority ranking (high-value algorithms first)

**Verification**:
```bash
# Verify research completeness before Wave 1 implementation
ls .research/
# Should contain:
# - algorithm_matrix.md
# - validation_plan.py
# - benchmark_plan.md
# - architecture_design.md
# - sympy_comparison_suite.py
# - test_oracle.json (500+ test cases)
# - ode_classes.txt
# - eigenvals.txt
# - factorization.txt
# - groebner.txt
```

---

### Wave 1: Ordinary Differential Equations (ODEs) (16-20 hours)

**Goal**: Implement ODE solvers for first-order and second-order ODEs

**Priority**: HIGH - ODEs are fundamental for physics, engineering, biology

**Tasks**:

1. **First-Order ODEs**:
   ```rust
   pub fn solve_ode_first_order(
       ode: &Expression,        // dy/dx = f(x, y)
       dependent: &Symbol,      // y
       independent: &Symbol,    // x
       initial_condition: Option<(Expression, Expression)>  // (x0, y0)
   ) -> Result<Expression, SolverError>
   ```
   - Separable ODEs: `dy/dx = g(x)h(y)`
   - Linear first-order: `dy/dx + p(x)y = q(x)` (integrating factor method)
   - Exact ODEs: `M(x,y)dx + N(x,y)dy = 0` where ∂M/∂y = ∂N/∂x
   - Homogeneous ODEs: `dy/dx = f(y/x)`

2. **Second-Order Linear ODEs**:
   ```rust
   pub fn solve_ode_second_order_linear(
       ode: &Expression,        // y'' + p(x)y' + q(x)y = r(x)
       dependent: &Symbol,
       independent: &Symbol,
   ) -> Result<Expression, SolverError>
   ```
   - Constant coefficients: characteristic equation method
   - Cauchy-Euler equations
   - Variation of parameters

3. **Educational Integration**:
   - Step-by-step explanations for each method
   - Show separation of variables steps
   - Show integrating factor derivation

**Reference**: SymPy's `dsolve()` in `sympy/solvers/ode/`

**Verification**:
```bash
#!/bin/bash
# verify_wave_1_ode.sh
cargo test -p mathhook-core ode --quiet
python3 verify_ode_against_sympy.py  # Compare 50 test cases
```

**Deliverables**:
- First-order ODE solver (4 methods)
- Second-order linear ODE solver (3 methods)
- 100+ tests validated against SymPy
- Educational explanations

---

### Wave 2: Advanced Linear Algebra (18-22 hours)

**Goal**: Complete matrix decomposition and eigenvalue algorithms

**Priority**: HIGH - Essential for scientific computing, ML applications

**Tasks**:

1. **Matrix Decompositions**:
   ```rust
   pub fn qr_decomposition(matrix: &Matrix) -> Result<(Matrix, Matrix), MatrixError>
   pub fn svd(matrix: &Matrix) -> Result<SVD, MatrixError>
   pub fn lu_decomposition(matrix: &Matrix) -> Result<LU, MatrixError>
   pub fn cholesky(matrix: &Matrix) -> Result<Matrix, MatrixError>
   ```

2. **Eigenvalue Algorithms**:
   ```rust
   pub fn eigenvalues_symbolic(matrix: &Matrix) -> Vec<Expression>
   pub fn eigenvectors_symbolic(matrix: &Matrix) -> Vec<(Expression, Vec<Expression>)>
   pub fn characteristic_polynomial(matrix: &Matrix) -> Expression
   pub fn jordan_normal_form(matrix: &Matrix) -> Result<Jordan, MatrixError>
   ```

3. **Matrix Properties**:
   ```rust
   pub fn matrix_rank(matrix: &Matrix) -> usize
   pub fn matrix_nullspace(matrix: &Matrix) -> Vec<Vec<Expression>>
   pub fn matrix_column_space(matrix: &Matrix) -> Vec<Vec<Expression>>
   pub fn is_positive_definite(matrix: &Matrix) -> bool
   ```

4. **Performance**:
   - SIMD-optimized numerical algorithms
   - Sparse matrix support for large systems
   - Symbolic-exact for small matrices, numerical for large

**Reference**: SymPy's `sympy/matrices/`

**Verification**: Compare against SymPy and numpy for numerical accuracy

**Deliverables**:
- 4 matrix decompositions
- Complete eigenvalue/eigenvector algorithms
- Matrix property analysis
- 150+ tests

---

### Wave 3: Number Theory & Polynomial Algorithms (20-24 hours)

**Goal**: Advanced factorization, primes, Gröbner bases

**Priority**: MEDIUM-HIGH - Required for symbolic computation power

**Tasks**:

1. **Advanced Factorization**:
   ```rust
   pub fn factor_polynomial_multivariate(poly: &Expression) -> Vec<(Expression, usize)>
   pub fn factor_integer(n: &BigInt) -> Vec<(BigInt, usize)>
   pub fn factor_over_extension_field(poly: &Expression, field: &Field) -> Vec<Expression>
   ```

2. **Prime Number Functions**:
   ```rust
   pub fn is_prime(n: &BigInt) -> bool  // Miller-Rabin test
   pub fn next_prime(n: &BigInt) -> BigInt
   pub fn prime_factorization(n: &BigInt) -> Vec<(BigInt, usize)>
   pub fn totient(n: &BigInt) -> BigInt  // Euler's totient
   ```

3. **Gröbner Bases**:
   ```rust
   pub fn groebner_basis(
       polynomials: &[Expression],
       variables: &[Symbol],
       order: MonomialOrder
   ) -> Vec<Expression>
   ```
   - Buchberger's algorithm
   - F4 algorithm (faster alternative)

4. **Polynomial GCD**:
   ```rust
   pub fn polynomial_gcd(p1: &Expression, p2: &Expression) -> Expression
   pub fn polynomial_gcd_multivariate(polys: &[Expression]) -> Expression
   ```

**Reference**: SymPy's `sympy/polys/` and `sympy/ntheory/`

**Deliverables**:
- Multivariate factorization
- Prime number functions
- Gröbner basis implementation
- Polynomial GCD algorithms
- 200+ tests

---

### Wave 4: Series Expansions & Special Functions (16-20 hours)

**Goal**: Taylor/Laurent series, Fourier series, special functions

**Priority**: MEDIUM - Important for analysis and approximation

**Tasks**:

1. **Series Expansions**:
   ```rust
   pub fn taylor_series(
       expr: &Expression,
       variable: &Symbol,
       point: &Expression,
       order: usize
   ) -> Expression

   pub fn laurent_series(
       expr: &Expression,
       variable: &Symbol,
       point: &Expression,
       order_neg: isize,
       order_pos: usize
   ) -> Expression

   pub fn fourier_series(
       expr: &Expression,
       variable: &Symbol,
       period: &Expression,
       order: usize
   ) -> Expression
   ```

2. **Special Functions**:
   ```rust
   // Gamma function family
   pub fn gamma(z: &Expression) -> Expression
   pub fn beta(a: &Expression, b: &Expression) -> Expression
   pub fn polygamma(n: usize, z: &Expression) -> Expression

   // Error functions
   pub fn erf(x: &Expression) -> Expression
   pub fn erfc(x: &Expression) -> Expression

   // Bessel functions
   pub fn bessel_j(n: &Expression, z: &Expression) -> Expression
   pub fn bessel_y(n: &Expression, z: &Expression) -> Expression

   // Hypergeometric functions
   pub fn hypergeometric_1f1(a: &Expression, b: &Expression, z: &Expression) -> Expression
   pub fn hypergeometric_2f1(a: &Expression, b: &Expression, c: &Expression, z: &Expression) -> Expression
   ```

3. **Asymptotic Expansions**:
   - Stirling's approximation for factorial
   - Asymptotic series for special functions

**Reference**: SymPy's `sympy/series/` and `sympy/functions/special/`

**Deliverables**:
- Taylor/Laurent/Fourier series
- 10+ special functions
- Asymptotic approximations
- 150+ tests

---

### Wave 5: Partial Differential Equations (PDEs) (12-16 hours)

**Goal**: Basic PDE solver for separable and common PDEs

**Priority**: MEDIUM - Important for physics/engineering

**Tasks**:

1. **Separation of Variables**:
   ```rust
   pub fn solve_pde_separation_of_variables(
       pde: &Expression,
       dependent: &Symbol,
       independents: &[Symbol]
   ) -> Result<Expression, SolverError>
   ```
   - Heat equation: ∂u/∂t = α∇²u
   - Wave equation: ∂²u/∂t² = c²∇²u
   - Laplace equation: ∇²u = 0

2. **Method of Characteristics** (for first-order PDEs):
   ```rust
   pub fn solve_pde_characteristics(
       pde: &Expression,
       dependent: &Symbol,
       independents: &[Symbol]
   ) -> Result<Expression, SolverError>
   ```

3. **Common PDEs**:
   - Recognize and solve standard forms
   - Provide general solutions with arbitrary functions

**Reference**: SymPy's `sympy/solvers/pde/`

**Deliverables**:
- Separation of variables solver
- Method of characteristics
- Solutions for 3 standard PDEs
- 50+ tests

---

### Wave 6: Numerical Methods & Integration (14-18 hours)

**Goal**: Robust numerical methods for cases where symbolic fails

**Priority**: MEDIUM-HIGH - Fallback when symbolic methods fail

**Tasks**:

1. **Numerical Integration**:
   ```rust
   pub fn integrate_numerical(
       expr: &Expression,
       variable: &Symbol,
       lower: f64,
       upper: f64,
       method: IntegrationMethod
   ) -> Result<f64, NumericalError>
   ```
   - Gaussian quadrature
   - Adaptive Simpson's rule
   - Romberg integration

2. **Numerical Equation Solving**:
   ```rust
   pub fn solve_numerical(
       equation: &Expression,
       variable: &Symbol,
       initial_guess: f64
   ) -> Result<f64, SolverError>
   ```
   - Newton-Raphson method
   - Secant method
   - Bisection method

3. **Numerical ODEs**:
   ```rust
   pub fn solve_ode_numerical(
       ode: &Expression,
       initial_condition: (f64, f64),
       t_span: (f64, f64),
       method: ODEMethod
   ) -> Result<Vec<(f64, f64)>, SolverError>
   ```
   - Runge-Kutta methods (RK4, RK45)
   - Adams-Bashforth methods

4. **Error Estimation**:
   - Provide error bounds for all numerical methods
   - Adaptive step size control

**Reference**: SymPy's `sympy/integrals/quad.py` and SciPy

**Deliverables**:
- 3 numerical integration methods
- 3 numerical equation solvers
- Numerical ODE solvers
- Error estimation
- 100+ tests

---

## Final Success Criteria

### Wave Completion Checklist
- [ ] Wave 0: Algorithm research & architecture (SymPy/Symbolica study, test oracle, architecture design)
- [ ] Wave 1: ODE solvers (first/second order)
- [ ] Wave 2: Advanced linear algebra (decompositions, eigenvalues)
- [ ] Wave 3: Number theory & polynomial algorithms (factorization, Gröbner bases)
- [ ] Wave 4: Series expansions & special functions
- [ ] Wave 5: PDE solvers (basic cases)
- [ ] Wave 6: Numerical methods (fallback for symbolic)

### Quality Metrics
- All waves score ≥ 8/10
- 100% SymPy correctness validation (test against 500+ cases)
- Zero mathematical errors
- 676/677 minimum test pass rate maintained

### Deliverables Checklist
- [ ] ODE solver module (first/second order, 7 methods)
- [ ] Advanced linear algebra (4 decompositions, eigenvalues)
- [ ] Number theory (factorization, primes, Gröbner bases)
- [ ] Series expansions (Taylor, Laurent, Fourier)
- [ ] 10+ special functions
- [ ] PDE solver (3 methods)
- [ ] Numerical methods (integration, solving, ODEs)

### Exit Criteria
- **Feature Parity**: Matches/exceeds SymPy core functionality
- **Performance**: 10-100x faster than SymPy for all implemented features
- **Correctness**: 100% SymPy validation pass rate
- **Educational**: All new features have step-by-step explanations

---

## Competitive Impact

**After Plan 7 Completion**:

**vs SymPy**:
- ✅ Feature parity on core CAS functions
- ✅ 10-100x faster (Rust vs Python)
- ✅ Educational features (unique differentiator)

**vs Mathematica**:
- ✅ Free vs $25K/year
- ✅ Open source
- ⚠️  Mathematica still has more specialized functions (acceptable gap)

**vs Symbolica**:
- ✅ Educational features (unique)
- ✅ ODEs/PDEs (Symbolica lacks these)
- ✅ Multi-language APIs

**Market Position**: MathHook becomes viable SymPy replacement for 80%+ of CAS use cases

---

## Timeline & Dependencies

**Total Timeline**: 24-36 weeks (includes 2-3 week Wave 0 research phase)

**Wave Timeline Breakdown**:
- Wave 0 (Algorithm Research): 2-3 weeks
- Wave 1 (ODEs): 2.5-3 weeks
- Wave 2 (Advanced Linear Algebra): 3-3.5 weeks
- Wave 3 (Number Theory): 3.5-4 weeks
- Wave 4 (Series/Special Functions): 2.5-3 weeks
- Wave 5 (PDEs): 2-2.5 weeks
- Wave 6 (Numerical Methods): 2.5-3 weeks
- Integration & Testing: 4-6 weeks
- **Total**: 24-36 weeks

**Dependencies**:
- **AFTER Plan 1** (Performance Recovery): Don't add features until performance is validated
- **PARALLEL with Plans 2-6**: Can run concurrently with educational integration, APIs, and MCP

**Recommended Execution**:
- Phase 1: Execute Plan 1 (Performance) FIRST
- Phase 2: Execute Plans 2, 3, 4, 7 in PARALLEL (different feature areas)
- Phase 3: Execute Plans 5, 6 after Phase 2 complete

**Wave Coordination**: Each wave is independent - can parallelize within Plan 7

---

## Notes

**Why This Matters**: Without core CAS features (ODEs, advanced linear algebra, etc.), MathHook cannot replace SymPy for many real-world use cases. This plan closes the gap.

**After Plan 7**: MathHook will be production-ready for:
- Scientific computing (physics, engineering)
- Machine learning (linear algebra, optimization)
- Education (step-by-step for all operations)
- Neuro-symbolic AI (regulatory compliance)
