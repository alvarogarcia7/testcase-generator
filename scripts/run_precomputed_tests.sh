#!/bin/bash

# Script to run all precomputed test cases and generate results

set -e

VERIFIER="./target/debug/verifier"
OUTPUT_DIR="test_results/precomputed"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "=================================================="
echo "Running Precomputed Test Cases"
echo "Timestamp: $TIMESTAMP"
echo "=================================================="
echo ""

# Initialize counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a single test case
run_test() {
    local test_case_id=$1
    local log_file=$2
    local expected_result=$3  # "pass" or "fail"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo "[$TOTAL_TESTS] Running: $test_case_id"
    echo "  Log file: $log_file"
    
    # Run verifier and capture output
    local output_file="$OUTPUT_DIR/${test_case_id}_${TIMESTAMP}.yaml"
    
    if $VERIFIER --log "$log_file" --test-case "$test_case_id" --match-strategy precomputed --format yaml > "$output_file" 2>&1; then
        local actual_result="pass"
        local exit_code=0
    else
        local actual_result="fail"
        local exit_code=$?
    fi
    
    # Check if result matches expectation
    if [ "$actual_result" == "$expected_result" ]; then
        echo "  Result: ✅ $actual_result (as expected)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "  Result: ❌ $actual_result (expected: $expected_result)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    
    echo "  Output saved to: $output_file"
    echo ""
}

# Successful test cases (expected to pass)
echo "=== Successful Test Cases ==="
echo ""

run_test "TEST_PRECOMP_ALL_PASS_001" \
    "testcases/verifier_scenarios/successful/TEST_PRECOMP_ALL_PASS_001_execution_log.json" \
    "pass"

run_test "TEST_PRECOMP_SINGLE_STEP_001" \
    "testcases/verifier_scenarios/successful/TEST_PRECOMP_SINGLE_STEP_001_execution_log.json" \
    "pass"

run_test "TEST_PRECOMP_EMPTY_OUTPUT_001" \
    "testcases/verifier_scenarios/successful/TEST_PRECOMP_EMPTY_OUTPUT_001_execution_log.json" \
    "pass"

run_test "TEST_PRECOMP_LONG_OUTPUT_001" \
    "testcases/verifier_scenarios/successful/TEST_PRECOMP_LONG_OUTPUT_001_execution_log.json" \
    "pass"

run_test "TEST_PRECOMP_FIVE_STEPS_001" \
    "testcases/verifier_scenarios/successful/TEST_PRECOMP_FIVE_STEPS_001_execution_log.json" \
    "pass"

run_test "TEST_PRECOMP_SPECIAL_CHARS_001" \
    "testcases/verifier_scenarios/successful/TEST_PRECOMP_SPECIAL_CHARS_001_execution_log.json" \
    "pass"

run_test "TEST_PRECOMP_NUMERIC_OUTPUT_001" \
    "testcases/verifier_scenarios/edge_cases/TEST_PRECOMP_NUMERIC_OUTPUT_001_execution_log.json" \
    "pass"

# Failed first test cases (expected to fail)
echo "=== Failed First Test Cases ==="
echo ""

run_test "TEST_PRECOMP_RESULT_FAIL_001" \
    "testcases/verifier_scenarios/failed_first/TEST_PRECOMP_RESULT_FAIL_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_OUTPUT_FAIL_001" \
    "testcases/verifier_scenarios/failed_first/TEST_PRECOMP_OUTPUT_FAIL_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_BOTH_FAIL_001" \
    "testcases/verifier_scenarios/failed_first/TEST_PRECOMP_BOTH_FAIL_001_execution_log.json" \
    "fail"

# Failed last test cases (expected to fail)
echo "=== Failed Last Test Cases ==="
echo ""

run_test "TEST_PRECOMP_LAST_FAIL_001" \
    "testcases/verifier_scenarios/failed_last/TEST_PRECOMP_LAST_FAIL_001_execution_log.json" \
    "fail"

# Failed intermediate test cases (expected to fail)
echo "=== Failed Intermediate Test Cases ==="
echo ""

run_test "TEST_PRECOMP_MID_FAIL_001" \
    "testcases/verifier_scenarios/failed_intermediate/TEST_PRECOMP_MID_FAIL_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_TWO_FAILURES_001" \
    "testcases/verifier_scenarios/failed_intermediate/TEST_PRECOMP_TWO_FAILURES_001_execution_log.json" \
    "fail"

# Multiple sequences test cases (expected to fail)
echo "=== Multiple Sequences Test Cases ==="
echo ""

run_test "TEST_PRECOMP_MULTI_SEQ_001" \
    "testcases/verifier_scenarios/multiple_sequences/TEST_PRECOMP_MULTI_SEQ_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_THREE_SEQ_001" \
    "testcases/verifier_scenarios/multiple_sequences/TEST_PRECOMP_THREE_SEQ_001_execution_log.json" \
    "fail"

# Edge cases (expected to fail)
echo "=== Edge Cases ==="
echo ""

run_test "TEST_PRECOMP_PARTIAL_EXEC_001" \
    "testcases/verifier_scenarios/edge_cases/TEST_PRECOMP_PARTIAL_EXEC_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_MIXED_RESULTS_001" \
    "testcases/verifier_scenarios/edge_cases/TEST_PRECOMP_MIXED_RESULTS_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_ALL_FAIL_001" \
    "testcases/verifier_scenarios/edge_cases/TEST_PRECOMP_ALL_FAIL_001_execution_log.json" \
    "fail"

run_test "TEST_PRECOMP_ONLY_RESULT_PASS_001" \
    "testcases/verifier_scenarios/edge_cases/TEST_PRECOMP_ONLY_RESULT_PASS_001_execution_log.json" \
    "fail"

# Summary
echo "=================================================="
echo "Test Summary"
echo "=================================================="
echo "Total tests run: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo ""
echo "Results saved to: $OUTPUT_DIR"
echo "=================================================="

# Exit with failure if any tests didn't match expectations
if [ $FAILED_TESTS -gt 0 ]; then
    exit 1
fi

exit 0
