#!/usr/bin/env tsx

/**
 * Advanced MathHook Node.js/TypeScript Usage Examples
 *
 * This example demonstrates advanced mathematical operations,
 * complex expressions, and real-world use cases.
 */

import { Expression, symbols, parse, SmartEquationSolver, sin, cos, exp } from "../index.js";

console.log("MathHook TypeScript Advanced Usage Examples\n");

// ===== Complex Mathematical Expressions =====
console.log("Complex Mathematical Expressions:");

// Variables for advanced examples
const [x, y, z, a, b, c] = symbols("x y z a b c");

// Multi-variable polynomial: ax^2 + bxy + cy^2
const multiPoly = Expression.add([
  Expression.mul([a, Expression.pow(x, Expression.integer(2))]),
  Expression.mul([b, x, y]),
  Expression.mul([c, Expression.pow(y, Expression.integer(2))]),
]);

console.log(`Multi-variable polynomial: ${multiPoly.format()}`);
console.log(`Simplified: ${multiPoly.simplify().format()}\n`);

// ===== Nested Operations =====
console.log("Nested Mathematical Operations:");

// Nested expression: (x + y)^3
const cubed = Expression.pow(Expression.add([x, y]), Expression.integer(3));
console.log(`(x + y)^3: ${cubed.format()}`);
console.log(`Simplified: ${cubed.simplify().format()}`);

// Complex fraction-like expression: (x^2 + 2x + 1) / (x + 1)
const numerator = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([Expression.integer(2), x]),
  Expression.integer(1),
]);
const denominator = Expression.add([x, Expression.integer(1)]);
const fraction = Expression.div(numerator, denominator);

console.log(`Complex fraction: ${fraction.format()}`);
console.log(`Simplified: ${fraction.simplify().format()}\n`);

// ===== System of Equations (Conceptual) =====
console.log("System of Equations Example:");

try {
  // First equation: 2x + 3y = 12
  const eq1Left = Expression.add([
    Expression.mul([Expression.integer(2), x]),
    Expression.mul([Expression.integer(3), y]),
  ]);
  const eq1 = Expression.equation(eq1Left, Expression.integer(12));

  // Second equation: x - y = 1
  const eq2Left = Expression.add([x, Expression.mul([Expression.integer(-1), y])]);
  const eq2 = Expression.equation(eq2Left, Expression.integer(1));

  console.log(`Equation 1: ${eq1.format()}`);
  console.log(`Equation 2: ${eq2.format()}`);

  // Note: Current solver handles single equations
  // System solving would require additional implementation
  console.log("Note: System solving requires additional implementation\n");
} catch (error: unknown) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`System solver error: ${message}\n`);
}

// ===== Advanced Parsing Examples =====
console.log("Advanced Parsing Examples:");

try {
  // Complex LaTeX expressions - automatic detection
  const latexExpressions = [
    "\\sqrt{x^2 + y^2}",
    "\\frac{a^2 + b^2}{c^2}",
    "x^{2n+1} + y^{n-1}",
  ];

  console.log("LaTeX Expressions (Auto-detected):");
  latexExpressions.forEach((expr, index) => {
    try {
      const parsed = parse(expr);
      console.log(`  ${index + 1}. "${expr}" => ${parsed.format()}`);
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : String(e);
      console.log(`  ${index + 1}. "${expr}" => Parse Error: ${message}`);
    }
  });

  // Simple mathematical expressions
  const simpleExpressions = [
    "x^2 + 2*x + 1",
    "sin(x) + cos(y)",
    "exp(-x^2/2)",
  ];

  console.log("\nSimple Expressions:");
  simpleExpressions.forEach((expr, index) => {
    try {
      const parsed = parse(expr);
      console.log(`  ${index + 1}. "${expr}" => ${parsed.format()}`);
    } catch (e: unknown) {
      const message = e instanceof Error ? e.message : String(e);
      console.log(`  ${index + 1}. "${expr}" => Parse Error: ${message}`);
    }
  });
} catch (error: unknown) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`Advanced parser error: ${message}`);
}

