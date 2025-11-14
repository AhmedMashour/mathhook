# Git Worktree Orchestration - Quick Start Guide

**Purpose**: Get started with parallel agent execution using git worktrees in 30 minutes

---

## Prerequisites

- MathHook repository (current state: 676/677 tests passing)
- Git 2.5+ (worktree support)
- SuperClaude framework with MCP servers (Serena for coordination)
- 7 GTM plans in `.mathhook_sessions/gtm/`

---

## Step 1: Set Up Orchestrator Workspace (10 minutes)

```bash
# 1. Create conductor worktree
cd /Users/ahmedmashhour/Documents/work/math/mathhook
git worktree add worktrees/conductor conductor/orchestration

# 2. Set up orchestrator directory structure
cd worktrees/conductor
mkdir -p .agent-state verification-scripts orchestration-scripts

# 3. Create dependency graph template
cat > .agent-state/dependency-graph.yaml << 'EOF'
phases:
  phase_1_critical_foundation:
    duration: "4-6 weeks"
    agents: [agent-1-performance]
    dependencies: []
    blocks: [phase_2_parallel_build]

  phase_2_parallel_build:
    duration: "24-36 weeks"
    agents:
      - agent-2-educational
      - agent-3-python-api
      - agent-4-nodejs-api
      - agent-7-core-math
    dependencies:
      - phase_1_critical_foundation
      - agent-3: requires agent-2 Wave 3
      - agent-4: requires agent-2 Wave 3
    blocks: [phase_3_market_launch]

  phase_3_market_launch:
    duration: "12-16 weeks"
    agents: [agent-5-gtm-launch]
    dependencies: [phase_2_parallel_build]

  phase_4_mcp_integration:
    duration: "4-6 weeks"
    agents: [agent-6-mcp-server]
    dependencies: [agent-3-python-api]
EOF

# 4. Initialize agent progress tracker
cat > .agent-state/agent-progress.json << 'EOF'
{
  "last_updated": "2025-10-21T00:00:00Z",
  "active_phase": "phase_0_setup",
  "agents": []
}
EOF

# 5. Create phase gates configuration
cat > .agent-state/phase-gates.yaml << 'EOF'
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
EOF

echo "✅ Orchestrator workspace initialized"
```

---

## Step 2: Create Validation Scripts (15 minutes)

```bash
# Create verification script for Plan 1 (Performance Recovery)
cat > verification-scripts/verify_performance.sh << 'EOF'
#!/bin/bash
set -e

echo "🔍 Validating Agent 1: Performance Recovery"
echo "============================================="

# 1. Test Suite Validation
echo ""
echo "1️⃣ Test Suite Validation"
cargo test --no-fail-fast 2>&1 | tee /tmp/agent-1-tests.log
PASS_RATE=$(grep -oP '\d+(?= passed)' /tmp/agent-1-tests.log || echo "0")

if [ "$PASS_RATE" -lt 676 ]; then
    echo "❌ FAILED: Test pass rate $PASS_RATE/677 (required: ≥676)"
    exit 1
fi
echo "✅ Tests passing: $PASS_RATE/677"

# 2. Performance Regression Check
echo ""
echo "2️⃣ Performance Regression Check"
echo "Running benchmarks..."
cargo bench --bench performance_benchmarks 2>&1 | tee /tmp/agent-1-bench.log

# Compare against baseline
if [ -f .agent-state/baselines/performance_baseline.json ]; then
    python3 .agent-state/compare_benchmarks.py \
        --current=/tmp/agent-1-bench.log \
        --baseline=.agent-state/baselines/performance_baseline.json \
        --max-regression=0% || { echo "❌ Performance regressions detected"; exit 1; }
    echo "✅ No performance regressions"
else
    echo "⚠️  No baseline found - saving current as baseline"
    cp /tmp/agent-1-bench.log .agent-state/baselines/performance_baseline.json
fi

# 3. Mathematical Correctness (CRITICAL)
echo ""
echo "3️⃣ Mathematical Correctness (CRITICAL)"
cargo test --release -p mathhook-core --test correctness_suite || {
    echo "❌ CRITICAL FAILURE: Mathematical correctness regression"
    echo "⚠️  ROLLBACK REQUIRED - DO NOT MERGE"
    exit 1
}
echo "✅ Mathematical correctness maintained"

# 4. Expression Size Constraint
echo ""
echo "4️⃣ Expression Size Constraint"
SIZE=$(cargo test --release -p mathhook-core expression_size 2>&1 | grep "Expression size" | awk '{print $3}' || echo "unknown")
if [ "$SIZE" != "unknown" ] && [ "$SIZE" -le 48 ]; then
    echo "✅ Expression size: $SIZE bytes (≤48)"
else
    echo "❌ Expression size: $SIZE bytes (max: 48)"
    exit 1
fi

echo ""
echo "============================================="
echo "✅✅✅ AGENT 1: ALL VALIDATION PASSED ✅✅✅"
echo "============================================="
exit 0
EOF

chmod +x verification-scripts/verify_performance.sh

echo "✅ Validation scripts created"
```

