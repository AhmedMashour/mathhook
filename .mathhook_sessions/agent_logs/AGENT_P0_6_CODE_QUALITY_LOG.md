# Agent P0-6: Code Quality Enforcer

**Task**: P0-6 - Remove All Emojis and Fix ALL CAPS
**Status**: NOT_STARTED
**Progress**: 0%
**Priority**: P0 (CODE QUALITY)
**Estimated Duration**: 2-3 days
**Started**: -
**Last Update**: -

---

## Mission Briefing

Remove all emojis and fix ALL CAPS violations across the codebase. This is a CLAUDE.md compliance requirement for code quality standards.

**Current Problem**:
- 200+ emoji instances throughout codebase
- 100+ ALL CAPS violations in comments/docs
- Violates CLAUDE.md documentation standards

**CLAUDE.md Requirement**: "No emojis anywhere (code, comments, documentation, commit messages). No ALL CAPS except for constants."

**Reference Material**:
- Task details: `.mathhook_sessions/0.1_RELEASE_READINESS_AI_AGENT.md` (TASK P2-1)
- Documentation Standards: `CLAUDE.md` (Prohibited Content section)

---

## Current Objective

Waiting for launch command...

---

## Implementation Plan

### Phase 1: Locate All Violations (Day 1 Morning)
- [ ] Find all files with emojis: `rg "[\x{1F300}-\x{1F9FF}]" crates/ --files-with-matches > emoji_files.txt`
- [ ] Find ALL CAPS abuse: `rg "^[\s]*//[/!]? [A-Z\s]{10,}" crates/ --type rust > caps_violations.txt`
- [ ] Categorize by severity/file
- [ ] Create systematic cleanup plan

### Phase 2: Remove Emojis (Day 1-2)
- [ ] `src/algebra/solvers.rs` - remove emojis
- [ ] `src/algebra/solvers/linear.rs` - remove emojis
- [ ] `src/educational/message_registry.rs` - remove emojis (careful: user-facing messages!)
- [ ] `src/educational/enhanced_steps.rs` - remove emojis
- [ ] `src/serialize.rs` - remove emojis
- [ ] `benchmarks/benches/comprehensive_performance_suite.rs` - remove emojis
- [ ] `src/core/performance/profiler.rs` - remove emojis
- [ ] All other files flagged in emoji_files.txt
- [ ] Rewrite comments in plain, clear English

