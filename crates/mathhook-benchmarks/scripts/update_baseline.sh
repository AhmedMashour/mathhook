#!/bin/bash
# Unified Baseline Update Script
# Updates benchmark baselines across all platforms (Rust, Python, Node.js)
#
# Usage:
#   ./update_baseline.sh [rust|python|node|all]
#
# Examples:
#   ./update_baseline.sh rust       # Update Rust baselines only
#   ./update_baseline.sh python     # Update Python baselines only
#   ./update_baseline.sh all        # Update all platforms (default)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BENCHMARKS_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$(dirname "$BENCHMARKS_DIR")")"

# Baseline storage directories
BASELINE_DIR="${BENCHMARKS_DIR}/baselines"
RUST_BASELINE_DIR="${BASELINE_DIR}/rust"
PYTHON_BASELINE_DIR="${BASELINE_DIR}/python"
NODE_BASELINE_DIR="${BASELINE_DIR}/node"

# Ensure baseline directories exist
mkdir -p "$RUST_BASELINE_DIR"
mkdir -p "$PYTHON_BASELINE_DIR"
mkdir -p "$NODE_BASELINE_DIR"

# Function to print colored messages
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Update Rust baselines
update_rust() {
    print_info "Updating Rust benchmarks baseline..."

    cd "$PROJECT_ROOT"

    # Run benchmarks and save baseline
    if cargo bench -- --save-baseline current 2>&1 | tee "${RUST_BASELINE_DIR}/last_run.log"; then

        # Copy Criterion baseline files to our baseline directory
        if [ -d "target/criterion" ]; then
            # Archive previous baseline with timestamp
            if [ -f "${RUST_BASELINE_DIR}/current.json" ]; then
                TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
                cp "${RUST_BASELINE_DIR}/current.json" "${RUST_BASELINE_DIR}/current_${TIMESTAMP}.json"
                print_info "Archived previous Rust baseline to current_${TIMESTAMP}.json"
            fi

            # Create summary JSON from Criterion results
            # Note: Criterion stores detailed results in target/criterion/<benchmark>/*/estimates.json
            # We create a simple summary for version control
            echo "{" > "${RUST_BASELINE_DIR}/current.json"
            echo "  \"timestamp\": \"$(date -Iseconds)\"," >> "${RUST_BASELINE_DIR}/current.json"
            echo "  \"commit\": \"$(git rev-parse HEAD 2>/dev/null || echo 'unknown')\"," >> "${RUST_BASELINE_DIR}/current.json"
            echo "  \"baselines\": \"Stored in target/criterion/<benchmark>/current/\"" >> "${RUST_BASELINE_DIR}/current.json"
            echo "}" >> "${RUST_BASELINE_DIR}/current.json"

            print_success "Rust baselines updated successfully"
            print_info "Detailed Criterion baselines stored in target/criterion/"
        else
            print_warning "Criterion target directory not found"
        fi
    else
        print_error "Rust benchmarks failed to run"
        return 1
    fi
}

# Update Python baselines
update_python() {
    print_info "Updating Python benchmarks baseline..."

    cd "${BENCHMARKS_DIR}/public/python"

    # Check if Python bindings are available
    if ! python3 -c "import mathhook" 2>/dev/null; then
        print_warning "MathHook Python bindings not installed"
        print_info "Building Python bindings..."

        cd "${PROJECT_ROOT}/crates/mathhook-python"
        if command -v maturin &> /dev/null; then
            maturin develop --release
        else
            print_error "maturin not found. Install with: pip install maturin"
            return 1
        fi

        cd "${BENCHMARKS_DIR}/public/python"
    fi

    # Run Python baseline update script
    if [ -f "update_baseline.py" ]; then
        if python3 update_baseline.py; then
            # Archive previous baseline
            if [ -f "${PYTHON_BASELINE_DIR}/baseline.json" ]; then
                TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
                cp "${PYTHON_BASELINE_DIR}/baseline.json" "${PYTHON_BASELINE_DIR}/baseline_${TIMESTAMP}.json"
                print_info "Archived previous Python baseline to baseline_${TIMESTAMP}.json"
            fi

            # Copy new baseline
            if [ -f "baseline.json" ]; then
                cp baseline.json "${PYTHON_BASELINE_DIR}/baseline.json"
                print_success "Python baselines updated successfully"
            fi
        else
            print_error "Python baseline update failed"
            return 1
        fi
    else
        print_warning "update_baseline.py not found in public/python/"
        print_info "Running benchmarks manually..."

        if python3 bench_mathhook.py --json > "${PYTHON_BASELINE_DIR}/baseline.json"; then
            print_success "Python baselines updated successfully"
        else
            print_error "Python benchmarks failed"
            return 1
        fi
    fi
}

