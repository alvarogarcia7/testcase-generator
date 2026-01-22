#!/bin/bash
#
# E2E integration test for test-orchestrator
#
# This test validates:
# 1. test-orchestrator binary exists
# 2. test-orchestrator run executes test case and captures execution log
# 3. test-orchestrator verify validates execution log against test case
# 4. Verification output contains expected messages and statistics
# 5. Color-coded pass/fail indicators are present
#
# Usage: ./tests/integration/test_orchestrator_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_ORCHESTRATOR_BINARY="$PROJECT_ROOT/target/debug/test-orchestrator"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

echo "=========================================="
echo "Test Orchestrator E2E Integration Test"
echo "=========================================="
echo ""

# Helper functions
pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((TESTS_PASSED++))
}

fail() {
    echo -e "${RED}✗${NC} $1"
    ((TESTS_FAILED++))
}

info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

section() {
    echo ""
    echo -e "${YELLOW}=== $1 ===${NC}"
}

# ============================================================================
# Test 1: Check binary exists
# ============================================================================
section "Test 1: Checking Prerequisites"

if [[ ! -f "$TEST_ORCHESTRATOR_BINARY" ]]; then
    fail "test-orchestrator binary not found at $TEST_ORCHESTRATOR_BINARY"
    echo "Run 'cargo build' first"
    exit 1
fi
pass "test-orchestrator binary found"

# Check test case file exists
TEST_CASE_FILE="$PROJECT_ROOT/testcases/self_validated_example.yml"
if [[ ! -f "$TEST_CASE_FILE" ]]; then
    fail "Test case file not found at $TEST_CASE_FILE"
    exit 1
fi
pass "Test case file found: self_validated_example.yml"

# Check execution log exists
EXECUTION_LOG_FILE="$PROJECT_ROOT/testcases/self_validated_example_execution_log.json"
if [[ ! -f "$EXECUTION_LOG_FILE" ]]; then
    fail "Execution log file not found at $EXECUTION_LOG_FILE"
    exit 1
fi
pass "Execution log file found: self_validated_example_execution_log.json"

# ============================================================================
# Test 2: Run test-orchestrator run to capture execution
# ============================================================================
section "Test 2: Running test-orchestrator run"

# Create temporary directory for test output
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

info "Using temporary directory: $TEMP_DIR"

RUN_OUTPUT="$TEMP_DIR/run_output.txt"
RUN_EXIT_CODE=0

# Run test-orchestrator with the self-validated test case
if "$TEST_ORCHESTRATOR_BINARY" --path "$PROJECT_ROOT/testcases" --output "$TEMP_DIR" run SELF_VALIDATED_EXAMPLE_001 -v > "$RUN_OUTPUT" 2>&1; then
    RUN_EXIT_CODE=$?
    pass "test-orchestrator run executed successfully (exit code: $RUN_EXIT_CODE)"
else
    RUN_EXIT_CODE=$?
    info "test-orchestrator run completed with exit code: $RUN_EXIT_CODE"
fi

# Check that execution log output was captured
if [[ -s "$RUN_OUTPUT" ]]; then
    pass "Execution run output captured"
else
    fail "Execution run output is empty"
fi

# Verify output contains execution information
if grep -q "Executing test" "$RUN_OUTPUT" || grep -q "Test execution" "$RUN_OUTPUT" || grep -q "Running" "$RUN_OUTPUT"; then
    pass "Run output contains execution indicators"
else
    info "Run output may not contain expected execution indicators (checking alternative patterns)"
    if grep -q "SELF_VALIDATED_EXAMPLE_001" "$RUN_OUTPUT"; then
        pass "Run output contains test case ID"
    else
        fail "Run output missing execution indicators"
    fi
fi

# ============================================================================
# Test 3: Run test-orchestrator verify with specific test case and log
# ============================================================================
section "Test 3: Running test-orchestrator verify"

VERIFY_OUTPUT="$TEMP_DIR/verify_output.txt"
VERIFY_EXIT_CODE=0

# Run verification with specific test case and execution log
if "$TEST_ORCHESTRATOR_BINARY" --path "$PROJECT_ROOT/testcases" verify \
    --test-case "$TEST_CASE_FILE" \
    --execution-log "$EXECUTION_LOG_FILE" > "$VERIFY_OUTPUT" 2>&1; then
    VERIFY_EXIT_CODE=$?
    pass "test-orchestrator verify executed successfully (exit code: 0)"
else
    VERIFY_EXIT_CODE=$?
    fail "test-orchestrator verify failed with exit code: $VERIFY_EXIT_CODE"
fi

# Check exit code is 0 (success)
if [[ $VERIFY_EXIT_CODE -eq 0 ]]; then
    pass "Verify command returned exit code 0"
else
    fail "Verify command returned non-zero exit code: $VERIFY_EXIT_CODE"
fi

# Check that verification output was captured
if [[ -s "$VERIFY_OUTPUT" ]]; then
    pass "Verification output captured"
else
    fail "Verification output is empty"
fi

# ============================================================================
# Test 4: Validate verification output contains expected messages
# ============================================================================
section "Test 4: Validating Verification Output Content"

# Check for test case ID
if grep -q "SELF_VALIDATED_EXAMPLE_001" "$VERIFY_OUTPUT"; then
    pass "Verification output contains test case ID"
else
    fail "Verification output missing test case ID"
fi

