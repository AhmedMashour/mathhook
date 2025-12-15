# Node.js/TypeScript API Guide

Complete guide to using MathHook from Node.js and TypeScript via NAPI bindings.

**Last Updated**: 2025-12-15T0200

## Installation

```bash
npm install mathhook
# or
yarn add mathhook
# or
pnpm add mathhook
```

**Requirements**:
- Node.js 16.0 or higher
- npm 7.0 or higher

**Platform Support**:
- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

**TypeScript Support**: Type definitions included (.d.ts files bundled)

---

## Quick Start

### JavaScript

```javascript
const { JsExpression, symbols, parse } = require('mathhook');

// Create symbols using symbols() function
const [x, y] = symbols('x y');

// Build expressions using method chaining
const expr = x.pow(2).add(x.multiply(2)).add(1);

// Simplify
const simplified = expr.simplify();
console.log(simplified.toString());  // x^2 + 2*x + 1

// Parse from string
const parsed = parse('sin(x)^2 + cos(x)^2');
console.log(parsed.simplify().toString());
```

### TypeScript

```typescript
import { JsExpression, symbols, parse, EvalContext } from 'mathhook';

// Create symbols
const [x, y] = symbols('x y');

// Build expressions with type safety
const expr = x.pow(2).add(x.multiply(2)).add(1);

// Simplify
const simplified = expr.simplify();
console.log(simplified.toString());
```

---

## Core Classes

### JsExpression

The main class for all symbolic expressions. Create expressions using static factory methods and manipulate them using instance methods.

#### Static Constructors

```typescript
import { JsExpression } from 'mathhook';

// Numbers
const num = JsExpression.integer(42);
const rat = JsExpression.rational(3, 4);     // 3/4
const flt = JsExpression.float(3.14159);

// Symbols
const x = JsExpression.symbol('x');
const alpha = JsExpression.symbol('alpha');

// Complex numbers
const z = JsExpression.complex(
    JsExpression.integer(3),
    JsExpression.integer(4)
);  // 3 + 4i

// Mathematical constants
const pi = JsExpression.pi();
const e = JsExpression.e();
const i = JsExpression.i();
const phi = JsExpression.goldenRatio();
const gamma = JsExpression.eulerGamma();

// Parse from string (auto-detects format)
const expr1 = JsExpression.parse('x^2 + 2*x + 1');
const expr2 = JsExpression.parse('\\frac{x}{2}');  // LaTeX
const expr3 = JsExpression.parse('Sin[x]');        // Wolfram

// Functions
const sinX = JsExpression.function('sin', [x]);
const logXY = JsExpression.function('log', [x, y]);

// Equations
const equation = JsExpression.equation(x, JsExpression.integer(5));
```

#### Arithmetic Methods

All arithmetic methods accept either a `JsExpression` or a number (auto-converted):

```typescript
const x = JsExpression.symbol('x');

// Addition
const sum1 = x.add(JsExpression.integer(2));  // x + 2
const sum2 = x.add(2);                         // x + 2 (auto-convert)

// Subtraction
const diff = x.subtract(3);                    // x - 3

// Multiplication
const prod = x.multiply(4);                    // 4*x

// Division
const quot = x.divide(2);                      // x/2

// Power
const pow1 = x.pow(2);                         // x^2
const pow2 = x.pow(JsExpression.rational(1, 2)); // sqrt(x)

// Negation
const neg = x.negate();                        // -x
```

#### Algebraic Operations

```typescript
const x = JsExpression.symbol('x');

// Simplification
const expr = x.add(x).add(x);
console.log(expr.simplify().toString());  // 3*x

// Expansion
const binomial = x.add(1).pow(2);
console.log(binomial.expand().toString());  // x^2 + 2*x + 1

// Factorization
const poly = x.pow(2).subtract(1);
console.log(poly.factor().toString());  // (x - 1)*(x + 1)

// Collect terms
const mixed = x.add(x).add(1);
console.log(mixed.collect('x').toString());

// Substitution
const expr2 = x.pow(2).add(x.multiply(2)).add(1);
const result = expr2.substitute({ x: JsExpression.integer(3) });
console.log(result.toString());  // 16
```

#### Calculus Operations

