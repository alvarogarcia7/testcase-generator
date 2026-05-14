#!/bin/bash
#
# Integration Test Runner for req-coverage
#
# This script runs integration tests for the requirement coverage tool,
# creating test scenarios, running the tool, and validating the output.
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMP_DIR=""
RESULTS_DIR="${SCRIPT_DIR}/results"
BINARY=""

# Ensure results directory exists
mkdir -p "${RESULTS_DIR}"

# Cleanup function
cleanup() {
    if [ -n "${TEMP_DIR}" ] && [ -d "${TEMP_DIR}" ]; then
        rm -rf "${TEMP_DIR}"
    fi
}

trap cleanup EXIT

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

# Find the req-coverage binary
find_binary() {
    local workspace_root="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"
    
    if [ -f "${workspace_root}/target/debug/req-coverage" ]; then
        BINARY="${workspace_root}/target/debug/req-coverage"
    elif [ -f "${workspace_root}/target/release/req-coverage" ]; then
        BINARY="${workspace_root}/target/release/req-coverage"
    else
        log_error "req-coverage binary not found. Please run 'cargo build -p req-coverage' first"
        exit 1
    fi
    
    log_info "Using binary: ${BINARY}"
}

# Create temporary test environment
setup_test_env() {
    TEMP_DIR=$(mktemp -d)
    mkdir -p "${TEMP_DIR}/testcases"
    mkdir -p "${TEMP_DIR}/results"
    log_info "Created test environment: ${TEMP_DIR}"
}

# Test case helper functions
create_requirement_file() {
    local req_id="$1"
    local req_text="$2"
    
    cat > "${TEMP_DIR}/requirements.yaml" <<EOF
requirements:
  - id: ${req_id}
    text: "${req_text}"
    description: "Test requirement"
EOF
}

create_multiple_requirements_file() {
    cat > "${TEMP_DIR}/requirements.yaml" <<'EOF'
requirements:
  - id: REQ-001
    text: "authenticate users with valid credentials"
    description: "Authentication requirement"
  
  - id: REQ-002
    text: "log security events and maintain audit trail"
    description: "Logging requirement"
  
  - id: REQ-003
    text: "validate input data"
    description: "Validation requirement"
EOF
}

create_test_case() {
    local tc_id="$1"
    local req_id="$2"
    local covers="${3:-}"
    
    local req_coverage=""
    if [ -n "${covers}" ]; then
        req_coverage="requirement_coverage:
  type: partial
  covers: \"${covers}\"
"
    fi
    
    cat > "${TEMP_DIR}/testcases/${tc_id}.yaml" <<EOF
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: ${req_id}
item: 1
tc: 1
id: ${tc_id}
description: Test case for ${req_id}
${req_coverage}general_initial_conditions:
  system:
  - Test system ready
initial_conditions:
  system:
  - Ready
test_sequences:
- id: 1
  name: Test sequence
  description: Test
  initial_conditions:
    system:
    - Ready
  steps:
  - step: 1
    description: Test step
    command: echo test
    expected:
      result: "0"
      output: "test"
    verification:
      result: '[[ \$EXIT_CODE -eq 0 ]]'
EOF
}

create_verification_result() {
    local tc_id="$1"
    local passed="$2"
    
    cat > "${TEMP_DIR}/results/${tc_id}_container.yaml" <<EOF
title: Verification Results
project: Test Project
test_date: "2024-01-20"
test_results:
  - test_case_id: ${tc_id}
    description: Test result
    overall_pass: ${passed}
EOF
}

