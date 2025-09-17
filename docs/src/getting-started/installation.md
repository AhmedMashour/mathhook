# Installation

This guide covers installation of MathHook for Rust, Python, and Node.js.

## Rust

### Requirements

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Adding to Your Project

Add MathHook to your `Cargo.toml`:

```toml
[dependencies]
mathhook-core = "0.1"
```

For the high-level API with ergonomic macros:

```toml
[dependencies]
mathhook = "0.1"
```

### Verifying Installation

Create a simple test program:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

fn main() {
    let x = symbol!(x);
    let expr = expr!(x ^ 2);
    println!("Expression: {}", expr);
}
```

Run with:

```bash
cargo run
```

## Python

### Requirements

- Python 3.8 or higher
- pip

### Installing via pip

```bash
pip install mathhook
```

### Installing from Source

For the latest development version:

```bash
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook/crates/mathhook-python
pip install maturin
maturin develop
```

### Verifying Installation

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2)
print(f"Expression: {expr}")
```

### Virtual Environments

We recommend using a virtual environment:

```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install mathhook
```

## Node.js/TypeScript

### Requirements

- Node.js 18 or higher
- npm or yarn

### Installing via npm

```bash
npm install mathhook-node
```

Or with yarn:

```bash
yarn add mathhook-node
```

### Installing from Source

For the latest development version:

```bash
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook/crates/mathhook-node
npm install
npm run build
```

### Verifying Installation

Create a test file `test.ts`:

```typescript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2);
console.log(`Expression: ${expr.toString()}`);
```

Run with:

```bash
tsx test.ts
```

Or for JavaScript:

```javascript
const { Expression } = require('mathhook-node');

const x = Expression.symbol('x');
const expr = x.pow(2);
console.log(`Expression: ${expr.toString()}`);
```

## Building from Source

### Prerequisites

- Rust toolchain (rustup recommended)
- Git
- For Python bindings: Python 3.8+, maturin
- For Node.js bindings: Node.js 18+, npm

### Clone the Repository

```bash
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook
```

### Build the Core

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Build Python Bindings

```bash
cd crates/mathhook-python
pip install maturin
maturin develop --release
```

### Build Node.js Bindings

```bash
cd crates/mathhook-node
npm install
npm run build
```

## Platform-Specific Notes

### Windows

- Ensure you have Visual Studio Build Tools installed
- Python bindings require Microsoft C++ Build Tools

### macOS

- XCode Command Line Tools are required:
  ```bash
  xcode-select --install
  ```

### Linux

- GCC or Clang is required
- For Python bindings: `python3-dev` package
  ```bash
  # Ubuntu/Debian
  sudo apt-get install python3-dev

  # Fedora/RHEL
  sudo dnf install python3-devel
  ```

## Optional Dependencies

### SIMD Support

MathHook automatically detects and uses SIMD instructions (AVX2, SSE2) if available. No configuration needed.

To explicitly enable/disable:

```toml
[dependencies]
mathhook-core = { version = "0.1", features = ["simd"] }
```

### Parallel Processing

For parallel bulk operations:

```toml
[dependencies]
mathhook-core = { version = "0.1", features = ["parallel"] }
```

## Troubleshooting

### Rust: Compilation Errors

If you encounter LALRPOP-related errors:

```bash
cargo install lalrpop
cargo clean
cargo build
```

### Python: Import Errors

If `import mathhook` fails:

```bash
pip install --force-reinstall mathhook
```

### Node.js: Native Module Errors

If you see native module loading errors:

```bash
npm rebuild mathhook-node
```

### Permission Errors

On Linux/macOS, you may need to use `pip install --user`:

```bash
pip install --user mathhook
```

## Next Steps

Now that MathHook is installed, continue to:

- [Quick Start](./quick-start.md) - Your first 5 minutes with MathHook
- [Basic Usage](./basic-usage.md) - Learn the fundamentals
- [Common Patterns](./common-patterns.md) - Idioms and best practices
