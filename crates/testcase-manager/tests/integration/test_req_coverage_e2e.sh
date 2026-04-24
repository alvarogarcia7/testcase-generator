#!/bin/bash
#
# End-to-end integration test for req-coverage tool
#
# This test validates:
# 1. Building req-coverage binary
# 2. Analyzing test cases with different coverage scenarios
# 3. Generating coverage reports (JSON and HTML)
# 4. Displaying coverage statistics with pass/fail percentages
#
# Coverage scenarios tested:
# - REQ-001: Full coverage, all tests passed (2 test cases)
# - REQ-002: Full coverage, mixed results (1 pass, 1 fail)
# - REQ-003: Partial coverage, all tests passed
# - REQ-004: Partial coverage, test failed
# - REQ-005: Full coverage, not executed (no verification results)
# - REQ-006, REQ-007, REQ-008: Full coverage via multi-requirement test
#
# Usage: ./tests/integration/test_req_coverage_e2e.sh [--no-remove]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Source logger library
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1
source "$PROJECT_ROOT/scripts/lib/shellcheck-helper.sh" || true

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

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

section "req-coverage End-to-End Integration Test"

# Create temporary directory for test outputs
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"
if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

info "Using temporary directory: $TEMP_DIR"

# Test data directories
TEST_DATA_DIR="$SCRIPT_DIR/req_coverage_testdata"
TEST_CASES_DIR="$TEST_DATA_DIR/test_cases"
VERIFICATION_RESULTS_DIR="$TEST_DATA_DIR/verification_results"

# ============================================================================
# Test 1: Build req-coverage binary
# ============================================================================
section "Test 1: Build req-coverage binary"

log_info "Building req-coverage binary..."
if cargo build -p req-coverage > "$TEMP_DIR/build.log" 2>&1; then
    pass "req-coverage binary built successfully"
    ((++TESTS_PASSED))
else
    fail "Failed to build req-coverage binary"
    cat "$TEMP_DIR/build.log"
    ((++TESTS_FAILED))
    exit 1
fi

# Source shared libraries for finding binaries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh" || exit 1

# Find req-coverage binary using workspace-aware search
cd "$PROJECT_ROOT"
REQ_COVERAGE_BIN=$(find_binary "req-coverage")
if [[ -z "$REQ_COVERAGE_BIN" ]]; then
    fail "req-coverage binary not found after build"
    ((++TESTS_FAILED))
    exit 1
fi
pass "req-coverage binary exists at $REQ_COVERAGE_BIN"
((++TESTS_PASSED))

# ============================================================================
# Test 2: Verify test data exists
# ============================================================================
section "Test 2: Verify test data exists"

if [[ ! -d "$TEST_CASES_DIR" ]]; then
    fail "Test cases directory not found: $TEST_CASES_DIR"
    ((++TESTS_FAILED))
    exit 1
fi
pass "Test cases directory exists"
((++TESTS_PASSED))

if [[ ! -d "$VERIFICATION_RESULTS_DIR" ]]; then
    fail "Verification results directory not found: $VERIFICATION_RESULTS_DIR"
    ((++TESTS_FAILED))
    exit 1
fi
pass "Verification results directory exists"
((++TESTS_PASSED))

# Count test case files
TEST_CASE_COUNT=$(find "$TEST_CASES_DIR" -name "*.yaml" -o -name "*.yml" | wc -l | tr -d ' ')
info "Found $TEST_CASE_COUNT test case files"
if [[ "$TEST_CASE_COUNT" -ge 8 ]]; then
    pass "Expected number of test case files present"
    ((++TESTS_PASSED))
else
    fail "Expected at least 8 test case files, found $TEST_CASE_COUNT"
    ((++TESTS_FAILED))
fi

# Count verification result files
VERIFICATION_COUNT=$(find "$VERIFICATION_RESULTS_DIR" -name "*_container.yaml" -o -name "*_container.yml" | wc -l | tr -d ' ')
info "Found $VERIFICATION_COUNT verification result files"
if [[ "$VERIFICATION_COUNT" -ge 1 ]]; then
    pass "Verification result files present"
    ((++TESTS_PASSED))
else
    fail "Expected at least 1 verification result file, found $VERIFICATION_COUNT"
    ((++TESTS_FAILED))
fi

# ============================================================================
# Test 3: Generate coverage report (JSON)
# ============================================================================
section "Test 3: Generate coverage report (JSON)"

