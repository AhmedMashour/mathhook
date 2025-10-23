# SHARED_DATA_MIGRATION_ORCHESTRATOR.md

**Orchestrator Command for Migrating MathHook Functions to Shared Data Architecture**

**Version**: 1.0
**Created**: 2025-10-23
**Target**: Migrate all 32 MathHook functions from embedded special values to shared data architecture

---

## BOOTSTRAP BLOCK (READ FIRST - NON-NEGOTIABLE)

### Quick Access Commands (Copy-Paste These)

**Read these documents in exact order BEFORE starting any work:**

```bash
# 1. Architecture Document (1725 lines) - COMPLETE pattern specification
cat .mathhook_sessions/SHARED_DATA_ARCHITECTURE_COMPLETE.md

# 2. CLAUDE.md - Authoritative source (overrides all other docs)
cat CLAUDE.md | grep -A 100 "Function Evaluation Architecture"

# 3. Current implementations to migrate (7 special functions)
cat crates/mathhook-core/src/functions/special/gamma.rs
cat crates/mathhook-core/src/functions/special/zeta.rs
cat crates/mathhook-core/src/functions/special/bessel.rs

# 4. Example orchestrator template
cat .mathhook_sessions/INTEGRATION_ENHANCEMENT_ORCHESTRATOR_COMMAND.md | head -200
```

**Verification Command - Did you read?**
```bash
# Confirm you understand the architecture by answering:
# Q1: Where do function implementations go? (Answer: src/core/functions/FUNCNAME/mod.rs)
# Q2: Where do special values go? (Answer: src/core/functions/FUNCNAME/data.rs)
# Q3: How many functions need REFACTORING? (Answer: 7 - gamma, beta, digamma, polygamma, zeta, bessel_j, bessel_y)
# Q4: How many functions need IMPLEMENTATION? (Answer: 21 - elementary + polynomial)
```

### Critical Context Documents

**MUST READ in this exact order before proceeding:**

1. **SHARED_DATA_ARCHITECTURE_COMPLETE.md** (1725 lines)
   - Lines 1-100: Architecture overview and motivation
   - Lines 100-500: Elementary functions pattern (sin, cos, exp, log, sqrt, abs)
   - Lines 500-900: Special functions pattern (gamma, zeta, bessel)
   - Lines 900-1200: Polynomial functions pattern (hermite, laguerre, chebyshev, legendre)
   - Lines 1200-1500: Number theory functions (NO HashMap - algorithmic)
   - Lines 1500-1725: Registry derivation pattern and single source of truth

2. **CLAUDE.md** (worktree)
   - **Function Evaluation Architecture** section (lines 44-106)
   - Current implementation: Metadata-only registry + direct function calls
   - Registry stores FunctionProperties, does NOT dispatch evaluation
   - Direct functions (gamma(), sin()) contain full implementation

3. **INTEGRATION_ENHANCEMENT_ORCHESTRATOR_COMMAND.md** (917 lines)
   - Template for orchestrator document structure
   - Wave-based implementation pattern
   - Success criteria and verification protocols.

### Single Source of Truth Hierarchy

1. **CLAUDE.md** (worktree) - Overrides all other documentation
2. **This Document** - Orchestrator plan derived from CLAUDE.md + architecture doc
3. **SHARED_DATA_ARCHITECTURE_COMPLETE.md** - Architecture specification
4. **Implementation Code** - Must match plan and CLAUDE.md


**If conflicts arise**: CLAUDE.md > This Document > Architecture Doc

## MAIN ORCHESTRATION RULE
 **Strict waves validation rules** - Must create verification scripts for each wave, and run them at the end, so to decide to accept the work of the agent or not

---

## EXECUTIVE SUMMARY

### CRITICAL CLARIFICATION: IMPLEMENTATION STATUS (ACCURATE)

**THIS IS BOTH A REFACTORING AND IMPLEMENTATION TASK**

MathHook functions fall into **TWO DISTINCT CATEGORIES**:

#### ✅ CATEGORY A: IMPLEMENTED (Need Refactoring Only)

**Special Functions with `pub fn` implementations** (5 functions):
- ✅ **gamma, beta, digamma, polygamma** - Fully implemented in `functions/special/gamma.rs`
  - Current: Hardcoded factorial computation, half-integer special cases
  - Migration: Move hardcoded values → shared `GAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap>`

- ✅ **zeta** - Fully implemented in `functions/special/zeta.rs`
  - Current: Hardcoded match statements for ζ(0), ζ(2), ζ(4), etc.
  - Migration: Move hardcoded values → shared `ZETA_SPECIAL_VALUES: LazyLock<SpecialValuesMap>`

- ✅ **bessel_j, bessel_y** - Fully implemented in `functions/special/bessel.rs`
  - Current: Direct numerical evaluation with few special cases
  - Migration: Extract special cases → shared `BESSEL_J_SPECIAL_VALUES`, `BESSEL_Y_SPECIAL_VALUES`

**Task**: REFACTOR existing implementations to use shared data architecture

#### ❌ CATEGORY B: NOT IMPLEMENTED (Need Implementation + Shared Data)

**Elementary Functions** (17 functions) - **ONLY have intelligence/properties structs, NO `pub fn` implementations**:
- ❌ **sin, cos, tan, cot, sec, csc** - Only `CircularTrigIntelligence` struct exists
- ❌ **asin, acos, atan, acot, asec, acsc** - Only `InverseTrigIntelligence` struct exists
- ❌ **sinh, cosh, tanh** - Only `HyperbolicIntelligence` struct exists
- ❌ **exp, ln** - Only `ExponentialIntelligence` struct exists (helpers exist, not main functions)
- ❌ **sqrt, abs** - Only helpers exist (simplify_sqrt, simplify_abs), not callable `pub fn sqrt()`

**Polynomial Functions** (4 functions) - **ONLY have intelligence/properties structs, NO `pub fn` implementations**:
- ❌ **hermite, laguerre, chebyshev, legendre** - Only intelligence structs exist
  - Helper functions exist: `expand_hermite_symbolic()`, `evaluate_hermite_numerical()`
  - But NO callable `pub fn hermite()` for general use

**Task**: IMPLEMENT from scratch WITH shared data architecture from the start

#### Summary

- **5 functions** need **REFACTORING** (Category A): Move existing hardcoded values → shared data
- **21 functions** need **IMPLEMENTATION** (Category B): Implement `pub fn` WITH shared data from start
- **4 functions** are algorithmic (gcd, lcm, mod, is_prime) - NO migration needed

### Goal Statement

**Refactor** all 32 MathHook functions from current architecture (embedded special values in properties structs or hardcoded in function logic) to the planned shared data architecture (separate data files with `LazyLock<SpecialValuesMap>`), establishing a single source of truth that serves both implementation and registry and have the best performance and mathematically correct.

### Current vs Target Architecture

**Current Architecture:**
```rust
// Special values EMBEDDED in properties struct
FunctionProperties::Elementary(Box::new(ElementaryProperties {
    special_values: vec![
        SpecialValue {
            input: "0".to_string(),
            output: Expression::integer(0),
            latex_explanation: "\\sin(0) = 0".to_string(),
        },
    ],
    // ...
}))

// OR hardcoded in function implementation
pub fn gamma(z: &Expression) -> Expression {
    match z {
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            // Direct factorial computation (hardcoded)
            let mut result = 1i64;
            for i in 1..val { result *= i; }
            Expression::Number(Number::Integer(result))
        }
        // ...
    }
}
```

**Target Architecture:**
```rust
// Separate data file: functions/data/trig/sin_special_values.rs
pub static SIN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::new();
    map.insert(expr!(0), special_value!(0, "\\sin(0) = 0"));
    map.insert(expr!(pi / 6), special_value!(1/2, "\\sin(\\frac{\\pi}{6}) = \\frac{1}{2}"));
    // ...
    map
});

// Implementation uses shared data
pub fn sin(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values (exact or error)
    if let Some(result) = SIN_SPECIAL_VALUES.get(arg) {
        match result {
            SpecialValueResult::Exact { output, .. } => return Ok(output.clone()),
            SpecialValueResult::Error { error, .. } => return Err(error.clone()),
        }
    }
    // 2. Computed special values
    // 3. Identities
    // 4. Numerical evaluation
    // 5. Unevaluated (symbolic)
}

// Registry DERIVES from shared data (prevents drift)
FunctionProperties {
    name: "sin",
    special_values: SIN_SPECIAL_VALUES.iter()
        .map(|(input, result)| SpecialValue { ... })
        .collect(),
    dispatch: sin_dispatch,
}
```

