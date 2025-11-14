# Plan 7: Core Mathematical Features Integration - Orchestrator Bootstrap Command

**Purpose**: Complete and verify all Plan 7 core mathematical features (ODEs, PDEs, Matrix Equations, Gröbner Basis, Special Functions, Numerical Methods)
**Date Created**: 2025-10-22
**Current Progress**: 60% complete (Waves 1, 3, 5 done; Waves 2, 4, 6 need work)

**Scope**: 4-phase verification and enhancement plan:
- **Phase 1**: Verification Waves (verify Waves 2, 3, 6 integration)
- **Phase 2**: Gap Filling (complete Wave 4 - Special Functions)
- **Phase 3**: Quality Assurance (SymPy validation, performance benchmarks, CLAUDE.md audit)
- **Phase 4**: Cleanup & Documentation (remove stubs, polish docs)

---

## Current State Summary

**What's Done** (60% complete):
- Wave 1 (ODE Solvers): 100% - 7 methods, 100+ tests, fully integrated
- Wave 3 (Gröbner Basis): 90% - Integration verified (7/10), stub returns Partial (mathematically honest)
- Wave 5 (PDE Solvers): 100% - 3 methods (separation of variables, characteristics), integrated

**What's Pending** (40% remaining):
- Wave 2 (Matrix Equations): 70-80% - Needs integration verification
- Wave 4 (Special Functions): 40-60% - BIGGEST GAP, needs implementation
- Wave 6 (Numerical Methods): 90-100% - Needs integration verification

**Integration Status**:
- All completed waves integrate with SmartEquationSolver
- No critical blocking issues
- 13 failing tests (need categorization)

**Quality Baseline**:
- Build passing: YES
- Tests: 676/677 minimum maintained
- CLAUDE.md compliance: 85% (B+ grade)
- Wave 3-INT verified: 7/10 quality score

---

## Copy-Paste This Entire Block Into New Claude Code Session

