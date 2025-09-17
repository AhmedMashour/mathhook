#!/usr/bin/env python3
"""
Update Python Baseline

Single command to update the Python performance baseline.
Runs benchmarks and stores results with git metadata.

Usage:
    python update_baseline.py [--iterations N]

Output:
    baselines/python/latest.json
    baselines/python/history/YYYY-MM-DD_vX.Y.Z_commit_HASH.json

Last Updated: 2025-11-29T2000
"""

import json
import subprocess
import sys
import os
from datetime import datetime
from pathlib import Path
import argparse

# Add bench script to path
sys.path.insert(0, str(Path(__file__).parent))
from bench_mathhook import run_all_benchmarks


def get_git_metadata() -> dict:
    """Extract git metadata for baseline tagging."""
    try:
        commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
        commit_short = subprocess.check_output(['git', 'rev-parse', '--short', 'HEAD'], text=True).strip()
        branch = subprocess.check_output(['git', 'rev-parse', '--abbrev-ref', 'HEAD'], text=True).strip()

        # Check if working directory is dirty
        dirty = subprocess.call(['git', 'diff-index', '--quiet', 'HEAD']) != 0

        return {
            'git_commit': commit_short,
            'git_commit_full': commit,
            'git_branch': branch,
            'git_dirty': dirty
        }
    except subprocess.CalledProcessError:
        return {
            'git_commit': 'unknown',
            'git_commit_full': 'unknown',
            'git_branch': 'unknown',
            'git_dirty': False
        }


def get_system_info() -> dict:
    """Get system information."""
    import platform
    import multiprocessing

    return {
        'os': platform.system() + ' ' + platform.release(),
        'arch': platform.machine(),
        'cpu': platform.processor() or 'unknown',
        'cores': multiprocessing.cpu_count()
    }


def get_version() -> str:
    """Get MathHook version from Cargo.toml."""
    cargo_toml = Path(__file__).parents[3] / 'Cargo.toml'

    if cargo_toml.exists():
        with open(cargo_toml, 'r') as f:
            for line in f:
                if line.strip().startswith('version'):
                    # Extract version from: version = "0.1.0"
                    version = line.split('=')[1].strip().strip('"')
                    return version

    return '0.0.0'


def create_baseline(benchmark_results: dict, iterations: int) -> dict:
    """Create baseline JSON with metadata."""
    git_meta = get_git_metadata()
    system_info = get_system_info()
    version = get_version()

    baseline = {
        'metadata': {
            'timestamp': datetime.utcnow().isoformat() + 'Z',
            'git_commit': git_meta['git_commit'],
            'git_branch': git_meta['git_branch'],
            'git_dirty': git_meta['git_dirty'],
            'platform': 'python',
            'version': version,
            'system': system_info,
            'iterations': iterations
        },
        'benchmarks': benchmark_results['benchmarks']
    }

    return baseline


def save_baseline(baseline: dict, baselines_dir: Path):
    """Save baseline to latest.json and historical file."""
    # Ensure directories exist
    baselines_dir.mkdir(parents=True, exist_ok=True)
    history_dir = baselines_dir / 'history'
    history_dir.mkdir(exist_ok=True)

    # Save as latest.json
    latest_path = baselines_dir / 'latest.json'
    with open(latest_path, 'w') as f:
        json.dump(baseline, f, indent=2)
    print(f"Updated: {latest_path}")

    # Save historical baseline
    meta = baseline['metadata']
    date = datetime.utcnow().strftime('%Y-%m-%d')
    version = meta['version']
    commit = meta['git_commit']

    history_filename = f"{date}_v{version}_commit_{commit}.json"
    history_path = history_dir / history_filename

    with open(history_path, 'w') as f:
        json.dump(baseline, f, indent=2)
    print(f"Archived: {history_path}")

    # Summary
    print("\nBaseline Summary:")
    print(f"  Version: {version}")
    print(f"  Commit: {commit}")
    print(f"  Branch: {meta['git_branch']}")
    print(f"  Dirty: {meta['git_dirty']}")
    print(f"  Benchmarks: {len(baseline['benchmarks'])}")

    if meta['git_dirty']:
        print("\nWARNING: Working directory has uncommitted changes!")
        print("Consider committing changes before creating baselines.")


def main():
    parser = argparse.ArgumentParser(description='Update Python Performance Baseline')
    parser.add_argument('--iterations', type=int, default=100,
                        help='Number of benchmark iterations (default: 100)')
    args = parser.parse_args()

    # Determine baselines directory (relative to this script)
    script_dir = Path(__file__).parent
    baselines_dir = script_dir.parent.parent / 'baselines' / 'python'

    print("=" * 80)
    print("MathHook Python Baseline Update")
    print("=" * 80)
    print(f"Iterations: {args.iterations}")
    print(f"Baselines directory: {baselines_dir}")
    print()

    # Run benchmarks
    print("Running benchmarks...")
    benchmark_results = run_all_benchmarks(iterations=args.iterations)

    # Create baseline with metadata
    baseline = create_baseline(benchmark_results, args.iterations)

    # Save baseline
    save_baseline(baseline, baselines_dir)

    print("\nBaseline updated successfully!")


if __name__ == '__main__':
    main()