**Key Architectural Difference:**
- **Current**: Data duplicated (properties + implementation) OR hardcoded
- **Target**: Data in ONE place (shared HashMap), implementation + registry both use it

### File Organization: Per-Function Modules

**ALL function implementations organized under `src/core/functions/` with one folder per function:**

```
src/core/functions/
  sin/
    data.rs           # SIN_SPECIAL_VALUES: LazyLock<SpecialValuesMap>
    mod.rs            # pub fn sin() implementation + tests
  cos/
    data.rs           # COS_SPECIAL_VALUES: LazyLock<SpecialValuesMap>
    mod.rs            # pub fn cos() implementation + tests
  tan/
    data.rs           # TAN_SPECIAL_VALUES: LazyLock<SpecialValuesMap>
    mod.rs            # pub fn tan() implementation + tests
  gamma/
    data.rs           # GAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap>
    mod.rs            # pub fn gamma() implementation + tests
  zeta/
    data.rs           # ZETA_SPECIAL_VALUES: LazyLock<SpecialValuesMap>
    mod.rs            # pub fn zeta() implementation + tests
  hermite/
    data.rs           # HERMITE_SPECIAL_VALUES: LazyLock<SpecialValuesMap>
    mod.rs            # pub fn hermite() implementation + tests
  ... (32 function folders total)
  mod.rs              # Re-exports: pub use sin::sin; pub use cos::cos; etc.
```

**Benefits:**
- **One folder per function**: Clear, discoverable module boundaries
- **Co-located data + logic**: HashMap lives with implementation
- **Independent testing**: Each `mod.rs` contains function-specific tests
- **Clean imports**: `use crate::core::functions::sin;`
- **Easy navigation**: Everything about `gamma` is in `src/core/functions/gamma/`

**CRITICAL: Migration of Implemented Functions**

For the **7 IMPLEMENTED functions** (gamma, beta, digamma, polygamma, zeta, bessel_j, bessel_y):
- **MUST** carefully copy existing implementations from `src/functions/special/*.rs`
- **PRESERVE** all mathematical logic exactly (zero tolerance for regressions)
- **EXTRACT** hardcoded special values → separate `data.rs` files
- **MAINTAIN** all tests (copy from old location, verify all pass)
- **VERIFY** against SymPy after migration

**Migration Steps (Category A - REFACTORING)**:
1. Create new folder: `src/core/functions/gamma/`
2. Copy `pub fn gamma()` from `src/functions/special/gamma.rs` → `src/core/functions/gamma/mod.rs`
3. Extract hardcoded values → `src/core/functions/gamma/data.rs` with `GAMMA_SPECIAL_VALUES`
4. Update implementation to use `GAMMA_SPECIAL_VALUES.get()`
5. Copy all tests from old file → new `mod.rs`
6. Verify: `cargo test -p mathhook-core gamma` (all tests must pass)
7. Delete old file ONLY after verification

**CRITICAL: Use Shell Commands for File Operations**

**DO NOT** manually copy-paste code or use editor operations. **USE SHELL COMMANDS ONLY**:

```bash
# ✅ CORRECT: Use shell commands
mkdir -p crates/mathhook-core/src/core/functions/gamma
cp crates/mathhook-core/src/functions/special/gamma.rs \
   crates/mathhook-core/src/core/functions/gamma/mod.rs

# ✅ CORRECT: Verify file copied successfully
cat crates/mathhook-core/src/core/functions/gamma/mod.rs | head -20

# ✅ CORRECT: After verification, move old file to backup
mv crates/mathhook-core/src/functions/special/gamma.rs \
   crates/mathhook-core/src/functions/special/gamma.rs.bak

# ❌ WRONG: Manual copy-paste (introduces errors)
# ❌ WRONG: Editor-based operations (can lose formatting, tests, comments)
```

**Shell Command Checklist for Migration**:
- ✅ `mkdir -p` to create directory structure
- ✅ `cp` to copy existing implementations (preserves exact content)
- ✅ `cat` to verify file contents before/after
- ✅ `mv` to rename/backup old files
- ✅ `diff` to compare old vs new after refactoring
- ✅ `git diff` to review changes before committing
- ❌ NEVER use manual editor operations for file copying

**Why Shell Commands**:
1. **Exact Preservation**: `cp` copies byte-for-byte (no transcription errors)
2. **Verifiable**: Can `diff` source and destination
3. **Reversible**: Easy to restore with backups (`*.bak` files)
4. **Auditable**: Shell commands provide clear trail of what was done
5. **No Human Error**: Eliminates copy-paste mistakes, missing code blocks

**For NOT IMPLEMENTED functions** (Category B):
- Create from scratch following architecture templates
- No existing code to migrate
- Still use shell commands for file creation: `touch`, `mkdir -p`

### Success Metrics

1. **Zero Duplication**: Special values exist in exactly ONE location per function
2. **Single Source of Truth**: `LazyLock<SpecialValuesMap>` serves both implementation AND registry
3. **No Drift**: Registry special values derived automatically from shared data
4. **Separation of Concerns**: Data files separate from implementation logic
5. **Performance**: O(1) lookup maintained, LazyLock adds <100ns overhead
6. **Mathematical Correctness**: All existing tests pass, no regressions
7. **Complete Coverage**: All 32 functions migrated (excluding number theory - algorithmic)

---

## COMPREHENSIVE FUNCTION INVENTORY

### All 32 MathHook Functions

**Total Functions**: 32
**Functions Requiring Migration**: 28 (excludes 4 number theory - algorithmic)

#### Elementary Functions (17 functions - ALL NOT IMPLEMENTED)

**Trigonometric (6)** - ❌ NOT IMPLEMENTED:
- `sin` - Current: ONLY `CircularTrigIntelligence` struct, NO `pub fn sin()`
- `cos` - Current: ONLY `CircularTrigIntelligence` struct, NO `pub fn cos()`
- `tan` - Current: ONLY `CircularTrigIntelligence` struct, NO `pub fn tan()`
- `cot` - Current: ONLY `CircularTrigIntelligence` struct, NO `pub fn cot()`
- `sec` - Current: ONLY `CircularTrigIntelligence` struct, NO `pub fn sec()`
- `csc` - Current: ONLY `CircularTrigIntelligence` struct, NO `pub fn csc()`

**Inverse Trigonometric (6)** - ❌ NOT IMPLEMENTED:
- `asin` - Current: ONLY `InverseTrigIntelligence` struct, NO `pub fn asin()`
- `acos` - Current: ONLY `InverseTrigIntelligence` struct, NO `pub fn acos()`
- `atan` - Current: ONLY `InverseTrigIntelligence` struct, NO `pub fn atan()`
- `acot` - Current: ONLY `InverseTrigIntelligence` struct, NO `pub fn acot()`
- `asec` - Current: ONLY `InverseTrigIntelligence` struct, NO `pub fn asec()`
- `acsc` - Current: ONLY `InverseTrigIntelligence` struct, NO `pub fn acsc()`

**Hyperbolic (3)** - ❌ NOT IMPLEMENTED:
- `sinh` - Current: ONLY `HyperbolicIntelligence` struct, NO `pub fn sinh()`
- `cosh` - Current: ONLY `HyperbolicIntelligence` struct, NO `pub fn cosh()`
- `tanh` - Current: ONLY `HyperbolicIntelligence` struct, NO `pub fn tanh()`

**Exponential/Logarithmic (2)** - ❌ NOT IMPLEMENTED:
- `exp` - Current: ONLY `ExponentialIntelligence` struct, NO `pub fn exp()`
- `ln` - Current: ONLY `ExponentialIntelligence` struct, NO `pub fn ln()`

