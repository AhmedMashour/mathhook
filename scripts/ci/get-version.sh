#!/bin/bash
# Get version from various package files
# Usage: get-version.sh [cargo|npm|python|workspace] [path]
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

TYPE="${1:-workspace}"
PATH_ARG="${2:-.}"

case "$TYPE" in
    cargo)
        get_cargo_version "$PATH_ARG/Cargo.toml"
        ;;
    npm)
        get_npm_version "$PATH_ARG/package.json"
        ;;
    python)
        get_python_version "$PATH_ARG/pyproject.toml"
        ;;
    workspace)
        # Get workspace version from root Cargo.toml
        if [[ -f "$PATH_ARG/Cargo.toml" ]]; then
            grep -A5 '\[workspace.package\]' "$PATH_ARG/Cargo.toml" | grep '^version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
        else
            log_error "Cargo.toml not found at $PATH_ARG"
            exit 1
        fi
        ;;
    *)
        log_error "Unknown type: $TYPE. Use: cargo, npm, python, or workspace"
        exit 1
        ;;
esac
