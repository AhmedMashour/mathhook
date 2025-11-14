# Number Theory & Polynomial Functions Completion - Orchestrator Bootstrap Command

**Purpose**: Fix critical gaps identified in deep analysis of number theory and polynomial functions
**Date Created**: 2025-10-19
**Based On**: Analysis documents (ANALYSIS_SUMMARY.md, NUMBER_THEORY_POLYNOMIAL_ANALYSIS.md, SYMPY_FEATURE_COMPARISON.md)

---

## Copy-Paste This Entire Block Into New Claude Code Session

```
You are the Orchestrator for the Number Theory & Polynomial Functions Completion project.

CRITICAL FIRST STEP - Read these files in order and line by line to learn the proven methodology:

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - This is the SINGLE SOURCE OF TRUTH for all development rules
   - Contains architectural constraints, coding standards, and non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md
   - Complete orchestration methodology from Educational Waves 1-5
   - Contains wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/NUMBER_THEORY_POLYNOMIAL_ANALYSIS.md
   - CRITICAL: Deep analysis revealing actual implementation status
   - Shows what works, what's broken, what's missing
   - Contains source code locations and specific line numbers
   - Identifies LCM bug, polynomial evaluation gap, and GCD incompleteness

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ANALYSIS_SUMMARY.md
   - Executive summary of the 4 critical objectives
   - Priority rankings and effort estimates
   - Implementation recommendations

5. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/EDUCATIONAL_WAVE_5_VERIFICATION_REPORT.md
   - Example of complete wave verification report
   - Shows the quality standards and verification depth required

6. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/EDUCATIONAL_QUALITY_AUDIT.md
   - Example of comprehensive quality audit (8.5/10 system)
   - Shows how to score implementations and assess production readiness

MANDATORY ORCHESTRATION RULES (From Proven Methodology):

1. You Are Always The Orchestrator
   - You plan, launch agents, verify, and make decisions
   - Agents execute specific tasks; you maintain control and continuity
   - NEVER delegate orchestration responsibilities to agents

2. Sequential Waves, Parallel Agents
   - Work proceeds in waves: Wave 1 → verify → Wave 2 → verify → ...
   - Within a wave, launch multiple agents in parallel when possible
   - NEVER skip verification between waves

3. Mandatory Verification Protocol
   - Create verification script BEFORE launching agents (bash script with 8-10 categories)
   - Run verification script AFTER agents complete
   - Create comprehensive verification report
   - NEVER declare work complete without running verification

4. Strict CLAUDE.md Enforcement
   - All agent prompts MUST include CLAUDE.md requirements explicitly
   - Enforce: max 500 lines/file, no emojis, proper docs, no placeholders, build passes
   - Zero tolerance for violations
   - CLAUDE.md overrides ALL other guidance

5. Maintain Momentum
   - Don't stop between waves unless verification fails
   - Use TodoWrite to track progress through all waves
   - Keep user informed of progress without asking unnecessary questions

MATHEMATICAL CORRECTNESS - HIGHEST PRIORITY:

From CLAUDE.md: "Mathematical Correctness First: Every mathematical operation must be correct in ALL cases. No exceptions."

**Critical Mathematical References**:
- SymPy: ~/Documents/work/math/sympy/ (Primary reference for algorithm validation)
- Symbolica: ~/Documents/work/math/symbolica/ (Secondary reference)
- ALWAYS verify correctness against these references

**Testing Standards**:
- Test edge cases: zero, infinity, undefined, complex numbers
- Test mathematical properties: commutativity, associativity, distributivity
- Test domain boundaries and restrictions
- Validate against SymPy for correctness

CONFIRMATION REQUIRED:

After reading all files above line by line, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. "I have reviewed the Number Theory & Polynomial Analysis and understand the 4 critical objectives"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the 5 phases of a standard wave
5. State the 4 objectives in priority order with effort estimates
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
The goal is: Complete Number Theory & Polynomial Functions to 100% working status

Context: Deep analysis revealed critical gaps:
- Number theory functions: 40% complete (GCD works for integers, LCM broken, polynomial GCD incomplete)
- Polynomial functions: 40% complete (properties 100%, evaluation 0% - cannot compute any values)

Structure - 4 Waves Following Foundation → Implementation → Verification Strategy:

Wave 1: Fix LCM Bug & Verify Number Theory (1-2 hours)
- Scope: Critical correctness bug in symbolic LCM
- Priority: HIGHEST (correctness issue)
- Objectives:
  1. Fix LCM implementation in gcd.rs (returns a*b instead of LCM(a,b))
  2. Verify MOD operation implementation exists and works
  3. Verify is_prime implementation exists and works
  4. Document actual status of all number theory functions
  5. Add comprehensive tests for GCD, LCM, MOD (if exists), is_prime (if exists)
  6. Test against SymPy for correctness validation
- Deliverables:
  - Fixed LCM in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs
  - MOD and is_prime status verified (implementation found or marked as missing)
  - 15+ new number theory tests with SymPy validation
  - Verification report documenting all number theory function status

Wave 2: Polynomial Recurrence Evaluation Engine (12-15 hours)
- Scope: Implement evaluation capability for all 4 polynomial families
- Priority: HIGH (makes polynomials actually usable)
- Objectives:
  1. Design generic recurrence evaluator using existing property definitions
  2. Implement numerical evaluation: P_n(x) for any n, x using three-term recurrence
  3. Implement for all 4 families: Legendre, Hermite, Laguerre, Chebyshev (T_n and U_n)
  4. Add coefficient evaluation helpers (alpha, beta, gamma functions)
  5. Validate against SymPy for mathematical correctness
  6. Performance: Target <1ms for n≤100, x∈[-10,10]
- Deliverables:
  - Generic evaluation trait/system for polynomial properties
  - Working evaluate(n, x) for all 5 polynomials
  - 25+ tests validating correctness against SymPy
  - Benchmarks showing performance targets met
  - Documentation with usage examples

Wave 3: Symbolic Polynomial Expansion (6-8 hours)
- Scope: Generate explicit polynomial formulas from recurrence
- Priority: MEDIUM (educational and symbolic manipulation value)
- Objectives:
  1. Implement expand_symbolic(n) → Expression for each polynomial
  2. Generate P_n(x) = a_n*x^n + ... + a_0 symbolically
  3. Simplify using existing simplification system
  4. Validate symbolic results match numerical evaluation
  5. Integration with Expression system
- Deliverables:
  - expand_symbolic() method for all polynomials
  - 15+ tests comparing symbolic vs numerical
  - Examples: P_3(x) = (5x³-3x)/2, H_3(x) = 8x³-12x, etc.

Wave 4: Complete Polynomial GCD & Final Verification (18-20 hours)
- Scope: Implement full polynomial division and Euclidean algorithm
- Priority: MEDIUM-HIGH (needed for rational simplification)
- Objectives:
  1. Implement polynomial long division algorithm
  2. Implement quotient and remainder operations
  3. Complete Euclidean GCD algorithm for polynomials
  4. Support univariate polynomials (multivariate deferred)
  5. Test against SymPy polynomial GCD
  6. Integration with existing factorization system
- Deliverables:
  - Polynomial division: div(), quo(), rem() operations
  - Full Euclidean GCD for polynomials
  - 20+ tests including edge cases (zero, constants, high degree)
  - SymPy comparison tests
  - Final quality audit of entire number theory + polynomial system

Target Metrics:
- Quality Score: 8.5+/10 average across all waves
- Test Count: Add 75+ new content validation tests (15+15+15+30)
- SymPy Validation: 100% of mathematical operations validated against SymPy
- Build: Zero errors, zero regressions
- CLAUDE.md: 100% compliance (new work)
- Performance: Polynomial evaluation <1ms for n≤100

Success Criteria:
1. ✅ LCM bug fixed and validated against SymPy
2. ✅ MOD and is_prime status verified and documented
3. ✅ All 5 polynomial functions can evaluate P_n(x) numerically
4. ✅ Symbolic polynomial expansion working for all families
5. ✅ Polynomial GCD complete with division algorithm
6. ✅ 75+ new tests all passing with content validation
7. ✅ Zero regressions in existing 970+ tests
8. ✅ CLAUDE.md 100% compliant
9. ✅ Mathematical correctness verified against SymPy
10. ✅ Quality audit shows 8.5+/10 average

Mathematical Correctness Emphasis:
- Every operation validated against SymPy
- Edge cases thoroughly tested (zero, infinity, complex)
- Domain restrictions documented and enforced
- No approximations unless explicitly documented
- Rational arithmetic preferred over floating point

Standard orchestration protocol:
- You are orchestrator, maintain momentum
- Create verification scripts per wave (verify_wave_N_[name].sh)
- Launch agents with strict CLAUDE.md enforcement
- Verify everything before declaring complete
- Create comprehensive verification reports
- Track with TodoWrite
- Use SymPy for mathematical validation

Let's begin with Wave 1: Fix LCM Bug & Verify Number Theory
```

