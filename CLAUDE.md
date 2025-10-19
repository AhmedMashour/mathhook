# CLAUDE.md

This file provides guidance to Claude Code when working with the MathHook CAS codebase.

---

## CRITICAL: Core Mathematical Correctness Directive

**This is the absolute highest priority. All other guidelines are secondary.**

**IMPORTANT**: This CLAUDE.md file is the single source of truth. If anything in `.mathhook_sessions/` or elsewhere contradicts this document, CLAUDE.md takes precedence. Ignore contradicting information and flag the conflict.

### Non-Negotiable Requirements

1. **Mathematical Correctness First**: Every mathematical operation must be correct in ALL cases. No exceptions.
   - Equation solving must handle all valid cases correctly
   - Simplification must preserve mathematical equivalence
   - Evaluation must respect domain restrictions (e.g., sqrt of negatives, division by zero)
   - Symbolic operations must maintain algebraic properties

2. **Zero Tolerance for Regressions**: Any modification that breaks existing correct functionality is an unacceptable failure, regardless of what new functionality it adds.

3. **Verification Before Completion**: Before marking any task complete:
   - Run full test suite: `cargo test`
   - Verify affected functionality manually with edge cases
   - Check that doctests still pass: `cargo test --doc`
   - Confirm no mathematical accuracy was lost

4. **Authoritative References for Validation**:
   - **SymPy** (`~/Documents/work/math/sympy/`): Primary reference for algorithms and correctness validation
   - **Symbolica** (`~/Documents/work/math/symbolica`): Secondary reference for symbolic mathematics
   - When implementing any mathematical algorithm, verify against these references first

---

## Project Overview

MathHook is a high-performance educational computer algebra system (CAS) written in Rust. It provides symbolic mathematics, equation solving, LaTeX parsing, and step-by-step explanations optimized for both performance and educational use.

### Key Technical Constraints

These are non-negotiable architectural constraints. Violating them requires explicit discussion.

#### Expression Type (32-byte hard constraint)
- **Size**: Must remain 32 bytes to fit two expressions per 64-byte cache line (standard on modern CPUs)
- **Structure**: Enum with variants for all expression types (Add, Multiply, Power, Function, Symbol, Number, etc.)
- **Memory**: Uses `Box<T>` for recursive structures to maintain size constraint
- **Performance Impact**: Fits in cache line; adding bytes hurts performance significantly

#### Number Type (16-byte hard constraint)
- **Size**: Exactly 16 bytes (do not modify)
- **Representation**: Tagged union supporting:
  - Integers (arbitrary precision via `BigInt` pointer)
  - Rationals (numerator/denominator as `BigInt` pointers)
  - Floats (f64)
- **Operations**: All arithmetic must preserve exactness when possible
  - **ALWAYS use rationals** for symbolic/exact arithmetic (e.g., 1/3, 2/5)
  - **ONLY use floats** for numerical approximation or when exact representation is impossible
  - **NEVER mix rationals and floats** in symbolic operations—convert consistently to one type
  - When converting rational to float, document precision loss

#### Symbol Type (Critical for variable handling)
- **Purpose**: Represents mathematical variables (x, y, theta, etc.)
- **Implementation**: String interning for O(1) equality comparison
- **Comparison**: Symbols with same name are identical (pointer equality after interning)
- **Cloning**: Cheap (just increments reference count)
- **Usage**: Never use raw strings for variables; always use `Symbol::new(name)`
- **Properties**: Symbols can have assumptions (positive, real, integer, etc.) - respect these in simplification

#### Function Type (Universal Function Intelligence System)
- **Architecture**: Modular intelligence per function family
  - Elementary: `sin`, `cos`, `exp`, `log`, `sqrt`, etc.
  - Special: `gamma`, `zeta`, `bessel`, `erf`, etc.
  - Number theory: `factorial`, `gcd`, `lcm`, etc.
  - Polynomials: Polynomial-specific functions
  
- **Registry**: `UniversalFunctionRegistry` provides O(1) lookup
  - Maps function name → function intelligence
  - Each function has: evaluation strategy, properties, simplification rules, educational explanations

- **Properties**: Functions know their own mathematical properties
  - Domain/range restrictions
  - Symmetry (even, odd, periodic)
  - Special values (e.g., sin(0) = 0, sin(π/2) = 1)
  - Derivatives and integrals

- **Evaluation**: Multi-strategy evaluation
  - Exact symbolic evaluation when possible
  - SIMD-optimized numerical evaluation
  - Handles special cases (sin(π) → 0, not 1.2246467991473532e-16)

#### LALRPOP Grammar (Parser Architecture)

**Critical Understanding**: The parser has two stages:
1. **Lexer** (`parser/lexer/`): Tokenizes input and inserts implicit multiplication tokens
2. **Parser** (`parser/grammar.lalrpop`): LALRPOP-generated LR(1) parser

**Implicit Multiplication Rules** (handled in lexer):
```
2x        → 2 * x
(a)(b)    → a * b
2(x+1)    → 2 * (x + 1)
sin(x)    → sin(x) * (no implicit multiplication before function args)
x y       → x * y (when both are identifiers)
2.5x      → 2.5 * x
```

**Operator Precedence** (highest to lowest):
1. Function application: `sin(x)`, `log(y)`
2. Exponentiation: `^` (right-associative!)
3. Implicit/explicit multiplication: `*`, `/` (left-associative)
4. Addition/subtraction: `+`, `-` (left-associative)

**LALRPOP Specifics**:
- **Grammar Type**: LR(1) - one token lookahead
- **Conflict Resolution**: Defaults to shift on shift/reduce conflicts
- **Right Associativity**: Exponentiation is special-cased as right-associative (e.g., `2^3^4` → `2^(3^4)`)
- **Location**: `crates/mathhook-core/src/parser/grammar.lalrpop`
- **Generated Output**: `crates/mathhook-core/src/parser/grammar.rs` (do not edit manually)

**Common Parser Pitfalls**:
- **Shift/Reduce Conflicts**: LALRPOP reports these; they usually indicate ambiguous grammar
- **Precedence Errors**: Test thoroughly with complex expressions: `2+3*4^5`
- **Implicit Multiplication Bugs**: The lexer handles this; don't duplicate in grammar
- **Function Call Ambiguity**: `sin(x)(y)` should parse as `sin(x) * y`, not `(sin(x))(y)`

**Testing Parser Changes**: Always test these cases after grammar modifications:
```rust
// Basic operations
"2+3*4"           // Should: 2 + (3 * 4) = 14
"2^3^4"           // Should: 2^(3^4) = 2^81 (right-associative)

// Implicit multiplication
"2x"              // Should: 2 * x
"2(x+1)"          // Should: 2 * (x + 1)
"(a+b)(c+d)"      // Should: (a + b) * (c + d)
"2.5x"            // Should: 2.5 * x

// Functions
"sin(x)cos(y)"    // Should: sin(x) * cos(y)
"2sin(x)"         // Should: 2 * sin(x)
"sin(x)"           // Should: sin(x) (NOT sin * x)
"log(x)/log(y)"   // Should: log(x) / log(y)

// Edge cases
"--x"             // Should: -(-x) = x (double negation)
"2+-3"            // Should: 2 + (-3) = -1
"x^-2"            // Should: x^(-2) = 1/x^2
```

**Grammar Rule Ordering and Ambiguity Resolution**:

LALRPOP (LR(1) parser) has strict limitations that require careful grammar design:

1. **More Specific Patterns Must Come First**:
   - In choice alternatives (e.g., `Atom: Expression = { ... }`), LALRPOP matches in order
   - Put longer, more specific patterns before shorter, more general ones
   - Example: `FractionNotation` must come before `IdentifierOrFunction` in the `Atom` alternatives

2. **Ambiguity with Lookahead**:
   - LR(1) has only ONE token lookahead
   - Patterns like `\frac{dy}{dx}` (derivative) vs `\frac{a}{b}` (fraction) are ambiguous
   - The parser cannot distinguish until it sees the SECOND token inside the numerator
   - **Solution**: Either handle in lexer preprocessing OR use separate entry points

