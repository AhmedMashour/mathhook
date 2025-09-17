#!/usr/bin/env bash
#
# MathHook Local Publishing Script
# Publishes packages directly from your machine (no CI)
#
# Usage:
#   ./scripts/publish-local.sh [--dry-run] [--crates] [--pypi] [--npm] [--all]
#
# Required environment variables (or pass as arguments):
#   CARGO_REGISTRY_TOKEN - crates.io API token
#   PYPI_API_TOKEN       - PyPI API token
#   NPM_TOKEN            - npm automation token
#
# Examples:
#   ./scripts/publish-local.sh --all                    # Publish to all registries
#   ./scripts/publish-local.sh --crates                 # Publish only to crates.io
#   ./scripts/publish-local.sh --dry-run --all          # Dry run all
#   ./scripts/publish-local.sh --skip-validation --all  # Skip validation checks

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Defaults
DRY_RUN=false
SKIP_VALIDATION=false
PUBLISH_CRATES=false
PUBLISH_PYPI=false
PUBLISH_NPM=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-validation)
            SKIP_VALIDATION=true
            shift
            ;;
        --crates)
            PUBLISH_CRATES=true
            shift
            ;;
        --pypi)
            PUBLISH_PYPI=true
            shift
            ;;
        --npm)
            PUBLISH_NPM=true
            shift
            ;;
        --all)
            PUBLISH_CRATES=true
            PUBLISH_PYPI=true
            PUBLISH_NPM=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--dry-run] [--skip-validation] [--crates] [--pypi] [--npm] [--all]"
            echo ""
            echo "Options:"
            echo "  --dry-run          Don't actually publish, just validate"
            echo "  --skip-validation  Skip fmt, clippy, and test checks"
            echo "  --crates           Publish to crates.io"
            echo "  --pypi             Publish to PyPI"
            echo "  --npm              Publish to npm"
            echo "  --all              Publish to all registries"
            echo ""
            echo "Environment variables:"
            echo "  CARGO_REGISTRY_TOKEN - crates.io API token"
            echo "  PYPI_API_TOKEN       - PyPI API token (or use ~/.pypirc)"
            echo "  NPM_TOKEN            - npm automation token"
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check at least one target is selected
if ! $PUBLISH_CRATES && ! $PUBLISH_PYPI && ! $PUBLISH_NPM; then
    log_error "No publish target specified. Use --crates, --pypi, --npm, or --all"
    exit 1
fi

cd "$PROJECT_ROOT"

# Get current version
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
log_info "Publishing MathHook v$VERSION"

if $DRY_RUN; then
    log_warn "DRY RUN MODE - No actual publishing will occur"
fi

echo ""

# ============================================================================
# Validate before publishing
# ============================================================================
if $SKIP_VALIDATION; then
    log_warn "Skipping validation (--skip-validation flag set)"
else
    log_info "Running validation..."
    cargo fmt --all -- --check || {
        log_error "Formatting check failed. Run 'cargo fmt' first."
        exit 1
    }
    cargo clippy --all-targets --all-features -- -D warnings || {
        log_error "Clippy check failed."
        exit 1
    }
    cargo test --all-features --workspace || {
        log_error "Tests failed."
        exit 1
    }
    log_success "All validations passed!"
fi
echo ""

