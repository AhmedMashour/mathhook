import test from "ava";
import {
  Expression,
  Symbol as MathSymbol,
  symbols,
  parse,
  sin,
  cos,
  tan,
  exp,
  ln,
  sqrt,
  abs,
  gamma,
  factorial,
} from "../index.js";

// Helper functions for cleaner test code
const add = (...args: Expression[]): Expression => Expression.add(args);
const mul = (...args: Expression[]): Expression => Expression.mul(args);
const pow = (base: Expression, exponent: Expression): Expression =>
  Expression.pow(base, exponent);
const int = (n: number): Expression => Expression.integer(n);
const float = (n: number): Expression => Expression.float(n);

// Test 1: Complete workflow with static API
test("complete workflow with static API", (t) => {
  const [x, y] = symbols("x y");
  // (x^2 + y^2) * 2 - 1
  const expr = add(mul(int(2), add(pow(x, int(2)), pow(y, int(2)))), int(-1));

  t.truthy(expr);
  t.truthy(expr.toString());
  t.is(typeof expr.toString(), "string");
});

// Test 2: Function composition
test("function composition", (t) => {
  const [x] = symbols("x");
  const expr = sin(cos(exp(x)));

  t.truthy(expr);
  const str = expr.format();
  t.true(str.includes("sin"));
  t.true(str.includes("cos"));
  t.true(str.includes("exp"));
});

