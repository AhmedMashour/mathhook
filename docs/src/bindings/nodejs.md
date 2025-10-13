# Node.js/TypeScript API Guide

Complete guide to using MathHook from Node.js and TypeScript via NAPI bindings.

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

## Quick Start

### JavaScript

```javascript
const { Symbol, parse, simplify } = require('mathhook');

// Create symbols
const x = new Symbol('x');
const y = new Symbol('y');

// Build expressions
const expr = parse('x^2 + 2*x + 1');

// Simplify
const simplified = simplify(expr);
console.log(simplified.toString());  // (x + 1)^2
```

### TypeScript

```typescript
import { Symbol, Expression, parse, simplify } from 'mathhook';

// Create symbols (with type safety)
const x: Symbol = new Symbol('x');
const y: Symbol = new Symbol('y');

// Build expressions
const expr: Expression = parse('x^2 + 2*x + 1');

// Simplify
const simplified: Expression = simplify(expr);
console.log(simplified.toString());  // (x + 1)^2
```

## Why MathHook for Node.js?

### Performance Comparison

**Native Performance in JavaScript**:
- Rust core compiled to native code via NAPI
- No V8 overhead for mathematical operations
- 50-100x faster than pure JavaScript CAS libraries

```javascript
const { parse, simplify } = require('mathhook');

// Large polynomial expression
const terms = Array.from({length: 100}, (_, i) => `${i}*x^${i}`);
const exprStr = terms.join(' + ');

// MathHook
const start = Date.now();
const expr = parse(exprStr);
const result = simplify(expr);
const mathhookTime = Date.now() - start;

console.log(`MathHook: ${mathhookTime}ms`);
// Typical: MathHook 1-5ms vs JavaScript CAS 100-500ms
```

### When to Use MathHook

**Use MathHook when**:
- Building web applications with symbolic math (calculators, graphing, education)
- Server-side computation for math APIs
- Real-time symbolic computation requirements
- Need LaTeX parsing and rendering

**Integration Points**:
- Express/Fastify APIs for math endpoints
- Next.js/Nuxt.js server-side rendering
- WebSocket servers for interactive math applications
- GraphQL resolvers for mathematical queries

---

## API Reference

### Symbols

```typescript
import { Symbol } from 'mathhook';

const x = new Symbol('x');
const y = new Symbol('y');
const theta = new Symbol('theta');
```

**Equality**:
```typescript
const x1 = new Symbol('x');
const x2 = new Symbol('x');
console.log(x1.equals(x2));  // true (same name)
```

### Expressions

#### Creating Expressions

**Method 1: Parsing** (Recommended for Node.js)

```typescript
import { parse } from 'mathhook';

const expr = parse('x^2 + 2*x + 1');
const expr2 = parse('\\frac{x^2 + 1}{x - 1}');  // LaTeX
const expr3 = parse('sin(x) + cos(x)');
```

**Method 2: Builder API**

```typescript
import { Expression, Symbol } from 'mathhook';

const x = new Symbol('x');

// Build expressions programmatically
const expr = Expression.add([
    Expression.pow(x.toExpression(), Expression.integer(2)),
    Expression.mul([Expression.integer(2), x.toExpression()]),
    Expression.integer(1)
]);
```

**Method 3: From JSON** (for serialization)

```typescript
import { Expression } from 'mathhook';

const exprJson = {
    type: 'Add',
    terms: [
        { type: 'Symbol', name: 'x' },
        { type: 'Integer', value: 1 }
    ]
};

const expr = Expression.fromJSON(exprJson);
```

#### Expression Operations

**Simplification**:
```typescript
import { parse, simplify } from 'mathhook';

const expr = parse('x + x');
const result = simplify(expr);
console.log(result.toString());  // 2*x
```

**Expansion**:
```typescript
import { parse, expand } from 'mathhook';

const expr = parse('(x + 1)^2');
const result = expand(expr);
console.log(result.toString());  // x^2 + 2*x + 1
```

