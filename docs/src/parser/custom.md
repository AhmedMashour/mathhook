# Custom Parsers and Extensions

> 📍 **You are here:** Parser > Custom Parsers
>
> **Related Topics:** [LaTeX Notation](latex.md) | [Wolfram Notation](wolfram.md) | [Formatting](formatting.md) | [Functions](../operations/functions.md)
>
> **Skill Level:** ⭐⭐⭐ Advanced

Extend MathHook's parser for domain-specific mathematical notation.

## Quick Start (⭐ Start here)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Add custom function recognition
let parser = ParserBuilder::new()
    .add_function("erf", "error_function")   // erf(x) → error_function(x)
    .add_operator("×", "*")                  // × is multiplication
    .build();

let result = parser.parse("erf(x) × 2")?;
// Parsed as: error_function(x) * 2
```

## Understanding Parser Extension

### What Can You Extend?

MathHook's parser is modular and extensible. You can add:

- **Custom Functions**: Domain-specific functions (chemistry, physics, engineering)
- **Custom Operators**: New infix/prefix/postfix operators
- **Custom Notation**: LaTeX macros, specialized symbols
- **Parser Preprocessors**: Transform input before parsing
- **Lexer Tokens**: New token types for specialized syntax

### When to Extend the Parser

**Use Built-In Features When:**
- Standard mathematical notation suffices
- Functions can be named conventionally
- LaTeX or Wolfram notation covers your needs

**Extend the Parser When:**
- Domain-specific notation is essential (chemistry: `→`, physics: `⊗`)
- Custom operators with special precedence
- Proprietary mathematical notation
- Legacy system compatibility

### Architecture Overview

```
Input String
    ↓
Preprocessor (optional) - Transform syntax before parsing
    ↓
Lexer - Tokenize input (recognizes custom tokens)
    ↓
Parser (LALRPOP) - Build expression tree
    ↓
Post-Processor (optional) - Transform parsed expression
    ↓
Expression
```

## Basic Usage (⭐)

### Adding Custom Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Method 1: Simple name mapping
let parser = ParserBuilder::new()
    .add_function("erf", "error_function")
    .add_function("Si", "sine_integral")
    .add_function("Ci", "cosine_integral")
    .build();

let expr = parser.parse("erf(x) + Si(x)")?;
// Parsed as: error_function(x) + sine_integral(x)

// Method 2: Pattern-based recognition
let parser = ParserBuilder::new()
    .add_function_pattern(r"^[A-Z][a-z]*$", |name| {
        format!("special_{}", name.to_lowercase())
    })
    .build();

let expr = parser.parse("Bessel(0, x)")?;
// Parsed as: special_bessel(0, x)
```

### Adding Custom Operators

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Add custom infix operators
let parser = ParserBuilder::new()
    .add_operator("×", "*")      // Cross product symbol
    .add_operator("⊗", "tensor") // Tensor product
    .add_operator("∘", "compose")// Function composition
    .build();

let expr = parser.parse("A ⊗ B")?;
// Parsed as: tensor(A, B)
```

### Operator Precedence

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Define custom operator with precedence
let parser = ParserBuilder::new()
    .add_operator_with_precedence(
        "⊕",                     // Symbol
        "direct_sum",            // Function name
        Precedence::Addition     // Same as + and -
    )
    .build();

let expr = parser.parse("A ⊕ B ⊕ C")?;
// Parsed with left-associativity: direct_sum(direct_sum(A, B), C)
```

## Intermediate Usage (⭐⭐)

### Preprocessor Transformations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Add preprocessing step
let parser = ParserBuilder::new()
    .add_preprocessor(|input| {
        // Replace custom notation before parsing
        input.replace("→", "->")   // Arrow notation
             .replace("×", "*")     // Cross product
             .replace("÷", "/")     // Division symbol
    })
    .build();

let expr = parser.parse("x → ∞")?;  // Limit notation
// Preprocessed to: x -> ∞
// Then parsed normally
```

### Post-Processing Parsed Expressions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::core::Expression;

// Transform parsed expression
let parser = ParserBuilder::new()
    .add_postprocessor(|expr| {
        // Convert all cross(a, b) to determinant form
        match expr {
            Expression::Function(name, args) if name == "cross" => {
                // Transform to determinant representation
                Expression::function("det", vec![/* matrix form */])
            }
            _ => expr
        }
    })
    .build();
```

