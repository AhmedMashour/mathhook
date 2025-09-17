#!/usr/bin/env node
/**
 * Solving Benchmarks
 * ==================
 *
 * Mirrors: benches/solving_benchmarks.rs
 * Tests: Linear, quadratic, polynomial, system, and matrix equation solving
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { JsExpression, JsSymbol, JsMathSolver, parse } = mathhook;

// ============================================================================
// Linear Equation Solving
// ============================================================================

function bench_linear_simple_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // 2x + 3 = 0
    const eq = JsExpression.add([
        JsExpression.multiply([JsExpression.integer(2), JsExpression.symbol(x)]),
        JsExpression.integer(3)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_linear_simple_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("2*x + 3 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_linear_large_coefficient_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // 1000x + 500 = 0
    const eq = JsExpression.add([
        JsExpression.multiply([JsExpression.integer(1000), JsExpression.symbol(x)]),
        JsExpression.integer(500)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_linear_large_coefficient_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("1000*x + 500 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

// ============================================================================
// Quadratic Equation Solving
// ============================================================================

function bench_quadratic_real_roots_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // x^2 - 4 = 0 (real roots: ±2)
    const eq = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(-4)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_quadratic_real_roots_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("x^2 - 4 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_quadratic_complex_roots_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // x^2 + 1 = 0 (complex roots: ±i)
    const eq = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.integer(1)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_quadratic_complex_roots_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("x^2 + 1 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_quadratic_general_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // x^2 - 5x + 6 = 0 (roots: 2, 3)
    const eq = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.multiply([JsExpression.integer(-5), JsExpression.symbol(x)]),
        JsExpression.integer(6)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_quadratic_general_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("x^2 - 5*x + 6 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

// ============================================================================
// Polynomial Solving
// ============================================================================

function bench_polynomial_cubic_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // x^3 - 6x^2 + 11x - 6 = 0 (roots: 1, 2, 3)
    const eq = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(3)),
        JsExpression.multiply([JsExpression.integer(-6), JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2))]),
        JsExpression.multiply([JsExpression.integer(11), JsExpression.symbol(x)]),
        JsExpression.integer(-6)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_polynomial_cubic_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("x^3 - 6*x^2 + 11*x - 6 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_polynomial_quartic_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    // x^4 - 5x^2 + 4 = 0 (roots: ±1, ±2)
    const eq = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(4)),
        JsExpression.multiply([JsExpression.integer(-5), JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2))]),
        JsExpression.integer(4)
    ]);
    const solutions = solver.solve(eq, x);
    return solutions;
}

function bench_polynomial_quartic_with_parsing() {
    const solver = new JsMathSolver();
    const eq = parse("x^4 - 5*x^2 + 4 = 0");
    const x = new JsSymbol("x");
    const solutions = solver.solve(eq, x);
    return solutions;
}

// ============================================================================
// Main Runner
// ============================================================================

function main() {
    const args = process.argv.slice(2);
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const jsonOutput = args.includes('--json');

    const benchmarks = {
        // Linear solving
        bench_linear_simple_direct,
        bench_linear_simple_with_parsing,
        bench_linear_large_coefficient_direct,
        bench_linear_large_coefficient_with_parsing,

        // Quadratic solving
        bench_quadratic_real_roots_direct,
        bench_quadratic_real_roots_with_parsing,
        bench_quadratic_complex_roots_direct,
        bench_quadratic_complex_roots_with_parsing,
        bench_quadratic_general_direct,
        bench_quadratic_general_with_parsing,

        // Polynomial solving
        bench_polynomial_cubic_direct,
        bench_polynomial_cubic_with_parsing,
        bench_polynomial_quartic_direct,
        bench_polynomial_quartic_with_parsing
    };

    console.error("=".repeat(80));
    console.error("Solving Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Linear Equation Solving', ['bench_linear']],
            ['Quadratic Equation Solving', ['bench_quadratic']],
            ['Polynomial Equation Solving', ['bench_polynomial']]
        ];

        printResults(results, categories);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
