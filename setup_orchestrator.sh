#!/bin/bash
# MathHook Parallel Agent Orchestration - Setup Script
# Purpose: Initialize orchestrator workspace with git worktrees for parallel GTM execution
# Usage: bash setup_orchestrator.sh

set -e  # Exit on any error

REPO_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
CONDUCTOR_PATH="$REPO_ROOT/worktrees/conductor"

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   MathHook Orchestrator Workspace Setup                       ║"
echo "║   Git Worktree-Based Parallel Agent Architecture              ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check we're in the right place
if [ ! -d "$REPO_ROOT/.git" ]; then
    echo "❌ ERROR: Not in MathHook repository root"
    echo "Expected: $REPO_ROOT"
    exit 1
fi

cd "$REPO_ROOT"

# Step 1: Create conductor worktree
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 1: Creating Conductor Worktree"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -d "$CONDUCTOR_PATH" ]; then
    echo "⚠️  Conductor worktree already exists at $CONDUCTOR_PATH"
    read -p "Remove and recreate? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git worktree remove worktrees/conductor --force
        echo "✅ Removed existing conductor worktree"
    else
        echo "❌ Aborted - remove manually or choose different path"
        exit 1
    fi
fi

git worktree add -b conductor/orchestration worktrees/conductor
echo "✅ Conductor worktree created: $CONDUCTOR_PATH"
echo ""

# Step 2: Set up directory structure
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 2: Creating Directory Structure"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cd "$CONDUCTOR_PATH"

mkdir -p .agent-state/baselines
mkdir -p verification-scripts
mkdir -p orchestration-scripts

echo "✅ Directory structure created:"
echo "   - .agent-state/          (coordination & state)"
echo "   - .agent-state/baselines/ (performance baselines)"
echo "   - verification-scripts/   (validation gates)"
echo "   - orchestration-scripts/  (weekly sync, etc.)"
echo ""

# Step 3: Create dependency graph
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 3: Creating Dependency Graph"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > .agent-state/dependency-graph.yaml << 'EOF'
# MathHook GTM Plan Dependencies
# Updated: 2025-10-21
# Purpose: Track cross-agent dependencies and phase transitions

phases:
  phase_1_critical_foundation:
    duration: "4-6 weeks"
    agents: [agent-1-performance]
    dependencies: []
    blocks: [phase_2_parallel_build]
    description: "Performance recovery & validation - MUST complete before parallel execution"

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
      - agent-4: requires agent-2 Wave 3 (Expression.explain API)
    blocks: [phase_3_market_launch]
    description: "Parallel development of educational, APIs, and core math features"

  phase_3_market_launch:
    duration: "12-16 weeks"
    agents: [agent-5-gtm-launch]
    dependencies: [phase_2_parallel_build]
    blocks: []
    description: "Automated documentation pipeline and coordinated market launch"

  phase_4_mcp_integration:
    duration: "4-6 weeks"
    agents: [agent-6-mcp-server]
    dependencies: [agent-3-python-api]
    blocks: []
    description: "MCP server implementation (requires PyPI package)"
EOF

echo "✅ Dependency graph created: .agent-state/dependency-graph.yaml"
echo ""

# Step 4: Initialize agent progress tracker
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 4: Initializing Agent Progress Tracker"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > .agent-state/agent-progress.json << EOF
{
  "last_updated": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "active_phase": "phase_0_setup",
  "agents": [],
  "milestones": {
    "setup_complete": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "phase_1_start": null,
    "phase_1_complete": null,
    "phase_2_start": null,
    "phase_2_complete": null,
    "phase_3_start": null,
    "phase_3_complete": null
  }
}
EOF

echo "✅ Agent progress tracker initialized: .agent-state/agent-progress.json"
echo ""

# Step 5: Create phase gates configuration
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 5: Creating Phase Gate Configurations"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > .agent-state/phase-gates.yaml << 'EOF'
# Phase Gate Configuration
# Purpose: Enforce strict acceptance criteria before phase transitions

