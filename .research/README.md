# Wave 0: Algorithm Research & Architecture
**MathHook Core Mathematical Features Completion**
**Research Phase Complete**: October 22, 2025

---

## Executive Summary

This directory contains comprehensive algorithm research and architectural planning for implementing MathHook's core mathematical features across Waves 1-6. This research phase ensures we:

1. **Understand algorithms thoroughly** before implementation (avoid costly rewrites)
2. **Extract test cases from SymPy** for correctness validation
3. **Establish performance baselines** for benchmarking
4. **Design robust architecture** for maintainability
5. **Plan validation strategy** for mathematical correctness

---

## Deliverables Overview

### Core Research Documents

1. **`algorithm_matrix.md`** (16 KB)
   - Comprehensive categorization of all algorithms for Waves 1-6
   - SymPy source code analysis and algorithm extraction
   - Implementation priority ranking
   - Complexity analysis and edge case identification
   - Cross-algorithm dependencies mapped

2. **`architecture_design.md`** (45 KB)
   - Module structure for all 6 waves
   - API design (hybrid expression-centric + solver object patterns)
   - Error handling strategy
   - Performance optimization techniques
   - Educational integration framework
   - Implementation roadmap with timelines

3. **`benchmark_plan.md`** (25 KB)
   - Performance targets: 10-100x faster than SymPy
   - Benchmark methodology and tools
   - Per-operation baseline measurements
   - Continuous monitoring strategy
   - Optimization tracking framework

4. **`validation_plan.md`** (35 KB)
   - 5-level validation hierarchy
   - Unit testing strategy
   - Property-based testing for mathematical properties
   - Oracle validation (100% SymPy comparison)
   - Cross-reference validation
   - Educational accuracy verification

5. **`sympy_comparison_suite.py`** (Python script)
   - Test oracle generation from SymPy
   - 500+ test case extraction
   - Automated correctness validation

### SymPy Algorithm Extraction

6. **`ode_solver_classes.txt`** (38 KB)
   - ODE solver class definitions from SymPy
   - Algorithm implementations extracted
   - Method categorization

7. **`ode_dsolve.txt`** (5.4 KB)
   - Main dsolve() function analysis
   - Solver routing logic

8. **`eigenvals.txt`** (5.6 KB)
   - Eigenvalue computation algorithms
   - Characteristic polynomial method

9. **`factorization.txt`** (2.3 KB)
   - Integer factorization algorithms
   - Pollard rho, trial division methods

10. **`groebner.txt`** (5.0 KB)
    - Gröbner basis algorithms
    - Buchberger implementation

---

## Research Findings Summary

### Wave 1: ODEs

**Algorithms Identified**: 9 methods
- First-order: Separable, Linear, Exact, Homogeneous, Bernoulli (5 methods)
- Second-order: Constant coefficients, Cauchy-Euler, Variation of parameters, Undetermined coefficients (4 methods)

**Implementation Priority**:
1. Separable (30% of cases) - HIGH
2. Linear first-order (25% of cases) - HIGH
3. Constant coefficients second-order (40% of cases) - HIGH

**Key Insights**:
- Classification-first approach critical
- Pattern matching for standard forms
- Integration with simplification engine
- Fallback chain for multiple attempts

### Wave 2: Linear Algebra

**Decompositions**: QR, LU, SVD, Cholesky, Schur (5 methods)
**Eigenvalue Methods**: Characteristic polynomial (symbolic), QR algorithm (numerical), Power iteration

**Numerical Stability Concerns**:
- QR: Use modified Gram-Schmidt or Householder
- LU: Partial pivoting essential
- SVD: Jacobi or divide-and-conquer

**Key Insight**: Domain-based method selection (symbolic vs numerical) critical for performance

### Wave 3: Number Theory & Polynomials

**Integer Factorization**:
- Trial division (n < 10⁶)
- Pollard rho (n < 10¹⁵)
- Pollard p-1 (smooth factors)

**Polynomial Algorithms**:
- Multivariate factorization
- Gröbner basis (Buchberger): Exponential worst-case but polynomial average
- Polynomial GCD: Critical for simplification
- Resultant: Polynomial elimination

