# Wolfram Language Notation

> 📍 **You are here:** Parser > Wolfram Language Notation
>
> **Related Topics:** [LaTeX Notation](latex.md) | [Formatting](formatting.md) | [Custom Parsers](custom.md) | [Expressions](../core/expressions.md)
>
> **Skill Level:** ⭐⭐ Intermediate

Parse and generate Mathematica/Wolfram Language syntax for compatibility with Wolfram products.

## Quick Start (⭐ Start here)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;
// Parse Wolfram → Expression
let parsed = parse_wolfram("D[x^2, x]")?;  // Derivative

// Expression → Wolfram
let formatter = WolframFormatter::new();
let x = symbol!(x);
let expr = expr!(x^2);
let wolfram = formatter.format(&expr);  // Returns: x^2
```

## Understanding Wolfram Notation

### What is Wolfram Language?

Wolfram Language (used in Mathematica, Wolfram Alpha, Wolfram Cloud) is a symbolic computation language with:

- **Capital letter functions**: `Sin`, `Cos`, `Exp` (not `sin`, `cos`, `exp`)
- **Bracket notation**: Function calls use `[]` (e.g., `Sin[x]`, not `Sin(x)`)
- **Symbolic core**: Everything is an expression tree (similar to MathHook)
- **Pattern matching**: Powerful transformation rules (MathHook: simplification)

### Why Wolfram Compatibility?

- **Academic Migration**: Many researchers use Mathematica
- **Cross-Platform**: Export MathHook results to Wolfram Alpha
- **Data Exchange**: Import/export equations between systems
- **Validation**: Compare MathHook results with Mathematica

### How It Works (Architecture)

**Parser:**
- Recognizes Wolfram function tokens (`Sin`, `Cos`, `D`, `Integrate`)
- Bracket parsing: `f[x, y, z]` → Function call
- PascalCase → snake_case conversion for custom functions

**Formatter:**
- Lowercase → Capitalized function names (`sin` → `Sin`)
- Parentheses → Brackets (`(...)` → `[...]`)
- Operator precedence matching Wolfram

## Basic Usage (⭐)

### Parsing Wolfram Expressions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Basic arithmetic
let expr = parse_wolfram("2 + 3 * 4")?;  // Same precedence as Rust

// Functions (capital letters, brackets)
let expr = parse_wolfram("Sin[x]")?;
let expr = parse_wolfram("Exp[x]")?;
let expr = parse_wolfram("Log[x]")?;

// Powers
let expr = parse_wolfram("x^2")?;  // Uses ^ (not Power[])

// Lists (brackets with commas)
let expr = parse_wolfram("{1, 2, 3}")?;  // List notation
```

### Common Wolfram Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Trigonometric
let expr = parse_wolfram("Sin[x] + Cos[y]")?;

// Exponential and logarithmic
let expr = parse_wolfram("Exp[x] + Log[y]")?;

// Square root
let expr = parse_wolfram("Sqrt[x]")?;

// Absolute value
let expr = parse_wolfram("Abs[x]")?;

// Special functions
let expr = parse_wolfram("Gamma[n]")?;
```

### Calculus Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Derivative: D[expr, var]
let expr = parse_wolfram("D[x^2, x]")?;  // 2x

// Integral: Integrate[expr, var]
let expr = parse_wolfram("Integrate[x^2, x]")?;  // x^3/3

// Definite integral: Integrate[expr, {var, a, b}]
let expr = parse_wolfram("Integrate[x^2, {x, 0, 1}]")?;

// Limit: Limit[expr, var -> value]
let expr = parse_wolfram("Limit[Sin[x]/x, x -> 0]")?;

// Sum: Sum[expr, {i, start, end}]
let expr = parse_wolfram("Sum[i^2, {i, 1, n}]")?;
```

## Intermediate Usage (⭐⭐)

### Generating Wolfram Output

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

let formatter = WolframFormatter::new();
let x = symbol!(x);

// Simple expressions
let expr = expr!(x + 1);
println!("{}", formatter.format(&expr));  // x + 1

// Functions
let expr = expr!(sin(x));
println!("{}", formatter.format(&expr));  // Sin[x]

// Complex expressions
let expr = expr!((x + 1) / (x - 1));
println!("{}", formatter.format(&expr));  // (x + 1)/(x - 1)

