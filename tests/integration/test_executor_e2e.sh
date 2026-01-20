#!/bin/bash
#
# End-to-end integration test for test-executor
#
# This test validates:
# 1. Test case YAML validation against schema
# 2. Shell script generation with syntax validation
# 3. Test execution with passing verification (exit code 0)
# 4. Test execution with failing verification (non-zero exit code + error output)
#
# Usage: ./tests/integration/test_executor_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_EXECUTOR_BIN="$PROJECT_ROOT/target/debug/test-executor"
VALIDATE_YAML_BIN="$PROJECT_ROOT/target/debug/validate-yaml"
SCHEMA_FILE="$PROJECT_ROOT/data/schema.json"

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

echo "======================================"
echo "test-executor End-to-End Integration Test"
echo "======================================"
echo ""

# Function to print test status
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

# Check prerequisites
section "Checking Prerequisites"

if [[ ! -f "$TEST_EXECUTOR_BIN" ]]; then
    fail "test-executor binary not found at $TEST_EXECUTOR_BIN"
    echo "Run 'cargo build' first"
    exit 1
fi
pass "test-executor binary found"

if [[ ! -f "$VALIDATE_YAML_BIN" ]]; then
    fail "validate-yaml binary not found at $VALIDATE_YAML_BIN"
    echo "Run 'cargo build' first"
    exit 1
fi
pass "validate-yaml binary found"

if [[ ! -f "$SCHEMA_FILE" ]]; then
    fail "Schema file not found at $SCHEMA_FILE"
    exit 1
fi
pass "Schema file found"

if ! command -v bash &> /dev/null; then
    fail "bash not found"
    exit 1
fi
pass "bash available"

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

info "Using temporary directory: $TEMP_DIR"

# Create test YAML file with passing verification
section "Creating Test YAML Files"

PASSING_YAML="$TEMP_DIR/test_passing.yaml"
cat > "$PASSING_YAML" << 'EOF'
requirement: TEST001
item: 1
tc: 1
id: TEST_PASSING
description: Test case with passing verification
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Passing Sequence
    description: This sequence should pass all verifications
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Echo test that should pass
        command: echo 'hello'
        expected:
          success: true
          result: "0"
          output: hello
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]"
      - step: 2
        description: True command that should pass
        command: "true"
        expected:
          success: true
          result: "0"
          output: none
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
EOF

pass "Created passing test YAML"

# Create test YAML file with failing verification
FAILING_YAML="$TEMP_DIR/test_failing.yaml"
cat > "$FAILING_YAML" << 'EOF'
requirement: TEST002
item: 1
tc: 2
id: TEST_FAILING
description: Test case with failing verification
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Failing Sequence
    description: This sequence should fail verification
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Echo test with wrong expected output
        command: echo 'hello'
        expected:
          success: false
          result: "1"
          output: goodbye
        verification:
          result: "[ $EXIT_CODE -eq 99 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"goodbye\" ]"
EOF

pass "Created failing test YAML"

# Test 1: Validate YAML files against schema
section "Test 1: YAML Schema Validation"

if "$VALIDATE_YAML_BIN" "$PASSING_YAML" "$SCHEMA_FILE" > /dev/null 2>&1; then
    pass "Passing YAML validates against schema"
else
    fail "Passing YAML failed schema validation"
fi

if "$VALIDATE_YAML_BIN" "$FAILING_YAML" "$SCHEMA_FILE" > /dev/null 2>&1; then
    pass "Failing YAML validates against schema"
else
    fail "Failing YAML failed schema validation"
fi

# Test 2: Generate shell scripts and validate syntax
section "Test 2: Shell Script Generation and Syntax Validation"

PASSING_SCRIPT="$TEMP_DIR/test_passing.sh"
if "$TEST_EXECUTOR_BIN" generate "$PASSING_YAML" -o "$PASSING_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from passing YAML"
else
    fail "Failed to generate script from passing YAML"
fi

if [[ -f "$PASSING_SCRIPT" ]]; then
    pass "Passing script file created"
else
    fail "Passing script file not found"
fi

# Validate shell script syntax
if bash -n "$PASSING_SCRIPT" 2>/dev/null; then
    pass "Passing script has valid bash syntax"
else
    fail "Passing script has invalid bash syntax"
fi

FAILING_SCRIPT="$TEMP_DIR/test_failing.sh"
if "$TEST_EXECUTOR_BIN" generate "$FAILING_YAML" -o "$FAILING_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from failing YAML"
else
    fail "Failed to generate script from failing YAML"
fi

if [[ -f "$FAILING_SCRIPT" ]]; then
    pass "Failing script file created"
else
    fail "Failing script file not found"
fi

# Validate shell script syntax
if bash -n "$FAILING_SCRIPT" 2>/dev/null; then
    pass "Failing script has valid bash syntax"