```
You are the Orchestrator for Plan 7: Core Mathematical Features Integration.

CRITICAL FIRST STEP - Read these files in order and line by line:

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - This is the SINGLE SOURCE OF TRUTH for all development rules
   - Contains architectural constraints, coding standards, and non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation
   - Pay special attention to: Expression size (32-byte target), mathematical correctness, no emojis, file size limits (500 lines)

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md
   - Complete orchestration methodology proven across Educational Waves 1-5
   - Contains wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_7_ORCHESTRATOR_COMMAND.md
   - This file - complete context for Plan 7

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_7_PHASED_ROADMAP.md
   - Detailed 4-phase breakdown with all wave objectives
   - Current status for each wave (complete, partial, pending)
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

CONFIRMATION REQUIRED:

After reading all files line by line, respond with:

1. "I have read and understood CLAUDE.md, ORCHESTRATION_METHODOLOGY.md, PLAN_7_ORCHESTRATOR_COMMAND.md, and PLAN_7_PHASED_ROADMAP.md"
2. "I understand Plan 7's current state: Waves 1/3/5 done, Waves 2/4/6 pending"
3. "I understand the 4-phase structure: Verification Waves, Gap Filling, Quality Assurance, Cleanup & Documentation"
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
The goal is: Complete Plan 7 Integration and Quality Assurance

Context: Plan 7 has 6 waves of core mathematical features. Three waves are complete (ODE, Gröbner, PDE), three need work (Matrix Equations verification, Special Functions implementation, Numerical Methods verification). Current progress is 60% with quality baseline of 7/10. We need to reach 100% completion with 9+/10 quality.

Current Status (Updated 2025-10-22):
- Wave 1 (ODE): 100% complete, integrated, 100+ tests
- Wave 2 (Matrix Equations): 70-80% complete, needs integration verification
- Wave 3 (Gröbner Basis): 90% complete, verified at 7/10 quality (documentation gap deferred to Phase 3)
- Wave 4 (Special Functions): 40-60% complete, BIGGEST GAP, needs implementation
- Wave 5 (PDE): 100% complete, integrated
- Wave 6 (Numerical): 90-100% complete, needs integration verification

Phase Structure (4 phases, not 6 waves):

PHASE 1: Verification Waves (Current - Weeks 1-2)
- Wave 2-INT: Matrix Equation Solver Integration Verification (3-4 hours)
- Wave 3-INT: Gröbner Basis Integration Verification [COMPLETE 7/10]
- Wave 6-INT: Numerical Methods Integration Verification (2-3 hours)
- Goal: Verify all implemented waves integrate with SmartEquationSolver

PHASE 2: Gap Filling (Weeks 3-4)
- Wave 4-IMPL: Special Functions Implementation (1-2 weeks)
- Wave 4-INT: Special Functions Integration Verification (3-4 hours)
- Wave 2-COMPLETE: Matrix Equation Gaps (conditional, 2-3 days)
- Goal: Bring all waves to 90%+ completion

PHASE 3: Quality Assurance (Week 5)
- QA-1: SymPy Validation Suite (2-3 days)
- QA-2: Performance Benchmarking (2-3 days)
- QA-3: CLAUDE.md Full Compliance Audit (1 day)
- QA-4 (DOC-1): Systems.rs Documentation Improvement (2-4 hours)
- Goal: 95%+ SymPy validation, performance targets met, 100% CLAUDE.md compliance

PHASE 4: Cleanup & Documentation (Week 6)
- WAVE-CLEANUP: Stub Removal & Implementation Completion (1-2 weeks)
- DOC-FINAL: Plan 7 Documentation & Examples (2-3 days)
- Goal: Zero critical stubs, production-ready quality, 100% completion

PHASE 1 Details (Current Focus):

Wave 2-INT: Matrix Equation Solver Integration Verification (3-4 hours)
Priority: MEDIUM (next after Wave 3-INT)

Objectives:
1. Verify MatrixEquationSolver integration with SmartEquationSolver
2. Check left/right division handling (Wave 10 noncommutative work)
3. Run matrix equation benchmarks
4. Validate against SymPy (matrix equations)
5. Fix any Wave 2-related failing tests

Success Criteria:
- All matrix equation tests pass
- Integration verified (follows Wave 1 & 5 pattern)
- CLAUDE.md compliant
- Quality score >= 8/10

Deliverables:
- Verification script: /tmp/verify_wave_2_int.sh (8 categories)
- Verification report: .mathhook_sessions/gtm/WAVE_2_INT_VERIFICATION_REPORT.md
- Matrix equation benchmarks baseline
- Quality score report

Test Approach:
- Unit tests: Test MatrixEquationSolver directly
- Integration tests: Test through SmartEquationSolver (end-to-end)
- SymPy validation: 20+ matrix equations
- Regression tests: Ensure existing tests still pass

Verification Categories:
1. Compilation (10 points)
2. Tests (15 points)
3. Integration Tests (10 points)
4. CLAUDE.md Compliance (10 points)
5. API Integration (10 points)
6. Documentation (10 points)
7. Benchmarks (5 points)
8. SymPy Validation (10 points)

Target: 64/80 points (8/10 quality score)

Wave 3-INT: Gröbner Basis Integration Verification [COMPLETE 7/10]
Status: COMPLETE (verified 2025-10-22)
Quality Score: 7.0/10 (73%)

Results:
- Compilation: 10/10
- Tests: 15/15 (17 tests passing)
- Integration: 10/10
- API Integration: 10/10
- Benchmarks: 5/5
- Documentation: 4/10 (gap - deferred to QA-4)
- CLAUDE.md: 5/10 (file size violation - deferred to QA-3)
- SymPy: 0/10 (deferred to QA-1)

Gaps Identified:
- Documentation: Only 1 doc comment in systems.rs (deferred to Phase 3: QA-4)
- File size: 773 lines > 500 limit (deferred to Phase 3: QA-3)
- SymPy validation: Not implemented (deferred to Phase 3: QA-1)

Report: .mathhook_sessions/gtm/WAVE_3_INT_VERIFICATION_REPORT.md

Wave 6-INT: Numerical Methods Integration Verification (2-3 hours)
Priority: MEDIUM

Objectives:
1. Verify numerical solvers integration (Newton-Raphson, bisection, secant)
2. Check convergence and accuracy
3. Run numerical benchmarks
4. Validate against SymPy/SciPy
5. Fix any Wave 6-related failing tests

Success Criteria:
- All numerical tests pass
- Integration verified
- CLAUDE.md compliant
- Quality score >= 8/10

Deliverables:
- Verification script: /tmp/verify_wave_6_int.sh
- Verification report: .mathhook_sessions/gtm/WAVE_6_INT_VERIFICATION_REPORT.md
- Numerical methods benchmarks baseline

PHASE 2 Details:

Wave 4-IMPL: Special Functions Implementation (1-2 weeks)
Priority: HIGH (BIGGEST GAP at 40-60%)

Objectives:
1. Complete gamma function implementation
2. Complete Bessel functions
3. Complete zeta function
4. Add special function benchmarks
5. Validate against SymPy/SciPy

Success Criteria:
- All special functions implemented
- Tests pass (gamma, Bessel, zeta)
- CLAUDE.md compliant
- Quality score >= 8/10

Deliverables:
- Implemented special functions
- Integration with SmartEquationSolver
- 100+ tests
- SymPy validation for 50+ cases

Wave 4-INT: Special Functions Integration Verification (3-4 hours)
Priority: HIGH (follows Wave 4-IMPL)

Objectives:
- Verify special functions integration
- Run benchmarks
- Validate against SymPy/SciPy
- Educational explanations

Success Criteria:
- All tests pass
- Integration verified
- Quality score >= 8/10

Wave 2-COMPLETE: Matrix Equation Gaps (2-3 days, conditional)
Priority: MEDIUM (depends on Wave 2-INT findings)

Objectives:
- Complete any gaps found in Wave 2-INT verification
- Fill missing matrix equation types
- Enhance noncommutative algebra support if needed

Success Criteria:
- Wave 2 at 100% completion
- All matrix tests pass
- Quality score >= 8/10

PHASE 3 Details:

QA-1: SymPy Validation Suite (2-3 days)

Objectives:
- Create comprehensive SymPy validation suite
- Test ALL solver types against SymPy:
  * Linear equations
  * Quadratic equations
  * Polynomial equations
  * System equations (Gaussian & Gröbner)
  * Matrix equations
  * ODE (separable, linear first-order)
  * PDE (basic cases)
  * Special functions
- Document any discrepancies
- Fix correctness issues

Success Criteria:
- 95%+ agreement with SymPy
- All critical correctness issues resolved
- Discrepancies documented (if intentional)

Deliverables:
- Comprehensive validation test suite
- Report: .mathhook_sessions/gtm/SYMPY_VALIDATION_REPORT.md

QA-2: Performance Benchmarking & Optimization (2-3 days)

Objectives:
- Run full benchmark suite
- Compare against SymPy (10-100x faster target)
- Compare against Symbolica (within 2x target)
- Identify performance bottlenecks
- Optimize critical paths if needed

Success Criteria:
- All benchmarks baselined
- Performance targets met or documented
- No performance regressions

Deliverables:
- Report: .mathhook_sessions/gtm/PERFORMANCE_BENCHMARK_REPORT.md
- Baseline benchmarks for all solvers

QA-3: CLAUDE.md Full Compliance Audit (1 day)

Objectives:
- Full codebase scan for CLAUDE.md violations
- Check all Plan 7 files:
  * No emojis
  * Proper documentation style
  * File size limits (<500 lines)
  * Expression type (32 bytes)
  * Number type (16 bytes)
  * No critical stubs
- Fix all violations

Success Criteria:
- 100% CLAUDE.md compliance
- All violations fixed
- Grade: A (95%+)

Deliverables:
- Report: .mathhook_sessions/gtm/CLAUDE_MD_COMPLIANCE_AUDIT.md

QA-4 (DOC-1): Systems.rs Documentation Improvement (2-4 hours)

Objectives (from Wave 3-INT gap):
- Add doc comments to all public functions in systems.rs
- Add module-level documentation
- Clean up excessive inline comments (40+ obvious ones)
- Document algorithm choices (Gaussian vs Gröbner)
- Add usage examples

Success Criteria:
- 50+ doc comments in systems.rs
- All public functions documented
- CLAUDE.md documentation standards met
- Wave 3-INT quality score improves from 7/10 to 8-8.5/10

PHASE 4 Details:

WAVE-CLEANUP: Stub Removal & Implementation Completion (1-2 weeks)

Objectives:
- Remove/complete all 39 stubs identified
- Priority 1: Gröbner solution extraction (systems.rs:770)
- Priority 2: ODE educational methods, zero detection
- Priority 3: GPU placeholders, future enhancements
- Document all remaining TODOs as future enhancements

Success Criteria:
- Zero critical stubs remaining
- All TODOs documented in FUTURE.md
- Quality score >= 8/10

Deliverables:
- Production-ready codebase
- Report: .mathhook_sessions/gtm/WAVE_CLEANUP_REPORT.md

DOC-FINAL: Plan 7 Documentation & Examples (2-3 days)

Objectives:
- Update all Plan 7 documentation
- Create comprehensive examples for all solver types
- Update status to 100% complete
- Create integration guide
- Polish educational explanations

Success Criteria:
- All solvers documented
- Examples cover all major use cases
- Documentation CLAUDE.md compliant

Deliverables:
- Report: .mathhook_sessions/gtm/PLAN_7_FINAL_REPORT.md
- Updated documentation
- Comprehensive examples

Target Metrics (Full Plan 7 Completion):
- Quality Score: 9.5+/10
- SymPy Validation: 95%+ agreement
- Build: Zero errors, zero regressions
- CLAUDE.md: 100% compliance
- Performance: 10-100x faster than SymPy
- Coverage: All 6 waves at 100%
- File Size: All files <= 500 lines
- Tests: 676/677 minimum maintained

Success Criteria (All Phases):

Phase 1 Success:
- All verification waves pass (quality >= 8)
- Integration verified for Waves 2, 3, 6
- All failing tests identified and categorized

Phase 2 Success:
- Wave 4 at 100% completion
- All waves at 90%+ completion
- No critical gaps remaining

Phase 3 Success:
- 95%+ SymPy validation agreement
- Performance targets met
- CLAUDE.md compliance: A grade (95%+)

Phase 4 Success:
- Zero critical stubs
- Plan 7 at 100% completion
- Production-ready quality

Standard orchestration protocol:
- You are orchestrator, maintain momentum
- Create verification scripts per wave (verify_wave_N_int.sh)
- Launch agents with strict CLAUDE.md enforcement
- Verify everything before declaring complete
- Create comprehensive verification reports
- Track with TodoWrite
- Compare against SymPy for mathematical validation

Current Immediate Next Step: Wave 2-INT or Wave 6-INT verification
```

