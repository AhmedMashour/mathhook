#!/usr/bin/env node
/**
 * Run All Benchmarks
 * ===================
 *
 * Execute all Node.js benchmark modules and generate a comprehensive report.
 *
 * Usage:
 *     node runAll.js [--json] [--samples N] [--save FILEPATH]
 *
 * Options:
 *     --json          Output results as JSON
 *     --samples N     Number of samples per benchmark (default: 100)
 *     --save PATH     Save results to file
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
 * Main runner
 */
function main() {
    const args = process.argv.slice(2);
    const jsonOutput = args.includes('--json');
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const saveIdx = args.indexOf('--save');
    const savePath = saveIdx !== -1 ? args[saveIdx + 1] : null;

    console.error("=".repeat(80));
    console.error("MathHook Node.js Comprehensive Benchmark Suite");
    console.error("=".repeat(80));
    console.error(`Samples per benchmark: ${samples}`);
    console.error(`Node.js version: ${process.version}`);
    console.error();

    const allResults = {
        platform: 'nodejs-mathhook',
        node_version: process.version,
        timestamp: new Date().toISOString(),
        samples,
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
        console.error("-".repeat(60));

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
            allResults.suites[name] = JSON.parse(output);

        } catch (e) {
            console.error(`ERROR in ${name}: ${e.message}`);
            allResults.suites[name] = { error: e.message };
        }
    });

    // Output results
    if (jsonOutput) {
        console.log(JSON.stringify(allResults, null, 2));
    } else {
        printSummary(allResults);
    }

    // Save results if requested
    if (savePath) {
        saveResults(allResults, savePath);
        console.error(`\nResults saved to: ${savePath}`);
    }
}

/**
 * Print human-readable summary of all benchmark results
 */
function printSummary(allResults) {
    console.log("\n" + "=".repeat(80));
    console.log("Benchmark Suite Summary");
    console.log("=".repeat(80));
    console.log(`Node.js version: ${allResults.node_version}`);
    console.log(`Timestamp: ${allResults.timestamp}`);
    console.log(`Samples per benchmark: ${allResults.samples}`);

    Object.entries(allResults.suites).forEach(([suiteName, suiteData]) => {
        console.log(`\n${suiteName.toUpperCase()}:`);
        console.log("-".repeat(60));

        if (suiteData.error) {
            console.log(`  ERROR: ${suiteData.error}`);
            return;
        }

        if (!suiteData.benchmarks) {
            console.log("  No benchmark data");
            return;
        }

        // Calculate statistics
        const benchmarks = Object.values(suiteData.benchmarks).filter(b => !b.error);
        if (benchmarks.length === 0) {
            console.log("  No successful benchmarks");
            return;
        }

        const meanTimes = benchmarks.map(b => b.mean_ns);
        const totalMean = meanTimes.reduce((a, b) => a + b, 0) / meanTimes.length;
        const minTime = Math.min(...meanTimes);
        const maxTime = Math.max(...meanTimes);

        console.log(`  Total benchmarks: ${Object.keys(suiteData.benchmarks).length}`);
        console.log(`  Successful: ${benchmarks.length}`);
        console.log(`  Failed: ${Object.keys(suiteData.benchmarks).length - benchmarks.length}`);
        console.log(`  Average time: ${(totalMean / 1000).toFixed(2)} us`);
        console.log(`  Min time: ${(minTime / 1000).toFixed(2)} us`);
        console.log(`  Max time: ${(maxTime / 1000).toFixed(2)} us`);
    });

    // Overall statistics
    console.log("\n" + "=".repeat(80));
    console.log("Overall Statistics");
    console.log("=".repeat(80));

    let totalBenchmarks = 0;
    let totalSuccessful = 0;
    let totalFailed = 0;
    const allMeanTimes = [];

    Object.values(allResults.suites).forEach(suiteData => {
        if (suiteData.error || !suiteData.benchmarks) return;

        const benchmarkCount = Object.keys(suiteData.benchmarks).length;
        totalBenchmarks += benchmarkCount;

        Object.values(suiteData.benchmarks).forEach(benchmark => {
            if (benchmark.error) {
                totalFailed++;
            } else {
                totalSuccessful++;
                allMeanTimes.push(benchmark.mean_ns);
            }
        });
    });

    console.log(`Total benchmark count: ${totalBenchmarks}`);
    console.log(`Successful: ${totalSuccessful}`);
    console.log(`Failed: ${totalFailed}`);

    if (allMeanTimes.length > 0) {
        const overallMean = allMeanTimes.reduce((a, b) => a + b, 0) / allMeanTimes.length;
        const overallMin = Math.min(...allMeanTimes);
        const overallMax = Math.max(...allMeanTimes);

        console.log(`Overall average time: ${(overallMean / 1000).toFixed(2)} us`);
        console.log(`Overall min time: ${(overallMin / 1000).toFixed(2)} us`);
        console.log(`Overall max time: ${(overallMax / 1000).toFixed(2)} us`);
    }
}

if (require.main === module) {
    main();
}

module.exports = { main };
