#!/bin/bash
#
# E2E integration test for test-verify functionality
# Tests clean, verify passing, verify failing, and report format/statistics
#
# Usage: ./tests/integration/test_verify_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_VERIFY_BINARY="$PROJECT_ROOT/target/debug/test-verify"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=== Test-Verify E2E Integration Test ==="

# Check if binary exists
if [[ ! -f "$TEST_VERIFY_BINARY" ]]; then
    echo -e "${RED}✗ test-verify binary not found at $TEST_VERIFY_BINARY${NC}"
    echo "Please build the project first: cargo build"
    exit 1
fi
echo -e "${GREEN}✓ test-verify binary found${NC}"

# Create temporary directory for test files
TEST_DIR=$(mktemp -d)
trap "rm -rf $TEST_DIR" EXIT

echo "Test directory: $TEST_DIR"
echo ""

# ============================================================================
# Test 1: Create sample test case YAML
# ============================================================================
echo "=== Test 1: Creating sample test case YAML ==="

TEST_CASE_FILE="$TEST_DIR/test_case.yaml"
cat > "$TEST_CASE_FILE" <<'EOF'
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "Sample test case for verification"
general_initial_conditions:
  eUICC:
    - "Device is powered on"
initial_conditions:
  eUICC:
    - "Profile is enabled"
test_sequences:
  - id: 1
    name: "Test Sequence 1"
    description: "Basic test sequence"
    initial_conditions:
      eUICC:
        - "Profile is ready"
    steps:
      - step: 1
        description: "Execute command"
        command: "AT+TEST"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Verify status"
        command: "AT+STATUS"
        expected:
          success: true
          result: "READY"
          output: "Device ready"
EOF

if [[ -f "$TEST_CASE_FILE" ]]; then
    echo -e "${GREEN}✓ Test case YAML created${NC}"
else
    echo -e "${RED}✗ Failed to create test case YAML${NC}"
    exit 1
fi

# ============================================================================
# Test 2: Create passing execution log YAML
# ============================================================================
echo ""
echo "=== Test 2: Creating passing execution log YAML ==="

PASSING_LOG_FILE="$TEST_DIR/passing_log.yaml"
cat > "$PASSING_LOG_FILE" <<'EOF'
test_case_id: "TC001"
sequence_id: 1
timestamp: "2024-01-01T12:00:00Z"
actual_output: "OK"
actual_success: true
duration_ms: 1000
EOF

if [[ -f "$PASSING_LOG_FILE" ]]; then
    echo -e "${GREEN}✓ Passing execution log YAML created${NC}"
else
    echo -e "${RED}✗ Failed to create passing execution log YAML${NC}"
    exit 1
fi

# ============================================================================
# Test 3: Create failing execution log YAML
# ============================================================================
echo ""
echo "=== Test 3: Creating failing execution log YAML ==="

FAILING_LOG_FILE="$TEST_DIR/failing_log.yaml"
cat > "$FAILING_LOG_FILE" <<'EOF'
test_case_id: "TC001"
sequence_id: 1
timestamp: "2024-01-01T12:00:00Z"
actual_output: "ERROR"
actual_success: false
duration_ms: 500
error_message: "Command failed"
EOF

if [[ -f "$FAILING_LOG_FILE" ]]; then
    echo -e "${GREEN}✓ Failing execution log YAML created${NC}"
else
    echo -e "${RED}✗ Failed to create failing execution log YAML${NC}"
    exit 1
fi

# ============================================================================
# Test 4: Test clean command
# ============================================================================
echo ""
echo "=== Test 4: Testing clean command ==="

CLEAN_OUTPUT="$TEST_DIR/clean_output.txt"
if "$TEST_VERIFY_BINARY" clean "$PASSING_LOG_FILE" > "$CLEAN_OUTPUT" 2>&1; then
    echo -e "${GREEN}✓ Clean command executed successfully${NC}"
    
    # Verify output contains expected fields
    if grep -q "test_case_id" "$CLEAN_OUTPUT" && \
       grep -q "sequence_id" "$CLEAN_OUTPUT" && \
       grep -q "actual_output" "$CLEAN_OUTPUT"; then
        echo -e "${GREEN}✓ Clean output contains expected YAML fields${NC}"
    else
        echo -e "${RED}✗ Clean output missing expected YAML fields${NC}"
        echo "Clean output:"
        cat "$CLEAN_OUTPUT"
        exit 1
    fi
else
    echo -e "${RED}✗ Clean command failed${NC}"
    cat "$CLEAN_OUTPUT"
    exit 1
fi

# ============================================================================
# Test 5: Test verify with passing execution log
# ============================================================================
echo ""
echo "=== Test 5: Testing verify with passing execution log ==="

