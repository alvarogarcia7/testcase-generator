# Test-Verify Workflow Guide

This guide demonstrates a complete workflow for using test-verify in a real-world testing scenario.

## Workflow Overview

```
1. Create Test Cases (tcm)
2. Execute Tests → Generate Logs
3. Verify Results (test-verify)
4. Review Reports
5. CI/CD Integration
```

## Step-by-Step Workflow

### Step 1: Create Test Cases

Use the the tools (tcm) to create test case definitions:

```bash
# Interactive test case creation
tcm create-interactive

# Or use the complete workflow
tcm complete
```

Example test case structure:
```yaml
requirement: GSMA-4.4.2.2
item: 4
tc: 2
id: ProfileDownload_TC
description: Profile Download and Activation
test_sequences:
  - id: 1
    name: Standard Download
    steps:
      - step: 1
        description: Download profile
        command: ssh esim download
        expected:
          success: true
          result: SW=0x9000
          output: Profile downloaded successfully
```

### Step 2: Execute Tests

Run your actual tests and generate execution logs in the required format:

```bash
# Your test execution script should output logs like:
[2024-01-15T10:00:00Z] TestCase: ProfileDownload_TC, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Profile downloaded successfully
```

**Example test runner script** (`run_tests.sh`):

```bash
#!/bin/bash
# run_tests.sh - Execute tests and generate logs

LOG_DIR="test-logs"
mkdir -p "$LOG_DIR"

# Run test and capture results
run_test() {
    local TEST_CASE_ID=$1
    local LOG_FILE="$LOG_DIR/${TEST_CASE_ID}_$(date +%Y%m%d_%H%M%S).log"
    
    echo "Running test: $TEST_CASE_ID"
    echo "# Test execution for $TEST_CASE_ID" > "$LOG_FILE"
    echo "# Started: $(date -Iseconds)" >> "$LOG_FILE"
    
    # Execute your actual test here and format output
    # Example: your_test_tool --test "$TEST_CASE_ID" | format_as_log >> "$LOG_FILE"
    
    echo "Log written to: $LOG_FILE"
}

# Run all tests
run_test "ProfileDownload_TC"
run_test "ProfileManagement_TC"
```

### Step 3: Verify Test Results

Use test-verify to compare execution logs against test case definitions:

#### Single Test Verification

```bash
# Verify one test with text output
test-verify single \
  --log test-logs/ProfileDownload_TC_20240115.log \
  --test-case-id ProfileDownload_TC

# Or with JSON output
test-verify single \
  --log test-logs/ProfileDownload_TC_20240115.log \
  --test-case-id ProfileDownload_TC \
  --format json > results.json
```

#### Batch Verification

```bash
# Verify all logs with text report
test-verify batch \
  --logs test-logs/*.log \
  --test-case-dir testcases

# Generate JUnit XML for CI/CD
test-verify batch \
  --logs test-logs/*.log \
  --test-case-dir testcases \
  --format junit \
  --output junit-report.xml

# Generate JSON report
test-verify batch \
  --logs test-logs/*.log \
  --test-case-dir testcases \
  --format json \
  --output verification-report.json
```

### Step 4: Review Reports

#### Text Report

The text report shows a summary and detailed results:

```
═══════════════════════════════════════════════════════════
           BATCH VERIFICATION REPORT
═══════════════════════════════════════════════════════════
Generated: 2024-01-15T10:30:00Z

SUMMARY:
───────────────────────────────────────────────────────────
Test Cases:  3 total
             2 passed (66%)
             1 failed

Steps:       12 total
             10 passed (83%)
             2 failed
             0 not executed

TEST CASE RESULTS:
═══════════════════════════════════════════════════════════

✓ PASS ProfileDownload_TC
  Description: Profile Download and Activation
  Steps: 2/2 passed

✗ FAIL ProfileManagement_TC
  Description: Profile Management Operations
  Steps: 3/5 passed, 2 failed
  └─ Sequence #1: List and Switch
     ✗ Step 2: Switch to profile 2 - Result mismatch: expected 'SW=0x9000', got 'SW=0x6A82'
```

