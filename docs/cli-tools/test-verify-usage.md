# Test-Verify Binary Usage Guide

The `test-verify` binary is a comprehensive test verification tool that processes test execution logs and compares them against test case definitions to generate verification reports.

## Features

- **Single Test Verification**: Verify a single test execution log against a specific test case
- **Batch Verification**: Process multiple test execution logs simultaneously
- **Auto-locate Test Cases**: Uses TestCaseStorage to automatically find test case definitions
- **Multiple Output Formats**: Supports text, JSON, and JUnit XML formats
- **Aggregated Statistics**: Generates pass/fail statistics per test case and overall
- **CI/CD Integration**: JUnit XML output for seamless CI/CD pipeline integration
- **Flexible Matching**: Supports exact matching, wildcards, and regex patterns

## Installation

Build the binary:

```bash
cargo build --release --bin test-verify
```

The binary will be available at `target/release/test-verify`.

## Test Execution Log Format

Test execution logs should follow this format:

```
[TIMESTAMP] TestCase: <id>, Sequence: <seq_id>, Step: <step_num>, Success: <true/false/null/->, Result: <result>, Output: <output>
```

Example:
```
[2024-01-15T10:30:00Z] TestCase: TC001, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Command executed successfully
[2024-01-15T10:30:05Z] TestCase: TC001, Sequence: 1, Step: 2, Success: true, Result: OK, Output: Test completed
```

Fields:
- **TIMESTAMP**: Optional ISO 8601 timestamp (e.g., `2024-01-15T10:30:00Z`)
- **TestCase**: Test case ID (must match test case file)
- **Sequence**: Test sequence ID (numeric)
- **Step**: Step number (numeric)
- **Success**: Boolean or null (`true`, `false`, `null`, `none`, `-`)
- **Result**: Actual result value
- **Output**: Actual output value

## Commands

### Single Test Verification

Verify a single test execution log against a specific test case:

```bash
test-verify single --log <log_file> --test-case-id <test_case_id> [OPTIONS]
```

**Options:**
- `-l, --log <LOG>`: Path to test execution log file (required)
- `-t, --test-case-id <ID>`: Test case ID to verify against (required)
- `-d, --test-case-dir <DIR>`: Path to test case storage directory (default: `testcases`)
- `-f, --format <FORMAT>`: Output format: `text`, `json`, or `junit` (default: `text`)

**Examples:**

```bash
# Verify with text output
test-verify single --log execution.log --test-case-id TC001

# Verify with JSON output
test-verify single --log execution.log --test-case-id TC001 --format json

# Verify with JUnit XML output
test-verify single --log execution.log --test-case-id TC001 --format junit > results.xml

# Specify custom test case directory
test-verify single --log execution.log --test-case-id TC001 --test-case-dir data/testcases
```

### Batch Verification

Process multiple test execution logs and generate aggregated reports:

```bash
test-verify batch --logs <log_file1> <log_file2> ... [OPTIONS]
```

**Options:**
- `-l, --logs <LOGS>...`: Path(s) to test execution log file(s) (required, can specify multiple)
- `-d, --test-case-dir <DIR>`: Path to test case storage directory (default: `testcases`)
- `-f, --format <FORMAT>`: Output format: `text`, `json`, or `junit` (default: `text`)
- `-o, --output <FILE>`: Output file path (optional, defaults to stdout)

**Examples:**

```bash
# Batch verify multiple logs with text output
test-verify batch --logs log1.log log2.log log3.log

# Batch verify with JSON output to file
test-verify batch --logs logs/*.log --format json --output report.json

# Batch verify with JUnit XML output for CI/CD
test-verify batch --logs logs/*.log --format junit --output junit-report.xml

# Batch verify with custom test case directory
test-verify batch --logs logs/*.log --test-case-dir data/testcases
```

### Parse Log

Parse and display test execution log contents without verification:

```bash
test-verify parse-log --log <log_file> [OPTIONS]
```

**Options:**
- `-l, --log <LOG>`: Path to test execution log file (required)
- `-f, --format <FORMAT>`: Output format: `text` or `json` (default: `text`)

**Examples:**

```bash
# Parse log with text output
test-verify parse-log --log execution.log

# Parse log with JSON output
test-verify parse-log --log execution.log --format json
```

## Output Formats

### Text Format

Human-readable format with detailed step-by-step results:

