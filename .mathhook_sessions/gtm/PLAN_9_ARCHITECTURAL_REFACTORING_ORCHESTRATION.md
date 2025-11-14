# Plan 9: Architectural Refactoring - Orchestrator Bootstrap Command

**AI-Optimized Bootstrap Command** - Use this to orchestrate Plan 9 architectural refactoring.

**Purpose**: Transform MathHook Core architecture from organic complexity to clean, maintainable structure with clear boundaries
**Created**: 2025-11-13
**Status**: 🟢 Ready for Execution (Phase 0: CONTEXT.md Preparation)
**Performance Guarantee**: 0% runtime degradation confirmed, potential 5-15% improvement

---

## Copy-Paste Bootstrap Command (Ready for New Claude Code Session)

```
You are the Orchestrator for Plan 9: Architectural Refactoring.

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

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_9_ARCHITECTURAL_REFACTORING_ORCHESTRATION.md
   - This file - complete context for Plan 9
   - Token budget: ~3K tokens

5. Reference: Module CONTEXT.md files as created in Phase 0
   - 60% token reduction for module-specific work
   - Token budget: ~0.5K each (only read when needed)

🟢 REFERENCE (Only if needed - ~3K tokens):

6. Reference: .mathhook_sessions/gtm/module_inventory.md
   - Module inventory if additional context needed

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

**Performance Guarantee**:
- ✅ 0% runtime performance degradation CONFIRMED
- ✅ Potential 5-15% performance improvement (from duplicate elimination)
- ✅ All architectural changes are compile-time only
- ✅ Expression/Number/Symbol constraints preserved (32-byte/16-byte)

Critical References:
- 🔴 SymPy: ~/Documents/work/math/sympy/ (Primary algorithm validation)
- 🔴 100% test pass rate REQUIRED (no regressions)
- 🔴 ALWAYS verify mathematical correctness after refactoring

---

📋 PLAN 9 ARCHITECTURE ANALYSIS

Current State: 200+ modules, 9 major domains

Architectural Smells:
✅ Strong foundation: Core types well-architected
⚠️ Module bloat: algebra/ has 20+ submodules mixing concerns
⚠️ Deep nesting: calculus/derivatives/ has 4-5 levels
⚠️ Code duplication: 15+ duplicate helper functions
⚠️ API inconsistency: 3 different solver patterns

Critical Issues Identified:
1. Module Bloat: algebra/ contains symbolic + solving + numerical
2. Deep Nesting: Parallel concepts buried 4-5 levels deep
3. Code Duplication: extract_exponents appears 4 times
4. API Inconsistency: LinearSolver vs QuadraticSolver use different patterns
5. Unclear Dependencies: No explicit layer hierarchy

---

✅ CONFIRMATION CHECKLIST

After reading all files above, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. "I have reviewed Plan 9 architecture analysis: 200+ modules with identified smells"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the 4 phases of Plan 9
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
- Plan-specific files:           ~3K tokens   (this file)
- Module CONTEXT.md (CRITICAL):  ~0.5K tokens (60% reduction!)
- Agent work:                    ~40-60K tokens (implementation)
- Reserved for reasoning:        ~70K tokens

Smart reading:
✅ Use priority markers: 🔴 → 🟡 → 🟢
✅ Read headers first, then drill down
✅ Skip archive/ unless explicitly needed
✅ USE MODULE CONTEXT.md for 60% token reduction (CRITICAL for Plan 9)
❌ Don't load entire codebase without reason
```

---

## 🎯 Goal Statement (Provide After Orchestrator Confirms)

