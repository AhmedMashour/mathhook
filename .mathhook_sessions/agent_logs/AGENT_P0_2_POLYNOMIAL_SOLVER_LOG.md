# Agent P0-2: Polynomial Solver Fixer

**Task**: P0-2 - Fix Polynomial Solver Fake Roots
**Status**: NOT_STARTED
**Progress**: 0%
**Priority**: P0 (CRITICAL - MATHEMATICAL CORRECTNESS)
**Estimated Duration**: 1 day
**Started**: -
**Last Update**: -

---

## Mission Briefing

Fix the polynomial solver that currently returns FAKE PLACEHOLDER ROOTS. This is a critical mathematical correctness violation.

**Current Problem**: 
- Lines 155-162: Cubic solver pads with fake `i` when can't find all 3 roots
- Lines 241-248: Quartic solver does the same
- Lines 309-311: Helper returns hardcoded fake roots

**CLAUDE.md Violation**: "Mathematical Correctness First: Every mathematical operation must be correct in ALL cases. No exceptions."

**Reference Material**:
- Task details: `.mathhook_sessions/0.1_RELEASE_READINESS_AI_AGENT.md` (TASK P0-2)
- File: `crates/mathhook-core/src/algebra/solvers/polynomial.rs`

---

## Current Objective

Waiting for launch command...

---

## Implementation Plan

### Step 1: Add Partial Result Variant (30 min)
- [ ] Add `Partial(Vec<Expression>)` to `SolverResult` enum
- [ ] Document this variant

### Step 2: Fix Cubic Solver (1 hour)
- [ ] Remove lines 155-162 (fake root padding loop)
- [ ] Return `SolverResult::Partial(found_roots)` instead
- [ ] Update tests to expect Partial results

### Step 3: Fix Quartic Solver (1 hour)
- [ ] Remove lines 241-248 (fake root padding loop)
- [ ] Return `SolverResult::Partial(found_roots)` instead
- [ ] Update tests

### Step 4: Fix/Remove Helper Function (30 min)
- [ ] Fix `find_remaining_cubic_roots()` or remove it entirely
- [ ] Ensure no other code depends on it

### Step 5: Validation (2 hours)
- [ ] Add tests that verify returned roots actually solve the equation
- [ ] Add tests for partial solutions
- [ ] Document limitations in API docs

---

## Completed Work

_Nothing yet - waiting for launch_

---

## Files to Modify

- [ ] `crates/mathhook-core/src/algebra/solvers/polynomial.rs` (main fix)
- [ ] `crates/mathhook-core/src/algebra/solvers.rs` (add Partial variant)
- [ ] Tests in polynomial.rs

---

## Tests Status

**Current**: Likely passing with fake roots (BAD)
**Target**: All tests verify mathematical correctness

### Tests to Add/Update
- [ ] Verify cubic partial solutions are correct
- [ ] Verify quartic partial solutions are correct
- [ ] Test that returned roots actually solve the equation
- [ ] Document Partial result variant behavior

---

## Blockers

**Current Blockers**: None

---

## Next Steps

1. Await launch
2. Add `Partial` variant to `SolverResult`
3. Remove fake root generation
4. Test correctness

---

## Verification Checklist

When marking COMPLETE, verify:
- [ ] NO fake roots returned anywhere
- [ ] All returned roots actually solve the equation (test this!)
- [ ] `Partial` variant properly documented
- [ ] Tests validate correctness of partial solutions
- [ ] Code follows CLAUDE.md guidelines
- [ ] No regressions in existing linear/quadratic solvers

---

**Agent Status**: STANDBY - Ready to launch
