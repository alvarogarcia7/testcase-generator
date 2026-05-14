# Integration Tests for req-coverage

## Overview

This document describes the integration test suite for the string-based requirement coverage verification feature.

## Test Infrastructure

### Shell Script Runner

**File:** `run_integration_tests.sh`

The integration test runner is a Bash script that:
1. Runs all unit tests
2. Runs all integration tests
3. Saves timestamped results to `test_results/`
4. Creates a symlink to the latest results
5. Displays a summary

**Usage:**
```bash
cd crates/req-coverage
./run_integration_tests.sh
```

**Output:**
- Results saved to `test_results/integration_test_results_YYYYMMDD_HHMMSS.txt`
- Latest results symlinked at `test_results/latest_results.txt`
- Summary displayed in terminal

### Test Results Storage

**Directory:** `test_results/`

Contains:
- Timestamped test result files
- `latest_results.txt` symlink to most recent run
- System information (hostname, Rust version, etc.)
- Complete test output including stdout/stderr

**Result File Format:**
```
======================================
req-coverage Integration Test Results
======================================
Timestamp: [date/time]
Host: [hostname]
Rust version: [version]
Cargo version: [version]

======================================
Unit Test Results
======================================
[unit test output]

======================================
Integration Test Results
======================================
[integration test output]

======================================
Test Summary
======================================
Unit Tests: PASSED/FAILED
Integration Tests: PASSED/FAILED
Completed at: [date/time]
======================================
```

## Integration Test Suite

### Test File

**File:** `tests/string_verification_tests.rs`

Contains 13 comprehensive integration tests covering all aspects of string-based coverage verification.

### Test Cases

#### 1. test_full_coverage_with_single_test_case
**Purpose:** Validates that a single test case can fully cover a requirement when the `covers` string matches the entire requirement text.

**Setup:**
- Requirement: "authenticate users"
- Test case with covers: "authenticate users"

**Assertions:**
- Total requirements: 1
- Fully covered: 1
- Coverage type: Full
- Requirement text stored correctly
- Covered portions contain the string
- No coverage errors

#### 2. test_partial_coverage_with_multiple_test_cases
**Purpose:** Validates that partial coverage is correctly identified when test cases don't cover all requirement text.

**Setup:**
- Requirement: "The system shall authenticate users and deny access"
- Test case 1 covers: "authenticate users"
- Test case 2 covers: "deny access"

**Assertions:**
- Total requirements: 1
- Partially covered: 1
- Coverage type: Partial
- Both covered portions tracked
- No coverage errors

#### 3. test_full_coverage_with_multiple_test_cases
**Purpose:** Validates that multiple test cases with different `covers` strings can combine to fully cover a requirement.

**Setup:**
- Requirement: "authenticate users and deny access"
- Test case 1 covers: "authenticate users "
- Test case 2 covers: "and "
- Test case 3 covers: "deny access"

**Assertions:**
- Total requirements: 1
- Fully covered: 1
- Coverage type: Full
- Three covered portions tracked

#### 4. test_invalid_covers_string_error
**Purpose:** Ensures errors are reported when a test case claims to cover text not found in the requirement.

**Setup:**
- Requirement: "authenticate users"
- Test case covers: "invalid text not in requirement"

**Assertions:**
- Coverage errors present
- Error message contains "not found in requirement"

#### 5. test_missing_requirement_definition
**Purpose:** Ensures errors are reported when a test case references a requirement not in the definitions file.

**Setup:**
- Requirements file contains: REQ-001
- Test case references: REQ-999

**Assertions:**
- Coverage errors present
- Error message contains "Requirement definition not found"

#### 6. test_without_requirements_file
**Purpose:** Ensures the tool works in legacy mode without a requirements file.

**Setup:**
- No requirements file provided
- Test case with covers string

**Assertions:**
- Total requirements: 1
- No requirement text stored
- No covered portions tracked
- No coverage errors
- Backward compatibility maintained

#### 7. test_json_requirements_file
**Purpose:** Validates that JSON format works for requirement definitions (in addition to YAML).

**Setup:**
- Requirements defined in JSON format
- Test case with covers string

**Assertions:**
- Requirements loaded from JSON successfully
- Requirement text stored correctly

#### 8. test_multiple_requirements
**Purpose:** Tests handling of multiple requirements with different coverage states.

**Setup:**
- Three requirements: REQ-001, REQ-002, REQ-003
- REQ-001 fully covered
- REQ-002 partially covered
- REQ-003 uncovered

**Assertions:**
- Total requirements: 3
- Fully covered: 1
- Partially covered: 1
- Uncovered: 1

#### 9. test_coverage_with_test_failures
**Purpose:** Validates that coverage status correctly reflects test pass/fail states.

**Setup:**
- Requirement with full coverage
- Test case marked as failed

**Assertions:**
- Coverage status: CoveredFail

#### 10. test_duplicate_covers_strings
**Purpose:** Tests handling of multiple test cases with identical `covers` strings.

**Setup:**
- Two test cases with same covers string

**Assertions:**
- Both covers strings recorded
- Both test cases counted

#### 11. test_overlapping_covers_strings
**Purpose:** Tests handling of overlapping coverage text.

**Setup:**
- Test case 1 covers: "authenticate users"
- Test case 2 covers: "users and"

**Assertions:**
- Both covers strings recorded
- Overlap handled correctly

#### 12. test_case_sensitive_matching
**Purpose:** Validates that string matching is case-sensitive.

**Setup:**
- Requirement: "Authenticate Users" (capital A and U)
- Test case covers: "authenticate users" (lowercase)

