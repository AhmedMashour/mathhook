# MATHHOOK HYBRID API ARCHITECTURE - LOCKED DESIGN

**Date**: September 19, 2025  
**Status**: LOCKED - Ready for Implementation  
**Architecture**: Domain-Focused Hybrid API  

## EXECUTIVE SUMMARY

This document defines the complete architecture for MathHook's multi-language ecosystem with Python and Node.js bindings. The design follows a **hybrid approach** combining Expression-centric mathematical operations with separate objects for stateful operations, optimized for performance following Rust Performance Book principles.

## WORKSPACE STRUCTURE

### Domain-Focused Crate Organization
```
mathhook/                           # Workspace root
├── Cargo.toml                      # Workspace definition
├── README.md                       # Main project documentation
├── .mathhook_sessions/             # Architecture documentation
├── crates/
│   ├── mathhook-core/              # Pure math engine (Expression + Solvers)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs              # Core exports
│   │   │   ├── expression/         # Complete Expression system
│   │   │   │   ├── mod.rs
│   │   │   │   ├── types.rs        # All 18 expression variants
│   │   │   │   ├── constructors.rs # All creation methods
│   │   │   │   ├── operations.rs   # Mathematical operations
│   │   │   │   ├── introspection.rs # Type checking, access
│   │   │   │   └── conversion.rs   # Format conversion
│   │   │   ├── number/             # Optimized number system
│   │   │   │   ├── mod.rs
│   │   │   │   ├── types.rs        # SmallInt, BigInt, Rational, Float
│   │   │   │   └── arithmetic.rs   # Fast operations
│   │   │   ├── algebra/            # Mathematical operations
│   │   │   │   ├── mod.rs
│   │   │   │   ├── simplify.rs     # Core simplification
│   │   │   │   ├── factor.rs       # Factorization
│   │   │   │   ├── expand.rs       # Expression expansion
│   │   │   │   ├── gcd.rs          # GCD operations
│   │   │   │   ├── rational.rs     # Rational expressions
│   │   │   │   ├── calculus.rs     # Derivatives, integrals
│   │   │   │   └── matrix.rs       # Matrix operations
│   │   │   ├── solver/             # Equation solving
│   │   │   │   ├── mod.rs
│   │   │   │   ├── core.rs         # MathSolver
│   │   │   │   ├── linear.rs       # Linear equations
│   │   │   │   ├── quadratic.rs    # Quadratic equations
│   │   │   │   ├── polynomial.rs   # Higher-order polynomials
│   │   │   │   └── systems.rs      # System of equations
│   │   │   ├── educational/        # Educational features
│   │   │   │   ├── mod.rs
│   │   │   │   ├── teaching.rs     # TeachingSolver
│   │   │   │   ├── steps.rs        # Step-by-step explanations
│   │   │   │   └── difficulty.rs   # Difficulty assessment
│   │   │   ├── symbol.rs           # Symbol handling
│   │   │   └── api.rs              # Binding-safe wrapper API
│   │   └── benches/                # Performance benchmarks
│   ├── mathhook-parser/            # Multi-format parsing engine
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs              # Parser exports
│   │   │   ├── universal.rs        # Multi-format parser
│   │   │   ├── latex.rs            # LaTeX-specific parsing
│   │   │   ├── wolfram.rs          # Wolfram-specific parsing
│   │   │   ├── simple.rs           # Simple math notation
│   │   │   ├── formatter.rs        # Output formatting
│   │   │   └── api.rs              # Binding-friendly parser API
│   │   └── tests/                  # Parser-specific tests
│   ├── mathhook-python/            # Python bindings (PyO3)
│   │   ├── Cargo.toml              # PyO3 + Maturin dependencies
│   │   ├── pyproject.toml          # Python packaging (Maturin)
│   │   ├── src/
│   │   │   ├── lib.rs              # PyO3 module definition
│   │   │   ├── expression.rs       # Python Expression wrapper
│   │   │   ├── solver.rs           # Python Solver wrappers
│   │   │   ├── parser.rs           # Python Parser wrapper
│   │   │   └── types.rs            # Python type conversions
│   │   ├── python/
│   │   │   └── mathhook/
│   │   │       ├── __init__.py     # Python package
│   │   │       ├── types.py        # Python type hints
│   │   │       └── utils.py        # Python utilities
│   │   └── tests/                  # Python integration tests
│   ├── mathhook-node/              # Node.js bindings (NAPI-RS)
│   │   ├── Cargo.toml              # NAPI-RS dependencies
│   │   ├── package.json            # NPM configuration
│   │   ├── src/
│   │   │   ├── lib.rs              # NAPI module definition
│   │   │   ├── expression.rs       # JS Expression wrapper
│   │   │   ├── solver.rs           # JS Solver wrappers
│   │   │   ├── parser.rs           # JS Parser wrapper
│   │   │   └── types.rs            # JS type conversions
│   │   ├── index.d.ts              # TypeScript definitions
│   │   └── __test__/               # Node.js tests
│   └── mathhook/                   # Main Rust crate
│       ├── Cargo.toml              # Rust-specific features
│       ├── src/
│       │   ├── lib.rs              # Re-exports + prelude
│       │   ├── macros/             # Rust-only macros
│       │   │   ├── mod.rs
│       │   │   ├── simple.rs       # expr!, symbol!, const_expr!
│       │   │   ├── parsing.rs      # parse!, to_format!
│       │   │   └── calculus.rs     # calculus!
│       │   └── prelude.rs          # Convenient imports
│       └── examples/               # Rust usage examples
```

