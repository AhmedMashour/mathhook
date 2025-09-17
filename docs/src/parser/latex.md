# LaTeX Notation

> 📍 **You are here:** Parser > LaTeX Notation
>
> **Related Topics:** [Formatting](formatting.md) | [Wolfram Notation](wolfram.md) | [Custom Parsers](custom.md) | [Expressions](../core/expressions.md)
>
> **Skill Level:** ⭐ Beginner

Parse and generate beautiful LaTeX notation for mathematical expressions.

## Quick Start (⭐ Start here)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::parser::{Parser, ParserConfig};
use mathhook::formatter::latex::LaTeXFormatter;

// Parse LaTeX → Expression
let parser = Parser::new(ParserConfig::default());
let parsed = parser.parse(r"\frac{x^2 + 1}{x - 1}")?;

// Expression → LaTeX (via LaTeXFormatter trait)
let x = symbol!(x);
let expr = expr!(x^2 / 2);
let latex = expr.to_latex(None)?;  // Returns: \frac{x^{2}}{2}
```

## Understanding LaTeX Parsing

### What is LaTeX Notation?

LaTeX is the standard mathematical typesetting language used in academic papers, textbooks, and presentations. MathHook provides:

- **Full bidirectional support**: Parse LaTeX → Expression, Expression → LaTeX
- **Automatic type inference**: `\mathbf{A}` creates matrix symbols, `\hat{p}` creates operator symbols
- **Implicit multiplication**: Handles `2x`, `\pi x`, `(a)(b)` correctly
- **Comprehensive coverage**: 150+ LaTeX commands for functions, symbols, operators, calculus

### How It Works (Architecture)

**Two-Stage Processing:**

1. **Lexer** (Token Generation):
   - Inserts implicit multiplication tokens (`2x` → `2*x`)
   - Classifies tokens (number, identifier, function, operator)
   - O(1) HashMap lookups for LaTeX commands (`\sin`, `\pi`, `\alpha`)

2. **Parser** (LALRPOP Grammar):
   - LR(1) parser with operator precedence
   - Right-associative exponentiation: `2^3^4` → `2^(3^4)`
   - Context-aware function resolution (indexed functions, calculus operators)

**Performance:**
- >100K simple expressions/second
- Thread-local caching for common expressions
- Zero-copy string processing where possible

## Basic Usage (⭐)

### Parsing LaTeX Expressions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Basic arithmetic
let expr = parse_latex(r"2 + 3 \cdot 4")?;  // 2 + 3*4 = 14

// Fractions
let expr = parse_latex(r"\frac{x^2 + 1}{x - 1}")?;

// Functions
let expr = parse_latex(r"\sin(x) + \cos(y)")?;

// Square roots
let expr = parse_latex(r"\sqrt{x^2 + y^2}")?;

// Exponents
let expr = parse_latex(r"e^{-x^2}")?;  // Gaussian
```

### Greek Letters and Constants

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Greek symbols (lowercase)
let expr = parse_latex(r"\alpha + \beta + \gamma")?;

// Greek symbols (uppercase, often functions)
let expr = parse_latex(r"\Gamma(n)")?;  // Gamma function

// Mathematical constants
let expr = parse_latex(r"\pi r^2")?;          // π*r²
let expr = parse_latex(r"e^{i\pi} + 1")?;     // Euler's identity
let expr = parse_latex(r"\phi = \frac{1+\sqrt{5}}{2}")?;  // Golden ratio
```

### Trigonometric Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Basic trig
let expr = parse_latex(r"\sin(x) + \cos(x)")?;

// Hyperbolic trig
let expr = parse_latex(r"\sinh(x) + \cosh(x)")?;

// Inverse trig (two notations)
let expr = parse_latex(r"\arcsin(x)")?;           // Explicit arcsin
let expr = parse_latex(r"\sin^{-1}(x)")?;         // Power notation
```

## Intermediate Usage (⭐⭐)

### Calculus Notation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Indefinite integral
let expr = parse_latex(r"\int x^2 \, dx")?;

// Definite integral
let expr = parse_latex(r"\int_0^{\infty} e^{-x} \, dx")?;

// Partial derivatives (parsed as regular fractions currently)
let expr = parse_latex(r"\frac{\partial f}{\partial x}")?;

// Summations
let expr = parse_latex(r"\sum_{i=1}^{n} i^2")?;

// Products
let expr = parse_latex(r"\prod_{i=1}^{n} i")?;  // n!