### Custom LaTeX Macros

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Add LaTeX macro expansion
let parser = LatexParserBuilder::new()
    .add_macro(r"\RR", r"\mathbb{R}")         // Real numbers
    .add_macro(r"\CC", r"\mathbb{C}")         // Complex numbers
    .add_macro(r"\NN", r"\mathbb{N}")         // Natural numbers
    .add_macro(r"\QQ", r"\mathbb{Q}")         // Rational numbers
    .add_macro(r"\ZZ", r"\mathbb{Z}")         // Integers
    .add_macro(r"\dd", r"\mathrm{d}")         // Differential d
    .build();

let expr = parser.parse(r"f: \RR \to \CC")?;
// Expands to: f: \mathbb{R} \to \mathbb{C}
```

### Domain-Specific Notation (Chemistry Example)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Chemistry notation
let chem_parser = ParserBuilder::new()
    .add_operator("→", "yields")          // Chemical reaction
    .add_operator("⇌", "equilibrium")     // Reversible reaction
    .add_function("Δ", "enthalpy_change") // Enthalpy change
    .add_preprocessor(|input| {
        // H2O → H_2*O (subscripts as multiplication)
        input.replace("₂", "_2")
    })
    .build();

let reaction = chem_parser.parse("H₂ + O₂ → H₂O")?;
```

### Domain-Specific Notation (Physics Example)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Quantum mechanics notation
let physics_parser = ParserBuilder::new()
    .add_operator("⊗", "tensor_product")     // Tensor product
    .add_operator("⟨", "bra")                // Bra notation
    .add_operator("⟩", "ket")                // Ket notation
    .add_function("⟨|⟩", "inner_product")    // Inner product
    .add_function("Tr", "trace")             // Trace
    .build();

let expr = physics_parser.parse("Tr(ρ ⊗ σ)")?;
```

## Advanced Usage (⭐⭐⭐)

### Modifying LALRPOP Grammar

**Location:** `crates/mathhook-core/src/parser/grammar.lalrpop`

**Example: Adding a new operator precedence level**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// In grammar.lalrpop

// 1. Add token to match block
match {
    // ... existing tokens
    "⊗" => TENSOR_PRODUCT,
}

// 2. Add precedence level (between Multiplication and Power)
TensorProduct: Expression = {
    <l:TensorProduct> TENSOR_PRODUCT <r:Power> =>
        Expression::function("tensor_product", vec![l, r]),
    Power,
};

// 3. Update precedence chain
Multiplication: Expression = {
    <l:Multiplication> MULTIPLY <r:TensorProduct> => expr!(l * r),
    // ...
    TensorProduct,
};
```

**Regenerate Parser:**
```bash
lalrpop crates/mathhook-core/src/parser/grammar.lalrpop
cargo build -p mathhook-core
```

### Custom Lexer Tokens

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// In parser/lexer/tokens.rs

#[derive(Debug, Clone, PartialEq)]
pub enum CustomToken {
    // Add your custom token types
    ChemicalArrow,
    EquilibriumArrow,
    // ...
}

// Implement tokenization logic
impl Lexer {
    pub fn recognize_custom_token(&mut self, input: &str) -> Option<CustomToken> {
        match input {
            "→" => Some(CustomToken::ChemicalArrow),
            "⇌" => Some(CustomToken::EquilibriumArrow),
            _ => None
        }
    }
}
```

### Parser Cache for Performance

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Create parser with caching
let parser = ParserBuilder::new()
    .with_cache(ParseCache::new(1000))  // Cache 1000 expressions
    .build();

// First parse: ~50μs
let expr1 = parser.parse("x^2 + 2*x + 1")?;

// Second parse: ~1μs (cache hit)
let expr2 = parser.parse("x^2 + 2*x + 1")?;
```

### Context-Aware Parsing

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Create context with assumptions
let context = ParseContext::new()
    .with_variable_type("A", SymbolType::Matrix)
    .with_variable_type("p", SymbolType::Operator)
    .with_constant("hbar", Expression::constant("reduced_planck"));

let parser = ParserBuilder::new()
    .with_context(context)
    .build();

// Parser knows A is a matrix
let expr = parser.parse("A * X = B")?;
// Automatically infers matrix symbols without \mathbf{}
```

### Error Recovery and Suggestions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Custom error handler with suggestions
let parser = ParserBuilder::new()
    .with_error_handler(|error| {
        match error {
            ParseError::UnknownFunction(name) => {
                // Suggest similar function names
                let suggestions = suggest_similar(name);
                format!("Unknown function '{}'. Did you mean: {}?",
                    name, suggestions.join(", "))
            }
            _ => error.to_string()
        }
    })
    .build();

let result = parser.parse("sine(x)")?;
// Error: Unknown function 'sine'. Did you mean: sin, sinh, asin?
```

