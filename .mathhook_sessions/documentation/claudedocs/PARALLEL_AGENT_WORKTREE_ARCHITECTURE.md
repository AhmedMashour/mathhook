# Parallel Agent Worktree Orchestration Architecture

**Version**: 1.0
**Date**: 2025-10-21
**Purpose**: Enable true parallel agent execution for MathHook GTM using git worktrees with automated validation and strict acceptance criteria

---

## Executive Summary

This architecture enables **real parallel development** across 7 GTM plans (48-64 weeks realistic timeline) by:

1. **Git Worktrees**: Each agent gets isolated workspace (zero conflicts)
2. **Orchestrator Coordination**: PM Agent coordinates dependencies, validation, merges
3. **Automated Validation**: Strict quality gates enforced before any merge
4. **Serena MCP Integration**: Cross-agent state management and dependency tracking
5. **AI-Powered Conflict Resolution**: Sequential MCP resolves merge conflicts systematically

**Key Innovation**: Agents never step on each other's toes because they work in completely isolated worktrees.

---

## Architecture Overview

### Worktree Structure

```
mathhook/                          # Main repo (read-only during parallel work)
├── .git/                          # Shared git database
├── worktrees/
│   ├── conductor/                 # Orchestrator's coordination workspace
│   ├── agent-1-performance/       # Plan 1: Performance recovery
│   ├── agent-2-educational/       # Plan 2: Educational integration
│   ├── agent-3-python-api/        # Plan 3: Python API
│   ├── agent-4-nodejs-api/        # Plan 4: Node.js API
│   ├── agent-5-gtm-launch/        # Plan 5: GTM & docs automation
│   ├── agent-6-mcp-server/        # Plan 6: MCP server
│   └── agent-7-core-math/         # Plan 7: Core math features (ODEs, PDEs, etc.)
```

### Orchestrator Workspace

```
worktrees/conductor/
├── .agent-state/                  # Cross-agent coordination
│   ├── dependency-graph.yaml      # Task dependencies
│   ├── agent-progress.json        # Real-time status
│   ├── phase-gates.yaml           # Acceptance criteria
│   └── risk-register.json         # Risk tracking
├── verification-scripts/          # Automated validation
│   ├── verify_performance.sh      # Plan 1 gates
│   ├── verify_educational.sh      # Plan 2 gates
│   ├── verify_python_api.sh       # Plan 3 gates
│   ├── verify_nodejs_api.sh       # Plan 4 gates
│   ├── verify_gtm_launch.sh       # Plan 5 gates
│   ├── verify_mcp_server.sh       # Plan 6 gates
│   └── verify_core_math.sh        # Plan 7 gates
└── orchestration-log.md           # Timestamped decisions
```

---

## Orchestrator Role (Conductor Agent)

### Responsibilities

1. **Spawn Agents**: Create worktrees + launch agents
2. **Track Dependencies**: Monitor cross-agent dependencies via Serena MCP
3. **Validate Work**: Run verification scripts before merge
4. **Coordinate Merges**: Sequential validated merges to integration branch
5. **Risk Management**: Detect blockers, trigger investigations
6. **Progress Reporting**: Weekly syncs, real-time dashboard

### Key Capabilities

- **Dependency Graph Management**: Knows which agents block which
- **Automated Validation**: Enforces strict acceptance criteria
- **Conflict Resolution**: AI-powered merge conflict handling
- **Phase Gate Enforcement**: Prevents premature phase transitions

---

## Workflow: Phase-by-Phase Execution

### Phase 1: Setup & Spawn