**Substitution**:
```typescript
import { Symbol, parse } from 'mathhook';

const x = new Symbol('x');
const expr = parse('x^2 + 2*x + 1');

// Substitute x = 3
const result = expr.substitute(x, Expression.integer(3));
console.log(result.toString());  // 16
```

### Calculus Operations

#### Derivatives

```typescript
import { Symbol, parse, derivative } from 'mathhook';

const x = new Symbol('x');
const expr = parse('x^3');

// First derivative
const df = derivative(expr, x);
console.log(df.toString());  // 3*x^2

// Second derivative
const d2f = derivative(expr, x, { order: 2 });
console.log(d2f.toString());  // 6*x
```

#### Integration

```typescript
import { Symbol, parse, integrate } from 'mathhook';

const x = new Symbol('x');
const expr = parse('x^2');

// Indefinite integral
const integral = integrate(expr, x);
console.log(integral.toString());  // x^3 / 3 + C

// Definite integral
const definite = integrate(expr, x, { lower: 0, upper: 2 });
console.log(definite.toString());  // 8/3
```

### Equation Solving

```typescript
import { Symbol, parse, solve } from 'mathhook';

const x = new Symbol('x');

// Quadratic equation: x^2 - 5*x + 6 = 0
const expr = parse('x^2 - 5*x + 6');
const solutions = solve(expr, x);

solutions.forEach(sol => {
    console.log(sol.toString());
});
// Output: x = 2, x = 3
```

### LaTeX Support

```typescript
import { parse, toLatex } from 'mathhook';

// Parse LaTeX
const expr = parse('\\frac{x^2 + 1}{x - 1}');

// Convert to LaTeX
const latex = toLatex(expr);
console.log(latex);  // \frac{x^{2} + 1}{x - 1}
```

---

## Integration Patterns

### Express.js API

```typescript
import express from 'express';
import { parse, simplify, toLatex } from 'mathhook';

const app = express();
app.use(express.json());

// Simplify endpoint
app.post('/api/simplify', (req, res) => {
    try {
        const { expression } = req.body;
        const expr = parse(expression);
        const simplified = simplify(expr);

        res.json({
            original: expression,
            simplified: simplified.toString(),
            latex: toLatex(simplified)
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
        const x = new Symbol(variable);
        const deriv = derivative(expr, x);

        res.json({
            expression: expression,
            derivative: deriv.toString(),
            latex: toLatex(deriv)
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

import { parse, simplify, derivative } from 'mathhook';

export async function simplifyExpression(expression: string) {
    try {
        const expr = parse(expression);
        const simplified = simplify(expr);
        return {
            success: true,
            result: simplified.toString()
        };
    } catch (error) {
        return {
            success: false,
            error: error.message
        };
    }
}

export async function computeDerivative(expression: string, variable: string) {
    try {
        const expr = parse(expression);
        const x = new Symbol(variable);
        const deriv = derivative(expr, x);
        return {
            success: true,
            result: deriv.toString()
        };
    } catch (error) {
        return {
            success: false,
            error: error.message
        };
    }
}
```

### React Component Example

```typescript
// components/Calculator.tsx
'use client';

import { useState } from 'react';
import { simplifyExpression } from '@/app/actions/math';

export default function Calculator() {
    const [input, setInput] = useState('');
    const [result, setResult] = useState('');

    const handleSimplify = async () => {
        const response = await simplifyExpression(input);
        if (response.success) {
            setResult(response.result);
        } else {
            setResult(`Error: ${response.error}`);
        }
    };

    return (
        <div>
            <input
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder="Enter expression (e.g., x^2 + 2*x + 1)"
            />
            <button onClick={handleSimplify}>Simplify</button>
            {result && <div>Result: {result}</div>}
        </div>
    );
}
```

### WebSocket Server