---

## What This Command Achieves

### Wave 1: Fix LCM Bug & Verify Number Theory (1-2 hours)
**Critical**: Fixes broken LCM implementation
**Verification**: Confirms actual status of MOD/is_prime
**Output**: Fully working number theory basics

### Wave 2: Polynomial Recurrence Evaluation (12-15 hours)
**Critical**: Makes polynomials actually usable
**Implementation**: Generic evaluation using existing properties
**Output**: Can compute P_5(0.5), H_3(2.0), T_10(0.7), L_4(1.5)

### Wave 3: Symbolic Polynomial Expansion (6-8 hours)
**Value**: Educational and symbolic manipulation
**Implementation**: Generate explicit formulas from recurrence
**Output**: P_3(x) = (5x³-3x)/2 symbolically

### Wave 4: Complete Polynomial GCD (18-20 hours)
**Critical**: Required for rational function work
**Implementation**: Full Euclidean algorithm with division
**Output**: Working polynomial GCD for simplification

---

## Key Architectural Decisions Baked Into This Plan

### 1. Foundation → Implementation Strategy
- Wave 1 fixes foundation (LCM bug)
- Waves 2-3 build on working foundation
- Wave 4 adds advanced capability

### 2. Mathematical Correctness First
- Every wave requires SymPy validation
- Edge cases mandated in success criteria
- Domain restrictions must be documented

