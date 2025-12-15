#!/usr/bin/env bash
#
# MathHook Release Script
# Usage: ./scripts/release.sh <version|major|minor|patch>
#
# Examples:
#   ./scripts/release.sh 0.1.0       # Set explicit version
#   ./scripts/release.sh patch       # 0.1.0 -> 0.1.1
#   ./scripts/release.sh minor       # 0.1.0 -> 0.2.0
#   ./scripts/release.sh major       # 0.1.0 -> 1.0.0
#   ./scripts/release.sh 0.2.0-alpha.1  # Pre-release version
#
# This script:
# 1. Updates versions in all Cargo.toml files
# 2. Updates pyproject.toml and package.json
# 3. Runs validation (fmt, clippy, tests)
# 4. Commits and tags the release
# 5. Optionally pushes to trigger CI release
#
# Required: git, cargo, jq (for JSON manipulation)

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Log functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Get current version from workspace Cargo.toml
get_current_version() {
    # Extract version from [workspace.package] section
    grep -A10 '\[workspace\.package\]' "$PROJECT_ROOT/Cargo.toml" | grep '^version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# Parse semantic version
parse_semver() {
    local version=$1
    # Handle pre-release versions like 0.1.0-alpha.1
    local base_version="${version%%-*}"
    echo "$base_version" | tr '.' ' '
}

# Bump version based on type
bump_version() {
    local current=$1
    local bump_type=$2

    # Extract base version (before any pre-release suffix)
    local base="${current%%-*}"

    read -r major minor patch <<< "$(parse_semver "$base")"

    case "$bump_type" in
        major)
            echo "$((major + 1)).0.0"
            ;;
        minor)
            echo "$major.$((minor + 1)).0"
            ;;
        patch)
            echo "$major.$minor.$((patch + 1))"
            ;;
        *)
            # If it's not a bump type, treat it as an explicit version
            echo "$bump_type"
            ;;
    esac
}

# Validate semver format
validate_version() {
    local version=$1
    # Match: 0.1.0, 1.0.0, 0.1.0-alpha.1, 1.0.0-rc.1, etc.
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+(\.[0-9]+)?)?$ ]]; then
        log_error "Invalid version format: $version"
        log_info "Expected format: MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-PRERELEASE"
        exit 1
    fi
}

# Update version in a Cargo.toml file
update_cargo_version() {
    local file=$1
    local version=$2

    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS sed
        sed -i '' "s/^version = \".*\"/version = \"$version\"/" "$file"
    else
        # Linux sed
        sed -i "s/^version = \".*\"/version = \"$version\"/" "$file"
    fi
}

# Update workspace version
update_workspace_version() {
    local version=$1
    log_info "Updating workspace Cargo.toml to version $version"

    # Update the workspace version in [workspace.package]
    local cargo_file="$PROJECT_ROOT/Cargo.toml"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "/\[workspace\.package\]/,/^\[/ s/^version = \".*\"/version = \"$version\"/" "$cargo_file"
    else
        sed -i "/\[workspace\.package\]/,/^\[/ s/^version = \".*\"/version = \"$version\"/" "$cargo_file"
    fi
}

# Update pyproject.toml version
update_pyproject_version() {
    local version=$1
    local file="$PROJECT_ROOT/crates/mathhook-python/pyproject.toml"

    if [ -f "$file" ]; then
        log_info "Updating pyproject.toml to version $version"
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s/^version = \".*\"/version = \"$version\"/" "$file"
        else
            sed -i "s/^version = \".*\"/version = \"$version\"/" "$file"
        fi
    fi
}

# Update package.json version
update_package_json_version() {
    local version=$1
    local file="$PROJECT_ROOT/crates/mathhook-node/package.json"

    if [ -f "$file" ]; then
        log_info "Updating package.json to version $version"
        # Use jq if available, otherwise use sed
        if command -v jq &> /dev/null; then
            local tmp=$(mktemp)
            jq ".version = \"$version\"" "$file" > "$tmp" && mv "$tmp" "$file"
        else
            if [[ "$OSTYPE" == "darwin"* ]]; then
                sed -i '' "s/\"version\": \".*\"/\"version\": \"$version\"/" "$file"
            else
                sed -i "s/\"version\": \".*\"/\"version\": \"$version\"/" "$file"
            fi
        fi
    fi
}

