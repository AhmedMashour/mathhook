#!/usr/bin/env python3
"""Export Criterion results as baseline JSON."""

import json
from pathlib import Path
import argparse

def export_baseline(criterion_dir: Path, output_path: Path):
    """Export current criterion results as baseline."""
    baseline = {}

    if not criterion_dir.exists():
        print(f"Error: Criterion directory {criterion_dir} does not exist!")
        return

    for bench_file in criterion_dir.rglob("base/estimates.json"):
        bench_name = bench_file.parent.parent.name
        with open(bench_file) as f:
            data = json.load(f)
            baseline[bench_name] = {
                'mean': data['mean']['point_estimate'],
                'std_dev': data['std_dev']['point_estimate'],
                'median': data['median']['point_estimate']
            }

    if not baseline:
        print(f"Warning: No benchmarks found in {criterion_dir}")
        return

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w') as f:
        json.dump(baseline, f, indent=2)

    print(f"Exported {len(baseline)} benchmarks to {output_path}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    export_baseline(args.input, args.output)
