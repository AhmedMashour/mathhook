# Agent P0-1: Pattern Matching Architect

**Task**: P0-1 - Implement Pattern Matching & Substitution System
**Status**: IN_PROGRESS
**Progress**: 70% (Reality Check: Tests show critical issues)
**Priority**: P0 (CRITICAL BLOCKER)
**Estimated Duration**: 2-3 weeks
**Started**: 2025-10-13
**Last Update**: 2025-10-13 05:20 - VERIFICATION: 23/31 tests pass, 8 failures due to missing simplification integration

---

## Mission Briefing

Implement a complete pattern matching and substitution system for MathHook. This is the **most critical missing feature** - both SymPy and Symbolica have this as fundamental functionality.

**Why Critical**: Without pattern matching & substitution, MathHook cannot:
- Verify equation solutions
- Implement integration by substitution
- Apply transformation rules systematically
- Perform basic algebraic manipulation

**Reference Material**:
- Task details: `.mathhook_sessions/0.1_RELEASE_READINESS_AI_AGENT.md` (TASK P0-1)
- Guidelines: `CLAUDE.md`
- SymPy reference: `~/Documents/work/math/sympy/`

---

## Current Objective

Waiting for launch command...

---

## Implementation Plan

### Phase 1: Basic Substitution (Week 1)
- [ ] Create `crates/mathhook-core/src/pattern/mod.rs`
- [ ] Create `crates/mathhook-core/src/pattern/substitution.rs`
- [ ] Implement `Expression::subs(old, new)` for basic variable substitution
- [ ] Implement recursive tree walking
- [ ] Handle edge cases (substituting in functions, nested substitutions)
- [ ] Add 20 basic substitution tests

### Phase 2: Multiple Substitutions (Week 1-2)
- [ ] Implement `Expression::subs_multiple(pairs)`
- [ ] Optimize for efficiency (single-pass substitution)
- [ ] Add tests for simultaneous substitutions

### Phase 3: Pattern Matching Infrastructure (Week 2)
- [ ] Create `crates/mathhook-core/src/pattern/matching.rs`
- [ ] Design `Pattern` struct
- [ ] Implement structural pattern matching
- [ ] Support wildcards and constraints
- [ ] Add 15 pattern matching tests

### Phase 4: Pattern Replacement (Week 2-3)
- [ ] Implement `Expression::matches(pattern)` returning bindings
- [ ] Implement `Expression::replace(pattern, replacement)`
- [ ] Support recursive pattern matching
- [ ] Add 15 pattern replacement tests

### Phase 5: Integration & Testing (Week 3)
- [ ] Integration tests with equation solving
- [ ] Integration tests with simplification rules
- [ ] Verify solution checking works: `eq.subs(x, solution).simplify() == 0`
- [ ] Performance testing
- [ ] Documentation and examples

---

## Completed Work

_Nothing yet - waiting for launch_

---

## Files to Create

- [ ] `crates/mathhook-core/src/pattern/mod.rs`
- [ ] `crates/mathhook-core/src/pattern/substitution.rs`
- [ ] `crates/mathhook-core/src/pattern/matching.rs`
- [ ] `crates/mathhook-core/tests/pattern_tests.rs`

---

## Files to Modify

- [ ] `crates/mathhook-core/src/lib.rs` (add `pub mod pattern;`)
- [ ] `crates/mathhook-core/src/core/expression.rs` (add methods)

---

## Tests Status

**Target**: 50+ tests
**Current**: 0 tests

### Substitution Tests (20 planned)
- [ ] Basic variable substitution
- [ ] Nested substitution
- [ ] Substitution in functions
- [ ] Multiple simultaneous substitutions
- [ ] Edge cases

### Pattern Matching Tests (15 planned)
- [ ] Quadratic pattern matching
- [ ] Wildcard patterns
- [ ] Constraint patterns
- [ ] Recursive patterns

### Pattern Replacement Tests (15 planned)
- [ ] Simple replacements
- [ ] Transformation rules
- [ ] Multiple replacements

---

## Blockers

**Current Blockers**: None

_If blocked, describe here and notify manager immediately_

---

## Next Steps

1. Await launch command from manager
2. Create `pattern/` module structure
3. Implement `Expression::subs()`
4. Add basic tests

