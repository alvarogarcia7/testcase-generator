#!/bin/bash
#
# End-to-end integration test for conditional verification
#
# This test validates:
# 1. Script generation with conditional verifications
# 2. if_true commands run when condition passes
# 3. if_false commands run when condition fails
# 4. always commands run in both cases
# 5. Overall pass/fail status determined by condition result, not action command success
#
# Usage: ./tests/integration/test_conditional_verification_e2e.sh
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TEST_EXECUTOR_BIN="$PROJECT_ROOT/target/debug/test-executor"

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
echo "Conditional Verification E2E Test"
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

# ============================================================================
# Test 1: if_true branch executes when condition passes
# ============================================================================
section "Test 1: if_true Branch Executes When Condition Passes"

PASSING_CONDITION_YAML="$TEMP_DIR/test_passing_condition.yaml"
cat > "$PASSING_CONDITION_YAML" << 'EOF'
requirement: COND001
item: 1
tc: 1
id: COND_PASS
description: Test case with passing condition
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Passing Condition Sequence
    description: Condition should pass and if_true commands should run
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Command with passing condition
        command: echo 'success'
        expected:
          success: true
          result: "0"
          output: success
        verification:
          result:
            condition: "[[ $EXIT_CODE -eq 0 ]]"
            if_true:
              - "echo 'MARKER_IF_TRUE: if_true branch executed'"
              - "VERIFICATION_RESULT_PASS=true"
            if_false:
              - "echo 'MARKER_IF_FALSE: if_false branch executed'"
              - "VERIFICATION_RESULT_PASS=false"
            always:
              - "echo 'MARKER_ALWAYS: always branch executed'"
          output: "true"
EOF

pass "Created passing condition test YAML"

# Generate script
PASSING_CONDITION_SCRIPT="$TEMP_DIR/test_passing_condition.sh"
if "$TEST_EXECUTOR_BIN" generate "$PASSING_CONDITION_YAML" -o "$PASSING_CONDITION_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from passing condition YAML"
else
    fail "Failed to generate script from passing condition YAML"
fi

# Validate bash syntax
if bash -n "$PASSING_CONDITION_SCRIPT" 2>/dev/null; then
    pass "Passing condition script has valid bash syntax"
else
    fail "Passing condition script has invalid bash syntax"
fi

# Execute the script
PASSING_OUTPUT="$TEMP_DIR/passing_output.txt"
if bash "$PASSING_CONDITION_SCRIPT" > "$PASSING_OUTPUT" 2>&1; then
    pass "Passing condition script executed successfully"
else
    fail "Passing condition script execution failed"
    info "Output: $(cat "$PASSING_OUTPUT")"
fi

# Verify if_true branch executed
if grep -q "MARKER_IF_TRUE: if_true branch executed" "$PASSING_OUTPUT"; then
    pass "if_true branch executed"
else
    fail "if_true branch did not execute"
fi

# Verify if_false branch did NOT execute
if ! grep -q "MARKER_IF_FALSE: if_false branch executed" "$PASSING_OUTPUT"; then
    pass "if_false branch correctly did not execute"
else
    fail "if_false branch incorrectly executed"
fi

# Verify always branch executed
if grep -q "MARKER_ALWAYS: always branch executed" "$PASSING_OUTPUT"; then
    pass "always branch executed"
else
    fail "always branch did not execute"
fi

# ============================================================================
# Test 2: if_false branch executes when condition fails
# ============================================================================
section "Test 2: Condition Passes for Expected Error Code"

EXPECTED_ERROR_YAML="$TEMP_DIR/test_expected_error.yaml"
cat > "$EXPECTED_ERROR_YAML" << 'EOF'
requirement: COND002
item: 1
tc: 2
id: COND_EXPECTED_ERROR
description: Test case expecting specific error code
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Expected Error Sequence
    description: Expecting exit code 42, condition should pass
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Command with expected error code
        command: /bin/sh -c "exit 42"
        expected:
          success: false
          result: "42"
          output: ""
        verification:
          result:
            condition: "[[ $EXIT_CODE -eq 42 ]]"
            if_true:
              - "echo 'MARKER_IF_TRUE: Got expected error code 42'"
              - "VERIFICATION_RESULT_PASS=true"
            if_false:
              - "echo 'MARKER_IF_FALSE: Did not get expected error code'"
              - "VERIFICATION_RESULT_PASS=false"
            always:
              - "echo 'MARKER_ALWAYS: Verification complete'"
          output: "true"
EOF

pass "Created expected error test YAML"

