# Plan 8: Code Quality, Mathematical Correctness, and CLAUDE.md Compliance - Orchestrator Bootstrap Command

**Purpose**: Address ALL issues found in comprehensive deep analysis (clippy, tests, mathematical correctness, CLAUDE.md compliance)
**Date Created**: 2025-01-13
**Baseline**: 14 failing tests, 6 critical issues, 42 high priority issues, ~300 medium priority issues
**Target**: Zero critical issues, all tests passing, 100% CLAUDE.md compliance, 9.5+/10 quality

**Scope**: 4-phase systematic remediation:
- **Phase 1**: Critical Mathematical Correctness (1 week)
- **Phase 2**: High Priority Quality & Safety (1 week)
- **Phase 3**: Medium Priority Cleanup (2 weeks, incremental)
- **Phase 4**: Long-term Architectural Improvements (ongoing)

---

## Current State Summary (From Deep Analysis)

**Critical Issues (6 - MUST FIX IMMEDIATELY)**:
1. C1: Grevlex Monomial Ordering Bug - Mathematical correctness violation
2. C2: Newton-Raphson Zero Derivative - Infinite loop risk
3. C3: Bisection Method Validation - Missing precondition checks
4. C4: 4 ODE Tests for Unimplemented Features - CLAUDE.md violation
5. C5: Symbol::new() Used 287 Times - MASSIVE Priority 1 violation
6. C6: Expression Size Unchecked - Silent performance regression risk

**High Priority Issues (42)**:
- H1-H77: 77 unused imports (auto-fixable)
- H78: Ambiguous glob re-exports
- H79-H82: Numerical algorithm precision issues (4 root-finding tests failing)
- H83: 30+ unwrap/expect in library code (panic policy violation)

