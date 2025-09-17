# MathHook Documentation

This directory contains the MathHook book documentation built with mdbook.

## Building the Book

```bash
mdbook build
```

The generated book will be in `book/`.

## Testing Code Examples

The book contains Rust code examples that use mathhook. To test these examples:

```bash
./test.sh
```

This script:
1. Builds the `mathhook-book` crate (which provides mathhook dependencies)
2. Runs `mdbook test` with the correct library paths

### How It Works

- **Cargo.toml**: Defines dependencies on `mathhook` and `mathhook-core`
- **src-lib/lib.rs**: A minimal library that re-exports mathhook crates
- **Code Examples**: Use hidden lines (`# extern crate mathhook_book`) to access dependencies

### Writing Code Examples

When adding code examples to the book, use this pattern:

\`\`\`rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 2);
\`\`\`

Lines starting with `#` are hidden from the rendered book but included in tests.

**What users see:**
```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 2);
```

**What gets tested (with hidden lines):**
```rust
extern crate mathhook_book;  // Hidden - gives access to mathhook
use mathhook_book::mathhook;  // Hidden - brings mathhook into scope
use mathhook::prelude::*;     // Visible - what users should write

let x = symbol!(x);           // Visible - actual example code
let expr = expr!(x ^ 2);
```

## Serving Locally

```bash
mdbook serve
```

Then open http://localhost:3000

## Project Structure

- `src/`: Markdown source files
- `src-lib/`: Rust library for test dependencies
- `book/`: Generated HTML output (git-ignored)
- `book.toml`: mdbook configuration
- `Cargo.toml`: Dependencies for code examples
- `test.sh`: Convenient test runner
