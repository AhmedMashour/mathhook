# MathHook Node.js/TypeScript Usage Guide

Updated: 2025-12-27T23:45

Comprehensive guide for using MathHook in Node.js and TypeScript projects.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Symbol and Expression Creation](#symbol-and-expression-creation)
4. [Static Expression Methods](#static-expression-methods)
5. [Mathematical Functions](#mathematical-functions)
6. [Expression Methods](#expression-methods)
7. [Algebraic Operations](#algebraic-operations)
8. [Calculus](#calculus)
9. [Equation Solving](#equation-solving)
10. [Parsing and Formatting](#parsing-and-formatting)
11. [TypeScript Integration](#typescript-integration)
12. [Performance Tips](#performance-tips)
13. [Error Handling](#error-handling)

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
const {
  Expression,
  symbols,
  parse,
  SmartEquationSolver,
  sin,
  cos
} = require('mathhook-node');

// Create symbols
const [x, y] = symbols('x y');

// Build expressions using static methods
const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([x, Expression.integer(2)]),
  Expression.integer(1)
]);

// Simplify and format
console.log(expr.simplify().format());

// Solve equations
const solver = new SmartEquationSolver();
const solutions = solver.solveWithEquation(
  Expression.add([Expression.pow(x, Expression.integer(2)), Expression.integer(-4)]),
  'x'
);
console.log(solutions);
```

### JavaScript (ESM)

```javascript
import { Expression, symbols, parse, sin, cos } from 'mathhook-node';

const [x, y] = symbols('x y');
const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(1)
]);
console.log(expr.simplify().format());
```

### TypeScript

```typescript
import { Expression, symbols, parse, sin, cos } from 'mathhook-node';

const [x, y] = symbols('x y');
const expr: Expression = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([x, Expression.integer(2)]),
  Expression.integer(1)
]);
console.log(expr.simplify().format());
```

## Symbol and Expression Creation

### Creating Symbols

```typescript
import { symbol, symbols } from 'mathhook-node';

// Single symbol
const x = symbol('x');
const y = symbol('y');

// Multiple symbols (returns array)
const [a, b, c] = symbols('a b c');
const [x1, x2, x3] = symbols('x1 x2 x3');
```

### Creating Expressions

```typescript
import { Expression } from 'mathhook-node';

// Integers
const two = Expression.integer(2);
const fortyTwo = Expression.integer(42);

// Floats
const pi = Expression.float(3.14159);

// Rationals
const half = Expression.rational(1, 2);
const threeQuarters = Expression.rational(3, 4);
```

## Static Expression Methods

### Arithmetic Operations

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x, y] = symbols('x y');

// Addition - takes array of terms
const sum = Expression.add([x, y, Expression.integer(1)]);

// Multiplication - takes array of factors
const product = Expression.mul([x, y, Expression.integer(2)]);

// Power
const squared = Expression.pow(x, Expression.integer(2));
const cubed = Expression.pow(x, Expression.integer(3));

// Division
const quotient = Expression.div(x, y);

// Safe division (returns null on divide by zero)
const safeQuotient = Expression.divChecked(x, y);
```

### Building Complex Expressions

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

// Polynomial: x^2 + 2x + 1
const polynomial = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([Expression.integer(2), x]),
  Expression.integer(1)
]);

// Rational expression: (x + 1) / (x - 1)
const rational = Expression.div(
  Expression.add([x, Expression.integer(1)]),
  Expression.add([x, Expression.integer(-1)])
);

// Nested: ((x + 1)^2) / x
const nested = Expression.div(
  Expression.pow(
    Expression.add([x, Expression.integer(1)]),
    Expression.integer(2)
  ),
  x
);
```

## Mathematical Functions

### Trigonometric Functions

```typescript
import { symbols, sin, cos, tan, asin, acos, atan, sinh, cosh, tanh } from 'mathhook-node';

const [x] = symbols('x');

// Basic trigonometric
const sine = sin(x);
const cosine = cos(x);
const tangent = tan(x);

// Inverse trigonometric
const arcsine = asin(x);
const arccosine = acos(x);
const arctangent = atan(x);

// Hyperbolic
const hypSine = sinh(x);
const hypCosine = cosh(x);
const hypTangent = tanh(x);
```

### Exponential and Logarithmic

```typescript
import { symbols, exp, log, ln, sqrt, Expression } from 'mathhook-node';

const [x] = symbols('x');

// Exponential
const exponential = exp(x);

// Natural logarithm
const naturalLog = ln(x);

// Logarithm with base
const log10 = log(x, 10);
const log2 = log(x, 2);

// Square root
const squareRoot = sqrt(x);
```

### Special Functions

```typescript
import {
  symbols,
  gamma,
  factorial,
  erf,
  erfc,
  besselJ,
  besselY,
  beta,
  zeta,
  digamma,
  Expression
} from 'mathhook-node';

const [x, n] = symbols('x n');

// Gamma function
const gammaX = gamma(x);

// Factorial
const fact5 = factorial(Expression.integer(5));  // 120

// Error functions
const errorFunc = erf(x);
const compErrorFunc = erfc(x);

// Bessel functions
const besselFirst = besselJ(n, x);
const besselSecond = besselY(n, x);

// Beta function
const betaFunc = beta(x, n);

// Zeta function
const zetaFunc = zeta(x);

// Digamma function
const psi = digamma(x);
```

### Polynomial Functions

```typescript
import {
  expandLegendreSymbolic,
  expandChebyshevFirstSymbolic,
  expandChebyshevSecondSymbolic,
  expandHermiteSymbolic,
  expandLaguerreSymbolic
} from 'mathhook-node';

// Orthogonal polynomials (symbolic expansion)
const legendre5 = expandLegendreSymbolic(5);
const chebyshev1_4 = expandChebyshevFirstSymbolic(4);
const chebyshev2_4 = expandChebyshevSecondSymbolic(4);
const hermite3 = expandHermiteSymbolic(3);
const laguerre3 = expandLaguerreSymbolic(3);
```

## Expression Methods

### Simplification

```typescript
import { Expression, symbols, sin, cos } from 'mathhook-node';

const [x] = symbols('x');

// Basic simplification
const expr = Expression.add([x, x, x]);
const simplified = expr.simplify();
console.log(simplified.format());  // 3*x

// Trigonometric simplification
const trig = Expression.add([
  Expression.pow(sin(x), Expression.integer(2)),
  Expression.pow(cos(x), Expression.integer(2))
]);
console.log(trig.simplifyTrigonometric().format());  // 1

// Rational simplification
const rational = Expression.div(
  Expression.add([
    Expression.pow(x, Expression.integer(2)),
    Expression.integer(-1)
  ]),
  Expression.add([x, Expression.integer(-1)])
);
console.log(rational.simplifyRational().format());  // x + 1

// Logarithm simplification
const logExpr = ln(exp(x));
console.log(logExpr.simplifyLogarithms().format());  // x
```

### Expansion

```typescript
import { Expression, symbols, expand } from 'mathhook-node';

const [x, y] = symbols('x y');

// Method call
const expr = Expression.pow(
  Expression.add([x, Expression.integer(1)]),
  Expression.integer(2)
);
const expanded = expr.expand();
console.log(expanded.format());  // x^2 + 2*x + 1

// Or use the function
const expanded2 = expand(expr);
```

### Factorization

```typescript
import { Expression, symbols, factor } from 'mathhook-node';

const [x] = symbols('x');

// Factor difference of squares
const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(-1)
]);
const factored = expr.factor();
console.log(factored.format());  // (x - 1)(x + 1)

// Factor out GCD
const expr2 = Expression.add([
  Expression.mul([Expression.integer(2), Expression.pow(x, Expression.integer(2))]),
  Expression.mul([Expression.integer(4), x])
]);
const factoredGcd = expr2.factorGcd();
console.log(factoredGcd.format());
```

### Substitution

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x, y] = symbols('x y');

const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  x,
  Expression.integer(1)
]);

// Substitute value
const result = expr.substitute({ x: Expression.integer(3) });
console.log(result.simplify().format());  // 13
```

### Formatting

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');
const expr = Expression.div(
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(2)
);

// Default format
console.log(expr.format());  // x^2/2

// Format as LaTeX
console.log(expr.formatAs('latex'));  // \frac{x^{2}}{2}

// Format as Wolfram
console.log(expr.formatAs('wolfram'));  // Divide[Power[x, 2], 2]
```

## Calculus

### Derivatives

```typescript
import { Expression, symbols, sin, cos, exp } from 'mathhook-node';

const [x] = symbols('x');

// First derivative
const expr = Expression.pow(x, Expression.integer(3));
const derivative = expr.derivative(x);
console.log(derivative.format());  // 3*x^2

// Higher-order derivatives
const secondDeriv = expr.nthDerivative(x, 2);
console.log(secondDeriv.format());  // 6*x

// Chain rule
const sinX2 = sin(Expression.pow(x, Expression.integer(2)));
console.log(sinX2.derivative(x).format());  // 2*x*cos(x^2)
```

### Integration

```typescript
import { Expression, symbols, sin } from 'mathhook-node';

const [x] = symbols('x');

// Indefinite integral
const expr = Expression.pow(x, Expression.integer(2));
const integral = expr.integrate(x, 0);
console.log(integral.format());  // x^3/3

// Trigonometric integral
const sinIntegral = sin(x).integrate(x, 0);
console.log(sinIntegral.format());  // -cos(x)
```

### Limits

```typescript
import { Expression, symbols, sin } from 'mathhook-node';

const [x] = symbols('x');

// Limit at a point
const expr = Expression.div(sin(x), x);
const lim = expr.limit(x, Expression.integer(0));
console.log(lim.format());  // 1

// Limit at infinity
const expr2 = Expression.div(Expression.integer(1), x);
const limInf = expr2.limitAtInfinity(x);
console.log(limInf.format());  // 0

// Limit at negative infinity
const limNegInf = expr2.limitAtNegativeInfinity(x);
console.log(limNegInf.format());  // 0
```

### Series Expansion

```typescript
import { Expression, symbols, sin, exp } from 'mathhook-node';

const [x] = symbols('x');

// Taylor series
const sinSeries = sin(x).taylorSeries(x, Expression.integer(0), 5);
console.log(sinSeries.format());  // x - x^3/6 + x^5/120

// Maclaurin series
const expMaclaurin = exp(x).maclaurinSeries(x, 4);
console.log(expMaclaurin.format());
```

### Summation

```typescript
import { Expression, symbols } from 'mathhook-node';

const [k] = symbols('k');

// Finite sum
const expr = Expression.pow(k, Expression.integer(2));
const finiteSum = expr.finiteSum(k, Expression.integer(1), Expression.integer(10));
console.log(finiteSum.simplify().format());

// Power sum formula
const powerSum = Expression.powerSum(Expression.integer(2), Expression.integer(10));
console.log(powerSum.format());
```

## Equation Solving

### SmartEquationSolver

```typescript
import { Expression, symbols, SmartEquationSolver } from 'mathhook-node';

const [x] = symbols('x');
const solver = new SmartEquationSolver();

// Linear equation: 2x + 3 = 7 → 2x - 4 = 0
const linear = Expression.add([
  Expression.mul([Expression.integer(2), x]),
  Expression.integer(-4)
]);
const linSolutions = solver.solveWithEquation(linear, 'x');
console.log(linSolutions);  // [2]

// Quadratic: x^2 - 4 = 0
const quadratic = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(-4)
]);
const quadSolutions = solver.solveWithEquation(quadratic, 'x');
console.log(quadSolutions);  // [2, -2]

