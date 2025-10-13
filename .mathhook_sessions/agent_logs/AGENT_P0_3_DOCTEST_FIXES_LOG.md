# Agent P0-3: Doctest Healer

**Task**: P0-3 - Fix All 103 Failing Doctests
**Status**: NOT_STARTED
**Progress**: 0%
**Priority**: P0 (CRITICAL - USER TRUST)
**Estimated Duration**: 1 week
**Started**: -
**Last Update**: -

---

## Mission Briefing

Fix 103 failing doctests across 20+ files. Currently, 39% of documentation examples don't work, which destroys user trust in the documentation.

**Current Problem**:
- 103 failing doctests out of 266 total
- Common issues: missing imports, wrong method signatures, outdated API usage
- Users cannot trust documentation examples

**CLAUDE.md Requirement**: "Every public function MUST include working examples in doctests"

**Reference Material**:
- Task details: `.mathhook_sessions/0.1_RELEASE_READINESS_AI_AGENT.md` (TASK P0-3)
- Documentation Standards: `CLAUDE.md` (lines 577-671)

---

## Current Objective

Waiting for launch command...

---

## Implementation Plan

### Phase 1: Categorize Failures (Day 1)
- [ ] Run `cargo test --doc -p mathhook-core 2>&1 | grep "FAILED" > doctest_failures.txt`
- [ ] Analyze failure patterns
- [ ] Group by failure type (missing imports, wrong API, non-existent methods)
- [ ] Create systematic fix plan

### Phase 2: Fix Missing Imports (Day 2-3)
- [ ] Fix all doctests missing `use mathhook_core::prelude::*;`
- [ ] Fix doctests missing trait imports (Derivative, etc.)
- [ ] Verify each fix compiles
- [ ] Target: ~40 fixes

### Phase 3: Fix Wrong Method Signatures (Day 3-4)
- [ ] Update old API usage (e.g., `derivative(&x, 1)` → `derivative(x)`)
- [ ] Fix LaTeX formatter calls (returns Result now)
- [ ] Fix higher-order derivatives (use `nth_derivative()`)
- [ ] Target: ~35 fixes

### Phase 4: Fix Non-Existent Methods (Day 4-5)
- [ ] Mark with `no_run` if method truly doesn't exist yet
- [ ] Document as "planned for v0.2"
- [ ] Fix if method exists but was renamed
- [ ] Target: ~28 fixes

### Phase 5: Validation (Day 5)
- [ ] Run full doctest suite: `cargo test --doc -p mathhook-core`
- [ ] Verify 0 failures
- [ ] Ensure all examples demonstrate working API
- [ ] Update documentation if needed

---

## Completed Work

_Nothing yet - waiting for launch_

---

## Affected Files (46 failures across)

### Calculus Files
- [ ] `src/calculus/derivatives/basic.rs`
- [ ] `src/calculus/derivatives/chain_rule.rs`
- [ ] `src/calculus/derivatives/checker.rs`
- [ ] `src/calculus/derivatives/higher_order.rs`
- [ ] `src/calculus/derivatives/power_rule.rs`
- [ ] `src/calculus/derivatives/partial/*.rs` (multiple files)
- [ ] `src/calculus/integrals/*.rs` (multiple files)
- [ ] `src/calculus/limits.rs`
- [ ] `src/calculus/residues.rs`

### Algebra Files
- [ ] `src/algebra/complex.rs`
- [ ] Other algebra files with doctests

### Formatter Files
- [ ] Files using `to_latex()` method

---

## Common Failure Patterns

### Pattern 1: Missing Imports (~40 cases)
```rust
// BROKEN:
/// ```
/// let x = symbol!(x);
/// let derivative = expr.derivative(x);
/// ```

// FIX:
/// ```
/// use mathhook_core::prelude::*;
/// use mathhook_core::Derivative;
///
/// let x = symbol!(x);
/// let derivative = expr.derivative(x);
/// ```
```

### Pattern 2: Wrong Method Signatures (~35 cases)
```rust
// BROKEN (old API):
/// ```
/// let derivative = expr.derivative(&x, 1);
/// ```

// FIX (current API):
/// ```
/// use mathhook_core::Derivative;
/// let derivative = expr.derivative(x);
/// // For higher order:
/// let second = expr.nth_derivative(x, 2);
/// ```
```

### Pattern 3: Methods That Don't Exist (~28 cases)
```rust
// BROKEN:
/// ```
/// let latex = expr.to_latex();
/// ```

// FIX (if method doesn't exist):
/// ```no_run
/// // This feature is planned for v0.2
/// let latex = expr.to_latex();
/// ```

// OR FIX (if method exists differently):
/// ```
/// use mathhook_core::formatter::LaTeXFormatter;
/// let latex = expr.to_latex(&Default::default())?;
/// ```
```

---

## Tests Status

**Current**: 103 failures (39% failure rate)
**Target**: 0 failures (100% success rate)

### Progress Tracking
- [ ] Missing imports fixed: 0 / ~40
- [ ] Wrong signatures fixed: 0 / ~35
- [ ] Non-existent methods handled: 0 / ~28
- [ ] Total fixed: 0 / 103

---

## Blockers

**Current Blockers**: None

_If blocked by unclear API or missing functionality, document here_

---

## Next Steps

1. Await launch command
2. Run `cargo test --doc` and capture all failures
3. Create systematic categorization
4. Begin fixing in batches by pattern type

---

## Questions for Manager

_Will add questions as they arise during implementation_

---

## Verification Checklist

When marking COMPLETE, verify:
- [ ] `cargo test --doc -p mathhook-core` shows 0 failures
- [ ] All code examples compile successfully
- [ ] All code examples run successfully
- [ ] Examples demonstrate actual working API (not fake/outdated)
- [ ] Documentation is trustworthy for users
- [ ] `no_run` is only used for genuinely unimplemented features
- [ ] Code follows CLAUDE.md documentation standards
- [ ] No regressions in non-doc tests

---

**Agent Status**: STANDBY - Ready to launch
**Impact**: User trust in documentation