3. **Operator Body Expressions**:
   - Use `Atom` level (not `Power` or `Multiplication`) for operator bodies to avoid ambiguity
   - Example: `LATEX_SUM ... <summand:Atom>` prevents ambiguity with factorial
   - **Trade-off**: Complex expressions need parentheses (`\int (x^2) dx`, not `\int x^2 dx`)
   - This is acceptable: most mathematical notation uses parentheses anyway

4. **Calculus Operators**:
   - `\int`, `\sum`, `\prod`, `\lim` are function-like prefix operators, not atoms
   - Place them in `IdentifierOrFunction` rule, not as standalone rules
   - Use `Atom` for their body expressions to prevent grammar conflicts

5. **Fraction Notation**:
   - `\frac{num}{den}` successfully added by placing early in `Atom` alternatives
   - Converts to division: `Expression::mul(vec![num, Expression::pow(den, Expression::integer(-1))])`
   - Partial derivatives also supported: `\frac{\partial f}{\partial x}`

6. **Known Limitations**:
   - Full derivative notation (`\frac{d}{dx} expr`, `\frac{dy}{dx}`) deferred due to LR(1) ambiguity
   - Would require lexer preprocessing or procedural macro parser
   - Current workaround: Parse as regular fractions, users can use explicit derivative functions

7. **Avoid Left Recursion** (CRITICAL for LALRPOP):
   - **Direct left recursion**: `A: Expression = { A ... => ... }` will cause infinite loop during parser generation
   - **Indirect left recursion**: More subtle and dangerous:
     ```
     Atom: Expression = { NablaOperators, ... };
     NablaOperators: Expression = { LATEX_NABLA <expr:Factorial> => ... };
     Factorial: Expression = { Atom, ... };
     ```
     This creates cycle: `Atom → NablaOperators → Factorial → Atom` (infinite loop!)
   - **Solution**: Break the cycle by creating restricted rule types that don't include the problematic alternative
   - **Example Fix**: Create `NablaArgument` rule that includes all `Atom` alternatives EXCEPT `NablaOperators`:
     ```rust
     NablaArgument: Expression = {
         VectorWrappers,
         FractionNotation,
         GreekSymbol,
         Number,
         Constant,
         ParenExpression,
         IdentifierOrFunction,
         AbsoluteValue,
         Set,
         Interval,
     };

     NablaOperators: Expression = {
         LATEX_NABLA <expr:NablaArgument> => ...  // Now safe!
     };
     ```
   - **Detection**: If `cargo build` hangs forever during parser generation, you likely have left recursion
   - **Testing After Fix**: Verify the specific operators still parse correctly

**When Modifying Grammar**:
- Test with `lalrpop grammar.lalrpop` first to see errors before cargo build
- Always run `cargo test -p mathhook-core parser` after changes
- Check the generated `.rs` file for shift/reduce conflicts (LALRPOP reports these)
- If ambiguity errors occur, reorder rules or simplify patterns

#### Zero-Copy Parsing
- Parse strings directly into AST without intermediate allocations where possible
- Use string slices (`&str`) over `String` during parsing
- Convert to owned types only when storing in `Expression`

#### SIMD Operations
- Located in `core/performance/`
- Vectorized operations for arrays of numbers
- Must provide scalar fallback for non-SIMD targets
- Test with both AVX2 and SSE2 feature flags

---

## Mathematical Correctness Architecture

These architectural decisions ensure mathematical correctness across all operations.

### Domain and Range Handling

**Critical for CAS Correctness**: Every operation must handle domain restrictions properly.

**Domain Restrictions Strategy:**

1. **Identify Domain** (compile-time or runtime):
   - `sqrt(x)`: Real domain requires `x ≥ 0`; complex domain allows all `x`
   - `log(x)`: Real domain requires `x > 0`; complex domain has branch cut on negative real axis
   - `1/x`: Undefined at `x = 0` (pole)
   - `tan(x)`: Undefined at `x = π/2 + nπ`

2. **Handle Out-of-Domain Inputs**:
   - **Symbolic context**: Keep expression symbolic (e.g., `sqrt(-1)` → `i` in complex domain, stays `sqrt(-1)` in real)
   - **Numerical context**: Return error or promote to complex (based on domain setting)
   - **NEVER produce mathematically incorrect results silently**

3. **Branch Cuts** (for multi-valued functions):
   - Document which branch is chosen (e.g., principal branch for `log`)
   - Complex `log`: Branch cut on negative real axis
   - Complex `sqrt`: Branch cut on negative real axis
   - Arctrig functions: Document range

4. **Undefined vs Indeterminate**:
   - `1/0`: Undefined (pole) → error or symbolic infinity
   - `0/0`: Indeterminate form → may require limit analysis
   - `0^0`: Convention-dependent (often 1 in combinatorics, indeterminate in analysis)

**Implementation Requirements:**
- Use `Result<Expression, DomainError>` for operations that can fail
- Document domain restrictions in function documentation
- Test boundary cases extensively

### Canonical Forms and Expression Equality

**Critical for CAS Consistency**: Expressions must have canonical forms for reliable equality checking and simplification.

**Canonical Form Rules:**

1. **Commutative Operations** (`Add`, `Mul`):
   - Sort terms/factors in consistent order (e.g., lexicographic by symbol name, then by type)
   - Example: `y + x` → `x + y`
   - Example: `2 * y * x` → `2 * x * y`

2. **Identity Elements**:
   - Addition: `x + 0` → `x`
   - Multiplication: `x * 1` → `x`
   - Exponentiation: `x^1` → `x`, `x^0` → `1` (except `0^0`)

3. **Associativity Flattening**:
   - `(a + b) + c` → `Add(a, b, c)` (flat, not nested)
   - `(a * b) * c` → `Mul(a, b, c)` (flat, not nested)

4. **Negation and Subtraction**:
   - Subtraction as addition: `a - b` → `a + (-1 * b)`
   - Negation as multiplication: `-x` → `(-1) * x`

5. **Division and Reciprocals**:
   - Division as multiplication: `a / b` → `a * b^(-1)`

6. **Rational Numbers**:
   - Always reduce to lowest terms: `6/4` → `3/2`
   - Never represent as `(6) / (4)`; use `Number::Rational(3, 2)`

**Expression Equality:**
- Structural equality: After canonical form conversion, use `==` comparison
- Symbolic equality: May require simplification + zero detection
- Numerical equality: Epsilon comparison for floats

**Implementation Requirements:**
- Every constructor (`add`, `mul`, `pow`) should produce canonical form
- Provide `to_canonical()` method for legacy code
- Document canonical form in each variant's documentation

### Symbol Assumptions System

**Critical for Correct Simplification**: Symbols can have assumptions that affect simplification.

**Assumption Types:**

1. **Domain Assumptions**:
   - `real`: Symbol represents a real number
   - `complex`: Symbol represents a complex number
   - `positive`: `x > 0`
   - `negative`: `x < 0`
   - `nonnegative`: `x ≥ 0`
   - `nonzero`: `x ≠ 0`

2. **Type Assumptions**:
   - `integer`: Symbol is an integer
   - `rational`: Symbol is a rational number
   - `prime`: Symbol is a prime integer
   - `even` / `odd`: Parity assumptions

3. **Bounds Assumptions**:
   - `bounded(a, b)`: `a ≤ x ≤ b`
   - Useful for optimization and domain checking

**Assumption Propagation:**

When deriving new expressions, propagate assumptions:
- `x > 0` and `y > 0` implies `x + y > 0` and `x * y > 0`
- `x > 0` implies `x^2 > 0` and `sqrt(x)` is real
- `x is_real` and `y is_real` implies `x + y is_real`

**Conflict Detection:**

Detect and error on conflicting assumptions:
- `x > 0` and `x < 0` is impossible
- `x is_integer` and `x = 1/2` is impossible