**Roots/Absolute Value (2)** - ❌ NOT IMPLEMENTED:
- `sqrt` - Current: Helper `simplify_sqrt` exists, NO callable `pub fn sqrt()`
- `abs` - Current: Helper `simplify_abs` exists, NO callable `pub fn abs()`

#### Special Functions (5 functions - ALL IMPLEMENTED)

**Gamma Family (4)** - ✅ IMPLEMENTED:
- `gamma` - Current: IMPLEMENTED in gamma.rs with hardcoded factorial/half-integers
- `beta` - Current: IMPLEMENTED in gamma.rs (uses gamma function)
- `digamma` - Current: IMPLEMENTED in gamma.rs (returns symbolic)
- `polygamma` - Current: IMPLEMENTED in gamma.rs (returns symbolic)

**Bessel Functions (2)** - ✅ IMPLEMENTED:
- `bessel_j` - Current: IMPLEMENTED in bessel.rs with numerical evaluation
- `bessel_y` - Current: IMPLEMENTED in bessel.rs with numerical evaluation

**Zeta Function (1)** - ✅ IMPLEMENTED:
- `zeta` - Current: IMPLEMENTED in zeta.rs with hardcoded special values

#### Polynomial Functions (4 functions - ALL NOT IMPLEMENTED)

**Orthogonal Polynomials (4)** - ❌ NOT IMPLEMENTED:
- `hermite` - Current: ONLY `HermiteIntelligence` struct, NO `pub fn hermite()`
  - Helpers exist: `expand_hermite_symbolic()`, `evaluate_hermite_numerical()`
- `laguerre` - Current: ONLY `LaguerreIntelligence` struct, NO `pub fn laguerre()`
  - Helpers exist: `expand_laguerre_symbolic()`, `evaluate_laguerre_numerical()`
- `chebyshev` - Current: ONLY `ChebyshevIntelligence` struct, NO `pub fn chebyshev()`
  - Helpers exist: `expand_chebyshev_symbolic()`, `evaluate_chebyshev_numerical()`
- `legendre` - Current: ONLY `LegendreIntelligence` struct, NO `pub fn legendre()`
  - Helpers exist: `expand_legendre_symbolic()`, `evaluate_legendre_numerical()`

#### Number Theory Functions (4 functions - NO MIGRATION)

**Algorithmic Functions (4 - NO HashMap needed)**:
- `gcd` - Algorithmic (Euclidean algorithm)
- `lcm` - Algorithmic (uses gcd)
- `mod` - Algorithmic (modulo operation)
- `is_prime` - Algorithmic (primality testing)

**Note**: Number theory functions use algorithms, NOT special value lookups. No migration needed.

---

## DETAILED CURRENT STATE ANALYSIS

### File Structure Discovery

**Current Function Organization:**
```
functions/
├── elementary/
│   ├── abs.rs                    - Absolute value (properties + special values)
│   ├── exponential.rs            - exp, sqrt (properties + special values)
│   ├── hyperbolic.rs             - sinh, cosh, tanh (properties + special values)
│   ├── logarithmic.rs            - ln, log (properties + special values)
│   ├── sqrt.rs                   - Additional sqrt implementation
│   └── trigonometric/
│       ├── trig_circular.rs      - sin, cos, tan, etc. (properties + special values)
│       └── trig_inverse.rs       - asin, acos, atan, etc. (properties + special values)
├── special/
│   ├── gamma.rs                  - gamma, beta (hardcoded match statements)
│   ├── zeta.rs                   - zeta (hardcoded match statements)
│   └── bessel.rs                 - bessel_j, bessel_y (numerical evaluation)
├── polynomials/
│   ├── hermite.rs                - Hermite polynomials (properties + recurrence)
│   ├── laguerre.rs               - Laguerre polynomials (properties + recurrence)
│   ├── chebyshev.rs              - Chebyshev polynomials (properties + recurrence)
│   └── legendre.rs               - Legendre polynomials (properties + recurrence)
└── number_theory.rs              - gcd, lcm, mod, is_prime (algorithmic)
```

### Current Architecture Patterns

**Pattern 1: Properties Struct with Special Values (Most Elementary Functions)**
```rust
// Example: trig_circular.rs (sin, cos, tan)
CircularTrigIntelligence {
    properties: HashMap::with_capacity(6),
}

self.properties.insert(
    "sin".to_string(),
    FunctionProperties::Elementary(Box::new(ElementaryProperties {
        special_values: vec![
            SpecialValue {
                input: "0".to_string(),
                output: Expression::integer(0),
                latex_explanation: "\\sin(0) = 0".to_string(),
            },
            // More special values embedded here...
        ],
        derivative_rule: Some(...),
        domain_range: Box::new(...),
    }))
);
```

**Files Using Pattern 1:**
- `elementary/trigonometric/trig_circular.rs` - sin, cos, tan, cot, sec, csc
- `elementary/trigonometric/trig_inverse.rs` - asin, acos, atan, acot, asec, acsc
- `elementary/hyperbolic.rs` - sinh, cosh, tanh
- `elementary/exponential.rs` - exp, sqrt
- `elementary/logarithmic.rs` - ln, log
- `elementary/abs.rs` - abs
- `polynomials/hermite.rs` - hermite
- `polynomials/laguerre.rs` - laguerre
- `polynomials/chebyshev.rs` - chebyshev
- `polynomials/legendre.rs` - legendre

**Pattern 2: Hardcoded Match Statements (Special Functions)**
```rust
// Example: gamma.rs
pub fn gamma(z: &Expression) -> Expression {
    match z {
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            // Hardcoded factorial computation
            let mut result = 1i64;
            for i in 1..val { result *= i; }
            Expression::Number(Number::Integer(result))
        }
        Expression::Number(Number::Float(x)) => {
            // Hardcoded half-integer check
            if (twice - twice.round()).abs() < 1e-10 {
                gamma_half_integer(*x)
            } else {
                let result = lanczos_gamma(*x);
                Expression::Number(Number::Float(result))
            }
        }
        _ => Expression::function("gamma", vec![z.clone()]),
    }
}
```

**Files Using Pattern 2:**
- `special/gamma.rs` - gamma, beta
- `special/zeta.rs` - zeta
- `special/bessel.rs` - bessel_j, bessel_y (mostly numerical, some special cases)

### Gap Analysis: Current vs Target

**For Each Function, Identify:**

1. **Where Special Values Currently Live**:
   - Pattern 1: In `ElementaryProperties::special_values` vector
   - Pattern 2: Hardcoded in `match` statements or conditional logic

2. **What Needs to Be Created**:
   - Shared data file: `functions/data/<category>/<function>_special_values.rs`
   - `LazyLock<SpecialValuesMap>` static

3. **What Needs to Be Modified**:
   - Implementation function to use shared HashMap first
   - Registry derivation to use shared HashMap (not duplicate data)

4. **What Needs to Be Verified**:
   - All existing tests pass
   - No regressions in mathematical correctness
   - Performance maintained (O(1) lookup)

---

## MIGRATION WAVE STRUCTURE

### Overview

**Total Waves**: 6
**Estimated Timeline**: 6-8 weeks (1-1.5 weeks per wave)

### Wave Breakdown

**Wave 1**: Foundation + Trigonometric (6 functions)
**Wave 2**: Inverse Trigonometric (6 functions)
**Wave 3**: Hyperbolic + Exponential/Log + Roots/Abs (7 functions)
**Wave 4**: Special Functions - Gamma Family (2 functions)
**Wave 5**: Special Functions - Bessel + Zeta (5 functions)
**Wave 6**: Polynomial Functions (4 functions)

---

## WAVE 1: FOUNDATION + TRIGONOMETRIC FUNCTIONS (IMPLEMENTATION)

### Goal

Establish shared data architecture foundation and **IMPLEMENT** 6 circular trigonometric functions from scratch.

**Task Type**: IMPLEMENTATION (Category B)
**Current State**: Only `CircularTrigIntelligence` struct exists, NO `pub fn` implementations
**Target State**: Fully implemented functions with shared data architecture

