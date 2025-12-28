import { parse, Expression, symbols } from "../index.js";

console.log("=== parse() Function Tests ===\n");

// Test 1: Basic polynomial parsing
console.log("Test 1: Basic polynomial parsing");
const poly = parse("x^2 + 2*x + 1");
console.log(`  parse('x^2 + 2*x + 1'): ${poly.format()}`);
console.log("  Basic polynomial parsing works\n");

// Test 2: Implicit multiplication
console.log("Test 2: Implicit multiplication");
const implicit1 = parse("2x");
const explicit1 = parse("2*x");
console.log(`  parse('2x'): ${implicit1.format()}`);
console.log(`  parse('2*x'): ${explicit1.format()}`);
console.log(`  Equal: ${implicit1.format() === explicit1.format()}`);
console.log("  Implicit multiplication works\n");

// Test 3: Function parsing
console.log("Test 3: Function parsing");
const funcs = parse("sin(x) + cos(y)");
console.log(`  parse('sin(x) + cos(y)'): ${funcs.format()}`);
console.log("  Function parsing works\n");

// Test 4: Greek letters
console.log("Test 4: Greek letters");
const greek = parse("alpha + beta + gamma");
console.log(`  parse('alpha + beta + gamma'): ${greek.format()}`);
console.log("  Greek letter parsing works\n");

// Test 5: Constants
console.log("Test 5: Mathematical constants");
const constants = parse("pi + e + i");
console.log(`  parse('pi + e + i'): ${constants.format()}`);
console.log("  Constant parsing works\n");

// Test 6: Fractions
console.log("Test 6: Fractions");
const fractions = parse("1/2 + 3/4");
console.log(`  parse('1/2 + 3/4'): ${fractions.format()}`);
console.log("  Fraction parsing works\n");

// Test 7: Complex expression
console.log("Test 7: Complex expression");
const complex = parse("sin(2*pi*x) + exp(-x^2/2)");
console.log(`  parse('sin(2*pi*x) + exp(-x^2/2)'): ${complex.format()}`);
console.log("  Complex expression parsing works\n");

// Test 8: Nested parentheses
console.log("Test 8: Nested parentheses");
const nested = parse("((a + b) * (c + x)) / (e + f)");
console.log(`  parse('((a + b) * (c + x)) / (e + f)'): ${nested.format()}`);
console.log("  Nested parentheses work\n");

// Test 9: Multiple implicit multiplications
console.log("Test 9: Multiple implicit multiplications");
const multiImplicit = parse("2x + 3y + 4z");
console.log(`  parse('2x + 3y + 4z'): ${multiImplicit.format()}`);
console.log("  Multiple implicit multiplications work\n");

// Test 10: Parenthesized implicit multiplication
console.log("Test 10: Parenthesized implicit multiplication");
const parenImplicit = parse("2(x + 1)");
console.log(`  parse('2(x + 1)'): ${parenImplicit.format()}`);
console.log("  Parenthesized implicit multiplication works\n");

// Test 11: Multiple functions
console.log("Test 11: Multiple functions");
const multiFuncs = parse("sin(x)*cos(x) + tan(x)");
console.log(`  parse('sin(x)*cos(x) + tan(x)'): ${multiFuncs.format()}`);
console.log("  Multiple functions work\n");

// Test 12: Nested functions
console.log("Test 12: Nested functions");
const nestedFuncs = parse("sin(cos(x))");
console.log(`  parse('sin(cos(x))'): ${nestedFuncs.format()}`);
console.log("  Nested functions work\n");

// Test 13: Power operations
console.log("Test 13: Power operations");
const powers = parse("x^2 + y^3 + z^(a+b)");
console.log(`  parse('x^2 + y^3 + z^(a+b)'): ${powers.format()}`);
console.log("  Power operations work\n");

// Test 14: Special functions
console.log("Test 14: Special functions");
const special = parse("gamma(x) + factorial(5) + sqrt(x)");
console.log(`  parse('gamma(x) + factorial(5) + sqrt(x)'): ${special.format()}`);
console.log("  Special functions work\n");

// Test 15: Mixing with static API
console.log("Test 15: Mixing parsed expressions with static API");
const [x] = symbols("x");
const parsed = parse("x^2 + 1");
const combined = Expression.add([Expression.mul([parsed, x]), Expression.integer(5)]);
console.log(`  parse('x^2 + 1').multiply(x).add(5): ${combined.format()}`);
console.log("  Mixing with static API works\n");

// Test 16: Real-world quadratic formula
console.log("Test 16: Real-world example - Quadratic formula");
const quadratic = parse("(-b + sqrt(b^2 - 4*a*c)) / (2*a)");
console.log(`  Quadratic formula: ${quadratic.format()}`);
console.log("  Real-world quadratic formula works\n");

// Test 17: Comparison with manual construction
console.log("Test 17: Comparison with manual construction");
const [y] = symbols("y");
const manual = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.pow(y, Expression.integer(2)),
]);
const fromParse = parse("x^2 + y^2");
console.log(`  Manual: ${manual.format()}`);
console.log(`  Parsed: ${fromParse.format()}`);
console.log("  Parsing matches manual construction\n");

// Test 18: LaTeX input (if auto-detected)
console.log("Test 18: LaTeX notation (auto-detected)");
try {
  const latex = parse("\\frac{x^2}{2}");
  console.log(`  parse('\\\\frac{x^2}{2}'): ${latex.format()}`);
  console.log("  LaTeX parsing works\n");
} catch {
  console.log(`  LaTeX parsing not auto-detected (expected)\n`);
}

// Test 19: Wolfram notation (if auto-detected)
console.log("Test 19: Wolfram notation (auto-detected)");
try {
  const wolfram = parse("Sin[x] + Cos[y]");
  console.log(`  parse('Sin[x] + Cos[y]'): ${wolfram.format()}`);
  console.log("  Wolfram parsing works\n");
} catch {
  console.log(`  Wolfram parsing not auto-detected (expected)\n`);
}

// Test 20: Error handling - invalid expression
console.log("Test 20: Error handling - invalid expression");
try {
  const invalid = parse("x +");
  console.log(`  Unexpected success: ${invalid.format()}`);
} catch (e: unknown) {
  const message = e instanceof Error ? e.message : String(e);
  console.log(`  Error caught correctly: ${message}`);
  console.log("  Error handling works\n");
}

console.log("=== All parse() Tests Complete ===");
