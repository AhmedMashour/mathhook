# Algebra Module Context

**Purpose**: Algebraic operations including simplification, expansion, factoring, equation solving, polynomial arithmetic, and rational expression manipulation

**Last Updated**: 2025-10-30

---

## Module Structure

### Files in This Module

**Core Operations** (1,828 lines):
- `expand.rs` (473 lines) - Expression expansion (distribute, multiply out)
- `rational.rs` (480 lines) - Rational expression simplification
- `gcd.rs` (562 lines) - Polynomial GCD and LCM operations
- `polynomial_division.rs` (475 lines) - Polynomial division, quotient, remainder
- `zero_detection.rs` (454 lines) - Detecting when expressions equal zero
- `complex.rs` (14 lines) - Complex number operations module aggregator
- `complex/operations.rs` (370 lines) - Complex arithmetic operations
- `complex/arithmetic.rs` (508 lines) - Complex number manipulation

**Advanced Features** (1,007 lines):
- `polynomial_advanced.rs` (381 lines) - Advanced polynomial operations
- `advanced_simplify/mod.rs` (271 lines) - Advanced simplification strategies
- `advanced_simplify/helpers.rs` (328 lines) - Simplification helper functions
- `equation_analyzer.rs` (290 lines) - Equation type detection and smart solving
- `solvers.rs` (262 lines) - Main solver interfaces and result types

**Collection Operations** (676 lines):
- `collect/mod.rs` (318 lines) - Collect like terms and factors
- `collect/terms.rs` (236 lines) - Term collection implementation
- `collect/coefficients.rs` (141 lines) - Coefficient extraction

**Factoring** (705 lines):
- `factor/mod.rs` (403 lines) - Factoring interface and common cases
- `factor/common.rs` (158 lines) - Common factor extraction
- `factor/quadratic.rs` (30 lines) - Quadratic factoring
- `factor/noncommutative.rs` (144 lines) - Noncommutative factoring (matrices, operators)

**Solvers Subdirectory** (2,489 lines):
- `solvers/linear.rs` (500 lines) - Linear equation solver
- `solvers/quadratic.rs` (415 lines) - Quadratic equation solver
- `solvers/systems.rs` (563 lines) - System of equations solver
- `solvers/matrix_equations.rs` (552 lines) - Matrix equation solver (left/right division)
- `solvers/polynomial/mod.rs` (9 lines) - Polynomial solver aggregator
- `solvers/polynomial/solver.rs` (299 lines) - Polynomial equation solver
- `solvers/polynomial/educational.rs` (239 lines) - Educational polynomial solving
- `solvers/polynomial/tests.rs` (201 lines) - Polynomial solver tests

**Tests** (222 lines):
- `matrix_correctness_tests.rs` (81 lines) - Matrix algebra correctness tests
- `diagonal_matrix_tests.rs` (138 lines) - Diagonal matrix tests

**Total Module Size**: ~6,927 lines across 27 files

---

## Public API

### Key Traits
- `pub trait Expand` - Expression expansion interface
- `pub trait AdvancedSimplify` - Advanced simplification strategies
- `pub trait ComplexOperations` - Complex number operations
- `pub trait ZeroDetection` - Detect when expressions equal zero
- `pub trait PolynomialGcd` - GCD/LCM for polynomials
- `pub trait Factor` - Factoring interface
- `pub trait EquationSolver` - Single equation solving
- `pub trait SystemEquationSolver` - System solving
- `pub trait SolverStepByStep` - Educational solving with explanations
- `pub trait AdvancedPolynomial` - Advanced polynomial operations
- `pub trait RationalSimplify` - Rational expression simplification
- `pub trait Collect` - Collect like terms and factors