// Limits
let expr = parse_latex(r"\lim_{x \to 0} \frac{\sin(x)}{x}")?;
```

### Vector Calculus (Nabla Operators)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Gradient: ∇f
let expr = parse_latex(r"\nabla f")?;

// Divergence: ∇·F
let expr = parse_latex(r"\nabla \cdot \vec{F}")?;

// Curl: ∇×F
let expr = parse_latex(r"\nabla \times \vec{F}")?;

// Laplacian: ∇²f
let expr = parse_latex(r"\nabla^2 f")?;
```

### Noncommutative Symbols (Type Inference)

MathHook automatically infers symbol types from LaTeX notation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Matrix symbols (noncommutative): \mathbf{A}
let expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")?;
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
let expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")?;
// Creates operator symbols (quantum mechanics commutator)

// Scalar symbols (default, commutative)
let expr = parse_latex(r"x + y = y + x")?;
// Creates scalar symbols where order doesn't matter
```

**Why Type Inference Matters:**
- Matrix equations: `AX = B` has solution `X = A^(-1)B` (left division)
- Different from: `XA = B` with solution `X = BA^(-1)` (right division)
- Order matters for matrices: `A^(-1)B ≠ BA^(-1)` in general

### Indexed Functions (Special Functions)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Bessel functions: J_n(x)
let expr = parse_latex(r"J_0(x) + J_1(x)")?;

// Legendre polynomials: P_l(x)
let expr = parse_latex(r"P_2(x)")?;

// Associated Legendre: P_l^m(x)
let expr = parse_latex(r"P_2^1(x)")?;

// Hermite polynomials: H_n(x)
let expr = parse_latex(r"H_3(x)")?;

// Laguerre polynomials: L_n(x)
let expr = parse_latex(r"L_2(x)")?;
```

### Sets, Intervals, and Lists

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Sets (escaped braces): \{...\}
let expr = parse_latex(r"\{1, 2, 3, 5, 8\}")?;

// Intervals (closed)
let expr = parse_latex(r"[0, 1]")?;  // Closed interval

// Intervals (open)
let expr = parse_latex(r"(0, 1)")?;  // Open interval

// Half-open intervals
let expr = parse_latex(r"[0, 1)")?;  // Closed-open
let expr = parse_latex(r"(0, 1]")?;  // Open-closed
```

## Advanced Usage (⭐⭐⭐)

### Implicit Multiplication Rules

The lexer automatically inserts multiplication tokens:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Number × Identifier
let expr = parse_latex(r"2x")?;          // 2*x
let expr = parse_latex(r"3\pi")?;       // 3*π

// Identifier × Identifier
let expr = parse_latex(r"xy")?;         // x*y (split into x, y)

// Parenthesis patterns
let expr = parse_latex(r"2(x+1)")?;     // 2*(x+1)
let expr = parse_latex(r"(a)(b)")?;     // a*b

// Constant × Symbol
let expr = parse_latex(r"\pi r^2")?;    // π*r²

// Function calls (NO implicit multiplication)
let expr = parse_latex(r"\sin(x)")?;    // sin(x), NOT sin*x
```

**Performance:** O(1) token classification using precomputed HashMap lookups.

### Complete LaTeX Command Reference

**Trigonometric Functions:**
```
\sin, \cos, \tan, \sec, \csc, \cot
\sinh, \cosh, \tanh, \sech, \csch, \coth
\arcsin, \arccos, \arctan, \arcsec, \arccsc, \arccot
```

**Logarithmic Functions:**
```
\ln, \log, \log_10, \log_2
```

**Calculus Operators:**
```
\int, \iint, \iiint, \oint          # Integrals
\sum, \prod                          # Summation, product
\lim                                 # Limits
\partial                             # Partial derivatives
\nabla                               # Nabla operator
```

**Greek Letters (Lowercase):**
```
\alpha, \beta, \gamma, \delta, \epsilon, \zeta, \eta, \theta
\iota, \kappa, \lambda, \mu, \nu, \xi, \omicron, \pi, \rho
\sigma, \tau, \upsilon, \phi, \chi, \psi, \omega
```

**Greek Letters (Uppercase, often functions):**
```
\Gamma        # Gamma function
\Delta        # Dirac delta
\Psi          # Digamma function
\Zeta         # Riemann zeta function
```

**Mathematical Constants:**
```
\pi           # π ≈ 3.14159...
\phi, \varphi # Golden ratio φ ≈ 1.618...
\infty        # ∞
\EulerGamma   # Euler-Mascheroni constant γ ≈ 0.5772...
```

**Operators:**
```
\cdot         # Multiplication (·)
\times        # Cross product (×)
\div          # Division (÷)
\pm, \mp      # Plus-minus (±), minus-plus (∓)
\leq, \geq    # ≤, ≥
\neq          # ≠
\equiv        # ≡ (equivalence)
\approx       # ≈ (approximately)
\sim          # ~ (similar to)
\propto       # ∝ (proportional to)
```