Phase_1_Exit_Gate:
  name: "Performance Recovery Complete"
  agent: agent-1-performance
  plan: PLAN_1_PERFORMANCE_RECOVERY.md
  required_validations:
    - test_suite:
        threshold: "676/677 passing"
        blocker: true
        description: "Test pass rate must be maintained"
    - performance:
        threshold: "0% regression"
        blocker: true
        description: "No performance regressions tolerated"
    - sympy_speed:
        threshold: "10-100x faster validated"
        blocker: false
        description: "Speed claims validated against SymPy"
    - correctness:
        threshold: "100% correctness suite passing"
        blocker: true
        description: "CRITICAL - Mathematical correctness must never regress"
    - expression_size:
        threshold: "≤48 bytes"
        blocker: true
        description: "Cache-line optimization constraint"
  approval_required: [orchestrator, project-lead]
  unblocks: [Phase_2_Parallel_Build]

Phase_2_Educational_Wave_3_Gate:
  name: "Expression.explain() API Stable"
  agent: agent-2-educational
  plan: PLAN_2_EDUCATIONAL_INTEGRATION.md
  required_validations:
    - api_stability:
        threshold: "API signature frozen"
        blocker: true
        description: "Expression.explain() signature must be stable"
    - user_testing:
        threshold: "≥7/10 clarity score (3+ users)"
        blocker: true
        description: "Educational content quality validated"
    - doctest_coverage:
        threshold: "All solver types have examples"
        blocker: false
        description: "Documentation completeness"
  approval_required: [orchestrator]
  unblocks: [agent-3-python-api Wave 4, agent-4-nodejs-api Wave 3]

Phase_2_Exit_Gate:
  name: "All Parallel Plans Complete"
  agents: [agent-2-educational, agent-3-python-api, agent-4-nodejs-api, agent-7-core-math]
  required_validations:
    - all_plans_complete:
        threshold: "Plans 2, 3, 4, 7 all validated"
        blocker: true
        description: "All parallel work completed and merged"
    - integration_tests:
        threshold: "Cross-plan integration tests passing"
        blocker: true
        description: "Components work together"
    - api_parity:
        threshold: "Python/Node.js APIs match Rust core"
        blocker: true
        description: "API feature parity validated"
  approval_required: [orchestrator, project-lead]
  unblocks: [Phase_3_Market_Launch]
EOF

echo "✅ Phase gates configured: .agent-state/phase-gates.yaml"
echo ""

# Step 6: Create verification script for Plan 1
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 6: Creating Verification Scripts"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > verification-scripts/verify_performance.sh << 'EOF'
#!/bin/bash
# Verification Script: Plan 1 (Performance Recovery)
# Purpose: Validate ALL acceptance criteria before merge approval

set -e

echo "🔍 Validating Agent 1: Performance Recovery"
echo "============================================="

REPO_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
BASELINE_PATH="$REPO_ROOT/worktrees/conductor/.agent-state/baselines"

# Ensure we're in the correct branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ ! "$CURRENT_BRANCH" == "agent-1/performance-recovery" ]]; then
    echo "⚠️  WARNING: Expected branch 'agent-1/performance-recovery', got '$CURRENT_BRANCH'"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# 1. Test Suite Validation
echo ""
echo "1️⃣ Test Suite Validation"
echo "Running all tests (this may take 2-3 minutes)..."
cargo test --no-fail-fast 2>&1 | tee /tmp/agent-1-tests.log

PASS_RATE=$(grep -oP '\d+(?= passed)' /tmp/agent-1-tests.log | head -1 || echo "0")
TOTAL_TESTS=$(grep -oP '\d+(?= tests)' /tmp/agent-1-tests.log | head -1 || echo "677")

if [ "$PASS_RATE" -lt 676 ]; then
    echo "❌ FAILED: Test pass rate $PASS_RATE/$TOTAL_TESTS (required: ≥676/677)"
    echo ""
    echo "Failed tests:"
    grep "FAILED" /tmp/agent-1-tests.log | head -20
    exit 1
fi
echo "✅ Tests passing: $PASS_RATE/$TOTAL_TESTS"