### Multi-Stage Parsing Pipeline

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Complex parsing pipeline
let parser = ParserBuilder::new()
    // Stage 1: Lexer preprocessing
    .add_stage(ParseStage::Preprocessor(Box::new(|input| {
        normalize_unicode(input)
    })))
    // Stage 2: Custom token recognition
    .add_stage(ParseStage::Lexer(Box::new(custom_lexer)))
    // Stage 3: Grammar parsing
    .add_stage(ParseStage::Parser(Box::new(lalrpop_parser)))
    // Stage 4: Post-processing
    .add_stage(ParseStage::Postprocessor(Box::new(|expr| {
        expr.simplify()
    })))
    .build();
```

## Real-World Applications

### 1. Chemistry Equation Parser

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn create_chemistry_parser() -> Parser {
    ParserBuilder::new()
        .add_operator("→", "yields")
        .add_operator("⇌", "equilibrium")
        .add_operator("+", "plus")  // Reactant combination
        .add_preprocessor(|input| {
            // H2O → H_2*O
            // CO2 → C*O_2
            expand_chemical_formulas(input)
        })
        .add_postprocessor(|expr| {
            // Balance chemical equations
            balance_equation(expr)
        })
        .build()
}

// Usage
let parser = create_chemistry_parser();
let reaction = parser.parse("H₂ + O₂ → H₂O")?;
let balanced = reaction.balance();  // 2H₂ + O₂ → 2H₂O
```

### 2. Quantum Mechanics Notation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn create_quantum_parser() -> Parser {
    ParserBuilder::new()
        .add_operator("⊗", "tensor_product")
        .add_function("⟨|⟩", "inner_product")
        .add_function("Tr", "trace")
        .add_function("[,]", "commutator")
        .add_function("{,}", "anticommutator")
        .with_context(ParseContext::new()
            .with_symbol_type_pattern(r"^\w$", SymbolType::Operator) // Single letters = operators
            .with_symbol_type_pattern(r"^\|\w⟩$", SymbolType::Ket)   // |ψ⟩ notation
        )
        .build()
}

// Usage
let parser = create_quantum_parser();
let expr = parser.parse("[x, p] = iℏ")?;  // Canonical commutation
```

### 3. Financial Mathematics

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn create_finance_parser() -> Parser {
    ParserBuilder::new()
        .add_function("NPV", "net_present_value")
        .add_function("IRR", "internal_rate_of_return")
        .add_function("FV", "future_value")
        .add_function("PV", "present_value")
        .add_operator("%", "percent")
        .add_preprocessor(|input| {
            // $1,000 → 1000
            input.replace("$", "").replace(",", "")
        })
        .build()
}

// Usage
let parser = create_finance_parser();
let formula = parser.parse("NPV(0.1, [100, 200, 300])")?;
```

### 4. Control Theory Notation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn create_control_parser() -> Parser {
    ParserBuilder::new()
        .add_operator("*", "convolution")   // * is convolution in control theory
        .add_function("ℒ", "laplace_transform")
        .add_function("ℒ⁻¹", "inverse_laplace")
        .add_function("TF", "transfer_function")
        .add_operator("//", "feedback")     // Feedback connection
        .build()
}

// Usage
let parser = create_control_parser();
let system = parser.parse("G(s) // H(s)")?;  // Feedback system
```

## Common Patterns (Cookbook)

### Register Multiple Custom Functions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let parser = ParserBuilder::new()
    .add_functions(vec![
        ("erf", "error_function"),
        ("erfc", "complementary_error_function"),
        ("Si", "sine_integral"),
        ("Ci", "cosine_integral"),
        ("Ei", "exponential_integral"),
    ])
    .build();
```

### Create Domain-Specific Parser

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn create_domain_parser(domain: &str) -> Parser {
    match domain {
        "chemistry" => chemistry_parser(),
        "physics" => physics_parser(),
        "finance" => finance_parser(),
        _ => ParserBuilder::new().build()
    }
}

// Use appropriate parser for context
let parser = create_domain_parser("chemistry");
```

### Parse with Fallback

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
fn parse_with_fallback(input: &str) -> Result<Expression, ParseError> {
    // Try LaTeX first
    if let Ok(expr) = parse_latex(input) {
        return Ok(expr);
    }

    // Try Wolfram notation
    if let Ok(expr) = parse_wolfram(input) {
        return Ok(expr);
    }

    // Try custom parser
    let custom_parser = create_custom_parser();
    custom_parser.parse(input)
}
```

