# Next Orchestrator Bootstrap Command

## Copy-Paste This Entire Block Into New Claude Code Session

```
You are the Orchestrator for this MathHook development session.

CRITICAL FIRST STEP - Read these files in order and line by line to learn the proven methodology:

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - This is the SINGLE SOURCE OF TRUTH for all development rules
   - Contains architectural constraints, coding standards, and non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/README.md
   - Navigation guide to all session documents
   - Overview of what's available and where to find it

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md
   - Complete orchestration methodology from Educational Waves 1-5
   - Contains wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results

4. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/VERIFICATION_COMPLETE.md
   - Recent completion: Number theory & polynomial work (9.25/10 quality)
   - Shows current project status (514 tests passing, 70-75% SymPy coverage)

5. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/NEXT_PRIORITIES_ROADMAP.md
   - Strategic 6-month development roadmap
   - Tier 1: Integration + ODEs, Tier 2: Gamma + quick wins, Tier 3: Cubic/quartic
   - Shows what to build next and why

6. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/POLYNOMIAL_ARCHITECTURE_AUDIT.md
   - Recent audit showing polynomial work is 9.0/10 quality, production-ready
   - Demonstrates quality standards and architectural review process

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

CONFIRMATION REQUIRED:

After reading all files above line by line, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. Summarize the 5 mandatory orchestration rules in your own words
3. List the 5 phases of a standard wave (Planning, Agent Execution, Verification, Reporting, Decision)
4. Acknowledge current project status: "514 tests passing, 9.25/10 recent quality, polynomial work complete"
5. Say: "I am ready to orchestrate. What is the goal?"

Then WAIT for the user to provide the goal.

DO NOT proceed with any work until you have:
- Read all required files
- Confirmed understanding
- Received the goal from the user
```

---

## When Orchestrator Confirms, Provide This Goal

```
The goal is: Quick Wins Bundle - Elementary Functions Foundation (Month 1, Week 1 of roadmap)

Structure:
- Wave 1: Absolute Value Function |x| with full function intelligence
- Wave 2: Square Root Function √x enhanced from x^(1/2)
- Wave 3: Polynomial Division Public API polish

Target:
- Build 3 high-value, low-effort features
- Achieve 10/10 quality on ALL metrics
- ZERO regressions (maintain 514+ tests passing)
- 30+ new content validation tests
- 100% SymPy validation
- Production-quality documentation

Success Criteria (STRICT - 10/10 means PERFECT):
- Quality Score: 10/10 minimum per wave (flawless implementation)
- Tests: 30-36 total (10-12 per wave), 100% content validation
- CLAUDE.md: 100% compliance (files ≤500 lines, zero emojis, comprehensive docs)
- Build: 0 errors, 0 warnings (cargo check + cargo clippy)
- Regressions: Zero (all 514 existing tests must pass)
- Mathematical Correctness: 100% SymPy validation for all operations
- Documentation: Production-quality with runnable examples and doctests
- Integration: Seamless with UniversalFunctionRegistry (O(1) lookup)
- File Sizes: All new files ≤250 lines (small scope = perfect execution)
- Verification: All 10 categories must pass per wave

Wave Details:

Wave 1 - Absolute Value |x| (3-4 hours):
- File: functions/elementary/abs.rs (~150-200 lines)
- Properties: derivative d/dx|x| = x/|x|, integral ∫|x|dx = x|x|/2 + C
- Domain: ℝ (real), ℂ (complex with |a+bi| = √(a²+b²))
- Simplification: |-x| = |x|, |x²| = x², |a*b| = |a|*|b|
- Tests: 10-12 comprehensive tests with SymPy validation
- API: Add .abs() method to Expression

Wave 2 - Square Root √x (3-4 hours):
- File: functions/elementary/sqrt.rs (~200-250 lines)
- Enhanced from x^(1/2) with better domain handling
- Properties: derivative d/dx√x = 1/(2√x), integral ∫√x dx = (2/3)x^(3/2) + C
- Domain: [0,∞) for real, ℂ for complex (with branch cut)
- Simplification: √(x²) = |x|, √(ab) = √a·√b, √(-1) = i (complex)
- LaTeX output: \sqrt{x} instead of x^{1/2}
- Tests: 10-12 comprehensive tests with SymPy validation
- API: Add .sqrt() method to Expression

Wave 3 - Polynomial Division API (2-3 hours):
- Enhance: algebra/polynomial_division.rs documentation
- Add: Trait convenience methods to PolynomialGcd trait
  - .div_polynomial(divisor, var) → (quotient, remainder)
  - .quo_polynomial(divisor, var) → quotient
  - .rem_polynomial(divisor, var) → remainder
- Create: examples/polynomial_division_usage.rs with comprehensive examples
- Tests: 10+ API usage tests
- Documentation: Production-quality module docs with examples

Orchestration Protocol:
- Create verification script BEFORE each wave (10 categories: file size, emojis, build, tests, regressions, SymPy, content validation, docs, registry, math correctness)
- Launch ONE agent per wave (scope is small enough)
- Agent prompts MUST include: exact file paths, line count targets, CLAUDE.md requirements, SymPy validation examples, verification script reference
- Verify EVERYTHING before moving to next wave
- Track with TodoWrite throughout
- Create comprehensive verification report per wave
- Final bundle completion report with 10/10 quality assessment

Expected Timeline:
- Wave 1: 4-5 hours (implementation + verification)
- Wave 2: 4-5 hours (implementation + verification)
- Wave 3: 3-4 hours (implementation + verification)
- Total: 12-15 hours (~2 focused work days)

This is the foundation for Gamma function (Month 1, Weeks 2-4) and integration work (Months 2-3).
```

---

## What This Achieves

✅ **Uses proven command format** from NEW_ORCHESTRATOR_COMMAND.md
✅ **Forces methodology learning** - Orchestrator reads all reference docs first
✅ **Establishes authority** - Orchestrator knows they're in charge
✅ **Sets quality bar** - 10/10 target is explicit and non-negotiable
✅ **Provides strategic context** - Roadmap alignment is clear
✅ **Strict success criteria** - No ambiguity about what "done" means
✅ **Small wave scope** - 3-4 hours per wave enables perfect execution

---

## File Location

Saved in: `.mathhook_sessions/NEXT_ORCHESTRATOR_BOOTSTRAP.md`