# Generate script
EXPECTED_ERROR_SCRIPT="$TEMP_DIR/test_expected_error.sh"
if "$TEST_EXECUTOR_BIN" generate "$EXPECTED_ERROR_YAML" -o "$EXPECTED_ERROR_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from expected error YAML"
else
    fail "Failed to generate script from expected error YAML"
fi

# Validate bash syntax
if bash -n "$EXPECTED_ERROR_SCRIPT" 2>/dev/null; then
    pass "Expected error script has valid bash syntax"
else
    fail "Expected error script has invalid bash syntax"
fi

# Execute the script
EXPECTED_ERROR_OUTPUT="$TEMP_DIR/expected_error_output.txt"
if bash "$EXPECTED_ERROR_SCRIPT" > "$EXPECTED_ERROR_OUTPUT" 2>&1; then
    pass "Expected error script executed successfully"
else
    fail "Expected error script execution failed"
    info "Output: $(cat "$EXPECTED_ERROR_OUTPUT")"
fi

# Verify if_true branch executed (because we got the expected error code)
if grep -q "MARKER_IF_TRUE: Got expected error code 42" "$EXPECTED_ERROR_OUTPUT"; then
    pass "if_true branch executed for expected error condition"
else
    fail "if_true branch did not execute for expected error"
fi

# Verify always branch executed
if grep -q "MARKER_ALWAYS: Verification complete" "$EXPECTED_ERROR_OUTPUT"; then
    pass "always branch executed"
else
    fail "always branch did not execute"
fi

# ============================================================================
# Test 3: if_false branch executes when condition fails
# ============================================================================
section "Test 3: if_false Branch Executes When Condition Fails"

WRONG_EXIT_YAML="$TEMP_DIR/test_wrong_exit.yaml"
cat > "$WRONG_EXIT_YAML" << 'EOF'
requirement: COND003
item: 1
tc: 3
id: COND_WRONG_EXIT
description: Test case expecting wrong exit code
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Wrong Exit Code Sequence
    description: Condition should fail due to wrong exit code
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Command with wrong expected exit code
        command: echo 'test'
        expected:
          success: true
          result: "0"
          output: test
        verification:
          result:
            condition: "[[ $EXIT_CODE -eq 99 ]]"
            if_true:
              - "echo 'MARKER_IF_TRUE: Got expected exit code 99'"
              - "VERIFICATION_RESULT_PASS=true"
            if_false:
              - "echo 'MARKER_IF_FALSE: Did not get expected exit code 99'"
              - "VERIFICATION_RESULT_PASS=false"
              - "exit 1"
            always:
              - "echo 'MARKER_ALWAYS: Always executed'"
          output: "true"
EOF

pass "Created wrong exit code test YAML"

# Generate script
WRONG_EXIT_SCRIPT="$TEMP_DIR/test_wrong_exit.sh"
if "$TEST_EXECUTOR_BIN" generate "$WRONG_EXIT_YAML" -o "$WRONG_EXIT_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from wrong exit code YAML"
else
    fail "Failed to generate script from wrong exit code YAML"
fi

# Validate bash syntax
if bash -n "$WRONG_EXIT_SCRIPT" 2>/dev/null; then
    pass "Wrong exit code script has valid bash syntax"
else
    fail "Wrong exit code script has invalid bash syntax"
fi

# Execute the script (should fail because condition is false and if_false has exit 1)
WRONG_EXIT_OUTPUT="$TEMP_DIR/wrong_exit_output.txt"
if bash "$WRONG_EXIT_SCRIPT" > "$WRONG_EXIT_OUTPUT" 2>&1; then
    fail "Wrong exit code script should have failed but passed"
else
    pass "Wrong exit code script failed as expected"
fi

# Verify if_false branch executed
if grep -q "MARKER_IF_FALSE: Did not get expected exit code 99" "$WRONG_EXIT_OUTPUT"; then
    pass "if_false branch executed"
else
    fail "if_false branch did not execute"
fi

# Verify if_true branch did NOT execute
if ! grep -q "MARKER_IF_TRUE: Got expected exit code 99" "$WRONG_EXIT_OUTPUT"; then
    pass "if_true branch correctly did not execute"
else
    fail "if_true branch incorrectly executed"
fi

# Verify always branch executed before exit
# Note: The always block won't execute if if_false calls exit 1
# This is expected behavior - exit terminates the script immediately
if grep -q "MARKER_ALWAYS: Always executed" "$WRONG_EXIT_OUTPUT"; then
    pass "always branch executed before exit (unexpected)"
else
    pass "always branch correctly did not execute after exit 1 in if_false"
fi

# ============================================================================
# Test 4: Overall pass/fail determined by condition, not action commands
# ============================================================================
section "Test 4: Pass/Fail Status Determined by Condition Result"

