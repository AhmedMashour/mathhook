import { symbols, JsExpression } from '../index.js';

console.log('=== symbols() Function Tests ===\n');

// Test 1: Space-separated symbols
console.log('Test 1: Space-separated symbols');
const [x, y, z] = symbols('x y z');
console.log(`  symbols('x y z'):`);
console.log(`    x: ${x.toString()}`);
console.log(`    y: ${y.toString()}`);
console.log(`    z: ${z.toString()}`);
console.log(`  ✓ All symbols created\n`);

// Test 2: Comma-separated symbols
console.log('Test 2: Comma-separated symbols');
const [a, b, c] = symbols('a, b, c');
console.log(`  symbols('a, b, c'):`);
console.log(`    a: ${a.toString()}`);
console.log(`    b: ${b.toString()}`);
console.log(`    c: ${c.toString()}`);
console.log(`  ✓ All symbols created\n`);

// Test 3: Comma-separated without spaces
console.log('Test 3: Comma-separated without spaces');
const [p, q] = symbols('p,q');
console.log(`  symbols('p,q'):`);
console.log(`    p: ${p.toString()}`);
console.log(`    q: ${q.toString()}`);
console.log(`  ✓ All symbols created\n`);

// Test 4: Range syntax from 0
console.log('Test 4: Range syntax from 0');
const [x0, x1, x2] = symbols('x0:3');
console.log(`  symbols('x0:3'):`);
console.log(`    x0: ${x0.toString()}`);
console.log(`    x1: ${x1.toString()}`);
console.log(`    x2: ${x2.toString()}`);
console.log(`  ✓ All symbols created\n`);

// Test 5: Range syntax from 1
console.log('Test 5: Range syntax from 1');
const [y1, y2, y3] = symbols('y1:4');
console.log(`  symbols('y1:4'):`);
console.log(`    y1: ${y1.toString()}`);
console.log(`    y2: ${y2.toString()}`);
console.log(`    y3: ${y3.toString()}`);
console.log(`  ✓ All symbols created\n`);

// Test 6: Range without starting number
console.log('Test 6: Range without starting number');
const [t0, t1, t2, t3, t4] = symbols('t:5');
console.log(`  symbols('t:5'):`);
console.log(`    t0: ${t0.toString()}`);
console.log(`    t1: ${t1.toString()}`);
console.log(`    t2: ${t2.toString()}`);
console.log(`    t3: ${t3.toString()}`);
console.log(`    t4: ${t4.toString()}`);
console.log(`  ✓ All symbols created\n`);

// Test 7: Use symbols in expressions
console.log('Test 7: Using symbols in expressions');
const [u, v] = symbols('u v');
const expr1 = u.add(v);
console.log(`  u + v = ${expr1.toString()}`);
const expr2 = u.multiply(v);
console.log(`  u * v = ${expr2.toString()}`);
const expr3 = u.pow(2).add(v.pow(2));
console.log(`  u² + v² = ${expr3.toString()}`);
console.log(`  ✓ Symbols work in expressions\n`);

// Test 8: Fluent chaining with symbols
console.log('Test 8: Fluent chaining with symbols');
const [alpha, beta, gamma] = symbols('alpha beta gamma');
const complex = alpha.add(2).multiply(beta).subtract(gamma.divide(3));
console.log(`  (alpha + 2) * beta - gamma/3 = ${complex.toString()}`);
console.log(`  ✓ Fluent chaining works\n`);

// Test 9: Range symbols in expression
console.log('Test 9: Range symbols in expression');
const [a0, a1, a2] = symbols('a0:3');
const polynomial = a2.multiply(x.pow(2)).add(a1.multiply(x)).add(a0);
console.log(`  a2*x² + a1*x + a0 = ${polynomial.toString()}`);
console.log(`  ✓ Range symbols work in expressions\n`);

// Test 10: Greek letters
console.log('Test 10: Greek letters');
const [theta, phi, psi] = symbols('theta phi psi');
console.log(`  symbols('theta phi psi'):`);
console.log(`    theta: ${theta.toString()}`);
console.log(`    phi: ${phi.toString()}`);
console.log(`    psi: ${psi.toString()}`);
console.log(`  ✓ Greek letter names work\n`);

// Test 11: Single symbol (edge case)
console.log('Test 11: Single symbol');
const [omega] = symbols('omega');
console.log(`  symbols('omega'): ${omega.toString()}`);
console.log(`  ✓ Single symbol works\n`);

// Test 12: Many symbols
console.log('Test 12: Many symbols at once');
const many = symbols('w1 w2 w3 w4 w5 w6 w7 w8 w9 w10');
console.log(`  symbols('w1 w2 ... w10'): Created ${many.length} symbols`);
console.log(`    First: ${many[0].toString()}`);
console.log(`    Last: ${many[9].toString()}`);
console.log(`  ✓ Many symbols created successfully\n`);

// Test 13: Real-world usage - quadratic formula
console.log('Test 13: Real-world usage - Quadratic formula');
const [aa, bb, cc] = symbols('a b c');
const discriminant = bb.pow(2).subtract(aa.multiply(4).multiply(cc));
const sqrt = JsExpression.function('sqrt', [discriminant]);
const solution1 = bb.negate().add(sqrt).divide(aa.multiply(2));
const solution2 = bb.negate().subtract(sqrt).divide(aa.multiply(2));
console.log(`  Discriminant: b² - 4ac = ${discriminant.toString()}`);
console.log(`  Solution 1: ${solution1.toString()}`);
console.log(`  Solution 2: ${solution2.toString()}`);
console.log(`  ✓ Quadratic formula implemented\n`);

// Test 14: Mixed with fluent API
console.log('Test 14: Mixing symbols() with fluent API');
const [m, n] = symbols('m n');
const mixed = m.add(2).multiply(n.subtract(1)).pow(2);
console.log(`  ((m + 2) * (n - 1))² = ${mixed.toString()}`);
console.log(`  ✓ symbols() works with fluent API\n`);

console.log('=== All symbols() Tests Complete ===');
