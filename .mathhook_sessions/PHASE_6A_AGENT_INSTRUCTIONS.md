# Phase 6A Agent Instructions: Registry Optimization

**Date**: 2025-10-13
**Phase**: 6A - Registry Optimization (Store Expression Objects Directly)
**Prerequisite**: Phase 5 Complete (Refactoring done, tests passing)
**Goal**: Eliminate 237 lines of helper functions by storing `Expression` objects in registry

---

## Executive Summary

**Goal**: Optimize registry architecture by storing actual `Expression` objects (or builder closures) instead of unit variants, eliminating the need for 237 lines of helper construction functions.

**Orchestration Method**: **Wave-by-wave** with verification gates (Phase 5 methodology)

**Total Waves**: 3
- **Wave 1**: Update type system (AntiderivativeRule to store Expression builders)
- **Wave 2**: Update 16 function registrations (Phase 4 revisit)
- **Wave 3**: Simplify function_integrals.rs (remove 237 lines of helpers)

**Success Criteria**:
- ✅ Registry stores Expression builders (Box<dyn Fn(Symbol) -> Expression>)
- ✅ All 16 functions re-registered with Expression objects
- ✅ Helper functions removed from function_integrals.rs (~237 lines)
- ✅ Zero test regressions (26 passed; 0 failed; 10 ignored maintained)
- ✅ Target: function_integrals.rs from 473 → ~200 lines (~270 line reduction)
- ✅ CLAUDE.md 100% compliant

---

## Architecture Change Overview

### Current Architecture (Phase 5)

**AntiderivativeRuleType** (unit variants):
```rust
pub enum AntiderivativeRuleType {
    Simple { antiderivative_fn: String, coefficient: Expression },
    NonElementary { result_expr: Expression },  // Placeholder
    ByParts { result_expr: Expression },        // Placeholder
    // ...
}
```

**Problem**: `result_expr` is just a placeholder. Actual construction happens in 237 lines of helper functions in function_integrals.rs.

### Target Architecture (Phase 6A)

**AntiderivativeRuleType** (with Expression builders):
```rust
pub enum AntiderivativeRuleType {
    Simple { antiderivative_fn: String, coefficient: Expression },
    Custom {
        builder: Box<dyn Fn(Symbol) -> Expression + Send + Sync>
    },
    // LinearSubstitution, TrigSubstitution, PartialFractions remain for future use
}
```

**Benefit**: Registry stores the actual construction logic. No helper functions needed.

**Result**: function_integrals.rs simplified from 473 → ~200 lines.

---

## Wave 1: Type System Update

### Agent F: Type System Architect

**Responsibility**: Update `AntiderivativeRule` and `AntiderivativeRuleType` to support Expression builders

**Complexity**: MEDIUM
**Estimated Time**: 1-2 hours
**Risk Level**: MEDIUM (type system changes affect Phase 4 registrations)

---

### Step 1: Update AntiderivativeRuleType Enum

**File**: `crates/mathhook-core/src/functions/properties.rs`
**Location**: Find `AntiderivativeRuleType` enum definition (around lines 224-284 based on Phase 1)

**Current Definition** (Phase 5):
```rust
pub enum AntiderivativeRuleType {
    Simple {
        antiderivative_fn: String,
        coefficient: Expression,
    },
    NonElementary {
        result_expr: Expression,  // Currently unused placeholder
    },
    ByParts {
        result_expr: Expression,  // Currently unused placeholder
    },
    LinearSubstitution {
        // ... future use
    },
    TrigSubstitution {
        // ... future use
    },
    PartialFractions {
        // ... future use
    },
}
```

**New Definition** (Phase 6A):
```rust
/// Type representing different strategies for computing antiderivatives
///
/// Each variant stores the necessary information to construct the antiderivative
/// expression given an integration variable.
#[derive(Clone)]
pub enum AntiderivativeRuleType {
    /// Simple antiderivative: ∫f(x)dx = coefficient * F(x)
    ///
    /// Example: ∫sin(x)dx = -1 * cos(x)
    Simple {
        antiderivative_fn: String,
        coefficient: Expression,
    },

    /// Custom antiderivative with expression builder
    ///
    /// Stores a closure that constructs the antiderivative expression
    /// given the integration variable. Used for complex expressions like
    /// tan, cot, sec, csc, ln, log, arcsin, arccos, arctan, tanh, sqrt.
    ///
    /// Example: ∫tan(x)dx = builder(x) → -ln|cos(x)|
    Custom {
        #[allow(clippy::type_complexity)]
        builder: Arc<dyn Fn(Symbol) -> Expression + Send + Sync>,
    },

    /// Linear substitution pattern (future use)
    LinearSubstitution {
        coefficient: Expression,
        inner_rule: Box<AntiderivativeRule>,
    },

    /// Trigonometric substitution pattern (future use)
    TrigSubstitution {
        substitution_type: String,
    },

    /// Partial fractions decomposition (future use)
    PartialFractions {
        decomposition: Vec<Expression>,
    },
}
```

