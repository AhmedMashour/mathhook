# .mathhook_sessions/ Naming Conventions

**Purpose**: Establish clear, consistent naming patterns so any agent can quickly find relevant context.

**Last Updated**: 2025-10-13

---

## Document Type Classification

All documents in `.mathhook_sessions/` fall into these categories:

### 1. Agent Work Logs (in `agent_logs/` subdirectory)

**Pattern**: `AGENT_{PRIORITY}_{WAVE}_{DESCRIPTIVE_NAME}_LOG.md`

**Examples**:
- `AGENT_P0_1_PATTERN_MATCHING_LOG.md`
- `AGENT_P0_6_CODE_QUALITY_LOG.md`
- `AGENT_P1_4_SYSTEM_SOLVER_LOG.md`

**Components**:
- `AGENT_` - Prefix indicating agent work
- `P0`, `P1`, `P2` - Priority level (P0 = critical bugs, P1 = high priority features, P2 = enhancements)
- `1`, `2`, `3` - Wave number or task sequence
- `DESCRIPTIVE_NAME` - Short, clear description of agent's task (e.g., PATTERN_MATCHING, SYSTEM_SOLVER)
- `_LOG` - Suffix indicating this is a work log

**Purpose**: Track individual agent's work, decisions, and results. Each agent creates exactly one log file.

**Location**: `agent_logs/` subdirectory

---

### 2. Project Session Logs (master logs)

**Pattern**: `{PROJECT_NAME}_SESSION_LOG.md`

**Examples**:
- `INTEGRAL_REGISTRY_SESSION_LOG.md`
- `COMPLEX_ARITHMETIC_SESSION_LOG.md`

**Components**:
- `PROJECT_NAME` - Clear identifier for the feature/project (e.g., INTEGRAL_REGISTRY, COMPLEX_ARITHMETIC)
- `_SESSION_LOG` - Indicates this is a master log tracking all phases

**Purpose**: Master log tracking entire project lifecycle across all phases, agents, and waves. Updated by orchestrator after each phase.

**Location**: Root of `.mathhook_sessions/`

**Key Characteristics**:
- Covers multiple phases/waves
- Updated throughout project lifecycle
- Contains metrics, lessons learned, next steps
- Serves as handoff document for future orchestrators

---

### 3. Phase-Specific Documents

**Pattern**: `PHASE_{N}_{DOCUMENT_TYPE}_{DESCRIPTION}.md`

**Examples**:
- `PHASE_3_ANALYSIS_FUNCTION_INTEGRALS_REFACTORING.md`
- `PHASE_4_AGENT_INSTRUCTIONS.md`
- `PHASE_4_COMPLETION_REPORT.md`

**Components**:
- `PHASE_` - Indicates phase-scoped document
- `{N}` - Phase number (1, 2, 3, 4, etc.)
- `{DOCUMENT_TYPE}` - Type of document:
  - `ANALYSIS` - Pre-implementation analysis and planning
  - `AGENT_INSTRUCTIONS` - Detailed instructions for agents in this phase
  - `COMPLETION_REPORT` - Post-phase summary of results
  - `VERIFICATION` - Verification strategy/results for phase
- `{DESCRIPTION}` - Optional brief description

**Purpose**: Phase-specific planning, instructions, or results.

**Location**: Root of `.mathhook_sessions/`

---

### 4. Wave Reports

**Pattern**: `WAVE_{N}_{REPORT_TYPE}.md`

**Examples**:
- `WAVE_1_COMPLETION_REPORT.md`
- `WAVE_2_PROGRESS_REPORT.md`
- `WAVE_2_VERIFICATION_CHECKERS.md`
- `WAVE_2_FINAL_VERIFICATION_REPORT.md`

**Components**:
- `WAVE_` - Indicates wave-scoped document
- `{N}` - Wave number (1, 2, 3, etc.)
- `{REPORT_TYPE}` - Type of report:
  - `COMPLETION_REPORT` - Final results after wave completion
  - `PROGRESS_REPORT` - Interim status during wave execution
  - `VERIFICATION_CHECKERS` - Verification strategy/checklist
  - `FINAL_VERIFICATION_REPORT` - Comprehensive verification after wave