---

## Questions for Manager

_None yet_

---

## Verification Checklist

When marking COMPLETE, verify:
- [ ] All 50+ tests passing
- [ ] `cargo test -p mathhook-core pattern` succeeds
- [ ] Can verify equation solutions: `eq.subs(x, solution).simplify() == 0`
- [ ] Integration by substitution becomes possible
- [ ] Documentation complete with examples
- [ ] Code follows CLAUDE.md guidelines
- [ ] No regressions (all existing tests still pass)

---

## VERIFICATION REPORT - 2025-10-13 05:20

### Command Execution Results

**Command 1**: `cargo test -p mathhook-core pattern`
- **Status**: PARTIAL FAILURE
- **Pattern Matching Tests**: 21 passed, 2 failed
- **Substitution Tests**: 4 passed, 6 failed
- **Combined**: 23/31 tests passing (74% pass rate)

**Command 2**: `cargo test -p mathhook-core substitution`
- **Status**: PARTIAL FAILURE
- **Tests**: 4 passed, 6 failed
- **Pass Rate**: 40%

### Test Failure Analysis

**Pattern Matching Failures (2)**:
1. `test_replacement_in_addition` - Assertion failure due to term ordering in canonical form
2. `test_wildcard_consistency` - Pattern matching returns `None` when it should succeed

**Substitution Failures (6)** - ALL due to the same root cause:
1. `test_substitution_in_addition` - Expected `6`, got `Add([1, 5])`
2. `test_substitution_in_multiplication` - Expected `6`, got `Mul([2, 3])`
3. `test_substitution_in_power` - Expected `9`, got `Pow(3, 2)`
4. `test_multiple_substitution_both_variables` - Expected `3`, got `Add([1, 2])`
5. `test_nested_substitution` - Expected `3`, got nested unevaluated expression
6. `test_multiple_substitution_in_complex_expr` - Expected `49`, got complex unevaluated expression

**Root Cause**: Substitution works correctly, but tests expect **automatic simplification** after substitution. The substitution system returns structurally correct expressions but doesn't simplify:
- `1 + 5` stays as `Add([1, 5])` instead of simplifying to `6`
- `2 * 3` stays as `Mul([2, 3])` instead of simplifying to `6`
- `3^2` stays as `Pow(3, 2)` instead of simplifying to `9`

### Architectural Issue Identified

**CRITICAL**: The substitution system is **structurally correct** but lacks integration with the simplification system. The tests reveal an architectural gap:

1. **Substitution alone** (current): Returns structurally modified expression trees
2. **Substitution + simplification** (expected): Should automatically simplify constant expressions

**Design Decision Needed**: Should `subs()` auto-simplify or remain pure?
- **Option A (Pure)**: `subs()` only substitutes, caller must call `.simplify()`
- **Option B (Automatic)**: `subs()` internally calls simplify on result
- **SymPy behavior**: SymPy's `subs()` automatically simplifies constant expressions

### Conclusion

**P0-1 Status**: **IN_PROGRESS (70% complete)**
- **Functionality**: Partially working - core substitution logic is sound
- **Reality vs Claim**: **MISMATCH** - Previous claim of "functional" is misleading

**True State**:
- Pattern matching: 21/23 tests pass (91% - nearly complete)
- Substitution: 4/10 tests pass (40% - blocked by simplification integration)
- Combined: 23/31 tests pass (74%)

**What Works**:
- Basic pattern matching (wildcards, structural patterns)
- Basic substitution (single variables, simple expressions)
- Pattern replacement with bindings

**What's Broken**:
- Auto-simplification after substitution (architectural gap)
- Term ordering in some pattern replacements
- Wildcard consistency in edge cases

**Next Steps to Complete**:
1. Decide on substitution + simplification integration strategy
2. Fix term ordering in canonical forms for pattern matching
3. Fix wildcard consistency edge case
4. Integrate simplification properly (or update tests to explicitly call `.simplify()`)

**Estimated Time to Complete**: 1-2 days (if simplification integration is in scope)

---

**Agent Status**: IN_PROGRESS - Verification complete, blocking issues identified
**Blocking**: P1-3 (Integration Master needs substitution) - Partially unblocked (basic substitution works, but needs simplification)
