#!/usr/bin/env node
/**
 * Compare Benchmark Results Against Baseline
 * ===========================================
 *
 * Compare current benchmark results against a saved baseline to detect regressions.
 *
 * Usage:
 *     node compareBaseline.js --baseline BASELINE.json [--current CURRENT.json]
 *
 * Options:
 *     --baseline PATH     Path to baseline results file (required)
 *     --current PATH      Path to current results file (optional, runs benchmarks if not provided)
 *     --samples N         Number of samples for current run (default: 100)
 *     --threshold PERCENT Regression threshold percentage (default: 10.0)
 *
 * Last Updated: 2025-11-29T2300
 */

const path = require('path');
const fs = require('fs');
const { loadResults } = require('./benchmarkUtils');

/**
 * Compare two benchmark result sets
 */
function compareResults(baseline, current, threshold = 10.0) {
    const comparison = {
        baseline_timestamp: baseline.timestamp,
        current_timestamp: current.timestamp,
        threshold_percent: threshold,
        suites: {}
    };

    // Compare each suite
    Object.keys(baseline.suites).forEach(suiteName => {
        const baselineSuite = baseline.suites[suiteName];
        const currentSuite = current.suites[suiteName];

        if (!currentSuite) {
            comparison.suites[suiteName] = { error: 'Missing in current results' };
            return;
        }

        if (baselineSuite.error || currentSuite.error) {
            comparison.suites[suiteName] = {
                error: baselineSuite.error || currentSuite.error
            };
            return;
        }

        const suiteComparison = {
            benchmarks: {},
            summary: {
                total: 0,
                improved: 0,
                regressed: 0,
                unchanged: 0
            }
        };

        // Compare each benchmark
        Object.keys(baselineSuite.benchmarks).forEach(benchName => {
            const baselineBench = baselineSuite.benchmarks[benchName];
            const currentBench = currentSuite.benchmarks[benchName];

            if (!currentBench) {
                suiteComparison.benchmarks[benchName] = { error: 'Missing in current results' };
                return;
            }

            if (baselineBench.error || currentBench.error) {
                suiteComparison.benchmarks[benchName] = {
                    error: baselineBench.error || currentBench.error
                };
                return;
            }

            // Calculate difference
            const baselineMean = baselineBench.mean_ns;
            const currentMean = currentBench.mean_ns;
            const diffNs = currentMean - baselineMean;
            const diffPercent = (diffNs / baselineMean) * 100;

            suiteComparison.benchmarks[benchName] = {
                baseline_mean_ns: baselineMean,
                current_mean_ns: currentMean,
                diff_ns: diffNs,
                diff_percent: diffPercent,
                status: Math.abs(diffPercent) < threshold ? 'unchanged' :
                        diffPercent < 0 ? 'improved' : 'regressed'
            };

            // Update summary
            suiteComparison.summary.total++;
            if (Math.abs(diffPercent) < threshold) {
                suiteComparison.summary.unchanged++;
            } else if (diffPercent < 0) {
                suiteComparison.summary.improved++;
            } else {
                suiteComparison.summary.regressed++;
            }
        });

        comparison.suites[suiteName] = suiteComparison;
    });

    return comparison;
}

/**
 * Print comparison results
 */
