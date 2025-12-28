# binding-codegen

**Last Updated:** 2025-12-27T1400

**Smart, fully automatic code generator for Python and Node.js bindings.**

## Philosophy

**Zero markers in mathhook-core.** This tool discovers everything from `pub` visibility alone. mathhook-core remains a pure math library with no binding concerns.

**Zero hardcoding.** All type/trait detection is generic, based on scanned metadata (visibility, cfg gates, trait implementations). No hardcoded skip lists or type name patterns.

## How It Works

```
┌─────────────────┐      ┌──────────────────┐      ┌─────────────────────┐
│ mathhook-core/  │ ──▶  │ binding-codegen  │ ──▶  │ COMPLETE .rs FILES  │
│ (untouched)     │ scan │ (auto-discovery) │ emit │ + .pyi / .d.ts      │
└─────────────────┘      └──────────────────┘      └─────────────────────┘
```

1. **Scan** - Walks `mathhook-core/src/**/*.rs`, finds all `pub` items
2. **Classify** - Determines primary types vs helpers using heuristics
3. **Analyze** - Detects patterns (Result, Iterator, Callback, etc.)
4. **Generate** - Emits **complete .rs files** (not fragments)
5. **Stubs** - Generates .pyi (Python) and .d.ts (Node.js) type definitions
6. **Manifest** - Creates `manifest.json` for incremental rebuilds and CI staleness detection

## Why Complete Files (Not Fragments)?

**Validated against PyO3 and NAPI-RS documentation:**

| Approach | PyO3 | NAPI-RS |
|----------|------|---------|
| `include!()` inside `#[pymethods]` | ❓ Untested, risky | ❌ **BROKEN** |
| Multiple `#[napi]` impl blocks | N/A | ❌ Name collision error |
| Single complete file | ✅ Works | ✅ Works |

We generate **complete .rs files** with all methods in one `#[pymethods]`/`#[napi]` block.

## Auto-Discovery Heuristics

A type becomes **Primary** (gets a wrapper class) if ANY of these are true:

| Criterion | Example |
|-----------|---------|
| Has >5 public methods | `Expression` (60+ methods) |
| Appears in public API signatures | `Symbol` used in `diff(var: Symbol)` |
| Is returned by a primary type | `Number` from `Expression::evaluate()` |
| Is a parameter of a primary type | `EvalContext` in `evaluate_with_context()` |
| Explicitly in config | `force_include` in `binding-config.toml` |

**No hardcoding required** - the tool discovers everything automatically.

## Type Skip Logic

Types are skipped (not generated) based on **scanned metadata only**:

| Criterion | Detection Method |
|-----------|------------------|
| Private type | `!type_info.is_public` |
| Cfg-gated (test/feature) | `type_info.is_cfg_gated` |
| Not bindable | `!bindability_registry.is_bindable_type()` |

No hardcoded type name lists. If a type needs special handling, it must be detected generically.

## Naming Convention

Automatic prefix-based naming:

```
Core Type      → Python         → Node.js
Expression     → PyExpression   → JsExpression
MathSolver     → PyMathSolver   → JsMathSolver
Symbol         → PySymbol       → JsSymbol
```

## Generated Output

### Python

```
mathhook-python/
├── src/generated/
│   ├── mod.rs              # Module exports
│   ├── expression.rs       # Complete PyExpression
│   ├── symbol.rs           # Complete PySymbol
│   ├── ...                 # ~240 primary + helper type files
│   └── manifest.json       # Generation metadata + hash
```

### Node.js

```
mathhook-node/
├── src/generated/
│   ├── mod.rs              # Module exports
│   ├── expression.rs       # Complete JsExpression
│   ├── symbol.rs           # Complete JsSymbol
│   ├── ...                 # ~240 primary + helper type files
│   └── manifest.json       # Generation metadata + hash
```

## Commands

### Scan
Discover all public types, functions, and methods:
```bash
cargo run -p binding-codegen -- scan
```

Output:
```
Building module visibility graph...
Found 85 public modules
Scanned 156 files (skipped 42 private, 8 test)
Also scanned 12 private files for impl blocks
Found 349 types, 311 functions, 520 impl blocks
```

### Analyze
Analyze API patterns and type mappings:
```bash
cargo run -p binding-codegen -- analyze
```

