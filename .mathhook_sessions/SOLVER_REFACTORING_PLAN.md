# Solver Refactoring Plan - Wave 2 Implementation Guide

## Executive Summary

This document provides a detailed, actionable plan for Wave 2 agents to execute the solver refactoring. The goal is to delete the unnecessary wrapper layer (`src/solvers.rs`, 236 lines) and promote the complete `SmartEquationSolver` to the public API.

**Approach**: Rename `SmartEquationSolver` to `MathSolver` (reuse the familiar name) and update all usage.

**Estimated Complexity**: Medium
- Files to DELETE: 1 file (236 lines)
- Files to MODIFY: ~15 files (exports, bindings, tests, examples)
- Test updates: ~58 occurrences of `MathSolver::new()`
- Expected outcome: Zero regressions, expanded capabilities

## 1. API Naming Decision

### Decision: Rename `SmartEquationSolver` to `MathSolver`

**Rationale**:

1. **Familiar name**: Users already know `MathSolver` from existing API
2. **Simple and clear**: "MathSolver" is more user-friendly than "SmartEquationSolver"
3. **Minimal migration**: Tests and documentation already use `MathSolver`
4. **Clean upgrade**: Users get MORE functionality with the same name

**Alternatives Considered**:

- **Option A**: Keep `SmartEquationSolver` name
  - Pros: No name collision, explicit "smart" capability
  - Cons: Less friendly name, documentation must change everywhere
  - Verdict: REJECTED - worse user experience

- **Option B**: Different name entirely (e.g., `EquationSolver`, `Solver`)
  - Pros: Fresh start
  - Cons: Even more documentation changes, less clear
  - Verdict: REJECTED - unnecessary complexity

**Chosen Option C**: Rename `SmartEquationSolver` to `MathSolver`
- Pros: Familiar name, minimal migration, clear purpose
- Cons: None significant
- Verdict: SELECTED

## 2. Export Strategy

### 2.1 Current Exports (Before Refactoring)

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/lib.rs`

**Current state** (Lines 19, 32):
```rust
pub mod solvers;  // Exports wrapper module

pub use solvers::*;  // Exports: MathSolver (wrapper), SolverConfig, SolverResult (simplified)
```

**Problem**: This exports the WRONG types (wrapper instead of complete implementation).

### 2.2 New Exports (After Refactoring)

**Step 1**: Remove old `solvers` module export (Line 19):
```rust
// DELETE THIS LINE:
pub mod solvers;
```

**Step 2**: Add explicit exports for equation solver types (new line after Line 18):
```rust
// ADD THESE LINES:
pub use algebra::equation_analyzer::{EquationAnalyzer, EquationType, MathSolver};
pub use algebra::solvers::{EquationSolver, SolverResult, SystemEquationSolver};
```

**Step 3**: Update prelude (Lines 35-44):
```rust
pub mod prelude {
    pub use crate::macros::*;
    pub use crate::{expr, function, parse, symbol};
    pub use crate::{
        AdvancedSimplify, Collect, ComplexOperations, Expand, Factor, PolynomialGcd,
        RationalSimplify, Simplify, ZeroDetection,
    };
    pub use crate::{Expression, MathConstant, Number, Symbol};

    // UPDATE THIS LINE (remove SolverConfig):
    pub use crate::{MathSolver, SolverResult};  // Now references complete types
}
```

**Note**: `SolverConfig` is removed because it was never used by `SmartEquationSolver`.

### 2.3 Export Verification Commands

After making changes, verify exports:

```bash
# Check that MathSolver is accessible
cargo doc --no-deps --open

# Check that complete SolverResult is exported
rg "pub enum SolverResult" crates/mathhook-core/src/

# Verify no references to old wrapper remain
rg "crate::solvers::" crates/mathhook-core/src/
```

## 3. Step-by-Step Migration Path

### Step 1: Rename `SmartEquationSolver` to `MathSolver`

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/equation_analyzer.rs`

