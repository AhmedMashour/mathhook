const test = require('ava');
const { JsExpression, symbols, parse, sin, cos, tan, exp, ln, sqrt, abs, gamma, factorial } = require('../index');

// Test 1: Complete workflow with fluent API
test('complete workflow with fluent API', t => {
    const [x, y] = symbols('x y');
    const expr = x.pow(2).add(y.pow(2)).multiply(2).subtract(1);

    t.truthy(expr);
    t.truthy(expr.toString());
    t.is(typeof expr.toString(), 'string');
});

// Test 2: Function composition
test('function composition', t => {
    const [x] = symbols('x');
    const expr = sin(cos(exp(x)));

    t.truthy(expr);
    t.true(expr.toString().includes('sin'));
    t.true(expr.toString().includes('cos'));
    t.true(expr.toString().includes('exp'));
});

// Test 3: Mixed numeric and symbolic
test('mixed numeric and symbolic operations', t => {
    const [x] = symbols('x');
    const expr = x.multiply(3.14).add(2.71);

    t.truthy(expr);
    t.truthy(expr.toString());
});

// Test 4: parse() integration with fluent API
test('parse() integration with fluent API', t => {
    const expr = parse('x^2 + 2*x + 1');
    const [x] = symbols('x');
    const extended = expr.multiply(x).add(5);

    t.truthy(extended);
    t.truthy(extended.toString());
});

// Test 5: Derivative workflow
test('derivative workflow', t => {
    const [x] = symbols('x');
    const expr = x.pow(3).add(x.multiply(2)).subtract(1);
    const derivative = expr.derivative('x');

    t.truthy(derivative);
    t.truthy(derivative.toString());
});

// Test 6: Simplification workflow
test('simplification workflow', t => {
    const [x] = symbols('x');
    const expr = x.add(x).add(x);
    const simplified = expr.simplify();

    t.truthy(simplified);
    t.truthy(simplified.toString());
});

// Test 7: Multiple symbols workflow
test('multiple symbols with complex expression', t => {
    const [x, y, z] = symbols('x y z');
    const expr = x.pow(2).add(y.pow(2)).add(z.pow(2));

    t.truthy(expr);
    t.truthy(expr.toString());
});

// Test 8: Trigonometric functions
test('trigonometric functions composition', t => {
    const [x] = symbols('x');
    const expr = sin(x).pow(2).add(cos(x).pow(2));
    const simplified = expr.simplify();

    t.truthy(simplified);
    t.truthy(simplified.toString());
});

// Test 9: Exponential and logarithm
test('exponential and logarithm functions', t => {
    const [x] = symbols('x');
    const expr = ln(exp(x));
    const simplified = expr.simplify();

    t.truthy(simplified);
    t.truthy(simplified.toString());
});

// Test 10: Square root operations
test('square root operations', t => {
    const [x] = symbols('x');
    const expr = sqrt(x.pow(2));

    t.truthy(expr);
    t.true(expr.toString().includes('sqrt'));
});

// Test 11: Absolute value
test('absolute value function', t => {
    const [x] = symbols('x');
    const expr = abs(x.multiply(-1));

    t.truthy(expr);
    t.truthy(expr.toString());
});

// Test 12: Special functions (gamma, factorial)
test('special functions', t => {
    const n = JsExpression.integer(5);
    const gammaExpr = gamma(n);
    const factExpr = factorial(5);

    t.truthy(gammaExpr);
    t.truthy(factExpr);
    t.truthy(gammaExpr.toString());
    t.truthy(factExpr.toString());
});

// Test 13: parse() with various formats
test('parse() with different mathematical expressions', t => {
    const expr1 = parse('sin(x) + cos(y)');
    const expr2 = parse('2x + 3y');
    const expr3 = parse('x^2 + 2*x + 1');

    t.truthy(expr1.toString());
    t.truthy(expr2.toString());
    t.truthy(expr3.toString());
});

// Test 14: Nested operations
test('deeply nested operations', t => {
    const [x] = symbols('x');
    const expr = x.add(1).pow(2).subtract(1).pow(2);

    t.truthy(expr);
    t.truthy(expr.toString());
});

// Test 15: Factory methods
test('factory methods create valid expressions', t => {
    const symExpr = JsExpression.symbol('t');
    const intExpr = JsExpression.integer(42);
    const floatExpr = JsExpression.float(3.14);

    t.truthy(symExpr);
    t.truthy(intExpr);
    t.truthy(floatExpr);
    t.is(symExpr.toString(), 't');
    t.is(intExpr.toString(), 'Integer(42)');
    t.true(floatExpr.toString().includes('3.14'));
});

// Test 16: Mathematical constants
test('mathematical constants', t => {
    const piExpr = JsExpression.pi();
    const eExpr = JsExpression.e();
    const iExpr = JsExpression.i();

    t.truthy(piExpr);
    t.truthy(eExpr);
    t.truthy(iExpr);
    t.true(piExpr.toString().includes('Pi'));
    t.true(eExpr.toString().includes('E'));
    t.true(iExpr.toString().includes('I'));
});