# Test function wrapper
run_test() {
    local test_name="$1"
    local test_func="$2"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    log_info "Running: ${test_name}"
    
    # Setup fresh environment for each test
    setup_test_env
    
    if ${test_func}; then
        log_success "${test_name}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        log_error "${test_name}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

# Validation helpers
validate_json_field() {
    local json_file="$1"
    local field="$2"
    local expected="$3"
    
    local actual=$(jq -r ".${field}" "${json_file}")
    
    if [ "${actual}" = "${expected}" ]; then
        return 0
    else
        log_error "Field ${field}: expected '${expected}', got '${actual}'"
        return 1
    fi
}

validate_requirement_count() {
    local json_file="$1"
    local total="$2"
    local fully_covered="$3"
    local partially_covered="$4"
    local uncovered="$5"
    
    validate_json_field "${json_file}" "total_requirements" "${total}" || return 1
    validate_json_field "${json_file}" "fully_covered_requirements" "${fully_covered}" || return 1
    validate_json_field "${json_file}" "partially_covered_requirements" "${partially_covered}" || return 1
    validate_json_field "${json_file}" "uncovered_requirements" "${uncovered}" || return 1
    
    return 0
}

# Individual test cases

test_full_coverage_single_test() {
    create_requirement_file "REQ-001" "authenticate users"
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_verification_result "TC-001" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_full_coverage_single.log" 2>&1
    
    [ -f "${output}" ] || { log_error "Output file not created"; return 1; }
    
    validate_requirement_count "${output}" 1 1 0 0 || return 1
    
    local coverage_type=$(jq -r '.requirements[0].coverage_type' "${output}")
    [ "${coverage_type}" = "full" ] || { log_error "Expected full coverage"; return 1; }
    
    # Save results
    cp "${output}" "${RESULTS_DIR}/test_full_coverage_single.json"
    
    return 0
}

test_partial_coverage_multiple_tests() {
    create_requirement_file "REQ-001" "The system shall authenticate users and deny access"
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_test_case "TC-002" "REQ-001" "deny access"
    create_verification_result "TC-001" "true"
    create_verification_result "TC-002" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_partial_coverage.log" 2>&1
    
    validate_requirement_count "${output}" 1 0 1 0 || return 1
    
    local coverage_type=$(jq -r '.requirements[0].coverage_type' "${output}")
    [ "${coverage_type}" = "partial" ] || { log_error "Expected partial coverage"; return 1; }
    
    cp "${output}" "${RESULTS_DIR}/test_partial_coverage.json"
    
    return 0
}

test_invalid_covers_string() {
    create_requirement_file "REQ-001" "authenticate users"
    create_test_case "TC-001" "REQ-001" "invalid text not in requirement"
    create_verification_result "TC-001" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_invalid_covers.log" 2>&1
    
    local has_errors=$(jq -r '.requirements[0].coverage_errors != null' "${output}")
    [ "${has_errors}" = "true" ] || { log_error "Expected coverage errors"; return 1; }
    
    cp "${output}" "${RESULTS_DIR}/test_invalid_covers.json"
    
    return 0
}

test_without_requirements_file() {
    create_test_case "TC-001" "REQ-001" "some text"
    create_verification_result "TC-001" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        > "${RESULTS_DIR}/test_without_requirements.log" 2>&1
    
    validate_requirement_count "${output}" 1 0 1 0 || return 1
    
    local has_req_text=$(jq -r '.requirements[0].requirement_text' "${output}")
    [ "${has_req_text}" = "null" ] || { log_error "Should not have requirement_text"; return 1; }
    
    cp "${output}" "${RESULTS_DIR}/test_without_requirements.json"
    
    return 0
}

test_json_requirements_format() {
    cat > "${TEMP_DIR}/requirements.json" <<'EOF'
{
  "requirements": [
    {
      "id": "REQ-001",
      "text": "authenticate users",
      "description": "Test requirement"
    }
  ]
}
EOF
    
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_verification_result "TC-001" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.json" \
        > "${RESULTS_DIR}/test_json_format.log" 2>&1
    
    validate_requirement_count "${output}" 1 1 0 0 || return 1
    
    cp "${output}" "${RESULTS_DIR}/test_json_format.json"
    
    return 0
}

test_multiple_requirements() {
    create_multiple_requirements_file
    create_test_case "TC-001" "REQ-001" "authenticate users with valid credentials"
    create_test_case "TC-002" "REQ-002" "log security events"
    create_test_case "TC-003" "REQ-002" "maintain audit trail"
    create_verification_result "TC-001" "true"
    create_verification_result "TC-002" "true"
    create_verification_result "TC-003" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_multiple_requirements.log" 2>&1
    
    validate_requirement_count "${output}" 3 1 1 1 || return 1
    
    cp "${output}" "${RESULTS_DIR}/test_multiple_requirements.json"
    
    return 0
}

test_coverage_with_failures() {
    create_requirement_file "REQ-001" "authenticate users"
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_verification_result "TC-001" "false"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_with_failures.log" 2>&1
    
    local status=$(jq -r '.requirements[0].status' "${output}")
    [ "${status}" = "covered_fail" ] || { log_error "Expected covered_fail status"; return 1; }
    
    cp "${output}" "${RESULTS_DIR}/test_with_failures.json"
    
    return 0
}

test_html_generation() {
    create_requirement_file "REQ-001" "authenticate users"
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_verification_result "TC-001" "true"
    
    local coverage_json="${TEMP_DIR}/coverage.json"
    local html_dir="${TEMP_DIR}/html"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${coverage_json}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_html_verify.log" 2>&1
    
    "${BINARY}" print \
        --format html \
        --input "${coverage_json}" \
        --output "${html_dir}" \
        > "${RESULTS_DIR}/test_html_print.log" 2>&1
    
    [ -f "${html_dir}/index.html" ] || { log_error "HTML report not generated"; return 1; }
    
    # Check that HTML contains expected content
    grep -q "authenticate users" "${html_dir}/index.html" || { log_error "HTML missing requirement text"; return 1; }
    
    cp -r "${html_dir}" "${RESULTS_DIR}/test_html_output"
    
    return 0
}

test_case_sensitive_matching() {
    create_requirement_file "REQ-001" "Authenticate Users"
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_verification_result "TC-001" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_case_sensitive.log" 2>&1
    
    local has_errors=$(jq -r '.requirements[0].coverage_errors != null' "${output}")
    [ "${has_errors}" = "true" ] || { log_error "Expected case mismatch error"; return 1; }
    
    cp "${output}" "${RESULTS_DIR}/test_case_sensitive.json"
    
    return 0
}

test_duplicate_covers_strings() {
    create_requirement_file "REQ-001" "authenticate users"
    create_test_case "TC-001" "REQ-001" "authenticate users"
    create_test_case "TC-002" "REQ-001" "authenticate users"
    create_verification_result "TC-001" "true"
    create_verification_result "TC-002" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_duplicates.log" 2>&1
    
    local portions_count=$(jq -r '.requirements[0].covered_portions | length' "${output}")
    [ "${portions_count}" = "2" ] || { log_error "Expected 2 covered portions"; return 1; }
    
    local tc_count=$(jq -r '.requirements[0].test_cases | length' "${output}")
    [ "${tc_count}" = "2" ] || { log_error "Expected 2 test cases"; return 1; }
    
    cp "${output}" "${RESULTS_DIR}/test_duplicates.json"
    
    return 0
}

# Main execution
main() {
    echo "==============================================="
    echo "  req-coverage Integration Test Suite"
    echo "==============================================="
    echo ""
    
    find_binary
    
    # Check for jq
    if ! command -v jq &> /dev/null; then
        log_error "jq is required for these tests. Please install it first."
        exit 1
    fi
    
    log_info "Starting integration tests..."
    echo ""
    
    # Run all tests
    run_test "Full coverage with single test case" test_full_coverage_single_test
    run_test "Partial coverage with multiple tests" test_partial_coverage_multiple_tests
    run_test "Invalid covers string detection" test_invalid_covers_string
    run_test "Without requirements file (backward compatibility)" test_without_requirements_file
    run_test "JSON requirements file format" test_json_requirements_format
    run_test "Multiple requirements handling" test_multiple_requirements
    run_test "Coverage with test failures" test_coverage_with_failures
    run_test "HTML report generation" test_html_generation
    run_test "Case-sensitive matching" test_case_sensitive_matching
    run_test "Duplicate covers strings" test_duplicate_covers_strings
    
    # Summary
    echo ""
    echo "==============================================="
    echo "  Test Summary"
    echo "==============================================="
    echo -e "Total:  ${TOTAL_TESTS}"
    echo -e "${GREEN}Passed: ${PASSED_TESTS}${NC}"
    echo -e "${RED}Failed: ${FAILED_TESTS}${NC}"
    echo ""
    echo "Results saved to: ${RESULTS_DIR}"
    echo ""
    
    if [ ${FAILED_TESTS} -eq 0 ]; then
        log_success "All tests passed!"
        exit 0
    else
        log_error "Some tests failed"
        exit 1
    fi
}

# Run main if script is executed directly
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi
