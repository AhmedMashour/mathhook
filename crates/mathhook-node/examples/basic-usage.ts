#!/usr/bin/env tsx

/**
 * Basic MathHook Node.js/TypeScript Usage Examples
 *
 * This example demonstrates the fundamental operations available
 * in the MathHook Node.js bindings using TypeScript.
 */

import {
  Expression,
  symbols,
  parse,
  SmartEquationSolver,
} from "../index.js";

console.log("MathHook TypeScript Basic Usage Examples\n");

// ===== Expression Creation =====
console.log("Creating Mathematical Expressions:");

// Create symbols and constants
const [x, y] = symbols("x y");
const two = Expression.integer(2);
const three = Expression.integer(3);
const five = Expression.integer(5);

console.log(`Variable x: ${x.format()}`);
console.log(`Variable y: ${y.format()}`);
console.log(`Constant 2: ${two.format()}`);
console.log(`Constant 3: ${three.format()}\n`);

// ===== Basic Arithmetic Operations =====
console.log("Basic Arithmetic Operations:");

// Addition
const sum = Expression.add([x, two]);
console.log(`x + 2 = ${sum.format()}`);

// Multiplication
const product = Expression.mul([x, three]);
console.log(`x * 3 = ${product.format()}`);

// Exponentiation
const power = Expression.pow(x, two);
console.log(`x^2 = ${power.format()}`);

// Chained operations
const complex = Expression.add([Expression.mul([two, x]), three]);
console.log(`2x + 3 = ${complex.format()}\n`);

// ===== Expression Simplification =====
console.log("Expression Simplification:");

// Simple arithmetic simplification
const arithmetic = Expression.add([two, three]);
console.log(`Before: 2 + 3 = ${arithmetic.format()}`);
console.log(`After:  ${arithmetic.simplify().format()}`);

// Algebraic simplification
const algebraic = Expression.add([x, x, Expression.mul([two, three])]);
console.log(`Before: x + x + 2*3 = ${algebraic.format()}`);
console.log(`After:  ${algebraic.simplify().format()}\n`);

// ===== Equation Creation and Solving =====
console.log("Equation Solving:");

try {
  const solver = new SmartEquationSolver();

  // Simple equation: x = 5
  const equation1 = Expression.equation(x, five);
  console.log(`Equation 1: ${equation1.format()}`);
  const [solution1, explanation1] = solver.solveWithEquation(equation1, x.asSymbol()!);
  console.log(`Solution 1 valid: ${solution1.isValidSolution()}, count: ${solution1.solutionCount()}`);

  // Linear equation: 2x + 3 = 7
  const seven = Expression.integer(7);
  const leftSide = Expression.add([Expression.mul([two, x]), three]);
  const equation2 = Expression.equation(leftSide, seven);
  console.log(`Equation 2: ${equation2.format()}`);
  const [solution2, explanation2] = solver.solveWithEquation(equation2, x.asSymbol()!);
  console.log(`Solution 2 valid: ${solution2.isValidSolution()}, count: ${solution2.solutionCount()}\n`);
} catch (error: unknown) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`Solver error: ${message}\n`);
}

// ===== Integrated Parsing =====
console.log("Integrated Mathematical Expression Parsing:");

try {
  // Direct parsing with automatic language detection
  const parsed1 = parse("2*x + sin(y)");
  console.log(`Auto-detect "2*x + sin(y)": ${parsed1.format()}`);
  console.log(`Simplified: ${parsed1.simplify().format()}`);

  // LaTeX automatic detection
  const parsed2 = parse("\\frac{x}{2} + y^2");
  console.log(`LaTeX "\\frac{x}{2} + y^2": ${parsed2.format()}`);

  // Simple expression
  const parsed3 = parse("sin(x) + cos(y)");
  console.log(`"sin(x) + cos(y)": ${parsed3.format()}`);

  // Format conversion
  const expr = parse("x^2");
  console.log(`Expression: ${expr.format()}\n`);
} catch (error: unknown) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`Parser error: ${message}\n`);
}

// ===== Polynomial Operations =====
console.log("Polynomial Operations:");

// Create a quadratic polynomial: x^2 + 2x + 1
const quadratic = Expression.add([
  Expression.pow(x, two),
  Expression.mul([two, x]),
  Expression.integer(1),
]);

console.log(`Quadratic: ${quadratic.format()}`);
console.log(`Simplified: ${quadratic.simplify().format()}`);

// Create a more complex expression: (x + y)^2
const binomial = Expression.pow(Expression.add([x, y]), two);
console.log(`Binomial: ${binomial.format()}`);
console.log(`Simplified: ${binomial.simplify().format()}\n`);

console.log("All basic examples completed successfully!");
console.log("Try running the advanced examples next: tsx advanced-usage.ts");
