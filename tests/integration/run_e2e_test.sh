#!/bin/bash
#
# Wrapper script to run the end-to-end integration test
#
# Usage: ./tests/integration/run_e2e_test.sh [--build]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
E2E_TEST="$SCRIPT_DIR/e2e_complete_workflow.exp"

# Source shared library for finding binaries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"

# Check if --build flag is provided
BUILD=false
if [[ "$1" == "--build" ]]; then
    BUILD=true
fi

echo "======================================"
echo "E2E Integration Test Runner"
echo "======================================"
echo "Project root: $PROJECT_ROOT"
echo "Test script: $E2E_TEST"
echo ""

# Build the project if requested
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
    echo "Run with --build flag to build the project first"
    exit 1
fi
echo "Using binary: $BINARY"
echo ""

# Check if expect is installed
if ! command -v expect &> /dev/null; then
    echo "ERROR: expect command not found"
    echo "Please install expect:"
    echo "  - Ubuntu/Debian: sudo apt-get install expect"
    echo "  - macOS: brew install expect"
    echo "  - RHEL/CentOS: sudo yum install expect"
    exit 1
fi

# Make test script executable
chmod +x "$E2E_TEST"

# Run the test
echo "==> Running E2E integration test..."
echo ""

cd "$PROJECT_ROOT"
"$E2E_TEST" "$BINARY"

exit $?
