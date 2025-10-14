# MathHook 0.1 Multi-Agent Orchestration Plan

**Manager**: Primary Claude Code Instance
**Session Directory**: `.mathhook_sessions/`
**Status**: READY TO LAUNCH

---

## Agent Team Structure

### Wave 1: Critical Blockers (P0) - 6 Specialized Agents

**Agent P0-1: Pattern Matching Architect**
- **Primary Task**: P0-1 - Implement pattern matching & substitution system
- **Estimated Duration**: 2-3 weeks
- **Output File**: `AGENT_P0_1_PATTERN_MATCHING_LOG.md`
- **Deliverables**:
  - `crates/mathhook-core/src/pattern/mod.rs`
  - `crates/mathhook-core/src/pattern/substitution.rs`
  - `crates/mathhook-core/src/pattern/matching.rs`
  - `crates/mathhook-core/tests/pattern_tests.rs`
  - 50+ tests passing

**Agent P0-2: Polynomial Solver Fixer**
- **Primary Task**: P0-2 - Fix polynomial solver fake roots
- **Estimated Duration**: 1 day
- **Output File**: `AGENT_P0_2_POLYNOMIAL_SOLVER_LOG.md`
- **Deliverables**:
  - Fixed `crates/mathhook-core/src/algebra/solvers/polynomial.rs`
  - Added `SolverResult::Partial` variant
  - Removed all fake root generation
  - Tests validating correctness

**Agent P0-3: Doctest Healer**
- **Primary Task**: P0-3 - Fix all 103 failing doctests
- **Estimated Duration**: 1 week
- **Output File**: `AGENT_P0_3_DOCTEST_FIXES_LOG.md`
- **Deliverables**:
  - Fixed doctests in 20+ files
  - `cargo test --doc -p mathhook-core` shows 0 failures
  - All imports added, signatures corrected

**Agent P0-4: Number Safety Engineer**
- **Primary Task**: P0-4 - Add number type overflow handling
- **Estimated Duration**: 1 week
- **Output File**: `AGENT_P0_4_NUMBER_OVERFLOW_LOG.md`
- **Deliverables**:
  - `impl Add/Sub/Mul/Div for Number` with checked arithmetic
  - Overflow → BigInt promotion
  - 20+ overflow tests passing

**Agent P0-5: Domain Guardian**
- **Primary Task**: P0-5 - Implement domain error system
- **Estimated Duration**: 3-4 days
- **Output File**: `AGENT_P0_5_DOMAIN_ERRORS_LOG.md`
- **Deliverables**:
  - `crates/mathhook-core/src/error.rs` with `MathError` enum
  - Replaced all symbolic "undefined" with errors
  - 20+ domain error tests passing

**Agent P0-6: Code Quality Enforcer**
- **Primary Task**: P2-1 - Remove emojis and ALL CAPS
- **Estimated Duration**: 2-3 days
- **Output File**: `AGENT_P0_6_CODE_QUALITY_LOG.md`
- **Deliverables**:
  - Zero emojis in codebase
  - No ALL CAPS except constants
  - All files cleaned and reformatted

---

### Wave 2: High Priority (P1) - 5 Specialized Agents

**Agent P1-1: Registry Refactorer**
- **Primary Task**: P1-1 - Refactor hardcoded function matching
- **Estimated Duration**: 1-2 weeks
- **Output File**: `AGENT_P1_1_REGISTRY_REFACTOR_LOG.md`
- **Prerequisites**: Wave 1 complete
- **Deliverables**:
  - Enhanced `UniversalFunctionRegistry`
  - Refactored `simplify/functions.rs`
  - Refactored `calculus/derivatives/chain_rule.rs`

**Agent P1-2: Complex Numbers Specialist**
- **Primary Task**: P1-2 - Complete complex number arithmetic
- **Estimated Duration**: 3-5 days
- **Output File**: `AGENT_P1_2_COMPLEX_ARITHMETIC_LOG.md`
- **Prerequisites**: P0-5 (error system)
- **Deliverables**:
  - Complex arithmetic operations
  - conjugate, abs, arg methods
  - Polar conversion
  - 20+ complex tests passing

**Agent P1-3: Integration Master**
- **Primary Task**: P1-3 - Complete integration table
- **Estimated Duration**: 1 week
- **Output File**: `AGENT_P1_3_INTEGRATION_LOG.md`
- **Prerequisites**: P0-1 (substitution for integration)
- **Deliverables**:
  - Complete elementary integrals
  - Integration by parts (simple cases)
  - 30+ integration tests

**Agent P1-4: System Solver Engineer**
- **Primary Task**: P1-4 - System of linear equations solver
- **Estimated Duration**: 1 week
- **Output File**: `AGENT_P1_4_SYSTEM_SOLVER_LOG.md`
- **Prerequisites**: None (independent)
- **Deliverables**:
  - Gaussian elimination implementation
  - Matrix-based solving
  - System solver tests