## EXPOSED API SURFACE

### Primary Objects (3 Total)

#### 1. Expression (40+ Methods)
```rust
impl Expression {
    // ========== CREATION (Static Methods) ==========
    pub fn parse(input: &str) -> Result<Self, String>
    pub fn parse_latex(latex: &str) -> Result<Self, String>
    pub fn parse_wolfram(wolfram: &str) -> Result<Self, String>
    
    pub fn symbol(name: &str) -> Self
    pub fn number(value: f64) -> Self
    pub fn integer(value: i64) -> Self
    pub fn rational(num: i64, den: i64) -> Self
    pub fn complex(real: Self, imag: Self) -> Self
    pub fn matrix(rows: Vec<Vec<Self>>) -> Self
    pub fn equation(left: Self, right: Self) -> Self
    pub fn set(elements: Vec<Self>) -> Self
    pub fn interval(start: Self, end: Self, start_inc: bool, end_inc: bool) -> Self
    
    // Constants
    pub fn pi() -> Self
    pub fn e() -> Self
    pub fn i() -> Self
    pub fn infinity() -> Self
    
    // ========== MATHEMATICAL OPERATIONS (Chainable) ==========
    // Move semantics for performance
    pub fn add(self, other: Self) -> Self
    pub fn subtract(self, other: Self) -> Self
    pub fn multiply(self, other: Self) -> Self
    pub fn divide(self, other: Self) -> Self
    pub fn power(self, exponent: Self) -> Self
    
    // Algebra operations (pure mathematical transforms)
    pub fn simplify(self) -> Self
    pub fn advanced_simplify(self) -> Self
    pub fn factor(self) -> Self
    pub fn expand(self) -> Self
    pub fn rationalize(self) -> Self
    pub fn collect_terms(self) -> Self
    
    // Borrowed operations (when you need to keep original)
    pub fn add_ref(&self, other: &Self) -> Self
    pub fn simplify_ref(&self) -> Self
    pub fn factor_ref(&self) -> Self
    
    // Calculus operations (pure mathematical transforms)
    pub fn derivative(self, variable: &str) -> Self
    pub fn derivative_nth(self, variable: &str, order: u32) -> Self
    pub fn integral(self, variable: &str) -> Self
    pub fn definite_integral(self, variable: &str, start: Self, end: Self) -> Self
    pub fn limit(self, variable: &str, approach: Self) -> Self
    
    // ========== INTROSPECTION (Zero-Copy) ==========
    pub fn kind(&self) -> ExpressionKind
    pub fn variables(&self) -> &[String]        // Borrow, don't clone
    pub fn is_zero(&self) -> bool
    pub fn is_one(&self) -> bool
    pub fn is_equation(&self) -> bool
    pub fn is_matrix(&self) -> bool
    pub fn is_number(&self) -> bool
    pub fn is_polynomial(&self) -> bool
    
    // Specialized access
    pub fn matrix_dimensions(&self) -> Option<(usize, usize)>
    pub fn matrix_get(&self, row: usize, col: usize) -> Option<Self>
    pub fn function_name(&self) -> Option<&str>
    pub fn function_args(&self) -> Option<&[Expression]>
    pub fn equation_sides(&self) -> Option<(&Expression, &Expression)>
    
    // ========== FORMAT CONVERSION ==========
    pub fn to_string(&self) -> String
    pub fn to_latex(&self) -> String
    pub fn to_wolfram(&self) -> String
    pub fn to_json(&self) -> String
}
```