**Action**: Global search and replace within this file

**Search**: `SmartEquationSolver`
**Replace**: `MathSolver`

**Occurrences** (estimated 10 occurrences):
- Line 127: `pub struct SmartEquationSolver {`
- Line 135: `impl SmartEquationSolver {`
- Line 258: `Symbol::new(&name)` (example, if SmartEquationSolver is mentioned in comments)

**Verification**:
```bash
# After rename, check that no old name remains
rg "SmartEquationSolver" crates/mathhook-core/src/algebra/equation_analyzer.rs

# Should return zero results
```

**Important**: Update the module documentation comment (Line 1-2) to reflect the new name:

```rust
//! Analyzes LaTeX equations and routes to appropriate solvers
//! This module provides the MathSolver, the primary equation solving interface
```

### Step 2: Delete Old Wrapper Layer

**File to DELETE**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs`

**Size**: 236 lines

**Action**: Delete the entire file

**Command**:
```bash
rm /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs
```

**Verification**:
```bash
# Verify file is deleted
ls /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs

# Should return: No such file or directory
```

**Git tracking**:
```bash
git rm crates/mathhook-core/src/solvers.rs
```

### Step 3: Update Module Exports in `lib.rs`

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/lib.rs`

**Change 1**: Remove old `solvers` module (Line 19):
```rust
// DELETE THIS LINE:
pub mod solvers;
```

**Change 2**: Add new exports (insert after line 18, before line 20):
```rust
// ADD THESE LINES:
pub use algebra::equation_analyzer::{EquationAnalyzer, EquationType, MathSolver};
pub use algebra::solvers::{EquationSolver, SolverResult, SystemEquationSolver};
```

**Change 3**: Update prelude (Lines 43):
```rust
// OLD:
pub use crate::{MathSolver, SolverConfig, SolverResult};

// NEW:
pub use crate::{MathSolver, SolverResult};
```

**Verification**:
```bash
# Check that exports compile
cargo check --lib

# Expected: Should compile (may have unrelated errors in calculus/integrals)
```

### Step 4: Update `algebra.rs` Module Exports

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra.rs`

**Current state** (Line 33):
```rust
pub use equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
```

**Change**: Update renamed type:
```rust
// NEW:
pub use equation_analyzer::{EquationAnalyzer, EquationType, MathSolver};
```

**Note**: This change is optional if we're exporting directly from `lib.rs`, but keeps consistency.

### Step 5: Update Python Bindings

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-python/src/lib.rs`

**Change 1**: Update import (Line 7):
```rust
// OLD:
use mathhook_core::{Expression, MathSolver, Parser, Simplify, Symbol};

// NEW (no change needed if we renamed to MathSolver):
use mathhook_core::{Expression, MathSolver, Parser, Simplify, Symbol};
```

**Change 2**: Update `PyMathSolver::solve()` to return structured result (Lines 254-258):

**OLD**:
```rust
pub fn solve(&mut self, equation: &PyExpression, variable: &str) -> String {
    let symbol = Symbol::new(variable);
    let result = self.inner.solve(&equation.inner, &symbol);
    format!("{:?}", result)  // Returns Debug string
}
```

**NEW** (expose complete result):
```rust
pub fn solve(&mut self, equation: &PyExpression, variable: &str) -> String {
    let symbol = Symbol::new(variable);
    let (result, _explanation) = self.inner.solve_with_equation(&equation.inner, &symbol);
    format!("{:?}", result)  // Returns complete SolverResult with Parametric/Partial
}
```

**Enhancement (Optional for Wave 2, document for future)**:

Add method to expose step-by-step explanations:
```rust
pub fn solve_with_explanation(&mut self, equation: &PyExpression, variable: &str) -> (String, String) {
    let symbol = Symbol::new(variable);
    let (result, explanation) = self.inner.solve_with_equation(&equation.inner, &symbol);
    (format!("{:?}", result), format!("{}", explanation))
}
```