# ============================================================================
# Publish to crates.io
# ============================================================================
if $PUBLISH_CRATES; then
    log_info "Publishing to crates.io..."

    if [ -z "${CARGO_REGISTRY_TOKEN:-}" ]; then
        log_warn "CARGO_REGISTRY_TOKEN not set. Using stored credentials (~/.cargo/credentials.toml)"
    fi

    # Order matters: dependencies first
    CRATES_ORDER=("mathhook-macros" "mathhook-core" "mathhook")

    # Build publish flags
    PUBLISH_FLAGS="--allow-dirty"
    if $SKIP_VALIDATION; then
        PUBLISH_FLAGS="$PUBLISH_FLAGS --no-verify"
    fi

    for crate in "${CRATES_ORDER[@]}"; do
        log_info "Publishing $crate..."
        cd "$PROJECT_ROOT/crates/$crate"

        if $DRY_RUN; then
            cargo publish --dry-run $PUBLISH_FLAGS ${CARGO_REGISTRY_TOKEN:+--token "$CARGO_REGISTRY_TOKEN"}
            log_info "[DRY RUN] Would publish $crate"
        else
            # Try to publish, continue if already published
            if cargo publish $PUBLISH_FLAGS ${CARGO_REGISTRY_TOKEN:+--token "$CARGO_REGISTRY_TOKEN"} 2>&1; then
                log_success "$crate published!"
            else
                log_warn "$crate may already be published (continuing...)"
            fi
            # Wait for index update
            if [[ "$crate" != "mathhook" ]]; then
                log_info "Waiting 30s for crates.io index update..."
                sleep 30
            fi
        fi
    done

    cd "$PROJECT_ROOT"
    log_success "crates.io publishing complete!"
    echo ""
fi

# ============================================================================
# Publish to PyPI (cross-platform wheels)
# ============================================================================
if $PUBLISH_PYPI; then
    log_info "Publishing to PyPI (cross-platform)..."

    # Check for maturin
    if ! command -v maturin &> /dev/null; then
        log_error "maturin not found. Install with: pip install maturin"
        exit 1
    fi

    # Check for zig (needed for cross-compilation)
    if ! command -v zig &> /dev/null; then
        log_warn "zig not found. Install with: brew install zig"
        log_warn "Without zig, only native platform wheel will be built"
        HAS_ZIG=false
    else
        HAS_ZIG=true
    fi

    cd "$PROJECT_ROOT/crates/mathhook-python"

    # Clean old wheels
    rm -rf "$PROJECT_ROOT/target/wheels/"*.whl 2>/dev/null || true

    # Define targets
    PYPI_TARGETS=()

    # Native target (always build)
    if [[ "$(uname -m)" == "arm64" ]]; then
        PYPI_TARGETS+=("aarch64-apple-darwin")
    else
        PYPI_TARGETS+=("x86_64-apple-darwin")
    fi

    # Cross-compilation targets (if zig available)
    if $HAS_ZIG; then
        # Add the other macOS arch
        if [[ "$(uname -m)" == "arm64" ]]; then
            PYPI_TARGETS+=("x86_64-apple-darwin")
        else
            PYPI_TARGETS+=("aarch64-apple-darwin")
        fi
        # Linux targets
        PYPI_TARGETS+=("x86_64-unknown-linux-gnu")
        PYPI_TARGETS+=("aarch64-unknown-linux-gnu")
    fi

    # Ensure rust targets are installed
    log_info "Ensuring rust targets are installed..."
    for target in "${PYPI_TARGETS[@]}"; do
        rustup target add "$target" 2>/dev/null || true
    done

    # Build wheels for each target
    for target in "${PYPI_TARGETS[@]}"; do
        log_info "Building wheel for $target..."

        # Use zig for Linux cross-compilation
        if [[ "$target" == *"linux"* ]] && $HAS_ZIG; then
            maturin build --release --target "$target" --zig || {
                log_warn "Failed to build for $target (continuing...)"
            }
        else
            maturin build --release --target "$target" || {
                log_warn "Failed to build for $target (continuing...)"
            }
        fi
    done

    # List built wheels
    log_info "Built wheels:"
    ls -la "$PROJECT_ROOT/target/wheels/"*.whl 2>/dev/null || true

    if $DRY_RUN; then
        log_info "[DRY RUN] Would upload wheels to PyPI"
    else
        log_info "Uploading wheels to PyPI..."

        if [ -n "${PYPI_API_TOKEN:-}" ]; then
            MATURIN_PYPI_TOKEN="$PYPI_API_TOKEN" maturin upload --skip-existing "$PROJECT_ROOT/target/wheels/"*.whl
        else
            maturin upload --skip-existing "$PROJECT_ROOT/target/wheels/"*.whl
        fi

        log_success "PyPI publishing complete!"
    fi

    cd "$PROJECT_ROOT"
    echo ""
