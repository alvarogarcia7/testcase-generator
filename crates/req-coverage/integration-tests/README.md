# Integration Tests for req-coverage

This directory contains shell script-based integration tests for the `req-coverage` tool.

## Overview

The integration tests validate the complete workflow of the req-coverage tool including:
- Coverage analysis with requirement definitions
- String-based coverage verification
- Error detection
- HTML report generation
- Backward compatibility
- JSON and YAML format support

## Running the Tests

### Prerequisites

- `jq` - JSON processor (install with: `brew install jq` on macOS or `apt-get install jq` on Ubuntu)
- Rust toolchain (for building req-coverage)

### Run All Tests

```bash
cd crates/req-coverage/integration-tests
./run_integration_tests.sh
```

The script will:
1. Build the req-coverage binary
2. Run 8 comprehensive integration tests
3. Generate test results and logs
4. Display a summary of passed/failed tests

### Test Results

Results are saved to `crates/req-coverage/integration-tests/results/`:
- `test_summary.txt` - Overall test summary
- `test*_coverage.json` - Coverage JSON outputs for each test
- `test*.log` - Detailed logs for each test
- `html_report/` - Generated HTML report (from Test 6)

## Test Cases

### Test 1: Full Coverage with Single Test Case
Validates that a single test case can fully cover a requirement when the `covers` string matches the entire requirement text.

### Test 2: Partial Coverage with Multiple Test Cases
Validates that multiple test cases with different `covers` strings result in partial coverage when they don't cover all requirement text.

### Test 3: Invalid Covers String Error Detection
Ensures errors are properly reported when a test case claims to cover text not found in the requirement.

### Test 4: Backward Compatibility
Tests that the tool works without a requirements file (legacy mode).

### Test 5: JSON Requirements File Format
Validates that JSON format works for requirement definitions in addition to YAML.

### Test 6: HTML Report Generation
Tests the generation of HTML reports from coverage JSON.

### Test 7: Multiple Requirements with Different Coverage States
Tests handling of multiple requirements with different coverage levels (full, partial, uncovered).

### Test 8: Coverage with Failing Tests
Validates that coverage status correctly reflects test pass/fail states.

## Test Data Structure

The test script creates temporary test data in `test-data/`:
```
test-data/
├── testcases/          # Generated test case YAML files
├── verification_results/  # Generated verification result files
└── requirements.yaml   # Generated requirement definitions
```

All test data is cleaned up between tests.

## Continuous Integration

Add to your CI pipeline:

```yaml
- name: Run Integration Tests
  run: |
    cd crates/req-coverage/integration-tests
    ./run_integration_tests.sh
```

The script exits with code 0 if all tests pass, or 1 if any test fails.

## Troubleshooting

### Missing jq

If you get "jq: command not found":
```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq

# CentOS/RHEL
sudo yum install jq
```

### Build Failures

If the binary fails to build, ensure you're in the workspace root or the build command can find Cargo.toml.

### Permission Denied

If you get permission denied:
```bash
chmod +x run_integration_tests.sh
```

## Adding New Tests

To add a new test:

1. Add a new test section to `run_integration_tests.sh`
2. Follow the pattern:
   ```bash
   echo "Test N: Description"
   setup_test_env
   # Create test data
   create_requirements_file
   create_test_case "TC-XXX" "REQ-XXX" "coverage text"
   create_verification_result "TC-XXX" "true"
   
   # Run the command
   "$BINARY" verify ... > "$RESULTS_DIR/testN.log" 2>&1
   
   # Validate results
   if [ $? -eq 0 ]; then
       # Check output with jq
       print_result "Test N" "PASS"
   else
       print_result "Test N" "FAIL" "reason"
   fi
   ```

3. Update this README with the new test description

## Test Coverage

The integration tests cover:
- ✅ String-based requirement verification
- ✅ Full coverage detection
- ✅ Partial coverage detection
- ✅ Error detection and reporting
- ✅ Multiple requirements
- ✅ YAML format support
- ✅ JSON format support
- ✅ Backward compatibility (without requirements file)
- ✅ Test pass/fail status integration
- ✅ HTML report generation
- ✅ Coverage statistics accuracy