#### JSON Report

The JSON report provides machine-readable data:

```json
{
  "test_cases": [
    {
      "test_case_id": "ProfileDownload_TC",
      "description": "Profile Download and Activation",
      "overall_pass": true,
      "total_steps": 2,
      "passed_steps": 2,
      "failed_steps": 0
    }
  ],
  "total_test_cases": 3,
  "passed_test_cases": 2,
  "failed_test_cases": 1,
  "total_steps": 12,
  "passed_steps": 10,
  "failed_steps": 2
}
```

#### JUnit XML Report

The JUnit XML is compatible with CI/CD systems:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="Batch Test Verification" tests="12" failures="2" errors="0">
  <testcase name="ProfileDownload_TC.seq1.step1" classname="ProfileDownload_TC.Standard Download" time="0.000"/>
  <testcase name="ProfileDownload_TC.seq1.step2" classname="ProfileDownload_TC.Standard Download" time="0.000"/>
  <testcase name="ProfileManagement_TC.seq1.step2" classname="ProfileManagement_TC.List and Switch" time="0.000">
    <failure message="Result mismatch" type="VerificationFailure">
Expected: SW=0x9000
Got: SW=0x6A82
    </failure>
  </testcase>
</testsuite>
```

### Step 5: CI/CD Integration

#### GitHub Actions

Complete workflow file (`.github/workflows/test-verify.yml`):

```yaml
name: Test Verification

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test-and-verify:
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Build test-verify
        run: cargo build --release --bin test-verify
      
      - name: Run tests
        run: |
          # Your test execution script
          ./scripts/run_tests.sh
      
      - name: Verify test results
        run: |
          ./target/release/test-verify batch \
            --logs test-logs/*.log \
            --test-case-dir testcases \
            --format junit \
            --output junit-report.xml
      
      - name: Publish Test Results
        uses: EnricoMi/publish-unit-test-result-action@v2
        if: always()
        with:
          files: junit-report.xml
      
      - name: Upload reports as artifacts
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: test-reports
          path: |
            junit-report.xml
            test-logs/
```

#### Jenkins Pipeline

```groovy
pipeline {
    agent any
    
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release --bin test-verify'
            }
        }
        
        stage('Run Tests') {
            steps {
                sh './scripts/run_tests.sh'
            }
        }
        
        stage('Verify Results') {
            steps {
                sh '''
                    ./target/release/test-verify batch \
                        --logs test-logs/*.log \
                        --test-case-dir testcases \
                        --format junit \
                        --output junit-report.xml
                '''
            }
        }
    }
    
    post {
        always {
            junit 'junit-report.xml'
            archiveArtifacts artifacts: 'test-logs/*.log, junit-report.xml', fingerprint: true
        }
        failure {
            emailext(
                subject: "Test Verification Failed: ${env.JOB_NAME}",
                body: "Test verification failed. Check the reports for details.",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
```

#### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - build
  - test
  - verify

variables:
  CARGO_HOME: "${CI_PROJECT_DIR}/.cargo"

build:
  stage: build
  script:
    - cargo build --release --bin test-verify
  artifacts:
    paths:
      - target/release/test-verify
    expire_in: 1 hour

run_tests:
  stage: test
  script:
    - ./scripts/run_tests.sh
  artifacts:
    paths:
      - test-logs/
    expire_in: 1 week

verify_results:
  stage: verify
  dependencies:
    - build
    - run_tests
  script:
    - |
      ./target/release/test-verify batch \
        --logs test-logs/*.log \
        --test-case-dir testcases \
        --format junit \
        --output junit-report.xml
  artifacts:
    when: always
    reports:
      junit: junit-report.xml
    paths:
      - junit-report.xml
```

## Advanced Scenarios

### Parallel Test Execution

Run tests in parallel and combine logs:

```bash
#!/bin/bash
# parallel_test_runner.sh

TESTS=(
    "ProfileDownload_TC"
    "ProfileManagement_TC"
    "ProfileDelete_TC"
)

LOG_DIR="test-logs/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$LOG_DIR"

# Run tests in parallel
for test in "${TESTS[@]}"; do
    (
        echo "Running $test..."
        run_test "$test" > "$LOG_DIR/${test}.log" 2>&1
    ) &
done

# Wait for all tests to complete
wait

# Verify all results
test-verify batch \
    --logs "$LOG_DIR"/*.log \
    --format junit \
    --output "$LOG_DIR/junit-report.xml"
```

### Selective Test Verification

Verify only specific test cases:

```bash
# Verify only failed tests from previous run
grep "Success: false" previous-run.log | \
    cut -d',' -f1 | \
    cut -d':' -f2 | \
    sort -u | \
    while read test_id; do
        test-verify single \
            --log "test-logs/${test_id}.log" \
            --test-case-id "$test_id"
    done
```

### Continuous Monitoring

Monitor test execution and verify in real-time:

```bash
#!/bin/bash
# monitor_and_verify.sh

WATCH_DIR="test-logs"
TEST_CASE_DIR="testcases"

# Monitor directory for new log files
inotifywait -m -e close_write --format '%f' "$WATCH_DIR" | \
while read filename; do
    if [[ "$filename" == *.log ]]; then
        echo "New log file detected: $filename"
        
        # Extract test case ID from filename
        test_id="${filename%.log}"
        
        # Verify immediately
        test-verify single \
            --log "$WATCH_DIR/$filename" \
            --test-case-id "$test_id" \
            --format text
    fi
done
```

### Report Aggregation

Combine multiple verification runs:

```bash
#!/bin/bash
# aggregate_reports.sh

REPORT_DIR="reports"
mkdir -p "$REPORT_DIR"

# Generate individual reports
for run_dir in test-runs/*/; do
    run_name=$(basename "$run_dir")
    
    test-verify batch \
        --logs "$run_dir"/*.log \
        --format json \
        --output "$REPORT_DIR/${run_name}_report.json"
done

# Combine reports (requires custom script)
./scripts/combine_reports.py "$REPORT_DIR"/*.json > "$REPORT_DIR/combined_report.json"
```

## Best Practices

1. **Standardize Log Format**: Ensure all test runners output logs in the correct format
2. **Use Timestamps**: Include timestamps for chronological tracking
3. **Organize Logs**: Structure log files by date, test run, or test suite
4. **Archive Reports**: Keep verification reports alongside test logs
5. **Version Control**: Store test cases in version control
6. **Automate**: Integrate verification into your CI/CD pipeline
7. **Monitor Trends**: Track pass/fail rates over time
8. **Act on Failures**: Set up alerts for test failures

## Troubleshooting

### Log Not Parsed

**Problem**: Log entries are not being recognized

**Solution**: 
- Check log format matches exactly
- Ensure proper comma and space separators
- Verify timestamp format if included
- Use `parse-log` command to debug

### Test Case Not Found

**Problem**: "Failed to load test case" error

**Solution**:
- Verify test case ID matches filename
- Check test case directory path
- Ensure .yaml or .yml extension
- Use correct test case ID from log

### Verification Failures

**Problem**: Tests failing verification unexpectedly

**Solution**:
- Review expected vs actual values in report
- Consider using wildcards for variable data
- Use regex for pattern matching
- Check success field expectations

## Summary

The test-verify workflow provides:
1. Integration with test case management (tcm)
2. Flexible test execution with log generation
3. Comprehensive verification and reporting
4. Seamless CI/CD integration
5. Multiple output formats for different needs

For more information:
- Usage guide: `docs/TEST_VERIFY_USAGE.md`
- Quick reference: `docs/TEST_VERIFY_QUICK_REFERENCE.md`
- Examples: `examples/test_verify_*.rs`
