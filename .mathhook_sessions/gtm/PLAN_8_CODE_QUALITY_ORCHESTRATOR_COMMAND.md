# Plan 8: Code Quality, Mathematical Correctness, and CLAUDE.md Compliance - Orchestrator Bootstrap Command

**AI-Optimized Bootstrap Command** - Use this to orchestrate Plan 8 code quality remediation.

**Purpose**: Address ALL issues found in comprehensive deep analysis (clippy, tests, mathematical correctness, CLAUDE.md compliance)
**Date Created**: 2025-01-13
**Baseline**: 14 failing tests, 6 critical issues, 42 high priority issues, ~300 medium priority issues
**Target**: Zero critical issues, all tests passing, 100% CLAUDE.md compliance, 9.5+/10 quality

---

## Copy-Paste Bootstrap Command (Ready for New Claude Code Session)

```
You are the Orchestrator for Plan 8: Code Quality, Mathematical Correctness, and CLAUDE.md Compliance.

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

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_8_CODE_QUALITY_ORCHESTRATOR_COMMAND.md
   - This file - complete context for Plan 8 (skip copy-paste block and goal statement)
   - Token budget: ~3K tokens

5. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_8_PHASED_ROADMAP.md
   - Detailed 4-phase breakdown with all wave objectives
   - Current status for each issue category
   - Success criteria and deliverables for each phase
   - Token budget: ~2K tokens

🟢 REFERENCE (Only if needed - ~3K tokens):

6. Reference: Deep analysis reports (if additional issue details needed)
   - Use for understanding specific issue categories

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

From Deep Analysis: "The mathhook-core codebase is fundamentally sound but suffers from:
1. Quality debt: Extensive CLAUDE.md violations (comments, Symbol usage)
2. Mathematical bugs: Root-finding and monomial ordering
3. Incomplete features: ODE solver with tests but no implementation
4. Testing gap: 14 failing tests expose numerical instability"

Critical References:
- 🔴 SymPy: ~/Documents/work/math/sympy/ (Primary algorithm validation)
- 🔴 Manual proofs: Verify all mathematical operations against manual calculus
- 🔴 ALWAYS verify against manual mathematical proofs

Testing Standards:
✅ Test edge cases: zero, infinity, undefined, complex numbers
✅ Test mathematical properties: algebraic correctness, numerical stability
✅ Test domain boundaries and restrictions
✅ Validate against manual mathematical proofs
❌ Never trust output without mathematical verification

---

📋 PLAN 8 ISSUE BREAKDOWN

Current Status: 367 warnings, 14 failing tests, 85% CLAUDE.md compliance

🔴 CRITICAL ISSUES (6 - MUST FIX IMMEDIATELY):

C1: Grevlex Monomial Ordering Bug
   - Mathematical correctness violation
   - Affects Gröbner basis computation

C2: Newton-Raphson Zero Derivative
   - Infinite loop risk
   - Numerical stability issue

C3: Bisection Method Validation
   - Missing precondition checks
   - Potential runtime failures

C4: 4 ODE Tests for Unimplemented Features
   - CLAUDE.md violation: incomplete implementation
   - Tests exist but code missing

C5: Symbol::new() Used 287 Times
   - MASSIVE Priority 1 violation
   - Should use symbol!() macro

C6: Expression Size Unchecked
   - Silent performance regression risk
   - 32-byte constraint not enforced

🟡 HIGH PRIORITY ISSUES (42):

H1-H77: 77 unused imports (auto-fixable)
H78: Ambiguous glob re-exports
H79-H82: Numerical algorithm precision issues (4 root-finding tests failing)
H83: 30+ unwrap/expect in library code (panic policy violation)

🟢 MEDIUM PRIORITY ISSUES (~300):

M1-M290: 290 non-snake-case warnings (caused by Symbol::new)
M291-M4838: 4,838 inline comments (many should be /// or removed)
M4839: Matrix canonical form TODO

⚪ LOW PRIORITY (Technical Debt):

L1: 3,193 excessive clone operations
L2: Missing systematic benchmarks

Quality Baseline:
- Build: ✅ PASSING (with 367 warnings)
- Tests: 676/677 minimum maintained
- CLAUDE.md compliance: 85% (B+ grade, targeting 100%)

---

✅ CONFIRMATION CHECKLIST

After reading all files above, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. "I have reviewed Plan 8 issue breakdown: 6 critical, 42 high priority, ~300 medium priority"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the 4 phases of Plan 8
5. Confirm verification scripts exist at `.mathhook_sessions/scripts/verify_wave_*.sh` (check which ones)
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
- Plan-specific files:           ~5K tokens   (issue breakdown + roadmap)
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
The goal is: Complete Plan 8 Code Quality and Mathematical Correctness Remediation

Context: Comprehensive deep analysis identified 6 critical issues, 42 high priority issues, and ~300 medium priority issues across mathematical correctness, code quality, and CLAUDE.md compliance. Current baseline: 14 failing tests, 85% CLAUDE.md compliance (B+ grade), 367 warnings. Target: Zero critical issues, all tests passing, 100% CLAUDE.md compliance, 9.5+/10 quality.

Analysis Findings Summary:
- 6 CRITICAL mathematical/architectural issues requiring immediate attention
- 42 HIGH PRIORITY quality and safety issues
- ~300 MEDIUM PRIORITY style and cleanup issues
- 14 failing tests with root causes identified
- 287 uses of deprecated Symbol::new() (should use symbol!() macro)
- 4,838 inline comments needing conversion or removal

Structure - 4 Phases Following Priority-Based Strategy:

PHASE 1: Critical Mathematical Correctness (Week 1)
Priority: 🔴 CRITICAL
Objectives:
  1. Fix all 6 critical issues immediately
  2. Restore 100% mathematical correctness
  3. Zero tolerance for mathematical bugs

Wave 1.1: Mathematical Correctness Fixes (2-3 days)
- Scope: Fix C1 (Grevlex), C2 (Newton-Raphson), C3 (Bisection)
- Deliverables:
  - All mathematical bugs fixed
  - Tests passing for affected algorithms
  - Verification report with score ≥95/100

Wave 1.2: Feature Completion (2-3 days)
- Scope: Fix C4 (ODE implementation)
- Deliverables:
  - Complete implementations for all tested features
  - All 4 ODE tests passing
  - Verification report with score ≥90/100

Wave 1.3: Architectural Compliance (1-2 days)
- Scope: Fix C5 (Symbol macro migration), C6 (Expression size checks)
- Deliverables:
  - All Symbol::new() replaced with symbol!() macro
  - Expression size validation in CI
  - Verification report with score ≥90/100

PHASE 2: High Priority Quality & Safety (Week 2)
Priority: 🟡 HIGH
Objectives:
  1. Fix all 42 high priority issues
  2. Eliminate panic risks
  3. Improve numerical stability

Wave 2.1: Auto-fixable Warnings (1 day)
- Scope: Fix H1-H77 (unused imports), H78 (ambiguous re-exports)
- Deliverables:
  - Zero auto-fixable warnings
  - Clippy happy
  - Verification report

Wave 2.2: Numerical Stability (2-3 days)
- Scope: Fix H79-H82 (root-finding precision)
- Deliverables:
  - All 4 numerical tests passing
  - Precision improvements validated
  - Verification report with score ≥90/100

Wave 2.3: Panic Policy Enforcement (2-3 days)
- Scope: Fix H83 (unwrap/expect in library code)
- Deliverables:
  - All 30+ unwrap/expect replaced with Result
  - Zero panic risk in public API
  - Verification report with score ≥90/100

PHASE 3: Medium Priority Cleanup (Weeks 3-4, Incremental)
Priority: 🟢 MEDIUM
Objectives:
  1. Address ~300 medium priority issues
  2. 100% CLAUDE.md compliance
  3. Clean codebase ready for 0.2 release

Wave 3.1: Symbol Migration Cleanup (2-3 days)
- Scope: Fix M1-M290 (non-snake-case warnings from Symbol::new)
- Note: Should be resolved by Wave 1.3, verify cleanup
- Deliverables:
  - Zero non-snake-case warnings
  - Verification report

Wave 3.2: Comment Audit (1 week, incremental)
- Scope: Fix M291-M4838 (inline comment violations)
- Deliverables:
  - Convert /// where appropriate
  - Remove redundant comments
  - CLAUDE.md documentation compliance ≥95%
  - Verification report with score ≥85/100

Wave 3.3: Remaining TODOs (2-3 days)
- Scope: Fix M4839 (matrix canonical form) and other deferred items
- Deliverables:
  - All critical TODOs resolved
  - Verification report

PHASE 4: Long-term Architectural Improvements (Ongoing)
Priority: ⚪ LOW (Technical Debt)
Objectives:
  1. Address technical debt systematically
  2. Improve performance characteristics
  3. Establish systematic benchmarking

Wave 4.1: Clone Optimization (ongoing)
- Scope: Address L1 (3,193 excessive clones)
- Note: Incremental, non-blocking
- Deliverables:
  - Performance improvement measurements
  - Benchmark comparisons

Wave 4.2: Benchmark Infrastructure (1 week)
- Scope: Address L2 (missing benchmarks)
- Deliverables:
  - Comprehensive benchmark suite
  - Performance regression detection
  - CI integration

Target Metrics:
- Quality Score: 9.5+/10 per wave
- Test Count: 100% passing (zero failures)
- Build: Zero errors, zero warnings
- CLAUDE.md: 100% compliance
- Verification: All scripts score ≥90/100
- Mathematical Correctness: 100% (no exceptions)

Success Criteria:
1. ✅ All 6 critical issues fixed
2. ✅ All 42 high priority issues fixed
3. ✅ All 14 failing tests passing
4. ✅ Zero clippy warnings
5. ✅ 100% CLAUDE.md compliance
6. ✅ Zero panic risk in public API
7. ✅ Quality score 9.5+/10 across all waves
8. ✅ Performance benchmarks established
9. ✅ Mathematical correctness 100% verified
10. ✅ Codebase ready for 0.2 release

Start with Phase 1, Wave 1.1 immediately after confirmation.
```

---

## 📂 File Organization for Plan 8

Investigation Phase:
- Deep analysis reports (scattered across codebase analysis)
- `.mathhook_sessions/gtm/PLAN_8_PHASED_ROADMAP.md` - Detailed roadmap

Orchestration Phase:
- `.mathhook_sessions/gtm/PLAN_8_CODE_QUALITY_ORCHESTRATOR_COMMAND.md` - This file
- `.mathhook_sessions/scripts/verify_wave_*.sh` - Verification scripts (create per wave)

Execution Phase:
- `.mathhook_sessions/waves/WAVE_*.md` - Wave documentation
- `.mathhook_sessions/reports/WAVE_*_VERIFICATION_REPORT.md` - Verification reports

---

**Template Version**: 2.0 (AI-Optimized)
**Last Updated**: 2025-01-14
**Based On**: Educational Waves 1-5 (100% success rate), NEW_ORCHESTRATOR_COMMAND template
**Enhancements**: Priority markers, token efficiency, decision trees, checklists
**Status**: READY FOR USE
