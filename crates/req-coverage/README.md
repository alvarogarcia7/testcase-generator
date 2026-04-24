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

## Future Enhancements

See [PRD_REQ_COVERAGE.md](../../docs/PRD_REQ_COVERAGE.md) for planned features:
- Requirement import from external systems
- Trend analysis over time
- PDF export
- Advanced filtering
- REST API for programmatic access