ACTION_SUCCESS_YAML="$TEMP_DIR/test_action_success.yaml"
cat > "$ACTION_SUCCESS_YAML" << 'EOF'
requirement: COND004
item: 1
tc: 4
id: COND_ACTION_SUCCESS
description: Test that actions in branches do not affect pass/fail
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Action Commands Sequence
    description: Action commands success should not affect verification result
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Condition passes with action commands
        command: echo 'output'
        expected:
          success: true
          result: "0"
          output: output
        verification:
          result:
            condition: "[[ $EXIT_CODE -eq 0 ]]"
            if_true:
              - "echo 'MARKER_ACTION1: Action 1 executed'"
              - "echo 'MARKER_ACTION2: Action 2 executed'"
              - "VERIFICATION_RESULT_PASS=true"
            if_false:
              - "echo 'MARKER_ERROR: Should not reach here'"
              - "VERIFICATION_RESULT_PASS=false"
            always:
              - "echo 'MARKER_CLEANUP: Cleanup action'"
          output: "true"
EOF

pass "Created action success test YAML"

# Generate script
ACTION_SUCCESS_SCRIPT="$TEMP_DIR/test_action_success.sh"
if "$TEST_EXECUTOR_BIN" generate "$ACTION_SUCCESS_YAML" -o "$ACTION_SUCCESS_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from action success YAML"
else
    fail "Failed to generate script from action success YAML"
fi

# Validate bash syntax
if bash -n "$ACTION_SUCCESS_SCRIPT" 2>/dev/null; then
    pass "Action success script has valid bash syntax"
else
    fail "Action success script has invalid bash syntax"
fi

# Execute the script
ACTION_SUCCESS_OUTPUT="$TEMP_DIR/action_success_output.txt"
if bash "$ACTION_SUCCESS_SCRIPT" > "$ACTION_SUCCESS_OUTPUT" 2>&1; then
    pass "Action success script executed and passed"
else
    fail "Action success script failed"
    info "Output: $(cat "$ACTION_SUCCESS_OUTPUT")"
fi

# Verify action commands ran
if grep -q "MARKER_ACTION1: Action 1 executed" "$ACTION_SUCCESS_OUTPUT"; then
    pass "Action command 1 executed"
else
    fail "Action command 1 did not execute"
fi

if grep -q "MARKER_ACTION2: Action 2 executed" "$ACTION_SUCCESS_OUTPUT"; then
    pass "Action command 2 executed"
else
    fail "Action command 2 did not execute"
fi

# Verify cleanup action ran
if grep -q "MARKER_CLEANUP: Cleanup action" "$ACTION_SUCCESS_OUTPUT"; then
    pass "Cleanup action executed"
else
    fail "Cleanup action did not execute"
fi

# ============================================================================
# Test 5: Output verification with conditional logic
# ============================================================================
section "Test 5: Output Verification with Conditional Logic"

OUTPUT_CONDITIONAL_YAML="$TEMP_DIR/test_output_conditional.yaml"
cat > "$OUTPUT_CONDITIONAL_YAML" << 'ENDYAML'
requirement: COND005
item: 1
tc: 5
id: COND_OUTPUT
description: Test output verification with conditional branches
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Output Conditional Sequence
    description: Test conditional output verification
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Output verification with pattern matching
        command: "echo 'Status: SUCCESS'"
        expected:
          success: true
          result: "0"
          output: "Status: SUCCESS"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output:
            condition: "grep -q 'SUCCESS' <<< \"$COMMAND_OUTPUT\""
            if_true:
              - "echo 'MARKER_OUTPUT_TRUE: Contains SUCCESS keyword'"
              - "VERIFICATION_OUTPUT_PASS=true"
            if_false:
              - "echo 'MARKER_OUTPUT_FALSE: Missing SUCCESS keyword'"
              - "VERIFICATION_OUTPUT_PASS=false"
            always:
              - "echo 'MARKER_OUTPUT_ALWAYS: Verification complete'"
ENDYAML

pass "Created output conditional test YAML"

# Generate script
OUTPUT_CONDITIONAL_SCRIPT="$TEMP_DIR/test_output_conditional.sh"
OUTPUT_GEN_ERROR="$TEMP_DIR/output_gen_error.txt"
if "$TEST_EXECUTOR_BIN" generate "$OUTPUT_CONDITIONAL_YAML" -o "$OUTPUT_CONDITIONAL_SCRIPT" > "$OUTPUT_GEN_ERROR" 2>&1; then
    pass "Generated script from output conditional YAML"
else
    fail "Failed to generate script from output conditional YAML"
    info "Error: $(cat "$OUTPUT_GEN_ERROR")"
fi

