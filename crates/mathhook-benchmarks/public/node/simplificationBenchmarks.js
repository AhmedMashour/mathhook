#!/usr/bin/env node
/**
 * Simplification Benchmarks
 * ==========================
 *
 * Mirrors: benches/simplification_benchmarks.rs
 * Tests: Algebraic simplification, trigonometric identities, polynomial reduction
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { JsExpression, JsSymbol, parse } = mathhook;

// ============================================================================
// Basic Algebraic Simplification
// ============================================================================

function bench_simplify_like_terms_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.add([
        JsExpression.symbol(x),
        JsExpression.symbol(x),
        JsExpression.symbol(x)
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_like_terms_with_parsing() {
    const expr = parse("x + x + x");
    const result = expr.simplify();
    return result;
}

function bench_simplify_polynomial_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.multiply([JsExpression.integer(2), JsExpression.symbol(x)]),
        JsExpression.integer(1),
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(-1)
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_polynomial_with_parsing() {
    const expr = parse("x^2 + 2*x + 1 + x^2 - 1");
    const result = expr.simplify();
    return result;
}

function bench_simplify_difference_of_squares_direct() {
    const x = new JsSymbol("x");
    // (x+1)^2 - (x^2 + 2x + 1) should simplify to 0
    const expr1 = JsExpression.pow(
        JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]),
        JsExpression.integer(2)
    );
    const expr2 = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.multiply([JsExpression.integer(2), JsExpression.symbol(x)]),
        JsExpression.integer(1)
    ]);
    const expr = JsExpression.add([expr1, JsExpression.multiply([JsExpression.integer(-1), expr2])]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_difference_of_squares_with_parsing() {
    const expr = parse("(x+1)^2 - (x^2 + 2*x + 1)");
    const result = expr.simplify();
    return result;
}

// ============================================================================
// Trigonometric Simplification
// ============================================================================

function bench_simplify_trig_identity_direct() {
    const x = new JsSymbol("x");
    // sin^2(x) + cos^2(x) should simplify to 1
    const expr = JsExpression.add([
        JsExpression.pow(JsExpression.function("sin", [JsExpression.symbol(x)]), JsExpression.integer(2)),
        JsExpression.pow(JsExpression.function("cos", [JsExpression.symbol(x)]), JsExpression.integer(2))
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_trig_identity_with_parsing() {
    const expr = parse("sin(x)^2 + cos(x)^2");
    const result = expr.simplify();
    return result;
}

function bench_simplify_trig_double_angle_direct() {
    const x = new JsSymbol("x");
    // 2*sin(x)*cos(x) could simplify to sin(2x)
    const expr = JsExpression.multiply([
        JsExpression.integer(2),
        JsExpression.function("sin", [JsExpression.symbol(x)]),
        JsExpression.function("cos", [JsExpression.symbol(x)])
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_trig_double_angle_with_parsing() {
    const expr = parse("2*sin(x)*cos(x)");
    const result = expr.simplify();
    return result;
}

// ============================================================================
// Multivariate Simplification
// ============================================================================

function bench_simplify_multivariate_direct() {
    const x = new JsSymbol("x");
    const y = new JsSymbol("y");
    const expr = JsExpression.add([
        JsExpression.multiply([JsExpression.symbol(x), JsExpression.symbol(y)]),
        JsExpression.multiply([JsExpression.symbol(y), JsExpression.symbol(x)]),
        JsExpression.multiply([JsExpression.symbol(x), JsExpression.symbol(new JsSymbol("z"))]),
        JsExpression.multiply([JsExpression.symbol(new JsSymbol("z")), JsExpression.symbol(x)])
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_multivariate_with_parsing() {
    const expr = parse("x*y + y*x + x*z + z*x");
    const result = expr.simplify();
    return result;
}

// ============================================================================
// Rational Expression Simplification
// ============================================================================

function bench_simplify_rational_direct() {
    const x = new JsSymbol("x");
    // (x^2 - 1) / (x - 1) should simplify to (x + 1)
    const numerator = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(-1)
    ]);
    const denominator = JsExpression.add([
        JsExpression.symbol(x),
        JsExpression.integer(-1)
    ]);
    const expr = JsExpression.multiply([
        numerator,
        JsExpression.pow(denominator, JsExpression.integer(-1))
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_rational_with_parsing() {
    const expr = parse("(x^2 - 1) / (x - 1)");
    const result = expr.simplify();
    return result;
}

// ============================================================================
// Complex Simplification
// ============================================================================

function bench_simplify_complex_nested_direct() {
    const x = new JsSymbol("x");
    // ((x+1)*(x-1) + 1) / x should simplify to x
    const expr = JsExpression.multiply([
        JsExpression.add([
            JsExpression.multiply([
                JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]),
                JsExpression.add([JsExpression.symbol(x), JsExpression.integer(-1)])
            ]),
            JsExpression.integer(1)
        ]),
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(-1))
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplify_complex_nested_with_parsing() {
    const expr = parse("((x+1)*(x-1) + 1) / x");
    const result = expr.simplify();
    return result;
}

// ============================================================================
// Main Runner
// ============================================================================

function main() {
    const args = process.argv.slice(2);
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const jsonOutput = args.includes('--json');

    const benchmarks = {
        // Basic algebraic
        bench_simplify_like_terms_direct,
        bench_simplify_like_terms_with_parsing,
        bench_simplify_polynomial_direct,
        bench_simplify_polynomial_with_parsing,
        bench_simplify_difference_of_squares_direct,
        bench_simplify_difference_of_squares_with_parsing,

        // Trigonometric
        bench_simplify_trig_identity_direct,
        bench_simplify_trig_identity_with_parsing,
        bench_simplify_trig_double_angle_direct,
        bench_simplify_trig_double_angle_with_parsing,

        // Multivariate
        bench_simplify_multivariate_direct,
        bench_simplify_multivariate_with_parsing,

        // Rational expressions
        bench_simplify_rational_direct,
        bench_simplify_rational_with_parsing,

        // Complex nested
        bench_simplify_complex_nested_direct,
        bench_simplify_complex_nested_with_parsing
    };

    console.error("=".repeat(80));
    console.error("Simplification Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Basic Algebraic Simplification', ['bench_simplify_like_terms', 'bench_simplify_polynomial', 'bench_simplify_difference']],
            ['Trigonometric Simplification', ['bench_simplify_trig']],
            ['Multivariate Simplification', ['bench_simplify_multivariate']],
            ['Rational Expression Simplification', ['bench_simplify_rational']],
            ['Complex Nested Simplification', ['bench_simplify_complex']]
        ];

        printResults(results, categories);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