function printComparison(comparison) {
    console.log("=".repeat(80));
    console.log("Benchmark Comparison Report");
    console.log("=".repeat(80));
    console.log(`Baseline: ${comparison.baseline_timestamp}`);
    console.log(`Current:  ${comparison.current_timestamp}`);
    console.log(`Threshold: ±${comparison.threshold_percent}%`);
    console.log();

    let totalRegressions = 0;
    let totalImprovements = 0;

    Object.entries(comparison.suites).forEach(([suiteName, suiteData]) => {
        console.log(`\n${suiteName.toUpperCase()}:`);
        console.log("-".repeat(60));

        if (suiteData.error) {
            console.log(`  ERROR: ${suiteData.error}`);
            return;
        }

        const { summary } = suiteData;
        console.log(`  Total benchmarks: ${summary.total}`);
        console.log(`  Improved: ${summary.improved} (faster)`);
        console.log(`  Regressed: ${summary.regressed} (slower)`);
        console.log(`  Unchanged: ${summary.unchanged} (within threshold)`);

        totalRegressions += summary.regressed;
        totalImprovements += summary.improved;

        // Show regressions in detail
        if (summary.regressed > 0) {
            console.log(`\n  Regressions:`);
            Object.entries(suiteData.benchmarks).forEach(([benchName, benchData]) => {
                if (benchData.status === 'regressed') {
                    const baselineUs = benchData.baseline_mean_ns / 1000;
                    const currentUs = benchData.current_mean_ns / 1000;
                    console.log(`    ⚠️  ${benchName}`);
                    console.log(`       Baseline: ${baselineUs.toFixed(2)} us`);
                    console.log(`       Current:  ${currentUs.toFixed(2)} us`);
                    console.log(`       Change:   +${benchData.diff_percent.toFixed(2)}% (slower)`);
                }
            });
        }

        // Show improvements
        if (summary.improved > 0) {
            console.log(`\n  Improvements:`);
            Object.entries(suiteData.benchmarks).forEach(([benchName, benchData]) => {
                if (benchData.status === 'improved') {
                    const baselineUs = benchData.baseline_mean_ns / 1000;
                    const currentUs = benchData.current_mean_ns / 1000;
                    console.log(`    ✓  ${benchName}`);
                    console.log(`       Baseline: ${baselineUs.toFixed(2)} us`);
                    console.log(`       Current:  ${currentUs.toFixed(2)} us`);
                    console.log(`       Change:   ${benchData.diff_percent.toFixed(2)}% (faster)`);
                }
            });
        }
    });

    // Overall summary
    console.log("\n" + "=".repeat(80));
    console.log("Overall Summary");
    console.log("=".repeat(80));
    console.log(`Total improvements: ${totalImprovements}`);
    console.log(`Total regressions: ${totalRegressions}`);

    if (totalRegressions > 0) {
        console.log(`\n⚠️  WARNING: ${totalRegressions} performance regressions detected!`);
        return false;
    } else {
        console.log(`\n✓ No significant performance regressions detected.`);
        return true;
    }
}

/**
 * Main function
 */
function main() {
    const args = process.argv.slice(2);

    // Parse arguments
    const baselineIdx = args.indexOf('--baseline');
    if (baselineIdx === -1 || baselineIdx + 1 >= args.length) {
        console.error('ERROR: --baseline argument required');
        console.error('Usage: node compareBaseline.js --baseline BASELINE.json [--current CURRENT.json]');
        process.exit(1);
    }

    const baselinePath = args[baselineIdx + 1];
    const currentIdx = args.indexOf('--current');
    const currentPath = currentIdx !== -1 && currentIdx + 1 < args.length ? args[currentIdx + 1] : null;
    const samples = args.includes('--samples') ? parseInt(args[args.indexOf('--samples') + 1]) : 100;
    const threshold = args.includes('--threshold') ? parseFloat(args[args.indexOf('--threshold') + 1]) : 10.0;

    // Load baseline
    let baseline;
    try {
        baseline = loadResults(baselinePath);
    } catch (e) {
        console.error(`ERROR loading baseline: ${e.message}`);
        process.exit(1);
    }

    // Load or run current benchmarks
    let current;
    if (currentPath) {
        try {
            current = loadResults(currentPath);
        } catch (e) {
            console.error(`ERROR loading current results: ${e.message}`);
            process.exit(1);
        }
    } else {
        console.error('No --current specified, running benchmarks...\n');
        const runAll = require('./runAll');

        // Capture runAll output
        const originalArgv = process.argv;
        process.argv = ['node', 'runAll', '--json', '--samples', samples.toString()];

        let output = '';
        const originalStdoutWrite = process.stdout.write;
        process.stdout.write = (chunk) => {
            output += chunk;
            return true;
        };

        runAll.main();

        process.stdout.write = originalStdoutWrite;
        process.argv = originalArgv;

        try {
            current = JSON.parse(output);
        } catch (e) {
            console.error(`ERROR parsing benchmark results: ${e.message}`);
            process.exit(1);
        }
    }

    // Compare results
    const comparison = compareResults(baseline, current, threshold);

    // Print comparison
    const success = printComparison(comparison);

    // Exit with appropriate code
    process.exit(success ? 0 : 1);
}

if (require.main === module) {
    main();
}

module.exports = { compareResults, printComparison, main };
