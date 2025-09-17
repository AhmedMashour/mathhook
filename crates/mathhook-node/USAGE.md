# MathHook Node.js/TypeScript Usage Guide

Comprehensive guide for using MathHook in Node.js and TypeScript projects.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Expression Creation](#expression-creation)
4. [Algebraic Operations](#algebraic-operations)
5. [Calculus](#calculus)
6. [Solving Equations](#solving-equations)
7. [Matrix Operations](#matrix-operations)
8. [Parsing and Formatting](#parsing-and-formatting)
9. [Educational Features](#educational-features)
10. [Performance Tips](#performance-tips)
11. [TypeScript Integration](#typescript-integration)
12. [Async Operations](#async-operations)
13. [Error Handling](#error-handling)
14. [Testing](#testing)

## Installation

```bash
npm install mathhook-node
```

For TypeScript projects:

```bash
npm install --save-dev @types/node typescript
```

## Quick Start

### JavaScript (CommonJS)

```javascript
const { Expression, symbol, solve, sin, cos } = require('mathhook-node');

const x = symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);
const simplified = expr.simplify();

console.log(simplified.toString());
```

### JavaScript (ESM)

```javascript
import { Expression, symbol, solve } from 'mathhook-node';

const x = symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);
console.log(expr.simplify().toString());
```

### TypeScript

```typescript
import { Expression, Symbol, symbol } from 'mathhook-node';

const x: Symbol = symbol('x');
const expr: Expression = x.pow(2).add(x.multiply(2)).add(1);
console.log(expr.simplify().toString());
```

## Expression Creation

### Basic Expressions

```typescript
import { Expression, symbol } from 'mathhook-node';

// Integers
const num = Expression.integer(42);

// Floats
const pi = Expression.float(3.14159);

// Rationals
const half = Expression.rational(1, 2);  // 1/2

// Symbols
const x = symbol('x');
const y = symbol('y');
```

### Method Chaining

MathHook supports fluent API design:

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');

// Chain operations
const expr = x.pow(2)
              .multiply(2)
              .add(x.multiply(3))
              .subtract(5);

// Complex chains
const result = x.add(1)
                .multiply(x.subtract(1))
                .expand()
                .simplify();
```

### Arithmetic Operations

```typescript
import { symbol, Expression } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Basic operations
const sum = x.add(y);
const difference = x.subtract(y);
const product = x.multiply(y);
const quotient = x.divide(y);
const power = x.pow(2);

// Negate
const negative = x.negate();  // -x
```

### Mathematical Functions

```typescript
import {
  // Trigonometric
  sin, cos, tan, cot, sec, csc,
  asin, acos, atan, atan2,

  // Hyperbolic
  sinh, cosh, tanh,

  // Exponential and logarithmic
  exp, log, ln,

  // Roots and powers
  sqrt, cbrt,

  // Special functions
  factorial, gamma, beta,

  symbol
} from 'mathhook-node';

const x = symbol('x');

// Trigonometric
const trig = sin(x).pow(2).add(cos(x).pow(2));  // = 1

// Exponential
const exponential = exp(x);

// Logarithmic
const naturalLog = ln(x);
const log10 = log(x, 10);

// Special
const fact = factorial(Expression.integer(5));  // 120
```

### Constants

```typescript
import { pi, e, I, infinity } from 'mathhook-node';

const x = symbol('x');

// Mathematical constants
const circleCircumference = Expression.integer(2).multiply(pi).multiply(r);
const eulerFormula = exp(I.multiply(pi)).add(1);  // = 0
const limit = Expression.integer(1).divide(infinity);  // = 0
```

## Algebraic Operations

### Simplification

```typescript
import { symbol, sin, cos } from 'mathhook-node';

const x = symbol('x');

// Automatic simplification
let expr = x.add(x).add(x);
console.log(expr.toString());  // May already be simplified

// Explicit simplification
expr = expr.simplify();
console.log(expr.toString());  // 3*x

// Trigonometric simplification
const trig = sin(x).pow(2).add(cos(x).pow(2));
console.log(trig.simplify().toString());  // 1

// Rational simplification
const rational = x.pow(2).subtract(1).divide(x.subtract(1));
console.log(rational.simplify().toString());  // x + 1
```

### Expansion

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Expand products
let expr = x.add(1).multiply(x.add(2));
console.log(expr.expand().toString());  // x^2 + 3*x + 2

// Expand powers
expr = x.add(y).pow(2);
console.log(expr.expand().toString());  // x^2 + 2*x*y + y^2

// Expand complex expressions
expr = x.add(1).pow(2).multiply(x.subtract(1));
console.log(expr.expand().toString());  // x^3 + x^2 - x - 1
```

### Factorization

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');

// Factor polynomials
let expr = x.pow(2).subtract(1);
console.log(expr.factor().toString());  // (x - 1)(x + 1)

// Factor quadratics
expr = x.pow(2).add(x.multiply(5)).add(6);
console.log(expr.factor().toString());  // (x + 2)(x + 3)
```

### Collecting Terms

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Collect like terms
const expr = x.multiply(y)
              .add(x.multiply(y.pow(2)))
              .add(y.pow(2))
              .add(y);

const collected = expr.collect(x);
console.log(collected.toString());
// x*(y + y^2) + y^2 + y
```

### Substitution

```typescript
import { symbol, Expression } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

const expr = x.pow(2).add(x.multiply(2)).add(1);

// Substitute value
let result = expr.subs(x, Expression.integer(3));
console.log(result.toString());  // 16

// Substitute expression
result = expr.subs(x, y.add(1));
console.log(result.toString());  // (y + 1)^2 + 2*(y + 1) + 1

// Multiple substitutions
const expr2 = x.add(y);
result = expr2.subs([
  { symbol: x, value: Expression.integer(1) },
  { symbol: y, value: Expression.integer(2) }
]);
console.log(result.toString());  // 3
```

## Calculus

### Derivatives

```typescript
import { symbol, sin, cos, exp } from 'mathhook-node';

const x = symbol('x');

// First derivative
let expr = x.pow(3);
console.log(expr.diff(x).toString());  // 3*x^2

// Higher-order derivatives
console.log(expr.diff(x, 2).toString());  // 6*x
console.log(expr.diff(x, 3).toString());  // 6

// Chain rule
expr = sin(x.pow(2));
console.log(expr.diff(x).toString());  // 2*x*cos(x^2)

// Product rule
expr = x.multiply(exp(x));
console.log(expr.diff(x).toString());  // x*exp(x) + exp(x)

// Partial derivatives
const y = symbol('y');
expr = x.pow(2).multiply(y).add(y.pow(2));
console.log(expr.diff(x).toString());  // 2*x*y
console.log(expr.diff(y).toString());  // x^2 + 2*y
```

### Integrals

```typescript
import { symbol, sin, Expression } from 'mathhook-node';

const x = symbol('x');

// Indefinite integrals
let expr = x.pow(2);
console.log(expr.integrate(x).toString());  // x^3/3

// Definite integrals
const result = expr.integrate(x, Expression.integer(0), Expression.integer(1));
console.log(result.toString());  // 1/3

// Trigonometric integrals
expr = sin(x);
console.log(expr.integrate(x).toString());  // -cos(x)
```

### Limits

```typescript
import { symbol, sin, infinity, Expression } from 'mathhook-node';

const x = symbol('x');

// Finite limits
let expr = sin(x).divide(x);
const lim = expr.limit(x, Expression.integer(0));
console.log(lim.toString());  // 1

// Limits at infinity
expr = Expression.integer(1).divide(x);
console.log(expr.limit(x, infinity).toString());  // 0

// One-sided limits
expr = Expression.integer(1).divide(x);
const limRight = expr.limit(x, Expression.integer(0), { direction: '+' });
const limLeft = expr.limit(x, Expression.integer(0), { direction: '-' });
```

### Series Expansions

```typescript
import { symbol, sin, exp } from 'mathhook-node';

const x = symbol('x');

// Taylor series around 0
const seriesSin = sin(x).series(x, 0, 6);
console.log(seriesSin.toString());
// x - x^3/6 + x^5/120 + O(x^6)

// Series around different point
const seriesExp = exp(x).series(x, 1, 4);
console.log(seriesExp.toString());
// e + e*(x-1) + e*(x-1)^2/2 + ...

// Remove order term
const pure = seriesSin.removeO();
console.log(pure.toString());
// x - x^3/6 + x^5/120
```

## Solving Equations

### Algebraic Equations

```typescript
import { symbol, solve, Expression } from 'mathhook-node';

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

// Cubic and higher
solutions = solve(x.pow(3).subtract(8), x);
console.log(solutions);  // [2, complex roots...]
```

### Systems of Equations

```typescript
import { symbol, solveSystem } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Linear system
const solutions = solveSystem(
  [
    x.add(y).subtract(5),
    x.subtract(y).subtract(1)
  ],
  [x, y]
);

console.log(solutions);  // { x: 3, y: 2 }

// Nonlinear system
const solutions2 = solveSystem(
  [
    x.pow(2).add(y.pow(2)).subtract(25),
    x.subtract(y).subtract(1)
  ],
  [x, y]
);
```

### Differential Equations

```typescript
import { symbol, Function, dsolve, Expression } from 'mathhook-node';

const x = symbol('x');
const f = Function('f');

// First-order ODE: dy/dx = y
const eq = f(x).diff(x).subtract(f(x));
const solution = dsolve(eq, f(x));
console.log(solution.toString());
// f(x) = C1*exp(x)

// With initial conditions
const solution2 = dsolve(
  eq,
  f(x),
  { initialConditions: { [f(0)]: 1 } }
);
console.log(solution2.toString());
// f(x) = exp(x)
```

## Matrix Operations

### Creating Matrices

```typescript
import { Matrix, Expression } from 'mathhook-node';

// From nested arrays
const A = new Matrix([
  [1, 2, 3],
  [4, 5, 6]
]);

// From expressions
const x = symbol('x');
const B = new Matrix([
  [x, x.pow(2)],
  [Expression.integer(1), x]
]);

// Special matrices
const I = Matrix.identity(3);          // 3x3 identity
const Z = Matrix.zeros(2, 3);          // 2x3 zero matrix
const D = Matrix.diagonal([1, 2, 3]);  // Diagonal matrix

// From flat array
const C = Matrix.fromArray([1, 2, 3, 4], 2, 2);  // 2x2 matrix
```

### Basic Operations

```typescript
import { Matrix } from 'mathhook-node';

const A = new Matrix([[1, 2], [3, 4]]);
const B = new Matrix([[5, 6], [7, 8]]);

// Addition
const C = A.add(B);

// Subtraction
const D = A.subtract(B);

// Scalar multiplication
const E = A.multiplyScalar(2);

// Matrix multiplication
const F = A.multiply(B);

// Element-wise multiplication
const G = A.multiplyElementwise(B);

// Transpose
const AT = A.transpose();
// or
const AT2 = A.T();
```

### Matrix Properties

```typescript
import { Matrix } from 'mathhook-node';

const A = new Matrix([[1, 2], [3, 4]]);

// Determinant
console.log(A.determinant());  // -2

// Trace
console.log(A.trace());  // 5

// Rank
console.log(A.rank());  // 2

// Inverse
const Ainv = A.inverse();

// Condition number
const cond = A.conditionNumber();

// Is square?
console.log(A.isSquare());  // true

// Is symmetric?
console.log(A.isSymmetric());  // false
```

### Matrix Decomposition

```typescript
import { Matrix } from 'mathhook-node';

const A = new Matrix([[4, 2], [2, 3]]);

// Eigenvalues and eigenvectors
const eigenvals = A.eigenvalues();
console.log(eigenvals);

const eigenvects = A.eigenvectors();
console.log(eigenvects);

// LU decomposition
const { L, U, P } = A.luDecomposition();

// QR decomposition
const { Q, R } = A.qrDecomposition();

// Cholesky decomposition (for positive definite)
const LChol = A.choleskyDecomposition();

// SVD
const { U: USVD, S, V } = A.svd();

// Diagonalization
const { P: PDiag, D } = A.diagonalize();
```

### Solving Linear Systems

```typescript
import { Matrix } from 'mathhook-node';

const A = new Matrix([[2, 1], [1, 3]]);
const b = new Matrix([[5], [6]]);

// Solve Ax = b
const x = A.solve(b);
console.log(x.toString());  // [[1], [3]]

// Using LU decomposition
const xLU = A.solveLU(b);

// Least squares solution
const xLS = A.solveLeastSquares(b);
```

## Parsing and Formatting

### Parsing Expressions

```typescript
import { parse, parseLatex, parseWolfram } from 'mathhook-node';

// Auto-detect format
let expr = parse("2*x + sin(y)");

// LaTeX
expr = parseLatex(String.raw`\frac{x^2}{2} + \sqrt{y}`);
expr = parseLatex(String.raw`\int_{0}^{1} x^2 \, dx`);

// Wolfram Language
expr = parseWolfram("Sin[x] + Cos[y]");
expr = parseWolfram("D[x^2, x]");

// With parser options
expr = parse("2x + 3y", { implicitMultiplication: true });
```

### Output Formatting

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const expr = x.pow(2).divide(2);

// String representation
console.log(expr.toString());  // x^2/2

// LaTeX
console.log(expr.toLatex());  // \frac{x^{2}}{2}

// Wolfram
console.log(expr.toWolfram());  // Divide[Power[x, 2], 2]

// Pretty print (Unicode)
console.log(expr.toPretty());
//  2
// x
// ──
// 2

// JSON
const json = expr.toJSON();
console.log(JSON.stringify(json, null, 2));
```

## Educational Features

### Step-by-Step Solutions

```typescript
import { symbol, Expression } from 'mathhook-node';

const x = symbol('x');
const expr = x.add(1).multiply(x.subtract(1));

// Get expansion steps
const steps = expr.expand({ showSteps: true });

if (steps.steps) {
  steps.steps.forEach((step, i) => {
    console.log(`\nStep ${i + 1}: ${step.title}`);
    console.log(`Description: ${step.description}`);
    console.log(`Result: ${step.expression}`);
  });
}
```

### Derivative Explanation

```typescript
import { symbol, sin } from 'mathhook-node';

const x = symbol('x');
const expr = sin(x.pow(2));

// Get derivative explanation
const explanation = expr.diff(x, { explain: true });

console.log(explanation.steps);
// Shows chain rule application step by step
```

## Performance Tips

### Configuration

```typescript
import { configure } from 'mathhook-node';

// Use Node.js-optimized configuration
configure({
  binding: 'nodejs',
  simdEnabled: true,
  parallelEnabled: true
});

// Custom settings
configure({
  cacheSize: 20000,
  simdThreshold: 100,
  parallelThreshold: 500
});
```

### Caching

```typescript
import { symbol, Expression } from 'mathhook-node';

// Manual caching
const cache = new Map<string, Expression>();

function getOrCompute(key: string, compute: () => Expression): Expression {
  if (!cache.has(key)) {
    cache.set(key, compute());
  }
  return cache.get(key)!;
}

const x = symbol('x');
const expr = getOrCompute('x^100', () => x.pow(100).expand());
```

### Bulk Operations

```typescript
import { simplifyMany, differentiateMany } from 'mathhook-node';

const x = symbol('x');

// Simplify many expressions at once
const expressions = Array.from({ length: 1000 }, (_, i) =>
  x.pow(2).add(Expression.integer(i))
);

const simplified = simplifyMany(expressions);  // Uses parallelization

// Differentiate many expressions
const derivatives = differentiateMany(expressions, x);
```

### Memory Management

```typescript
import { clearCache, getMemoryUsage } from 'mathhook-node';

// Check memory usage
console.log(getMemoryUsage());

// Clear caches
clearCache();

// Manual garbage collection hint (if available)
if (global.gc) {
  global.gc();
}
```

## TypeScript Integration

### Type-Safe Functions

```typescript
import { Expression, Symbol, symbol } from 'mathhook-node';

function polynomial(
  coefficients: number[],
  variable: Symbol
): Expression {
  return coefficients.reduce((sum, coef, power) =>
    sum.add(
      Expression.integer(coef).multiply(variable.pow(power))
    ),
    Expression.integer(0)
  );
}

const x = symbol('x');
const poly = polynomial([1, 2, 3], x);  // 1 + 2x + 3x^2
```

### Generic Math Operations

```typescript
import { Expression, Symbol } from 'mathhook-node';

class MathOperations<T extends Expression> {
  constructor(private expr: T) {}

  apply(operation: (e: Expression) => Expression): MathOperations<Expression> {
    return new MathOperations(operation(this.expr));
  }

  get result(): T {
    return this.expr;
  }
}

// Usage
const x = symbol('x');
const result = new MathOperations(x.pow(2))
  .apply(e => e.add(1))
  .apply(e => e.simplify())
  .result;
```

### Type Guards

```typescript
import { Expression, isSymbol, isNumber, isFunction } from 'mathhook-node';

function processExpression(expr: Expression): string {
  if (isSymbol(expr)) {
    return `Symbol: ${expr.name}`;
  } else if (isNumber(expr)) {
    return `Number: ${expr.value}`;
  } else if (isFunction(expr)) {
    return `Function: ${expr.name}`;
  }
  return 'Other expression';
}
```

## Async Operations

### Long-Running Computations

```typescript
import { solveAsync, integrateAsync } from 'mathhook-node';

async function complexComputation() {
  const x = symbol('x');

  // Non-blocking operations
  const [solutions, integral] = await Promise.all([
    solveAsync(x.pow(10).subtract(1), x),
    integrateAsync(x.pow(100), x)
  ]);

  return { solutions, integral };
}

// Usage
complexComputation().then(result => {
  console.log(result);
});
```

### Worker Threads

```typescript
import { Worker } from 'worker_threads';

// worker.js
if (isMainThread) {
  const worker = new Worker(__filename);
  worker.on('message', (result) => {
    console.log('Result:', result);
  });
  worker.postMessage({ expr: 'x^100' });
} else {
  parentPort?.on('message', ({ expr }) => {
    const { parse } = require('mathhook-node');
    const result = parse(expr).expand();
    parentPort?.postMessage(result.toString());
  });
}
```

## Error Handling

### Try-Catch Pattern

```typescript
import { parse, MathError } from 'mathhook-node';

try {
  const expr = parse("invalid expression");
} catch (error) {
  if (error instanceof MathError) {
    console.error('Math error:', error.message);
    console.error('Context:', error.context);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

### Validation

```typescript
import { symbol, Expression } from 'mathhook-node';

function safeDiv(a: Expression, b: Expression): Expression | null {
  // Check for division by zero
  if (b.equals(Expression.integer(0))) {
    console.error('Division by zero');
    return null;
  }
  return a.divide(b);
}

const x = symbol('x');
const result = safeDiv(x, Expression.integer(0));  // null
```

## Testing

### Unit Tests (Jest)

```typescript
import { symbol, Expression } from 'mathhook-node';

describe('MathHook Operations', () => {
  let x: Symbol;

  beforeEach(() => {
    x = symbol('x');
  });

  test('simplification works', () => {
    const expr = x.add(x);
    const simplified = expr.simplify();
    expect(simplified.toString()).toBe('2*x');
  });

  test('derivative of x^2', () => {
    const expr = x.pow(2);
    const derivative = expr.diff(x);
    expect(derivative.toString()).toBe('2*x');
  });

  test('equation solving', () => {
    const solutions = solve(x.pow(2).subtract(4), x);
    expect(solutions.length).toBe(2);
    expect(solutions.map(s => s.toString())).toContain('2');
    expect(solutions.map(s => s.toString())).toContain('-2');
  });
});
```

### Integration Tests

```typescript
import { parse, symbol } from 'mathhook-node';

describe('End-to-end workflows', () => {
  test('complete calculus workflow', () => {
    const x = symbol('x');

    // Parse
    const expr = parse('x^2 + 2*x + 1');

    // Differentiate
    const derivative = expr.diff(x);

    // Integrate back
    const integral = derivative.integrate(x);

    // Should match original (up to constant)
    const difference = expr.subtract(integral).simplify();
    expect(difference.isConstant()).toBe(true);
  });
});
```

## Best Practices

1. **Use TypeScript**: Leverage type safety for fewer runtime errors
2. **Cache expensive operations**: Store results of complex computations
3. **Process in bulk**: Use batch operations for multiple expressions
4. **Configure for your use case**: Tune performance settings appropriately
5. **Handle errors gracefully**: Always wrap parsing and solving in try-catch
6. **Use async for heavy computations**: Prevent blocking the event loop
7. **Profile before optimizing**: Measure actual performance bottlenecks

## Common Patterns

### Polynomial Builder

```typescript
function buildPolynomial(coeffs: number[], x: Symbol): Expression {
  return coeffs.reduce((sum, coef, i) =>
    sum.add(Expression.integer(coef).multiply(x.pow(i))),
    Expression.integer(0)
  );
}

const x = symbol('x');
const poly = buildPolynomial([1, -2, 1], x);  // 1 - 2x + x^2
```

### Expression Evaluator

```typescript
function evaluateAt(expr: Expression, values: Record<string, number>): number {
  let result = expr;
  for (const [varName, value] of Object.entries(values)) {
    result = result.subs(symbol(varName), Expression.float(value));
  }
  return result.evaluate();
}

const x = symbol('x');
const y = symbol('y');
const expr = x.pow(2).add(y.pow(2));
console.log(evaluateAt(expr, { x: 3, y: 4 }));  // 25
```

## Further Reading

- [API Reference](https://mathhook.readthedocs.io)
- [TypeScript Definitions](index.d.ts)
- [Examples](examples/)
- [Rust Core Documentation](../../README.md)
- [Performance Benchmarks](../../docs/benchmarks.md)