```typescript
import { WebSocketServer } from 'ws';
import { parse, simplify, derivative } from 'mathhook';

const wss = new WebSocketServer({ port: 8080 });

wss.on('connection', (ws) => {
    ws.on('message', (data) => {
        try {
            const request = JSON.parse(data.toString());

            switch (request.operation) {
                case 'simplify': {
                    const expr = parse(request.expression);
                    const result = simplify(expr);
                    ws.send(JSON.stringify({
                        operation: 'simplify',
                        result: result.toString()
                    }));
                    break;
                }
                case 'derivative': {
                    const expr = parse(request.expression);
                    const x = new Symbol(request.variable);
                    const result = derivative(expr, x);
                    ws.send(JSON.stringify({
                        operation: 'derivative',
                        result: result.toString()
                    }));
                    break;
                }
            }
        } catch (error) {
            ws.send(JSON.stringify({ error: error.message }));
        }
    });
});
```

---

## Advanced Features

### Async Operations

For long-running operations, use Worker Threads:

```typescript
import { Worker } from 'worker_threads';

// math-worker.ts
import { parentPort } from 'worker_threads';
import { parse, simplify } from 'mathhook';

parentPort?.on('message', (expression) => {
    try {
        const expr = parse(expression);
        const result = simplify(expr);
        parentPort?.postMessage({ success: true, result: result.toString() });
    } catch (error) {
        parentPort?.postMessage({ success: false, error: error.message });
    }
});

// main.ts
function simplifyAsync(expression: string): Promise<string> {
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
        worker.postMessage(expression);
    });
}
```

### Caching Results

```typescript
import { parse, simplify } from 'mathhook';
import NodeCache from 'node-cache';

const cache = new NodeCache({ stdTTL: 600 }); // 10 minutes

function simplifyWithCache(expression: string): string {
    const cached = cache.get<string>(expression);
    if (cached) {
        return cached;
    }

    const expr = parse(expression);
    const result = simplify(expr).toString();
    cache.set(expression, result);
    return result;
}
```

### Error Handling

```typescript
import { parse, MathError, DomainError } from 'mathhook';

try {
    const expr = parse('1/0');
    const evaluated = expr.evaluate();
} catch (error) {
    if (error instanceof DomainError) {
        console.error('Domain error:', error.message);
    } else if (error instanceof MathError) {
        console.error('Math error:', error.message);
    } else {
        console.error('Unknown error:', error);
    }
}
```

---

## Performance Best Practices

### 1. Parse Once, Use Many Times

```typescript
// GOOD: Parse once
const expr = parse('x^2 + 2*x + 1');
for (let i = 0; i < 1000; i++) {
    const result = expr.substitute(x, Expression.integer(i));
}

// BAD: Parse repeatedly
for (let i = 0; i < 1000; i++) {
    const expr = parse('x^2 + 2*x + 1');  // Wasteful!
    const result = expr.substitute(x, Expression.integer(i));
}
```

### 2. Use Caching for Repeated Operations

```typescript
const cache = new Map<string, Expression>();

function getCachedParse(expr: string): Expression {
    if (!cache.has(expr)) {
        cache.set(expr, parse(expr));
    }
    return cache.get(expr)!;
}
```

### 3. Batch Operations When Possible

```typescript
// GOOD: Batch process
const expressions = ['x^2', 'x + 1', '2*x'];
const results = expressions.map(e => simplify(parse(e)));

// Also consider parallel processing with Worker Threads for large batches
```

### 4. V8 Optimization Tips

```typescript
// Use consistent object shapes for V8 optimization
interface MathRequest {
    expression: string;
    operation: 'simplify' | 'derivative' | 'integrate';
    variable?: string;
}

function processMathRequest(req: MathRequest): string {
    // V8 can optimize this better with consistent types
    const expr = parse(req.expression);
    switch (req.operation) {
        case 'simplify':
            return simplify(expr).toString();
        case 'derivative':
            return derivative(expr, new Symbol(req.variable!)).toString();
        case 'integrate':
            return integrate(expr, new Symbol(req.variable!)).toString();
    }
}
```

---

## Testing

### Jest Example

```typescript
import { parse, simplify, derivative } from 'mathhook';

describe('MathHook Integration', () => {
    test('simplifies expression', () => {
        const expr = parse('x + x');
        const result = simplify(expr);
        expect(result.toString()).toBe('2*x');
    });

    test('computes derivative', () => {
        const expr = parse('x^2');
        const x = new Symbol('x');
        const deriv = derivative(expr, x);
        expect(deriv.toString()).toBe('2*x');
    });

    test('handles errors gracefully', () => {
        expect(() => parse('invalid syntax +')).toThrow();
    });
});
```

