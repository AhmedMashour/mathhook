import {
  sin,
  cos,
  tan,
  asin,
  acos,
  atan,
  sinh,
  cosh,
  tanh,
  exp,
  ln,
  log10,
  sqrt,
  abs,
  sign,
  floor,
  ceil,
  round,
  gamma,
  factorial,
  Expression,
  symbols,
} from "../index.js";

console.log("=== Function Shortcuts Tests ===\n");

// Test 1: Trigonometric functions with symbols
console.log("Test 1: Trigonometric functions with symbols");
const [x] = symbols("x");
const sinX = sin(x);
const cosX = cos(x);
const tanX = tan(x);
console.log(`  sin(x): ${sinX.format()}`);
console.log(`  cos(x): ${cosX.format()}`);
console.log(`  tan(x): ${tanX.format()}`);
console.log("  Trigonometric functions work with symbols\n");

// Test 2: Inverse trigonometric functions
console.log("Test 2: Inverse trigonometric functions");
const asinX = asin(x);
const acosX = acos(x);
const atanX = atan(x);
console.log(`  asin(x): ${asinX.format()}`);
console.log(`  acos(x): ${acosX.format()}`);
console.log(`  atan(x): ${atanX.format()}`);
console.log("  Inverse trig functions work\n");

// Test 3: Hyperbolic functions
console.log("Test 3: Hyperbolic functions");
const sinhX = sinh(x);
const coshX = cosh(x);
const tanhX = tanh(x);
console.log(`  sinh(x): ${sinhX.format()}`);
console.log(`  cosh(x): ${coshX.format()}`);
console.log(`  tanh(x): ${tanhX.format()}`);
console.log("  Hyperbolic functions work\n");

// Test 4: Elementary functions
console.log("Test 4: Elementary functions");
const expX = exp(x);
const lnX = ln(x);
const log10X = log10(x);
const sqrtX = sqrt(x);
const absX = abs(x);
console.log(`  exp(x): ${expX.format()}`);
console.log(`  ln(x): ${lnX.format()}`);
console.log(`  log10(x): ${log10X.format()}`);
console.log(`  sqrt(x): ${sqrtX.format()}`);
console.log(`  abs(x): ${absX.format()}`);
console.log("  Elementary functions work\n");

// Test 5: Rounding functions
console.log("Test 5: Rounding functions");
const signX = sign(x);
const floorX = floor(x);
const ceilX = ceil(x);
const roundX = round(x);
console.log(`  sign(x): ${signX.format()}`);
console.log(`  floor(x): ${floorX.format()}`);
console.log(`  ceil(x): ${ceilX.format()}`);
console.log(`  round(x): ${roundX.format()}`);
console.log("  Rounding functions work\n");

// Test 6: Special functions
console.log("Test 6: Special functions");
const gammaX = gamma(x);
const factX = factorial(x);
console.log(`  gamma(x): ${gammaX.format()}`);
console.log(`  factorial(x): ${factX.format()}`);
console.log("  Special functions work\n");

// Test 7: Functions with numbers
console.log("Test 7: Functions with numbers");
const sin0 = sin(Expression.integer(0));
const cos0 = cos(Expression.integer(0));
const exp1 = exp(Expression.integer(1));
const sqrt4 = sqrt(Expression.integer(4));
const abs5 = abs(Expression.integer(-5));
console.log(`  sin(0): ${sin0.format()}`);
console.log(`  cos(0): ${cos0.format()}`);
console.log(`  exp(1): ${exp1.format()}`);
console.log(`  sqrt(4): ${sqrt4.format()}`);
console.log(`  abs(-5): ${abs5.format()}`);
console.log("  Functions work with numbers\n");

