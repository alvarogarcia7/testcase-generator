#!/bin/bash
#
# Integration test for BDD prerequisites/initial conditions
#
# This test validates:
# 1. BDD step definitions are correctly loaded from TOML
# 2. BDD statements are parsed and converted to shell commands
# 3. Generated scripts execute BDD prerequisites successfully
# 4. Multiple BDD steps can be combined in initial conditions
# 5. Parameter substitution works correctly in BDD statements
#
# Usage: ./tests/integration/test_bdd_initial_conditions.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_EXECUTOR_BIN="$PROJECT_ROOT/target/debug/test-executor"
BDD_DEFINITIONS="$PROJECT_ROOT/data/bdd_step_definitions.toml"

# Source logger library
source "$SCRIPT_DIR/../../scripts/lib/logger.sh" || exit 1

# Handle --no-remove flag
REMOVE_TEMP=1
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-remove)
            REMOVE_TEMP=0
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

echo "======================================"
echo "BDD Initial Conditions Integration Test"
echo "======================================"
echo ""

# Check prerequisites
section "Checking Prerequisites"

if [[ ! -f "$TEST_EXECUTOR_BIN" ]]; then
    fail "test-executor binary not found at $TEST_EXECUTOR_BIN"
    echo "Run 'cargo build' first"
    exit 1
fi
pass "test-executor binary found"

if [[ ! -f "$BDD_DEFINITIONS" ]]; then
    fail "BDD definitions file not found at $BDD_DEFINITIONS"
    exit 1
fi
pass "BDD definitions file found"

if ! command -v bash &> /dev/null; then
    fail "bash not found"
    exit 1
fi
pass "bash available"

# Create temporary directory for test files
TEMP_DIR=$(mktemp -d)
setup_cleanup "$TEMP_DIR"
if [[ $REMOVE_TEMP -eq 0 ]]; then
    disable_cleanup
    info "Temporary files will not be removed: $TEMP_DIR"
fi

info "Using temporary directory: $TEMP_DIR"

# Test 1: Generate and execute script with BDD prerequisites
section "Test 1: Create Test YAML with BDD Initial Conditions"

TEST_YAML="$TEMP_DIR/test_bdd_prerequisites.yaml"
cat > "$TEST_YAML" << 'EOF'
requirement: BDD001
item: 1
tc: 1
id: TEST_BDD_PREREQUISITES
description: Test case with BDD-style initial conditions
general_initial_conditions:
  System:
    - "create directory \"/tmp/bdd_test\""
    - "set environment variable \"TEST_MODE\" to \"integration\""
    - "wait for 1 second"
initial_conditions:
  Device:
    - "create file \"/tmp/bdd_test/config.txt\" with content:"
    - "file \"/tmp/bdd_test/config.txt\" should exist"
test_sequences:
  - id: 1
    name: BDD Prerequisites Test
    description: Verify BDD prerequisites are executed correctly
    initial_conditions:
      LPA:
        - "create directory \"/tmp/bdd_test/logs\""
        - "ping device \"127.0.0.1\" with 2 retries"
    steps:
      - step: 1
        description: Verify test directory exists
        command: test -d /tmp/bdd_test
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Verify config file was created
        command: test -f /tmp/bdd_test/config.txt
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Verify logs directory exists
        command: test -d /tmp/bdd_test/logs
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 4
        description: Verify environment variable is set
        command: echo $TEST_MODE
        expected:
          success: true
          result: "0"
          output: integration
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"integration\" ]"
      - step: 5
        description: Clean up test directory
        command: rm -rf /tmp/bdd_test
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
EOF

pass "Created test YAML with BDD prerequisites"

# Test 2: Generate shell script from YAML
section "Test 2: Generate Shell Script from YAML"

GENERATED_SCRIPT="$TEMP_DIR/test_bdd_prerequisites.sh"
if "$TEST_EXECUTOR_BIN" generate "$TEST_YAML" -o "$GENERATED_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from YAML with BDD prerequisites"
else
    fail "Failed to generate script from YAML"
    exit 1
fi

if [[ -f "$GENERATED_SCRIPT" ]]; then
    pass "Generated script file exists"
else
    fail "Generated script file not found"
    exit 1
fi

# Validate shell script syntax
if bash -n "$GENERATED_SCRIPT" 2>/dev/null; then
    pass "Generated script has valid bash syntax"
else
    fail "Generated script has invalid bash syntax"
    exit 1
fi

# Test 3: Verify script contains BDD prerequisite commands
section "Test 3: Verify Script Contains BDD Commands"