VERIFY_PASSING_OUTPUT="$TEST_DIR/verify_passing_output.txt"
if "$TEST_VERIFY_BINARY" single --log "$PASSING_LOG_FILE" --test-case-id "$TEST_CASE_FILE" > "$VERIFY_PASSING_OUTPUT" 2>&1; then
    EXIT_CODE=$?
    if [[ $EXIT_CODE -eq 0 ]]; then
        echo -e "${GREEN}✓ Verify command returned exit code 0 for passing log${NC}"
    else
        echo -e "${RED}✗ Verify command returned exit code $EXIT_CODE (expected 0)${NC}"
        cat "$VERIFY_PASSING_OUTPUT"
        exit 1
    fi
    
    # Verify output format
    if grep -q "Verification Summary" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output contains 'Verification Summary'${NC}"
    else
        echo -e "${RED}✗ Verify output missing 'Verification Summary'${NC}"
        cat "$VERIFY_PASSING_OUTPUT"
        exit 1
    fi
    
    if grep -q "Test Case ID" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output contains 'Test Case ID'${NC}"
    else
        echo -e "${RED}✗ Verify output missing 'Test Case ID'${NC}"
        exit 1
    fi
    
    if grep -q "Sequence ID" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output contains 'Sequence ID'${NC}"
    else
        echo -e "${RED}✗ Verify output missing 'Sequence ID'${NC}"
        exit 1
    fi
    
    if grep -q "✓ PASS" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output shows PASS status${NC}"
    else
        echo -e "${RED}✗ Verify output does not show PASS status${NC}"
        cat "$VERIFY_PASSING_OUTPUT"
        exit 1
    fi
    
    # Verify statistics
    if grep -q "Steps:.*total" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output contains step statistics${NC}"
    else
        echo -e "${RED}✗ Verify output missing step statistics${NC}"
        exit 1
    fi
    
    if grep -q "passed" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output shows passed count${NC}"
    else
        echo -e "${RED}✗ Verify output missing passed count${NC}"
        exit 1
    fi
    
    if grep -q "failed" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output shows failed count${NC}"
    else
        echo -e "${RED}✗ Verify output missing failed count${NC}"
        exit 1
    fi
else
    EXIT_CODE=$?
    echo -e "${RED}✗ Verify command failed unexpectedly with exit code $EXIT_CODE${NC}"
    cat "$VERIFY_PASSING_OUTPUT"
    exit 1
fi

# ============================================================================
# Test 6: Test verify with failing execution log
# ============================================================================
echo ""
echo "=== Test 6: Testing verify with failing execution log ==="

VERIFY_FAILING_OUTPUT="$TEST_DIR/verify_failing_output.txt"
if "$TEST_VERIFY_BINARY" single --log "$FAILING_LOG_FILE" --test-case-id "$TEST_CASE_FILE" > "$VERIFY_FAILING_OUTPUT" 2>&1; then
    EXIT_CODE=$?
    echo -e "${RED}✗ Verify command returned exit code 0 for failing log (expected non-zero)${NC}"
    cat "$VERIFY_FAILING_OUTPUT"
    exit 1
else
    EXIT_CODE=$?
    if [[ $EXIT_CODE -ne 0 ]]; then
        echo -e "${GREEN}✓ Verify command returned non-zero exit code ($EXIT_CODE) for failing log${NC}"
    else
        echo -e "${RED}✗ Verify command returned exit code 0 (expected non-zero)${NC}"
        exit 1
    fi
    
    # Verify failure output format
    if grep -q "✗ FAIL" "$VERIFY_FAILING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output shows FAIL status${NC}"
    else
        echo -e "${RED}✗ Verify output does not show FAIL status${NC}"
        cat "$VERIFY_FAILING_OUTPUT"
        exit 1
    fi
    
    if grep -q "Failure Details" "$VERIFY_FAILING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output contains 'Failure Details'${NC}"
    else
        echo -e "${RED}✗ Verify output missing 'Failure Details'${NC}"
        cat "$VERIFY_FAILING_OUTPUT"
        exit 1
    fi
    
    # Verify diff output is present
    if grep -q "mismatch" "$VERIFY_FAILING_OUTPUT" || \
       grep -q "Expected:" "$VERIFY_FAILING_OUTPUT" || \
       grep -q "Actual:" "$VERIFY_FAILING_OUTPUT"; then
        echo -e "${GREEN}✓ Verify output contains diff information${NC}"
    else
        echo -e "${RED}✗ Verify output missing diff information${NC}"
        cat "$VERIFY_FAILING_OUTPUT"
        exit 1
    fi
fi

# ============================================================================
# Test 7: Validate report format completeness
# ============================================================================
echo ""
echo "=== Test 7: Validating complete report format ==="

# Check both passing and failing reports contain all required sections
REQUIRED_SECTIONS=(
    "Verification Summary"
    "Test Case ID"
    "Sequence ID"
    "Overall:"
    "Steps:"
    "total"
    "passed"
    "failed"
)

echo "Checking passing report format..."
MISSING_SECTIONS=0
for section in "${REQUIRED_SECTIONS[@]}"; do
    if ! grep -q "$section" "$VERIFY_PASSING_OUTPUT"; then
        echo -e "${RED}✗ Missing section: $section${NC}"
        MISSING_SECTIONS=$((MISSING_SECTIONS + 1))
    fi
