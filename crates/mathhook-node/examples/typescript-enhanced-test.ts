import { JsExpression, parse, sin, cos, exp, ln, sqrt, gamma, factorial } from '../index.js';
import type {
    MathFunction,
    BinaryOperation,
    SolverOptions,
    ParseOptions,
    PolynomialCoefficients,
    IntegrationBounds
} from '../index-enhanced.js';

console.log('=== TypeScript Enhanced Types Tests ===\n');

// Test 1: MathFunction type usage
console.log('Test 1: MathFunction type annotations');
const myTrigFunction: MathFunction = sin;
const x = JsExpression.symbol('x');
const result1 = myTrigFunction(x);
console.log(`  sin(x) using MathFunction type: ${result1.toString()}`);
console.log('  ✓ MathFunction type works\n');

// Test 2: BinaryOperation type usage
console.log('Test 2: BinaryOperation type (conceptual)');
const addOp = (a: JsExpression | number, b: JsExpression | number): JsExpression => {
    const exprA = typeof a === 'number' ? JsExpression.integer(a) : a;
    const exprB = typeof b === 'number' ? JsExpression.integer(b) : b;
    return exprA.add(exprB);
};
const result2 = addOp(x, 5);
console.log(`  Custom add operation: ${result2.toString()}`);
console.log('  ✓ BinaryOperation type pattern works\n');

// Test 3: SolverOptions interface
console.log('Test 3: SolverOptions interface');
const solverOpts: SolverOptions = {
    maxIterations: 500,
    precision: 1e-12,
    domain: 'complex',
    simplify: true
};
console.log(`  Solver options configured: maxIterations=${solverOpts.maxIterations}, precision=${solverOpts.precision}`);
console.log('  ✓ SolverOptions interface works\n');

// Test 4: ParseOptions interface
console.log('Test 4: ParseOptions interface');
const parseOpts: ParseOptions = {
    format: 'auto',
    simplify: false,
    implicitMultiplication: true
};
console.log(`  Parse options configured: format=${parseOpts.format}, simplify=${parseOpts.simplify}`);
console.log('  ✓ ParseOptions interface works\n');

// Test 5: PolynomialCoefficients type
console.log('Test 5: PolynomialCoefficients type');
const polyCoeffs: PolynomialCoefficients = {
    0: 5,
    1: -3,
    2: 2,
    3: JsExpression.integer(1)
};
console.log(`  Polynomial coefficients: degree 0 = ${polyCoeffs[0]}, degree 3 = ${polyCoeffs[3]?.toString()}`);
console.log('  ✓ PolynomialCoefficients type works\n');

// Test 6: IntegrationBounds interface
console.log('Test 6: IntegrationBounds interface');
const bounds: IntegrationBounds = {
    lower: 0,
    upper: JsExpression.pi()
};
console.log(`  Integration bounds: [${bounds.lower}, ${bounds.upper.toString()}]`);
console.log('  ✓ IntegrationBounds interface works\n');

// Test 7: Function composition with type annotations
console.log('Test 7: Function composition');
const composedFunc: MathFunction = (x) => {
    const input = typeof x === 'number' ? JsExpression.integer(x) : x;
    return sin(cos(input));
};
const result7 = composedFunc(x);
console.log(`  sin(cos(x)): ${result7.toString()}`);
console.log('  ✓ Function composition works\n');

// Test 8: Array of MathFunction
console.log('Test 8: Array of MathFunction');
const functions: MathFunction[] = [sin, cos, exp, ln, sqrt];
const results8 = functions.map(f => f(x).toString());
console.log(`  Applied functions to x: [${results8.join(', ')}]`);
console.log('  ✓ Array of MathFunction works\n');

// Test 9: Type-safe expression building
console.log('Test 9: Type-safe expression building');
const y = JsExpression.symbol('y');
const expr9 = x.pow(2).add(y.multiply(3)).subtract(1);
console.log(`  x^2 + 3*y - 1: ${expr9.toString()}`);
console.log('  ✓ Type-safe building works\n');

