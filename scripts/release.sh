#!/usr/bin/env bash
#
# MathHook Unified Release Script
#
# Usage:
#   ./scripts/release.sh patch              # Auto-detect: Docker if available, else CI
#   ./scripts/release.sh minor --docker     # Force local Docker build + publish
#   ./scripts/release.sh 0.2.0 --ci         # Force CI (just bump + push tag)
#   ./scripts/release.sh patch --dry-run    # Test without publishing
#
# Modes:
#   --docker  Build and publish locally via Docker (when CI is blocked)
#   --ci      Just bump version and push tag (let GitHub Actions build)
#   (default) Auto-detect based on Docker availability
#
# Setup:
#   cp .env.example .env   # Add tokens for --docker mode

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_step() { echo -e "${CYAN}>>>${NC} $1"; }

# ============================================================================
# Argument parsing
# ============================================================================
VERSION_ARG=""
MODE=""  # "docker", "ci", or "" (auto-detect)
DRY_RUN=false
SKIP_BUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --docker|--local)
            MODE="docker"
            shift
            ;;
        --ci|--push)
            MODE="ci"
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --skip-build)
            SKIP_BUILD=true
            shift
            ;;
        -h|--help)
            cat << 'EOF'
MathHook Release Script

Usage: ./scripts/release.sh <version> [options]

Version:
  patch         Bump patch (0.1.0 -> 0.1.1)
  minor         Bump minor (0.1.0 -> 0.2.0)
  major         Bump major (0.1.0 -> 1.0.0)
  0.2.0         Explicit version

Mode:
  --docker      Build locally via Docker, publish directly
  --ci          Just bump + push tag, let GitHub Actions build
  (default)     Auto-detect: Docker if running, else CI

Options:
  --dry-run     Build but don't publish
  --skip-build  Skip build step (use existing artifacts)

Examples:
  ./scripts/release.sh patch                 # Auto-detect mode
  ./scripts/release.sh minor --docker        # Local Docker build
  ./scripts/release.sh 0.2.0 --ci            # Push to CI
  ./scripts/release.sh patch --dry-run       # Test run
EOF
            exit 0
            ;;
        -*)
            log_error "Unknown option: $1"
            exit 1
            ;;
        *)
            VERSION_ARG="$1"
            shift
            ;;
    esac
done

cd "$PROJECT_ROOT"

# ============================================================================
# Version calculation
# ============================================================================
get_current_version() {
    grep -A10 '\[workspace\.package\]' "$PROJECT_ROOT/Cargo.toml" | \
        grep '^version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

bump_version() {
    local current=$1 bump_type=$2
    local base="${current%%-*}"
    IFS='.' read -r major minor patch <<< "$base"

    case "$bump_type" in
        major) echo "$((major + 1)).0.0" ;;
        minor) echo "$major.$((minor + 1)).0" ;;
        patch) echo "$major.$minor.$((patch + 1))" ;;
        *) echo "$bump_type" ;;
    esac
}

if [ -z "$VERSION_ARG" ]; then
    CURRENT=$(get_current_version)
    log_error "Version required. Current: v$CURRENT"
    echo "Usage: $0 <patch|minor|major|x.y.z>"
    exit 1
fi

CURRENT_VERSION=$(get_current_version)
case "$VERSION_ARG" in
    major|minor|patch) NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$VERSION_ARG") ;;
    *) NEW_VERSION="$VERSION_ARG" ;;
esac

# ============================================================================
# Mode detection
# ============================================================================
if [ -z "$MODE" ]; then
    if docker info &> /dev/null; then
        log_info "Docker detected - will build locally"
        MODE="docker"
    else
        log_info "Docker not running - will push to CI"
        MODE="ci"
    fi
fi

# ============================================================================
# Token validation (docker mode only)
# ============================================================================
if [ "$MODE" = "docker" ] && ! $DRY_RUN; then
    if [ -f "$PROJECT_ROOT/.env" ]; then
        set -a; source "$PROJECT_ROOT/.env"; set +a
    fi

    missing=()
    [ -z "${CARGO_REGISTRY_TOKEN:-}" ] && missing+=("CARGO_REGISTRY_TOKEN")
    [ -z "${PYPI_API_TOKEN:-}" ] && missing+=("PYPI_API_TOKEN")
    [ -z "${NPM_TOKEN:-}" ] && missing+=("NPM_TOKEN")

    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing tokens: ${missing[*]}"
        echo "Create .env file (see .env.example) or use --ci mode"
        exit 1
    fi
fi

# ============================================================================
# Pre-flight checks
# ============================================================================
log_step "Running pre-flight checks..."

# Check for dirty working directory
if ! git diff --quiet HEAD 2>/dev/null; then
    log_warn "Working directory has uncommitted changes"
    git status --short
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r; echo
    [[ ! $REPLY =~ ^[Yy]$ ]] && exit 1