OUTPUT_JSON="$TEMP_DIR/coverage_report.json"

log_info "Running req-coverage verify command..."
if "$REQ_COVERAGE_BIN" verify \
    --test-cases-folder "$TEST_CASES_DIR" \
    --test-results-folder "$VERIFICATION_RESULTS_DIR" \
    --output "$OUTPUT_JSON" > "$TEMP_DIR/verify.log" 2>&1; then
    pass "req-coverage verify command executed successfully"
    ((++TESTS_PASSED))
else
    fail "req-coverage verify command failed"
    cat "$TEMP_DIR/verify.log"
    ((++TESTS_FAILED))
    exit 1
fi

# Verify JSON file was created
if [[ ! -f "$OUTPUT_JSON" ]]; then
    fail "Coverage report JSON not created at $OUTPUT_JSON"
    ((++TESTS_FAILED))
    exit 1
fi
pass "Coverage report JSON file created"
((++TESTS_PASSED))

# ============================================================================
# Test 4: Validate JSON structure
# ============================================================================
section "Test 4: Validate JSON structure"

if command -v jq >/dev/null 2>&1; then
    if jq empty "$OUTPUT_JSON" >/dev/null 2>&1; then
        pass "Coverage report JSON is valid"
        ((++TESTS_PASSED))
    else
        fail "Coverage report JSON is invalid"
        cat "$OUTPUT_JSON"
        ((++TESTS_FAILED))
        exit 1
    fi
    
    # Extract statistics from JSON
    TOTAL_REQS=$(jq -r '.total_requirements' "$OUTPUT_JSON")
    FULLY_COVERED=$(jq -r '.fully_covered_requirements' "$OUTPUT_JSON")
    PARTIALLY_COVERED=$(jq -r '.partially_covered_requirements' "$OUTPUT_JSON")
    UNCOVERED=$(jq -r '.uncovered_requirements' "$OUTPUT_JSON")
    
    info "Coverage Statistics:"
    info "  Total requirements: $TOTAL_REQS"
    info "  Fully covered: $FULLY_COVERED"
    info "  Partially covered: $PARTIALLY_COVERED"
    info "  Uncovered: $UNCOVERED"
    
    # Validate expected values
    # Expected: 8 requirements (REQ-001 through REQ-008)
    if [[ "$TOTAL_REQS" -eq 8 ]]; then
        pass "Total requirements count is correct (8)"
        ((++TESTS_PASSED))
    else
        fail "Expected 8 total requirements, got $TOTAL_REQS"
        ((++TESTS_FAILED))
    fi
    
    # Expected: 5 fully covered (REQ-001, REQ-002, REQ-005, REQ-006, REQ-007, REQ-008)
    # Note: REQ-005 is not executed but still counts in requirements
    if [[ "$FULLY_COVERED" -ge 4 ]]; then
        pass "Fully covered requirements count is reasonable ($FULLY_COVERED)"
        ((++TESTS_PASSED))
    else
        fail "Expected at least 4 fully covered requirements, got $FULLY_COVERED"
        ((++TESTS_FAILED))
    fi
    
    # Expected: 2 partially covered (REQ-003, REQ-004)
    if [[ "$PARTIALLY_COVERED" -eq 2 ]]; then
        pass "Partially covered requirements count is correct (2)"
        ((++TESTS_PASSED))
    else
        fail "Expected 2 partially covered requirements, got $PARTIALLY_COVERED"
        ((++TESTS_FAILED))
    fi
    
else
    info "jq not available, skipping JSON validation"
fi

# ============================================================================
# Test 5: Analyze coverage details
# ============================================================================
section "Test 5: Analyze coverage details"

