#!/usr/bin/env bash
# Example: Generate test comparison report and analyze results
#
# This example demonstrates how to:
# 1. Generate a test comparison report
# 2. Extract useful information from the report
# 3. Display insights about test organization

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

# Source logging library
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

section "Test Comparison Report Example"

# Create reports directory
mkdir -p reports

# Generate the report
log_info "Generating test comparison report..."
log_info "This will run tests on both main and split-binaries-into-crates branches"
echo ""

if ! uv run python3.14 scripts/test_comparison_report.py \
    --run-tests \
    --before-ref main \
    --after-ref split-binaries-into-crates \
    --output reports/test_comparison_report.json \
    --verbose; then
    fail "Failed to generate test comparison report"
    exit 1
fi

pass "Report generated successfully"
echo ""

# Check if jq is available
if ! command -v jq &> /dev/null; then
    log_warning "jq is not installed - install it to view formatted JSON"
    log_info "Raw report saved to: reports/test_comparison_report.json"
    exit 0
fi

section "Report Analysis"

# Extract and display key metrics
log_info "Extracting summary statistics..."
echo ""

TESTS_BEFORE=$(jq -r '.summary.tests_before' reports/test_comparison_report.json)
TESTS_AFTER=$(jq -r '.summary.tests_after' reports/test_comparison_report.json)
NEW_TESTS=$(jq -r '.summary.new_tests' reports/test_comparison_report.json)
REMOVED_TESTS=$(jq -r '.summary.removed_tests' reports/test_comparison_report.json)
DURATION_BEFORE=$(jq -r '.summary.total_duration_before_seconds' reports/test_comparison_report.json)
DURATION_AFTER=$(jq -r '.summary.total_duration_after_seconds' reports/test_comparison_report.json)
DURATION_CHANGE=$(jq -r '.summary.duration_percent_change' reports/test_comparison_report.json)

echo "📊 Test Count:"
echo "  Before: $TESTS_BEFORE tests"
echo "  After:  $TESTS_AFTER tests"
echo "  New:    $NEW_TESTS tests"
echo "  Removed: $REMOVED_TESTS tests"
echo ""

echo "⏱️  Execution Time:"
printf "  Before: %.2fs\n" "$DURATION_BEFORE"
printf "  After:  %.2fs\n" "$DURATION_AFTER"
printf "  Change: %.1f%%\n" "$DURATION_CHANGE"
echo ""

# Show tests by crate
section "Tests by Crate (After Splitting)"

if jq -e '.after.tests_by_crate' reports/test_comparison_report.json > /dev/null 2>&1; then
    jq -r '.after.tests_by_crate | to_entries[] | "  \(.key): \(.value.test_count) tests"' \
        reports/test_comparison_report.json | sort
else
    log_warning "Crate information not available in report"
fi

echo ""

# Show new tests if any
if [ "$NEW_TESTS" -gt 0 ]; then
    section "New Tests"
    jq -r '.changes.new_tests[]' reports/test_comparison_report.json | head -10 | while read -r test; do
        echo "  + $test"
    done
    if [ "$NEW_TESTS" -gt 10 ]; then
        echo "  ... and $(($NEW_TESTS - 10)) more"
    fi
    echo ""
fi

# Show removed tests if any
if [ "$REMOVED_TESTS" -gt 0 ]; then
    section "Removed Tests"
    jq -r '.changes.removed_tests[]' reports/test_comparison_report.json | head -10 | while read -r test; do
        echo "  - $test"
    done
    if [ "$REMOVED_TESTS" -gt 10 ]; then
        echo "  ... and $(($REMOVED_TESTS - 10)) more"
    fi
    echo ""
fi

# Performance interpretation
section "Performance Analysis"

if (( $(echo "$DURATION_CHANGE < 0" | bc -l) )); then
    pass "Tests run faster after crate splitting (${DURATION_CHANGE}% improvement)"
elif (( $(echo "$DURATION_CHANGE > 0" | bc -l) )); then
    log_warning "Tests run slower after crate splitting (+${DURATION_CHANGE}%)"
else
    info "No significant performance change"
fi

echo ""

# Display full report location
section "Full Report"
log_info "Complete report saved to: reports/test_comparison_report.json"
log_info ""
log_info "View with:"
log_info "  cat reports/test_comparison_report.json | jq ."
log_info ""
log_info "Query examples:"
log_info "  # Summary only"
log_info "  jq '.summary' reports/test_comparison_report.json"
log_info ""
log_info "  # Tests in specific crate"
log_info "  jq '.after.tests_by_crate[\"testcase-models\"]' reports/test_comparison_report.json"
log_info ""
log_info "  # All test names"
log_info "  jq '.after.tests[].name' reports/test_comparison_report.json"

section "Done"
pass "Example completed successfully"
