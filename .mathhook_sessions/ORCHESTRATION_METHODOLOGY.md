# MathHook Orchestration Methodology

**Version**: 1.0
**Status**: Production-Proven (Educational Waves 1-5)
**Success Rate**: 100% (5/5 waves completed with excellence)

---

## Purpose

This document captures the proven orchestration methodology used for Educational Waves 1-5. Use this methodology for any large, multi-phase work (0.2 release, new features, major refactors, etc.).

---

## Quick Start: Command Template

To initiate orchestrated work for a new goal, use this command:

```
Let's orchestrate [GOAL NAME] using the standard methodology:

Wave Structure: [Describe waves/phases]
Target: [What are we building/achieving]
Success Criteria: [Key metrics - quality score, test count, compliance, etc.]

I want you as orchestrator to:
- Create verification scripts for each wave
- Launch agents with strict CLAUDE.md enforcement
- Do all necessary verifications and logging
- Always confirm verifications before declaring complete
- Maintain momentum between waves

Let's go with Wave 1.
```

**Example** (for 0.2 release):
```
Let's orchestrate Educational System 0.2 Release using the standard methodology:

Wave Structure:
- Wave 1: Advanced Integration Techniques
- Wave 2: Multivariable Calculus Basics
- Wave 3: Differential Equations
- Wave 4: Testing & QA

Target: Expand educational coverage from 80% to 95%
Success Criteria: 8+/10 quality, 150+ total tests, CLAUDE.md compliance

I want you as orchestrator to:
- Create verification scripts for each wave
- Launch agents with strict CLAUDE.md enforcement
- Do all necessary verifications and logging
- Always confirm verifications before declaring complete
- Maintain momentum between waves

Let's go with Wave 1.
```

---

## Orchestration Principles (MANDATORY)

### 1. You Are Always The Orchestrator

**Never forget**: You (Claude) are the orchestrator. You manage:
- Wave planning and sequencing
- Agent launching and management
- Verification script creation and execution
- CLAUDE.md enforcement
- Progress tracking and reporting
- Final approval/rejection decisions

**Agents are autonomous but you verify everything.**

### 2. Sequential Waves, Parallel Agents

**Wave Structure**:
- Waves execute sequentially (Wave 1 → Wave 2 → Wave 3...)
- Agents within a wave can run in parallel (Agent 2A || Agent 2B)
- Never start next wave until current wave is verified

**Example**:
```
Wave 1 (Foundation) → VERIFY → Wave 2 (Implementation) → VERIFY → Wave 3 (QA)
    ↓                              ↓
Agent 1A || Agent 1B          Agent 2A || Agent 2B || Agent 2C
```

### 3. Mandatory Verification

**NEVER declare a wave complete without**:
1. Creating a verification script
2. Running the verification script
3. Creating a verification report
4. Confirming all success criteria met

**If verification fails**: Launch continuation agent or reject work.

### 4. Strict CLAUDE.md Enforcement

**Between you and agents**:
- Tell agents explicitly: "You must follow CLAUDE.md strictly"
- Include CLAUDE.md requirements in every agent prompt
- Verify compliance in verification scripts
- Reject work that violates CLAUDE.md

**Key CLAUDE.md Rules to Enforce**:
- Maximum 500 lines per file
- No emojis anywhere (zero tolerance)
- Documentation style (`//!` module, `///` items, minimal `//` inline)
- No `todo!()` macros in production code
- Build must pass with 0 errors
- No stubs or "for now" or "not yet implemented"

### 5. Maintain Momentum

**User wants momentum**:
- Don't ask "should we proceed?" between waves
- Complete verification → immediately launch next wave
- Keep user informed with progress updates
- Only stop if critical issues found

**Balance**: Speed with quality (never sacrifice quality for speed)

---

## Standard Wave Template

Every wave follows this structure:

### Phase 1: Planning (Orchestrator)

**Actions**:
1. Define wave scope and goals
2. Identify agents needed (1-3 agents per wave)
3. Create verification script (before launching agents)
4. Define success criteria
5. Update todo list