### Key Structs
- `pub struct EquationAnalyzer` - Analyzes equation types for smart solving
- `pub struct SmartEquationSolver` - Main intelligent equation solver
- `pub struct PolynomialArithmetic` - Polynomial arithmetic operations
- `pub struct QuadraticSolver` - Quadratic equation solver
- `pub struct LinearSolver` - Linear equation solver
- `pub struct MatrixEquationSolver` - Matrix equation solver (Wave 10)
- `pub struct SystemSolver` - System of equations solver
- `pub struct PolynomialSolver` - Polynomial equation solver

### Key Enums
- `pub enum SolverResult` - Success, no solution, infinite solutions
- `pub enum SolverError` - Solver error types
- `pub enum EquationType` - Linear, quadratic, polynomial, system, matrix, etc.

### Key Functions
- `pub fn polynomial_div()` - Polynomial long division
- `pub fn polynomial_quo()` - Polynomial quotient
- `pub fn polynomial_rem()` - Polynomial remainder
- `pub fn solve_with_explanation()` - Educational polynomial solving

---

## Dependencies

### Imports FROM Other Modules
**Core Types** (Heavy usage):
- `core/expression/` - Expression, Add, Mul, Pow, Function variants
- `core/symbol.rs` - Symbol type for variables
- `core/number.rs` - Number type (exact rationals, integers)

**Functions** (Moderate usage):
- `functions/elementary/` - sin, cos, exp, log for simplification
- `functions/polynomials/` - Polynomial utility functions

**Simplification** (Heavy usage):
- `simplify/` - Canonical form, simplification strategies

**Educational** (Light usage):
- `educational/message_registry/` - Step-by-step explanation messages

### Used BY Other Modules
**Primary Consumers**:
- `calculus/` - Uses simplification for derivatives, integrals, limits
- `educational/` - Uses solver explanations for step-by-step
- `parser/` - Uses equation analyzer for parsed equations
- `formatter/` - Uses canonical forms for output formatting

**Secondary Consumers**:
- `matrix/` - Uses matrix equation solver
- `pattern/` - Uses pattern matching on algebraic forms

---

## Testing

### Module-Specific Test Commands
```bash
# All algebra tests
cargo test -p mathhook-core algebra

# Solver tests only
cargo test -p mathhook-core algebra::solvers

# Polynomial tests
cargo test -p mathhook-core polynomial

# Matrix equation tests
cargo test -p mathhook-core matrix_equations

# Integration tests
cargo test -p mathhook-core test_algebraic_solving
```

### Test Coverage
- Unit tests: ~115 `#[test]` functions
- Integration tests: Multiple cross-module tests
- Doctests: Examples in public API documentation

**Key Test Files**:
- `solvers/polynomial/tests.rs` (201 lines) - Comprehensive polynomial solver tests
- `matrix_correctness_tests.rs` - Matrix algebra correctness
- `diagonal_matrix_tests.rs` - Diagonal matrix special cases

---

## External References

### SymPy Equivalent
**Location**: `~/Documents/work/math/sympy/sympy/solvers/`
**Key Files**:
- `sympy/solvers/solvers.py` - Main solving interface
- `sympy/solvers/polysys.py` - Polynomial systems
- `sympy/solvers/solveset.py` - Modern solving interface
- `sympy/core/expand.py` - Expression expansion
- `sympy/simplify/simplify.py` - Simplification

### Symbolica Equivalent
**Location**: `~/Documents/work/math/symbolica/src/`
**Key Files**:
- `symbolica/src/poly/` - Polynomial operations
- `symbolica/src/domains/` - Algebraic domains
- `symbolica/src/atom/` - Expression atoms and operations

---

## Common Patterns & Pitfalls

### Design Patterns Used
1. **Trait-based Polymorphism**: All operations use traits (Expand, Factor, Collect, etc.)
2. **Smart Solver Pattern**: `EquationAnalyzer` detects equation type → routes to specialized solver
3. **Result Types**: All solving returns `Result<Vec<Expression>, SolverError>`
4. **Canonical Forms**: All results simplified to canonical form for consistency