// Derivatives
let df = expr!(x^2).derivative(&x, 1);
println!("{}", formatter.format(&df));    // 2*x
```

### Wolfram ↔ MathHook Translation Table

| **Operation** | **Wolfram** | **MathHook** | **Notes** |
|---------------|-------------|--------------|-----------|
| Addition | `a + b` | `a + b` | Same |
| Multiplication | `a * b` or `a b` | `a * b` | Same |
| Division | `a / b` | `a / b` | Same |
| Power | `a^b` | `a^b` | Same |
| Function call | `f[x]` | `f(x)` | Brackets vs parens |
| Derivative | `D[f, x]` | `f.derivative(&x, 1)` | Functional vs method |
| Integral | `Integrate[f, x]` | `f.integrate(&x)` | Functional vs method |
| Limit | `Limit[f, x -> a]` | Programmatic | Not yet parsed |
| Sum | `Sum[expr, {i, a, b}]` | `expr!(sum(expr, i, a, b))` | Function call |
| Product | `Product[expr, {i, a, b}]` | `expr!(product(expr, i, a, b))` | Function call |
| List | `{a, b, c}` | `expr!({a, b, c})` | Set notation |
| Sqrt | `Sqrt[x]` | `sqrt(x)` | Capital vs lowercase |
| Sin | `Sin[x]` | `sin(x)` | Capital vs lowercase |
| Cos | `Cos[x]` | `cos(x)` | Capital vs lowercase |
| Exp | `Exp[x]` | `exp(x)` | Capital vs lowercase |
| Log | `Log[x]` | `log(x)` | Capital vs lowercase |

### Matrix Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Determinant
let expr = parse_wolfram("Det[A]")?;

// Transpose
let expr = parse_wolfram("Transpose[A]")?;

// Inverse
let expr = parse_wolfram("Inverse[A]")?;

// Eigenvalues
let expr = parse_wolfram("Eigenvalues[A]")?;

// Matrix multiplication (same as regular multiplication)
let expr = parse_wolfram("A . B")?;  // Dot product
```

### Number Theory Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// GCD and LCM
let expr = parse_wolfram("GCD[12, 18]")?;
let expr = parse_wolfram("LCM[12, 18]")?;

// Factorial
let expr = parse_wolfram("Factorial[5]")?;  // or 5!

// Binomial coefficient
let expr = parse_wolfram("Binomial[n, k]")?;

// Prime counting function
let expr = parse_wolfram("PrimePi[100]")?;

// Euler phi function
let expr = parse_wolfram("EulerPhi[10]")?;

// Möbius function
let expr = parse_wolfram("MoebiusMu[n]")?;
```

### Polynomial Operations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Polynomial GCD
let expr = parse_wolfram("PolynomialGCD[x^2 - 1, x^2 - 2*x + 1]")?;

// Discriminant
let expr = parse_wolfram("Discriminant[a*x^2 + b*x + c, x]")?;

// Resultant
let expr = parse_wolfram("Resultant[p, q, x]")?;

// Cyclotomic polynomial
let expr = parse_wolfram("CyclotomicPolynomial[n, x]")?;

// Minimal polynomial
let expr = parse_wolfram("MinimalPolynomial[Sqrt[2], x]")?;

// Gröbner basis
let expr = parse_wolfram("GroebnerBasis[{x^2 + y^2 - 1, x - y}, {x, y}]")?;
```

## Advanced Usage (⭐⭐⭐)

### Custom Function Name Conversion

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// PascalCase → snake_case (automatic)
let expr = parse_wolfram("MyCustomFunction[x, y]")?;
// Parsed as: my_custom_function(x, y)

// Capital letters preserved for known functions
let expr = parse_wolfram("Sin[x]")?;  // Recognized as sin(x)

// Unknown functions converted
let expr = parse_wolfram("BesselJ[0, x]")?;  // bessel_j(0, x)
```

### Piecewise Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Piecewise notation
let expr = parse_wolfram(r"
    Piecewise[{
        {x, x >= 0},
        {-x, x < 0}
    }]
")?;

// Equivalent to: |x|
```

### Integration with Mathematica

**Exporting from MathHook to Mathematica:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// Generate Wolfram code
let formatter = WolframFormatter::new();
let wolfram_code = formatter.format(&expr);