if grep -q 'mkdir -p "/tmp/bdd_test"' "$GENERATED_SCRIPT"; then
    pass "Script contains 'create directory' BDD command"
else
    fail "Script missing 'create directory' BDD command"
fi

if grep -q "export TEST_MODE=integration" "$GENERATED_SCRIPT"; then
    pass "Script contains 'set environment variable' BDD command"
else
    fail "Script missing 'set environment variable' BDD command"
fi

if grep -q "sleep 1" "$GENERATED_SCRIPT"; then
    pass "Script contains 'wait for seconds' BDD command"
else
    fail "Script missing 'wait for seconds' BDD command"
fi

if grep -q "echo.*> /tmp/bdd_test/config.txt" "$GENERATED_SCRIPT"; then
    pass "Script contains 'create file' BDD command"
else
    fail "Script missing 'create file' BDD command"
fi

if grep -q 'test -f "/tmp/bdd_test/config.txt"' "$GENERATED_SCRIPT"; then
    pass "Script contains 'file should exist' BDD command"
else
    fail "Script missing 'file should exist' BDD command"
fi

if grep -q 'ping -c 2 "127.0.0.1"' "$GENERATED_SCRIPT"; then
    pass "Script contains 'ping device with retries' BDD command"
else
    fail "Script missing 'ping device with retries' BDD command"
fi

# Test 4: Execute generated script
section "Test 4: Execute Generated Script"

TEST_OUTPUT="$TEMP_DIR/test_output.txt"
if bash "$GENERATED_SCRIPT" > "$TEST_OUTPUT" 2>&1; then
    TEST_EXIT_CODE=0
    pass "Script execution completed with exit code 0"
else
    TEST_EXIT_CODE=$?
    fail "Script execution failed with exit code $TEST_EXIT_CODE"
    info "Script output:"
    cat "$TEST_OUTPUT"
fi

if [[ $TEST_EXIT_CODE -eq 0 ]]; then
    pass "All test steps passed"
else
    fail "Some test steps failed"
fi

# Test 5: Verify script output
section "Test 5: Verify Script Output"

if grep -q "PASS.*Step 1" "$TEST_OUTPUT" 2>/dev/null; then
    pass "Step 1 verification passed (directory exists)"
else
    fail "Step 1 verification failed"
fi

if grep -q "PASS.*Step 2" "$TEST_OUTPUT" 2>/dev/null; then
    pass "Step 2 verification passed (config file created)"
else
    fail "Step 2 verification failed"
fi

if grep -q "PASS.*Step 3" "$TEST_OUTPUT" 2>/dev/null; then
    pass "Step 3 verification passed (logs directory exists)"
else
    fail "Step 3 verification failed"
fi

if grep -q "PASS.*Step 4" "$TEST_OUTPUT" 2>/dev/null; then
    pass "Step 4 verification passed (environment variable set)"
else
    fail "Step 4 verification failed"
fi

# Test 6: Create another test with more complex BDD prerequisites
section "Test 6: Complex BDD Prerequisites Test"

COMPLEX_YAML="$TEMP_DIR/test_complex_bdd.yaml"
cat > "$COMPLEX_YAML" << 'EOF'
requirement: BDD002
item: 1
tc: 2
id: TEST_COMPLEX_BDD
description: Test case with complex BDD prerequisites
general_initial_conditions:
  System:
    - "create directory \"/tmp/bdd_complex\""
    - "create directory \"/tmp/bdd_complex/data\""
    - "create directory \"/tmp/bdd_complex/backup\""
initial_conditions:
  Device:
    - "create file \"/tmp/bdd_complex/data/input.txt\" with content:"
    - "append \"Test data line 1\" to file \"/tmp/bdd_complex/data/input.txt\""
    - "append \"Test data line 2\" to file \"/tmp/bdd_complex/data/input.txt\""
test_sequences:
  - id: 1
    name: Complex Prerequisites Test
    description: Test multiple BDD prerequisites
    initial_conditions:
      LPA:
        - "set environment variable \"DATA_DIR\" to \"/tmp/bdd_complex/data\""
        - "set environment variable \"BACKUP_DIR\" to \"/tmp/bdd_complex/backup\""
    steps:
      - step: 1
        description: List directory contents
        command: ls -la /tmp/bdd_complex
        expected:
          success: true
          result: "0"
          output: partial
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 2
        description: Verify input file has content
        command: wc -l /tmp/bdd_complex/data/input.txt
        expected:
          success: true
          result: "0"
          output: partial
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
      - step: 3
        description: Clean up
        command: rm -rf /tmp/bdd_complex
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
EOF

