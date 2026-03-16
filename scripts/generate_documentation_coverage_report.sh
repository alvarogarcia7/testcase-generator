#!/usr/bin/env bash
#
# generate_documentation_coverage_report.sh - Generate code coverage report for documentation generation
#
# This script runs cargo-tarpaulin across all document generation code paths
# exercised by the sample test cases, generating a coverage report showing
# which functions and branches in the template rendering and document generation
# modules were executed.
#
# Usage: ./scripts/generate_documentation_coverage_report.sh [OPTIONS]
#
# Options:
#   --output-dir DIR         Output directory for coverage reports (default: reports/coverage/documentation)
#   --html                   Generate HTML coverage report
#   --logs-dir DIR           Directory containing execution logs (default: testcases/verifier_scenarios)
#   --test-case-dir DIR      Directory containing test case YAML files (default: testcases)
#   --help                   Show this help message
#
# Output:
#   - coverage.json          Coverage data in JSON format
#   - coverage.txt           Coverage summary in text format
#   - coverage/              HTML coverage report (if --html is specified)
#

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source required libraries
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Default configuration
OUTPUT_DIR="$PROJECT_ROOT/reports/coverage/documentation"
LOGS_DIR="$PROJECT_ROOT/testcases/verifier_scenarios"
TEST_CASE_DIR="$PROJECT_ROOT/testcases"
GENERATE_HTML=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --html)
            GENERATE_HTML=true
            shift
            ;;
        --logs-dir)
            LOGS_DIR="$2"
            shift 2
            ;;
        --test-case-dir)
            TEST_CASE_DIR="$2"
            shift 2
            ;;
        --help)
            head -n 25 "$0" | tail -n +2 | sed 's/^# //'
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Display configuration
section "Documentation Generation Coverage Report"
log_info "Configuration:"
log_info "  Output directory: $OUTPUT_DIR"
log_info "  Logs directory: $LOGS_DIR"
log_info "  Test case directory: $TEST_CASE_DIR"
log_info "  Generate HTML: $GENERATE_HTML"
echo ""

# Create output directories
mkdir -p "$OUTPUT_DIR"
log_verbose "Created output directory structure"

# ============================================================================
# Step 1: Check for cargo-tarpaulin
# ============================================================================

section "Step 1: Check Coverage Tool"

if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
    log_warning "cargo-tarpaulin not found, attempting to install..."
    
    if cargo install cargo-tarpaulin 2>&1 | while IFS= read -r line; do
        log_verbose "$line"
    done; then
        pass "cargo-tarpaulin installed successfully"
    else
        fail "Failed to install cargo-tarpaulin"
        log_error "Please install cargo-tarpaulin manually:"
        log_error "  cargo install cargo-tarpaulin"
        exit 1
    fi
else
    pass "cargo-tarpaulin is available"
    log_info "Version: $(cargo-tarpaulin --version)"
fi

# ============================================================================
# Step 2: Build verifier binary for coverage instrumentation
# ============================================================================

section "Step 2: Build Verifier Binary"

log_info "Building verifier binary..."
cd "$PROJECT_ROOT"

if cargo build --bin verifier 2>&1 | while IFS= read -r line; do
    log_verbose "$line"
done; then
    pass "Verifier binary built successfully"
else
    fail "Failed to build verifier binary"
    exit 1
fi

VERIFIER_BIN="$PROJECT_ROOT/target/debug/verifier"

if [[ ! -f "$VERIFIER_BIN" ]]; then
    fail "Verifier binary not found: $VERIFIER_BIN"
    exit 1
fi

# ============================================================================
# Step 3: Build test-plan-documentation-generator-compat binary
# ============================================================================

section "Step 3: Build test-plan-documentation-generator-compat Binary"

log_info "Building test-plan-documentation-generator-compat binary..."

if cargo build --bin test-plan-documentation-generator-compat 2>&1 | while IFS= read -r line; do
    log_verbose "$line"
