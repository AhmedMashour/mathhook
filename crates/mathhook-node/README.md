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

Requires Node.js 10 or higher.

### Platform Support

Pre-built binaries are available for:
- Windows (x64, ARM64)
- macOS (x64, ARM64/Apple Silicon)
- Linux (x64, ARM64, ARMv7, musl)

## Quick Start

### JavaScript

```javascript
const { JsExpression, symbol, symbols, parse, JsMathSolver } = require('mathhook-node');

// Create symbols
const x = symbol('x');

// Build expressions
const expr = x.pow(2).add(x.multiply(2)).add(1);

// Simplify
const simplified = expr.simplify();
console.log(simplified.toSimple());  // x^2 + 2*x + 1

// Solve equations
const solver = new JsMathSolver();
const result = solver.solve(x.pow(2).subtract(4), 'x');
console.log(result.solutions);  // ['2', '-2']

// Calculus
const derivative = expr.diff('x');
console.log(derivative.toSimple());  // 2*x + 2
```

### TypeScript

```typescript
import { JsExpression, symbol, symbols, JsMathSolver, JsSolverResult } from 'mathhook-node';

// Full type safety
const x: JsExpression = symbol('x');
const expr: JsExpression = x.pow(2).add(x.multiply(2)).add(1);

// Type-safe operations
const simplified: JsExpression = expr.simplify();

// Solve equations
const solver = new JsMathSolver();
const result: JsSolverResult = solver.solve(x.pow(2).subtract(4), 'x');
```

## Expression Creation

### Basic Types

```typescript
import { JsExpression } from 'mathhook-node';

// Numbers
const integer = JsExpression.integer(42);
const rational = JsExpression.rational(3, 4);  // 3/4
const floating = JsExpression.float(3.14159);

// Complex numbers
const complex = JsExpression.complex(JsExpression.integer(3), JsExpression.integer(4));  // 3 + 4i
```

### Symbols

```typescript
import { symbol, symbols } from 'mathhook-node';

// Single symbol
const x = symbol('x');

// Multiple symbols (space-separated string)
const [a, b, c] = symbols('a b c');

// Comma-separated also works
const [p, q, r] = symbols('p, q, r');

// Range syntax
const [x0, x1, x2] = symbols('x0:3');
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
import { sin, cos, tan, exp, ln, sqrt, symbol } from 'mathhook-node';

const x = symbol('x');

// Trigonometric
const trig = sin(x).pow(2).add(cos(x).pow(2));  // = 1

// Exponential and logarithmic
const exponential = exp(x);
const logarithm = ln(x);

// Square root
const root = sqrt(x.pow(2).add(1));
```

### Constants

```typescript
import { JsExpression, symbol } from 'mathhook-node';

const r = symbol('r');

// Mathematical constants (as methods)
const pi = JsExpression.pi();
const e = JsExpression.e();
const i = JsExpression.i();  // imaginary unit
const phi = JsExpression.goldenRatio();
const gamma = JsExpression.eulerGamma();

// Use in expressions
const circleArea = pi.multiply(r.pow(2));
```

## Algebraic Operations

### Simplification

```typescript
import { symbol, sin, cos } from 'mathhook-node';

const x = symbol('x');

// Algebraic simplification
const expr = x.add(x).add(x);
console.log(expr.simplify().toSimple());  // 3*x

// Trigonometric identities
const trig = sin(x).pow(2).add(cos(x).pow(2));
console.log(trig.simplify().toSimple());  // 1

// Rational expressions
const rational = x.pow(2).subtract(1)
                  .divide(x.subtract(1));
console.log(rational.simplify().toSimple());  // x + 1
```

### Expansion

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

// Expand products
const expr = x.add(1).multiply(x.add(2));
console.log(expr.expand().toSimple());  // x^2 + 3*x + 2

// Expand powers
const power = x.add(y).pow(2);
console.log(power.expand().toSimple());  // x^2 + 2*x*y + y^2
```

### Factorization

```typescript
import { symbol } from 'mathhook-node';

const x = symbol('x');

// Factor polynomials
const expr = x.pow(2).subtract(1);
console.log(expr.factor().toSimple());  // (x - 1)*(x + 1)

const poly = x.pow(2).add(x.multiply(5)).add(6);
console.log(poly.factor().toSimple());  // (x + 2)*(x + 3)
```

### Substitution

```typescript
import { JsExpression, symbol } from 'mathhook-node';

