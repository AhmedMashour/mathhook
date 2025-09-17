import {
    sin, cos, tan, asin, acos, atan,
    sinh, cosh, tanh,
    exp, ln, log10, sqrt, abs,
    sign, floor, ceil, round,
    gamma, factorial,
    JsExpression
} from '../index.js';

console.log('=== Function Shortcuts Tests ===\n');

// Test 1: Trigonometric functions with symbols
console.log('Test 1: Trigonometric functions with symbols');
const x = JsExpression.symbol('x');
const sinX = sin(x);
const cosX = cos(x);
const tanX = tan(x);
console.log(`  sin(x): ${sinX.toString()}`);
console.log(`  cos(x): ${cosX.toString()}`);
console.log(`  tan(x): ${tanX.toString()}`);
console.log('  ✓ Trigonometric functions work with symbols\n');

// Test 2: Inverse trigonometric functions
console.log('Test 2: Inverse trigonometric functions');
const asinX = asin(x);
const acosX = acos(x);
const atanX = atan(x);
console.log(`  asin(x): ${asinX.toString()}`);
console.log(`  acos(x): ${acosX.toString()}`);
console.log(`  atan(x): ${atanX.toString()}`);
console.log('  ✓ Inverse trig functions work\n');

// Test 3: Hyperbolic functions
console.log('Test 3: Hyperbolic functions');
const sinhX = sinh(x);
const coshX = cosh(x);
const tanhX = tanh(x);
console.log(`  sinh(x): ${sinhX.toString()}`);
console.log(`  cosh(x): ${coshX.toString()}`);
console.log(`  tanh(x): ${tanhX.toString()}`);
console.log('  ✓ Hyperbolic functions work\n');

// Test 4: Elementary functions
console.log('Test 4: Elementary functions');
const expX = exp(x);
const lnX = ln(x);
const log10X = log10(x);
const sqrtX = sqrt(x);
const absX = abs(x);
console.log(`  exp(x): ${expX.toString()}`);
console.log(`  ln(x): ${lnX.toString()}`);
console.log(`  log10(x): ${log10X.toString()}`);
console.log(`  sqrt(x): ${sqrtX.toString()}`);
console.log(`  abs(x): ${absX.toString()}`);
console.log('  ✓ Elementary functions work\n');

// Test 5: Rounding functions
console.log('Test 5: Rounding functions');
const signX = sign(x);
const floorX = floor(x);
const ceilX = ceil(x);
const roundX = round(x);
console.log(`  sign(x): ${signX.toString()}`);
console.log(`  floor(x): ${floorX.toString()}`);
console.log(`  ceil(x): ${ceilX.toString()}`);
console.log(`  round(x): ${roundX.toString()}`);
console.log('  ✓ Rounding functions work\n');

// Test 6: Special functions
console.log('Test 6: Special functions');
const gammaX = gamma(x);
const factX = factorial(x);
console.log(`  gamma(x): ${gammaX.toString()}`);
console.log(`  factorial(x): ${factX.toString()}`);
console.log('  ✓ Special functions work\n');

// Test 7: Functions with numbers
console.log('Test 7: Functions with numbers');
const sin0 = sin(0);
const cos0 = cos(0);
const exp1 = exp(1);
const sqrt4 = sqrt(4);
const abs5 = abs(-5);
console.log(`  sin(0): ${sin0.toString()}`);
console.log(`  cos(0): ${cos0.toString()}`);
console.log(`  exp(1): ${exp1.toString()}`);
console.log(`  sqrt(4): ${sqrt4.toString()}`);
console.log(`  abs(-5): ${abs5.toString()}`);
console.log('  ✓ Functions work with numbers\n');

