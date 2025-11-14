# Orchestrator Delegation Pattern

**Purpose**: Define how the meta-orchestrator (Conductor) delegates plan execution to specialist agents while explicitly assigning them the orchestrator role for their plan.

---

## Role Hierarchy

```
Meta-Orchestrator (Conductor)
    ↓ delegates plan execution + orchestrator authority
Plan Orchestrator (Specialist Agent)
    ↓ delegates wave execution
Wave Executor (Sub-Agents)
```

**Key Insight**: When executing a multi-wave plan, the specialist agent becomes the **orchestrator for that plan**, not just an executor.

---

## Delegation Pattern

### Meta-Orchestrator → Plan Orchestrator

```bash
# Meta-Orchestrator (Conductor) spawns Plan Orchestrator
# Example: Delegating Plan 1 (Performance Recovery)

/sc:spawn rust-engineer "

=================================================================
YOU ARE THE ORCHESTRATOR FOR PLAN 1: PERFORMANCE RECOVERY
=================================================================

🎯 YOUR ROLE: Plan Orchestrator (NOT just executor)

As the orchestrator for this plan, you are responsible for:
1. **Planning**: Reading PLAN_1_PERFORMANCE_RECOVERY.md and understanding all 4 waves
2. **Delegation**: Spawning sub-agents for each wave as needed
3. **Coordination**: Managing dependencies between waves
4. **Validation**: Running verification scripts after each wave
5. **Reporting**: Signaling completion to meta-orchestrator (Conductor)

=================================================================
ORCHESTRATION AUTHORITY GRANTED
=================================================================

You have authority to:
✅ Spawn sub-agents for wave-specific tasks
✅ Make technical decisions within plan scope
✅ Run validation scripts to verify work
✅ Signal wave completion to meta-orchestrator
✅ Escalate blockers to meta-orchestrator when needed

❌ You CANNOT:
- Modify other plans
- Merge to integration branch (meta-orchestrator only)
- Skip validation gates
- Override mathematical correctness requirements

=================================================================
EXECUTION CONTEXT
=================================================================

Workspace: worktrees/agent-1-performance
Branch: agent-1/performance-recovery
Plan: .mathhook_sessions/gtm/PLAN_1_PERFORMANCE_RECOVERY.md
Validation: ../../worktrees/conductor/verification-scripts/verify_performance.sh

Dependencies: None (Phase 1 - must complete first)
Unblocks: Phase 2 (Plans 2, 3, 4, 7)

=================================================================
MANDATORY ORCHESTRATION WORKFLOW
=================================================================

1. Read plan document:
   cat .mathhook_sessions/gtm/PLAN_1_PERFORMANCE_RECOVERY.md

2. Create execution plan with wave breakdown:
   /sc:task 'Analyze PLAN_1, create wave-by-wave execution strategy with TodoWrite'

3. Execute waves sequentially:
   For each wave:
     - Spawn sub-agent OR execute directly (your choice)
     - Validate wave completion
     - Signal completion via commit: 'git commit -m \"✅ Wave X Complete\"'

4. After all waves complete, run validation:
   ../../worktrees/conductor/verification-scripts/verify_performance.sh

5. Signal plan completion to meta-orchestrator:
   git commit -m \"✅ PLAN 1 COMPLETE - Ready for validation\"

=================================================================
DELEGATION EXAMPLE (Wave 1: Baseline & Regression Analysis)
=================================================================

You can delegate to sub-agents:

/sc:spawn rust-engineer 'Execute Wave 1: Baseline & Regression Analysis

Context: You are executing Wave 1 of Performance Recovery Plan.

Tasks:
1. Run all benchmarks: cargo bench --bench performance_benchmarks
2. Save baseline: .agent-state/baselines/performance_baseline.json
3. Identify regressions using flamegraphs
4. Document findings in performance_analysis.md

Deliverable: Regression report with specific files/functions identified

Validation: Ensure all 4 benchmarks run successfully
'

OR execute directly if straightforward.

=================================================================
SUCCESS CRITERIA (CRITICAL - DO NOT SKIP)
=================================================================

Before declaring plan complete, ALL must be true:
✅ All 4 waves executed and validated
✅ Test suite: ≥676/677 passing
✅ Performance: 0% regressions vs baseline
✅ Correctness: 100% correctness suite passing
✅ Expression size: ≤48 bytes
✅ Verification script passes: verify_performance.sh

=================================================================
START ORCHESTRATION NOW
=================================================================

Your first action should be:
1. Read PLAN_1_PERFORMANCE_RECOVERY.md
2. Create TodoWrite with all waves
3. Begin Wave 1 execution (delegate or execute)

You are the orchestrator. Plan and execute systematically.
"
```

