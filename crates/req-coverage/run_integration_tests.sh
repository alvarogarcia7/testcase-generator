#!/bin/bash

# Integration Test Runner for req-coverage
# This script runs the integration tests and saves the results

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="${SCRIPT_DIR}/test_results"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULTS_FILE="${RESULTS_DIR}/integration_test_results_${TIMESTAMP}.txt"

# Create results directory if it doesn't exist
mkdir -p "${RESULTS_DIR}"

echo "======================================"
echo "req-coverage Integration Test Runner"
echo "======================================"
echo "Timestamp: $(date)"
echo "Results will be saved to: ${RESULTS_FILE}"
echo ""

# Save header to results file
{
    echo "======================================"
    echo "req-coverage Integration Test Results"
    echo "======================================"
    echo "Timestamp: $(date)"
    echo "Host: $(hostname)"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
} > "${RESULTS_FILE}"

# Function to run tests and capture output
run_tests() {
    echo "Running integration tests..."
    echo ""
    
    # Run the tests and capture output
    if cargo test -p req-coverage --test string_verification_tests -- --nocapture 2>&1 | tee -a "${RESULTS_FILE}"; then
        echo ""
        echo "✅ All integration tests PASSED!"
        TEST_STATUS="PASSED"
    else
        echo ""
        echo "⚠️  Some integration tests FAILED - see results for details"
        TEST_STATUS="FAILED"
    fi
    
    return 0
}

# Function to run unit tests
run_unit_tests() {
    echo "Running unit tests..."
    echo ""
    
    {
        echo "======================================"
        echo "Unit Test Results"
        echo "======================================"
        echo ""
    } >> "${RESULTS_FILE}"
    
    if cargo test -p req-coverage --lib 2>&1 | tee -a "${RESULTS_FILE}"; then
        echo ""
        echo "✅ All unit tests PASSED!"
        UNIT_TEST_STATUS="PASSED"
    else
        echo ""
        echo "⚠️  Some unit tests FAILED"
        UNIT_TEST_STATUS="FAILED"
    fi
    
    return 0
}

# Run unit tests first
run_unit_tests

echo ""
echo "======================================"
echo ""

# Run integration tests
{
    echo ""
    echo "======================================"
    echo "Integration Test Results"
    echo "======================================"
    echo ""
} >> "${RESULTS_FILE}"

run_tests

# Generate summary
{
    echo ""
    echo "======================================"
    echo "Test Summary"
    echo "======================================"
    echo "Unit Tests: ${UNIT_TEST_STATUS}"
    echo "Integration Tests: ${TEST_STATUS}"
    echo "Completed at: $(date)"
    echo "======================================"
} >> "${RESULTS_FILE}"

echo ""
echo "======================================"
echo "Test Summary"
echo "======================================"
echo "Unit Tests: ${UNIT_TEST_STATUS}"
echo "Integration Tests: ${TEST_STATUS}"
echo ""
echo "Full results saved to: ${RESULTS_FILE}"
echo ""

# Create a symlink to the latest results
ln -sf "$(basename "${RESULTS_FILE}")" "${RESULTS_DIR}/latest_results.txt"
echo "Latest results also available at: ${RESULTS_DIR}/latest_results.txt"
echo ""

# List all result files
echo "All test result files:"
ls -lht "${RESULTS_DIR}"/integration_test_results_*.txt | head -5

exit 0
