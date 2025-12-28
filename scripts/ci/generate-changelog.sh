#!/bin/bash
# Generate changelog from git history
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

VERSION="${GITHUB_REF#refs/tags/}"

log_info "Generating changelog for $VERSION..."

# Find previous tag, safely handling regex metacharacters
PREV_TAG=""
while IFS= read -r tag; do
    if [[ "$tag" != "$VERSION" ]]; then
        PREV_TAG="$tag"
        break
    fi
done < <(git tag --sort=-v:refname 2>/dev/null || true)

if [[ -n "$PREV_TAG" ]]; then
    log_info "Generating changelog from $PREV_TAG to HEAD"
    CHANGELOG=$(git log --pretty=format:"* %s (%h)" "$PREV_TAG..HEAD" | head -50)
else
    log_info "No previous tag found, using last 20 commits"
    CHANGELOG=$(git log --pretty=format:"* %s (%h)" HEAD~20..HEAD 2>/dev/null || git log --pretty=format:"* %s (%h)")
fi

# Output for GitHub Actions
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
    {
        echo "changelog<<EOF"
        echo "$CHANGELOG"
        echo "EOF"
    } >> "$GITHUB_OUTPUT"
fi

echo "$CHANGELOG"