### Phase 3: Fix ALL CAPS (Day 2-3)
- [ ] Fix module-level documentation (//!)
- [ ] Fix function documentation (///)
- [ ] Fix inline comments (//)
- [ ] Preserve constants like `MAX_DEPTH`, `PI`, etc.
- [ ] Rewrite in proper sentence case

### Phase 4: Verification (Day 3)
- [ ] Run emoji search again: should return 0 results
- [ ] Run ALL CAPS search: should only return const names
- [ ] Verify code still compiles: `cargo build`
- [ ] Verify all tests pass: `cargo test -p mathhook-core`
- [ ] Review all changes for clarity

---

## Completed Work

_Nothing yet - waiting for launch_

---

## Files Affected (Partial List)

### Algebra Files
- [ ] `src/algebra/solvers.rs`
- [ ] `src/algebra/solvers/linear.rs`

### Educational Files
- [ ] `src/educational/message_registry.rs` (CRITICAL: user-facing messages)
- [ ] `src/educational/enhanced_steps.rs`
- [ ] `src/educational/step_by_step.rs`

### Core Files
- [ ] `src/core/performance/profiler.rs`
- [ ] `src/serialize.rs`

### Benchmark Files
- [ ] `benchmarks/benches/comprehensive_performance_suite.rs`

### Other Files
- [ ] Additional files discovered during search

---

## Example Fixes

### Example 1: Module Documentation
```rust
// BEFORE:
//! 🎯 EQUATION SOLVERS MODULE - MODERN RUST STRUCTURE
//!
//! This module provides POWERFUL EQUATION SOLVING capabilities

// AFTER:
//! Equation solvers module with modern Rust structure
//!
//! This module provides powerful equation solving capabilities
```

### Example 2: Function Documentation
```rust
// BEFORE:
/// 🧠 SMART SOLVER - Detects equation patterns
///
/// This function is REALLY IMPORTANT for solving equations

// AFTER:
/// Smart solver that detects equation patterns
///
/// This function is important for solving equations
```

### Example 3: Inline Comments
```rust
// BEFORE:
// 🎓 STEP-BY-STEP INTEGRATION (CRITICAL USER REQUIREMENT)
// THIS IS SUPER IMPORTANT!

// AFTER:
// Step-by-step integration (critical user requirement)
// This is important for user experience.
```

### Example 4: Preserve Constants
```rust
// BEFORE (CORRECT - keep as is):
const MAX_DEPTH: usize = 100;
const PI: f64 = 3.14159;

// These are fine - constants should be ALL CAPS
```

---

## Search Commands

### Find All Emojis
```bash
# Search for any emoji Unicode characters
rg "[\x{1F300}-\x{1F9FF}]" crates/mathhook-core/src --files-with-matches

# Or more comprehensive emoji range:
rg "[\x{1F000}-\x{1FFFF}]" crates/ --type rust
```

### Find ALL CAPS Violations
```bash
# Find comments with 10+ consecutive capitals
rg "^[\s]*//[/!]? [A-Z\s]{10,}" crates/mathhook-core/src --type rust

# Find documentation with excessive caps
rg "^[\s]*///? [A-Z\s]{15,}" crates/ --type rust
```

### Verification (Should Return Nothing)
```bash
# After cleanup, these should return 0 results:
rg "[\x{1F300}-\x{1F9FF}]" crates/ --type rust
rg "🎯|🧠|🎓|✨|🚀|💡|⚠️" crates/ --type rust
```

---

## Tests Status

**Current**: Tests should pass with emojis/caps present
**Target**: Tests pass with all violations removed

### Verification Tests
- [ ] `cargo build` - code compiles
- [ ] `cargo test -p mathhook-core` - all tests pass
- [ ] `cargo clippy` - no new warnings
- [ ] `cargo fmt --check` - formatting unchanged

---

## Special Considerations

### User-Facing Messages
**CRITICAL**: Files like `educational/message_registry.rs` may contain emojis in user-facing messages. Handle carefully:
- If emojis are in strings shown to users, discuss with manager first
- May need to preserve some educational emojis if they're part of UX
- Definitely remove from comments/docs though

### Constants
Preserve ALL CAPS for actual constant names:
```rust
const MAX_ITERATIONS: usize = 1000;  // ✓ KEEP
const PI: f64 = 3.14159;             // ✓ KEEP
const EULER: f64 = 2.71828;          // ✓ KEEP
```

---

## Progress Tracking

### Emojis Removed
- [ ] Total emoji instances: ~200
- [ ] Removed so far: 0
- [ ] Files cleaned: 0

### ALL CAPS Fixed
- [ ] Total violations: ~100
- [ ] Fixed so far: 0
- [ ] Files cleaned: 0

---

## Blockers

**Current Blockers**: None

**Potential Blocker**: User-facing emoji usage in educational messages
- _If found, document here and ask manager for guidance_

---

## Next Steps

1. Await launch command
2. Run comprehensive emoji search
3. Run ALL CAPS search
4. Begin systematic cleanup, file by file
5. Test frequently to ensure no breakage

---

## Questions for Manager

- Should educational messages shown to users keep emojis, or remove those too?
- Any specific files to prioritize or avoid?

---

## Verification Checklist

When marking COMPLETE, verify:
- [ ] Zero emoji characters in entire codebase
- [ ] No ALL CAPS except constant names (MAX_*, PI, etc.)
- [ ] All comments rewritten in clear, professional English
- [ ] `cargo build` succeeds
- [ ] `cargo test -p mathhook-core` passes (no regressions)
- [ ] `cargo clippy` has no new warnings
- [ ] Code is more professional and readable
- [ ] CLAUDE.md compliance achieved
- [ ] Git diff reviewed for any accidental changes

---

**Agent Status**: STANDBY - Ready to launch
**Impact**: Code quality, professional appearance, CLAUDE.md compliance
**Scope**: 200+ emojis, 100+ ALL CAPS violations across multiple files
