#!/usr/bin/env node
/**
 * Benchmark Node.js bindings performance for hand-written vs macro-generated functions.
 *
 * This benchmarks the FULL STACK: Node.js → NAPI → Rust
 */

const { JsExpression, sin, cos, tan, sin_macro_generated, cos_macro_generated, tan_macro_generated } = require('./index.js');

function benchFunction(funcName, func, iterations = 1000000) {
    // Create a symbol once
    const x = JsExpression.symbol("x");

    // Warmup
    for (let i = 0; i < 1000; i++) {
        func(x);
    }

    // Benchmark with statistical analysis
    const times = [];
    const batchSize = iterations / 100;

    for (let batch = 0; batch < 100; batch++) {
        const start = process.hrtime.bigint();
        for (let i = 0; i < batchSize; i++) {
            func(x);
        }
        const end = process.hrtime.bigint();
        const nsPerCall = Number(end - start) / batchSize;
        times.push(nsPerCall);
    }

    // Calculate statistics
    const mean = times.reduce((a, b) => a + b, 0) / times.length;
    const variance = times.reduce((sum, time) => sum + Math.pow(time - mean, 2), 0) / times.length;
    const stdev = Math.sqrt(variance);
    const median = times.sort((a, b) => a - b)[Math.floor(times.length / 2)];
    const min = Math.min(...times);
    const max = Math.max(...times);

    return { mean, stdev, median, min, max };
}

function main() {
    console.log("=".repeat(80));
    console.log("Node.js Bindings Performance Benchmark");
    console.log("=".repeat(80));
    console.log();
    console.log("Testing: Node.js → NAPI → Rust (full stack)");
    console.log("Iterations: 1,000,000 per function");
    console.log();

    // Benchmark hand-written functions
    console.log("Hand-Written Bindings (Current Approach):");
    console.log("-".repeat(80));

    const handWritten = [
        { name: 'sin', func: sin },
        { name: 'cos', func: cos },
        { name: 'tan', func: tan }
    ];

    const hwResults = {};
    for (const { name, func } of handWritten) {
        const result = benchFunction(name, func);
        hwResults[name] = result;
        console.log(`${name.padEnd(20)} ${result.mean.toFixed(2).padStart(8)} ns/call  ` +
                    `(σ=${result.stdev.toFixed(2).padStart(6)} ns, median=${result.median.toFixed(2).padStart(8)} ns)`);
    }

    console.log();

    // Benchmark macro-generated functions
    console.log("Macro-Generated Bindings (Proposed Approach):");
    console.log("-".repeat(80));

    const macroGenerated = [
        { name: 'sin_macro_generated', func: sin_macro_generated },
        { name: 'cos_macro_generated', func: cos_macro_generated },
        { name: 'tan_macro_generated', func: tan_macro_generated }
    ];

    const mgResults = {};
    for (const { name, func } of macroGenerated) {
        const result = benchFunction(name, func);
        mgResults[name] = result;
        console.log(`${name.padEnd(20)} ${result.mean.toFixed(2).padStart(8)} ns/call  ` +
                    `(σ=${result.stdev.toFixed(2).padStart(6)} ns, median=${result.median.toFixed(2).padStart(8)} ns)`);
    }

    console.log();
    console.log("=".repeat(80));
    console.log("Comparison Analysis:");
    console.log("=".repeat(80));
    console.log();

    // Compare corresponding functions
    const comparisons = [
        ['sin', 'sin_macro_generated'],
        ['cos', 'cos_macro_generated'],
        ['tan', 'tan_macro_generated']
    ];

    for (const [hwName, mgName] of comparisons) {
        const hwMean = hwResults[hwName].mean;
        const mgMean = mgResults[mgName].mean;
        const diffNs = mgMean - hwMean;
        const diffPct = (diffNs / hwMean) * 100;

        console.log(`${hwName} vs ${mgName}:`);
        console.log(`  Hand-written:     ${hwMean.toFixed(2).padStart(8)} ns/call`);
        console.log(`  Macro-generated:  ${mgMean.toFixed(2).padStart(8)} ns/call`);
        console.log(`  Difference:       ${(diffNs >= 0 ? '+' : '') + diffNs.toFixed(2).padStart(7)} ns (${(diffPct >= 0 ? '+' : '') + diffPct.toFixed(2)}%)`);

        if (Math.abs(diffPct) < 5) {
            console.log(`  Verdict:          ✅ IDENTICAL (within 5% noise)`);
        } else if (diffPct < 0) {
            console.log(`  Verdict:          ✅ MACRO FASTER by ${(-diffPct).toFixed(2)}%`);
        } else {
            console.log(`  Verdict:          ⚠️  MACRO SLOWER by ${diffPct.toFixed(2)}%`);
        }
        console.log();
    }

    // Overall summary
    console.log("=".repeat(80));
    console.log("Overall Summary:");
    console.log("=".repeat(80));

    const avgHw = Object.values(hwResults).reduce((sum, r) => sum + r.mean, 0) / Object.keys(hwResults).length;
    const avgMg = Object.values(mgResults).reduce((sum, r) => sum + r.mean, 0) / Object.keys(mgResults).length;
    const avgDiff = avgMg - avgHw;
    const avgDiffPct = (avgDiff / avgHw) * 100;

    console.log(`Average hand-written:     ${avgHw.toFixed(2).padStart(8)} ns/call`);
    console.log(`Average macro-generated:  ${avgMg.toFixed(2).padStart(8)} ns/call`);
    console.log(`Average difference:       ${(avgDiff >= 0 ? '+' : '') + avgDiff.toFixed(2).padStart(7)} ns (${(avgDiffPct >= 0 ? '+' : '') + avgDiffPct.toFixed(2)}%)`);
    console.log();

    if (Math.abs(avgDiffPct) < 5) {
        console.log("✅ RESULT: Macro-generated bindings have ZERO overhead");
        console.log("   Performance is identical to hand-written (within measurement noise)");
    } else if (avgDiffPct < 0) {
        console.log(`✅ RESULT: Macro-generated bindings are ${(-avgDiffPct).toFixed(2)}% FASTER`);
    } else {
        console.log(`⚠️  RESULT: Macro-generated bindings are ${avgDiffPct.toFixed(2)}% SLOWER`);
    }
}

main();