**Key Changes**:
1. **Merged** `NonElementary` and `ByParts` into single `Custom` variant
2. **Replaced** `result_expr: Expression` with `builder: Arc<dyn Fn(Symbol) -> Expression>`
3. **Added** `Arc` for cheap cloning (important for registry)
4. **Added** `Send + Sync` bounds for thread safety
5. **Kept** `Simple` unchanged (already optimal)
6. **Kept** future variants for extensibility

**Why Arc instead of Box**:
- Registry is shared across threads
- `Arc` allows cheap cloning of the builder
- `dyn Fn` requires `Send + Sync` for thread safety

---

### Step 2: Implement Clone for AntiderivativeRuleType

**Challenge**: `dyn Fn` is not `Clone` by default

**Solution**: Manual Clone implementation

**Add After Enum Definition**:
```rust
impl Clone for AntiderivativeRuleType {
    fn clone(&self) -> Self {
        match self {
            AntiderivativeRuleType::Simple { antiderivative_fn, coefficient } => {
                AntiderivativeRuleType::Simple {
                    antiderivative_fn: antiderivative_fn.clone(),
                    coefficient: coefficient.clone(),
                }
            }
            AntiderivativeRuleType::Custom { builder } => {
                AntiderivativeRuleType::Custom {
                    builder: Arc::clone(builder),
                }
            }
            AntiderivativeRuleType::LinearSubstitution { coefficient, inner_rule } => {
                AntiderivativeRuleType::LinearSubstitution {
                    coefficient: coefficient.clone(),
                    inner_rule: inner_rule.clone(),
                }
            }
            AntiderivativeRuleType::TrigSubstitution { substitution_type } => {
                AntiderivativeRuleType::TrigSubstitution {
                    substitution_type: substitution_type.clone(),
                }
            }
            AntiderivativeRuleType::PartialFractions { decomposition } => {
                AntiderivativeRuleType::PartialFractions {
                    decomposition: decomposition.clone(),
                }
            }
        }
    }
}
```

---

### Step 3: Add Required Imports

**File**: `crates/mathhook-core/src/functions/properties.rs`
**Location**: Top of file (imports section)

**Add**:
```rust
use std::sync::Arc;
use crate::core::{Expression, Symbol};
```

**Note**: `Expression` and `Symbol` may already be imported. Verify and add `Arc` if missing.

---

### Step 4: Remove #[derive(Clone)] from Enum

**Current**:
```rust
#[derive(Clone)]
pub enum AntiderivativeRuleType {
    // ...
}
```

**Change To**:
```rust
pub enum AntiderivativeRuleType {
    // ...
}
```

**Reason**: We implement `Clone` manually due to `Arc<dyn Fn>`.

---

### Verification (Wave 1)

```bash
# 1. Compilation check
cargo check -p mathhook-core

# Expected: Compilation errors in function_integrals.rs (expected - Wave 3 will fix)
# Expected: Compilation errors in Phase 4 registration files (expected - Wave 2 will fix)

# This is NORMAL. Wave 1 changes the type system.
# Wave 2 will update registrations.
# Wave 3 will update function_integrals.rs.
```

**Agent F Deliverables**:
1. Modified `properties.rs` with new `AntiderivativeRuleType`
2. Manual `Clone` implementation
3. Required imports added
4. Compilation status report (expect errors - document them)
5. Log: `agent_logs/AGENT_P1_6A_WAVE1_TYPE_SYSTEM_LOG.md`

**Agent F Must Report**:
- Enum definition updated: `Custom` variant with `Arc<dyn Fn>`
- Clone implementation added
- Compilation errors documented (expected in dependent files)
- Files modified: `properties.rs` (line ranges)

---

## Wave 1 Verification Gate