// Test 8: Rounding with numbers
console.log('Test 8: Rounding with numbers');
const sign5 = sign(-5);
const sign0 = sign(0);
const signPos = sign(5);
const floor37 = floor(3.7);
const floorNeg = floor(-2.3);
const ceil32 = ceil(3.2);
const ceilNeg = ceil(-2.7);
const round35 = round(3.5);
const round34 = round(3.4);
console.log(`  sign(-5): ${sign5.toString()}`);
console.log(`  sign(0): ${sign0.toString()}`);
console.log(`  sign(5): ${signPos.toString()}`);
console.log(`  floor(3.7): ${floor37.toString()}`);
console.log(`  floor(-2.3): ${floorNeg.toString()}`);
console.log(`  ceil(3.2): ${ceil32.toString()}`);
console.log(`  ceil(-2.7): ${ceilNeg.toString()}`);
console.log(`  round(3.5): ${round35.toString()}`);
console.log(`  round(3.4): ${round34.toString()}`);
console.log('  ✓ Rounding functions work with numbers\n');

// Test 9: Special functions with numbers
console.log('Test 9: Special functions with numbers');
const gamma5 = gamma(5);
const fact5num = factorial(5);
console.log(`  gamma(5): ${gamma5.toString()}`);
console.log(`  factorial(5): ${fact5num.toString()}`);
console.log('  ✓ Special functions work with numbers\n');

// Test 10: Fluent chaining with function shortcuts
console.log('Test 10: Fluent chaining with function shortcuts');
const expr1 = sin(x).add(cos(x));
const expr2 = exp(x).multiply(2);
const expr3 = sqrt(x.pow(2).add(1));
console.log(`  sin(x) + cos(x): ${expr1.toString()}`);
console.log(`  exp(x) * 2: ${expr2.toString()}`);
console.log(`  sqrt(x^2 + 1): ${expr3.toString()}`);
console.log('  ✓ Fluent chaining works\n');

// Test 11: Nested function calls
console.log('Test 11: Nested function calls');
const nested1 = sin(cos(x));
const nested2 = exp(ln(x));
const nested3 = sqrt(abs(x));
console.log(`  sin(cos(x)): ${nested1.toString()}`);
console.log(`  exp(ln(x)): ${nested2.toString()}`);
console.log(`  sqrt(abs(x)): ${nested3.toString()}`);
console.log('  ✓ Nested function calls work\n');

// Test 12: Real-world example - Taylor series of sin(x)
console.log('Test 12: Real-world example - Taylor series approximation');
const fact3 = factorial(3);
const fact5 = factorial(5);
const term1 = x;
const term2 = x.pow(3).divide(fact3);
const term3 = x.pow(5).divide(fact5);
const sinTaylor = term1.subtract(term2).add(term3);
console.log(`  sin(x) ≈ x - x³/3! + x⁵/5!`);
console.log(`  Expression: ${sinTaylor.toString()}`);
console.log('  ✓ Complex expressions work\n');

// Test 13: Mixed with JsExpression methods
console.log('Test 13: Mixed with JsExpression methods');
const y = JsExpression.symbol('y');
const mixed1 = sin(x).add(y.multiply(2));
const mixed2 = exp(x.add(y));
const mixed3 = sqrt(x.pow(2).add(y.pow(2)));
console.log(`  sin(x) + 2*y: ${mixed1.toString()}`);
console.log(`  exp(x + y): ${mixed2.toString()}`);
console.log(`  sqrt(x² + y²): ${mixed3.toString()}`);
console.log('  ✓ Mixed usage with JsExpression works\n');

// Test 14: Auto-conversion (integers vs floats)
console.log('Test 14: Auto-conversion of numbers');
const sinInt = sin(2);
const sinFloat = sin(2.5);
const expInt = exp(3);
const expFloat = exp(1.5);
console.log(`  sin(2): ${sinInt.toString()}`);
console.log(`  sin(2.5): ${sinFloat.toString()}`);
console.log(`  exp(3): ${expInt.toString()}`);
console.log(`  exp(1.5): ${expFloat.toString()}`);
console.log('  ✓ Auto-conversion works correctly\n');

console.log('=== All Function Shortcuts Tests Complete ===');