fi

# ============================================================================
# Publish to npm (cross-platform binaries)
# ============================================================================
if $PUBLISH_NPM; then
    log_info "Publishing to npm (cross-platform)..."

    # Check for npm
    if ! command -v npm &> /dev/null; then
        log_error "npm not found. Install Node.js first."
        exit 1
    fi

    # Check for zig (needed for cross-compilation)
    if ! command -v zig &> /dev/null; then
        log_warn "zig not found. Install with: brew install zig"
        log_warn "Without zig, only native platform binary will be built"
        HAS_ZIG=false
    else
        HAS_ZIG=true
    fi

    cd "$PROJECT_ROOT/crates/mathhook-node"

    # Install dependencies
    npm install

    # Clean old binaries
    rm -f *.node 2>/dev/null || true

    # Define targets
    NPM_TARGETS=()

    # Native target (always build)
    if [[ "$(uname -m)" == "arm64" ]]; then
        NPM_TARGETS+=("aarch64-apple-darwin")
    else
        NPM_TARGETS+=("x86_64-apple-darwin")
    fi

    # Cross-compilation targets (if zig available)
    if $HAS_ZIG; then
        # Add the other macOS arch
        if [[ "$(uname -m)" == "arm64" ]]; then
            NPM_TARGETS+=("x86_64-apple-darwin")
        else
            NPM_TARGETS+=("aarch64-apple-darwin")
        fi
        # Linux targets
        NPM_TARGETS+=("x86_64-unknown-linux-gnu")
        NPM_TARGETS+=("aarch64-unknown-linux-gnu")
    fi

    # Ensure rust targets are installed
    log_info "Ensuring rust targets are installed..."
    for target in "${NPM_TARGETS[@]}"; do
        rustup target add "$target" 2>/dev/null || true
    done

    # Build for each target
    for target in "${NPM_TARGETS[@]}"; do
        log_info "Building npm binary for $target..."

        # Use zig for Linux cross-compilation
        if [[ "$target" == *"linux"* ]] && $HAS_ZIG; then
            npx napi build --platform --release --target "$target" --zig || {
                log_warn "Failed to build for $target (continuing...)"
            }
        else
            npx napi build --platform --release --target "$target" || {
                log_warn "Failed to build for $target (continuing...)"
            }
        fi
    done

    # List built binaries
    log_info "Built binaries:"
    ls -la *.node 2>/dev/null || true

    if $DRY_RUN; then
        log_info "[DRY RUN] Would publish to npm"
        npm pack --dry-run
    else
        # Set npm token if provided
        if [ -n "${NPM_TOKEN:-}" ]; then
            echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > .npmrc
        fi

        # Publish
        npm publish --access public || {
            log_warn "npm publish may have failed (version might already exist)"
        }

        # Clean up .npmrc if we created it
        if [ -n "${NPM_TOKEN:-}" ]; then
            rm -f .npmrc
        fi

        log_success "npm publishing complete!"
    fi

    cd "$PROJECT_ROOT"
    echo ""
fi

# ============================================================================
# Summary
# ============================================================================
echo ""
log_success "=========================================="
log_success "Publishing complete for MathHook v$VERSION"
log_success "=========================================="
echo ""

if ! $DRY_RUN; then
    log_info "Verify your packages:"
    if $PUBLISH_CRATES; then
        log_info "  crates.io: https://crates.io/crates/mathhook"
    fi
    if $PUBLISH_PYPI; then
        log_info "  PyPI: https://pypi.org/project/mathhook/"
    fi
    if $PUBLISH_NPM; then
        log_info "  npm: https://www.npmjs.com/package/mathhook-node"
    fi
fi