**Orchestrator Must Verify**:
```bash
# Check properties.rs compiles in isolation
cargo check -p mathhook-core 2>&1 | grep "properties.rs"

# Expected: properties.rs itself should compile
# Expected: Errors in other files using old enum variants (NORMAL)
```

**If properties.rs itself has errors, DO NOT proceed to Wave 2. Fix issues first.**

**If only dependent files have errors, proceed to Wave 2 (this is expected).**

---

## Wave 2: Registry Population Update

### Agent G: Registry Population Specialist

**Responsibility**: Update all 16 function registrations to use new `Custom` variant with Expression builders

**Complexity**: HIGH (16 functions to update)
**Estimated Time**: 2-3 hours
**Risk Level**: HIGH (mathematical correctness critical)

---

### Step 1: Understand Current Registration Pattern

**Current Pattern** (Phase 5 - unit variants):
```rust
// Example: tan function
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::NonElementary {
        result_expr: Expression::integer(0),  // Placeholder!
    },
    result_template: "-ln(abs(cos(x))) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**Problem**: `result_expr` is just a placeholder. Actual construction in helper functions.

---

### Step 2: New Registration Pattern with Builders

**New Pattern** (Phase 6A - Expression builders):
```rust
// Example: tan function
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            // ∫tan(x)dx = -ln|cos(x)| + C
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function(
                    "ln",
                    vec![Expression::function(
                        "abs",
                        vec![Expression::function(
                            "cos",
                            vec![Expression::symbol(var)],
                        )],
                    )],
                ),
            ])
        }),
    },
    result_template: "-ln(abs(cos(x))) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

**Key Points**:
- `Arc::new(|var: Symbol| { ... })` - closure that builds expression
- `var` is the integration variable (can be any symbol)
- Construction logic moved FROM function_integrals.rs TO registry

---

### Step 3: Update All 16 Functions

**Files to Modify** (from Phase 4):
1. `crates/mathhook-core/src/functions/elementary/trigonometric.rs` (9 functions)
2. `crates/mathhook-core/src/functions/elementary/exponential.rs` (2 functions)
3. `crates/mathhook-core/src/functions/elementary/hyperbolic.rs` (3 functions)
4. `crates/mathhook-core/src/functions/elementary/logarithmic.rs` (2 functions)

**Function Categories**:

**Simple Functions (5) - NO CHANGE NEEDED**:
- sin, cos, exp, sinh, cosh
- Already use `Simple` variant - leave unchanged

**Custom Functions (11) - MUST UPDATE**:
- tan, cot, sec, csc (trigonometric)
- ln, log (logarithmic)
- arcsin, arccos, arctan (inverse trig)
- tanh (hyperbolic)
- sqrt (power)

---

### Step 4: Reference - All 11 Custom Function Builders

**Use these exact builders** (mathematical correctness validated in Phase 4-5):