**Verification**:
```bash
# Check Python bindings compile
cargo check -p mathhook-python
```

### Step 6: Update Node.js Bindings

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-node/src/lib.rs`

**Change 1**: Update import (Line 8):
```rust
// OLD:
use mathhook_core::{Expression, MathSolver, Simplify, Symbol};

// NEW (no change needed if we renamed to MathSolver):
use mathhook_core::{Expression, MathSolver, Simplify, Symbol};
```

**Change 2**: Update `JsMathSolver::solve()` to return complete result (Lines 264-268):

**OLD**:
```rust
#[napi]
pub fn solve(&mut self, equation: &JsExpression, variable: String) -> String {
    let symbol = Symbol::new(variable);
    let result = self.inner.solve(&equation.inner, &symbol);
    format!("{:?}", result)
}
```

**NEW** (expose complete result):
```rust
#[napi]
pub fn solve(&mut self, equation: &JsExpression, variable: String) -> String {
    let symbol = Symbol::new(variable);
    let (result, _explanation) = self.inner.solve_with_equation(&equation.inner, &symbol);
    format!("{:?}", result)  // Now includes Parametric/Partial variants
}
```

**Enhancement (Optional for Wave 2, document for future)**:

Add method to expose step-by-step explanations:
```rust
#[napi]
pub fn solve_with_explanation(&mut self, equation: &JsExpression, variable: String) -> Vec<String> {
    let symbol = Symbol::new(variable);
    let (result, explanation) = self.inner.solve_with_equation(&equation.inner, &symbol);
    vec![format!("{:?}", result), format!("{}", explanation)]
}
```

**Verification**:
```bash
# Check Node bindings compile
cargo check -p mathhook-node
```

### Step 7: Update TypeScript Definitions (Node.js)

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-node/index.d.ts`

**Action**: Search for `MathSolver` and verify TypeScript definitions match new API.

**Expected**: Should require minimal or no changes (method names stay the same).

**Verification**: Open file and review `JsMathSolver` interface.

### Step 8: Update Core Tests

**Strategy**: Global search and replace across test files

**Search pattern**: `MathSolver::new()` (58 occurrences)

**Action**: NO CHANGE NEEDED if we renamed `SmartEquationSolver` to `MathSolver`

**Rationale**: Tests already use `MathSolver::new()`, which will now refer to the renamed (and better) implementation.

**Verification command**:
```bash
# Check that tests still reference MathSolver
rg "MathSolver::new" crates/mathhook-core/tests/

# Should return ~58 occurrences (unchanged)
```

**Potential issue**: Tests that check `SolverResult` enum variants may need updates.

**Example test that may need adjustment**:

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/sympy_validation/solver_tests.rs`

**OLD test**:
```rust
match result {
    SolverResult::Single(solution) => { /* ... */ }
    _ => panic!("Expected single solution"),
}
```

**NEW test** (if solver now returns Partial or Parametric):
```rust
match result {
    SolverResult::Single(solution) => { /* ... */ }
    SolverResult::Partial(solutions) if solutions.len() == 1 => {
        // Handle partial solution case
    }
    _ => panic!("Expected single or partial solution"),
}
```

**Action for Wave 2**: Run tests after refactoring, update any that fail due to new result variants.

### Step 9: Update Integration Tests

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/integration/mathematical_workflows.rs`

**Estimated occurrences**: 3

**Action**: Verify integration tests still pass with renamed solver.

**Expected**: No changes needed (same API, better implementation).

