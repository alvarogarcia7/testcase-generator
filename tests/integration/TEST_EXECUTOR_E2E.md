# test-executor End-to-End Integration Test

## Overview

The `test_executor_e2e.sh` script provides comprehensive end-to-end testing of the `test-executor` binary, which is responsible for generating and executing test scripts from YAML test case files.

## What It Tests

### 1. YAML Schema Validation
- Validates test case YAML files against the JSON schema (`schemas/schema.json`)
- Tests with two YAML files:
  - **Passing test case**: Contains verification expressions that will pass
  - **Failing test case**: Contains verification expressions that will fail
- Ensures both YAML files are valid according to the schema

### 2. Shell Script Generation
- Calls `test-executor generate` command to create bash scripts
- Validates that:
  - Output files are created successfully
  - Generated scripts have valid bash syntax (checked with `bash -n`)
  - Scripts contain required elements:
    - Bash shebang (`#!/bin/bash`)
    - Test case identifiers
    - Verification logic variables (`VERIFICATION_RESULT_PASS`, etc.)

### 3. Test Execution with Passing Verification
- Executes `test-executor execute` with a test case designed to pass
- Validates:
  - Exit code is 0 (success)
  - Output contains success messages
  - Test completes without errors

### 4. Test Execution with Failing Verification
- Executes `test-executor execute` with a test case designed to fail
- Validates:
  - Exit code is non-zero (failure)
  - Output contains error indicators (FAIL, failed, etc.)
  - Output includes verification details (EXIT_CODE, COMMAND_OUTPUT)

### 5. Generated Script Structure Verification
- Analyzes the generated bash scripts to ensure they contain:
  - Error handling (`set -e`)
  - Command output capturing (`COMMAND_OUTPUT=`)
  - Exit code capturing (`EXIT_CODE=`)
  - PASS/FAIL output messages
  - Proper exit codes (0 for success, 1 for failure)

## Test Files Created

The test creates temporary YAML files:

### test_passing.yaml
```yaml
requirement: TEST001
item: 1
tc: 1
id: TEST_PASSING
description: Test case with passing verification
test_sequences:
  - id: 1
    name: Passing Sequence
    steps:
      - step: 1
        description: Echo test that should pass
        command: echo 'hello'
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]"
```

### test_failing.yaml
```yaml
requirement: TEST002
item: 1
tc: 2
id: TEST_FAILING
description: Test case with failing verification
test_sequences:
  - id: 1
    name: Failing Sequence
    steps:
      - step: 1
        description: Echo test with wrong expected output
        command: echo 'hello'
        verification:
          result: "[ $EXIT_CODE -eq 99 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"goodbye\" ]"
```

## Running the Test

### Via Make
```bash
make test-e2e-executor
```

This will:
1. Build the project (via `make build`)
2. Run the integration test script

### Direct Execution
```bash
./tests/integration/test_executor_e2e.sh
```

Note: Make sure the binaries are built first:
```bash
cargo build
```

### As Part of Full Test Suite
```bash
make test-e2e    # Runs all e2e tests including this one
make test-all    # Runs unit tests + e2e tests
```

## Prerequisites

- Bash shell
- `test-executor` binary (built from `src/bin/test-executor.rs`)
- `validate-yaml` binary (built from `src/bin/validate-yaml.rs`)
- JSON schema file at `schemas/schema.json`

## Expected Output

When all tests pass, you'll see output like:

```
======================================
test-executor End-to-End Integration Test
======================================

=== Checking Prerequisites ===
✓ test-executor binary found
✓ validate-yaml binary found
✓ Schema file found
✓ bash available
ℹ Using temporary directory: /tmp/tmp.XXXXXXXXXX

=== Creating Test YAML Files ===
✓ Created passing test YAML
✓ Created failing test YAML

=== Test 1: YAML Schema Validation ===
✓ Passing YAML validates against schema
✓ Failing YAML validates against schema

=== Test 2: Shell Script Generation and Syntax Validation ===
✓ Generated script from passing YAML
✓ Passing script file created
✓ Passing script has valid bash syntax
✓ Generated script from failing YAML
✓ Failing script file created
✓ Failing script has valid bash syntax
✓ Passing script has bash shebang
✓ Passing script contains test case ID
✓ Passing script contains verification logic

=== Test 3: Execute Test with Passing Verification ===
✓ Passing test execution returned exit code 0
✓ Passing test has correct exit code
✓ Passing test output contains success message

=== Test 4: Execute Test with Failing Verification ===
✓ Failing test execution returned non-zero exit code: 1
✓ Failing test has non-zero exit code
✓ Failing test output contains error indicators
✓ Failing test output includes verification details

=== Test 5: Verify Generated Script Structure ===
✓ Script contains 'set -e' for error handling
✓ Script captures command output
✓ Script captures exit code
✓ Script includes PASS output for steps
✓ Script includes FAIL output for steps
✓ Script includes failure exit code
✓ Script includes success exit code

=== Test Summary ===

Tests Passed: 29
Tests Failed: 0

All tests passed!
```

## Exit Codes

- **0**: All tests passed
- **1**: One or more tests failed

## Troubleshooting

### Binary not found errors
Build the project first:
```bash
cargo build
```

### Schema validation errors
Ensure `schemas/schema.json` exists and is valid JSON.

### Temporary directory permission errors
The test creates a temporary directory using `mktemp -d`. Ensure you have write permissions in your temp directory.

## Cleanup

The test automatically cleans up all temporary files using a trap on EXIT. Even if the test fails, the temporary directory will be removed.

## Integration with CI/CD

This test can be run in CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Run test-executor E2E tests
  run: make test-e2e-executor
```

The test:
- Requires no interactive input
- Has clear exit codes
- Provides detailed output for debugging
- Cleans up after itself