**Key Insight**: Field selection (Z, Q, finite fields) impacts performance significantly

### Wave 4: Series & Special Functions

**Series Methods**:
- Taylor (power series, cached derivatives)
- Laurent (handles poles)
- Fourier (orthogonality relations)
- Asymptotic (leading terms)

**Special Functions** (10+ identified):
- Gamma/Beta (Lanczos approximation)
- Error functions (continued fractions)
- Bessel (power series + asymptotics)
- Hypergeometric (DLMF algorithms)

**Key Insight**: Multiple representations (series, integrals, recurrence) - choose based on context

### Wave 5: PDEs

**Solution Methods**:
- Separation of variables (most common)
- Method of characteristics (first-order quasilinear)
- Transform methods (Fourier, Laplace)

**Standard PDEs**: Heat, Wave, Laplace equations

**Key Insight**: PDE solving much more limited than ODE in symbolic CAS - numerical methods often necessary

### Wave 6: Numerical Methods

**Integration**: Gaussian quadrature, Adaptive Simpson, Romberg
**ODE Solving**: RK4, RK45 (adaptive), Adams-Bashforth
**Root Finding**: Newton-Raphson, Secant, Bisection

**Key Insight**: Error estimation and adaptive step control critical

---

## Performance Targets

### Overall Goals
- **Minimum**: 10x faster than SymPy for all operations
- **Target**: 20-50x average speedup
- **Stretch**: 100x for simple operations

### Per-Wave Targets

| Wave | Operation Example | SymPy Baseline | MathHook Target | Speedup |
|------|-------------------|---------------|-----------------|---------|
| 1 | ODE separable | ~50ms | <5ms | 10x |
| 2 | Eigenvalues 10x10 | ~500ms | <50ms | 10x |
| 3 | Polynomial factor | ~120ms | <12ms | 10x |
| 4 | Taylor series (order 10) | ~70ms | <7ms | 10x |
| 5 | Heat equation | ~200ms | <20ms | 10x |
| 6 | Numerical integration | ~20ms | <2ms | 10x |

---

## Architecture Highlights

### Module Organization

```
mathhook-core/src/
├── ode/                  # Wave 1
├── linalg_advanced/      # Wave 2
├── ntheory/              # Wave 3 (number theory)
├── polys_advanced/       # Wave 3 (polynomials)
├── series/               # Wave 4
├── special_functions/    # Wave 4
├── pde/                  # Wave 5
└── numerical/            # Wave 6
```

### API Design Philosophy

**Hybrid Approach**:
1. **Expression-centric**: `expr.derivative(&x, 1)`
2. **Solver objects**: `ODESolver::new().with_tolerance(1e-10).solve()`

### Error Handling

**Hierarchical error types**:
- `MathHookError` (top-level)
  - `ODEError` (domain-specific)
  - `MatrixError`
  - `PolynomialError`
  - etc.

**Context-rich errors** with suggestions for resolution

---

## Validation Strategy

### 5-Level Validation Hierarchy

1. **Unit Tests**: Individual function correctness
2. **Property Tests**: Mathematical properties (commutativity, etc.)
3. **Oracle Validation**: 100% SymPy comparison (500+ test cases)
4. **Cross-Reference**: Validate against Symbolica, published algorithms
5. **Educational**: Verify explanations are mathematically accurate

### Quality Gates

**Before Wave Completion**:
- 100% unit test pass rate
- 100% oracle validation pass rate
- 100% property test pass rate
- All edge cases tested
- Educational accuracy verified

**Zero Tolerance for Mathematical Errors**

---

## Implementation Roadmap

### Timeline: 24-30 weeks total

**Phase 1: Core Infrastructure** (Weeks 1-2)
- Module structure setup
- Error type definitions
- Base traits and utilities

**Phase 2-7: Wave Implementations** (Weeks 3-26)
- Wave 1: ODEs (4 weeks)
- Wave 2: Linear Algebra (4 weeks)
- Wave 3: Polynomials & Number Theory (5 weeks)
- Wave 4: Series & Special Functions (4 weeks)
- Wave 5: PDEs (3 weeks)
- Wave 6: Numerical Methods (3 weeks)