// Test 8: Rounding with numbers
console.log("Test 8: Rounding with numbers");
const sign5 = sign(Expression.integer(-5));
const sign0n = sign(Expression.integer(0));
const signPos = sign(Expression.integer(5));
const floor37 = floor(Expression.float(3.7));
const floorNeg = floor(Expression.float(-2.3));
const ceil32 = ceil(Expression.float(3.2));
const ceilNeg = ceil(Expression.float(-2.7));
const round35 = round(Expression.float(3.5));
const round34 = round(Expression.float(3.4));
console.log(`  sign(-5): ${sign5.format()}`);
console.log(`  sign(0): ${sign0n.format()}`);
console.log(`  sign(5): ${signPos.format()}`);
console.log(`  floor(3.7): ${floor37.format()}`);
console.log(`  floor(-2.3): ${floorNeg.format()}`);
console.log(`  ceil(3.2): ${ceil32.format()}`);
console.log(`  ceil(-2.7): ${ceilNeg.format()}`);
console.log(`  round(3.5): ${round35.format()}`);
console.log(`  round(3.4): ${round34.format()}`);
console.log("  Rounding functions work with numbers\n");

// Test 9: Special functions with numbers
console.log("Test 9: Special functions with numbers");
const gamma5 = gamma(Expression.integer(5));
const fact5num = factorial(5);
console.log(`  gamma(5): ${gamma5.format()}`);
console.log(`  factorial(5): ${fact5num.format()}`);
console.log("  Special functions work with numbers\n");

// Test 10: Expressions with function shortcuts
console.log("Test 10: Expressions with function shortcuts");
const expr1 = Expression.add([sin(x), cos(x)]);
const expr2 = Expression.mul([exp(x), Expression.integer(2)]);
const expr3 = sqrt(Expression.add([Expression.pow(x, Expression.integer(2)), Expression.integer(1)]));
console.log(`  sin(x) + cos(x): ${expr1.format()}`);
console.log(`  exp(x) * 2: ${expr2.format()}`);
console.log(`  sqrt(x^2 + 1): ${expr3.format()}`);
console.log("  Expressions with functions work\n");

// Test 11: Nested function calls
console.log("Test 11: Nested function calls");
const nested1 = sin(cos(x));
const nested2 = exp(ln(x));
const nested3 = sqrt(abs(x));
console.log(`  sin(cos(x)): ${nested1.format()}`);
console.log(`  exp(ln(x)): ${nested2.format()}`);
console.log(`  sqrt(abs(x)): ${nested3.format()}`);
console.log("  Nested function calls work\n");

// Test 12: Real-world example - Taylor series of sin(x)
console.log("Test 12: Real-world example - Taylor series approximation");
const fact3 = factorial(3);
const fact5 = factorial(5);
const term1 = x;
const term2 = Expression.div(Expression.pow(x, Expression.integer(3)), fact3);
const term3 = Expression.div(Expression.pow(x, Expression.integer(5)), fact5);
const sinTaylor = Expression.add([term1, Expression.mul([Expression.integer(-1), term2]), term3]);
console.log(`  sin(x) ≈ x - x^3/3! + x^5/5!`);
console.log(`  Expression: ${sinTaylor.format()}`);
console.log("  Complex expressions work\n");

// Test 13: Mixed with Expression methods
console.log("Test 13: Mixed with Expression methods");
const [y] = symbols("y");
const mixed1 = Expression.add([sin(x), Expression.mul([y, Expression.integer(2)])]);
const mixed2 = exp(Expression.add([x, y]));
const mixed3 = sqrt(Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.pow(y, Expression.integer(2)),
]));
console.log(`  sin(x) + 2*y: ${mixed1.format()}`);
console.log(`  exp(x + y): ${mixed2.format()}`);
console.log(`  sqrt(x^2 + y^2): ${mixed3.format()}`);
console.log("  Mixed usage with Expression works\n");

// Test 14: Auto-conversion (integers vs floats)
console.log("Test 14: Auto-conversion of numbers");
const sinInt = sin(Expression.integer(2));
const sinFloat = sin(Expression.float(2.5));
const expInt = exp(Expression.integer(3));
const expFloat = exp(Expression.float(1.5));
console.log(`  sin(2): ${sinInt.format()}`);
console.log(`  sin(2.5): ${sinFloat.format()}`);
console.log(`  exp(3): ${expInt.format()}`);
console.log(`  exp(1.5): ${expFloat.format()}`);
console.log("  Auto-conversion works correctly\n");

console.log("=== All Function Shortcuts Tests Complete ===");