console.log("\n");

// ===== Performance Testing =====
console.log("Performance Testing:");

const performanceTest = () => {
  const startTime = process.hrtime.bigint();

  // Create and simplify 1000 expressions
  for (let i = 0; i < 1000; i++) {
    const expr = Expression.add([
      Expression.mul([x, Expression.integer(i)]),
      Expression.pow(y, Expression.integer(2)),
      Expression.integer(i * 2),
    ]);
    expr.simplify();
  }

  const endTime = process.hrtime.bigint();
  const duration = Number(endTime - startTime) / 1_000_000; // Convert to milliseconds

  console.log(
    `Created and simplified 1000 expressions in ${duration.toFixed(2)}ms`
  );
  console.log(`Average: ${(duration / 1000).toFixed(4)}ms per expression`);
};

performanceTest();

// ===== Memory Usage Example =====
console.log("\nMemory Usage Example:");

const memoryTest = () => {
  const expressions: Expression[] = [];

  // Create a large number of expressions
  for (let i = 0; i < 10000; i++) {
    const expr = Expression.add([
      Expression.pow(x, Expression.integer(i % 5)),
      Expression.mul([y, Expression.integer(i)]),
      Expression.pow(z, Expression.integer(2)),
    ]);
    expressions.push(expr);
  }

  console.log(`Created ${expressions.length} expressions in memory`);

  // Simplify all expressions
  const simplified = expressions.map((expr) => expr.simplify());
  console.log(`Simplified ${simplified.length} expressions`);

  // Clear references (JavaScript GC will handle cleanup)
  expressions.length = 0;
  simplified.length = 0;

  console.log("Memory test completed");
};

memoryTest();

// ===== Real-world Use Case: Quadratic Formula =====
console.log("\nReal-world Use Case: Quadratic Formula");

const quadraticFormula = () => {
  try {
    const solver = new SmartEquationSolver();

    // Quadratic equation: ax^2 + bx + c = 0
    // For example: x^2 - 5x + 6 = 0
    const a_val = Expression.integer(1);
    const b_val = Expression.integer(-5);
    const c_val = Expression.integer(6);

    const quadratic = Expression.add([
      Expression.mul([a_val, Expression.pow(x, Expression.integer(2))]),
      Expression.mul([b_val, x]),
      c_val,
    ]);

    const equation = Expression.equation(quadratic, Expression.integer(0));

    console.log(`Quadratic equation: ${equation.format()}`);

    const [solution, explanation] = solver.solveWithEquation(equation, x.asSymbol()!);
    console.log(`Solution valid: ${solution.isValidSolution()}, count: ${solution.solutionCount()}`);

    // The solutions should be x = 2 and x = 3 (since (x-2)(x-3) = x^2 - 5x + 6)
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`Quadratic formula error: ${message}`);
  }
};

quadraticFormula();

// ===== Trigonometric Expression Building =====
console.log("\nTrigonometric Expression Building:");

try {
  // sin^2(x) + cos^2(x)
  const trig = Expression.add([
    Expression.pow(sin(x), Expression.integer(2)),
    Expression.pow(cos(x), Expression.integer(2)),
  ]);
  console.log(`sin^2(x) + cos^2(x) = ${trig.format()}`);
  console.log(`Simplified: ${trig.simplify().format()}`);

  // exp(i*x) relationship
  const complex = exp(Expression.mul([Expression.i(), x]));
  console.log(`exp(i*x) = ${complex.format()}`);
} catch (error: unknown) {
  const message = error instanceof Error ? error.message : String(error);
  console.error(`Trig error: ${message}`);
}

console.log("\nAll advanced examples completed successfully!");
console.log(
  "You now know how to use MathHook with TypeScript for complex mathematical operations!"
);
