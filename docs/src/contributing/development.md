# Development Guide

Setup and workflow for MathHook development.

## Prerequisites

- **Rust 1.70+** (stable)
- **Git**
- **Python 3.8+** with SymPy (for validation)

## Quick Start

### Clone and Build

```bash
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook
cargo build --release
```

### Run Tests

```bash
# All tests (recommended)
gtimeout 600s cargo test --no-fail-fast

# Doctests only
gtimeout 600s cargo test --no-fail-fast --doc

# Specific crate
cargo test -p mathhook-core
```

### Run Benchmarks

```bash
./scripts/bench.sh run       # Full benchmarks
./scripts/bench.sh quick     # Quick run
./scripts/bench.sh save pre  # Save baseline before changes
./scripts/bench.sh compare pre  # Compare after changes
```

## Project Structure

```
mathhook/
├── crates/
│   ├── mathhook-core/       # Core mathematical engine
│   ├── mathhook-macros/     # Procedural macros (expr!, symbol!, etc.)
│   ├── mathhook/            # High-level user API
│   ├── mathhook-python/     # Python bindings (PyO3)
│   ├── mathhook-node/       # Node.js bindings (NAPI)
│   └── mathhook-benchmarks/ # Performance benchmarks
├── docs/                    # mdbook documentation
├── scripts/                 # Build and validation scripts
└── CLAUDE.md                # Development rules and standards
```

### Crate Responsibilities

| Crate | Purpose |
|-------|---------|
| `mathhook-core` | Expression types, parser, operations, functions |
| `mathhook-macros` | `symbol!`, `symbols!`, `function!`, `expr!` macros |
| `mathhook` | Re-exports, prelude, user-friendly API |
| `mathhook-python` | PyO3 bindings for Python |
| `mathhook-node` | NAPI bindings for Node.js |

## Development Workflow

### Before Starting

```bash
git status                    # Check clean state
git checkout -b feature/name  # Create feature branch
```

### Development Cycle

1. **Read [CONTRIBUTING.md](../../../CONTRIBUTING.md)** - Understand rules and priorities
2. **Plan the change** - Consider mathematical correctness first
3. **Implement** - Use macros, follow style guide
4. **Test** - Run `cargo test`, add new tests
5. **Validate** - Run `./scripts/validate.sh` for math operations
6. **Format** - Run `cargo fmt`
7. **Lint** - Run `cargo clippy -- -D warnings`

### Before Committing

```bash
cargo fmt                      # Format code
cargo clippy -- -D warnings    # Zero warnings
cargo test                     # All tests pass
```

## Key Commands

| Task | Command |
|------|---------|
| Build | `cargo build --release` |
| Test | `gtimeout 600s cargo test --no-fail-fast` |
| Doctest | `gtimeout 600s cargo test --no-fail-fast --doc` |
| Format | `cargo fmt` |
| Lint | `cargo clippy -- -D warnings` |
| Benchmark | `./scripts/bench.sh run` |
| Validate Math | `./scripts/validate.sh` |
| Build Docs | `cd docs && mdbook build` |
| Serve Docs | `cd docs && mdbook serve --open` |

## Documentation

### mdbook

```bash
cd docs
mdbook serve --open  # Preview at localhost:3000
mdbook build         # Build static site
```

### Rust Docs

```bash
cargo doc --open     # Generate and view API docs
```

## Validation

### SymPy Reference

Always verify mathematical operations against SymPy:

```python
from sympy import *
x = Symbol('x')
print(simplify(sin(x)**2 + cos(x)**2))  # Should be 1
```

### Validation Scripts

```bash
./scripts/validate.sh           # All validations
./scripts/validate.sh simplify  # Specific module
./scripts/validate.sh ode       # ODE solver
```

## Performance

### Benchmarking Workflow

```bash
# 1. Save baseline before changes
./scripts/bench.sh save before-change

# 2. Make your changes

# 3. Compare performance
./scripts/bench.sh compare before-change
```

### Size Constraints

| Type | Maximum Size |
|------|-------------|
| `Expression` | 32 bytes |
| `Number` | 16 bytes |
| Source file | 500 lines |

## Common Issues

### Parser Regeneration

```bash
# If grammar changes
lalrpop crates/mathhook-core/src/parser/grammar.lalrpop
```

### Macro Not Found

Ensure `mathhook-macros` is in dependencies and macros are re-exported.

### Test Timeout

Use `gtimeout` prefix for long-running tests:
```bash
gtimeout 600s cargo test --no-fail-fast
```

## Getting Help

1. **Read [CONTRIBUTING.md](../../../CONTRIBUTING.md)** - Contains all development rules
2. **Check existing code** - Follow established patterns
3. **Run tests** - They document expected behavior
4. **Validate against SymPy** - For mathematical correctness