---

## Step 3: Test Orchestration with Plan 1 (5 minutes)

```bash
# 1. Return to main repo
cd /Users/ahmedmashhour/Documents/work/math/mathhook

# 2. Create worktree for Agent 1 (Performance Recovery)
git worktree add worktrees/agent-1-performance agent-1/performance-recovery

# 3. Copy Plan 1 into agent workspace
cp .mathhook_sessions/gtm/PLAN_1_PERFORMANCE_RECOVERY.md \
   worktrees/agent-1-performance/PLAN.md

# 4. Spawn Agent 1 (in Claude Code)
# Open new terminal in worktrees/agent-1-performance
cd worktrees/agent-1-performance
# Then in Claude Code:
/sc:spawn rust-engineer "Execute Plan 1: Performance Recovery & Validation

Context: You are Agent 1 working in dedicated worktree.

Mission:
1. Read PLAN.md (PLAN_1_PERFORMANCE_RECOVERY.md)
2. Execute Waves 1-4 sequentially
3. Signal wave completion via commit messages: '✅ Wave X Complete'
4. After all waves complete, run: ../../worktrees/conductor/verification-scripts/verify_performance.sh

Constraints:
- Work ONLY in this worktree (no touching main repo)
- Maintain ≥676/677 test pass rate
- Zero performance regressions tolerated
- Mathematical correctness is CRITICAL (never compromise)

Reporting:
- After each wave: git commit with '✅ Wave X Complete' message
- Final validation: Run verify_performance.sh before declaring done
"

# Agent will execute Plan 1 in isolation
```

---

## Step 4: Validate Agent 1 Work (Before Merge)

```bash
# After Agent 1 signals completion, orchestrator validates

cd worktrees/conductor

# 1. Fetch Agent 1's work
git fetch origin agent-1/performance-recovery

# 2. Checkout Agent 1's branch (in conductor workspace)
git checkout agent-1/performance-recovery

# 3. Run validation
./verification-scripts/verify_performance.sh

# If validation passes:
# ✅✅✅ AGENT 1: ALL VALIDATION PASSED ✅✅✅

# 4. Merge to integration branch
git checkout integration
git merge --no-ff agent-1/performance-recovery -m "✅ Merge Agent 1: Performance Recovery

Acceptance Criteria Met:
- Tests: 676/677 passing ✅
- Performance: No regressions ✅
- Correctness: 100% ✅
- Expression Size: ≤48 bytes ✅

Validated-By: verify_performance.sh
Approved-By: Orchestrator
"

# 5. Tag milestone
git tag -a "phase-1-complete" -m "Phase 1: Performance Recovery Complete"

# 6. Push to remote
git push origin integration phase-1-complete
```

---

## Step 5: Scale to Parallel Execution (Phase 2)