```typescript
const x = JsExpression.symbol('x');

// First derivative
const f = x.pow(3);
const df = f.derivative('x');
console.log(df.toString());  // 3*x^2

// Higher-order derivatives
const d2f = f.nthDerivative('x', 2);
console.log(d2f.toString());  // 6*x

// Indefinite integral
const integral = f.integrate('x');
console.log(integral.toString());  // x^4/4

// Definite integral
const lower = JsExpression.integer(0);
const upper = JsExpression.integer(1);
const definite = f.integrateDefinite('x', lower, upper);
console.log(definite.toString());  // 1/4

// Limits
const sinc = JsExpression.function('sin', [x]).divide(x);
const lim = sinc.limit('x', JsExpression.integer(0));
console.log(lim.toString());  // 1

// Limit at infinity
const reciprocal = JsExpression.integer(1).divide(x);
const limInf = reciprocal.limitInfinity('x');
console.log(limInf.toString());  // 0

// Taylor series
const sinX = JsExpression.function('sin', [x]);
const series = sinX.series('x', JsExpression.integer(0), 5);
console.log(series.toString());

// Partial derivatives
const y = JsExpression.symbol('y');
const fxy = x.multiply(y);
const partial = fxy.partialDerivative(['x', 'y']);
console.log(partial.toString());  // 1
```

#### Matrix Operations

```typescript
const { JsExpression } = require('mathhook');

// Create matrix from rows
const a = JsExpression.integer(1);
const b = JsExpression.integer(2);
const c = JsExpression.integer(3);
const d = JsExpression.integer(4);
const matrix = JsExpression.matrix([[a, b], [c, d]]);

// Special matrices
const identity = JsExpression.identityMatrix(3);  // 3x3 identity
const zeros = JsExpression.zeroMatrix(2, 3);      // 2x3 zeros

// Determinant
const det = matrix.determinant();
console.log(det.toString());  // -2

// Inverse
const inv = matrix.inverse();

// Transpose
const transposed = matrix.transpose();

// Matrix decompositions
const lu = matrix.luDecomposition();
console.log(lu.l.toString());  // Lower triangular
console.log(lu.u.toString());  // Upper triangular
if (lu.p) console.log(lu.p.toString());  // Permutation (if pivoting)

const qr = matrix.qrDecomposition();
console.log(qr.q.toString());  // Orthogonal
console.log(qr.r.toString());  // Upper triangular

const svd = matrix.svdDecomposition();
console.log(svd.u.toString());     // Left singular vectors
console.log(svd.sigma.toString()); // Singular values
console.log(svd.vt.toString());    // Right singular vectors (transposed)

// Cholesky (for positive definite matrices)
const pd = JsExpression.matrix([
    [JsExpression.integer(4), JsExpression.integer(2)],
    [JsExpression.integer(2), JsExpression.integer(3)]
]);
const chol = pd.choleskyDecomposition();
```

#### Polynomial Operations

```typescript
const x = JsExpression.symbol('x');

// Polynomial degree
const poly = x.pow(3).add(x.pow(2)).add(x);
const deg = poly.polynomialDegree('x');
console.log(deg);  // 3

// Leading coefficient
const lc = poly.polynomialLeadingCoefficient('x');

// Content (GCD of coefficients)
const scaled = JsExpression.integer(6).multiply(x.pow(2))
    .add(JsExpression.integer(9).multiply(x));
const content = scaled.polynomialContent();
console.log(content.toString());  // 3

// Primitive part
const primitive = scaled.polynomialPrimitivePart();

// Resultant (for elimination)
const p1 = x.pow(2).subtract(1);
const p2 = x.subtract(1);
const res = JsExpression.resultant(p1, p2, 'x');
console.log(res.toString());  // 0 (share root x=1)

// Discriminant
const quadratic = x.pow(2).add(x.multiply(2)).add(1);
const disc = JsExpression.discriminant(quadratic, 'x');
```

#### Output Formats

```typescript
const x = JsExpression.symbol('x');
const expr = x.pow(2).divide(2);

// String representation
console.log(expr.toString());    // x^2/2

// Simple notation
console.log(expr.toSimple());    // x^2/2

// LaTeX
console.log(expr.toLatex());     // \frac{x^{2}}{2}

// Wolfram Language
console.log(expr.toWolfram());   // Divide[Power[x, 2], 2]
```

#### Educational Features (Step-by-Step)