### 3. Leverage Existing Architecture
- Wave 2 uses existing polynomial properties (recurrence relations already defined)
- Wave 3 integrates with existing Expression system
- Wave 4 integrates with existing factorization

### 4. Performance Targets
- Polynomial evaluation: <1ms for n≤100
- Benchmarks required in Wave 2
- Performance regression testing

### 5. CLAUDE.md Compliance
- Max 500 lines/file enforced
- No emojis, proper documentation
- Build must pass with zero errors
- Content validation tests required

---

## Files The Orchestrator Will Reference

All analysis context:
1. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/NUMBER_THEORY_POLYNOMIAL_ANALYSIS.md` - Detailed status
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ANALYSIS_SUMMARY.md` - Executive summary
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/SYMPY_FEATURE_COMPARISON.md` - Gap analysis

Orchestration guidance:
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven methodology
5. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/EDUCATIONAL_WAVE_5_VERIFICATION_REPORT.md` - Verification example
6. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/EDUCATIONAL_QUALITY_AUDIT.md` - Quality example

Code to modify:
7. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/gcd.rs` - LCM bug (line 40-53)
8. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/polynomials/*.rs` - Add evaluation
9. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/number_theory.rs` - Verify status

SymPy reference:
10. `~/Documents/work/math/sympy/` - Primary validation source

---

## Expected Timeline

- **Wave 1**: 1-2 hours (critical bug fix + verification)
- **Wave 2**: 12-15 hours (generic evaluation engine)
- **Wave 3**: 6-8 hours (symbolic expansion)
- **Wave 4**: 18-20 hours (polynomial division + GCD)

**Total**: 37-45 hours of agent work
**With orchestration overhead**: ~50-55 hours total

**Can be parallelized**: Waves 2 and 3 could run in parallel after Wave 1 completes

---

## Quality Targets (Based on Educational Waves Success)

| Metric | Target | Based On |
|--------|--------|----------|
| Quality Score | 8.5+/10 | Educational Waves averaged 8.5/10 |
| Tests Added | 75+ | Educational Waves added 110 tests |
| Content Validation | 80%+ | Educational Wave 5 standard |
| CLAUDE.md Compliance | 100% | Non-negotiable |
| Regressions | 0 | Zero tolerance |
| SymPy Validation | 100% | Mathematical correctness priority |

---

## Success Definition

At the end of Wave 4, we should be able to:

```rust
// Number Theory - ALL WORKING
let a = Expression::integer(12);
let b = Expression::integer(8);
assert_eq!(a.gcd(&b), Expression::integer(4));     // ✅ Already works
assert_eq!(a.lcm(&b), Expression::integer(24));    // ✅ FIXED (was 96)

// Polynomial Evaluation - ALL WORKING
use mathhook_core::functions::polynomials::*;

let legendre = LegendreIntelligence::new();
assert_eq!(legendre.evaluate(5, 0.5), 0.08984375);  // ✅ NEW

let hermite = HermiteIntelligence::new();
assert_eq!(hermite.evaluate(3, 2.0), 40.0);         // ✅ NEW

// Symbolic Expansion - ALL WORKING
let p3_symbolic = legendre.expand_symbolic(3);
// Returns: (5x³ - 3x)/2                            // ✅ NEW

// Polynomial GCD - ALL WORKING
let poly1 = parse("x^2 - 1");
let poly2 = parse("x^2 - 2x + 1");
assert_eq!(poly1.gcd(&poly2), parse("x - 1"));     // ✅ NEW
```

---

## Why This Plan Will Succeed

1. **Based on proven methodology** - Educational Waves 1-5 (100% success)
2. **Analysis-driven** - Deep understanding of what's broken vs what works
3. **Foundation-first** - Fix bugs before building new features
4. **Mathematical correctness priority** - SymPy validation built into every wave
5. **Realistic effort estimates** - Based on actual code analysis
6. **Clear success criteria** - Measurable, testable outcomes
7. **Leverages existing architecture** - Uses defined properties, doesn't reinvent
8. **CLAUDE.md enforced** - Quality and standards maintained

---

**This orchestrator command is ready to use. Copy the bootstrap block and goal statement into a new Claude Code session.**