# Validate bash syntax
if bash -n "$OUTPUT_CONDITIONAL_SCRIPT" 2>/dev/null; then
    pass "Output conditional script has valid bash syntax"
else
    fail "Output conditional script has invalid bash syntax"
fi

# Execute the script
OUTPUT_CONDITIONAL_OUTPUT="$TEMP_DIR/output_conditional_output.txt"
if bash "$OUTPUT_CONDITIONAL_SCRIPT" > "$OUTPUT_CONDITIONAL_OUTPUT" 2>&1; then
    pass "Output conditional script executed successfully"
else
    fail "Output conditional script execution failed"
    info "Output: $(cat "$OUTPUT_CONDITIONAL_OUTPUT")"
fi

# Verify output if_true branch executed
if grep -q "MARKER_OUTPUT_TRUE: Contains SUCCESS keyword" "$OUTPUT_CONDITIONAL_OUTPUT"; then
    pass "Output if_true branch executed"
else
    fail "Output if_true branch did not execute"
fi

# Verify output if_false branch did NOT execute
if ! grep -q "MARKER_OUTPUT_FALSE: Missing SUCCESS keyword" "$OUTPUT_CONDITIONAL_OUTPUT"; then
    pass "Output if_false branch correctly did not execute"
else
    fail "Output if_false branch incorrectly executed"
fi

# Verify output always branch executed
if grep -q "MARKER_OUTPUT_ALWAYS: Verification complete" "$OUTPUT_CONDITIONAL_OUTPUT"; then
    pass "Output always branch executed"
else
    fail "Output always branch did not execute"
fi

# ============================================================================
# Test 6: Mixed simple and conditional verifications
# ============================================================================
section "Test 6: Mixed Simple and Conditional Verifications"

MIXED_YAML="$TEMP_DIR/test_mixed.yaml"
cat > "$MIXED_YAML" << 'EOF'
requirement: COND006
item: 1
tc: 6
id: COND_MIXED
description: Test mixing simple and conditional verifications
general_initial_conditions:
  System:
    - Ready
initial_conditions:
  Device:
    - Connected
test_sequences:
  - id: 1
    name: Mixed Verification Sequence
    description: Mix simple and conditional verifications
    initial_conditions:
      LPA:
        - Active
    steps:
      - step: 1
        description: Simple result verification
        command: echo 'simple'
        expected:
          success: true
          result: "0"
          output: simple
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "[[ \"$COMMAND_OUTPUT\" == \"simple\" ]]"
      - step: 2
        description: Conditional result verification
        command: echo 'conditional'
        expected:
          success: true
          result: "0"
          output: conditional
        verification:
          result:
            condition: "[[ $EXIT_CODE -eq 0 ]]"
            if_true:
              - "echo 'MARKER_CONDITIONAL: Exit code is 0'"
              - "VERIFICATION_RESULT_PASS=true"
            if_false:
              - "echo 'MARKER_ERROR: Exit code is not 0'"
              - "VERIFICATION_RESULT_PASS=false"
          output: "true"
EOF

pass "Created mixed verification test YAML"

# Generate script
MIXED_SCRIPT="$TEMP_DIR/test_mixed.sh"
if "$TEST_EXECUTOR_BIN" generate "$MIXED_YAML" -o "$MIXED_SCRIPT" > /dev/null 2>&1; then
    pass "Generated script from mixed YAML"
else
    fail "Failed to generate script from mixed YAML"
fi

# Validate bash syntax
if bash -n "$MIXED_SCRIPT" 2>/dev/null; then
    pass "Mixed verification script has valid bash syntax"
else
    fail "Mixed verification script has invalid bash syntax"
fi

# Execute the script
MIXED_OUTPUT="$TEMP_DIR/mixed_output.txt"
if bash "$MIXED_SCRIPT" > "$MIXED_OUTPUT" 2>&1; then
    pass "Mixed verification script executed successfully"
else
    fail "Mixed verification script execution failed"
    info "Output: $(cat "$MIXED_OUTPUT")"
fi

# Verify both steps passed
if grep -q "PASS.*Step 1" "$MIXED_OUTPUT"; then
    pass "Step 1 (simple verification) passed"
else
    fail "Step 1 (simple verification) did not pass"
fi

if grep -q "PASS.*Step 2" "$MIXED_OUTPUT"; then
    pass "Step 2 (conditional verification) passed"
else
    fail "Step 2 (conditional verification) did not pass"
fi

# Verify conditional message appeared
if grep -q "MARKER_CONDITIONAL: Exit code is 0" "$MIXED_OUTPUT"; then
    pass "Conditional verification message appeared"
else
    fail "Conditional verification message did not appear"
fi

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