```typescript
const x = JsExpression.symbol('x');
const expr = JsExpression.integer(2).add(JsExpression.integer(3));

// Get simplification steps
const explanation = expr.explainSimplification();
explanation.steps.forEach((step, i) => {
    console.log(`Step ${i + 1}: ${step.title}`);
    console.log(`  ${step.description}`);
    console.log(`  Before: ${step.before}`);
    console.log(`  After: ${step.after}`);
});

// Get derivative steps
const f = x.pow(2);
const derivSteps = f.derivativeWithSteps('x');
derivSteps.steps.forEach((step, i) => {
    console.log(`Step ${i + 1}: ${step.title}`);
});
```

---

## Standalone Functions

### Symbol Creation

```typescript
import { symbols } from 'mathhook';

// Space-separated
const [x, y, z] = symbols('x y z');

// Comma-separated
const [a, b, c] = symbols('a, b, c');

// Range syntax
const [x0, x1, x2] = symbols('x0:3');
```

### Parsing

```typescript
import { parse } from 'mathhook';

// Standard notation
const expr1 = parse('x^2 + 2*x + 1');

// Implicit multiplication
const expr2 = parse('2x + 3y');  // Same as 2*x + 3*y

// LaTeX (auto-detected)
const expr3 = parse('\\frac{x^2}{2}');

// Wolfram (auto-detected)
const expr4 = parse('Sin[x] + Cos[y]');

// Functions
const expr5 = parse('sin(x) + cos(y)');

// Greek letters
const expr6 = parse('alpha + beta');
```

### Mathematical Functions

#### Trigonometric

```typescript
import { sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, symbols } from 'mathhook';

const [x] = symbols('x');

const sinX = sin(x);     // sin(x)
const cosX = cos(x);     // cos(x)
const tanX = tan(x);     // tan(x)

const asinX = asin(x);   // arcsin(x)
const acosX = acos(x);   // arccos(x)
const atanX = atan(x);   // arctan(x)

const sinhX = sinh(x);   // sinh(x)
const coshX = cosh(x);   // cosh(x)
const tanhX = tanh(x);   // tanh(x)

// Also accepts numbers
const sinHalf = sin(0.5);
```

#### Exponential and Logarithmic

```typescript
import { exp, ln, log10, sqrt, symbols } from 'mathhook';

const [x] = symbols('x');

const expX = exp(x);     // e^x
const lnX = ln(x);       // ln(x)
const log10X = log10(x); // log10(x)
const sqrtX = sqrt(x);   // sqrt(x) = x^(1/2)

// With numbers
const sqrt4 = sqrt(4);   // 2
```

#### Rounding and Sign

```typescript
import { abs, sign, floor, ceil, round, symbols } from 'mathhook';

const [x] = symbols('x');

const absX = abs(x);     // |x|
const signX = sign(x);   // sign(x)
const floorX = floor(x); // floor(x)
const ceilX = ceil(x);   // ceil(x)
const roundX = round(x); // round(x)

// With numbers
const abs5 = abs(-5);    // 5
const sign5 = sign(-5);  // -1
```

#### Special Functions

```typescript
import { gamma, factorial, digamma, zeta, erf, erfc, symbols } from 'mathhook';

const [x] = symbols('x');

// Gamma function
const gammaX = gamma(x);

// Factorial
const fact5 = factorial(5);  // 120

// Digamma (logarithmic derivative of gamma)
const digammaX = digamma(x);

// Riemann zeta function
const zeta2 = zeta(2);  // pi^2/6

// Error functions
const erfX = erf(x);
const erfcX = erfc(x);  // 1 - erf(x)
```

#### Advanced Special Functions

```typescript
import { polygamma, bessel_j, bessel_y, beta, symbols } from 'mathhook';

const [x, n] = symbols('x n');

// Polygamma (nth derivative of digamma)
const trigamma = polygamma(1, x);  // Trigamma function

// Bessel functions
const j0 = bessel_j(0, x);  // Bessel J_0(x)
const y0 = bessel_y(0, x);  // Bessel Y_0(x)

// Beta function
const betaVal = beta(2, 3);  // B(2,3)
```

#### Number Theory

```typescript
import { gcd, lcm, modulo, isprime } from 'mathhook';

// Greatest common divisor
const g = gcd(12, 18);  // 6

// Least common multiple
const l = lcm(4, 6);    // 12

// Modulo
const m = modulo(17, 5); // 2

// Primality test
const p1 = isprime(17);  // true
const p2 = isprime(18);  // false
```

#### Polynomial Functions

