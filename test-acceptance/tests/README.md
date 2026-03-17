# Acceptance Suite E2E Tests

This directory contains end-to-end integration tests for the acceptance test suite orchestrator.

## Overview

The E2E tests validate that `run_acceptance_suite.sh` works correctly by running it on a subset of test cases and verifying all stages complete successfully.

## Test Script

### `test_acceptance_suite_e2e.sh`

Comprehensive E2E integration test that validates the entire acceptance suite workflow.

**Features:**
- Creates isolated test environments with subset of test cases (5 success, 3 failure, 2 hook scenarios)
- Validates all 7 stages complete successfully
- Checks expected files are created at each stage
- Validates final report is generated with correct statistics
- Tests --skip-* flags work correctly
- Ensures --verbose flag increases logging detail
- Verifies error handling for missing dependencies (TPDG not available)
- Tests timeout handling for long-running scripts
- Confirms cleanup of temporary files after completion

**Test Cases Included:**

Success Cases (5):
- TC_SUCCESS_CMD_CHAIN_001.yaml
- TC_SUCCESS_COMPLEX_DATA_001.yaml
- TC_SUCCESS_CONDITIONAL_001.yaml
- TC_SUCCESS_EMPTY_OUTPUT_001.yaml
- TC_SUCCESS_ENV_VARS_001.yaml

Failure Cases (3):
- TC_FAILURE_COMMAND_NOT_FOUND_001.yaml
- TC_FAILURE_EXIT_CODE_MISMATCH_001.yaml
- TC_FAILURE_OUTPUT_MISMATCH_001.yaml

Hook Cases (2):
- HOOKS_AFTER_SEQUENCE_001.yaml
- HOOKS_AFTER_STEP_001.yaml

## Running Tests

### Using Make

```bash
# Build binaries and run E2E acceptance tests
make test-e2e-acceptance
```

### Direct Execution

```bash
# Build required binaries first
make build-acceptance-binaries

# Run the E2E test
./test-acceptance/tests/test_acceptance_suite_e2e.sh
```

### With Verbose Output

```bash
VERBOSE=1 ./test-acceptance/tests/test_acceptance_suite_e2e.sh
```

## Test Coverage

The E2E test validates 12 different scenarios:

1. **Basic Execution** - Verifies all 6 stages complete successfully
2. **File Creation** - Checks expected files are created at each stage
3. **Final Report Statistics** - Validates report structure and statistics
4. **Skip Generation Flag** - Tests `--skip-generation` flag
5. **Skip Execution Flag** - Tests `--skip-execution` flag
6. **Skip Verification Flag** - Tests `--skip-verification` flag
7. **Skip Documentation Flag** - Tests `--skip-documentation` flag
8. **Verbose Flag** - Ensures `--verbose` increases logging detail
9. **Missing TPDG Handling** - Verifies graceful handling when TPDG unavailable
10. **Timeout Handling** - Tests timeout handling for long-running scripts
11. **Cleanup** - Confirms temporary files are cleaned up after completion
12. **Multiple Skip Flags** - Tests combining multiple `--skip-*` flags

## Test Isolation

Each test case creates an isolated test environment:
- Temporary directory with subset of test cases
- Separate directories for scripts, logs, results, and reports
- No interference with actual acceptance test data
- Automatic cleanup after test completion

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Prerequisites

The following binaries must be built before running the tests:
- `validate-yaml`
- `test-executor`
- `verifier`

These are automatically built when using `make test-e2e-acceptance`.

## Test Output

The test provides detailed output for each test case:
- Section headers for each test
- Pass/fail status with checkmarks (✓) and X marks (✗)
- Final summary with test counts
- Verbose logging when enabled

Example output:
```
=== Test: Basic Execution - All Stages Complete ===
[INFO] Creating isolated test environment...
[INFO] Running acceptance suite on test subset...
✓ All 6 stages executed
✓ Basic Execution - All Stages Complete

=== Test Summary ===
[INFO] Tests run:    12
[INFO] Tests passed: 12
[INFO] Tests failed: 0

✓ All tests passed!
```

## Troubleshooting

### Tests Fail Due to Missing Binaries

**Error:** "Required binary not found"

**Solution:** Build the required binaries:
```bash
make build-acceptance-binaries
```

### TPDG Not Available

**Note:** Tests gracefully handle missing TPDG (test-plan-documentation-generator). The missing TPDG handling test verifies this behavior.

If you want full documentation generation testing, install TPDG:
```bash
cargo install test-plan-documentation-generator
```

Or set the path:
```bash
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator
```

### Tests Timeout

Tests have a 300-second (5-minute) timeout for the acceptance suite execution. If tests timeout:
1. Check system performance
2. Verify no background processes are interfering
3. Check individual test case execution times

### Cleanup Issues

If temporary files are not cleaned up properly:
1. Check for errors in test output
2. Manually remove stray temp directories: `rm -rf /tmp/tmp.*/`
3. Verify trap handlers are working correctly

## Development

### Adding New Tests

To add a new test case to the E2E test:

1. Define a new test function:
```bash
test_new_feature() {
    local test_env="$TEST_WORKSPACE/new_feature"
    create_test_environment "$test_env"
    
    # Test logic here
    
    return 0  # success
}
```

2. Add the test to the main function:
```bash
run_test "New Feature Description" test_new_feature
```

### Modifying Test Subset

To change which test cases are included in the subset:

Edit the arrays in `create_test_environment()`:
```bash
local success_tests=(
    "TC_SUCCESS_CMD_CHAIN_001.yaml"
    # Add more success tests
)

local failure_tests=(
    "TC_FAILURE_COMMAND_NOT_FOUND_001.yaml"
    # Add more failure tests
)

local hook_tests=(
    "HOOKS_AFTER_SEQUENCE_001.yaml"
    # Add more hook tests
)
```

## Integration with CI/CD

The E2E test is designed to be run in CI/CD pipelines:
- Exit codes indicate success/failure
- Output is machine-parseable
- Isolated test environments prevent interference
- Automatic cleanup ensures no residual state

Add to your CI pipeline:
```yaml
- name: Run Acceptance Suite E2E Tests
  run: make test-e2e-acceptance
```