# Check for verification status indicators (PASS or success indicators)
if grep -q "✓ PASS" "$VERIFY_OUTPUT" || grep -q "PASS" "$VERIFY_OUTPUT"; then
    pass "Verification output shows PASS status"
else
    fail "Verification output missing PASS status indicator"
fi

# Check for step information
if grep -q "step" "$VERIFY_OUTPUT" || grep -q "Step" "$VERIFY_OUTPUT"; then
    pass "Verification output contains step information"
else
    fail "Verification output missing step information"
fi

# Check for sequence information
if grep -q "Sequence" "$VERIFY_OUTPUT" || grep -q "sequence" "$VERIFY_OUTPUT"; then
    pass "Verification output contains sequence information"
else
    info "Verification output may not explicitly show sequence information"
fi

# ============================================================================
# Test 5: Validate verification statistics
# ============================================================================
section "Test 5: Validating Verification Statistics"

# Check for statistics (steps passed/total)
if grep -q "[0-9]*/[0-9]*.*passed" "$VERIFY_OUTPUT" || grep -q "([0-9]*/[0-9]* steps passed)" "$VERIFY_OUTPUT"; then
    pass "Verification output contains step statistics"
else
    fail "Verification output missing step statistics"
fi

# Check for total steps count
if grep -q "total" "$VERIFY_OUTPUT" || grep -q "[0-9]* steps" "$VERIFY_OUTPUT"; then
    pass "Verification output shows total steps count"
else
    info "Verification output may show statistics in different format"
fi

# Check for passed count
if grep -q "passed" "$VERIFY_OUTPUT"; then
    pass "Verification output shows passed count"
else
    fail "Verification output missing passed count"
fi

# ============================================================================
# Test 6: Validate color-coded indicators
# ============================================================================
section "Test 6: Validating Color-Coded Indicators"

# Check for pass indicator (✓)
if grep -q "✓" "$VERIFY_OUTPUT"; then
    pass "Verification output contains ✓ pass indicator"
else
    fail "Verification output missing ✓ pass indicator"
fi

# Verify the output shows success for the self-validated test case
# The self_validated_example should pass all verifications
if grep -q "✓ PASS" "$VERIFY_OUTPUT"; then
    pass "Verification output shows ✓ PASS indicator"
else
    info "Checking for alternative success indicators"
    if grep -q "PASS" "$VERIFY_OUTPUT"; then
        pass "Verification output shows PASS status"
    else
        fail "Verification output missing PASS indicator"
    fi
fi

# ============================================================================
# Test 7: Validate specific verification messages
# ============================================================================
section "Test 7: Validating Verification Messages"

# Check for description or test details
if grep -q "description" "$VERIFY_OUTPUT" || grep -q "demonstrating" "$VERIFY_OUTPUT" || grep -q "Simple self-validated" "$VERIFY_OUTPUT"; then
    pass "Verification output contains test description"
else
    info "Verification output may not include detailed description"
fi

# Verify output structure
if grep -q "===" "$VERIFY_OUTPUT" || grep -q "---" "$VERIFY_OUTPUT" || grep -q "Verifying" "$VERIFY_OUTPUT"; then
    pass "Verification output has structured format"
else
    info "Verification output may use alternative formatting"
fi

# ============================================================================
# Test 8: Test with verbose flag
# ============================================================================
section "Test 8: Testing Verbose Verification Output"

VERIFY_VERBOSE_OUTPUT="$TEMP_DIR/verify_verbose_output.txt"
VERIFY_VERBOSE_EXIT_CODE=0

# Run verification with verbose flag
if "$TEST_ORCHESTRATOR_BINARY" --path "$PROJECT_ROOT/testcases" verify \
    --test-case "$TEST_CASE_FILE" \
    --execution-log "$EXECUTION_LOG_FILE" \
    --verbose > "$VERIFY_VERBOSE_OUTPUT" 2>&1; then
    VERIFY_VERBOSE_EXIT_CODE=$?
    pass "Verbose verify command executed successfully"
else
    VERIFY_VERBOSE_EXIT_CODE=$?
    info "Verbose verify command completed with exit code: $VERIFY_VERBOSE_EXIT_CODE"
fi

# Verify verbose output has more detail
VERBOSE_LINE_COUNT=$(wc -l < "$VERIFY_VERBOSE_OUTPUT")
NORMAL_LINE_COUNT=$(wc -l < "$VERIFY_OUTPUT")

if [[ $VERBOSE_LINE_COUNT -gt $NORMAL_LINE_COUNT ]]; then
    pass "Verbose output contains more detail than normal output"
else
    info "Verbose output line count: $VERBOSE_LINE_COUNT, normal: $NORMAL_LINE_COUNT"
fi

# Check verbose output contains step details
if grep -q "Step [0-9]" "$VERIFY_VERBOSE_OUTPUT"; then
    pass "Verbose output contains detailed step information"
else
    fail "Verbose output missing detailed step information"
fi

# ============================================================================
# Test 9: Display sample output for verification
# ============================================================================
section "Test 9: Sample Verification Output"

info "First 20 lines of verification output:"
echo "----------------------------------------"
head -20 "$VERIFY_OUTPUT"
echo "----------------------------------------"

# ============================================================================
# Summary
# ============================================================================
section "Test Summary"

echo ""
echo "Tests Passed: $TESTS_PASSED"
echo "Tests Failed: $TESTS_FAILED"
echo ""

if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
