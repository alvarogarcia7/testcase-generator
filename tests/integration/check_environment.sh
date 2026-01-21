#!/bin/bash
#
# Check if the environment is ready for running integration tests
#
# Usage: ./tests/integration/check_environment.sh
#

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Source shared library for finding binaries
source "$PROJECT_ROOT/scripts/lib/find-binary.sh"

echo "=========================================="
echo "Integration Test Environment Check"
echo "=========================================="
echo ""

ERRORS=0
WARNINGS=0

# Check for expect
echo -n "Checking for expect... "
if command -v expect &> /dev/null; then
    EXPECT_VERSION=$(expect -version 2>&1 | head -n1)
    echo "✓ Found: $EXPECT_VERSION"
else
    echo "✗ NOT FOUND"
    echo "  Install: sudo apt-get install expect (Ubuntu/Debian)"
    echo "           brew install expect (macOS)"
    ((ERRORS++))
fi

# Check for git
echo -n "Checking for git... "
if command -v git &> /dev/null; then
    GIT_VERSION=$(git --version)
    echo "✓ Found: $GIT_VERSION"
else
    echo "✗ NOT FOUND"
    ((ERRORS++))
fi

# Check for cargo
echo -n "Checking for cargo... "
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo "✓ Found: $CARGO_VERSION"
else
    echo "✗ NOT FOUND"
    ((ERRORS++))
fi

# Check if binary exists
echo -n "Checking for testcase-manager binary... "
cd "$PROJECT_ROOT"
BINARY=$(find_binary "testcase-manager")
if [[ -n "$BINARY" ]]; then
    echo "✓ Found at $BINARY"
else
    echo "⚠ NOT FOUND"
    echo "  Run: cargo build"
    ((WARNINGS++))
fi

# Check for test scripts
echo -n "Checking for test scripts... "
MISSING_SCRIPTS=0

for script in e2e_basic_workflow.exp e2e_complete_workflow.exp run_e2e_test.sh run_all_tests.sh; do
    if [[ ! -f "$SCRIPT_DIR/$script" ]]; then
        echo "✗ Missing: $script"
        ((MISSING_SCRIPTS++))
    fi
done

if [[ $MISSING_SCRIPTS -eq 0 ]]; then
    echo "✓ All test scripts present"
else
    echo "✗ Missing $MISSING_SCRIPTS script(s)"
    ((ERRORS++))
fi

# Check for executable permissions
echo -n "Checking script permissions... "
MISSING_EXEC=0

for script in e2e_basic_workflow.exp e2e_complete_workflow.exp run_e2e_test.sh run_all_tests.sh ci_test.sh; do
    if [[ -f "$SCRIPT_DIR/$script" && ! -x "$SCRIPT_DIR/$script" ]]; then
        ((MISSING_EXEC++))
    fi
done

if [[ $MISSING_EXEC -eq 0 ]]; then
    echo "✓ All scripts executable"
else
    echo "⚠ $MISSING_EXEC script(s) not executable"
    echo "  Run: chmod +x tests/integration/*.sh tests/integration/*.exp"
    ((WARNINGS++))
fi

# Check git configuration
echo -n "Checking git configuration... "
GIT_USER_NAME=$(git config user.name 2>/dev/null)
GIT_USER_EMAIL=$(git config user.email 2>/dev/null)

if [[ -n "$GIT_USER_NAME" && -n "$GIT_USER_EMAIL" ]]; then
    echo "✓ Git configured ($GIT_USER_NAME <$GIT_USER_EMAIL>)"
else
    echo "⚠ Git not fully configured"
    echo "  Tests will use default values"
    ((WARNINGS++))
fi

# Check for leftover test directories
echo -n "Checking for leftover test artifacts... "
LEFTOVER_COUNT=$(ls -d test_e2e_* test_basic_* 2>/dev/null | wc -l)
if [[ $LEFTOVER_COUNT -eq 0 ]]; then
    echo "✓ No leftover artifacts"
else
    echo "⚠ Found $LEFTOVER_COUNT leftover test director(ies)"
    echo "  Run: rm -rf test_e2e_* test_basic_*"
    ((WARNINGS++))
fi

# Check disk space
echo -n "Checking disk space... "
AVAILABLE_KB=$(df . | tail -1 | awk '{print $4}')
AVAILABLE_MB=$((AVAILABLE_KB / 1024))

if [[ $AVAILABLE_MB -gt 100 ]]; then
    echo "✓ ${AVAILABLE_MB}MB available"
else
    echo "⚠ Only ${AVAILABLE_MB}MB available"
    ((WARNINGS++))
fi

# Summary
echo ""
echo "=========================================="
echo "Summary"
echo "=========================================="
echo "Errors: $ERRORS"
echo "Warnings: $WARNINGS"
echo ""

if [[ $ERRORS -eq 0 ]]; then
    echo "✓ Environment is ready for integration tests"
    if [[ $WARNINGS -gt 0 ]]; then
        echo "  Note: $WARNINGS warning(s) found but tests should still work"
    fi
    exit 0
else
    echo "✗ Environment is NOT ready"
    echo "  Please fix the errors listed above"
    exit 1
fi
