# Agent 2.1B: Test Files Symbol Migration

## Mission
Migrate Symbol::new() uses in test files to the symbol!() macro. This agent handles **test files across the workspace** (~40+ uses from real_world_problems.rs and others).

## Context
- **Wave**: 2.1 - Symbol Constructor Migration Part 1
- **Target**: Reduce 227 → 114 Symbol::new() uses (50% reduction)
- **Agent Scope**: Test files (~40+ uses)
- **Priority**: MANDATORY (CLAUDE.md Priority 1)

## Files Assigned
Primary focus:
- `crates/mathhook/tests/real_world_problems.rs` (40 uses)
- `crates/mathhook-core/tests/educational_noncommutative_error_tests.rs` (8 uses)
- `crates/mathhook-core/tests/test_pde_integration.rs` (7 uses)
- `crates/mathhook-core/tests/test_ode_integration.rs` (6 uses)
- `crates/mathhook/tests/integration_api_choice.rs` (5 uses)
- `crates/mathhook/tests/integration_ui_integration.rs` (4 uses)
- `crates/mathhook/tests/integration_step_by_step.rs` (3 uses)
- Other test files with Symbol::new() uses

## Critical Constraints

### CLAUDE.md Compliance
**PRIORITY 1 (MANDATORY)**: Symbol Creation Rules
- ❌ **NEVER** use `Symbol::new()` in application code (tests count as application code)
- ✅ **ALWAYS** use `symbol!(x)` macro for scalar symbols
- ✅ Use `symbol!(A; matrix)` for matrix symbols
- ✅ Use `symbol!(p; operator)` for operator symbols

**Exception**: Tests that **specifically test Symbol constructors** can keep Symbol::new()
- Example: `test_symbol_creation()`, `test_symbol_equality()`
- All other test code MUST use macros

### Test-Specific Considerations

**Advantage**: Test code is perfect for macros!
- Test names are known at compile time
- Tests use literal variable names (x, y, z, theta, etc.)
- Tests benefit from readable macro syntax

## Migration Patterns

### Pattern 1: Test Setup (Most Common)
```rust
// ❌ Before:
#[test]
fn test_derivative() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    let expr = ...
}

// ✅ After:
#[test]
fn test_derivative() {
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = ...
}
```

### Pattern 2: Bulk Symbol Creation in Tests
```rust
// ❌ Before:
let x = Symbol::new("x");
let y = Symbol::new("y");
let z = Symbol::new("z");

// ✅ After (Recommended - cleaner):
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// ✅ After (Alternative - symbols![] macro):
let syms = symbols![x, y, z];
let x = &syms[0];
let y = &syms[1];
let z = &syms[2];
```

### Pattern 3: Matrix/Operator Symbols in Tests
```rust
// ❌ Before:
let A = Symbol::matrix("A");
let p = Symbol::operator("p");

// ✅ After:
let A = symbol!(A; matrix);
let p = symbol!(p; operator);
```

### Pattern 4: Dynamic Test Cases (KEEP Symbol::new!)
```rust
// ✅ CORRECT - Runtime variables, keep as-is:
for var_name in ["x", "y", "z", "theta"] {
    let sym = Symbol::new(var_name);  // Runtime value - DO NOT change to macro
    test_something(sym);
}
```

## Verification Steps

After migration, verify:

1. **Count Check**:
   ```bash
   # Should show significant reduction
   rg "Symbol::new\(" --type rust --glob '**/tests/*.rs' -c | awk -F: '{sum+=$2} END {print sum}'
   ```

2. **Test Suite**:
   ```bash
   # ALL tests must pass
   cargo test --all
   ```

3. **Integration Tests**:
   ```bash
   cargo test -p mathhook --test integration_api_choice
   cargo test -p mathhook --test integration_ui_integration
   cargo test -p mathhook --test real_world_problems
   ```

4. **No Runtime Variables in Macros**:
   ```bash
   # Should return 0
   rg "for .* in.*symbol!\(" --type rust --glob '**/tests/*.rs' -c
   ```

## Expected Outcomes

- **Symbol::new() uses**: Reduce from ~40 to ~20 in test files
- **All tests pass**: Zero regressions in test functionality
- **Better test readability**: Tests become more readable with macro syntax
- **Exception handling**: Tests that specifically test Symbol constructors are documented and kept as-is

## Mathematical Correctness

Test file migration should NOT affect mathematical correctness:
- Symbols created with macros are identical to those created with constructors
- Symbol equality and identity are unchanged
- Test assertions remain valid

However, verify:
- No test failures after migration
- Integration tests still exercise correct behavior
- Educational tests maintain their explanatory value

## Quality Standards

- Zero test regressions (ALL tests must pass)
- Zero build errors
- Improved test readability with macro syntax
- Document any tests kept with Symbol::new() (constructor-testing tests)

## Special Cases

### real_world_problems.rs (40 uses)
This file likely has many test cases with variable setup. Perfect candidate for bulk migration:

```rust
// Example transformation:
fn test_physics_problem() {
    // ❌ Before:
    let t = Symbol::new("t");
    let v0 = Symbol::new("v0");
    let a = Symbol::new("a");

    // ✅ After:
    let t = symbol!(t);
    let v0 = symbol!(v0);
    let a = symbol!(a);
}
```

### Constructor Tests (EXCEPTION)
If you find tests like:
```rust
#[test]
fn test_symbol_new() {
    let sym = Symbol::new("x");
    assert_eq!(sym.name(), "x");
}
```

**Keep as-is** and add comment:
```rust
#[test]
fn test_symbol_new() {
    // EXCEPTION: This test specifically tests Symbol::new() constructor
    let sym = Symbol::new("x");
    assert_eq!(sym.name(), "x");
}
```

## Deliverables

1. List of modified test files
2. Test suite verification output (cargo test --all)
3. Count reduction summary
4. Documentation of any exception cases (constructor-testing tests)

## Success Criteria

- ✅ Symbol::new() uses in test files reduced by ~50%
- ✅ ALL tests pass (cargo test --all)
- ✅ No clippy warnings introduced
- ✅ No runtime variables misused in macros
- ✅ Exception cases documented (if any)

**START with real_world_probalems.rs - it has 40 uses and will give you the biggest reduction!**
