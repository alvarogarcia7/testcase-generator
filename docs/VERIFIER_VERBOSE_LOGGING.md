# Verifier Verbose Logging

## Overview

The verifier supports verbose logging mode through the `--verbose` flag, which enables debug-level output for detailed verification information. When enabled, verbose mode provides:

- **File Discovery Details**: Shows which test case and log files are found or skipped during folder scanning
- **Test Case Loading**: Displays information about loading and parsing YAML test case files
- **Step-by-Step Verification**: Shows detailed comparisons between expected values and actual command output for each verification step
- **Log Parsing Information**: Indicates whether JSON or text format logs are being parsed and any parsing issues encountered
- **Matching Logic Details**: Reveals the internal decision-making process when comparing expected results, output patterns, and success conditions
- **Report Generation**: Shows how verification results are aggregated and formatted into the final report
- **Batch Processing**: Displays progress through multiple test cases when processing folders

Verbose mode is invaluable for troubleshooting verification failures, understanding why certain tests pass or fail, and debugging test case definitions.

## Usage

### Single File Mode

Verify a single test case with verbose output:

```bash
cargo run --bin verifier -- \
  --test-case testcases/example/test.yaml \
  --log-file testcases/example/execution.log \
  --verbose
```

**Example Verbose Output:**

```
DEBUG: Loading test case from: testcases/example/test.yaml
DEBUG: Test case loaded successfully: "Example Test Case"
DEBUG: Parsing log file: testcases/example/execution.log
DEBUG: Detected JSON format log file
DEBUG: Found 5 log entries in file
DEBUG: Starting verification for sequence 1: "Basic Operations"
DEBUG: Verifying step 1.1: "Create test directory"
DEBUG:   Expected result: 0
DEBUG:   Actual exit code: 0
DEBUG:   Result match: ✓
DEBUG:   Expected output pattern: "Created directory: /tmp/test"
DEBUG:   Actual output: "Created directory: /tmp/test_12345"
DEBUG:   Output match: ✓
DEBUG: Step 1.1 verification: PASSED
DEBUG: Verifying step 1.2: "List directory contents"
DEBUG:   Expected success: true
DEBUG:   Actual exit code: 0
DEBUG:   Success check: ✓
DEBUG: Step 1.2 verification: PASSED
DEBUG: Sequence 1 verification: PASSED (2/2 steps passed)
DEBUG: Generating verification report
INFO: Verification Result: PASSED
INFO: Total Sequences: 1 | Passed: 1 | Failed: 0
INFO: Total Steps: 2 | Passed: 2 | Failed: 0
```

### Folder Mode

Verify all test cases in a folder with verbose output:

```bash
cargo run --bin verifier -- \
  --folder testcases/ \
  --verbose
```

**Example Verbose Output:**

```
DEBUG: Scanning folder for test cases: testcases/
DEBUG: Found test case file: testcases/basic/test.yaml
DEBUG: Found test case file: testcases/advanced/test.yaml
DEBUG: Skipping non-YAML file: testcases/README.md
DEBUG: Total test case files discovered: 2
DEBUG: Processing test case 1/2: testcases/basic/test.yaml
DEBUG: Loading test case from: testcases/basic/test.yaml
DEBUG: Test case loaded successfully: "Basic Test"
DEBUG: Looking for log file: testcases/basic/execution.log
DEBUG: Log file found: testcases/basic/execution.log
DEBUG: Parsing log file: testcases/basic/execution.log
DEBUG: Detected text format log file
DEBUG: Extracted 3 step results from log
DEBUG: Starting verification for sequence 1: "Simple Commands"
DEBUG: Verifying step 1.1: "Echo test"
DEBUG:   Expected result: 0
DEBUG:   Actual exit code: 0
DEBUG:   Result match: ✓
DEBUG: Step 1.1 verification: PASSED
DEBUG: Sequence 1 verification: PASSED (1/1 steps passed)
DEBUG: Test case testcases/basic/test.yaml: PASSED
DEBUG: Processing test case 2/2: testcases/advanced/test.yaml
DEBUG: Loading test case from: testcases/advanced/test.yaml
DEBUG: Test case loaded successfully: "Advanced Test"
DEBUG: Looking for log file: testcases/advanced/execution.log
DEBUG: Log file found: testcases/advanced/execution.log
DEBUG: Parsing log file: testcases/advanced/execution.log
DEBUG: Detected JSON format log file
DEBUG: Found 10 log entries in file
DEBUG: Starting verification for sequence 1: "Complex Operations"
DEBUG: Verifying step 1.1: "Database query"
DEBUG:   Expected output pattern: "Records found: [0-9]+"
DEBUG:   Actual output: "Records found: 42"
DEBUG:   Output match: ✓
DEBUG: Step 1.1 verification: PASSED
DEBUG: Sequence 1 verification: PASSED (1/1 steps passed)
DEBUG: Test case testcases/advanced/test.yaml: PASSED
DEBUG: Aggregating batch report for 2 test cases
INFO: 
=== Batch Verification Report ===
Total Test Cases: 2
Passed: 2
Failed: 0
Success Rate: 100.00%
```

