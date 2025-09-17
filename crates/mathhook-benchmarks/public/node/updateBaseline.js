#!/usr/bin/env node
/**
 * Update Baseline Benchmarks
 * ===========================
 *
 * Run all benchmarks and save results as a new baseline for future comparisons.
 *
 * Usage:
 *     node updateBaseline.js [--samples N] [--output FILEPATH]
 *
 * Options:
 *     --samples N     Number of samples per benchmark (default: 100)
 *     --output PATH   Baseline file path (default: ./baseline_node.json)
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const fs = require('fs');
const { saveResults } = require('./benchmarkUtils');

// Benchmark modules
const corePerformance = require('./corePerformance');
const calculusBenchmarks = require('./calculusBenchmarks');
const solvingBenchmarks = require('./solvingBenchmarks');
const simplificationBenchmarks = require('./simplificationBenchmarks');
const functionEvaluationBenchmarks = require('./functionEvaluationBenchmarks');
const polynomialBenchmarks = require('./polynomialBenchmarks');
const parsingBenchmarks = require('./parsingBenchmarks');

/**
 * Main function to update baseline
 */
function main() {
    const args = process.argv.slice(2);
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const outputIdx = args.indexOf('--output');
    const outputPath = outputIdx !== -1 ? args[outputIdx + 1] : './baseline_node.json';

    console.error("=".repeat(80));
    console.error("Updating Baseline Benchmarks");
    console.error("=".repeat(80));
    console.error(`Samples per benchmark: ${samples}`);
    console.error(`Output file: ${outputPath}`);
    console.error(`Node.js version: ${process.version}`);
    console.error();

    const baselineResults = {
        platform: 'nodejs-mathhook',
        node_version: process.version,
        timestamp: new Date().toISOString(),
        samples,
        description: 'Baseline benchmark results for MathHook Node.js bindings',
        suites: {}
    };

    // Run all benchmark suites
    const suites = [
        { name: 'core_performance', module: corePerformance },
        { name: 'calculus', module: calculusBenchmarks },
        { name: 'solving', module: solvingBenchmarks },
        { name: 'simplification', module: simplificationBenchmarks },
        { name: 'function_evaluation', module: functionEvaluationBenchmarks },
        { name: 'polynomial', module: polynomialBenchmarks },
        { name: 'parsing', module: parsingBenchmarks }
    ];

    suites.forEach(({ name, module }) => {
        console.error(`\nRunning ${name} benchmarks...`);

        try {
            // Run the module's main function
            const originalArgv = process.argv;
            process.argv = ['node', 'benchmark', '--json', '--samples', samples.toString()];

            // Capture stdout
            let output = '';
            const originalStdoutWrite = process.stdout.write;
            process.stdout.write = (chunk) => {
                output += chunk;
                return true;
            };

            module.main();

            // Restore stdout
            process.stdout.write = originalStdoutWrite;
            process.argv = originalArgv;

            // Parse results
            baselineResults.suites[name] = JSON.parse(output);
            console.error(`  ✓ Completed ${Object.keys(baselineResults.suites[name].benchmarks).length} benchmarks`);

        } catch (e) {
            console.error(`  ✗ ERROR: ${e.message}`);
            baselineResults.suites[name] = { error: e.message };
        }
    });

    // Save baseline
    try {
        saveResults(baselineResults, outputPath);
        console.error(`\n${"=".repeat(80)}`);
        console.error(`Baseline saved successfully to: ${outputPath}`);
        console.error(`${"=".repeat(80)}`);

        // Print summary
        printSummary(baselineResults);

        // Print usage instructions
        console.error(`\nTo compare future benchmarks against this baseline:`);
        console.error(`  node compareBaseline.js --baseline ${outputPath} [--current RESULTS.json]`);

    } catch (e) {
        console.error(`\nERROR saving baseline: ${e.message}`);
        process.exit(1);
    }
}

/**
 * Print summary of baseline results
 */
function printSummary(baselineResults) {
    console.error(`\nBaseline Summary:`);
    console.error("-".repeat(60));

    let totalBenchmarks = 0;
    let totalSuccessful = 0;

    Object.entries(baselineResults.suites).forEach(([suiteName, suiteData]) => {
        if (suiteData.error || !suiteData.benchmarks) {
            console.error(`  ${suiteName}: ERROR`);
            return;
        }

        const benchmarkCount = Object.keys(suiteData.benchmarks).length;
        const successful = Object.values(suiteData.benchmarks).filter(b => !b.error).length;

        totalBenchmarks += benchmarkCount;
        totalSuccessful += successful;

        console.error(`  ${suiteName}: ${successful}/${benchmarkCount} successful`);
    });

    console.error("-".repeat(60));
    console.error(`  Total: ${totalSuccessful}/${totalBenchmarks} benchmarks successful`);
}

if (require.main === module) {
    main();
}

module.exports = { main };
