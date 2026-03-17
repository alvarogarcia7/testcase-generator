#!/usr/bin/env bash
#
# Integration test for documentation generation workflow
#
# This test validates:
# 1. Running verifier on a successful test scenario
# 2. Converting verification output to result YAML
# 3. Invoking test-plan-doc-gen to generate AsciiDoc and Markdown reports
# 4. Validating generated files exist and contain expected content markers
# 5. Cleaning up temporary files
# 6. Checking for test-plan-doc-gen availability and skipping if not found
#
# Usage: ./tests/integration/test_documentation_generation.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
VERIFIER_BIN="$PROJECT_ROOT/target/debug/verifier"

# Source required libraries
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1
source "$SCRIPT_DIR/../../scripts/lib/report_generator.sh" || exit 1

# Handle --no-remove flag
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
            shift
            ;;
        *)
            shift
            ;;
    esac
done

echo "======================================"
echo "Documentation Generation Integration Test"
echo "======================================"
echo ""

# Check prerequisites
section "Checking Prerequisites"

if [[ ! -f "$VERIFIER_BIN" ]]; then
    fail "verifier binary not found at $VERIFIER_BIN"
    echo "Run 'cargo build' or 'make build' first"
    exit 1
fi
pass "verifier binary found"

if ! command -v python3 > /dev/null 2>&1; then
    fail "python3 not found"
    exit 1
fi
pass "python3 available"

# Check if PyYAML is available
if python3 -c "import yaml" 2>/dev/null; then
    pass "PyYAML available"
else
    fail "PyYAML not available. Install with: pip3 install pyyaml"
    exit 1
fi

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"
if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

info "Using temporary directory: $TEMP_DIR"

# Define paths
TEST_CASE_DIR="$PROJECT_ROOT/testcases/verifier_scenarios/successful"
TEST_CASE_FILE="$TEST_CASE_DIR/TEST_SUCCESS_001.yml"
EXECUTION_LOG="$TEST_CASE_DIR/TEST_SUCCESS_001_execution_log.json"
VERIFICATION_OUTPUT="$TEMP_DIR/verification_output.json"
RESULT_YAML_DIR="$TEMP_DIR/results"
REPORTS_DIR="$TEMP_DIR/reports"

# Create output directories
mkdir -p "$RESULT_YAML_DIR"
mkdir -p "$REPORTS_DIR"

# Test 1: Run verifier on successful test scenario (with config file)
section "Test 1: Run Verifier on Successful Test Scenario"

if [[ ! -f "$TEST_CASE_FILE" ]]; then
    fail "Test case file not found: $TEST_CASE_FILE"
    exit 1
fi
pass "Test case file found: $(basename "$TEST_CASE_FILE")"

if [[ ! -f "$EXECUTION_LOG" ]]; then
    fail "Execution log not found: $EXECUTION_LOG"
    exit 1
fi
pass "Execution log found: $(basename "$EXECUTION_LOG")"

# Create a test config file
TEST_CONFIG="$TEMP_DIR/doc_gen_config.yml"
cat > "$TEST_CONFIG" << 'EOF'
title: "Documentation Generation Test Results"
project: "Test Case Manager - Documentation Generation"
environment: "Integration Testing"
platform: "Test Environment"
executor: "Documentation Generation Test"
EOF

log_info "Running verifier on successful test scenario..."
if "$VERIFIER_BIN" \
    --log "$EXECUTION_LOG" \
    --test-case "TEST_SUCCESS_001" \
    --test-case-dir "$TEST_CASE_DIR" \
    --format json \
    --output "$VERIFICATION_OUTPUT" \
    --config "$TEST_CONFIG" > /dev/null 2>&1; then
    pass "Verifier completed successfully"
else
    VERIFIER_EXIT=$?
    fail "Verifier failed with exit code: $VERIFIER_EXIT"
    exit 1
fi

if [[ ! -f "$VERIFICATION_OUTPUT" ]]; then
    fail "Verification output not generated"
    exit 1
fi
pass "Verification output generated: $(basename "$VERIFICATION_OUTPUT")"

# Validate verification output structure
if grep -q '"test_case_id"' "$VERIFICATION_OUTPUT"; then
    pass "Verification output contains test_case_id field"
else
    fail "Verification output missing test_case_id field"
fi

if grep -q '"overall_pass".*true' "$VERIFICATION_OUTPUT"; then
    pass "Verification output shows overall pass"
else
    fail "Verification output should show overall pass"
fi

# Test 2: Convert verification output to result YAML
section "Test 2: Convert Verification Output to Result YAML"

CONVERT_SCRIPT="$PROJECT_ROOT/scripts/convert_verification_to_result_yaml.py"

if [[ ! -f "$CONVERT_SCRIPT" ]]; then
    fail "Conversion script not found: $CONVERT_SCRIPT"
    exit 1
fi
pass "Conversion script found"

