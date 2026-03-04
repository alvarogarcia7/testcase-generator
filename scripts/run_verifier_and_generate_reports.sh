#!/usr/bin/env bash
#
# Run verifier on all test scenarios and generate PDF reports
#
# Usage: ./scripts/run_verifier_and_generate_reports.sh
#

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "======================================================================="
echo "Test Verifier Report Generator"
echo "======================================================================="
echo ""
echo "Project root: $PROJECT_ROOT"

# Create output directory
OUTPUT_DIR="$PROJECT_ROOT/reports/verifier_scenarios"
mkdir -p "$OUTPUT_DIR"
echo "Output directory: $OUTPUT_DIR"

# Build verifier binary
echo ""
echo "=== Building verifier binary ==="
cd "$PROJECT_ROOT"
cargo build --release --bin verifier

if [ $? -ne 0 ]; then
    echo "✗ Failed to build verifier binary"
    exit 1
fi

echo "✓ Verifier binary built successfully"

# Define test scenarios
declare -a SCENARIOS=(
    "successful:TEST_SUCCESS_001"
    "failed_first:TEST_FAILED_FIRST_001"
    "failed_intermediate:TEST_FAILED_INTERMEDIATE_001"
    "failed_last:TEST_FAILED_LAST_001"
    "interrupted:TEST_INTERRUPTED_001"
    "multiple_sequences:TEST_MULTI_SEQ_001"
    "hooks:TEST_HOOK_SCRIPT_START_001"
)

declare -a VERIFICATION_FILES=()

# Process each scenario
for SCENARIO_ENTRY in "${SCENARIOS[@]}"; do
    IFS=':' read -r SCENARIO_DIR TEST_CASE_ID <<< "$SCENARIO_ENTRY"
    
    echo ""
    echo "======================================================================="
    echo "Processing: $TEST_CASE_ID"
    echo "======================================================================="
    
    EXECUTION_LOG="$PROJECT_ROOT/testcases/verifier_scenarios/$SCENARIO_DIR/${TEST_CASE_ID}_execution_log.json"
    VERIFICATION_OUTPUT="$OUTPUT_DIR/${TEST_CASE_ID}_verification.json"
    
    # Check if execution log exists
    if [ ! -f "$EXECUTION_LOG" ]; then
        echo "⚠ Execution log not found: $EXECUTION_LOG"
        echo "  Skipping $TEST_CASE_ID"
        continue
    fi
    
    echo "✓ Execution log found: $EXECUTION_LOG"
    
    # Run verifier
    echo ""
    echo "Running verifier..."
    
    cargo run --release --bin verifier -- \
        --log "$EXECUTION_LOG" \
        --test-case "$TEST_CASE_ID" \
        --format json \
        --output "$VERIFICATION_OUTPUT" 2>&1 | tail -20
    
    VERIFIER_EXIT=$?
    
    # Exit codes: 0 = all tests passed, 1 = some tests failed (expected for failure scenarios)
    if [ $VERIFIER_EXIT -ne 0 ] && [ $VERIFIER_EXIT -ne 1 ]; then
        echo "✗ Verifier failed with unexpected exit code: $VERIFIER_EXIT"
        continue
    fi
    
    if [ -f "$VERIFICATION_OUTPUT" ]; then
        echo "✓ Verification report: $VERIFICATION_OUTPUT"
        VERIFICATION_FILES+=("$VERIFICATION_OUTPUT")
    else
        echo "✗ Verification report not generated"
    fi
done

# Summary of verification reports
echo ""
echo "======================================================================="
echo "Verification Reports Generated"
echo "======================================================================="
echo ""
echo "Generated ${#VERIFICATION_FILES[@]} verification reports:"
for VF in "${VERIFICATION_FILES[@]}"; do
    echo "  • $VF"
done

# Check for reportlab and generate PDF reports
echo ""
echo "======================================================================="
echo "PDF Report Generation"
echo "======================================================================="
echo ""

uv sync

source .venv/bin/activate

if command -v python3 >/dev/null 2>&1; then
    echo "Python 3 found. Checking for reportlab..."
    
    if uv run python3 -c "import reportlab" 2>/dev/null; then
        echo "✓ reportlab is installed"
        echo ""
        echo "Generating PDF reports..."
        
        cd "$PROJECT_ROOT"
        uv run python3 scripts/generate_verifier_reports.py
        
        if [ $? -eq 0 ]; then
            echo ""
            echo "✓ PDF reports generated successfully"
            echo ""
            echo "Generated PDF reports:"
            for VF in "${VERIFICATION_FILES[@]}"; do
                PDF_REPORT="${VF%.json}_report.pdf"
                PDF_REPORT="${PDF_REPORT/_verification_report.pdf/_report.pdf}"
                if [ -f "$PDF_REPORT" ]; then
                    echo "  • $PDF_REPORT"
                fi
            done
        else
            echo "✗ PDF report generation failed"
        fi
    else
        echo "⚠ reportlab not installed"
        echo ""
        echo "To generate PDF reports, install reportlab:"
        echo "  pip3 install reportlab"
        echo ""
        echo "Then run:"
        echo "  uv run python3 scripts/generate_verifier_reports.py"
    fi
else
    echo "⚠ Python 3 not found"
    echo ""
    echo "PDF report generation requires Python 3 and reportlab."
fi

echo ""
echo "======================================================================="
echo "Complete"
echo "======================================================================="
echo ""
echo "All verification reports are in: $OUTPUT_DIR"

exit 0
