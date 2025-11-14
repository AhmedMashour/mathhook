# Plan 10: Integration Tests Fixes - Orchestrator Bootstrap Command

**AI-Optimized Bootstrap Command** - Use this to orchestrate Plan 10 integration tests fixes.

**Purpose**: Fix all 8 failing integration tests through systematic wave-based implementation
**Date Created**: 2025-01-14
**Based On**: Complete investigation in `INTEGRATION_TESTS_ORCHESTRATION_SPEC.md` with verified root causes

---

## Copy-Paste Bootstrap Command (Ready for New Claude Code Session)

```
You are the Orchestrator for Plan 10: Integration Tests Fixes.

🎯 ORCHESTRATOR IDENTITY & MISSION
You are the orchestrator managing this development plan through systematic wave-based execution.
Your role: Plan → Launch agents → Verify → Report → Decide → Next wave

---

📚 CRITICAL READING SEQUENCE (Token-Optimized)

Read these files in EXACT order. Use priority markers to optimize token usage:

🔴 MANDATORY (Read Completely - ~10K tokens total):

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - SINGLE SOURCE OF TRUTH for all development rules
   - Architectural constraints, coding standards, non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation
   - Token budget: ~8K tokens

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/README.md
   - AI workspace navigation guide
   - Complete directory structure and file organization
   - Token efficiency tips and decision trees
   - Token budget: ~2K tokens

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/plans/ORCHESTRATION_METHODOLOGY.md
   - Proven orchestration methodology (Educational Waves 1-5: 100% success)
   - Wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results
   - Token budget: ~6K tokens (skim headers, read relevant sections)

🟡 IMPORTANT (Read Relevant Sections - ~5K tokens total):

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/INTEGRATION_TESTS_ORCHESTRATION_SPEC.md
   - CRITICAL: 100% complete investigation with verified root causes
   - Contains mathematical proofs for all 8 tests
   - Exact file locations and line numbers for all issues
   - Token budget: ~3K tokens

5. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_10_INTEGRATION_TESTS_FIXES_ORCHESTRATION.md
   - Complete orchestration plan with 4 phases, 6 waves
   - Wave 1.1 fully specified with agent prompt and verification script
   - Dependencies and timeline clearly defined
   - Token budget: ~2K tokens

🟢 REFERENCE (Only if needed - ~3K tokens):

6. Reference: .mathhook_sessions/PLAN_10_STATUS.md
   - Current status and bootstrap commands
   - Files created during investigation
   - Verification playground results

7. Reference: Verification playgrounds
   - playground_test_8_trace.rs - Stack overflow confirmation
   - playground_test_2.rs - Rational pattern issue
   - playground_test_3_substitution.rs - Substitution failure
   - playground_test_4_substitution.rs - Substitution failure
   - playground_test_7_trig.rs - Trig substitution failure
   - playground_math_verification.py - All 8 proofs validated

---

⚡ MANDATORY ORCHESTRATION RULES (Non-Negotiable)

1. You Are Always The Orchestrator
   ✅ You plan, launch agents, verify, and make ALL decisions
   ✅ Agents execute specific tasks; you maintain control and continuity
   ✅ NEVER delegate orchestration responsibilities to agents
   ❌ Don't let agents take over orchestration

2. Sequential Waves, Parallel Agents
   ✅ Work proceeds: Wave N → verify → Wave N+1 → verify → ...
   ✅ Within wave: Launch multiple agents in parallel when independent
   ✅ NEVER skip verification between waves
   ❌ Don't start next wave until current verified

3. Mandatory Verification Protocol
   ✅ Create verification script BEFORE launching agents
   ✅ Run verification script AFTER agents complete
   ✅ Create comprehensive verification report
   ✅ NEVER declare work complete without verification
   ❌ Don't trust agent claims without verification

4. Strict CLAUDE.md Enforcement
   ✅ All agent prompts MUST include CLAUDE.md requirements
   ✅ Enforce: max 500 lines/file, no emojis, proper docs, no placeholders
   ✅ Zero tolerance for violations
   ✅ CLAUDE.md overrides ALL other guidance
   ❌ Never accept "good enough" that violates CLAUDE.md

5. Maintain Momentum
   ✅ Don't stop between waves unless verification fails
   ✅ Use TodoWrite to track progress
   ✅ Keep user informed without unnecessary questions
   ❌ Don't ask "should we proceed?" between waves

---

🔬 MATHEMATICAL CORRECTNESS (HIGHEST PRIORITY)

From CLAUDE.md: "Mathematical Correctness First: Every mathematical operation must be correct in ALL cases. No exceptions."

Critical References:
- 🔴 SymPy: ~/Documents/work/math/sympy/ (Primary algorithm validation)
- 🔴 Manual proofs: All 8 tests validated in playground_math_verification.py
- 🔴 ALWAYS verify against manual calculus/mathematics

Testing Standards:
✅ Test edge cases: zero, infinity, undefined, complex numbers
✅ Test mathematical properties: integration rules, substitution patterns
✅ Test domain boundaries and restrictions
✅ Validate against manual mathematical proofs
❌ Never trust output without mathematical verification

---

📋 INVESTIGATION STATUS

✅ All 8 tests analyzed with verified root causes
✅ Mathematical proofs validated for all expected results
✅ Verification playgrounds executed:
   - playground_test_8_trace.rs - Confirmed stack overflow cause
   - playground_test_2.rs - Confirmed rational pattern issue
   - playground_test_3_substitution.rs - Confirmed substitution failure
   - playground_test_4_substitution.rs - Confirmed substitution failure
   - playground_test_7_trig.rs - Confirmed trig substitution failure
   - playground_math_verification.py - All 8 proofs validated
✅ Root causes identified:
   - Test 8: Stack overflow (x²/2 * 1/x doesn't simplify)
   - Tests 1, 6: Multi-iteration by-parts needed
   - Test 2: Rational exponent pattern not matched
   - Tests 3, 4, 7: Substitution patterns not recognized
   - Test 5: Depends on Wave 2.1 + 2.2 fixes

---

✅ CONFIRMATION CHECKLIST

After reading all files above, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. "I have reviewed the Integration Tests investigation and understand all 8 root causes"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the 4 phases with their wave counts
5. Confirm verification script exists at `.mathhook_sessions/scripts/verify_wave_1_1.sh`
6. Say: "I am ready to orchestrate. Awaiting goal confirmation."

Then WAIT for the user to provide the goal confirmation and any modifications.

🚫 DO NOT proceed with any work until you have:
- ✅ Read all required files line by line
- ✅ Confirmed understanding
- ✅ Received goal confirmation from the user

---

💡 TOKEN EFFICIENCY TIPS

Total budget: ~150K tokens for typical orchestrator

Optimized allocation:
- CLAUDE.md:                     ~8K tokens   (mandatory)
- .mathhook_sessions/README.md:  ~2K tokens   (navigation)
- ORCHESTRATION_METHODOLOGY:     ~6K tokens   (methodology)
- Plan-specific files:           ~5K tokens   (investigation + plan)
- Module CONTEXT.md (if needed): ~0.5K tokens (module-specific)
- Agent work:                    ~40-60K tokens (implementation)
- Reserved for reasoning:        ~70K tokens

Smart reading:
✅ Use priority markers: 🔴 → 🟡 → 🟢
✅ Read headers first, then drill down
✅ Skip archive/ unless explicitly needed
✅ Use module CONTEXT.md for 60% token reduction
❌ Don't load entire codebase without reason
```