else
    fail "Failing script has invalid bash syntax"
fi

# Verify script contains expected elements
if grep -q "#!/bin/bash" "$PASSING_SCRIPT"; then
    pass "Passing script has bash shebang"
else
    fail "Passing script missing bash shebang"
fi

if grep -q "TEST_PASSING" "$PASSING_SCRIPT"; then
    pass "Passing script contains test case ID"
else
    fail "Passing script missing test case ID"
fi

if grep -q "VERIFICATION_RESULT_PASS" "$PASSING_SCRIPT"; then
    pass "Passing script contains verification logic"
else
    fail "Passing script missing verification logic"
fi

# Test 3: Execute test with passing verification
section "Test 3: Execute Test with Passing Verification"

PASSING_OUTPUT="$TEMP_DIR/passing_output.txt"
if "$TEST_EXECUTOR_BIN" execute "$PASSING_YAML" > "$PASSING_OUTPUT" 2>&1; then
    PASSING_EXIT_CODE=0
    pass "Passing test execution returned exit code 0"
else
    PASSING_EXIT_CODE=$?
    fail "Passing test execution returned non-zero exit code: $PASSING_EXIT_CODE"
fi

if [[ $PASSING_EXIT_CODE -eq 0 ]]; then
    pass "Passing test has correct exit code"
else
    fail "Passing test has incorrect exit code: expected 0, got $PASSING_EXIT_CODE"
fi

# Check output contains success indicators
if grep -q "Test execution completed successfully" "$PASSING_OUTPUT" 2>/dev/null || 
   grep -q "All test sequences completed successfully" "$PASSING_OUTPUT" 2>/dev/null; then
    pass "Passing test output contains success message"
else
    fail "Passing test output missing success message"
fi

# Test 4: Execute test with failing verification
section "Test 4: Execute Test with Failing Verification"

FAILING_OUTPUT="$TEMP_DIR/failing_output.txt"
if "$TEST_EXECUTOR_BIN" execute "$FAILING_YAML" > "$FAILING_OUTPUT" 2>&1; then
    FAILING_EXIT_CODE=0
    fail "Failing test execution returned exit code 0 (should be non-zero)"
else
    FAILING_EXIT_CODE=$?
    pass "Failing test execution returned non-zero exit code: $FAILING_EXIT_CODE"
fi

if [[ $FAILING_EXIT_CODE -ne 0 ]]; then
    pass "Failing test has non-zero exit code"
else
    fail "Failing test has exit code 0 (should be non-zero)"
fi

# Check output contains error information
if grep -q "FAIL" "$FAILING_OUTPUT" 2>/dev/null || 
   grep -q "failed" "$FAILING_OUTPUT" 2>/dev/null ||
   grep -q "Test execution failed" "$FAILING_OUTPUT" 2>/dev/null; then
    pass "Failing test output contains error indicators"
else
    fail "Failing test output missing error indicators"
    info "Output: $(cat "$FAILING_OUTPUT")"
fi

# Verify error output includes verification details
if grep -q "verification" "$FAILING_OUTPUT" 2>/dev/null ||
   grep -q "EXIT_CODE" "$FAILING_OUTPUT" 2>/dev/null ||
   grep -q "COMMAND_OUTPUT" "$FAILING_OUTPUT" 2>/dev/null; then
    pass "Failing test output includes verification details"
else
    fail "Failing test output missing verification details"
fi

# Test 5: Verify generated script structure
section "Test 5: Verify Generated Script Structure"

# Check for essential script components
if grep -q "set -e" "$PASSING_SCRIPT"; then
    pass "Script contains 'set -e' for error handling"
else
    fail "Script missing 'set -e'"
fi

if grep -q "COMMAND_OUTPUT=" "$PASSING_SCRIPT"; then
    pass "Script captures command output"
else
    fail "Script doesn't capture command output"
fi

if grep -q "EXIT_CODE=" "$PASSING_SCRIPT"; then
    pass "Script captures exit code"
else
    fail "Script doesn't capture exit code"
fi

if grep -q "echo.*PASS.*Step" "$PASSING_SCRIPT"; then
    pass "Script includes PASS output for steps"
else
    fail "Script missing PASS output"
fi

if grep -q "echo.*FAIL.*Step" "$PASSING_SCRIPT"; then
    pass "Script includes FAIL output for steps"
else
    fail "Script missing FAIL output"
fi

if grep -q "exit 1" "$PASSING_SCRIPT"; then
    pass "Script includes failure exit code"
else
    fail "Script missing failure exit code"
fi

if grep -q "exit 0" "$PASSING_SCRIPT"; then
    pass "Script includes success exit code"
else
    fail "Script missing success exit code"
fi

# Summary
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
