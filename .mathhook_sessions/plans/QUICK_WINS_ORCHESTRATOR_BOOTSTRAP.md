# Quick Wins Bundle - Orchestrator Bootstrap Command

**Purpose**: Elementary Functions Foundation (Month 1, Week 1 of roadmap) with 10/10 quality target
**Date Created**: 2025-10-19
**Based On**: NEXT_PRIORITIES_ROADMAP.md, VERIFICATION_COMPLETE.md, POLYNOMIAL_ARCHITECTURE_AUDIT.md

---

## Copy-Paste This Entire Block Into New Claude Code Session

```
You are the Orchestrator for the Quick Wins Bundle project.

CRITICAL FIRST STEP - Read these files in order and line by line to learn the proven methodology:

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - This is the SINGLE SOURCE OF TRUTH for all development rules
   - Contains architectural constraints, coding standards, and non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md
   - Complete orchestration methodology from Educational Waves 1-5
   - Contains wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/NEXT_PRIORITIES_ROADMAP.md
   - Strategic 6-month development roadmap
   - Month 1, Week 1: Quick wins (abs, sqrt, polynomial API)  ← THIS PROJECT
   - Rationale: Foundation for Gamma function and integration work

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/VERIFICATION_COMPLETE.md
   - Recent completion: Number theory & polynomial work (9.25/10 quality, 103 tests)
   - Current status: 514 tests passing, 70-75% SymPy coverage
   - Shows quality bar to maintain

5. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/POLYNOMIAL_ARCHITECTURE_AUDIT.md
   - Recent audit showing polynomial system is 9.0/10 quality, production-ready
   - Demonstrates architectural review process and quality standards

6. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/EDUCATIONAL_WAVE_5_VERIFICATION_REPORT.md
   - Example of complete wave verification report
   - Shows the quality standards and verification depth required (10-category verification)

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
- 100% content validation (NO structure-only tests)

CONFIRMATION REQUIRED:

After reading all files above line by line, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. "I have reviewed the roadmap and understand Quick Wins Bundle is Month 1, Week 1"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the 5 phases of a standard wave
5. State the 3 waves in this bundle with quality targets
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
The goal is: Quick Wins Bundle - Elementary Functions Foundation (Month 1, Week 1)

Context: Building on recent success (polynomial work 9.25/10, 514 tests passing). This is foundation for Month 1 Weeks 2-4 (Gamma function) and Months 2-3 (integration).

Structure - 3 Waves Following Foundation → Polish Strategy:

Wave 1: Absolute Value Function |x| (3-4 hours)
- Scope: New elementary function with full intelligence
- Priority: HIGHEST (foundation for complex modulus, simplification rules)
- Objectives:
  1. Create functions/elementary/abs.rs (~150-200 lines) with complete function intelligence
  2. Implement derivative: d/dx|x| = x/|x| for x ≠ 0, undefined at x=0
  3. Implement antiderivative: ∫|x|dx = x|x|/2 + C
  4. Domain: ℝ (real), ℂ (complex with |a+bi| = √(a²+b²))
  5. Simplification rules: |-x| = |x|, |x²| = x², |a*b| = |a|*|b|, |x/y| = |x|/|y|
  6. Add .abs() method to Expression API
  7. Integrate with UniversalFunctionRegistry (O(1) lookup)
  8. 10-12 comprehensive tests with 100% SymPy validation
  9. Production-quality documentation with runnable examples
- Deliverables:
  - functions/elementary/abs.rs (≤500 lines, target ~150-200)
  - tests/abs_tests.rs with 10-12 content validation tests
  - API integration (.abs() method)
  - Registry integration
  - Documentation with examples and doctests
  - Verification report with 10/10 quality score

Wave 2: Square Root Function √x (3-4 hours)
- Scope: Enhanced sqrt from x^(1/2) with better domain handling
- Priority: HIGH (foundation for radicals, complex roots, Gamma function)
- Objectives:
  1. Create functions/elementary/sqrt.rs (~200-250 lines) with complete function intelligence
  2. Implement derivative: d/dx√x = 1/(2√x) for x > 0, undefined at x=0
  3. Implement antiderivative: ∫√x dx = (2/3)x^(3/2) + C
  4. Domain handling: [0,∞) for real, ℂ for complex (with branch cut on negative real axis)
  5. Simplification rules: √(x²) = |x|, √(ab) = √a·√b (non-negative), √(x⁴) = x², √(-1) = i (complex)
  6. LaTeX output: \sqrt{x} instead of x^{1/2}
  7. Add .sqrt() method to Expression API
  8. Integrate with UniversalFunctionRegistry
  9. 10-12 comprehensive tests with 100% SymPy validation
  10. Production-quality documentation with runnable examples
- Deliverables:
  - functions/elementary/sqrt.rs (≤500 lines, target ~200-250)
  - tests/sqrt_tests.rs with 10-12 content validation tests
  - API integration (.sqrt() method)
  - Registry integration
  - LaTeX formatting enhancement
  - Documentation with examples and doctests
  - Verification report with 10/10 quality score

Wave 3: Polynomial Division Public API Enhancement (2-3 hours)
- Scope: Polish existing polynomial division with better API and documentation
- Priority: MEDIUM-HIGH (usability improvement for existing complete implementation)
- Objectives:
  1. Enhance documentation in algebra/polynomial_division.rs (already complete implementation)
  2. Add trait convenience methods to PolynomialGcd trait:
     - .div_polynomial(divisor, var) → (quotient, remainder)
     - .quo_polynomial(divisor, var) → quotient
     - .rem_polynomial(divisor, var) → remainder
  3. Create examples/polynomial_division_usage.rs with comprehensive usage examples
  4. Write tests/polynomial_division_api_tests.rs (10+ tests for new API surface)
  5. Update algebra/mod.rs documentation with polynomial division section
  6. Production-quality module docs with runnable examples
- Deliverables:
  - Enhanced documentation for polynomial_division.rs
  - Trait convenience methods in algebra/gcd.rs
  - examples/polynomial_division_usage.rs (~100-150 lines)
  - tests/polynomial_division_api_tests.rs with 10+ API tests
  - Updated module documentation
  - Verification report with 10/10 quality score

Target Metrics:
- Quality Score: 10/10 per wave (PERFECT execution - no excuse for less on small scope)
- Test Count: Add 30-36 new content validation tests (10-12 + 10-12 + 10-12)
- SymPy Validation: 100% of mathematical operations validated against SymPy
- Build: Zero errors, zero warnings (cargo check + cargo clippy)
- Regressions: Zero (all 514+ existing tests must pass)
- CLAUDE.md: 100% compliance (new work)
- File Sizes: All new files ≤250 lines (small scope enables perfection)
- Documentation: Production-quality with runnable doctests
- Performance: No performance regressions

Success Criteria (10/10 Quality Checklist):
1. ✅ All files ≤500 lines (target: ≤250 lines per file)
2. ✅ Zero emojis in source code
3. ✅ Build: 0 errors, 0 warnings (check + clippy)
4. ✅ All new tests pass (30-36 tests)
5. ✅ Zero regressions (514+ existing tests pass)
6. ✅ 100% SymPy validation for all operations
7. ✅ 100% content validation (no structure-only tests)
8. ✅ Documentation complete with examples
9. ✅ All doctests compile and pass
10. ✅ Registry integration verified (O(1) lookup)

Mathematical Correctness Emphasis:
- Every operation validated against SymPy
- Edge cases thoroughly tested (zero, infinity, complex)
- Domain restrictions documented and enforced
- No approximations unless explicitly documented
- Exact arithmetic preferred (rationals over floats)

Standard orchestration protocol:
- You are orchestrator, maintain momentum
- Create verification scripts per wave (verify_wave_N_[name].sh with 10 categories)
- Launch ONE agent per wave (scope is small, single agent sufficient)
- Verify everything before declaring complete
- Create comprehensive verification reports
- Track with TodoWrite
- Use SymPy for mathematical validation
- Final bundle completion report with quality assessment

Let's begin with Wave 1: Absolute Value Function
```