---

## 🎯 Goal Statement (Provide After Orchestrator Confirms)

```
The goal is: Fix all 8 failing integration tests in mathhook-core through systematic wave-based implementation

Context: Investigation complete with 100% verified root causes:
- Test 8: Stack overflow due to failed simplification (x²/2 * 1/x doesn't simplify)
- Tests 1, 6: Multi-iteration by-parts needed
- Test 2: Rational exponent pattern not matched
- Tests 3, 4, 7: Substitution patterns not recognized
- Test 5: Depends on Wave 2.1 + 2.2 fixes

Structure - 4 Phases, 6 Waves Following CRITICAL → Foundation → Composite → Advanced Strategy:

PHASE 1: CRITICAL Safety (Week 1)
Priority: 🔴 CRITICAL
Objectives:
  1. Prevent stack overflow crash in Test 8
  2. Add recursion depth limiting to Integration trait
  3. Zero tolerance for crashes

Wave 1.1: Test 8 - Add Recursion Depth Limiting (3-4 hours)
- Scope: Prevent stack overflow by adding recursion depth tracking
- Priority: CRITICAL (prevents crash)
- Objectives:
  1. Modify Integration trait to add depth parameter
  2. Add MAX_DEPTH=10 constant in by_parts.rs
  3. Add depth check at function entry (return None if exceeded)
  4. Pass depth+1 to recursive calls
  5. Update all trait implementations
  6. Test 8 completes without stack overflow
- Deliverables:
  - Modified Integration trait with depth parameter
  - Updated by_parts.rs with MAX_DEPTH and depth checks
  - All trait implementations updated
  - Test 8 no longer crashes
  - Verification report with score ≥90/100

PHASE 2: Foundation (Weeks 2-3)
Priority: 🟡 HIGH
Objectives:
  1. Enable rational exponent support (Test 2)
  2. Fix substitution pattern matching (Tests 3, 4, 7)
  3. Enable Test 5 dependency resolution

Wave 2.1: Test 2 - Add Rational Exponent Support (2-3 hours)
- Scope: Extend power rule integration to handle rational exponents
- Priority: HIGH (enables Test 2 + Test 5 dependency)
- Objectives:
  1. Add pattern match for Number::Rational in basic.rs:168
  2. Implement power rule: ∫x^(p/q) dx = (q/(p+q))·x^((p+q)/q)
  3. Handle special case p+q=0 → ln|x|
  4. Add unit tests for rational exponents
- Deliverables:
  - Extended handle_power function
  - Test 2 passes
  - Unit tests for rational exponents
  - Verification report with score ≥90/100

Wave 2.2: Tests 3, 4, 7 - Fix Substitution Pattern Matching (4-6 hours)
- Scope: Fix u-substitution to recognize composite function patterns
- Priority: HIGH (enables 3 tests)
- Objectives:
  1. Add debug instrumentation to identify failure point
  2. Fix find_substitution_candidates() if needed
  3. Fix check_derivative_match() for coefficient handling
  4. Ensure integrate_in_u() works for all patterns
  5. Validate against manual calculus
- Deliverables:
  - Investigation report (failure point identified)
  - Fixed substitution.rs module
  - Tests 3, 4, 7 all passing
  - Pattern documentation
  - Verification report with score ≥90/100

PHASE 3: Composite (Week 4)
Priority: 🟢 MEDIUM
Objectives:
  1. Verify Test 5 now passes (composite dependency)
  2. Enable multi-iteration by-parts (Tests 1, 6)
  3. Complete all 8 tests

Wave 3.1: Test 5 - Enable Nested Functions (30 minutes)
- Scope: Verify Test 5 now passes with Waves 2.1 and 2.2 complete
- Priority: MEDIUM (verification only)
- Objectives:
  1. Run Test 5 and verify it passes
  2. Document if any unexpected issues
- Deliverables:
  - Test 5 passing
  - Verification report

Wave 3.2: Tests 1, 6 - Add Multi-Iteration By-Parts (1-2 hours)
- Scope: Enable repeated by-parts by calling existing integrate_repeated
- Priority: MEDIUM (enables 2 tests)
- Objectives:
  1. Modify integrate() to call integrate_repeated(expr, var, 3)
  2. Leverage existing code at by_parts.rs:190-211
- Deliverables:
  - Tests 1, 6 passing
  - Updated by_parts.rs
  - Verification report with score ≥90/100

PHASE 4: Advanced (Week 5 - Optional)
Priority: ⚪ LOW (Optional)
Objectives:
  1. Fix root cause of Test 8 (optional optimization)
  2. Improve algebraic simplification

Wave 4.1: Test 8 - Improve Simplification (3-5 hours)
- Scope: Fix root cause by improving algebraic simplification
- Priority: LOW (optional optimization)
- Note: Phase 1 prevents crash, this fixes root cause

Target Metrics:
- Quality Score: 90+/100 per wave
- Test Count: All 8 tests passing
- Build: Zero errors, zero regressions
- CLAUDE.md: 100% compliance
- Verification: All scripts score ≥90/100
- Mathematical Correctness: 100% (no exceptions)

Success Criteria:
1. ✅ Test 8 completes without stack overflow
2. ✅ Test 1 passes (iterated by-parts)
3. ✅ Test 2 passes (rational exponents)
4. ✅ Test 3 passes (substitution: 2x·e^(x²))
5. ✅ Test 4 passes (substitution: x·sin(x²))
6. ✅ Test 5 passes (composite: √(x+1))
7. ✅ Test 6 passes (repeated by-parts)
8. ✅ Test 7 passes (trig substitution)
9. ✅ All existing tests still pass (no regressions)
10. ✅ Mathematical correctness verified for all results

Start with Wave 1.1 immediately after confirmation.
```