### Bootstrap

1. **Read SHARED_DATA_ARCHITECTURE_COMPLETE.md**:
   - Lines 1-100: Architecture overview
   - Lines 100-500: Elementary functions pattern
   - Lines 200-300: SpecialValuesMap structure

2. **Read CLAUDE.md**:
   - Function Evaluation Architecture section
   - Expression Type (32-byte constraint)
   - Number Type (16-byte constraint)

3. **Understand Current State**:
   - Read: `functions/elementary/trigonometric/trig_circular.rs`
   - Identify: Special values currently in `CircularTrigIntelligence`

### Deliverables

#### 1.1: Foundation Infrastructure

**Create**:
- `functions/data/mod.rs` - Data module root
- `functions/data/trig/mod.rs` - Trigonometric data module
- `functions/data/types.rs` - Shared data types:
  ```rust
  pub struct SpecialValueResult {
      pub kind: SpecialValueKind,
      pub output: Expression,
      pub latex_explanation: String,
      pub educational_notes: Option<String>,
  }

  pub enum SpecialValueKind {
      Exact,           // Exact symbolic value
      DomainError,     // Pole, branch cut, undefined
      BranchPoint,     // Multi-valued function branch
  }

  pub type SpecialValuesMap = HashMap<Expression, SpecialValueResult>;
  ```

**Verification**:
- `cargo build` succeeds
- Module structure compiles
- Types are properly exported

#### 1.2: Sin Function Implementation (NEW - Not Migration)

**Task**: Implement `pub fn sin()` from scratch with shared data architecture

**Create**: `functions/data/trig/sin_special_values.rs`
```rust
use once_cell::sync::LazyLock;
use crate::core::Expression;
use super::types::{SpecialValuesMap, SpecialValueResult, SpecialValueKind};

/// Special values for sin(x) function
///
/// Single source of truth for sin special values used by:
/// - Implementation (functions/elementary/trigonometric.rs)
/// - Registry (functions/properties/elementary.rs)
pub static SIN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(13); // Exact count

    // Exact values
    map.insert(
        Expression::integer(0),
        SpecialValueResult {
            kind: SpecialValueKind::Exact,
            output: Expression::integer(0),
            latex_explanation: "\\sin(0) = 0".to_string(),
            educational_notes: Some("Sine of zero radians".to_string()),
        }
    );

    map.insert(
        Expression::div(Expression::pi(), Expression::integer(6)),
        SpecialValueResult {
            kind: SpecialValueKind::Exact,
            output: Expression::rational(1, 2),
            latex_explanation: "\\sin(\\frac{\\pi}{6}) = \\frac{1}{2}".to_string(),
            educational_notes: Some("30-degree angle (π/6 radians)".to_string()),
        }
    );

    // ... more exact values (π/4, π/3, π/2, π, 3π/2, 2π, etc.)

    map
});
```

**Create**: `functions/elementary/trigonometric.rs` (or add to existing file)
```rust
use crate::functions::data::trig::SIN_SPECIAL_VALUES;
use crate::core::{Expression, Number, MathError};

/// Sine function sin(x)
///
/// **NEWLY IMPLEMENTED** - This is the first implementation with shared data architecture.
///
/// Uses shared special values data from `sin_special_values.rs`
///
/// # Arguments
///
/// * `arg` - Expression to evaluate sine at
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::elementary::sin;
/// use mathhook_core::{Expression, Number};
///
/// let result = sin(&Expression::integer(0)).unwrap();
/// assert_eq!(result, Expression::integer(0)); // sin(0) = 0
/// ```
pub fn sin(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values
    if let Some(result) = SIN_SPECIAL_VALUES.get(arg) {
        match result.kind {
            SpecialValueKind::Exact => return Ok(result.output.clone()),
            _ => {}
        }
    }

    // 2. Computed special values (symmetry, periodicity)
    // sin(-x) = -sin(x)
    // sin(x + 2π) = sin(x)
    // (TODO: Implement these transformations)

    // 3. Numerical evaluation
    match arg {
        Expression::Number(Number::Float(x)) => {
            Ok(Expression::Number(Number::Float(x.sin())))
        }
        Expression::Number(Number::Integer(n)) => {
            let x = *n as f64;
            Ok(Expression::Number(Number::Float(x.sin())))
        }
        _ => {
            // 4. Symbolic (unevaluated)
            Ok(Expression::function("sin", vec![arg.clone()]))
        }
    }
}
```

**Modify**: `functions/properties/elementary.rs` (registry derivation)
```rust
use crate::functions::data::trig::SIN_SPECIAL_VALUES;

FunctionProperties {
    name: "sin",
    special_values: SIN_SPECIAL_VALUES.iter()
        .filter_map(|(input, result)| {
            if matches!(result.kind, SpecialValueKind::Exact) {
                Some(SpecialValue {
                    input: input.to_string(),
                    output: result.output.clone(),
                    latex_explanation: result.latex_explanation.clone(),
                })
            } else {
                None
            }
        })
        .collect(),
    derivative_rule: Some(...),
    domain_range: Box::new(...),
}
```

**Verification**:
- All sin tests pass: `cargo test -p mathhook-core sin`
- No regressions: `cargo test -p mathhook-core`
- Performance: Benchmark sin evaluation (<100ns overhead)

#### 1.3: Cos, Tan, Cot, Sec, Csc Migration

Repeat 1.2 pattern for:
- `cos_special_values.rs`
- `tan_special_values.rs`
- `cot_special_values.rs`
- `sec_special_values.rs`
- `csc_special_values.rs`

### Success Criteria

- [ ] Foundation types created and exported
- [ ] 6 trigonometric functions migrated
- [ ] All special values in shared HashMaps
- [ ] Registry derives from shared data (zero duplication)
- [ ] All existing tests pass
- [ ] No performance regressions
- [ ] Mathematical correctness verified against SymPy

### Testing Protocol

```bash
# Unit tests
cargo test -p mathhook-core elementary::trigonometric

# Integration tests
cargo test -p mathhook-core --test integration_tests

# Regression tests
cargo test -p mathhook-core --no-fail-fast