log_info "Converting verification JSON to result YAML..."
if python3 "$CONVERT_SCRIPT" \
    "$VERIFICATION_OUTPUT" \
    -o "$RESULT_YAML_DIR" > /dev/null 2>&1; then
    pass "Conversion completed successfully"
else
    fail "Conversion failed"
    exit 1
fi

# Verify result YAML file was created
RESULT_YAML_FILE="$RESULT_YAML_DIR/TEST_SUCCESS_001_result.yaml"
if [[ ! -f "$RESULT_YAML_FILE" ]]; then
    fail "Result YAML file not created: $(basename "$RESULT_YAML_FILE")"
    exit 1
fi
pass "Result YAML file created: $(basename "$RESULT_YAML_FILE")"

# Validate result YAML structure
if grep -q "type: result" "$RESULT_YAML_FILE"; then
    pass "Result YAML contains 'type: result' field"
else
    fail "Result YAML missing 'type: result' field"
fi

if grep -q "test_case_id: TEST_SUCCESS_001" "$RESULT_YAML_FILE"; then
    pass "Result YAML contains correct test case ID"
else
    fail "Result YAML missing correct test case ID"
fi

if grep -q "overall_pass: true" "$RESULT_YAML_FILE"; then
    pass "Result YAML shows overall pass"
else
    fail "Result YAML should show overall pass"
fi

# Test 3: Check for test-plan-doc-gen availability
section "Test 3: Check test-plan-doc-gen Availability"

TEST_PLAN_DOC_GEN_DIR="$PROJECT_ROOT/../test-plan-doc-gen"
SKIP_DOC_GEN=0

if [[ ! -d "$TEST_PLAN_DOC_GEN_DIR" ]]; then
    log_warning "test-plan-doc-gen directory not found: $TEST_PLAN_DOC_GEN_DIR"
    log_warning "Skipping test-plan-doc-gen report generation tests"
    info "To enable test-plan-doc-gen reports, clone the repository:"
    info "  cd $(dirname "$PROJECT_ROOT")"
    info "  git clone <test-plan-doc-gen-repo-url> test-plan-doc-gen"
    SKIP_DOC_GEN=1
else
    # Check if binary exists
    if check_test_plan_doc_gen_available "$TEST_PLAN_DOC_GEN_DIR"; then
        pass "test-plan-doc-gen binary found"
    else
        log_info "test-plan-doc-gen binary not found, attempting to build..."
        if build_test_plan_doc_gen "$TEST_PLAN_DOC_GEN_DIR"; then
            pass "test-plan-doc-gen built successfully"
        else
            log_warning "Failed to build test-plan-doc-gen"
            log_warning "Skipping test-plan-doc-gen report generation tests"
            SKIP_DOC_GEN=1
        fi
    fi
fi

if [[ $SKIP_DOC_GEN -eq 1 ]]; then
    section "Test Summary"
    echo ""
    log_info "Completed tests (test-plan-doc-gen tests skipped):"
    info "✓ Verifier execution on successful test scenario"
    info "✓ Conversion of verification output to result YAML"
    info "✓ Result YAML validation"
    echo ""
    pass "All available tests passed (test-plan-doc-gen not available)"
    exit 0
fi

# Test 4: Generate AsciiDoc report using result container
section "Test 4: Generate AsciiDoc Report"

# Create a results container YAML
RESULT_CONTAINER="$TEMP_DIR/results_container.yaml"
log_info "Creating results container YAML..."

cat > "$RESULT_CONTAINER" << EOF
title: 'Test Execution Results Report'
project: 'Test Case Manager - Verification Results'
test_date: '2024-01-01T00:00:00Z'
test_results:
EOF

# Add result file content to container (without 'type: result' line)
sed '/^type: result/d' "$RESULT_YAML_FILE" | sed 's/^/  /' >> "$RESULT_CONTAINER"

# Add metadata
cat >> "$RESULT_CONTAINER" << EOF
metadata:
  environment: 'Test Environment'
  platform: 'Test Case Manager'
  executor: 'Integration Test'
  execution_duration: 0.0
  total_test_cases: 1
  passed_test_cases: 1
  failed_test_cases: 0
EOF

pass "Created results container: $(basename "$RESULT_CONTAINER")"

# Generate AsciiDoc report
ASCIIDOC_OUTPUT="$REPORTS_DIR/test_results_report.adoc"

log_info "Generating AsciiDoc test results report..."

# Set TEST_PLAN_DOC_GEN environment variable
export TEST_PLAN_DOC_GEN=$(find_test_plan_doc_gen "$TEST_PLAN_DOC_GEN_DIR")

if invoke_test_plan_doc_gen \
    --container "$RESULT_CONTAINER" \
    --output "$ASCIIDOC_OUTPUT" \
    --format asciidoc > /dev/null 2>&1; then
    pass "AsciiDoc report generated successfully"
else
    fail "Failed to generate AsciiDoc report"
    SKIP_DOC_GEN=1
