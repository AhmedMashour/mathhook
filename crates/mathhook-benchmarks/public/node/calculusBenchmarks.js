#!/usr/bin/env node
/**
 * Calculus Benchmarks
 * ===================
 *
 * Mirrors: benches/calculus_benchmarks.rs, public/python/calculus_benchmarks.py
 * Tests: Derivatives (power, product, chain, quotient rules), Integrals, Multi-variable
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { JsExpression, JsSymbol, parse } = mathhook;

// ============================================================================
// Derivative Benchmarks
// ============================================================================

function bench_derivative_power_rule_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(5));
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_power_rule_with_parsing() {
    const expr = parse("x^5");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_product_rule_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.multiply([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.function("sin", [JsExpression.symbol(x)])
    ]);
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_product_rule_with_parsing() {
    const expr = parse("x^2 * sin(x)");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_chain_rule_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("sin", [
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2))
    ]);
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_chain_rule_with_parsing() {
    const expr = parse("sin(x^2)");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_quotient_rule_direct() {
    const x = new JsSymbol("x");
    const numerator = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(1)
    ]);
    const denominator = JsExpression.add([
        JsExpression.symbol(x),
        JsExpression.integer(-1)
    ]);
    const expr = JsExpression.multiply([
        numerator,
        JsExpression.pow(denominator, JsExpression.integer(-1))
    ]);
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_quotient_rule_with_parsing() {
    const expr = parse("(x^2 + 1) / (x - 1)");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_trigonometric_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.add([
        JsExpression.function("sin", [JsExpression.symbol(x)]),
        JsExpression.function("cos", [JsExpression.symbol(x)])
    ]);
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_trigonometric_with_parsing() {
    const expr = parse("sin(x) + cos(x)");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_exponential_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("exp", [
        JsExpression.multiply([
            JsExpression.integer(2),
            JsExpression.symbol(x)
        ])
    ]);
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_exponential_with_parsing() {
    const expr = parse("exp(2*x)");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_logarithmic_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("log", [JsExpression.symbol(x)]);
    const result = expr.derivative(x, 1);
    return result;
}

function bench_derivative_logarithmic_with_parsing() {
    const expr = parse("log(x)");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 1);
    return result;
}

// Higher order derivatives
function bench_derivative_higher_order_2nd_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(10));
    const result = expr.derivative(x, 2);
    return result;
}

function bench_derivative_higher_order_2nd_with_parsing() {
    const expr = parse("x^10");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 2);
    return result;
}

function bench_derivative_higher_order_3rd_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(10));
    const result = expr.derivative(x, 3);
    return result;
}

function bench_derivative_higher_order_3rd_with_parsing() {
    const expr = parse("x^10");
    const x = new JsSymbol("x");
    const result = expr.derivative(x, 3);
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
        // Power rule
        bench_derivative_power_rule_direct,
        bench_derivative_power_rule_with_parsing,

        // Product rule
        bench_derivative_product_rule_direct,
        bench_derivative_product_rule_with_parsing,

        // Chain rule
        bench_derivative_chain_rule_direct,
        bench_derivative_chain_rule_with_parsing,

        // Quotient rule
        bench_derivative_quotient_rule_direct,
        bench_derivative_quotient_rule_with_parsing,

        // Trigonometric
        bench_derivative_trigonometric_direct,
        bench_derivative_trigonometric_with_parsing,

        // Exponential
        bench_derivative_exponential_direct,
        bench_derivative_exponential_with_parsing,

        // Logarithmic
        bench_derivative_logarithmic_direct,
        bench_derivative_logarithmic_with_parsing,

        // Higher order
        bench_derivative_higher_order_2nd_direct,
        bench_derivative_higher_order_2nd_with_parsing,
        bench_derivative_higher_order_3rd_direct,
        bench_derivative_higher_order_3rd_with_parsing
    };

    console.error("=".repeat(80));
    console.error("Calculus Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Power Rule Derivatives', ['bench_derivative_power_rule']],
            ['Product Rule Derivatives', ['bench_derivative_product_rule']],
            ['Chain Rule Derivatives', ['bench_derivative_chain_rule']],
            ['Quotient Rule Derivatives', ['bench_derivative_quotient_rule']],
            ['Trigonometric Derivatives', ['bench_derivative_trigonometric']],
            ['Exponential Derivatives', ['bench_derivative_exponential']],
            ['Logarithmic Derivatives', ['bench_derivative_logarithmic']],
            ['Higher Order Derivatives', ['bench_derivative_higher_order']]
        ];

        printResults(results, categories);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