**Set Theory:**
```
\in, \notin                # ∈, ∉
\subset, \supset           # ⊂, ⊃
\subseteq, \supseteq       # ⊆, ⊇
\cup, \cap                 # ∪, ∩
\emptyset                  # ∅
```

**Logic:**
```
\forall, \exists, \nexists # ∀, ∃, ∄
\land, \lor, \lnot         # ∧, ∨, ¬
\implies, \iff             # ⟹, ⟺
```

**Number Theory:**
```
\gcd, \lcm                 # Greatest common divisor, least common multiple
\binom{n}{k}               # Binomial coefficient
\phi(n)                    # Euler's totient function
\mu(n)                     # Möbius function
```

**Delimiters:**
```
\left( ... \right)         # Auto-sized parentheses
\left[ ... \right]         # Auto-sized brackets
\left\{ ... \right\}       # Auto-sized braces
\left| ... \right|         # Auto-sized absolute value
```

**Formatting:**
```
\mathbf{A}                 # Bold (inferred as matrix symbol)
\hat{p}                    # Hat (inferred as operator symbol)
\vec{v}                    # Vector arrow
\overline{z}               # Overline (often conjugate)
\tilde{x}                  # Tilde
\bar{x}                    # Bar
\text{if}                  # Text mode
\mathcal{F}                # Calligraphic (fancy fonts)
\mathbb{R}                 # Blackboard bold (ℝ, ℤ, ℚ)
```

### Custom Function Names with \text{}

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Special functions via \text{}
let expr = parse_latex(r"\text{erf}(x)")?;      // Error function
let expr = parse_latex(r"\text{Var}(X)")?;      // Variance
let expr = parse_latex(r"\text{Cov}(X, Y)")?;   // Covariance

// Custom identifiers
let expr = parse_latex(r"\text{cost}(n)")?;     // Cost function
```

### Matrix Notation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Matrix symbols (type inference)
let expr = parse_latex(r"\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}")?;

// Matrix transpose (method call)
let expr = parse_latex(r"\mathbf{A}^T")?;  // Parsed, but prefer explicit method

// Determinant
let expr = parse_latex(r"\det(\mathbf{A})")?;
```

### Piecewise Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Absolute value as piecewise
let expr = parse_latex(r"|x| = \begin{cases} x & x \geq 0 \\ -x & x < 0 \end{cases}")?;
```

## Real-World Applications

### 1. Academic Papers and Presentations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Parse equation from LaTeX document
let eq = parse_latex(r"\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}")?;

// Verify symbolically
let lhs = /* extract left side */;
let rhs = /* extract right side */;
assert!(lhs.simplify().equals(&rhs.simplify()));
```

### 2. Teaching Materials

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Student inputs LaTeX
let student_answer = parse_latex(r"\frac{d}{dx}(x^3) = 3x^2")?;

// Compare with correct answer
let x = symbol!(x);
let correct = expr!(x^3).derivative(&x, 1);
let expected = expr!(3 * x^2);

if correct.equals(&expected) {
    println!("Correct!");
}
```

### 3. Jupyter Notebook Integration

```python
# Python bindings
from mathhook import parse_latex, LatexFormatter

# Display equation
eq = parse_latex(r"\nabla \times \vec{E} = -\frac{\partial \vec{B}}{\partial t}")
display(Math(LatexFormatter().format(eq.simplify())))
```

### 4. Web Applications

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::parser::{Parser, ParserConfig};
use mathhook::formatter::latex::LaTeXFormatter;

// User input from web form
let user_input = r"\sin(x)^2 + \cos(x)^2";
let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(user_input)?;

// Simplify and render
let simplified = expr.simplify();
let output = simplified.to_latex(None)?;  // "1"

// Send back to frontend for MathJax rendering
```

## Common Patterns (Cookbook)

### Parse and Simplify

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = parse_latex(r"\sin(x)^2 + \cos(x)^2")?;
let simplified = expr.simplify();  // Returns: 1
```

### Solve Equations from LaTeX

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
use mathhook::{parser::parse_latex, MathSolver, symbol};

let equation = parse_latex(r"x^2 - 5x + 6 = 0")?;
let x = symbol!(x);

let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x)?;
// Returns: [2, 3]
```

### Extract Variables from LaTeX

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = parse_latex(r"x^2 + 2xy + y^2")?;
let vars = expr.variables();  // Returns: [x, y]
```

### Convert Between Notations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
use mathhook::{parser::parse_latex, formatter::WolframFormatter};

// LaTeX → Wolfram
let expr = parse_latex(r"\sin(x)^2")?;
let wolfram = WolframFormatter::new().format(&expr);  // "Sin[x]^2"
```

