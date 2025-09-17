#!/usr/bin/env node
/**
 * Benchmark Utilities for MathHook Node.js Benchmarks
 * ====================================================
 *
 * Shared utilities for timing, statistics, and result formatting.
 * Mirrors: public/python/core_performance.py benchmark utilities
 *
 * Last Updated: 2025-11-29T2300
 */

/**
 * Benchmark result class with statistical analysis
 */
class BenchmarkResult {
    constructor(name, timesNs) {
        this.name = name;
        this.timesNs = timesNs;
        this.samples = timesNs.length;

        // Calculate statistics
        const sum = timesNs.reduce((a, b) => a + b, 0);
        this.meanNs = sum / timesNs.length;

        const variance = timesNs.reduce((acc, t) => acc + Math.pow(t - this.meanNs, 2), 0) / timesNs.length;
        this.stdDevNs = Math.sqrt(variance);

        const sorted = [...timesNs].sort((a, b) => a - b);
        this.medianNs = sorted[Math.floor(sorted.length / 2)];
        this.minNs = sorted[0];
        this.maxNs = sorted[sorted.length - 1];
    }

    toString() {
        return `${this.name}: ${this.meanNs.toFixed(2)}ns ± ${this.stdDevNs.toFixed(2)}ns (median: ${this.medianNs.toFixed(2)}ns, range: [${this.minNs.toFixed(2)}, ${this.maxNs.toFixed(2)}])`;
    }

    toJSON() {
        return {
            name: this.name,
            mean_ns: this.meanNs,
            std_dev_ns: this.stdDevNs,
            median_ns: this.medianNs,
            min_ns: this.minNs,
            max_ns: this.maxNs,
            samples: this.samples
        };
    }
}

/**
 * Benchmark a function with warmup and multiple samples
 *
 * @param {Function} func - Function to benchmark (no arguments)
 * @param {number} samples - Number of timing samples to collect
 * @param {number} warmup - Number of warmup iterations
 * @returns {BenchmarkResult} Timing statistics
 */
function benchmark(func, samples = 100, warmup = 10) {
    // Warmup
    for (let i = 0; i < warmup; i++) {
        func();
    }

    // Collect samples (nanosecond precision)
    const times = [];
    for (let i = 0; i < samples; i++) {
        const start = process.hrtime.bigint();
        func();
        const end = process.hrtime.bigint();
        times.push(Number(end - start));
    }

    return new BenchmarkResult(func.name, times);
}

/**
 * Run multiple benchmarks and collect results
 *
 * @param {Object} benchmarks - Object mapping benchmark names to functions
 * @param {number} samples - Number of samples per benchmark
 * @param {boolean} verbose - Print progress to stderr
 * @returns {Object} Results object with metadata and benchmark results
 */
function runBenchmarks(benchmarks, samples = 100, verbose = true) {
    const results = {
        platform: 'nodejs-mathhook',
        node_version: process.version,
        timestamp: new Date().toISOString(),
        benchmarks: {}
    };

    const benchNames = Object.keys(benchmarks);
    const total = benchNames.length;

    benchNames.forEach((name, index) => {
        if (verbose) {
            process.stderr.write(`[${index + 1}/${total}] Running ${name}...`);
        }

        try {
            const result = benchmark(benchmarks[name], samples);
            results.benchmarks[name] = result.toJSON();

            if (verbose) {
                process.stderr.write(` ${result.meanNs.toFixed(2)}ns\n`);
            }
        } catch (e) {
            results.benchmarks[name] = { error: e.message };

            if (verbose) {
                process.stderr.write(` ERROR: ${e.message}\n`);
            }
        }
    });

    return results;
}

/**
 * Calculate parsing overhead between direct and parsed variants
 *
 * @param {Object} results - Benchmark results object
 * @param {Array} pairs - Array of [label, directName, parsedName] tuples
 * @returns {Object} Overhead analysis
 */
function calculateParsingOverhead(results, pairs) {
    const overhead = {};

    pairs.forEach(([label, directName, parsedName]) => {
        if (results.benchmarks[directName] && results.benchmarks[parsedName]) {
            const directTime = results.benchmarks[directName].mean_ns;
            const parsedTime = results.benchmarks[parsedName].mean_ns;
            const overheadNs = parsedTime - directTime;
            const overheadPct = directTime > 0 ? (overheadNs / directTime) * 100 : 0;

            overhead[label] = {
                direct_ns: directTime,
                parsed_ns: parsedTime,
                overhead_ns: overheadNs,
                overhead_pct: overheadPct
            };
        }
    });

    return overhead;
}

/**
 * Print human-readable results
 *
 * @param {Object} results - Benchmark results object
 * @param {Array} categories - Optional category grouping
 */
function printResults(results, categories = null) {
    console.log("=".repeat(80));
    console.log("MathHook Node.js Benchmark Results");
    console.log("=".repeat(80));
    console.log(`Node.js version: ${results.node_version}`);
    console.log(`Timestamp: ${results.timestamp}`);
    console.log();

    if (categories) {
        // Print by category
        categories.forEach(([categoryName, prefixes]) => {
            console.log(`\n${categoryName}:`);
            console.log("-".repeat(60));

            Object.entries(results.benchmarks).forEach(([name, data]) => {
                if (prefixes.some(p => name.startsWith(p))) {
                    if (data.error) {
                        console.log(`  ${name.padEnd(40)} ERROR: ${data.error}`);
                    } else {
                        const meanUs = data.mean_ns / 1000;
                        const stddevUs = data.std_dev_ns / 1000;
                        console.log(`  ${name.padEnd(40)} ${meanUs.toFixed(2).padStart(12)} us  (±${stddevUs.toFixed(2)} us)`);
                    }
                }
            });
        });
    } else {
        // Print all results
        console.log("Detailed Results:");
        console.log("-".repeat(80));

        Object.entries(results.benchmarks).forEach(([name, data]) => {
            if (data.error) {
                console.log(`${name}: ERROR - ${data.error}`);
            } else {
                const result = new BenchmarkResult(name, []);
                Object.assign(result, data);
                console.log(result.toString());
            }
        });
    }
}

/**
 * Save results to JSON file
 *
 * @param {Object} results - Benchmark results object
 * @param {string} filepath - Output file path
 */
function saveResults(results, filepath) {
    const fs = require('fs');
    fs.writeFileSync(filepath, JSON.stringify(results, null, 2));
}

/**
 * Load results from JSON file
 *
 * @param {string} filepath - Input file path
 * @returns {Object} Benchmark results object
 */
function loadResults(filepath) {
    const fs = require('fs');
    return JSON.parse(fs.readFileSync(filepath, 'utf8'));
}

module.exports = {
    BenchmarkResult,
    benchmark,
    runBenchmarks,
    calculateParsingOverhead,
    printResults,
    saveResults,
    loadResults
};
