# Test Orchestrator Example Data

This directory contains example input and output files for each subcommand of the `test-orchestrator` binary.

## Example Files

### For `run` Subcommand
- `EXAMPLE_RUN_001.yml` - Test case for the run subcommand
- `EXAMPLE_RUN_002.yml` - Second test case for the run subcommand

### For `run-all` Subcommand
- `EXAMPLE_RUN_ALL_A.yml` - First test case for run-all subcommand
- `EXAMPLE_RUN_ALL_B.yml` - Second test case for run-all subcommand

### For `verify` Subcommand
- `EXAMPLE_VERIFY_001.yml` - Test case for verification
- `EXAMPLE_VERIFY_001_execution_log.json` - Execution log for verification

## Subcommand Examples

### 1. `run` - Execute Specific Test Cases

The `run` subcommand executes specific test cases by ID.

**Create a temporary test directory:**
```bash
mkdir -p /tmp/orchestrator-run-test
cp examples/EXAMPLE_RUN_001.yml /tmp/orchestrator-run-test/
cp examples/EXAMPLE_RUN_002.yml /tmp/orchestrator-run-test/
```

**Usage:**
```bash
# Run a single test case
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p /tmp/orchestrator-run-test

# Run multiple test cases
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 EXAMPLE_RUN_002 -p /tmp/orchestrator-run-test

# Run with verbose output
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p /tmp/orchestrator-run-test -v

# Run with retry enabled
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p /tmp/orchestrator-run-test --retry --max-retries 3

# Run with exponential backoff
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p /tmp/orchestrator-run-test --retry --exponential-backoff

# Run with multiple workers
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 EXAMPLE_RUN_002 -p /tmp/orchestrator-run-test -w 2

# Run and save results
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p /tmp/orchestrator-run-test --save

# Run and generate report
cargo run --bin test-orchestrator -- run EXAMPLE_RUN_001 -p /tmp/orchestrator-run-test --report
```

### 2. `run-all` - Execute All Test Cases

The `run-all` subcommand executes all available test cases in the storage directory.

**Create a temporary test directory:**
```bash
mkdir -p /tmp/orchestrator-run-all-test
cp examples/EXAMPLE_RUN_ALL_A.yml /tmp/orchestrator-run-all-test/
cp examples/EXAMPLE_RUN_ALL_B.yml /tmp/orchestrator-run-all-test/
```

**Usage:**
```bash
# Run all test cases
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test

# Run all with verbose output
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test -v

# Run all with multiple workers
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test -w 4

# Run all with retry
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test --retry --max-retries 2

# Run all and save results
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test --save

# Run all and generate report
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test --report

# Run all with all options
cargo run --bin test-orchestrator -- run-all -p /tmp/orchestrator-run-all-test -w 8 --retry --exponential-backoff --save --report -v
```

### 3. `verify` - Verify Test Execution Results

The `verify` subcommand verifies test execution results from log files.

**Usage:**
```bash
# Verify with specific test case and execution log
cargo run --bin test-orchestrator -- verify \
  --test-case examples/EXAMPLE_VERIFY_001.yml \
  --execution-log examples/EXAMPLE_VERIFY_001_execution_log.json

# Verify with verbose output
cargo run --bin test-orchestrator -- verify \
  --test-case examples/EXAMPLE_VERIFY_001.yml \
  --execution-log examples/EXAMPLE_VERIFY_001_execution_log.json \
  -v

# Verify multiple log files from test-output directory
# (requires running tests first to generate log files)
cargo run --bin test-orchestrator -- verify test-output/*.json
```

### 4. `info` - Show Orchestrator Configuration

The `info` subcommand displays the current orchestrator configuration and available test cases.

**Usage:**
```bash
# Show info with default path
cargo run --bin test-orchestrator -- info

# Show info with custom test case path
cargo run --bin test-orchestrator -- info -p /tmp/orchestrator-run-test

# Show info with custom output path
cargo run --bin test-orchestrator -- info -p /tmp/orchestrator-run-all-test -o custom-output
```

## Quick Test Script

To test all subcommands with the example data, run:

```bash
make test-e2e-orchestrator-examples
```

Or run the script directly:

```bash
./tests/integration/test_orchestrator_examples.sh
```

This script tests:
- `run` subcommand (single and multiple test cases)
- `run-all` subcommand
- `verify` subcommand
- `info` subcommand

The test is automatically included in the `make test` goal.

## Expected Output Examples

### Run Command Output
```
=== Test Execution Started ===
Workers: 4 | Total tests: 1 | Retry: disabled

Progress: [##########] 100% | Completed: 1/1 | Passed: 1 | Failed: 0 | Running: 0
Elapsed: 0.5s | Success rate: 100.00%

=== Execution Summary ===
Total tests: 1
Passed: 1
Failed: 0
Total duration: 0.5s
Average duration: 0.5s per test
```

### Verify Command Output
```
=== Verifying Specific Test Case ===

✓ PASS EXAMPLE_VERIFY_001 - Example test case for the verify subcommand (4/4 steps passed)
```

### Info Command Output
```
=== Test Orchestrator Configuration ===

Test case storage path: /tmp/orchestrator-run-test
Output directory: test-output

Available test cases: 2

Test Cases:
  1. EXAMPLE_RUN_001 - Example test case for the run subcommand
  2. EXAMPLE_RUN_002 - Second example test case for the run subcommand

Default Configuration:
  Workers: 4
  Retry policy: No retry
  Verbose mode: Disabled

Usage Examples:
  # Run specific test cases
  test-orchestrator run TC001 TC002

  # Run all test cases with 8 workers
  test-orchestrator run-all -w 8

  # Run with retry (3 attempts)
  test-orchestrator run TC001 --retry --max-retries 3
```

## Notes

- All example test cases use simple shell commands (echo, true, false, date) that are available on any Unix-like system
- The test cases are designed to always pass when executed properly
- Execution logs are in JSON format with timestamp, exit code, and output for each step
- Test case files are in YAML format following the testcase-manager schema
- Use temporary directories for testing to avoid cluttering the project testcases directory
