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
const { Expression, symbols, parse, SmartEquationSolver, sin, cos } = require('mathhook-node');

// Create symbols
const [x, y] = symbols('x y');

// Build expressions using static methods
const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([x, Expression.integer(2)]),
  Expression.integer(1)
]);

// Simplify and format
console.log(expr.simplify().format());  // x^2 + 2*x + 1

// Solve equations
const solver = new SmartEquationSolver();
const equation = Expression.equation(
  Expression.add([Expression.pow(x, Expression.integer(2)), Expression.integer(-4)]),
  Expression.integer(0)
);
const [solution, explanation] = solver.solveWithEquation(equation, x.asSymbol());
console.log(`Valid: ${solution.isValidSolution()}, Count: ${solution.solutionCount()}`);

// Calculus
const derivative = expr.diff(x.asSymbol());
console.log(derivative.format());  // 2*x + 2
```

### TypeScript

```typescript
import { Expression, symbols, SmartEquationSolver, sin, cos } from 'mathhook-node';

// Full type safety
const [x, y] = symbols('x y');

// Build expressions with static methods
const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([x, Expression.integer(2)]),
  Expression.integer(1)
]);

// Type-safe operations
const simplified = expr.simplify();
console.log(simplified.format());

// Solve equations
const solver = new SmartEquationSolver();
const equation = Expression.equation(x, Expression.integer(5));
const [solution, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);
```

## Expression Creation

### Basic Types

```typescript
import { Expression, symbols } from 'mathhook-node';

// Integers
const num = Expression.integer(42);

// Floats
const pi_approx = Expression.float(3.14159);

// Rationals (exact fractions)
const half = Expression.rational(1, 2);

// Symbols
const [x, y, z] = symbols('x y z');

// Single symbol
const a = symbols('a')[0];
```

### Static Expression Methods

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x, y] = symbols('x y');

// Addition - takes array of expressions
const sum = Expression.add([x, Expression.integer(2)]);

// Multiplication - takes array of expressions
const product = Expression.mul([x, Expression.integer(3)]);

// Power - takes base and exponent
const power = Expression.pow(x, Expression.integer(2));

// Division - takes numerator and denominator
const quotient = Expression.div(x, Expression.integer(2));

// Complex expression: 2x + 3
const linear = Expression.add([
  Expression.mul([Expression.integer(2), x]),
  Expression.integer(3)
]);
```

### Mathematical Functions

```typescript
import { sin, cos, tan, exp, ln, sqrt, abs, symbols, Expression } from 'mathhook-node';

const [x] = symbols('x');

// Trigonometric
const trig = Expression.add([
  Expression.pow(sin(x), Expression.integer(2)),
  Expression.pow(cos(x), Expression.integer(2))
]);  // = 1

// Exponential and logarithmic
const exponential = exp(x);
const logarithm = ln(x);

// Square root
const root = sqrt(Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(1)
]));

// Nested functions
const nested = sin(cos(x));
```

### Constants

```typescript
import { Expression, symbols } from 'mathhook-node';

const [r] = symbols('r');

// Mathematical constants (as static methods)
const pi = Expression.pi();
const e = Expression.e();
const i = Expression.i();  // imaginary unit

// Use in expressions
const circleArea = Expression.mul([pi, Expression.pow(r, Expression.integer(2))]);
```

## Algebraic Operations

### Simplification

```typescript
import { Expression, symbols, sin, cos } from 'mathhook-node';

const [x] = symbols('x');

// Algebraic simplification
const expr = Expression.add([x, x, x]);
console.log(expr.simplify().format());  // 3*x

// Trigonometric identities
const trig = Expression.add([
  Expression.pow(sin(x), Expression.integer(2)),
  Expression.pow(cos(x), Expression.integer(2))
]);
console.log(trig.simplify().format());  // 1
```

### Expression Methods

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

const expr = Expression.pow(x, Expression.integer(2));

// Simplify
const simplified = expr.simplify();

// Negate
const negated = expr.negate();

// Format to string
console.log(expr.format());

// Get symbol if expression is a symbol
const sym = x.asSymbol();
```

## Calculus

### Derivatives

```typescript
import { Expression, symbols, sin } from 'mathhook-node';

const [x] = symbols('x');

// First derivative
const expr = Expression.pow(x, Expression.integer(3));
const derivative = expr.diff(x.asSymbol()!);
console.log(derivative.format());  // 3*x^2

