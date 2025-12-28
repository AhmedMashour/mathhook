#!/bin/bash
# Generate benchmark dashboard
set -eo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib.sh"

ARTIFACTS_DIR="${1:-artifacts}"
OUTPUT_DIR="${2:-gh-pages}"

ensure_dir "$OUTPUT_DIR"

log_info "Generating benchmark dashboard..."

# Prepare Criterion reports if available
if [[ -d "$ARTIFACTS_DIR/criterion-reports" ]]; then
    ensure_dir criterion-reports
    cp -r "$ARTIFACTS_DIR/criterion-reports/"* criterion-reports/ 2>/dev/null || true
    log_info "Criterion reports prepared"
else
    log_warn "No Criterion reports found"
fi

# Generate dashboard using Python script if available
if [[ -f "scripts/ci/generate_dashboard.py" ]]; then
    python3 scripts/ci/generate_dashboard.py "$ARTIFACTS_DIR" "$OUTPUT_DIR" criterion-reports
    log_success "Dashboard generated using Python script"
else
    cat > "$OUTPUT_DIR/index.html" <<'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>MathHook Benchmarks</title>
    <style>
        body { font-family: sans-serif; margin: 40px; }
        h1 { color: #333; }
        .warning { color: #856404; background: #fff3cd; padding: 10px; border-radius: 4px; }
    </style>
</head>
<body>
    <h1>MathHook Benchmarks</h1>
    <div class="warning">Dashboard generator script not found. Add scripts/ci/generate_dashboard.py to enable detailed benchmarks.</div>
</body>
</html>
EOF
    gh_warning "Dashboard generator script not found"
fi

log_success "Dashboard generated in $OUTPUT_DIR"