---

## Architecture: Wave-Based Verification Methodology

Standard Wave Structure (5 phases):

1. **Planning Phase**:
   - Create verification script with 8 categories
   - Define success criteria (target >= 8/10)
   - Identify files to verify
   - Plan SymPy validation cases

2. **Analysis Phase**:
   - Read current implementation
   - Identify issues/gaps
   - Document findings

3. **Fix Phase**:
   - Fix compilation errors
   - Fix mathematical correctness issues
   - Fix CLAUDE.md violations
   - Defer non-critical issues to appropriate phase

4. **Verification Phase**:
   - Run verification script
   - Measure quality score
   - Compare against target (>= 8/10)

5. **Report Phase**:
   - Create comprehensive verification report
   - Document gaps and deferrals
   - Update roadmap status

Verification Script Template (8 categories, 80 points):

```bash
#!/bin/bash
# Wave N-INT Verification Script
# Target Quality Score: >= 8/10

PROJECT_ROOT="/path/to/mathhook"
cd "$PROJECT_ROOT"

TOTAL_SCORE=0
MAX_SCORE=80

# Category 1: Compilation (10 points)
# Category 2: Tests (15 points)
# Category 3: Integration Tests (10 points)
# Category 4: CLAUDE.md Compliance (10 points)
# Category 5: API Integration (10 points)
# Category 6: Documentation (10 points)
# Category 7: Benchmarks (5 points)
# Category 8: SymPy Validation (10 points)

PERCENTAGE=$((TOTAL_SCORE * 100 / MAX_SCORE))
QUALITY_SCORE=$((TOTAL_SCORE * 10 / MAX_SCORE))

if [[ $QUALITY_SCORE -ge 8 ]]; then
    echo "VERIFICATION PASSED: Quality score >= 8"
    exit 0
elif [[ $QUALITY_SCORE -ge 6 ]]; then
    echo "VERIFICATION PARTIAL: Quality score 6-7"
    exit 1
else
    echo "VERIFICATION FAILED: Quality score < 6"
    exit 1
fi
```