```
The goal is: Transform MathHook Core Architecture Through Systematic Refactoring

Context: MathHook has 200+ modules with architectural smells: module bloat (algebra/ with 20+ submodules), deep nesting (4-5 levels), code duplication (15+ helpers), API inconsistency (3 solver patterns). Goal: Clean, maintainable structure with clear boundaries while preserving 100% mathematical correctness and 0% runtime performance degradation.

Performance Validation: All changes are compile-time only. Zero runtime cost confirmed. Potential 5-15% improvement from better inlining.

Structure - 4 Phases Following Token-Efficient Strategy:

PHASE 0: CONTEXT.md Preparation (Week 1)
Priority: 🔴 CRITICAL (Enables 60% token reduction)
Objectives:
  1. Create module-specific CONTEXT.md files for all 25 modules
  2. Enable 60% token reduction in agent execution
  3. Clear module boundaries for AI agents

Wave 0.1: Module Inventory and Planning (Days 1-2)
- Scope: Identify all 25 modules requiring CONTEXT.md
- Deliverables:
  - Complete module inventory
  - CONTEXT.md template defined
  - Module dependency map

Wave 0.2: Collaborative CONTEXT.md Creation (Days 3-7)
- Scope: Create CONTEXT.md for all 25 modules with user collaboration
- Process: 30 min per module × 25 = 12.5 hours over 5 days
- Deliverables:
  - 25 CONTEXT.md files in module directories
  - Master CONTEXT.md index
  - User approval for all files

Wave 0.3: Validation and Agent Testing (Week 1 final)
- Scope: Validate completeness and test token reduction
- Deliverables:
  - Validation report
  - Token reduction metrics (target: 60%)
  - Phase 0 completion certificate

PHASE 1: Module Reorganization (Weeks 2-5, 4 weeks)
Priority: 🟡 HIGH
Objectives:
  1. Flatten deep nesting
  2. Split bloated modules
  3. Consolidate duplicates
  4. Zero runtime cost (compile-time only)

Wave 1.1: Split algebra/ Module (Week 2)
- Scope: Break algebra/ into symbolic/, solvers/, numerical/
- Deliverables:
  - algebra/ removed
  - 3 new top-level modules
  - All tests passing (100% regression-free)
  - Backward compatibility via re-exports
  - Verification report with score ≥90/100

Wave 1.2: Flatten calculus/derivatives/ (Week 3)
- Scope: Move implicit.rs, parametric.rs, vector.rs up from advanced_differentiation/
- Deliverables:
  - Flat 3-level structure
  - All tests passing
  - Verification report

Wave 1.3: Consolidate Duplicate Utilities (Week 4)
- Scope: Create utils/ module for 15+ duplicate helpers
- Deliverables:
  - utils/polynomial.rs, utils/expression.rs, utils/numeric.rs
  - All duplicates removed
  - All tests passing
  - Verification report

Wave 1.4: Groebner Basis Relocation (Week 5)
- Scope: Move algebra/groebner/ to symbolic/polynomials/groebner/
- Deliverables:
  - Better module organization
  - All tests passing
  - Verification report

PHASE 2: API Standardization (Weeks 6-9, 4 weeks)
Priority: 🟡 HIGH
Objectives:
  1. Unify solver APIs
  2. Standardize educational framework
  3. Consistent patterns across codebase
  4. Zero runtime cost (static dispatch)

Wave 2.1: Solver API Unification (Weeks 6-7)
- Scope: Standardize LinearSolver, QuadraticSolver, PolynomialSolver APIs
- Deliverables:
  - Solver trait with consistent interface
  - All solvers implement trait
  - Zero runtime cost (static dispatch)
  - All tests passing
  - Verification report with score ≥90/100

Wave 2.2: Educational Framework Standardization (Weeks 8-9)
- Scope: Unify 2 different educational patterns
- Deliverables:
  - Single educational framework pattern
  - All educational features migrated
  - All tests passing
  - Verification report

PHASE 3: Dependency Management & Testing (Weeks 10-13, 4 weeks)
Priority: 🟢 MEDIUM
Objectives:
  1. Define explicit layer hierarchy
  2. Enforce architectural boundaries
  3. Comprehensive integration testing
  4. Compile-time enforcement only

Wave 3.1: Layer Hierarchy Definition (Week 10)
- Scope: Define and document 6-layer architecture
- Deliverables:
  - Layer 0 (core) → Layer 5 (educational) hierarchy
  - Dependency rules documented
  - CLAUDE.md updated

Wave 3.2: Architectural Boundary Enforcement (Week 11)
- Scope: Add compile-time checks for layer violations
- Deliverables:
  - CI checks for dependency violations
  - Zero runtime cost
  - Documentation

Wave 3.3: Integration Test Suite (Weeks 12-13)
- Scope: Comprehensive cross-module integration tests
- Deliverables:
  - Integration test coverage ≥95%
  - All tests passing
  - Performance benchmarks
  - Final verification report

PHASE 4: Documentation & Migration (Week 14, ongoing)
Priority: 🟢 LOW
Objectives:
  1. Update all documentation
  2. Migration guide for users
  3. Deprecation plan
  4. Final cleanup

Activities:
- Update CLAUDE.md with new structure
- Create migration guide
- Deprecation timeline for old paths
- Final verification and quality audit

Target Metrics:
- Quality Score: 9+/10 per wave
- Test Count: 100% passing (zero regressions)
- Build: Zero errors, zero warnings
- CLAUDE.md: 100% compliance
- Verification: All scripts score ≥90/100
- Mathematical Correctness: 100% (no exceptions)
- Performance: 0% runtime degradation (compile-time only changes)
- Token Efficiency: 60% reduction via CONTEXT.md

Success Criteria:
1. ✅ All 25 CONTEXT.md files created and validated (Phase 0)
2. ✅ Module count reduced by 20-30% through consolidation
3. ✅ Zero deep nesting (max 3 levels)
4. ✅ Zero code duplication (all helpers in utils/)
5. ✅ Unified API patterns across all solvers
6. ✅ Explicit layer hierarchy enforced
7. ✅ 100% test pass rate (zero regressions)
8. ✅ 0% runtime performance degradation
9. ✅ 100% CLAUDE.md compliance
10. ✅ 60% token reduction via CONTEXT.md
11. ✅ Clean migration path for users
12. ✅ Quality score 9+/10 across all waves

Start with Phase 0, Wave 0.1 immediately after confirmation.
```

---

## 📂 File Organization for Plan 9

Investigation Phase:
- `.mathhook_sessions/gtm/PLAN_9_ARCHITECTURAL_REFACTORING_ORCHESTRATION.md` - This file
- `.mathhook_sessions/gtm/module_inventory.md` - Module inventory (create in Wave 0.1)
- `.mathhook_sessions/gtm/CONTEXT_TEMPLATE.md` - Template (create in Wave 0.1)
- `.mathhook_sessions/gtm/context_index.md` - Master index (create in Wave 0.2)

Orchestration Phase:
- `.mathhook_sessions/scripts/verify_wave_*.sh` - Verification scripts (create per wave)
- `crates/mathhook-core/src/[module]/CONTEXT.md` - Module-specific context (25 files)

Execution Phase:
- `.mathhook_sessions/waves/WAVE_*.md` - Wave documentation
- `.mathhook_sessions/reports/WAVE_*_VERIFICATION_REPORT.md` - Verification reports

---

**Template Version**: 2.0 (AI-Optimized)
**Last Updated**: 2025-01-14
**Based On**: Educational Waves 1-5 (100% success rate), NEW_ORCHESTRATOR_COMMAND template
**Enhancements**: Priority markers, token efficiency, CONTEXT.md focus, checklists
**Status**: READY FOR USE
