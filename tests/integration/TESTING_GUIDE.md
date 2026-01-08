# Integration Testing Guide

Complete guide for running and understanding the testcase-manager integration tests.

## Quick Start

```bash
# 1. Check your environment
./tests/integration/check_environment.sh

# 2. Build the project
cargo build

# 3. Run all integration tests
make test-e2e-all
```

## What Gets Tested

The integration tests use the **Expect** automation tool to simulate real user interactions with the CLI. They test the complete workflow from start to finish:

### Complete Workflow Test

Simulates a power user creating a comprehensive test case:

```
User Input Sequence:
1. Metadata (requirement, item, tc, id, description)
2. General initial conditions (via editor)
3. Initial conditions (device + conditions)
4. Test sequence with name and description
5. Multiple steps with commands and expected results
6. Git commits at each checkpoint

Validations:
✓ Each input is accepted correctly
✓ Schema validation passes at each step
✓ Git commits are created with correct messages
✓ Output YAML file has correct structure
✓ All required fields are present
✓ Data matches what was entered
```

### Basic Workflow Test

Quick smoke test for rapid validation:

```
User Input Sequence:
1. Minimal metadata
2. Skip general initial conditions
3. One device with one condition
4. One test sequence (no steps)

Validations:
✓ Basic flow works end-to-end
✓ File is created
✓ Git commits are made
✓ Schema validation passes
```

## Test Architecture

### Technology Stack

- **Expect**: TCL-based automation tool for interactive programs
  - Timeout: 60 seconds (increased to handle slower systems)
  - Log output: Enabled for debugging
  - Editor/Fuzzy search: Automatically skipped by sending "n"
- **Bash**: Test runners and environment setup
- **Git**: Version control validation
- **YAML**: Output format validation

### File Structure

```
tests/integration/
├── e2e_complete_workflow.exp   # Full workflow test (30s)
├── e2e_basic_workflow.exp      # Quick smoke test (10s)
├── run_e2e_test.sh            # Single test runner
├── run_all_tests.sh           # All tests runner
├── ci_test.sh                 # CI-friendly runner
├── check_environment.sh       # Environment checker
├── README.md                  # User documentation
├── TESTING_GUIDE.md          # This file
└── test_scenarios.md         # Test coverage matrix
```

## Running Tests

### Individual Test Execution

Run the complete workflow test:
```bash
./tests/integration/e2e_complete_workflow.exp ./target/debug/testcase-manager
```

Run the basic workflow test:
```bash
./tests/integration/e2e_basic_workflow.exp ./target/debug/testcase-manager
```

### Using Make Targets

```bash
# Run complete workflow test only
make test-e2e

# Run all integration tests
make test-e2e-all

# Run unit tests + integration tests
make test-all
```

### Using Shell Wrappers

```bash
# With automatic build
./tests/integration/run_e2e_test.sh --build

# Run all tests with build
./tests/integration/run_all_tests.sh --build
```

### CI/CD Execution

```bash
# CI-friendly output format
./tests/integration/ci_test.sh
```

## Understanding Test Output

### Successful Test Run

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

[... more steps ...]

==========================================
VALIDATION PHASE
==========================================

==> Validating output YAML file...
✓ Output file exists
✓ All required fields present in YAML
[... more validations ...]

==> Validating git commits...
✓ Found 7 commits
[... commit validations ...]

==> Cleaning up test environment...
✓ Test directory removed

==========================================
ALL TESTS PASSED ✓
==========================================
```

### Failed Test Run

```
==> Starting testcase-manager complete workflow...
✓ Workflow started

==> Entering metadata...
ERROR: Timeout waiting for Item prompt
```

Common failure points:
1. **Timeout errors**: CLI not responding or prompts changed
2. **Validation errors**: Schema validation failing
3. **File errors**: Output file not created
4. **Git errors**: Commits not being made
5. **Process exit errors**: CLI crashed

## Debugging Tests

### Enable Detailed Logging

Add to the top of your `.exp` file:

```tcl
exp_internal 1  # Show internal Expect operations
log_user 1      # Show all output (already enabled by default)
```

### Adjust Timeout

The tests use a 60-second timeout by default. To change it:

```tcl
set timeout 120  # Increase to 2 minutes
set timeout -1   # Disable timeout (wait forever - for debugging only)
```

### Manual Inspection

Run the test and inspect artifacts before cleanup:

```bash
# Comment out cleanup in test file
# exec rm -rf $test_dir

# Run test
./tests/integration/e2e_complete_workflow.exp

# Inspect output
ls -la test_e2e_*/
cat test_e2e_*/output_test.yaml
git -C test_e2e_*/ log --oneline
```

### Expect Matching Debugging

Add before problematic expect:

```tcl
expect {
    -re "(.*)" {
        puts "Received: $expect_out(1,string)"
        exp_continue
    }
    timeout { puts "Timeout"; exit 1 }
}
```

## Test Data Reference

### Metadata Values

| Field | Complete Test | Basic Test |
|-------|---------------|------------|
| requirement | SGP.22_v3.0 | TEST_REQ |
| item | 4 | 1 |
| tc | 2 | 1 |
| id | test_e2e_001 | basic_test_001 |
| description | E2E integration test case | Basic test |

### Initial Conditions

Complete test:
```yaml
initial_conditions:
  eUICC:
    - Device in test mode
    - Profile storage available
```

Basic test:
```yaml
initial_conditions:
  eUICC:
    - Test condition
