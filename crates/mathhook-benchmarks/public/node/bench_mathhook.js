#!/usr/bin/env node
/**
 * MathHook Node.js Benchmark Suite
 *
 * Comprehensive polynomial operation benchmarks using MathHook Node.js bindings.
 * Public benchmarks with no external dependencies.
 *
 * Usage:
 *     node bench_mathhook.js [--json] [--iterations N]
 *
 * Output: JSON for baseline comparison or human-readable report.
 *
 * Last Updated: 2025-11-29T2000
 */

const path = require('path');
const fs = require('fs');

// Load MathHook Node.js bindings (search multiple paths)
let mathhook;
const possiblePaths = [
    path.join(__dirname, '../../../mathhook-node/index.js'),
    path.join(__dirname, '../../mathhook-node/index.js'),
    'mathhook-node'
];

for (const modulePath of possiblePaths) {
    try {
        mathhook = require(modulePath);
        break;
    } catch (e) {
        continue;
    }
}

if (!mathhook) {
    console.error('Error: MathHook Node.js bindings not found');
    console.error('Build with: cd crates/mathhook-node && npm install && npm run build');
    process.exit(1);
}

const { JsExpression, parse } = mathhook;

/**
 * Run benchmark with statistical analysis
 */
function benchmark(func, iterations = 100, warmup = 10) {
    // Warmup
    for (let i = 0; i < warmup; i++) {
        func();
    }

    // Collect timing samples (in nanoseconds)
    const times = [];
    for (let i = 0; i < iterations; i++) {
        const start = process.hrtime.bigint();
        func();
        const end = process.hrtime.bigint();
        times.push(Number(end - start));
    }

    // Statistics
    const sum = times.reduce((a, b) => a + b, 0);
    const mean_ns = sum / times.length;

    const variance = times.reduce((acc, t) => acc + Math.pow(t - mean_ns, 2), 0) / times.length;
    const stdev_ns = Math.sqrt(variance);

    times.sort((a, b) => a - b);
    const median_ns = times[Math.floor(times.length / 2)];
    const min_ns = times[0];
    const max_ns = times[times.length - 1];

    return {
        mean_ns,
        stdev_ns,
        median_ns,
        min_ns,
        max_ns,
        iterations
    };
}

// Parsing Benchmarks

function benchParseSimple() {
    return parse("x^2 + 2*x + 1");
}

function benchParseMedium() {
    return parse("x^5 + 3*x^4 - 2*x^3 + 7*x^2 - x + 5");
}

function benchParseLarge() {
    const terms = Array.from({length: 21}, (_, i) => `${i+1}*x^${i}`).join(" + ");
    return parse(terms);
}

function benchParseMultivariate() {
    return parse("x^2*y + x*y^2 + x*y + x + y + 1");
}

function benchParseLatex() {
    return parse("\\frac{x^2 + 1}{x - 1}");
}

// Polynomial Multiplication Benchmarks

function benchMulSimple() {
    const p1 = parse("x + 1");
    const p2 = parse("x + 1");
    return p1.multiply(p2);
}

function benchMulMedium() {
    const p1 = parse("x^10 + x^9 + x^8 + x^7 + x^6 + x^5 + x^4 + x^3 + x^2 + x + 1");
    const p2 = parse("2*x^10 + 2*x^9 + 2*x^8 + 2*x^7 + 2*x^6 + 2*x^5 + 2*x^4 + 2*x^3 + 2*x^2 + 2*x + 2");
    return p1.multiply(p2);
}

function benchMulLarge() {
    const terms = Array.from({length: 51}, (_, i) => `x^${i}`).join(" + ");
    const p1 = parse(terms);
    const p2 = parse(terms);
    return p1.multiply(p2);
}

function benchMulSparse() {
    const p1 = parse("1 + x^50 + x^100");
    const p2 = parse("1 + x^25 + x^75");
    return p1.multiply(p2);
}

// Polynomial Division Benchmarks

function benchDivSimple() {
    const p1 = parse("x^2 - 1");
    const p2 = parse("x - 1");
    return p1.divide(p2);
}

function benchDivMedium() {
    const p1 = parse("x^10 + x^5 + x^2 + 1");
    const p2 = parse("x^3 + x + 1");
    return p1.divide(p2);
}

// Expansion Benchmarks

