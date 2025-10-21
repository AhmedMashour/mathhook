# Plan 4: Node.js API to Production

**Priority**: 🟢 HIGH
**Timeline**: 7-9 weeks
**Waves**: 5
**Orchestrator**: `/sc:spawn`

## Executive Summary

**Current State**: 20% complete (mirrors Python limitations)
- ✅ NAPI-RS infrastructure setup
- ❌ Same 15 basic methods as Python
- ❌ Missing: TypeScript types, async/Promise API, npm package, Node.js idioms

**Goal**: Production-ready npm package with proper Node.js patterns

**User's Emphasis**: "please also have node with python" - Node.js is NOT just a Python mirror, needs independent development with Node.js-specific features

**Market Opportunity**: JavaScript/TypeScript ecosystem, web-based CAS, serverless functions, Deno/Bun compatibility

---

## Bootstrap Command

```bash
/sc:spawn backend-architect "Execute Wave-Based Node.js API Production Plan for MathHook"
```

**Orchestrator Prompt**:

```markdown
You are the Orchestrator for **MathHook Node.js API Production**.

**Context**: You are the `backend-architect` agent - Expert Node.js/TypeScript developer specializing in creating production-ready npm packages with native bindings via NAPI-RS.

**Your Mission**: Execute a 5-wave plan to bring MathHook Node.js API from 20% complete to production-ready npm package.

**Mandatory Reading** (in this order):
1. `/Users/ahmedmashhour/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven wave-based methodology
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` - Project constraints (Rust backend)
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_4_NODEJS_API_PRODUCTION.md` - This plan

**5 Mandatory Rules**:
1. **You Are Always The Orchestrator** - Delegate to backend-architect agents
2. **Sequential Waves, Parallel Agents** - Complete waves in order
3. **Mandatory Verification** - Each wave ends with verification
4. **Strict CLAUDE.md Enforcement** - Follow documentation standards for Rust bindings
5. **Maintain Momentum** - Report after each wave
```

---

## Wave Breakdown

### Wave 1: Build Infrastructure & npm Package (4-6 hours)

**Goal**: Make `npm install mathhook` work

**Tasks**:
1. Build with NAPI-RS: `cd crates/mathhook-node && npm run build`
2. Test import: `node -e "const m = require('.'); console.log(m.Expression.integer(42).toString())"`
3. Fix build errors
4. Create package.json for npm publication
5. Test in clean node_modules

**Deliverables**:
- Working `npm install`
- TypeScript declaration files (.d.ts)
- Native module loading works
- npm-ready structure

---

### Wave 2: TypeScript Types & Node.js Ergonomics (10-14 hours)

**Goal**: First-class TypeScript support with Node.js patterns

**Current** (basic):
```typescript
const x = Expression.symbol("x");
const result = Expression.add(x, Expression.integer(2));
```

**Target** (Node.js idioms):
```typescript
import { symbol, symbols, solve, simplify } from 'mathhook';

const x = symbol('x');
const [a, b, c] = symbols('a', 'b', 'c');

// Operator overloading isn't possible in JS, but:
const expr = x.add(2).multiply(3).pow(2);  // Method chaining