# Update Node.js baselines
update_node() {
    print_info "Updating Node.js benchmarks baseline..."

    cd "${BENCHMARKS_DIR}/public/node"

    # Check if Node bindings are available
    if ! node -e "require('mathhook')" 2>/dev/null; then
        print_warning "MathHook Node.js bindings not found"
        print_info "Building Node.js bindings..."

        cd "${PROJECT_ROOT}/crates/mathhook-node"
        if npm install && npm run build; then
            print_success "Node.js bindings built successfully"
        else
            print_error "Failed to build Node.js bindings"
            return 1
        fi

        cd "${BENCHMARKS_DIR}/public/node"
    fi

    # Run Node baseline update script
    if [ -f "update_baseline.js" ]; then
        if node update_baseline.js; then
            # Archive previous baseline
            if [ -f "${NODE_BASELINE_DIR}/baseline.json" ]; then
                TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
                cp "${NODE_BASELINE_DIR}/baseline.json" "${NODE_BASELINE_DIR}/baseline_${TIMESTAMP}.json"
                print_info "Archived previous Node baseline to baseline_${TIMESTAMP}.json"
            fi

            # Copy new baseline
            if [ -f "baseline.json" ]; then
                cp baseline.json "${NODE_BASELINE_DIR}/baseline.json"
                print_success "Node.js baselines updated successfully"
            fi
        else
            print_error "Node.js baseline update failed"
            return 1
        fi
    else
        print_warning "update_baseline.js not found in public/node/"
        print_info "Running benchmarks manually..."

        if node bench_mathhook.js --json > "${NODE_BASELINE_DIR}/baseline.json"; then
            print_success "Node.js baselines updated successfully"
        else
            print_error "Node.js benchmarks failed"
            return 1
        fi
    fi
}

# Main script logic
main() {
    PLATFORM="${1:-all}"

    print_info "MathHook Baseline Update Script"
    print_info "Platform: $PLATFORM"
    echo ""

    case "$PLATFORM" in
        rust)
            update_rust
            ;;
        python)
            update_python
            ;;
        node)
            update_node
            ;;
        all)
            FAILED=0

            if ! update_rust; then
                print_warning "Rust baseline update failed, continuing..."
                FAILED=$((FAILED + 1))
            fi
            echo ""

            if ! update_python; then
                print_warning "Python baseline update failed, continuing..."
                FAILED=$((FAILED + 1))
            fi
            echo ""

            if ! update_node; then
                print_warning "Node.js baseline update failed, continuing..."
                FAILED=$((FAILED + 1))
            fi

            echo ""
            if [ $FAILED -eq 0 ]; then
                print_success "All baselines updated successfully!"
            else
                print_warning "$FAILED platform(s) failed to update"
                return 1
            fi
            ;;
        *)
            print_error "Unknown platform: $PLATFORM"
            echo ""
            echo "Usage: $0 [rust|python|node|all]"
            echo ""
            echo "Options:"
            echo "  rust    - Update Rust benchmarks baseline only"
            echo "  python  - Update Python benchmarks baseline only"
            echo "  node    - Update Node.js benchmarks baseline only"
            echo "  all     - Update all platforms (default)"
            return 1
            ;;
    esac

    echo ""
    print_info "Baseline update complete"
    print_info "Baselines stored in: $BASELINE_DIR"
}

# Run main function
main "$@"