```typescript
import { degree, roots, symbols, parse } from 'mathhook';

const [x] = symbols('x');

// Polynomial degree
const poly = parse('x^3 + 2*x^2 + x + 1');
const deg = degree(poly, 'x');

// Polynomial roots
const quadratic = parse('x^2 - 5*x + 6');
const r = roots(quadratic, 'x');
```

---

## Advanced Classes

### EvalContext

Controls evaluation behavior for `evaluateWithContext()`:

```typescript
import { JsExpression, EvalContext, symbols } from 'mathhook';

const [x, y] = symbols('x y');
const expr = x.pow(2).add(y);

// Numerical evaluation with substitutions
const ctxNumeric = EvalContext.numeric([
    ['x', JsExpression.integer(3)],
    ['y', JsExpression.integer(4)]
]);
const result = expr.evaluateWithContext(ctxNumeric);
console.log(result.toString());  // 13

// Symbolic evaluation (no numerical conversion)
const ctxSymbolic = EvalContext.symbolic();
const symbolic = expr.evaluateWithContext(ctxSymbolic);
console.log(symbolic.toString());  // x^2 + y (stays symbolic)

// Custom configuration
const ctxCustom = new EvalContext({
    numeric: true,
    precision: 128,
    simplifyFirst: true
});

// Method chaining
const ctx = EvalContext.symbolic()
    .withPrecision(128)
    .withSimplifyFirst(true);
```

**Configuration Options**:
- `numeric`: Perform numerical evaluation (default: true)
- `precision`: Bits of precision (default: 53 for f64)
- `simplifyFirst`: Simplify before evaluation (default: true)

### JsMathSolver

For solving algebraic equations:

```typescript
import { JsMathSolver, JsExpression } from 'mathhook';

const solver = new JsMathSolver();

// Create equation: x = 5
const x = JsExpression.symbol('x');
const equation = JsExpression.equation(x, JsExpression.integer(5));

// Solve
const result = solver.solve(equation, 'x');

console.log(result.resultType);  // 'single', 'multiple', 'no_solution', or 'infinite_solutions'
console.log(result.solutions);   // ['5']
console.log(result.count);       // 1
console.log(result.metadata);    // 'Single solution found'
```

**Result Types**:
- `'single'`: One solution found
- `'multiple'`: Multiple solutions (for polynomials)
- `'no_solution'`: No solution exists
- `'infinite_solutions'`: Identity equation (always true)

### JsPDESolver

For solving partial differential equations:

```typescript
import { JsPDESolver, JsExpression } from 'mathhook';

const solver = new JsPDESolver();

// Heat equation: du/dt = alpha * d^2u/dx^2
const alpha = JsExpression.integer(1);
const heat = solver.solveHeatEquation('u', 'x', 't', alpha);
console.log(heat.solution);
console.log(heat.method);      // 'Separation of Variables (Heat Equation)'
console.log(heat.eigenvalues); // Array of eigenvalues
console.log(heat.coefficients); // Fourier coefficients

// Wave equation: d^2u/dt^2 = c^2 * d^2u/dx^2
const c = JsExpression.integer(1);
const wave = solver.solveWaveEquation('u', 'x', 't', c);
console.log(wave.solution);
console.log(wave.method);  // 'Separation of Variables (Wave Equation)'

// Laplace equation: d^2u/dx^2 + d^2u/dy^2 = 0
const laplace = solver.solveLaplaceEquation('u', 'x', 'y');
console.log(laplace.solution);
console.log(laplace.method);  // 'Separation of Variables (Laplace Equation)'
```

### Groebner Basis

```typescript
import { groebnerBasis, JsExpression, symbols } from 'mathhook';

const [x, y] = symbols('x y');

// System of polynomial equations
const p1 = x.pow(2).add(y.pow(2)).subtract(1);
const p2 = x.subtract(y);

// Compute Groebner basis
// Order: 'lex' (lexicographic), 'grlex' (graded lex), 'grevlex' (graded reverse lex)
const basis = groebnerBasis([p1, p2], ['x', 'y'], 'lex');

basis.forEach((poly, i) => {
    console.log(`Basis ${i + 1}: ${poly.toString()}`);
});
```

---

## Integration Patterns

### Express.js API

