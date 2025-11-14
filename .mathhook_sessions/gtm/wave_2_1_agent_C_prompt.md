# Agent 2.1C: Core/Macro/Bench Symbol Migration

## Mission
Migrate Symbol::new() uses in core, macro, benchmark, and archive files to the symbol!() macro. This agent handles **the remaining ~50+ uses** spread across support files.

## Context
- **Wave**: 2.1 - Symbol Constructor Migration Part 1
- **Target**: Reduce 227 → 114 Symbol::new() uses (50% reduction)
- **Agent Scope**: Core, macros, benchmarks, archive (~50+ uses)
- **Priority**: MANDATORY (CLAUDE.md Priority 1)

## Files Assigned
Primary focus:
- `crates/mathhook-core/src/macros/calculus.rs` (7 uses)
- `crates/mathhook-core/src/core/symbol.rs` (5 uses)
- `crates/mathhook-benchmarks/tests/gcd_symbolica_cases.rs` (5 uses)
- `benches/optimization_bench.rs` (5 uses)
- `crates/mathhook-core/src/serialize.rs` (4 uses)
- `crates/mathhook-core/src/macros/simplification.rs` (4 uses)
- `crates/mathhook-benchmarks/benches/realistic_cas_benchmarks.rs` (4 uses)
- `benches/symbolica_challenge_bench.rs` (4 uses)
- `crates/mathhook-benchmarks/src/lib.rs` (3 uses)
- Archive files (7-6 uses each)

## Critical Constraints

### CLAUDE.md Compliance
**PRIORITY 1 (MANDATORY)**: Symbol Creation Rules
- ❌ **NEVER** use `Symbol::new()` in application code
- ✅ **ALWAYS** use `symbol!(x)` macro for scalar symbols
- ✅ Use `symbol!(A; matrix)` for matrix symbols
- ✅ Use `symbol!(p; operator)` for operator symbols

**EXCEPTIONS** (Symbol::new() is ONLY allowed in):
1. **Macro implementations** (`macros/expressions.rs`) - legitimate use
2. **Test code that specifically tests Symbol constructors**
3. **core/symbol.rs**: If testing Symbol methods, may keep some uses

### Special File Considerations

#### 1. core/symbol.rs (5 uses)
This is the **Symbol implementation file**. Check each use:
- If testing Symbol methods → Keep as-is, add exception comment
- If example code or documentation → Migrate to macro
- If internal implementation → Keep if necessary, document reason

#### 2. macros/calculus.rs & macros/simplification.rs (7+4 uses)
These implement **declarative macros**. Check if Symbol::new() is:
- Part of macro expansion → May keep, document
- In macro implementation helper functions → Migrate if possible
- In macro test code → Migrate

#### 3. Benchmark files (5+4+4+3 uses)
Benchmarks use runtime data for performance testing. Check if:
- Literal symbol names (e.g., "x", "y") → Migrate to macro
- Runtime variable names from loop/data → Keep Symbol::new()

#### 4. serialize.rs (4 uses)
Serialization may use Symbol::new() for deserialization. Check if:
- Deserializing from data → Keep Symbol::new() (runtime strings)
- Test/example code → Migrate to macro

#### 5. Archive files
Archive files are old code kept for reference. **Lower priority** - migrate if easy, otherwise document and skip.

## Migration Patterns

### Pattern 1: Macro Implementation Files
```rust
// In macros/calculus.rs or macros/simplification.rs

// ❌ If this is in macro expansion:
macro_rules! some_macro {
    ($var:ident) => {
        Symbol::new(stringify!($var))  // May keep - inside macro expansion
    };
}

// ✅ If this is in helper functions:
fn create_test_symbol() -> Symbol {
    symbol!(x)  // Should migrate
}
```

### Pattern 2: Benchmark Files
```rust
// ❌ Before (literal names):
fn bench_something(c: &mut Criterion) {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    // benchmark code...
}

// ✅ After:
fn bench_something(c: &mut Criterion) {
    let x = symbol!(x);
    let y = symbol!(y);
    // benchmark code...
}

// ✅ KEEP (runtime data):
for var_name in benchmark_data.variables {
    let sym = Symbol::new(var_name);  // Runtime - keep as-is
}
```

### Pattern 3: core/symbol.rs Methods
```rust
// In Symbol implementation

// ✅ KEEP (testing internal methods):
#[test]
fn test_symbol_equality() {
    // Testing Symbol::new() behavior - exception
    let s1 = Symbol::new("x");
    let s2 = Symbol::new("x");
    assert_eq!(s1, s2);
}

// ✅ MIGRATE (examples in docs):
/// Example usage:
/// ```rust
/// let x = symbol!(x);  // Migrate doc examples
/// ```
```

### Pattern 4: Serialization
```rust
// In serialize.rs

// ✅ KEEP (deserializing runtime data):
impl Deserialize for Expression {
    fn deserialize() -> Self {
        let name = read_string();
        Expression::Symbol(Symbol::new(name))  // Runtime - keep
    }
}

// ✅ MIGRATE (test code):
#[test]
fn test_serialize() {
    let x = symbol!(x);  // Migrate
    let serialized = serialize(&x);
    //...
}
```

## Verification Steps

After migration, verify:

1. **Count Check**:
   ```bash
   # Check reduction in each category
   rg "Symbol::new\(" crates/mathhook-core/src/macros/ -c | awk -F: '{sum+=$2} END {print sum}'
   rg "Symbol::new\(" crates/mathhook-core/src/core/symbol.rs -c
   rg "Symbol::new\(" crates/mathhook-benchmarks/ -c | awk -F: '{sum+=$2} END {print sum}'
   ```

2. **Build Check**:
   ```bash
   cargo build --all
   ```

3. **Test Check**:
   ```bash
   cargo test -p mathhook-core
   cargo test -p mathhook-benchmarks
   ```

4. **Benchmark Check**:
   ```bash
   cargo bench --no-run  # Verify benchmarks compile
   ```

## Expected Outcomes

- **Symbol::new() uses**: Reduce from ~50 to ~25 in assigned files
- **All tests pass**: No regressions
- **Benchmarks compile**: No breaking changes
- **Exceptions documented**: Clear comments for legitimate Symbol::new() uses

## Exception Documentation

For any Symbol::new() uses you keep, add a comment:

```rust
// EXCEPTION: [reason]
// - Macro expansion / Internal testing / Runtime deserialization
let sym = Symbol::new(var_name);
```

## Mathematical Correctness

Core and macro files are critical for mathematical correctness. Verify:
- Macro expansions still produce correct symbols
- Symbol identity and equality unchanged
- Serialization/deserialization preserves symbol semantics

## Quality Standards

- Zero test regressions
- Zero build errors
- All exceptions documented with clear reasons
- No performance degradation in benchmarks

## Deliverables

1. List of modified files by category (core/macro/bench/archive)
2. Count reduction per category
3. List of exceptions with documented reasons
4. Verification command outputs

## Success Criteria

- ✅ Symbol::new() uses reduced by ~50% in assigned files
- ✅ All tests pass (cargo test --all)
- ✅ Benchmarks compile (cargo bench --no-run)
- ✅ Exceptions properly documented
- ✅ No regressions in mathematical correctness

**START with macro files (calculus.rs, simplification.rs) - they're highest priority for CLAUDE.md compliance!**