**Outputs**:
- Wave plan
- Verification script (`.mathhook_sessions/verify_[wave_name].sh`)
- Success criteria checklist

### Phase 2: Agent Execution (Agents)

**Actions**:
1. Launch agent(s) with comprehensive prompt
2. Monitor progress (check agent hasn't hung)
3. Let agents work autonomously
4. Agents report completion

**Agent Prompt Must Include**:
- Wave number and name
- Exact scope (what to implement)
- CLAUDE.md requirements (explicit)
- Success criteria
- Testing requirements
- Reference to verification script
- Reporting template

### Phase 3: Verification (Orchestrator)

**Actions**:
1. Run verification script
2. Analyze results
3. Identify any issues
4. Create verification report

**Verification Script Categories** (8-10):
- File size compliance
- Emoji compliance
- Build status
- Test pass rate
- Test count (if applicable)
- Content validation ratio
- Implementation completeness
- Quality indicators

### Phase 4: Reporting (Orchestrator)

**Actions**:
1. Create comprehensive verification report
2. Document metrics (before/after)
3. Assign quality score
4. Identify technical debt
5. Make recommendation (approve/reject/continue)

**Report Structure**:
- Executive summary
- Agent-by-agent verification
- Success criteria evaluation
- Files modified summary
- Quality assessment
- Lessons learned
- Recommendation

### Phase 5: Decision (Orchestrator)

**Approve**: Wave meets all criteria → Proceed to next wave
**Reject**: Wave has critical issues → Refuse to proceed
**Continue**: Wave incomplete → Launch continuation agent

---

## Verification Script Template

Every wave needs a verification script. Template:

```bash
#!/bin/bash

# [WAVE NAME] Verification Script
# [PURPOSE]
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "[WAVE NAME] VERIFICATION"
echo "[DESCRIPTION]"
echo "========================================"

FAILURES=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# CATEGORY 1: FILE SIZE VIOLATIONS
echo "========================================"
echo "CATEGORY 1: FILE SIZE VIOLATIONS"
echo "CLAUDE.md: Maximum 500 lines per file"
echo "========================================"

# Check modified files
# [Implementation]

if [ $VIOLATIONS -eq 0 ]; then
    echo -e "${GREEN}✓ All files comply${NC}"
else
    echo -e "${RED}✗ $VIOLATIONS violations${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "CLAUDE.md: No emojis in code"
echo "========================================"

EMOJI_COUNT=$(grep -r "✅\|❌\|⚠️\|🚀\|✨" [paths] 2>/dev/null | wc -l)

if [ "$EMOJI_COUNT" -gt 0 ]; then
    echo -e "${RED}✗ Found $EMOJI_COUNT emojis${NC}"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ No emojis found${NC}"
fi

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Must compile successfully"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)

if echo "$BUILD_OUTPUT" | grep -q "Finished"; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build has errors${NC}"
    FAILURES=$((FAILURES + 1))
fi

# CATEGORY 4: TEST VALIDATION
echo "========================================"
echo "CATEGORY 4: TEST VALIDATION"
echo "[TEST REQUIREMENTS]"
echo "========================================"

# Run tests
# [Implementation]

# ... Add 5-10 categories total ...

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo "[WAVE NAME] is VERIFIED COMPLETE"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED: $FAILURES issue(s)${NC}"
    echo "[WAVE NAME] requires fixes before approval"
    exit 1
fi
```

---

## Agent Prompt Template

Every agent prompt must include these sections:

```markdown
# [WAVE NAME]: [AGENT PURPOSE]

## Mission: [One-sentence goal]

You are [AGENT ID] within [WAVE NAME]. Your mission is [detailed goal].

## Critical Context

**Your Identity**: You are [AGENT ID] within [WAVE NAME]
**Orchestrator**: I am the orchestrator managing sequential waves with parallel agents
**CLAUDE.md Enforcement**: MANDATORY compliance - the orchestrator WILL verify strictly

### Current Progress (Previous Waves)
[Summary of what's been done]

### Current Status
[Where we are now]

## Your Scope: [TASK NAME]

### Primary Deliverables
1. [Deliverable 1]
2. [Deliverable 2]
3. [Deliverable 3]

[Detailed requirements with examples]

## CLAUDE.md Compliance Requirements (STRICTLY ENFORCED)

### File Size
- **Maximum 500 lines per file**
- Check: `wc -l <file>.rs`
- If over, split immediately

### No Emojis
- **Zero tolerance**
- Check: `grep -r "✅\|❌\|⚠️" <file>`

### Documentation
- `//!` for module-level only
- `///` for items only
- Minimize inline `//`

### Build
- Must compile: `cargo check -p mathhook-core` with 0 errors

### No Placeholders
- No `todo!()` macros
- Every function fully implemented

## Success Criteria (MANDATORY)

1. ✅ [Criterion 1]
2. ✅ [Criterion 2]
3. ✅ [Criterion 3]
...

## Verification Protocol

When you complete, the orchestrator WILL run:
```bash
bash .mathhook_sessions/verify_[wave_name].sh
```

**If verification fails**, the orchestrator will launch a continuation agent or reject the work.

## Execution Protocol

1. [Step 1]
2. [Step 2]
3. [Step 3]
...

## Reporting Template

When complete, provide this report:

```markdown
# [AGENT ID]: [TASK NAME] - COMPLETE

## [Section 1]
[Results]

## [Section 2]
[Results]

...

## Verification Results

**Local Verification**:
- [Check 1]: ✅/❌
- [Check 2]: ✅/✅
...

**Ready for orchestrator verification**: YES/NO
```

## Important Notes

1. **You are NOT the orchestrator** - you are [AGENT ID], a specialized agent
2. **Focus ONLY on [scope]** - don't modify unrelated code
3. **Quality over speed** - [quality bar]
4. **CLAUDE.md is law** - 100% compliance required

## Begin Implementation

[Start instructions]

**Return your final report** when all success criteria are met.
```

---

## Verification Report Template

After verification, create this report:

```markdown
# [WAVE NAME] Complete Verification Report

**Date**: [DATE]
**Orchestrator**: Claude Code
**Verification Protocol**: MANDATORY with custom verification script
**Enforcement**: Strict CLAUDE.md compliance

---

## Executive Summary

[Status] **VERIFIED [COMPLETE/FAILED]**: [One sentence summary]

**Result**: [Summary of what was delivered]

---

## [Wave Name] Journey

### [Agent ID]: [Agent Name] [✅/❌]
- **Scope**: [What they were supposed to do]
- **Delivered**: [What they actually delivered]
- **Status**: [COMPLETE/PARTIAL/FAILED]
- **Quality**: [Score]/10

---

## Final Verified Metrics

| Metric | Before [Wave] | After [Wave] | Change | Status |
|--------|---------------|--------------|--------|--------|
| [Metric 1] | [Before] | [After] | [Change] | [Status] |
| [Metric 2] | [Before] | [After] | [Change] | [Status] |

---

## Verification Script Output

[Paste script output or summarize results]

### Category 1: [Category Name] [✅/❌]
[Results]

### Category 2: [Category Name] [✅/❌]
[Results]

[Continue for all categories]

---

## Agent Verification [✅/❌]

**Claimed**: [What agent claimed to deliver]

**Verified**: [What was actually verified]
- ✅/❌ [Item 1]
- ✅/❌ [Item 2]
...

**Quality**: [Score]/10 - [Justification]

---

## CLAUDE.md Enforcement Results

### Orchestrator Actions Taken
1. [Action 1]
2. [Action 2]
...

### Agent Compliance
- ✅/❌ [Compliance item 1]
- ✅/❌ [Compliance item 2]
...

### CLAUDE.md Violations Found
**Critical**: [Count]
**Major**: [Count]
**Minor**: [Count]

---

## Implementation Quality Assessment

[Detailed quality analysis with scores]

---

## Files Modified Summary

### Created ([Count] new files)
1. [File 1] ([lines] lines)
2. [File 2] ([lines] lines)

### Modified ([Count] files)
1. [File 1] - [Description]
2. [File 2] - [Description]

---

## Success Criteria Evaluation

| Criterion | Target | Actual | Status |
|-----------|--------|--------|--------|
| [Criterion 1] | [Target] | [Actual] | [Status] |
...

---

## Lessons Learned

### What Worked Well ✅
1. [Lesson 1]
2. [Lesson 2]

### What Could Improve ⚠️
1. [Issue 1]
2. [Issue 2]

### Orchestrator Improvements Applied 🎯
1. [Improvement 1]
2. [Improvement 2]

---

## Conclusion

[Status] **[WAVE NAME] VERIFIED [COMPLETE/FAILED]**

### Recommendation

[APPROVED/REJECTED/CONTINUATION NEEDED]

[Justification]

---

**Verification Date**: [DATE]
**Verified By**: Claude Code (Orchestrator)
**Confidence Level**: [HIGH/MEDIUM/LOW] [✅/⚠️/❌]
**Status**: [FINAL STATUS]
```

---

## Quality Score Guidelines

Assign honest quality scores (1-10) to all work:

- **10/10**: Perfect implementation (rare)
- **9/10**: Excellent, minor improvements possible
- **8/10**: Production-ready, solid work (MINIMUM TARGET)
- **7/10**: Good, some improvements needed
- **6/10**: Functional but needs work
- **5/10 or below**: Needs significant rework

**Target**: 8+/10 average for production release

**Factors**:
- Code quality and structure
- Test coverage and quality
- Documentation completeness
- CLAUDE.md compliance
- Mathematical correctness (if applicable)
- Integration with existing systems

---

## Testing Standards

### Content Validation Tests (MANDATORY)

**Pattern**:
```rust
#[test]
fn test_[operation]_[aspect]() {
    // Setup
    let result = perform_operation();

    // ✅ Content validation (NOT just structure)
    assert!(
        has_step_containing(&result, "actual mathematical content"),
        "Must validate real content, not just structure"
    );

    // Multiple flexible checks
    assert!(
        has_step_containing(&result, "keyword1")
        || has_step_containing(&result, "keyword2"),
        "Flexible matching for real content"
    );
}

fn has_step_containing(explanation: &StepByStepExplanation, text: &str) -> bool {
    let text_lower = text.to_lowercase();
    explanation.steps.iter().any(|step| {
        step.description.to_lowercase().contains(&text_lower)
            || step.title.to_lowercase().contains(&text_lower)
    })
}
```

**Requirements**:
- NO structure-only tests (just checking step count)
- Validate actual mathematical content
- Use flexible string matching (not exact strings)
- Target: ≥80% content validation ratio

---

## Common Pitfalls to Avoid

### 1. Declaring Complete Without Verification
**WRONG**: Agent reports "done" → You say "great, move to next wave"
**RIGHT**: Agent reports "done" → You run verification script → Create report → Then approve/reject

### 2. Accepting CLAUDE.md Violations
**WRONG**: "This file is 600 lines but it's comprehensive so it's okay"
**RIGHT**: "This file is 600 lines, split it using module aggregator pattern"

**Exception**: Pre-existing violations documented in previous waves can be accepted, but mark as technical debt

### 3. False Positive Tests
**WRONG**: `assert!(explanation.steps.len() >= 5)` (structure only)
**RIGHT**: `assert!(has_step_containing(&explanation, "power rule"))` (actual content)

### 4. Vague Agent Prompts
**WRONG**: "Implement derivatives with education"
**RIGHT**: "Implement 6 derivative rules (power, chain, product, quotient, sum, constant) with 5-8 step explanations each, domain restrictions noted, 10+ content validation tests"

### 5. Not Using Continuation Agents
**WRONG**: Agent delivers partial work → Accept it → Move on
**RIGHT**: Agent delivers partial work → Launch continuation agent to complete

### 6. Ignoring Pre-existing Issues
**WRONG**: "This module has 2000 lines but I didn't create it, not my problem"
**RIGHT**: "This module has 2000 lines. It's pre-existing (documented in Wave X), acceptable for this wave, but marked as technical debt for future cleanup"

---

## Wave Sequencing Strategies

### Strategy 1: Foundation → Implementation → QA
**Example**: Educational Waves 1-5
- Wave 1: Foundation (message registry, traits)
- Waves 2-4: Implementation (algebra, calculus, functions)
- Wave 5: Testing & QA (audit, fixes, release prep)

**Use when**: Building a new system from scratch

### Strategy 2: Feature → Feature → Feature → Integration
**Example**: 0.2 Release
- Wave 1: Advanced integration
- Wave 2: Multivariable calculus
- Wave 3: Differential equations
- Wave 4: Integration & QA

**Use when**: Adding independent features

### Strategy 3: Refactor → Test → Verify
**Example**: Large refactoring
- Wave 1: Code restructure
- Wave 2: Test updates
- Wave 3: Verification & benchmarking

**Use when**: Refactoring existing code

---

## Continuation Agent Pattern

**When to use**: Agent delivers partial work or claims "blocked"

**Process**:
1. Verify agent's work (may be partially good)
2. Identify what's incomplete
3. Launch continuation agent with:
   - Previous agent's work summary
   - What remains to be done
   - Why previous agent stopped
   - Explicit: "Complete the incomplete work"

**Example**: Educational Wave 2
- Agent 2A: Did polynomial solver, claimed "blocked" on system solver
- Orchestrator: Verified polynomial good, system solver missing
- Agent 2A.1 (continuation): Completed system solver + tests

---

## Progress Tracking (TodoWrite)

Use TodoWrite tool to track progress:

**Pattern**:
```json
[
  {"content": "Wave 1: [Name]", "status": "completed", "activeForm": "[Summary]"},
  {"content": "Wave 2: [Name]", "status": "in_progress", "activeForm": "[Current work]"},
  {"content": "Wave 3: [Name]", "status": "pending", "activeForm": "[Waiting]"},
  {"content": "Verification: Wave 2", "status": "pending", "activeForm": "[Next]"}
]
```

**Update frequently**:
- When starting a wave
- When launching agents
- When verification completes
- When moving to next wave

---

## Documentation Standards

### Create These Documents Per Wave

1. **Verification Script**: `.mathhook_sessions/verify_[wave_name].sh`
2. **Verification Report**: `.mathhook_sessions/[WAVE_NAME]_VERIFICATION_REPORT.md`
3. **Agent Logs** (optional): `.mathhook_sessions/agent_logs/AGENT_[ID]_[NAME]_LOG.md`

### Create These Documents Per Release

1. **Quality Audit**: `.mathhook_sessions/[PROJECT]_QUALITY_AUDIT.md`
2. **Release Readiness**: `.mathhook_sessions/[PROJECT]_[VERSION]_READY.md`
3. **Orchestration Summary**: `.mathhook_sessions/ORCHESTRATION_SUMMARY.md` (this type of doc)

---

## Example: Educational Waves 1-5 Summary

**Goal**: Implement comprehensive educational system for 0.1 release

**Wave Structure**:
- Wave 1: Foundation (message registry, traits, integration)
- Wave 2: Algebra (equations, manipulation)
- Wave 3: Calculus (derivatives, integrals, limits)
- Wave 4: Functions (elementary, polynomial, number theory)
- Wave 5: Testing & QA (fixes, audit, release prep)

**Success Criteria**:
- 8+/10 quality average
- 100+ content validation tests
- CLAUDE.md 100% compliance
- Build passing, zero regressions
- Mathematical correctness verified

**Results**:
- 5/5 waves completed successfully
- 8.5/10 quality average (exceeded target)
- 110 content validation tests (exceeded target)
- 100% CLAUDE.md compliance
- 970+ tests passing, zero regressions
- APPROVED FOR 0.1 RELEASE

**Key Success Factors**:
1. Verification scripts for every wave
2. Strict CLAUDE.md enforcement
3. Content validation focus (no false positives)
4. Continuation agent when needed (Agent 2A.1)
5. Honest quality assessment
6. Momentum maintained (5 waves in single session)

---

## Troubleshooting

### Issue: Agent Hangs or Produces No Output
**Solution**: Check agent status, may need to re-launch with clearer prompt

### Issue: Verification Script Fails
**Solution**: Analyze failure reason. If critical issue, launch continuation agent. If minor, document as acceptable or reject.

### Issue: Quality Below 8/10
**Solution**: Provide specific feedback to continuation agent or reject wave

### Issue: Tests Failing
**Solution**: Never approve wave with failing tests. Launch continuation agent to fix.

### Issue: CLAUDE.md Violations
**Solution**: Never approve wave with violations. Require fixes (split files, remove emojis, etc.)

### Issue: Unclear Success Criteria
**Solution**: Define explicit criteria in wave planning phase. Numbers and metrics, not vague goals.

---

## Metrics to Track

Per Wave:
- Operations/features implemented
- Tests added (content validation vs total)
- Test pass rate
- Quality score
- File size violations
- CLAUDE.md compliance
- Lines of code added/modified

Cumulative:
- Total operations with feature
- Total tests passing
- Average quality score
- Total technical debt
- Progress toward release goal

---

## Release Readiness Criteria

Before declaring any release ready:

1. **Quality**: ≥8/10 average across all waves
2. **Tests**: Target number achieved (e.g., 100+ for 0.1)
3. **Coverage**: Target percentage achieved (e.g., 80% for 0.1)
4. **CLAUDE.md**: 100% compliance (new work)
5. **Build**: Passing with 0 errors
6. **Regressions**: Zero regressions
7. **Documentation**: Quality audit + release readiness document
8. **Critical Bugs**: Zero critical bugs
9. **Mathematical Correctness** (if applicable): Verified
10. **Technical Debt**: Documented (acceptable or not)

---

## Module-Focused Agents with CONTEXT.md

**Added**: 2025-10-30 (Post Educational Waves 1-5)
**Purpose**: Reduce agent token consumption by 60% for module-focused work

### The Problem

Large codebase token consumption during module-focused work:
- Agents working on algebra module load entire codebase context
- Token usage: ~150K for typical agent working on single module
- Most context is irrelevant to the specific module being modified
- Context window pressure limits agent effectiveness

### The Solution: Hybrid Approach

**Keep proven orchestration methodology** + **Add per-module CONTEXT.md files**

Each major module now has a CONTEXT.md file:
- `crates/mathhook-core/src/algebra/CONTEXT.md`
- `crates/mathhook-core/src/calculus/CONTEXT.md`
- `crates/mathhook-core/src/parser/CONTEXT.md`
- `crates/mathhook-core/src/functions/CONTEXT.md`
- `crates/mathhook-core/src/educational/CONTEXT.md`
- `crates/mathhook-core/src/simplify/CONTEXT.md`

### CONTEXT.md Contents

Each CONTEXT.md provides focused module information (~400-450 lines):
1. **Module Structure**: Files, sizes, organization
2. **Public API**: Traits, structs, enums, functions
3. **Dependencies**: What module imports, what imports module
4. **Testing**: Module-specific test commands
5. **External References**: SymPy/Symbolica equivalent locations
6. **Common Patterns & Pitfalls**: Module-specific gotchas
7. **CLAUDE.md Constraints**: Module-specific rules and file size violations
8. **Recent Changes**: Last 3 major modifications
9. **Technical Debt**: Known issues and future improvements
10. **Integration Points**: How module integrates with rest of system

### Expected Token Reduction

**Before** (without CONTEXT.md):
- Agent loads: CLAUDE.md (full) + explores module + reads multiple files
- Token consumption: ~150K tokens
- Context pressure: HIGH

**After** (with CONTEXT.md):
- Agent loads: CLAUDE.md (relevant sections) + module CONTEXT.md + specific files
- Token consumption: ~40-60K tokens
- Context pressure: LOW
- **Reduction**: 60% (90-110K tokens saved)

### When to Use Module CONTEXT.md

**Use for module-focused agents** when work is:
- Contained within single module (algebra, calculus, parser, functions, educational, simplify)
- Modifying 2-5 files within one module
- No cross-module architectural changes
- Standard feature addition, bug fix, or refactoring within module

**Don't use for**:
- Cross-module architectural changes
- Work spanning multiple modules
- New module creation
- Major refactoring across modules

### Updated Agent Prompt Template (Module-Focused)

For module-focused agents, update the prompt template:

```markdown
# [WAVE NAME]: [AGENT PURPOSE]

## Mission: [One-sentence goal]

You are [AGENT ID] within [WAVE NAME]. Your mission is [detailed goal] **within the [MODULE NAME] module**.

## Critical Context

**Your Identity**: You are [AGENT ID] within [WAVE NAME], focusing on **[MODULE NAME] module only**
**Orchestrator**: I am the orchestrator managing sequential waves with parallel agents
**CLAUDE.md Enforcement**: MANDATORY compliance - the orchestrator WILL verify strictly

### Module Context (READ THIS FIRST)

**Before reading any code**, read the module CONTEXT.md:
```
crates/mathhook-core/src/[module]/CONTEXT.md
```

This CONTEXT.md provides:
- Complete module structure and organization
- Public API documentation
- Dependencies (imports and consumers)
- Common pitfalls specific to this module
- CLAUDE.md constraints for this module
- Recent changes and technical debt

**Token Optimization**: By reading CONTEXT.md first, you avoid loading irrelevant context. Expected token usage: 40-60K (vs 150K without CONTEXT.md).

### Your Scope: [TASK NAME] (Within [MODULE NAME])

**Boundary**: Your changes MUST stay within [MODULE NAME] module
- Files: `crates/mathhook-core/src/[module]/**`
- If you need changes outside this module, STOP and consult orchestrator

[Continue with rest of standard template...]
```

### Verification Script Addition

Add module boundary check to verification scripts:

```bash
# CATEGORY X: MODULE BOUNDARY COMPLIANCE
echo "========================================"
echo "CATEGORY X: MODULE BOUNDARY COMPLIANCE"
echo "Agent should only modify [MODULE] files"
echo "========================================"

OUTSIDE_CHANGES=$(git diff --name-only | grep -v "^crates/mathhook-core/src/[module]/" | wc -l)

if [ "$OUTSIDE_CHANGES" -gt 0 ]; then
    echo -e "${RED}✗ $OUTSIDE_CHANGES files modified outside [module]/ boundary${NC}"
    git diff --name-only | grep -v "^crates/mathhook-core/src/[module]/"
    FAILURES=$((FAILURES + 1))
else
    echo -e "${GREEN}✓ All changes within [module]/ boundary${NC}"
fi
```

### Benefits

1. **Token Efficiency**: 60% reduction in token consumption
2. **Focus**: Agents stay within module boundaries
3. **Speed**: Less context to load = faster agent startup
4. **Quality**: More focused context = better understanding
5. **Maintainability**: CONTEXT.md documents module architecture
6. **Onboarding**: New agents/developers can understand module quickly

### Maintenance

**Keep CONTEXT.md files updated**:
- Update when file structure changes significantly
- Update when public API changes
- Update "Recent Changes" section after each wave
- Update "Technical Debt" when new debt identified
- Review quarterly for accuracy

---

## Future Improvements

As you use this methodology, improve it:

1. Add new patterns that work
2. Document new pitfalls discovered
3. Refine verification script templates
4. Add domain-specific guidance (if needed)
5. Update success criteria based on learnings
6. **Expand CONTEXT.md system** to more granular sub-modules if needed

**This document should evolve with experience.**

---

## Summary: Your Command for Next Time

To start orchestrated work for any goal:

```
Orchestrate [GOAL] using the proven methodology from Educational Waves 1-5:

Structure: [Your wave breakdown]
Target: [Your goal metrics]
Success Criteria: [Your quality bars]

Standard orchestration protocol:
- You are orchestrator, maintain momentum
- Create verification scripts per wave
- Launch agents with strict CLAUDE.md enforcement
- Verify everything before declaring complete
- Create comprehensive verification reports
- Track with TodoWrite

Let's begin with Wave 1: [First wave name]
```

**That's it. The orchestrator will handle the rest using this methodology.**

---

**Proven**: Educational Waves 1-5 (100% success rate)
**Ready for**: 0.2 Release, new features, refactoring, any large work
**Maintained by**: Project orchestrator (update as methodology improves)