**Assertions:**
- Coverage error reported
- Error indicates mismatch

#### 13. test_empty_covers_string
**Purpose:** Tests behavior when `covers` field is not specified.

**Setup:**
- Test case without covers field

**Assertions:**
- No covered portions tracked (or empty list)

## Helper Functions

### setup_test_env()
Creates temporary test environment with:
- Test cases directory
- Test results directory
- Requirements file path

Returns: `(TempDir, PathBuf, PathBuf, PathBuf)`

### create_test_case_file()
Generates a valid test case YAML file with:
- All required TestCase fields
- Optional requirement_coverage section
- Minimal but valid test sequence structure

**Parameters:**
- `dir`: Directory to create file in
- `id`: Test case ID
- `requirement`: Requirement ID
- `covers`: Optional coverage string

### create_verification_result()
Generates a verification container YAML file with:
- Test result for a specific test case
- Pass/fail status

**Parameters:**
- `dir`: Directory to create file in
- `test_case_id`: Test case ID
- `passed`: Boolean pass/fail status

### create_requirements_file()
Generates a requirements definition YAML file with:
- List of requirements with ID and text

**Parameters:**
- `path`: File path to create
- `requirements`: Array of (id, text) tuples

## Running the Tests

### Using the Shell Script (Recommended)

```bash
cd crates/req-coverage
./run_integration_tests.sh
```

**Output:**
```
======================================
req-coverage Integration Test Runner
======================================
Timestamp: [timestamp]
Results will be saved to: [path]

Running unit tests...
[unit test output]

✅ All unit tests PASSED!

======================================

Running integration tests...
[integration test output]

✅ All integration tests PASSED!

======================================
Test Summary
======================================
Unit Tests: PASSED
Integration Tests: PASSED

Full results saved to: [path]
Latest results also available at: test_results/latest_results.txt

All test result files:
[list of result files]
```

### Manual Cargo Commands

Run integration tests only:
```bash
cargo test -p req-coverage --test string_verification_tests
```

Run with detailed output:
```bash
cargo test -p req-coverage --test string_verification_tests -- --nocapture
```

Run specific test:
```bash
cargo test -p req-coverage --test string_verification_tests test_full_coverage_with_single_test_case
```

## Viewing Test Results

### Latest Results
```bash
cat crates/req-coverage/test_results/latest_results.txt
```

### Specific Run
```bash
cat crates/req-coverage/test_results/integration_test_results_20240120_143022.txt
```

### List All Results
```bash
ls -lht crates/req-coverage/test_results/*.txt
```

## Test Data Format

### Valid Test Case YAML

The integration tests generate minimal but complete test case YAML files:

```yaml
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: REQ-001
item: 1
tc: 1
id: TC-001
description: Test case for REQ-001
requirement_coverage:
  type: partial
  covers: "authenticate users"
general_initial_conditions:
  system:
  - Test system ready
initial_conditions:
  system:
  - Ready
test_sequences:
- id: 1
  name: Test sequence
  description: Test
  initial_conditions:
    system:
    - Ready
  steps:
  - step: 1
    description: Test step
    command: echo test
    expected:
      result: "0"
      output: test
    verification:
      result: '[[ $EXIT_CODE -eq 0 ]]'
```

**Key Points:**
- `expected.result` is a string (not number)
- `expected.output` is required
- `test_sequences[].initial_conditions` is required
- All required fields from TestCase model are present

### Verification Container YAML

```yaml
title: Verification Results
project: Test Project
test_date: "2024-01-20"
test_results:
  - test_case_id: TC-001
    description: Test result
    overall_pass: true
```

### Requirements Definition YAML

```yaml
requirements:
  - id: REQ-001
    text: "authenticate users"
    description: "Test requirement"
```

## Maintenance

### Adding New Tests

1. Add test function to `tests/string_verification_tests.rs`
2. Follow naming convention: `test_[description]`
3. Use existing helper functions
4. Document the test purpose and assertions
5. Run `./run_integration_tests.sh` to verify
6. Update this documentation

### Cleaning Old Results

Keep only last 10 results:
```bash
cd crates/req-coverage/test_results
ls -t integration_test_results_*.txt | tail -n +11 | xargs rm -f
```

### Troubleshooting

**Test YAML parsing errors:**
- Ensure all required fields are present
- Check that `expected.result` is a string
- Verify `test_sequences[].initial_conditions` exists

**Tests not loading:**
- Check directory structure in temp env
- Verify YAML is valid with `serde_yaml`
- Enable debug logging: `RUST_LOG=debug`

## CI/CD Integration

### GitLab CI Example

```yaml
test-req-coverage:
  stage: test
  script:
    - cd crates/req-coverage
    - ./run_integration_tests.sh
  artifacts:
    paths:
      - crates/req-coverage/test_results/
    expire_in: 30 days
    when: always
```

### GitHub Actions Example

```yaml
- name: Run Integration Tests
  run: |
    cd crates/req-coverage
    ./run_integration_tests.sh
    
- name: Upload Test Results
  uses: actions/upload-artifact@v3
  if: always()
  with:
    name: test-results
    path: crates/req-coverage/test_results/
```

## Summary

The integration test suite provides comprehensive validation of the string-based requirement coverage verification feature with:

- **13 integration tests** covering all scenarios
- **Automated test runner** with result archival
- **Complete test data generation** with valid YAML formats
- **Timestamped results** for historical tracking
- **Easy CI/CD integration**

All tests validate end-to-end functionality from test case loading through coverage analysis to report generation.