**Medium Priority Issues (~300)**:
- M1-M290: 290 non-snake-case warnings (caused by Symbol::new)
- M291-M4838: 4,838 inline comments (many should be /// or removed)
- M4839: Matrix canonical form TODO

**Low Priority (Technical Debt)**:
- L1: 3,193 excessive clone operations
- L2: Missing systematic benchmarks

**Test Status**:
- Total: 676/677 minimum maintained
- Failing: 14 tests (categorized in analysis)
- Root causes identified for all failures

**Build Status**: PASSING (with 367 warnings)
**CLAUDE.md Compliance**: 85% (B+ grade) - needs 100%

---

## Copy-Paste This Entire Block Into New Claude Code Session

```
You are the Orchestrator for Plan 8: Code Quality, Mathematical Correctness, and CLAUDE.md Compliance.

CRITICAL FIRST STEP - Read these files in order and line by line:

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - This is the SINGLE SOURCE OF TRUTH for all development rules
   - Contains architectural constraints, coding standards, and non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation
   - Pay special attention to: Expression size (32-byte target), mathematical correctness, no emojis, file size limits (500 lines), Symbol macro usage

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md
   - Complete orchestration methodology proven across Educational Waves 1-5
   - Contains wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_8_CODE_QUALITY_ORCHESTRATOR_COMMAND.md
   - This file - complete context for Plan 8 and skip copy-paste block and goal statement block

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_8_PHASED_ROADMAP.md
   - Detailed 4-phase breakdown with all wave objectives
   - Current status for each issue category
   - Success criteria and deliverables for each phase
   - Timeline and risk mitigation strategies

MANDATORY ORCHESTRATION RULES (From Proven Methodology):

1. You Are Always The Orchestrator
   - You plan, launch agents, verify, and make decisions
   - Agents execute specific tasks; you maintain control and continuity
   - NEVER delegate orchestration responsibilities to agents

2. Sequential Waves, Parallel Agents
   - Work proceeds in waves: Wave N → verify → Wave N+1 → verify
   - Within a wave, launch multiple agents in parallel when possible
   - NEVER skip verification between waves

3. Mandatory Verification Protocol
   - Create verification script BEFORE launching agents (bash script with 8-10 categories)
   - Run verification script AFTER agents complete
   - Create comprehensive verification report
   - NEVER declare work complete without running verification script

4. Strict CLAUDE.md Enforcement
   - All agent prompts MUST include CLAUDE.md requirements explicitly
   - Enforce: max 500 lines/file, NO EMOJIS, proper docs, no placeholders, build passes
   - Zero tolerance for violations
   - CLAUDE.md overrides ALL other guidance

5. Maintain Momentum
   - Don't stop between waves unless verification fails
   - Use TodoWrite to track progress through all phases
   - Keep user informed of progress without asking unnecessary questions

MATHEMATICAL CORRECTNESS - HIGHEST PRIORITY:

From CLAUDE.md: "Mathematical Correctness First: Every mathematical operation must be correct in ALL cases. No exceptions."

From Deep Analysis: "The mathhook-core codebase is fundamentally sound but suffers from:
1. Quality debt: Extensive CLAUDE.md violations (comments, Symbol usage)
2. Mathematical bugs: Root-finding and monomial ordering
3. Incomplete features: ODE solver with tests but no implementation
4. Testing gap: 14 failing tests expose numerical instability"

CONFIRMATION REQUIRED:

After reading all files line by line, respond with:

1. "I have read and understood CLAUDE.md, ORCHESTRATION_METHODOLOGY.md, PLAN_8_CODE_QUALITY_ORCHESTRATOR_COMMAND.md, and PLAN_8_PHASED_ROADMAP.md"
2. "I understand Plan 8's scope: Fix 6 critical issues, 42 high priority issues, ~300 medium priority issues, address technical debt"
3. "I understand the 4-phase structure: Critical Correctness, High Priority Quality, Medium Priority Cleanup, Long-term Architecture"
4. Summarize the 5 mandatory orchestration rules in your own words
5. List the 5 phases of a standard wave
6. Say: "I am ready to orchestrate. Awaiting goal confirmation."

Then WAIT for the user to provide the goal confirmation and any modifications.

DO NOT proceed with any work until you have:
- Read all required files line by line
- Confirmed understanding
- Received goal confirmation from the user
```

---

## Goal Statement (Provide After Orchestrator Confirms)

```
The goal is: Complete Plan 8 Code Quality and Mathematical Correctness Remediation

Context: Comprehensive deep analysis identified 6 critical issues, 42 high priority issues, and ~300 medium priority issues across mathematical correctness, code quality, and CLAUDE.md compliance. Current baseline: 14 failing tests, 85% CLAUDE.md compliance (B+ grade), 367 warnings. Target: Zero critical issues, all tests passing, 100% CLAUDE.md compliance, 9.5+/10 quality.

Analysis Findings Summary:
- 6 CRITICAL mathematical/architectural issues requiring immediate attention
- 42 HIGH PRIORITY quality and safety issues
- ~300 MEDIUM PRIORITY style and cleanup issues
- 14 failing tests with root causes identified
- 287 uses of deprecated Symbol::new() (should be symbol! macro)
- 77 unused imports (auto-fixable)
- 30+ unwrap/expect violations in library code
- 4,838 inline comments (many should be /// documentation)

Phase Structure (4 phases):

PHASE 1: CRITICAL MATHEMATICAL CORRECTNESS (Week 1 - IMMEDIATE)
Estimated: 2-3 days for critical fixes

Wave 1.1: Immediate Safety Fixes (4 hours)
Priority: CRITICAL (Day 1)
Agent: rust-engineer

Objectives:
1. Mark 4 ODE tests as #[ignore] with explanation (C4)
2. Remove 77 unused imports automatically (H1-H77)
3. Add Expression size compile-time assertion (C6)
4. Verify build still passes

Success Criteria:
- 4 ODE tests properly ignored with feature flag explanation
- Zero unused import warnings
- Expression size assertion added: const _: () = assert!(std::mem::size_of::<Expression>() == 32);
- Build passes with 290 fewer warnings
- Quality score >= 9/10

Deliverables:
- Verification script: /tmp/verify_wave_1_1_safety.sh
- Verification report: .mathhook_sessions/gtm/WAVE_1_1_VERIFICATION_REPORT.md
- Clean build output

Verification Categories (10):
1. Build Status (15 points)
2. Warning Reduction (15 points)
3. ODE Tests Properly Ignored (10 points)
4. Expression Size Assertion (10 points)
5. No New Issues Introduced (10 points)
6. CLAUDE.md Compliance (10 points)
7. Documentation Quality (10 points)
8. Code Organization (10 points)
9. Test Suite Integrity (5 points)
10. Git Hygiene (5 points)

Target: 90/100 points (9/10 quality score)

Wave 1.2: Root-Finding Mathematical Correctness (1 day)
Priority: CRITICAL (Day 2)
Agent: rust-engineer with SymPy validation

Objectives:
1. Fix C2: Newton-Raphson zero derivative handling
   - Add robust epsilon-based derivative magnitude check
   - Return MathError::NumericalError when |f'(x)| < ε
   - Add tests for zero derivative cases
   - Verify against SymPy behavior

2. Fix C3: Bisection method bracket validation
   - Add proper opposite-sign validation at endpoints
   - Return clear error when f(a)*f(b) >= 0
   - Add comprehensive bracket validation tests
   - Verify against SymPy behavior

3. Fix H79-H82: Numerical algorithm precision
   - Make tolerances adaptive based on problem scale
   - Add condition number estimates where applicable
   - Improve convergence criteria
   - Add better error messages

4. Validate ALL root-finding against SymPy
   - Extract 20+ test cases from SymPy
   - Compare outputs for correctness
   - Document any intentional differences

Success Criteria:
- Newton-Raphson properly detects zero derivatives (test_newton_zero_derivative_fails passes)
- Bisection validates brackets (3 bisection tests pass)
- All 4 root-finding tests pass
- 95%+ agreement with SymPy on 20+ test cases
- Quality score >= 9/10

Deliverables:
- Fixed: crates/mathhook-core/src/algebra/root_finding/newton_raphson.rs
- Fixed: crates/mathhook-core/src/algebra/root_finding/bisection.rs
- Verification script: /tmp/verify_wave_1_2_rootfinding.sh
- Verification report: .mathhook_sessions/gtm/WAVE_1_2_VERIFICATION_REPORT.md
- SymPy validation results: .mathhook_sessions/gtm/ROOTFINDING_SYMPY_VALIDATION.md

Verification Categories (10):
1. Compilation (10 points)
2. Root-Finding Tests (20 points - CRITICAL)
3. SymPy Validation (15 points)
4. Error Handling (10 points)
5. Numerical Stability (10 points)
6. CLAUDE.md Compliance (10 points)
7. Documentation (10 points)
8. Code Quality (10 points)
9. Test Coverage (3 points)
10. Mathematical Correctness (10 points - CRITICAL)

Target: 90/100 points (9/10 quality score)

Wave 1.3: Grevlex Monomial Ordering (1 day)
Priority: CRITICAL (Day 3)
Agent: rust-engineer with SymPy/Symbolica validation

Objectives:
1. Fix C1: Grevlex comparison returns wrong ordering
   - Study SymPy's grevlex implementation (~/Documents/work/math/sympy/sympy/polys/orderings.py)
   - Study Symbolica's implementation if available
   - Fix comparison logic in monomial_order.rs:332
   - Add comprehensive tests covering all comparison cases

2. Validate against SymPy
   - Extract 30+ monomial comparison test cases from SymPy
   - Verify all comparisons match SymPy exactly
   - Test edge cases (equal monomials, zero exponents, etc.)

3. Verify Gröbner basis still works
   - Run all Gröbner tests (should have 1 more passing: test_grevlex_ordering)
   - Verify systems.rs integration unchanged
   - Run Wave 3-INT verification again to ensure no regression

Success Criteria:
- test_grevlex_ordering passes (currently fails with Less instead of Greater)
- All Gröbner tests pass
- 100% agreement with SymPy on 30+ monomial comparisons
- No regression in systems.rs functionality
- Quality score >= 9/10

Deliverables:
- Fixed: crates/mathhook-core/src/algebra/groebner/monomial_order.rs
- Verification script: /tmp/verify_wave_1_3_grevlex.sh
- Verification report: .mathhook_sessions/gtm/WAVE_1_3_VERIFICATION_REPORT.md
- SymPy validation: .mathhook_sessions/gtm/GREVLEX_SYMPY_VALIDATION.md

Verification Categories (10):
1. Compilation (10 points)
2. Grevlex Tests (20 points - CRITICAL)
3. SymPy Validation (15 points)
4. Gröbner Integration (10 points)
5. Mathematical Correctness (15 points - CRITICAL)
6. CLAUDE.md Compliance (10 points)
7. Documentation (10 points)
8. Code Quality (5 points)
9. Test Coverage (3 points)
10. No Regressions (10 points)

Target: 90/100 points (9/10 quality score)

PHASE 1 Summary:
- Duration: 2-3 days
- Waves: 3 waves (safety, root-finding, grevlex)
- Fixes: All 6 critical issues
- Expected Test Improvement: 14 failing → 5-6 failing (root-finding + grevlex fixed)
- Quality: 9+/10 for all waves

---

PHASE 2: HIGH PRIORITY QUALITY & SAFETY (Week 2)
Estimated: 5-7 days for high priority issues

Wave 2.1: Symbol Constructor Migration - Part 1 (2 days)
Priority: HIGH (C5 partial remediation)
Agent: rust-engineer

Scope: Fix Symbol::new() in core modules (50% of 287 uses)
Target: algebra/, calculus/, parser/ modules

Objectives:
1. Automated migration where possible:
   ```bash
   # Search and replace patterns:
   Symbol::new("x") → symbol!(x)
   Symbol::scalar("x") → symbol!(x)
   Symbol::matrix("A") → symbol!(A; matrix)
   Symbol::operator("p") → symbol!(p; operator)
   Symbol::quaternion("i") → symbol!(i; quaternion)
   ```

2. Manual migration for complex cases:
   - Variables in loops (use Expression::integer(i))
   - Runtime symbol creation (use explicit API)
   - Test code bulk creation (use symbols![...] macro)

3. Fix ~145 non-snake-case warnings (half of 290)

4. Test thoroughly:
   - Ensure all affected tests still pass
   - Verify symbol types correct (scalar/matrix/operator/quaternion)
   - Check macro expansion is correct

Success Criteria:
- 50% reduction in Symbol::new() usage (287 → ~140)
- 145 fewer non-snake-case warnings
- All tests in affected modules pass
- Build passes with significantly fewer warnings
- Quality score >= 8/10

Deliverables:
- Migration script: /tmp/migrate_symbols_phase1.sh
- Verification script: /tmp/verify_wave_2_1_symbols.sh
- Verification report: .mathhook_sessions/gtm/WAVE_2_1_VERIFICATION_REPORT.md
- Migration log: .mathhook_sessions/gtm/SYMBOL_MIGRATION_LOG_PHASE1.md

Verification Categories (10):
1. Compilation (10 points)
2. Symbol Usage Reduction (20 points)
3. Warning Reduction (15 points)
4. Test Pass Rate (15 points)
5. CLAUDE.md Compliance (10 points)
6. Migration Correctness (10 points)
7. Code Quality (10 points)
8. Documentation (5 points)
9. No Regressions (3 points)
10. Symbol Types Correct (10 points)

Target: 80/100 points (8/10 quality score)

Wave 2.2: Symbol Constructor Migration - Part 2 (2 days)
Priority: HIGH (C5 complete remediation)
Agent: rust-engineer

Scope: Fix remaining Symbol::new() in functions/, educational/, ode/, matrix/
Target: Complete migration (287 → 0)

Objectives:
1. Complete automated migration
2. Fix all remaining non-snake-case warnings (290 → 0)
3. Update all test code to use macros
4. Verify all modules comply with Priority 1 MANDATORY rule
5. Add pre-commit check to prevent Symbol::new() usage

Success Criteria:
- Zero Symbol::new() usage remaining
- Zero non-snake-case warnings
- All tests pass
- Pre-commit hook prevents future violations
- Quality score >= 9/10

Deliverables:
- Migration complete: all 287 uses fixed
- Pre-commit hook: .git/hooks/pre-commit (checks for Symbol::new)
- Verification script: /tmp/verify_wave_2_2_symbols_complete.sh
- Verification report: .mathhook_sessions/gtm/WAVE_2_2_VERIFICATION_REPORT.md
- Final migration log: .mathhook_sessions/gtm/SYMBOL_MIGRATION_COMPLETE.md

Verification Categories (10):
1. Compilation (10 points)
2. Zero Symbol::new() Usage (25 points - CRITICAL)
3. Zero Non-Snake-Case Warnings (15 points)
4. Test Pass Rate (15 points)
5. Pre-commit Hook (10 points)
6. CLAUDE.md Compliance (10 points)
7. Code Quality (10 points)
8. Documentation (3 points)
9. No Regressions (10 points)
10. Symbol Types Correct (10 points)

Target: 90/100 points (9/10 quality score)

Wave 2.3: Panic-Free Library Code (2 days)
Priority: HIGH (H83)
Agent: rust-engineer

Objectives:
1. Identify all unwrap/expect in library code (30+ instances)
   - Exclude: tests, examples, documentation
   - Focus: ode/systems/linear.rs, educational/, ode/educational/wrapper.rs

2. Replace with proper Result returns:
   ```rust
   // Before:
   let value = some_operation().unwrap();

   // After:
   let value = some_operation()
       .map_err(|e| MathError::ComputationFailed {
           operation: "some_operation".to_string(),
           reason: e.to_string()
       })?;
   ```

3. Update function signatures to return Result
4. Propagate errors properly through call chain
5. Add comprehensive error types if needed

Success Criteria:
- Zero unwrap/expect in library code (tests OK)
- All affected functions return Result<T, MathError>
- Error messages are clear and actionable
- All tests pass with new error handling
- Quality score >= 8/10

Deliverables:
- Fixed: 30+ library functions
- Updated: MathError enum with new variants
- Verification script: /tmp/verify_wave_2_3_panic_free.sh
- Verification report: .mathhook_sessions/gtm/WAVE_2_3_VERIFICATION_REPORT.md
- Error handling guide: .mathhook_sessions/gtm/ERROR_HANDLING_PATTERNS.md

Verification Categories (10):
1. Compilation (10 points)
2. Zero Unwrap/Expect (25 points - CRITICAL)
3. Error Propagation (15 points)
4. Error Messages Quality (10 points)
5. Test Pass Rate (15 points)
6. CLAUDE.md Compliance (10 points)
7. API Backward Compatibility (5 points)
8. Documentation (5 points)
9. Code Quality (3 points)
10. Safety Improvement (10 points)

Target: 80/100 points (8/10 quality score)

Wave 2.4: Glob Ambiguity and Cleanup (1 day)
Priority: HIGH (H78)
Agent: rust-engineer

Objectives:
1. Fix H78: Ambiguous glob re-exports in lib.rs:30,34
   - Rename conflicting `types` module exports
   - Make re-exports explicit
   - Test all dependent code

2. Verify no import ambiguities remain
3. Clean up any other glob import issues
4. Update documentation for module organization

Success Criteria:
- Zero ambiguous glob imports
- Build passes without warnings about ambiguous imports
- All re-exports clear and explicit
- Quality score >= 8/10

Deliverables:
- Fixed: lib.rs exports
- Verification script: /tmp/verify_wave_2_4_globs.sh
- Verification report: .mathhook_sessions/gtm/WAVE_2_4_VERIFICATION_REPORT.md

Verification Categories (8):
1. Compilation (15 points)
2. Zero Ambiguities (25 points)
3. Export Clarity (15 points)
4. Test Pass Rate (15 points)
5. CLAUDE.md Compliance (10 points)
6. Documentation (10 points)
7. Code Quality (5 points)
8. No Regressions (10 points)

Target: 80/100 points (8/10 quality score)

PHASE 2 Summary:
- Duration: 5-7 days
- Waves: 4 waves (symbols part 1, symbols part 2, panic-free, globs)
- Fixes: C5 (287 Symbol::new), H83 (30+ unwrap/expect), H78 (glob ambiguity), H1-H77 (done in Phase 1)
- Expected Warning Reduction: 367 → ~70-80 warnings
- Quality: 8-9/10 for all waves

---

PHASE 3: MEDIUM PRIORITY CLEANUP (Weeks 3-4, Incremental)
Estimated: 10-15 days for medium priority issues (can be done incrementally)

Wave 3.1: Comment Migration - Public API Priority (3 days)
Priority: MEDIUM (M291-M4838 partial)
Agent: rust-engineer

Scope: Migrate comments in public API modules first
Target: algebra/, calculus/, parser/, functions/ public functions

Objectives:
1. Identify public functions with inline // comments
2. Convert to /// documentation comments:
   ```rust
   // Before:
   // This function solves quadratic equations
   pub fn solve_quadratic(...) { ... }

   // After:
   /// Solves quadratic equations using the quadratic formula
   ///
   /// # Arguments
   /// * `a` - Coefficient of x²
   /// * `b` - Coefficient of x
   /// * `c` - Constant term
   ///
   /// # Examples
   /// ```
   /// let solutions = solve_quadratic(1, -5, 6);
   /// ```
   pub fn solve_quadratic(...) { ... }
   ```

3. Remove obvious comments (state the obvious)
4. Keep mathematical formulas and non-obvious explanations
5. Target: Convert ~1000 comments (20% of 4838)

Success Criteria:
- 1000+ comments migrated or removed
- All public functions have /// documentation
- Doctests added where missing
- Quality score >= 7/10

Deliverables:
- Converted: ~1000 comments
- Verification script: /tmp/verify_wave_3_1_comments.sh
- Verification report: .mathhook_sessions/gtm/WAVE_3_1_VERIFICATION_REPORT.md
- Comment migration guide: .mathhook_sessions/gtm/COMMENT_MIGRATION_GUIDE.md

Verification Categories (10):
1. Compilation (10 points)
2. Doc Comment Coverage (20 points)
3. Doctest Quality (10 points)
4. Comment Reduction (15 points)
5. Documentation Clarity (10 points)
6. CLAUDE.md Compliance (10 points)
7. Code Quality (10 points)
8. Test Pass Rate (10 points)
9. No Regressions (3 points)
10. API Documentation (10 points)

Target: 70/100 points (7/10 quality score)

Wave 3.2: CI Quality Gates (1 day)
Priority: MEDIUM (architectural improvement)
Agent: rust-engineer

Objectives:
1. Add GitHub Actions / CI checks:
   ```yaml
   - Clippy strict mode: cargo clippy --all-targets -- -D warnings
   - Expression size check: cargo test expression_size_constraint
   - Unused imports: cargo fix --allow-dirty && git diff --exit-code
   - Format check: cargo fmt -- --check
   - Test coverage: >= 85% threshold
   ```

2. Add pre-commit hooks:
   ```bash
   #!/bin/bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   cargo test --quiet
   # Check for Symbol::new() usage
   ```

3. Document CI setup in CONTRIBUTING.md

Success Criteria:
- CI pipeline configured
- Pre-commit hooks working
- All checks pass on current codebase
- Documentation complete
- Quality score >= 8/10

Deliverables:
- .github/workflows/quality.yml
- .git/hooks/pre-commit
- Updated CONTRIBUTING.md
- Verification script: /tmp/verify_wave_3_2_ci.sh
- Verification report: .mathhook_sessions/gtm/WAVE_3_2_VERIFICATION_REPORT.md

Verification Categories (8):
1. CI Configuration (20 points)
2. Pre-commit Hooks (15 points)
3. Documentation (10 points)
4. All Checks Pass (25 points)
5. CLAUDE.md Compliance (10 points)
6. Code Quality (10 points)
7. Integration (5 points)
8. Maintainability (10 points)

Target: 80/100 points (8/10 quality score)

Wave 3.3: Matrix Canonical Form TODO (2 days)
Priority: MEDIUM (M4839)
Agent: rust-engineer

Objectives:
1. Fix TODO in core/expression/matrix_methods.rs:
   - "TODO: Fix canonical form to respect noncommutative Function expressions"

2. Implement noncommutative-aware canonical form:
   - Respect operator ordering (don't sort noncommutative symbols)
   - Preserve matrix multiplication order
   - Handle quantum operator commutators correctly

3. Add comprehensive tests:
   - Test with matrix symbols
   - Test with operator symbols
   - Test with mixed scalar/noncommutative
   - Verify simplification preserves order

Success Criteria:
- TODO resolved with full implementation
- All matrix/operator tests pass
- Canonical form respects noncommutativity
- Quality score >= 8/10

Deliverables:
- Fixed: core/expression/matrix_methods.rs
- Verification script: /tmp/verify_wave_3_3_canonical.sh
- Verification report: .mathhook_sessions/gtm/WAVE_3_3_VERIFICATION_REPORT.md
- Noncommutative canonical form tests

Verification Categories (10):
1. Compilation (10 points)
2. Implementation Complete (20 points)
3. Test Pass Rate (15 points)
4. Noncommutative Correctness (15 points)
5. Mathematical Correctness (10 points)
6. CLAUDE.md Compliance (10 points)
7. Documentation (10 points)
8. Code Quality (5 points)
9. Test Coverage (3 points)
10. No Regressions (10 points)

Target: 80/100 points (8/10 quality score)

PHASE 3 Summary:
- Duration: 10-15 days (can be incremental)
- Waves: 3 waves (comments, CI, canonical form)
- Fixes: ~1000 comments migrated, CI gates, matrix TODO
- Can be done in parallel with Phase 4
- Quality: 7-8/10 for waves

---

PHASE 4: LONG-TERM ARCHITECTURAL IMPROVEMENTS (Ongoing)
Estimated: 4+ weeks (low priority, can defer)

Wave 4.1: Performance Benchmarking Baseline (1 week)
Priority: LOW (prerequisite for L1 optimization)
Agent: rust-engineer

Objectives:
1. Add Criterion benchmarks for hot paths:
   - Expression creation
   - Simplification
   - Derivative computation
   - Matrix operations
   - Solver methods

2. Baseline current performance
3. Compare against SymPy (should be 10-100x faster)
4. Compare against Symbolica (should be within 2x)
5. Identify actual bottlenecks (not assumptions)

Success Criteria:
- Comprehensive benchmark suite
- Baseline performance documented
- Comparison with SymPy/Symbolica
- Bottlenecks identified
- Quality score >= 7/10

Deliverables:
- Benchmarks: crates/mathhook-benchmarks/benches/
- Report: .mathhook_sessions/gtm/PERFORMANCE_BASELINE_REPORT.md
- Verification script: /tmp/verify_wave_4_1_benchmarks.sh
- Verification report: .mathhook_sessions/gtm/WAVE_4_1_VERIFICATION_REPORT.md

Wave 4.2: Clone Optimization (2-3 weeks)
Priority: LOW (L1 - architectural)
Agent: rust-engineer

Objectives:
1. Profile clone hotspots using benchmarks from Wave 4.1
2. Introduce lifetimes where possible
3. Use Cow<> for copy-on-write semantics
4. Consider arena allocation for bulk operations
5. Measure impact of each optimization

Success Criteria:
- Measurable clone reduction in hot paths
- No performance regressions
- All tests pass
- Quality score >= 7/10

Deliverables:
- Optimized modules
- Performance comparison report
- Verification script: /tmp/verify_wave_4_2_clones.sh
- Verification report: .mathhook_sessions/gtm/WAVE_4_2_VERIFICATION_REPORT.md

PHASE 4 Summary:
- Duration: 4+ weeks (ongoing)
- Waves: 2 waves (benchmarks, clone optimization)
- Fixes: L1 (clones), L2 (benchmarks)
- Can be deferred to post-release
- Quality: 7/10 for waves

---

## Success Criteria By Phase

**Phase 1 Success (Week 1)**:
- All 6 critical issues fixed
- 14 failing tests → 5-6 failing tests (8-9 tests fixed)
- Grevlex, Newton-Raphson, Bisection all pass
- Expression size assertion added
- ODE tests properly ignored
- Unused imports removed
- Quality: 9+/10 average

**Phase 2 Success (Week 2)**:
- Zero Symbol::new() usage (287 → 0)
- Zero non-snake-case warnings (290 → 0)
- Zero unwrap/expect in library code (30+ → 0)
- Glob ambiguities resolved
- Warnings: 367 → ~70-80
- Quality: 8-9/10 average

**Phase 3 Success (Weeks 3-4)**:
- 1000+ comments migrated to /// or removed
- CI quality gates operational
- Pre-commit hooks working
- Matrix canonical form TODO resolved
- Quality: 7-8/10 average

**Phase 4 Success (4+ weeks)**:
- Comprehensive benchmarks
- Performance baseline established
- Clone optimization if needed
- Quality: 7/10 average

**Overall Success (Full Plan 8)**:
- Zero critical issues
- All mathematical correctness bugs fixed
- All high priority quality issues resolved
- 1000+ medium priority issues addressed
- 100% CLAUDE.md compliance
- All tests passing (676/677 minimum maintained)
- 9.5+/10 overall quality score

---

## Timeline Overview

```
Week 1:    PHASE 1 - Critical Mathematical Correctness
           - Wave 1.1: Immediate Safety (4h)
           - Wave 1.2: Root-Finding Correctness (1 day)
           - Wave 1.3: Grevlex Ordering (1 day)
           Expected: 6 critical issues fixed, 8-9 tests passing

Week 2:    PHASE 2 - High Priority Quality & Safety
           - Wave 2.1: Symbol Migration Part 1 (2 days)
           - Wave 2.2: Symbol Migration Part 2 (2 days)
           - Wave 2.3: Panic-Free Library (2 days)
           - Wave 2.4: Glob Cleanup (1 day)
           Expected: 287 Symbol::new → 0, 367 warnings → ~70-80

Week 3-4:  PHASE 3 - Medium Priority Cleanup (Incremental)
           - Wave 3.1: Comment Migration (3 days)
           - Wave 3.2: CI Quality Gates (1 day)
           - Wave 3.3: Matrix Canonical Form (2 days)
           Expected: 1000+ comments migrated, CI operational

Week 5+:   PHASE 4 - Long-term Architecture (Ongoing/Deferred)
           - Wave 4.1: Performance Benchmarks (1 week)
           - Wave 4.2: Clone Optimization (2-3 weeks)
           Expected: Performance baseline, optimization if needed
```

**Total Duration**: 4-6 weeks to full completion (Phases 1-2 are 2 weeks, critical)

---

## Critical SymPy References

All mathematical correctness fixes must validate against SymPy.

**SymPy Location**: ~/Documents/work/math/sympy/

**Key Modules for Validation**:
1. Root-finding: ~/Documents/work/math/sympy/sympy/solvers/solvers.py
2. Gröbner: ~/Documents/work/math/sympy/sympy/polys/groebnertools.py
3. Monomial ordering: ~/Documents/work/math/sympy/sympy/polys/orderings.py
4. Numerical methods: ~/Documents/work/math/sympy/sympy/solvers/solvers.py

**Validation Strategy**:
- Extract 20-50 test cases per wave from SymPy
- Run SymPy to get expected outputs
- Compare MathHook outputs against SymPy
- Target: 95%+ agreement
- Document any intentional discrepancies
- Fix any mathematical correctness issues

---

## Verification Script Template (Standard 10 Categories)

```bash
#!/bin/bash
# Wave N.M Verification Script
# Target Quality Score: >= [TARGET]/10

PROJECT_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
cd "$PROJECT_ROOT"

TOTAL_SCORE=0
MAX_SCORE=100

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "========================================"
echo "WAVE [N.M]: [NAME] VERIFICATION"
echo "========================================"

# CATEGORY 1: COMPILATION (10-15 points)
echo "========================================"
echo "CATEGORY 1: COMPILATION"
echo "========================================"
cargo check -p mathhook-core 2>&1
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful${NC}"
    TOTAL_SCORE=$((TOTAL_SCORE + 10))
else
    echo -e "${RED}✗ Build failed${NC}"
fi

# CATEGORY 2: PRIMARY OBJECTIVE (20-25 points)
echo "========================================"
echo "CATEGORY 2: PRIMARY OBJECTIVE"
echo "========================================"
# [Wave-specific checks]

# CATEGORY 3: TEST PASS RATE (15 points)
echo "========================================"
echo "CATEGORY 3: TEST PASS RATE"
echo "========================================"
cargo test -p mathhook-core --quiet 2>&1
# [Count passing tests]

# CATEGORY 4-10: [Wave-specific categories]

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"
PERCENTAGE=$((TOTAL_SCORE * 100 / MAX_SCORE))
QUALITY_SCORE=$((TOTAL_SCORE * 10 / MAX_SCORE))

echo "Total Score: $TOTAL_SCORE / $MAX_SCORE ($PERCENTAGE%)"
echo "Quality Score: $QUALITY_SCORE / 10"

if [[ $QUALITY_SCORE -ge 8 ]]; then
    echo -e "${GREEN}✓ VERIFICATION PASSED: Quality >= 8${NC}"
    exit 0
elif [[ $QUALITY_SCORE -ge 6 ]]; then
    echo -e "${YELLOW}⚠ VERIFICATION PARTIAL: Quality 6-7${NC}"
    exit 1
else
    echo -e "${RED}✗ VERIFICATION FAILED: Quality < 6${NC}"
    exit 1
fi
```

---

## Standard Orchestration Protocol

For every wave:
1. You (orchestrator) create verification script BEFORE launching agent
2. Launch agent with comprehensive prompt (includes CLAUDE.md, success criteria, verification script reference)
3. Agent works autonomously
4. You run verification script when agent reports complete
5. You create comprehensive verification report
6. You decide: APPROVE (move to next wave) / REJECT (launch continuation agent) / CONTINUE (incomplete work)
7. Update TodoWrite with progress
8. Maintain momentum (don't stop unless verification fails)

---

## Files and Locations

**Verification Scripts**: /tmp/verify_wave_[N]_[M]_[name].sh
**Verification Reports**: .mathhook_sessions/gtm/WAVE_[N]_[M]_VERIFICATION_REPORT.md
**Migration Logs**: .mathhook_sessions/gtm/[TOPIC]_MIGRATION_LOG.md
**Validation Reports**: .mathhook_sessions/gtm/[TOPIC]_SYMPY_VALIDATION.md

**Core Files to Fix**:
- Root-finding: crates/mathhook-core/src/algebra/root_finding/newton_raphson.rs
- Root-finding: crates/mathhook-core/src/algebra/root_finding/bisection.rs
- Monomial ordering: crates/mathhook-core/src/algebra/groebner/monomial_order.rs
- Matrix canonical: crates/mathhook-core/src/core/expression/matrix_methods.rs
- ODE tests: crates/mathhook-core/src/ode/first_order/separable.rs
- Expression size: crates/mathhook-core/src/core/expression.rs

**Symbol Usage Locations** (287 total):
- Pervasive across all modules
- Test files heavily affected
- Migration requires systematic search-replace + manual fixes

---

## What This Achieves

**Phase 1 Outcome**: Zero critical issues, 8-9 tests fixed, mathematical correctness restored
**Phase 2 Outcome**: Zero CLAUDE.md Priority 1 violations, panic-free library, clean warnings
**Phase 3 Outcome**: 1000+ comments migrated, CI operational, matrix TODO resolved
**Phase 4 Outcome**: Performance baseline, optimization path clear

**Final State After Full Plan 8**:
- Zero critical mathematical bugs
- Zero CLAUDE.md Priority 1 violations
- Zero unwrap/expect in library code
- Zero failing tests (all 14 fixed)
- 100% CLAUDE.md compliance (A grade)
- 9.5+/10 quality score
- Production-ready, release-quality codebase
- CI/CD quality gates preventing regressions

---

**This orchestrator command is ready to use. Copy the bootstrap block and goal statement into a new Claude Code session.**

**Document Status**: Complete orchestrator command for Plan 8 code quality
**Baseline**: 6 critical, 42 high, ~300 medium issues
**Timeline**: 4-6 weeks (Phases 1-2 critical: 2 weeks)
**Quality Target**: 9.5+/10 overall
**Success**: Zero critical issues, all tests passing, 100% CLAUDE.md compliance