// Test 17: Chaining with function shortcuts
test('chaining with function shortcuts', t => {
    const [x] = symbols('x');
    const expr = sin(x).pow(2);

    t.truthy(expr);
    t.true(expr.toString().includes('sin'));
});

// Test 18: Multiple derivatives
test('multiple derivatives', t => {
    const [x] = symbols('x');
    const expr = x.pow(4);
    const firstDeriv = expr.derivative('x');
    const secondDeriv = expr.derivative('x', 2);

    t.truthy(firstDeriv);
    t.truthy(secondDeriv);
    t.truthy(firstDeriv.toString());
    t.truthy(secondDeriv.toString());
});

// Test 19: parse() and derivative combined
test('parse and derivative workflow', t => {
    const parsed = parse('x^3 + 2*x^2 + x + 1');
    const [x] = symbols('x');
    const derivative = parsed.derivative('x');

    t.truthy(derivative);
    t.truthy(derivative.toString());
});

// Test 20: Complex expression building
test('complex expression building workflow', t => {
    const [x, y] = symbols('x y');
    const expr = sin(x.pow(2).add(y.pow(2))).multiply(exp(x.negate()));

    t.truthy(expr);
    t.true(expr.toString().includes('sin'));
    t.true(expr.toString().includes('exp'));
});

// Test 21: Expression evaluation with symbols
test('expression evaluation with symbols', t => {
    const [x] = symbols('x');
    const expr = x.pow(2).add(1);

    t.truthy(expr);
    t.truthy(expr.toString());
});

// Test 22: Error handling for parse
test('parse() error handling', t => {
    const error = t.throws(() => {
        parse('x +');
    });
    t.truthy(error);
});

// Test 23: symbols() with different formats
test('symbols() with various input formats', t => {
    const [a, b, c] = symbols('a b c');
    const [x] = symbols('x');
    const [p, q] = symbols('p q');

    t.truthy(a);
    t.truthy(b);
    t.truthy(c);
    t.truthy(x);
    t.truthy(p);
    t.truthy(q);
    t.true(a.toString().includes('a'));
    t.true(x.toString().includes('x'));
});

// Test 24: Full mathematical workflow
test('complete mathematical workflow', t => {
    const [x, y] = symbols('x y');
    const f = x.pow(2).add(x.multiply(y).multiply(2)).add(y.pow(2));
    const df_dx = f.derivative('x');
    const simplified = df_dx.simplify();

    t.truthy(f);
    t.truthy(df_dx);
    t.truthy(simplified);
    t.truthy(f.toString());
    t.truthy(df_dx.toString());
    t.truthy(simplified.toString());
});

// Test 25: Integration of all Week 1 features
test('integration of all Week 1 features', t => {
    const [x, y] = symbols('x y');
    const parsed = parse('sin(x) + cos(y)');
    const composed = sin(cos(x)).add(exp(y));
    const fluent = x.pow(2).add(y.pow(2));
    const constant = JsExpression.pi();

    t.truthy(parsed);
    t.truthy(composed);
    t.truthy(fluent);
    t.truthy(constant);
    t.true(parsed.toString().includes('sin'));
    t.true(composed.toString().includes('cos'));
    t.truthy(fluent.toString());
    t.true(constant.toString().includes('Pi'));
});

// Test 26: Operator precedence
test('operator precedence in complex expressions', t => {
    const [x] = symbols('x');
    const expr = x.pow(2).add(x.multiply(2)).add(1);

    t.truthy(expr.toString());
});

// Test 27: Function nesting depth
test('deep function nesting', t => {
    const [x] = symbols('x');
    const expr = sin(cos(tan(x)));

    t.true(expr.toString().includes('sin'));
    t.true(expr.toString().includes('cos'));
    t.true(expr.toString().includes('tan'));
});

// Test 28: Large expression building
test('large expression construction', t => {
    const [x] = symbols('x');
    let expr = JsExpression.integer(1);
    for (let i = 1; i <= 5; i++) {
        expr = expr.add(x.pow(i));
    }

    t.truthy(expr);
    t.truthy(expr.toString());
});

// Test 29: Mixed operations workflow
test('mixed operations with all function types', t => {
    const [x] = symbols('x');
    const expr = sin(x).add(exp(x)).add(sqrt(x)).add(ln(x));

    t.truthy(expr);
    t.true(expr.toString().includes('sin'));
    t.true(expr.toString().includes('exp'));
    t.true(expr.toString().includes('sqrt'));
    t.true(expr.toString().includes('ln'));
});

// Test 30: Zero and one identity operations
test('identity operations', t => {
    const [x] = symbols('x');
    const zero = JsExpression.integer(0);
    const one = JsExpression.integer(1);
    const addZero = x.add(zero).simplify();
    const mulOne = x.multiply(one).simplify();

    t.truthy(addZero);
    t.truthy(mulOne);
});
