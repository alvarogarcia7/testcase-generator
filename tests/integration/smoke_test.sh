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
BINARY="$PROJECT_ROOT/target/debug/testcase-manager"

echo "=== Quick Smoke Test ==="

# Check binary exists
if [[ ! -f "$BINARY" ]]; then
    echo "✗ Binary not found"
    exit 1
fi
echo "✓ Binary found"

# Check binary is executable
if [[ ! -x "$BINARY" ]]; then
    echo "✗ Binary not executable"
    exit 1
fi
echo "✓ Binary executable"

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