```typescript
import express from 'express';
import { parse, symbols, JsExpression } from 'mathhook';

const app = express();
app.use(express.json());

// Simplify endpoint
app.post('/api/simplify', (req, res) => {
    try {
        const { expression } = req.body;
        const expr = parse(expression);
        const simplified = expr.simplify();

        res.json({
            original: expression,
            simplified: simplified.toString(),
            latex: simplified.toLatex()
        });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

// Derivative endpoint
app.post('/api/derivative', (req, res) => {
    try {
        const { expression, variable } = req.body;
        const expr = parse(expression);
        const deriv = expr.derivative(variable);

        res.json({
            expression,
            derivative: deriv.toString(),
            latex: deriv.toLatex()
        });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

app.listen(3000, () => {
    console.log('Math API running on port 3000');
});
```

### Next.js Server Actions

```typescript
// app/actions/math.ts
'use server';

import { parse } from 'mathhook';

export async function simplifyExpression(expression: string) {
    try {
        const expr = parse(expression);
        const simplified = expr.simplify();
        return {
            success: true,
            result: simplified.toString(),
            latex: simplified.toLatex()
        };
    } catch (error) {
        return {
            success: false,
            error: (error as Error).message
        };
    }
}

export async function computeDerivative(expression: string, variable: string) {
    try {
        const expr = parse(expression);
        const deriv = expr.derivative(variable);
        return {
            success: true,
            result: deriv.toString(),
            latex: deriv.toLatex()
        };
    } catch (error) {
        return {
            success: false,
            error: (error as Error).message
        };
    }
}
```

### Worker Threads for Heavy Computation

```typescript
// math-worker.ts
import { parentPort } from 'worker_threads';
import { parse } from 'mathhook';

parentPort?.on('message', (data) => {
    try {
        const expr = parse(data.expression);
        let result;

        switch (data.operation) {
            case 'simplify':
                result = expr.simplify();
                break;
            case 'expand':
                result = expr.expand();
                break;
            case 'derivative':
                result = expr.derivative(data.variable);
                break;
            default:
                throw new Error(`Unknown operation: ${data.operation}`);
        }

        parentPort?.postMessage({
            success: true,
            result: result.toString(),
            latex: result.toLatex()
        });
    } catch (error) {
        parentPort?.postMessage({
            success: false,
            error: (error as Error).message
        });
    }
});

// main.ts
import { Worker } from 'worker_threads';

function computeAsync(expression: string, operation: string, variable?: string): Promise<string> {
    return new Promise((resolve, reject) => {
        const worker = new Worker('./math-worker.ts');
        worker.on('message', (result) => {
            if (result.success) {
                resolve(result.result);
            } else {
                reject(new Error(result.error));
            }
            worker.terminate();
        });
        worker.postMessage({ expression, operation, variable });
    });
}
```

---

## Complete API Reference

### JsExpression Static Methods

| Method | Description |
|--------|-------------|
| `integer(value)` | Create integer expression |
| `rational(num, den)` | Create rational number |
| `float(value)` | Create floating-point number |
| `complex(real, imag)` | Create complex number |
| `symbol(name)` | Create symbolic variable |
| `parse(input)` | Parse string (LaTeX/Wolfram/standard) |
| `pi()` | Mathematical constant pi |
| `e()` | Euler's number e |
| `i()` | Imaginary unit i |
| `goldenRatio()` | Golden ratio phi |
| `eulerGamma()` | Euler-Mascheroni constant |
| `function(name, args)` | Create function expression |
| `equation(left, right)` | Create equation |
| `matrix(rows)` | Create matrix |
| `identityMatrix(size)` | Create identity matrix |
| `zeroMatrix(rows, cols)` | Create zero matrix |
| `resultant(p1, p2, var)` | Polynomial resultant |
| `discriminant(poly, var)` | Polynomial discriminant |

### JsExpression Instance Methods

