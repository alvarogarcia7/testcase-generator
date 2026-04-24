# req-coverage

Requirement coverage analysis tool for tracking test case coverage of requirements.

## Overview

`req-coverage` generates requirement coverage reports by analyzing test cases and their verification results. It produces JSON coverage data and beautiful HTML reports showing which requirements are covered by tests and their pass/fail status.

## Features

- **Coverage Analysis**: Analyzes test cases to determine requirement coverage
- **Verification Integration**: Reads verification container YAML files to determine test pass/fail status
- **JSON Reports**: Generates structured JSON reports for programmatic processing
- **HTML Reports**: Creates interactive, self-contained HTML reports for easy viewing
- **Full & Partial Coverage**: Supports both full and partial requirement coverage tracking

## Installation

Build from the workspace root:

```bash
cargo build --release -p req-coverage
```

The binary will be available at `target/release/req-coverage`.

## Usage

### 1. Generate Coverage Report (JSON)

Analyze test cases and verification results to produce a JSON coverage report:

```bash
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-results \
  --output req-coverage.json
```

**Options:**
- `--test-cases-folder`: Directory containing test case YAML files
- `--test-results-folder`: Directory containing verification container YAML files (e.g., `*_container.yaml`)
- `--output`: Path to output JSON file

### 2. Generate HTML Report

Convert the JSON coverage report to an interactive HTML report:

```bash
req-coverage print \
  --format html \
  --input req-coverage.json \
  --output ./coverage-report/
```

**Options:**
- `--format`: Output format (currently only `html` supported)
- `--input`: Path to coverage JSON file
- `--output`: Output directory for HTML files
- `--template`: (Optional) Path to custom HTML template file

The HTML report will be created as `index.html` in the output directory.

### 2a. Generate HTML Report with Custom Template

Use a custom HTML template to style your coverage report:

```bash
req-coverage print \
  --format html \
  --input req-coverage.json \
  --output ./coverage-report/ \
  --template ./my-template.html
```

See `docs/examples/html_template_example.html` for a template example with available placeholders:
- `{{GENERATED_AT}}` - Report generation timestamp
- `{{TOTAL_REQUIREMENTS}}` - Total number of requirements
- `{{FULLY_COVERED}}` - Number of fully covered requirements
- `{{PARTIALLY_COVERED}}` - Number of partially covered requirements
- `{{UNCOVERED}}` - Number of uncovered requirements
- `{{REQUIREMENTS_ROWS}}` - HTML table rows for requirements

### 3. View the Report

Open the generated HTML report in your browser:

```bash
open ./coverage-report/index.html
```

## Test Case Requirements Coverage

Test cases specify requirement coverage in their YAML definition:

### Full Coverage

A test case that covers the entire requirement:

```yaml
requirement: REQ-001
requirement_coverage:
  type: full
```

### Partial Coverage

A test case that covers specific aspects of a requirement:

```yaml
requirement: REQ-002
requirement_coverage:
  type: partial
  covers: "Authentication with valid credentials"
```

### Multiple Requirements Coverage

A test case can cover multiple requirements using the `additional_requirements` field:

```yaml
requirement: REQ-002
requirement_coverage:
  type: partial
  covers: "Password reset via email"
  additional_requirements:
    - AUTH-003
    - SEC-005
```

This test case will be counted toward coverage for REQ-002, AUTH-003, and SEC-005.

**Note**: The tool defaults to `full` coverage type if `requirement_coverage` is not specified.

## Requirement Definitions File

To enable string-based coverage verification, you can provide a requirement definitions file that contains the full text of each requirement. The tool will then verify that:

1. The `covers` strings in test cases are actually present in the requirement text
2. All parts of the requirement text are covered by the cumulation of `covers` strings across all test cases
3. Returns errors if test cases claim to cover text not found in the requirement

### Requirement Definitions File Format

Create a YAML or JSON file with the following structure:

