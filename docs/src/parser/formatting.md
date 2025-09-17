# Expression Formatting

> 📍 **You are here:** Parser > Expression Formatting
>
> **Related Topics:** [LaTeX Notation](latex.md) | [Wolfram Notation](wolfram.md) | [Custom Parsers](custom.md) | [Step-by-Step](../educational/step-by-step.md)
>
> **Skill Level:** ⭐ Beginner

Format mathematical expressions for display in multiple notations.

## Quick Start (⭐ Start here)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use {LatexFormatter, UnicodeFormatter, WolframFormatter};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// String (default Debug format)
println!("{:?}", expr);  // Add(...)

// LaTeX
let latex = LatexFormatter::new().format(&expr);
println!("{}", latex);   // x^{2} + 2 \cdot x + 1

// Unicode (pretty-print)
let unicode = UnicodeFormatter::new().format(&expr);
println!("{}", unicode); // x² + 2·x + 1

// Wolfram
let wolfram = WolframFormatter::new().format(&expr);
println!("{}", wolfram); // x^2 + 2*x + 1
```

## Understanding Expression Formatting

### What is Expression Formatting?

Expression formatting converts internal `Expression` structures to human-readable or machine-parseable strings. MathHook provides multiple output formats:

- **LaTeX**: Academic papers, presentations, web rendering (MathJax/KaTeX)
- **Unicode**: Pretty terminal output, notebooks, readable display
- **Wolfram**: Mathematica/Wolfram Language compatibility
- **String**: Rust Debug format for debugging

### How It Works (Architecture)

**Formatter Trait**:
```rust
pub trait Formatter {
    fn format(&self, expr: &Expression) -> String;
}
```

**Implementations**:
- `LatexFormatter` - Recursive descent with LaTeX command generation
- `UnicodeFormatter` - Unicode mathematical symbols (superscripts, subscripts)
- `WolframFormatter` - Wolfram Language bracket notation
- Type-aware formatting for noncommutative symbols

## Basic Usage (⭐)

### Default String Formatting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!(x + 1);

// Display trait (user-friendly)
println!("{}", expr);      // x + 1

// Debug trait (detailed structure)
println!("{:?}", expr);    // Add([Symbol("x"), Integer(1)])

// Pretty debug (indented structure)
println!("{:#?}", expr);   // Add([
                           //   Symbol("x"),
                           //   Integer(1)
                           // ])
```

### LaTeX Formatting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

let formatter = LatexFormatter::new();
let x = symbol!(x);

// Basic expressions
let expr = expr!(x^2);
println!("{}", formatter.format(&expr));  // x^{2}

// Fractions
let expr = expr!(x / 2);
println!("{}", formatter.format(&expr));  // \frac{x}{2}

// Functions
let expr = expr!(sin(x));
println!("{}", formatter.format(&expr));  // \sin(x)

// Greek letters
let alpha = symbol!(alpha);
let expr = expr!(alpha + 1);
println!("{}", formatter.format(&expr));  // \alpha + 1
```

### Unicode Formatting (Pretty-Print)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use UnicodeFormatter;

let formatter = UnicodeFormatter::new();
let x = symbol!(x);

// Superscripts for powers
let expr = expr!(x^2);
println!("{}", formatter.format(&expr));  // x²

// Multiplication dot
let expr = expr!(2*x);
println!("{}", formatter.format(&expr));  // 2·x

// Greek letters
let pi = symbol!(pi);
let expr = expr!(pi * r^2);
println!("{}", formatter.format(&expr));  // π·r²

// Square roots
let expr = expr!(sqrt(x));
println!("{}", formatter.format(&expr));  // √(x)
```

### Wolfram Formatting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

let formatter = WolframFormatter::new();
let x = symbol!(x);

// Functions use capital letters
let expr = expr!(sin(x));
println!("{}", formatter.format(&expr));  // Sin[x]

// Powers use ^ (not brackets)
let expr = expr!(x^2);
println!("{}", formatter.format(&expr));  // x^2

// Lists use brackets
let expr = expr!(f(a, b, c));
println!("{}", formatter.format(&expr));  // f[a, b, c]
```

## Intermediate Usage (⭐⭐)

### Customizing LaTeX Output

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

// Configure formatter
let formatter = LatexFormatter::new()
    .with_precision(6)           // Float precision
    .with_explicit_multiplication(true)  // Show all * as \cdot
    .with_compact_fractions(false);      // Use \frac always

let expr = expr!(2*x / 3);
println!("{}", formatter.format(&expr));  // \frac{2 \cdot x}{3}
```