```
═══════════════════════════════════════════════════════════
           BATCH VERIFICATION REPORT
═══════════════════════════════════════════════════════════
Generated: 2024-01-15T10:30:00Z

SUMMARY:
───────────────────────────────────────────────────────────
Test Cases:  2 total
             2 passed (100%)
             0 failed

Steps:       6 total
             6 passed (100%)
             0 failed
             0 not executed
```

### JSON Format

Machine-readable JSON format with complete details:

```json
{
  "test_cases": [
    {
      "test_case_id": "TC001",
      "description": "Test Case 1",
      "sequences": [...],
      "total_steps": 2,
      "passed_steps": 2,
      "failed_steps": 0,
      "not_executed_steps": 0,
      "overall_pass": true
    }
  ],
  "total_test_cases": 1,
  "passed_test_cases": 1,
  "failed_test_cases": 0,
  "total_steps": 2,
  "passed_steps": 2,
  "failed_steps": 0,
  "not_executed_steps": 0,
  "generated_at": "2024-01-15T10:30:00Z"
}
```

### JUnit XML Format

Standard JUnit XML format for CI/CD integration:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="Batch Test Verification" tests="2" failures="0" errors="0" skipped="0" time="0.000" timestamp="2024-01-15T10:30:00Z">
  <testcase name="TC001.seq1.step1 - Initialize test" classname="TC001.Sequence 1" time="0.000"/>
  <testcase name="TC001.seq1.step2 - Execute command" classname="TC001.Sequence 1" time="0.000"/>
</testsuite>
```

## Verification Logic

### Step Verification

Each step is verified by comparing:

1. **Success Field**: If defined in expected results and in log, must match
2. **Result Field**: Must match expected result (supports wildcards and regex)
3. **Output Field**: Must match expected output (supports wildcards and regex)

### Matching Rules

**Exact Match:**
```
Expected: "SW=0x9000"
Actual: "SW=0x9000"
Result: ✓ PASS
```

**Wildcard Match:**
```
Expected: "SW=*"
Actual: "SW=0x9000"
Result: ✓ PASS
```

**Regex Match:**
```
Expected: "/SW=0x[0-9A-F]{4}/"
Actual: "SW=0x9000"
Result: ✓ PASS
```

### Test Case Status

A test case passes if:
- All steps have matching execution logs
- All steps pass verification
- No steps failed or were not executed

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Test Verification

on: [push, pull_request]

jobs:
  verify-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build test-verify
        run: cargo build --release --bin test-verify
        
      - name: Run test verification
        run: |
          ./target/release/test-verify batch \
            --logs test-logs/*.log \
            --format junit \
            --output junit-report.xml
            
      - name: Publish Test Results
        uses: EnricoMi/publish-unit-test-result-action@v2
        if: always()
        with:
          files: junit-report.xml
```

### Jenkins Example

```groovy
pipeline {
    agent any
    
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release --bin test-verify'
            }
        }
        
        stage('Verify Tests') {
            steps {
                sh '''
                    ./target/release/test-verify batch \
                        --logs test-logs/*.log \
                        --format junit \
                        --output junit-report.xml
                '''
            }
        }
    }
    
    post {
        always {
            junit 'junit-report.xml'
        }
    }
}
```

## Exit Codes

- **0**: All tests passed
- **1**: One or more tests failed or errors occurred

## Example Workflow

1. **Generate Test Execution Logs**: Run your tests and generate logs in the required format

2. **Verify Single Test**:
   ```bash
   test-verify single --log my_test.log --test-case-id TC001
   ```

3. **Batch Verify Multiple Tests**:
   ```bash
   test-verify batch --logs logs/*.log --format json --output report.json
   ```

4. **Generate JUnit XML for CI/CD**:
   ```bash
   test-verify batch --logs logs/*.log --format junit --output junit-report.xml
   ```

5. **Review Results**: Examine the report to identify failed tests and steps

## Troubleshooting

### Test Case Not Found

If you see "Failed to load test case", ensure:
- The test case file exists in the test case directory
- The test case ID in the log matches the filename (without extension)
- The test case directory path is correct

### Log Parsing Errors

If logs aren't being parsed:
- Check the log format matches the expected format exactly
- Ensure field separators are correct (comma and space)
- Verify timestamp format if included

### No Steps Verified

If steps show as "NOT EXECUTED":
- Verify sequence IDs and step numbers match between log and test case
- Check test case ID is correct in log entries
- Ensure the test case definition contains the expected sequences and steps

## See Also

- [Test Case Manager Documentation](../index.md)
- [Test Case Format Documentation](../user-guide/test-case-format.md)
- [Example Test Execution Logs](../data/example_test_execution.log)
