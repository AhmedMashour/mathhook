#!/bin/bash

set -e

echo "========================================="
echo "Wave 4: CI Integration Verification"
echo "========================================="
echo ""

ERRORS=0

verify_file() {
    local file=$1
    if [ -f "$file" ]; then
        echo "✅ $file exists"
        return 0
    else
        echo "❌ $file missing"
        ERRORS=$((ERRORS + 1))
        return 1
    fi
}

verify_executable() {
    local file=$1
    if [ -x "$file" ]; then
        echo "✅ $file is executable"
        return 0
    else
        echo "❌ $file is not executable"
        ERRORS=$((ERRORS + 1))
        return 1
    fi
}

echo "Phase 1: GitHub Actions Workflow"
echo "---------------------------------"
verify_file ".github/workflows/benchmark.yml"
echo ""

echo "Phase 2: Comparison Scripts"
echo "---------------------------"
verify_file "scripts/compare_benchmarks.py"
verify_executable "scripts/compare_benchmarks.py"
verify_file "scripts/export_baseline.py"
verify_executable "scripts/export_baseline.py"
echo ""

echo "Phase 3: Baseline Data"
echo "----------------------"
verify_file "benchmarks/baseline.json"

if [ -f "benchmarks/baseline.json" ]; then
    BENCH_COUNT=$(python3 -c "import json; data=json.load(open('benchmarks/baseline.json')); print(len(data))")
    echo "📊 Baseline contains $BENCH_COUNT benchmarks"

    if [ "$BENCH_COUNT" -gt 0 ]; then
        echo "✅ Baseline has benchmark data"
    else
        echo "❌ Baseline is empty"
        ERRORS=$((ERRORS + 1))
    fi
fi
echo ""

echo "Phase 4: Script Functionality Testing"
echo "--------------------------------------"

echo "Testing export_baseline.py..."
if python3 scripts/export_baseline.py --input target/criterion --output /tmp/test_baseline.json 2>&1 | grep -q "Exported"; then
    echo "✅ export_baseline.py works"
    rm -f /tmp/test_baseline.json
else
    echo "❌ export_baseline.py failed"
    ERRORS=$((ERRORS + 1))
fi

echo "Testing compare_benchmarks.py..."
if python3 scripts/compare_benchmarks.py \
    --baseline benchmarks/baseline.json \
    --current target/criterion \
    --threshold 20 \
    --output /tmp/test_comparison.md 2>&1 | grep -q "Benchmark Results"; then
    echo "✅ compare_benchmarks.py works"
    rm -f /tmp/test_comparison.md /tmp/regression_detected
else
    echo "❌ compare_benchmarks.py failed"
    ERRORS=$((ERRORS + 1))
fi
echo ""

echo "Phase 5: Documentation Check"
echo "----------------------------"
verify_file ".mathhook_sessions/gtm/wave4/WAVE4_COMPLETION_REPORT.md" || echo "⚠️  Completion report not created yet (will be created at the end)"
echo ""

echo "========================================="
echo "Verification Summary"
echo "========================================="

if [ $ERRORS -eq 0 ]; then
    echo "✅ ALL CHECKS PASSED"
    echo ""
    echo "Wave 4 Components:"
    echo "  - GitHub Actions workflow: ✅"
    echo "  - Comparison scripts: ✅"
    echo "  - Baseline data: ✅"
    echo "  - Script functionality: ✅"
    echo ""
    echo "Next Steps:"
    echo "  1. Test the workflow locally (if possible)"
    echo "  2. Create a test PR to verify CI integration"
    echo "  3. Document the setup in completion report"
    exit 0
else
    echo "❌ VERIFICATION FAILED"
    echo "Errors found: $ERRORS"
    exit 1
fi
