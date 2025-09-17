#!/usr/bin/env node
/**
 * Compare Against Node.js Baseline
 *
 * Runs current benchmarks and compares against stored baseline.
 * Detects performance regressions and improvements.
 *
 * Usage:
 *     node compare_baseline.js [--iterations N] [--threshold PERCENT]
 *
 * Exit Codes:
 *     0: No regressions detected
 *     1: Performance regressions detected (slower than threshold)
 *     2: Baseline not found or error
 *
 * Last Updated: 2025-11-29T2000
 */

const fs = require('fs');
const path = require('path');

// Import benchmark runner
const { runAllBenchmarks } = require('./bench_mathhook');

function loadBaseline(baselinesDir) {
    const latestPath = path.join(baselinesDir, 'latest.json');

    if (!fs.existsSync(latestPath)) {
        console.error(`ERROR: No baseline found at ${latestPath}`);
        console.error('Run: node update_baseline.js');
        process.exit(2);
    }

    const content = fs.readFileSync(latestPath, 'utf-8');
    return JSON.parse(content);
}

function compareBenchmarks(current, baseline, thresholdPercent) {
    const currentBenches = current.benchmarks;
    const baselineBenches = baseline.benchmarks;

    console.log('\n' + '='.repeat(80));
    console.log('Performance Comparison vs Baseline');
    console.log('='.repeat(80));
    console.log(`Baseline: ${baseline.metadata.git_commit} (${baseline.metadata.timestamp})`);
    console.log(`Threshold: ${thresholdPercent}% slowdown allowed`);
    console.log();

    // Track regressions
    const regressions = [];
    const improvements = [];
    const unchanged = [];

    // Compare each benchmark
    for (const [name, currentData] of Object.entries(currentBenches)) {
        if (currentData.error) {
            console.log(`SKIP ${name.padEnd(30)} (error in current)`);
            continue;
        }

        if (!(name in baselineBenches)) {
            console.log(`NEW  ${name.padEnd(30)} (not in baseline)`);
            continue;
        }

        const baselineData = baselineBenches[name];
        if (baselineData.error) {
            console.log(`SKIP ${name.padEnd(30)} (error in baseline)`);
            continue;
        }

        // Calculate percentage change (using median for stability)
        const currentNs = currentData.median_ns;
        const baselineNs = baselineData.median_ns;

        const percentChange = ((currentNs - baselineNs) / baselineNs) * 100;

        // Categorize
        if (percentChange > thresholdPercent) {
            regressions.push({ name, percentChange, currentNs, baselineNs });
        } else if (percentChange < -thresholdPercent) {
            improvements.push({ name, percentChange, currentNs, baselineNs });
        } else {
            unchanged.push({ name, percentChange, currentNs, baselineNs });
        }
    }

    // Print results
    if (regressions.length > 0) {
        console.log('\nREGRESSIONS (slower than baseline):');
        console.log('-'.repeat(80));
        regressions.sort((a, b) => b.percentChange - a.percentChange);
        for (const { name, percentChange, currentNs, baselineNs } of regressions) {
            const currentUs = currentNs / 1000;
            const baselineUs = baselineNs / 1000;
            console.log(`  ${name.padEnd(30)} ${percentChange.toFixed(2).padStart(7)}%  (${currentUs.toFixed(2)} us vs ${baselineUs.toFixed(2)} us)`);
        }
    }

    if (improvements.length > 0) {
        console.log('\nIMPROVEMENTS (faster than baseline):');
        console.log('-'.repeat(80));
        improvements.sort((a, b) => a.percentChange - b.percentChange);
        for (const { name, percentChange, currentNs, baselineNs } of improvements) {
            const currentUs = currentNs / 1000;
            const baselineUs = baselineNs / 1000;
            console.log(`  ${name.padEnd(30)} ${percentChange.toFixed(2).padStart(7)}%  (${currentUs.toFixed(2)} us vs ${baselineUs.toFixed(2)} us)`);
        }
    }

    if (unchanged.length > 0) {
        console.log(`\nUNCHANGED (within ${thresholdPercent}% threshold): ${unchanged.length} benchmarks`);
    }

    // Summary
    console.log('\n' + '='.repeat(80));
    console.log('Summary:');
    console.log(`  Regressions: ${regressions.length}`);
    console.log(`  Improvements: ${improvements.length}`);
    console.log(`  Unchanged: ${unchanged.length}`);
    console.log('='.repeat(80));

    return regressions.length > 0;
}

function main() {
    const args = process.argv.slice(2);
    let iterations = 100;
    let threshold = 10.0;

    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--iterations' && i + 1 < args.length) {
            iterations = parseInt(args[i + 1], 10);
            i++;
        } else if (args[i] === '--threshold' && i + 1 < args.length) {
            threshold = parseFloat(args[i + 1]);
            i++;
        }
    }

    // Determine baselines directory
    const baselinesDir = path.join(__dirname, '../../baselines/node');

    console.log('='.repeat(80));
    console.log('MathHook Node.js Baseline Comparison');
    console.log('='.repeat(80));
    console.log(`Iterations: ${iterations}`);
    console.log(`Threshold: ${threshold}%`);
    console.log();

    // Load baseline
    console.log('Loading baseline...');
    const baseline = loadBaseline(baselinesDir);

    // Run current benchmarks
    console.log('Running current benchmarks...');
    const current = runAllBenchmarks(iterations);

    // Compare
    const hasRegressions = compareBenchmarks(current, baseline, threshold);

    // Exit with appropriate code
    if (hasRegressions) {
        console.log('\nFAILURE: Performance regressions detected!');
        process.exit(1);
    } else {
        console.log('\nSUCCESS: No performance regressions detected!');
        process.exit(0);
    }
}

if (require.main === module) {
    main();
}

module.exports = { compareBenchmarks };