```

### Test Sequence Steps

Complete test includes 2 steps:

**Step 1:**
- Description: "Initialize connection to eUICC"
- Command: "ssh init"
- Expected:
  - success: true
  - result: "SW=9000"
  - output: "Connection established"

**Step 2:**
- Description: "Download profile package"
- Command: "download profile.pkg"
- Expected:
  - result: "Package downloaded"
  - output: "100% complete"

## Validation Details

### File Structure Validation

Checks performed on output YAML:

1. **File Existence**: File created at expected location
2. **YAML Parsing**: File is valid YAML
3. **Required Fields**: All mandatory fields present
4. **Field Values**: Values match input
5. **Structure**: Nested structures correct
6. **Schema**: Passes JSON schema validation

### Git Validation

Checks performed on git repository:

1. **Repository Exists**: `.git` directory present
2. **Commit Count**: Minimum number of commits
3. **Commit Messages**: Expected keywords in messages
4. **Working Directory**: Clean (no uncommitted changes)
5. **Branch**: On expected branch

### Schema Validation

Uses the testcase-manager's built-in validator:

```bash
testcase-manager validate --file output.yaml
```

Validates:
- Data types (string, integer, array)
- Required fields
- Field constraints
- Structure hierarchy

## Performance Benchmarks

Expected execution times (on standard hardware):

| Test | Duration | Operations |
|------|----------|------------|
| Basic Workflow | ~10 seconds | Create + 1 sequence + commit |
| Complete Workflow | ~30 seconds | Create + 1 sequence + 2 steps + commits |
| Environment Check | ~1 second | System checks |
| All Tests | ~45 seconds | Both workflows + validation |

## Troubleshooting

### "expect: command not found"

**Problem**: Expect is not installed

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install expect

# macOS
brew install expect

# RHEL/CentOS
sudo yum install expect
```

### "Binary not found"

**Problem**: testcase-manager not built

**Solution**:
```bash
cargo build
# or
./tests/integration/run_e2e_test.sh --build
```

### "Timeout waiting for prompt"

**Problem**: CLI not responding or prompt text changed

**Solution**:
The timeout is now set to 60 seconds which should accommodate slower systems. If you still see timeouts:
1. Check if CLI is working: `./target/debug/testcase-manager --help`
2. Manually run the workflow to see actual prompts
3. Update test script with correct prompt text
4. Add `exp_internal 1` at the top of the `.exp` file for detailed debugging output

### "Permission denied"

**Problem**: Test scripts not executable

**Solution**:
```bash
chmod +x tests/integration/*.sh tests/integration/*.exp
```

### Test leaves directories behind

**Problem**: Test failed before cleanup

**Solution**:
```bash
rm -rf test_e2e_* test_basic_*
```

### Git commit errors

**Problem**: Git not configured

**Solution**:
```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
```

Or tests will use defaults from environment variables.

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Integration Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: sudo apt-get install -y expect
      - run: cargo build
      - run: ./tests/integration/ci_test.sh
```

### GitLab CI Example

```yaml
integration-tests:
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y expect
  script:
    - cargo build
    - ./tests/integration/ci_test.sh
```

### Jenkins Example

```groovy
stage('Integration Tests') {
    steps {
        sh 'sudo apt-get install -y expect'
        sh 'cargo build'
        sh './tests/integration/ci_test.sh'
    }
}
```

## Extending Tests

### Adding a New Test Scenario

1. **Create test file**:
   ```bash
   cp tests/integration/e2e_basic_workflow.exp \
      tests/integration/e2e_my_scenario.exp
   ```

2. **Modify test flow**:
   - Change test directory name
   - Update user inputs
   - Add specific validations

3. **Add to runner**:
   Edit `run_all_tests.sh`:
   ```bash
   run_test "My Scenario" "$SCRIPT_DIR/e2e_my_scenario.exp"
   ```

4. **Document**:
   - Update README.md
   - Add to test_scenarios.md
   - Update this guide

### Test Template

```tcl
#!/usr/bin/expect -f
set timeout 30
set binary_path [lindex $argv 0]
set test_dir "test_my_scenario_[clock seconds]"

# Setup
file mkdir $test_dir
exec git init $test_dir

# Run test
spawn $binary_path [args]

# Interact
expect "prompt:" { send "input\r" }

# Validate
expect "result" { puts "✓ Validated" }

# Cleanup
exec rm -rf $test_dir
exit 0
```

## Best Practices

### Writing Tests

1. **Use Unique Test Directories**: Timestamp-based names prevent conflicts
2. **Clean Up Always**: Use `exec rm -rf` even on failure
3. **Validate Everything**: Check file, git, and schema
4. **Timeout Appropriately**: 30s is usually enough
5. **Log Progress**: Print status messages with ✓/✗

### Maintaining Tests

1. **Keep Tests Fast**: Aim for <1 minute per test
2. **Make Tests Atomic**: Each test independent
3. **Test Realistic Scenarios**: Use actual user workflows
4. **Update with Changes**: Keep tests in sync with CLI
5. **Document Assumptions**: Note what each test validates

### Running Tests

1. **Check Environment First**: Use check_environment.sh
2. **Run Locally Before Push**: Catch issues early
3. **Review Failures**: Don't just re-run, understand why
4. **Keep Git Clean**: Clean up leftover test directories
5. **Monitor Performance**: Watch for slow tests

## Additional Resources

- [Expect Documentation](https://core.tcl-lang.org/expect/index)
- [TCL Tutorial](https://www.tcl.tk/man/tcl/tutorial/tcltutorial.html)
- [testcase-manager README](../../README.md)
- [Test Scenarios Matrix](test_scenarios.md)
- [Integration Tests README](README.md)

## Support

For issues with integration tests:

1. Check this guide's troubleshooting section
2. Run environment checker
3. Review test output carefully
4. Check if CLI prompts changed
5. Open an issue with details

## Contributing

When adding new features to testcase-manager:

1. Add corresponding integration test
2. Update test documentation
3. Ensure all tests pass
4. Update coverage matrix
5. Document test data used
