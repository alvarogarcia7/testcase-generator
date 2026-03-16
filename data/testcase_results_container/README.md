# Test Case Results Container Schema

## Overview

This directory contains the JSON schema for container YAML files that are accepted by the `test-plan-doc-gen` tool. The container format is used to aggregate multiple test case results with metadata for generating comprehensive test reports in AsciiDoc, Markdown, and PDF formats.

## Schema File

**File**: `schema.json`

**Description**: Defines the structure for container YAML files that aggregate test case verification results. This format is designed to be compatible with the external `test-plan-doc-gen` tool used for generating professional test documentation.

## Container YAML Structure

### Required Fields

#### `test_results` (array, required)

The `test_results` array is the only strictly required field. It must contain at least one test result object.

Each test result object must have:
- `test_case_id`: Unique identifier (non-empty string)
- `sequences`: Array of test sequences (at least one)
- `total_steps`: Total number of steps (integer)
- `passed_steps`: Number of passed steps (integer)
- `failed_steps`: Number of failed steps (integer)
- `not_executed_steps`: Number of not executed steps (integer)
- `overall_pass`: Overall pass/fail status (boolean)

### Recommended Fields

These fields are optional but highly recommended for generating complete reports:

- **`title`**: Report title (e.g., "GSMA eUICC Test Suite Results - Q1 2024")
- **`project`**: Project name (e.g., "GSMA SGP.22 Compliance Testing")
- **`test_date`**: Test execution date in ISO 8601 format (e.g., "2024-03-15T14:30:00Z")
- **`metadata`**: Metadata object containing:
  - `environment`: Test environment description
  - `platform`: Test platform information
  - `executor`: Test executor/framework information
  - `execution_duration`: Duration in seconds
  - `total_test_cases`: Total number of test cases
  - `passed_test_cases`: Number of passed test cases
  - `failed_test_cases`: Number of failed test cases

## Example Container YAML

```yaml
title: 'GSMA eUICC Test Suite Results - Q1 2024'
project: 'GSMA SGP.22 Compliance Testing'
test_date: '2024-03-15T14:30:00Z'
test_results:
  - test_case_id: 'TC_001'
    description: 'Basic functionality test'
    requirement: "REQ_100"
    item: 1
    tc: 1
    sequences:
      - sequence_id: 1
        name: "Test Sequence #01"
        step_results:
          - Pass:
              step: 1
              description: "Execute command"
          - Fail:
              step: 2
              description: "Verify output"
              expected:
                success: true
                result: "0"
                output: "Success"
              actual_result: "1"
              actual_output: "Error"
              reason: "Exit code mismatch"
          - NotExecuted:
              step: 3
              description: "Cleanup"
        all_steps_passed: false
    total_steps: 3
    passed_steps: 1
    failed_steps: 1
    not_executed_steps: 1
    overall_pass: false
metadata:
  environment: 'Test Environment'
  platform: 'Test Case Manager v1.0'
  executor: 'Automated Test Framework'
  execution_duration: 123.45
  total_test_cases: 1
  passed_test_cases: 0
  failed_test_cases: 1
```

## Step Result Variants

Step results use an enum-based format with three variants:

### Pass

Indicates successful step execution.

```yaml
Pass:
  step: 1
  description: "Step description"
```

### Fail

Indicates failed step execution with detailed information.

```yaml
Fail:
  step: 2
  description: "Step description"
  expected:
    success: true
    result: "0"
    output: "Expected output"
  actual_result: "1"
  actual_output: "Actual output"
  reason: "Detailed failure reason"
```

**Note**: The `expected`, `actual_result`, `actual_output`, and `reason` fields are recommended for the `Fail` variant to provide comprehensive failure information.

### NotExecuted

Indicates a step that was not executed (typically due to a previous failure).

```yaml
NotExecuted:
  step: 3
  description: "Step description"
```

## Compatibility Verification

