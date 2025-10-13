# Agent P0-1: Pattern Matching Architect

**Task**: P0-1 - Implement Pattern Matching & Substitution System
**Status**: NOT_STARTED
**Progress**: 0%
**Priority**: P0 (CRITICAL BLOCKER)
**Estimated Duration**: 2-3 weeks
**Started**: -
**Last Update**: -

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

**Agent Status**: STANDBY - Ready to launch
**Blocking**: P1-3 (Integration Master needs substitution)