---

## What This Command Achieves

### Wave 1: Absolute Value |x| (3-4 hours)
**Critical**: Foundation for complex modulus and simplification
**Implementation**: New elementary function with full intelligence
**Output**: Working |x| with derivative, integral, simplification
**Quality Target**: 10/10 (flawless)

### Wave 2: Square Root √x (3-4 hours)
**Critical**: Foundation for radicals and Gamma function
**Implementation**: Enhanced from x^(1/2) with domain handling
**Output**: Working √x with better LaTeX, domain checking
**Quality Target**: 10/10 (flawless)

### Wave 3: Polynomial Division API (2-3 hours)
**Critical**: Usability improvement for existing feature
**Implementation**: Trait methods + documentation + examples
**Output**: Ergonomic API for polynomial division
**Quality Target**: 10/10 (flawless)

---

## Key Architectural Decisions Baked Into This Plan

### 1. Small Scope = Perfect Execution
- Each wave is 150-250 lines implementation
- Single agent per wave (no parallel needed)
- Achievable 10/10 quality

### 2. Foundation for Roadmap
- Wave 1 & 2: Prerequisites for Gamma function (Month 1, Weeks 2-4)
- All 3: Foundation for integration (Months 2-3)
- Sets quality bar for future work

### 3. Leverage Existing Architecture
- Follow patterns from existing elementary functions (sin, cos, exp, log)
- Use UniversalFunctionRegistry (O(1) lookup)
- Integrate with Expression system