done; then
    pass "test-plan-documentation-generator-compat binary built successfully"
else
    fail "Failed to build test-plan-documentation-generator-compat binary"
    exit 1
fi

COMPAT_BIN="$PROJECT_ROOT/target/debug/test-plan-documentation-generator-compat"

if [[ ! -f "$COMPAT_BIN" ]]; then
    fail "test-plan-documentation-generator-compat binary not found: $COMPAT_BIN"
    exit 1
fi

# ============================================================================
# Step 4: Run documentation generation under tarpaulin instrumentation
# ============================================================================

section "Step 4: Run Documentation Generation Under Coverage"

log_info "Running documentation generation with coverage instrumentation..."

# Create temporary directory for coverage data
COVERAGE_DATA_DIR="$OUTPUT_DIR/coverage_data"
mkdir -p "$COVERAGE_DATA_DIR"

# Define modules to track for coverage
# Focus on document generation and template rendering modules
INCLUDE_MODULES="src/lib.rs,src/verification.rs,src/verification_templates.rs,src/parser.rs,src/models.rs,src/bin/verifier.rs,src/bin/test-plan-documentation-generator-compat.rs"

log_info "Tracking coverage for modules:"
log_info "  - src/lib.rs (library exports)"
log_info "  - src/verification.rs (verification and report generation)"
log_info "  - src/verification_templates.rs (template rendering)"
log_info "  - src/parser.rs (YAML parsing)"
log_info "  - src/models.rs (data models)"
log_info "  - src/bin/verifier.rs (verifier binary)"
log_info "  - src/bin/test-plan-documentation-generator-compat.rs (documentation generator compatibility)"
echo ""

# Build tarpaulin command
TARPAULIN_CMD=(
    cargo tarpaulin
    --out Json
    --out Stdout
    --output-dir "$OUTPUT_DIR"
    --exclude-files "src/fuzzy.rs"
    --exclude-files "src/prompts.rs"
    --exclude-files "src/main_editor.rs"
    --workspace
    --bins
    --no-fail-fast
    --timeout 300
)

# Add HTML output if requested
if [[ "$GENERATE_HTML" == true ]]; then
    TARPAULIN_CMD+=(--out Html)
    log_info "HTML coverage report will be generated"
fi

log_verbose "Running: ${TARPAULIN_CMD[*]}"
echo ""

# Run verifier on sample test cases under tarpaulin
log_info "Executing sample test case verification under coverage..."

# Define sample scenarios to test
declare -a SCENARIOS=(
    "successful:TEST_SUCCESS_001"
    "failed_first:TEST_FAILED_FIRST_001"
    "multiple_sequences:TEST_MULTI_SEQ_001"
)

# Create a test script that will be run under tarpaulin
TEST_SCRIPT="$COVERAGE_DATA_DIR/run_coverage_tests.sh"

cat > "$TEST_SCRIPT" << 'EOF'
#!/usr/bin/env bash
set -e

PROJECT_ROOT="$1"
LOGS_DIR="$2"
TEST_CASE_DIR="$3"
OUTPUT_DIR="$4"

# Process each scenario
SCENARIOS=(
    "successful:TEST_SUCCESS_001"
    "failed_first:TEST_FAILED_FIRST_001"
    "multiple_sequences:TEST_MULTI_SEQ_001"
)

for SCENARIO_ENTRY in "${SCENARIOS[@]}"; do
    IFS=':' read -r SCENARIO_DIR TEST_CASE_ID <<< "$SCENARIO_ENTRY"
    
    EXECUTION_LOG="$LOGS_DIR/$SCENARIO_DIR/${TEST_CASE_ID}_execution_log.json"
    VERIFICATION_OUTPUT="$OUTPUT_DIR/${TEST_CASE_ID}_verification.json"
    
    if [ -f "$EXECUTION_LOG" ]; then
        echo "Processing: $TEST_CASE_ID"
        
        # Run verifier
        cargo run --bin verifier -- \
            --log "$EXECUTION_LOG" \
            --test-case "$TEST_CASE_ID" \
            --format json \
            --output "$VERIFICATION_OUTPUT" >/dev/null 2>&1 || true
        
        # Run compatibility checker
        if [ -f "$VERIFICATION_OUTPUT" ]; then
            cargo run --bin test-plan-documentation-generator-compat -- \
                validate "$VERIFICATION_OUTPUT" >/dev/null 2>&1 || true
        fi
    fi