function benchExpandSimple() {
    const p = parse("(x + 1)^2");
    return p.expand();
}

function benchExpandMedium() {
    const p = parse("(x + 1)^5");
    return p.expand();
}

function benchExpandLarge() {
    const p = parse("(x + 1)^10");
    return p.expand();
}

// Simplification Benchmarks

function benchSimplifySimple() {
    const p = parse("x + x + x");
    return p.simplify();
}

function benchSimplifyPolynomial() {
    const p = parse("x^2 + 2*x + 1 + x^2 - 1");
    return p.simplify();
}

function benchSimplifyLarge() {
    const p = parse("(x+1)^2 - (x^2 + 2*x + 1)");
    return p.simplify();
}

// Factorization Benchmarks

function benchFactorSimple() {
    const p = parse("x^2 - 1");
    return p.factor();
}

function benchFactorQuadratic() {
    const p = parse("x^2 + 2*x + 1");
    return p.factor();
}

// Main Benchmark Runner

function runAllBenchmarks(iterations = 100) {
    const benchmarks = {
        // Parsing
        parse_simple: benchParseSimple,
        parse_medium: benchParseMedium,
        parse_large: benchParseLarge,
        parse_multivariate: benchParseMultivariate,
        parse_latex: benchParseLatex,

        // Multiplication
        mul_simple: benchMulSimple,
        mul_medium: benchMulMedium,
        mul_large: benchMulLarge,
        mul_sparse: benchMulSparse,

        // Division
        div_simple: benchDivSimple,
        div_medium: benchDivMedium,

        // Expansion
        expand_simple: benchExpandSimple,
        expand_medium: benchExpandMedium,
        expand_large: benchExpandLarge,

        // Simplification
        simplify_simple: benchSimplifySimple,
        simplify_polynomial: benchSimplifyPolynomial,
        simplify_large: benchSimplifyLarge,

        // Factorization
        factor_simple: benchFactorSimple,
        factor_quadratic: benchFactorQuadratic,
    };

    const results = {
        platform: 'node-mathhook',
        binding: 'NAPI',
        node_version: process.version,
        benchmarks: {}
    };

    for (const [name, func] of Object.entries(benchmarks)) {
        try {
            process.stderr.write(`Running ${name}...\n`);
            const result = benchmark(func, iterations);
            results.benchmarks[name] = result;
        } catch (e) {
            process.stderr.write(`Error in ${name}: ${e.message}\n`);
            results.benchmarks[name] = { error: e.message };
        }
    }

    return results;
}

function printHumanReadable(results) {
    console.log('='.repeat(80));
    console.log('MathHook Node.js Benchmark Results');
    console.log('='.repeat(80));
    console.log(`Binding: ${results.binding}`);
    console.log(`Node version: ${results.node_version}`);
    console.log();

    const categories = {
        'Parsing': ['parse_'],
        'Multiplication': ['mul_'],
        'Division': ['div_'],
        'Expansion': ['expand_'],
        'Simplification': ['simplify_'],
        'Factorization': ['factor_'],
    };

    for (const [category, prefixes] of Object.entries(categories)) {
        console.log(`\n${category}:`);
        console.log('-'.repeat(60));

        for (const [name, data] of Object.entries(results.benchmarks)) {
            if (prefixes.some(prefix => name.startsWith(prefix))) {
                if (data.error) {
                    console.log(`  ${name.padEnd(30)} ERROR: ${data.error}`);
                } else {
                    const mean_us = data.mean_ns / 1000;
                    const stdev_us = data.stdev_ns / 1000;
                    console.log(`  ${name.padEnd(30)} ${mean_us.toFixed(2).padStart(12)} us  (stdev: ${stdev_us.toFixed(2).padStart(8)} us)`);
                }
            }
        }
    }
}

// CLI Handling

function main() {
    const args = process.argv.slice(2);
    let jsonOutput = false;
    let iterations = 100;

    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--json') {
            jsonOutput = true;
        } else if (args[i] === '--iterations' && i + 1 < args.length) {
            iterations = parseInt(args[i + 1], 10);
            i++;
        }
    }

    const results = runAllBenchmarks(iterations);

    if (jsonOutput) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        printHumanReadable(results);
    }
}

if (require.main === module) {
    main();
}

module.exports = { runAllBenchmarks, benchmark };