#### 2. MathSolver (8 Methods)
```rust
pub struct MathSolver {
    // Internal state: SmartEquationSolver, workspace allocation
}

impl MathSolver {
    pub fn new() -> Self
    pub fn with_capacity(size: usize) -> Self  // Pre-allocate workspace
    
    // Core solving (requires state/context)
    pub fn solve(&mut self, expression: &Expression, variable: &str) -> SolverResult
    pub fn solve_system(&mut self, equations: Vec<&Expression>) -> SolverResult
    pub fn solve_batch(&mut self, expressions: Vec<&Expression>, variable: &str) -> Vec<SolverResult>
    
    // Analysis (may need caching/state)
    pub fn analyze(&self, expression: &Expression) -> EquationType
    pub fn difficulty(&self, expression: &Expression) -> DifficultyLevel
    pub fn can_solve(&self, expression: &Expression) -> bool
}
```

#### 3. TeachingSolver (6 Methods)
```rust
pub struct TeachingSolver {
    // Internal state: MathSolver + educational context
}

impl TeachingSolver {
    pub fn new() -> Self
    
    // Educational solving (needs rich state)
    pub fn solve_with_steps(&mut self, expression: &Expression, variable: &str) -> EducationalResult
    pub fn explain_simplification(&self, expression: &Expression) -> StepExplanation
    pub fn explain_factorization(&self, expression: &Expression) -> StepExplanation
    pub fn explain_expansion(&self, expression: &Expression) -> StepExplanation
    pub fn get_hints(&self, expression: &Expression) -> Vec<String>
    pub fn assess_difficulty(&self, expression: &Expression) -> DifficultyLevel
}
```

### Supporting Data Types (8 Total)

#### Result Containers
```rust
#[derive(Debug, Clone)]
pub enum SolverResult {
    Single(Expression),
    Multiple(Vec<Expression>),
    NoSolution,
    InfiniteSolutions,
    Parametric(Vec<Expression>),
}

#[derive(Debug, Clone)]
pub struct EducationalResult {
    pub solution: SolverResult,
    pub explanation: StepExplanation,
    pub difficulty: DifficultyLevel,
    pub equation_type: EquationType,
    pub insights: Vec<String>,
    pub latex_input: String,
}

#[derive(Debug, Clone)]
pub struct StepExplanation {
    pub steps: Vec<Step>,
    pub summary: String,
    pub total_steps: usize,
    pub to_human_text(&self) -> String,
    pub to_json(&self) -> String,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub title: String,
    pub description: String,
    pub latex: String,
    pub rule_applied: String,
}
```

#### Classification Enums
```rust
#[repr(C)]  // C-compatible for efficient FFI
pub enum ExpressionKind {
    Number = 0, Symbol = 1, Addition = 2, Multiplication = 3,
    Power = 4, Function = 5, Complex = 6, Matrix = 7,
    Constant = 8, Equation = 9, Piecewise = 10, Set = 11,
    Interval = 12, Derivative = 13, Integral = 14, 
    Limit = 15, Sum = 16, Product = 17,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EquationType {
    Linear, Quadratic, Cubic, Quartic, System, 
    Transcendental, Constant, Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DifficultyLevel {
    Beginner,     // Linear equations
    Intermediate, // Quadratic, systems
    Advanced,     // Cubic, quartic
    Expert,       // Transcendental
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: Option<usize>,
    pub error_type: String,
}
```