### Generate
Generate binding code for a target:
```bash
# Python bindings
cargo run -p binding-codegen -- generate --target python --output crates/mathhook-python/src/generated

# Node.js bindings
cargo run -p binding-codegen -- generate --target node --output crates/mathhook-node/src/generated

# Short form (uses default output paths)
cargo run -p binding-codegen -- --target python
cargo run -p binding-codegen -- --target node

# Dry run (preview without writing)
cargo run -p binding-codegen -- generate --dry-run
```

Output:
```
Generating node bindings...
Classification:
  Primary types: 129
  Helper types: 182
  Skipped types: 38
  Standalone functions: 311

  Generated: .../mathhook-node/src/generated/expression.rs
  Generated: .../mathhook-node/src/generated/symbol.rs
  ... (240 total files)

✅ Successfully generated 240 binding files
```

### Check
Verify bindings are up-to-date (for CI):
```bash
cargo run -p binding-codegen -- check
```

Exit codes:
- `0`: All bindings up-to-date
- `1`: Bindings stale or missing

### Debug
Inspect generated code for a specific type (without writing files):
```bash
# Default: show Expression bindings
cargo run -p binding-codegen -- debug

# Inspect a specific type
cargo run -p binding-codegen -- debug --type-name Symbol
cargo run -p binding-codegen -- debug --type-name Number
```

Use this to preview what code would be generated for a type before running `generate`.

## Type Stub Generation

### Python (.pyi)

Uses `pyo3-stub-gen` crate. Our codegen emits the right attributes:

```rust
use pyo3_stub_gen::derive::gen_stub_pymethods;

#[gen_stub_pymethods]
#[pymethods]
impl PyExpression {
    #[doc = "Expand the expression algebraically."]
    pub fn expand(&self) -> PyResult<Self> { ... }
}
```

### Node.js (.d.ts)

NAPI-RS generates automatically during `napi build`:

```bash
cd crates/mathhook-node
napi build
```

This creates `index.d.ts` with full TypeScript type definitions.

## Configuration (Edge Cases Only)

`binding-config.toml`:

```toml
[discovery]
force_include = ["RareButImportant"]

[naming.overrides]
"OldTypeName" = "BetterName"

[stubs]
python_stub_generator = "pyo3-stub-gen"
generate_py_typed = true

[docs]
python_docstring_style = "google"
generate_examples = true
```

**Note:** Do NOT use `skip_types` or `skip_patterns` - the tool uses generic detection from scanned metadata instead.

## Type Pattern Mapping

| Pattern | Detection | Generated Code |
|---------|-----------|----------------|
| **Direct** | `fn foo() -> T` | Direct return |
| **Result** | `fn foo() -> Result<T, E>` | Exception on Err |
| **Option** | `fn foo() -> Option<T>` | `None` / `null` |
| **Iterator** | `fn foo() -> impl Iterator` | `.collect::<Vec<_>>()` |
| **Callback** | `fn foo(f: impl Fn)` | Wrap PyFunction/JsFunction |
| **Reference** | `fn foo() -> &T` | Clone at boundary |
| **Unsupported** | `FnOnce`, complex lifetimes | Log warning, skip |

## Trait Detection

The tool automatically detects and generates bindings for:

| Trait | Python | Node.js |
|-------|--------|---------|
| `Display` | `__str__` | `toString` |
| `Clone` | `__copy__` | `clone_value` |
| `PartialEq` | `__eq__` | `equals` |
| `Default` | `#[new]` constructor | `#[napi(constructor)]` |
| Domain traits | Methods exposed | Methods exposed |

Trait import paths are resolved dynamically from scanned `TraitDefinition` data.

## Manifest Schema

The `manifest.json` tracks generation metadata for change detection:

```json
{
  "version": "1.0",
  "generated_at": "2025-12-27T14:00:00.000000+00:00",
  "core_hash": "27c556c0e4fef0d4f82fd14725edb11f...",
  "pattern_stats": {
    "direct_mappings": 240,
    "result_types": 45,
    "collected_iterators": 12,
    "callback_wrappers": 3,
    "cloned_refs": 8,
    "unsupported_skipped": 15
  },
  "types": {
    "Expression": {
      "core_type": "Expression",
      "python_wrapper": "PyExpression",
      "node_wrapper": "JsExpression",
      "method_count": 87
    }
  }
}
```

