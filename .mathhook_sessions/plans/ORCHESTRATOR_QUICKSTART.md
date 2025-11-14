# MathHook Orchestrator Quick Start Guide

**For the Second Claude Instance (Orchestrator)**

## Setup Status: ✅ COMPLETE

The orchestration infrastructure is fully operational and ready for agent execution.

---

## Step 1: Launch Command

Copy and paste this command to start Phase 1 (Performance Recovery):

```bash
/sc:spawn rust-engineer "Execute Wave-Based Performance Recovery & Validation Plan for MathHook - Read PLAN_1_PERFORMANCE_RECOVERY.md from .mathhook_sessions/gtm/"
```

**What this does**:
- Spawns a `rust-engineer` agent (specialized Rust expert)
- Provides the agent with the comprehensive performance recovery plan
- Agent will execute 5 waves: profiling, fixes, comprehensive benchmarks, SymPy validation, CI integration

---

## Step 2: What Happens Next

### Wave 1: Performance Profiling & Root Cause Analysis
- **Agent**: rust-engineer
- **Duration**: 3-4 days
- **Deliverables**: Profiling reports, bottleneck identification, regression analysis
- **Verification**: `./verify_performance_wave_1.sh`

### Wave 2: Performance Recovery
- **Agent**: rust-engineer
- **Duration**: 5-7 days
- **Deliverables**: Optimized code, benchmark improvements, validation reports
- **Verification**: `./verify_performance_wave_2.sh`

### Wave 3: Comprehensive Benchmark Suite
- **Agent**: rust-engineer
- **Duration**: 4-5 days
- **Deliverables**: Full coverage benchmarks (calculus, solving, educational, matrix, symbolic)
- **Verification**: `./verify_performance_wave_3.sh`

### Wave 3.5: SymPy Comparison Benchmarks
- **Agent**: rust-engineer
- **Duration**: 2-3 days
- **Deliverables**: Side-by-side comparison with SymPy, performance analysis
- **Verification**: `./verify_performance_wave_3.5.sh`

### Wave 4: CI Integration & Documentation
- **Agent**: rust-engineer
- **Duration**: 2-3 days
- **Deliverables**: GitHub Actions CI, performance regression detection, documentation
- **Verification**: `./verify_performance_wave_4.sh`

---

## Step 3: Monitor Progress

### Dashboard (Real-Time Status)

```bash
cd /Users/ahmedmashhour/Documents/work/math/mathhook/worktrees/conductor
./orchestration-scripts/dashboard.sh
```

### Weekly Sync Report (Comprehensive Progress)

```bash
cd /Users/ahmedmashhour/Documents/work/math/mathhook/worktrees/conductor
./orchestration-scripts/weekly_sync.sh
```

---

## Critical Files to Read

The orchestrator (you, the second Claude) MUST read these files **in order** before starting:

1. **Orchestration Methodology** (How to orchestrate):
   `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md`

2. **Project Rules** (MathHook constraints):
   `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md`

3. **Agent Specification** (rust-engineer capabilities):
   `/Users/ahmedmashhour/Documents/work/math/mathhook/.claude/agents/rust-engineer.md`

4. **Plan 1 Details** (Performance Recovery waves):
   `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/PLAN_1_PERFORMANCE_RECOVERY.md`

---

## The 5 Mandatory Rules

These are **NON-NEGOTIABLE**. Follow them strictly:

### Rule 1: You Are Always The Orchestrator
- **NEVER** implement directly
- **ALWAYS** delegate to agents
- Your role: Plan → Spawn → Monitor → Verify

### Rule 2: Sequential Waves, Parallel Agents
- Complete waves **in order**: Wave 1 → Wave 2 → Wave 3 → Wave 3.5 → Wave 4
- Within a wave, you CAN spawn multiple agents in parallel
- Never skip waves

### Rule 3: Mandatory Verification
- Each wave **must** end with verification script execution
- Verification scripts are in `.mathhook_sessions/` (e.g., `verify_performance_wave_1.sh`)
- Do **NOT** proceed to next wave until verification passes

### Rule 4: Strict CLAUDE.md Enforcement
- **32-byte Expression** constraint (hard limit)
- **16-byte Number** constraint (hard limit)
- All agents MUST respect these constraints
- If agent violates constraints, stop and fix immediately

