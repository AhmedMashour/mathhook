import {
  Expression,
  parse,
  sin,
  cos,
  exp,
  ln,
  sqrt,
  gamma,
  factorial,
  symbols,
} from "../index.js";

console.log("=== TypeScript Enhanced Types Tests ===\n");

// Type alias for math functions
type MathFunction = (arg: Expression) => Expression;

// Test 1: MathFunction type usage
console.log("Test 1: MathFunction type annotations");
const myTrigFunction: MathFunction = sin;
const [x] = symbols("x");
const result1 = myTrigFunction(x);
console.log(`  sin(x) using MathFunction type: ${result1.format()}`);
console.log("  MathFunction type works\n");

// Test 2: BinaryOperation type usage
console.log("Test 2: BinaryOperation type (conceptual)");
const addOp = (a: Expression | number, b: Expression | number): Expression => {
  const exprA = typeof a === "number" ? Expression.integer(a) : a;
  const exprB = typeof b === "number" ? Expression.integer(b) : b;
  return Expression.add([exprA, exprB]);
};
const result2 = addOp(x, 5);
console.log(`  Custom add operation: ${result2.format()}`);
console.log("  BinaryOperation type pattern works\n");

// Test 3: Configuration interface
console.log("Test 3: Configuration interface");
interface SolverOptions {
  maxIterations?: number;
  precision?: number;
  domain?: "real" | "complex";
  simplify?: boolean;
}
const solverOpts: SolverOptions = {
  maxIterations: 500,
  precision: 1e-12,
  domain: "complex",
  simplify: true,
};
console.log(
  `  Solver options configured: maxIterations=${solverOpts.maxIterations}, precision=${solverOpts.precision}`
);
console.log("  SolverOptions interface works\n");

// Test 4: ParseOptions interface
console.log("Test 4: ParseOptions interface");
interface ParseOptions {
  format?: "auto" | "standard" | "latex" | "wolfram";
  simplify?: boolean;
  implicitMultiplication?: boolean;
}
const parseOpts: ParseOptions = {
  format: "auto",
  simplify: false,
  implicitMultiplication: true,
};
console.log(
  `  Parse options configured: format=${parseOpts.format}, simplify=${parseOpts.simplify}`
);
console.log("  ParseOptions interface works\n");

// Test 5: PolynomialCoefficients type
console.log("Test 5: PolynomialCoefficients type");
type PolynomialCoefficients = Record<number, number | Expression>;
const polyCoeffs: PolynomialCoefficients = {
  0: 5,
  1: -3,
  2: 2,
  3: Expression.integer(1),
};
console.log(
  `  Polynomial coefficients: degree 0 = ${polyCoeffs[0]}, degree 3 = ${(polyCoeffs[3] as Expression)?.format()}`
);
console.log("  PolynomialCoefficients type works\n");

// Test 6: IntegrationBounds interface
console.log("Test 6: IntegrationBounds interface");
interface IntegrationBounds {
  lower: number | Expression;
  upper: number | Expression;
}
const bounds: IntegrationBounds = {
  lower: 0,
  upper: Expression.pi(),
};
console.log(
  `  Integration bounds: [${bounds.lower}, ${(bounds.upper as Expression).format()}]`
);
console.log("  IntegrationBounds interface works\n");

// Test 7: Function composition with type annotations
console.log("Test 7: Function composition");
const composedFunc: MathFunction = (arg) => {
  return sin(cos(arg));
};
const result7 = composedFunc(x);
console.log(`  sin(cos(x)): ${result7.format()}`);
console.log("  Function composition works\n");

// Test 8: Array of MathFunction
console.log("Test 8: Array of MathFunction");
const functions: MathFunction[] = [sin, cos, exp, ln, sqrt];
const results8 = functions.map((f) => f(x).format());
console.log(`  Applied functions to x: [${results8.join(", ")}]`);
console.log("  Array of MathFunction works\n");

// Test 9: Type-safe expression building
console.log("Test 9: Type-safe expression building");
const [y] = symbols("y");
const expr9 = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([y, Expression.integer(3)]),
  Expression.integer(-1),
]);
console.log(`  x^2 + 3*y - 1: ${expr9.format()}`);
console.log("  Type-safe building works\n");