// Chain rule
const chain = sin(Expression.pow(x, Expression.integer(2)));
console.log(chain.diff(x.asSymbol()!).format());  // 2*x*cos(x^2)
```

## Equation Solving

### Using SmartEquationSolver

```typescript
import { Expression, symbols, SmartEquationSolver } from 'mathhook-node';

const [x] = symbols('x');
const solver = new SmartEquationSolver();

// Create equation: expression = value
const equation = Expression.equation(
  Expression.add([Expression.mul([Expression.integer(2), x]), Expression.integer(3)]),
  Expression.integer(7)  // 2x + 3 = 7
);

// Solve and get solution with explanation
const [solution, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);

console.log(`Valid solution: ${solution.isValidSolution()}`);
console.log(`Solution count: ${solution.solutionCount()}`);
```

### Quadratic Equations

```typescript
import { Expression, symbols, SmartEquationSolver } from 'mathhook-node';

const [x] = symbols('x');
const solver = new SmartEquationSolver();

// x² - 5x + 6 = 0
const quadratic = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([Expression.integer(-5), x]),
  Expression.integer(6)
]);
const equation = Expression.equation(quadratic, Expression.integer(0));

const [solution, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);
// Solutions: x = 2 and x = 3
```

## Parsing

### Multi-Format Parser

```typescript
import { parse } from 'mathhook-node';

// Standard notation
const expr1 = parse('2*x + sin(y)');
console.log(expr1.format());

// Implicit multiplication
const expr2 = parse('2x + 3y');
console.log(expr2.format());

// Functions
const expr3 = parse('sin(x) + cos(y)');
console.log(expr3.format());

// Greek letters
const expr4 = parse('alpha + beta + gamma');
console.log(expr4.format());

// Constants
const expr5 = parse('pi + e + i');
console.log(expr5.format());

// Complex expressions
const expr6 = parse('sin(2*pi*x) + exp(-x^2/2)');
console.log(expr6.format());

// LaTeX (auto-detected)
const latex = parse('\\frac{x^2}{2}');
console.log(latex.format());

// Wolfram (auto-detected)
const wolfram = parse('Sin[x] + Cos[y]');
console.log(wolfram.format());
```

### Mixing Parsed and Constructed Expressions

```typescript
import { parse, Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

const parsed = parse('x^2 + 1');
const combined = Expression.add([
  Expression.mul([parsed, x]),
  Expression.integer(5)
]);
console.log(combined.format());
```

## Special Functions

```typescript
import {
  sin, cos, tan, asin, acos, atan,
  sinh, cosh, tanh,
  exp, ln, log10, sqrt,
  gamma, factorial,
  erf, erfc,
  besselJ, besselY,
  zeta, digamma, polygamma,
  gcd, lcm, isprime,
  abs, sign, floor, ceil, round,
  symbols, Expression
} from 'mathhook-node';

const [x] = symbols('x');

// Trigonometric
const trig = sin(x);

// Hyperbolic
const hyp = sinh(x);

// Special functions
const g = gamma(Expression.integer(5));  // 24 (4!)
const f = factorial(5);                   // 120
const e = erf(x);                         // Error function
const j = besselJ(Expression.integer(0), x);  // Bessel function of first kind
const z = zeta(Expression.integer(2));    // Riemann zeta: π²/6

// Number theory
const g2 = gcd(Expression.integer(12), Expression.integer(18));  // 6
const l = lcm(Expression.integer(4), Expression.integer(6));     // 12
const p = isprime(Expression.integer(17));                        // true

// Rounding functions
const s = sign(Expression.integer(-5));   // -1
const fl = floor(Expression.float(3.7));  // 3
const cl = ceil(Expression.float(3.2));   // 4
const rn = round(Expression.float(3.5));  // 4
```

## Real-World Examples

### Quadratic Formula

```typescript
import { Expression, symbols, sqrt } from 'mathhook-node';

const [a, b, c] = symbols('a b c');

// Discriminant: b² - 4ac
const discriminant = Expression.add([
  Expression.pow(b, Expression.integer(2)),
  Expression.mul([Expression.integer(-4), a, c])
]);

// Solutions: (-b ± √discriminant) / 2a
const sqrtDisc = sqrt(discriminant);
const denom = Expression.mul([a, Expression.integer(2)]);

const solution1 = Expression.div(
  Expression.add([b.negate(), sqrtDisc]),
  denom
);
const solution2 = Expression.div(
  Expression.add([b.negate(), Expression.mul([Expression.integer(-1), sqrtDisc])]),
  denom
);

console.log(`x₁ = ${solution1.format()}`);
console.log(`x₂ = ${solution2.format()}`);
```

### Taylor Series Approximation

```typescript
import { Expression, symbols, factorial } from 'mathhook-node';

const [x] = symbols('x');

// sin(x) ≈ x - x³/3! + x⁵/5!
const fact3 = factorial(3);
const fact5 = factorial(5);

const term1 = x;
const term2 = Expression.div(
  Expression.pow(x, Expression.integer(3)),
  fact3
);
const term3 = Expression.div(
  Expression.pow(x, Expression.integer(5)),
  fact5
);

const sinTaylor = Expression.add([
  term1,
  Expression.mul([Expression.integer(-1), term2]),
  term3
]);

console.log(`sin(x) ≈ ${sinTaylor.format()}`);
```

### Pythagorean Distance

```typescript
import { Expression, symbols, sqrt } from 'mathhook-node';

const [x, y] = symbols('x y');

// distance = √(x² + y²)
const distance = sqrt(Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.pow(y, Expression.integer(2))
]));