**Implementation Requirements:**
- Store assumptions in `Symbol` type
- Check assumptions during simplification
- Query assumptions: `symbol.is_positive()`, `symbol.is_real()`
- Provide `with_assumption()` method

### Complex Number System

**Critical for Completeness**: Complex numbers are fundamental to CAS.

**Complex Number Representation:**

1. **Two Forms**:
   - **Symbolic**: `a + b*i` where `a`, `b` are expressions and `i` is the imaginary unit
   - **Explicit**: `Complex(Box<ComplexData>)` for numerical complex values

2. **When to Use Each**:
   - Symbolic `a + b*i`: For algebraic manipulation, symbolic expressions
   - Explicit `Complex`: For numerical computation with complex numbers

3. **Operations**:
   - Addition: `(a + bi) + (c + di) = (a + c) + (b + d)i`
   - Multiplication: `(a + bi)(c + di) = (ac - bd) + (ad + bc)i`
   - Division: Use conjugate: `(a + bi)/(c + di) = [(a + bi)(c - di)] / (c² + d²)`

**Branch Cuts and Principal Values:**

For multi-valued functions, document which branch:
- `sqrt(z)`: Principal branch, branch cut on negative real axis
- `log(z)`: Principal branch, `log(z) = ln|z| + i*arg(z)` where `-π < arg(z) ≤ π`
- `z^w`: Use principal branch of `log`

**Real vs Complex Domain:**

