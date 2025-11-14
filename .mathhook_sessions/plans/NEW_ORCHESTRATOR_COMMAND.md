# New Orchestrator Command Template

**AI-Optimized Bootstrap Command** - Use this template when creating orchestration plans for AI agents.

---

## 🤖 For AI Planning Agents: How to Use This Template

**Purpose**: This template creates orchestrator bootstrap commands for new plans. When creating a new `PLAN_X_ORCHESTRATOR_COMMAND.md`, use this template and fill in the plan-specific details.

**Token Efficiency**: This template is designed to minimize token usage while maximizing AI agent effectiveness.

---

## Copy-Paste Bootstrap Command (Customize Sections Marked with [BRACKETS])

```
You are the Orchestrator for [PLAN NAME].

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

4. Read: [PLAN-SPECIFIC INVESTIGATION FILE]
   - Example: `.mathhook_sessions/[PLAN_NAME]_SPEC.md`
   - Contains: Root cause analysis, mathematical proofs, verified findings
   - Token budget: ~3K tokens

5. Read: [PLAN-SPECIFIC ORCHESTRATION PLAN]
   - Example: `.mathhook_sessions/[PLAN_NAME]_ORCHESTRATION.md`
   - Contains: Complete plan with phases, waves, dependencies
   - Token budget: ~2K tokens

🟢 REFERENCE (Only if needed - ~3K tokens):

6. Reference: .mathhook_sessions/reports/EDUCATIONAL_SYSTEM_0.1_READY.md
   - Latest release status and quality benchmarks
   - Use for understanding current system state

7. Reference: .mathhook_sessions/waves/WAVE_*.md
   - Past wave examples for patterns and approaches
   - Use only if similar work exists

---

⚡ MANDATORY ORCHESTRATION RULES (Non-Negotiable)

1. You Are Always The Orchestrator
   ✅ You plan, launch agents, verify, and make ALL decisions
   ✅ Agents execute specific tasks; you maintain control and continuity
   ✅ NEVER delegate orchestration responsibilities to agents
   ❌ Don't let agents take over orchestration

2. Sequential Waves, Parallel Agents
   ✅ Work proceeds: Wave 1 → verify → Wave 2 → verify → ...
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
- 🔴 Manual proofs: [PLAN-SPECIFIC VERIFICATION FILE]
- 🔴 ALWAYS verify against manual calculus/mathematics

Testing Standards:
✅ Test edge cases: zero, infinity, undefined, complex numbers
✅ Test mathematical properties: [DOMAIN-SPECIFIC PROPERTIES]
✅ Test domain boundaries and restrictions
✅ Validate against manual mathematical proofs
❌ Never trust output without mathematical verification

---

📋 INVESTIGATION STATUS

[CUSTOMIZE THIS SECTION WITH PLAN-SPECIFIC DETAILS]

Example:
✅ All [N] tests/issues analyzed with verified root causes
✅ Mathematical proofs validated for all expected results
✅ Verification playgrounds executed:
   - [FILE 1] - [WHAT IT VERIFIED]
   - [FILE 2] - [WHAT IT VERIFIED]
✅ Root cause identified: [SUMMARY]

---

✅ CONFIRMATION CHECKLIST

After reading all files above, respond with:

1. "I have read and understood the orchestration methodology from Educational Waves 1-5"
2. "I have reviewed the [PLAN NAME] investigation and understand all [N] root causes/objectives"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the [N] phases with their wave counts
5. Confirm verification script exists at `.mathhook_sessions/scripts/verify_[wave_name].sh`
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

## 🎯 Goal Statement Template (Provide After Orchestrator Confirms)

```
The goal is: [CLEAR OBJECTIVE STATEMENT]

Context: [BRIEF BACKGROUND]
- [KEY POINT 1]
- [KEY POINT 2]
- [KEY POINT 3]

Structure - [N] Phases, [M] Waves Following [STRATEGY NAME]:

Phase 1: [PHASE NAME] ([TIMEFRAME])
Wave [X.Y]: [WAVE NAME] - [BRIEF DESCRIPTION] ([DURATION])
- Scope: [WHAT THIS WAVE DOES]
- Priority: [CRITICAL|HIGH|MEDIUM|LOW]
- Objectives:
  1. [OBJECTIVE 1]
  2. [OBJECTIVE 2]
  3. [OBJECTIVE 3]
- Deliverables:
  - [DELIVERABLE 1]
  - [DELIVERABLE 2]
  - Verification report with score ≥90/100

[REPEAT FOR EACH WAVE]

Target Metrics:
- Quality Score: 90+/100 per wave
- Test Count: [TARGET] tests passing
- Build: Zero errors, zero regressions
- CLAUDE.md: 100% compliance
- Verification: All scripts score ≥90/100

Success Criteria:
1. ✅ [CRITERION 1]
2. ✅ [CRITERION 2]
3. ✅ [CRITERION 3]
[...]
N. ✅ [CRITERION N]

