#!/bin/bash
#
# CI-friendly test script for integration tests
# Returns proper exit codes and outputs in a CI-parseable format
#
# Usage: ./tests/integration/ci_test.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BINARY="$PROJECT_ROOT/target/debug/testcase-manager"

echo "::group::Integration Test Setup"
echo "Project root: $PROJECT_ROOT"
echo "Binary: $BINARY"
echo "Expect version: $(expect -version 2>&1 || echo 'not installed')"
echo "::endgroup::"

# Verify prerequisites
if ! command -v expect &> /dev/null; then
    echo "::error::expect command not found"
    exit 1
fi

if [[ ! -f "$BINARY" ]]; then
    echo "::error::Binary not found at $BINARY"
    exit 1
fi

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test
run_test() {
    local test_name=$1
    local test_script=$2
    
    ((TOTAL_TESTS++))
    
    echo "::group::Running $test_name"
    
    if "$test_script" "$BINARY"; then
        echo "::notice::✓ $test_name PASSED"
        ((PASSED_TESTS++))
        echo "::endgroup::"
        return 0
    else
        echo "::error::✗ $test_name FAILED"
        ((FAILED_TESTS++))
        echo "::endgroup::"
        return 1
    fi
}

# Run tests
run_test "Basic Workflow Test" "$SCRIPT_DIR/e2e_basic_workflow.exp" || true
run_test "Complete Workflow Test" "$SCRIPT_DIR/e2e_complete_workflow.exp" || true

# Summary
echo "::group::Test Summary"
echo "Total: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo "::endgroup::"

if [[ $FAILED_TESTS -gt 0 ]]; then
    echo "::error::$FAILED_TESTS test(s) failed"
    exit 1
else
    echo "::notice::All $PASSED_TESTS test(s) passed"
    exit 0
fi