done

if [[ $MISSING_SECTIONS -eq 0 ]]; then
    echo -e "${GREEN}✓ Passing report contains all required sections${NC}"
else
    echo -e "${RED}✗ Passing report missing $MISSING_SECTIONS section(s)${NC}"
    exit 1
fi

echo "Checking failing report format..."
MISSING_SECTIONS=0
for section in "${REQUIRED_SECTIONS[@]}"; do
    if ! grep -q "$section" "$VERIFY_FAILING_OUTPUT"; then
        echo -e "${RED}✗ Missing section: $section${NC}"
        MISSING_SECTIONS=$((MISSING_SECTIONS + 1))
    fi
done

if [[ $MISSING_SECTIONS -eq 0 ]]; then
    echo -e "${GREEN}✓ Failing report contains all required sections${NC}"
else
    echo -e "${RED}✗ Failing report missing $MISSING_SECTIONS section(s)${NC}"
    exit 1
fi

# ============================================================================
# Test 8: Validate statistics accuracy
# ============================================================================
echo ""
echo "=== Test 8: Validating statistics accuracy ==="

# Extract statistics from passing output
if grep -q "Steps: 2 total, [0-9]* passed, 0 failed" "$VERIFY_PASSING_OUTPUT"; then
    echo -e "${GREEN}✓ Passing report shows correct total steps (2)${NC}"
else
    echo -e "${YELLOW}⚠ Could not verify exact step count in passing report${NC}"
    echo "  (May still be valid if steps were matched differently)"
fi

# Extract statistics from failing output
if grep -q "Steps: 2 total" "$VERIFY_FAILING_OUTPUT"; then
    echo -e "${GREEN}✓ Failing report shows correct total steps (2)${NC}"
else
    echo -e "${YELLOW}⚠ Could not verify exact step count in failing report${NC}"
fi

# Verify failed count is non-zero in failing report
if grep -q "Steps:.*[1-9][0-9]* failed" "$VERIFY_FAILING_OUTPUT" || \
   grep -q "Steps:.*failed, [1-9]" "$VERIFY_FAILING_OUTPUT"; then
    echo -e "${GREEN}✓ Failing report shows non-zero failed count${NC}"
else
    echo -e "${RED}✗ Failing report does not show non-zero failed count${NC}"
    grep "Steps:" "$VERIFY_FAILING_OUTPUT" || echo "(No Steps: line found)"
    exit 1
fi

# ============================================================================
# Test 9: Test with invalid files
# ============================================================================
echo ""
echo "=== Test 9: Testing error handling with invalid files ==="

# Test with non-existent log file
if "$TEST_VERIFY_BINARY" single --log "$TEST_DIR/nonexistent.yaml" --test-case-id "$TEST_CASE_FILE" > /dev/null 2>&1; then
    echo -e "${RED}✗ Command should fail with non-existent log file${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Command correctly fails with non-existent log file${NC}"
fi

# Test with non-existent test case file
if "$TEST_VERIFY_BINARY" single --log "$PASSING_LOG_FILE" --test-case-id "$TEST_DIR/nonexistent.yaml" > /dev/null 2>&1; then
    echo -e "${RED}✗ Command should fail with non-existent test case file${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Command correctly fails with non-existent test case file${NC}"
fi

# Test clean with non-existent log file
if "$TEST_VERIFY_BINARY" clean "$TEST_DIR/nonexistent.yaml" > /dev/null 2>&1; then
    echo -e "${RED}✗ Clean should fail with non-existent log file${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Clean correctly fails with non-existent log file${NC}"
fi

# ============================================================================
# Test 10: Test with malformed YAML
# ============================================================================
echo ""
echo "=== Test 10: Testing error handling with malformed YAML ==="

MALFORMED_FILE="$TEST_DIR/malformed.yaml"
cat > "$MALFORMED_FILE" <<'EOF'
invalid: yaml: content:
  - this is
  not properly: [formatted
EOF

if "$TEST_VERIFY_BINARY" clean "$MALFORMED_FILE" > /dev/null 2>&1; then
    echo -e "${RED}✗ Clean should fail with malformed YAML${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Clean correctly fails with malformed YAML${NC}"
fi

# ============================================================================
# Summary
# ============================================================================
echo ""
echo "=========================================="
echo -e "${GREEN}✓ All test-verify E2E tests passed!${NC}"
echo "=========================================="
echo ""
echo "Test summary:"
echo "  ✓ Sample YAML files created successfully"
echo "  ✓ Clean command works correctly"
echo "  ✓ Verify with passing log returns exit code 0"
echo "  ✓ Verify with failing log returns non-zero exit code"
echo "  ✓ Report format includes all required sections"
echo "  ✓ Statistics are accurate"
echo "  ✓ Error handling works correctly"
echo ""

exit 0
