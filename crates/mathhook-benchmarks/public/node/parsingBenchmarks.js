#!/usr/bin/env node
/**
 * Parsing Benchmarks
 * ==================
 *
 * Mirrors: benches/parsing_benchmarks.rs
 * Tests: Standard notation, LaTeX parsing, complex expressions, multivariate
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { parse } = mathhook;

// ============================================================================
// Standard Notation Parsing
// ============================================================================

function bench_parse_simple() {
    return parse("x^2 + 2*x + 1");
}

function bench_parse_medium() {
    return parse("x^5 + 3*x^4 - 2*x^3 + 7*x^2 - x + 5");
}

function bench_parse_large() {
    const terms = Array.from({length: 21}, (_, i) => `${i+1}*x^${i}`).join(" + ");
    return parse(terms);
}

function bench_parse_multivariate() {
    return parse("x^2*y + x*y^2 + x*y + x + y + 1");
}

function bench_parse_nested() {
    return parse("sin(cos(tan(x)))");
}

function bench_parse_complex_expression() {
    return parse("(x^2 + y^2) / (x*y + 1) - sin(x*y) + exp(x+y)");
}

// ============================================================================
// LaTeX Parsing
// ============================================================================

function bench_parse_latex_simple() {
    return parse("\\frac{x^2 + 1}{x - 1}");
}

function bench_parse_latex_fraction() {
    return parse("\\frac{x^2}{y^2} + \\frac{a}{b}");
}

function bench_parse_latex_sqrt() {
    return parse("\\sqrt{x^2 + 1}");
}

function bench_parse_latex_trig() {
    return parse("\\sin(x) + \\cos(y) + \\tan(z)");
}

function bench_parse_latex_integral() {
    return parse("\\int x^2 dx");
}

function bench_parse_latex_sum() {
    return parse("\\sum_{i=1}^{n} i");
}

function bench_parse_latex_complex() {
    return parse("\\frac{\\sin(x)}{\\cos(x)} + \\sqrt{x^2 + y^2}");
}

// ============================================================================
// Expression with Functions
// ============================================================================

function bench_parse_trig_functions() {
    return parse("sin(x) + cos(x) + tan(x)");
}

function bench_parse_exp_log_functions() {
    return parse("exp(x) + log(x) + ln(x)");
}

function bench_parse_inverse_trig() {
    return parse("arcsin(x) + arccos(x) + arctan(x)");
}

function bench_parse_hyperbolic() {
    return parse("sinh(x) + cosh(x) + tanh(x)");
}

// ============================================================================
// Arithmetic Operations
// ============================================================================

function bench_parse_addition() {
    return parse("x + y + z + w + a + b + c");
}

function bench_parse_multiplication() {
    return parse("x * y * z * w * a * b * c");
}

function bench_parse_division() {
    return parse("x / y / z");
}

function bench_parse_power() {
    return parse("x^2^3");
}

function bench_parse_mixed_operations() {
    return parse("x^2 + 2*x*y - y^2 / z + w");
}

// ============================================================================
// Parentheses and Precedence
// ============================================================================

function bench_parse_parentheses_simple() {
    return parse("(x + y) * (x - y)");
}

function bench_parse_parentheses_nested() {
    return parse("((x + y) * (x - y)) / (x^2 + y^2)");
}

function bench_parse_precedence_test() {
    return parse("2 + 3 * 4^5");
}

// ============================================================================
// Main Runner
// ============================================================================

function main() {
    const args = process.argv.slice(2);
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const jsonOutput = args.includes('--json');

    const benchmarks = {
        // Standard notation
        bench_parse_simple,
        bench_parse_medium,
        bench_parse_large,
        bench_parse_multivariate,
        bench_parse_nested,
        bench_parse_complex_expression,

        // LaTeX parsing
        bench_parse_latex_simple,
        bench_parse_latex_fraction,
        bench_parse_latex_sqrt,
        bench_parse_latex_trig,
        bench_parse_latex_integral,
        bench_parse_latex_sum,
        bench_parse_latex_complex,

        // Functions
        bench_parse_trig_functions,
        bench_parse_exp_log_functions,
        bench_parse_inverse_trig,
        bench_parse_hyperbolic,

        // Arithmetic operations
        bench_parse_addition,
        bench_parse_multiplication,
        bench_parse_division,
        bench_parse_power,
        bench_parse_mixed_operations,

        // Parentheses and precedence
        bench_parse_parentheses_simple,
        bench_parse_parentheses_nested,
        bench_parse_precedence_test
    };

    console.error("=".repeat(80));
    console.error("Parsing Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Standard Notation', ['bench_parse_simple', 'bench_parse_medium', 'bench_parse_large', 'bench_parse_multivariate', 'bench_parse_nested', 'bench_parse_complex']],
            ['LaTeX Parsing', ['bench_parse_latex']],
            ['Function Parsing', ['bench_parse_trig', 'bench_parse_exp', 'bench_parse_inverse', 'bench_parse_hyperbolic']],
            ['Arithmetic Operations', ['bench_parse_addition', 'bench_parse_multiplication', 'bench_parse_division', 'bench_parse_power', 'bench_parse_mixed']],
            ['Parentheses and Precedence', ['bench_parse_parentheses', 'bench_parse_precedence']]
        ];

        printResults(results, categories);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