```bash
# 1. Orchestrator analyzes GTM plans
/sc:pm analyze "Read GTM plans, create dependency graph, identify parallelization opportunities"

# 2. Spawns agents into worktrees (sequential for Phase 1)
git worktree add worktrees/agent-1-performance agent-1/performance-recovery
/sc:spawn rust-engineer "Execute PLAN_1_PERFORMANCE_RECOVERY.md" --worktree=worktrees/agent-1-performance

# 3. Waits for Phase 1 completion + validation
./verification-scripts/verify_performance.sh

# 4. After Phase 1 gate passes, spawns Phase 2 agents (PARALLEL)
git worktree add worktrees/agent-2-educational agent-2/educational-integration
git worktree add worktrees/agent-3-python-api agent-3/python-api-production
git worktree add worktrees/agent-4-nodejs-api agent-4/nodejs-api-production
git worktree add worktrees/agent-7-core-math agent-7/core-math-features

# Spawn all 4 agents in SINGLE message (true parallel)
/sc:spawn rust-engineer "Execute PLAN_2_EDUCATIONAL_INTEGRATION.md" --worktree=worktrees/agent-2-educational
/sc:spawn python-expert "Execute PLAN_3_PYTHON_API_PRODUCTION.md" --worktree=worktrees/agent-3-python-api
/sc:spawn backend-architect "Execute PLAN_4_NODEJS_API_PRODUCTION.md" --worktree=worktrees/agent-4-nodejs-api
/sc:spawn rust-engineer "Execute PLAN_7_CORE_MATH_FEATURES.md" --worktree=worktrees/agent-7-core-math
```

### Phase 2: Parallel Execution with Coordination

#### Agent Isolation
- Each agent: Dedicated worktree + branch + IDE window
- Zero file conflicts during development
- Independent test execution

#### Coordination via Serena MCP
```python
# Agent 2 signals Wave 3 completion
from serena import write_memory

write_memory("agent-2-wave-3-complete", {
    "status": "completed",
    "api": "Expression.explain()",
    "timestamp": "2025-10-21T10:30:00Z",
    "commit": "abc123",
    "tests_passing": 95,
    "unblocks": ["agent-3-python-api", "agent-4-nodejs-api"]
})

# Agent 3 checks dependency before proceeding
from serena import read_memory

deps = read_memory("agent-2-wave-3-complete")
if deps["status"] == "completed":
    # Proceed with Wave 4
    pass
```

#### Git Hooks for Automated Notifications
```bash
# .git/hooks/post-commit (in each agent worktree)
#!/bin/bash
AGENT_ID=$(basename $(git rev-parse --show-toplevel))
COMMIT_SHA=$(git rev-parse HEAD)

# Update orchestrator progress
python3 ../conductor/.agent-state/update_progress.py \
    --agent="$AGENT_ID" \
    --commit="$COMMIT_SHA"

# Check for wave completion markers
if git log -1 --pretty=%B | grep -q "✅ Wave [0-9]* Complete"; then
    python3 ../conductor/.agent-state/check_dependencies.py \
        --agent="$AGENT_ID" \
        --trigger-unblocked-agents
fi
```

### Phase 3: Validation & Acceptance (Strict Quality Gates)

#### Automated Validation Script Example

```bash
# verification-scripts/verify_performance.sh
#!/bin/bash
set -e

echo "🔍 Validating Agent 1: Performance Recovery"

# 1. Test Suite Validation
cargo test --no-fail-fast 2>&1 | tee /tmp/agent-1-tests.log
PASS_RATE=$(grep -oP '\d+(?= passed)' /tmp/agent-1-tests.log)
[ "$PASS_RATE" -ge 676 ] || { echo "❌ Tests: $PASS_RATE/677"; exit 1; }
echo "✅ Tests: $PASS_RATE/677"

# 2. Performance Regression Check
cargo bench 2>&1 | tee /tmp/agent-1-bench.log
python3 .agent-state/compare_benchmarks.py \
    --current=/tmp/agent-1-bench.log \
    --baseline=.agent-state/baselines/performance_baseline.json \
    --max-regression=0%
[ $? -eq 0 ] || { echo "❌ Performance regressions"; exit 1; }
echo "✅ No regressions"

# 3. SymPy Validation
python3 .agent-state/sympy_comparison.py --validate-claims
[ $? -eq 0 ] || { echo "❌ Speed claims invalid"; exit 1; }
echo "✅ 10-100x faster validated"

# 4. Mathematical Correctness (CRITICAL)
cargo test --release -p mathhook-core --test correctness_suite
[ $? -eq 0 ] || { echo "❌ CRITICAL: Correctness regression"; exit 1; }
echo "✅ Correctness maintained"

# 5. Expression Size Constraint
SIZE=$(cargo test expression_size 2>&1 | grep -oP '\d+(?= bytes)')
[ "$SIZE" -le 48 ] || { echo "❌ Size: $SIZE bytes"; exit 1; }
echo "✅ Size: $SIZE bytes"

echo "✅✅✅ ALL VALIDATION PASSED ✅✅✅"
exit 0
```