pass "Created complex BDD test YAML"

COMPLEX_SCRIPT="$TEMP_DIR/test_complex_bdd.sh"
if "$TEST_EXECUTOR_BIN" generate "$COMPLEX_YAML" -o "$COMPLEX_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from complex BDD YAML"
else
    fail "Failed to generate script from complex BDD YAML"
fi

if bash -n "$COMPLEX_SCRIPT" 2>/dev/null; then
    pass "Complex BDD script has valid syntax"
else
    fail "Complex BDD script has invalid syntax"
fi

# Test 7: Execute complex BDD script
section "Test 7: Execute Complex BDD Script"

COMPLEX_OUTPUT="$TEMP_DIR/complex_output.txt"
if bash "$COMPLEX_SCRIPT" > "$COMPLEX_OUTPUT" 2>&1; then
    COMPLEX_EXIT_CODE=0
    pass "Complex BDD script executed successfully"
else
    COMPLEX_EXIT_CODE=$?
    fail "Complex BDD script execution failed with exit code $COMPLEX_EXIT_CODE"
    info "Script output:"
    cat "$COMPLEX_OUTPUT"
fi

if [[ $COMPLEX_EXIT_CODE -eq 0 ]]; then
    pass "All complex BDD test steps passed"
else
    fail "Some complex BDD test steps failed"
fi

# Test 8: Verify BDD step definitions are used correctly
section "Test 8: Verify BDD Step Definitions Usage"

if grep -q 'mkdir -p "/tmp/bdd_complex' "$COMPLEX_SCRIPT"; then
    pass "Multiple directory creation commands present"
else
    fail "Missing directory creation commands"
fi

if grep -q "echo.*>> /tmp/bdd_complex/data/input.txt" "$COMPLEX_SCRIPT"; then
    pass "Append to file commands present"
else
    fail "Missing append to file commands"
fi

if grep -q "export DATA_DIR=/tmp/bdd_complex/data" "$COMPLEX_SCRIPT"; then
    pass "Environment variable export commands present"
else
    fail "Missing environment variable commands"
fi

# Test 9: Test parameter extraction accuracy
section "Test 9: Verify Parameter Extraction"

TEST_PARAMS_YAML="$TEMP_DIR/test_params.yaml"
cat > "$TEST_PARAMS_YAML" << 'EOF'
requirement: BDD003
item: 1
tc: 3
id: TEST_PARAM_EXTRACTION
description: Test parameter extraction from BDD statements
general_initial_conditions:
  System:
    - "ping device \"8.8.8.8\" with 5 retries"
    - "create directory \"/tmp/param_test\""
    - "wait for 2 seconds"
initial_conditions:
  Device:
    - "set environment variable \"HOST\" to \"localhost\""
    - "set environment variable \"PORT\" to \"8080\""
test_sequences:
  - id: 1
    name: Parameter Test
    description: Verify parameter extraction
    steps:
      - step: 1
        description: Echo test
        command: echo "test"
        expected:
          success: true
          result: "0"
          output: test
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]"
      - step: 2
        description: Cleanup
        command: rm -rf /tmp/param_test
        expected:
          success: true
          result: "0"
          output: ""
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
EOF

PARAMS_SCRIPT="$TEMP_DIR/test_params.sh"
if "$TEST_EXECUTOR_BIN" generate "$TEST_PARAMS_YAML" -o "$PARAMS_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script with various parameter types"
else
    fail "Failed to generate script with parameters"
fi

if grep -q 'ping -c 5 "8.8.8.8"' "$PARAMS_SCRIPT"; then
    pass "Numeric and IP parameters extracted correctly"
else
    fail "Incorrect parameter extraction for ping command"
fi

if grep -q 'mkdir -p "/tmp/param_test"' "$PARAMS_SCRIPT"; then
    pass "Path parameter extracted correctly"
else
    fail "Incorrect parameter extraction for directory command"
fi

if grep -q "sleep 2" "$PARAMS_SCRIPT"; then
    pass "Wait duration parameter extracted correctly"
else
    fail "Incorrect parameter extraction for wait command"
fi

if grep -q "export HOST=localhost" "$PARAMS_SCRIPT"; then
    pass "String parameter extracted correctly"
else
    fail "Incorrect parameter extraction for environment variable"
fi

if grep -q "export PORT=8080" "$PARAMS_SCRIPT"; then
    pass "Numeric environment value extracted correctly"
else
    fail "Incorrect parameter extraction for numeric environment variable"
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