println!("Export to Mathematica:");
println!("{}", wolfram_code);
// Output: x^2 + 2*x + 1

// Can copy-paste into Mathematica notebook
```

**Importing from Mathematica to MathHook:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Copy from Mathematica, paste as string
let mathematica_output = "Solve[x^2 - 5*x + 6 == 0, x]";

// Note: Solve returns rules, may need preprocessing
// Better approach: Export from Mathematica as expression string
let expr_str = "x^2 - 5*x + 6";
let expr = parse_wolfram(expr_str)?;

// Now solve in MathHook
let x = symbol!(x);
let solutions = MathSolver::new().solve(&expr, &x)?;
```

### Comparison with LaTeX Parsing

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Same mathematical concept, different syntax

// LaTeX
let latex_expr = parse_latex(r"\frac{x^2 + 1}{x - 1}")?;

// Wolfram
let wolfram_expr = parse_wolfram("(x^2 + 1)/(x - 1)")?;

// Both produce equivalent Expression trees
assert_eq!(latex_expr, wolfram_expr);
```

### Operator Precedence Comparison

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Wolfram precedence (same as Rust/MathHook)

// Exponentiation: Highest (right-associative)
parse_wolfram("2^3^4")?;  // 2^(3^4) = 2^81

// Multiplication and Division: Medium
parse_wolfram("2 * 3 / 4")?;  // (2 * 3) / 4 = 1.5

// Addition and Subtraction: Lowest
parse_wolfram("2 + 3 * 4")?;  // 2 + (3 * 4) = 14

// Same as MathHook and LaTeX parsers
```

## Real-World Applications

### 1. Migrating from Mathematica

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::MathSolver;

// Old Mathematica code
let mathematica_code = r#"
    Solve[x^2 - 5*x + 6 == 0, x]
"#;

// Extract equation part
let equation = parse_wolfram("x^2 - 5*x + 6")?;

// Solve in MathHook
let x = symbol!(x);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x)?;

println!("Solutions: {:?}", solutions);  // [2, 3]
```

### 2. Cross-Platform Validation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

// Compute derivative in MathHook
let x = symbol!(x);
let f = expr!(x^3 + 2*x^2 + x);
let df = f.derivative(&x, 1);

// Export to Wolfram for verification
let formatter = WolframFormatter::new();
let wolfram_code = formatter.format(&df);

println!("Verify in Wolfram Alpha:");
println!("Simplify[{}]", wolfram_code);
// Compare with Wolfram Alpha result
```

### 3. Batch Processing from Mathematica Notebooks

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::fs;

// Read Mathematica notebook export (plain text format)
let notebook_content = fs::read_to_string("mathematica_export.txt")?;

// Parse each line
for line in notebook_content.lines() {
    if let Ok(expr) = parse_wolfram(line) {
        println!("Parsed: {:?}", expr);
        // Process expression in MathHook
        let simplified = expr.simplify();
        println!("Simplified: {:?}", simplified);
    }
}
```

### 4. Wolfram Alpha Integration

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

// Generate query for Wolfram Alpha API
let x = symbol!(x);
let equation = expr!(x^2 - 4);

let formatter = WolframFormatter::new();
let query = format!("solve {} == 0", formatter.format(&equation));

println!("Wolfram Alpha query: {}", query);
// Use with Wolfram Alpha API or copy-paste to website
```

## Common Patterns (Cookbook)

### Parse and Solve Equations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::MathSolver;

let equation = parse_wolfram("x^2 - 5*x + 6")?;
let x = symbol!(x);

let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x)?;
println!("Solutions: {:?}", solutions);  // [2, 3]
```

### Convert Between Notations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use {WolframFormatter, LatexFormatter};

// Wolfram → MathHook → LaTeX
let wolfram_expr = parse_wolfram("Sin[x]^2 + Cos[x]^2")?;
let latex_formatter = LatexFormatter::new();
let latex = latex_formatter.format(&wolfram_expr);
println!("LaTeX: {}", latex);  // \sin(x)^{2} + \cos(x)^{2}

// LaTeX → MathHook → Wolfram
let latex_expr = parse_latex(r"\frac{x^2}{2}")?;
let wolfram_formatter = WolframFormatter::new();
let wolfram = wolfram_formatter.format(&latex_expr);
println!("Wolfram: {}", wolfram);  // x^2/2
```

