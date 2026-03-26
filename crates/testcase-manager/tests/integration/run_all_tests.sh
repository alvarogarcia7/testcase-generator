#!/bin/bash
#
# Run all integration tests
#
# Usage: ./tests/integration/run_all_tests.sh [--build]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# Source shared libraries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

BUILD=false
if [[ "$1" == "--build" ]]; then
    BUILD=true
fi

section "Running All Integration Tests"

# Build if requested
if [[ "$BUILD" == true ]]; then
    log_info "Building project..."
    cd "$PROJECT_ROOT"
    cargo build
    echo "✓ Build complete"
    echo ""
fi

# Find the binary
cd "$PROJECT_ROOT"
BINARY=$(find_binary "testcase-manager")
if [[ -z "$BINARY" ]]; then
    fail "Binary not found in target/release or target/debug"
    log_error "Run with --build flag to build first"
    exit 1
fi
info "Using binary: $BINARY"

# Check expect is installed
if ! command -v expect &> /dev/null; then
    echo "ERROR: expect command not found"
    exit 1
fi

TESTS_PASSED=0
TESTS_FAILED=0

# Run basic test
section "Test 1: Basic Workflow"
if "$SCRIPT_DIR/e2e_basic_workflow.exp" "$BINARY"; then
    pass "Basic workflow test PASSED"
    TESTS_PASSED=$((TESTS_PASSED+1))
else
    fail "Basic workflow test FAILED"
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