// Cubic: x^3 - 8 = 0
const cubic = Expression.add([
  Expression.pow(x, Expression.integer(3)),
  Expression.integer(-8)
]);
const cubicSolutions = solver.solveWithEquation(cubic, 'x');
console.log(cubicSolutions);  // [2, complex roots...]
```

### Direct Expression Solving

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

// Direct linear solve (fast path)
const linearExpr = Expression.add([
  Expression.mul([Expression.integer(2), x]),
  Expression.integer(-4)
]);
const linearResult = linearExpr.solveLinear(x);
console.log(linearResult.solutions);

// Direct quadratic solve (fast path)
const quadExpr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(-4)
]);
const quadResult = quadExpr.solveQuadratic(x);
console.log(quadResult.solutions);

// Direct polynomial solve (fast path)
const polyExpr = Expression.add([
  Expression.pow(x, Expression.integer(4)),
  Expression.integer(-16)
]);
const polyResult = polyExpr.solvePolynomial(x);
console.log(polyResult.solutions);
```

## Parsing and Formatting

### Parsing Expressions

```typescript
import { parse } from 'mathhook-node';

// Standard notation
const expr1 = parse('x^2 + 2*x + 1');

// Implicit multiplication
const expr2 = parse('2x');  // Parsed as 2*x
const expr3 = parse('xy');  // Parsed as x*y

// Function calls
const expr4 = parse('sin(x) + cos(x)');
const expr5 = parse('exp(x) * log(y)');

// LaTeX notation
const expr6 = parse('\\frac{x^2}{2}');
const expr7 = parse('\\sqrt{x + 1}');

// Complex expressions
const expr8 = parse('sin(x)^2 + cos(x)^2');
const expr9 = parse('(x + 1) / (x - 1)');
```