# Performance benchmarks
cargo bench --bench trig_functions
```

---

## WAVE 2: INVERSE TRIGONOMETRIC FUNCTIONS

### Goal

Migrate 6 inverse trigonometric functions to shared data architecture.

### Functions

- `asin`, `acos`, `atan`, `acot`, `asec`, `acsc`

### Pattern

**Same as Wave 1**, but inverse trig functions have:
- **Domain restrictions** (branch cuts)
- **Range restrictions** (principal values)
- **Domain errors** need to be in SpecialValuesMap

**Example**: `asin_special_values.rs`
```rust
pub static ASIN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(15);

    // Exact values
    map.insert(Expression::integer(0), exact_value!(0, "\\arcsin(0) = 0"));
    map.insert(Expression::rational(1, 2), exact_value!(pi/6, "\\arcsin(\\frac{1}{2}) = \\frac{\\pi}{6}"));

    // Domain errors
    map.insert(
        Expression::integer(2),
        SpecialValueResult {
            kind: SpecialValueKind::DomainError,
            output: Expression::integer(0), // Placeholder
            latex_explanation: "\\arcsin(2) \\text{ undefined (domain is } [-1,1] \\text{)}".to_string(),
            educational_notes: Some("Arcsine domain restriction".to_string()),
        }
    );

    map
});
```

### Deliverables

- 6 data files: `asin_special_values.rs`, `acos_special_values.rs`, etc.
- Implementation modifications: Use shared data for both exact values AND domain errors
- Registry derivations: Filter out domain errors, only show exact values

### Success Criteria

- [ ] 6 inverse trig functions migrated
- [ ] Domain errors handled via SpecialValuesMap
- [ ] All tests pass
- [ ] No regressions

---

## WAVE 3: HYPERBOLIC + EXPONENTIAL/LOG + ROOTS/ABS

### Goal

Migrate remaining elementary functions (7 total).

### Functions

**Hyperbolic (3)**: `sinh`, `cosh`, `tanh`
**Exponential/Log (2)**: `exp`, `ln`
**Roots/Abs (2)**: `sqrt`, `abs`

### Special Considerations

**Logarithm (ln)**:
- Domain: x > 0
- Branch cut: Negative real axis
- Domain errors in SpecialValuesMap

**Sqrt**:
- Domain: x ≥ 0 (real domain)
- Branch cut: Negative real axis (complex domain)

**Example**: `ln_special_values.rs`
```rust
pub static LN_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(10);

    // Exact values
    map.insert(Expression::integer(1), exact_value!(0, "\\ln(1) = 0"));
    map.insert(Expression::e(), exact_value!(1, "\\ln(e) = 1"));

    // Domain errors
    map.insert(
        Expression::integer(0),
        SpecialValueResult {
            kind: SpecialValueKind::DomainError,
            output: Expression::integer(0), // Placeholder
            latex_explanation: "\\ln(0) = -\\infty \\text{ (pole)}".to_string(),
            educational_notes: Some("Logarithmic singularity at zero".to_string()),
        }
    );

    map
});
```

### Deliverables

- 7 data files for elementary functions
- Implementation modifications
- Registry derivations

### Success Criteria

- [ ] 7 elementary functions migrated
- [ ] Domain errors and poles handled
- [ ] All tests pass

---

## WAVE 4: SPECIAL FUNCTIONS - GAMMA FAMILY (REFACTORING)

### Goal

**REFACTOR** gamma family functions from hardcoded match statements to shared data architecture.

**Task Type**: REFACTORING (Category A)
**Current State**: FULLY IMPLEMENTED functions with hardcoded special values
**Target State**: Same functionality, but using shared data architecture

### Functions

- `gamma` - ✅ IMPLEMENTED in `special/gamma.rs` (has hardcoded factorial/half-integers)
- `beta` - ✅ IMPLEMENTED in `special/gamma.rs` (uses gamma)
- `digamma` - ✅ IMPLEMENTED in `special/gamma.rs` (returns symbolic)
- `polygamma` - ✅ IMPLEMENTED in `special/gamma.rs` (returns symbolic)

### Current State

**gamma.rs** (hardcoded):
```rust
pub fn gamma(z: &Expression) -> Expression {
    match z {
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            // Hardcoded factorial: Γ(n) = (n-1)!
            let mut result = 1i64;
            for i in 1..val { result *= i; }
            Expression::Number(Number::Integer(result))
        }
        Expression::Number(Number::Float(x)) => {
            // Hardcoded half-integer check
            if (twice - twice.round()).abs() < 1e-10 {
                gamma_half_integer(*x)
            } else {
                let result = lanczos_gamma(*x);
                Expression::Number(Number::Float(result))
            }
        }
        _ => Expression::function("gamma", vec![z.clone()]),
    }
}
```

### Target State

**gamma_special_values.rs**:
```rust
pub static GAMMA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(30);

    // Exact integer values: Γ(n) = (n-1)!
    map.insert(Expression::integer(1), exact_value!(1, "\\Gamma(1) = 1"));
    map.insert(Expression::integer(2), exact_value!(1, "\\Gamma(2) = 1"));
    map.insert(Expression::integer(3), exact_value!(2, "\\Gamma(3) = 2"));
    map.insert(Expression::integer(4), exact_value!(6, "\\Gamma(4) = 6"));
    map.insert(Expression::integer(5), exact_value!(24, "\\Gamma(5) = 24"));
    // ... up to Γ(10) or Γ(20)

    // Half-integers: Γ(1/2), Γ(3/2), Γ(5/2), ...
    map.insert(
        Expression::rational(1, 2),
        exact_value!(sqrt(pi), "\\Gamma(\\frac{1}{2}) = \\sqrt{\\pi}")
    );
    map.insert(
        Expression::rational(3, 2),
        exact_value!(sqrt(pi) / 2, "\\Gamma(\\frac{3}{2}) = \\frac{\\sqrt{\\pi}}{2}")
    );
    // ... more half-integers

    // Poles: Γ(0), Γ(-1), Γ(-2), ...
    map.insert(
        Expression::integer(0),
        SpecialValueResult {
            kind: SpecialValueKind::DomainError,
            output: Expression::integer(0),
            latex_explanation: "\\Gamma(0) \\text{ has pole}".to_string(),
            educational_notes: Some("Gamma function has poles at non-positive integers".to_string()),
        }
    );

    map
});
```

**gamma.rs** (REFACTORED - preserves all existing functionality):
```rust
/// Gamma function Γ(z)
///
/// **REFACTORED**: Previously used hardcoded match statements, now uses shared data
///
/// The Gamma function extends the factorial to complex numbers:
/// Γ(n) = (n-1)! for positive integers n
///
/// Uses shared special values data from `gamma_special_values.rs`
pub fn gamma(z: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values FIRST (REFACTORED - was hardcoded before)
    if let Some(result) = GAMMA_SPECIAL_VALUES.get(z) {
        match result.kind {
            SpecialValueKind::Exact => return Ok(result.output.clone()),
            SpecialValueKind::DomainError => {
                return Err(MathError::DomainError {
                    operation: "gamma".to_string(),
                    value: z.clone(),
                    reason: result.latex_explanation.clone(),
                });
            }
            _ => {}
        }
    }

    // 2. Computed special values (reflection formula, duplication formula)
    // PRESERVE all existing logic here

    // 3. Numerical evaluation (PRESERVE existing Lanczos approximation)
    match z {
        Expression::Number(Number::Float(x)) => {
            Ok(Expression::Number(Number::Float(lanczos_gamma(*x))))
        }
        _ => Ok(Expression::function("gamma", vec![z.clone()])),
    }
}
```

**CRITICAL**: This is a REFACTORING task, NOT a new implementation.
- ✅ All existing tests MUST continue to pass
- ✅ Preserve all existing numerical methods (lanczos_gamma, gamma_half_integer)
- ✅ Preserve all existing functionality (reflection formula, etc.)
- ❌ DO NOT change mathematical behavior
- ✅ ONLY change: Move hardcoded values → shared data file

### Deliverables

- `functions/data/special/gamma_special_values.rs`
- `functions/data/special/beta_special_values.rs` (if beta has independent special values)
- Modify: `functions/special/gamma.rs` to use shared data
- Modify: Registry to derive from shared data

### Success Criteria (REFACTORING)

- [ ] Gamma family functions refactored (NOT re-implemented)
- [ ] All hardcoded match statements replaced with shared data lookups
- [ ] Factorial values (Γ(n) = (n-1)!) moved to shared data
- [ ] Half-integer values moved to shared data
- [ ] Poles moved to shared data (as domain errors)
- [ ] **ALL EXISTING TESTS PASS** (zero regressions)
- [ ] Existing numerical methods preserved (lanczos_gamma, gamma_half_integer)
- [ ] Mathematical behavior identical to before refactoring

---

## WAVE 5: SPECIAL FUNCTIONS - BESSEL + ZETA (REFACTORING)

### Goal

**REFACTOR** Bessel functions and zeta function to shared data architecture.

**Task Type**: REFACTORING (Category A)
**Current State**: FULLY IMPLEMENTED functions with hardcoded special values
**Target State**: Same functionality, but using shared data architecture

### Functions

- `bessel_j` - ✅ IMPLEMENTED in `special/bessel.rs` (numerical evaluation with few special cases)
- `bessel_y` - ✅ IMPLEMENTED in `special/bessel.rs` (numerical evaluation with few special cases)
- `zeta` - ✅ IMPLEMENTED in `special/zeta.rs` (hardcoded ζ(0), ζ(2), ζ(4), etc.)

### Special Considerations

**Bessel Functions**:
- Most values are numerical (computed via series expansions or asymptotic formulas)
- Special values: J_0(0) = 1, J_n(0) = 0 (n > 0), Y_n(0) = -∞ (pole)

**Zeta Function**:
- Currently has hardcoded match statements in `zeta.rs`
- Special values: ζ(0) = -1/2, ζ(2) = π²/6, ζ(4) = π⁴/90, etc.

**Example**: `zeta_special_values.rs`
```rust
pub static ZETA_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(20);

    // Exact values
    map.insert(Expression::integer(0), exact_value!(-1/2, "\\zeta(0) = -\\frac{1}{2}"));
    map.insert(
        Expression::integer(2),
        exact_value!(pi^2 / 6, "\\zeta(2) = \\frac{\\pi^2}{6}")
    );
    map.insert(
        Expression::integer(4),
        exact_value!(pi^4 / 90, "\\zeta(4) = \\frac{\\pi^4}{90}")
    );
    // ... more even zeta values

    // Pole
    map.insert(
        Expression::integer(1),
        SpecialValueResult {
            kind: SpecialValueKind::DomainError,
            output: Expression::integer(0),
            latex_explanation: "\\zeta(1) \\text{ has simple pole}".to_string(),
            educational_notes: Some("Riemann zeta function has pole at s=1".to_string()),
        }
    );

    map
});
```

### Deliverables

- 5 data files: `bessel_j_special_values.rs`, `bessel_y_special_values.rs`, `bessel_i_special_values.rs`, `zeta_special_values.rs`, `erf_special_values.rs`
- Modify implementations to use shared data
- Registry derivations

### Success Criteria

- [ ] 5 special functions migrated
- [ ] All hardcoded values replaced
- [ ] Poles and domain errors handled
- [ ] All tests pass

---

## WAVE 6: POLYNOMIAL FUNCTIONS (IMPLEMENTATION)

### Goal

**IMPLEMENT** 4 orthogonal polynomial functions from scratch with shared data architecture.

**Task Type**: IMPLEMENTATION (Category B)
**Current State**: Only intelligence structs exist, NO `pub fn` implementations
**Target State**: Fully implemented functions with shared data architecture

### Functions

- `hermite` - ❌ NOT IMPLEMENTED (only `HermiteIntelligence` struct + helpers)
- `laguerre` - ❌ NOT IMPLEMENTED (only `LaguerreIntelligence` struct + helpers)
- `chebyshev` - ❌ NOT IMPLEMENTED (only `ChebyshevIntelligence` struct + helpers)
- `legendre` - ❌ NOT IMPLEMENTED (only `LegendreIntelligence` struct + helpers)

### Special Considerations

**Polynomial Functions**:
- Primarily use recurrence relations (not special value lookups)
- Special values: P_n(0), P_n(1), P_n(-1) for some polynomials
- Orthogonality properties (not special values per se)

**Example**: `legendre_special_values.rs`
```rust
pub static LEGENDRE_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(10);

    // P_n(1) = 1 for all n
    // P_n(-1) = (-1)^n for all n
    // P_n(0) depends on n

    // These are pattern-based, not individual special values
    // May need a different representation or computed approach

    map
});
```

**Note**: Polynomial functions may require a different shared data pattern since they rely more on recurrence relations than special value lookups.

### Deliverables

- 4 data files (or alternative representation if special values are sparse)
- Modify implementations
- Registry derivations

### Success Criteria (IMPLEMENTATION)

- [ ] 4 polynomial functions IMPLEMENTED from scratch
- [ ] Callable `pub fn hermite()`, `pub fn laguerre()`, etc. created
- [ ] Shared data files created for base cases
- [ ] Recurrence relations implemented (use existing helpers)
- [ ] New tests added for each function (since they're newly implemented)
- [ ] Integration with registry completed

---

## CROSS-WAVE VERIFICATION PROTOCOLS

### After Each Wave

1. **Unit Tests**: `cargo test -p mathhook-core <module>`
2. **Integration Tests**: `cargo test -p mathhook-core --test integration_tests`
3. **Regression Tests**: `cargo test -p mathhook-core --no-fail-fast`
4. **Doctests**: `cargo test --doc`
5. **Performance Benchmarks**: `cargo bench --bench <wave_functions>`

### After All Waves Complete

1. **Full Test Suite**: `cargo test`
2. **SymPy Validation**: Compare all function evaluations with SymPy
3. **Performance Regression Check**: Ensure no >10% slowdown
4. **Memory Profile**: Verify LazyLock overhead is acceptable (<1MB per function)
5. **Documentation Review**: All shared data files have complete documentation

---

## RISK MITIGATION

### Identified Risks

1. **Performance Degradation**:
   - **Mitigation**: Benchmark each wave, LazyLock is <100ns overhead
   - **Fallback**: If severe, add caching layer

2. **Data Duplication Creep**:
   - **Mitigation**: CI checks for duplicated special values (grep for patterns)
   - **Fallback**: Automated cleanup scripts

3. **Incomplete Migration**:
   - **Mitigation**: Comprehensive function inventory, track completion per wave
   - **Fallback**: Partial migration acceptable (documented)

4. **Mathematical Correctness Regressions**:
   - **Mitigation**: SymPy validation after each wave
   - **Fallback**: Revert to previous implementation if correctness violated

---

## SUCCESS METRICS (FINAL)

### Quantitative Metrics

- [ ] **28 functions migrated** (excluding 4 number theory)
- [ ] **28 shared data files created** (one per function)
- [ ] **Zero duplication**: Special values in exactly ONE location
- [ ] **All tests pass**: 100% test success rate
- [ ] **Performance maintained**: <10% slowdown acceptable
- [ ] **Code reduction**: Estimated 30-40% reduction in duplication

### Qualitative Metrics

- [ ] **Single source of truth**: Registry derives from shared data
- [ ] **Separation of concerns**: Data files separate from implementation
- [ ] **Educational clarity**: LaTeX explanations and notes in data
- [ ] **Maintainability**: New special values added in ONE place only

---

## POST-MIGRATION CLEANUP

### Tasks

1. **Remove Old Code**:
   - Delete embedded special values from properties structs
   - Remove hardcoded match statements for special values

2. **Documentation Update**:
   - Update CLAUDE.md with new architecture pattern
   - Document shared data pattern in each function module

3. **CI Integration**:
   - Add linter rule: No special values in properties structs
   - Add validation: Registry must derive from shared data

---

## APPENDIX A: FUNCTION-BY-FUNCTION STATUS MATRIX

| Function | Category | Implementation Status | Current Pattern | Target Data File | Task Type | Wave | Status |
|----------|----------|----------------------|-----------------|------------------|-----------|------|--------|
| sin | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | sin_special_values.rs | IMPLEMENT | 1 | Not Started |
| cos | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | cos_special_values.rs | IMPLEMENT | 1 | Not Started |
| tan | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | tan_special_values.rs | IMPLEMENT | 1 | Not Started |
| cot | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | cot_special_values.rs | IMPLEMENT | 1 | Not Started |
| sec | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | sec_special_values.rs | IMPLEMENT | 1 | Not Started |
| csc | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | csc_special_values.rs | IMPLEMENT | 1 | Not Started |
| asin | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | asin_special_values.rs | IMPLEMENT | 2 | Not Started |
| acos | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | acos_special_values.rs | IMPLEMENT | 2 | Not Started |
| atan | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | atan_special_values.rs | IMPLEMENT | 2 | Not Started |
| acot | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | acot_special_values.rs | IMPLEMENT | 2 | Not Started |
| asec | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | asec_special_values.rs | IMPLEMENT | 2 | Not Started |
| acsc | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | acsc_special_values.rs | IMPLEMENT | 2 | Not Started |
| sinh | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | sinh_special_values.rs | IMPLEMENT | 3 | Not Started |
| cosh | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | cosh_special_values.rs | IMPLEMENT | 3 | Not Started |
| tanh | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | tanh_special_values.rs | IMPLEMENT | 3 | Not Started |
| exp | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | exp_special_values.rs | IMPLEMENT | 3 | Not Started |
| ln | Elementary | ❌ NOT IMPLEMENTED | Intelligence struct only | ln_special_values.rs | IMPLEMENT | 3 | Not Started |
| sqrt | Elementary | ❌ NOT IMPLEMENTED | Helper exists, no pub fn | sqrt_special_values.rs | IMPLEMENT | 3 | Not Started |
| abs | Elementary | ❌ NOT IMPLEMENTED | Helper exists, no pub fn | abs_special_values.rs | IMPLEMENT | 3 | Not Started |
| gamma | Special | ✅ IMPLEMENTED | Hardcoded factorial/half-int | gamma_special_values.rs | REFACTOR | 4 | Not Started |
| beta | Special | ✅ IMPLEMENTED | Uses gamma | beta_special_values.rs | REFACTOR | 4 | Not Started |
| digamma | Special | ✅ IMPLEMENTED | Returns symbolic | (included in gamma file) | REFACTOR | 4 | Not Started |
| polygamma | Special | ✅ IMPLEMENTED | Returns symbolic | (included in gamma file) | REFACTOR | 4 | Not Started |
| bessel_j | Special | ✅ IMPLEMENTED | Numerical evaluation | bessel_j_special_values.rs | REFACTOR | 5 | Not Started |
| bessel_y | Special | ✅ IMPLEMENTED | Numerical evaluation | bessel_y_special_values.rs | REFACTOR | 5 | Not Started |
| zeta | Special | ✅ IMPLEMENTED | Hardcoded special values | zeta_special_values.rs | REFACTOR | 5 | Not Started |
| hermite | Polynomial | ❌ NOT IMPLEMENTED | Intelligence struct + helpers | hermite_special_values.rs | IMPLEMENT | 6 | Not Started |
| laguerre | Polynomial | ❌ NOT IMPLEMENTED | Intelligence struct + helpers | laguerre_special_values.rs | IMPLEMENT | 6 | Not Started |
| chebyshev | Polynomial | ❌ NOT IMPLEMENTED | Intelligence struct + helpers | chebyshev_special_values.rs | IMPLEMENT | 6 | Not Started |
| legendre | Polynomial | ❌ NOT IMPLEMENTED | Intelligence struct + helpers | legendre_special_values.rs | IMPLEMENT | 6 | Not Started |
| gcd | Number Theory | ✅ IMPLEMENTED | Algorithmic (Euclidean) | N/A (no HashMap) | None | - | Not Applicable |
| lcm | Number Theory | ✅ IMPLEMENTED | Algorithmic (uses gcd) | N/A (no HashMap) | None | - | Not Applicable |
| mod | Number Theory | ✅ IMPLEMENTED | Algorithmic (modulo op) | N/A (no HashMap) | None | - | Not Applicable |
| is_prime | Number Theory | ✅ IMPLEMENTED | Algorithmic (primality) | N/A (no HashMap) | None | - | Not Applicable |

---

## APPENDIX B: CODE TEMPLATES

### CRITICAL: Two Different Task Types

This migration involves **TWO DISTINCT PATTERNS**:

1. **REFACTORING Pattern** (Category A - gamma, zeta, bessel):
   - Existing `pub fn` implementation already works
   - Extract hardcoded special values → move to shared data file
   - Refactor implementation to check shared data first
   - Preserve all existing functionality and tests

2. **IMPLEMENTATION Pattern** (Category B - sin, cos, hermite, etc.):
   - NO existing `pub fn` implementation
   - Create shared data file from intelligence struct special values
   - Implement `pub fn` WITH shared data from the start
   - Add new tests for the newly implemented function

---

### Template: Shared Data File (Both Patterns)

```rust
//! Special values for <FUNCTION> function
//!
//! Single source of truth for <FUNCTION> special values used by:
//! - Implementation (functions/<category>/<file>.rs)
//! - Registry (functions/properties/<category>.rs)

