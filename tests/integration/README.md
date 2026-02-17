# Integration Tests

This directory contains end-to-end integration tests for the testcase-manager CLI tool.

## Overview

The integration tests use the [Expect](https://core.tcl-lang.org/expect/index) automation tool to simulate user interactions with the testcase-manager CLI, validating the complete workflow from metadata entry through test sequence and step creation.

## Prerequisites

- **Expect**: The automation tool that sends commands and validates responses
  - Ubuntu/Debian: `sudo apt-get install expect`
  - macOS: `brew install expect`
  - RHEL/CentOS: `sudo yum install expect`

- **Git**: Required for validating git commit functionality
- **Rust toolchain**: Required to build the testcase-manager binary

## Test Suites

### testcase-manager CLI Tests

End-to-end integration tests for the interactive CLI workflow.

### validate-files.sh Tests

Comprehensive integration tests for the file validation framework with caching.

## Test Files

### `test_executor_e2e.sh`

End-to-end integration test for the `test-executor` binary that validates:

1. **YAML Schema Validation**
   - Validates test case YAML files against the JSON schema
   - Tests both passing and failing test cases

2. **Shell Script Generation**
   - Generates executable shell scripts from YAML test cases
   - Validates bash syntax of generated scripts
   - Verifies script structure and essential components

3. **Test Execution with Passing Verification**
   - Executes test cases with passing verification expressions
   - Validates exit code 0 for successful tests
   - Checks for success messages in output

4. **Test Execution with Failing Verification**
   - Executes test cases with failing verification expressions
   - Validates non-zero exit code for failed tests
   - Checks for error messages and verification details in output

5. **Generated Script Structure**
   - Verifies presence of essential bash components (shebang, set -e)
   - Checks for command output and exit code capturing
   - Validates verification logic implementation
   - Confirms PASS/FAIL output formatting

Run with:
```bash
make test-e2e-executor
# or directly
./tests/integration/test_executor_e2e.sh
```

### `test_json_escape_e2e.sh`

End-to-end integration test for the json-escape feature that validates:

1. **Binary Building**
   - Builds the json-escape binary from source
   - Verifies binary exists and is executable

2. **Special Character Escaping**
   - Tests escaping of quotes, newlines, backslashes, tabs
   - Validates JSON string escaping correctness
   - Tests validation mode with --test flag

3. **Script Generation Modes**
   - **RustBinary mode**: Scripts use json-escape binary for JSON escaping
   - **ShellFallback mode**: Scripts use sed/awk for JSON escaping (no binary dependency)
   - **Auto mode**: Scripts check for json-escape and fall back to shell if unavailable

4. **Generated Script Execution**
   - Executes test scripts with commands containing special characters
   - Validates JSON log files are created and valid
   - Uses jq for JSON validation when available

5. **Auto Mode Fallback Testing**
   - Tests with json-escape binary in PATH (should use binary)
   - Tests with json-escape removed from PATH (should use shell fallback)
   - Verifies both paths produce valid JSON output

6. **Complex Character Testing**
   - Mixed special characters (quotes, newlines, tabs, backslashes)
   - JSON-like output strings
   - Multiple special characters in single command

7. **Shell Fallback Validation**
   - Verifies sed/awk escaping patterns work correctly
   - Tests BSD/GNU compatibility (sed -E, not -r)
   - Validates portable shell constructs (bash 3.2+)

Run with:
```bash
./tests/integration/test_json_escape_e2e.sh

# Keep temporary files for debugging
./tests/integration/test_json_escape_e2e.sh --no-remove
```

### `e2e_complete_workflow.exp`

The main end-to-end integration test that validates:

**Note**: This test uses a 60-second timeout to accommodate slower systems and editor/fuzzy search prompt handling. The test automatically skips editor interactions and fuzzy search by sending "n" responses.

1. **Metadata Creation**
   - Prompts for requirement, item, tc, id, and description fields
   - Validates metadata against the schema
   - Commits metadata to git

2. **General Initial Conditions**
   - Prompts for general initial conditions
   - Validates the structure
   - Commits to git

3. **Initial Conditions**
   - Prompts for device-specific initial conditions
   - Validates the structure
   - Commits to git

4. **Test Sequence Creation**
   - Creates test sequences with interactive prompts
   - Validates sequence structure
   - Commits each sequence to git

5. **Step Collection**
   - Adds steps to sequences interactively
   - Validates each step
   - Commits each step to git

6. **Final Output**
   - Validates the complete YAML output file
   - Checks that all required fields are present
   - Verifies the structure matches the expected schema

7. **Git Validation**
   - Verifies that git commits were created at appropriate checkpoints
   - Checks commit messages
   - Validates repository state

### `e2e_basic_workflow.exp`

A simplified version of the complete workflow test that runs faster for smoke testing.

### `validate_files_integration.exp`

Comprehensive integration test suite for `validate-files.sh` that validates:

1. **Validator Command Detection**
   - File-based validator detection and execution
   - Command-based validator detection (e.g., `true`, `false`)
   - Validator accessibility and permission checking

2. **Dual-Layer Caching**
   - Layer 1: Modification time (mtime) checking for fast cache hits
   - Layer 2: SHA256 hash comparison for content-based caching
   - Cache invalidation on content changes

3. **Cache Hit Rate Calculation**
   - Accurate percentage calculation (0%, partial, 100%)
   - Statistics tracking for validated vs cached files

4. **Regex Pattern Matching**
   - POSIX extended regex pattern support
   - Nested directory traversal
   - Multiple file extension matching
   - Edge cases (no matches, hidden files)

5. **Parallel Validation**
   - Multiple file validation in sequence
   - Bulk cache operations
   - Statistics accuracy with many files

6. **Error Propagation**
   - Validation failure detection and reporting
   - Caching of failed validation results
   - Missing validator error handling
   - Non-executable validator detection

7. **CLI Options**
   - Custom cache directories
   - Verbose mode output
   - Help documentation
   - Missing argument detection

See [VALIDATE_FILES_TEST_COVERAGE.md](VALIDATE_FILES_TEST_COVERAGE.md) for detailed coverage documentation.

### `run_e2e_test.sh`

A wrapper script that:
- Checks prerequisites
- Optionally builds the project
- Runs the Expect test script
- Reports results

### `run_validate_files_test.sh`

A wrapper script for running the validate-files.sh integration test suite:
- Checks for expect installation
- Validates test environment
- Runs comprehensive test suite
- Reports test results

## Running the Tests

### Check Environment First

Before running tests, verify your environment is ready:

```bash
./tests/integration/check_environment.sh
```

This will check for:
- Required tools (expect, git, cargo)
- Test script presence and permissions
- Binary existence
- Leftover test artifacts
- Disk space

### Quick Run (assuming binary is built)

```bash
make test-e2e
```

### Run with build

```bash
./tests/integration/run_e2e_test.sh --build
```

### Run all tests (unit + integration)

```bash
make test-all
```

### Run validate-files.sh tests

```bash
./tests/integration/run_validate_files_test.sh
```

### Direct execution

```bash
# CLI workflow tests
cd /path/to/project
./tests/integration/e2e_complete_workflow.exp ./target/debug/testcase-manager

# validate-files.sh tests
./tests/integration/validate_files_integration.exp
```

## Test Workflow

The test performs the following sequence:

```
1. Start CLI with `complete` command
2. Enter metadata fields:
   - Requirement: SGP.22_v3.0
   - Item: 4
   - TC: 2
   - ID: test_e2e_001
   - Description: E2E integration test case
3. Commit metadata → verify git commit
4. Add general initial conditions → commit
5. Add initial conditions (eUICC device) → commit
6. Create test sequence "Basic Profile Installation"
   - Add 2 steps with commands and expected results
   - Commit sequence
   - Commit each step
7. Complete and commit final test case
8. Validate output YAML structure
9. Validate git commit history
10. Cleanup test artifacts
```

## Expected Output

When successful, the test produces output similar to:

```
==========================================
E2E Integration Test for testcase-manager
==========================================
Test directory: test_e2e_1234567890
Output file: test_e2e_1234567890/output_test.yaml
Binary: ./target/debug/testcase-manager

==> Starting testcase-manager complete workflow...
✓ Workflow started

==> Entering metadata...
✓ Metadata validated
✓ Metadata committed to git

==> Adding general initial conditions...
✓ General initial conditions validated
✓ General initial conditions committed to git

==> Adding initial conditions...
✓ Initial conditions validated
✓ Initial conditions committed to git

==> Adding test sequence #1...
✓ Starting sequence 1
✓ Test sequence 1 validated and added
✓ Test sequence 1 committed to git

==> Adding steps to sequence #1...
✓ Adding step 1
✓ Step 1 validated and added
✓ Step 1 committed to git
✓ Adding step 2
✓ Step 2 validated and added
✓ Step 2 committed to git

==> Saving and committing final test case...
✓ Test case saved to file
✓ Final test case committed to git
✓ Workflow completed successfully

==========================================
VALIDATION PHASE
==========================================

==> Validating output YAML file...
✓ Output file exists
✓ All required fields present in YAML
✓ Metadata values correct
✓ Test sequences section present
✓ Steps section present
✓ Test sequence name correct
✓ Step descriptions present
✓ Expected fields present in steps

==> Validating git commits...
✓ Found 7 commits
✓ Commit found: Add test case metadata
✓ Commit found: Add general initial conditions
✓ Commit found: Add initial conditions
✓ Commit found: Add test sequence
✓ Commit found: Add step
✓ Commit found: Complete test case
✓ Git repository is clean

==> Validating with testcase-manager validator...
✓ File passes schema validation

==> Cleaning up test environment...
✓ Test directory removed

==========================================
ALL TESTS PASSED ✓
==========================================
```

## Troubleshooting

### "expect command not found"

Install the Expect package for your system (see Prerequisites section).

### "Binary not found"

Build the project first:
```bash
cargo build
```

Or run with the `--build` flag:
```bash
./tests/integration/run_e2e_test.sh --build
```

### "Timeout waiting for prompt"

The test may timeout if:
- The CLI hangs or crashes
- The prompts have changed
- The timeout value (30 seconds) is too short for your system

To debug, you can add `exp_internal 1` to the top of the `.exp` file to see detailed matching information.

### Test leaves artifacts

If the test fails, it attempts to clean up but may leave a `test_e2e_*` directory. Clean up manually:
```bash
rm -rf test_e2e_*
```

## Extending the Tests

To add additional test scenarios:

1. Create a new `.exp` file in this directory
2. Follow the pattern from `e2e_complete_workflow.exp`
3. Add validation steps for your specific scenario
4. Update the `Makefile` if needed

## CI/CD Integration

These tests can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run E2E Tests
  run: |
    sudo apt-get install -y expect
    make test-e2e
```

## Notes

- Tests create temporary directories with timestamp-based names
- Each test run uses a fresh git repository
- Tests set `GIT_AUTHOR_NAME` and `GIT_AUTHOR_EMAIL` environment variables
- The test validates both the YAML output and git commit history
- All test artifacts are cleaned up on successful completion