### Formatting Output

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');
const expr = Expression.div(
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(2)
);

// Default format
console.log(expr.format());  // x^2/2

// LaTeX format
console.log(expr.formatAs('latex'));  // \frac{x^{2}}{2}

// Wolfram format
console.log(expr.formatAs('wolfram'));  // Divide[Power[x, 2], 2]

// Simple format
console.log(expr.formatAs('simple'));  // x^2/2
```

## TypeScript Integration

### Type-Safe Functions

```typescript
import { Expression, symbols } from 'mathhook-node';

function polynomial(
  coefficients: number[],
  variable: Expression
): Expression {
  const terms = coefficients.map((coef, power) =>
    Expression.mul([
      Expression.integer(coef),
      Expression.pow(variable, Expression.integer(power))
    ])
  );
  return Expression.add(terms);
}

const [x] = symbols('x');
const poly = polynomial([1, 2, 3], x);  // 1 + 2x + 3x^2
console.log(poly.format());
```

### Working with Results

```typescript
import { Expression, symbols, SmartEquationSolver } from 'mathhook-node';

const [x] = symbols('x');
const solver = new SmartEquationSolver();

const equation = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(-4)
]);

const result = solver.solveWithEquation(equation, 'x');

// Result is SolverResult with solutions array
if (result.solutions.length > 0) {
  console.log(`Found ${result.solutions.length} solutions:`);
  for (const solution of result.solutions) {
    console.log(`  x = ${solution.format()}`);
  }
}
```

## Performance Tips

### Reuse Symbols

```typescript
import { symbols, Expression } from 'mathhook-node';