use once_cell::sync::LazyLock;
use crate::core::Expression;
use crate::functions::data::types::{SpecialValuesMap, SpecialValueResult, SpecialValueKind};

/// Special values for <FUNCTION>(x) function
///
/// # Mathematical Properties
///
/// [Describe key properties, domain, range, special cases]
///
/// # Coverage
///
/// - Exact values: [List key exact values]
/// - Domain errors: [List poles, branch cuts, undefined points]
/// - Branch points: [List multi-valued cases if applicable]
pub static <FUNCTION_UPPER>_SPECIAL_VALUES: LazyLock<SpecialValuesMap> = LazyLock::new(|| {
    let mut map = SpecialValuesMap::with_capacity(<COUNT>); // Exact count

    // Exact values
    map.insert(
        <INPUT_EXPR>,
        SpecialValueResult {
            kind: SpecialValueKind::Exact,
            output: <OUTPUT_EXPR>,
            latex_explanation: "<LATEX>".to_string(),
            educational_notes: Some("<EDUCATIONAL_NOTE>".to_string()),
        }
    );

    // Domain errors (poles, branch cuts, undefined)
    map.insert(
        <ERROR_INPUT_EXPR>,
        SpecialValueResult {
            kind: SpecialValueKind::DomainError,
            output: Expression::integer(0), // Placeholder
            latex_explanation: "<ERROR_LATEX>".to_string(),
            educational_notes: Some("<ERROR_NOTE>".to_string()),
        }
    );

    // More entries...

    map
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_<function>_special_values_coverage() {
        // Verify expected count
        assert_eq!(<FUNCTION_UPPER>_SPECIAL_VALUES.len(), <COUNT>);

        // Verify key exact values
        assert!(matches!(
            <FUNCTION_UPPER>_SPECIAL_VALUES.get(&<INPUT_EXPR>),
            Some(SpecialValueResult { kind: SpecialValueKind::Exact, .. })
        ));

        // Verify domain errors
        assert!(matches!(
            <FUNCTION_UPPER>_SPECIAL_VALUES.get(&<ERROR_INPUT_EXPR>),
            Some(SpecialValueResult { kind: SpecialValueKind::DomainError, .. })
        ));
    }

    #[test]
    fn test_<function>_lazy_initialization() {
        // Verify LazyLock works correctly
        let start = std::time::Instant::now();
        let _map = &*<FUNCTION_UPPER>_SPECIAL_VALUES;
        let duration = start.elapsed();

        // First access should be fast (lazy init)
        assert!(duration.as_millis() < 100);
    }
}
```

### Template A: REFACTORING Existing Implementation (Category A - gamma, zeta, bessel)

**Use this pattern when the function already has a working `pub fn` implementation with hardcoded special values.**

```rust
use crate::functions::data::<category>::<FUNCTION_UPPER>_SPECIAL_VALUES;
use crate::core::{Expression, MathError};

/// <FUNCTION> function implementation
///
/// Uses shared special values data from `<FUNCTION_LOWER>_special_values.rs`
///
/// **REFACTORED**: Previously used hardcoded match statements, now uses shared data
pub fn <function>(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values FIRST (REFACTORED - was hardcoded before)
    if let Some(result) = <FUNCTION_UPPER>_SPECIAL_VALUES.get(arg) {
        match result.kind {
            SpecialValueKind::Exact => return Ok(result.output.clone()),
            SpecialValueKind::DomainError => {
                return Err(MathError::DomainError {
                    operation: "<function>".to_string(),
                    value: arg.clone(),
                    reason: result.latex_explanation.clone(),
                });
            }
            _ => {} // Branch points, etc.
        }
    }

    // 2. Computed special values (symmetry, reflection formula, etc.)
    // PRESERVE existing logic here - don't remove

    // 3. Numerical evaluation (PRESERVE existing numerical methods)
    // For gamma: lanczos_gamma()
    // For zeta: zeta_numerical()
    // For bessel: series expansion or asymptotic formulas
    match arg {
        Expression::Number(Number::Float(x)) => {
            Ok(Expression::Number(Number::Float(<existing_numerical_function>(*x))))
        }
        _ => {
            // 4. Unevaluated (symbolic)
            Ok(Expression::function("<function>", vec![arg.clone()]))
        }
    }
}
```

**Example: Refactoring gamma.rs**
```rust
// BEFORE (hardcoded):
pub fn gamma(z: &Expression) -> Expression {
    match z {
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            // Hardcoded factorial
            let mut result = 1i64;
            for i in 1..val { result *= i; }
            Expression::Number(Number::Integer(result))
        }
        // ...
    }
}

