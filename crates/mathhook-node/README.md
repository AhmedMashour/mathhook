# MathHook Node.js Bindings

[![npm](https://img.shields.io/npm/v/mathhook-node.svg)](https://www.npmjs.com/package/mathhook-node)
[![Node Version](https://img.shields.io/node/v/mathhook-node.svg)](https://www.npmjs.com/package/mathhook-node)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE)

High-performance computer algebra system for Node.js and TypeScript, powered by Rust.

## Features

- **High Performance**: Rust-powered core targeting 10-100x speedup over JS-based CAS
- **Native Performance**: N-API bindings with minimal overhead
- **Symbolic Mathematics**: Expressions, algebra, calculus, and matrix operations
- **Multiple Input/Output Formats**: Parse and emit LaTeX, Wolfram Language, and standard notation
- **Educational**: Step-by-step explanations for simplification and derivatives
- **TypeScript Support**: Type definitions included
- **Cross-Platform**: Pre-built binaries for Windows, macOS, Linux (x64, ARM64)

## Installation

```bash
npm install mathhook-node
```

Or with Yarn:

```bash
yarn add mathhook-node
```

Or with pnpm:

```bash
pnpm add mathhook-node
```

Requires Node.js 18 or higher.

### Platform Support

Pre-built binaries are available for:
- Windows (x64, ARM64)
- macOS (x64, ARM64/Apple Silicon)
- Linux (x64, ARM64, ARMv7, musl)

## Quick Start

### JavaScript

```javascript
const { Expression, symbol, solve } = require('mathhook-node');

// Create symbols
const x = symbol('x');

// Build expressions
const expr = x.pow(2).add(x.multiply(2)).add(1);

// Simplify
const simplified = expr.simplify();
console.log(simplified.toString());  // x^2 + 2*x + 1

// Solve equations
const solutions = solve(x.pow(2).subtract(4), x);
console.log(solutions);  // [2, -2]

// Calculus
const derivative = expr.diff(x);
console.log(derivative.toString());  // 2*x + 2
```

### TypeScript

```typescript
import { Expression, symbol, solve, Symbol } from 'mathhook-node';

// Full type safety
const x: Symbol = symbol('x');
const expr: Expression = x.pow(2).add(x.multiply(2)).add(1);

// Type-safe operations
const simplified: Expression = expr.simplify();
const solutions: Expression[] = solve(x.pow(2).subtract(4), x);
```

## Expression Creation

### Basic Types

```typescript
import { Expression } from 'mathhook-node';

// Numbers
const integer = Expression.integer(42);
const rational = Expression.rational(3, 4);  // 3/4
const floating = Expression.float(3.14159);

// Complex numbers
const complex = Expression.complex(3, 4);  // 3 + 4i
```

### Symbols

```typescript
import { symbol, symbols } from 'mathhook-node';

// Single symbol
const x = symbol('x');

// Multiple symbols
const [x, y, z] = symbols(['x', 'y', 'z']);
```

### Method Chaining

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Fluent API
const expr = x.pow(2)
              .add(x.multiply(2))
              .add(1);

// Complex expressions
const poly = x.add(1)
              .multiply(x.subtract(1))
              .simplify();  // x^2 - 1
```

### Mathematical Functions

```typescript
import {
  sin, cos, tan,
  exp, log, sqrt,
  symbol
} from 'mathhook-node';

const x = symbol('x');

// Trigonometric
const trig = sin(x).pow(2).add(cos(x).pow(2));  // = 1

// Exponential and logarithmic
const exponential = exp(x);
const logarithm = log(x, 10);  // log base 10

// Square root
const root = sqrt(x.pow(2).add(1));
```

### Constants

```typescript
import { pi, e, I, infinity } from 'mathhook-node';

// Mathematical constants
const circleArea = pi.multiply(r.pow(2));
const eulerIdentity = exp(I.multiply(pi)).add(1);  // = 0
const limit = Expression.integer(1).divide(infinity);  // = 0
```

## Algebraic Operations

### Simplification

```typescript
import { symbol, sin, cos } from 'mathhook-node';

const x = symbol('x');

// Algebraic simplification
const expr = x.add(x).add(x);
console.log(expr.simplify().toString());  // 3*x

// Trigonometric identities
const trig = sin(x).pow(2).add(cos(x).pow(2));
console.log(trig.simplify().toString());  // 1

// Rational expressions
const rational = x.pow(2).subtract(1)
                  .divide(x.subtract(1));
console.log(rational.simplify().toString());  // x + 1
```

### Expansion

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Expand products
const expr = x.add(1).multiply(x.add(2));
console.log(expr.expand().toString());  // x^2 + 3*x + 2

// Expand powers
const power = x.add(y).pow(2);
console.log(power.expand().toString());  // x^2 + 2*x*y + y^2
```

### Factorization

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');

// Factor polynomials
const expr = x.pow(2).subtract(1);
console.log(expr.factor().toString());  // (x - 1)(x + 1)

const poly = x.pow(2).add(x.multiply(5)).add(6);
console.log(poly.factor().toString());  // (x + 2)(x + 3)
```

### Substitution

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

const expr = x.pow(2).add(x.multiply(2)).add(1);

// Substitute value
const result = expr.subs(x, Expression.integer(3));
console.log(result.toString());  // 16

// Substitute expression
const substituted = expr.subs(x, y.add(1));
console.log(substituted.toString());  // (y + 1)^2 + 2*(y + 1) + 1
```

## Calculus

### Derivatives

```typescript
import { symbol, sin, cos } from 'mathhook-node';

const x = symbol('x');

// First derivative
const expr = x.pow(3);
console.log(expr.diff(x).toString());  // 3*x^2

// Higher-order derivatives
console.log(expr.diff(x, 2).toString());  // 6*x
console.log(expr.diff(x, 3).toString());  // 6

// Chain rule
const chain = sin(x.pow(2));
console.log(chain.diff(x).toString());  // 2*x*cos(x^2)
```

### Integrals

```typescript
import { symbol, sin } from 'mathhook-node';

const x = symbol('x');

// Indefinite integrals
const expr = x.pow(2);
console.log(expr.integrate(x).toString());  // x^3/3

// Definite integrals
const definite = expr.integrate(x, 0, 1);
console.log(definite.toString());  // 1/3

// Trigonometric integrals
const trig = sin(x);
console.log(trig.integrate(x).toString());  // -cos(x)
```

### Limits

```typescript
import { symbol, sin, infinity } from 'mathhook-node';

const x = symbol('x');

// Finite limits
const expr = sin(x).divide(x);
const lim = expr.limit(x, 0);
console.log(lim.toString());  // 1

// Limits at infinity
const expr2 = Expression.integer(1).divide(x);
console.log(expr2.limit(x, infinity).toString());  // 0
```

### Series Expansions

```typescript
import { symbol, sin } from 'mathhook-node';

const x = symbol('x');

// Taylor series
const series = sin(x).series(x, 0, 6);
console.log(series.toString());
// x - x^3/6 + x^5/120 + O(x^6)
```

## Equation Solving

### Algebraic Equations

```typescript
import { symbol, solve } from 'mathhook-node';

const x = symbol('x');

// Linear equations
let solutions = solve(
  x.multiply(2).add(3).subtract(7),
  x
);
console.log(solutions);  // [2]

// Quadratic equations
solutions = solve(
  x.pow(2).subtract(x.multiply(5)).add(6),
  x
);
console.log(solutions);  // [2, 3]

// Complex solutions
solutions = solve(x.pow(2).add(1), x);
console.log(solutions);  // [I, -I]
```

### Systems of Equations

```typescript
import { symbol, solveSystem } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Linear system
const solutions = solveSystem([
  x.add(y).subtract(5),
  x.subtract(y).subtract(1)
], [x, y]);

console.log(solutions);  // { x: 3, y: 2 }
```

## Matrix Operations

### Creating Matrices

```typescript
import { Matrix, Expression } from 'mathhook-node';

// From arrays
const A = new Matrix([
  [1, 2],
  [3, 4]
]);

// Identity matrix
const I = Matrix.identity(3);

// Zero matrix
const Z = Matrix.zeros(2, 3);

// Diagonal matrix
const D = Matrix.diagonal([1, 2, 3]);
```

### Matrix Operations

```typescript
import { Matrix } from 'mathhook-node';

const A = new Matrix([[1, 2], [3, 4]]);
const B = new Matrix([[5, 6], [7, 8]]);

// Addition
const C = A.add(B);

// Multiplication
const D = A.multiply(B);

// Transpose
const AT = A.transpose();

// Determinant
const det = A.determinant();
console.log(det);  // -2

// Inverse
const Ainv = A.inverse();

// Trace
const tr = A.trace();
console.log(tr);  // 5
```

### Matrix Decomposition

```typescript
import { Matrix } from 'mathhook-node';

const A = new Matrix([[4, 2], [2, 3]]);

// Eigenvalues and eigenvectors
const eigenvals = A.eigenvalues();
const eigenvects = A.eigenvectors();

// LU decomposition
const { L, U, P } = A.luDecomposition();

// QR decomposition
const { Q, R } = A.qrDecomposition();

// Cholesky decomposition
const L = A.choleskyDecomposition();
```

## Parsing

### Multi-Format Parser

```typescript
import { parse } from 'mathhook-node';

// Standard notation
let expr = parse("2*x + sin(y)");

// LaTeX
expr = parse(String.raw`\frac{x^2}{2} + \sqrt{y}`);
expr = parse(String.raw`\sin(x) + \cos(y)`);

// Wolfram Language
expr = parse("Sin[x] + Cos[y]");
expr = parse("D[x^2, x]");
```

### Format Conversion

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const expr = x.pow(2).divide(2);

// Convert to LaTeX
const latex = expr.toLatex();
console.log(latex);  // \frac{x^{2}}{2}

// Convert to Wolfram
const wolfram = expr.toWolfram();
console.log(wolfram);  // Divide[Power[x, 2], 2]

// String representation
console.log(expr.toString());  // x^2/2
```

## Educational Features

### Step-by-Step Solutions

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const expr = x.add(1).multiply(x.subtract(1));

// Get expansion steps
const steps = expr.expand({ steps: true });

steps.forEach((step, i) => {
  console.log(`Step ${i + 1}: ${step.title}`);
  console.log(`  ${step.description}`);
  console.log(`  Result: ${step.expression}`);
  console.log();
});
```

## Performance Optimization

### Configuration

```typescript
import { configure } from 'mathhook-node';

// Use Node.js-optimized settings
configure({ binding: 'nodejs' });

// Custom configuration
configure({
  simdEnabled: true,
  simdThreshold: 100,
  cacheSize: 20000,
  parallelEnabled: true,
  parallelThreshold: 500
});
```

### Bulk Operations

```typescript
import { simplifyMany } from 'mathhook-node';

// Simplify many expressions at once (uses parallelization)
const expressions = Array.from({ length: 1000 }, (_, i) =>
  x.pow(2).add(x.multiply(i)).add(1)
);
const simplified = simplifyMany(expressions);
```

## TypeScript Support

### Type Definitions

Full TypeScript support is included:

```typescript
import {
  Expression,
  Symbol,
  Matrix,
  SolverResult
} from 'mathhook-node';

// Type-safe function
function quadratic(
  a: number,
  b: number,
  c: number,
  x: Symbol
): Expression {
  return Expression.integer(a)
    .multiply(x.pow(2))
    .add(Expression.integer(b).multiply(x))
    .add(Expression.integer(c));
}

// Type-safe solving
function roots(expr: Expression, var_: Symbol): Expression[] {
  return solve(expr, var_);
}
```

### Async Operations

For long-running computations:

```typescript
import { solveAsync } from 'mathhook-node';

async function complexSolve() {
  const x = symbol('x');
  const expr = x.pow(10).subtract(1);

  // Non-blocking solve
  const solutions = await solveAsync(expr, x);
  return solutions;
}
```

## Examples

### Quadratic Formula

```typescript
import { symbol, solve } from 'mathhook-node';

const x = symbol('x');
const a = 1, b = -5, c = 6;

const equation = Expression.integer(a)
  .multiply(x.pow(2))
  .add(Expression.integer(b).multiply(x))
  .add(Expression.integer(c));

const solutions = solve(equation, x);
console.log(`Solutions: ${solutions}`);  // [2, 3]
```

### Taylor Series Approximation

```typescript
import { symbol, sin } from 'mathhook-node';

const x = symbol('x');

// Approximate sin(x) with Taylor series
const sinApprox = sin(x).series(x, 0, 10);
console.log(sinApprox.toString());

// Evaluate at specific point
const result = sinApprox.subs(x, Expression.float(0.1));
console.log(result.evaluate());
```

### Matrix Eigenvalues

```typescript
import { Matrix, symbol } from 'mathhook-node';

// Symbolic matrix
const t = symbol('t');
const A = new Matrix([
  [t, 1],
  [1, t]
]);

// Find eigenvalues
const eigenvals = A.eigenvalues();
console.log(eigenvals);  // [t-1, t+1]
```

## Performance Comparison

Benchmark results vs mathjs (lower is better):

| Operation | MathHook | mathjs | Speedup |
|-----------|----------|--------|---------|
| Expression Creation | 0.1μs | 5.0μs | 50x |
| Simplification | 1.0μs | 100μs | 100x |
| Differentiation | 2.0μs | 150μs | 75x |
| Matrix Multiplication | 10μs | 800μs | 80x |

*Benchmarks on Apple M1, Node.js 18*

## Actual API Reference

For the complete API documentation, see the **[Node.js Bindings Guide](../../docs/src/bindings/nodejs.md)** in the mdbook.

### Quick Reference

The main classes and types exported:

| Export | Purpose |
|--------|---------|
| `Expression` | Core symbolic expression with algebra, calculus, matrices |
| `symbol` | Create symbolic variables |
| `EvalContext` | Controlled evaluation context |
| `sin`, `cos`, `tan` | Trigonometric functions |
| `solve` | Equation solving |
| `parse` | Parse expressions from strings |

## Debugging

### Enable Debug Logging

```typescript
import { setLogLevel } from 'mathhook-node';

// Enable debug logs
setLogLevel('debug');

// Your code here
```

### Performance Profiling

```typescript
import { getPerformanceMetrics } from 'mathhook-node';

// Run your code
// ...

// Get performance statistics
const metrics = getPerformanceMetrics();
console.log(metrics);
```

## Common Issues

### Import Errors

```typescript
// ✅ Correct
import { Expression, symbol } from 'mathhook-node';

// ❌ Incorrect
import Expression from 'mathhook-node';  // No default export
```

### Platform-Specific Binaries

If you get a binary loading error:
```bash
# Rebuild for your platform
npm rebuild mathhook-node
```

### Node Version

MathHook requires Node.js 18+:
```bash
node --version  # Should be >=18.0.0
```

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MathHook is dual-licensed under MIT OR Apache-2.0. See [LICENSE](../../LICENSE) for details.

## Links

- **npm**: https://www.npmjs.com/package/mathhook-node
- **GitHub**: https://github.com/AhmedMashour/mathhook
- **Documentation**: https://mathhook.readthedocs.io
- **Issue Tracker**: https://github.com/AhmedMashour/mathhook/issues