### Optional Utilities (2 Total)

#### Advanced Parser Control
```rust
pub struct MathParser {
    // Stateful parsing with variable tracking
}

impl MathParser {
    pub fn new() -> Self
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError>
    pub fn parse_latex(&mut self, latex: &str) -> Result<Expression, ParseError>
    pub fn parse_wolfram(&mut self, wolfram: &str) -> Result<Expression, ParseError>
    pub fn detect_format(&self, input: &str) -> String
    pub fn get_variables(&self) -> Vec<String>
    pub fn clear_variables(&mut self)
}
```

#### Batch Formatting
```rust
pub struct MathFormatter;

impl MathFormatter {
    pub fn to_latex(expression: &Expression) -> String
    pub fn to_wolfram(expression: &Expression) -> String
    pub fn to_simple(expression: &Expression) -> String
    pub fn batch_format(expressions: Vec<&Expression>, format: &str) -> Vec<String>
}
```

## PERFORMANCE OPTIMIZATIONS

### Rust Performance Book Compliance

#### Memory Efficiency
- **32-byte Expression** optimization maintained
- **16-byte Number** compact representation
- **Move semantics** for mathematical operations (consume `self`)
- **Borrowed introspection** (`&[String]` instead of `Vec<String>`)
- **Pre-allocated workspaces** in solver objects

#### Zero-Cost Abstractions
- **Static dispatch** for all mathematical operations
- **Inline functions** for hot paths
- **C-compatible enums** for efficient FFI
- **Minimal wrapper overhead** in binding layer

#### Allocation Patterns
- **Single allocation** for parsing operations
- **Batch operations** for multiple expressions
- **Arena allocation** available for high-performance scenarios
- **Workspace reuse** in solver objects

## LANGUAGE BINDING PATTERNS

### Python Bindings (PyO3 + Maturin)
```python
# Expression-centric mathematical operations
expr = mathhook.Expression.parse("x^2 + 2*x + 1")
result = expr.simplify().factor()  # Method chaining

# Separate objects for stateful operations
solver = mathhook.MathSolver()
solution = solver.solve(expr, "x")

# Educational features
teacher = mathhook.TeachingSolver()
explanation = teacher.solve_with_steps(expr, "x")
print(explanation.to_human_text())

# NumPy integration
matrix_expr = mathhook.Expression.matrix([[1, 2], [3, 4]])
numpy_array = matrix_expr.to_numpy()  # Convert to NumPy
```

### Node.js Bindings (NAPI-RS)
```typescript
// TypeScript definitions
class Expression {
    static parse(input: string): Expression;
    static symbol(name: string): Expression;
    static number(value: number): Expression;
    
    simplify(): Expression;
    factor(): Expression;
    add(other: Expression): Expression;
    
    kind(): ExpressionKind;
    variables(): string[];
    toString(): string;
    toLatex(): string;
}

class MathSolver {
    constructor();
    solve(expression: Expression, variable: string): SolverResult;
    analyze(expression: Expression): EquationType;
}

// Usage
const expr = Expression.parse("x^2 + 1 = 0");
const simplified = expr.simplify().factor();

const solver = new MathSolver();
const solution = solver.solve(expr, "x");
```

### Rust Native (Full API + Macros)
```rust
// Macro convenience (Rust only)
let expr = expr!(x^2 + 2*x + 1);
let symbols = symbol!(x, y, z);

// Direct API (same as bindings)
let expr = Expression::parse("x^2 + 2*x + 1")?;
let result = expr.simplify().factor();

let mut solver = MathSolver::new();
let solution = solver.solve(&expr, "x");
```

## IMPLEMENTATION PHASES

### Phase 1: Core Restructuring (Week 1)
1. **Create workspace structure**
   - Set up `Cargo.toml` workspace with 5 crates
   - Define inter-crate dependencies
   - Establish version synchronization

