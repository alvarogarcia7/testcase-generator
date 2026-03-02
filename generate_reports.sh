#!/usr/bin/env bash
set -e

# Generate PDF reports for verifier scenarios

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "======================================================================="
echo "Test Verifier Report Generator"
echo "======================================================================="

# Create output directory
OUTPUT_DIR="reports/verifier_scenarios"
mkdir -p "$OUTPUT_DIR"

echo ""
echo "Output directory: $OUTPUT_DIR"

# Build binaries
echo ""
echo "=== Building binaries ==="
cargo build --release --bin verifier

if [ $? -ne 0 ]; then
    echo "✗ Failed to build binaries. Exiting."
    exit 1
fi

echo "✓ Binaries built successfully"

# Define scenarios
declare -a TEST_CASE_IDS=(
    "TEST_SUCCESS_001"
    "TEST_FAILED_FIRST_001"
    "TEST_FAILED_INTERMEDIATE_001"
    "TEST_FAILED_LAST_001"
    "TEST_INTERRUPTED_001"
    "TEST_MULTI_SEQ_001"
    "TEST_HOOK_SCRIPT_START_001"
)

declare -a EXECUTION_LOGS=(
    "testcases/verifier_scenarios/successful/TEST_SUCCESS_001_execution_log.json"
    "testcases/verifier_scenarios/failed_first/TEST_FAILED_FIRST_001_execution_log.json"
    "testcases/verifier_scenarios/failed_intermediate/TEST_FAILED_INTERMEDIATE_001_execution_log.json"
    "testcases/verifier_scenarios/failed_last/TEST_FAILED_LAST_001_execution_log.json"
    "testcases/verifier_scenarios/interrupted/TEST_INTERRUPTED_001_execution_log.json"
    "testcases/verifier_scenarios/multiple_sequences/TEST_MULTI_SEQ_001_execution_log.json"
    "testcases/verifier_scenarios/hooks/TEST_HOOK_SCRIPT_START_001_execution_log.json"
)

declare -a GENERATED_REPORTS=()

# Process each scenario
for i in "${!TEST_CASE_IDS[@]}"; do
    TEST_CASE_ID="${TEST_CASE_IDS[$i]}"
    EXECUTION_LOG="${EXECUTION_LOGS[$i]}"
    
    echo ""
    echo "======================================================================="
    echo "Processing: $TEST_CASE_ID"
    echo "======================================================================="
    
    # Check if execution log exists
    if [ ! -f "$EXECUTION_LOG" ]; then
        echo "⚠ Skipping $TEST_CASE_ID: execution log not found: $EXECUTION_LOG"
        continue
    fi
    
    echo "✓ Execution log exists: $EXECUTION_LOG"
    
    # Run verifier
    echo ""
    echo "=== Running verifier for $TEST_CASE_ID ==="
    
    VERIFICATION_OUTPUT="$OUTPUT_DIR/${TEST_CASE_ID}_verification.json"
    
    cargo run --release --bin verifier -- \
        --log "$EXECUTION_LOG" \
        --test-case "$TEST_CASE_ID" \
        --format json \
        --output "$VERIFICATION_OUTPUT"
    
    VERIFIER_EXIT_CODE=$?
    
    # Exit code 0 = passed, 1 = failed (expected for some scenarios)
    if [ $VERIFIER_EXIT_CODE -ne 0 ] && [ $VERIFIER_EXIT_CODE -ne 1 ]; then
        echo "✗ Verifier failed for $TEST_CASE_ID with unexpected exit code: $VERIFIER_EXIT_CODE"
        continue
    fi
    
    if [ -f "$VERIFICATION_OUTPUT" ]; then
        echo "✓ Verification report generated: $VERIFICATION_OUTPUT"
        GENERATED_REPORTS+=("$VERIFICATION_OUTPUT")
    else
        echo "✗ Verification report not created: $VERIFICATION_OUTPUT"
    fi
done

# Summary
echo ""
echo "======================================================================="
echo "Verification Reports Generated"
echo "======================================================================="
echo ""
echo "Generated ${#GENERATED_REPORTS[@]} verification reports:"
for REPORT in "${GENERATED_REPORTS[@]}"; do
    echo "  • $REPORT"
done

echo ""
echo "To generate PDF reports from these verification files, install reportlab:"
echo "  pip3 install reportlab"
echo ""
echo "Then run:"
echo "  python3 scripts/generate_verifier_reports.py"

exit 0
