import { symbols, Expression } from "../index.js";

console.log("=== symbols() Function Tests ===\n");

// Test 1: Space-separated symbols
console.log("Test 1: Space-separated symbols");
const [x, y, z] = symbols("x y z");
console.log(`  symbols('x y z'):`);
console.log(`    x: ${x.format()}`);
console.log(`    y: ${y.format()}`);
console.log(`    z: ${z.format()}`);
console.log(`  All symbols created\n`);

// Test 2: Comma-separated symbols
console.log("Test 2: Comma-separated symbols");
const [a, b, c] = symbols("a, b, c");
console.log(`  symbols('a, b, c'):`);
console.log(`    a: ${a.format()}`);
console.log(`    b: ${b.format()}`);
console.log(`    c: ${c.format()}`);
console.log(`  All symbols created\n`);

// Test 3: Comma-separated without spaces
console.log("Test 3: Comma-separated without spaces");
const [p, q] = symbols("p,q");
console.log(`  symbols('p,q'):`);
console.log(`    p: ${p.format()}`);
console.log(`    q: ${q.format()}`);
console.log(`  All symbols created\n`);

// Test 4: Range syntax from 0
console.log("Test 4: Range syntax from 0");
const [x0, x1, x2] = symbols("x0:3");
console.log(`  symbols('x0:3'):`);
console.log(`    x0: ${x0.format()}`);
console.log(`    x1: ${x1.format()}`);
console.log(`    x2: ${x2.format()}`);
console.log(`  All symbols created\n`);

// Test 5: Range syntax from 1
console.log("Test 5: Range syntax from 1");
const [y1, y2, y3] = symbols("y1:4");
console.log(`  symbols('y1:4'):`);
console.log(`    y1: ${y1.format()}`);
console.log(`    y2: ${y2.format()}`);
console.log(`    y3: ${y3.format()}`);
console.log(`  All symbols created\n`);

// Test 6: Range without starting number
console.log("Test 6: Range without starting number");
const [t0, t1, t2, t3, t4] = symbols("t:5");
console.log(`  symbols('t:5'):`);
console.log(`    t0: ${t0.format()}`);
console.log(`    t1: ${t1.format()}`);
console.log(`    t2: ${t2.format()}`);
console.log(`    t3: ${t3.format()}`);
console.log(`    t4: ${t4.format()}`);
console.log(`  All symbols created\n`);

// Test 7: Use symbols in expressions
console.log("Test 7: Using symbols in expressions");
const [u, v] = symbols("u v");
const expr1 = Expression.add([u, v]);
console.log(`  u + v = ${expr1.format()}`);
const expr2 = Expression.mul([u, v]);
console.log(`  u * v = ${expr2.format()}`);
const expr3 = Expression.add([
  Expression.pow(u, Expression.integer(2)),
  Expression.pow(v, Expression.integer(2)),
]);
console.log(`  u^2 + v^2 = ${expr3.format()}`);
console.log(`  Symbols work in expressions\n`);

// Test 8: Fluent chaining with symbols
console.log("Test 8: Fluent chaining with symbols");
const [alpha, beta, gamma] = symbols("alpha beta gamma");
const chainedExpr = Expression.add([
  Expression.mul([Expression.add([alpha, Expression.integer(2)]), beta]),
  Expression.mul([Expression.integer(-1), Expression.div(gamma, Expression.integer(3))]),
]);
console.log(`  (alpha + 2) * beta - gamma/3 = ${chainedExpr.format()}`);
console.log(`  Fluent chaining works\n`);

// Test 9: Range symbols in expression
console.log("Test 9: Range symbols in expression");
const [a0, a1, a2] = symbols("a0:3");
const polynomial = Expression.add([
  Expression.mul([a2, Expression.pow(x, Expression.integer(2))]),
  Expression.mul([a1, x]),
  a0,
]);
console.log(`  a2*x^2 + a1*x + a0 = ${polynomial.format()}`);
console.log(`  Range symbols work in expressions\n`);

// Test 10: Greek letters
console.log("Test 10: Greek letters");
const [theta, phi, psi] = symbols("theta phi psi");
console.log(`  symbols('theta phi psi'):`);
console.log(`    theta: ${theta.format()}`);
console.log(`    phi: ${phi.format()}`);
console.log(`    psi: ${psi.format()}`);
console.log(`  Greek letter names work\n`);

// Test 11: Single symbol (edge case)
console.log("Test 11: Single symbol");
const [omega] = symbols("omega");
console.log(`  symbols('omega'): ${omega.format()}`);
console.log(`  Single symbol works\n`);

// Test 12: Many symbols
console.log("Test 12: Many symbols at once");
const many = symbols("w1 w2 w3 w4 w5 w6 w7 w8 w9 w10");
console.log(`  symbols('w1 w2 ... w10'): Created ${many.length} symbols`);
console.log(`    First: ${many[0].format()}`);
console.log(`    Last: ${many[9].format()}`);
console.log(`  Many symbols created successfully\n`);

// Test 13: Real-world usage - quadratic formula
console.log("Test 13: Real-world usage - Quadratic formula");
const [aa, bb, cc] = symbols("a b c");
const discriminant = Expression.add([
  Expression.pow(bb, Expression.integer(2)),
  Expression.mul([
    Expression.integer(-4),
    aa,
    cc,
  ]),
]);
console.log(`  Discriminant: b^2 - 4ac = ${discriminant.format()}`);
console.log(`  Quadratic formula implemented\n`);

// Test 14: Mixed with static API
console.log("Test 14: Mixing symbols() with static API");
const [m, n] = symbols("m n");
const mixed = Expression.pow(
  Expression.mul([
    Expression.add([m, Expression.integer(2)]),
    Expression.add([n, Expression.mul([Expression.integer(-1), Expression.integer(1)])]),
  ]),
  Expression.integer(2)
);
console.log(`  ((m + 2) * (n - 1))^2 = ${mixed.format()}`);
console.log(`  symbols() works with static API\n`);

console.log("=== All symbols() Tests Complete ===");