```bash
# After Phase 1 gate passes, spawn Phase 2 agents in parallel

# 1. Create all agent worktrees
cd /Users/ahmedmashhour/Documents/work/math/mathhook

git worktree add worktrees/agent-2-educational agent-2/educational-integration
git worktree add worktrees/agent-3-python-api agent-3/python-api-production
git worktree add worktrees/agent-4-nodejs-api agent-4/nodejs-api-production
git worktree add worktrees/agent-7-core-math agent-7/core-math-features

# 2. Copy plans into each workspace
cp .mathhook_sessions/gtm/PLAN_2_EDUCATIONAL_INTEGRATION.md worktrees/agent-2-educational/PLAN.md
cp .mathhook_sessions/gtm/PLAN_3_PYTHON_API_PRODUCTION.md worktrees/agent-3-python-api/PLAN.md
cp .mathhook_sessions/gtm/PLAN_4_NODEJS_API_PRODUCTION.md worktrees/agent-4-nodejs-api/PLAN.md
cp .mathhook_sessions/gtm/PLAN_7_CORE_MATH_FEATURES.md worktrees/agent-7-core-math/PLAN.md

# 3. In Claude Code, spawn ALL 4 agents in SINGLE message (parallel execution)
/sc:spawn rust-engineer "Agent 2: Educational Integration - Execute PLAN.md in worktrees/agent-2-educational"
/sc:spawn python-expert "Agent 3: Python API Production - Execute PLAN.md in worktrees/agent-3-python-api (WAIT for Agent 2 Wave 3)"
/sc:spawn backend-architect "Agent 4: Node.js API Production - Execute PLAN.md in worktrees/agent-4-nodejs-api (WAIT for Agent 2 Wave 3)"
/sc:spawn rust-engineer "Agent 7: Core Math Features - Execute PLAN.md in worktrees/agent-7-core-math (CRITICAL PATH)"

# Agents will coordinate via Serena MCP for dependencies
```

---

## Monitoring & Coordination

### Check Agent Progress
```bash
# View all active worktrees
git worktree list

# Check agent commits
cd worktrees/agent-2-educational
git log --oneline -10

# View orchestrator state
cd worktrees/conductor
cat .agent-state/agent-progress.json
```

### Weekly Sync
```bash
# Run orchestrator sync script (create this)
cd worktrees/conductor
cat > orchestration-scripts/weekly_sync.sh << 'EOF'
#!/bin/bash
echo "📊 Weekly Agent Progress Report (Week $(date +%U))"
echo "================================================"

for agent in agent-{1,2,3,4,5,6,7}-*; do
    if [ -d "../../worktrees/$agent" ]; then
        cd "../../worktrees/$agent"
        echo ""
        echo "Agent: $agent"
        echo "  Branch: $(git branch --show-current)"
        echo "  Latest commit: $(git log -1 --oneline)"
        echo "  Files changed: $(git diff --stat origin/main | tail -1)"
        cd -
    fi
done
EOF

chmod +x orchestration-scripts/weekly_sync.sh
./orchestration-scripts/weekly_sync.sh
```

---

## Common Operations

### Cleanup Completed Agent
```bash
# After successful merge
git worktree remove worktrees/agent-1-performance
git branch -d agent-1/performance-recovery  # if fully merged
```

### Handle Failing Validation
```bash
# If validation fails
cd worktrees/conductor
git checkout agent-X/branch-name
./verification-scripts/verify_X.sh

# Review failures, communicate with agent
# Agent fixes issues, re-validates
```

### Emergency Rollback
```bash
# If integrated work breaks something
cd worktrees/conductor
git checkout integration
git revert <commit-sha> -m "Rollback Agent X: <reason>"
```

---

## Next Steps

1. **Complete Phase 1**: Validate orchestration system with Agent 1
2. **Refine validation scripts**: Based on Phase 1 learnings
3. **Launch Phase 2**: Spawn 4 parallel agents
4. **Monitor dependencies**: Agent 2 Wave 3 blocks Agents 3, 4
5. **Iterate weekly**: Run sync, address blockers

---

## Troubleshooting

### Worktree Creation Fails
```bash
# Ensure no existing worktree at path
git worktree list
git worktree remove <path>

# Retry creation
git worktree add <path> <branch>
```

### Validation Script Errors
```bash
# Debug validation
bash -x verification-scripts/verify_performance.sh

# Check paths, permissions
ls -la verification-scripts/
```

### Merge Conflicts
```bash
# Use AI-powered resolution (future implementation)
# Or manual resolution:
git merge --abort
# Coordinate with agent to resolve conflicts in their worktree
```

---

**Quick Start Complete!** You're ready to orchestrate parallel agents with git worktrees.

**Time to First Parallel Execution**: ~30 minutes setup + 4-6 weeks Plan 1
**Expected Timeline**: 48-64 weeks total (12-16 months) for all 7 plans
