#!/usr/bin/env node
/**
 * Update Node.js Baseline
 *
 * Single command to update the Node.js performance baseline.
 * Runs benchmarks and stores results with git metadata.
 *
 * Usage:
 *     node update_baseline.js [--iterations N]
 *
 * Output:
 *     baselines/node/latest.json
 *     baselines/node/history/YYYY-MM-DD_vX.Y.Z_commit_HASH.json
 *
 * Last Updated: 2025-11-29T2000
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Import benchmark runner
const { runAllBenchmarks } = require('./bench_mathhook');

function getGitMetadata() {
    try {
        const commit = execSync('git rev-parse HEAD', { encoding: 'utf-8' }).trim();
        const commitShort = execSync('git rev-parse --short HEAD', { encoding: 'utf-8' }).trim();
        const branch = execSync('git rev-parse --abbrev-ref HEAD', { encoding: 'utf-8' }).trim();

        // Check if working directory is dirty
        let dirty = false;
        try {
            execSync('git diff-index --quiet HEAD');
        } catch (e) {
            dirty = true;
        }

        return {
            git_commit: commitShort,
            git_commit_full: commit,
            git_branch: branch,
            git_dirty: dirty
        };
    } catch (e) {
        return {
            git_commit: 'unknown',
            git_commit_full: 'unknown',
            git_branch: 'unknown',
            git_dirty: false
        };
    }
}

function getSystemInfo() {
    return {
        os: `${os.type()} ${os.release()}`,
        arch: os.arch(),
        cpu: os.cpus()[0]?.model || 'unknown',
        cores: os.cpus().length
    };
}

function getVersion() {
    // Read version from workspace Cargo.toml
    const cargoTomlPath = path.join(__dirname, '../../../Cargo.toml');

    if (fs.existsSync(cargoTomlPath)) {
        const content = fs.readFileSync(cargoTomlPath, 'utf-8');
        const match = content.match(/version\s*=\s*"([^"]+)"/);
        if (match) {
            return match[1];
        }
    }

    return '0.0.0';
}

function createBaseline(benchmarkResults, iterations) {
    const gitMeta = getGitMetadata();
    const systemInfo = getSystemInfo();
    const version = getVersion();

    return {
        metadata: {
            timestamp: new Date().toISOString(),
            git_commit: gitMeta.git_commit,
            git_branch: gitMeta.git_branch,
            git_dirty: gitMeta.git_dirty,
            platform: 'node',
            version: version,
            system: systemInfo,
            iterations: iterations
        },
        benchmarks: benchmarkResults.benchmarks
    };
}

function saveBaseline(baseline, baselinesDir) {
    // Ensure directories exist
    if (!fs.existsSync(baselinesDir)) {
        fs.mkdirSync(baselinesDir, { recursive: true });
    }

    const historyDir = path.join(baselinesDir, 'history');
    if (!fs.existsSync(historyDir)) {
        fs.mkdirSync(historyDir, { recursive: true });
    }

    // Save as latest.json
    const latestPath = path.join(baselinesDir, 'latest.json');
    fs.writeFileSync(latestPath, JSON.stringify(baseline, null, 2));
    console.log(`Updated: ${latestPath}`);

    // Save historical baseline
    const meta = baseline.metadata;
    const date = new Date().toISOString().split('T')[0];
    const version = meta.version;
    const commit = meta.git_commit;

    const historyFilename = `${date}_v${version}_commit_${commit}.json`;
    const historyPath = path.join(historyDir, historyFilename);

    fs.writeFileSync(historyPath, JSON.stringify(baseline, null, 2));
    console.log(`Archived: ${historyPath}`);

    // Summary
    console.log('\nBaseline Summary:');
    console.log(`  Version: ${version}`);
    console.log(`  Commit: ${commit}`);
    console.log(`  Branch: ${meta.git_branch}`);
    console.log(`  Dirty: ${meta.git_dirty}`);
    console.log(`  Benchmarks: ${Object.keys(baseline.benchmarks).length}`);

    if (meta.git_dirty) {
        console.log('\nWARNING: Working directory has uncommitted changes!');
        console.log('Consider committing changes before creating baselines.');
    }
}

function main() {
    const args = process.argv.slice(2);
    let iterations = 100;

    for (let i = 0; i < args.length; i++) {
        if (args[i] === '--iterations' && i + 1 < args.length) {
            iterations = parseInt(args[i + 1], 10);
            i++;
        }
    }

    // Determine baselines directory
    const baselinesDir = path.join(__dirname, '../../baselines/node');

    console.log('='.repeat(80));
    console.log('MathHook Node.js Baseline Update');
    console.log('='.repeat(80));
    console.log(`Iterations: ${iterations}`);
    console.log(`Baselines directory: ${baselinesDir}`);
    console.log();

    // Run benchmarks
    console.log('Running benchmarks...');
    const benchmarkResults = runAllBenchmarks(iterations);

    // Create baseline with metadata
    const baseline = createBaseline(benchmarkResults, iterations);

    // Save baseline
    saveBaseline(baseline, baselinesDir);

    console.log('\nBaseline updated successfully!');
}

if (require.main === module) {
    main();
}

module.exports = { createBaseline, saveBaseline };