const x = symbol('x');
const y = symbol('y');

const expr = x.pow(2).add(x.multiply(2)).add(1);

// Substitute with object syntax
const result = expr.substitute({ x: JsExpression.integer(3) });
console.log(result.toSimple());  // 16

// Substitute expression
const substituted = expr.substitute({ x: y.add(1) });
console.log(substituted.toSimple());  // (y + 1)^2 + 2*(y + 1) + 1
```

## Calculus

### Derivatives

```typescript
import { symbol, sin, cos } from 'mathhook-node';

const x = symbol('x');

// First derivative (using string variable name)
const expr = x.pow(3);
console.log(expr.diff('x').toSimple());  // 3*x^2

// Or using symbol expression
console.log(expr.diff(x).toSimple());  // 3*x^2

// Higher-order derivatives
console.log(expr.nthDerivative('x', 2).toSimple());  // 6*x
console.log(expr.nthDerivative('x', 3).toSimple());  // 6

// Chain rule
const chain = sin(x.pow(2));
console.log(chain.diff('x').toSimple());  // 2*x*cos(x^2)
```

### Integrals

```typescript
import { JsExpression, symbol, sin } from 'mathhook-node';

const x = symbol('x');

// Indefinite integrals
const expr = x.pow(2);
console.log(expr.integrate('x').toSimple());  // x^3/3

// Definite integrals
const lower = JsExpression.integer(0);
const upper = JsExpression.integer(1);
const definite = expr.integrateDefinite('x', lower, upper);
console.log(definite.toSimple());  // 1/3

// Trigonometric integrals
const trig = sin(x);
console.log(trig.integrate('x').toSimple());  // -cos(x)
```

### Limits

```typescript
import { JsExpression, symbol, sin } from 'mathhook-node';

const x = symbol('x');

// Finite limits
const expr = sin(x).divide(x);
const lim = expr.limit('x', JsExpression.integer(0));
console.log(lim.toSimple());  // 1

// Limits at infinity
const expr2 = JsExpression.integer(1).divide(x);
console.log(expr2.limitInfinity('x').toSimple());  // 0
```

### Series Expansions

```typescript
import { JsExpression, symbol, sin } from 'mathhook-node';

const x = symbol('x');

// Taylor series around x=0
const series = sin(x).series('x', JsExpression.integer(0), 5);
console.log(series.toSimple());
// x - x^3/6 + x^5/120 + O(x^6)
```

## Equation Solving

### Algebraic Equations

```typescript
import { JsExpression, symbol, JsMathSolver } from 'mathhook-node';

const x = symbol('x');
const solver = new JsMathSolver();

// Linear equations (solve for expression = 0)
let result = solver.solve(
  x.multiply(2).add(3).subtract(7),  // 2x + 3 - 7 = 0
  'x'
);
console.log(result.solutions);  // ['2']

// Quadratic equations
result = solver.solve(
  x.pow(2).subtract(x.multiply(5)).add(6),  // x² - 5x + 6 = 0
  'x'
);
console.log(result.solutions);  // ['2', '3']

// Complex solutions
result = solver.solve(x.pow(2).add(1), 'x');  // x² + 1 = 0
console.log(result.solutions);  // ['i', '-i']

// Using explicit equation
const equation = JsExpression.equation(x, JsExpression.integer(5));  // x = 5
result = solver.solve(equation, 'x');
console.log(result.solutions);  // ['5']
```

### Solver Result Structure

```typescript
interface JsSolverResult {
  resultType: string;      // "single", "multiple", "no_solution", "infinite_solutions"
  solutions: string[];     // Solution expressions as strings
  count: number;           // Number of solutions
  metadata?: string;       // Optional info about the solution
}
```

## Matrix Operations

### Creating Matrices

```typescript
import { JsExpression } from 'mathhook-node';

// From arrays of expressions
const a = JsExpression.integer(1);
const b = JsExpression.integer(2);
const c = JsExpression.integer(3);
const d = JsExpression.integer(4);

const A = JsExpression.matrix([[a, b], [c, d]]);

// Identity matrix
const I = JsExpression.identityMatrix(3);

// Zero matrix
const Z = JsExpression.zeroMatrix(2, 3);
```

### Matrix Operations

```typescript
import { JsExpression } from 'mathhook-node';

