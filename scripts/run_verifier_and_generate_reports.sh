#!/usr/bin/env bash
#
# Run verifier on all test scenarios and generate documentation reports
#
# Usage: ./scripts/run_verifier_and_generate_reports.sh
#

set -e

BUILD_VARIANT="${BUILD_VARIANT:---release}"

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1
source "$SCRIPT_DIR/lib/report_generator.sh" || exit 1

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
cargo build ${BUILD_VARIANT} --bin verifier

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
    
    TEST_CASE_DIR="$PROJECT_ROOT/testcases/verifier_scenarios/$SCENARIO_DIR"
    
    cargo run ${BUILD_VARIANT} --bin verifier -- \
        --log "$EXECUTION_LOG" \
        --test-case "$TEST_CASE_ID" \
        --test-case-dir "$TEST_CASE_DIR" \
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

# Generate test-plan-doc-gen reports
echo ""
echo "======================================================================="
echo "Test Plan Documentation Report Generation"
echo "======================================================================="
echo ""

# Determine test-plan-doc-gen directory
if [ -n "$TESTPLAN_DOC_GEN_DIR" ]; then
    DOC_GEN_DIR="$TESTPLAN_DOC_GEN_DIR"
else
    # Default to sibling directory
    DOC_GEN_DIR="$(cd "$PROJECT_ROOT/.." && pwd)/test-plan-doc-gen"
fi

echo "test-plan-doc-gen directory: $DOC_GEN_DIR"

# Resolve test-plan-doc-gen directory path
if [[ ! "$DOC_GEN_DIR" = /* ]]; then
    DOC_GEN_DIR="$PROJECT_ROOT/$DOC_GEN_DIR"
fi

# Check if test-plan-doc-gen directory exists
if [[ ! -d "$DOC_GEN_DIR" ]]; then
    echo "⚠ test-plan-doc-gen directory not found: $DOC_GEN_DIR"
    echo ""
    echo "Skipping test-plan-doc-gen report generation"
    echo ""
    echo "To enable test-plan-doc-gen reports, clone the repository:"
    echo "  cd $(dirname "$PROJECT_ROOT")"
    echo "  git clone <test-plan-doc-gen-repo-url> test-plan-doc-gen"
    SKIP_DOC_GEN=1
else
    SKIP_DOC_GEN=0
    
    # Check if binary exists, build if needed
    if check_test_plan_doc_gen_available "$DOC_GEN_DIR"; then
        echo "✓ test-plan-doc-gen binary found"
    else
        echo "Building test-plan-doc-gen..."
        if build_test_plan_doc_gen "$DOC_GEN_DIR"; then
            echo "✓ test-plan-doc-gen built successfully"
        else
            echo "✗ Failed to build test-plan-doc-gen"
            SKIP_DOC_GEN=1
        fi
    fi
fi

# Set TEST_PLAN_DOC_GEN environment variable for report_generator.sh
if [[ $SKIP_DOC_GEN -eq 0 ]]; then
    export TEST_PLAN_DOC_GEN=$(find_test_plan_doc_gen "$DOC_GEN_DIR")
    
    echo ""
    echo "Generating reports for all test scenarios..."
    echo ""
    
    declare -a GENERATED_REPORTS=()
    
    # Generate reports for each scenario
    for SCENARIO_ENTRY in "${SCENARIOS[@]}"; do
        IFS=':' read -r SCENARIO_DIR TEST_CASE_ID <<< "$SCENARIO_ENTRY"
        
        TEST_CASE_FILE="$PROJECT_ROOT/testcases/verifier_scenarios/$SCENARIO_DIR/${TEST_CASE_ID}.yml"
        
        # Check if test case file exists
        if [ ! -f "$TEST_CASE_FILE" ]; then
            echo "⚠ Test case file not found: $TEST_CASE_FILE"
            echo "  Skipping $TEST_CASE_ID"
            continue
        fi
        
        echo "Generating reports for: $TEST_CASE_ID"
        
        # Generate AsciiDoc report
        ASCIIDOC_OUTPUT="$OUTPUT_DIR/${TEST_CASE_ID}_test_plan.adoc"
        
        if invoke_test_plan_doc_gen \
            --test-case "$TEST_CASE_FILE" \
            --output "$ASCIIDOC_OUTPUT" \
            --format asciidoc >/dev/null 2>&1; then
            echo "  ✓ AsciiDoc report: $ASCIIDOC_OUTPUT"
            GENERATED_REPORTS+=("$ASCIIDOC_OUTPUT")
        else
            echo "  ✗ Failed to generate AsciiDoc report for $TEST_CASE_ID"
        fi
        
        # Generate Markdown report
        MARKDOWN_OUTPUT="$OUTPUT_DIR/${TEST_CASE_ID}_test_plan.md"
        
        if invoke_test_plan_doc_gen \
            --test-case "$TEST_CASE_FILE" \
            --output "$MARKDOWN_OUTPUT" \
            --format markdown >/dev/null 2>&1; then
            echo "  ✓ Markdown report: $MARKDOWN_OUTPUT"
            GENERATED_REPORTS+=("$MARKDOWN_OUTPUT")
        else
            echo "  ✗ Failed to generate Markdown report for $TEST_CASE_ID"
        fi
        
        echo ""
    done
    
    if [ ${#GENERATED_REPORTS[@]} -gt 0 ]; then
        echo "✓ Generated ${#GENERATED_REPORTS[@]} test plan documentation report(s)"
    else
        echo "✗ No test plan documentation reports were generated"
    fi
fi

# Generate additional documentation reports via generate_documentation_reports.sh
echo ""
echo "======================================================================="
echo "Additional Documentation Report Generation"
echo "======================================================================="
echo ""

# Run documentation report generation script (if available)
DOC_SCRIPT="$SCRIPT_DIR/generate_documentation_reports.sh"

if [ -f "$DOC_SCRIPT" ]; then
    echo "Running documentation report generator..."
    echo ""
    
    "$DOC_SCRIPT" \
        --output-dir "$OUTPUT_DIR" \
        --test-case-dir "$PROJECT_ROOT/testcases" \
        --test-plan-doc-gen "$DOC_GEN_DIR"
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✓ Documentation reports generated successfully"
    else
        echo ""
        echo "⚠ Documentation report generation encountered issues"
    fi
else
    echo "⚠ Documentation report generator not found: $DOC_SCRIPT"
    echo ""
    echo "To generate documentation reports, ensure the script exists at:"
    echo "  $DOC_SCRIPT"
fi

echo ""
echo "======================================================================="
echo "Complete"
echo "======================================================================="
echo ""
echo "All verification reports are in: $OUTPUT_DIR"

exit 0