## Common Pitfalls

### ❌ WRONG → ✅ CORRECT

**Pitfall 1: Left recursion in grammar**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Left recursion causes infinite loop in LALRPOP
Atom: Expression = {
    NablaOperators,
    // ...
};

NablaOperators: Expression = {
    LATEX_NABLA <expr:Factorial> => ...
};

Factorial: Expression = {
    Atom,  // Cycle: Atom → NablaOperators → Factorial → Atom
    // ...
};

// ✅ CORRECT: Break the cycle with restricted rule
NablaArgument: Expression = {
    // Include all Atom alternatives EXCEPT NablaOperators
    FractionNotation,
    GreekSymbol,
    // ... (no NablaOperators here)
};

NablaOperators: Expression = {
    LATEX_NABLA <expr:NablaArgument> => ...  // Safe!
};
```

**Pitfall 2: Token matching order**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Generic patterns before specific ones
match {
    r"[a-zA-Z]+" => IDENTIFIER,  // Matches "sin", "cos", "exp"
    "sin" => SIN,                  // Never reached!
}

// ✅ CORRECT: Specific patterns first
match {
    "sin" => SIN,
    "cos" => COS,
    r"[a-zA-Z]+" => IDENTIFIER,  // Matches other identifiers
}
```

**Pitfall 3: Operator precedence conflicts**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Ambiguous precedence
// Is "a ⊗ b + c" parsed as "(a ⊗ b) + c" or "a ⊗ (b + c)"?

// ✅ CORRECT: Explicit precedence definition
let parser = ParserBuilder::new()
    .add_operator_with_precedence(
        "⊗",
        "tensor",
        Precedence::Multiplication  // Same as * (higher than +)
    )
    .build();
```

**Pitfall 4: Forgetting to regenerate parser**

```bash
# ❌ WRONG: Modify grammar.lalrpop and cargo build
# Old parser still used!

# ✅ CORRECT: Regenerate parser first
lalrpop crates/mathhook-core/src/parser/grammar.lalrpop
cargo build -p mathhook-core
```

**Pitfall 5: Unicode normalization**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ⚠️ Different Unicode representations can look identical

// "é" can be:
// - Single character: U+00E9 (é)
// - Composite: U+0065 + U+0301 (e + ́)

// ✅ CORRECT: Normalize before parsing
use unicode_normalization::UnicodeNormalization;

let parser = ParserBuilder::new()
    .add_preprocessor(|input| {
        input.nfc().collect::<String>()  // Normalize to NFC form
    })
    .build();
```

**Pitfall 6: Parser state pollution**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Mutable parser state affects subsequent parses
struct StatefulParser {
    context: Context,  // Mutable state
}

// ✅ CORRECT: Immutable parser, pass context explicitly
let parser = ParserBuilder::new().build();
let expr1 = parser.parse_with_context(input, &context1)?;
let expr2 = parser.parse_with_context(input, &context2)?;
```

## Performance Considerations

### Parser Generation Overhead

```bash
# LALRPOP compilation (one-time)
# - Small grammar: ~1s
# - Large grammar: ~10s

# Runtime parsing (repeated)
# - Simple expressions: ~10μs
# - Complex expressions: ~50-100μs
```

### Optimization Strategies

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// 1. Cache parsed expressions
let parser = ParserBuilder::new()
    .with_cache(ParseCache::new(1000))
    .build();

// 2. Batch preprocessing
let inputs = vec!["expr1", "expr2", "expr3"];
let preprocessed = inputs.iter()
    .map(|s| normalize(s))
    .collect::<Vec<_>>();

// 3. Parallel parsing (expressions are independent)
use rayon::prelude::*;
let expressions: Vec<_> = inputs.par_iter()
    .map(|input| parser.parse(input))
    .collect();
```

### Memory Efficiency

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Parser itself is lightweight (~1KB)
// Expressions are 32 bytes (cache-line optimized)
// Use Rc/Arc for shared subexpressions
```

## See Also

- **[LaTeX Notation](latex.md)** - Standard LaTeX mathematical notation
- **[Wolfram Notation](wolfram.md)** - Mathematica/Wolfram Language syntax
- **[Formatting](formatting.md)** - Output expressions in multiple formats
- **[Functions](../operations/functions.md)** - Registering custom mathematical functions
- **[Pattern Matching](../advanced/pattern-matching.md)** - Transform parsed expressions
- **[LALRPOP Documentation](https://lalrpop.github.io/lalrpop/)** - Parser generator reference
