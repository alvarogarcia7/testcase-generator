#!/bin/bash
#
# Run all integration tests
#
# Usage: ./tests/integration/run_all_tests.sh [--build]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source shared library for finding binaries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"

BUILD=false
if [[ "$1" == "--build" ]]; then
    BUILD=true
fi

echo "=========================================="
echo "Running All Integration Tests"
echo "=========================================="
echo ""

# Build if requested
if [[ "$BUILD" == true ]]; then
    echo "==> Building project..."
    cd "$PROJECT_ROOT"
    cargo build
    echo "✓ Build complete"
    echo ""
fi

# Find the binary
cd "$PROJECT_ROOT"
BINARY=$(find_binary "testcase-manager")
if [[ -z "$BINARY" ]]; then
    echo "ERROR: Binary not found in target/release or target/debug"
    echo "Run with --build flag to build first"
    exit 1
fi
echo "Using binary: $BINARY"
echo ""

# Check expect is installed
if ! command -v expect &> /dev/null; then
    echo "ERROR: expect command not found"
    exit 1
fi

TESTS_PASSED=0
TESTS_FAILED=0

# Run basic test
echo "=========================================="
echo "Test 1: Basic Workflow"
echo "=========================================="
if "$SCRIPT_DIR/e2e_basic_workflow.exp" "$BINARY"; then
    echo "✓ Basic workflow test PASSED"
    TESTS_PASSED=$((TESTS_PASSED+1))
else
    echo "✗ Basic workflow test FAILED"
    TESTS_FAILED=$((TESTS_FAILED+1))
fi
echo ""

# Run complete test
echo "=========================================="
echo "Test 2: Complete Workflow"
echo "=========================================="
if "$SCRIPT_DIR/e2e_complete_workflow.exp" "$BINARY"; then
    echo "✓ Complete workflow test PASSED"
    TESTS_PASSED=$((TESTS_PASSED+1))
else
    echo "✗ Complete workflow test FAILED"
    TESTS_FAILED=$((TESTS_FAILED+1))
fi
echo ""

# Run validate-files.sh integration tests
echo "=========================================="
echo "Test 3: validate-files.sh Integration"
echo "=========================================="
if "$SCRIPT_DIR/validate_files_integration.exp"; then
    echo "✓ validate-files.sh test PASSED"
    TESTS_PASSED=$((TESTS_PASSED+1))
else
    echo "✗ validate-files.sh test FAILED"
    TESTS_FAILED=$((TESTS_FAILED+1))
fi
echo ""

# Run variable display e2e test
echo "=========================================="
echo "Test 4: Variable Display E2E"
echo "=========================================="
if "$SCRIPT_DIR/test_variable_display_e2e.sh"; then
    echo "✓ Variable display e2e test PASSED"
    TESTS_PASSED=$((TESTS_PASSED+1))
else
    echo "✗ Variable display e2e test FAILED"
    TESTS_FAILED=$((TESTS_FAILED+1))
fi
echo ""

# Summary
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -gt 0 ]]; then
    echo "Some tests FAILED ✗"
    exit 1
else
    echo "All tests PASSED ✓"
    exit 0
fi