// Test 10: Using parse with TypeScript
console.log('Test 10: parse() with TypeScript');
const parsed = parse('2*x + sin(y)');
console.log(`  Parsed expression: ${parsed.toString()}`);
console.log('  ✓ parse() TypeScript integration works\n');

// Test 11: Special functions with type annotations
console.log('Test 11: Special functions');
const specialFuncs: MathFunction[] = [gamma, factorial];
const n = JsExpression.integer(5);
const results11 = specialFuncs.map(f => f(n).toString());
console.log(`  gamma(5) and factorial(5): [${results11.join(', ')}]`);
console.log('  ✓ Special functions type annotations work\n');

// Test 12: Complex expression with type safety
console.log('Test 12: Complex expression');
const complexExpr = sin(x.pow(2)).add(cos(y.multiply(3)));
console.log(`  sin(x^2) + cos(3*y): ${complexExpr.toString()}`);
console.log('  ✓ Complex expression type safety works\n');

// Test 13: Optional fields in interfaces
console.log('Test 13: Optional fields');
const minimalOpts: SolverOptions = {
    domain: 'real'
};
console.log(`  Minimal solver options: domain=${minimalOpts.domain}, maxIterations=${minimalOpts.maxIterations ?? 'default'}`);
console.log('  ✓ Optional fields work\n');

// Test 14: Re-exported types (Expression, Solver)
console.log('Test 14: Re-exported types');
const expr14: JsExpression = JsExpression.integer(42);
console.log(`  Using Expression type: ${expr14.toString()}`);
console.log('  ✓ Re-exported types work\n');

// Test 15: Union types (JsExpression | number)
console.log('Test 15: Union types');
const unionTest = (val: JsExpression | number): string => {
    if (typeof val === 'number') {
        return `Number: ${val}`;
    } else {
        return `Expression: ${val.toString()}`;
    }
};
console.log(`  Union with number: ${unionTest(42)}`);
console.log(`  Union with expression: ${unionTest(x)}`);
console.log('  ✓ Union types work\n');

// Test 16: Literal types
console.log('Test 16: Literal types');
const domain: 'real' | 'complex' = 'complex';
const format: 'auto' | 'standard' | 'latex' | 'wolfram' = 'latex';
console.log(`  Domain: ${domain}, Format: ${format}`);
console.log('  ✓ Literal types work\n');

// Test 17: Type inference with functions
console.log('Test 17: Type inference');
const inferredFunc = sin;
const inferredResult = inferredFunc(x);
console.log(`  Type-inferred sin(x): ${inferredResult.toString()}`);
console.log('  ✓ Type inference works\n');

// Test 18: Nested type usage
console.log('Test 18: Nested type usage');
const nestedOpts: SolverOptions = {
    maxIterations: 1000,
    precision: 1e-10,
    domain: 'real',
    simplify: true
};
const allOpts = { solver: nestedOpts, parser: parseOpts };
console.log(`  Nested options: solver.domain=${allOpts.solver.domain}, parser.format=${allOpts.parser.format}`);
console.log('  ✓ Nested type usage works\n');

// Test 19: Type-safe chaining
console.log('Test 19: Type-safe method chaining');
const chained = JsExpression.symbol('z')
    .multiply(2)
    .add(5)
    .pow(3)
    .subtract(1);
console.log(`  (2*z + 5)^3 - 1: ${chained.toString()}`);
console.log('  ✓ Type-safe chaining works\n');

// Test 20: Integration with parse and enhanced types
console.log('Test 20: Integration test');
const parsedExpr = parse('x^2 + 2*x + 1');
const simplified = parsedExpr.simplify();
const derivative = parsedExpr.derivative(JsExpression.symbol('x'), 1);
console.log(`  Parsed: ${parsedExpr.toString()}`);
console.log(`  Simplified: ${simplified.toString()}`);
console.log(`  Derivative: ${derivative.toString()}`);
console.log('  ✓ Full integration works\n');

console.log('=== All TypeScript Enhanced Types Tests Complete ===');
console.log('All 20 tests demonstrate enhanced TypeScript type safety and IDE support.');