---

## Orchestrator Persona Injection

### Before Delegation
```python
# Meta-orchestrator prepares context
orchestrator_context = f"""
YOU ARE THE ORCHESTRATOR FOR {plan_name}.

Your responsibilities:
1. Strategic: Understand entire plan scope
2. Tactical: Break down into executable waves
3. Operational: Execute or delegate each wave
4. Quality: Validate after each wave
5. Communication: Report to meta-orchestrator

Authority granted: {authority_list}
Constraints: {constraint_list}
Success criteria: {success_criteria}

Workspace: {worktree_path}
Validation script: {validation_script_path}
Dependencies: {dependencies}
Unblocks: {unblocks}
"""

# Inject into agent prompt
spawn_agent(agent_type, orchestrator_context + plan_details)
```

---

## Explicit Role Assignment Examples

### Plan 2 (Educational Integration)

```bash
/sc:spawn rust-engineer "

=================================================================
YOU ARE THE ORCHESTRATOR FOR PLAN 2: EDUCATIONAL INTEGRATION
=================================================================

🎯 YOUR ROLE: Plan Orchestrator

PLAN SCOPE: 5 waves, 6-8 weeks, educational features end-to-end integration

YOUR ORCHESTRATION RESPONSIBILITIES:
1. **Wave 0**: Audit current state (30% complete)
2. **Wave 1**: Clean up code duplication (780 lines)
3. **Wave 2**: Implement EducationalOperation for ALL solvers
4. **Wave 3**: Design Expression.explain() API (CRITICAL - blocks Plans 3, 4)
5. **Wave 4**: User testing (3-5 users, ≥7/10 clarity score)
6. **Wave 5**: Production integration

=================================================================
CRITICAL DEPENDENCY MANAGEMENT
=================================================================

⚠️  Wave 3 completion is CRITICAL:
- Blocks: Plan 3 (Python API) Wave 4
- Blocks: Plan 4 (Node.js API) Wave 3
- Requirement: Expression.explain() API signature must be FROZEN

When Wave 3 completes:
1. Signal via Serena MCP:
   write_memory('agent-2-wave-3-complete', {
     'status': 'completed',
     'api': 'Expression.explain()',
     'commit': '<sha>',
     'unblocks': ['agent-3-python-api', 'agent-4-nodejs-api']
   })

2. Commit message: 'git commit -m \"✅ Wave 3 Complete - Expression.explain() API STABLE\"'

3. Notify meta-orchestrator of API freeze

=================================================================
DELEGATION STRATEGY
=================================================================

Complex waves → Delegate to sub-agents
Simple waves → Execute directly

Example (Wave 2: EducationalOperation implementation):

/sc:spawn rust-engineer 'Implement EducationalOperation for linear solver

Context: Wave 2 of Educational Integration
Task: Add EducationalOperation trait to crates/mathhook-core/src/solvers/linear.rs
Pattern: Follow existing implementations in quadratic_solver.rs

Deliverable:
- EducationalOperation impl for LinearSolver
- Doctests showing step-by-step explanation
- Tests passing

Validation: cargo test -p mathhook-core linear_solver
'

=================================================================
VALIDATION & REPORTING
=================================================================

After each wave:
1. Run validation: ../../worktrees/conductor/verification-scripts/verify_educational.sh
2. Signal completion: git commit -m \"✅ Wave X Complete\"
3. Update TodoWrite

After Wave 3 (API freeze):
1. Signal via Serena MCP (see above)
2. Notify meta-orchestrator
3. Continue to Waves 4, 5

After all waves:
1. Run full validation script
2. Signal: git commit -m \"✅ PLAN 2 COMPLETE\"
3. Await meta-orchestrator merge approval

=================================================================
START ORCHESTRATION
=================================================================

Read: .mathhook_sessions/gtm/PLAN_2_EDUCATIONAL_INTEGRATION.md
Create: TodoWrite with 5 waves + sub-tasks
Execute: Wave 0 (audit current state)

You are the orchestrator. Proceed systematically.
"
```

### Plan 3 (Python API) - With Dependency Waiting

```bash
/sc:spawn python-expert "

=================================================================
YOU ARE THE ORCHESTRATOR FOR PLAN 3: PYTHON API PRODUCTION
=================================================================

🎯 YOUR ROLE: Plan Orchestrator

PLAN SCOPE: 5 waves, 8-10 weeks, PyPI-ready Python package

=================================================================
CRITICAL DEPENDENCY: WAIT FOR AGENT 2 WAVE 3
=================================================================

⚠️  BEFORE starting Wave 4 (Complete API Bindings):

Check dependency via Serena MCP:
```python
from serena import read_memory

