# Phase 4: Registry Population - Agent Instructions

**Date Created**: 2025-10-13 06:50:00
**Status**: READY TO START
**Prerequisites**: ALL MET ✅

---

## Critical: Read This Before Starting Phase 4

This document provides exact instructions for the 3 parallel agents that will populate the integral registry. Each agent has a specific, non-overlapping responsibility.

**Previous Work Completed**:
- ✅ Phase 1: Type system implemented (AntiderivativeRule, AntiderivativeRuleType, ConstantOfIntegration)
- ✅ Phase 2: Test infrastructure created (36 tests, 26 passing, 10 awaiting your work)
- ✅ Phase 3: Refactoring analysis complete (all 18 functions cataloged)

**Your Mission**: Register antiderivative rules for 18 functions to enable registry-based integration

---

## Agent A: Simple Functions (6 functions)

### Your Sole Responsibility
Register antiderivative rules for the 6 simplest integral functions using `AntiderivativeRuleType::Simple`.

### Functions Assigned to You
1. **sin** → `-cos(x)`
2. **cos** → `sin(x)`
3. **exp** → `exp(x)`
4. **sinh** → `cosh(x)`
5. **cosh** → `sinh(x)`
6. **sqrt** → `(2/3)x^(3/2)`

### Files You Will Modify

1. **crates/mathhook-core/src/functions/elementary/trigonometric.rs**
   - Find `sin` function definition (search for `pub fn sin_function()`)
   - Update the `ElementaryProperties` field `antiderivative_rule` from `None` to:
   ```rust
   antiderivative_rule: Some(AntiderivativeRule {
       rule_type: AntiderivativeRuleType::Simple {
           antiderivative_fn: "cos".to_string(),
           coefficient: Expression::integer(-1),
       },
       result_template: "∫sin(x)dx = -cos(x) + C".to_string(),
       constant_handling: ConstantOfIntegration::AddConstant,
   }),
   ```

   - Find `cos` function definition
   - Update `antiderivative_rule` to:
   ```rust
   antiderivative_rule: Some(AntiderivativeRule {
       rule_type: AntiderivativeRuleType::Simple {
           antiderivative_fn: "sin".to_string(),
           coefficient: Expression::integer(1),
       },
       result_template: "∫cos(x)dx = sin(x) + C".to_string(),
       constant_handling: ConstantOfIntegration::AddConstant,
   }),
   ```

2. **crates/mathhook-core/src/functions/elementary/exponential.rs**
   - Find `exp` function definition
   - Update `antiderivative_rule` to:
   ```rust
   antiderivative_rule: Some(AntiderivativeRule {
       rule_type: AntiderivativeRuleType::Simple {
           antiderivative_fn: "exp".to_string(),
           coefficient: Expression::integer(1),
       },
       result_template: "∫exp(x)dx = exp(x) + C".to_string(),
       constant_handling: ConstantOfIntegration::AddConstant,
   }),
   ```

3. **crates/mathhook-core/src/functions/elementary/hyperbolic.rs**
   - Find `sinh` function definition
   - Update `antiderivative_rule` to:
   ```rust
   antiderivative_rule: Some(AntiderivativeRule {
       rule_type: AntiderivativeRuleType::Simple {
           antiderivative_fn: "cosh".to_string(),
           coefficient: Expression::integer(1),
       },
       result_template: "∫sinh(x)dx = cosh(x) + C".to_string(),
       constant_handling: ConstantOfIntegration::AddConstant,
   }),
   ```

   - Find `cosh` function definition
   - Update `antiderivative_rule` to:
   ```rust
   antiderivative_rule: Some(AntiderivativeRule {
       rule_type: AntiderivativeRuleType::Simple {
           antiderivative_fn: "sinh".to_string(),
           coefficient: Expression::integer(1),
       },
       result_template: "∫cosh(x)dx = sinh(x) + C".to_string(),
       constant_handling: ConstantOfIntegration::AddConstant,
   }),
   ```

4. **For sqrt**: This is a special case (power rule). For now, set to:
   ```rust
   antiderivative_rule: Some(AntiderivativeRule {
       rule_type: AntiderivativeRuleType::NonElementary,
       result_template: "∫√x dx = (2/3)x^(3/2) + C (requires power rule)".to_string(),
       constant_handling: ConstantOfIntegration::AddConstant,
   }),
   ```
   Note: sqrt will be properly implemented in Phase 4 Agent B with custom evaluator.

### Verification Commands

After making changes, run:

```bash
# Check compilation
cargo check -p mathhook-core

# Run integral registry tests
cargo test -p mathhook-core --test integral_registry_tests

# Run function property tests
cargo test -p mathhook-core --lib properties
```

### Success Criteria

**Before your work**:
- Ignored tests: 10
- Passing tests: 26

**After your work** (expected):
- Ignored tests: 5 (sin, cos, exp, sinh, cosh tests should now pass)
- Passing tests: 31

**Report Format**:
```
Agent A: Simple Functions - COMPLETE ✅

Files Modified:
- trigonometric.rs: sin, cos (lines X-Y)
- exponential.rs: exp (lines X-Y)
- hyperbolic.rs: sinh, cosh (lines X-Y)

Verification:
- cargo check: PASS ✅
- cargo test integral_registry_tests: 31 passed; 0 failed; 5 ignored ✅

Functions Registered:
✅ sin   → -cos(x)
✅ cos   → sin(x)
✅ exp   → exp(x)
✅ sinh  → cosh(x)
✅ cosh  → sinh(x)
⏭ sqrt  → deferred to Agent B (power rule needed)
```

### What NOT to Do
- ❌ Do NOT modify function_integrals.rs (that's Phase 5)
- ❌ Do NOT implement complex functions (that's Agent C)
- ❌ Do NOT create new test files (tests already exist)
- ❌ Do NOT modify any agent B or C functions (tan, sec, ln, etc.)

---

## Agent B: Medium Complexity Functions (4 functions)

### Your Sole Responsibility
Register antiderivative rules for medium-complexity functions that require expression composition (2-3 levels deep).

### Functions Assigned to You
1. **tan** → `-ln|cos(x)|`
2. **cot** → `ln|sin(x)|`
3. **tanh** → `ln(cosh(x))`
4. **sqrt** → `(2/3)x^(3/2)` (deferred from Agent A)

### Implementation Strategy

These functions require **custom evaluators** because their antiderivatives involve function composition. You cannot use `AntiderivativeRuleType::Simple`.

**Read**: `/.mathhook_sessions/PHASE_3_ANALYSIS_FUNCTION_INTEGRALS_REFACTORING.md` lines 490-506 for exact implementation details.

### Example Implementation (for tan)

**File**: `crates/mathhook-core/src/functions/elementary/trigonometric.rs`

**Current code** (find the tan function, line ~XXX):
```rust
ElementaryProperties {
    // ... other fields
    antiderivative_rule: None,
}
```

**Change to**:
```rust
ElementaryProperties {
    // ... other fields
    antiderivative_rule: Some(AntiderivativeRule {
        rule_type: AntiderivativeRuleType::Custom,
        result_template: "∫tan(x)dx = -ln|cos(x)| + C".to_string(),
        constant_handling: ConstantOfIntegration::AddConstant,
    }),
}
```

**IMPORTANT**: For Phase 4, we're using `AntiderivativeRuleType::Custom` without evaluator. The evaluator will be added in Phase 5 when we refactor `function_integrals.rs`. For now, the registry just needs to KNOW that tan has an antiderivative rule.

### Files You Will Modify

1. **crates/mathhook-core/src/functions/elementary/trigonometric.rs**
   - tan (line ~XXX)
   - cot (line ~XXX)

2. **crates/mathhook-core/src/functions/elementary/hyperbolic.rs**
   - tanh (line ~XXX)

3. **For sqrt**: Create a proper power rule implementation
   - You may need to add custom handling or mark as `Custom` with proper template

### Registry Entries to Add

**tan**:
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫tan(x)dx = -ln|cos(x)| + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**cot**:
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫cot(x)dx = ln|sin(x)| + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**tanh**:
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫tanh(x)dx = ln(cosh(x)) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**sqrt**:
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫√x dx = (2/3)x^(3/2) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

### Verification Commands

```bash
# Check compilation
cargo check -p mathhook-core

# Run integral registry tests
cargo test -p mathhook-core --test integral_registry_tests

# Run function property tests
cargo test -p mathhook-core --lib properties
```

### Success Criteria

**Before your work** (after Agent A completes):
- Ignored tests: 5
- Passing tests: 31

**After your work** (expected):
- Ignored tests: 1 (only composite tests remain)
- Passing tests: 35

**Report Format**:
```
Agent B: Medium Complexity Functions - COMPLETE ✅

Files Modified:
- trigonometric.rs: tan, cot (lines X-Y)
- hyperbolic.rs: tanh (lines X-Y)

Verification:
- cargo check: PASS ✅
- cargo test integral_registry_tests: 35 passed; 0 failed; 1 ignored ✅

Functions Registered:
✅ tan   → -ln|cos(x)|
✅ cot   → ln|sin(x)|
✅ tanh  → ln(cosh(x))
✅ sqrt  → (2/3)x^(3/2)
```

### What NOT to Do
- ❌ Do NOT implement evaluator functions yet (Phase 5)
- ❌ Do NOT modify function_integrals.rs (Phase 5)
- ❌ Do NOT implement Agent C's high-complexity functions

---

## Agent C: High Complexity Functions (6 functions)

### Your Sole Responsibility
Register antiderivative rules for the most complex integral functions (by-parts and complex compositions).

### Functions Assigned to You
1. **sec** → `ln|sec(x)+tan(x)|`
2. **csc** → `-ln|csc(x)+cot(x)|`
3. **ln** → `x·ln(x) - x`
4. **log** → `(1/ln(10))·(x·ln(x) - x)`
5. **arcsin** → `x·arcsin(x) + √(1-x²)`
6. **arccos** → `x·arccos(x) - √(1-x²)`
7. **arctan** → `x·arctan(x) - ½ln(1+x²)`

### Implementation Strategy

These functions use **integration by parts** or **very complex compositions** (4-5 levels deep).

**Read**: `/.mathhook_sessions/PHASE_3_ANALYSIS_FUNCTION_INTEGRALS_REFACTORING.md` lines 590-646 for mathematical background.

### Registry Entry Pattern

For Phase 4, mark these as `ByParts` or `Custom` without evaluators (evaluators added in Phase 5):

**ln** (by-parts result):
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::ByParts {
        u_pattern: "ln(x)".to_string(),
        dv_pattern: "1".to_string(),
    },
    result_template: "∫ln(x)dx = x·ln(x) - x + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**log** (base conversion + by-parts):
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫log(x)dx = (1/ln(10))·(x·ln(x) - x) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**arcsin** (by-parts result):
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::ByParts {
        u_pattern: "arcsin(x)".to_string(),
        dv_pattern: "1".to_string(),
    },
    result_template: "∫arcsin(x)dx = x·arcsin(x) + √(1-x²) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**arccos** (by-parts result):
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::ByParts {
        u_pattern: "arccos(x)".to_string(),
        dv_pattern: "1".to_string(),
    },
    result_template: "∫arccos(x)dx = x·arccos(x) - √(1-x²) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**arctan** (by-parts result):
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::ByParts {
        u_pattern: "arctan(x)".to_string(),
        dv_pattern: "1".to_string(),
    },
    result_template: "∫arctan(x)dx = x·arctan(x) - ½ln(1+x²) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**sec**:
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫sec(x)dx = ln|sec(x)+tan(x)| + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**csc**:
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom,
    result_template: "∫csc(x)dx = -ln|csc(x)+cot(x)| + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

### Files You Will Modify

1. **crates/mathhook-core/src/functions/elementary/trigonometric.rs**
   - sec (find function definition)
   - csc (find function definition)

2. **crates/mathhook-core/src/functions/elementary/logarithmic.rs**
   - ln (find function definition)
   - log (find function definition)

3. **crates/mathhook-core/src/functions/elementary/inverse_trig.rs** (or wherever inverse trig is defined)
   - arcsin
   - arccos
   - arctan

### Verification Commands

```bash
# Check compilation
cargo check -p mathhook-core

# Run integral registry tests
cargo test -p mathhook-core --test integral_registry_tests

# Run function property tests
cargo test -p mathhook-core --lib properties
```

### Success Criteria

**Before your work** (after Agent B completes):
- Ignored tests: 1
- Passing tests: 35

**After your work** (expected):
- Ignored tests: 0 (all registry population tests should pass)
- Passing tests: 36 (all tests passing)

**Report Format**:
```
Agent C: High Complexity Functions - COMPLETE ✅

Files Modified:
- trigonometric.rs: sec, csc (lines X-Y)
- logarithmic.rs: ln, log (lines X-Y)
- inverse_trig.rs: arcsin, arccos, arctan (lines X-Y)

Verification:
- cargo check: PASS ✅
- cargo test integral_registry_tests: 36 passed; 0 failed; 0 ignored ✅

Functions Registered:
✅ sec    → ln|sec(x)+tan(x)|
✅ csc    → -ln|csc(x)+cot(x)|
✅ ln     → x·ln(x) - x
✅ log    → (1/ln(10))·(x·ln(x) - x)
✅ arcsin → x·arcsin(x) + √(1-x²)
✅ arccos → x·arccos(x) - √(1-x²)
✅ arctan → x·arctan(x) - ½ln(1+x²)
```

### What NOT to Do
- ❌ Do NOT implement evaluator functions yet (Phase 5)
- ❌ Do NOT modify function_integrals.rs (Phase 5)
- ❌ Do NOT modify tests (they're already correct)

---

## Orchestration Instructions for Next Session

### Launch Command

When ready to start Phase 4, launch ALL 3 agents IN PARALLEL:

```
Launch Agent A, Agent B, and Agent C simultaneously with separation of concerns:

Agent A: Simple functions (sin, cos, exp, sinh, cosh)
Agent B: Medium complexity (tan, cot, tanh, sqrt)
Agent C: High complexity (sec, csc, ln, log, arcsin, arccos, arctan)

Each agent:
1. Reads PHASE_4_AGENT_INSTRUCTIONS.md (their section)
2. Reads PHASE_3_ANALYSIS (for function details)
3. Modifies assigned function intelligence files
4. Runs verification tests
5. Reports exact test counts
```

### Verification After All Agents Complete

Run comprehensive verification:

```bash
# Full test suite
cargo test -p mathhook-core

# Integral registry specifically
cargo test -p mathhook-core --test integral_registry_tests

# Expected result: 36 passed; 0 failed; 0 ignored
```

### Success Gate for Phase 5

**Proceed to Phase 5 ONLY IF**:
- [x] All 3 agents completed successfully
- [x] All 18 functions have `antiderivative_rule: Some(...)`
- [x] cargo test integral_registry_tests shows: 36 passed; 0 failed; 0 ignored
- [x] cargo check passes with no errors
- [x] No CLAUDE.md violations introduced

---

## Common Pitfalls to Avoid

### For All Agents

1. ❌ **Don't hardcode function names in implementation logic**
   - ✅ DO: Register rules in function intelligence modules
   - ❌ DON'T: Add match statements or hardcoded checks

2. ❌ **Don't implement evaluator functions yet**
   - Phase 4 is REGISTRATION only
   - Evaluators are Phase 5 (when refactoring function_integrals.rs)

3. ❌ **Don't modify function_integrals.rs**
   - That file will be refactored in Phase 5
   - Your job is ONLY to populate the registry

4. ❌ **Don't estimate test results**
   - Always run `cargo test` and report EXACT counts
   - No "should pass" or "probably works"

5. ❌ **Don't create new test files**
   - Tests already exist in integral_registry_tests.rs
   - They're currently ignored, waiting for your registry entries

### CLAUDE.md Compliance

Each agent must verify:
- [ ] No inline `//` comments added (except formulas)
- [ ] All `antiderivative_rule` entries use proper type variants
- [ ] No emojis in code (only in test output/reports)
- [ ] No ALL CAPS in code
- [ ] Registry pattern used (no hardcoded matches)

---

## After Phase 4 Completion

### Update Session Documentation

After all 3 agents complete, update:

1. **INTEGRAL_REGISTRY_SESSION_LOG.md**
   - Add Phase 4 completion timestamp
   - Document agent assignments and results
   - Update status: Phase 4 COMPLETE ✅

2. **WAVE_2_VERIFICATION_CHECKERS.md**
   - Update test counts
   - Mark integral registry work as progressed

### Next Phase Preview

**Phase 5: Refactoring function_integrals.rs**
- Replace 171 lines of hardcoded match with registry lookups
- Implement evaluator functions for complex rules
- Verify zero regressions (all existing behavior preserved)
- Estimated time: 6-9 hours

---

## Questions and Troubleshooting

### Q: What if I can't find the function definition?
**A**: Use grep to locate it:
```bash
rg "pub fn sin_function" crates/mathhook-core/
rg "ElementaryProperties" crates/mathhook-core/src/functions/elementary/
```

### Q: What if compilation fails after my changes?
**A**:
1. Check that you imported `AntiderivativeRule`, `AntiderivativeRuleType`, `ConstantOfIntegration`
2. Verify syntax (missing commas, brackets)
3. Run `cargo check` to see exact error
4. DO NOT proceed to next function until compilation succeeds

### Q: What if tests still show "ignored" after my changes?
**A**:
1. Verify you actually changed `antiderivative_rule` from `None` to `Some(...)`
2. Check that function name matches exactly (case-sensitive)
3. Run tests with `--nocapture` to see details: `cargo test integral_registry_tests -- --nocapture`

### Q: Can I do Agent B's work if I'm Agent A?
**A**: NO. Strict separation of concerns. Each agent does ONLY their assigned functions.

---

**Document End**

**Status**: Ready for Phase 4 execution
**Next Action**: Launch 3 parallel agents using instructions above
**Estimated Duration**: 2-3 hours (parallel execution)