### Type-Aware Formatting (Noncommutative Symbols)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

// Matrix symbols (bold)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let matrix_expr = expr!(A * B);

let formatter = LatexFormatter::new();
println!("{}", formatter.format(&matrix_expr));
// Output: \mathbf{A}\mathbf{B}

// Operator symbols (hat)
let p = symbol!(p; operator);
let x = symbol!(x; operator);
let op_expr = expr!(p * x);

println!("{}", formatter.format(&op_expr));
// Output: \hat{p}\hat{x}

// Scalar symbols (default)
let x_scalar = symbol!(x);
let y_scalar = symbol!(y);
let scalar_expr = expr!(x_scalar * y_scalar);

println!("{}", formatter.format(&scalar_expr));
// Output: x \cdot y
```

### Formatting Complex Expressions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use {LatexFormatter, UnicodeFormatter};

// Nested fractions
let x = symbol!(x);
let complex_expr = expr!((x + 1) / (x - 1));

let latex_fmt = LatexFormatter::new();
println!("{}", latex_fmt.format(&complex_expr));
// \frac{x + 1}{x - 1}

// Integrals with bounds
let integral = expr!(int(x^2, x, 0, 1));  // ∫₀¹ x² dx
println!("{}", latex_fmt.format(&integral));
// \int_{0}^{1} x^{2} \, dx

// Summations
let sum = expr!(sum(i, i, 1, n));  // Σᵢ₌₁ⁿ i
println!("{}", latex_fmt.format(&sum));
// \sum_{i=1}^{n} i
```

### Format Comparison Table

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use {LatexFormatter, UnicodeFormatter, WolframFormatter};

let x = symbol!(x);
let expr = expr!(x^2 + sqrt(2)*x + 1);

// Side-by-side comparison
let latex = LatexFormatter::new().format(&expr);
let unicode = UnicodeFormatter::new().format(&expr);
let wolfram = WolframFormatter::new().format(&expr);

println!("LaTeX:   {}", latex);   // x^{2} + \sqrt{2} \cdot x + 1
println!("Unicode: {}", unicode); // x² + √2·x + 1
println!("Wolfram: {}", wolfram); // x^2 + Sqrt[2]*x + 1
```

## Advanced Usage (⭐⭐⭐)

### Custom Formatter Implementation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;
use Formatter;

struct CustomFormatter {
    indent_level: usize,
}

impl Formatter for CustomFormatter {
    fn format(&self, expr: &Expression) -> String {
        match expr {
            Expression::Add(terms) => {
                let formatted: Vec<_> = terms.iter()
                    .map(|t| self.format(t))
                    .collect();
                format!("(+ {})", formatted.join(" "))
            }
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Number(n) => format!("{}", n),
            // ... handle other variants
            _ => format!("{:?}", expr),
        }
    }
}

// Use custom formatter
let formatter = CustomFormatter { indent_level: 0 };
let expr = expr!(x + 1);
println!("{}", formatter.format(&expr));  // (+ x 1)
```

### Precision Control for Numerical Output

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

let x = symbol!(x);

// Low precision (default: 6 digits)
let expr = expr!(pi * r^2);
let formatter = LatexFormatter::new().with_precision(2);
println!("{}", formatter.format(&expr));  // 3.14 \cdot r^{2}

// High precision
let formatter = LatexFormatter::new().with_precision(15);
println!("{}", formatter.format(&expr));  // 3.141592653589793 \cdot r^{2}

// Exact symbolic (no numerical evaluation)
let formatter = LatexFormatter::new().with_exact_mode(true);
println!("{}", formatter.format(&expr));  // \pi \cdot r^{2}
```

### Conditional Formatting Based on Context

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

// Academic paper mode (verbose)
let academic_formatter = LatexFormatter::new()
    .with_explicit_multiplication(true)  // Show all \cdot
    .with_function_parentheses(true)     // Always show (x)
    .with_expanded_fractions(true);      // Always use \frac

// Compact mode (terse)
let compact_formatter = LatexFormatter::new()
    .with_implicit_multiplication(true)  // Hide obvious *
    .with_inline_fractions(true);        // Use / when possible

let x = symbol!(x);
let expr = expr!(2*x / 3);

println!("Academic: {}", academic_formatter.format(&expr));
// \frac{2 \cdot x}{3}

println!("Compact:  {}", compact_formatter.format(&expr));
// 2x/3
```

