#!/bin/bash
#
# Ultra-quick smoke test to verify basic functionality
# Runs in <5 seconds for immediate feedback
#
# Usage: ./tests/integration/smoke_test.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source shared library for finding binaries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"

# Change to project root for binary search
cd "$PROJECT_ROOT"

echo "=== Quick Smoke Test ==="

# Find the testcase-manager binary
BINARY=$(find_binary "testcase-manager")
if [[ -z "$BINARY" ]]; then
    echo "✗ Binary not found in target/release or target/debug"
    echo "  Run: cargo build"
    exit 1
fi
echo "✓ Binary found: $BINARY"

# Check help command works
if "$BINARY" --help > /dev/null 2>&1; then
    echo "✓ Help command works"
else
    echo "✗ Help command failed"
    exit 1
fi

# Check version command works
if "$BINARY" --version > /dev/null 2>&1; then
    echo "✓ Version command works"
else
    echo "✗ Version command failed"
    exit 1
fi

# Check expect is available
if command -v expect &> /dev/null; then
    echo "✓ Expect available"
else
    echo "⚠ Expect not available (integration tests will fail)"
fi

# Check git is available
if command -v git &> /dev/null; then
    echo "✓ Git available"
else
    echo "✗ Git not available"
    exit 1
fi

echo ""
echo "=== Smoke Test Passed ✓ ==="
echo "Ready to run full integration tests:"
echo "  make test-e2e        # Run complete workflow test"
echo "  make test-e2e-all    # Run all integration tests"

exit 0
