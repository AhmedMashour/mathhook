# Wave 9.1: Enhanced symbols!() Macro Syntax

**Goal**: Redesign symbols!() macro with Rust-idiomatic syntax and hybrid type specification

**Priority**: HIGH (API design quality)
**Effort**: 2-3 hours
**Impact**: Better Rust idioms, clearer intent, flexible type mixing

---

## Problem with Current Design (Wave 9)

### Current Syntax:
```rust
let syms = symbols!("x y z");              // String parsing (not idiomatic)
let matrices = symbols!("A B C"; matrix);  // Semicolon unclear
```

**Issues:**
1. ❌ String parsing at compile time (not Rust-idiomatic)
2. ❌ No syntax highlighting for symbol names
3. ❌ Semicolon suggests statement separator (confusing)
4. ❌ Type applies to all or none (no mixing)
5. ❌ Type scope unclear: does `matrix` apply to all?

---

## New Design: Hybrid Syntax (User's Excellent Idea!)

### Syntax Rules:

1. **Comma-separated identifiers** (Rust-idiomatic)
2. **Arrow `=>` for bulk type** (applies to all unmarked symbols)
3. **Colon `:` for per-symbol type** (overrides bulk type)
4. **Default to `scalar`** if no type specified

### Examples:

```rust
// All same type (simple cases):
let syms = symbols![x, y, z];                    // All scalars (default)
let matrices = symbols![A, B, C => matrix];      // All matrices
let operators = symbols![p, x, H => operator];   // All operators
let quats = symbols![i, j, k => quaternion];     // All quaternions

// Mixed types (hybrid - THE KEY FEATURE):
let mixed = symbols![
    x,              // Scalar (default)
    A: matrix,      // Explicitly matrix (overrides default)
    y,              // Scalar (default)
    p: operator,    // Explicitly operator (overrides default)
    z               // Scalar (default)
];

// Bulk type with overrides:
let mostly_matrices = symbols![
    A, B, C,        // Matrices (from bulk type)
    x: scalar,      // Override to scalar
    D, E            // Matrices (from bulk type)
    => matrix       // Bulk type for unmarked symbols
];

// Quantum mechanics example:
let quantum = symbols![
    x, y, z,        // Position operators
    E: scalar,      // Energy (scalar)
    p_x, p_y, p_z   // Momentum operators
    => operator     // Bulk type
];
```

### Type Precedence:

1. **Per-symbol type** (highest priority): `A: matrix`
2. **Bulk type**: `=> matrix`
3. **Default**: `scalar` (lowest priority)

---

## Implementation Strategy

### Macro Pattern Matching:

```rust
macro_rules! symbols {
    // Case 1: No types specified - all scalars
    ($($name:ident),+ $(,)?) => {{
        vec![$(Symbol::scalar(stringify!($name))),+]
    }};

    // Case 2: Bulk type only - all same type
    ($($name:ident),+ $(,)? => $type:ident) => {{
        vec![$(Symbol::$type(stringify!($name))),+]
    }};

    // Case 3: Mixed types (the complex case)
    // Parse each symbol, check if it has `: type`, otherwise use bulk or default
    (@parse_symbol $name:ident : $type:ident, $bulk:ident) => {
        Symbol::$type(stringify!($name))  // Per-symbol type wins
    };
    (@parse_symbol $name:ident, $bulk:ident) => {
        Symbol::$bulk(stringify!($name))  // Use bulk type
    };

    // Entry point for mixed syntax
    ($($tokens:tt)*) => {{
        // Parse token tree, identify bulk type, process each symbol
        // This is the complex case requiring careful pattern matching
    }};
}
```

**Challenge**: Declarative macros have limitations with this complexity. May need:
- Multiple helper patterns
- Token tree munching
- Or accept simpler version (no mixing in v1)

### Phased Implementation:

**Phase 1** (Simple - 1 hour):
- `symbols![x, y, z]` → all scalars
- `symbols![A, B, C => matrix]` → all same type
- NO mixing yet

**Phase 2** (Complex - 1-2 hours):
- `symbols![x, A: matrix, y]` → mixed types
- `symbols![A, B, x: scalar => matrix]` → bulk + overrides

---

## Changes Required

### 1. Update Macro Implementation
**File**: `crates/mathhook-core/src/macros/symbols.rs`
- Rewrite `symbols!()` macro (~100-150 lines)
- Keep backward compatibility helper (optional)

### 2. Update Tests
**File**: `crates/mathhook-core/tests/macro_enhancement_tests.rs`
- Update all 25 tests to new syntax
- Add 10+ new tests for hybrid syntax
- Test type precedence rules

### 3. Update CLAUDE.md
**File**: `CLAUDE.md`
- Update examples (lines 740-825, 1001-1014, 1170-1174)
- Show new syntax patterns
- Document type precedence rules
- Update "Golden Rule" examples