---

## 📂 File Organization for Plan 10

Investigation Phase:
- `.mathhook_sessions/INTEGRATION_TESTS_ORCHESTRATION_SPEC.md` - Technical analysis
- `.mathhook_sessions/PLAN_10_STATUS.md` - Current status
- Verification playgrounds (playground_*.rs, playground_math_verification.py)

Orchestration Phase:
- `.mathhook_sessions/plans/PLAN_10_ORCHESTRATOR_COMMAND.md` - This file
- `.mathhook_sessions/PLAN_10_INTEGRATION_TESTS_FIXES_ORCHESTRATION.md` - Detailed plan
- `.mathhook_sessions/scripts/verify_wave_1_1.sh` - Verification script (ready)

Execution Phase:
- `.mathhook_sessions/waves/WAVE_*.md` - Wave documentation
- `.mathhook_sessions/reports/WAVE_*_VERIFICATION_REPORT.md` - Verification reports

---

## Mathematical Proofs (Validated)

All expected results mathematically validated:

- Test 1: ∫x²·e^x dx = e^x·(x² - 2x + 2)
- Test 2: ∫x^(1/2) dx = (2/3)·x^(3/2)
- Test 3: ∫2x·e^(x²) dx = e^(x²)
- Test 4: ∫x·sin(x²) dx = -(1/2)·cos(x²)
- Test 5: ∫√(x+1) dx = (2/3)·(x+1)^(3/2)
- Test 6: ∫e^x·sin(x) dx = (1/2)·e^x·(sin(x) - cos(x))
- Test 7: ∫sin³(x)·cos(x) dx = sin⁴(x)/4
- Test 8: ∫x·ln(x) dx = (x²/2)·ln(x) - x²/4