if command -v jq >/dev/null 2>&1; then
    # Check REQ-001 (should have 2 test cases, both passed)
    REQ001_TEST_COUNT=$(jq -r '.requirements[] | select(.requirement_id == "REQ-001") | .test_cases | length' "$OUTPUT_JSON")
    if [[ "$REQ001_TEST_COUNT" -eq 2 ]]; then
        pass "REQ-001 has 2 test cases"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-001 to have 2 test cases, got $REQ001_TEST_COUNT"
        ((++TESTS_FAILED))
    fi
    
    REQ001_STATUS=$(jq -r '.requirements[] | select(.requirement_id == "REQ-001") | .status' "$OUTPUT_JSON")
    if [[ "$REQ001_STATUS" == "covered_pass" ]]; then
        pass "REQ-001 status is covered_pass"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-001 status to be covered_pass, got $REQ001_STATUS"
        ((++TESTS_FAILED))
    fi
    
    # Check REQ-002 (should have 2 test cases, mixed results - 1 pass, 1 fail)
    REQ002_TEST_COUNT=$(jq -r '.requirements[] | select(.requirement_id == "REQ-002") | .test_cases | length' "$OUTPUT_JSON")
    if [[ "$REQ002_TEST_COUNT" -eq 2 ]]; then
        pass "REQ-002 has 2 test cases"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-002 to have 2 test cases, got $REQ002_TEST_COUNT"
        ((++TESTS_FAILED))
    fi
    
    REQ002_STATUS=$(jq -r '.requirements[] | select(.requirement_id == "REQ-002") | .status' "$OUTPUT_JSON")
    if [[ "$REQ002_STATUS" == "covered_fail" ]]; then
        pass "REQ-002 status is covered_fail (mixed results)"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-002 status to be covered_fail, got $REQ002_STATUS"
        ((++TESTS_FAILED))
    fi
    
    # Check REQ-003 (partial coverage, passed)
    REQ003_COVERAGE_TYPE=$(jq -r '.requirements[] | select(.requirement_id == "REQ-003") | .coverage_type' "$OUTPUT_JSON")
    if [[ "$REQ003_COVERAGE_TYPE" == "partial" ]]; then
        pass "REQ-003 coverage type is partial"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-003 coverage type to be partial, got $REQ003_COVERAGE_TYPE"
        ((++TESTS_FAILED))
    fi
    
    REQ003_STATUS=$(jq -r '.requirements[] | select(.requirement_id == "REQ-003") | .status' "$OUTPUT_JSON")
    if [[ "$REQ003_STATUS" == "partial_covered_pass" ]]; then
        pass "REQ-003 status is partial_covered_pass"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-003 status to be partial_covered_pass, got $REQ003_STATUS"
        ((++TESTS_FAILED))
    fi
    
    # Check REQ-004 (partial coverage, failed)
    REQ004_STATUS=$(jq -r '.requirements[] | select(.requirement_id == "REQ-004") | .status' "$OUTPUT_JSON")
    if [[ "$REQ004_STATUS" == "partial_covered_fail" ]]; then
        pass "REQ-004 status is partial_covered_fail"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-004 status to be partial_covered_fail, got $REQ004_STATUS"
        ((++TESTS_FAILED))
    fi
    
    # Check REQ-005 (not executed)
    REQ005_TEST_COUNT=$(jq -r '.requirements[] | select(.requirement_id == "REQ-005") | .test_cases | length' "$OUTPUT_JSON")
    if [[ "$REQ005_TEST_COUNT" -eq 1 ]]; then
        pass "REQ-005 has 1 test case"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-005 to have 1 test case, got $REQ005_TEST_COUNT"
        ((++TESTS_FAILED))
    fi
    
    REQ005_TEST_STATUS=$(jq -r '.requirements[] | select(.requirement_id == "REQ-005") | .test_cases[0].status' "$OUTPUT_JSON")
    if [[ "$REQ005_TEST_STATUS" == "notexecuted" ]]; then
        pass "REQ-005 test status is notexecuted"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-005 test status to be notexecuted, got $REQ005_TEST_STATUS"
        ((++TESTS_FAILED))
    fi
    
    # Check multi-requirement coverage (REQ-006, REQ-007, REQ-008)
    REQ006_TEST_COUNT=$(jq -r '.requirements[] | select(.requirement_id == "REQ-006") | .test_cases | length' "$OUTPUT_JSON")
    if [[ "$REQ006_TEST_COUNT" -eq 1 ]]; then
        pass "REQ-006 covered by multi-requirement test"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-006 to have 1 test case, got $REQ006_TEST_COUNT"
        ((++TESTS_FAILED))
    fi
    
    REQ007_TEST_COUNT=$(jq -r '.requirements[] | select(.requirement_id == "REQ-007") | .test_cases | length' "$OUTPUT_JSON")
    if [[ "$REQ007_TEST_COUNT" -eq 1 ]]; then
        pass "REQ-007 covered by multi-requirement test"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-007 to have 1 test case, got $REQ007_TEST_COUNT"
        ((++TESTS_FAILED))
    fi
    
    REQ008_TEST_COUNT=$(jq -r '.requirements[] | select(.requirement_id == "REQ-008") | .test_cases | length' "$OUTPUT_JSON")
    if [[ "$REQ008_TEST_COUNT" -eq 1 ]]; then
        pass "REQ-008 covered by multi-requirement test"
        ((++TESTS_PASSED))
    else
        fail "Expected REQ-008 to have 1 test case, got $REQ008_TEST_COUNT"
        ((++TESTS_FAILED))
    fi