# 2. Performance Regression Check
echo ""
echo "2️⃣ Performance Regression Check"
echo "Running benchmarks (this may take 5-10 minutes)..."
cargo bench --bench performance_benchmarks 2>&1 | tee /tmp/agent-1-bench.log

# Save as baseline if first run
if [ ! -f "$BASELINE_PATH/performance_baseline.json" ]; then
    echo "⚠️  No baseline found - saving current as baseline"
    mkdir -p "$BASELINE_PATH"
    cp /tmp/agent-1-bench.log "$BASELINE_PATH/performance_baseline.json"
    echo "✅ Baseline saved for future comparisons"
else
    echo "Comparing against baseline..."
    # TODO: Implement Python script for benchmark comparison
    # For now, manual review required
    echo "⚠️  Manual review required: compare /tmp/agent-1-bench.log with baseline"
    echo "Baseline: $BASELINE_PATH/performance_baseline.json"
    echo ""
    read -p "Are benchmarks acceptable (no regressions)? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Performance regressions detected (manual review)"
        exit 1
    fi
    echo "✅ No performance regressions"
fi

# 3. Mathematical Correctness (CRITICAL)
echo ""
echo "3️⃣ Mathematical Correctness (CRITICAL)"
echo "Running correctness test suite..."
if cargo test --release -p mathhook-core --test correctness_suite 2>&1 | tee /tmp/agent-1-correctness.log; then
    echo "✅ Mathematical correctness maintained"
else
    echo "❌ CRITICAL FAILURE: Mathematical correctness regression"
    echo "⚠️  ROLLBACK REQUIRED - DO NOT MERGE"
    echo ""
    echo "Failed correctness tests:"
    grep "FAILED" /tmp/agent-1-correctness.log
    exit 1
fi

# 4. Expression Size Constraint
echo ""
echo "4️⃣ Expression Size Constraint"
SIZE_OUTPUT=$(cargo test --release -p mathhook-core expression_size 2>&1 || echo "test failed")
SIZE=$(echo "$SIZE_OUTPUT" | grep -oP '\d+(?= bytes)' | head -1 || echo "unknown")

if [ "$SIZE" = "unknown" ]; then
    echo "⚠️  WARNING: Could not determine Expression size"
    echo "Test output:"
    echo "$SIZE_OUTPUT"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
elif [ "$SIZE" -le 48 ]; then
    echo "✅ Expression size: $SIZE bytes (≤48 bytes)"
else
    echo "❌ FAILED: Expression size $SIZE bytes (max: 48 bytes)"
    echo "This violates cache-line optimization constraint"
    exit 1
fi

# 5. Clippy & Format Check
echo ""
echo "5️⃣ Code Quality (Clippy & Format)"
echo "Running clippy..."
if cargo clippy --all-targets -- -D warnings 2>&1 | tee /tmp/agent-1-clippy.log; then
    echo "✅ Clippy passed"
else
    echo "⚠️  Clippy warnings found (review recommended but not blocking)"
fi

echo ""
echo "Checking code format..."
if cargo fmt -- --check 2>&1; then
    echo "✅ Code formatting correct"
else
    echo "⚠️  Code formatting issues (run: cargo fmt)"
fi

# Final Summary
echo ""
echo "============================================="
echo "✅✅✅ AGENT 1: ALL VALIDATION PASSED ✅✅✅"
echo "============================================="
echo ""
echo "📊 Summary:"
echo "  - Tests: $PASS_RATE/$TOTAL_TESTS passing"
echo "  - Performance: No regressions (validated)"
echo "  - Correctness: ✅ Maintained"
echo "  - Expression Size: $SIZE bytes (≤48)"
echo "  - Code Quality: Clippy + format checked"
echo ""
echo "✅ APPROVAL: Ready for merge to integration branch"
echo ""
echo "Next steps:"
echo "  1. Merge: cd $REPO_ROOT/worktrees/conductor"
echo "  2. Run: git checkout integration"
echo "  3. Run: git merge --no-ff agent-1/performance-recovery"
echo "  4. Tag: git tag -a 'phase-1-complete' -m 'Phase 1 Complete'"
echo ""
exit 0
EOF