**Purpose**: Wave-level coordination and results tracking.

**Location**: Root of `.mathhook_sessions/`

**Note**: Waves contain multiple agents working in parallel on independent tasks within same priority tier.

---

### 5. Orchestrator Documents

**Pattern**: `ORCHESTRATOR_{DOCUMENT_TYPE}_{DATE}.md`

**Examples**:
- `ORCHESTRATOR_HANDOFF_2025_10_13.md`
- `ORCHESTRATOR_PLAN_2025_10_13.md`

**Components**:
- `ORCHESTRATOR_` - Indicates orchestrator-created document
- `{DOCUMENT_TYPE}` - Type:
  - `HANDOFF` - Context transfer between orchestrators/sessions
  - `PLAN` - High-level orchestration strategy
- `{DATE}` - Date in YYYY_MM_DD format

**Purpose**: Orchestrator-to-orchestrator communication, session handoffs, high-level planning.

**Location**: Root of `.mathhook_sessions/`

---

### 6. Session Summaries

**Pattern**: `SESSION_{SUMMARY_TYPE}_{DATE}.md`

**Examples**:
- `SESSION_COMPLETION_SUMMARY_2025_10_13.md`
- `SESSION_METRICS_SUMMARY_2025_10_13.md`

**Components**:
- `SESSION_` - Indicates session-scoped summary
- `{SUMMARY_TYPE}` - Type:
  - `COMPLETION_SUMMARY` - End-of-session wrap-up
  - `METRICS_SUMMARY` - Metrics and statistics
- `{DATE}` - Date in YYYY_MM_DD format

**Purpose**: Session-level summaries, typically created at end of development session.

**Location**: Root of `.mathhook_sessions/`

---

### 7. Architecture and Design Documents

**Pattern**: `{PROJECT_NAME}_ARCHITECTURE_DESIGN.md` or `{PROJECT_NAME}_DESIGN_DOC.md`

**Examples**:
- `INTEGRAL_REGISTRY_ARCHITECTURE_DESIGN.md`
- `COMPLEX_ARITHMETIC_DESIGN_DOC.md`

**Components**:
- `{PROJECT_NAME}` - Clear project identifier
- `_ARCHITECTURE_DESIGN` or `_DESIGN_DOC` - Indicates design document

**Purpose**: Detailed architectural designs, created before implementation begins.

**Location**: Root of `.mathhook_sessions/`

**Characteristics**:
- Created during planning phase (before Phase 1)
- Rarely updated after implementation starts
- Serves as architectural reference throughout project

---

### 8. General/Utility Documents

**Pattern**: `{DESCRIPTIVE_NAME}.md` (no specific pattern, use clear descriptive names)

**Examples**:
- `ORCHESTRATION_PLAN.md`
- `PROGRESS_TRACKER.md`
- `0.1_RELEASE_READINESS_AI_AGENT.md`
- `NAMING_CONVENTIONS.md` (this document)

**Purpose**: Cross-cutting concerns, utilities, reference documents.

**Location**: Root of `.mathhook_sessions/`

**Guidelines**:
- Use clear, descriptive ALL_CAPS names
- Separate words with underscores
- Include version numbers if applicable (e.g., `0.1_RELEASE_READINESS`)

---

## Quick Reference: What File Do I Need?

### "I'm a new agent, where do I start?"
1. **Read handoff first**: `ORCHESTRATOR_HANDOFF_{RECENT_DATE}.md`
2. **Read project log**: `{PROJECT_NAME}_SESSION_LOG.md` (e.g., `INTEGRAL_REGISTRY_SESSION_LOG.md`)
3. **Read your agent instructions**: `PHASE_{N}_AGENT_INSTRUCTIONS.md` (if applicable)

