# Wave 10: Equation Solvers Integration for Noncommutative Algebra

**Goal**: Update equation solvers to handle noncommutative algebra (matrices, operators, quaternions)

**Priority**: HIGH (Critical for matrix equation solving)
**Effort**: 3-4 hours
**Impact**: Enables matrix equations, operator equations, quantum mechanics applications

---

## Problem with Current Equation Solvers

### Current Solvers Assume Commutativity

The existing equation solvers in `solvers/` assume all operations are commutative:

```rust
// Current behavior (WRONG for matrices/operators):
A*X = B  →  X = B/A  (treats as commutative division)

// But in matrix algebra:
A*X = B  →  X = A^(-1)*B  (left multiplication)
X*A = B  →  X = B*A^(-1)  (right multiplication)

// These are DIFFERENT! Order matters for noncommutative types
```

**Critical Issue**: Equation solvers must distinguish:
1. **Left division**: `A*X = B` → `X = A^(-1)*B`
2. **Right division**: `X*A = B` → `X = B*A^(-1)`

---

## Wave 10 Scope

### 1. Linear Equation Solver Updates

**File**: `crates/mathhook-core/src/solvers/linear.rs`

**Changes Needed**:
- Detect commutativity of equation terms
- Distinguish `A*X = B` (left multiplication) from `X*A = B` (right multiplication)
- Apply correct inverse operation based on position

**New Functions**:
```rust
/// Solve A*X = B for X (left multiplication)
pub fn solve_left_division(A: Expression, B: Expression, X: Symbol) -> Result<Expression, SolverError>

/// Solve X*A = B for X (right multiplication)
pub fn solve_right_division(A: Expression, B: Expression, X: Symbol) -> Result<Expression, SolverError>

/// Detect if equation is left or right division
fn classify_division_side(equation: &Expression, var: &Symbol) -> DivisionSide
```

### 2. Equation Analyzer Updates

