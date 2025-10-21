# Plan 4: Node.js API to Production

**Priority**: 🟢 HIGH
**Timeline**: 8-10 weeks
**Waves**: 6
**Orchestrator**: `/sc:spawn`
**Version**: V2 (Added Wave 0: Async POC & Simplified Wave 3)

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

**Your Mission**: Execute a 6-wave plan to bring MathHook Node.js API from 20% complete to production-ready npm package (Wave 0 validates async architecture).

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

### Wave 0: Async POC & Architecture Validation (1 week)

**Objective**: Prototype async/Promise API and validate architecture BEFORE full implementation to ensure Node.js idioms work correctly.

**Critical Success Criteria**:
- ✅ Async POC implemented with NAPI-RS Promises
- ✅ Async-first architecture validated (not sync-first with async wrappers)
- ✅ Performance benchmarks show async doesn't add significant overhead
- ✅ TypeScript types for async API prototyped

**Tasks**:

1. **Async Architecture Research** (1 day):
   - Study NAPI-RS async/Promise patterns
   - Identify which operations benefit from async (computationally expensive ones)
   - Document sync vs async API design decision
   - Examples from ecosystem:
     - Sharp (image processing) - async by default
     - SQLite - sync and async APIs
     - Bcrypt - async for hashing
   - **Decision**: Make expensive operations async-first (solve, integrate, simplify large expressions)

2. **Implement Async POC** (2 days):
   - Create minimal async API for 3-4 operations:
     ```rust
     #[napi]
     impl JsExpression {
         #[napi(ts_return_type = "Promise<Expression>")]
         pub async fn simplify_async(&self) -> napi::Result<JsExpression> {
             // Spawn tokio task for Rust async
             let inner = self.inner.clone();
             let simplified = tokio::task::spawn_blocking(move || {
                 inner.simplify()
             }).await?;
             Ok(JsExpression::from(simplified))
         }

         #[napi(ts_return_type = "Promise<Expression[]>")]
         pub async fn solve_async(&self, var: &JsSymbol) -> napi::Result<Vec<JsExpression>> {
             // Async solve for expensive equation solving
         }
     }
     ```

   - Prototype TypeScript types:
     ```typescript
     // mathhook.d.ts
     export class Expression {
         simplifyAsync(): Promise<Expression>;
         solveAsync(variable: Symbol): Promise<Expression[]>;
         // Keep sync versions for simple operations
         toString(): string;
         toLatex(): string;
     }
     ```

3. **Performance Benchmarks** (2 days):
   - Compare async vs sync overhead:
     ```typescript
     import { symbol, solve, solveAsync } from 'mathhook';

     const x = symbol('x');
     const equation = x.pow(10).subtract(1); // Polynomial of degree 10

     // Sync API
     console.time('sync');
     const syncResult = solve(equation, x);
     console.timeEnd('sync');

     // Async API
     console.time('async');
     const asyncResult = await solveAsync(equation, x);
     console.timeEnd('async');
     ```

   - Acceptance criteria: Async overhead <10% for same operation
   - Document when to use async vs sync:
     - **Async**: solve(), integrate(), simplify(large expressions), factor()
     - **Sync**: toString(), toLatex(), basic arithmetic, small expressions

4. **API Design Freeze** (1 day):
   - Decision gate: **Only proceed to Wave 1 if async POC proves viable**
   - If overhead >10%: Revisit architecture (consider sync-first with optional async)
   - Document final API design in `NODEJS_API_DESIGN.md`
   - Freeze: Which operations are async, which are sync
   - TypeScript type definitions finalized

**Agent Delegation**:
```bash
/sc:spawn backend-architect "Implement Wave 0: Async POC & Architecture Validation"
```