// AFTER (uses shared data):
pub fn gamma(z: &Expression) -> Result<Expression, MathError> {
    // Check shared data FIRST
    if let Some(result) = GAMMA_SPECIAL_VALUES.get(z) {
        match result.kind {
            SpecialValueKind::Exact => return Ok(result.output.clone()),
            SpecialValueKind::DomainError => { /* ... */ }
            _ => {}
        }
    }

    // Preserve existing numerical evaluation
    match z {
        Expression::Number(Number::Float(x)) => {
            Ok(Expression::Number(Number::Float(lanczos_gamma(*x))))
        }
        _ => Ok(Expression::function("gamma", vec![z.clone()])),
    }
}
```

### Template B: NEW IMPLEMENTATION (Category B - sin, cos, hermite, etc.)

**Use this pattern when implementing a function from scratch (no existing `pub fn`).**

```rust
use crate::functions::data::<category>::<FUNCTION_UPPER>_SPECIAL_VALUES;
use crate::core::{Expression, Number, MathError};

/// <FUNCTION> function implementation
///
/// Uses shared special values data from `<FUNCTION_LOWER>_special_values.rs`
///
/// **NEWLY IMPLEMENTED**: This function is being implemented for the first time
/// with shared data architecture from the start.
pub fn <function>(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values (exact or error)
    if let Some(result) = <FUNCTION_UPPER>_SPECIAL_VALUES.get(arg) {
        match result.kind {
            SpecialValueKind::Exact => return Ok(result.output.clone()),
            SpecialValueKind::DomainError => {
                return Err(MathError::DomainError {
                    operation: "<function>".to_string(),
                    value: arg.clone(),
                    reason: result.latex_explanation.clone(),
                });
            }
            _ => {} // Branch points, etc.
        }
    }

    // 2. Computed special values (symmetry, periodicity, identities)
    // Implement based on mathematical properties
    // Example for sin: sin(-x) = -sin(x), sin(x + 2π) = sin(x)

    // 3. Numerical evaluation (implement based on function type)
    match arg {
        Expression::Number(Number::Float(x)) => {
            // Use Rust standard library or custom implementation
            Ok(Expression::Number(Number::Float(x.<std_function>())))
        }
        Expression::Number(Number::Integer(n)) => {
            // Convert to float for numerical evaluation
            let x = *n as f64;
            Ok(Expression::Number(Number::Float(x.<std_function>())))
        }
        _ => {
            // 4. Unevaluated (symbolic)
            Ok(Expression::function("<function>", vec![arg.clone()]))
        }
    }
}
```

**Example: Implementing sin from scratch**
```rust
use crate::functions::data::trig::SIN_SPECIAL_VALUES;
use crate::core::{Expression, Number, MathError};