### Step 10: Update Doctests in `lib.rs`

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/lib.rs`

**Doctest location**: Lines 82-104

**OLD doctest**:
```rust
#[test]
fn test_solver_object_api() {
    let mut solver = MathSolver::new();
    let equation = Expression::equation(expr!(x), expr!(42));
    let result = solver.solve(&equation, &symbol!(x));
    match result {
        SolverResult::Single(solution) => { /* ... */ }
        _ => panic!("Expected single solution"),
    }
}
```

**NEW doctest** (update to use `solve_with_equation` if desired):
```rust
#[test]
fn test_solver_object_api() {
    let mut solver = MathSolver::new();
    let equation = Expression::equation(expr!(x), expr!(42));
    let (result, _explanation) = solver.solve_with_equation(&equation, &symbol!(x));
    match result {
        SolverResult::Single(solution) => { /* ... */ }
        _ => panic!("Expected single solution"),
    }
}
```

**Alternative**: Keep the old test as-is if we add a simple `solve()` method to the new `MathSolver` that discards the explanation.

### Step 11: Update Documentation Files

**Files to update**:

1. `/Users/ahmedmashhour/Documents/work/math/mathhook/README.md`
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/USAGE.md`
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/docs/src/introduction.md`
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/docs/src/getting-started/quick-start.md`
5. `/Users/ahmedmashhour/Documents/work/math/mathhook/docs/src/getting-started/common-patterns.md`

**Action for each file**:

1. **Search for**: `MathSolver` references
2. **Verify**: Examples are still correct (API is compatible)
3. **Add**: Documentation for new capabilities:
   - `SolverResult::Parametric` variant
   - `SolverResult::Partial` variant
   - `solve_with_equation()` method for educational explanations

**Example addition to README.md**:

```markdown
## Enhanced Solver Results

MathSolver now provides complete information about solution types:

- `Single(expr)`: Exactly one solution
- `Multiple(exprs)`: Multiple distinct solutions
- `Parametric(exprs)`: Infinite solutions parameterized by free variables
- `Partial(exprs)`: Some solutions found, others may exist
- `NoSolution`: No solution exists
- `InfiniteSolutions`: Infinite solutions (no parameterization)

Get step-by-step explanations:

\```rust
let (result, explanation) = solver.solve_with_equation(&equation, &variable);
println!("{}", explanation);
\```
```

**Verification**: Build documentation and check for broken links:
```bash
cargo doc --no-deps --open
```

### Step 12: Update Internal Usage in `core/expression/methods.rs`

**File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/methods.rs`

**Expected**: Contains internal usage of `SmartEquationSolver`

**Action**: Search and replace `SmartEquationSolver` with `MathSolver`

**Verification**:
```bash
rg "SmartEquationSolver" crates/mathhook-core/src/core/expression/methods.rs

# Should return zero results after replacement
```

### Step 13: Update Example Files (If Any)

**Search for examples**:
```bash
find crates/ -name "*.rs" -path "*/examples/*" -exec grep -l "MathSolver" {} \;
```

**Expected**: Examples in:
- `crates/mathhook-python/examples/` (Python scripts, not Rust)
- `crates/mathhook-node/examples/` (TypeScript/JavaScript, not Rust)

**Action**: Review examples and update if necessary (likely minimal changes).

## 4. Complete File List

### Files to DELETE

1. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs` (236 lines)

### Files to MODIFY (Core Implementation)

1. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/equation_analyzer.rs`
   - Rename `SmartEquationSolver` to `MathSolver` (global search-replace)
   - Update module documentation

2. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/lib.rs`
   - Remove `pub mod solvers;` (line 19)
   - Add explicit exports for `MathSolver`, `SolverResult`, etc.
   - Update prelude (remove `SolverConfig`, keep `MathSolver` and `SolverResult`)

3. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra.rs`
   - Update export: `SmartEquationSolver` → `MathSolver` (line 33)

4. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/methods.rs`
   - Replace `SmartEquationSolver` with `MathSolver` (if used)

### Files to MODIFY (Bindings)

5. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-python/src/lib.rs`
   - Update `solve()` method to use `solve_with_equation()`
   - Optional: Add `solve_with_explanation()` method

6. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-node/src/lib.rs`
   - Update `solve()` method to use `solve_with_equation()`
   - Optional: Add `solve_with_explanation()` method

7. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-node/index.d.ts`
   - Review TypeScript definitions (likely no changes needed)

### Files to MODIFY (Tests)

8. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/sympy_validation/solver_tests.rs`
   - Verify tests pass with new implementation
   - Update any tests that fail due to new result variants

9. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/algebra/equation_verification.rs`
   - Verify tests pass
   - Update if necessary

10. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/tests/integration/mathematical_workflows.rs`
    - Verify integration tests pass
    - Update if necessary

**Note**: Approximately 50+ test files may contain `MathSolver::new()`, but if we rename `SmartEquationSolver` to `MathSolver`, most tests should work without modification.

### Files to MODIFY (Documentation)

11. `/Users/ahmedmashhour/Documents/work/math/mathhook/README.md`
    - Add documentation for new result variants
    - Add example of `solve_with_equation()`

12. `/Users/ahmedmashhour/Documents/work/math/mathhook/USAGE.md`
    - Update usage examples if needed

13. `/Users/ahmedmashhour/Documents/work/math/mathhook/docs/src/introduction.md`
    - Verify examples are correct

14. `/Users/ahmedmashhour/Documents/work/math/mathhook/docs/src/getting-started/quick-start.md`
    - Verify quick-start examples are correct

15. `/Users/ahmedmashhour/Documents/work/math/mathhook/docs/src/getting-started/common-patterns.md`
    - Add patterns for new capabilities

## 5. Success Criteria (MANDATORY Verification)

### 5.1 Architecture Criteria

1. ✅ Old `src/solvers.rs` deleted (236 lines removed)
2. ✅ `SmartEquationSolver` renamed to `MathSolver` in `algebra/equation_analyzer.rs`
3. ✅ `MathSolver` exported from `lib.rs`
4. ✅ Complete `SolverResult` enum exported from `lib.rs` (with Parametric and Partial variants)
5. ✅ No references to old wrapper remain in codebase

**Verification commands**:
```bash
# Verify old file deleted
ls crates/mathhook-core/src/solvers.rs  # Should error

# Verify new exports present
rg "pub use algebra::equation_analyzer::.*MathSolver" crates/mathhook-core/src/lib.rs

# Verify no old wrapper references
rg "crate::solvers::" crates/mathhook-core/src/
```

### 5.2 Bindings Criteria

6. ✅ Python bindings updated to use `solve_with_equation()`
7. ✅ Node bindings updated to use `solve_with_equation()`
8. ✅ Python can access complete `SolverResult` (Parametric/Partial solutions)
9. ✅ Node can access complete `SolverResult` (Parametric/Partial solutions)

**Verification commands**:
```bash
# Verify bindings compile
cargo check -p mathhook-python
cargo check -p mathhook-node

# Verify solve methods use solve_with_equation
rg "solve_with_equation" crates/mathhook-python/src/lib.rs
rg "solve_with_equation" crates/mathhook-node/src/lib.rs
```

### 5.3 Testing Criteria

10. ✅ Core library compiles: `cargo build --lib`
11. ✅ All tests run (account for pre-existing build errors)
12. ✅ Test count unchanged or increased (baseline: check current count)
13. ✅ Zero regressions in passing tests
14. ✅ SymPy validation tests still pass

**Verification commands**:
```bash
# Build check (will have pre-existing errors in calculus/integrals)
cargo build --lib 2>&1 | tee build_output.txt

# Run tests (exclude tests that require calculus integration which has errors)
cargo test --lib --bins

# Check test count
cargo test --workspace --lib 2>&1 | grep "test result:"
```

**Important**: The codebase currently has 11 pre-existing compilation errors in `calculus/integrals/risch` module. These are UNRELATED to solver refactoring. Success means:
- Solver code compiles
- Solver tests pass
- No NEW errors introduced

### 5.4 Build Criteria

15. ✅ `cargo build --all-targets` compiles (with pre-existing errors documented)
16. ✅ `cargo check --workspace` passes (with pre-existing errors documented)
17. ✅ `cargo doc --no-deps` builds documentation successfully
18. ✅ No clippy warnings in modified files: `cargo clippy --lib`

