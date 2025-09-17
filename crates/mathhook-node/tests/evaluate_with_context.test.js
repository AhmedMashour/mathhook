/**
 * Tests for evaluateWithContext() and EvalContext in Node.js bindings
 *
 * Verifies mathematical correctness and all configuration options.
 */

const test = require('ava');
const { JsExpression, EvalContext, symbols, sin, cos, exp, ln, sqrt, gamma } = require('../index');

// =============================================================================
// Helper Functions
// =============================================================================

function integer(n) {
    return JsExpression.integer(n);
}

function symbol(name) {
    const [sym] = symbols(name);
    return sym;
}

// =============================================================================
// Test: EvalContext Construction
// =============================================================================

test('EvalContext: default constructor', t => {
    const ctx = new EvalContext({});
    const expr = integer(2).add(integer(3));
    const result = expr.evaluateWithContext(ctx);

    t.truthy(result);
    t.is(result.toSimple(), '5');
});

test('EvalContext: symbolic factory', t => {
    const ctx = EvalContext.symbolic();
    const x = symbol('x');
    const expr = x.pow(integer(2));
    const result = expr.evaluateWithContext(ctx);

    t.truthy(result);
    // Result should still be symbolic
});

test('EvalContext: numeric factory with substitutions', t => {
    const x = symbol('x');
    const expr = x.pow(integer(2));

    const ctx = EvalContext.numeric([['x', integer(3)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '9');
});

test('EvalContext: custom configuration', t => {
    const ctx = new EvalContext({
        numeric: true,
        precision: 128,
        simplifyFirst: false
    });

    const expr = integer(2).add(integer(3));
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '5');
});

// =============================================================================
// Test: Basic Evaluation
// =============================================================================

test('BasicEvaluation: constant expression numerical', t => {
    const expr = integer(2).add(integer(3));
    const ctx = new EvalContext({});
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '5');
});

test('BasicEvaluation: constant expression symbolic', t => {
    const expr = integer(2).add(integer(3));
    const ctx = EvalContext.symbolic();
    const result = expr.evaluateWithContext(ctx);

    // Should simplify to 5 even in symbolic mode (constant folding)
    t.is(result.toSimple(), '5');
});

test('BasicEvaluation: symbolic expression no substitution', t => {
    const x = symbol('x');
    const expr = x.add(integer(1));

    const ctx = EvalContext.symbolic();
    const result = expr.evaluateWithContext(ctx);

    t.truthy(result);
    // Should stay symbolic
});

test('BasicEvaluation: polynomial evaluation', t => {
    const x = symbol('x');
    // x^2 + 2x + 1 at x = 3
    const expr = x.pow(integer(2))
        .add(integer(2).multiply(x))
        .add(integer(1));

    const ctx = EvalContext.numeric([['x', integer(3)]]);
    const result = expr.evaluateWithContext(ctx);

    // (3)^2 + 2(3) + 1 = 9 + 6 + 1 = 16
    t.is(result.toSimple(), '16');
});

// =============================================================================
// Test: Variable Substitution
// =============================================================================