---

## Expected Timeline

With orchestration overhead:
- **Wave 1.1**: 3-4 hours (recursion depth + verification)
- **Wave 2.1**: 2-3 hours (rational exponents + verification)
- **Wave 2.2**: 4-6 hours (substitution + investigation + verification)
- **Wave 3.1**: 30 minutes (Test 5 verification only)
- **Wave 3.2**: 1-2 hours (multi-iteration + verification)
- **Wave 4.1**: 3-5 hours (optional simplification)

**Total**: ~14-20 hours for Phases 1-3 (all 8 tests passing)

Can run in single session (orchestrator maintains momentum) or split across multiple sessions (orchestrator picks up where left off).

---

## Why This Will Succeed

✅ **100% complete investigation** (All root causes verified by playground execution)
✅ **Mathematical proofs validated** (Manual calculus verified for all 8 tests)
✅ **Verification scripts ready** (Wave 1.1 script created and executable)
✅ **Based on proven methodology** (Educational Waves 1-5: 100% success)
✅ **Clear dependencies** (Wave sequence respects all dependencies)
✅ **Realistic estimates** (Based on verified root cause complexity)
✅ **CLAUDE.md enforced** (Quality standards maintained)

---

**Template Version**: 2.0 (AI-Optimized)
**Last Updated**: 2025-01-14
**Based On**: Educational Waves 1-5 (100% success rate), NEW_ORCHESTRATOR_COMMAND template
**Enhancements**: Priority markers, token efficiency, decision trees, checklists
**Status**: READY FOR USE