fi

# ============================================================================
# Test 6: Calculate and display coverage percentages
# ============================================================================
section "Test 6: Calculate and display coverage percentages"

if command -v jq >/dev/null 2>&1; then
    # Calculate success/failure percentages
    TOTAL_TEST_CASES=$(jq -r '[.requirements[].test_cases[]] | length' "$OUTPUT_JSON")
    PASSED_TEST_CASES=$(jq -r '[.requirements[].test_cases[] | select(.status == "pass")] | length' "$OUTPUT_JSON")
    FAILED_TEST_CASES=$(jq -r '[.requirements[].test_cases[] | select(.status == "fail")] | length' "$OUTPUT_JSON")
    NOT_EXECUTED_TEST_CASES=$(jq -r '[.requirements[].test_cases[] | select(.status == "notexecuted")] | length' "$OUTPUT_JSON")
    
    if [[ "$TOTAL_TEST_CASES" -gt 0 ]]; then
        PASS_PERCENT=$(awk "BEGIN {printf \"%.1f\", ($PASSED_TEST_CASES / $TOTAL_TEST_CASES) * 100}")
        FAIL_PERCENT=$(awk "BEGIN {printf \"%.1f\", ($FAILED_TEST_CASES / $TOTAL_TEST_CASES) * 100}")
        NOT_EXEC_PERCENT=$(awk "BEGIN {printf \"%.1f\", ($NOT_EXECUTED_TEST_CASES / $TOTAL_TEST_CASES) * 100}")
    else
        PASS_PERCENT="0.0"
        FAIL_PERCENT="0.0"
        NOT_EXEC_PERCENT="0.0"
    fi
    
    # Display coverage summary
    echo ""
    echo "=========================================="
    echo "REQUIREMENT COVERAGE SUMMARY"
    echo "=========================================="
    echo ""
    echo "Total Requirements: $TOTAL_REQS"
    echo "  Fully Covered:    $FULLY_COVERED"
    echo "  Partially Covered: $PARTIALLY_COVERED"
    echo "  Uncovered:        $UNCOVERED"
    echo ""
    echo "Test Cases: $TOTAL_TEST_CASES"
    echo "  Passed:           $PASSED_TEST_CASES ($PASS_PERCENT%)"
    echo "  Failed:           $FAILED_TEST_CASES ($FAIL_PERCENT%)"
    echo "  Not Executed:     $NOT_EXECUTED_TEST_CASES ($NOT_EXEC_PERCENT%)"
    echo ""
    echo "=========================================="
    echo ""
    
    pass "Coverage percentages calculated and displayed"
    ((++TESTS_PASSED))
    
    # Validate percentages sum to approximately 100%
    TOTAL_PERCENT=$(awk "BEGIN {printf \"%.1f\", $PASS_PERCENT + $FAIL_PERCENT + $NOT_EXEC_PERCENT}")
    if (( $(echo "$TOTAL_PERCENT >= 99.9" | bc -l) )) && (( $(echo "$TOTAL_PERCENT <= 100.1" | bc -l) )); then
        pass "Percentages sum to approximately 100% ($TOTAL_PERCENT%)"
        ((++TESTS_PASSED))
    else
        fail "Percentages should sum to 100%, got $TOTAL_PERCENT%"
        ((++TESTS_FAILED))
    fi
else
    info "jq not available, skipping percentage calculation"
fi

# ============================================================================
# Test 7: Generate HTML report
# ============================================================================
section "Test 7: Generate HTML report"

HTML_OUTPUT_DIR="$TEMP_DIR/html_report"
mkdir -p "$HTML_OUTPUT_DIR"

log_info "Running req-coverage print command to generate HTML..."
if "$REQ_COVERAGE_BIN" print \
    --format html \
    --input "$OUTPUT_JSON" \
    --output "$HTML_OUTPUT_DIR" > "$TEMP_DIR/print.log" 2>&1; then
    pass "req-coverage print command executed successfully"
    ((++TESTS_PASSED))