### Export MathHook Results to Mathematica

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

let x = symbol!(x);
let f = expr!((x + 1)^2);
let expanded = f.expand();

let formatter = WolframFormatter::new();
println!("Copy to Mathematica:");
println!("{}", formatter.format(&expanded));
// x^2 + 2*x + 1
```

## Common Pitfalls

### ❌ WRONG → ✅ CORRECT

**Pitfall 1: Lowercase function names**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// ❌ WRONG: Wolfram requires capital letters
let expr = parse_wolfram("sin[x]")?;  // Error: unknown function

// ✅ CORRECT: Use capital letters
let expr = parse_wolfram("Sin[x]")?;  // Recognized as sin(x)
```

**Pitfall 2: Parentheses vs brackets**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// ❌ WRONG: Parentheses for function calls
let expr = parse_wolfram("Sin(x)")?;  // Parsed as Sin * (x)

// ✅ CORRECT: Brackets for function calls
let expr = parse_wolfram("Sin[x]")?;  // Function call
```

**Pitfall 3: List notation ambiguity**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// ⚠️ AMBIGUOUS: Single element in braces

// Wolfram: {x} is a list with one element
let expr = parse_wolfram("{x}")?;  // List/Set

// MathHook: {x} might be grouping (depends on context)
// Use explicit list constructor if needed
```

**Pitfall 4: Implicit multiplication**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Wolfram allows implicit multiplication
let expr = parse_wolfram("2 x")?;  // 2*x (space = multiplication)

// MathHook parser may or may not support this
// Prefer explicit: "2*x"
let expr = parse_wolfram("2*x")?;  // Explicit multiplication
```

**Pitfall 5: Derivative syntax differences**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Wolfram: D[f, x] (function call)
let wolfram_deriv = parse_wolfram("D[x^2, x]")?;

// MathHook: f.derivative(&x, 1) (method call)
let x = symbol!(x);
let mathhook_deriv = expr!(x^2).derivative(&x, 1);

// Both produce equivalent results: 2*x
```

**Pitfall 6: Rule syntax not supported**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ⚠️ LIMITATION: Wolfram rules not yet parsed

// Wolfram: x -> 2 (replacement rule)
// MathHook: Use substitute() method programmatically

let x = symbol!(x);
let expr = expr!(x + 1);
let result = expr.substitute(&x, &expr!(2));  // 3
```

**Pitfall 7: Equation vs expression**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Wolfram: Solve[x^2 == 4, x]
// Note: == is equality, not assignment

// MathHook parser:
let equation = parse_wolfram("x^2 == 4")?;  // Relation expression

// Or separate:
let lhs = parse_wolfram("x^2")?;
let rhs = parse_wolfram("4")?;
let equation = expr!(lhs == rhs);  // Build relation
```

## Performance Considerations

### Parser Performance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::wolfram::parse_wolfram;
// Fast: Simple expressions
parse_wolfram("x + y")?;  // ~10μs

// Moderate: Function calls
parse_wolfram("Sin[x] + Cos[y]")?;  // ~20μs

// Slow: Complex nested expressions
parse_wolfram("D[Sin[x^2], x]")?;  // ~50μs
```

### Formatter Performance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use WolframFormatter;

// Fast: Simple expressions
let expr = expr!(x + y);
WolframFormatter::new().format(&expr);  // ~1μs

// Moderate: Complex expressions
let expr = expr!(sin(x^2) + cos(y^2));
WolframFormatter::new().format(&expr);  // ~5μs
```

### Memory Efficiency

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Wolfram parser uses same expression representation as LaTeX
// No extra memory overhead for different notation styles
```

## See Also

- **[LaTeX Notation](latex.md)** - Standard academic mathematical notation
- **[Formatting](formatting.md)** - Output expressions in multiple formats
- **[Custom Parsers](custom.md)** - Extend parser for domain-specific syntax
- **[Expressions](../core/expressions.md)** - Core expression types
- **[Solving Equations](../operations/solving.md)** - Equation solving (migrating from Mathematica Solve)
- **[Calculus](../operations/calculus.md)** - Derivatives and integrals (D, Integrate in Wolfram)