### Common Pitfalls
1. **Domain Restrictions**: Always check for division by zero, sqrt of negatives
2. **Exact Arithmetic**: Use rational numbers, NOT floats, for symbolic operations
3. **Canonical Form**: Always return simplified, canonical form expressions
4. **Matrix Noncommutativity**: Matrix equations require left vs right division (Wave 10)
   - `A*X = B` → `X = A^(-1)*B` (left division)
   - `X*A = B` → `X = B*A^(-1)` (right division)
   - **CRITICAL**: `A^(-1)*B ≠ B*A^(-1)` for matrices!
5. **Zero Detection**: Use algebraic zero detection, not numerical epsilon comparison
6. **Polynomial Degree**: Off-by-one errors in polynomial degree calculations

---

## CLAUDE.md Constraints (Module-Specific)

### File Size Compliance
**Current Status**: ⚠️ 3 files exceed 500 lines (pre-existing, documented)
- `solvers/systems.rs` (563 lines) - Technical debt from Wave 10
- `solvers/matrix_equations.rs` (552 lines) - Technical debt from Wave 10
- `gcd.rs` (562 lines) - Pre-existing

**Target**: Split these files in future cleanup wave

### Module-Specific Rules
1. **Solver Interface**: All solvers MUST implement `EquationSolver` trait
2. **Error Handling**: All solving functions return `Result<Vec<Expression>, SolverError>`
3. **Educational Integration**: Complex solvers SHOULD provide step-by-step explanations
4. **Canonical Output**: All operations MUST return canonical form expressions
5. **No Hardcoded Operations**: Use trait-based dispatch, not hardcoded matches

---

## Recent Changes

### Last 3 Major Modifications
1. **Wave 10**: Matrix equation solver with left/right division (Oct 2024)
   - Added `MatrixEquationSolver` with noncommutative algebra support
   - 41 tests, 10/10 quality score (PERFECT)

2. **Wave 9**: Symbol macro system with type specification (Oct 2024)
   - Enhanced `symbol!()` macro for scalar/matrix/operator/quaternion types
   - 37 tests, 9.5/10 quality score

3. **Wave 8**: LaTeX parser type inference (Oct 2024)
   - Automatic symbol type detection from LaTeX notation
   - 27 tests, 9.5/10 quality score

---

## Technical Debt

### Known Issues
1. **File Size Violations**: 3 files exceed 500 lines (see above)
   - **Acceptable for now**: Pre-existing from successful waves
   - **Future**: Split using module aggregator pattern

2. **Test Coverage**: Some advanced simplification paths lack edge case tests
   - Target: Add 20+ tests for corner cases in future QA wave

3. **Performance**: Polynomial GCD can be slow for large degree
   - Consider optimization in future performance wave

### Future Improvements
1. Split large files (`systems.rs`, `matrix_equations.rs`, `gcd.rs`) using module pattern
2. Add more comprehensive tests for advanced simplification
3. Optimize polynomial GCD performance
4. Add more factoring strategies (cubic, quartic)
5. Implement Gröbner basis for system solving

---

## Integration Points

### Smart Solver Flow
```
User Equation → EquationAnalyzer.analyze()
    ↓
EquationType detected (Linear, Quadratic, Polynomial, Matrix, System)
    ↓
SmartEquationSolver.solve() routes to:
    - LinearSolver for linear equations
    - QuadraticSolver for quadratic equations
    - PolynomialSolver for higher-degree polynomials
    - MatrixEquationSolver for matrix equations (A*X = B)
    - SystemSolver for systems of equations
    ↓
Result → Simplified canonical form
```

### Educational Integration
Solvers with educational support:
- `PolynomialSolver` → `solve_with_explanation()`
- `QuadraticSolver` → Step-by-step quadratic formula
- Educational registry messages in `educational/message_registry/solvers.rs`

---

**Module Owner**: Core team
**Related Waves**: Wave 8 (parser), Wave 9 (macros), Wave 10 (matrix equations), Educational Waves 2-5
