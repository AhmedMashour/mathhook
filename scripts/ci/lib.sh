#!/bin/bash
# Shared library for CI scripts
# Source this file: source "$(dirname "$0")/lib.sh"

set -eo pipefail

# Logging functions with consistent formatting
log_info() {
    echo ">>> $1"
}

log_error() {
    echo "ERROR: $1" >&2
}

log_warn() {
    echo "WARNING: $1" >&2
}

log_success() {
    echo "✅ $1"
}

# GitHub Actions specific logging
gh_error() {
    echo "::error::$1"
}

gh_warning() {
    echo "::warning::$1"
}

gh_notice() {
    echo "::notice::$1"
}

gh_group_start() {
    echo "::group::$1"
}

gh_group_end() {
    echo "::endgroup::"
}

# Write to GitHub Step Summary if available
gh_summary() {
    if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
        echo "$1" >> "$GITHUB_STEP_SUMMARY"
    fi
}

# Get version from Cargo.toml
get_cargo_version() {
    local cargo_toml="${1:-Cargo.toml}"
    if [[ -f "$cargo_toml" ]]; then
        grep '^version' "$cargo_toml" | head -1 | sed 's/.*"\(.*\)".*/\1/'
    else
        echo ""
    fi
}

# Get version from package.json
get_npm_version() {
    local package_json="${1:-package.json}"
    if [[ -f "$package_json" ]]; then
        jq -r '.version // empty' "$package_json" 2>/dev/null || echo ""
    else
        echo ""
    fi
}

# Get version from pyproject.toml
get_python_version() {
    local pyproject="${1:-pyproject.toml}"
    if [[ -f "$pyproject" ]]; then
        grep '^version' "$pyproject" | head -1 | sed 's/.*"\(.*\)".*/\1/'
    else
        echo ""
    fi
}

# Validate that a command exists
require_cmd() {
    local cmd="$1"
    if ! command -v "$cmd" &>/dev/null; then
        log_error "Required command not found: $cmd"
        exit 1
    fi
}

# Validate that an environment variable is set
require_env() {
    local var="$1"
    if [[ -z "${!var:-}" ]]; then
        log_error "Required environment variable not set: $var"
        exit 1
    fi
}

# Create directory if it doesn't exist
ensure_dir() {
    local dir="$1"
    if [[ ! -d "$dir" ]]; then
        mkdir -p "$dir"
    fi
}

# Safe temporary file creation
make_temp_file() {
    mktemp "${TMPDIR:-/tmp}/mathhook.XXXXXX"
}

# Safe temporary directory creation
make_temp_dir() {
    mktemp -d "${TMPDIR:-/tmp}/mathhook.XXXXXX"
}

# Run command with timeout (requires gtimeout on macOS, timeout on Linux)
run_with_timeout() {
    local timeout_sec="$1"
    shift
    if command -v gtimeout &>/dev/null; then
        gtimeout "${timeout_sec}s" "$@"
    elif command -v timeout &>/dev/null; then
        timeout "${timeout_sec}s" "$@"
    else
        "$@"
    fi
}

# Retry a command with exponential backoff
# Usage: retry_with_backoff <max_attempts> <initial_delay_sec> command...
retry_with_backoff() {
    local max_attempts="$1"
    local delay="$2"
    shift 2

    local attempt=1
    while true; do
        if "$@"; then
            return 0
        fi

        if [[ $attempt -ge $max_attempts ]]; then
            log_error "Command failed after $max_attempts attempts: $*"
            return 1
        fi

        log_warn "Attempt $attempt failed, retrying in ${delay}s..."
        sleep "$delay"
        delay=$((delay * 2))
        attempt=$((attempt + 1))
    done
}

# Poll until condition is true or timeout
# Usage: poll_until <timeout_sec> <interval_sec> command...
poll_until() {
    local timeout="$1"
    local interval="$2"
    shift 2

    local elapsed=0
    while [[ $elapsed -lt $timeout ]]; do
        if "$@" 2>/dev/null; then
            return 0
        fi
        sleep "$interval"
        elapsed=$((elapsed + interval))
    done
    return 1
}

# Check if running in GitHub Actions
is_github_actions() {
    [[ -n "${GITHUB_ACTIONS:-}" ]]
}

# Check if running in CI (any CI system)
is_ci() {
    [[ -n "${CI:-}" ]] || is_github_actions
}

# Verify package is available on npm registry
check_npm_package() {
    local package="$1"
    local version="$2"
    npm view "${package}@${version}" version &>/dev/null
}

# Verify package is available on PyPI
check_pypi_package() {
    local package="$1"
    local version="$2"
    curl -sf "https://pypi.org/pypi/${package}/${version}/json" &>/dev/null
}

# Verify crate is available on crates.io
check_crate() {
    local crate="$1"
    local version="$2"
    curl -sf "https://crates.io/api/v1/crates/${crate}/${version}" &>/dev/null
}