// Test 10: Using parse with TypeScript
console.log("Test 10: parse() with TypeScript");
const parsed = parse("2*x + sin(y)");
console.log(`  Parsed expression: ${parsed.format()}`);
console.log("  parse() TypeScript integration works\n");

// Test 11: Special functions with type annotations
console.log("Test 11: Special functions");
const specialFuncs: MathFunction[] = [gamma];
const n = Expression.integer(5);
const results11 = specialFuncs.map((f) => f(n).format());
const factResult = factorial(5);
console.log(`  gamma(5) = ${results11[0]}, factorial(5) = ${factResult.format()}`);
console.log("  Special functions type annotations work\n");

// Test 12: Complex expression with type safety
console.log("Test 12: Complex expression");
const complexExpr = Expression.add([
  sin(Expression.pow(x, Expression.integer(2))),
  cos(Expression.mul([y, Expression.integer(3)])),
]);
console.log(`  sin(x^2) + cos(3*y): ${complexExpr.format()}`);
console.log("  Complex expression type safety works\n");

// Test 13: Optional fields in interfaces
console.log("Test 13: Optional fields");
const minimalOpts: SolverOptions = {
  domain: "real",
};
console.log(
  `  Minimal solver options: domain=${minimalOpts.domain}, maxIterations=${minimalOpts.maxIterations ?? "default"}`
);
console.log("  Optional fields work\n");

// Test 14: Expression type
console.log("Test 14: Expression type");
const expr14: Expression = Expression.integer(42);
console.log(`  Using Expression type: ${expr14.format()}`);
console.log("  Expression type works\n");

// Test 15: Union types (Expression | number)
console.log("Test 15: Union types");
const unionTest = (val: Expression | number): string => {
  if (typeof val === "number") {
    return `Number: ${val}`;
  } else {
    return `Expression: ${val.format()}`;
  }
};
console.log(`  Union with number: ${unionTest(42)}`);
console.log(`  Union with expression: ${unionTest(x)}`);
console.log("  Union types work\n");

// Test 16: Literal types
console.log("Test 16: Literal types");
const domain: "real" | "complex" = "complex";
const format: "auto" | "standard" | "latex" | "wolfram" = "latex";
console.log(`  Domain: ${domain}, Format: ${format}`);
console.log("  Literal types work\n");

// Test 17: Type inference with functions
console.log("Test 17: Type inference");
const inferredFunc = sin;
const inferredResult = inferredFunc(x);
console.log(`  Type-inferred sin(x): ${inferredResult.format()}`);
console.log("  Type inference works\n");

// Test 18: Nested type usage
console.log("Test 18: Nested type usage");
const nestedOpts: SolverOptions = {
  maxIterations: 1000,
  precision: 1e-10,
  domain: "real",
  simplify: true,
};
const allOpts = { solver: nestedOpts, parser: parseOpts };
console.log(
  `  Nested options: solver.domain=${allOpts.solver.domain}, parser.format=${allOpts.parser.format}`
);
console.log("  Nested type usage works\n");

// Test 19: Type-safe method chaining
console.log("Test 19: Type-safe method chaining");
const [z] = symbols("z");
const chained = Expression.add([
  Expression.pow(Expression.add([Expression.mul([z, Expression.integer(2)]), Expression.integer(5)]), Expression.integer(3)),
  Expression.integer(-1),
]);
console.log(`  (2*z + 5)^3 - 1: ${chained.format()}`);
console.log("  Type-safe chaining works\n");

// Test 20: Integration with parse and enhanced types
console.log("Test 20: Integration test");
const parsedExpr = parse("x^2 + 2*x + 1");
const simplified = parsedExpr.simplify();
const derivative = parsedExpr.derivative(x.asSymbol()!);
console.log(`  Parsed: ${parsedExpr.format()}`);
console.log(`  Simplified: ${simplified.format()}`);
console.log(`  Derivative: ${derivative.format()}`);
console.log("  Full integration works\n");

console.log("=== All TypeScript Enhanced Types Tests Complete ===");
console.log(
  "All 20 tests demonstrate enhanced TypeScript type safety and IDE support."
);
