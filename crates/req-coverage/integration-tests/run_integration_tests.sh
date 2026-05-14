#!/bin/bash

# Integration tests for req-coverage tool
# This script tests the complete workflow of the req-coverage tool

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="${SCRIPT_DIR}/test-data"
RESULTS_DIR="${SCRIPT_DIR}/results"
BINARY="${SCRIPT_DIR}/../../../target/debug/req-coverage"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Function to print test results
print_result() {
    local test_name=$1
    local status=$2
    local message=$3
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if [ "$status" == "PASS" ]; then
        echo -e "${GREEN}✓${NC} $test_name: PASSED"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗${NC} $test_name: FAILED - $message"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Function to setup test environment
setup_test_env() {
    rm -rf "$TEST_DIR" "$RESULTS_DIR"
    mkdir -p "$TEST_DIR/testcases"
    mkdir -p "$TEST_DIR/verification_results"
    mkdir -p "$RESULTS_DIR"
}

# Function to create a test case file
create_test_case() {
    local id=$1
    local requirement=$2
    local covers=$3
    local file="$TEST_DIR/testcases/${id}.yaml"
    
    if [ -n "$covers" ]; then
        cat > "$file" << EOF
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: $requirement
item: 1
tc: 1
id: $id
description: Test case for $requirement
requirement_coverage:
  type: partial
  covers: "$covers"
general_initial_conditions:
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
      result: 0
      output: test
    verification:
      result: '[[ \$EXIT_CODE -eq 0 ]]'
EOF
    else
        cat > "$file" << EOF
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: $requirement
item: 1
tc: 1
id: $id
description: Test case for $requirement
general_initial_conditions:
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
      result: 0
      output: test
    verification:
      result: '[[ \$EXIT_CODE -eq 0 ]]'
EOF
    fi
}

# Function to create verification result
create_verification_result() {
    local test_case_id=$1
    local passed=$2
    local file="$TEST_DIR/verification_results/${test_case_id}_container.yaml"
    
    cat > "$file" << EOF
title: Verification Results
project: Integration Test
test_date: "2024-01-20"
test_results:
  - test_case_id: $test_case_id
    description: Test result
    overall_pass: $passed
EOF
}

# Function to create requirements file
create_requirements_file() {
    local file="$TEST_DIR/requirements.yaml"
    cat > "$file" << 'EOF'
requirements:
  - id: REQ-001
    text: "authenticate users"
    description: "User authentication requirement"
  
  - id: REQ-002
    text: "The system shall log security events and validate input"
    description: "Security and validation requirement"
  
  - id: REQ-003
    text: "validate input data"
    description: "Input validation requirement"
EOF
}

# Build the binary if needed
echo "Building req-coverage..."
cd "${SCRIPT_DIR}/../../.."
cargo build -p req-coverage --quiet 2>&1 > /dev/null || {
    echo -e "${RED}Failed to build req-coverage${NC}"
    exit 1
}
cd "$SCRIPT_DIR"

echo -e "\n${YELLOW}=== Running Integration Tests ===${NC}\n"

# Test 1: Full coverage with single test case
echo "Test 1: Full coverage with single test case"
setup_test_env
create_requirements_file
create_test_case "TC-001" "REQ-001" "authenticate users"
create_verification_result "TC-001" "true"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test1_coverage.json" \
    --requirements-file "$TEST_DIR/requirements.yaml" \
    > "$RESULTS_DIR/test1.log" 2>&1

if [ $? -eq 0 ]; then
    # Check the results
    total_reqs=$(jq '.total_requirements' "$RESULTS_DIR/test1_coverage.json")
    fully_covered=$(jq '.fully_covered_requirements' "$RESULTS_DIR/test1_coverage.json")
    
    if [ "$total_reqs" == "1" ] && [ "$fully_covered" == "1" ]; then
        print_result "Test 1" "PASS"
    else
        print_result "Test 1" "FAIL" "Expected 1 total and 1 fully covered, got $total_reqs total and $fully_covered fully covered"
    fi
else
    print_result "Test 1" "FAIL" "Command failed"
fi

# Test 2: Partial coverage with multiple test cases
echo "Test 2: Partial coverage with multiple test cases"
setup_test_env
create_requirements_file
create_test_case "TC-001" "REQ-002" "log security events"
create_test_case "TC-002" "REQ-002" "validate input"
create_verification_result "TC-001" "true"
create_verification_result "TC-002" "true"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test2_coverage.json" \
    --requirements-file "$TEST_DIR/requirements.yaml" \
    > "$RESULTS_DIR/test2.log" 2>&1

if [ $? -eq 0 ]; then
    partially_covered=$(jq '.partially_covered_requirements' "$RESULTS_DIR/test2_coverage.json")
    
    if [ "$partially_covered" == "1" ]; then
        print_result "Test 2" "PASS"
    else
        print_result "Test 2" "FAIL" "Expected 1 partially covered, got $partially_covered"
    fi
else
    print_result "Test 2" "FAIL" "Command failed"
fi

# Test 3: Invalid covers string error detection
echo "Test 3: Invalid covers string error detection"
setup_test_env
create_requirements_file
create_test_case "TC-001" "REQ-001" "invalid text not in requirement"
create_verification_result "TC-001" "true"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test3_coverage.json" \
    --requirements-file "$TEST_DIR/requirements.yaml" \
    > "$RESULTS_DIR/test3.log" 2>&1

if [ $? -eq 0 ]; then
    has_errors=$(jq '.requirements[0].coverage_errors != null' "$RESULTS_DIR/test3_coverage.json")
    
    if [ "$has_errors" == "true" ]; then
        print_result "Test 3" "PASS"
    else
        print_result "Test 3" "FAIL" "Expected coverage errors"
    fi
else
    print_result "Test 3" "FAIL" "Command failed"
fi

# Test 4: Without requirements file (backward compatibility)
echo "Test 4: Backward compatibility without requirements file"
setup_test_env
create_test_case "TC-001" "REQ-001" "some text"
create_verification_result "TC-001" "true"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test4_coverage.json" \
    > "$RESULTS_DIR/test4.log" 2>&1

if [ $? -eq 0 ]; then
    total_reqs=$(jq '.total_requirements' "$RESULTS_DIR/test4_coverage.json")
    has_req_text=$(jq '.requirements[0].requirement_text' "$RESULTS_DIR/test4_coverage.json")
    
    if [ "$total_reqs" == "1" ] && [ "$has_req_text" == "null" ]; then
        print_result "Test 4" "PASS"
    else
        print_result "Test 4" "FAIL" "Expected 1 requirement with null requirement_text"
    fi
else
    print_result "Test 4" "FAIL" "Command failed"
fi

# Test 5: JSON requirements file format
echo "Test 5: JSON requirements file format support"
setup_test_env
cat > "$TEST_DIR/requirements.json" << 'EOF'
{
  "requirements": [
    {
      "id": "REQ-001",
      "text": "authenticate users",
      "description": "User authentication requirement"
    }
  ]
}
EOF

create_test_case "TC-001" "REQ-001" "authenticate users"
create_verification_result "TC-001" "true"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test5_coverage.json" \
    --requirements-file "$TEST_DIR/requirements.json" \
    > "$RESULTS_DIR/test5.log" 2>&1

if [ $? -eq 0 ]; then
    fully_covered=$(jq '.fully_covered_requirements' "$RESULTS_DIR/test5_coverage.json")
    
    if [ "$fully_covered" == "1" ]; then
        print_result "Test 5" "PASS"
    else
        print_result "Test 5" "FAIL" "Expected 1 fully covered"
    fi
else
    print_result "Test 5" "FAIL" "Command failed"
fi

# Test 6: HTML report generation
echo "Test 6: HTML report generation"
if [ -f "$RESULTS_DIR/test1_coverage.json" ]; then
    "$BINARY" print \
        --format html \
        --input "$RESULTS_DIR/test1_coverage.json" \
        --output "$RESULTS_DIR/html_report" \
        > "$RESULTS_DIR/test6.log" 2>&1
    
    if [ $? -eq 0 ] && [ -f "$RESULTS_DIR/html_report/index.html" ]; then
        print_result "Test 6" "PASS"
    else
        print_result "Test 6" "FAIL" "HTML report not generated"
    fi
else
    print_result "Test 6" "FAIL" "No coverage JSON available"
fi

# Test 7: Multiple requirements with different coverage states
echo "Test 7: Multiple requirements with different coverage states"
setup_test_env
cat > "$TEST_DIR/requirements.yaml" << 'EOF'
requirements:
  - id: REQ-001
    text: "authenticate users"
    description: "Requirement 1"
  
  - id: REQ-002
    text: "log security events"
    description: "Requirement 2"
  
  - id: REQ-003
    text: "validate input"
    description: "Requirement 3"
EOF

create_test_case "TC-001" "REQ-001" "authenticate users"
create_test_case "TC-002" "REQ-002" "log security"
create_verification_result "TC-001" "true"
create_verification_result "TC-002" "true"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test7_coverage.json" \
    --requirements-file "$TEST_DIR/requirements.yaml" \
    > "$RESULTS_DIR/test7.log" 2>&1

if [ $? -eq 0 ]; then
    total_reqs=$(jq '.total_requirements' "$RESULTS_DIR/test7_coverage.json")
    fully_covered=$(jq '.fully_covered_requirements' "$RESULTS_DIR/test7_coverage.json")
    partially_covered=$(jq '.partially_covered_requirements' "$RESULTS_DIR/test7_coverage.json")
    uncovered=$(jq '.uncovered_requirements' "$RESULTS_DIR/test7_coverage.json")
    
    if [ "$total_reqs" == "3" ] && [ "$fully_covered" == "1" ] && [ "$partially_covered" == "1" ] && [ "$uncovered" == "1" ]; then
        print_result "Test 7" "PASS"
    else
        print_result "Test 7" "FAIL" "Expected 3 total, 1 full, 1 partial, 1 uncovered. Got: $total_reqs total, $fully_covered full, $partially_covered partial, $uncovered uncovered"
    fi
else
    print_result "Test 7" "FAIL" "Command failed"
fi

# Test 8: Test with failures
echo "Test 8: Coverage with failing tests"
setup_test_env
create_requirements_file
create_test_case "TC-001" "REQ-001" "authenticate users"
create_verification_result "TC-001" "false"

"$BINARY" verify \
    --test-cases-folder "$TEST_DIR/testcases" \
    --test-results-folder "$TEST_DIR/verification_results" \
    --output "$RESULTS_DIR/test8_coverage.json" \
    --requirements-file "$TEST_DIR/requirements.yaml" \
    > "$RESULTS_DIR/test8.log" 2>&1

if [ $? -eq 0 ]; then
    status=$(jq -r '.requirements[0].status' "$RESULTS_DIR/test8_coverage.json")
    
    if [ "$status" == "covered_fail" ]; then
        print_result "Test 8" "PASS"
    else
        print_result "Test 8" "FAIL" "Expected covered_fail status, got $status"
    fi
else
    print_result "Test 8" "FAIL" "Command failed"
fi

# Summary
echo -e "\n${YELLOW}=== Test Summary ===${NC}"
echo "Total tests run: $TESTS_RUN"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
if [ $TESTS_FAILED -gt 0 ]; then
    echo -e "${RED}Failed: $TESTS_FAILED${NC}"
fi

# Save summary
cat > "$RESULTS_DIR/test_summary.txt" << EOF
Integration Test Results
========================
Date: $(date)
Total tests: $TESTS_RUN
Passed: $TESTS_PASSED
Failed: $TESTS_FAILED

Test Details:
EOF

ls -1 "$RESULTS_DIR"/*.log 2>/dev/null | while read log_file; do
    echo "" >> "$RESULTS_DIR/test_summary.txt"
    echo "=== $(basename $log_file) ===" >> "$RESULTS_DIR/test_summary.txt"
    cat "$log_file" >> "$RESULTS_DIR/test_summary.txt"
done

echo -e "\nResults saved to: $RESULTS_DIR/"
echo "Summary: $RESULTS_DIR/test_summary.txt"

# Exit with appropriate code
if [ $TESTS_FAILED -gt 0 ]; then
    exit 1
else
    exit 0
fi