#### 1. tan (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("ln", vec![
                    Expression::function("abs", vec![
                        Expression::function("cos", vec![Expression::symbol(var)])
                    ])
                ]),
            ])
        }),
    },
    result_template: "-ln(abs(cos(x))) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 2. cot (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::function("ln", vec![
                Expression::function("abs", vec![
                    Expression::function("sin", vec![Expression::symbol(var)])
                ])
            ])
        }),
    },
    result_template: "ln(abs(sin(x))) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 3. sec (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::function("ln", vec![
                Expression::function("abs", vec![
                    Expression::add(vec![
                        Expression::function("sec", vec![Expression::symbol(var.clone())]),
                        Expression::function("tan", vec![Expression::symbol(var)]),
                    ])
                ])
            ])
        }),
    },
    result_template: "ln(abs(sec(x) + tan(x))) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 4. csc (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("ln", vec![
                    Expression::function("abs", vec![
                        Expression::add(vec![
                            Expression::function("csc", vec![Expression::symbol(var.clone())]),
                            Expression::function("cot", vec![Expression::symbol(var)]),
                        ])
                    ])
                ]),
            ])
        }),
    },
    result_template: "-ln(abs(csc(x) + cot(x))) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 5. arcsin (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(var.clone()),
                    Expression::function("arcsin", vec![Expression::symbol(var.clone())]),
                ]),
                Expression::function("sqrt", vec![
                    Expression::add(vec![
                        Expression::integer(1),
                        Expression::mul(vec![
                            Expression::integer(-1),
                            Expression::pow(Expression::symbol(var), Expression::integer(2)),
                        ]),
                    ])
                ]),
            ])
        }),
    },
    result_template: "x*arcsin(x) + sqrt(1 - x^2) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 6. arccos (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(var.clone()),
                    Expression::function("arccos", vec![Expression::symbol(var.clone())]),
                ]),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::function("sqrt", vec![
                        Expression::add(vec![
                            Expression::integer(1),
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::pow(Expression::symbol(var), Expression::integer(2)),
                            ]),
                        ])
                    ]),
                ]),
            ])
        }),
    },
    result_template: "x*arccos(x) - sqrt(1 - x^2) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 7. arctan (trigonometric.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(var.clone()),
                    Expression::function("arctan", vec![Expression::symbol(var.clone())]),
                ]),
                Expression::mul(vec![
                    Expression::rational(-1, 2),
                    Expression::function("ln", vec![
                        Expression::add(vec![
                            Expression::integer(1),
                            Expression::pow(Expression::symbol(var), Expression::integer(2)),
                        ])
                    ]),
                ]),
            ])
        }),
    },
    result_template: "x*arctan(x) - (1/2)*ln(1 + x^2) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 8. ln (logarithmic.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::add(vec![
                Expression::mul(vec![
                    Expression::symbol(var.clone()),
                    Expression::function("ln", vec![Expression::symbol(var.clone())]),
                ]),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::symbol(var),
                ]),
            ])
        }),
    },
    result_template: "x*ln(x) - x + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 9. log (logarithmic.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::mul(vec![
                Expression::pow(
                    Expression::function("ln", vec![Expression::integer(10)]),
                    Expression::integer(-1),
                ),
                Expression::add(vec![
                    Expression::mul(vec![
                        Expression::symbol(var.clone()),
                        Expression::function("ln", vec![Expression::symbol(var.clone())]),
                    ]),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::symbol(var),
                    ]),
                ]),
            ])
        }),
    },
    result_template: "(1/ln(10)) * (x*ln(x) - x) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 10. tanh (hyperbolic.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::function("ln", vec![
                Expression::function("cosh", vec![Expression::symbol(var)])
            ])
        }),
    },
    result_template: "ln(cosh(x)) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

#### 11. sqrt (exponential.rs)
```rust
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Custom {
        builder: Arc::new(|var: Symbol| {
            Expression::mul(vec![
                Expression::rational(2, 3),
                Expression::pow(
                    Expression::symbol(var),
                    Expression::rational(3, 2),
                ),
            ])
        }),
    },
    result_template: "(2/3)*x^(3/2) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
}),
```

---

### Step 5: Add Required Imports to Registration Files

Each of the 4 registration files needs:
```rust
use std::sync::Arc;
```

Add this import to:
1. `trigonometric.rs`
2. `exponential.rs`
3. `hyperbolic.rs`
4. `logarithmic.rs`

---

### Verification (Wave 2)

```bash
# 1. Compilation check
cargo check -p mathhook-core

# Expected: Should compile now (all registrations updated)
# If errors remain, they should be in function_integrals.rs (Wave 3 will fix)

# 2. Integral registry tests
cargo test -p mathhook-core --test integral_registry_tests

# Expected: May fail initially because function_integrals.rs still has old code
# This is EXPECTED. Wave 3 will update function_integrals.rs.
```

**Agent G Deliverables**:
1. Updated 4 registration files with new `Custom` variant builders
2. All 11 custom functions re-registered with Expression builders
3. 5 simple functions verified unchanged
4. Compilation status (expect function_integrals.rs errors - document them)
5. Log: `agent_logs/AGENT_P1_6A_WAVE2_REGISTRY_UPDATE_LOG.md`

**Agent G Must Report**:
- Functions updated: 11/11 custom functions ✅
- Functions unchanged: 5/5 simple functions ✅
- Files modified: 4 (with line ranges)
- Compilation errors: Document (expected in function_integrals.rs)
- Mathematical formulas: Verify all match Phase 4-5 behavior

---

## Wave 2 Verification Gate

**Orchestrator Must Verify**:
```bash
# Check registration files compile
cargo check -p mathhook-core 2>&1 | grep -E "(trigonometric|exponential|hyperbolic|logarithmic)"

# Expected: Registration files should compile
# Expected: Errors only in function_integrals.rs (NORMAL - Wave 3 fixes this)
```

**If registration files have errors, DO NOT proceed to Wave 3. Fix issues first.**

**If only function_integrals.rs has errors, proceed to Wave 3 (this is expected).**

---

## Wave 3: Simplify function_integrals.rs

### Agent H: Simplification Specialist

**Responsibility**: Remove 237 lines of helper functions, simplify `apply_antiderivative_rule()`

**Complexity**: LOW (deletion and simplification)
**Estimated Time**: 30-60 minutes
**Risk Level**: LOW (mainly deletion, tests will validate)

---

### Step 1: Simplify apply_antiderivative_rule()

**File**: `crates/mathhook-core/src/calculus/integrals/function_integrals.rs`
**Location**: Find `apply_antiderivative_rule()` function (around lines 235-471 based on Phase 5)

**Current Implementation** (Phase 5 - 237 lines):
```rust
fn apply_antiderivative_rule(
    rule: &AntiderivativeRule,
    function_name: &str,
    variable: Symbol,
) -> Expression {
    match &rule.rule_type {
        AntiderivativeRuleType::Simple { antiderivative_fn, coefficient } => {
            Expression::mul(vec![
                coefficient.clone(),
                Expression::function(antiderivative_fn, vec![Expression::symbol(variable)])
            ])
        }

        AntiderivativeRuleType::NonElementary { result_expr } => {
            construct_non_elementary_result(/* 100+ lines */)
        }

        AntiderivativeRuleType::ByParts { result_expr } => {
            construct_by_parts_result(/* 100+ lines */)
        }

        // ... fallbacks
    }
}

// 237 lines of helper functions:
fn construct_non_elementary_result(...) { /* ~120 lines */ }
fn construct_by_parts_result(...) { /* ~117 lines */ }
```

**New Implementation** (Phase 6A - ~20 lines):
```rust
/// Apply antiderivative rule from registry to compute integral
///
/// Takes a rule from the function intelligence registry and constructs
/// the corresponding antiderivative expression.
///
/// # Arguments
///
/// * `rule` - The antiderivative rule from function intelligence registry
/// * `function_name` - Original function name (for error messages and fallback)
/// * `variable` - Integration variable
///
/// # Returns
///
/// The antiderivative expression. For unknown rule types, returns symbolic integral.
fn apply_antiderivative_rule(
    rule: &AntiderivativeRule,
    function_name: &str,
    variable: Symbol,
) -> Expression {
    match &rule.rule_type {
        AntiderivativeRuleType::Simple { antiderivative_fn, coefficient } => {
            // ∫f(x)dx = c * F(x)
            Expression::mul(vec![
                coefficient.clone(),
                Expression::function(antiderivative_fn, vec![Expression::symbol(variable)])
            ])
        }

        AntiderivativeRuleType::Custom { builder } => {
            // Builder constructs the expression directly
            builder(variable)
        }

        AntiderivativeRuleType::LinearSubstitution { .. } => {
            // Future implementation
            Expression::integral(
                Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                variable
            )
        }

        AntiderivativeRuleType::TrigSubstitution { .. } => {
            // Future implementation
            Expression::integral(
                Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                variable
            )
        }

        AntiderivativeRuleType::PartialFractions { .. } => {
            // Future implementation
            Expression::integral(
                Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                variable
            )
        }
    }
}
```

**Line Count**: ~237 lines → ~45 lines (**~192 line reduction**)

---

### Step 2: Delete Helper Functions

**Delete These Functions** (237 lines total):
1. `construct_non_elementary_result()` (~120 lines)
2. `construct_by_parts_result()` (~117 lines)

**These are NO LONGER NEEDED** - builders in registry do this work now.

---

### Step 3: Update Imports (if needed)

**File**: `function_integrals.rs`

**Current Imports**:
```rust
use crate::core::{Expression, Symbol};
use crate::functions::intelligence::get_universal_registry;
use crate::functions::properties::{AntiderivativeRule, AntiderivativeRuleType};
```

**No changes needed** - imports are already correct.

---

### Verification (Wave 3 - CRITICAL)

```bash
# 1. Compilation check
cargo check -p mathhook-core

# Expected: 0 errors (all waves complete)

# 2. Integral registry tests (CRITICAL)
cargo test -p mathhook-core --test integral_registry_tests

# Expected: 26 passed; 0 failed; 10 ignored (ZERO REGRESSIONS)
# If ANY test fails, STOP and debug

# 3. Full test suite
cargo test -p mathhook-core

# Expected: ≥823 passing (same as Phase 5 baseline)

# 4. Doctests
cargo test --doc -p mathhook-core

# Expected: All pass

# 5. Line count verification
wc -l crates/mathhook-core/src/calculus/integrals/function_integrals.rs

# Expected: ~200-250 lines (from 473)
```

