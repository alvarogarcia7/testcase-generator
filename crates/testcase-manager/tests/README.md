# Integration Tests

This directory contains integration tests for the testcase-manager application.

## End-to-End Integration Test

### Overview

The `test_end_to_end_complete_workflow()` test in `integration_test.rs` provides comprehensive end-to-end testing of the testcase-manager binary by:

1. **Spawning the binary as a child process** - Executes the `testcase-manager complete` command
2. **Providing scripted input** - Simulates user interactions through stdin
3. **Capturing output** - Collects stdout and stderr for validation
4. **Validating YAML generation** - Verifies the generated YAML file structure
5. **Parsing with existing parser** - Uses the TestCase model to validate parsing
6. **Checking git commits** - Verifies that expected commits were created

### Test Workflow

The test simulates the following user workflow:

#### 1. Metadata Entry
- Requirement: `REQ-TEST-001`
- Item: `42`
- TC: `1`
- ID: `TC_Integration_Test_001`
- Description: `End-to-end integration test for testcase-manager workflow`

#### 2. General Initial Conditions
- Device: `eUICC`
- Condition: `General initial condition 1`

#### 3. Initial Conditions
- Device: `eUICC`
- Conditions:
  - `Initial condition 1`
  - `Initial condition 2`

#### 4. Test Sequence #1
- Name: `Test Sequence 1`
- Sequence-specific initial condition: `Sequence-specific condition 1` (eUICC)
- Steps:
  - **Step 1**: Execute test command
    - Command: `ssh test-device`
    - Expected result: `0x9000`
    - Expected output: `Success output`
  - **Step 2**: Verify results
    - Command: `ssh verify`
    - Expected result: `OK`
    - Expected output: `Verification passed`

### Git Commits Validated

The test verifies that the following commits are created:

1. Metadata commit: `TEST: Add test case metadata`
2. General initial conditions commit: `TEST: Add general initial conditions`
3. Initial conditions commit: `TEST: Add initial conditions`
4. Test sequence commit: `TEST: Add test sequence #1`
5. Step commits: `TEST: Add step #1 to sequence #1: Execute test command`, etc.
6. Final commit: `TEST: Complete test case with all sequences and steps`

### Validation Steps

#### YAML Structure Validation
- Checks for presence of all required fields
- Validates field values match expected inputs
- Ensures nested structures (sequences, steps) are present

#### YAML Parsing Validation
- Parses the generated YAML using the `TestCase` struct
- Validates deserialized data matches expected structure
- Checks counts (2 initial conditions, 1 sequence, 2 steps)
- Validates step details (numbers, descriptions, commands, expected results)

#### Git Commit Validation
- Opens the git repository
- Retrieves commit log (up to 20 commits)
- Verifies presence of expected commit messages
- Ensures at least 5 commits were created
- Prints commit history for debugging

### Running the Test

```bash
# Build the binary first
cargo build

# Run the integration test
cargo test --test integration_test

# Run with output
cargo test --test integration_test -- --nocapture
```

### Test Requirements

- The `testcase-manager` binary must be built before running the test
- The test creates a temporary directory for the git repository
- Git author information is read from environment variables or defaults to:
  - Name: `Test Case Manager`
  - Email: `testcase@example.com`

### Test Output

The test provides detailed output including:
- Test directory path
- Binary path
- Scripted input (first 500 characters)
- Process stdout and stderr
- Generated YAML content
- Git commit history
- Validation results

### Notes

- The test uses `tempfile::TempDir` to create isolated test environments
- Each test run creates a fresh git repository
- The test avoids fuzzy search interactions by answering "n" to fuzzy prompts
- All user inputs are scripted to avoid requiring interactive terminal sessions