**Verification commands**:
```bash
cargo check --workspace 2>&1 | tee check_output.txt
cargo clippy --lib -- -D warnings
cargo doc --no-deps
```

### 5.5 Documentation Criteria

19. ✅ README updated with new capabilities
20. ✅ USAGE docs updated
21. ✅ API docs build and show `MathSolver` correctly
22. ✅ Doctests in `lib.rs` pass
23. ✅ No broken documentation links

**Verification commands**:
```bash
cargo test --doc
cargo doc --no-deps --open  # Manual review
```

### 5.6 Quality Criteria

24. ✅ No hardcoded type conversions remain (old `convert_solver_result()` deleted)
25. ✅ Complete `SolverResult` enum accessible in all contexts
26. ✅ Educational features accessible (step-by-step explanations)
27. ✅ Simpler architecture (one fewer layer)
28. ✅ CLAUDE.md principles followed (no information loss, complete functionality exposed)
29. ✅ Code reduction achieved (236 lines deleted, minimal additions)
30. ✅ 10/10 quality score on orchestrator verification

**Manual verification**:
- Review refactored code for clarity
- Verify no abstraction layers lost information
- Check that educational features are accessible

## 6. Risk Mitigation Plan

### 6.1 Pre-existing Build Errors

**Risk**: Codebase has 11 compilation errors in `calculus/integrals/risch` module (unrelated to solvers).

**Mitigation**:
1. Document these errors in verification report
2. Ensure refactoring does NOT introduce new errors
3. Focus verification on solver-related code
4. Compare pre-refactoring vs post-refactoring error count (should be identical)

**Acceptance**: Refactoring is successful if:
- Solver code compiles
- Solver tests pass
- Error count unchanged (11 errors remain, all in calculus/integrals/risch)

### 6.2 Test Migration Complexity

**Risk**: 58 occurrences of `MathSolver::new()` in tests may need updates.

**Mitigation**:
1. Rename `SmartEquationSolver` to `MathSolver` (tests already use this name)
2. Run full test suite after refactoring
3. Update only tests that fail (minimal expected)
4. Tests that check `SolverResult` variants may need adjustment for Parametric/Partial

**Rollback Plan**: If >10 tests fail:
1. Analyze common failure pattern
2. Create helper migration script if pattern is consistent
3. Update tests iteratively

### 6.3 Bindings Compatibility

**Risk**: Python/Node bindings may break if API changes.

**Mitigation**:
1. Bindings call `solve_with_equation()` instead of `solve()`
2. Wrapper still returns string representation (no breaking change for bindings)
3. Test bindings explicitly: `cargo check -p mathhook-python && cargo check -p mathhook-node`

**Rollback Plan**: If bindings fail:
1. Add compatibility method `solve()` that discards explanation
2. Keep bindings using simplified API temporarily
3. Document enhancement path for future

### 6.4 Documentation Staleness

**Risk**: Documentation may reference old API or be inconsistent.

**Mitigation**:
1. Update all documentation files in Step 11
2. Build and review docs: `cargo doc --no-deps --open`
3. Check for broken links or outdated examples
4. Add new examples showcasing Parametric/Partial results

**Rollback Plan**: If docs are broken:
1. Fix broken examples immediately
2. Mark new features as "experimental" in docs if needed
3. Complete documentation in Wave 3 verification

## 7. Verification Protocol for Wave 2

When Wave 2 implementation is complete, run this verification checklist:

### 7.1 Automated Verification Script

Create a verification script (Wave 3 responsibility):

