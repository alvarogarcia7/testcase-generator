# JUnit XML Export

This document describes the JUnit XML export functionality for test run results.

## Overview

The testcase-manager now supports exporting test run results to JUnit XML format, which is widely supported by CI/CD systems like Jenkins, GitLab CI, GitHub Actions, and many testing frameworks.

## Data Structures

### TestRunStatus

An enum representing the execution status of a test:
- `Pass`: Test passed successfully
- `Fail`: Test failed
- `Skip`: Test was skipped

### TestRun

A struct containing test execution information:
- `test_case_id`: String - Unique identifier for the test case
- `status`: TestRunStatus - Execution status
- `timestamp`: DateTime<Utc> - When the test was executed
- `duration`: f64 - Execution time in seconds
- `error_message`: Option<String> - Optional error message for failed/skipped tests
- `name`: Option<String> - Optional human-readable test name

## JUnit XML Format

The `TestRun` struct provides a `to_junit_xml()` method that generates JUnit XML:

```rust
let test_run = TestRun::new(
    "TC001".to_string(),
    TestRunStatus::Pass,
    Utc::now(),
    1.234,
);

let xml = test_run.to_junit_xml();
```

### XML Structure

The generated XML follows the JUnit XML schema:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="TestRun" tests="1" failures="0" skipped="0" time="1.234">
  <testcase id="TC001" name="TC001" time="1.234" timestamp="2024-01-15T10:30:00+00:00"/>
</testsuite>
```

#### For Failed Tests

```xml
<testcase id="TC002" name="Database Test" time="2.567" timestamp="2024-01-15T10:31:30+00:00">
  <failure message="Test failed">Assertion failed: expected true, got false</failure>
</testcase>
```

#### For Skipped Tests

```xml
<testcase id="TC003" name="TC003" time="0.000" timestamp="2024-01-15T10:32:00+00:00">
  <skipped message="Test skipped due to missing dependencies"/>
</testcase>
```

## CLI Usage

### Export Command

```bash
tcm export-junit-xml <input_file> [--output <output_file>]
```

#### Arguments

- `input`: Path to a JSON or YAML file containing test run data (required)
- `--output` or `-o`: Output file path, use '-' for stdout (default: stdout)

### Validate Command

```bash
tcm validate-junit-xml <xml_file>
```

Validates a JUnit XML file against the Maven Surefire XSD schema.

#### Arguments

- `xml_file`: Path to the JUnit XML file to validate (required)

#### Examples

**Export to stdout:**
```bash
tcm export-junit-xml data/sample_test_runs.json
```

**Export to file:**
```bash
tcm export-junit-xml data/sample_test_runs.json --output results.xml
```

**Export from YAML:**
```bash
tcm export-junit-xml test_runs.yaml -o junit-results.xml
```

**Validate an existing XML file:**
```bash
tcm validate-junit-xml junit-results.xml
```

### Input File Format

The input file should contain an array of TestRun objects in JSON or YAML format.

#### JSON Example

```json
[
  {
    "test_case_id": "TC001",
    "status": "Pass",
    "timestamp": "2024-01-15T10:30:00Z",
    "duration": 1.234,
    "name": "User Authentication Test"
  },
  {
    "test_case_id": "TC002",
    "status": "Fail",
    "timestamp": "2024-01-15T10:31:30Z",
    "duration": 2.567,
    "error_message": "Assertion failed: expected true, got false",
    "name": "Database Connection Test"
  },
  {
    "test_case_id": "TC003",
    "status": "Skip",
    "timestamp": "2024-01-15T10:32:00Z",
    "duration": 0.0,
    "error_message": "Test skipped due to missing dependencies"
  }
]
```

#### YAML Example

```yaml
- test_case_id: TC001
  status: Pass
  timestamp: "2024-01-15T10:30:00Z"
  duration: 1.234
  name: User Authentication Test

- test_case_id: TC002
  status: Fail
  timestamp: "2024-01-15T10:31:30Z"
  duration: 2.567
  error_message: "Assertion failed: expected true, got false"
  name: Database Connection Test

- test_case_id: TC003
  status: Skip
  timestamp: "2024-01-15T10:32:00Z"
  duration: 0.0
  error_message: Test skipped due to missing dependencies
```

## Integration with CI/CD

### Jenkins

```groovy
pipeline {
    stages {
        stage('Test') {
            steps {
                sh 'tcm export-junit-xml test_runs.json -o results.xml'
            }
            post {
                always {
                    junit 'results.xml'
                }
            }
        }
    }
}
```

### GitLab CI

```yaml
test:
  script:
    - tcm export-junit-xml test_runs.json -o results.xml
  artifacts:
    reports:
      junit: results.xml
```

### GitHub Actions

```yaml
- name: Generate JUnit Report
  run: tcm export-junit-xml test_runs.json -o results.xml

- name: Publish Test Results
  uses: EnricoMi/publish-unit-test-result-action@v2
  if: always()
  with:
    files: results.xml
```

## Edge Cases

The implementation handles several edge cases:

1. **Zero Duration**: Tests with 0.0 duration are formatted as "0.000"
2. **Missing Error Messages**: Failed tests without error messages get an empty failure element
3. **Missing Test Names**: If no name is provided, the test_case_id is used as the name
4. **Special Characters**: XML special characters in error messages are properly escaped

## API Usage

```rust
use testcase_manager::{TestRun, TestRunStatus};
use chrono::Utc;

// Create a passing test run
let pass = TestRun::new(
    "TC001".to_string(),
    TestRunStatus::Pass,
    Utc::now(),
    1.5,
);

// Create a failing test run with error message
let fail = TestRun::with_error(
    "TC002".to_string(),
    TestRunStatus::Fail,
    Utc::now(),
    2.0,
    "Test failed".to_string(),
);

// Generate XML
let xml = pass.to_junit_xml();
```

## Testing

Unit tests are provided in:
- `src/models.rs`: Core functionality tests
- `tests/junit_export_test.rs`: Integration tests with XSD validation

All generated JUnit XML is validated against the Maven Surefire XSD schema:
**https://maven.apache.org/surefire/maven-surefire-plugin/xsd/surefire-test-report.xsd**

For detailed information about XSD validation, see [JUNIT_XML_XSD_VALIDATION.md](JUNIT_XML_XSD_VALIDATION.md).

Run tests with:
```bash
cargo test
cargo test test_junit_xml_xsd_compliance  # Run XSD validation specifically
```
