# Wave 3-INT Survey Findings

**Date**: 2025-10-22
**Status**: Survey Complete
**Decision**: Proceed with Integration

---

## Module Status

### Gröbner Basis Module (`crates/mathhook-core/src/algebra/groebner/`)

**Files** (5 total):
- `mod.rs` (287 lines) - `GroebnerBasis` struct, API
- `buchberger.rs` (427 lines) - Buchberger's algorithm with optimizations
- `monomial_order.rs` - Lex, GrLex, GrevLex orderings
- `reduction.rs` - Polynomial reduction algorithms
- `s_polynomial.rs` - S-polynomial computation

**Implementation Quality**:
- ✅ Mathematically sound (follows Buchberger's algorithm correctly)
- ✅ Well-tested (comprehensive test suite)
- ✅ Documented (doctests, examples)
- ✅ Optimized (Buchberger's criteria, auto-reduction)
- ✅ Follows CLAUDE.md patterns (mostly - minor macro usage updates needed)

**Core Functionality**:
```rust
// Create Gröbner basis
let gb = GroebnerBasis::new(polynomials, variables, MonomialOrder::Lex);
gb.compute();  // Runs Buchberger's algorithm

// Ideal membership testing
gb.contains(&polynomial)  // true if polynomial ∈ ideal

// Basis reduction
gb.reduce();  // Produces minimal reduced basis
```

---

## Integration Status

### Current State: NOT INTEGRATED

**Evidence**:
1. ❌ No mentions in `equation_analyzer.rs` (0 grep matches for "groebner")
2. ❌ Not used in `SystemSolver` (solvers/systems.rs - 0 matches)
3. ❌ Not connected to `SmartEquationSolver`
4. ❌ No integration tests exist
5. ❌ Not exposed through high-level API

### Architecture Gap Identified

**SystemSolver Current Capability**:
- **ONLY** linear systems: `ax + by = c, dx + ey = f`
- Uses Gaussian elimination (LU decomposition)
- Specialized 2x2 solver for performance
- **CANNOT** handle nonlinear polynomial systems

**Gröbner Basis Capability**:
- **Polynomial** systems: `x² + y² = 1, xy = 0`
- Solves systems of any degree
- Multiple variables supported
- Ideal operations (intersection, quotient, elimination)

**Perfect Integration Opportunity**:
```rust
// BEFORE Integration
SystemSolver::solve_system(&[x² + y - 2, x - y + 1], &[x, y])
// Result: NoSolution (cannot handle nonlinear!)

// AFTER Integration
SystemSolver::solve_system(&[x² + y - 2, x - y + 1], &[x, y])
// Result: Uses Gröbner basis → Finds solutions!
```

---

## Integration Strategy

### Option 1: Extend SystemSolver (RECOMMENDED)

**Rationale**: Natural fit - SystemSolver already handles equation systems

**Implementation**:
1. Detect polynomial vs linear systems
2. Route linear systems → Gaussian elimination (existing)
3. Route polynomial systems → Gröbner basis (new)

```rust
impl SystemEquationSolver for SystemSolver {
    fn solve_system(&self, equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        // Detect system type
        if is_linear_system(equations, variables) {
            self.solve_nxn_system(equations, variables)  // Existing
        } else if is_polynomial_system(equations, variables) {
            self.solve_polynomial_system_groebner(equations, variables)  // NEW
        } else {
            SolverResult::NoSolution
        }
    }
}
```

**Advantages**:
- Natural API (users already call `solve_system`)
- Extends existing capability
- Clear separation of concerns
- Educational: "Linear vs Polynomial systems"

### Option 2: Separate Polynomial System Solver

**Rationale**: Keep Gröbner basis separate from linear system solver

**Implementation**:
- Create `PolynomialSystemSolver` struct
- Implement `SystemEquationSolver` trait
- Add to `SmartEquationSolver` as separate field

**Advantages**:
- Clear module boundaries
- Easier to test independently
- More modular architecture

**Disadvantages**:
- User needs to know which solver to use
- Duplication in routing logic

---

## Recommended Approach

**Decision**: **Option 1 - Extend SystemSolver**

**Reasoning**:
1. Better user experience (automatic routing)
2. Follows existing Wave 1-INT and Wave 5-INT patterns
3. SystemSolver is already the "system equation solver"
4. Polynomial systems are a superset of linear systems

---

## Implementation Tasks

### Phase 1: Detection (Day 1, 2-3 hours)
- [ ] Add `is_polynomial_system()` helper function
- [ ] Detect polynomial degree for each equation
- [ ] Classify as linear, polynomial, or other

### Phase 2: Solver Integration (Day 1-2, 4-5 hours)
- [ ] Add `solve_polynomial_system_groebner()` method to SystemSolver
- [ ] Integrate `GroebnerBasis::new()` and `compute()`
- [ ] Extract solutions from Gröbner basis
- [ ] Handle edge cases (no solutions, infinite solutions)

### Phase 3: Educational Integration (Day 2, 2-3 hours)
- [ ] Add educational explanations for Gröbner basis method
- [ ] Step-by-step explanation of basis computation
- [ ] Explain when to use Gröbner vs Gaussian elimination
- [ ] Add to message registry

### Phase 4: Testing (Day 2-3, 3-4 hours)
- [ ] Unit tests for polynomial system detection
- [ ] Integration tests through `SystemSolver`
- [ ] Integration tests through `SmartEquationSolver`
- [ ] Test cases from Gröbner basis literature

### Phase 5: Verification (Day 3, 2 hours)
- [ ] Create verification script
- [ ] Run all existing tests (ensure no regressions)
- [ ] Document completion report
- [ ] Update architectural diagrams

**Total Estimated Time**: 2-3 days

---

## Success Criteria

Wave 3-INT is **COMPLETE** when:

- ✅ `SystemSolver` can solve polynomial systems using Gröbner basis
- ✅ Linear systems still use Gaussian elimination (no regression)
- ✅ System type detection works correctly
- ✅ Educational explanations exist
- ✅ Integration tests pass through `SmartEquationSolver`
- ✅ All existing tests still pass (901+ passing, 0 new failures)
- ✅ Verification script passes

---

## Example Use Cases

### Linear System (Existing - No Change)
```rust
let x = symbol!(x);
let y = symbol!(y);
let system = vec![
    expr!((2*x) + y - 5),    // 2x + y = 5
    expr!(x - y - 1),        // x - y = 1
];
let result = solver.solve_system(&system, &vec![x, y]);
// Uses Gaussian elimination (existing behavior)
```

### Polynomial System (NEW Capability)
```rust
let x = symbol!(x);
let y = symbol!(y);
let system = vec![
    expr!((x^2) + (y^2) - 1),   // x² + y² = 1 (circle)
    expr!(x - y),                 // x = y (line)
];
let result = solver.solve_system(&system, &vec![x, y]);
// Uses Gröbner basis → Finds intersection points!
```

### Mixed Degree System (NEW Capability)
```rust
let x = symbol!(x);
let y = symbol!(y);
let system = vec![
    expr!((x^2) - y),      // x² = y (parabola)
    expr!(x + y - 2),      // x + y = 2 (line)
];
let result = solver.solve_system(&system, &vec![x, y]);
// Uses Gröbner basis → Solves mixed system!
```

---

## Next Steps

1. **Mark survey complete** ✅
2. **Begin implementation** (Phase 1: Detection)
3. **Follow Wave 1-INT completion report pattern**
4. **Continuous testing** (no regressions)

---

**Conclusion**: Gröbner basis module is high-quality, ready for integration. Clear integration path identified through SystemSolver extension. Estimated 2-3 days to complete Wave 3-INT.
