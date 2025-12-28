#!/usr/bin/env bash
# Generate binding code using mathhook-binding-codegen
# Usage: generate-bindings.sh [python|node|all]
# Must run BEFORE building Python/Node bindings
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# shellcheck source=lib.sh
source "${SCRIPT_DIR}/lib.sh"

TARGET="${1:-all}"

cd "$PROJECT_ROOT"

log_info "Generating bindings with mathhook-binding-codegen..."

case "$TARGET" in
    python)
        log_info "Generating Python bindings..."
        cargo run -p mathhook-binding-codegen -- generate --target python
        log_success "Python bindings generated"
        ;;
    node)
        log_info "Generating Node.js bindings..."
        cargo run -p mathhook-binding-codegen -- generate --target node
        log_success "Node.js bindings generated"
        ;;
    all)
        log_info "Generating Python bindings..."
        cargo run -p mathhook-binding-codegen -- generate --target python
        log_success "Python bindings generated"

        log_info "Generating Node.js bindings..."
        cargo run -p mathhook-binding-codegen -- generate --target node
        log_success "Node.js bindings generated"
        ;;
    *)
        log_error "Unknown target: $TARGET"
        log_error "Usage: generate-bindings.sh [python|node|all]"
        exit 1
        ;;
esac

log_success "Binding generation complete for target: $TARGET"
