# Hook Test Cases

This directory contains comprehensive test cases for all 8 hook types in the test execution lifecycle.

## Test Cases Overview

### Individual Hook Type Tests

1. **HOOKS_SCRIPT_START_001.yaml** - Script Start Hook
   - Tests `script_start` hook that sets up environment variables
   - Verifies environment setup at script beginning
   - Tests hook with `on_error: fail`

2. **HOOKS_SETUP_TEST_001.yaml** - Setup Test Hook
   - Tests `setup_test` hook that creates test resources
   - Creates workspace directory and initializes SQLite database
   - Verifies resource creation after script_start

3. **HOOKS_BEFORE_SEQUENCE_001.yaml** - Before Sequence Hook
   - Tests `before_sequence` hook with logging functionality
   - Verifies sequence context variables (TEST_SEQUENCE_ID, TEST_SEQUENCE_NAME)
   - Tests hook execution across multiple sequences

4. **HOOKS_AFTER_SEQUENCE_001.yaml** - After Sequence Hook
   - Tests `after_sequence` hook for cleanup operations
   - Combines with `before_sequence` to create and clean up temporary files
   - Tests hook with `on_error: continue`

5. **HOOKS_BEFORE_STEP_001.yaml** - Before Step Hook
   - Tests `before_step` hook for pre-step validation
   - Verifies step context variables (TEST_STEP_NUMBER, TEST_STEP_DESCRIPTION)
   - Tests hook execution for each step in sequence

6. **HOOKS_AFTER_STEP_001.yaml** - After Step Hook
   - Tests `after_step` hook for metric collection
   - Verifies step exit code tracking (STEP_EXIT_CODE)
   - Tests hook with `on_error: continue`

7. **HOOKS_TEARDOWN_TEST_001.yaml** - Teardown Test Hook
   - Tests `teardown_test` hook for final resource cleanup
   - Combines with `setup_test` to create and clean up resources
   - Verifies cleanup happens after all sequences

8. **HOOKS_SCRIPT_END_001.yaml** - Script End Hook
   - Tests `script_end` hook for execution summary generation
   - Tracks execution time and step count
   - Verifies final cleanup at script end

### Combination and Advanced Tests

9. **HOOKS_MULTI_COMBINED_001.yaml** - All Hooks Combined
   - Tests all 8 hook types working together
   - Verifies correct execution order and context
   - Tests comprehensive logging across entire lifecycle

10. **HOOKS_ON_ERROR_FAIL_001.yaml** - Hook Failure with on_error: fail
    - Tests that hook failure stops test execution
    - Verifies `on_error: fail` behavior
    - Tests fail-fast error handling

11. **HOOKS_ON_ERROR_CONTINUE_001.yaml** - Hook Failure with on_error: continue
    - Tests that hook failure allows test to continue
    - Verifies `on_error: continue` behavior
    - Tests graceful error handling

12. **HOOKS_VARIABLES_001.yaml** - Hooks with Variable Usage
    - Tests hooks accessing environment variables
    - Tests hooks accessing sequence variables
    - Tests hooks with captured variables

13. **HOOKS_SEQUENCE_CONTEXT_001.yaml** - Sequence Context Variables
    - Tests correct sequence context in before_sequence/after_sequence hooks
    - Verifies TEST_SEQUENCE_ID and TEST_SEQUENCE_NAME availability
    - Tests context across multiple sequences

14. **HOOKS_STEP_CONTEXT_001.yaml** - Step Context Variables
    - Tests correct step context in before_step/after_step hooks
    - Verifies TEST_STEP_NUMBER, TEST_STEP_DESCRIPTION availability
    - Tests STEP_EXIT_CODE in after_step hook

## Hook Types Covered

All 8 hook types are tested:

1. **script_start** - Executes once at script beginning
2. **setup_test** - Executes once after script_start
3. **before_sequence** - Executes before each sequence
4. **after_sequence** - Executes after each sequence
5. **before_step** - Executes before each step
6. **after_step** - Executes after each step
7. **teardown_test** - Executes once before script_end
8. **script_end** - Executes once at script end

## Error Handling

Tests cover both error handling modes:

- **on_error: fail** - Hook failure stops test execution immediately
- **on_error: continue** - Hook errors are logged but test continues

## Context Variables Tested

### All Hooks
- Standard environment variables
- Hydration variables (required/optional)

### before_sequence / after_sequence
- TEST_SEQUENCE_ID
- TEST_SEQUENCE_NAME

### before_step / after_step
- TEST_SEQUENCE_ID
- TEST_STEP_NUMBER
- TEST_STEP_DESCRIPTION
- Sequence-scoped variables

### after_step only
- STEP_EXIT_CODE
- COMMAND_OUTPUT

## Running the Tests

These test cases can be used with the test harness to verify hook functionality:

```bash
# Generate test script from a hook test case
cargo run --bin verifier -- testcases/hooks/HOOKS_SCRIPT_START_001.yaml

# Run the generated script
./generated_test_script.sh
```

## Hook Scripts

All hook scripts are located in `test-acceptance/scripts/hooks/` and are executable bash scripts compatible with bash 3.2+.