**Agent Prompt**:
```markdown
**Context**: You are the `backend-architect` agent for MathHook CAS project.

Prototype and validate async/Promise API architecture for Node.js bindings.

**Goal**: Ensure async API is performant and follows Node.js idioms BEFORE full implementation.

**Tasks**:

1. **Research async patterns**:
   - Study NAPI-RS async/Promise documentation
   - Review Sharp, SQLite, Bcrypt async patterns
   - Identify operations that benefit from async

2. **Implement POC**:
   - Create async versions of 3-4 operations (simplify, solve, integrate, factor)
   - Use tokio::spawn_blocking for CPU-bound work
   - Generate TypeScript .d.ts types
   - Test in Node.js with simple examples

3. **Benchmark performance**:
   - Measure async overhead vs sync
   - Target: <10% overhead
   - Document results in `.mathhook_sessions/nodejs_async_benchmark_results.md`

4. **API design freeze**:
   - Decision: Which operations async, which sync
   - Document in `NODEJS_API_DESIGN.md`
   - Freeze TypeScript type signatures

**Deliverables**:
- Async POC implementation (3-4 operations)
- Benchmark results showing <10% overhead
- API design document
- TypeScript type definitions

**Quality Target**: 9+/10 - Proven async architecture with validated performance
```

**Verification Script** (`verify_wave_0_nodejs_async_poc.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 0 Verification: Async POC ==="

# 1. Check POC implementation
if [ ! -f "crates/mathhook-node/src/async_poc.rs" ]; then
    echo "❌ FAIL: Async POC implementation not found"
    exit 1
fi
echo "✅ POC implemented"

# 2. Build POC
echo "Building async POC..."
cd crates/mathhook-node
npm run build
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Build failed"
    exit 1
fi
echo "✅ Build successful"

# 3. Run benchmark
if [ ! -f "../../.mathhook_sessions/nodejs_async_benchmark_results.md" ]; then
    echo "❌ FAIL: Benchmark results not found"
    exit 1
fi

# Check overhead is <10%
OVERHEAD=$(grep "Async Overhead:" ../../.mathhook_sessions/nodejs_async_benchmark_results.md | awk '{print $3}' | tr -d '%')
if (( $(echo "$OVERHEAD > 10" | bc -l) )); then
    echo "❌ FAIL: Async overhead $OVERHEAD% > 10%"
    echo "⚠️  Architecture needs revision"
    exit 1
fi
echo "✅ Async overhead: $OVERHEAD% (< 10% target)"

# 4. Check API design document
if [ ! -f "../../NODEJS_API_DESIGN.md" ]; then
    echo "❌ FAIL: API design document not found"
    exit 1
fi
echo "✅ API design frozen"

# 5. Check TypeScript types
if [ ! -f "mathhook.d.ts" ]; then
    echo "❌ FAIL: TypeScript types not generated"
    exit 1
fi

if ! grep -q "Promise<Expression>" mathhook.d.ts; then
    echo "❌ FAIL: Async types not found in TypeScript definitions"
    exit 1
fi
echo "✅ TypeScript types include async signatures"

# 6. Test async API
echo "Testing async API..."
node -e "
const m = require('.');
const x = m.symbol('x');
const eq = x.pow(2).subtract(5).multiply(x).add(6);
(async () => {
    const result = await m.solveAsync(eq, x);
    console.log('Async solve works:', result.length > 0);
})();
" 2>&1 | grep -q "Async solve works: true"
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Async API test failed"
    exit 1
fi
echo "✅ Async API functional"

cd ../..
echo ""
echo "=== Wave 0 Verification: PASSED ==="
echo "✅ Proceed to Wave 1: Build Infrastructure & npm Package"
```

**Deliverables**:
- `crates/mathhook-node/src/async_poc.rs`: Async API proof of concept
- `.mathhook_sessions/nodejs_async_benchmark_results.md`: Performance benchmarks
- `NODEJS_API_DESIGN.md`: Frozen API design with async/sync decisions
- `crates/mathhook-node/mathhook.d.ts`: TypeScript type definitions

**Exit Criteria**:
- [ ] Async POC implemented for 3-4 operations
- [ ] Async overhead <10% validated with benchmarks
- [ ] API design frozen (which operations async, which sync)
- [ ] TypeScript types generated and tested

**Risks**:
- High async overhead may force sync-first architecture (mitigation: use tokio efficiently)
- NAPI-RS async complexity may be prohibitive (mitigation: research community patterns first)
- TypeScript type generation may be manual work (acceptable if automated generation fails)

**Dependencies**: None (Wave 0 is foundation)

**Unblocks**: Wave 1 (build infrastructure can now implement proven async patterns)

**Critical Insight**: Async-first architecture must be proven performant BEFORE full implementation. Wave 0 prevents discovering fundamental async issues in Wave 3 when APIs are already built.

---

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

### Wave 3: Async Operations & Error Handling (6-8 hours)

**Goal**: Proper Node.js async patterns and error handling (streams deferred to future post-v1.0 release)

**Simplified Focus**: Core async patterns ONLY. Streams and worker threads add complexity without proven demand.

**Node.js-Specific Features**:

1. **Async Operations** (for expensive computations):
   ```typescript
   // Long-running operations don't block event loop
   const solution = await solve(complexEquation, x, { timeout: 30000 });

   // Progress callbacks (simple callback, not streams)
   const result = await integrate(expr, x, {
       onProgress: (percent) => console.log(`${percent}% complete`)
   });

   // Batch operations (simple Promise.all, not streams)
   const results = await Promise.all([
       solve(eq1, x),
       solve(eq2, y),
       solve(eq3, z)
   ]);
   ```

2. **Error Handling** (Node.js conventions):
   ```typescript
   try {
       const result = await solve(equation, x);
   } catch (err) {
       if (err instanceof MathHookError) {
           console.error(`Math error: ${err.message}`);
           console.error(`Error code: ${err.code}`); // e.g., 'DOMAIN_ERROR', 'NO_SOLUTION'
       }
   }

   // Custom error classes
   export class MathHookError extends Error {
       code: string;
       context?: any;
   }

   export class DomainError extends MathHookError {
       code = 'DOMAIN_ERROR';
   }

   export class NoSolutionError extends MathHookError {
       code = 'NO_SOLUTION';
   }
   ```

3. **Timeout and Cancellation**:
   ```typescript
   import { solve, AbortError } from 'mathhook';

   const controller = new AbortController();

   // Cancel after 5 seconds
   setTimeout(() => controller.abort(), 5000);

   try {
       const result = await solve(equation, x, { signal: controller.signal });
   } catch (err) {
       if (err instanceof AbortError) {
           console.log('Operation cancelled');
       }
   }
   ```

**Deliverables**:
- Async/await support for expensive operations
- Progress callback support (simple, not streams)
- Error classes with codes
- Timeout and cancellation support
- **DEFERRED**: Streams API (post-v1.0 if user demand exists)
- **DEFERRED**: Worker thread integration (post-v1.0 if needed)

**Rationale for Simplification**:
- Streams add significant complexity (readable, writable, transform streams)
- Worker threads require careful state management and serialization
- No proven user demand for these features yet
- Focus on getting core async patterns right first
- Can add streams/workers in v1.1+ based on user feedback

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

### Wave Completion Checklist
- [ ] Wave 0: Async POC validated, architecture frozen (<10% overhead)
- [ ] Wave 1: `npm install mathhook` works
- [ ] Wave 2: First-class TypeScript support, method chaining
- [ ] Wave 3: Async/await for expensive operations, error handling
- [ ] Wave 4: Complete feature parity with core
- [ ] Wave 5: Published to npm, >90% test coverage, Deno/Bun compatible

### Quality Metrics
- All waves score ≥ 8/10
- Async overhead <10% (validated in Wave 0)
- >90% test coverage
- Deno and Bun compatibility tested
- TypeScript types complete and validated

### Deliverables Checklist
- [ ] Wave 0: Async POC, performance benchmarks, API design document
- [ ] Working npm install
- [ ] TypeScript types and declarations
- [ ] Async/await API
- [ ] Error handling with custom classes
- [ ] Complete feature parity
- [ ] npm publication
- [ ] Comprehensive tests
- [ ] Deno/Bun support
- [ ] Complete documentation

**Exit Criteria**: Node.js developers can use MathHook as easily as Python developers, with proper async patterns and TypeScript types.

**Timeline**: 8-10 weeks to production npm package (added Wave 0 for async validation, simplified Wave 3)

**Simplification Note**: Streams API and Worker Thread integration deferred to post-v1.0 release based on user demand.