### Using Environment Variable

Instead of the `--verbose` flag, you can also use the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run --bin verifier -- \
  --test-case testcases/example/test.yaml \
  --log-file testcases/example/execution.log
```

Note: The `--verbose` flag takes precedence over `RUST_LOG` settings and sets the log level to `debug`.

## Information Logged at Verbose Level

### File Discovery and Loading

```
DEBUG: Scanning folder for test cases: testcases/
DEBUG: Found test case file: testcases/example/test.yaml
DEBUG: Skipping non-YAML file: testcases/README.md
DEBUG: Loading test case from: testcases/example/test.yaml
DEBUG: Test case loaded successfully: "Example Test"
DEBUG: Looking for log file: testcases/example/execution.log
DEBUG: Log file found: testcases/example/execution.log
```

### Log File Parsing

```
DEBUG: Parsing log file: testcases/example/execution.log
DEBUG: Detected JSON format log file
DEBUG: Found 8 log entries in file
```

or for text format logs:

```
DEBUG: Parsing log file: testcases/example/execution.log
DEBUG: Detected text format log file
DEBUG: Extracted 5 step results from log
```

### Step-by-Step Verification

For each verification step, verbose mode shows:

```
DEBUG: Verifying step 1.2: "Check configuration"
DEBUG:   Expected result: 0
DEBUG:   Actual exit code: 0
DEBUG:   Result match: ✓
DEBUG:   Expected output pattern: "config_version: v[0-9]+\.[0-9]+"
DEBUG:   Actual output: "config_version: v1.2.3"
DEBUG:   Output match: ✓
DEBUG:   Expected success: true
DEBUG:   Actual success: true
DEBUG:   Success check: ✓
DEBUG: Step 1.2 verification: PASSED
```

### Verification Results Aggregation

```
DEBUG: Sequence 1 verification: PASSED (3/3 steps passed)
DEBUG: Sequence 2 verification: FAILED (2/3 steps passed)
DEBUG:   Failed step: 2.2 - "Database migration"
DEBUG: Test case verification: FAILED (1/2 sequences passed)
DEBUG: Generating verification report
```

### Batch Processing

```
DEBUG: Processing test case 1/5: testcases/test1/test.yaml
DEBUG: Test case testcases/test1/test.yaml: PASSED
DEBUG: Processing test case 2/5: testcases/test2/test.yaml
DEBUG: Test case testcases/test2/test.yaml: FAILED
DEBUG: Processing test case 3/5: testcases/test3/test.yaml
DEBUG: Test case testcases/test3/test.yaml: PASSED
DEBUG: Aggregating batch report for 5 test cases
```

## Common Troubleshooting Scenarios

### 1. Log Parsing Failures

**Problem**: Verifier cannot parse the execution log file.

**Verbose Output Clues:**

```
DEBUG: Parsing log file: testcases/broken/execution.log
DEBUG: Detected JSON format log file
ERROR: Failed to parse JSON log entry at line 15
DEBUG: Falling back to text format parsing
DEBUG: Extracted 0 step results from log
```

**Resolution**: 
- Check that the log file is in the correct format (JSON lines or text format)
- Verify JSON entries are valid and complete
- Ensure the log file structure matches the expected schema

### 2. Step Matching Issues

**Problem**: Verification fails but you expect it to pass.

**Verbose Output Clues:**

```
DEBUG: Verifying step 1.3: "Validate output"
DEBUG:   Expected output pattern: "Status: SUCCESS"
DEBUG:   Actual output: "Status: Success"
DEBUG:   Output match: ✗ (case mismatch)
DEBUG: Step 1.3 verification: FAILED
```

**Resolution**:
- Check for case sensitivity in pattern matching
- Verify regex patterns are correctly escaped
- Ensure whitespace in patterns matches actual output
- Consider using more flexible regex patterns (e.g., `Status: [Ss]uccess` or `(?i)Status: success`)

### 3. Missing Test Cases

**Problem**: Expected test cases are not being verified in folder mode.

**Verbose Output Clues:**

```
DEBUG: Scanning folder for test cases: testcases/
DEBUG: Found test case file: testcases/test1/test.yaml
DEBUG: Skipping non-YAML file: testcases/test2/test.yml
DEBUG: Found test case file: testcases/test3/test.yaml
DEBUG: Total test case files discovered: 2
```

**Resolution**:
- Ensure test case files have `.yaml` extension (not `.yml`)
- Check that files are in the scanned directory
- Verify file permissions allow reading

### 4. Log File Not Found

**Problem**: Verifier cannot find the execution log for a test case.

**Verbose Output Clues:**

```
DEBUG: Loading test case from: testcases/example/test.yaml
DEBUG: Test case loaded successfully: "Example Test"
DEBUG: Looking for log file: testcases/example/execution.log
ERROR: Log file not found: testcases/example/execution.log
```

**Resolution**:
- Verify the test case has been executed and generated a log file
- Check that the log file is named `execution.log` and located in the same directory as the test case
- Ensure the test execution completed successfully

### 5. Incorrect Verification Logic

**Problem**: Step verification fails despite correct output.

**Verbose Output Clues:**

```
DEBUG: Verifying step 2.1: "Count files"
DEBUG:   Expected result: 0
DEBUG:   Actual exit code: 1
DEBUG:   Result match: ✗
DEBUG:   Command: ls nonexistent_directory | wc -l
DEBUG: Step 2.1 verification: FAILED
```

**Resolution**:
- Review the expected result value in the test case definition
- Verify the command is expected to succeed (exit code 0) or fail (non-zero)
- Check if the verification should use `expected_success: false` instead of `expected_result: 0`

### 6. Regex Pattern Mismatches

**Problem**: Output pattern doesn't match despite looking correct.

**Verbose Output Clues:**

```
DEBUG: Verifying step 1.5: "Extract version"
DEBUG:   Expected output pattern: "version: \d+\.\d+\.\d+"
DEBUG:   Actual output: "version: 1.2.3"
DEBUG:   Output match: ✗ (regex syntax error: invalid escape sequence)
DEBUG: Step 1.5 verification: FAILED
```

**Resolution**:
- Check regex pattern syntax for proper escaping in YAML
- Use raw strings or properly escape backslashes in patterns
- Test regex patterns independently before using in test cases
- Remember that some regex features may differ between implementations

### 7. Sequence Order Issues

**Problem**: Steps are verified out of order or skipped.

**Verbose Output Clues:**

```
DEBUG: Starting verification for sequence 1: "Setup"
DEBUG: Verifying step 1.1: "Create directory"
DEBUG: Step 1.1 verification: PASSED
DEBUG: Verifying step 1.3: "Set permissions"
ERROR: Missing log entry for step 1.2
DEBUG: Step 1.3 verification: FAILED (missing prerequisite)
```

**Resolution**:
- Ensure all steps in the sequence were executed
- Check log file for missing or incomplete entries
- Verify step numbering is sequential in the test case

## Integration with CI/CD Pipelines

### GitHub Actions

```yaml
name: Verify Test Cases