#### Phase Gates Configuration

```yaml
# .agent-state/phase-gates.yaml

Phase_1_Exit_Gate:
  name: "Performance Recovery Complete"
  agent: agent-1-performance
  required_validations:
    - test_suite: {threshold: "676/677 passing", blocker: true}
    - performance: {threshold: "0% regression", blocker: true}
    - sympy_speed: {threshold: "10-100x validated", blocker: false}
    - correctness: {threshold: "100% passing", blocker: true}
    - expression_size: {threshold: "≤48 bytes", blocker: true}
  approval_required: [orchestrator, project-lead]
  unblocks: [Phase_2_Parallel_Build]

Phase_2_Educational_Wave_3_Gate:
  name: "Expression.explain() API Stable"
  agent: agent-2-educational
  required_validations:
    - api_stability: {threshold: "API signature frozen", blocker: true}
    - user_testing: {threshold: "≥7/10 clarity (3+ users)", blocker: true}
    - doctest_coverage: {threshold: "All solver types", blocker: false}
  approval_required: [orchestrator]
  unblocks: [agent-3-python-api Wave 4, agent-4-nodejs-api Wave 3]
```

### Phase 4: Merge Strategy (Sequential Validated Merges)

```bash
# After validation passes, orchestrator merges

cd worktrees/conductor
git checkout integration
git merge --no-ff agent-1/performance-recovery -m "✅ Merge Agent 1: Performance Recovery

Acceptance Criteria Met:
- Tests: 676/677 passing ✅
- Performance: No regressions ✅
- SymPy: 10-100x faster ✅
- Correctness: 100% ✅
- Size: 32 bytes ✅

Validated-By: verify_performance.sh
Commit: abc123
"

# Run integration tests
./verification-scripts/verify_integration.sh

# Tag milestone
git tag -a "phase-1-complete" -m "Phase 1 Complete"

# Unblock dependent agents
python3 .agent-state/unblock_agents.py --phase=phase_2_parallel_build
```

---

## Dependency Management

### Dependency Graph Example

```yaml
# .agent-state/dependency-graph.yaml

phases:
  phase_1_critical_foundation:
    duration: "4-6 weeks"
    agents: [agent-1-performance]
    dependencies: []
    blocks: [phase_2_parallel_build]

  phase_2_parallel_build:
    duration: "24-36 weeks (longest: agent-7)"
    agents:
      - agent-2-educational    # 8-12 weeks
      - agent-3-python-api     # 10-14 weeks
      - agent-4-nodejs-api     # 10-13 weeks
      - agent-7-core-math      # 24-36 weeks (CRITICAL PATH)
    dependencies:
      - phase_1_critical_foundation
      - agent-3: requires agent-2 Wave 3 (Expression.explain API)
      - agent-4: requires agent-2 Wave 3
    blocks: [phase_3_market_launch]

  phase_3_market_launch:
    duration: "12-16 weeks"
    agents: [agent-5-gtm-launch]
    dependencies: [phase_2_parallel_build]
    blocks: []

  phase_4_mcp_integration:
    duration: "4-6 weeks"
    agents: [agent-6-mcp-server]
    dependencies: [agent-3-python-api]
    blocks: []
```

### Dependency Tracking System

```python
# .agent-state/check_dependencies.py

def check_dependencies(agent_id, event):
    """
    Check if agent completion unblocks other agents
    """
    graph = load_dependency_graph()

    # Find agents blocked by this event
    blocked_agents = []
    for agent in graph['agents']:
        if event in agent.get('blocked_by', []):
            blocked_agents.append(agent)

    # Verify all dependencies met
    for agent in blocked_agents:
        if all_dependencies_met(agent):
            notify_agent(agent, "UNBLOCKED", event)
            write_memory(f"{agent['id']}-unblocked", {
                "status": "ready",
                "unblocked_by": event,
                "timestamp": now()
            })
```

---

## Orchestrator Slash Commands

### `/sc:worktree` Command Suite

