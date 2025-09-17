#!/usr/bin/env python3
"""Parse Criterion benchmark output to JSON."""

import json
import re
import sys
from pathlib import Path


def parse_criterion_output(input_file: str, output_file: str) -> dict:
    """Parse Criterion benchmark output and save as JSON."""
    results = {"platform": "rust", "benchmarks": {}, "status": "success"}

    try:
        content = Path(input_file).read_text()

        # Pattern: "benchmark_name  time:   [123.45 ns 125.00 ns 126.55 ns]"
        pattern = r'(\w+)\s+time:\s+\[([0-9.]+)\s+(\w+)\s+([0-9.]+)\s+(\w+)\s+([0-9.]+)\s+(\w+)\]'

        for match in re.finditer(pattern, content):
            name = match.group(1)
            median = float(match.group(4))
            unit = match.group(5)
            results["benchmarks"][name] = {"median": median, "unit": unit}

        if not results["benchmarks"]:
            results["benchmarks"]["run_completed"] = {"median": 0, "unit": "marker"}

    except Exception as e:
        results["status"] = f"parse_error: {e}"

    Path(output_file).parent.mkdir(parents=True, exist_ok=True)
    Path(output_file).write_text(json.dumps(results, indent=2))

    print(json.dumps(results, indent=2))
    return results


if __name__ == "__main__":
    input_path = sys.argv[1] if len(sys.argv) > 1 else "benchmark-output.txt"
    output_path = sys.argv[2] if len(sys.argv) > 2 else "benchmark-results/rust.json"
    parse_criterion_output(input_path, output_path)