### "I'm an orchestrator continuing work"
1. **Read latest handoff**: `ORCHESTRATOR_HANDOFF_{RECENT_DATE}.md`
2. **Read project log**: `{PROJECT_NAME}_SESSION_LOG.md`
3. **Read latest wave/phase report**: `WAVE_{N}_COMPLETION_REPORT.md` or `PHASE_{N}_COMPLETION_REPORT.md`

### "I need to understand the architecture"
1. **Design doc**: `{PROJECT_NAME}_ARCHITECTURE_DESIGN.md`
2. **Analysis doc**: `PHASE_3_ANALYSIS_{DESCRIPTION}.md` (if exists)
3. **CLAUDE.md**: `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` (in project root)

### "I need to verify work was done correctly"
1. **Phase completion report**: `PHASE_{N}_COMPLETION_REPORT.md`
2. **Wave verification**: `WAVE_{N}_VERIFICATION_CHECKERS.md`
3. **Agent logs**: `agent_logs/AGENT_P{N}_{W}_{NAME}_LOG.md`

### "I need to understand what's been done"
1. **Project log**: `{PROJECT_NAME}_SESSION_LOG.md` (master timeline)
2. **Wave completion reports**: `WAVE_{N}_COMPLETION_REPORT.md`
3. **Session summaries**: `SESSION_COMPLETION_SUMMARY_{DATE}.md`

---

## Naming Best Practices

### DO:
- ✅ Use ALL_CAPS with underscores for separators
- ✅ Use descriptive names (PATTERN_MATCHING, not PM)
- ✅ Include priority/wave numbers where applicable
- ✅ Use consistent date format: YYYY_MM_DD
- ✅ Use standard suffixes: `_LOG`, `_REPORT`, `_SUMMARY`
- ✅ Put agent logs in `agent_logs/` subdirectory