---

## Critical SymPy References

All implementation must validate against SymPy for mathematical correctness.

**SymPy Location**: ~/Documents/work/math/sympy/

**Key Modules**:
1. ODEs: ~/Documents/work/math/sympy/sympy/solvers/ode/
2. Systems: ~/Documents/work/math/sympy/sympy/solvers/solveset.py
3. Matrix: ~/Documents/work/math/sympy/sympy/matrices/
4. PDEs: ~/Documents/work/math/sympy/sympy/solvers/pde/
5. Special Functions: ~/Documents/work/math/sympy/sympy/functions/special/

**Validation Strategy**:
- Extract 20-50 test cases per wave
- Run SymPy to get expected outputs
- Compare MathHook outputs against SymPy
- Document any intentional discrepancies
- Fix any mathematical correctness issues

---

## Timeline Overview

```
Week 1-2:  PHASE 1 - Verification Waves
           - Wave 2-INT (3-4h)
           - Wave 3-INT [COMPLETE 7/10]
           - Wave 6-INT (2-3h)

Week 3-4:  PHASE 2 - Gap Filling
           - Wave 4-IMPL (1-2 weeks)
           - Wave 4-INT (3-4h)
           - Wave 2-COMPLETE (2-3 days, conditional)

Week 5:    PHASE 3 - Quality Assurance
           - QA-1: SymPy Validation (2-3 days)
           - QA-2: Performance Benchmarking (2-3 days)
           - QA-3: CLAUDE.md Audit (1 day)
           - QA-4 (DOC-1): Documentation (2-4h)

Week 6:    PHASE 4 - Cleanup & Documentation
           - WAVE-CLEANUP: Stub Removal (1-2 weeks)
           - DOC-FINAL: Documentation (2-3 days)
```