on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run Test Cases
        run: |
          make test
          # Execution logs are generated in testcases/ directories
      
      - name: Verify Test Results (Verbose)
        run: |
          cargo run --bin verifier -- \
            --folder testcases/ \
            --verbose
        env:
          RUST_LOG: debug
      
      - name: Upload Verification Logs
        if: failure()
        uses: actions/upload-artifact@v3
        with:
          name: verification-logs
          path: |
            testcases/**/execution.log
            testcases/**/verification_report.txt
```

### GitLab CI

```yaml
verify-tests:
  stage: test
  image: rust:latest
  script:
    - cargo build --release --bin verifier
    - make test  # Generate execution logs
    - cargo run --bin verifier -- --folder testcases/ --verbose
  variables:
    RUST_LOG: debug
  artifacts:
    when: on_failure
    paths:
      - testcases/**/execution.log
      - testcases/**/verification_report.txt
    expire_in: 1 week
```

### Jenkins

```groovy
pipeline {
    agent any
    
    environment {
        RUST_LOG = 'debug'
    }
    
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release --bin verifier'
            }
        }
        
        stage('Run Tests') {
            steps {
                sh 'make test'
            }
        }
        
        stage('Verify Results') {
            steps {
                sh '''
                    cargo run --bin verifier -- \
                        --folder testcases/ \
                        --verbose 2>&1 | tee verification.log
                '''
            }
        }
    }
    
    post {
        failure {
            archiveArtifacts artifacts: 'testcases/**/execution.log,verification.log', fingerprint: true
        }
    }
}
```

### Docker Environment

For containerized CI/CD environments:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

RUN cargo build --release --bin verifier

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/verifier /usr/local/bin/verifier
COPY testcases/ /testcases/

ENV RUST_LOG=debug

CMD ["verifier", "--folder", "/testcases", "--verbose"]
```

