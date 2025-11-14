# Noncommutative Algebra API Reference

Complete API reference for MathHook's noncommutative algebra support.

## Symbol Creation API

### `symbol!(name)` - Create scalar symbol

Creates a scalar (commutative) symbol with the given name.

**Syntax**: `symbol!(identifier)`

**Returns**: `Symbol` with type `Scalar`

**Example**:
```rust
let x = symbol!(x);
let theta = symbol!(theta);
```

### `symbol!(name; type)` - Create typed symbol

Creates a symbol with specified type (matrix, operator, or quaternion).

**Syntax**: `symbol!(identifier; type_keyword)`

**Type keywords**:
- `matrix` - Matrix symbol (noncommutative)
- `operator` - Operator symbol (noncommutative)
- `quaternion` - Quaternion symbol (noncommutative)

**Returns**: `Symbol` with specified type

**Examples**:
```rust
let A = symbol!(A; matrix);        // Matrix
let p = symbol!(p; operator);      // Operator
let i = symbol!(i; quaternion);    // Quaternion
```

### `symbols![...]` - Create multiple symbols

Creates multiple symbols of the same type.

**Syntax**: `symbols![id1, id2, ... => type]`

**Returns**: `Vec<Symbol>`

**Examples**:
```rust
let scalars = symbols![x, y, z];                    // Default: scalar
let matrices = symbols![A, B, C => matrix];         // Matrices
let operators = symbols![p, x, h => operator];      // Operators
let quaternions = symbols![i, j, k => quaternion];  // Quaternions
```

## Symbol Type API

### `SymbolType` Enum

Defines the four symbol types:

```rust
pub enum SymbolType {
    Scalar,      // Commutative (default)
    Matrix,      // Noncommutative
    Operator,    // Noncommutative
    Quaternion,  // Noncommutative
}
```

### `Symbol::symbol_type()` - Get symbol type

Returns the type of a symbol.

**Signature**: `fn symbol_type(&self) -> SymbolType`

**Returns**: The symbol's type

**Example**:
```rust
let x = symbol!(x);
assert_eq!(x.symbol_type(), SymbolType::Scalar);

let A = symbol!(A; matrix);
assert_eq!(A.symbol_type(), SymbolType::Matrix);
```

### `Symbol::commutativity()` - Check commutativity

Returns whether the symbol is commutative.

**Signature**: `fn commutativity(&self) -> Commutativity`

**Returns**: `Commutative` or `Noncommutative`

**Example**:
```rust
let x = symbol!(x);
assert_eq!(x.commutativity(), Commutativity::Commutative);

let A = symbol!(A; matrix);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);
```

## Direct Constructors (Internal Use Only)

These constructors are available but `symbol!()` macro is strongly recommended:

### `Symbol::new(name)` - Create scalar symbol

**Signature**: `fn new(name: &str) -> Symbol`

**Returns**: Scalar symbol

**Note**: Use `symbol!(name)` macro instead in application code.

### `Symbol::scalar(name)` - Create scalar symbol

**Signature**: `fn scalar(name: &str) -> Symbol`

**Returns**: Scalar symbol

**Note**: Use `symbol!(name)` macro instead.

### `Symbol::matrix(name)` - Create matrix symbol

**Signature**: `fn matrix(name: &str) -> Symbol`

**Returns**: Matrix symbol

**Note**: Use `symbol!(name; matrix)` macro instead.

### `Symbol::operator(name)` - Create operator symbol

**Signature**: `fn operator(name: &str) -> Symbol`

**Returns**: Operator symbol

**Note**: Use `symbol!(name; operator)` macro instead.

### `Symbol::quaternion(name)` - Create quaternion symbol

**Signature**: `fn quaternion(name: &str) -> Symbol`

**Returns**: Quaternion symbol

**Note**: Use `symbol!(name; quaternion)` macro instead.

## Expression Creation API

Expressions can be created using constructors or operators.

### `Expression::symbol(sym)` - Create symbol expression

**Signature**: `fn symbol(sym: Symbol) -> Expression`

**Example**:
```rust
let x = symbol!(x);
let expr = Expression::symbol(x);
```

### `Expression::add(terms)` - Create addition

**Signature**: `fn add(terms: Vec<Expression>) -> Expression`

**Example**:
```rust
let x = symbol!(x);
let y = symbol!(y);
let sum = Expression::add(vec![
    Expression::symbol(x),
    Expression::symbol(y)
]);
```

### `Expression::mul(factors)` - Create multiplication

**Signature**: `fn mul(factors: Vec<Expression>) -> Expression`

**Note**: Order matters for noncommutative symbols.

**Example**:
```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
// A*B ≠ B*A in general
let ab = Expression::mul(vec![
    Expression::symbol(A),
    Expression::symbol(B)
]);
```

## Equation Solving API

### `MatrixEquationSolver::new()` - Create solver

**Signature**: `fn new() -> MatrixEquationSolver`

**Returns**: New solver instance

**Example**:
```rust
use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
let solver = MatrixEquationSolver::new();
```

### `solver.solve(equation, variable)` - Solve equation

**Signature**: `fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult`

**Parameters**:
- `equation`: Equation to solve (must equal zero)
- `variable`: Variable to solve for

**Returns**: `SolverResult` with solutions

**Example**:
```rust
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// A*X = B → A*X - B = 0
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B)]),
]);

let result = solver.solve(&equation, &X);
```

## LaTeX Formatting API

### `expr.to_latex(context)` - Format as LaTeX

**Signature**: `fn to_latex(&self, context: LaTeXContext) -> Result<String, FormattingError>`

**Parameters**:
- `context`: Formatting context (use `LaTeXContext::default()`)

**Returns**: LaTeX string or error

**Example**:
```rust
use mathhook_core::formatter::latex::LaTeXContext;

let A = symbol!(A; matrix);
let expr = Expression::symbol(A);
let latex = expr.to_latex(LaTeXContext::default()).unwrap();
// Output: "\mathbf{A}"
```

### LaTeX Output by Type

- **Scalar**: Standard notation (`x`, `\theta`)
- **Matrix**: Bold notation (`\mathbf{A}`, `\mathbf{B}`)
- **Operator**: Hat notation (`\hat{p}`, `\hat{x}`)
- **Quaternion**: Standard notation (`i`, `j`, `k`)

## Educational Messages API

### `MessageBuilder::new()` - Create message builder

**Signature**: `fn new(category, message_type, step) -> MessageBuilder`

**Parameters**:
- `category`: Message category (e.g., `NoncommutativeAlgebra`)
- `message_type`: Message type (e.g., `LeftMultiplyInverse`)
- `step`: Step number

**Example**:
```rust
use mathhook_core::educational::message_registry::{
    MessageBuilder, MessageCategory, MessageType
};

let msg = MessageBuilder::new(
    MessageCategory::NoncommutativeAlgebra,
    MessageType::LeftMultiplyInverse,
    0
).build();
```

## Error Handling

All operations that can fail return `Result` types:

- `Result<Expression, FormattingError>` - Formatting operations
- `Result<Expression, ParseError>` - Parsing operations
- `SolverResult` - Solving operations (may have no solution)

Handle errors appropriately:

```rust
match expr.to_latex(context) {
    Ok(latex) => println!("LaTeX: {}", latex),
    Err(e) => eprintln!("Error: {}", e),
}
```