**YAML format (requirements.yaml):**
```yaml
requirements:
  - id: REQ-001
    text: "The system shall authenticate users with valid credentials and deny access to users with invalid credentials."
    description: "User authentication requirement"
  
  - id: REQ-002
    text: "The system shall allow password reset via email and log all password reset attempts."
    description: "Password reset requirement"
```

**JSON format (requirements.json):**
```json
{
  "requirements": [
    {
      "id": "REQ-001",
      "text": "The system shall authenticate users with valid credentials and deny access to users with invalid credentials.",
      "description": "User authentication requirement"
    },
    {
      "id": "REQ-002",
      "text": "The system shall allow password reset via email and log all password reset attempts.",
      "description": "Password reset requirement"
    }
  ]
}
```

### Using Requirement Definitions

Pass the requirements file when running the verify command:

```bash
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-results \
  --output req-coverage.json \
  --requirements-file ./requirements.yaml
```

### String-Based Coverage Verification

When a requirements file is provided, the tool performs the following verification:

1. **Validation**: Each `covers` string in test cases must be a substring of the requirement text
2. **Cumulative Coverage**: The tool combines all `covers` strings from all test cases for each requirement
3. **Full vs Partial**: If the cumulative coverage accounts for all text in the requirement, it's marked as "full" coverage; otherwise "partial"
4. **Error Detection**: If a test case claims to cover text not found in the requirement, an error is reported

**Example Test Cases:**

```yaml
# Test case 1
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "authenticate users with valid credentials"

# Test case 2
requirement: REQ-001
requirement_coverage:
  type: partial
  covers: "deny access to users with invalid credentials"
```

With the requirement text: "The system shall authenticate users with valid credentials and deny access to users with invalid credentials."

- Both test cases' `covers` strings are validated to exist in the requirement
- The cumulative coverage checks if all parts of the requirement text are covered
- If "The system shall" and "and" are not covered by any test case, the requirement is marked as "partial"
- If all text is covered (accounting for all words), it's marked as "full"

### Coverage Report Details

When using requirement definitions, the coverage report includes:

- **Requirement Text**: The full text of the requirement
- **Covered Portions**: List of all `covers` strings from test cases
- **Coverage Errors**: Any errors where test cases claim to cover text not in the requirement
- **Automatic Type Determination**: Coverage type is automatically determined based on cumulative coverage

## Output Formats

### JSON Report Structure

```json
{
  "generated_at": "2024-01-15T10:30:00Z",
  "total_requirements": 10,
  "fully_covered_requirements": 7,
  "partially_covered_requirements": 2,
  "uncovered_requirements": 1,
  "requirements": [
    {
      "requirement_id": "REQ-001",
      "coverage_type": "full",
      "test_cases": [
        {
          "test_case_id": "TC-001",
          "status": "pass",
          "covers": null,
          "description": "Test login with valid credentials"
        }
      ],
      "status": "covered_pass"
    }
  ]
}
```

### Coverage Status Values

- `covered_pass`: Full coverage, all tests passed
- `covered_fail`: Full coverage, some tests failed
- `partial_covered_pass`: Partial coverage, all tests passed
- `partial_covered_fail`: Partial coverage, some tests failed
- `uncovered`: No test coverage

### Test Status Values

- `pass`: Test passed verification
- `fail`: Test failed verification
- `not_executed`: Test has no verification result

## HTML Report Features

The generated HTML report includes:

- **Dashboard**: Overview statistics with total requirements, coverage breakdown
- **Requirements Table**: Detailed table with:
  - Requirement ID
  - Coverage type (Full/Partial)
  - Status badge with color coding
  - Test case count
  - Pass/Fail ratio
- **Interactive Details**: Click requirement rows to expand and see:
  - Test case IDs and descriptions
  - Coverage details (for partial coverage)
  - Individual test status