// Create symbols once
const [x, y] = symbols('x y');

// Reuse them
const expr1 = Expression.add([Expression.pow(x, Expression.integer(2)), y]);
const expr2 = Expression.add([x, Expression.pow(y, Expression.integer(2))]);
```

### Use Fast Path Solvers

```typescript
import { symbols, Expression } from 'mathhook-node';

const [x] = symbols('x');
const quadratic = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.integer(-4)
]);

// Fast path (skips classification)
const result = quadratic.solveQuadratic(x);

// Slower (classifies equation first)
// const solver = new SmartEquationSolver();
// const result = solver.solveWithEquation(quadratic, 'x');
```

### Simplify Incrementally

```typescript
import { symbols, Expression } from 'mathhook-node';

const [x] = symbols('x');

// Simplify at key stages, not every operation
const step1 = Expression.pow(
  Expression.add([x, Expression.integer(1)]),
  Expression.integer(10)
);
const step2 = step1.expand();
const result = step2.simplify();  // Simplify once at the end
```

### Batch Operations

```typescript
import { parse } from 'mathhook-node';

// Parse all expressions first
const expressions = [
  'x^2 + 1',
  'x^3 + x',
  'x^4 + x^2 + 1'
].map(parse);

// Process in batch
const simplified = expressions.map(e => e.simplify());
```

## Error Handling

### Try-Catch Pattern

```typescript
import { parse } from 'mathhook-node';

try {
  const expr = parse('x +* y');  // Invalid syntax
} catch (error) {
  console.error('Parse error:', error);
}
```

### Safe Division

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

// Safe division returns null on zero
const safeResult = Expression.divChecked(
  x,
  Expression.integer(0)
);

if (safeResult === null) {
  console.log('Division by zero');
} else {
  console.log(safeResult.format());
}
```