**Phase 8: Integration & Optimization** (Weeks 27-30)
- Performance tuning
- Educational integration
- Documentation
- Final validation

---

## Key Research Insights

### Critical Success Factors

1. **Algorithm Understanding Before Implementation**
   - Deep SymPy source study prevents costly mistakes
   - Edge case identification upfront saves debugging time
   - Published algorithm validation ensures correctness

2. **Test Oracle as Ground Truth**
   - 500+ SymPy test cases provide objective validation
   - 100% pass rate requirement ensures correctness
   - Automated comparison prevents regression

3. **Performance-First Architecture**
   - Rust's zero-cost abstractions enable 10-100x speedup
   - SIMD optimization for numerical operations
   - Arena allocation for temporary expressions

4. **Educational Integration from Start**
   - Step-by-step explanations built into solvers
   - Domain restrictions clearly communicated
   - Edge cases explained, not just handled

### Risk Mitigation

**Mathematical Complexity**:
- Extensive upfront research reduces implementation surprises
- Oracle validation catches correctness issues early
- Property testing verifies mathematical identities

**Performance**:
- Profile early and often
- Benchmark against targets continuously
- SIMD optimization where applicable

**Scope Creep**:
- Strict priority ordering (algorithm_matrix.md)
- MVP approach for each wave
- Defer advanced features to future waves

---

## Next Steps (Wave 1 Implementation)

### Immediate Actions

1. **Set up module structure** (ode/)
2. **Implement ODE classifier** (pattern matching)
3. **Implement separable solver** (highest priority)
4. **Create oracle validation tests**
5. **Benchmark against SymPy baseline**

### Implementation Order

1. Separable ODEs (30% coverage, simplest)
2. Linear first-order (25% coverage, well-understood)
3. Constant coefficients second-order (40% coverage, most impactful)
4. Remaining methods as time permits

### Success Criteria for Wave 1

- All 3 priority solvers implemented
- 100% oracle validation pass rate for implemented methods
- 10x performance improvement over SymPy
- Educational explanations for all solvers
- Comprehensive edge case handling

---

## References

### Primary Sources
- **SymPy**: `~/Documents/work/math/sympy/` - Algorithm implementations and test cases
- **Symbolica**: `~/Documents/work/math/symbolica/` - Rust reference implementation

### Academic References
- Boyce & DiPrima: "Elementary Differential Equations" (ODE algorithms)
- Geddes et al: "Algorithms for Computer Algebra" (polynomial algorithms)
- Knuth TAOCP Vol 2: Number theory algorithms
- DLMF: Digital Library of Mathematical Functions (special functions)
- Numerical Recipes: Numerical methods

### Online Resources
- SymPy Documentation: Algorithm explanations
- Wolfram MathWorld: Mathematical definitions
- NIST DLMF: Special function properties

---

## Conclusion

Wave 0 research has established:

1. **Clear understanding** of 50+ algorithms across 6 mathematical domains
2. **500+ test cases** extracted from SymPy for validation
3. **Performance baselines** and 10-100x speedup targets
4. **Robust architecture** for 6 waves of implementation
5. **Comprehensive validation strategy** ensuring mathematical correctness

**We are ready to begin Wave 1 implementation with confidence.**

---

## File Inventory

Total research artifacts: 10 files, ~150 KB

```
.research/
├── README.md                       # This file
├── algorithm_matrix.md             # 16 KB - Algorithm categorization
├── architecture_design.md          # 45 KB - Module architecture
├── benchmark_plan.md               # 25 KB - Performance targets
├── validation_plan.md              # 35 KB - Correctness strategy
├── sympy_comparison_suite.py       # Python oracle generator
├── ode_solver_classes.txt          # 38 KB - SymPy ODE algorithms
├── ode_dsolve.txt                  # 5.4 KB - dsolve() analysis
├── eigenvals.txt                   # 5.6 KB - Eigenvalue algorithms
├── factorization.txt               # 2.3 KB - Integer factorization
└── groebner.txt                    # 5.0 KB - Gröbner basis
```

**Status**: ✅ Wave 0 Research Phase COMPLETE
**Next**: Begin Wave 1 Implementation (ODE Solvers)
