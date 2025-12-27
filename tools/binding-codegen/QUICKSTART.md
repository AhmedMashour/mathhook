# binding-codegen Quick Start

**Last Updated:** 2025-12-27T1430

This guide covers the most common workflows for the binding-codegen tool.

## First Time Setup

1. **Install the tool** (already done if in workspace):
   ```bash
   cargo build -p binding-codegen
   ```

2. **Scan mathhook-core** to see what's available:
   ```bash
   cargo run -p binding-codegen -- scan
   ```

## Daily Workflows

### I Changed mathhook-core API

You modified/added/removed public functions in mathhook-core. Update bindings:

```bash
# Generate Python bindings
cargo run -p binding-codegen -- generate --target python

# Generate Node.js bindings
cargo run -p binding-codegen -- generate --target node

# Verify everything compiles
cargo check -p mathhook-python
cargo check -p mathhook-node
```

### I Want to Preview Changes

Before running full generation, preview what code would be generated:

```bash
# See what Expression bindings look like
cargo run -p binding-codegen -- debug --type-name Expression

# Check Symbol bindings
cargo run -p binding-codegen -- debug --type-name Symbol

# Inspect any other type
cargo run -p binding-codegen -- debug --type-name YourType
```

### I'm About to Commit

Check if bindings are up-to-date (CI does this automatically):

```bash
cargo run -p binding-codegen -- check
```

If stale, regenerate:
```bash
cargo run -p binding-codegen -- generate
cargo run -p binding-codegen -- generate --target node
```

### I Need to Understand Type Classification

See how the tool classifies types (primary vs helper):

```bash
cargo run -p binding-codegen -- analyze
```

This shows:
- Which types get wrapper classes (primary)
- Which types are used as helpers
- Method counts and dependencies

## Common Issues

### "Bindings are STALE"

**Problem:** mathhook-core changed but bindings weren't regenerated.

**Solution:**
```bash
cargo run -p binding-codegen -- generate
cargo run -p binding-codegen -- generate --target node
```

### Type Not Being Exported

**Problem:** You added a public type but it's not in generated bindings.

**Debug:**
```bash
# See if it's being discovered
cargo run -p binding-codegen -- scan

# Check classification
cargo run -p binding-codegen -- analyze

# Preview what would be generated
cargo run -p binding-codegen -- debug --type-name YourType
```

**Common causes:**
- Type is not `pub` (must be publicly visible)
- Type is in a private module (module path contains `_`, `internal`, `private`)
- Type is cfg-gated (`#[cfg(test)]`, `#[cfg(feature = "...")]`)
- Type has <5 methods and isn't used in other APIs (classified as helper)
- Type uses unsupported patterns (complex lifetimes, FnOnce, etc.)
- Type isn't registered in BindabilityRegistry

**Override:** Add to `binding-config.toml`:
```toml
[discovery]
force_include = ["YourType"]
```

**Note:** The tool uses generic detection from scanned metadata. If a type is being skipped unexpectedly, check its visibility, module path, and cfg attributes.

### Compilation Errors in Generated Code

**Problem:** Generated bindings don't compile.

**Debug:**
```bash
# Check what was generated
cargo run -p binding-codegen -- debug --type-name ProblematicType

# See full error
cargo check -p mathhook-python
cargo check -p mathhook-node
```

**Common causes:**
- Type uses patterns we don't support yet (report as issue)
- Type has conflicting method names (overload resolution may help)
- Return type isn't Clone/Send/Sync for FFI
- Method uses unsupported parameter types

**Resolution:** The tool automatically detects unbindable types through:
- BindabilityRegistry (tracks which types can be bound)
- Pattern analysis (MappedType detection)
- Trait analysis (checks for required traits)

If a type can't be bound, it should be detected automatically. Report issues if detection fails.

## File Locations

```
Generated bindings:
├── crates/mathhook-python/src/generated/
│   ├── mod.rs              # Exports
│   ├── expression.rs       # PyExpression
│   ├── symbol.rs           # PySymbol
│   ├── ...                 # ~240 primary + helper type files
│   └── manifest.json       # Metadata + hash
│
└── crates/mathhook-node/src/generated/
    ├── mod.rs              # Exports
    ├── expression.rs       # JsExpression
    ├── symbol.rs           # JsSymbol
    ├── ...                 # ~240 primary + helper type files
    └── manifest.json       # Metadata + hash
```

Configuration:
```
tools/binding-codegen/binding-config.toml  # Override defaults (optional)
```

## Command Summary

| Command | Purpose | Use When |
|---------|---------|----------|
| `scan` | Discover all public APIs | First time, debugging |
| `analyze` | Show type classification | Understanding decisions |
| `generate` | Create binding files | After API changes |
| `check` | Verify freshness | Before commit, in CI |
| `debug` | Preview type bindings | Testing changes |

## Next Steps

- **Full documentation:** See [README.md](README.md)
- **Implementation details:** See [IMPLEMENTATION.md](IMPLEMENTATION.md)
- **Change history:** See [CHANGELOG.md](CHANGELOG.md)
- **PyO3 docs:** https://pyo3.rs
- **NAPI-RS docs:** https://napi.rs