### Formatting for Specific Backends

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// MathJax/KaTeX (web rendering)
let mathjax_formatter = LatexFormatter::new()
    .with_displaystyle(true)        // Use \displaystyle for integrals/sums
    .with_align_environment(false); // Single-line equations

// Beamer presentations (LaTeX slides)
let beamer_formatter = LatexFormatter::new()
    .with_color_support(true)       // Can use \color{}
    .with_large_operators(true);    // Bigger \sum, \int

// Academic papers (LaTeX documents)
let paper_formatter = LatexFormatter::new()
    .with_equation_numbers(true)    // Support \label{}
    .with_theorem_style(true);      // Support theorem environments
```

### Unicode Mathematical Symbols

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use UnicodeFormatter;

// Supported Unicode ranges:
// - Superscripts: ⁰¹²³⁴⁵⁶⁷⁸⁹
// - Subscripts: ₀₁₂₃₄₅₆₇₈₉
// - Greek: α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω
// - Operators: · × ÷ ± ∓ ≤ ≥ ≠ ≈ ∞
// - Roots: √ ∛ ∜
// - Set theory: ∈ ∉ ⊂ ⊃ ⊆ ⊇ ∪ ∩ ∅

let formatter = UnicodeFormatter::new();

let expr = expr!(x^2 + sqrt(3)*x + 1);
println!("{}", formatter.format(&expr));  // x² + √3·x + 1

let expr = expr!(alpha <= beta);
println!("{}", formatter.format(&expr));  // α ≤ β
```

## Real-World Applications

### 1. Academic Paper Generation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

// Generate LaTeX for paper
let x = symbol!(x);
let theorem_eq = expr!(x^2 + 1 >= 2*x);

let formatter = LatexFormatter::new();
let latex = formatter.format(&theorem_eq);

