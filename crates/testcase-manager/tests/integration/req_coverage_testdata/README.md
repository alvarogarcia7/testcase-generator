# Requirement Coverage E2E Test Data

This directory contains test data for the `req-coverage` end-to-end integration test.

## Directory Structure

```
req_coverage_testdata/
├── test_cases/              # Test case YAML files
│   ├── REQ001_PASS_TC.yaml
│   ├── REQ001_ADDITIONAL_PASS_TC.yaml
│   ├── REQ002_FAIL_TC.yaml
│   ├── REQ002_MIXED_RESULT_TC.yaml
│   ├── REQ003_PARTIAL_PASS_TC.yaml
│   ├── REQ004_PARTIAL_FAIL_TC.yaml
│   ├── REQ005_NOT_EXECUTED_TC.yaml
│   └── REQ006_MULTI_REQ_TC.yaml
└── verification_results/    # Verification container YAML files
    └── test_results_container.yaml
```

## Test Scenarios

The test data covers the following requirement coverage scenarios:

### 1. Full Coverage - All Tests Passed (REQ-001)

- **Test Cases**: 2 (REQ001_PASS_TC, REQ001_ADDITIONAL_PASS_TC)
- **Coverage Type**: Full
- **Verification Status**: Both passed
- **Expected Result**: `covered_pass` status

### 2. Full Coverage - Mixed Results (REQ-002)

- **Test Cases**: 2 (REQ002_FAIL_TC, REQ002_MIXED_RESULT_TC)
- **Coverage Type**: Full
- **Verification Status**: 1 failed, 1 passed
- **Expected Result**: `covered_fail` status (some tests failed)

### 3. Partial Coverage - All Tests Passed (REQ-003)

- **Test Cases**: 1 (REQ003_PARTIAL_PASS_TC)
- **Coverage Type**: Partial (covers "Authentication with valid credentials")
- **Verification Status**: Passed
- **Expected Result**: `partial_covered_pass` status

### 4. Partial Coverage - Test Failed (REQ-004)

- **Test Cases**: 1 (REQ004_PARTIAL_FAIL_TC)
- **Coverage Type**: Partial (covers "Password complexity validation")
- **Verification Status**: Failed
- **Expected Result**: `partial_covered_fail` status

### 5. Full Coverage - Not Executed (REQ-005)

- **Test Cases**: 1 (REQ005_NOT_EXECUTED_TC)
- **Coverage Type**: Full
- **Verification Status**: No verification results (not executed)
- **Expected Result**: Test status should be `notexecuted`

### 6. Multi-Requirement Coverage (REQ-006, REQ-007, REQ-008)

- **Test Cases**: 1 (REQ006_MULTI_REQ_TC)
- **Coverage Type**: Full
- **Additional Requirements**: REQ-007, REQ-008
- **Verification Status**: Passed
- **Expected Result**: All three requirements should have `covered_pass` status

## Expected Coverage Statistics

When running the e2e test, the following statistics should be displayed:

**Total Requirements**: 8 (REQ-001 through REQ-008)

**Coverage Breakdown**:
- **Fully Covered**: 5 (REQ-001, REQ-002, REQ-005, REQ-006, REQ-007, REQ-008)
  - Note: REQ-005 is counted as having coverage even though not executed
- **Partially Covered**: 2 (REQ-003, REQ-004)
- **Uncovered**: 1 (varies based on implementation)

**Test Case Results** (8 total test cases):
- **Passed**: 5 (62.5%)
  - REQ001_PASS_TC
  - REQ001_ADDITIONAL_PASS_TC
  - REQ002_MIXED_RESULT_TC
  - REQ003_PARTIAL_PASS_TC
  - REQ006_MULTI_REQ_TC
- **Failed**: 2 (25.0%)
  - REQ002_FAIL_TC
  - REQ004_PARTIAL_FAIL_TC
- **Not Executed**: 1 (12.5%)
  - REQ005_NOT_EXECUTED_TC

## Running the Test

Execute the e2e test from the project root:

```bash
# Run the test
bash crates/testcase-manager/tests/integration/test_req_coverage_e2e.sh

# Run the test and keep temporary files for inspection
bash crates/testcase-manager/tests/integration/test_req_coverage_e2e.sh --no-remove
```

## Expected Output

The test will display:

1. **Build Status**: Confirmation that `req-coverage` binary built successfully
2. **Test Data Validation**: Verification that test cases and results exist
3. **JSON Coverage Report**: Generated coverage report in JSON format
4. **Coverage Summary**: Display of:
   - Total requirements and coverage breakdown
   - Test case counts with pass/fail/not-executed percentages
5. **Detailed Breakdown**: Per-requirement coverage details
6. **HTML Report**: Generated HTML report for viewing in browser
7. **Test Results**: Pass/fail status for each validation step

## Test Case YAML Format

Each test case follows the standard test case schema with the addition of `requirement_coverage`:

```yaml
type: test_case
schema: tcms/test-case.schema.v1.json
requirement: REQ-001
item: 1
tc: 1
id: TEST_CASE_ID
description: Test case description
requirement_coverage:
  type: full  # or 'partial'
  covers: "Specific aspect covered"  # Optional, for partial coverage
  additional_requirements:  # Optional, for multi-requirement coverage
    - REQ-002
    - REQ-003
# ... rest of test case definition
```

## Verification Container Format

The verification container follows the standard container schema:

```yaml
title: Test Results Title
project: Project Name
test_date: 2026-01-20T10:00:00.000000Z
test_results:
  - test_case_id: TEST_CASE_ID
    description: Test case description
    overall_pass: true  # or false
  # ... more test results
metadata:
  environment: Test Environment
  execution_duration: 5.0
  total_test_cases: 7
  passed_test_cases: 5
  failed_test_cases: 2
```

## Files Created by Test

When the test runs, it creates temporary files:

- `coverage_report.json` - JSON coverage report
- `html_report/index.html` - HTML coverage report
- `build.log` - Build output log
- `verify.log` - Verify command output log
- `print.log` - Print command output log

Use `--no-remove` flag to inspect these files after test completion.