# Run validation
run_validation() {
    log_info "Running validation checks..."

    cd "$PROJECT_ROOT"

    log_info "Checking formatting..."
    cargo fmt --all -- --check || {
        log_warn "Formatting issues found. Running cargo fmt..."
        cargo fmt --all
    }

    log_info "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

    log_info "Running tests..."
    cargo test --all-features --workspace

    log_success "All validations passed!"
}

# Check git status
check_git_status() {
    cd "$PROJECT_ROOT"

    # Check if we're on a clean branch
    if ! git diff-index --quiet HEAD -- 2>/dev/null; then
        log_warn "You have uncommitted changes. They will be included in the release commit."
        read -p "Continue? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi

    # Check current branch
    local branch=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$branch" != "master" && "$branch" != "main" ]]; then
        log_warn "You're not on master/main branch (current: $branch)"
        read -p "Continue anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
}

# Create git commit and tag
create_release_commit() {
    local version=$1

    cd "$PROJECT_ROOT"

    log_info "Creating release commit..."
    git add -A
    git commit -m "release: v$version

- Update workspace version to $version
- Update Python bindings (pyproject.toml)
- Update Node.js bindings (package.json)

[skip ci]"

    log_info "Creating git tag v$version..."
    git tag -a "v$version" -m "Release v$version"

    log_success "Release commit and tag created!"
}

# Push to remote
push_release() {
    local version=$1

    log_info "Pushing to remote..."
    read -p "Push commit and tag to trigger CI release? (y/N) " -n 1 -r
    echo

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git push
        git push origin "v$version"
        log_success "Pushed! CI will now build and publish to:"
        log_info "  - crates.io (mathhook-macros, mathhook-core, mathhook)"
        log_info "  - PyPI (mathhook)"
        log_info "  - npm (mathhook-node)"
    else
        log_info "To push manually later:"
        log_info "  git push && git push origin v$version"
    fi
}

# Main function
main() {
    if [ $# -lt 1 ]; then
        echo "Usage: $0 <version|major|minor|patch>"
        echo ""
        echo "Examples:"
        echo "  $0 0.1.0       # Set explicit version"
        echo "  $0 patch       # Bump patch version (0.1.0 -> 0.1.1)"
        echo "  $0 minor       # Bump minor version (0.1.0 -> 0.2.0)"
        echo "  $0 major       # Bump major version (0.1.0 -> 1.0.0)"
        echo "  $0 0.2.0-alpha.1  # Pre-release version"
        echo ""
        echo "Current version: $(get_current_version)"
        exit 1
    fi

    local input=$1
    local current_version=$(get_current_version)
    local new_version

    # Determine new version
    case "$input" in
        major|minor|patch)
            new_version=$(bump_version "$current_version" "$input")
            ;;
        *)
            new_version="$input"
            ;;
    esac

    # Validate the version
    validate_version "$new_version"

    log_info "Current version: $current_version"
    log_info "New version: $new_version"
    echo ""

    # Confirmation
    read -p "Proceed with release v$new_version? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_info "Release cancelled."
        exit 0
    fi

    # Check git status
    check_git_status

    # Update all version files
    update_workspace_version "$new_version"
    update_pyproject_version "$new_version"
    update_package_json_version "$new_version"

    # Run validation
    run_validation

    # Create release commit and tag
    create_release_commit "$new_version"

    # Push to remote
    push_release "$new_version"

    echo ""
    log_success "Release v$new_version completed!"
    echo ""
    log_info "Next steps:"
    log_info "1. Monitor CI: https://github.com/AhmedMashour/mathhook/actions"
    log_info "2. Check packages after CI completes:"
    log_info "   - https://crates.io/crates/mathhook"
    log_info "   - https://pypi.org/project/mathhook/"
    log_info "   - https://www.npmjs.com/package/mathhook-node"
}

main "$@"