---

## Type Definitions

Full TypeScript type definitions:

```typescript
declare module 'mathhook' {
    export class Symbol {
        constructor(name: string);
        name(): string;
        equals(other: Symbol): boolean;
        toExpression(): Expression;
    }

    export class Expression {
        static integer(value: number): Expression;
        static rational(num: number, den: number): Expression;
        static float(value: number): Expression;
        static symbol(symbol: Symbol): Expression;
        static add(terms: Expression[]): Expression;
        static mul(factors: Expression[]): Expression;
        static pow(base: Expression, exp: Expression): Expression;
        static fromJSON(json: object): Expression;

        toString(): string;
        toJSON(): object;
        substitute(symbol: Symbol, value: Expression): Expression;
        evaluate(): number;
        equals(other: Expression): boolean;
    }

    export function parse(input: string): Expression;
    export function simplify(expr: Expression): Expression;
    export function expand(expr: Expression): Expression;
    export function factor(expr: Expression): Expression;
    export function derivative(expr: Expression, variable: Symbol, options?: { order?: number }): Expression;
    export function integrate(expr: Expression, variable: Symbol, options?: { lower?: number, upper?: number }): Expression;
    export function solve(expr: Expression, variable: Symbol): Expression[];
    export function toLatex(expr: Expression): string;
    export function toWolfram(expr: Expression): string;

    export class MathError extends Error {}
    export class DomainError extends MathError {}
    export class ParseError extends MathError {}
}
```

---

## Common Patterns

### REST API Math Service

Complete example of a production-ready math API:

```typescript
import express from 'express';
import { parse, simplify, derivative, integrate, solve, toLatex } from 'mathhook';
import { body, validationResult } from 'express-validator';

const app = express();
app.use(express.json());

// Validation middleware
const validateExpression = [
    body('expression').isString().notEmpty(),
    body('variable').optional().isString(),
];

// Simplify endpoint
app.post('/api/simplify',
    validateExpression,
    (req, res) => {
        const errors = validationResult(req);
        if (!errors.isEmpty()) {
            return res.status(400).json({ errors: errors.array() });
        }

        try {
            const expr = parse(req.body.expression);
            const simplified = simplify(expr);

            res.json({
                original: req.body.expression,
                simplified: simplified.toString(),
                latex: toLatex(simplified)
            });
        } catch (error) {
            res.status(400).json({ error: error.message });
        }
    }
);

// Rate limiting example
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100 // limit each IP to 100 requests per windowMs
});

app.use('/api/', limiter);

app.listen(3000);
```

---

## Migration from JavaScript CAS Libraries

### From Math.js

```javascript
// Math.js
const math = require('mathjs');
const expr = math.parse('x^2 + 2*x + 1');
const simplified = math.simplify(expr);

// MathHook
const { parse, simplify } = require('mathhook');
const expr = parse('x^2 + 2*x + 1');
const simplified = simplify(expr);
```

### From Algebrite

```javascript
// Algebrite
const Algebrite = require('algebrite');
const result = Algebrite.simplify('x + x').toString();

// MathHook
const { parse, simplify } = require('mathhook');
const result = simplify(parse('x + x')).toString();
```

---

## Next Steps

- [Python API Guide](./python.md) - Python bindings
- [LaTeX Parsing Guide](../parser/latex.md) - Advanced LaTeX support
- [Performance Benchmarks](../performance/benchmarking.md) - Detailed comparisons
- [Web Integration Examples](https://github.com/ahmedmashhour/mathhook-examples) - Production examples

---

## Getting Help

- **Documentation**: [https://mathhook.readthedocs.io](https://mathhook.readthedocs.io)
- **GitHub Issues**: Report bugs and request features
- **npm Package**: [https://www.npmjs.com/package/mathhook](https://www.npmjs.com/package/mathhook)
- **Discord**: Join our community for support
