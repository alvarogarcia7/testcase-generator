#!/bin/bash
#
# Wrapper script to run the end-to-end integration test
#
# Usage: ./tests/integration/run_e2e_test.sh [--build]
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
E2E_TEST="$SCRIPT_DIR/e2e_complete_workflow.exp"

# Source shared libraries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"
source "$PROJECT_ROOT/scripts/lib/logger.sh" || exit 1

# Check if --build flag is provided
BUILD=false
if [[ "$1" == "--build" ]]; then
    BUILD=true
fi

section "E2E Integration Test Runner"
info "Project root: $PROJECT_ROOT"
info "Test script: $E2E_TEST"

# Build the project if requested
if [[ "$BUILD" == true ]]; then
    log_info "Building project..."
    cd "$PROJECT_ROOT"
    cargo build
    pass "Build complete"
fi

# Find the binary
cd "$PROJECT_ROOT"
BINARY=$(find_binary "trm")
if [[ -z "$BINARY" ]]; then
    fail "Binary not found in target/release or target/debug"
    log_error "Run with --build flag to build the project first"
    exit 1
fi
info "Using binary: $BINARY"

# Check if expect is installed
if ! command -v expect &> /dev/null; then
    fail "expect command not found"
    log_error "Please install expect:"
    log_info "  - Ubuntu/Debian: sudo apt-get install expect"
    log_info "  - macOS: brew install expect"
    log_info "  - RHEL/CentOS: sudo yum install expect"
    exit 1
fi

# Make test script executable
chmod +x "$E2E_TEST"

# Run the test
log_info "Running E2E integration test..."

cd "$PROJECT_ROOT"
"$E2E_TEST" "$BINARY"

exit $?