**Agent P1-5: Validation Specialist**
- **Primary Task**: P1-5 - SymPy cross-validation suite
- **Estimated Duration**: Ongoing (100 tests to start)
- **Output File**: `AGENT_P1_5_SYMPY_VALIDATION_LOG.md`
- **Prerequisites**: Wave 1 complete (need working features to validate)
- **Deliverables**:
  - 100+ validation tests
  - Test categories: simplification, derivatives, integration, solving
  - Documented discrepancies

---

### Wave 3: Cleanup (P2) - 2 Specialized Agents

**Agent P2-1: Module Organizer**
- **Primary Task**: P2-2 - Split large modules
- **Estimated Duration**: 1 week
- **Output File**: `AGENT_P2_1_MODULE_SPLIT_LOG.md`
- **Prerequisites**: Wave 1 & 2 complete
- **Deliverables**:
  - 6 large files split into logical sub-modules
  - All modules under 500 lines
  - No broken functionality

**Agent P2-2: Error Handler**
- **Primary Task**: P2-3 - Replace panics with Results
- **Estimated Duration**: 2-3 days
- **Output File**: `AGENT_P2_2_PANIC_REMOVAL_LOG.md`
- **Prerequisites**: P0-5 (error system exists)
- **Deliverables**:
  - ~10 panics replaced with proper errors
  - Updated function signatures
  - Tests updated

---

## Orchestration Protocol

### Phase 1: Infrastructure Setup (Manager - Me)
1. Create progress tracking files
2. Create agent log templates
3. Establish communication protocol
4. Set up milestone checkpoints

### Phase 2: Wave 1 Launch (Critical Blockers)
1. **Launch all P0 agents in parallel** (6 agents)
2. Manager monitors progress via log files
3. Agents report blockers immediately
4. Manager resolves dependencies/conflicts
5. Milestone: All P0 tasks complete

### Phase 3: Wave 2 Launch (High Priority)
1. Verify Wave 1 completion
2. **Launch all P1 agents in parallel** (5 agents)
3. Manager coordinates dependencies
4. Agents validate against each other's work
5. Milestone: Core CAS functionality complete

### Phase 4: Wave 3 Launch (Cleanup)
1. Verify Wave 1 & 2 completion
2. **Launch P2 agents** (2 agents)
3. Final cleanup and polish
4. Milestone: Codebase ready for 0.1

### Phase 5: Final Review (Manager)
1. Run full test suite
2. Verify all checklists complete
3. Cross-validate agent deliverables
4. Generate 0.1 release report
5. **MILESTONE: READY FOR 0.1 RELEASE**

---

## Agent Communication Protocol

### Agent Log File Structure
Each agent maintains a log file with this structure:

```markdown
# Agent [ID]: [Name]
**Task**: [Task ID from AI_AGENT.md]
**Status**: [NOT_STARTED | IN_PROGRESS | BLOCKED | COMPLETE]
**Progress**: X%
**Started**: [timestamp]
**Last Update**: [timestamp]

## Current Objective
[What the agent is working on right now]

## Completed Work
- [x] Subtask 1
- [x] Subtask 2
- [ ] Subtask 3 (in progress)

## Blockers
- [If any - report to manager immediately]

## Files Modified
- path/to/file1.rs
- path/to/file2.rs

## Tests Added/Fixed
- test_name_1 ✅
- test_name_2 ✅

## Next Steps
1. [Next immediate action]
2. [Following action]

## Questions for Manager
- [Any clarifications needed]

## Verification Checklist
- [ ] All tests pass
- [ ] No regressions
- [ ] Documentation updated
- [ ] Code follows CLAUDE.md guidelines
```

### Manager Check-in Schedule
- **Daily**: Read all agent logs, update progress tracker
- **On Blocker**: Immediately assist blocked agent
- **On Milestone**: Verify completion, launch next wave
- **On Conflict**: Resolve merge conflicts, coordinate agents

---

## Progress Tracking System

### Master Progress File: `PROGRESS_TRACKER.md`

Tracks overall completion across all waves:
- Wave 1 (P0): X/6 agents complete
- Wave 2 (P1): X/5 agents complete
- Wave 3 (P2): X/2 agents complete
- Overall: X% complete
- Estimated time remaining: X weeks

### Milestone Markers
- ✅ Wave 1 Complete: All critical blockers resolved
- ✅ Wave 2 Complete: Core CAS functionality implemented
- ✅ Wave 3 Complete: Codebase polished and clean
- ✅ 0.1 Release Ready: All verification passed

---

## Agent Task Dependencies

```
P0-1 (Pattern Matching)
  └─> P1-3 (Integration - needs substitution)

P0-5 (Domain Errors)
  ├─> P1-2 (Complex - needs error types)
  └─> P2-2 (Panic Removal - needs error types)

Wave 1 Complete
  └─> P1-5 (SymPy Validation - needs working features)

Wave 1 & 2 Complete
  └─> P2-1 (Module Splitting - needs stability)
```