const one = JsExpression.integer(1);
const two = JsExpression.integer(2);
const three = JsExpression.integer(3);
const four = JsExpression.integer(4);

const A = JsExpression.matrix([[one, two], [three, four]]);

// Transpose
const AT = A.transpose();

// Determinant
const det = A.determinant();
console.log(det.toSimple());  // -2

// Inverse
const Ainv = A.inverse();

// Matrix arithmetic uses standard methods
// A.add(B), A.multiply(B), etc.
```

### Matrix Decomposition

```typescript
import { JsExpression } from 'mathhook-node';

// Create a matrix
const matrix = JsExpression.matrix([
  [JsExpression.integer(4), JsExpression.integer(2)],
  [JsExpression.integer(2), JsExpression.integer(3)]
]);

// LU decomposition
const lu = matrix.luDecomposition();
console.log(lu.l);  // Lower triangular
console.log(lu.u);  // Upper triangular
console.log(lu.p);  // Permutation matrix (if pivoting)

// QR decomposition
const qr = matrix.qrDecomposition();
console.log(qr.q);  // Orthogonal matrix
console.log(qr.r);  // Upper triangular

// SVD decomposition
const svd = matrix.svdDecomposition();
console.log(svd.u);      // Left singular vectors
console.log(svd.sigma);  // Singular values
console.log(svd.vt);     // Right singular vectors (transposed)

// Cholesky decomposition (for positive definite matrices)
const L = matrix.choleskyDecomposition();
```

## Parsing

### Multi-Format Parser

```typescript
import { parse } from 'mathhook-node';

// Standard notation
let expr = parse("2*x + sin(y)");

// LaTeX (auto-detected)
expr = parse("\\frac{x^2}{2} + \\sqrt{y}");
expr = parse("\\sin(x) + \\cos(y)");

// Wolfram Language (auto-detected)
expr = parse("Sin[x] + Cos[y]");

// Implicit multiplication
expr = parse("2x + 3y");  // Same as 2*x + 3*y
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

// Simple string representation
console.log(expr.toSimple());  // x^2/2

// toString() also works
console.log(expr.toString());  // x^2/2
```

## Educational Features

### Step-by-Step Explanations

```typescript
import { JsExpression, symbol, JsMathSolver, parse } from 'mathhook-node';

const x = symbol('x');
const expr = JsExpression.integer(2).add(JsExpression.integer(3));

// Get simplification steps
const explanation = expr.explainSimplification();

explanation.steps.forEach((step, i) => {
  console.log(`Step ${i + 1}: ${step.title}`);
  console.log(`  ${step.description}`);
  console.log(`  Before: ${step.before}`);
  console.log(`  After: ${step.after}`);
  console.log();
});

// Derivative with steps
const poly = x.pow(2);
const derivExplanation = poly.derivativeWithSteps('x');
```

### Solve with Steps

Get step-by-step explanations when solving equations:

```typescript
import { JsMathSolver, parse } from 'mathhook-node';

const solver = new JsMathSolver();
const equation = parse('x^2 - 4');

// Solve with educational steps
const result = solver.solveWithSteps(equation, 'x');

console.log('Result type:', result.resultType);  // 'multiple'
console.log('Solutions:', result.solutions.map(s => s.toSimple()));  // ['2', '-2']

// Display solving steps
for (const step of result.steps) {
  console.log(`${step.title}: ${step.description}`);
  console.log(`  Before: ${step.before}`);
  console.log(`  After: ${step.after}`);
}
```

Result structure:
```typescript
interface JsSolveWithStepsResult {
  resultType: string;        // "single", "multiple", "no_solution", "infinite", etc.
  solutions: JsExpression[]; // Actual expression objects (not strings)
  steps: JsStep[];           // Step-by-step explanation
}
```

## Evaluation

### Basic Evaluation

```typescript
import { JsExpression } from 'mathhook-node';

const expr = JsExpression.integer(2).add(JsExpression.integer(3));
const result = expr.evaluate();
console.log(result.toSimple());  // 5
```

### Evaluation with Context

```typescript
import { JsExpression, symbol, EvalContext } from 'mathhook-node';

const x = symbol('x');
const expr = x.pow(2).add(1);

// Symbolic evaluation (no numerical conversion)
const symbolicCtx = EvalContext.symbolic();
const symbolicResult = expr.evaluateWithContext(symbolicCtx);
// Result stays symbolic: x^2 + 1

