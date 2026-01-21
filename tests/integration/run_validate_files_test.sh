#!/usr/bin/env bash
#
# Test Runner for validate-files.sh Integration Tests
#
# This script runs the comprehensive integration test suite for validate-files.sh,
# which validates the script's functionality including caching, pattern matching,
# error handling, and more.
#
# Usage: ./tests/integration/run_validate_files_test.sh
#

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_SCRIPT="$SCRIPT_DIR/validate_files_integration.exp"

echo ""
echo "=========================================="
echo "validate-files.sh Integration Test Runner"
echo "=========================================="
echo ""

# Check if expect is installed
if ! command -v expect >/dev/null 2>&1; then
    echo "ERROR: 'expect' is not installed"
    echo ""
    echo "Please install expect:"
    echo "  Ubuntu/Debian: sudo apt-get install expect"
    echo "  macOS:         brew install expect"
    echo "  RHEL/CentOS:   sudo yum install expect"
    echo ""
    exit 1
fi

# Check if test script exists
if [[ ! -f "$TEST_SCRIPT" ]]; then
    echo "ERROR: Test script not found: $TEST_SCRIPT"
    exit 1
fi

# Check if test script is executable
if [[ ! -x "$TEST_SCRIPT" ]]; then
    echo "Making test script executable..."
    chmod +x "$TEST_SCRIPT"
fi

# Check if validate-files.sh exists
if [[ ! -f "$SCRIPT_DIR/../../scripts/validate-files.sh" ]]; then
    echo "ERROR: validate-files.sh not found at scripts/validate-files.sh"
    exit 1
fi

echo "Running integration test suite..."
echo ""

# Run the test
if "$TEST_SCRIPT"; then
    echo ""
    echo "=========================================="
    echo "✓ Integration tests PASSED"
    echo "=========================================="
    echo ""
    exit 0
else
    EXIT_CODE=$?
    echo ""
    echo "=========================================="
    echo "✗ Integration tests FAILED"
    echo "=========================================="
    echo ""
    exit $EXIT_CODE
fi
