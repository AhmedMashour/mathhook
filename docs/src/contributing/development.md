# Development Guide

For complete development documentation, please see [CLAUDE.md](../../CLAUDE.md).

## Quick Start

### Clone the Repository

```bash
git clone https://github.com/ahmedmashhour/mathhook.git
cd mathhook
```

### Build the Project

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Run Benchmarks

```bash
cargo bench
```

## Project Structure

```
mathhook/
├── crates/
│   ├── mathhook-core/      # Core mathematical engine
│   ├── mathhook/           # High-level API
│   ├── mathhook-python/    # Python bindings
│   ├── mathhook-node/      # Node.js bindings
│   └── mathhook-benchmarks/# Benchmarks
└── docs/                   # This documentation
```

For detailed architectural documentation, see [CLAUDE.md](../../CLAUDE.md).