// Numerical evaluation with substitutions
const numericCtx = EvalContext.numeric([['x', JsExpression.integer(3)]]);
const numericResult = expr.evaluateWithContext(numericCtx);
console.log(numericResult.toSimple());  // 10

// Custom configuration
const customCtx = new EvalContext({
  numeric: true,
  precision: 128,
  simplifyFirst: true
});
```

## Polynomial Operations

```typescript
import { JsExpression, symbol, degree, roots, groebnerBasis } from 'mathhook-node';

const x = symbol('x');
const poly = x.pow(3).add(x.pow(2).multiply(2)).add(x).add(1);

// Get degree
const deg = degree(poly, 'x');
console.log(deg.toSimple());  // 3

// Or use method
const degMethod = poly.polynomialDegree('x');
console.log(degMethod);  // 3

// Leading coefficient
const lc = poly.polynomialLeadingCoefficient('x');

// Content (GCD of coefficients)
const content = poly.polynomialContent();

// Primitive part
const primitive = poly.polynomialPrimitivePart();

// Find roots
const r = roots(poly, 'x');

// Resultant of two polynomials
const p1 = x.pow(2).subtract(1);
const p2 = x.subtract(1);
const res = JsExpression.resultant(p1, p2, 'x');

// Discriminant
const disc = JsExpression.discriminant(poly, 'x');

// Gröbner basis
const y = symbol('y');
const polys = [
  x.pow(2).add(y.pow(2)).subtract(1),
  x.subtract(y)
];
const basis = groebnerBasis(polys, ['x', 'y'], 'lex');
```

## PDE Solver

```typescript
import { JsExpression, JsPDESolver } from 'mathhook-node';

const solver = new JsPDESolver();

// Heat equation: ∂u/∂t = α∇²u
const alpha = JsExpression.integer(1);
const heatSolution = solver.solveHeatEquation('u', 'x', 't', alpha);
console.log(heatSolution.solution);
console.log(heatSolution.method);

// Wave equation: ∂²u/∂t² = c²∇²u
const c = JsExpression.integer(1);
const waveSolution = solver.solveWaveEquation('u', 'x', 't', c);

// Laplace equation: ∇²u = 0
const laplaceSolution = solver.solveLaplaceEquation('u', 'x', 'y');
```

## Special Functions

```typescript
import {
  sin, cos, tan, asin, acos, atan,
  sinh, cosh, tanh,
  exp, ln, log10, sqrt,
  gamma, beta, factorial,
  erf, erfc,
  besselJ, besselY,
  zeta, digamma, polygamma,
  gcd, lcm, isprime,
  abs, sign, floor, ceil, round,
  symbol
} from 'mathhook-node';

const x = symbol('x');

// Trigonometric
const trig = sin(x).add(cos(x));

// Hyperbolic
const hyp = sinh(x).add(cosh(x));

// Special functions
const g = gamma(5);           // 24 (4!)
const b = beta(2, 3);         // Beta function
const f = factorial(5);       // 120
const e = erf(x);             // Error function
const j = besselJ(0, x);      // Bessel function of first kind
const z = zeta(2);            // Riemann zeta: π²/6

// Number theory
const g2 = gcd(12, 18);       // 6
const l = lcm(4, 6);          // 12
const p = isprime(17);        // true
```

## TypeScript Types

```typescript
import {
  JsExpression,
  JsMathSolver,
  JsSolverResult,
  JsSolveWithStepsResult,
  JsStep,
  JsStepByStepExplanation,
  EvalContext,
  EvalContextOptions,
  LUDecompositionResult,
  QRDecompositionResult,
  SVDDecompositionResult,
  PdeSolution,
  JsPDESolver
} from 'mathhook-node';

// Type-safe function
function quadratic(
  a: number,
  b: number,
  c: number,
  x: JsExpression
): JsExpression {
  return JsExpression.integer(a)
    .multiply(x.pow(2))
    .add(JsExpression.integer(b).multiply(x))
    .add(JsExpression.integer(c));
}

