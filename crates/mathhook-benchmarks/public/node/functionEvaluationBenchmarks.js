#!/usr/bin/env node
/**
 * Function Evaluation Benchmarks
 * ===============================
 *
 * Mirrors: benches/function_evaluation_benchmarks.rs
 * Tests: Elementary functions (trig, exp, log), special functions, numerical evaluation
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { JsExpression, JsSymbol, parse } = mathhook;

// ============================================================================
// Trigonometric Functions
// ============================================================================

function bench_eval_sin_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("sin", [JsExpression.symbol(x)]);
    // Evaluate would need a value for x, so we'll just create the function
    return expr;
}

function bench_eval_sin_with_parsing() {
    const expr = parse("sin(x)");
    return expr;
}

function bench_eval_cos_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("cos", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_cos_with_parsing() {
    const expr = parse("cos(x)");
    return expr;
}

function bench_eval_tan_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("tan", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_tan_with_parsing() {
    const expr = parse("tan(x)");
    return expr;
}

// ============================================================================
// Exponential and Logarithmic Functions
// ============================================================================

function bench_eval_exp_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("exp", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_exp_with_parsing() {
    const expr = parse("exp(x)");
    return expr;
}

function bench_eval_log_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("log", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_log_with_parsing() {
    const expr = parse("log(x)");
    return expr;
}

function bench_eval_ln_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("ln", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_ln_with_parsing() {
    const expr = parse("ln(x)");
    return expr;
}

// ============================================================================
// Power and Root Functions
// ============================================================================

function bench_eval_sqrt_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("sqrt", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_sqrt_with_parsing() {
    const expr = parse("sqrt(x)");
    return expr;
}

function bench_eval_power_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(10));
    return expr;
}

function bench_eval_power_with_parsing() {
    const expr = parse("x^10");
    return expr;
}

// ============================================================================
// Inverse Trigonometric Functions
// ============================================================================

function bench_eval_arcsin_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("arcsin", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_arcsin_with_parsing() {
    const expr = parse("arcsin(x)");
    return expr;
}

function bench_eval_arccos_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("arccos", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_arccos_with_parsing() {
    const expr = parse("arccos(x)");
    return expr;
}

function bench_eval_arctan_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("arctan", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_arctan_with_parsing() {
    const expr = parse("arctan(x)");
    return expr;
}

// ============================================================================
// Hyperbolic Functions
// ============================================================================

function bench_eval_sinh_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("sinh", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_sinh_with_parsing() {
    const expr = parse("sinh(x)");
    return expr;
}

function bench_eval_cosh_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("cosh", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_cosh_with_parsing() {
    const expr = parse("cosh(x)");
    return expr;
}

function bench_eval_tanh_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.function("tanh", [JsExpression.symbol(x)]);
    return expr;
}

function bench_eval_tanh_with_parsing() {
    const expr = parse("tanh(x)");
    return expr;
}

// ============================================================================
// Main Runner
// ============================================================================

function main() {
    const args = process.argv.slice(2);
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const jsonOutput = args.includes('--json');

    const benchmarks = {
        // Trigonometric
        bench_eval_sin_direct,
        bench_eval_sin_with_parsing,
        bench_eval_cos_direct,
        bench_eval_cos_with_parsing,
        bench_eval_tan_direct,
        bench_eval_tan_with_parsing,

        // Exponential and logarithmic
        bench_eval_exp_direct,
        bench_eval_exp_with_parsing,
        bench_eval_log_direct,
        bench_eval_log_with_parsing,
        bench_eval_ln_direct,
        bench_eval_ln_with_parsing,

        // Power and root
        bench_eval_sqrt_direct,
        bench_eval_sqrt_with_parsing,
        bench_eval_power_direct,
        bench_eval_power_with_parsing,

        // Inverse trigonometric
        bench_eval_arcsin_direct,
        bench_eval_arcsin_with_parsing,
        bench_eval_arccos_direct,
        bench_eval_arccos_with_parsing,
        bench_eval_arctan_direct,
        bench_eval_arctan_with_parsing,

        // Hyperbolic
        bench_eval_sinh_direct,
        bench_eval_sinh_with_parsing,
        bench_eval_cosh_direct,
        bench_eval_cosh_with_parsing,
        bench_eval_tanh_direct,
        bench_eval_tanh_with_parsing
    };

    console.error("=".repeat(80));
    console.error("Function Evaluation Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Trigonometric Functions', ['bench_eval_sin', 'bench_eval_cos', 'bench_eval_tan']],
            ['Exponential and Logarithmic', ['bench_eval_exp', 'bench_eval_log', 'bench_eval_ln']],
            ['Power and Root Functions', ['bench_eval_sqrt', 'bench_eval_power']],
            ['Inverse Trigonometric', ['bench_eval_arcsin', 'bench_eval_arccos', 'bench_eval_arctan']],
            ['Hyperbolic Functions', ['bench_eval_sinh', 'bench_eval_cosh', 'bench_eval_tanh']]
        ];

        printResults(results, categories);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