```bash
#!/bin/bash
# File: verify_solver_refactoring_wave_2.sh

echo "=== Wave 2 Solver Refactoring Verification ==="

echo "[1/10] Checking old wrapper deleted..."
if [ -f "crates/mathhook-core/src/solvers.rs" ]; then
    echo "❌ FAIL: Old solvers.rs still exists"
    exit 1
fi
echo "✅ PASS: Old wrapper deleted"

echo "[2/10] Checking MathSolver renamed..."
if rg -q "SmartEquationSolver" crates/mathhook-core/src/algebra/equation_analyzer.rs; then
    echo "❌ FAIL: SmartEquationSolver name still present"
    exit 1
fi
echo "✅ PASS: MathSolver renamed"

echo "[3/10] Checking exports updated..."
if rg -q "pub use algebra::equation_analyzer::.*MathSolver" crates/mathhook-core/src/lib.rs; then
    echo "✅ PASS: MathSolver exported from lib.rs"
else
    echo "❌ FAIL: MathSolver not exported"
    exit 1
fi

echo "[4/10] Building core library..."
cargo build --lib 2>&1 | tee build_output.txt
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ PASS: Core library builds"
else
    echo "⚠️  WARN: Build has errors (check if they're pre-existing)"
fi

echo "[5/10] Checking bindings compile..."
cargo check -p mathhook-python -p mathhook-node
if [ $? -eq 0 ]; then
    echo "✅ PASS: Bindings compile"
else
    echo "❌ FAIL: Bindings have errors"
    exit 1
fi

echo "[6/10] Running core tests..."
cargo test --lib 2>&1 | tee test_output.txt
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ PASS: Core tests pass"
else
    echo "❌ FAIL: Core tests have failures"
    exit 1
fi

echo "[7/10] Running solver-specific tests..."
cargo test -p mathhook-core solver 2>&1 | grep "test result"
echo "✅ Solver tests completed (check output)"

echo "[8/10] Checking documentation builds..."
cargo doc --no-deps
if [ $? -eq 0 ]; then
    echo "✅ PASS: Documentation builds"
else
    echo "❌ FAIL: Documentation has errors"
    exit 1
fi

echo "[9/10] Running clippy on modified code..."
cargo clippy --lib -- -D warnings 2>&1 | tee clippy_output.txt
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ PASS: No clippy warnings"
else
    echo "⚠️  WARN: Clippy has warnings (review)"
fi

echo "[10/10] Checking for old wrapper references..."
if rg -q "crate::solvers::" crates/mathhook-core/src/; then
    echo "❌ FAIL: Old wrapper references remain"
    exit 1
fi
echo "✅ PASS: No old wrapper references"

echo ""
echo "=== Verification Summary ==="
echo "✅ All critical checks passed"
echo "Review warnings in output files:"
echo "  - build_output.txt"
echo "  - test_output.txt"
echo "  - clippy_output.txt"
```

### 7.2 Manual Verification Steps

After automated verification, manually check:

1. **API Accessibility**: Open documentation and verify `MathSolver` is public and well-documented
2. **Result Variants**: Check that `SolverResult::Parametric` and `SolverResult::Partial` are documented
3. **Educational Features**: Verify `solve_with_equation()` is documented and accessible
4. **Example Code**: Verify examples in README work correctly
5. **Bindings**: Test Python and Node bindings manually (if possible)

## 8. Rollback Plan

If Wave 2 fails verification, rollback strategy:

### 8.1 Git Rollback

```bash
# If changes are committed
git revert <commit-hash>

# If changes are not committed
git reset --hard HEAD

# Restore deleted file
git checkout HEAD -- crates/mathhook-core/src/solvers.rs
```

### 8.2 Partial Rollback

If some changes are good but others fail:

1. Keep architectural changes (rename, exports)
2. Rollback bindings changes
3. Rollback test changes
4. Create continuation agent for problematic areas

### 8.3 Continuation Agent

If Wave 2 doesn't complete all tasks:

1. Document completed tasks
2. Document remaining tasks
3. Launch Agent 2B to complete remaining work
4. Provide specific failure analysis

## 9. Wave 3 Preview: Verification & Documentation

After Wave 2 implementation, Wave 3 will:

