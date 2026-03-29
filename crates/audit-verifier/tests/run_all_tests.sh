#!/usr/bin/env bash
# Run all audit-verifier integration tests
#
# This script runs all shell-based integration tests for the audit-verifier crate.
#
# Usage: ./tests/run_all_tests.sh [--no-remove]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Parse arguments
NO_REMOVE_FLAG=""
if [[ "$1" == "--no-remove" ]]; then
    NO_REMOVE_FLAG="--no-remove"
fi

echo "========================================"
echo "Running All audit-verifier Tests"
echo "========================================"
echo ""

# Array to track test results
declare -a TEST_RESULTS
declare -a TEST_NAMES

run_test() {
    local test_script="$1"
    local test_name="$2"
    
    echo ""
    echo "========================================="
    echo "Running: $test_name"
    echo "========================================="
    echo ""
    
    if "$test_script" $NO_REMOVE_FLAG; then
        TEST_RESULTS+=("PASS")
        TEST_NAMES+=("$test_name")
        echo ""
        echo "✓ $test_name PASSED"
        echo ""
        return 0
    else
        TEST_RESULTS+=("FAIL")
        TEST_NAMES+=("$test_name")
        echo ""
        echo "✗ $test_name FAILED"
        echo ""
        return 1
    fi
}

# Build the binaries first
echo "Building audit-verifier binaries..."
cd "$PROJECT_ROOT"
if cargo build -p audit-verifier --quiet 2>&1; then
    echo "✓ Build successful"
else
    echo "✗ Build failed"
    exit 1
fi
echo ""

# Run all integration tests
FAILED_TESTS=0

run_test "$SCRIPT_DIR/integration/test_simple_workflow.sh" "Simple Workflow Test" || FAILED_TESTS=$((FAILED_TESTS + 1))
run_test "$SCRIPT_DIR/integration/test_audit_verifier_e2e.sh" "E2E Integration Test" || FAILED_TESTS=$((FAILED_TESTS + 1))
run_test "$SCRIPT_DIR/integration/test_audit_key_scenarios.sh" "Key Management Scenarios Test" || FAILED_TESTS=$((FAILED_TESTS + 1))

# Summary
echo ""
echo "========================================"
echo "Test Results Summary"
echo "========================================"
echo ""

for i in "${!TEST_NAMES[@]}"; do
    test_name="${TEST_NAMES[$i]}"
    test_result="${TEST_RESULTS[$i]}"
    
    if [[ "$test_result" == "PASS" ]]; then
        echo "✓ $test_name"
    else
        echo "✗ $test_name"
    fi
done

echo ""
echo "========================================"

TOTAL_TESTS=${#TEST_NAMES[@]}
PASSED_TESTS=$((TOTAL_TESTS - FAILED_TESTS))

echo "Total: $TOTAL_TESTS tests"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo ""

if [[ $FAILED_TESTS -eq 0 ]]; then
    echo "✓ All tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