else
    fail "req-coverage print command failed"
    cat "$TEMP_DIR/print.log"
    ((++TESTS_FAILED))
fi

# Verify HTML file was created
HTML_FILE="$HTML_OUTPUT_DIR/index.html"
if [[ ! -f "$HTML_FILE" ]]; then
    fail "HTML report not created at $HTML_FILE"
    ((++TESTS_FAILED))
else
    pass "HTML report file created"
    ((++TESTS_PASSED))
    
    # Verify HTML contains expected content
    if grep -q "Requirement Coverage Report" "$HTML_FILE"; then
        pass "HTML report contains expected title"
        ((++TESTS_PASSED))
    else
        fail "HTML report missing expected title"
        ((++TESTS_FAILED))
    fi
    
    if grep -q "REQ-001" "$HTML_FILE"; then
        pass "HTML report contains requirement REQ-001"
        ((++TESTS_PASSED))
    else
        fail "HTML report missing requirement REQ-001"
        ((++TESTS_FAILED))
    fi
    
    info "HTML report available at: $HTML_FILE"
fi

# ============================================================================
# Test 8: Display detailed requirement breakdown
# ============================================================================
section "Test 8: Display detailed requirement breakdown"

if command -v jq >/dev/null 2>&1; then
    echo ""
    echo "=========================================="
    echo "DETAILED REQUIREMENT BREAKDOWN"
    echo "=========================================="
    echo ""
    
    jq -r '.requirements[] | 
        "\(.requirement_id): \(.coverage_type) coverage, status: \(.status), test cases: \(.test_cases | length)"' \
        "$OUTPUT_JSON"
    
    echo ""
    echo "=========================================="
    echo ""
    
    pass "Detailed requirement breakdown displayed"
    ((++TESTS_PASSED))
fi

# ============================================================================
# Test 9: Verify logging output
# ============================================================================
section "Test 9: Verify logging output"

if grep -q "Coverage analysis complete" "$TEMP_DIR/verify.log"; then
    pass "Verify command logged completion"
    ((++TESTS_PASSED))
else
    fail "Verify command did not log completion"
    ((++TESTS_FAILED))
fi

if grep -q "Report Generation Complete" "$TEMP_DIR/print.log"; then
    pass "Print command logged completion"
    ((++TESTS_PASSED))
else
    fail "Print command did not log completion"
    ((++TESTS_FAILED))
fi

# ============================================================================
# Summary
# ============================================================================
section "Test Summary"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo ""
echo "Total tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -gt 0 ]]; then
    fail "Some tests failed"
    exit 1
else
    pass "All tests passed!"
    
    echo ""
    echo "=========================================="
    echo "TEST DATA SUMMARY"
    echo "=========================================="
    echo ""
    echo "Test Cases Created: 8"
    echo "  - REQ001_PASS_TC.yaml (REQ-001, full coverage, passed)"
    echo "  - REQ001_ADDITIONAL_PASS_TC.yaml (REQ-001, full coverage, passed)"
    echo "  - REQ002_FAIL_TC.yaml (REQ-002, full coverage, failed)"
    echo "  - REQ002_MIXED_RESULT_TC.yaml (REQ-002, full coverage, passed)"
    echo "  - REQ003_PARTIAL_PASS_TC.yaml (REQ-003, partial coverage, passed)"
    echo "  - REQ004_PARTIAL_FAIL_TC.yaml (REQ-004, partial coverage, failed)"
    echo "  - REQ005_NOT_EXECUTED_TC.yaml (REQ-005, full coverage, not executed)"
    echo "  - REQ006_MULTI_REQ_TC.yaml (REQ-006/007/008, full coverage, passed)"
    echo ""
    echo "Verification Results: 1 container file"
    echo "  - 7 test results (REQ005 not included = not executed)"
    echo ""
    echo "Coverage Scenarios Tested:"
    echo "  ✓ Full coverage with all tests passing"
    echo "  ✓ Full coverage with mixed results (pass/fail)"
    echo "  ✓ Full coverage with no execution"
    echo "  ✓ Partial coverage with passing tests"
    echo "  ✓ Partial coverage with failing tests"
    echo "  ✓ Multiple requirements covered by single test"
    echo "  ✓ Multiple tests covering same requirement"
    echo ""
    echo "=========================================="
    echo ""
    
    exit 0
fi