// Type-safe solving
function findRoots(expr: JsExpression, varName: string): string[] {
  const solver = new JsMathSolver();
  const result: JsSolverResult = solver.solve(expr, varName);
  return result.solutions;
}
```

## API Quick Reference

### Main Exports

| Export | Purpose |
|--------|---------|
| `JsExpression` | Core symbolic expression class |
| `symbol(name)` | Create a single symbol |
| `symbols(names)` | Create multiple symbols from string |
| `parse(str)` | Parse expression from string |
| `JsMathSolver` | Equation solver class |
| `EvalContext` | Evaluation context for controlled evaluation |
| `JsPDESolver` | PDE solver class |

### JsMathSolver Methods

| Method | Description |
|--------|-------------|
| `.solve(expr, var)` | Solve equation, returns result with solutions as strings |
| `.solveWithSteps(expr, var)` | Solve with step-by-step explanation, returns JsExpression solutions |

### Mathematical Functions

| Function | Description |
|----------|-------------|
| `sin`, `cos`, `tan` | Trigonometric |
| `asin`, `acos`, `atan` | Inverse trigonometric |
| `sinh`, `cosh`, `tanh` | Hyperbolic |
| `exp`, `ln`, `log10` | Exponential/logarithmic |
| `sqrt` | Square root |
| `gamma`, `beta`, `factorial` | Special functions |
| `erf`, `erfc` | Error functions |
| `besselJ`, `besselY` | Bessel functions |
| `zeta`, `digamma`, `polygamma` | Zeta and related |
| `gcd`, `lcm`, `isprime` | Number theory |
| `abs`, `sign`, `floor`, `ceil`, `round` | Numeric |
| `degree`, `roots`, `groebnerBasis` | Polynomial |

### JsExpression Methods

| Method | Description |
|--------|-------------|
| `.add(x)`, `.subtract(x)`, `.multiply(x)`, `.divide(x)` | Arithmetic |
| `.pow(n)`, `.negate()` | Power and negation |
| `.simplify()`, `.expand()`, `.factor()`, `.collect(var)` | Algebraic |
| `.substitute({var: expr})` | Substitution |
| `.diff(var)`, `.nthDerivative(var, n)` | Derivatives |
| `.integrate(var)`, `.integrateDefinite(var, lo, hi)` | Integrals |
| `.limit(var, val)`, `.limitInfinity(var)` | Limits |
| `.series(var, point, order)` | Series expansion |
| `.evaluate()`, `.evaluateWithContext(ctx)` | Evaluation |
| `.toSimple()`, `.toLatex()`, `.toWolfram()`, `.toString()` | Output formats |
| `.determinant()`, `.inverse()`, `.transpose()` | Matrix operations |
| `.luDecomposition()`, `.qrDecomposition()`, `.svdDecomposition()` | Decompositions |

### Static JsExpression Methods

| Method | Description |
|--------|-------------|
| `.integer(n)`, `.rational(num, den)`, `.float(x)` | Create numbers |
| `.symbol(name)` | Create symbol |
| `.complex(re, im)` | Create complex number |
| `.pi()`, `.e()`, `.i()`, `.goldenRatio()`, `.eulerGamma()` | Constants |
| `.function(name, args)` | Create function call |
| `.equation(left, right)` | Create equation |
| `.matrix(rows)`, `.identityMatrix(n)`, `.zeroMatrix(r, c)` | Create matrices |
| `.resultant(p1, p2, var)`, `.discriminant(p, var)` | Polynomial operations |
| `.parse(str)` | Parse expression |

## Common Issues

### Import Names

```typescript
// ✅ Correct - use JsExpression
import { JsExpression, symbol } from 'mathhook-node';

// ❌ Incorrect - Expression doesn't exist
import { Expression } from 'mathhook-node';
```

### Symbols Function

```typescript
// ✅ Correct - string input
const [x, y, z] = symbols('x y z');
const [a, b] = symbols('a, b');

// ❌ Incorrect - not array input
const [x, y] = symbols(['x', 'y']);
```

### Solving Equations

```typescript
// ✅ Correct - use JsMathSolver class
const solver = new JsMathSolver();
const result = solver.solve(expr, 'x');

// ❌ Incorrect - no standalone solve function
const result = solve(expr, x);
```

### Platform-Specific Binaries

If you get a binary loading error:
```bash
# Rebuild for your platform
npm rebuild mathhook-node
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

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MathHook is dual-licensed under MIT OR Apache-2.0. See [LICENSE](../../LICENSE) for details.

## Links

- **npm**: https://www.npmjs.com/package/mathhook-node
- **GitHub**: https://github.com/AhmedMashour/mathhook
- **Documentation**: https://docs.rs/mathhook
- **Issue Tracker**: https://github.com/AhmedMashour/mathhook/issues