fi

# Check if tag already exists
if git rev-parse "v$NEW_VERSION" >/dev/null 2>&1; then
    log_error "Tag v$NEW_VERSION already exists!"
    echo "Use a different version or delete the existing tag first."
    exit 1
fi

# Validate version format (basic semver check)
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    log_error "Invalid version format: $NEW_VERSION"
    echo "Expected: X.Y.Z or X.Y.Z-prerelease"
    exit 1
fi

log_success "Pre-flight checks passed"

# ============================================================================
# Confirmation
# ============================================================================
echo ""
echo "  Version: v$CURRENT_VERSION → v$NEW_VERSION"
echo "  Mode:    $MODE"
$DRY_RUN && echo "  (DRY RUN)"
echo ""

read -p "Proceed? (y/N) " -n 1 -r; echo
[[ ! $REPLY =~ ^[Yy]$ ]] && exit 0

# ============================================================================
# Update version files
# ============================================================================
log_step "Updating versions..."

sed_inplace() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "$@"
    else
        sed -i "$@"
    fi
}

# Cargo.toml
sed_inplace "/\[workspace\.package\]/,/^\[/ s/^version = \".*\"/version = \"$NEW_VERSION\"/" \
    "$PROJECT_ROOT/Cargo.toml"

# pyproject.toml
[ -f "$PROJECT_ROOT/crates/mathhook-python/pyproject.toml" ] && \
    sed_inplace "s/^version = \".*\"/version = \"$NEW_VERSION\"/" \
        "$PROJECT_ROOT/crates/mathhook-python/pyproject.toml"

# package.json (version + optionalDependencies)
if [ -f "$PROJECT_ROOT/crates/mathhook-node/package.json" ]; then
    if command -v jq &> /dev/null; then
        tmp=$(mktemp)
        # Update main version AND all optionalDependencies versions
        jq ".version = \"$NEW_VERSION\" | .optionalDependencies |= with_entries(.value = \"$NEW_VERSION\")" \
            "$PROJECT_ROOT/crates/mathhook-node/package.json" > "$tmp"
        mv "$tmp" "$PROJECT_ROOT/crates/mathhook-node/package.json"
    else
        # Update main version
        sed_inplace "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" \
            "$PROJECT_ROOT/crates/mathhook-node/package.json"
        # Update optionalDependencies (all mathhook-node-* packages)
        sed_inplace "s/\"mathhook-node-\([^\"]*\)\": \"[^\"]*\"/\"mathhook-node-\1\": \"$NEW_VERSION\"/g" \
            "$PROJECT_ROOT/crates/mathhook-node/package.json"
    fi
fi

log_success "Versions updated"

# ============================================================================
# Build & Publish (docker mode)
# ============================================================================
if [ "$MODE" = "docker" ]; then
    if ! $SKIP_BUILD; then
        log_step "Building all platforms..."
        make -C "$PROJECT_ROOT" build-all
        log_success "Build complete"
    fi

    if ! $DRY_RUN; then
        log_step "Publishing..."
        docker compose --profile publish run --rm \
            -e CARGO_REGISTRY_TOKEN="$CARGO_REGISTRY_TOKEN" \
            -e PYPI_API_TOKEN="$PYPI_API_TOKEN" \
            -e NPM_TOKEN="$NPM_TOKEN" \
            publish
        log_success "Published to all registries"
    else
        log_warn "Dry run - skipped publish"
    fi
fi

# ============================================================================
# Git commit & tag
# ============================================================================
log_step "Creating commit and tag..."

git add -A
git commit -m "release: v$NEW_VERSION

- Bump version to $NEW_VERSION
- Mode: $MODE

🤖 Generated with MathHook Release"

git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

log_success "Created tag v$NEW_VERSION"

# ============================================================================
# Push (ci mode) or finish
# ============================================================================
if [ "$MODE" = "ci" ]; then
    if ! $DRY_RUN; then
        log_step "Pushing to trigger CI..."
        git push && git push origin "v$NEW_VERSION"
        log_success "Pushed! CI will build and publish."
        echo ""
        echo "Monitor: https://github.com/AhmedMashour/mathhook-core/actions"
    else
        log_warn "Dry run - not pushing"
    fi
else
    echo ""
    log_info "To push the tag: git push && git push origin v$NEW_VERSION"
fi

# ============================================================================
# Summary
# ============================================================================
echo ""
echo "════════════════════════════════════════"
log_success "Release v$NEW_VERSION complete!"
echo "════════════════════════════════════════"

if ! $DRY_RUN && [ "$MODE" = "docker" ]; then
    echo ""
    echo "Published:"
    echo "  https://crates.io/crates/mathhook"
    echo "  https://pypi.org/project/mathhook/"
    echo "  https://www.npmjs.com/package/mathhook-node"
fi