### 4. Mathematical Correctness First
- SymPy validation required for all operations
- Edge cases mandated (zero, infinity, complex)
- Domain restrictions must be documented

### 5. CLAUDE.md Compliance
- Max 500 lines/file enforced (target ≤250)
- No emojis, proper documentation
- Build must pass with zero warnings
- Content validation tests required

---

## Files The Orchestrator Will Reference

Roadmap and context:
1. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/NEXT_PRIORITIES_ROADMAP.md` - Month 1, Week 1 details
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/VERIFICATION_COMPLETE.md` - Current status
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/POLYNOMIAL_ARCHITECTURE_AUDIT.md` - Quality standards

Orchestration guidance:
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven methodology
5. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/EDUCATIONAL_WAVE_5_VERIFICATION_REPORT.md` - Verification example

Reference implementations:
6. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/functions/elementary/*.rs` - Pattern to follow (sin, cos, exp, log)
7. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs` - Existing complete implementation for Wave 3

SymPy reference:
8. `~/Documents/work/math/sympy/` - Primary validation source

---

## Expected Timeline

- **Wave 1**: 3-4 hours (abs implementation + verification)
- **Wave 2**: 3-4 hours (sqrt implementation + verification)
- **Wave 3**: 2-3 hours (API polish + verification)

**Total**: 8-11 hours of agent work
**With orchestration overhead**: ~12-15 hours total

**Sequential execution**: Cannot parallelize (each wave is small, single-agent)

---

## Quality Targets (10/10 Standard)

| Metric | Target | Rationale |
|--------|--------|-----------|
| Quality Score | 10/10 per wave | Small scope enables perfection |
| Tests Added | 30-36 total | 10-12 per wave, 100% content validation |
| Content Validation | 100% | No structure-only tests allowed |
| CLAUDE.md Compliance | 100% | Zero tolerance for violations |
| Regressions | 0 | All 514+ tests must pass |
| SymPy Validation | 100% | Every operation verified |
| File Sizes | ≤250 lines | Target (max 500 hard limit) |
| Build Warnings | 0 | cargo check + clippy clean |

---

## Success Definition

At the end of Wave 3, we should have:

```rust
// Wave 1: Absolute Value - WORKING
use mathhook_core::{expr, symbol};

let result = expr!(abs(-5)).simplify();
assert_eq!(result, expr!(5));                    // ✅ NEW

let x = symbol!(x);
let abs_neg_x = expr!(abs(-x)).simplify();
assert_eq!(abs_neg_x, expr!(abs(x)));            // ✅ NEW

// Wave 2: Square Root - WORKING
let sqrt_4 = expr!(sqrt(4)).simplify();
assert_eq!(sqrt_4, expr!(2));                    // ✅ NEW

let sqrt_x_squared = expr!(sqrt(x^2)).simplify();
assert_eq!(sqrt_x_squared, expr!(abs(x)));       // ✅ NEW (not x!)

// Wave 3: Polynomial Division API - WORKING
use mathhook_core::algebra::PolynomialGcd;

let dividend = expr!(x^2 - 1);
let divisor = expr!(x - 1);
let (quot, rem) = dividend.div_polynomial(&divisor, &x);
assert_eq!(quot, expr!(x + 1));                  // ✅ NEW (API)
assert_eq!(rem, expr!(0));                       // ✅ NEW (API)
```

---

## Why This Plan Will Achieve 10/10

1. **Proven methodology** - Educational Waves 1-5 (100% success, 8.5/10 average)
2. **Small scope** - Each wave ≤250 lines (easy to perfect)
3. **Clear requirements** - SymPy provides ground truth
4. **Existing patterns** - Can copy-adapt from sin, cos, exp, log
5. **No complexity** - No algorithms to design (abs/sqrt are well-understood)
6. **Strict verification** - 10-category verification per wave
7. **Foundation focus** - Building on 9.25/10 polynomial work
8. **CLAUDE.md enforced** - Quality and standards maintained

---

**This orchestrator command is ready to use. Copy the bootstrap block and goal statement into a new Claude Code session.**
