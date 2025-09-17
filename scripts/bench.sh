#!/bin/bash
# MathHook Unified Benchmark Helper Script
# Last Updated: 2025-11-29T10:30:00
#
# Usage:
#   ./scripts/bench.sh [command] [options]
#
# Commands:
#   run           Run all benchmarks (default)
#   quick         Quick benchmark run (reduced sample size)
#   save          Save current results as baseline
#   compare       Compare against saved baseline
#   rust          Run Rust benchmarks only
#   python        Run Python benchmarks only
#   node          Run Node.js benchmarks only
#   group         Run specific benchmark group
#   ci            CI mode (fail on regression >10%)
#   clean         Clean benchmark artifacts
#   help          Show this help message

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BENCH_DIR="$PROJECT_ROOT/crates/mathhook-benchmarks"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}======================================${NC}"
    echo -e "${BLUE}  MathHook Benchmark Suite${NC}"
    echo -e "${BLUE}======================================${NC}"
    echo ""
}

print_step() {
    echo -e "${GREEN}[STEP]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

show_help() {
    cat << EOF
MathHook Unified Benchmark Helper

USAGE:
    ./scripts/bench.sh [COMMAND] [OPTIONS]

COMMANDS:
    run                     Run all Rust benchmarks (default)
    quick                   Quick benchmark run with reduced samples
    save <name>             Save baseline with given name
    compare <name>          Compare against named baseline
    rust [group]            Run Rust benchmarks, optionally specific group
    python                  Run Python benchmarks
    node                    Run Node.js benchmarks
    group <name>            Run specific benchmark group
    ci                      CI mode with regression detection
    clean                   Clean benchmark artifacts
    status                  Show benchmark infrastructure status
    help                    Show this help message

BENCHMARK GROUPS (Rust):
    core_performance        Basic operations and memory
    calculus_benchmarks     Derivatives and integrals
    solving_benchmarks      Equation solving
    simplification_benchmarks  Algebraic simplification
    function_evaluation_benchmarks  Function evaluation
    polynomial_benchmarks   Polynomial operations
    parsing_benchmarks      Parser performance

EXAMPLES:
    ./scripts/bench.sh run
    ./scripts/bench.sh save my-feature
    ./scripts/bench.sh compare main
    ./scripts/bench.sh rust gcd_algorithms
    ./scripts/bench.sh group polynomial_benchmarks
    ./scripts/bench.sh quick

BASELINE MANAGEMENT:
    Baselines are stored in crates/mathhook-benchmarks/baselines/
    Use 'save' to create a named baseline
    Use 'compare' to check performance against a baseline

CI INTEGRATION:
    The 'ci' command runs benchmarks and fails if:
    - Any benchmark regresses more than 10%
    - Critical operations regress more than 5%

EOF
}

run_rust_benchmarks() {
    local group="$1"
    local extra_args="$2"

    print_step "Running Rust benchmarks..."

    if [ -n "$group" ]; then
        cargo bench -p mathhook-benchmarks --bench "$group" -- $extra_args
    else
        cargo bench -p mathhook-benchmarks --benches -- $extra_args
    fi
}

run_rust_quick() {
    print_step "Running quick Rust benchmarks (sample-size=10)..."
    cargo bench -p mathhook-benchmarks --benches -- --sample-size 10
}

save_baseline() {
    local name="${1:-current}"
    print_step "Saving baseline as '$name'..."
    cargo bench -p mathhook-benchmarks --benches -- --save-baseline "$name"
    echo ""
    print_step "Baseline '$name' saved successfully"
}

compare_baseline() {
    local name="${1:-main}"
    print_step "Comparing against baseline '$name'..."
    cargo bench -p mathhook-benchmarks --benches -- --baseline "$name"
}

run_python_benchmarks() {
    print_step "Running Python benchmarks..."

    local python_dir="$BENCH_DIR/public/python"
    if [ ! -d "$python_dir" ]; then
        print_error "Python benchmarks directory not found: $python_dir"
        exit 1
    fi

    cd "$python_dir"

    if [ -f "bench_mathhook.py" ]; then
        python3 bench_mathhook.py
    else
        print_warn "bench_mathhook.py not found, running individual modules..."
        for module in core_performance.py calculus_benchmarks.py simplification_benchmarks.py; do
            if [ -f "$module" ]; then
                print_step "Running $module..."
                python3 "$module"
            fi
        done
    fi

    cd "$PROJECT_ROOT"
}

run_node_benchmarks() {
    print_step "Running Node.js benchmarks..."

    local node_dir="$BENCH_DIR/public/node"
    if [ ! -d "$node_dir" ]; then
        print_error "Node.js benchmarks directory not found: $node_dir"
        exit 1
    fi

    cd "$node_dir"

    if [ -f "bench_mathhook.js" ]; then
        node bench_mathhook.js
    else
        print_warn "bench_mathhook.js not found"
    fi

    cd "$PROJECT_ROOT"
}

run_group() {
    local group="$1"
    if [ -z "$group" ]; then
        print_error "Please specify a benchmark group"
        echo "Available groups:"
        echo "  core_performance, calculus_benchmarks, solving_benchmarks"
        echo "  simplification_benchmarks, function_evaluation_benchmarks"
        echo "  polynomial_benchmarks, parsing_benchmarks"
        exit 1
    fi

    print_step "Running benchmark group: $group"
    cargo bench -p mathhook-benchmarks --bench "$group"
}

run_ci_mode() {
    print_step "Running CI benchmark mode..."
    print_warn "This will fail if regressions exceed 10%"

    # Run benchmarks and compare against main baseline
    local result=0
    cargo bench -p mathhook-benchmarks --benches -- --baseline main 2>&1 | tee /tmp/bench_output.txt || result=$?

    # Check for significant regressions
    if grep -q "Performance has regressed" /tmp/bench_output.txt; then
        # Check if regression is more than 10%
        if grep -E "regressed.*\[.*(1[0-9]|[2-9][0-9])\..*%\]" /tmp/bench_output.txt; then
            print_error "Significant performance regression detected (>10%)"
            exit 1
        fi
    fi

    print_step "CI benchmarks passed"
}

clean_artifacts() {
    print_step "Cleaning benchmark artifacts..."

    # Clean Criterion target directory
    rm -rf "$PROJECT_ROOT/target/criterion"

    # Clean results directory (gitignored anyway)
    rm -rf "$BENCH_DIR/results"/*

    print_step "Benchmark artifacts cleaned"
}

show_status() {
    print_header

    echo "Benchmark Infrastructure Status:"
    echo ""

    echo "Rust Benchmarks:"
    ls -la "$BENCH_DIR/benches/"*.rs 2>/dev/null | wc -l | xargs echo "  Files:"

    echo ""
    echo "Python Benchmarks:"
    if [ -d "$BENCH_DIR/public/python" ]; then
        ls -la "$BENCH_DIR/public/python/"*.py 2>/dev/null | wc -l | xargs echo "  Files:"
    else
        echo "  Not configured"
    fi

    echo ""
    echo "Node.js Benchmarks:"
    if [ -d "$BENCH_DIR/public/node" ]; then
        ls -la "$BENCH_DIR/public/node/"*.js 2>/dev/null | wc -l | xargs echo "  Files:"
    else
        echo "  Not configured"
    fi

    echo ""
    echo "Comparison Benchmarks (gitignored):"
    if [ -d "$BENCH_DIR/comparison" ]; then
        echo "  Directory exists (local only, not in git)"
        ls -la "$BENCH_DIR/comparison/" 2>/dev/null | tail -n +4
    else
        echo "  Not configured"
    fi

    echo ""
    echo "Saved Baselines:"
    if [ -d "$PROJECT_ROOT/target/criterion" ]; then
        ls -d "$PROJECT_ROOT/target/criterion"/*/ 2>/dev/null | xargs -I {} basename {} | head -10
    else
        echo "  No baselines saved yet"
    fi
}

# Main command handler
main() {
    cd "$PROJECT_ROOT"

    local command="${1:-run}"
    shift || true

    case "$command" in
        run)
            print_header
            run_rust_benchmarks "" "$@"
            ;;
        quick)
            print_header
            run_rust_quick
            ;;
        save)
            print_header
            save_baseline "$1"
            ;;
        compare)
            print_header
            compare_baseline "$1"
            ;;
        rust)
            print_header
            run_rust_benchmarks "$1" "${@:2}"
            ;;
        python)
            print_header
            run_python_benchmarks
            ;;
        node)
            print_header
            run_node_benchmarks
            ;;
        group)
            print_header
            run_group "$1"
            ;;
        ci)
            print_header
            run_ci_mode
            ;;
        clean)
            print_header
            clean_artifacts
            ;;
        status)
            show_status
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            echo "Run './scripts/bench.sh help' for usage information"
            exit 1
            ;;
    esac
}

main "$@"
