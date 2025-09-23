#!/usr/bin/env tsx

/**
 * Basic MathHook Node.js/TypeScript Usage Examples
 *
 * This example demonstrates the fundamental operations available
 * in the MathHook Node.js bindings using TypeScript.
 */

import { JsExpression, JsMathSolver, JsMathParser } from "../mathhook-node.node";

console.log("🧮 MathHook TypeScript Basic Usage Examples\n");

// ===== Expression Creation =====
console.log("📝 Creating Mathematical Expressions:");

// Create symbols and constants
const x: JsExpression = JsExpression.symbol("x");
const y: JsExpression = JsExpression.symbol("y");
const two: JsExpression = JsExpression.integer(2);
const three: JsExpression = JsExpression.integer(3);
const five: JsExpression = JsExpression.integer(5);

console.log(`Variable x: ${x.toString()}`);
console.log(`Variable y: ${y.toString()}`);
console.log(`Constant 2: ${two.toString()}`);
console.log(`Constant 3: ${three.toString()}\n`);

// ===== Basic Arithmetic Operations =====
console.log("🔢 Basic Arithmetic Operations:");

// Addition
const sum: JsExpression = x.add(two);
console.log(`x + 2 = ${sum.toString()}`);

// Multiplication
const product: JsExpression = x.multiply(three);
console.log(`x × 3 = ${product.toString()}`);

// Exponentiation
const power: JsExpression = x.pow(two);
console.log(`x² = ${power.toString()}`);

// Chained operations
const complex: JsExpression = x.multiply(two).add(three);
console.log(`2x + 3 = ${complex.toString()}\n`);

// ===== Expression Simplification =====
console.log("⚡ Expression Simplification:");

// Simple arithmetic simplification
const arithmetic: JsExpression = two.add(three);
console.log(`Before: 2 + 3 = ${arithmetic.toString()}`);
console.log(`After:  ${arithmetic.simplify().toString()}`);

// Algebraic simplification
const algebraic: JsExpression = x.add(x).add(two.multiply(three));
console.log(`Before: x + x + 2×3 = ${algebraic.toString()}`);
console.log(`After:  ${algebraic.simplify().toString()}\n`);

// ===== Equation Creation and Solving =====
console.log("🎯 Equation Solving:");

try {
  const solver = new JsMathSolver();

  // Simple equation: x = 5
  const equation1: JsExpression = JsExpression.equation(x, five);
  console.log(`Equation 1: ${equation1.toString()}`);
  const solution1: string = solver.solve(equation1, "x");
  console.log(`Solution 1: ${solution1}`);

  // Linear equation: 2x + 3 = 7
  const seven: JsExpression = JsExpression.integer(7);
  const leftSide: JsExpression = two.multiply(x).add(three);
  const equation2: JsExpression = JsExpression.equation(leftSide, seven);
  console.log(`Equation 2: ${equation2.toString()}`);
  const solution2: string = solver.solve(equation2, "x");
  console.log(`Solution 2: ${solution2}\n`);
} catch (error: any) {
  console.error(`❌ Solver error: ${error.message}\n`);
}

// ===== 🆕 INTEGRATED PARSING (No separate parser needed!) =====
console.log("📖 Integrated Mathematical Expression Parsing:");

try {
  // 🆕 Direct parsing with automatic language detection
  const parsed1: JsExpression = JsExpression.parse("2*x + sin(y)");
  console.log(`Auto-detect "2*x + sin(y)": ${parsed1.toString()}`);
  console.log(`Simplified: ${parsed1.simplify().toString()}`);

  // 🆕 LaTeX automatic detection
  const parsed2: JsExpression = JsExpression.parse("\\frac{x}{2} + y^2");
  console.log(`LaTeX "\\frac{x}{2} + y^2": ${parsed2.toString()}`);
  console.log(`LaTeX output: ${parsed2.toLatex()}`);

  // 🆕 Wolfram automatic detection
  const parsed3: JsExpression = JsExpression.parse("Sin[x] + Cos[y]");
  console.log(`Wolfram "Sin[x] + Cos[y]": ${parsed3.toString()}`);
  console.log(`Wolfram output: ${parsed3.toWolfram()}`);

  // 🆕 Explicit language parsing
  const latexExpr = JsExpression.parseWithLanguage("\\sin(x)", "latex");
  const wolframExpr = JsExpression.parseWithLanguage("Sin[x]", "wolfram");
  const simpleExpr = JsExpression.parseWithLanguage("sin(x)", "simple");
  
  console.log(`Explicit LaTeX: ${latexExpr.toString()}`);
  console.log(`Explicit Wolfram: ${wolframExpr.toString()}`);
  console.log(`Explicit Simple: ${simpleExpr.toString()}`);

  // 🆕 Format conversion
  const expr = JsExpression.parse("x^2");
  console.log(`Expression: ${expr.toString()}`);
  console.log(`LaTeX: ${expr.toLatex()}`);
  console.log(`Simple: ${expr.toSimple()}`);
  console.log(`Wolfram: ${expr.toWolfram()}\n`);
} catch (error: any) {
  console.error(`❌ Parser error: ${error.message}\n`);
}

// ===== Polynomial Operations =====
console.log("📐 Polynomial Operations:");

// Create a quadratic polynomial: x² + 2x + 1
const quadratic: JsExpression = x
  .pow(two)
  .add(two.multiply(x))
  .add(JsExpression.integer(1));

console.log(`Quadratic: ${quadratic.toString()}`);
console.log(`Simplified: ${quadratic.simplify().toString()}`);

// Create a more complex expression: (x + y)²
const binomial: JsExpression = x.add(y).pow(two);
console.log(`Binomial: ${binomial.toString()}`);
console.log(`Simplified: ${binomial.simplify().toString()}\n`);

console.log("✅ All basic examples completed successfully!");
console.log("💡 Try running the advanced examples next: tsx advanced-usage.ts");
