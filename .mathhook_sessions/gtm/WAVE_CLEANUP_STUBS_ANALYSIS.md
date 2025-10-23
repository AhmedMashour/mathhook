# Wave CLEANUP: Stub and TODO Removal Analysis

**Date**: 2025-10-22
**Purpose**: Remove all stubs, TODOs, and placeholders from Plan 7 codebase
**Context**: Following Wave 3-INT (Gröbner Basis Integration), identified 39 instances of incomplete implementations

---

## Problem Statement

During Wave 3-INT, we added Gröbner basis integration to SystemSolver. However, the solution extraction was left as a stub with a comment:

```rust
// For now, return Partial with the basis as solution representation
// Full solution extraction requires:
// 1. Solve univariate polynomial in last variable
// 2. Back-substitute to find other variables
// 3. Handle multiple solutions (roots of polynomials)
```

**User Question**: "Why do we have stubs?"

**Answer**: Stubs exist because:
1. **Wave-based development**: We implement in phases, focusing on core integration first
2. **Mathematical complexity**: Some features (like polynomial root finding in Gröbner basis) require additional algorithms
3. **Testing priorities**: We verify the integration works before completing all features

**Goal**: Remove ALL stubs and complete all implementations to production quality

---

## Stub Inventory (39 total)

### High-Priority Stubs (Core Functionality)

#### 1. **Gröbner Basis Solution Extraction** (systems.rs)
**Location**: `crates/mathhook-core/src/algebra/solvers/systems.rs`
**Current State**: Partial solution extraction, returns NoSolution for complex cases
**Required Work**:
- Implement univariate polynomial solver for last variable
- Implement back-substitution algorithm
- Handle multiple solution sets
- Add proper error handling

**Impact**: High - This is the primary blocker for polynomial system solving

#### 2. **ODE Educational Methods** (ode/educational/wrapper.rs)
**Location**: `crates/mathhook-core/src/ode/educational/wrapper.rs`
**Current State**: Placeholder educational explanations
**Required Work**:
- Complete `solve_separable_with_steps` implementation
- Complete `solve_linear_with_steps` implementation
- Add proper step generation

**Impact**: Medium - Educational features are important but not blocking core functionality

#### 3. **Polynomial Advanced Features** (algebra/polynomial_advanced.rs)
**Location**: `crates/mathhook-core/src/algebra/polynomial_advanced.rs`
**Current State**: Various TODO markers for advanced polynomial operations
**Required Work**:
- Review each TODO and implement or remove
- Complete factorization algorithms
- Complete GCD computation

**Impact**: Medium - Advanced features, not critical for core

### Medium-Priority Stubs (Supporting Features)

#### 4. **Zero Detection** (algebra/zero_detection.rs)
**Location**: `crates/mathhook-core/src/algebra/zero_detection.rs`
**Current State**: Placeholder methods
**Required Work**:
- Implement robust zero detection
- Handle numerical zero detection
- Handle symbolic zero detection

**Impact**: Medium - Affects simplification quality

#### 5. **Risch Algorithm** (calculus/integrals/risch/)
**Location**: `crates/mathhook-core/src/calculus/integrals/risch/mod.rs`
**Current State**: Incomplete Risch algorithm implementation
**Required Work**:
- Complete Risch differential equation solver
- Complete integration algorithm
- Add comprehensive tests

**Impact**: Low - Symbolic integration is advanced feature

### Low-Priority Stubs (Future Features)

#### 6. **GPU Acceleration** (core/performance/gpu_acceleration.rs)
**Location**: `crates/mathhook-core/src/core/performance/gpu_acceleration.rs`
**Current State**: Placeholder for future GPU support
**Required Work**:
- Implement or remove (likely remove for now)

**Impact**: Low - Future optimization, not core functionality

#### 7. **Matrix Methods** (core/expression/matrix_methods.rs)
**Location**: `crates/mathhook-core/src/core/expression/matrix_methods.rs`
**Current State**: TODO markers for advanced matrix operations
**Required Work**:
- Review and implement or mark as future work

**Impact**: Low - Advanced linear algebra

---

## Cleanup Strategy

### Phase 1: Critical Stubs (Priority 1) - 2-3 days

**Target**: Complete Gröbner basis solution extraction

**Tasks**:
1. Implement univariate polynomial root finder
2. Implement back-substitution algorithm
3. Handle multiple solution sets
4. Add comprehensive tests
5. Update Wave 3-INT completion report

**Success Criteria**:
- Polynomial systems return actual solutions, not NoSolution
- Tests pass for circle-line intersection, parabola-line intersection
- No regressions in linear system solving

### Phase 2: Important Stubs (Priority 2) - 3-4 days

**Target**: Complete ODE educational features and zero detection

**Tasks**:
1. Complete ODE educational step generation
2. Implement robust zero detection
3. Clean up polynomial advanced features
4. Add tests for all completed features

**Success Criteria**:
- Educational ODE solvers provide complete explanations
- Zero detection works for symbolic and numerical cases
- All tests pass

### Phase 3: Documentation and Review (Priority 3) - 1-2 days

**Target**: Remove or document all remaining stubs

**Tasks**:
1. Review all remaining TODOs
2. Either implement or mark as "Future Enhancement"
3. Update documentation
4. Remove placeholder comments

**Success Criteria**:
- Zero instances of "For now" or "placeholder" in critical paths
- All TODOs either resolved or documented in FUTURE.md
- Codebase passes CLAUDE.md pre-commit checklist

---

## Estimated Timeline

**Total**: 6-9 days

- Phase 1 (Critical): 2-3 days
- Phase 2 (Important): 3-4 days
- Phase 3 (Cleanup): 1-2 days

---

## Integration with Plan 7

**This cleanup wave should be Wave 4 of Plan 7** (after Wave 3-INT)

**Rationale**:
- Wave 1-INT: Intelligence System (Complete)
- Wave 2-INT: Performance Recovery (Complete)
- Wave 3-INT: Gröbner Basis Integration (Complete, but with stubs)
- **Wave 4-CLEANUP**: Remove All Stubs (Proposed)
- Wave 5-INT: PDE Integration (Pending)

**Dependencies**:
- No blockers - can start immediately after Wave 3-INT verification
- Parallel work possible: Some stubs are independent

---

## Next Steps

1. Complete Wave 3-INT baseline benchmarks (in progress)
2. Document Wave 3-INT completion report
3. Begin Wave 4-CLEANUP Phase 1 (Gröbner solution extraction)
4. Track progress in `.mathhook_sessions/gtm/WAVE_4_CLEANUP_STATUS.md`

---

## Notes

**User Expectation**: Production-quality code without stubs or placeholders

**Technical Debt**: Stubs are acceptable during wave development, but each wave should complete its scope fully before marking as done. Wave 3-INT should have included complete solution extraction.

**Lesson Learned**: Future waves should complete ALL functionality within their scope, not leave stubs for "later". If a feature is too complex, break it into sub-waves.