/// Sine function sin(x)
///
/// Uses shared special values data from `sin_special_values.rs`
pub fn sin(arg: &Expression) -> Result<Expression, MathError> {
    // 1. Check shared special values
    if let Some(result) = SIN_SPECIAL_VALUES.get(arg) {
        match result.kind {
            SpecialValueKind::Exact => return Ok(result.output.clone()),
            _ => {}
        }
    }

    // 2. Computed special values (symmetry, periodicity)
    // sin(-x) = -sin(x)
    // sin(x + 2π) = sin(x)
    // (Implement these transformations)

    // 3. Numerical evaluation
    match arg {
        Expression::Number(Number::Float(x)) => {
            Ok(Expression::Number(Number::Float(x.sin())))
        }
        Expression::Number(Number::Integer(n)) => {
            let x = *n as f64;
            Ok(Expression::Number(Number::Float(x.sin())))
        }
        _ => {
            // 4. Symbolic (unevaluated)
            Ok(Expression::function("sin", vec![arg.clone()]))
        }
    }
}
```

### Template: Registry Derivation

```rust
use crate::functions::data::<category>::<FUNCTION_LOWER>_SPECIAL_VALUES;

FunctionProperties {
    name: "<function>",
    special_values: <FUNCTION_UPPER>_SPECIAL_VALUES.iter()
        .filter_map(|(input, result)| {
            // Only include exact values in registry (not domain errors)
            if matches!(result.kind, SpecialValueKind::Exact) {
                Some(SpecialValue {
                    input: input.to_string(),
                    output: result.output.clone(),
                    latex_explanation: result.latex_explanation.clone(),
                })
            } else {
                None
            }
        })
        .collect(),
    derivative_rule: Some(...),
    domain_range: Box::new(...),
    // ... other properties
}
```

---

## APPENDIX C: VERIFICATION CHECKLIST (PER WAVE)

### Pre-Wave Checklist

- [ ] Read SHARED_DATA_ARCHITECTURE_COMPLETE.md (relevant sections)
- [ ] Read CLAUDE.md (Function Evaluation Architecture section)
- [ ] Identify current state for all functions in wave
- [ ] Create wave-specific test plan

### During Wave

- [ ] Create shared data files for all functions
- [ ] Modify implementations to use shared data
- [ ] Modify registry to derive from shared data
- [ ] Remove old embedded special values
- [ ] Run unit tests after each function
- [ ] Run integration tests after all functions

### Post-Wave Checklist

- [ ] All wave tests pass: `cargo test -p mathhook-core <wave_module>`
- [ ] No regressions: `cargo test -p mathhook-core --no-fail-fast`
- [ ] Doctests pass: `cargo test --doc`
- [ ] Performance benchmarks: No >10% slowdown
- [ ] Code review: Zero duplication verified
- [ ] SymPy validation: Mathematical correctness confirmed
- [ ] Documentation: Shared data files fully documented
- [ ] Update status matrix: Mark functions as "Complete"

---

## APPENDIX D: MIGRATION PATTERNS BY FUNCTION TYPE

### Pattern A: Simple Exact Values (sin, cos, exp, etc.)

**Characteristics**:
- Few domain errors
- Primarily exact symbolic values
- Straightforward evaluation

**Migration**:
1. Extract special values from properties struct
2. Create shared data file with LazyLock
3. Implementation: Check shared data first
4. Registry: Derive from shared data

### Pattern B: Domain Restrictions (asin, ln, sqrt, etc.)

**Characteristics**:
- Domain errors (poles, branch cuts)
- Exact values AND error cases
- Need MathError handling

**Migration**:
1. Identify all domain errors
2. Create shared data file with both exact values AND domain errors
3. Implementation: Return Err on domain errors
4. Registry: Filter out domain errors (only show exact values)

### Pattern C: Hardcoded Match Statements (gamma, zeta)

**Characteristics**:
- Currently hardcoded in implementation
- Many special cases (integers, half-integers, poles)
- Complex evaluation logic

**Migration**:
1. Extract ALL hardcoded values from match statements
2. Create comprehensive shared data file
3. Refactor implementation: Check shared data before fallback to numerical
4. Registry: Derive from shared data

### Pattern D: Numerical Primary (bessel, erf)

**Characteristics**:
- Mostly numerical evaluation
- Few exact special values (often just domain boundaries)
- Performance-critical

**Migration**:
1. Identify sparse exact values (e.g., J_0(0) = 1)
2. Create minimal shared data file
3. Implementation: Quick check, then numerical evaluation
4. Registry: Derive from sparse data

### Pattern E: Recurrence Relations (hermite, legendre)

**Characteristics**:
- Polynomial recurrence relations
- Pattern-based values (not individual lookups)
- Orthogonality properties

**Migration**:
1. Consider if shared data pattern applies
2. May need alternative representation (recurrence parameters vs special values)
3. Shared data may store base cases only
4. Implementation: Check base cases, then use recurrence

---

## END OF ORCHESTRATOR COMMAND

**Next Steps**:
1. Read this document completely
2. Read SHARED_DATA_ARCHITECTURE_COMPLETE.md
3. Begin Wave 1: Foundation + Trigonometric Functions
4. Update status matrix as you progress
5. Verify after each function migration

**Questions or Clarifications**:
- Refer to CLAUDE.md first (single source of truth)
- Check SHARED_DATA_ARCHITECTURE_COMPLETE.md for detailed patterns
- Ask user if architectural decisions needed