**Total Duration**: 6 weeks to Plan 7 at 100% completion

---

## Files and Locations

**Verification Scripts**: /tmp/verify_wave_N_int.sh
**Verification Reports**: .mathhook_sessions/gtm/WAVE_N_INT_VERIFICATION_REPORT.md
**Quality Audit**: .mathhook_sessions/gtm/CLAUDE_MD_COMPLIANCE_AUDIT.md
**SymPy Validation**: .mathhook_sessions/gtm/SYMPY_VALIDATION_REPORT.md
**Performance**: .mathhook_sessions/gtm/PERFORMANCE_BENCHMARK_REPORT.md

**Core Files to Verify**:
- Matrix: crates/mathhook-core/src/algebra/solvers/matrix_equations.rs
- Gröbner: crates/mathhook-core/src/algebra/solvers/systems.rs
- Numerical: crates/mathhook-core/src/algebra/solvers/numerical.rs
- Special: crates/mathhook-core/src/functions/special/
- Integration: crates/mathhook-core/src/algebra/equation_analyzer.rs

**Test Files**:
- tests/test_wave_2_int_matrix.rs
- tests/test_wave_3_int_groebner.rs (existing)
- tests/test_systems_integration.rs (existing)
- tests/test_wave_6_int_numerical.rs

---

## What This Achieves

**Phase 1 Outcome**: All waves verified as integrated (quality >= 8/10)
**Phase 2 Outcome**: All waves at 90%+ completion, no critical gaps
**Phase 3 Outcome**: 95%+ SymPy validation, 100% CLAUDE.md compliance
**Phase 4 Outcome**: Production-ready, zero critical stubs, 100% complete

**Final State**:
- 6 waves all complete and integrated
- 9.5+/10 quality score
- 95%+ SymPy validation
- 10-100x faster than SymPy
- Zero critical stubs
- 100% CLAUDE.md compliant
- Production-ready for Market Launch

---

**This orchestrator command is ready to use. Copy the bootstrap block and goal statement into a new Claude Code session.**

**Document Status**: Complete orchestrator command for Plan 7 integration
**Current Progress**: 60% (Waves 1/3/5 done)
**Next Action**: Wave 2-INT or Wave 6-INT verification
**Timeline**: 6 weeks to 100% completion
**Quality Baseline**: 7/10 (Wave 3-INT verified)