| Method | Description |
|--------|-------------|
| `add(other)` | Addition |
| `subtract(other)` | Subtraction |
| `multiply(other)` | Multiplication |
| `divide(other)` | Division |
| `pow(exp)` | Power/exponentiation |
| `negate()` | Negation |
| `simplify()` | Algebraic simplification |
| `expand()` | Expand expression |
| `factor()` | Factor expression |
| `collect(var)` | Collect terms |
| `substitute(subs)` | Substitute variables |
| `evaluate()` | Numerical evaluation |
| `evaluateWithContext(ctx)` | Evaluation with context |
| `derivative(var)` | First derivative |
| `nthDerivative(var, n)` | Nth derivative |
| `integrate(var)` | Indefinite integral |
| `integrateDefinite(var, lo, hi)` | Definite integral |
| `limit(var, value)` | Limit at point |
| `limitInfinity(var)` | Limit at infinity |
| `series(var, point, order)` | Taylor series |
| `partialDerivative(vars)` | Partial derivatives |
| `determinant()` | Matrix determinant |
| `inverse()` | Matrix inverse |
| `transpose()` | Matrix transpose |
| `luDecomposition()` | LU decomposition |
| `qrDecomposition()` | QR decomposition |
| `svdDecomposition()` | SVD decomposition |
| `choleskyDecomposition()` | Cholesky decomposition |
| `polynomialDegree(var)` | Polynomial degree |
| `polynomialLeadingCoefficient(var)` | Leading coefficient |
| `polynomialContent()` | Content (GCD of coeffs) |
| `polynomialPrimitivePart()` | Primitive part |
| `toString()` | String representation |
| `toSimple()` | Simple notation |
| `toLatex()` | LaTeX notation |
| `toWolfram()` | Wolfram notation |
| `explainSimplification()` | Step-by-step simplification |
| `derivativeWithSteps(var)` | Step-by-step derivative |

### Standalone Functions

| Function | Description |
|----------|-------------|
| `symbols(names)` | Create multiple symbols |
| `parse(expr)` | Parse expression string |
| `groebnerBasis(polys, vars, order)` | Compute Groebner basis |
| `sqrt(x)` | Square root |
| `sin(x)`, `cos(x)`, `tan(x)` | Trigonometric |
| `asin(x)`, `acos(x)`, `atan(x)` | Inverse trig |
| `sinh(x)`, `cosh(x)`, `tanh(x)` | Hyperbolic |
| `exp(x)`, `ln(x)`, `log10(x)` | Exponential/log |
| `abs(x)`, `sign(x)` | Absolute/sign |
| `floor(x)`, `ceil(x)`, `round(x)` | Rounding |
| `gamma(x)`, `factorial(x)` | Gamma functions |
| `digamma(x)`, `polygamma(n, x)` | Digamma functions |
| `zeta(x)` | Riemann zeta |
| `erf(x)`, `erfc(x)` | Error functions |
| `bessel_j(n, x)`, `bessel_y(n, x)` | Bessel functions |
| `beta(a, b)` | Beta function |
| `gcd(a, b)`, `lcm(a, b)` | Number theory |
| `modulo(a, b)` | Modulo operation |
| `isprime(n)` | Primality test |
| `degree(poly, var)` | Polynomial degree |
| `roots(poly, var)` | Polynomial roots |

---

## Performance Best Practices

### 1. Parse Once, Use Many

```typescript
// GOOD: Parse once
const expr = parse('x^2 + 2*x + 1');
for (let i = 0; i < 1000; i++) {
    const result = expr.substitute({ x: JsExpression.integer(i) });
}

// BAD: Parse repeatedly
for (let i = 0; i < 1000; i++) {
    const expr = parse('x^2 + 2*x + 1');  // Wasteful!
}
```

### 2. Cache Parsed Expressions

```typescript
const cache = new Map<string, JsExpression>();

function getCachedParse(exprStr: string): JsExpression {
    if (!cache.has(exprStr)) {
        cache.set(exprStr, parse(exprStr));
    }
    return cache.get(exprStr)!;
}
```

### 3. Use Worker Threads for Heavy Operations

```typescript
// For computationally intensive operations, use worker threads
// to avoid blocking the event loop
```

---

## Error Handling

```typescript
import { parse } from 'mathhook';

try {
    const expr = parse('invalid syntax +');
} catch (error) {
    console.error('Parse error:', (error as Error).message);
}

// Decompositions can fail
const matrix = JsExpression.matrix([[...]]);
try {
    const lu = matrix.luDecomposition();
} catch (error) {
    console.error('Decomposition failed:', (error as Error).message);
    // Matrix may be singular
}
```

---

## Next Steps

- [Python API Guide](./python.md) - Python bindings
- [Rust API Guide](./rust.md) - Direct Rust usage
- [Performance Guide](../performance/benchmarking.md) - Optimization tips

---

## Getting Help

- **Documentation**: [https://mathhook.readthedocs.io](https://mathhook.readthedocs.io)
- **GitHub Issues**: Report bugs and request features
- **npm Package**: [https://www.npmjs.com/package/mathhook](https://www.npmjs.com/package/mathhook)
