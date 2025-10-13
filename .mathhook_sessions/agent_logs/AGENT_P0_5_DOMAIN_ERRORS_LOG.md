# Agent P0-5: Domain Guardian

**Task**: P0-5 - Implement Domain Error System
**Status**: NOT_STARTED
**Progress**: 0%
**Priority**: P0 (CRITICAL - CORRECTNESS)
**Estimated Duration**: 3-4 days
**Started**: -
**Last Update**: -

---

## Mission Briefing

Implement a comprehensive domain error system for mathematical operations. Currently, operations return symbolic "undefined" instead of proper errors, violating mathematical correctness.

**Current Problem**:
- No error types exist for domain violations
- Line 599-602 in `simplify/arithmetic.rs`: Returns `undefined` function for 0^(-1) instead of error
- Functions don't check domain restrictions (sqrt of negatives, log of non-positives, etc.)

**CLAUDE.md Requirement**: "Use Result<Expression, DomainError> for operations that can fail. Domain restrictions: Always check for division by zero, sqrt of negatives, log of non-positives"

**Reference Material**:
- Task details: `.mathhook_sessions/0.1_RELEASE_READINESS_AI_AGENT.md` (TASK P0-5)
- Error Handling Principles: `CLAUDE.md` (lines 89-137, Error Handling section)

---

## Current Objective

Waiting for launch command...

---

## Implementation Plan

### Phase 1: Define Error Types (Day 1)
- [ ] Create `crates/mathhook-core/src/error.rs`
- [ ] Define `MathError` enum with all variants
- [ ] Implement `Display` trait for user-friendly error messages
- [ ] Implement `std::error::Error` trait
- [ ] Add to `lib.rs` exports: `pub use error::*;`

### Phase 2: Fix Existing Violations (Day 1-2)
- [ ] Fix `simplify/arithmetic.rs` lines 599-602 (0^(-1) case)
- [ ] Change function signature to return `Result<Expression, MathError>`
- [ ] Update all call sites
- [ ] Replace all symbolic "undefined" returns with proper errors
- [ ] Update tests to expect errors

### Phase 3: Add Domain Checks (Day 2-3)
- [ ] Add domain checks for `sqrt(x)` - real domain requires x ≥ 0
- [ ] Add domain checks for `log(x)` - requires x > 0 (pole at 0)
- [ ] Add domain checks for `tan(x)` - poles at x = π/2 + nπ
- [ ] Add domain checks for `sec(x)`, `csc(x)` - similar poles
- [ ] Add domain checks for `arcsin(x)`, `arccos(x)` - domain [-1, 1]
- [ ] Add domain checks for division - denominator ≠ 0
- [ ] Add domain checks for power - handle 0^0, 0^(-n)

### Phase 4: Comprehensive Testing (Day 3-4)
- [ ] Test `sqrt(-1)` returns DomainError in real domain
- [ ] Test `log(0)` returns Pole error
- [ ] Test `log(-1)` returns BranchCut error in real domain
- [ ] Test `1/0` returns DivisionByZero
- [ ] Test `tan(π/2)` returns Pole error
- [ ] Test `0^0` returns Undefined error
- [ ] Test `arcsin(2)` returns DomainError
- [ ] Target: 20+ domain error tests

### Phase 5: Integration & Documentation (Day 4)
- [ ] Update function evaluation to check domains
- [ ] Update simplification to propagate errors
- [ ] Document error handling in API docs
- [ ] Add examples showing error handling
- [ ] Verify no regressions

---

## Completed Work

_Nothing yet - waiting for launch_

---

## Files to Create

- [ ] `crates/mathhook-core/src/error.rs` (NEW - main error types)
- [ ] `crates/mathhook-core/tests/domain_error_tests.rs` (NEW - comprehensive tests)

---

## Files to Modify

- [ ] `crates/mathhook-core/src/lib.rs` (add `pub use error::*;`)
- [ ] `crates/mathhook-core/src/simplify/arithmetic.rs` (fix lines 599-602)
- [ ] `crates/mathhook-core/src/functions/elementary/*.rs` (add domain checks)
- [ ] Any file returning symbolic "undefined"
- [ ] All call sites that need to handle `Result` returns

---

