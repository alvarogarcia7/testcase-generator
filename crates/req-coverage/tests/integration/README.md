# Shell-Based Integration Tests

This directory contains shell-based integration tests for the `req-coverage` tool. These tests validate the tool's functionality end-to-end by creating test scenarios, running the binary, and verifying the output.

## Prerequisites

- `jq` - JSON processor (install with `brew install jq` on macOS or `apt-get install jq` on Linux)
- Built `req-coverage` binary (run `cargo build -p req-coverage` first)

## Running the Tests

### Run All Tests

```bash
./test_runner.sh
```

### Check Test Results

Test results are saved to the `results/` directory:
- `*.json` - Coverage report outputs from each test
- `*.log` - Command output logs from each test
- `test_html_output/` - Generated HTML report (from HTML generation test)

## Test Coverage

The integration test suite includes:

1. **test_full_coverage_single_test** - Validates full coverage detection with a single test case that covers entire requirement text
2. **test_partial_coverage_multiple_tests** - Validates partial coverage detection when multiple test cases don't cover all requirement text
3. **test_invalid_covers_string** - Validates error reporting when a test case claims to cover text not in requirement
4. **test_without_requirements_file** - Validates backward compatibility when no requirements file is provided
5. **test_json_requirements_format** - Validates support for JSON format requirements file
6. **test_multiple_requirements** - Validates handling of multiple requirements with different coverage states
7. **test_coverage_with_failures** - Validates coverage status calculation when tests fail
8. **test_html_generation** - Validates HTML report generation
9. **test_case_sensitive_matching** - Validates case-sensitive string matching
10. **test_duplicate_covers_strings** - Validates handling of multiple test cases with same covers string

## Test Structure

Each test:
1. Creates a temporary test environment with:
   - `testcases/` directory with test case YAML files
   - `results/` directory with verification result YAML files
   - `requirements.yaml` or `requirements.json` file (when testing string verification)
2. Runs the `req-coverage` binary
3. Validates the output using `jq` to parse JSON
4. Saves results to the `results/` directory
5. Cleans up temporary files

## Adding New Tests

To add a new test:

1. Create a test function following the naming convention `test_<name>()`
2. Use helper functions:
   - `create_requirement_file <id> <text>` - Create a requirements.yaml file
   - `create_test_case <id> <req_id> <covers>` - Create a test case YAML file
   - `create_verification_result <id> <passed>` - Create a verification result YAML file
   - `validate_json_field <file> <field> <expected>` - Validate JSON field value
   - `validate_requirement_count <file> <total> <full> <partial> <uncovered>` - Validate coverage counts
3. Add the test to the main() function
4. Save test results to `${RESULTS_DIR}/<test_name>.json` and `${RESULTS_DIR}/<test_name>.log`

Example:

```bash
test_my_new_test() {
    create_requirement_file "REQ-001" "test requirement"
    create_test_case "TC-001" "REQ-001" "test"
    create_verification_result "TC-001" "true"
    
    local output="${TEMP_DIR}/coverage.json"
    
    "${BINARY}" verify \
        --test-cases-folder "${TEMP_DIR}/testcases" \
        --test-results-folder "${TEMP_DIR}/results" \
        --output "${output}" \
        --requirements-file "${TEMP_DIR}/requirements.yaml" \
        > "${RESULTS_DIR}/test_my_new_test.log" 2>&1
    
    # Validate output
    validate_requirement_count "${output}" 1 1 0 0 || return 1
    
    # Save results
    cp "${output}" "${RESULTS_DIR}/test_my_new_test.json"
    
    return 0
}
```

Then add to main():
```bash
run_test "My new test description" test_my_new_test
```

## Debugging Tests

If a test fails:

1. Check the log file in `results/<test_name>.log` for command output
2. Check the JSON output in `results/<test_name>.json` to see actual results
3. The temporary directory is cleaned up automatically, but you can modify the script to keep it by commenting out the cleanup trap

## Continuous Integration

These tests can be integrated into CI/CD pipelines:

```yaml
# Example GitLab CI
integration-tests:
  stage: test
  script:
    - cargo build -p req-coverage
    - cd crates/req-coverage/tests/integration
    - ./test_runner.sh
  artifacts:
    paths:
      - crates/req-coverage/tests/integration/results/
    when: always
```

## Troubleshooting

### "jq is required"
Install jq: `brew install jq` (macOS) or `apt-get install jq` (Linux)

### "req-coverage binary not found"
Run `cargo build -p req-coverage` from the workspace root first

### Tests fail with "Failed to parse YAML"
Check the test case YAML format matches the testcase-models schema. The `create_test_case` helper should generate valid YAML, but if the schema changes, the helper may need updates.