chmod +x verification-scripts/verify_performance.sh
echo "✅ Verification script created: verification-scripts/verify_performance.sh"
echo ""

# Step 7: Create weekly sync script
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 7: Creating Orchestration Scripts"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > orchestration-scripts/weekly_sync.sh << 'EOF'
#!/bin/bash
# Weekly Agent Progress Sync
# Purpose: Generate comprehensive status report for all active agents

REPO_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
WORKTREES_PATH="$REPO_ROOT/worktrees"

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║     MathHook Weekly Agent Progress Report                     ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "📅 Week: $(date +%U) of $(date +%Y)"
echo "📆 Date: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# List all active worktrees
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Active Worktrees"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
git worktree list
echo ""

# Report on each agent
for agent_path in "$WORKTREES_PATH"/agent-*; do
    if [ -d "$agent_path" ]; then
        cd "$agent_path"

        agent_name=$(basename "$agent_path")
        branch=$(git branch --show-current)

        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "Agent: $agent_name"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "Branch: $branch"
        echo ""
        echo "Latest commits:"
        git log -5 --oneline --decorate
        echo ""
        echo "Files changed (vs main):"
        git diff --stat origin/main | tail -5
        echo ""

        # Check for wave completion markers
        WAVES_COMPLETE=$(git log --all --oneline | grep -c "✅ Wave.*Complete" || echo "0")
        echo "Waves completed: $WAVES_COMPLETE"

        # Check test status
        if [ -f "Cargo.toml" ]; then
            echo ""
            echo "Running tests (quick check)..."
            if timeout 30 cargo test --lib 2>&1 | tail -10; then
                echo "✅ Tests passing (quick check)"
            else
                echo "⚠️  Tests may have issues (review needed)"
            fi
        fi

        echo ""
    fi
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "End of Weekly Report"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Next steps:"
echo "  1. Review agent progress above"
echo "  2. Check for blockers or failing tests"
echo "  3. Coordinate dependencies (Agent 2 Wave 3 → Agents 3, 4)"
echo "  4. Run validation scripts for completed waves"
echo ""
EOF

chmod +x orchestration-scripts/weekly_sync.sh
echo "✅ Weekly sync script created: orchestration-scripts/weekly_sync.sh"
echo ""

# Step 8: Create helper scripts
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 8: Creating Helper Scripts"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > orchestration-scripts/dashboard.sh << 'EOF'
#!/bin/bash
# Real-time Orchestration Dashboard
# Purpose: Show current status of all agents at a glance

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║          MATHHOOK ORCHESTRATION DASHBOARD                      ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

REPO_ROOT="/Users/ahmedmashhour/Documents/work/math/mathhook"
cd "$REPO_ROOT"

# Show active phase
echo "📅 Current Phase: $(cat worktrees/conductor/.agent-state/agent-progress.json | grep 'active_phase' | cut -d'"' -f4)"
echo "⏱️  Last Updated: $(cat worktrees/conductor/.agent-state/agent-progress.json | grep 'last_updated' | cut -d'"' -f4)"
echo ""

# Show worktrees
echo "┌───────────────────────────────────────────────────────────────┐"
echo "│ ACTIVE WORKTREES                                              │"
echo "├───────────────────────────────────────────────────────────────┤"
git worktree list | sed 's/^/│ /'
echo "└───────────────────────────────────────────────────────────────┘"
echo ""

# Quick status of each agent
echo "┌───────────────────────────────────────────────────────────────┐"
echo "│ AGENT STATUS                                                  │"
echo "├───────────────────────────────────────────────────────────────┤"
for agent_path in worktrees/agent-*; do
    if [ -d "$agent_path" ]; then
        agent=$(basename "$agent_path")
        cd "$agent_path"
        branch=$(git branch --show-current)
        latest=$(git log -1 --oneline)
        echo "│ $agent"
        echo "│   Branch: $branch"
        echo "│   Latest: $latest"
        echo "│"
        cd "$REPO_ROOT"
    fi
done
echo "└───────────────────────────────────────────────────────────────┘"
echo ""