### Validation

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

function safeDivide(
  numerator: Expression,
  denominator: Expression
): Expression | null {
  // Check for symbolic zero
  const simplified = denominator.simplify();
  const formatted = simplified.format();

  if (formatted === '0') {
    console.error('Division by zero detected');
    return null;
  }

  return Expression.div(numerator, denominator);
}
```

## Common Patterns

### Building Polynomials

```typescript
import { Expression, symbols } from 'mathhook-node';

function buildPolynomial(
  coeffs: number[],
  x: Expression
): Expression {
  const terms = coeffs.map((coef, power) => {
    if (power === 0) {
      return Expression.integer(coef);
    }
    return Expression.mul([
      Expression.integer(coef),
      Expression.pow(x, Expression.integer(power))
    ]);
  });
  return Expression.add(terms);
}

const [x] = symbols('x');
const poly = buildPolynomial([1, -2, 1], x);  // 1 - 2x + x^2
console.log(poly.format());
```

### Expression Evaluator

```typescript
import { Expression, symbols } from 'mathhook-node';

function evaluateAt(
  expr: Expression,
  values: Record<string, number>
): Expression {
  const substitutions: Record<string, Expression> = {};
  for (const [varName, value] of Object.entries(values)) {
    substitutions[varName] = Expression.float(value);
  }
  return expr.substitute(substitutions).simplify();
}

const [x, y] = symbols('x y');
const expr = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.pow(y, Expression.integer(2))
]);

const result = evaluateAt(expr, { x: 3, y: 4 });
console.log(result.format());  // 25
```

### Derivative Calculator

```typescript
import { Expression, symbols, symbol } from 'mathhook-node';

function computeDerivatives(
  expr: Expression,
  varName: string,
  maxOrder: number
): Expression[] {
  const variable = symbol(varName);
  const derivatives: Expression[] = [expr];

  let current = expr;
  for (let i = 1; i <= maxOrder; i++) {
    current = current.derivative(variable);
    derivatives.push(current.simplify());
  }

  return derivatives;
}

const [x] = symbols('x');
const expr = Expression.pow(x, Expression.integer(4));
const derivs = computeDerivatives(expr, 'x', 4);

derivs.forEach((d, i) => {
  console.log(`d^${i}/dx^${i}: ${d.format()}`);
});
```

## Advanced Features

### Step-by-Step Explanations

```typescript
import { Expression, symbols, explainPowerRule } from 'mathhook-node';

const [x] = symbols('x');
const base = x;
const exponent = Expression.integer(3);

// Get step-by-step derivative explanation
const explanation = explainPowerRule(base, exponent, x);
console.log(explanation);
```

### Polynomial Operations

```typescript
import { Expression, symbols } from 'mathhook-node';

const [x] = symbols('x');

const poly1 = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  x,
  Expression.integer(1)
]);

const poly2 = Expression.add([
  x,
  Expression.integer(1)
]);

// Polynomial multiplication
const product = Expression.multiplyPolynomials(poly1, poly2);
console.log(product.expand().format());

// Polynomial addition
const sum = Expression.addPolynomials(poly1, poly2);
console.log(sum.simplify().format());

// Polynomial division
const quotient = poly1.polynomialQuotient(poly2);
const remainder = poly1.polynomialRemainder(poly2);
```

### Matrix Operations

```typescript
import { Expression, symbols } from 'mathhook-node';

const [a, b, c, d] = symbols('a b c d');

// Create matrix from expressions
const matrix = Expression.matrix([
  [a, b],
  [c, d]
]);

// Matrix operations
const transpose = matrix.matrixTranspose();
const determinant = matrix.matrixDeterminant();
const inverse = matrix.matrixInverse();

console.log('Transpose:', transpose.format());
console.log('Determinant:', determinant.format());
```

## Further Reading

- [API Reference](https://mathhook.readthedocs.io)
- [TypeScript Definitions](index.d.ts)
- [Examples](examples/)
- [Rust Core Documentation](../../README.md)
- [Performance Benchmarks](../../docs/benchmarks.md)