- Operations default to **complex-safe** (don't assume real domain unless specified)
- `sqrt(-1)` → `i` (complex result)
- If user wants real-only domain, provide `sqrt_real(-1)` → `Error`

**Implementation Requirements:**
- Provide `to_complex()` conversion for all expressions
- Provide `real()` and `imag()` extraction methods
- Handle branch cuts correctly in function evaluation
- Test extensively with complex inputs

### Error Handling Principles

**Critical for Reliability**: Consistent error handling across all operations.

**Error Handling Strategy:**

1. **Return Types**:
   - **Constructors** (`add`, `mul`, `pow`): Return `Expression` directly (always succeed, produce canonical form)
   - **Evaluation** (`evaluate`, `simplify`): Return `Result<Expression, MathError>` (can fail on domain errors)
   - **Parsing**: Return `Result<Expression, ParseError>`
   - **Solving**: Return `Result<Vec<Expression>, SolverError>` (may have no solutions)

2. **Error Types**:
   ```rust
   pub enum MathError {
       DomainError { operation: String, value: Expression, reason: String },
       DivisionByZero,
       Undefined { expression: Expression },
       NumericOverflow,
       NotImplemented { feature: String },
   }
   ```

3. **When to Error vs Return Symbolic**:
   - **Numerical evaluation context**: Error on domain violations (e.g., `sqrt(-1).evaluate()` → `DomainError`)
   - **Symbolic context**: Keep symbolic (e.g., `sqrt(-1)` → stays as `sqrt(-1)` or simplifies to `i`)
   - **Division by zero**: Symbolic context → keep as `1/0` or `Infinity`; numerical context → `DivisionByZero`

4. **Panic Policy**:
   - **NEVER panic** in library code (panics are for programmer errors, not math errors)
   - Use `Result` for all fallible operations
   - Exception: Internal invariant violations (use `debug_assert!` and document assumptions)

**Implementation Requirements:**
- Define comprehensive error types in `core/error.rs`
- Provide helpful error messages with context
- Document error conditions in function documentation
- Test error paths as thoroughly as success paths

### Simplification Strategy

**Critical for Correctness**: Simplification must preserve mathematical equivalence while producing canonical form.

**Simplification Order** (must be followed in this order):

1. **Normalize to Canonical Form**:
   - Flatten associative operations: `(a + b) + c` → `Add(a, b, c)`
   - Sort commutative operations: `y + x` → `x + y`
   - Remove identity elements: `x + 0` → `x`, `x * 1` → `x`

2. **Apply Algebraic Identities**:
   - Combine like terms: `2x + 3x` → `5x`
   - Power rules: `x^a * x^b` → `x^(a+b)`
   - Trigonometric identities: `sin²(x) + cos²(x)` → `1`
   - Logarithm rules: `log(a) + log(b)` → `log(a*b)`

3. **Numerical Evaluation** (only if requested):
   - Evaluate constant subexpressions: `2 + 3` → `5`
   - Simplify exact symbolic values: `sin(π/2)` → `1`
   - NEVER approximate unless explicitly requested

**Zero Detection**:
- Exact zero: `x - x` → `0` after simplification
- Numerical zero: Use epsilon comparison for floats
- Symbolic zero: May require advanced techniques (Gröbner bases, etc.)

**Non-Simplification Principle**:
- If no simplification applies, return expression unchanged
- NEVER "simplify" in ways that make expression more complex
- NEVER lose information (e.g., don't factor unless requested)

**Implementation Requirements:**
- Implement simplification as series of rewrite rules
- Each rule must preserve mathematical equivalence
- Test that `simplify(simplify(x)) == simplify(x)` (idempotent)
- Test against SymPy for correctness

### Concurrency and Thread Safety

**Critical for Performance**: MathHook must support safe concurrent use.

**Thread Safety Guarantees:**

1. **Expression Type**:
   - `Expression` is **immutable** after creation
   - Safe to clone across threads (`Clone` is thread-safe)
   - Safe to share across threads with `Arc<Expression>`
   - All operations produce new expressions (no mutation)

2. **Symbol Type**:
   - String interning uses thread-safe interning mechanism
   - `Symbol` is `Send + Sync`
   - Symbol lookup is lock-free (or uses efficient read-write locks)

3. **UniversalFunctionRegistry**:
   - Global registry is **immutable** after initialization
   - Function lookup is thread-safe (read-only)
   - Lazy initialization uses `once_cell` or `lazy_static` for safety

4. **Number Type**:
   - `Number` is immutable
   - `BigInt` operations are thread-safe (no shared mutable state)
   - Arithmetic produces new `Number` instances

**Parallelization Opportunities:**

- Simplification of independent subexpressions (embarrassingly parallel)
- SIMD operations for array arithmetic
- Parallel solving of system equations

**Implementation Requirements:**
- All public types must be `Send + Sync` where applicable
- Use `Arc` for shared ownership, never `Rc` in public API
- Document thread-safety guarantees in type documentation
- Test with `cargo test -- --test-threads=16`

### Operator Overloading Guidelines

**Design Decision**: MathHook provides BOTH explicit API and operator overloading for ergonomics.

**Operator Overloading Support:**

1. **Arithmetic Operators** (implement for `Expression`):
   ```rust
   impl Add for Expression         // x + y
   impl Sub for Expression         // x - y
   impl Mul for Expression         // x * y
   impl Div for Expression         // x / y
   impl Neg for Expression         // -x
   // Note: ^ is not overloadable in Rust; use .pow() method
   ```

2. **When Operators Are Preferred**:
   - In test code: `x + y` is clearer than `Expression::add(vec![x, y])`
   - In mathematical algorithms where standard notation improves readability
   - When chaining operations: `(x + y) * (x - y)`

3. **When Explicit API Is Preferred**:
   - Variable number of arguments: `Expression::add(vec![x, y, z, w])`
   - Programmatic construction in loops
   - When type inference needs help

**Implementation Requirements:**
- Operators should delegate to canonical constructors
- Ensure operators produce canonical form
- Document operator precedence in Rust vs mathematical precedence
- Test operator overloading thoroughly

---

## Documentation Standards (Strictly Enforced)

### Module and Function Documentation

1. **Use `//!` ONLY for module documentation** (at the top of files)
2. **Use `///` ONLY for item documentation** (functions, structs, enums, traits)
3. **Minimize inline `//` comments**. Prefer documentation comments (`///`). Use inline comments only for:
   - Annotating specific mathematical formulas (e.g., `// Quadratic formula: x = (-b ± √(b²-4ac)) / 2a`)
   - Explaining algorithm rationale or mathematical properties
   - Clarifying non-obvious edge cases or domain restrictions
   - Avoid stating the obvious; let code be self-documenting when possible
   - Avoid exaggerating in what you say about something

### Documentation Requirements

Every public function MUST include:
- Clear description of what it does
- `# Arguments` section documenting each parameter
- `# Examples` section with a runnable doctest in a ````rust` block
- `# Panics` section if the function can panic
- `# Safety` section if the function is unsafe

### Prohibited Content

- **No emojis anywhere** (code, comments, documentation, commit messages)
- **No ALL CAPS** except for constants (e.g., `const MAX_DEPTH: usize = 100;`)
- **No TODO comments for incomplete critical functionality** - implement completely or don't implement at all. TODOs for future enhancements are acceptable if current behavior is mathematically correct.
- **No placeholder implementations** - if a function exists, it must be fully correct

### Legacy Cleanup

If you encounter violations of these rules, delete them immediately.

### Example of Correct Documentation

```rust
/// Computes the derivative of an expression with respect to a variable
///
/// Uses the chain rule, product rule, and quotient rule as needed. For polynomial
/// expressions, applies power rule. For transcendental functions, uses standard
/// calculus derivative rules.
///
/// # Arguments
///
/// * `expr` - The expression to differentiate
/// * `var` - The variable to differentiate with respect to
/// * `order` - The order of differentiation (1 for first derivative, 2 for second, etc.)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let expr = expr!(x ^ 2);
/// let derivative = expr.derivative(&x, 1);
/// assert_eq!(derivative.to_string(), "2*x");
/// ```
///
/// # Returns
///
/// Returns the expression itself if `order` is 0 (mathematically valid: 0th derivative equals the original function).
pub fn derivative(&self, var: &Symbol, order: u32) -> Expression {
    // Implementation
}
```

---

## Pre-Commit Verification Checklist

**MANDATORY**: Before marking any task complete or suggesting a commit, explicitly verify:

### 1. Comments Audit
- [ ] No `//` inline comments except mathematical formulas or critical non-obvious logic
- [ ] All `//!` are module-level only (top of file)
- [ ] All `///` are item documentation only (functions, structs, traits)
- Search command: `rg "^\s*//[^/!]" --type rust` (should return ONLY formulas/critical logic)

### 2. Forbidden Content
- [ ] No emojis anywhere
- [ ] No ALL CAPS (except constants like `MAX_DEPTH`)
- [ ] No TODO comments for incomplete critical functionality
- [ ] No placeholder implementations

### 3. Test Coverage
- [ ] Ran relevant tests: `cargo test -p mathhook-core <module>`
- [ ] No regressions (test count ≥ baseline)
- [ ] All doctests pass: `cargo test --doc`

### 4. Mathematical Correctness
- [ ] Verified against SymPy/Symbolica if implementing algorithms
- [ ] Edge cases tested (zero, infinity, undefined, complex)
- [ ] Domain restrictions documented and tested

### 5. Performance Impact
- [ ] No performance regressions (run benchmarks if modifying hot paths)
- [ ] Expression size constraint maintained (32 bytes)

**AI Agent Protocol**: State "✅ Verified against CLAUDE.md checklist" before marking tasks complete.

---

## Code Quality Principles (Priority Order)

After mathematical correctness, prioritize in this exact order:

1. **Performance & Memory Efficiency**: Write highly performant, cache-friendly code
   - Profile hot paths with `cargo bench`
   - Minimize allocations in tight loops
   - Use arena allocation for bulk operations
   - Prefer stack allocation over heap when possible

2. **Readability & Idiomatic Rust**: Follow Rust best practices
   - Reference: [The Rust Book](https://doc.rust-lang.org/book/)
   - Use iterators over manual loops
   - Leverage the type system for correctness
   - Prefer `impl Trait` over boxed trait objects when possible

3. **Memory Safety & Performance Optimization**:
   - Reference: [Rust Performance Book](https://nnethercote.github.io/perf-book/)
   - Minimize bounds checking in hot paths
   - Use `#[inline]` judiciously on small, frequently-called functions
   - Consider SIMD for vectorizable operations

4. **Modularity**: Design components to be loosely coupled with clear interfaces

5. **Meaningful Testing**: Write tests that validate correctness and edge cases, not just coverage

6. **Architectural Patterns Over Hardcoding**: Avoid hardcoded matches for mathematical elements
   - **NEVER hardcode function names** (sin, cos, etc.) in implementation logic
   - Use the `UniversalFunctionRegistry` for function-specific behavior
   - Leverage type system and traits over string matching
   - Example:
     ```rust
     // ❌ DON'T: Hardcoded function matching
     match func_name.as_str() {
         "sin" => /* special handling */,
         "cos" => /* special handling */,
         _ => /* generic */
     }

     // ✅ DO: Use registry-based dispatch
     if let Some(intelligence) = registry.get_function(func_name) {
         intelligence.evaluate(args)
     }
     ```
   - This principle applies to: functions, constants, operators, and mathematical patterns
   - Benefits: extensibility, maintainability, testability, performance (O(1) lookup)

---

## Common AI Pitfalls (CRITICAL - Read Carefully)

### Mathematical Errors to Avoid

1. **Off-by-one in polynomial degrees**: When working with polynomials, be extremely careful with degree calculations
2. **Sign errors in derivatives**: Chain rule application is error-prone for complex expressions
3. **Domain restrictions**: Always check for division by zero, sqrt of negatives, log of non-positives
4. **Floating point comparison**: Never use `==` for floats; use epsilon comparison or symbolic equality
5. **Rational arithmetic overflow**: Integer operations in `Number` type can overflow. Use checked arithmetic operations (`checked_add`, `checked_mul`, etc.). On overflow, promote to arbitrary precision BigInt or return an error. NEVER silently wrap.
6. **Numerical stability**: Watch for catastrophic cancellation in subtraction (e.g., `(x + 1) - x` for large `x`). Use mathematically equivalent stable forms when possible.
7. **Floating point special cases**: Handle `sin(π) → 0` symbolically when input is exact symbolic constant, not approximate float `0.000000000000001`. Detect exact vs approximate inputs.

### Rust-Specific Mistakes

1. **Unnecessary cloning**: Many operations can take references instead of owned values
2. **Iterator anti-patterns**: Don't collect() unnecessarily; chain iterators instead
3. **Premature optimization**: Profile before optimizing; don't assume what's slow
4. **Lifetime over-complication**: Keep lifetimes simple; prefer owned types when unclear

### Parser Gotchas (LALRPOP)

1. **Shift/reduce conflicts**: LALRPOP defaults to shift; this may not be what you want
2. **Implicit multiplication precedence**: Be extremely careful with operator precedence when modifying grammar
3. **Lexer-parser mismatch**: The lexer pre-processes for implicit multiplication; don't duplicate this in grammar
4. **Token lookahead**: LR(1) only has 1 token lookahead; complex disambiguation needs lexer help

When modifying `grammar.lalrpop`:
- Run `lalrpop crates/mathhook-core/src/parser/grammar.lalrpop` to see errors before cargo build
- Test extensively: `cargo test -p mathhook-core parser`
- Check that implicit multiplication still works: `2x`, `(a)(b)`, `2(x+1)`

---

## Macro Usage Guidelines

**PRIORITY SYSTEM**: Macros are **MANDATORY** for symbol creation and **STRONGLY RECOMMENDED** for expression creation. This section defines a strict 3-level priority hierarchy.

### Available Macros

MathHook provides declarative macros for ergonomic expression creation:

- **`symbol!(x)`** - Create single symbols with optional type specification (Scalar/Matrix/Operator/Quaternion)
- **`symbols!("x y z")`** - Create multiple symbols at once (PLANNED - not yet implemented, coming in Wave 9)
- **`expr!(...)`** - Create expressions with operator parsing
- **`function!(name, args...)`** - Create function expressions

### Quick Reference: When to Use What

| Scenario | Use | Example |
|----------|-----|---------|
| **Creating scalar symbol** | `symbol!(x)` (Priority 1) | `let x = symbol!(x);` |
| **Creating matrix symbol** | `symbol!(A; matrix)` (Priority 1) | `let A = symbol!(A; matrix);` |
| **Creating operator symbol** | `symbol!(p; operator)` (Priority 1) | `let p = symbol!(p; operator);` |
| **Creating quaternion symbol** | `symbol!(i; quaternion)` (Priority 1) | `let i = symbol!(i; quaternion);` |
| **Creating multiple symbols** | Multiple `symbol!()` calls (Priority 1) | `let x = symbol!(x); let y = symbol!(y);` |
| **Creating multiple symbols (FUTURE)** | `symbols!("x y z")` when available (Wave 9) | `let (x, y, z) = symbols!("x y z");` |
| **Simple expression** | `expr!(x + y)` (Priority 2) | `let sum = expr!(x + y);` |
| **Complex expression** | Explicit grouping or API (Priority 3) | `expr!((2*x) + (3*y))` |
| **Loop/runtime data** | Explicit API (Priority 3) | `Expression::integer(i)` |

**Golden Rule**: NEVER use `Symbol::new()`, `Symbol::scalar()`, `Symbol::matrix()`, etc. directly in application code. ALWAYS use `symbol!()` macro (or `symbols!()` when available in Wave 9).

### Macro Priority Hierarchy (Highest to Lowest)

#### **Priority 1: MANDATORY - Symbol Creation (NO EXCEPTIONS)**

**Rule**: NEVER use `Symbol::new()`, `Symbol::scalar()`, `Symbol::matrix()`, etc. directly. ALWAYS use macros.

**Single Symbol Creation**: Use `symbol!(x)` with optional type
```rust
// ❌ NEVER use direct constructors:
Symbol::new("x")          // WRONG - deprecated
Symbol::scalar("x")       // WRONG - use macro instead
Symbol::matrix("A")       // WRONG - use macro instead

// ✅ ALWAYS use symbol! macro:
symbol!(x)                // Scalar (default, commutative)
symbol!(theta)            // Scalar
symbol!(A; matrix)        // Matrix (noncommutative)
symbol!(p; operator)      // Operator (noncommutative)
symbol!(i; quaternion)    // Quaternion (noncommutative)
```

**Type-Specific Symbol Creation (CURRENT - Fully Supported)**:
```rust
// ✅ Scalar symbols (default, commutative)
let x = symbol!(x);
let y = symbol!(y);
let theta = symbol!(theta);

// ✅ Matrix symbols (noncommutative) - Wave 1 Complete
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// ✅ Operator symbols (noncommutative) - Wave 1 Complete
let p = symbol!(p; operator);      // Momentum operator
let x_op = symbol!(x; operator);   // Position operator
let H = symbol!(H; operator);      // Hamiltonian

// ✅ Quaternion symbols (noncommutative) - Wave 1 Complete
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);
```

**Bulk Symbol Creation (CURRENT - Manual Approach)**:
```rust
// ✅ Current approach - individual symbol!() calls:
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// ✅ With types:
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);
```

**Bulk Symbol Creation (FUTURE - After Wave 9)**:
```rust
// ✅ Will be available after symbols!() macro is implemented (Wave 9):
let (x, y, z) = symbols!("x y z");              // All scalars (default)
let (A, B, C) = symbols!("A B C"; matrix);      // All matrices (noncommutative)
let (p, x, h) = symbols!("p x h"; operator);    // All operators (noncommutative)
let (i, j, k) = symbols!("i j k"; quaternion);  // All quaternions (noncommutative)
```

#### **Priority 2: STRONGLY RECOMMENDED - Expression Creation**

**Rule**: Prefer macros for simple expressions. Use explicit API only when necessary.

**When to Use `expr!()` Macro:**

1. **Simple operations** (always use macros):
   ```rust
   // ✅ Good - clean and readable:
   expr!(x + y)
   expr!(2 * x)
   expr!(x ^ 2)
   expr!(sin(x))
   expr!((x + 1) * (x - 1))  // Use parens for grouping

   // ❌ Don't use verbose API for simple cases:
   Expression::add(vec![
       Expression::symbol(Symbol::new("x")),
       Expression::symbol(Symbol::new("y"))
   ])
   ```

2. **Test code and documentation** (makes examples immediately readable):
   ```rust
   #[test]
   fn test_derivative() {
       let x = symbol!(x);
       let f = expr!(x ^ 2);
       let df = f.derivative(&x, 1);
       assert_eq!(df, expr!(2 * x));  // ✅ Clear and readable
   }
   ```

3. **Multi-term operations** (use explicit macro helpers):
   ```rust
   // For many terms, use explicit helpers:
   expr!(add: x, y, z, w)      // ✅ Clear
   expr!(mul: 2, x, y, z)      // ✅ Clear
   ```

#### **Priority 3: USE EXPLICIT API WHEN APPROPRIATE**

**When to Use Explicit API Instead of Macros:**

1. **Complex mixed operations** (when precedence is unclear):
   ```rust
   // ⚠️  Unclear: expr!(2*x + y*z - 3)
   // ✅ Clear with explicit grouping:
   expr!((2*x) + (y*z) - 3)  // Good with parens
   // OR use explicit API for very complex cases
   ```

2. **Programmatic construction** (loops, conditionals, dynamic building):
   ```rust
   // Building terms in a loop - explicit API is clearer:
   let mut terms = Vec::new();
   for i in 0..n {
       terms.push(Expression::mul(vec![
           Expression::integer(coeffs[i]),
           Expression::pow(symbol!(x), Expression::integer(i))
       ]));
   }
   Expression::add(terms)
   ```

3. **Runtime variables** (macros see tokens, not values):
   ```rust
   // ❌ WRONG - macro sees token "i", not the value:
   for i in 0..10 {
       let expr = expr!(i);  // Creates Symbol::new("i")!
   }

   // ✅ CORRECT - use explicit API:
   for i in 0..10 {
       let expr = Expression::integer(i);
   }
   ```

### Macro Capabilities and Limitations

**What Works Well (Use These Patterns):**
```rust
// ✅ Literals and symbols
expr!(42)
expr!(x)
expr!(pi)    // Mathematical constants use dedicated constructors (Expression::pi(), Expression::e(), Expression::i())

// ✅ Single binary operations
expr!(x + y)
expr!(x - y)
expr!(x * y)
expr!(x / y)
expr!(x ^ 2)

// ✅ Unary operations
expr!(-x)

// ✅ Function calls (0-3 args)
expr!(sin(x))
expr!(log(x, y))
expr!(f(x, y, z))

// ✅ Grouped operations with explicit parentheses
expr!((x + 1) * (x - 1))
expr!((2*x) + 3)

// ✅ Multi-term explicit operations
expr!(add: x, y, z)
expr!(mul: 2, x, y)
```

**Limitations (Declarative Macro Constraints):**

1. **Mixed operators without parentheses may have precedence issues**
   ```rust
   // ⚠️  Precedence unclear:
   expr!(2*x + 3)
   
   // ✅ Use explicit grouping:
   expr!((2*x) + 3)
   ```

2. **Functions with 4+ arguments need explicit helper**
   ```rust
   // ❌ Won't match macro patterns:
   expr!(f(a, b, c, d, e))
   
   // ✅ Use function! macro:
   function!(f, expr!(a), expr!(b), expr!(c), expr!(d), expr!(e))
   ```

3. **Complex nested operations - use explicit API when clearer**
   ```rust
   // If readability suffers with macros, use explicit API
   ```

**Future Enhancement (Procedural Macro):**

A future procedural macro version will support full mathematical syntax:
```rust
expr!(2*x + 3*y^2 - sin(x)/cos(x))  // Future goal
```

Until then, use parentheses for grouping and explicit helpers for complex cases.

### Gradual Migration Strategy (Enforce During Code Modifications)

When touching existing code, actively migrate to macro usage:

**Priority 1: Always Migrate (No Exceptions)**

ALWAYS replace direct Symbol constructors with macros when modifying code:

```rust
// ❌ Find and replace these ALWAYS:
Symbol::new("x")          → symbol!(x)
Symbol::scalar("x")       → symbol!(x)
Symbol::matrix("A")       → symbol!(A; matrix)
Symbol::operator("p")     → symbol!(p; operator)
Symbol::quaternion("i")   → symbol!(i; quaternion)

// ❌ Replace bulk creation (old approach):
let x = Symbol::new("x");
let y = Symbol::new("y");
let z = Symbol::new("z");

// ✅ With individual symbol!() calls (current):
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// ✅ With types (current - fully supported):
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let p = symbol!(p; operator);

// ✅ FUTURE - Once symbols!() macro is implemented (Wave 9):
// let (x, y, z) = symbols!("x y z");
// let (A, B, C) = symbols!("A B C"; matrix);
```

**Priority 2: Migrate When Touching the Code**

When modifying existing code, migrate simple expressions to macros:

```rust
// Old verbose patterns:
Expression::add(vec![
    Expression::symbol(Symbol::new("x")),
    Expression::symbol(Symbol::new("y"))
])

// ✅ New clean pattern:
expr!(x + y)
```

```rust
// Old:
Expression::pow(
    Expression::symbol(Symbol::new("x")),
    Expression::integer(2)
)

// ✅ New:
expr!(x ^ 2)
```

```rust
// Old:
Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))])

// ✅ New:
expr!(sin(x))
```

**Migration Guidelines:**

1. **Always migrate Priority 1** - No exceptions for Symbol::new() replacement
2. **Migrate Priority 2 when touching code** - Simple expressions become clearer with macros
3. **Don't migrate if it makes code less clear** - Readability always wins
4. **Use explicit grouping** - When in doubt, add parentheses: `expr!((2*x) + (3*y))`
5. **Keep runtime variables explicit** - Don't use macros for loop variables or runtime data
6. **Test after migration** - Run relevant tests to ensure behavior unchanged

**Example Migration:**

```rust
// Before:
let derivative = Expression::mul(vec![
    Expression::integer(2),
    Expression::symbol(Symbol::new("x"))
]);

// After:
let derivative = expr!(2 * x);
```

### Critical Migration Pitfalls (Learn from Experience)

**These are common errors discovered during macro migration. Avoid them!**

#### 1. **Runtime Variables Cannot Use Macros**

```rust
// ❌ DOES NOT COMPILE - 'i' is a runtime variable
for i in 0..10 {
    let expr = expr!(i);  // Error: macro sees token "i", not the value
}

// ✅ CORRECT - Use explicit API
for i in 0..10 {
    let expr = Expression::integer(i);
}

// ❌ DOES NOT COMPILE - 'point' is a variable
let point = 42;
let expr = expr!(point);  // Creates Symbol::new("point"), not integer 42!

// ✅ CORRECT
let point = 42;
let expr = Expression::integer(point);
```

**Why:** Macros expand at compile time and see **tokens**, not **values**. The identifier `i` or `point` is just a name to the macro, not the runtime value it will hold.

**Rule:** If the value comes from a variable, loop, or conditional → use explicit API

#### 2. **Cannot Nest Macro Calls**

```rust
// ❌ DOES NOT COMPILE - Nested expr!() calls
let expr = expr!(add: expr!(2 * x), expr!(4));

// ✅ CORRECT - Use direct patterns
let expr = expr!(add: (2 * x), 4);

// ✅ ALSO CORRECT - Use intermediate variables
let term1 = expr!(2 * x);
let term2 = expr!(4);
let expr = expr!(add: term1, term2);
```

**Why:** The macro pattern matcher sees `expr!()` as tokens, not as expressions to evaluate first.

**Rule:** Never nest `expr!()` inside `expr!()` arguments

#### 3. **Variable Names vs Values Confusion**

```rust
let a_val = 12;
let b_val = 18;

// ❌ WRONG - Creates Symbol::new("a_val") and Symbol::new("b_val")
let a = expr!(a_val);
let b = expr!(b_val);

// ✅ CORRECT - Use values directly
let a = Expression::integer(a_val);
let b = Expression::integer(b_val);

// ✅ ALSO CORRECT - If you actually want symbols named 'a' and 'b'
let a = symbol!(a);
let b = symbol!(b);
```

**Rule:** The macro sees **names**, not **values**. If it's a variable holding a value → explicit API.

### Examples of Good Macro Usage

```rust
// ✅ Excellent - scalar symbols (default, commutative)
let x = symbol!(x);
let theta = symbol!(theta);

// ✅ Excellent - matrix symbols (noncommutative) - Wave 1 Complete
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// ✅ Excellent - operator symbols (noncommutative) - Wave 1 Complete
let p = symbol!(p; operator);      // Momentum operator
let x_op = symbol!(x; operator);   // Position operator
let H = symbol!(H; operator);      // Hamiltonian

// ✅ Excellent - quaternion symbols (noncommutative) - Wave 1 Complete
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);

// ✅ Excellent - current approach for multiple symbols
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// ✅ FUTURE - bulk symbol creation (after Wave 9 implementation):
// let (x, y, z) = symbols!("x y z");              // All scalars (default)
// let (r, theta, phi) = symbols!("r theta phi");  // Spherical coordinates
// let (A, B, C) = symbols!("A B C"; matrix);      // All matrix symbols
// let (p, x, h) = symbols!("p x h"; operator);    // All operator symbols
// let (i, j, k) = symbols!("i j k"; quaternion);  // All quaternion symbols

// ✅ Excellent - simple expressions are very readable with explicit grouping
let quadratic = expr!((a*(x^2)) + (b*x) + c);
let trig_identity = expr!((sin(x)^2) + (cos(x)^2));
let derivative = expr!(2 * x);

// ✅ Good - test cases are immediately understandable
#[test]
fn test_derivative_power_rule() {
    let x = symbol!(x);
    let f = expr!(x ^ 3);
    let df = f.derivative(&x, 1);
    assert_eq!(df, expr!(3 * (x ^ 2)));
}

// ✅ Good - documentation examples are clean
/// # Examples
/// ```rust
/// let x = symbol!(x);
/// let expr = expr!(sin(x) / cos(x));  // tan(x)
/// let simplified = expr.simplify();
/// assert_eq!(simplified, expr!(tan(x)));
/// ```

// ✅ Good - explicit grouping for complex expressions
let complex = expr!((2*x + 1) * (3*y - 2));

// ✅ Good - multi-term with direct patterns (no nesting)
let polynomial = expr!(add: (x ^ 3), (-2 * (x ^ 2)), (3 * x), -5);

// ❌ Avoid - nested expr!() calls don't work
let polynomial = expr!(add: expr!(x ^ 3), expr!(-2 * (x ^ 2)));  // Won't compile!

// ❌ Avoid - runtime variables in macros
let programmatic = {
    let mut terms = Vec::new();
    for i in 0..n {
        terms.push(expr!(i));  // Won't work! 'i' is runtime variable
    }
    Expression::add(terms)
};

// ✅ Better - use explicit API for programmatic construction
let programmatic = {
    let mut terms = Vec::new();
    for i in 0..n {
        terms.push(Expression::mul(vec![
            Expression::integer(coefficients[i]),
            Expression::pow(symbol!(x), Expression::integer(i))
        ]));
    }
    Expression::add(terms)
};
```

### Summary: Macro Priority Enforcement

**When reviewing or writing code, follow this checklist:**

1. ✅ **Priority 1 (MANDATORY)**: All direct Symbol constructors replaced with `symbol!()` macro
   - Replace `Symbol::new()` → `symbol!(x)`
   - Replace `Symbol::scalar()` → `symbol!(x)`
   - Replace `Symbol::matrix()` → `symbol!(A; matrix)`
   - Replace `Symbol::operator()` → `symbol!(p; operator)`
   - Replace `Symbol::quaternion()` → `symbol!(i; quaternion)`

2. ✅ **Priority 2 (RECOMMENDED)**: Simple expressions use `expr!()` macro

3. ✅ **Priority 3 (APPROPRIATE)**: Complex/programmatic cases use explicit API

4. ✅ **No runtime variables in macros**: Loop variables and runtime data use explicit API

5. ✅ **No nested macro calls**: Use intermediate variables or direct patterns

**Red Flags (Reject During Code Review):**
- ❌ Any `Symbol::new()` call found in code
- ❌ Any `Symbol::scalar()`, `Symbol::matrix()`, `Symbol::operator()`, or `Symbol::quaternion()` call in application code
- ❌ Simple expressions like `x + y` using verbose `Expression::add(vec![...])`
- ❌ Runtime variables passed to macros: `expr!(i)` in a loop
- ❌ (FUTURE) Bulk symbol creation without `symbols!()` macro once it's implemented (Wave 9)

**Exception**: Direct Symbol constructors (`Symbol::scalar()`, etc.) are ONLY allowed in:
- Macro implementations (`macros/expressions.rs`)
- Test code that specifically tests Symbol constructors
- Internal library code that has legitimate reasons (must be documented)

**This is non-negotiable**. The macro system is designed to prevent errors and improve readability. When you see violations, fix them immediately during any code modification.

---

## Architecture

### Workspace Structure

```
mathhook/
├── mathhook-core/       # Core mathematical engine
├── mathhook/            # High-level user API with ergonomic macros
├── mathhook-python/     # Python bindings (PyO3)
├── mathhook-node/       # Node.js bindings (NAPI)
└── mathhook-benchmarks/ # Performance benchmarks (Criterion)
```

### Core Module Organization (mathhook-core)

**Module Size Limit**: Maximum 500 lines per file. Split larger modules into focused sub-modules.

```
src/
├── core/                 # Foundational types (Expression, Symbol, Number)
│   ├── expression/       # Central Expression enum (32-byte target)
│   ├── number.rs         # 16-byte number representation
│   ├── symbol.rs         # Variable symbols
│   ├── constants.rs      # Mathematical constants (π, e, etc.)
│   ├── arena.rs          # Memory arena for bulk allocation
│   └── performance/      # SIMD operations
├── parser/               # Multi-format parsing (LaTeX, Wolfram, standard)
│   ├── grammar.lalrpop   # LALRPOP grammar (generates grammar.rs)
│   ├── lexer/            # Tokenization and implicit multiplication
│   └── cache.rs          # Parse caching
├── functions/            # Universal Function Intelligence System
│   ├── elementary/       # sin, cos, exp, log, etc.
│   ├── special/          # gamma, zeta, bessel, etc.
│   ├── number_theory/    # factorial, gcd, etc.
│   └── polynomials/      # Polynomial functions
├── algebra/              # Algebraic operations (simplify, expand, factor)
├── calculus/             # Derivatives and integrals
│   ├── derivatives/      # Symbolic differentiation
│   └── integrals/        # Symbolic and numeric integration
├── solvers/              # Equation solving (linear, quadratic, polynomial, systems)
├── matrix/               # Matrix operations and decomposition
├── educational/          # Step-by-step explanations
├── formatter/            # Output formatting (LaTeX, human-readable)
├── simplify/             # Simplification strategies and zero detection
└── serialize/            # Serialization support
```

### Hybrid API Design Philosophy

MathHook provides two complementary API styles:

1. **Expression-centric API** (functional, method chaining):
```rust
let result = Expression::add(vec![
    Expression::integer(2),
    Expression::integer(3)
]).simplify();
```

2. **Solver object API** (stateful, configuration-driven):
```rust
let mut solver = MathSolver::new()
    .with_precision(1e-10)
    .with_max_iterations(1000);
let x = symbol!(x);
let solutions = solver.solve(&equation, &x);
```

Choose the appropriate style for the use case. Don't force one pattern where the other is more natural.

---

## Performance Optimization Guidelines

### Hot Path Identification

Profile before optimizing:
```bash
cargo bench
cargo flamegraph --bench benchmark_name
```

**Performance Profiling Requirements:**

1. **Benchmarking Tools**:
   - Use Criterion for microbenchmarks (`cargo bench`)
   - Use `cargo flamegraph` for hot path identification
   - Use `perf` on Linux for CPU profiling
   - Use `heaptrack` or `valgrind --tool=massif` for memory profiling

2. **Performance Targets**:
   - Compare against SymPy (Python): Should be 10-100x faster
   - Compare against Symbolica (Rust): Should be competitive (within 2x)
   - Parse speed: >100K expressions/second for simple expressions
   - Simplification: <1ms for expressions with <100 nodes
   - Expression creation: <100ns (allocation overhead)

3. **What to Benchmark**:
   - Core operations: `add`, `mul`, `pow`, `simplify`
   - Parser throughput with realistic inputs
   - Function evaluation (both symbolic and numerical)
   - Matrix operations at various sizes
   - Derivative and integral computation

4. **Regression Prevention**:
   - Add benchmark for every performance-critical change
   - CI should fail if performance regresses >10% without justification
   - Track performance over time (benchmark history)

5. **Optimization Strategy**:
   - Profile first, optimize second
   - Measure impact of every optimization
   - Document optimization rationale in comments

### Memory Layout Constraints

1. **Expression**: Target 32 bytes (hard constraint for cache-line optimization)
   - Current size: Check with `std::mem::size_of::<Expression>()`
   - If modifications exceed 32 bytes, discuss architectural changes first

2. **Number**: Exactly 16 bytes (current, do not modify)
   - Supports: integers, rationals, floats
   - Tagged union with discriminant

### Optimization Techniques

1. **Arena Allocation**: Use for bulk expression allocation
   - Located in `core/arena.rs`
   - Use when creating many short-lived expressions

2. **SIMD Operations**: Available in `core/performance/`
   - Use for vectorizable arithmetic on arrays
   - Test with both AVX2 and SSE fallbacks

3. **Inlining**: Use `#[inline]` for:
   - Small functions (<10 lines)
   - Functions called in hot loops
   - Getters and simple operations

4. **Compile-Time Optimization**:
   - Use `const fn` where possible for compile-time expression evaluation
   - Enables zero-cost symbolic constants at compile time
   - Example: `const DERIVATIVE: Expression = expr!(2 * x);` (future goal)

5. **Avoid Allocations**:
   - Prefer `&[T]` over `Vec<T>` in function signatures when not modifying
   - Consider `SmallVec` for collections with known small size (<8 elements), but benchmark first—for some workloads `Vec` is faster
   - Reuse buffers when possible

### Benchmarking Requirements

Add benchmarks for:
- New core operations (derivatives, integrals, solving)
- Modifications to hot paths
- Parser changes

Location: `crates/mathhook-benchmarks/benches/`

---

## Testing Strategy

### Test Categories

1. **Unit Tests**: In each module (target 100% for core mathematical functionality)
2. **Integration Tests**: In `tests/` directory
3. **Doctests**: ALL public API functions must have working examples
4. **Property Tests**: Use `proptest` for algebraic properties (commutativity, associativity, etc.)
5. **Benchmarks**: For performance-critical paths

### Testing Philosophy

**DO**:
- Test edge cases: zero, infinity, undefined, complex numbers
- Test mathematical properties: `a + b == b + a`, `(a * b) * c == a * (b * c)`
- Test domain boundaries: sqrt(-1), log(0), division by zero
- Test numerical stability:
  - Condition numbers for matrix operations
  - Catastrophic cancellation: `(x + ε) - x` for small ε
  - Accuracy degradation with repeated operations
  - Overflow/underflow in intermediate calculations
- Test against SymPy/Symbolica for correctness validation
- Use meaningful test names: `test_quadratic_solver_handles_complex_roots`
- Test both success and failure cases
- Test error paths as thoroughly as success paths

**DON'T**:
- Test implementation details (test behavior, not internals)
- Write brittle tests that break with refactoring
- Test trivial getters/setters
- Aim for coverage metrics over meaningful validation

### Test Naming Convention

```rust
#[test]
fn test_<operation>_<scenario>_<expected_outcome>() {
    // Example: test_derivative_of_polynomial_returns_correct_result
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p mathhook-core

# Specific test
cargo test test_quadratic_solver

# Don't stop on first failure
cargo test --no-fail-fast

# With output
cargo test -- --nocapture

# Doctests only
cargo test --doc

# Release mode (for performance-sensitive tests)
cargo test --release
```

---

## Building and Development Commands

### Build Commands

```bash
# Full workspace build
cargo build

# Release build (optimized)
cargo build --release

# Specific crate
cargo build -p mathhook-core

# Check without building (fast)
cargo check

# Check all targets
cargo check --all-targets
```

### Parser Regeneration

The parser uses LALRPOP (LR(1) parser generator). When modifying the grammar:

```bash
# Option 1: Let cargo build handle it
cargo build -p mathhook-core

# Option 2: Manual regeneration (see errors immediately)
lalrpop crates/mathhook-core/src/parser/grammar.lalrpop

# The generated parser will be at:
# crates/mathhook-core/src/parser/grammar.rs
```

**CRITICAL**: After parser changes, always test:
```bash
cargo test -p mathhook-core parser
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy

# Clippy in CI mode (fail on warnings)
cargo clippy -- -D warnings
```

---

## Development Workflow

### When Adding New Features

1. **Design First**: Consider architectural impact before coding
2. **Write Documentation**: Document the API with examples (TDD for docs)
3. **Implement**: Write the feature
4. **Verify Doctests**: `cargo test --doc`
5. **Add Unit Tests**: Cover edge cases
6. **Add Benchmarks**: If performance-critical
7. **Verify No Regressions**: Run full test suite

### When Modifying the Parser

1. **Edit**: `crates/mathhook-core/src/parser/grammar.lalrpop`
2. **Regenerate**: `lalrpop crates/mathhook-core/src/parser/grammar.lalrpop` OR `cargo build -p mathhook-core`
3. **Test**: `cargo test -p mathhook-core parser`
4. **Manual Verification**: Test implicit multiplication edge cases
   - `2x` → `2 * x`
   - `(a)(b)` → `a * b`  
   - `2(x+1)` → `2 * (x + 1)`
   - `sin(x)cos(x)` → `sin(x) * cos(x)`

### When Adding Mathematical Functions

1. **Implement Intelligence**: Add to appropriate module in `functions/`
   - Elementary: `functions/elementary/`
   - Special: `functions/special/`
   - Number theory: `functions/number_theory/`
   - Polynomials: `functions/polynomials/`

2. **Register**: Add to `UniversalFunctionRegistry`

3. **Implement Methods**:
   - `evaluate()`: Numerical evaluation with SIMD optimization
   - `properties()`: O(1) lookup for mathematical properties
   - `explain()`: Educational explanation generation
   - `simplify()`: Simplification rules

4. **Add Tests**: Cover all mathematical properties

5. **Document**: Include mathematical definition and examples

6. **Verify Against SymPy**: Ensure correctness matches SymPy behavior

### When Solving Mathematical Bugs

1. **Reproduce**: Create a minimal test case that fails
2. **Validate Expected Behavior**: Check SymPy/Symbolica for correct answer
3. **Identify Root Cause**: Don't guess; use debugging and tests
4. **Fix**: Implement correction
5. **Prevent Regression**: Add test case to suite
6. **Verify Broadly**: Run full test suite to ensure no side effects

---

## Session Management

**Development notes are stored in** `.mathhook_sessions/`

### Document Precedence (CRITICAL)

**CLAUDE.md is the authoritative source.** If you encounter contradictions:

1. **CLAUDE.md always wins** - Follow the rules and patterns in this document
2. **Flag the conflict** - Tell me: "I found a conflict between CLAUDE.md and [session note X]. CLAUDE.md says [Y], but the session note says [Z]. I'm following CLAUDE.md."
3. **Don't waste time** - Don't try to reconcile or merge contradicting information; just follow CLAUDE.md
4. **Suggest cleanup** - After flagging, suggest: "Should I update [session note] to match CLAUDE.md?"

### When to Update Session Notes

When you complete significant work or discover important patterns/decisions:
1. Add findings to appropriate session note
2. Keep notes factual and concise (what was done, what was learned)
3. **Do not duplicate CLAUDE.md content** - Session notes are for specific implementation details, not general rules

### When to Update CLAUDE.md

Update this file when:
- A new architectural pattern emerges that should be standard
- A new constraint is discovered (performance, mathematical correctness)
- You repeatedly encounter the same mistake or confusion
- A critical dependency or build process changes
- You discover a better way to organize or explain something
- A rule in a session note should become a universal rule

**Tell me immediately when you update this file and explain why.**

### Session Notes Purpose

Session notes capture:
- Specific implementation decisions for features
- Debugging discoveries and solutions
- Performance profiling results
- Temporary workarounds (that should eventually be removed)
- Context for "why we did X instead of Y"

Session notes do NOT replace CLAUDE.md rules.

---

## Current Development Focus

Active areas of development:
- Enhanced function intelligence system
- Pattern matching and smart evaluation  
- System equation solving
- Advanced simplification strategies
- Macro system improvements (operator parsing)

Check `.mathhook_sessions/` for detailed notes on specific development sessions.

---

## Versioning and Compatibility

**Critical for Users**: Breaking changes must be communicated and managed carefully.

### Semantic Versioning Strategy

MathHook follows [Semantic Versioning 2.0.0](https://semver.org/):

1. **MAJOR version** (e.g., 1.0.0 → 2.0.0):
   - Breaking API changes
   - Mathematical correctness fixes that change output format
   - Removal of deprecated features
   - Changes to serialization format (expressions can't be deserialized from old version)

2. **MINOR version** (e.g., 1.0.0 → 1.1.0):
   - New features (backward-compatible)
   - New function support
   - Performance improvements
   - Non-breaking API additions

3. **PATCH version** (e.g., 1.0.0 → 1.0.1):
   - Bug fixes (including mathematical correctness bugs)
   - Documentation improvements
   - Performance optimizations (non-breaking)

### Mathematical Correctness Fixes

**Special Case**: Fixing a mathematical bug that changes output format:

- If the old behavior was **mathematically incorrect**, this is a MAJOR version bump
- Document the fix prominently in CHANGELOG
- Provide migration guide if possible
- Example: If `sqrt(4)` incorrectly returned `±2` instead of `2`, fixing this is a major version change

### Serialization Compatibility

**Critical for Data Persistence**:

1. **Forward Compatibility**: New versions should deserialize old formats
2. **Version Tagging**: All serialized expressions include version metadata
3. **Migration Path**: Provide tools to migrate serialized data between major versions
4. **Format Stability**: Within a major version, serialization format is stable

### Deprecation Policy

1. **Deprecation Period**: Features marked `#[deprecated]` for at least one minor version before removal
2. **Documentation**: Clear migration path in deprecation notice
3. **Warnings**: Compiler warnings guide users to new API

### API Stability Guarantees

1. **Public API** (`pub` items in `lib.rs`):
   - Follows semantic versioning strictly
   - Breaking changes only in major versions

2. **Internal API** (non-`pub` items):
   - No stability guarantee
   - Can change in minor versions

3. **Experimental Features** (behind feature flags):
   - Marked as unstable in documentation
   - Can change in minor versions
   - Stabilized in major versions

### Backward Compatibility Testing

- Maintain test suite that validates behavior across versions
- Test deserialization of expressions from previous versions
- Document known incompatibilities in CHANGELOG

---

## When to Update This Document

Update CLAUDE.md when:
- A new architectural pattern emerges
- A new constraint is discovered (performance, mathematical correctness)
- You repeatedly encounter the same mistake or confusion
- A new critical dependency is added
- Build/test processes change
- You discover a better way to organize or explain something

**Tell me immediately when you update this file and explain why.**
- To build the parser alone when you're working on it, you must use only "lalrpop crates/mathhook-core/src/parser/grammar.lalrpop" command not cargo build