1. Run comprehensive verification script
2. Update all documentation with new capabilities
3. Create migration guide for users (if any external users exist)
4. Write quality achievement report
5. Update CLAUDE.md if architectural insights emerged
6. Celebrate successful refactoring

**Wave 3 deliverables**:
- Verification report (similar to WAVE_10_VERIFICATION_REPORT.md)
- Quality achievement report (similar to WAVE_10_QUALITY_ACHIEVEMENT_REPORT.md)
- Updated documentation (README, USAGE, API docs)
- Migration guide (if needed)

## 10. Summary Checklist for Wave 2 Agent

When executing Wave 2, follow this checklist:

### Phase 1: Core Refactoring
- [ ] Step 1: Rename `SmartEquationSolver` to `MathSolver` in `equation_analyzer.rs`
- [ ] Step 2: Delete `src/solvers.rs` (236 lines)
- [ ] Step 3: Update exports in `lib.rs`
- [ ] Step 4: Update exports in `algebra.rs`
- [ ] Verify: `cargo check --lib` compiles (with pre-existing errors acceptable)

### Phase 2: Bindings Update
- [ ] Step 5: Update Python bindings
- [ ] Step 6: Update Node bindings
- [ ] Step 7: Review TypeScript definitions
- [ ] Verify: `cargo check -p mathhook-python -p mathhook-node` passes

### Phase 3: Test Migration
- [ ] Step 8: Update core tests (if needed)
- [ ] Step 9: Update integration tests (if needed)
- [ ] Step 10: Update doctests in `lib.rs`
- [ ] Verify: `cargo test --lib` passes (check test count unchanged)

### Phase 4: Documentation
- [ ] Step 11: Update documentation files (README, USAGE, docs/)
- [ ] Step 12: Update internal usage in `core/expression/methods.rs`
- [ ] Step 13: Review examples (if any)
- [ ] Verify: `cargo doc --no-deps` builds successfully

### Phase 5: Final Verification
- [ ] Run all 30 success criteria checks
- [ ] Document any deviations or issues
- [ ] Create completion report
- [ ] Hand off to Wave 3 (Verification & Documentation)

## 11. Estimated Time and Complexity

**Total Estimated Time**: 2-4 hours for experienced Rust developer

**Complexity Breakdown**:
- Core refactoring (Steps 1-4): 30 minutes (straightforward rename and delete)
- Bindings update (Steps 5-7): 30 minutes (update method calls)
- Test migration (Steps 8-10): 1-2 hours (depends on test failures)
- Documentation (Steps 11-13): 30-60 minutes (update examples)
- Verification (Steps in Section 6): 30 minutes (run checks)

**Difficulty**: Medium
- Low risk (clear plan, minimal changes)
- Medium complexity (multiple files to update)
- High value (architectural improvement, no information loss)

## 12. Conclusion

This refactoring plan provides a detailed, step-by-step guide to remove the unnecessary wrapper layer and promote the complete `SmartEquationSolver` (renamed to `MathSolver`) to the public API.

**Key Benefits**:
1. No information loss (Parametric and Partial variants accessible)
2. Educational features accessible (step-by-step explanations)
3. Simpler architecture (delete 236 lines)
4. Better API design (complete functionality exposed)
5. CLAUDE.md compliance (mathematical correctness, no unnecessary abstractions)

**Risk Assessment**: Low risk
- Clear rollback plan
- Pre-existing build errors documented
- Minimal API surface changes (rename, not redesign)

**Readiness**: Wave 2 can execute immediately based on this plan.

**Success Criteria**: 30 concrete, measurable criteria defined in Section 5.

**Next Step**: Wave 2 agent begins implementation following this plan.

---

**Document Metadata**:
- Author: Agent 1A (Wave 1: Analysis & Planning)
- Date: 2025-10-20
- Lines: 988
- Status: Complete, ready for Wave 2 execution
- Quality: Actionable, specific, comprehensive
