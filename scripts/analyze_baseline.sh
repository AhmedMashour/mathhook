#!/bin/bash
# Analyze baseline benchmark results

set -e

WAVE_DIR="/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/gtm/wave3"
OUTPUT_FILE="$WAVE_DIR/baseline_analysis.md"

echo "Analyzing baseline benchmarks..."

# Create analysis report header
cat > "$OUTPUT_FILE" <<'EOF'
# Wave 3 Baseline Performance Analysis

**Generated**: TIMESTAMP_PLACEHOLDER
**Worktree**: agent-1-performance

## Summary

This document contains analysis of all baseline benchmarks collected for Wave 3.

---

EOF

# Add actual timestamp
sed -i '' "s/TIMESTAMP_PLACEHOLDER/$(date)/" "$OUTPUT_FILE"

# Function to extract benchmark times from criterion output
extract_times() {
    local file="$1"
    local bench_name="$2"

    echo "" >> "$OUTPUT_FILE"
    echo "## Benchmark: $bench_name" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"

    if [ ! -f "$file" ]; then
        echo "**Status**: File not found" >> "$OUTPUT_FILE"
        return
    fi

    # Check for errors or timeouts
    if grep -q "ERROR:" "$file" || grep -q "TIMEOUT:" "$file"; then
        echo "**Status**: Failed or timed out" >> "$OUTPUT_FILE"
        grep -E "ERROR:|TIMEOUT:" "$file" | head -5 >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
        return
    fi

    # Extract timing information
    echo "**Status**: Completed" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"

    # Look for "time:" lines from criterion
    if grep -q "time:" "$file"; then
        echo "### Key Measurements" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
        echo '```' >> "$OUTPUT_FILE"
        grep -B 1 "time:" "$file" | grep -v "^--$" | head -50 >> "$OUTPUT_FILE"
        echo '```' >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    else
        echo "_No timing data found in output_" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi

    # Extract slowest operations (>10ms)
    if grep -E "time:.*[0-9]+\.[0-9]+ ms" "$file" | grep -qE "([1-9][0-9]|[0-9]{3,})\.[0-9]+ ms"; then
        echo "### Slow Operations (>10ms)" >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
        echo '```' >> "$OUTPUT_FILE"
        grep -B 1 "time:" "$file" | grep -E "time:.*([1-9][0-9]|[0-9]{3,})\.[0-9]+ ms" | head -20 >> "$OUTPUT_FILE"
        echo '```' >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi
}

# Process each baseline file
for baseline in "$WAVE_DIR"/baseline_*.txt; do
    if [ -f "$baseline" ]; then
        bench_name=$(basename "$baseline" .txt | sed 's/baseline_//')
        extract_times "$baseline" "$bench_name"
    fi
done

# Add optimization priorities section
cat >> "$OUTPUT_FILE" <<'EOF'

---

## Optimization Priorities

Based on the baseline analysis, the following areas should be prioritized:

1. **Operations >100ms**: Critical performance issues
2. **Operations 10-100ms**: High priority optimization targets
3. **Operations 1-10ms**: Medium priority (depends on usage frequency)
4. **Operations <1ms**: Low priority unless extremely high frequency

### Next Steps

1. Review slow operations above
2. Profile top 3-5 slowest operations with cargo flamegraph
3. Identify optimization opportunities (allocation patterns, algorithmic complexity)
4. Implement fixes in priority order
5. Verify improvements with follow-up benchmarks

EOF

echo ""
echo "Analysis complete!"
echo "Report saved to: $OUTPUT_FILE"
echo ""
echo "To view:"
echo "  cat $OUTPUT_FILE"