// Async/Promise API for long computations
const result = await solve(equation, x, { async: true });
```

**Tasks**:
1. **TypeScript Declarations**:
   ```typescript
   export class Expression {
       add(other: Expression | number | string): Expression;
       multiply(other: Expression | number | string): Expression;
       pow(exponent: Expression | number): Expression;
       simplify(options?: SimplifyOptions): Expression;
       toString(): string;
       toLatex(): string;
   }

   export function symbol(name: string): Expression;
   export function symbols(...names: string[]): Expression[];
   export function solve(
       equation: Expression | string,
       variable: Expression | string,
       options?: SolveOptions
   ): Expression[] | Promise<Expression[]>;
   ```

2. **Method Chaining** (builder pattern):
   ```rust
   #[napi]
   impl JsExpression {
       #[napi]
       pub fn add(&self, other: JsExpression) -> Self {
           // Return new JsExpression for chaining
       }
   }
   ```

3. **Async/Promise API**:
   ```rust
   #[napi]
   impl JsExpression {
       #[napi(ts_return_type = "Promise<Expression>")]
       pub async fn simplify_async(&self) -> napi::Result<JsExpression> {
           // Async Rust -> JS Promise
       }
   }
   ```

4. **Conversion Helpers**:
   ```typescript
   function parse(expr: string | Expression): Expression;
   function toNumber(expr: Expression): number | null;
   ```

**Deliverables**:
- Complete TypeScript types
- Method chaining API
- Async operations
- Conversion utilities

---

### Wave 3: Async Operations & Streams (8-12 hours)

**Goal**: Proper Node.js async patterns and streaming

**Node.js-Specific Features**:

1. **Async Operations** (for expensive computations):
   ```typescript
   // Long-running operations don't block event loop
   const solution = await solve(complexEquation, x, { timeout: 30000 });

   // Progress callbacks
   const result = await integrate(expr, x, {
       onProgress: (percent) => console.log(`${percent}% complete`)
   });
   ```

2. **Streams API** (for batch processing):
   ```typescript
   import { createSimplifyStream } from 'mathhook';

   const stream = createSimplifyStream();
   expressionsArray.forEach(expr => stream.write(expr));
   stream.on('data', simplified => console.log(simplified));
   stream.end();
   ```

3. **Worker Threads** (for parallel computation):
   ```typescript
   import { solveInWorker } from 'mathhook/parallel';

   // Uses Node.js worker threads for CPU-intensive tasks
   const solutions = await Promise.all([
       solveInWorker(eq1, 'x'),
       solveInWorker(eq2, 'y'),
       solveInWorker(eq3, 'z')
   ]);
   ```

4. **Error Handling** (Node.js conventions):
   ```typescript
   try {
       const result = solve(equation, x);
   } catch (err) {
       if (err instanceof MathHookError) {
           console.error(`Math error: ${err.message}`);
       }
   }
   ```

**Deliverables**:
- Async/await support
- Streaming API
- Worker thread integration
- Proper error classes

---

### Wave 4: Calculus & Mathematical Functions (12-16 hours)

**Goal**: Complete feature parity with core, Node.js style

**Tasks**: Same as Python Wave 4, but with:
- TypeScript types for all functions
- Async versions for expensive operations
- Promise-based API

**Example**:
```typescript
import { derivative, integrate, limit, series } from 'mathhook';

const x = symbol('x');
const f = x.pow(2).multiply(sin(x));

// All return Promise for consistency
const df = await derivative(f, x);
const integral = await integrate(f, x);
const lim = await limit(f, x, 0);
```

**Deliverables**:
- Complete calculus API
- All math functions
- Matrix operations
- TypeScript types

---

### Wave 5: Testing & npm Publication (8-10 hours)

**Goal**: Production npm package

**Tasks**:
1. **Tests** (Jest/Vitest):
   ```typescript
   import { symbol, solve } from 'mathhook';

   test('quadratic equation', () => {
       const x = symbol('x');
       const eq = x.pow(2).add(x.multiply(-5)).add(6);
       const solutions = solve(eq, x);
       expect(solutions).toHaveLength(2);
   });
   ```

2. **Deno/Bun Compatibility**:
   - Test with Deno
   - Test with Bun
   - Document compatibility

3. **Documentation**:
   - README with quickstart
   - TypeDoc API reference
   - Migration from mathjs

4. **npm Publication**:
   ```bash
   npm publish
   ```

5. **CI/CD**: GitHub Actions for testing + publishing

**Deliverables**:
- >90% test coverage
- npm package published
- Deno/Bun support
- Complete documentation

---

## Node.js-Specific Advantages

**Why Node.js is NOT just a Python mirror:**

1. **Async by Default**: Node.js event loop makes async operations natural
2. **Streaming**: Native stream support for batch processing
3. **Worker Threads**: Parallel computation without GIL limitations
4. **Web Integration**: Easy integration with web servers, APIs, serverless
5. **Type Safety**: First-class TypeScript support
6. **Ecosystem**: Integration with Express, Next.js, Electron, etc.

**Use Cases Python Can't Handle Well:**
- Real-time math server (WebSocket)
- Serverless CAS (AWS Lambda, Vercel Functions)
- Electron desktop apps with CAS
- Browser-based math (via WASM future)

---

## Final Success Criteria

- [ ] `npm install mathhook` works
- [ ] First-class TypeScript support
- [ ] Async/await for all operations
- [ ] Streaming API
- [ ] Complete feature parity with core
- [ ] Published to npm
- [ ] >90% test coverage
- [ ] Deno/Bun compatible

**Exit Criteria**: Node.js developers can use MathHook as easily as Python developers, with proper async patterns and TypeScript types.

**Timeline**: 7-9 weeks to production npm package