### Environment Variable Control

You can control logging levels through environment variables:

```bash
# Enable debug logging for all components
export RUST_LOG=debug
cargo run --bin verifier -- --folder testcases/

# Enable debug logging only for the verifier
export RUST_LOG=verifier=debug
cargo run --bin verifier -- --folder testcases/

# Enable trace logging for maximum detail
export RUST_LOG=trace
cargo run --bin verifier -- --folder testcases/ --verbose

# Disable verbose logging
unset RUST_LOG
cargo run --bin verifier -- --folder testcases/
```

### Parsing Verbose Output in CI/CD

To extract specific information from verbose logs in automated pipelines:

```bash
# Count failed verifications
cargo run --bin verifier -- --folder testcases/ --verbose 2>&1 | \
  grep -c "verification: FAILED"

# Extract failed test case names
cargo run --bin verifier -- --folder testcases/ --verbose 2>&1 | \
  grep "Test case.*FAILED" | \
  sed -E 's/.*Test case ([^:]+): FAILED.*/\1/'

# Check for specific error patterns
cargo run --bin verifier -- --folder testcases/ --verbose 2>&1 | \
  grep -i "ERROR:" || echo "No errors found"

# Generate summary report
cargo run --bin verifier -- --folder testcases/ --verbose 2>&1 | \
  tee full_log.txt | \
  grep -E "(PASSED|FAILED|Total)" > summary.txt
```

## Best Practices

1. **Use Verbose Mode for Development**: Enable `--verbose` during test case development to understand verification behavior in real-time.

2. **Selective Verbosity in CI/CD**: Enable verbose logging only for failed builds to reduce log volume while maintaining debuggability.

3. **Log Retention**: Archive verbose logs from failed verifications for post-mortem analysis.

4. **Pattern Development**: Use verbose output to iteratively refine regex patterns and verification logic.

5. **Performance Monitoring**: Watch for performance issues in verbose logs, especially with large test suites.

6. **Debug Incremental Changes**: When modifying test cases, run with `--verbose` to ensure changes have the intended effect.

7. **Document Patterns**: Use verbose output examples in test case documentation to show expected behavior.