// Test 3: Mixed numeric and symbolic using parse
test("mixed numeric and symbolic operations", (t) => {
  const expr = parse("x * 3.14 + 2.71");

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 4: parse() integration
test("parse() integration", (t) => {
  const expr = parse("x^2 + 2*x + 1");
  const [x] = symbols("x");
  const extended = add(mul(expr, x), int(5));

  t.truthy(extended);
  t.truthy(extended.toString());
});

// Test 5: Derivative workflow
test("derivative workflow", (t) => {
  const [x] = symbols("x");
  // x^3 + 2*x - 1
  const expr = add(pow(x, int(3)), mul(int(2), x), int(-1));
  const derivative = expr.derivative(x.asSymbol()!);

  t.truthy(derivative);
  t.truthy(derivative.toString());
});

// Test 6: Simplification workflow
test("simplification workflow", (t) => {
  const [x] = symbols("x");
  const expr = add(x, x, x);
  const simplified = expr.simplify();

  t.truthy(simplified);
  t.truthy(simplified.toString());
});

// Test 7: Multiple symbols workflow
test("multiple symbols with complex expression", (t) => {
  const [x, y, z] = symbols("x y z");
  const expr = add(pow(x, int(2)), pow(y, int(2)), pow(z, int(2)));

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 8: Trigonometric functions
test("trigonometric functions composition", (t) => {
  const [x] = symbols("x");
  // sin(x)^2 + cos(x)^2
  const expr = add(pow(sin(x), int(2)), pow(cos(x), int(2)));
  const simplified = expr.simplify();

  t.truthy(simplified);
  t.truthy(simplified.toString());
});

// Test 9: Exponential and logarithm
test("exponential and logarithm functions", (t) => {
  const [x] = symbols("x");
  const expr = ln(exp(x));
  const simplified = expr.simplify();

  t.truthy(simplified);
  t.truthy(simplified.toString());
});

// Test 10: Square root operations
test("square root operations", (t) => {
  const [x] = symbols("x");
  const expr = sqrt(pow(x, int(2)));

  t.truthy(expr);
  t.true(expr.format().includes("sqrt"));
});

// Test 11: Absolute value
test("absolute value function", (t) => {
  const [x] = symbols("x");
  const negX = x.negate();
  const expr = abs(negX);

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 12: Special functions (gamma, factorial)
test("special functions", (t) => {
  const n = int(5);
  const gammaExpr = gamma(n);
  // Test with number directly - ExpressionOrNumber supports both
  const factExpr = factorial(5);

  t.truthy(gammaExpr);
  t.truthy(factExpr);
  t.truthy(gammaExpr.toString());
  t.truthy(factExpr.toString());
});

// Test 13: parse() with various formats
test("parse() with different mathematical expressions", (t) => {
  const expr1 = parse("sin(x) + cos(y)");
  const expr2 = parse("2x + 3y");
  const expr3 = parse("x^2 + 2*x + 1");

  t.truthy(expr1.toString());
  t.truthy(expr2.toString());
  t.truthy(expr3.toString());
});

// Test 14: Nested operations using parse
test("deeply nested operations", (t) => {
  const expr = parse("((x + 1)^2 - 1)^2");

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 15: Factory methods
test("factory methods create valid expressions", (t) => {
  const [symExpr] = symbols("t");
  const intExpr = int(42);
  const floatExpr = float(3.14);

  t.truthy(symExpr);
  t.truthy(intExpr);
  t.truthy(floatExpr);
  t.is(symExpr.format(), "t");
  t.is(intExpr.format(), "42");
  t.true(floatExpr.format().includes("3.14"));
});

// Test 16: Mathematical constants
test("mathematical constants", (t) => {
  const piExpr = Expression.pi();
  const eExpr = Expression.e();
  const iExpr = Expression.i();

  t.truthy(piExpr);
  t.truthy(eExpr);
  t.truthy(iExpr);
  t.is(piExpr.format(), "\\pi");
  t.is(eExpr.format(), "e");
  t.is(iExpr.format(), "i");
});

// Test 17: Chaining with function shortcuts
test("chaining with function shortcuts", (t) => {
  const [x] = symbols("x");
  const expr = pow(sin(x), int(2));

  t.truthy(expr);
  t.true(expr.format().includes("sin"));
});

// Test 18: Multiple derivatives
test("multiple derivatives", (t) => {
  const [x] = symbols("x");
  const expr = pow(x, int(4));
  const firstDeriv = expr.derivative(x.asSymbol()!);
  const secondDeriv = expr.nthDerivative(x.asSymbol()!, 2);

  t.truthy(firstDeriv);
  t.truthy(secondDeriv);
  t.truthy(firstDeriv.toString());
  t.truthy(secondDeriv.toString());
});

// Test 19: parse and derivative combined
test("parse and derivative workflow", (t) => {
  const parsed = parse("x^3 + 2*x^2 + x + 1");
  const [x] = symbols("x");
  const derivative = parsed.derivative(x.asSymbol()!);

  t.truthy(derivative);
  t.truthy(derivative.toString());
});

// Test 20: Complex expression building
test("complex expression building workflow", (t) => {
  const [x, y] = symbols("x y");
  // sin(x^2 + y^2) * exp(-x)
  const expr = mul(sin(add(pow(x, int(2)), pow(y, int(2)))), exp(x.negate()));

  t.truthy(expr);
  t.true(expr.format().includes("sin"));
  t.true(expr.format().includes("exp"));
});

// Test 21: Expression evaluation with symbols
test("expression evaluation with symbols", (t) => {
  const [x] = symbols("x");
  const expr = add(pow(x, int(2)), int(1));

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 22: Error handling for parse
test("parse() error handling", (t) => {
  const error = t.throws(() => {
    parse("x +");
  });
  t.truthy(error);
});

// Test 23: symbols() with different formats
test("symbols() with various input formats", (t) => {
  const [a, b, c] = symbols("a b c");
  const [x] = symbols("x");
  const [p, q] = symbols("p q");

  t.truthy(a);
  t.truthy(b);
  t.truthy(c);
  t.truthy(x);
  t.truthy(p);
  t.truthy(q);
  t.true(a.format().includes("a"));
  t.true(x.format().includes("x"));
});

// Test 24: Full mathematical workflow
test("complete mathematical workflow", (t) => {
  const [x, y] = symbols("x y");
  // f = x^2 + 2*x*y + y^2
  const f = add(pow(x, int(2)), mul(int(2), x, y), pow(y, int(2)));
  const df_dx = f.derivative(x.asSymbol()!);
  const simplified = df_dx.simplify();

  t.truthy(f);
  t.truthy(df_dx);
  t.truthy(simplified);
  t.truthy(f.toString());
  t.truthy(df_dx.toString());
  t.truthy(simplified.toString());
});

// Test 25: Integration of all Week 1 features
test("integration of all Week 1 features", (t) => {
  const [x, y] = symbols("x y");
  const parsed = parse("sin(x) + cos(y)");
  const composed = add(sin(cos(x)), exp(y));
  const fluent = add(pow(x, int(2)), pow(y, int(2)));
  const constant = Expression.pi();

  t.truthy(parsed);
  t.truthy(composed);
  t.truthy(fluent);
  t.truthy(constant);
  t.true(parsed.format().includes("sin"));
  t.true(composed.format().includes("cos"));
  t.truthy(fluent.format());
  t.true(constant.format().includes("pi") || constant.format().includes("\\pi"));
});

// Test 26: Operator precedence
test("operator precedence in complex expressions", (t) => {
  const [x] = symbols("x");
  const expr = add(pow(x, int(2)), mul(int(2), x), int(1));

  t.truthy(expr.toString());
});

// Test 27: Function nesting depth
test("deep function nesting", (t) => {
  const [x] = symbols("x");
  const expr = sin(cos(tan(x)));

  t.true(expr.format().includes("sin"));
  t.true(expr.format().includes("cos"));
  t.true(expr.format().includes("tan"));
});

// Test 28: Large expression building
test("large expression construction", (t) => {
  const [x] = symbols("x");
  const terms: Expression[] = [int(1)];
  for (let i = 1; i <= 5; i++) {
    terms.push(pow(x, int(i)));
  }
  const expr = add(...terms);

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 29: Mixed operations workflow
test("mixed operations with all function types", (t) => {
  const [x] = symbols("x");
  const expr = add(sin(x), exp(x), sqrt(x), ln(x));

  t.truthy(expr);
  t.true(expr.format().includes("sin"));
  t.true(expr.format().includes("exp"));
  t.true(expr.format().includes("sqrt"));
  t.true(expr.format().includes("ln"));
});

// Test 30: Zero and one identity operations
test("identity operations", (t) => {
  const [x] = symbols("x");
  const zero = int(0);
  const one = int(1);
  const addZero = add(x, zero).simplify();
  const mulOne = mul(x, one).simplify();

  t.truthy(addZero);
  t.truthy(mulOne);
});

// Test 31: Division expression
test("division expression", (t) => {
  const [x] = symbols("x");
  const expr = Expression.div(x, int(2));

  t.truthy(expr);
  t.truthy(expr.toString());
});

// Test 32: Complex number creation
test("complex number creation", (t) => {
  const z = Expression.complex(int(3), int(4));

  t.truthy(z);
  t.truthy(z.toString());
});

// Test 33: Matrix creation
test("matrix creation", (t) => {
  const identity = Expression.identityMatrix(3);

  t.truthy(identity);
  t.true(identity.isIdentityMatrix());
});

// Test 34: Expression classification
test("expression classification", (t) => {
  const [x] = symbols("x");
  const poly = add(pow(x, int(2)), int(1));
  const classification = poly.classify();

  t.truthy(classification);
});

// Test 35: Expression expansion
test("expression expansion", (t) => {
  const expr = parse("(x + 1)^2");
  const expanded = expr.expand();

  t.truthy(expanded);
  t.truthy(expanded.toString());
});

// Test 36: Substitution
test("expression substitution", (t) => {
  const [x] = symbols("x");
  const expr = add(pow(x, int(2)), x);
  const substituted = expr.substitute({ x: int(2) });

  t.truthy(substituted);
  t.truthy(substituted.toString());
});

// Test 37: Find variables
test("find variables in expression", (t) => {
  const expr = parse("x^2 + y + z");
  const vars = expr.findVariables();

  t.truthy(vars);
  t.true(vars.length >= 1);
});

// Test 38: IsZero and IsOne checks
test("zero and one checks", (t) => {
  const zero = int(0);
  const one = int(1);
  const two = int(2);

  t.true(zero.isZero());
  t.true(one.isOne());
  t.false(two.isZero());
  t.false(two.isOne());
});

// Test 39: Polynomial operations
test("polynomial GCD", (t) => {
  const expr1 = parse("x^2 - 1");
  const expr2 = parse("x - 1");
  const gcd = expr1.polynomialGcd(expr2);

  t.truthy(gcd);
  t.truthy(gcd.toString());
});

// Test 40: LaTeX formatting
test("LaTeX formatting", (t) => {
  const [x] = symbols("x");
  const expr = pow(x, int(2));
  const latex = expr.format();

  t.truthy(latex);
  t.is(typeof latex, "string");
});