deps = read_memory('agent-2-wave-3-complete')
if deps['status'] != 'completed':
    print('⏸️  WAITING for Agent 2 Wave 3 (Expression.explain() API)')
    # PAUSE Wave 4 execution
    # Continue with Waves 1-3 meanwhile
```

When dependency met:
1. Verify Expression.explain() API signature
2. Proceed with Python bindings for explain()
3. Update TodoWrite

=================================================================
ORCHESTRATION STRATEGY
=================================================================

Waves 1-3: Execute in parallel with Agent 2
- Wave 1: Maturin build system (independent)
- Wave 2: Operator overloading (independent)
- Wave 3: Jupyter integration (independent)

Wave 4: WAIT for Agent 2 Wave 3
- Then: Bind Expression.explain() to Python
- Then: Complete all mathhook-core bindings

Wave 5: PyPI publication
- Test across platforms (Linux, macOS, Windows)
- Publish to PyPI

=================================================================
MULTI-PLATFORM VALIDATION REQUIREMENT
=================================================================

Before Wave 5 (PyPI publication), validate:
✅ maturin build succeeds on Linux x86_64
✅ maturin build succeeds on macOS ARM64
✅ maturin build succeeds on Windows x64
✅ pip install works on all platforms
✅ Type stubs (.pyi) generated correctly
✅ All tests pass: pytest tests/

Use GitHub Actions matrix or local testing.

=================================================================
DELEGATION PATTERN
=================================================================

/sc:spawn python-expert 'Generate type stubs for mathhook

Context: Wave 2, Python API Production
Task: Generate .pyi type stub files for all Rust bindings

Tools: pyo3-stub-gen OR manual generation
Validation: mypy --strict passes

Deliverable: mathhook-stubs/ directory with complete .pyi files
'

=================================================================
START ORCHESTRATION
=================================================================

Read: .mathhook_sessions/gtm/PLAN_3_PYTHON_API_PRODUCTION.md
Check dependency: read_memory('agent-2-wave-3-complete')
Create: TodoWrite with dependency-aware scheduling

Execute Waves 1-3 while waiting for Agent 2.

You are the orchestrator. Manage dependencies actively.
"
```

---

## Meta-Orchestrator Monitoring

### Checking Plan Orchestrator Progress

```bash
# Meta-orchestrator queries plan status

/sc:coordinate

# Output shows:
# Agent 2 (Plan Orchestrator): Wave 2/5 - Implementing EducationalOperation
# Agent 3 (Plan Orchestrator): WAITING for agent-2-wave-3-complete
# Agent 4 (Plan Orchestrator): WAITING for agent-2-wave-3-complete
# Agent 7 (Plan Orchestrator): Wave 1/6 - ODE solvers (4 tests failing)
```

### Escalation from Plan Orchestrator

```python
# Plan orchestrator signals blocker to meta-orchestrator

from serena import write_memory

write_memory('agent-7-blocker', {
    'status': 'blocked',
    'wave': 1,
    'issue': '4 ODE tests failing - separable equation solver',
    'context': 'Investigated for 2 days, root cause unclear',
    'escalation_level': 'high',
    'timestamp': now()
})

# Meta-orchestrator detects escalation
# Spawns investigation sub-agent OR provides guidance
```

---

## Benefits of Explicit Orchestrator Role

### 1. Clear Authority
- Plan orchestrator knows they can spawn sub-agents
- No ambiguity about decision-making scope

### 2. Systematic Execution
- Orchestrator mindset → TodoWrite + wave breakdown
- Not just "execute task" but "orchestrate plan"

### 3. Dependency Management
- Plan orchestrator actively checks dependencies
- Signals completion to unblock others

### 4. Quality Ownership
- Plan orchestrator responsible for validation
- Not meta-orchestrator micromanaging

### 5. Scalability
- Meta-orchestrator delegates to N plan orchestrators
- Plan orchestrators delegate to M sub-agents
- Hierarchical, scalable coordination

---

## Summary

**Before**: Meta-orchestrator spawns agent → agent executes tasks (no orchestration)

**After**: Meta-orchestrator spawns agent **AS ORCHESTRATOR** → agent orchestrates plan (systematic, validated, coordinated)

**Key Change**: Explicit role assignment with authority, constraints, and success criteria in delegation prompt.

---

**Next Steps**:
1. Update spawn commands to include orchestrator role injection
2. Test with Plan 1 (single agent, validate pattern)
3. Scale to parallel plans (4 orchestrators coordinating)

This pattern ensures each plan has a **responsible orchestrator** with clear authority and accountability.