Start with Wave [X.Y] immediately after confirmation.
```

---

## 📂 File Organization for This Plan

When creating a new orchestration plan, create these files:

### Investigation Phase
- `.mathhook_sessions/plans/PLAN_[N]_[NAME]_SPEC.md` - Technical analysis
- `.mathhook_sessions/plans/PLAN_[N]_STATUS.md` - Current status
- Playground verification files (if needed)

### Orchestration Phase
- `.mathhook_sessions/plans/PLAN_[N]_ORCHESTRATOR_COMMAND.md` - This file (from template)
- `.mathhook_sessions/plans/PLAN_[N]_[NAME]_ORCHESTRATION.md` - Detailed plan
- `.mathhook_sessions/scripts/verify_wave_[X]_[Y].sh` - Verification scripts (one per wave)

### Execution Phase
- `.mathhook_sessions/waves/WAVE_[X]_[Y].md` - Wave documentation
- `.mathhook_sessions/reports/WAVE_[X]_[Y]_VERIFICATION_REPORT.md` - Verification reports

---

## 🔄 How to Use This Template

### For Planning Agents (Creating New Orchestration Plans)

1. **Copy this entire file** as starting point
2. **Fill in bracketed sections** with plan-specific details:
   - `[PLAN NAME]` → Actual plan name
   - `[N]` → Number of issues/tests/objectives
   - `[PLAN_NAME]_SPEC.md` → Actual investigation file
   - Phase/Wave structure with actual tasks
   - Success criteria with actual metrics

3. **Customize reading sequence** based on plan needs:
   - Add plan-specific investigation files to 🟡 section
   - Remove irrelevant reference files from 🟢 section
   - Update token budgets if files are larger/smaller

4. **Create verification scripts** for each wave before writing orchestrator command

5. **Validate against examples**:
   - See `.mathhook_sessions/plans/PLAN_10_ORCHESTRATOR_COMMAND.md` for complete example
   - See `.mathhook_sessions/waves/` for wave documentation examples

### For Orchestrators (Using Generated Commands)

1. **Copy the bootstrap command block** (between triple backticks)
2. **Paste into new Claude Code session**
3. **Read all files in sequence** (follow priority markers)
4. **Confirm understanding** (checklist items)
5. **Wait for goal** from user
6. **Begin orchestration** following methodology

---

## ✅ Quality Standards for Orchestrator Commands

When creating a new orchestrator command, ensure:

- [ ] All file paths are absolute and correct
- [ ] Priority markers (🔴🟡🟢) used consistently
- [ ] Token budgets estimated for each reading section
- [ ] Investigation status accurately reflects completion
- [ ] Success criteria are measurable and specific
- [ ] Verification scripts mentioned are created
- [ ] Goal statement is clear and actionable
- [ ] Follows proven methodology from Educational Waves 1-5

---

## 📚 Examples and References

### Complete Example
See: `.mathhook_sessions/plans/PLAN_10_ORCHESTRATOR_COMMAND.md`
- Shows all sections filled in
- Demonstrates token budgeting
- Includes complete investigation status
- Has 4 phases, 6 waves structure

### Methodology Reference
See: `.mathhook_sessions/plans/ORCHESTRATION_METHODOLOGY.md`
- Section: "Agent Prompt Template" (line 305)
- Section: "Verification Script Template" (line 206)
- Section: "Wave Template" (line 127)

### Past Waves
Browse: `.mathhook_sessions/waves/WAVE_*.md`
- See how waves were documented
- Understand verification patterns
- Learn from past successes

---

## 💡 Tips for Effective Orchestration Commands

### Do's ✅
- Use priority markers (🔴🟡🟢) to guide AI reading
- Estimate token budgets for each section
- Provide concrete examples in templates
- Include verification scripts before launching
- Reference proven methodology explicitly
- Make success criteria measurable

### Don'ts ❌
- Don't assume AI knows project structure
- Don't skip token efficiency guidance
- Don't omit verification requirements
- Don't use vague success criteria
- Don't forget CLAUDE.md enforcement
- Don't leave out mathematical validation

---

## 🚀 Bootstrap Workflow Summary

```
1. Planning Agent creates investigation
   └─ Outputs: SPEC.md, STATUS.md, verification playgrounds

2. Planning Agent uses THIS TEMPLATE
   └─ Fills in plan-specific details
   └─ Creates: PLAN_[N]_ORCHESTRATOR_COMMAND.md

3. Planning Agent creates verification scripts
   └─ Outputs: verify_wave_*.sh files

4. User copies bootstrap command
   └─ Pastes into new Claude Code session

5. Orchestrator reads files (following priority markers)
   └─ Total: ~20K tokens for orientation

6. Orchestrator confirms understanding
   └─ User provides goal statement

7. Orchestrator begins Wave 1
   └─ Following proven methodology
```

---

**Template Version**: 2.0 (AI-Optimized)
**Last Updated**: 2024-11-14
**Based On**: Educational Waves 1-5 (100% success rate)
**Enhancements**: Priority markers, token efficiency, decision trees, checklists
**Status**: READY FOR USE

---

## 🔗 Related Files

- **Methodology**: `.mathhook_sessions/plans/ORCHESTRATION_METHODOLOGY.md`
- **Navigation**: `.mathhook_sessions/README.md`
- **Example**: `.mathhook_sessions/plans/PLAN_10_ORCHESTRATOR_COMMAND.md`
- **Project Rules**: `/CLAUDE.md` (project root)