**Critical Path**: P0-1 (Pattern Matching) is the longest task (2-3 weeks) and blocks P1-3 (Integration)

---

## Manager Responsibilities

### Before Launch
- [x] Create orchestration infrastructure
- [ ] Create all agent log templates
- [ ] Create progress tracker
- [ ] Verify `.mathhook_sessions/` is organized
- [ ] Brief all agents on protocol

### During Execution
- [ ] Monitor agent logs daily
- [ ] Unblock stuck agents immediately
- [ ] Coordinate dependencies (P0-1 → P1-3, P0-5 → P1-2/P2-2)
- [ ] Run integration tests between agents' work
- [ ] Resolve merge conflicts
- [ ] Validate deliverables

### Milestone Reviews
- [ ] Wave 1 Review: Verify all P0 tasks complete
- [ ] Wave 2 Review: Verify all P1 tasks complete
- [ ] Wave 3 Review: Verify all P2 tasks complete
- [ ] Final Review: Generate 0.1 release report

### On Completion
- [ ] Run full test suite: `cargo test --all`
- [ ] Run all doctests: `cargo test --doc`
- [ ] Verify 0 failing tests
- [ ] Generate release notes
- [ ] Mark 0.1 READY FOR RELEASE

---

## Launch Commands

### Wave 1 Launch (6 agents in parallel)
```bash
# Manager executes:
# Launch P0-1: Pattern Matching Architect
# Launch P0-2: Polynomial Solver Fixer
# Launch P0-3: Doctest Healer
# Launch P0-4: Number Safety Engineer
# Launch P0-5: Domain Guardian
# Launch P0-6: Code Quality Enforcer
```

### Wave 2 Launch (5 agents in parallel)
```bash
# After Wave 1 completion, manager executes:
# Launch P1-1: Registry Refactorer
# Launch P1-2: Complex Numbers Specialist
# Launch P1-3: Integration Master
# Launch P1-4: System Solver Engineer
# Launch P1-5: Validation Specialist
```

### Wave 3 Launch (2 agents in parallel)
```bash
# After Wave 2 completion, manager executes:
# Launch P2-1: Module Organizer
# Launch P2-2: Error Handler
```

---

## Success Criteria

### Wave 1 Success (Critical Blockers Resolved)
- [ ] Pattern matching & substitution system works
- [ ] Polynomial solver returns NO fake roots
- [ ] 0 failing doctests (was 103)
- [ ] Number overflow handled with BigInt promotion
- [ ] Domain errors properly implemented
- [ ] 0 emojis, proper capitalization throughout

### Wave 2 Success (Core CAS Complete)
- [ ] All functions use registry (no hardcoded matching)
- [ ] Complex number arithmetic fully operational
- [ ] Integration table complete with by-parts
- [ ] System linear equation solver works
- [ ] 100+ SymPy validation tests passing

### Wave 3 Success (Codebase Polished)
- [ ] All modules under 500 lines
- [ ] 0 panics in library code (only Result returns)
- [ ] Code quality at release standard

### Final Success (0.1 READY)
- [ ] All tests pass (target: 1000+ tests)
- [ ] All doctests pass
- [ ] Mathematical correctness validated vs SymPy
- [ ] Documentation accurate and working
- [ ] CLAUDE.md compliance 100%
- [ ] No critical TODOs remaining
- [ ] Performance benchmarks acceptable

---

## Timeline Estimate

**Wave 1** (P0): 2-3 weeks
- Bottleneck: P0-1 Pattern Matching (2-3 weeks)
- Others complete faster, assist P0-1 if needed

**Wave 2** (P1): 2-3 weeks
- Bottleneck: P1-1 Registry Refactor (1-2 weeks)
- P1-5 Validation ongoing throughout

**Wave 3** (P2): 1 week
- Parallel cleanup tasks

**Total**: 6-8 weeks (optimistic) to 10-12 weeks (realistic with testing)

---

## Emergency Protocol

### If Agent Gets Blocked
1. Agent updates log with `Status: BLOCKED`
2. Agent describes blocker in detail
3. Manager reads within 1 hour
4. Manager either:
   - Provides guidance
   - Reassigns subtask
   - Adjusts dependencies
   - Provides additional context

### If Tests Fail After Merge
1. Manager runs bisect to find breaking agent
2. Manager notifies agent
3. Agent fixes immediately (highest priority)
4. Other agents pause to avoid conflicts

### If Milestone Delayed
1. Manager assesses critical path
2. Reassign resources to bottleneck
3. Defer non-critical tasks
4. Update timeline estimate

---

## Ready to Launch

**Status**: Infrastructure designed and ready
**Next Step**: Manager creates log templates and progress tracker
**Then**: Launch Wave 1 (6 agents) on user command

**User Command to Start**: "Launch Wave 1" or "Start agents"
