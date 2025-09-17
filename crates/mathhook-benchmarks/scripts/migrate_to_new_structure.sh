#!/bin/bash
#
# Migrate to New Benchmark Structure
#
# Moves existing benchmark files from cross_platform/ to the new organized structure.
# Safe to run multiple times (uses cp instead of mv for safety).
#
# Usage:
#     ./migrate_to_new_structure.sh [--dry-run]
#
# Last Updated: 2025-11-29T2000

set -e

DRY_RUN=false

# Parse arguments
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
fi

# Colors
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "================================================================================"
echo "MathHook Benchmark Structure Migration"
echo "================================================================================"
echo "Dry run: $DRY_RUN"
echo ""

# Navigate to benchmark root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARK_ROOT="$SCRIPT_DIR/.."
cd "$BENCHMARK_ROOT"

# Function to safely copy file
copy_file() {
    local src="$1"
    local dest="$2"

    if [ ! -f "$src" ]; then
        echo -e "${YELLOW}SKIP: $src (not found)${NC}"
        return
    fi

    if [ "$DRY_RUN" = true ]; then
        echo -e "${BLUE}WOULD COPY: $src -> $dest${NC}"
    else
        mkdir -p "$(dirname "$dest")"
        cp "$src" "$dest"
        echo -e "${GREEN}COPIED: $src -> $dest${NC}"
    fi
}

echo "Migrating public benchmarks..."
echo "---"

# Public benchmarks (MathHook-only, no external deps)
copy_file "cross_platform/bench_mathhook_python.py" "public/python/bench_mathhook.py"
copy_file "cross_platform/bench_mathhook_node.js" "public/node/bench_mathhook.js"

echo ""
echo "Migrating utility scripts..."
echo "---"

# Utility scripts
copy_file "cross_platform/compare_results.py" "scripts/compare_results.py"
copy_file "cross_platform/compare_python_results.py" "scripts/compare_python_results.py"

echo ""
echo "================================================================================"

if [ "$DRY_RUN" = true ]; then
    echo "DRY RUN: No files were actually moved."
    echo "Run without --dry-run to perform migration."
else
    echo "Migration complete!"
    echo ""
    echo "IMPORTANT: Review migrated files before deleting originals:"
    echo "  - Check public/python/bench_mathhook.py"
    echo "  - Check public/node/bench_mathhook.js"
    echo "  - Check comparison/ directory"
    echo ""
    echo "When satisfied, you can remove cross_platform/ duplicates:"
    echo "  rm cross_platform/bench_mathhook_python.py"
    echo "  rm cross_platform/bench_mathhook_node.js"
    echo "  # etc."
fi

echo "================================================================================"