### Core Hash
SHA-256 hash of all `.rs` and `.lalrpop` files in `mathhook-core/src/`.
Used for staleness detection - bindings are considered stale if the hash doesn't match.

## CI Integration

Add to your CI workflow to ensure bindings stay fresh:

```yaml
- name: Check binding freshness
  run: cargo run -p binding-codegen -- check
```

This will fail the build if bindings are out of sync with mathhook-core.

## Architecture

```
src/
├── main.rs           # CLI interface (clap)
├── lib.rs            # Library exports
├── scanner.rs        # Scan mathhook-core, find pub items, build visibility graph
├── classifier.rs     # Primary vs helper type classification
├── analyzer.rs       # Pattern detection (MappedType), type mapping
├── types.rs          # Core types: MappedType, AnalyzedMethod, BindabilityRegistry
├── trait_analyzer.rs # Trait detection (SupportedTrait, DomainTraitMethod)
├── doc_transformer.rs # Doc comment transformation for PyO3/NAPI
├── manifest.rs       # manifest.json schema
├── config.rs         # binding-config.toml parsing
└── emitter/          # Code generation
    ├── mod.rs        # Emitter trait, overload resolution
    ├── python.rs     # PyO3 + pyo3-stub-gen
    └── node.rs       # NAPI-RS
```

## Development Workflow

1. **Modify mathhook-core API**
   ```bash
   vim crates/mathhook-core/src/...
   ```

2. **Regenerate bindings**
   ```bash
   cargo run -p binding-codegen -- --target python --output crates/mathhook-python/src/generated
   cargo run -p binding-codegen -- --target node --output crates/mathhook-node/src/generated
   ```

3. **Verify bindings**
   ```bash
   cargo check -p mathhook-python
   cargo check -p mathhook-node
   ```

4. **Run tests**
   ```bash
   cargo test -p binding-codegen
   ```

## Key Principles

> **mathhook-core stays 100% clean.**
>
> No `#[export]` attributes. No binding markers. No macros.
> Just normal Rust code with `pub` visibility.
>
> The codegen tool is smart enough to figure out what to bind.

> **Zero hardcoding in binding-codegen.**
>
> All skip/include decisions are based on scanned metadata:
> - Visibility (`pub` vs private)
> - Cfg gates (`#[cfg(test)]`, `#[cfg(feature)]`)
> - Trait implementations (Clone, Display, etc.)
> - Module path (public vs private modules)
>
> No hardcoded type name lists or pattern matching.

## Validation Status

Validated against official documentation:

- ✅ PyO3: Multiple `#[pymethods]` works with `multiple-pymethods` feature
- ✅ PyO3: Rust docs → Python `__doc__` via `#[doc = "..."]`
- ✅ PyO3: pyo3-stub-gen for .pyi generation
- ✅ NAPI-RS: Auto-generates .d.ts (built-in)
- ✅ NAPI-RS: Single `#[napi]` block per type (required)
- ✅ Rust 2018 module resolution (`foo.rs` → `foo/*.rs` submodules)
- ❌ `include!()` inside attr macros: Not viable

## Status

- [x] Wave 0: Framework validation
- [x] Wave 1: Scaffold complete
- [x] Wave 2a: Scanner (discovery engine with visibility graph)
- [x] Wave 2b: Classifier (primary/helper with dependency analysis)
- [x] Wave 2c: Pattern analyzer (MappedType detection)
- [x] Wave 2d: Code emitter (complete files with trait support)
- [x] Wave 3: Generate bindings (~240 files per target)
- [x] Manifest generation and CI check
- [x] Generic skip logic (no hardcoding)
- [x] Trait path resolution from scanned data
- [x] Overload resolution for method name conflicts
- [ ] Wave 4: Parity verification + full CI integration

## See Also

- [Quick Start Guide](QUICKSTART.md)
- [Implementation Notes](IMPLEMENTATION.md)
- [Changelog](CHANGELOG.md)
- [PyO3 Documentation](https://pyo3.rs)
- [NAPI-RS Documentation](https://napi.rs)
- [pyo3-stub-gen](https://github.com/PyO3/pyo3-stub-gen)