### Rule 5: Maintain Momentum
- Report after **each wave**
- Update progress in conductor worktree
- Keep me (the user) informed of status

---

## Expected Timeline

**Total Duration**: 4-5 weeks (16-22 days)

| Wave | Duration | Critical Path |
|------|----------|---------------|
| Wave 1 | 3-4 days | Profiling + root cause analysis |
| Wave 2 | 5-7 days | Performance fixes + validation |
| Wave 3 | 4-5 days | Comprehensive benchmarks |
| Wave 3.5 | 2-3 days | SymPy comparison |
| Wave 4 | 2-3 days | CI integration + docs |

**Buffer**: 2-3 days for unexpected issues

---

## Success Criteria

### Wave 1 (Profiling)
✅ Flamegraphs and profiling reports generated
✅ Regression hotspots identified
✅ Root cause analysis documented
✅ Verification script passes

### Wave 2 (Recovery)
✅ Performance regressions fixed (back to baseline)
✅ No new regressions introduced
✅ Benchmarks show improvement
✅ Verification script passes

### Wave 3 (Comprehensive Benchmarks)
✅ Benchmarks cover ALL core functionality:
  - ✅ Calculus (derivatives, integrals)
  - ✅ Solving (linear, quadratic, polynomial, systems)
  - ✅ Educational (step-by-step explanations)
  - ✅ Matrix operations
  - ✅ Symbolic manipulation
  - ✅ Simplification
  - ✅ Parser throughput
✅ All benchmarks registered in `Cargo.toml`
✅ Verification script passes

### Wave 3.5 (SymPy Comparison)
✅ Side-by-side benchmarks with SymPy
✅ Performance comparison documented
✅ Competitive analysis complete
✅ Verification script passes

### Wave 4 (CI Integration)
✅ GitHub Actions workflow created
✅ Benchmark regression detection automated
✅ CI runs on every PR
✅ Documentation updated
✅ Verification script passes

---

## What to Expect

### Agent Behavior
- The `rust-engineer` agent will:
  - Read the plan carefully
  - Execute tasks methodically
  - Run profiling and benchmarking tools
  - Write verification scripts
  - Document findings and optimizations
  - Report progress regularly

### Your Role as Orchestrator
- Monitor agent progress
- Ensure adherence to CLAUDE.md constraints
- Run verification scripts after each wave
- Coordinate between waves
- Keep documentation updated
- Report to user after each wave

---

## Troubleshooting

### If Agent Gets Stuck
1. Check if it read CLAUDE.md
2. Verify it's following the plan
3. Check for constraint violations (32-byte Expression, 16-byte Number)
4. Re-spawn agent with clearer instructions if needed

### If Verification Fails
1. Review agent's implementation
2. Check test output for specifics
3. Have agent fix issues before proceeding
4. Re-run verification script

### If Performance Gets Worse
1. Have agent revert changes
2. Analyze what went wrong
3. Try alternative optimization approach
4. Consider incremental optimization instead

---

## Communication Protocol

### After Each Wave
Send me a progress report with:
1. **Wave Completed**: Which wave finished
2. **Verification Status**: Pass/Fail
3. **Key Achievements**: What was accomplished
4. **Issues Encountered**: Any problems
5. **Next Steps**: What's coming in next wave

### Template

```markdown
# Wave [N] Progress Report

**Status**: ✅ COMPLETE / ⚠️ IN PROGRESS / ❌ BLOCKED

## Verification
- Script: `verify_performance_wave_[N].sh`
- Status: ✅ PASS / ❌ FAIL
- Details: [Summary of test results]

## Key Achievements
- [Achievement 1]
- [Achievement 2]
- [Achievement 3]

## Metrics
- [Relevant performance metrics]
- [Test coverage]
- [Benchmark results]

## Issues & Resolutions
- [Issue 1]: [How it was resolved]
- [Issue 2]: [How it was resolved]

## Next Wave Preview
- Wave [N+1]: [Brief description]
- Estimated Duration: [X days]
- Key Focus: [Primary objectives]
```

---

## Ready to Launch? ✅

If you've read this far, you're ready to begin. Execute the launch command:

```bash
/sc:spawn rust-engineer "Execute Wave-Based Performance Recovery & Validation Plan for MathHook - Read PLAN_1_PERFORMANCE_RECOVERY.md from .mathhook_sessions/gtm/"
```

**Good luck!** 🚀