2. **Extract mathhook-core**
   - Move all Expression variants and operations
   - Move all algebra operations (simplify, factor, expand, gcd, rational)
   - Move all solver implementations
   - Move educational features
   - Create binding-safe API layer

3. **Extract mathhook-parser**
   - Move parsing functionality
   - Clean up unused internal methods
   - Create simple API interface
   - Optimize for binding usage

### Phase 2: API Implementation (Week 2)
4. **Implement Expression methods**
   - Mathematical operations with move semantics
   - Zero-copy introspection methods
   - Format conversion methods
   - Performance-optimized constructors

5. **Implement Solver objects**
   - MathSolver with pre-allocated workspace
   - TeachingSolver with educational context
   - Efficient batch operations

6. **Design binding APIs**
   - C-compatible types and enums
   - Error handling strategy
   - Memory management patterns

### Phase 3: Python Bindings (Week 3)
7. **Set up PyO3 + Maturin**
   - Python package structure
   - PyO3 wrapper classes
   - Type conversion layer

8. **Implement Python wrappers**
   - Expression wrapper with method chaining
   - Solver wrapper classes
   - Rich educational features
   - NumPy integration

### Phase 4: Node.js Bindings (Week 4)
9. **Set up NAPI-RS**
   - NPM package structure
   - TypeScript definitions
   - Async patterns where beneficial

10. **Implement Node.js wrappers**
    - Expression wrapper with method chaining
    - Solver wrapper classes
    - Promise-based async operations

### Phase 5: Integration & Testing (Week 5)
11. **Cross-language testing**
    - Compatibility test suites
    - Performance benchmarks
    - Integration examples

12. **Documentation & Examples**
    - API documentation for all languages
    - Usage examples
    - Performance guides

## DEPENDENCY FLOW

```toml
# Workspace Cargo.toml
[workspace]
members = [
    "crates/mathhook-core",
    "crates/mathhook-parser", 
    "crates/mathhook-python",
    "crates/mathhook-node",
    "crates/mathhook",
]

# Dependency relationships:
# mathhook-python → mathhook-core + mathhook-parser
# mathhook-node   → mathhook-core + mathhook-parser  
# mathhook        → mathhook-core + mathhook-parser + macros
# mathhook-core   → (minimal external dependencies)
# mathhook-parser → mathhook-core
```

## KEY ARCHITECTURAL DECISIONS

### Expression-Centric Operations
- **Mathematical operations**: Belong on Expression (natural, chainable)
- **Format conversion**: Belongs on Expression (intrinsic property)
- **Introspection**: Belongs on Expression (zero-cost)
- **Move semantics**: Performance optimization for chaining

### Separate Objects for Stateful Operations
- **Solving**: Requires context, state, workspace allocation
- **Educational**: Needs rich context, explanation generation
- **Parsing**: May need variable tracking, format detection state

### Performance-First Design
- **Zero-cost abstractions**: All mathematical operations inline to direct calls
- **Minimal allocations**: Move semantics, borrowed introspection
- **Cache-friendly**: Maintain 32-byte Expression, 16-byte Number
- **Batch operations**: Efficient for multiple operations

### Binding-Friendly Design
- **Minimal surface**: 13 total types (3 primary objects)
- **C-compatible**: Enums with explicit discriminants
- **Clear ownership**: Move vs borrow patterns
- **Rich functionality**: All 18 expression variants supported

## SUCCESS METRICS

### API Usability
- **Single import**: Most operations through Expression
- **Natural chaining**: Mathematical operations flow intuitively
- **Rich features**: Educational, analysis, format conversion

### Performance Targets
- **Hot path optimization**: Mathematical operations zero-cost
- **Memory efficiency**: Minimal allocations in common operations
- **Binding overhead**: <5% performance cost vs native Rust

### Cross-Language Compatibility
- **Python**: Natural method chaining, NumPy integration
- **Node.js**: TypeScript support, async patterns
- **Rust**: Full macro ecosystem + direct API

---

**This architecture is LOCKED and ready for implementation.**

**Total Exposed Surface: 13 types (3 primary objects + 8 data types + 2 utilities)**

**Performance-optimized, binding-friendly, and feature-complete.**