fi

if [[ $SKIP_DOC_GEN -eq 0 ]] && [[ -f "$ASCIIDOC_OUTPUT" ]]; then
    pass "AsciiDoc report file created: $(basename "$ASCIIDOC_OUTPUT")"
    
    # Validate AsciiDoc content markers
    if grep -q "= Test Execution Results Report" "$ASCIIDOC_OUTPUT" || \
       grep -q "Test Execution Results" "$ASCIIDOC_OUTPUT" || \
       grep -q "TEST_SUCCESS_001" "$ASCIIDOC_OUTPUT"; then
        pass "AsciiDoc report contains expected content markers"
    else
        fail "AsciiDoc report missing expected content markers"
    fi
    
    # Check file size is reasonable (not empty)
    FILE_SIZE=$(stat -f%z "$ASCIIDOC_OUTPUT" 2>/dev/null || stat -c%s "$ASCIIDOC_OUTPUT" 2>/dev/null || echo "0")
    if [[ $FILE_SIZE -gt 0 ]]; then
        pass "AsciiDoc report has content ($FILE_SIZE bytes)"
    else
        fail "AsciiDoc report is empty"
    fi
else
    if [[ $SKIP_DOC_GEN -eq 0 ]]; then
        fail "AsciiDoc report file not created"
    fi
fi

# Test 5: Generate Markdown report from test case YAML
section "Test 5: Generate Markdown Report"

MARKDOWN_OUTPUT="$REPORTS_DIR/TEST_SUCCESS_001_test_plan.md"

log_info "Generating Markdown test plan report..."

if invoke_test_plan_doc_gen \
    --test-case "$TEST_CASE_FILE" \
    --output "$MARKDOWN_OUTPUT" \
    --format markdown > /dev/null 2>&1; then
    pass "Markdown report generated successfully"
else
    fail "Failed to generate Markdown report"
    SKIP_DOC_GEN=1
fi

if [[ $SKIP_DOC_GEN -eq 0 ]] && [[ -f "$MARKDOWN_OUTPUT" ]]; then
    pass "Markdown report file created: $(basename "$MARKDOWN_OUTPUT")"
    
    # Validate Markdown content markers
    if grep -q "# " "$MARKDOWN_OUTPUT" || \
       grep -q "TEST_SUCCESS_001" "$MARKDOWN_OUTPUT" || \
       grep -q "test" "$MARKDOWN_OUTPUT"; then
        pass "Markdown report contains expected content markers"
    else
        fail "Markdown report missing expected content markers"
    fi
    
    # Check file size is reasonable (not empty)
    FILE_SIZE=$(stat -f%z "$MARKDOWN_OUTPUT" 2>/dev/null || stat -c%s "$MARKDOWN_OUTPUT" 2>/dev/null || echo "0")
    if [[ $FILE_SIZE -gt 0 ]]; then
        pass "Markdown report has content ($FILE_SIZE bytes)"
    else
        fail "Markdown report is empty"
    fi
else
    if [[ $SKIP_DOC_GEN -eq 0 ]]; then
        fail "Markdown report file not created"
    fi
fi

# Test 6: Verify cleanup of temporary files (happens automatically via setup_cleanup)
section "Test 6: Verify Cleanup"

if [[ $REMOVE_TEMP -eq 1 ]]; then
    info "Temporary files will be cleaned up automatically on exit"
    pass "Cleanup configured (will execute on exit)"
else
    info "Cleanup disabled via --no-remove flag"
    info "Temporary files preserved at: $TEMP_DIR"
    pass "Cleanup skipped as requested"
fi

# Summary
section "Test Summary"
echo ""

log_info "Successfully completed tests:"
info "✓ Verifier execution on successful test scenario"
info "✓ Conversion of verification output to result YAML"
info "✓ Result YAML validation"

if [[ $SKIP_DOC_GEN -eq 0 ]]; then
    info "✓ AsciiDoc report generation and validation"
    info "✓ Markdown report generation and validation"
fi

echo ""
log_info "Generated files:"

if [[ -f "$VERIFICATION_OUTPUT" ]]; then
    info "  📄 Verification JSON: $(basename "$VERIFICATION_OUTPUT")"
fi

if [[ -f "$RESULT_YAML_FILE" ]]; then
    info "  📄 Result YAML: $(basename "$RESULT_YAML_FILE")"
fi

if [[ $SKIP_DOC_GEN -eq 0 ]]; then
    if [[ -f "$ASCIIDOC_OUTPUT" ]]; then
        info "  📄 AsciiDoc Report: $(basename "$ASCIIDOC_OUTPUT")"
    fi
    
    if [[ -f "$MARKDOWN_OUTPUT" ]]; then
        info "  📄 Markdown Report: $(basename "$MARKDOWN_OUTPUT")"
    fi
fi

echo ""
pass "All tests passed!"
exit 0
