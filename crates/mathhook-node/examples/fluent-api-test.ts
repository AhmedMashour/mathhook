import { Expression, symbols, sqrt } from "../index.js";

console.log("=== Static API Tests ===\n");

const [x, y] = symbols("x y");

console.log("Test 1: Add with number conversion");
const add1 = Expression.add([x, Expression.integer(2)]);
console.log(`  Expression.add([x, int(2)]): ${add1.format()}\n`);

console.log("Test 2: Multiply with number conversion");
const mul1 = Expression.mul([x, Expression.integer(3)]);
console.log(`  Expression.mul([x, int(3)]): ${mul1.format()}\n`);

console.log("Test 3: Power with number conversion");
const pow1 = Expression.pow(x, Expression.integer(2));
console.log(`  Expression.pow(x, int(2)): ${pow1.format()}\n`);

console.log("Test 4: Division");
const div1 = Expression.div(x, Expression.integer(2));
console.log(`  Expression.div(x, int(2)): ${div1.format()}\n`);

console.log("Test 5: Complex expression building");
const chain1 = Expression.pow(
  Expression.mul([Expression.add([x, Expression.integer(2)]), Expression.integer(3)]),
  Expression.integer(2)
);
console.log(`  ((x + 2) * 3)^2: ${chain1.format()}`);
const chain1Simplified = chain1.simplify();
console.log(`  Simplified: ${chain1Simplified.format()}\n`);

console.log("Test 6: Quadratic expression with static API");
const expr1 = Expression.add([
  Expression.pow(x, Expression.integer(2)),
  Expression.mul([x, Expression.integer(2)]),
  Expression.integer(1),
]);
console.log(`  x^2 + 2*x + 1: ${expr1.format()}`);
const expr1Simplified = expr1.simplify();
console.log(`  Simplified: ${expr1Simplified.format()}\n`);

console.log("Test 7: Float values");
const float1 = Expression.mul([x, Expression.float(3.14)]);
console.log(`  x * 3.14: ${float1.format()}\n`);

console.log("Test 8: Negate expression");
const neg1 = x.negate();
console.log(`  x.negate(): ${neg1.format()}\n`);

console.log("Test 9: Quadratic formula example: (-b +/- sqrt(b^2 - 4ac)) / 2a");
const [a, b, c] = symbols("a b c");
const discriminant = Expression.add([
  Expression.pow(b, Expression.integer(2)),
  Expression.mul([Expression.integer(-4), a, c]),
]);
const sqrtDisc = sqrt(discriminant);
const denom = Expression.mul([a, Expression.integer(2)]);
const solution1 = Expression.div(Expression.add([b.negate(), sqrtDisc]), denom);
const solution2 = Expression.div(
  Expression.add([b.negate(), Expression.mul([Expression.integer(-1), sqrtDisc])]),
  denom
);
console.log(`  Discriminant: ${discriminant.format()}`);
console.log(`  Solution 1: ${solution1.format()}`);
console.log(`  Solution 2: ${solution2.format()}\n`);

console.log("=== All Static API Tests Complete ===");