**Agent H Deliverables**:
1. Simplified `apply_antiderivative_rule()` function (~20 lines)
2. Deleted helper functions (237 lines removed)
3. All verification outputs (tests, line count)
4. Log: `agent_logs/AGENT_P1_6A_WAVE3_SIMPLIFICATION_LOG.md`

**Agent H Must Report**:
- Helper functions deleted: 237 lines ✅
- New `apply_antiderivative_rule()`: ~45 lines
- File size: Before 473 → After ~XXX lines
- Tests: 26 passed; 0 failed; 10 ignored ✅
- Full suite: XXX passing ✅
- Zero regressions: CONFIRMED ✅

---

## Wave 3 Verification Gate (FINAL)

**Orchestrator Must Run Complete Verification**:
```bash
# Complete verification script
./verify_phase6a_final.sh

# Or manual verification:
cargo check -p mathhook-core
cargo test -p mathhook-core --test integral_registry_tests
cargo test -p mathhook-core
cargo test --doc -p mathhook-core
wc -l crates/mathhook-core/src/calculus/integrals/function_integrals.rs
```

**ALL must pass before Phase 6A is complete.**

---

## Critical Instructions for All Agents

### MUST DO:
1. **Read CLAUDE.md First**: All rules apply (CLAUDE.md is superior)
2. **Follow Wave Order**: Wave 1 → Verify → Wave 2 → Verify → Wave 3 → Verify
3. **Run Verification After EVERY Wave**: Use verification commands provided
4. **Report Exact Test Counts**: Never estimate, always run actual tests
5. **Document All Changes**: Line-by-line report with before/after
6. **Maintain Mathematical Correctness**: All 11 custom function builders must be exact

### MUST NOT DO:
1. **Do NOT skip verification gates** - Must verify each wave before next
2. **Do NOT estimate test results** - Always run actual `cargo test`
3. **Do NOT add emojis** - CLAUDE.md violation
4. **Do NOT add ALL CAPS comments** - CLAUDE.md violation
5. **Do NOT proceed if tests fail** - Fix issues first
6. **Do NOT modify test files** - Only modify type system, registrations, function_integrals.rs

### CLAUDE.md Compliance (SUPERIOR):
- ❌ NO emojis in code
- ❌ NO inline comments (except mathematical formulas)
- ❌ NO ALL CAPS (except constants)
- ✅ USE `///` for documentation
- ✅ RUN actual tests (never estimate)
- ✅ REPORT exact test counts
- ✅ Architectural patterns over hardcoding (already achieved, maintain it)

---

## Success Criteria Checklist

**Phase 6A Complete When ALL Pass**:
- [ ] AntiderivativeRuleType updated with `Custom { builder: Arc<dyn Fn> }`
- [ ] All 16 functions re-registered (11 custom with builders, 5 simple unchanged)
- [ ] Helper functions removed from function_integrals.rs (~237 lines)
- [ ] Tests: 26 passed; 0 failed; 10 ignored (ZERO REGRESSIONS)
- [ ] Full suite: ≥823 tests passing
- [ ] function_integrals.rs: 473 → ~200-250 lines (~220-270 line reduction)
- [ ] Zero emojis in code
- [ ] Zero compilation errors
- [ ] Zero test regressions
- [ ] CLAUDE.md 100% compliant

---

## Orchestrator Responsibilities

1. **Launch Wave 1**: Agent F (Type System)
2. **Verify Wave 1**: Check properties.rs compiles - MUST PASS before Wave 2
3. **Launch Wave 2**: Agent G (Registry Updates) - ONLY after Wave 1 verified
4. **Verify Wave 2**: Check all registrations compile - MUST PASS before Wave 3
5. **Launch Wave 3**: Agent H (Simplification) - ONLY after Wave 2 verified
6. **Verify Wave 3**: Run complete verification - ALL tests must pass
7. **Create Completion Report**: `PHASE_6A_COMPLETION_REPORT.md`
8. **Update Session Log**: `INTEGRAL_REGISTRY_SESSION_LOG.md`

---

**Document End**

**Next Step**: Orchestrator launches Wave 1 (Agent F) with these instructions.

**CLAUDE.md is SUPERIOR** - all agents must follow it strictly.