```bash
# Create worktree + spawn agent
/sc:worktree create {agent-name} {plan-file} --branch={branch-name}

# Validate before merge
/sc:worktree validate {agent-name} --strict

# Merge validated work
/sc:worktree merge {agent-name} --target=integration

# Cleanup completed worktree
/sc:worktree cleanup {agent-name}
```

### `/sc:coordinate` Dashboard

Real-time orchestration dashboard showing:
- Active agents (status, progress, ETA)
- Dependency graph (what blocks what)
- Risk register (failing tests, blockers)
- Next actions (for orchestrator)

Example output:
```
╔════════════════════════════════════════════════════════════╗
║          MATHHOOK ORCHESTRATION DASHBOARD                  ║
╚════════════════════════════════════════════════════════════╝

📅 Phase: Phase 2 (Parallel Build)
⏱️  Week: 3/36
🎯 Critical Path: Agent 7 (Core Math - 24-36 weeks)

ACTIVE AGENTS (4/7):
✅ Agent 1 [COMPLETE]
🔄 Agent 2 [45% - Wave 2/5] - Blocks Agents 3, 4
⏸️  Agent 3 [WAITING] - Blocked by Agent 2 Wave 3
⏸️  Agent 4 [WAITING] - Blocked by Agent 2 Wave 3
🔄 Agent 7 [12% - Wave 1/6] - ⚠️ 4 failing tests

RISKS:
⚠️ HIGH: Agent 7 has 4 failing ODE tests → Investigation spawned
```

---

## Conflict Resolution (AI-Powered)

```python
# .agent-state/merge_orchestrator.py

def merge_with_ai_resolution(agent_branch, target_branch):
    """
    AI-powered conflict resolution using Sequential MCP
    """
    result = subprocess.run(['git', 'merge', '--no-commit', agent_branch])

    if result.returncode != 0:
        # Conflicts detected
        conflicts = get_git_conflicts()

        # Use Sequential MCP for systematic resolution
        resolution = sequential_thinking(f"""
        Resolve merge conflicts between {agent_branch} and {target_branch}.

        Conflicts: {conflicts}

        Requirements:
        1. Preserve mathematical correctness (CRITICAL)
        2. Maintain performance optimizations
        3. Respect both agents' intent
        4. Document rationale
        """)

        apply_resolution(resolution)

        # Re-validate after resolution
        if not validate_merge():
            escalate_to_human(conflicts, resolution)

    git.commit(message=generate_merge_message(agent_branch))
```

---

## Benefits Summary

### 1. True Parallelism
- Each agent = isolated worktree + branch
- Zero file conflicts during development
- Independent test execution

### 2. Automated Validation
- Every merge requires validation pass
- Phase gates enforce quality standards
- Mathematical correctness CRITICAL gate

### 3. Intelligent Coordination
- Dependency tracking via Serena MCP
- Automatic unblocking
- Real-time progress visibility

### 4. Risk Mitigation
- Failing tests → immediate investigation
- Blocked agents → escalation
- Integration conflicts → AI resolution

### 5. Audit Trail
- Every decision logged
- Git history shows contributions
- Validation results timestamped

---

## Implementation Timeline

### Phase 0: Infrastructure Setup (1-2 weeks)
- Create orchestrator workspace
- Implement verification scripts
- Set up Serena MCP integration
- Configure git hooks

### Phase 1: Performance Recovery (4-6 weeks)
- Single agent execution
- Validate orchestration system
- Refine validation scripts

### Phase 2: Parallel Execution (24-36 weeks)
- 4 agents in parallel
- Real-world dependency coordination
- Continuous validation

### Phase 3+: Full Scale (48-64 weeks total)
- All 7 plans complete
- Automated merge orchestration
- Production-ready workflow

---

## Next Steps

1. **Review this architecture** with project lead
2. **Set up orchestrator workspace** (worktrees/conductor)
3. **Create verification scripts** for each plan
4. **Configure Serena MCP** for cross-agent state
5. **Test with Plan 1** (single agent, validate system)
6. **Scale to parallel** (Plans 2, 3, 4, 7)

---

**Document Version**: 1.0
**Last Updated**: 2025-10-21
**Status**: Ready for Implementation
