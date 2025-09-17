#!/bin/bash
# Update benchmark baselines from CI artifacts
set -e

ARTIFACTS_DIR="${1:-artifacts}"
BASELINES_DIR="benchmarks"

mkdir -p "$BASELINES_DIR"

for platform in rust python node; do
    src="$ARTIFACTS_DIR/${platform}-benchmark-results/${platform}.json"
    dst="$BASELINES_DIR/${platform}_baseline.json"

    if [ -f "$src" ]; then
        if python3 -c "import json; json.load(open('$src'))" 2>/dev/null; then
            cp "$src" "$dst"
            echo "Updated $platform baseline"
        else
            echo "Invalid JSON in $src, skipping"
        fi
    else
        echo "No results for $platform"
    fi
done

ls -la "$BASELINES_DIR/" 2>/dev/null || true