### 4. Update Documentation
**Files**: Various docstrings
- Macro documentation in `symbols.rs`
- Examples in all relevant modules

---

## Success Criteria

1. ✅ `symbols![x, y, z]` creates 3 scalars
2. ✅ `symbols![A, B, C => matrix]` creates 3 matrices
3. ✅ `symbols![p, x, H => operator]` creates 3 operators
4. ✅ `symbols![i, j, k => quaternion]` creates 3 quaternions
5. ✅ `symbols![x, A: matrix, y]` creates scalar, matrix, scalar (HYBRID)
6. ✅ `symbols![A, B, x: scalar => matrix]` bulk + override works
7. ✅ 35+ tests covering all syntax patterns (25 existing + 10 new)
8. ✅ CLAUDE.md updated with new examples
9. ✅ Build passes with 0 errors
10. ✅ Zero regressions

---

## Testing Strategy

### Test Categories (35+ tests):

**Basic Syntax (10 tests)**:
1. All scalars: `symbols![x, y, z]`
2. All matrices: `symbols![A, B, C => matrix]`
3. All operators: `symbols![p, x, H => operator]`
4. All quaternions: `symbols![i, j, k => quaternion]`
5. Single symbol: `symbols![x]`
6. Single with type: `symbols![A => matrix]`
7. Many symbols (10+): `symbols![a, b, c, d, e, f, g, h, i, j]`
8. Trailing comma: `symbols![x, y, z,]`
9. Type verification for each
10. Commutativity verification

**Hybrid Syntax (15 tests)**:
11. Override in middle: `symbols![x, A: matrix, y]`
12. Override at start: `symbols![A: matrix, x, y]`
13. Override at end: `symbols![x, y, A: matrix]`
14. Multiple overrides: `symbols![x, A: matrix, y, p: operator, z]`
15. Bulk + single override: `symbols![A, B, x: scalar => matrix]`
16. Bulk + multiple overrides: `symbols![A, x: scalar, B, y: scalar => matrix]`
17. All four types mixed: `symbols![x, A: matrix, p: operator, i: quaternion]`
18. Verify type precedence: per-symbol > bulk > default
19. Verify correct commutativity per symbol
20. Complex quantum example: `symbols![x, p: operator, E: scalar => operator]`
21-25. Edge cases and error conditions

**Backward Compatibility (10 tests)**:
26-35. Ensure existing functionality still works

---

## Verification Script

**File**: `.mathhook_sessions/verify_wave_9.1_syntax.sh`

**Categories**:
1. Macro syntax compiles
2. All 35+ tests pass
3. CLAUDE.md updated
4. No emojis
5. File sizes under 500 lines
6. Build passes
7. Zero regressions
8. Documentation quality

---

## Migration from Wave 9

### Option A: Breaking Change
- Remove old string syntax entirely
- Update all existing tests
- Clean break, better long-term

### Option B: Dual Support (Recommended for now)
- Keep old `symbols!("x y z")` for backward compatibility
- Add new `symbols![x, y, z]` syntax
- Deprecate old syntax with warning
- Remove in future version

**Recommendation**: Option B initially, then Option A after Wave 12 complete.

---

## Timeline

**Phase 1: Simple Syntax** (1 hour)
- Implement `symbols![x, y, z]` and `symbols![A, B => matrix]`
- Update 25 existing tests
- Update CLAUDE.md

**Phase 2: Hybrid Syntax** (1-2 hours)
- Implement per-symbol type overrides
- Add 10+ new tests for hybrid cases
- Document type precedence

**Phase 3: Verification** (30 min)
- Run all tests
- Update documentation
- Create verification report

**Total**: 2.5-3.5 hours

---

## Rationale: Why This Is Better

### Current (Wave 9):
```rust
let syms = symbols!("x y z");              // String parsing 😞
let matrices = symbols!("A B C"; matrix);  // Semicolon confusing 😕
// Can't mix types 😞
```

### New (Wave 9.1):
```rust
let syms = symbols![x, y, z];              // Rust-idiomatic ✅
let matrices = symbols![A, B, C => matrix]; // Arrow clear ✅
let mixed = symbols![x, A: matrix, y];     // Can mix types! ✅
```

**Benefits**:
1. ✅ Rust-idiomatic (comma-separated identifiers)
2. ✅ Syntax highlighting works
3. ✅ Clear type scope (arrow for bulk, colon for per-symbol)
4. ✅ Flexible (can mix types!)
5. ✅ Matches Rust patterns (`:` for types, `=>` for transformations)

---

## Agent Prompt Structure (When Ready)

**Agent 9.1A**: Macro Syntax Enhancement
- Rewrite `symbols!()` macro with new syntax
- Start with Phase 1 (simple), then Phase 2 (hybrid)
- Update 25 existing tests + add 10 new
- Update CLAUDE.md
- Target: 35+ tests, 9.5+/10 quality

---

**This Wave 9.1 will make MathHook's API much more Rust-idiomatic and flexible!**