## Common Pitfalls

### ❌ WRONG → ✅ CORRECT

**Pitfall 1: Braces for grouping vs sets**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// ❌ WRONG: Regular braces create sets if multiple elements
let expr = parse_latex(r"{x, y}")?;  // Set {x, y}

// ✅ CORRECT: Use parentheses for grouping
let expr = parse_latex(r"(x + y)")?;

// ✅ CORRECT: Use escaped braces for sets explicitly
let expr = parse_latex(r"\{x, y\}")?;  // Set notation
```

**Pitfall 2: Implicit multiplication precedence**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// ❌ WRONG: Unclear precedence
let expr = parse_latex(r"2x^2")?;  // Parsed as 2*(x^2)

// ✅ CORRECT: Use explicit grouping when ambiguous
let expr = parse_latex(r"2 \cdot x^2")?;  // Clear multiplication
let expr = parse_latex(r"(2x)^2")?;       // If you meant (2x)²
```

**Pitfall 3: Function calls vs multiplication**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// ❌ WRONG: Confusing function with variable
let expr = parse_latex(r"f x")?;  // Parsed as f*x (multiplication)

// ✅ CORRECT: Always use parentheses for function calls
let expr = parse_latex(r"f(x)")?;  // Function call
```

**Pitfall 4: Derivative notation**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// ⚠️ LIMITATION: Full derivative notation not yet supported
// \frac{dy}{dx} parsed as regular fraction dy/dx

// ✅ WORKAROUND: Use explicit derivative function
let expr = parse_latex(r"\frac{d}{dx} f(x)")?;  // Limited support

// ✅ BETTER: Use programmatic API
let x = symbol!(x);
let f = expr!(x^2);
let df_dx = f.derivative(&x, 1);
```

**Pitfall 5: Matrix vs scalar operations**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// ❌ WRONG: Assuming commutative multiplication
let expr = parse_latex(r"\mathbf{A}\mathbf{B} = \mathbf{B}\mathbf{A}")?;
// This is FALSE for matrices!

// ✅ CORRECT: Respect noncommutativity
let expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")?;
// Solver handles left vs right division correctly
```

**Pitfall 6: Greek letters as functions**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Greek letters can be constants OR functions (context-dependent)

// Constant: \phi (golden ratio)
let expr = parse_latex(r"\phi \approx 1.618")?;

// Function: \phi(n) (Euler's totient function)
let expr = parse_latex(r"\phi(10) = 4")?;

// Parser uses context (presence of parentheses) to disambiguate
```

## Performance Considerations

### Parser Performance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Fast path: Simple expressions
let expr = parse_latex(r"x + y")?;  // ~10μs

// Moderate: Complex expressions
let expr = parse_latex(r"\int_0^{\infty} \frac{e^{-x}}{\sqrt{x}} dx")?;  // ~50μs

// Slow path: Very large expressions
let expr = parse_latex(r"x^{1000} + x^{999} + ... + x + 1")?;  // ~1ms
```

**Optimization Tips:**

1. **Cache parsed expressions** if reused frequently
2. **Batch parsing** when possible (parser initialization overhead)
3. **Avoid reparsing** - store `Expression` objects directly
4. **Use thread-local caching** for common subexpressions (automatic)

### Memory Usage

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Expression size: 32 bytes (cache-line optimized)
// Symbol size: String interning (O(1) equality comparison)
// Parser uses thread-local caches to minimize allocations
```

### Thread Safety

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# use mathhook::parser::latex::parse_latex;
// Parser is thread-safe (can parse in parallel)
use std::thread;

let handles: Vec<_> = (0..10)
    .map(|i| {
        thread::spawn(move || {
            parse_latex(&format!(r"x^{}", i))
        })
    })
    .collect();

for handle in handles {
    let expr = handle.join().unwrap()?;
    println!("{}", expr);
}
```

## See Also

- **[Formatting](formatting.md)** - Format expressions as LaTeX, Unicode, or Wolfram notation
- **[Wolfram Notation](wolfram.md)** - Parse and generate Mathematica/Wolfram Language syntax
- **[Custom Parsers](custom.md)** - Extend parser for domain-specific notation
- **[Expressions](../core/expressions.md)** - Core expression types and operations
- **[Noncommutative Algebra](../advanced/noncommutative-design.md)** - Matrix and operator symbols
- **[Educational Features](../educational/step-by-step.md)** - Generate step-by-step explanations from LaTeX
