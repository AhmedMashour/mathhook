#!/bin/bash
#
# Update Rust Baseline
#
# Single command to update the Rust performance baseline using Criterion.
# Runs benchmarks and stores results with git metadata.
#
# Usage:
#     ./baseline_updater.sh [--quick]
#
# Output:
#     baselines/rust/latest.json
#     baselines/rust/history/YYYY-MM-DD_vX.Y.Z_commit_HASH.json
#     target/criterion/* (Criterion data)
#
# Last Updated: 2025-11-29T2000

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Configuration
QUICK_MODE=false
ITERATIONS=""

# Parse arguments
for arg in "$@"; do
    case $arg in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        *)
            ;;
    esac
done

echo "================================================================================"
echo "MathHook Rust Baseline Update (Criterion)"
echo "================================================================================"

# Navigate to benchmark crate root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCHMARK_ROOT="$SCRIPT_DIR/../.."
cd "$BENCHMARK_ROOT"

echo "Benchmark directory: $BENCHMARK_ROOT"
echo "Quick mode: $QUICK_MODE"
echo ""

# Run Criterion benchmarks
echo "Running Criterion benchmarks..."
if [ "$QUICK_MODE" = true ]; then
    cargo bench --benches -- --quick
else
    cargo bench --benches
fi

echo ""
echo "Criterion benchmarks completed."
echo "Results stored in: target/criterion/"
echo ""

# Extract git metadata
GIT_COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
GIT_COMMIT_FULL=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
GIT_DIRTY=false
if ! git diff-index --quiet HEAD 2>/dev/null; then
    GIT_DIRTY=true
fi

# Extract version from workspace Cargo.toml
VERSION=$(grep '^version' ../../Cargo.toml | head -1 | sed 's/version *= *"\(.*\)"/\1/')

# Get system info
OS_NAME=$(uname -s)
OS_VERSION=$(uname -r)
ARCH=$(uname -m)
CORES=$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "unknown")

# Create baselines directory
BASELINES_DIR="$BENCHMARK_ROOT/baselines/rust"
mkdir -p "$BASELINES_DIR/history"

# Convert Criterion results to our JSON format
echo "Converting Criterion results to baseline JSON format..."

# This is a simplified conversion - Criterion stores data in target/criterion/
# We'll create a basic JSON structure pointing to Criterion data
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
DATE=$(date -u +"%Y-%m-%d")

cat > "$BASELINES_DIR/latest.json" <<EOF
{
  "metadata": {
    "timestamp": "$TIMESTAMP",
    "git_commit": "$GIT_COMMIT",
    "git_branch": "$GIT_BRANCH",
    "git_dirty": $GIT_DIRTY,
    "platform": "rust",
    "version": "$VERSION",
    "system": {
      "os": "$OS_NAME $OS_VERSION",
      "arch": "$ARCH",
      "cores": $CORES
    },
    "criterion_data": "target/criterion/"
  },
  "benchmarks": {
    "_note": "Rust benchmarks use Criterion's built-in baseline system.",
    "_location": "See target/criterion/ for detailed Criterion results",
    "_usage": "Use 'cargo bench --baseline <name>' for Criterion comparisons"
  }
}
EOF

echo "${GREEN}Updated: $BASELINES_DIR/latest.json${NC}"

# Archive historical baseline
HISTORY_FILE="$BASELINES_DIR/history/${DATE}_v${VERSION}_commit_${GIT_COMMIT}.json"
cp "$BASELINES_DIR/latest.json" "$HISTORY_FILE"
echo "${GREEN}Archived: $HISTORY_FILE${NC}"

# Summary
echo ""
echo "Baseline Summary:"
echo "  Version: $VERSION"
echo "  Commit: $GIT_COMMIT"
echo "  Branch: $GIT_BRANCH"
echo "  Dirty: $GIT_DIRTY"
echo "  Criterion data: target/criterion/"
echo ""

if [ "$GIT_DIRTY" = true ]; then
    echo -e "${YELLOW}WARNING: Working directory has uncommitted changes!${NC}"
    echo "Consider committing changes before creating baselines."
    echo ""
fi

echo -e "${GREEN}Baseline updated successfully!${NC}"
echo ""
echo "Criterion baseline is now stored in target/criterion/."
echo "Use 'cargo bench --baseline <name>' to compare against this baseline."