console.log(`Distance: ${distance.format()}`);
```

## TypeScript Types

```typescript
import {
  Expression,
  Symbol,
  SmartEquationSolver,
  SolverResult,
  EvalContext
} from 'mathhook-node';

// Type-safe function
function quadratic(
  a: number,
  b: number,
  c: number,
  x: Expression
): Expression {
  return Expression.add([
    Expression.mul([Expression.integer(a), Expression.pow(x, Expression.integer(2))]),
    Expression.mul([Expression.integer(b), x]),
    Expression.integer(c)
  ]);
}
```

## API Quick Reference

### Main Exports

| Export | Purpose |
|--------|---------|
| `Expression` | Core symbolic expression class |
| `symbols(names)` | Create multiple symbols from space-separated string |
| `parse(str)` | Parse expression from string (auto-detects format) |
| `SmartEquationSolver` | Equation solver class |
| `EvalContext` | Evaluation context for controlled evaluation |

### Expression Static Methods

| Method | Description |
|--------|-------------|
| `.integer(n)` | Create integer expression |
| `.float(x)` | Create float expression |
| `.rational(num, den)` | Create rational expression |
| `.add(exprs[])` | Add array of expressions |
| `.mul(exprs[])` | Multiply array of expressions |
| `.pow(base, exp)` | Raise base to exponent |
| `.div(num, den)` | Divide numerator by denominator |
| `.equation(left, right)` | Create equation |
| `.pi()`, `.e()`, `.i()` | Mathematical constants |

### Expression Instance Methods

| Method | Description |
|--------|-------------|
| `.simplify()` | Simplify expression |
| `.diff(symbol)` | Differentiate with respect to symbol |
| `.negate()` | Negate expression |
| `.format()` | Format as string |
| `.asSymbol()` | Get Symbol if expression is a symbol |

### Mathematical Functions

| Function | Description |
|----------|-------------|
| `sin`, `cos`, `tan` | Trigonometric |
| `asin`, `acos`, `atan` | Inverse trigonometric |
| `sinh`, `cosh`, `tanh` | Hyperbolic |
| `exp`, `ln`, `log10` | Exponential/logarithmic |
| `sqrt`, `abs` | Square root, absolute value |
| `gamma`, `factorial` | Special functions |
| `erf`, `erfc` | Error functions |
| `besselJ`, `besselY` | Bessel functions |
| `zeta`, `digamma`, `polygamma` | Zeta and related |
| `gcd`, `lcm`, `isprime` | Number theory |
| `sign`, `floor`, `ceil`, `round` | Numeric |

## Common Issues

### Import Names

```typescript
// ✅ Correct - use Expression
import { Expression, symbols } from 'mathhook-node';

// ❌ Incorrect - JsExpression is an alias
import { JsExpression } from 'mathhook-node';  // Works but prefer Expression
```

### Symbols Function

```typescript
// ✅ Correct - destructure from array
const [x, y, z] = symbols('x y z');

// ✅ Also correct - space-separated
const [a, b] = symbols('a b');
```

### Expression Creation

```typescript
// ✅ Correct - use static methods
const sum = Expression.add([x, Expression.integer(2)]);

// ✅ Correct - use Expression.integer() for numbers
const five = Expression.integer(5);

// ❌ Incorrect - don't pass raw numbers
const wrong = Expression.add([x, 2]);  // Won't work
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
