# Wave 3-INT: Gröbner Basis Integration Verification Plan

**Date**: 2025-01-22
**Orchestrator**: Claude Code
**Status**: Phase 1 Complete - Planning Done, Awaiting User Go-Ahead

---

## Phase 1: Planning (COMPLETE)

### Wave Objectives

**Goal**: Verify that Wave 3 (Gröbner Basis) is fully integrated into the MathHook architecture and ready for production use.

**Scope**:
1. Verify integration with `SmartEquationSolver` (follows Wave 1 & 5 pattern)
2. Confirm all tests pass (currently 13 failing tests in Plan 7)
3. Validate CLAUDE.md compliance
4. Assess the stub at `systems.rs:770` - determine if it's a blocker or acceptable future enhancement
5. Run baseline benchmarks for Gröbner basis system solving
6. Verify SymPy correctness for polynomial system solving

### Success Criteria

**Must Achieve (Quality Score ≥ 8)**:
- [ ] All Gröbner basis tests pass (no regressions)
- [ ] Integration with `SmartEquationSolver` verified and documented
- [ ] CLAUDE.md compliance: No emojis, proper documentation, file size limits
- [ ] Stub at `systems.rs:770` evaluated: Either complete implementation OR document as acceptable future enhancement per CLAUDE.md
- [ ] Baseline benchmarks created for polynomial system solving
- [ ] At least 3 test cases validated against SymPy for correctness

**Stretch Goals (Quality Score = 10)**:
- [ ] Complete stub at `systems.rs:770` (full Gröbner solution extraction)
- [ ] Performance comparison: MathHook vs SymPy for polynomial systems
- [ ] Educational explanations for Gröbner basis solving

---

## Phase 2: Agent Execution (PENDING USER GO-AHEAD)

### Agent 1: Integration Verification Agent
**Task**: Verify SmartEquationSolver integration pattern

**Specific Actions**:
- Read `SmartEquationSolver` implementation
- Verify `SystemSolver` is registered and callable
- Check that polynomial systems route correctly to Gröbner basis
- Compare integration pattern against Wave 1 (ODE) and Wave 5 (PDE) as reference
- Document integration status

**Expected Output**: Integration status report

---

### Agent 2: Test Validation Agent
**Task**: Identify and fix failing tests

**Specific Actions**:
- Run full test suite: `cargo test -p mathhook-core`
- Identify which of the 13 failing tests are related to Wave 3
- Analyze root causes
- Determine if failures are blockers or known issues
- Document test status

**Expected Output**: Test failure analysis report

---

### Agent 3: CLAUDE.md Compliance Agent
**Task**: Verify all Wave 3 code follows CLAUDE.md rules

**Specific Actions**:
- Check for emojis in code/comments/docs
- Verify documentation style (//! for modules, /// for items, minimal inline //)
- Check file sizes (<500 lines)
- Verify Expression type constraints (32 bytes)
- Verify Number type constraints (16 bytes)
- Document compliance status

**Expected Output**: CLAUDE.md compliance report

---

### Agent 4: Stub Evaluation Agent
**Task**: Evaluate stub at `systems.rs:770`

**Specific Actions**:
- Read stub implementation and comments
- Determine mathematical correctness of current behavior (returns NoSolution)
- Assess whether this is acceptable per CLAUDE.md ("TODOs for future enhancements if current behavior is correct")
- If blocker: Outline implementation plan
- If acceptable: Document as future enhancement

**Expected Output**: Stub evaluation report with recommendation

---

## Phase 3: Verification (PENDING)

### Verification Script: `verify_wave_3_int.sh`

**Categories (8-10)**:
1. **Compilation**: `cargo build -p mathhook-core`
2. **Tests**: `cargo test -p mathhook-core` (focus on Gröbner tests)
3. **Integration Tests**: `cargo test test_wave_3_int_groebner`
4. **CLAUDE.md Compliance**: Check style, size, constraints
5. **API Integration**: Verify `SmartEquationSolver` routing
6. **Documentation**: Check examples, doctests
7. **Benchmarks**: Run Gröbner basis system solving benchmarks
8. **SymPy Validation**: Compare 3+ test cases against SymPy

**Quality Scoring**: 1-10 scale, target ≥ 8 for production

---

## Phase 4: Reporting (PENDING)

### Verification Report: `WAVE_3_INT_VERIFICATION_REPORT.md`

**Contents**:
- Quality score (1-10)
- Agent findings summary
- Test results
- CLAUDE.md compliance status
- Stub evaluation recommendation
- Benchmark results
- SymPy validation results
- Issues found (if any)
- Recommendations

---

## Phase 5: Decision (PENDING)

**Decision Criteria**:
- Quality score ≥ 8: Proceed to next wave
- Quality score < 8: Iterate with fixes

**Next Wave Options**:
1. If Wave 3-INT passes → Wave 5-INT or Wave 2-INT verification
2. If Wave 3-INT needs fixes → Iteration with specific fix tasks
3. If stub is blocker → Wave 3-FIX to complete Gröbner solution extraction

---

## Timeline Estimate

**Total Duration**: 4-6 hours
- Phase 2 (Agent Execution): 2-3 hours (parallel agents)
- Phase 3 (Verification): 1 hour
- Phase 4 (Reporting): 30 minutes
- Phase 5 (Decision & Fixes): 30-60 minutes (if needed)

---

## Notes

**Context from PLAN_7_DEEP_ANALYSIS.md**:
- Wave 3 status: 80-90% complete
- Known stub at systems.rs:770 (Gröbner solution extraction)
- Integration pattern should follow Wave 1 & 5 (SmartEquationSolver)
- 13 failing tests in Plan 7 overall (need to identify which are Wave 3-related)

**CLAUDE.md Reminder**:
- Stubs are acceptable if: "TODOs for future enhancements if current behavior is correct"
- Current stub returns NoSolution (mathematically honest)
- Need to verify this meets CLAUDE.md standard

---

## Status: AWAITING USER INSTRUCTION

**Orchestrator Ready**: I have completed Phase 1 (Planning) and documented the full wave plan.

**Waiting For**: User instruction on what to do next.