## Error Type Definition

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Domain error - operation not valid for given input
    DomainError {
        operation: String,
        value: Expression,
        reason: String,
    },

    /// Division by zero
    DivisionByZero,

    /// Undefined expression (e.g., 0^0)
    Undefined {
        expression: Expression,
        reason: String,
    },

    /// Numeric overflow
    NumericOverflow {
        operation: String,
    },

    /// Feature not yet implemented
    NotImplemented {
        feature: String,
    },

    /// Pole singularity (e.g., tan(π/2))
    Pole {
        function: String,
        at: Expression,
    },

    /// Branch cut issue for multi-valued function
    BranchCut {
        function: String,
        value: Expression,
    },
}
```

---

## Functions Needing Domain Checks

### Critical Functions
- [ ] `sqrt(x)` - real domain: x ≥ 0
- [ ] `log(x)` - domain: x > 0 (pole at 0, branch cut for x < 0)
- [ ] `tan(x)` - poles at x = π/2 + nπ
- [ ] `sec(x)` - poles at x = π/2 + nπ
- [ ] `csc(x)` - poles at x = nπ
- [ ] `arcsin(x)`, `arccos(x)` - real domain: [-1, 1]
- [ ] Division - denominator ≠ 0
- [ ] Power - special cases: 0^0, 0^(-n)

---

## Tests Status

**Target**: 20+ domain error tests
**Current**: 0 tests

### Test Categories
- [ ] Square root domain errors (2 tests)
- [ ] Logarithm domain errors (3 tests)
- [ ] Trigonometric poles (3 tests)
- [ ] Inverse trig domain errors (2 tests)
- [ ] Division by zero (2 tests)
- [ ] Power edge cases (3 tests)
- [ ] Error message quality (5+ tests)

---

## Key Test Cases

### Test 1: Square Root Domain
```rust
#[test]
fn test_sqrt_negative_real_domain() {
    let expr = Expression::function("sqrt", vec![Expression::integer(-1)]);
    let result = expr.evaluate_real();  // Real domain evaluation

    match result {
        Err(MathError::DomainError { operation, .. }) => {
            assert_eq!(operation, "sqrt");
        }
        _ => panic!("Expected domain error for sqrt(-1)"),
    }
}
```

### Test 2: Log Pole
```rust
#[test]
fn test_log_zero() {
    let expr = Expression::function("log", vec![Expression::integer(0)]);
    let result = expr.evaluate();

    match result {
        Err(MathError::Pole { function, .. }) => {
            assert_eq!(function, "log");
        }
        _ => panic!("Expected pole error for log(0)"),
    }
}
```

### Test 3: Division By Zero
```rust
#[test]
fn test_division_by_zero() {
    let expr = Expression::div(
        Expression::integer(1),
        Expression::integer(0)
    );
    let result = expr.evaluate();

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}
```

### Test 4: 0^0 Indeterminate
```rust
#[test]
fn test_zero_to_zero() {
    let expr = Expression::pow(
        Expression::integer(0),
        Expression::integer(0)
    );
    let result = expr.evaluate();

    match result {
        Err(MathError::Undefined { reason, .. }) => {
            assert!(reason.contains("indeterminate"));
        }
        _ => panic!("Expected undefined error for 0^0"),
    }
}
```

---

## Critical Fix Location

**File**: `crates/mathhook-core/src/simplify/arithmetic.rs`
**Lines**: 599-602

```rust
// CURRENT (BROKEN):
(Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(-1))) => {
    Expression::function("undefined".to_string(), vec![])  // BAD!
}

// FIX:
// Change function signature:
pub fn simplify_power(base: &Expression, exp: &Expression) -> Result<Expression, MathError> {
    match (base, exp) {
        (Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(-1))) => {
            Err(MathError::DivisionByZero)  // 0^(-1) = 1/0
        }

        (Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(0))) => {
            Err(MathError::Undefined {
                expression: Expression::pow(
                    Expression::integer(0),
                    Expression::integer(0)
                ),
                reason: "0^0 is indeterminate".to_string(),
            })
        }

        // ... other cases return Ok(Expression)
    }
}
```

---

## Blockers

**Current Blockers**:
- May need coordination with P0-4 (Number Safety Engineer) for shared `MathError` type

_Will document any blockers during implementation_

---

## Next Steps

1. Await launch command
2. Create `error.rs` with comprehensive `MathError` enum
3. Fix `simplify/arithmetic.rs` violation
4. Add domain checks to elementary functions

---

## Questions for Manager

- Should I coordinate with P0-4 agent for shared error type definition?
- Priority order for adding domain checks (start with most common: sqrt, log, division)?

---

## Verification Checklist

When marking COMPLETE, verify:
- [ ] `MathError` enum defined with all necessary variants
- [ ] Display and Error traits implemented
- [ ] No symbolic "undefined" returns anywhere in codebase
- [ ] All domain violations return proper errors
- [ ] 20+ domain error tests passing
- [ ] Error messages are clear and helpful
- [ ] Function signatures return `Result<Expression, MathError>` where appropriate
- [ ] All call sites updated to handle Result
- [ ] Documentation shows error handling examples
- [ ] Code follows CLAUDE.md error handling principles
- [ ] No regressions in existing tests

---

**Agent Status**: STANDBY - Ready to launch
**Blocking**: Proper mathematical error handling system-wide
**Coordination**: May need to sync with P0-4 for shared error types