- **Visual Indicators**: Color-coded status badges:
  - 🟢 Green: All tests passed
  - 🔴 Red: Some tests failed
  - 🟡 Yellow: Partial coverage, all passed
  - 🟠 Orange: Partial coverage, some failed
  - ⚪ Gray: No coverage

## Logging

Control logging verbosity:

```bash
# Info level (default)
req-coverage verify --test-cases-folder ./testcases --test-results-folder ./results --output report.json

# Debug level
req-coverage verify --verbose --test-cases-folder ./testcases --test-results-folder ./results --output report.json

# Custom log level
req-coverage verify --log-level debug --test-cases-folder ./testcases --test-results-folder ./results --output report.json

# Environment variable
RUST_LOG=debug req-coverage verify --test-cases-folder ./testcases --test-results-folder ./results --output report.json
```

## Examples

### Complete Workflow

```bash
# 1. Generate coverage report JSON
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-acceptance/verification_results \
  --output coverage.json

# 2. Generate HTML report
req-coverage print \
  --format html \
  --input coverage.json \
  --output ./coverage-html

# 3. View in browser
open ./coverage-html/index.html
```

### CI/CD Integration

```yaml
# GitLab CI example
coverage-report:
  stage: report
  script:
    - cargo build --release -p req-coverage
    - ./target/release/req-coverage verify --test-cases-folder testcases --test-results-folder verification_results --output coverage.json
    - ./target/release/req-coverage print --format html --input coverage.json --output public/coverage
  artifacts:
    paths:
      - public/coverage
      - coverage.json
    expire_in: 30 days
```

## Architecture

The `req-coverage` tool is a **Layer 5 binary crate** in the workspace architecture:

**Dependencies:**
- `testcase-models` (Layer 1): Core data structures
- `testcase-common` (Layer 2): Shared utilities
- `testcase-storage` (Layer 3): Test case loading and storage

**Modules:**
- `main.rs`: CLI entry point and command handling
- `models.rs`: Coverage data structures
- `coverage.rs`: Coverage analysis logic
- `report.rs`: Report loading and saving
- `html.rs`: HTML report generation

## Testing

### Unit Tests

Run unit tests for the library:
```bash
cargo test -p req-coverage --lib
```

### Integration Tests

Shell script-based integration tests are available in `integration-tests/`:
```bash
cd integration-tests
./run_integration_tests.sh
```

The integration tests validate:
- Full and partial coverage detection
- Error detection and reporting
- Multiple requirements handling
- YAML and JSON format support
- Backward compatibility
- HTML report generation

See [integration-tests/README.md](integration-tests/README.md) for details.

## Troubleshooting

### No test cases found

**Error**: "Loaded 0 test cases"

**Solution**: Ensure test case YAML files have `.yaml` or `.yml` extension and are valid test case format.

### No verification results found

**Error**: "Loaded 0 verification results"

**Solution**: Ensure verification results are in container format with `_container.yaml` or `_container.yml` suffix.

### Missing requirement field

**Error**: "Failed to parse YAML from..."

**Solution**: Ensure all test case YAML files have a `requirement` field.

## Testing

### Running Tests

The project includes comprehensive unit and integration tests.

**Quick Start - Run all tests with results saved:**
```bash
cd crates/req-coverage
./run_integration_tests.sh
```

This script runs all tests and saves timestamped results to `test_results/`.

**Manual test commands:**
```bash
# Run all tests
cargo test -p req-coverage

# Run unit tests only
cargo test -p req-coverage --lib

# Run integration tests only
cargo test -p req-coverage --test string_verification_tests
```

**View test results:**
```bash
cat crates/req-coverage/test_results/latest_results.txt
```

See [INTEGRATION_TESTS.md](INTEGRATION_TESTS.md) for detailed test documentation.

## Future Enhancements

See [PRD_REQ_COVERAGE.md](../../docs/PRD_REQ_COVERAGE.md) for planned features:
- Requirement import from external systems
- Trend analysis over time
- PDF export
- Advanced filtering
- REST API for programmatic access