### Using the Compatibility Checker

The `test-plan-documentation-generator-compat` binary validates container YAML files against the expected format:

```bash
# Validate a single file
cargo run --bin test-plan-documentation-generator-compat -- validate container.yaml

# Batch validation
cargo run --bin test-plan-documentation-generator-compat -- batch reports/

# Test against verifier scenarios
cargo run --bin test-plan-documentation-generator-compat -- test-verifier-scenarios

# Generate compatibility report
cargo run --bin test-plan-documentation-generator-compat -- report reports/ --format markdown
```

### Using the Test Script

Run the comprehensive compatibility test script:

```bash
./scripts/test_container_yaml_compatibility.sh
```

This script:
1. Runs the verifier on test scenarios to generate verification results
2. Converts verification results to container YAML format
3. Validates all container YAML files
4. Generates a detailed compatibility report

## Schema Compatibility Notes

### Compatible with Verifier Output

The container schema is designed to be fully compatible with the output format of the verifier tool. The conversion process is:

1. **Verifier** generates JSON verification results (`*_verification.json`)
2. **convert_verification_to_result_yaml.py** converts JSON to individual result YAML files (`*_result.yaml`)
3. Result YAML files can be aggregated into a container YAML with metadata

### test-plan-doc-gen Integration

The container format is specifically designed for the `test-plan-doc-gen` tool, which:

- Accepts container YAML files via the `--container` flag
- Generates AsciiDoc reports from container data
- Supports multiple test results in a single document
- Uses metadata for report headers and summaries

Example usage:

```bash
test-plan-doc-gen \
  --container results_container.yaml \
  --output test_results_report.adoc \
  --format asciidoc
```

## Validation Rules

The compatibility checker enforces these validation rules:

### Errors (will fail validation)

1. Missing or empty `test_results` array
2. Empty `test_case_id` in any test result
3. Empty `sequences` array in any test result
4. Invalid step result format (not a mapping)
5. Step result with no variant key or multiple variant keys

### Warnings (informational)

1. Missing recommended fields (`title`, `project`, `test_date`, `metadata`)
2. Empty sequence names
3. Empty `step_results` in sequences
4. Missing fields in `Fail` variant (`expected`, `actual_result`, `reason`)

### Compatibility Issues (informational)

1. Step count mismatch (passed + failed + not_executed ≠ total_steps)
2. Unknown step result variants (not Pass/Fail/NotExecuted)

## Related Files

- **Schema**: `data/testcase_results_container/schema.json`
- **Example**: `testcases/expected_output_reports/container_data.yml`
- **Verifier Output Schema**: `schemas/verification-result.schema.json`
- **Conversion Script**: `scripts/convert_verification_to_result_yaml.py`
- **Compatibility Checker**: `src/bin/test-plan-documentation-generator-compat.rs`
- **Test Script**: `scripts/test_container_yaml_compatibility.sh`

## Usage in Report Generation Workflow

The container YAML format fits into the overall report generation workflow:

```
Test Case (YAML)
      ↓
Test Execution (generates execution log JSON)
      ↓
Verifier (generates verification result JSON)
      ↓
Conversion Script (generates result YAML files)
      ↓
Container Aggregation (combines into container YAML)
      ↓
test-plan-doc-gen (generates AsciiDoc/Markdown/PDF reports)
```

See `docs/report_generation.md` for complete workflow documentation.

## Contributing

When modifying the container schema:

1. Update `data/testcase_results_container/schema.json`
2. Update example files in `testcases/expected_output_reports/`
3. Run validation tests: `./scripts/test_container_yaml_compatibility.sh`
4. Update this README with any schema changes
5. Verify compatibility with `test-plan-doc-gen` tool

## References

- **Report Generation**: `docs/report_generation.md`
- **Schema Documentation**: `schemas/README.md`
- **Verifier Usage**: `docs/TEST_VERIFY_USAGE.md`