**File**: `crates/mathhook-core/src/solvers/analyzer.rs` (or create if doesn't exist)

**Changes Needed**:
- Add commutativity detection to equation analysis
- Classify equation as commutative vs noncommutative
- Identify variable position in noncommutative products

**New Analysis**:
```rust
pub struct EquationAnalysis {
    pub is_commutative: bool,
    pub division_side: Option<DivisionSide>,
    pub variable_position: VariablePosition,  // Left, Right, Mixed
    // ... existing fields
}

pub enum DivisionSide {
    Left,   // A*X = B
    Right,  // X*A = B
    Mixed,  // Both sides have variable
}
```

### 3. Matrix-Specific Solver (NEW)

**File**: `crates/mathhook-core/src/solvers/matrix_equations.rs` (create new)

**Purpose**: Specialized matrix equation solving

**Functions**:
```rust
/// Solve matrix equation A*X = B
pub fn solve_matrix_equation_left(A: Expression, B: Expression) -> Result<Expression, SolverError>

/// Solve matrix equation X*A = B
pub fn solve_matrix_equation_right(A: Expression, B: Expression) -> Result<Expression, SolverError>

/// Solve system of matrix equations
pub fn solve_matrix_system(equations: Vec<Expression>, variables: Vec<Symbol>) -> Result<Vec<Expression>, SolverError>
```

### 4. Test Suite (NEW)

**File**: `crates/mathhook-core/tests/matrix_equation_solver_tests.rs` (create new)

**Test Categories** (35+ tests total):
1. **Left division tests** (10 tests)
2. **Right division tests** (10 tests)
3. **Mixed equations** (5 tests)
4. **Operator equations** (5 tests)
5. **Quaternion equations** (5 tests)

---

## Implementation Strategy

### Phase 1: Analysis Infrastructure (1 hour)

1. **Add commutativity detection to equation analysis**
   - Check if all terms in equation are commutative
   - Identify variable position in products

2. **Create classification function**
   - Distinguish left/right/mixed division
   - Handle edge cases (variable appears multiple times)

### Phase 2: Linear Solver Updates (1 hour)

1. **Update existing linear solver**
   - Add commutativity check before division
   - Route to correct division handler

2. **Implement left/right division**
   - Left: `A*X = B` → `X = A^(-1)*B`
   - Right: `X*A = B` → `X = B*A^(-1)`

### Phase 3: Matrix Equation Solver (1 hour)

1. **Create new matrix_equations.rs module**
   - Specialized for Matrix symbol types
   - Leverage linear solver infrastructure

2. **Implement matrix-specific solutions**
   - Use matrix inverse operations
   - Handle singular matrices (error case)

### Phase 4: Testing (1 hour)

1. **Create comprehensive test suite**
   - 35+ tests covering all cases
   - Test with Matrix, Operator, Quaternion types
   - Test error handling (singular matrices, mixed types)

---

## Success Criteria

1. ✅ `solve(A*X = B, X)` correctly returns `A^(-1)*B` (left division)
2. ✅ `solve(X*A = B, X)` correctly returns `B*A^(-1)` (right division)
3. ✅ Matrix equations with Matrix symbols work
4. ✅ Operator equations with Operator symbols work
5. ✅ Quaternion equations with Quaternion symbols work
6. ✅ Commutative equations still work (backward compatible)
7. ✅ Error on mixed noncommutative types (A*X*B = C with noncommutative A, B)
8. ✅ 35+ tests covering all scenarios
9. ✅ Build passes with 0 errors
10. ✅ Zero regressions (all existing solver tests pass)

---

## Testing Strategy

### Test Categories (35+ tests):

**Left Division Tests (10 tests)**:
1. Simple matrix left division: `A*X = B`
2. Operator left division: `H*psi = E*psi` (Schrödinger equation form)
3. Quaternion left division: `q*x = r`
4. Scalar coefficient: `2*A*X = B`
5. Multiple matrices: `A*B*X = C`
6. Identity matrix: `I*X = B`
7. Zero RHS: `A*X = 0`
8. Variable appears once on left
9. Verify inverse is left-multiplied
10. Error on singular matrix

**Right Division Tests (10 tests)**:
11. Simple matrix right division: `X*A = B`
12. Operator right division: `psi*H = E*psi`
13. Quaternion right division: `x*q = r`
14. Scalar coefficient: `X*A*2 = B`
15. Multiple matrices: `X*A*B = C`
16. Identity matrix: `X*I = B`
17. Zero RHS: `X*A = 0`
18. Variable appears once on right
19. Verify inverse is right-multiplied
20. Error on singular matrix

**Mixed Equations (5 tests)**:
21. Variable on both sides: `A*X = X*B` (error or special handling)
22. Variable in middle: `A*X*B = C` (error for noncommutative)
23. Multiple variables: `A*X + B*Y = C`
24. Commutative mixed with noncommutative
25. Edge case: `X*A*X = B` (nonlinear, should error)

**Operator Equations (5 tests)**:
26. Quantum commutator: `[H, X] = 0` (special form)
27. Position-momentum: `p*x - x*p = -i*hbar`
28. Operator eigenvalue: `H*psi = E*psi`
29. Multiple operators: `H1*X + H2*X = B`
30. Pauli matrices: `sigma_x*X = Y`

**Quaternion Equations (5 tests)**:
31. Quaternion multiplication: `i*x = j`
32. Quaternion division: `x*j = k`
33. Quaternion inverse: `q*x = 1`
34. Mixed: `i*x*j = k` (error)
35. Quaternion magnitude: `q*conj(q) = |q|^2`

---

## Verification Script

**File**: `.mathhook_sessions/verify_wave_10_equation_solvers.sh`

**Categories**:
1. File size compliance (all solver files ≤500 lines)
2. Emoji compliance (zero emojis)
3. Build status (passes with 0 errors)
4. Linear solver updated (left/right division exists)
5. Matrix equation solver exists
6. Test count (≥35 tests)
7. All solver tests pass
8. Commutativity detection works
9. Documentation updated
10. Zero regressions (all existing tests pass)

---

## Files to Modify/Create

### Modify:
1. **crates/mathhook-core/src/solvers/linear.rs**
   - Add left/right division support
   - Add commutativity checking
   - Update existing `solve()` function

2. **crates/mathhook-core/src/solvers/mod.rs**
   - Add `pub mod matrix_equations;`
   - Export new functions

### Create:
3. **crates/mathhook-core/src/solvers/matrix_equations.rs** (NEW)
   - Matrix-specific equation solving
   - ~200-300 lines

4. **crates/mathhook-core/tests/matrix_equation_solver_tests.rs** (NEW)
   - 35+ comprehensive tests
   - ~500-600 lines

### Optional Enhancement:
5. **crates/mathhook-core/src/solvers/analyzer.rs** (create if doesn't exist)
   - Equation analysis utilities
   - Commutativity detection
   - Variable position classification

---

## Mathematical Background

### Matrix Equations

**Left Multiplication**: `A*X = B`
- Solution: `X = A^(-1)*B` (assuming A is invertible)
- Matrix inverse applied on the **left**

**Right Multiplication**: `X*A = B`
- Solution: `X = B*A^(-1)` (assuming A is invertible)
- Matrix inverse applied on the **right**

**Why Order Matters**:
```
A^(-1)*(A*X) = A^(-1)*B  // Left associativity
(A^(-1)*A)*X = A^(-1)*B  // A^(-1)*A = I
I*X = A^(-1)*B
X = A^(-1)*B
```

But:
```
A^(-1)*(X*A) ≠ (A^(-1)*X)*A  // Cannot reassociate!
```

### Operator Equations (Quantum Mechanics)

**Schrödinger Equation**: `H*psi = E*psi`
- H: Hamiltonian operator (noncommutative)
- psi: wavefunction
- E: energy eigenvalue (scalar, commutative)
- Solution: psi is eigenvector of H with eigenvalue E

**Commutator Relations**: `[A, B] = A*B - B*A`
- If `[A, B] = 0`, then A and B commute
- If `[A, B] ≠ 0`, order matters

### Quaternion Equations

**Quaternion Multiplication**: Not commutative
- `i*j = k`, but `j*i = -k`
- `j*k = i`, but `k*j = -i`
- `k*i = j`, but `i*k = -j`

**Division**:
- Left division: `q*x = r` → `x = q^(-1)*r`
- Right division: `x*q = r` → `x = r*q^(-1)`

---

## CLAUDE.md Compliance Requirements

1. **File Size**: All files ≤500 lines
2. **No Emojis**: Zero emojis in code/comments/docs
3. **Tests**: 35+ comprehensive tests
4. **Build**: Must pass with 0 errors
5. **Regressions**: Zero (all existing tests pass)
6. **Documentation**: All new functions have `///` docs with examples
7. **Module Docs**: Use `//!` for module-level documentation
8. **Mathematical Correctness**: Verify against SymPy/Symbolica
9. **Backward Compatibility**: Existing commutative equations still work
10. **Error Handling**: Proper error types for unsolvable cases

---

## Agent 10A Prompt Structure (When Ready)

**Agent 10A**: Equation Solvers Integration for Noncommutative Algebra

**Task**:
- Update linear solver for left/right division
- Create matrix equation solver module
- Add commutativity detection to equation analysis
- Create 35+ comprehensive tests
- Maintain zero regressions

**Deliverables**:
1. Modified `solvers/linear.rs` with left/right division
2. New `solvers/matrix_equations.rs` module
3. New `tests/matrix_equation_solver_tests.rs` with 35+ tests
4. All files ≤500 lines
5. Build passes, zero regressions
6. Documentation updated

**Target Quality**: 9.5+/10

---

## Timeline

**Phase 1**: Analysis Infrastructure (1 hour)
**Phase 2**: Linear Solver Updates (1 hour)
**Phase 3**: Matrix Equation Solver (1 hour)
**Phase 4**: Testing and Verification (1 hour)

**Total**: 3-4 hours

---

**This Wave 10 will enable MathHook to solve matrix equations correctly, unlocking quantum mechanics, linear algebra, and quaternion applications!**