done

# Test batch validation mode
cargo run --bin test-plan-documentation-generator-compat -- \
    batch "$OUTPUT_DIR" --continue-on-error >/dev/null 2>&1 || true

echo "Coverage test execution complete"
EOF

chmod +x "$TEST_SCRIPT"

log_verbose "Created test script: $TEST_SCRIPT"

# Run the coverage collection
cd "$PROJECT_ROOT"

# Use tarpaulin to instrument and collect coverage
COVERAGE_OUTPUT="$OUTPUT_DIR/tarpaulin-report.json"

log_info "Running tarpaulin coverage collection..."

if "${TARPAULIN_CMD[@]}" --run-types Bins -- \
    bash "$TEST_SCRIPT" "$PROJECT_ROOT" "$LOGS_DIR" "$TEST_CASE_DIR" "$COVERAGE_DATA_DIR" \
    2>&1 | tee "$OUTPUT_DIR/coverage_run.log" | tail -50; then
    pass "Coverage collection completed"
else
    # Tarpaulin may return non-zero if tests fail, but coverage data is still collected
    log_warning "Coverage collection completed with errors (this is expected for failure test cases)"
fi

# ============================================================================
# Step 5: Process and display coverage results
# ============================================================================

section "Step 5: Process Coverage Results"

# Parse coverage percentage from JSON report
if [[ -f "$COVERAGE_OUTPUT" ]]; then
    log_info "Coverage report generated: $COVERAGE_OUTPUT"
    
    # Extract overall coverage percentage using Python or jq if available
    if command -v jq >/dev/null 2>&1; then
        COVERAGE_PCT=$(jq -r '.files | map(.coverage) | add / length' "$COVERAGE_OUTPUT" 2>/dev/null || echo "N/A")
        
        if [[ "$COVERAGE_PCT" != "N/A" ]]; then
            COVERAGE_PCT=$(printf "%.2f" "$COVERAGE_PCT")
        fi
    elif command -v python3 >/dev/null 2>&1; then
        COVERAGE_PCT=$(python3 -c "
import json
import sys
try:
    with open('$COVERAGE_OUTPUT') as f:
        data = json.load(f)
    if 'files' in data:
        files = data['files']
        if files:
            total_coverage = sum(f.get('coverage', 0) for f in files.values() if isinstance(files, dict)) / len(files)
            print(f'{total_coverage:.2f}')
        else:
            print('N/A')
    else:
        print('N/A')
except Exception as e:
    print('N/A')
" 2>/dev/null || echo "N/A")
    else
        COVERAGE_PCT="N/A"
    fi
else
    log_warning "Coverage report file not found: $COVERAGE_OUTPUT"
    COVERAGE_PCT="N/A"
fi

# ============================================================================
# Step 6: Generate coverage summary report
# ============================================================================

section "Step 6: Generate Coverage Summary"

SUMMARY_FILE="$OUTPUT_DIR/coverage_summary.txt"

cat > "$SUMMARY_FILE" << EOF
================================================================================
Documentation Generation Code Coverage Report
================================================================================

Generated: $(date)
Project: Test Case Manager - Documentation Generation

Configuration:
  Logs Directory: $LOGS_DIR
  Test Case Directory: $TEST_CASE_DIR
  Output Directory: $OUTPUT_DIR

Modules Tracked:
  - src/lib.rs (library exports)
  - src/verification.rs (verification and report generation)
  - src/verification_templates.rs (template rendering)
  - src/parser.rs (YAML parsing)
  - src/models.rs (data models)
  - src/bin/verifier.rs (verifier binary)
  - src/bin/test-plan-documentation-generator-compat.rs (documentation generator)

Coverage Tool: cargo-tarpaulin $(cargo-tarpaulin --version 2>/dev/null || echo "version unknown")

Test Scenarios Executed:
  - TEST_SUCCESS_001 (successful execution)
  - TEST_FAILED_FIRST_001 (failed first step)
  - TEST_MULTI_SEQ_001 (multiple sequences)

================================================================================
Coverage Results
================================================================================

EOF

# Append coverage percentage to summary
if [[ "$COVERAGE_PCT" != "N/A" ]]; then
    cat >> "$SUMMARY_FILE" << EOF
Total Coverage: ${COVERAGE_PCT}%

EOF
else
    cat >> "$SUMMARY_FILE" << EOF
Total Coverage: Unable to calculate (see detailed reports)

EOF
fi

# Append file-level coverage details if available
if [[ -f "$COVERAGE_OUTPUT" ]] && command -v jq >/dev/null 2>&1; then
    cat >> "$SUMMARY_FILE" << EOF
File-Level Coverage:
EOF
    
    jq -r '.files | to_entries | .[] | "  \(.key): \(.value.coverage | tonumber | round)%"' "$COVERAGE_OUTPUT" 2>/dev/null >> "$SUMMARY_FILE" || true
    
    echo "" >> "$SUMMARY_FILE"
fi

cat >> "$SUMMARY_FILE" << EOF

================================================================================
Report Files
================================================================================

EOF

# List generated report files
if [[ -f "$COVERAGE_OUTPUT" ]]; then
    echo "  JSON Report:    $COVERAGE_OUTPUT" >> "$SUMMARY_FILE"
fi

if [[ "$GENERATE_HTML" == true ]] && [[ -d "$OUTPUT_DIR/html" ]]; then
    echo "  HTML Report:    $OUTPUT_DIR/html/index.html" >> "$SUMMARY_FILE"
fi

echo "  Summary Report: $SUMMARY_FILE" >> "$SUMMARY_FILE"
echo "  Run Log:        $OUTPUT_DIR/coverage_run.log" >> "$SUMMARY_FILE"

cat >> "$SUMMARY_FILE" << EOF

================================================================================
Notes
================================================================================

This coverage report focuses on the code paths exercised during documentation
generation from sample test cases. It includes:

1. Verification of test execution logs (verifier binary)
2. Container YAML validation (test-plan-documentation-generator-compat)
3. Template rendering and report generation
4. YAML parsing and model serialization

The report identifies which functions and branches in the template rendering
and document generation modules were executed during the sample test runs.

For detailed line-by-line coverage, see the HTML report (if generated) or
the JSON report for programmatic access.

================================================================================
EOF

pass "Coverage summary generated: $SUMMARY_FILE"

# Display summary to stdout
echo ""
cat "$SUMMARY_FILE"
echo ""

# Print coverage percentage prominently
if [[ "$COVERAGE_PCT" != "N/A" ]]; then
    section "Coverage Result"
    pass "Total Documentation Generation Coverage: ${COVERAGE_PCT}%"
    echo ""
else
    section "Coverage Result"
    log_warning "Coverage percentage could not be calculated"
    log_info "Please review the detailed coverage reports in: $OUTPUT_DIR"
    echo ""
fi

# Generate completion message
section "Coverage Report Complete"

log_info "All coverage reports saved to: $OUTPUT_DIR"
echo ""

if [[ "$GENERATE_HTML" == true ]]; then
    if [[ -f "$OUTPUT_DIR/html/index.html" ]]; then
        pass "HTML report available at: $OUTPUT_DIR/html/index.html"
        log_info "Open in browser: file://$OUTPUT_DIR/html/index.html"
    else
        log_warning "HTML report was not generated (check $OUTPUT_DIR for available reports)"
    fi
fi

exit 0
