import { JsExpression } from '../index.js';

console.log('=== Fluent API Tests ===\n');

const x = JsExpression.symbol('x');
const y = JsExpression.symbol('y');

console.log('Test 1: Add with number auto-conversion');
const add1 = x.add(JsExpression.integer(2));
const add2 = x.add(2);
console.log(`  x.add(JsExpression.integer(2)): ${add1.toString()}`);
console.log(`  x.add(2): ${add2.toString()}`);
console.log(`  Results match: ${add1.toString() === add2.toString()}\n`);

console.log('Test 2: Multiply with number auto-conversion');
const mul1 = x.multiply(JsExpression.integer(3));
const mul2 = x.multiply(3);
console.log(`  x.multiply(JsExpression.integer(3)): ${mul1.toString()}`);
console.log(`  x.multiply(3): ${mul2.toString()}`);
console.log(`  Results match: ${mul1.toString() === mul2.toString()}\n`);

console.log('Test 3: Power with number auto-conversion');
const pow1 = x.pow(JsExpression.integer(2));
const pow2 = x.pow(2);
console.log(`  x.pow(JsExpression.integer(2)): ${pow1.toString()}`);
console.log(`  x.pow(2): ${pow2.toString()}`);
console.log(`  Results match: ${pow1.toString() === pow2.toString()}\n`);

console.log('Test 4: Subtract with number auto-conversion');
const sub1 = x.subtract(JsExpression.integer(5));
const sub2 = x.subtract(5);
console.log(`  x.subtract(JsExpression.integer(5)): ${sub1.toString()}`);
console.log(`  x.subtract(5): ${sub2.toString()}`);
console.log(`  Results match: ${sub1.toString() === sub2.toString()}\n`);

console.log('Test 5: Divide with number auto-conversion');
const div1 = x.divide(JsExpression.integer(2));
const div2 = x.divide(2);
console.log(`  x.divide(JsExpression.integer(2)): ${div1.toString()}`);
console.log(`  x.divide(2): ${div2.toString()}`);
console.log(`  Results match: ${div1.toString() === div2.toString()}\n`);

console.log('Test 6: Fluent chaining with numbers');
const chain1 = x.add(2).multiply(3).pow(2);
console.log(`  x.add(2).multiply(3).pow(2): ${chain1.toString()}`);
const chain1Simplified = chain1.simplify();
console.log(`  Simplified: ${chain1Simplified.toString()}\n`);

console.log('Test 7: Complex expression with mixed types');
const expr1 = x.pow(2).add(x.multiply(2)).add(1);
console.log(`  x.pow(2).add(x.multiply(2)).add(1): ${expr1.toString()}`);
const expr1Simplified = expr1.simplify();
console.log(`  Simplified: ${expr1Simplified.toString()}\n`);

console.log('Test 8: Float conversion (3.14 should remain float)');
const float1 = x.multiply(3.14);
console.log(`  x.multiply(3.14): ${float1.toString()}\n`);

console.log('Test 9: Integer conversion (2.0 should become integer)');
const int1 = x.multiply(2.0);
console.log(`  x.multiply(2.0): ${int1.toString()}\n`);

console.log('Test 10: Negate expression');
const neg1 = x.negate();
console.log(`  x.negate(): ${neg1.toString()}\n`);

console.log('Test 11: Quadratic formula example: (-b ± sqrt(b^2 - 4ac)) / 2a');
const a = JsExpression.symbol('a');
const b = JsExpression.symbol('b');
const c = JsExpression.symbol('c');
const discriminant = b.pow(2).subtract(a.multiply(4).multiply(c));
const sqrt = JsExpression.function('sqrt', [discriminant]);
const numerator1 = b.negate().add(sqrt);
const numerator2 = b.negate().subtract(sqrt);
const denom = a.multiply(2);
const solution1 = numerator1.divide(denom);
const solution2 = numerator2.divide(denom);
console.log(`  Discriminant: ${discriminant.toString()}`);
console.log(`  Solution 1: ${solution1.toString()}`);
console.log(`  Solution 2: ${solution2.toString()}\n`);

console.log('=== All Fluent API Tests Complete ===');