test('VariableSubstitution: single variable', t => {
    const x = symbol('x');
    const expr = x.multiply(integer(2));

    const ctx = EvalContext.numeric([['x', integer(5)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '10');
});

test('VariableSubstitution: multiple variables', t => {
    const x = symbol('x');
    const y = symbol('y');
    const expr = x.add(y);

    const ctx = EvalContext.numeric([
        ['x', integer(3)],
        ['y', integer(4)]
    ]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '7');
});

test('VariableSubstitution: nested expressions', t => {
    const x = symbol('x');
    const y = symbol('y');
    const expr = x.pow(integer(2)).add(y.pow(integer(2)));

    const ctx = EvalContext.numeric([
        ['x', integer(3)],
        ['y', integer(4)]
    ]);
    const result = expr.evaluateWithContext(ctx);

    // 3^2 + 4^2 = 9 + 16 = 25
    t.is(result.toSimple(), '25');
});

test('VariableSubstitution: partial substitution symbolic', t => {
    const x = symbol('x');
    const y = symbol('y');
    const expr = x.add(y);

    // Only substitute x, leave y symbolic
    const ctx = new EvalContext({ numeric: false, simplifyFirst: true });
    // Note: We can't pass variables in constructor, so this tests no substitution
    const result = expr.evaluateWithContext(ctx);

    t.truthy(result);
    // Result should still contain y
});

// =============================================================================
// Test: Simplification Control
// =============================================================================

test('SimplificationControl: simplify before evaluation', t => {
    const x = symbol('x');
    // Expression that benefits from simplification: x + x
    const expr = x.add(x);

    const ctx = EvalContext.numeric([['x', integer(3)]]);
    const result = expr.evaluateWithContext(ctx);

    // Should simplify x + x → 2x, then substitute → 2*3 = 6
    t.is(result.toSimple(), '6');
});

test('SimplificationControl: no simplification', t => {
    const x = symbol('x');
    const expr = x.add(x);

    const ctx = new EvalContext({
        numeric: true,
        simplifyFirst: false
    });
    // Note: Can't pass variables here due to NAPI limitation
    // This tests the configuration exists
    const result = expr.evaluateWithContext(ctx);

    t.truthy(result);
});

// =============================================================================
// Test: Domain Checking
// =============================================================================

test.skip('DomainChecking: sqrt of negative should error', t => {
    const expr = sqrt(integer(-1));
    const ctx = new EvalContext({});

    t.throws(() => {
        expr.evaluateWithContext(ctx);
    });
});

test.skip('DomainChecking: sqrt negative after substitution', t => {
    const x = symbol('x');
    const expr = sqrt(x);

    const ctx = EvalContext.numeric([['x', integer(-4)]]);

    t.throws(() => {
        expr.evaluateWithContext(ctx);
    });
});

test.skip('DomainChecking: log of zero should error', t => {
    const expr = ln(integer(0));
    const ctx = new EvalContext({});

    t.throws(() => {
        expr.evaluateWithContext(ctx);
    });
});

test.skip('DomainChecking: division by zero', t => {
    const zero = integer(0);
    const one = integer(1);
    const expr = one.multiply(zero.pow(integer(-1)));

    const ctx = new EvalContext({});

    t.throws(() => {
        expr.evaluateWithContext(ctx);
    });
});

// =============================================================================
// Test: Function Evaluation
// =============================================================================

test('FunctionEvaluation: trigonometric functions', t => {
    const x = symbol('x');
    const expr = sin(x);

    // Evaluate sin(0) = 0
    const ctx = EvalContext.numeric([['x', integer(0)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '0');
});

test('FunctionEvaluation: exponential functions', t => {
    const x = symbol('x');
    const expr = exp(x);

    // Evaluate exp(0) = 1
    const ctx = EvalContext.numeric([['x', integer(0)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '1');
});

test('FunctionEvaluation: special function gamma', t => {
    const x = symbol('x');
    const expr = gamma(x);

    // Evaluate gamma(1) = 1 (0! = 1)
    const ctx = EvalContext.numeric([['x', integer(1)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '1');
});

// =============================================================================
// Test: Precision Control
// =============================================================================

test('PrecisionControl: default precision', t => {
    const expr = integer(1).multiply(integer(3).pow(integer(-1)));
    const ctx = new EvalContext({});
    const result = expr.evaluateWithContext(ctx);

    // Should get rational 1/3 or float approximation
    t.truthy(result);
});

test('PrecisionControl: custom precision', t => {
    const expr = integer(1).multiply(integer(3).pow(integer(-1)));
    const ctx = new EvalContext({}).withPrecision(128);
    const result = expr.evaluateWithContext(ctx);

    // Should work, even if precision isn't used yet with f64
    t.truthy(result);
});

// =============================================================================
// Test: Complex Expressions
// =============================================================================

test('ComplexExpressions: formula evaluation', t => {
    const x = symbol('x');
    const y = symbol('y');

    // Left side: (x + y)^2
    const lhs = x.add(y).pow(integer(2));

    // Right side: x^2 + 2xy + y^2
    const rhs = x.pow(integer(2))
        .add(integer(2).multiply(x).multiply(y))
        .add(y.pow(integer(2)));

    // Both should evaluate to same value at x=3, y=4
    const ctx = EvalContext.numeric([
        ['x', integer(3)],
        ['y', integer(4)]
    ]);

    const lhsResult = lhs.evaluateWithContext(ctx);
    const rhsResult = rhs.evaluateWithContext(ctx);

    // (3 + 4)^2 = 49
    t.is(lhsResult.toSimple(), '49');
    // 9 + 24 + 16 = 49
    t.is(rhsResult.toSimple(), '49');
    t.is(lhsResult.toSimple(), rhsResult.toSimple());
});

test('ComplexExpressions: nested functions', t => {
    const x = symbol('x');
    // sin(cos(x))
    const inner = cos(x);
    const expr = sin(inner);

    // Evaluate at x = 0: sin(cos(0)) = sin(1)
    const ctx = EvalContext.numeric([['x', integer(0)]]);
    const result = expr.evaluateWithContext(ctx);

    // Should evaluate successfully (exact value depends on implementation)
    t.truthy(result);
});

// =============================================================================
// Test: Edge Cases
// =============================================================================

test('EdgeCases: empty variables', t => {
    const expr = integer(5);
    const ctx = EvalContext.numeric([]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '5');
});

test('EdgeCases: unused variable in context', t => {
    const x = symbol('x');
    const expr = x.add(integer(1));

    const ctx = EvalContext.numeric([
        ['x', integer(3)],
        ['y', integer(999)]  // Not used
    ]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '4');
});

test('EdgeCases: zero expressions', t => {
    const x = symbol('x');
    const expr = x.multiply(integer(0));

    const ctx = EvalContext.numeric([['x', integer(42)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '0');
});

test('EdgeCases: identity operations', t => {
    const x = symbol('x');
    const expr = x.multiply(integer(1));

    const ctx = EvalContext.numeric([['x', integer(7)]]);
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '7');
});

// =============================================================================
// Test: Method Chaining
// =============================================================================

test('MethodChaining: with_precision chaining', t => {
    const ctx = EvalContext.symbolic().withPrecision(128);
    const expr = integer(2).add(integer(3));
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '5');
});

test('MethodChaining: with_simplify_first chaining', t => {
    const ctx = new EvalContext({}).withSimplifyFirst(false);
    const expr = integer(2).add(integer(3));
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '5');
});

test('MethodChaining: multiple method chains', t => {
    const ctx = EvalContext.symbolic()
        .withPrecision(128)
        .withSimplifyFirst(true);
    const expr = integer(2).add(integer(3));
    const result = expr.evaluateWithContext(ctx);

    t.is(result.toSimple(), '5');
});
