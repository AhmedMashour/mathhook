#!/usr/bin/env node
/**
 * Core Performance Benchmarks
 * ============================
 *
 * Mirrors: benches/core_performance.rs, public/python/core_performance.py
 * Tests: Expression creation, simplification, basic solving, polynomial operations
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const { runBenchmarks, printResults, calculateParsingOverhead } = require('./benchmarkUtils');

// Load MathHook Node.js bindings
const mathhook = require(path.join(__dirname, '../../../mathhook-node'));
const { JsExpression, JsSymbol, JsMathSolver, parse } = mathhook;

// ============================================================================
// Arithmetic Operations Benchmarks
// ============================================================================

function bench_expression_creation_direct() {
    const x = new JsSymbol("x");
    const expr = JsExpression.add([JsExpression.symbol(x), JsExpression.integer(42)]);
    return expr;
}

function bench_expression_creation_with_parsing() {
    const expr = parse("x + 42");
    return expr;
}

function bench_simplification_direct() {
    const expr = JsExpression.add([
        JsExpression.integer(2),
        JsExpression.integer(3),
        JsExpression.integer(5)
    ]);
    const result = expr.simplify();
    return result;
}

function bench_simplification_with_parsing() {
    const expr = parse("2 + 3 + 5");
    const result = expr.simplify();
    return result;
}

// ============================================================================
// Solver Operations Benchmarks
// ============================================================================

function bench_basic_solving_direct() {
    const solver = new JsMathSolver();
    const x = new JsSymbol("x");
    const equation = JsExpression.equation(JsExpression.symbol(x), JsExpression.integer(42));
    const solutions = solver.solve(equation, x);
    return solutions;
}

function bench_basic_solving_with_parsing() {
    const solver = new JsMathSolver();
    const equation = parse("x = 42");
    const x = new JsSymbol("x");
    const solutions = solver.solve(equation, x);
    return solutions;
}

// ============================================================================
// Polynomial Operations Benchmarks
// ============================================================================

function bench_polynomial_creation_direct() {
    const x = new JsSymbol("x");
    // Create polynomial: x^10 + 2x^9 + ... + 10x + 11
    const terms = [];
    for (let i = 0; i < 11; i++) {
        terms.push(JsExpression.multiply([
            JsExpression.integer(i + 1),
            JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(10 - i))
        ]));
    }
    const poly = JsExpression.add(terms);
    return poly;
}

function bench_polynomial_creation_with_parsing() {
    const poly = parse("x^10 + 2*x^9 + 3*x^8 + 4*x^7 + 5*x^6 + 6*x^5 + 7*x^4 + 8*x^3 + 9*x^2 + 10*x + 11");
    return poly;
}

function bench_polynomial_simplification_direct() {
    const x = new JsSymbol("x");
    const poly = JsExpression.add([
        JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2)),
        JsExpression.multiply([JsExpression.integer(-5), JsExpression.symbol(x)]),
        JsExpression.integer(6)
    ]);
    const result = poly.simplify();
    return result;
}

function bench_polynomial_simplification_with_parsing() {
    const poly = parse("x^2 - 5*x + 6");
    const result = poly.simplify();
    return result;
}

// ============================================================================
// Memory Efficiency Benchmarks
// ============================================================================

function bench_expression_size_verification() {
    // JavaScript objects don't have direct size like Rust's 32-byte constraint,
    // but we can benchmark object creation overhead
    const x = new JsSymbol("x");
    const expr = JsExpression.pow(JsExpression.symbol(x), JsExpression.integer(2));
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
        // Arithmetic operations
        bench_expression_creation_direct,
        bench_expression_creation_with_parsing,
        bench_simplification_direct,
        bench_simplification_with_parsing,

        // Solver operations
        bench_basic_solving_direct,
        bench_basic_solving_with_parsing,

        // Polynomial operations
        bench_polynomial_creation_direct,
        bench_polynomial_creation_with_parsing,
        bench_polynomial_simplification_direct,
        bench_polynomial_simplification_with_parsing,

        // Memory efficiency
        bench_expression_size_verification
    };

    console.error("=".repeat(80));
    console.error("Core Performance Benchmarks");
    console.error("=".repeat(80));

    const results = runBenchmarks(benchmarks, samples);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        const categories = [
            ['Arithmetic Operations', ['bench_expression_creation', 'bench_simplification']],
            ['Solver Operations', ['bench_basic_solving']],
            ['Polynomial Operations', ['bench_polynomial']],
            ['Memory Efficiency', ['bench_expression_size']]
        ];

        printResults(results, categories);

        // Parsing overhead analysis
        console.log("\nParsing Overhead Analysis:");
        console.log("-".repeat(80));

        const overheadPairs = [
            ['expression_creation', 'bench_expression_creation_direct', 'bench_expression_creation_with_parsing'],
            ['simplification', 'bench_simplification_direct', 'bench_simplification_with_parsing'],
            ['solving', 'bench_basic_solving_direct', 'bench_basic_solving_with_parsing'],
            ['polynomial_creation', 'bench_polynomial_creation_direct', 'bench_polynomial_creation_with_parsing'],
            ['polynomial_simplification', 'bench_polynomial_simplification_direct', 'bench_polynomial_simplification_with_parsing']
        ];

        const overhead = calculateParsingOverhead(results, overheadPairs);

        Object.entries(overhead).forEach(([label, data]) => {
            console.log(`${label.padEnd(30)}: ${data.overhead_ns.toFixed(2).padStart(10)}ns (${data.overhead_pct.toFixed(2).padStart(6)}%)`);
        });
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
