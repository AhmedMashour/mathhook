#!/usr/bin/env node
/**
 * Polynomial Benchmarks
 * =====================
 *
 * Mirrors: benches/polynomial_benchmarks.rs
 * Tests: Polynomial creation, multiplication, division, expansion, factorization
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { JsExpression, JsSymbol, parse } = mathhook;

// ============================================================================
// Polynomial Creation
// ============================================================================

function bench_poly_create_simple_direct() {
    const x = new JsSymbol("x");
    // x^2 + 2x + 1
    const poly = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.multiply([JsExpression.integer(2), JsExpression.symbol(x)]),
        JsExpression.integer(1)
    ]);
    return poly;
}

function bench_poly_create_simple_with_parsing() {
    const poly = parse("x^2 + 2*x + 1");
    return poly;
}

function bench_poly_create_medium_direct() {
    const x = new JsSymbol("x");
    // x^5 + 3x^4 - 2x^3 + 7x^2 - x + 5
    const poly = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(5)),
        JsExpression.multiply([JsExpression.integer(3), JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(4))]),
        JsExpression.multiply([JsExpression.integer(-2), JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(3))]),
        JsExpression.multiply([JsExpression.integer(7), JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2))]),
        JsExpression.multiply([JsExpression.integer(-1), JsExpression.symbol(x)]),
        JsExpression.integer(5)
    ]);
    return poly;
}

function bench_poly_create_medium_with_parsing() {
    const poly = parse("x^5 + 3*x^4 - 2*x^3 + 7*x^2 - x + 5");
    return poly;
}

function bench_poly_create_large_direct() {
    const x = new JsSymbol("x");
    // Create polynomial with 21 terms
    const terms = [];
    for (let i = 0; i <= 20; i++) {
        terms.push(JsExpression.multiply([
            JsExpression.integer(i + 1),
            JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(i))
        ]));
    }
    const poly = JsExpression.add(terms);
    return poly;
}

function bench_poly_create_large_with_parsing() {
    const terms = Array.from({length: 21}, (_, i) => `${i+1}*x^${i}`).join(" + ");
    const poly = parse(terms);
    return poly;
}

// ============================================================================
// Polynomial Multiplication
// ============================================================================

function bench_poly_mul_simple_direct() {
    const x = new JsSymbol("x");
    const p1 = JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]);
    const p2 = JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]);
    const result = JsExpression.multiply([p1, p2]);
    return result;
}

function bench_poly_mul_simple_with_parsing() {
    const p1 = parse("x + 1");
    const p2 = parse("x + 1");
    const result = JsExpression.multiply([p1, p2]);
    return result;
}

function bench_poly_mul_medium_direct() {
    const x = new JsSymbol("x");
    const terms1 = [];
    const terms2 = [];
    for (let i = 0; i <= 10; i++) {
        terms1.push(JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(i)));
        terms2.push(JsExpression.multiply([
            JsExpression.integer(2),
            JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(i))
        ]));
    }
    const p1 = JsExpression.add(terms1);
    const p2 = JsExpression.add(terms2);
    const result = JsExpression.multiply([p1, p2]);
    return result;
}

function bench_poly_mul_medium_with_parsing() {
    const p1 = parse("x^10 + x^9 + x^8 + x^7 + x^6 + x^5 + x^4 + x^3 + x^2 + x + 1");
    const p2 = parse("2*x^10 + 2*x^9 + 2*x^8 + 2*x^7 + 2*x^6 + 2*x^5 + 2*x^4 + 2*x^3 + 2*x^2 + 2*x + 2");
    const result = JsExpression.multiply([p1, p2]);
    return result;
}

// ============================================================================
// Polynomial Division
// ============================================================================

function bench_poly_div_simple_direct() {
    const x = new JsSymbol("x");
    // (x^2 - 1) / (x - 1)
    const numerator = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(-1)
    ]);
    const denominator = JsExpression.add([
        JsExpression.symbol(x),
        JsExpression.integer(-1)
    ]);
    const result = JsExpression.multiply([
        numerator,
        JsExpression.pow(denominator, JsExpression.integer(-1))
    ]);
    return result;
}

function bench_poly_div_simple_with_parsing() {
    const result = parse("(x^2 - 1) / (x - 1)");
    return result;
}

// ============================================================================
// Polynomial Expansion
// ============================================================================

function bench_poly_expand_simple_direct() {
    const x = new JsSymbol("x");
    // (x + 1)^2
    const base = JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]);
    const expr = JsExpression.pow(base, JsExpression.integer(2));
    const result = expr.expand ? expr.expand() : expr;
    return result;
}

function bench_poly_expand_simple_with_parsing() {
    const expr = parse("(x + 1)^2");
    const result = expr.expand ? expr.expand() : expr;
    return result;
}

function bench_poly_expand_medium_direct() {
    const x = new JsSymbol("x");
    // (x + 1)^5
    const base = JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]);
    const expr = JsExpression.pow(base, JsExpression.integer(5));
    const result = expr.expand ? expr.expand() : expr;
    return result;
}

function bench_poly_expand_medium_with_parsing() {
    const expr = parse("(x + 1)^5");
    const result = expr.expand ? expr.expand() : expr;
    return result;
}

function bench_poly_expand_large_direct() {
    const x = new JsSymbol("x");
    // (x + 1)^10
    const base = JsExpression.add([JsExpression.symbol(x), JsExpression.integer(1)]);
    const expr = JsExpression.pow(base, JsExpression.integer(10));
    const result = expr.expand ? expr.expand() : expr;
    return result;
}

function bench_poly_expand_large_with_parsing() {
    const expr = parse("(x + 1)^10");
    const result = expr.expand ? expr.expand() : expr;
    return result;
}

// ============================================================================
// Polynomial Factorization
// ============================================================================

function bench_poly_factor_simple_direct() {
    const x = new JsSymbol("x");
    // x^2 - 1 = (x-1)(x+1)
    const expr = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(-1)
    ]);
    const result = expr.factor ? expr.factor() : expr;
    return result;
}

function bench_poly_factor_simple_with_parsing() {
    const expr = parse("x^2 - 1");
    const result = expr.factor ? expr.factor() : expr;
    return result;
}

function bench_poly_factor_quadratic_direct() {
    const x = new JsSymbol("x");
    // x^2 + 2x + 1 = (x+1)^2
    const expr = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.multiply([JsExpression.integer(2), JsExpression.symbol(x)]),
        JsExpression.integer(1)
    ]);
    const result = expr.factor ? expr.factor() : expr;
    return result;
}

function bench_poly_factor_quadratic_with_parsing() {
    const expr = parse("x^2 + 2*x + 1");
    const result = expr.factor ? expr.factor() : expr;
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
        // Polynomial creation
        bench_poly_create_simple_direct,
        bench_poly_create_simple_with_parsing,
        bench_poly_create_medium_direct,
        bench_poly_create_medium_with_parsing,
        bench_poly_create_large_direct,
        bench_poly_create_large_with_parsing,

        // Polynomial multiplication
        bench_poly_mul_simple_direct,
        bench_poly_mul_simple_with_parsing,
        bench_poly_mul_medium_direct,
        bench_poly_mul_medium_with_parsing,

        // Polynomial division
        bench_poly_div_simple_direct,
        bench_poly_div_simple_with_parsing,

        // Polynomial expansion
        bench_poly_expand_simple_direct,
        bench_poly_expand_simple_with_parsing,
        bench_poly_expand_medium_direct,
        bench_poly_expand_medium_with_parsing,
        bench_poly_expand_large_direct,
        bench_poly_expand_large_with_parsing,

        // Polynomial factorization
        bench_poly_factor_simple_direct,
        bench_poly_factor_simple_with_parsing,
        bench_poly_factor_quadratic_direct,
        bench_poly_factor_quadratic_with_parsing
    };

    console.error("=".repeat(80));
    console.error("Polynomial Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Polynomial Creation', ['bench_poly_create']],
            ['Polynomial Multiplication', ['bench_poly_mul']],
            ['Polynomial Division', ['bench_poly_div']],
            ['Polynomial Expansion', ['bench_poly_expand']],
            ['Polynomial Factorization', ['bench_poly_factor']]
        ];

        printResults(results, categories);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