### DON'T:
- ❌ Mix naming conventions (don't use camelCase or kebab-case)
- ❌ Use abbreviations that aren't obvious (INTG_REG vs INTEGRAL_REGISTRY)
- ❌ Omit important context (just "LOG.md" is too generic)
- ❌ Use spaces in filenames (use underscores)
- ❌ Create deeply nested subdirectories (keep flat structure)

---

## Directory Structure

```
.mathhook_sessions/
├── agent_logs/                                    # All individual agent work logs
│   ├── AGENT_P0_1_PATTERN_MATCHING_LOG.md
│   ├── AGENT_P0_6_CODE_QUALITY_LOG.md
│   └── AGENT_P1_4_SYSTEM_SOLVER_LOG.md
│
├── {PROJECT}_SESSION_LOG.md                       # Master project logs
├── {PROJECT}_ARCHITECTURE_DESIGN.md               # Architecture designs
├── PHASE_{N}_{TYPE}_{DESC}.md                    # Phase documents
├── WAVE_{N}_{TYPE}.md                            # Wave reports
├── ORCHESTRATOR_{TYPE}_{DATE}.md                 # Orchestrator docs
├── SESSION_{TYPE}_{DATE}.md                      # Session summaries
├── verify_phase{N}.sh                            # Verification scripts
└── {DESCRIPTIVE_NAME}.md                         # General utilities
```

---

## Examples of Good vs Bad Naming

### ✅ GOOD:
- `INTEGRAL_REGISTRY_SESSION_LOG.md` - Clear project identifier, indicates master log
- `PHASE_4_COMPLETION_REPORT.md` - Clear phase, clear document type
- `AGENT_P0_6_CODE_QUALITY_LOG.md` - Priority, wave, descriptive name, proper suffix
- `ORCHESTRATOR_HANDOFF_2025_10_13.md` - Clear type, clear date
- `WAVE_2_VERIFICATION_CHECKERS.md` - Clear wave, clear purpose

### ❌ BAD:
- `log.md` - Too generic, no context
- `phase4.md` - Missing document type
- `agent_work.md` - No priority, wave, or task identifier
- `handoff.md` - Missing date, missing type
- `IR_SL.md` - Unclear abbreviations
- `integral-registry-log.md` - Wrong separator (dash instead of underscore)
- `IntegralRegistryLog.md` - Wrong case (camelCase instead of ALL_CAPS)

---

## When to Create New Files

### Create New File When:
1. Starting a new project/feature (create `{PROJECT}_SESSION_LOG.md`)
2. Beginning a new phase (create `PHASE_{N}_AGENT_INSTRUCTIONS.md` and later `PHASE_{N}_COMPLETION_REPORT.md`)
3. Launching an agent (agent creates `AGENT_P{N}_{W}_{NAME}_LOG.md`)
4. Completing a wave (create `WAVE_{N}_COMPLETION_REPORT.md`)
5. Handing off to new orchestrator (create `ORCHESTRATOR_HANDOFF_{DATE}.md`)

### Update Existing File When:
1. Project continues across phases (update `{PROJECT}_SESSION_LOG.md`)
2. Adding phase completion details (update session log, don't create new file unless it's a completion report)
3. Wave is progressing (update wave progress report)
4. General utilities need updates (update `PROGRESS_TRACKER.md`, etc.)

---

## File Lifecycle

### Planning Phase:
1. Create `{PROJECT}_ARCHITECTURE_DESIGN.md`
2. Create `{PROJECT}_SESSION_LOG.md` (with Phase 1-N outline)
3. Create `PHASE_1_AGENT_INSTRUCTIONS.md`

### Execution Phase:
1. Each agent creates `agent_logs/AGENT_P{N}_{W}_{NAME}_LOG.md`
2. After phase completes, create `PHASE_{N}_COMPLETION_REPORT.md`
3. Update `{PROJECT}_SESSION_LOG.md` with phase results

### Completion Phase:
1. Create `WAVE_{N}_COMPLETION_REPORT.md` (if wave-based orchestration)
2. Create `SESSION_COMPLETION_SUMMARY_{DATE}.md`
3. Create `ORCHESTRATOR_HANDOFF_{DATE}.md` for next session

---

## Special Cases

### Verification Scripts
**Pattern**: `verify_phase{N}.sh` or `verify_wave{N}.sh`

**Examples**:
- `verify_phase4.sh`
- `verify_wave_1.sh`

**Purpose**: Automated verification scripts for phases/waves.

**Location**: Root of `.mathhook_sessions/`

### Multi-Project Coordination
If multiple projects are active simultaneously, use project prefixes consistently:

```
INTEGRAL_REGISTRY_SESSION_LOG.md
COMPLEX_ARITHMETIC_SESSION_LOG.md
MATRIX_OPERATIONS_SESSION_LOG.md
```

---

## Summary: File Naming Decision Tree

```
Is this an agent's work log?
  → YES: agent_logs/AGENT_P{N}_{W}_{NAME}_LOG.md
  → NO: Continue...

Is this a master project log?
  → YES: {PROJECT}_SESSION_LOG.md
  → NO: Continue...

Is this phase-specific?
  → YES: PHASE_{N}_{TYPE}_{DESC}.md
  → NO: Continue...

Is this wave-specific?
  → YES: WAVE_{N}_{TYPE}.md
  → NO: Continue...

Is this orchestrator handoff/planning?
  → YES: ORCHESTRATOR_{TYPE}_{DATE}.md
  → NO: Continue...

Is this a session summary?
  → YES: SESSION_{TYPE}_{DATE}.md
  → NO: Use descriptive name with context
```

---

## Validation Checklist

Before creating a new file, verify:

- [ ] Name uses ALL_CAPS with underscores
- [ ] Name clearly indicates content type
- [ ] Name includes necessary context (project, phase, wave, date)
- [ ] Name follows one of the established patterns above
- [ ] Agent logs go in `agent_logs/` subdirectory
- [ ] File extension is `.md` (or `.sh` for scripts)
- [ ] Name is not too abbreviated (use full words)

---

**Document Owner**: Orchestrator agents
**Last Updated**: 2025-10-13
**Next Review**: When new document types emerge or patterns need refinement