// Output to LaTeX document
println!(r#"
\begin{{theorem}}
For all $x \in \mathbb{{R}}$, we have:
\[
    {}
\]
\end{{theorem}}
"#, latex);
```

### 2. Interactive Console Display

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use UnicodeFormatter;

// REPL-style output
let x = symbol!(x);
let expr = expr!(x^3 - 3*x^2 + 3*x - 1);

let formatter = UnicodeFormatter::new();
println!("Result: {}", formatter.format(&expr));
// Result: x³ - 3·x² + 3·x - 1
```

### 3. Web Application Backend

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;
use serde_json::json;

// API endpoint: Simplify and return LaTeX
let x = symbol!(x);
let user_expr = expr!((x + 1)^2);
let simplified = user_expr.expand();

let formatter = LatexFormatter::new();

let response = json!({
    "original": formatter.format(&user_expr),
    "simplified": formatter.format(&simplified),
});

// Send JSON to frontend for MathJax rendering
```

### 4. Jupyter Notebook Integration

```python
# Python bindings with formatters
from mathhook import expr, symbol, LatexFormatter
from IPython.display import display, Math

x = symbol('x')
equation = (x**2 + 1) / (x - 1)

# Display with LaTeX
formatter = LatexFormatter()
latex_str = formatter.format(equation)
display(Math(latex_str))
```

## Common Patterns (Cookbook)

### Format for Different Contexts

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use {LatexFormatter, UnicodeFormatter, WolframFormatter};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// Console display (Unicode)
println!("Console: {}", UnicodeFormatter::new().format(&expr));

// Web rendering (LaTeX)
let latex = LatexFormatter::new().format(&expr);
println!("Web: <span class='math'>{}</span>", latex);

// Mathematica export (Wolfram)
let wolfram = WolframFormatter::new().format(&expr);
println!("Export to Mathematica: {}", wolfram);
```

### Format Educational Steps

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// Generate step-by-step LaTeX
let formatter = LatexFormatter::new();

println!("Step 1: Start with {}", formatter.format(&expr));
let factored = expr.factor();  // (x+1)^2
println!("Step 2: Factor as {}", formatter.format(&factored));
```

### Batch Formatting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

let x = symbol!(x);
let expressions = vec![
    expr!(x + 1),
    expr!(x^2),
    expr!(sin(x)),
];

let formatter = LatexFormatter::new();
for (i, expr) in expressions.iter().enumerate() {
    println!("Equation {}: {}", i+1, formatter.format(expr));
}
```

## Common Pitfalls

### ❌ WRONG → ✅ CORRECT

**Pitfall 1: Unicode rendering issues**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Not all terminals support Unicode math
let formatter = UnicodeFormatter::new();
let expr = expr!(x^2);
println!("{}", formatter.format(&expr));  // May show: x�� (broken)

// ✅ CORRECT: Check terminal capabilities or fallback
use std::io::{self, IsTerminal};

let formatter = if io::stdout().is_terminal() {
    Box::new(UnicodeFormatter::new()) as Box<dyn Formatter>
} else {
    Box::new(LatexFormatter::new())
};
```

**Pitfall 2: LaTeX escaping in strings**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: LaTeX commands lost in regular strings
let latex = LatexFormatter::new().format(&expr);
println!("Equation: {}", latex);  // Missing $ delimiters

// ✅ CORRECT: Proper LaTeX embedding
println!("Equation: ${}$", latex);  // Inline math
println!("Display: \\[{}\\]", latex);  // Display math
```

**Pitfall 3: Precision loss in numerical formatting**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use LatexFormatter;

// ❌ WRONG: Default precision too low
let formatter = LatexFormatter::new();  // Default: 6 digits
let expr = expr!(pi * 10^15);
println!("{}", formatter.format(&expr));  // 3.14159e15 (imprecise)

// ✅ CORRECT: Set appropriate precision
let formatter = LatexFormatter::new().with_precision(15);
println!("{}", formatter.format(&expr));  // 3141592653589793
```

**Pitfall 4: Missing parentheses in complex expressions**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Ambiguous precedence in output
let expr = expr!(x + y * z);
let formatter = LatexFormatter::new().with_minimal_parens(true);
println!("{}", formatter.format(&expr));  // x + y \cdot z (OK)

let expr = expr!((x + y) * z);
println!("{}", formatter.format(&expr));  // x + y \cdot z (WRONG!)

// ✅ CORRECT: Preserve necessary parentheses
let formatter = LatexFormatter::new().with_safe_parens(true);
println!("{}", formatter.format(&expr));  // (x + y) \cdot z
```

**Pitfall 5: Symbol type not reflected in output**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ⚠️ Noncommutative symbols need type-aware formatting

// Matrix symbols should be bold
let A = symbol!(A; matrix);
let formatter = LatexFormatter::new();
println!("{}", formatter.format(&expr!(A)));  // \mathbf{A} ✓

// Operator symbols should have hat
let p = symbol!(p; operator);
println!("{}", formatter.format(&expr!(p)));  // \hat{p} ✓

// Verify formatter supports type-aware output
```

**Pitfall 6: Wolfram function name capitalization**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Lowercase function names
let expr = expr!(sin(x));
let formatter = WolframFormatter::new();
println!("{}", formatter.format(&expr));  // Sin[x] (not sin[x])

// ✅ CORRECT: Wolfram uses capital letters
// Formatter automatically capitalizes: sin → Sin, cos → Cos, etc.
```

## Performance Considerations

### Formatting Performance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Fast: Simple expressions
let expr = expr!(x + y);
let latex = LatexFormatter::new().format(&expr);  // ~1μs

// Moderate: Complex expressions
let expr = expr!((x + y)^10);
let latex = LatexFormatter::new().format(&expr);  // ~50μs

// Slow: Very large expressions
let expr = expr!(x^1000);
let latex = LatexFormatter::new().format(&expr);  // ~1ms
```

### Memory Efficiency

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Formatters are lightweight (no heap allocations)
let formatter = LatexFormatter::new();  // Stack-allocated

// Reuse formatters for multiple expressions
for expr in expressions {
    let latex = formatter.format(&expr);  // No reallocations
}
```

### String Allocation Optimization

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Preallocate string capacity for large expressions
let mut buffer = String::with_capacity(1024);
let formatter = LatexFormatter::new();
buffer.push_str(&formatter.format(&expr));
```

## See Also

- **[LaTeX Notation](latex.md)** - Parse LaTeX input and generate LaTeX output
- **[Wolfram Notation](wolfram.md)** - Mathematica/Wolfram Language compatibility
- **[Custom Parsers](custom.md)** - Extend formatters for domain-specific notation
- **[Expressions](../core/expressions.md)** - Core expression types being formatted
- **[Educational Features](../educational/step-by-step.md)** - Format step-by-step explanations
- **[Noncommutative Algebra](../advanced/noncommutative-design.md)** - Type-aware formatting for matrices and operators