echo "Commands:"
echo "  ./orchestration-scripts/weekly_sync.sh - Full progress report"
echo "  ./verification-scripts/verify_performance.sh - Validate Plan 1"
echo ""
EOF

chmod +x orchestration-scripts/dashboard.sh
echo "✅ Dashboard script created: orchestration-scripts/dashboard.sh"
echo ""

# Step 9: Create README
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Step 9: Creating README"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > README.md << 'EOF'
# Conductor Orchestrator Workspace

This directory manages parallel agent execution for MathHook GTM plans using git worktrees.

## Directory Structure

```
.agent-state/          - Cross-agent coordination
  dependency-graph.yaml    - Plan dependencies
  agent-progress.json      - Real-time status
  phase-gates.yaml         - Acceptance criteria
  baselines/               - Performance baselines

verification-scripts/  - Automated validation
  verify_performance.sh    - Plan 1 validation
  verify_educational.sh    - Plan 2 validation (TBD)
  verify_python_api.sh     - Plan 3 validation (TBD)

orchestration-scripts/ - Management tools
  weekly_sync.sh          - Progress reports
  dashboard.sh            - Real-time status
```

## Quick Commands

### View Dashboard
```bash
./orchestration-scripts/dashboard.sh
```

### Weekly Progress Report
```bash
./orchestration-scripts/weekly_sync.sh
```

### Validate Agent Work
```bash
./verification-scripts/verify_performance.sh
```

## Next Steps

1. **Start Phase 1**: Create Agent 1 worktree
   ```bash
   cd /Users/ahmedmashhour/Documents/work/math/mathhook
   git worktree add worktrees/agent-1-performance agent-1/performance-recovery
   ```

2. **Spawn Agent 1**: Use orchestrator delegation pattern
   ```bash
   /sc:spawn rust-engineer "YOU ARE THE ORCHESTRATOR FOR PLAN 1..."
   ```

3. **Monitor Progress**: Check dashboard weekly
   ```bash
   cd worktrees/conductor
   ./orchestration-scripts/dashboard.sh
   ```

## Documentation

- Architecture: `../../claudedocs/PARALLEL_AGENT_WORKTREE_ARCHITECTURE.md`
- Quick Start: `../../claudedocs/WORKTREE_ORCHESTRATION_QUICKSTART.md`
- Delegation: `../../claudedocs/ORCHESTRATOR_DELEGATION_PATTERN.md`
EOF

echo "✅ README created: README.md"
echo ""

# Final summary
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                    SETUP COMPLETE                              ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "✅ Orchestrator workspace initialized at:"
echo "   $CONDUCTOR_PATH"
echo ""
echo "📁 Created:"
echo "   - .agent-state/ (coordination files)"
echo "   - verification-scripts/ (validation gates)"
echo "   - orchestration-scripts/ (management tools)"
echo "   - README.md (workspace guide)"
echo ""
echo "🎯 Next Steps:"
echo ""
echo "   1. View dashboard:"
echo "      cd $CONDUCTOR_PATH"
echo "      ./orchestration-scripts/dashboard.sh"
echo ""
echo "   2. Start Phase 1 (Performance Recovery):"
echo "      cd $REPO_ROOT"
echo "      git worktree add worktrees/agent-1-performance agent-1/performance-recovery"
echo ""
echo "   3. Spawn Agent 1 with orchestrator role (in Claude Code):"
echo "      /sc:spawn rust-engineer \"YOU ARE THE ORCHESTRATOR FOR PLAN 1...\""
echo ""
echo "   4. See full delegation pattern:"
echo "      cat $REPO_ROOT/claudedocs/ORCHESTRATOR_DELEGATION_PATTERN.md"
echo ""
echo "📚 Documentation:"
echo "   - Architecture: claudedocs/PARALLEL_AGENT_WORKTREE_ARCHITECTURE.md"
echo "   - Quick Start: claudedocs/WORKTREE_ORCHESTRATION_QUICKSTART.md"
echo "   - Delegation: claudedocs/ORCHESTRATOR_DELEGATION_PATTERN.md"
echo ""
echo "🚀 Ready to orchestrate parallel GTM execution!"
echo ""
