# Sample Test Case Generation Implementation

## Overview

This implementation provides comprehensive sample test case generation covering all major test execution scenarios. The system generates test cases, execution logs, verification results, and documentation reports in both AsciiDoc and Markdown formats.

## Components Implemented

### 1. Sample Test Case Generator (`scripts/generate_all_sample_cases.sh`)

Generates comprehensive sample test cases covering all scenarios:

- **Successful Execution** (`SAMPLE_SUCCESS_001`)
  - All steps execute successfully
  - All verification checks pass
  - Demonstrates basic command execution

- **Failed First Step** (`SAMPLE_FAILED_FIRST_001`)
  - First step fails with exit code mismatch
  - Subsequent steps not executed
  - Demonstrates early failure handling

- **Failed Intermediate Step** (`SAMPLE_FAILED_INTERMEDIATE_001`)
  - Steps 1-2 pass successfully
  - Step 3 fails
  - Step 4 not executed
  - Demonstrates mid-sequence failure

- **Failed Last Step** (`SAMPLE_FAILED_LAST_001`)
  - All steps execute
  - Last step fails output verification
  - Demonstrates output mismatch handling

- **Multiple Sequences** (`SAMPLE_MULTI_SEQ_001`)
  - Sequence 1: All steps pass
  - Sequence 2: Step 2 fails with output mismatch
  - Sequence 3: Not executed due to previous failure
  - Demonstrates multi-sequence behavior

- **Complex Variable Capture** (`SAMPLE_COMPLEX_001`)
  - Variable capture from command output using regex
  - Variable usage in subsequent steps
  - Conditional verification based on platform
  - Demonstrates advanced features

- **Hook Execution** (Multiple samples)
  - `SAMPLE_HOOK_SCRIPT_START_001`: script_start hook
  - `SAMPLE_HOOK_BEFORE_SEQ_001`: before_sequence hook
  - Demonstrates hook integration at different lifecycle points

### 2. Complete Execution and Reporting Script (`scripts/run_all_samples_and_generate_reports.sh`)

Orchestrates the entire workflow:

1. **Sample Generation**
   - Calls `generate_all_sample_cases.sh` to create test cases
   - Creates directory structure for all scenario types

2. **Binary Building**
   - Builds test-orchestrator binary
   - Builds verifier binary

3. **Test Execution**
   - Runs orchestrator on each sample test case
   - Generates execution logs in JSON format
   - Archives logs for later inspection

4. **Verification**
   - Runs verifier in folder mode on all execution logs
   - Generates batch verification reports in JSON and YAML formats
   - Identifies pass/fail status for each test case

5. **Result Conversion**
   - Converts verification JSON to individual result YAML files
   - Creates results container YAML with all test results

6. **Documentation Generation**
   - Generates comprehensive AsciiDoc report
   - Generates comprehensive Markdown report
   - Includes executive summary, test results overview, and detailed results

## Generated File Structure

```
testcases/generated_samples/
├── successful/
│   ├── SAMPLE_SUCCESS_001.yml
│   └── SAMPLE_SUCCESS_001_execution_log.json
├── failed_first/
│   ├── SAMPLE_FAILED_FIRST_001.yml
│   └── SAMPLE_FAILED_FIRST_001_execution_log.json
├── failed_intermediate/
│   ├── SAMPLE_FAILED_INTERMEDIATE_001.yml
│   └── SAMPLE_FAILED_INTERMEDIATE_001_execution_log.json
├── failed_last/
│   ├── SAMPLE_FAILED_LAST_001.yml
│   └── SAMPLE_FAILED_LAST_001_execution_log.json
├── multiple_sequences/
│   ├── SAMPLE_MULTI_SEQ_001.yml
│   └── SAMPLE_MULTI_SEQ_001_execution_log.json
├── complex/
│   ├── SAMPLE_COMPLEX_001.yml
│   └── SAMPLE_COMPLEX_001_execution_log.json
└── hooks/
    ├── scripts/
    │   ├── hook_success.sh
    │   └── hook_fail.sh
    ├── SAMPLE_HOOK_SCRIPT_START_001.yml
    ├── SAMPLE_HOOK_SCRIPT_START_001_execution_log.json
    ├── SAMPLE_HOOK_BEFORE_SEQ_001.yml
    └── SAMPLE_HOOK_BEFORE_SEQ_001_execution_log.json

reports/generated_samples/
├── verification/
│   ├── batch_verification.json
│   └── batch_verification.yaml
├── results/
│   ├── SAMPLE_SUCCESS_001_result.yaml
│   ├── SAMPLE_FAILED_FIRST_001_result.yaml
│   ├── SAMPLE_FAILED_INTERMEDIATE_001_result.yaml
│   ├── SAMPLE_FAILED_LAST_001_result.yaml
│   ├── SAMPLE_MULTI_SEQ_001_result.yaml
│   ├── SAMPLE_COMPLEX_001_result.yaml
│   ├── SAMPLE_HOOK_SCRIPT_START_001_result.yaml
│   ├── SAMPLE_HOOK_BEFORE_SEQ_001_result.yaml
│   └── results_container.yaml
├── execution_logs/
│   └── *_execution_log.json (archived copies)
└── docs/
    ├── sample_execution_results.adoc
    └── sample_execution_results.md
```

## Usage

### Generate Samples Only

```bash
./scripts/generate_all_sample_cases.sh
```

**Options:**
- `--output-dir DIR`: Specify output directory (default: testcases/generated_samples)
- `--verbose`: Enable verbose output
- `--help`: Show help message

### Run Complete Workflow

```bash
./scripts/run_all_samples_and_generate_reports.sh
```

**Options:**
- `--samples-dir DIR`: Directory for sample test cases (default: testcases/generated_samples)
- `--reports-dir DIR`: Output directory for reports (default: reports/generated_samples)
- `--skip-generation`: Skip sample generation (use existing)
- `--skip-execution`: Skip test execution (use existing logs)
- `--skip-verification`: Skip verification (use existing results)
- `--format FORMAT`: Report format: `both`, `asciidoc`, `markdown` (default: both)
- `--verbose`: Enable verbose output
- `--help`: Show help message

### Generate Only Specific Format

**AsciiDoc only:**
```bash
./scripts/run_all_samples_and_generate_reports.sh --format asciidoc
```

**Markdown only:**
```bash
./scripts/run_all_samples_and_generate_reports.sh --format markdown
```

**Both formats:**
```bash
./scripts/run_all_samples_and_generate_reports.sh --format both
```

### Skip Steps for Faster Iteration

**Use existing samples and logs, regenerate reports only:**
```bash
./scripts/run_all_samples_and_generate_reports.sh --skip-generation --skip-execution --skip-verification
```

## Report Formats

### AsciiDoc Report (`sample_execution_results.adoc`)

Structured documentation format suitable for:
- Technical documentation
- PDF generation (via asciidoctor-pdf)
- HTML generation (via asciidoctor)
- Publishing systems

**Features:**
- Table of contents
- Section numbering
- Syntax highlighting support
- Cross-references
- Professional formatting

**Example Usage:**
```bash
# Generate PDF
asciidoctor-pdf reports/generated_samples/docs/sample_execution_results.adoc

# Generate HTML
asciidoctor reports/generated_samples/docs/sample_execution_results.adoc
```

### Markdown Report (`sample_execution_results.md`)

Universal format suitable for:
- GitHub/GitLab documentation
- Wiki pages
- README files
- General documentation

**Features:**
- GitHub-flavored Markdown
- Tables for structured data
- Code blocks
- Links and cross-references
- Wide compatibility

## Report Contents

Both report formats include:

### 1. Executive Summary
- Purpose of sample test cases
- Coverage of all major scenarios
- Test execution environment details

### 2. Test Results Overview
- Summary statistics table:
  - Total test cases
  - Passed test cases
  - Failed test cases
  - Total steps
  - Passed steps
  - Failed steps
  - Not executed steps

### 3. Detailed Test Case Results

For each test case:
- Test case ID and description
- Requirement identifier
- Test sequences overview
- Expected outcome
- Actual outcome reference
- Detailed step-by-step results

### 4. Scenario Categories

Organized by type:
- Successful Execution Scenarios
- Failed First Step Scenarios
- Failed Intermediate Step Scenarios
- Failed Last Step Scenarios
- Multiple Sequence Scenarios
- Complex Scenarios (with variable capture)
- Hook Execution Scenarios

### 5. Appendix

- Raw verification data file locations
- JSON and YAML format references
- Individual result file references

## Integration with Existing Tools

### Verifier Integration

The generated samples work seamlessly with the existing verifier:

```bash
# Verify single sample
cargo run --bin verifier -- \
  --log testcases/generated_samples/successful/SAMPLE_SUCCESS_001_execution_log.json \
  --test-case SAMPLE_SUCCESS_001 \
  --format yaml

# Verify all samples in batch
cargo run --bin verifier -- \
  --folder testcases/generated_samples \
  --format json \
  --output reports/batch_verification.json
```

### Documentation Generator Integration

The generated result container YAML is compatible with test-plan-doc-gen:

```bash
# Generate documentation from results
test-plan-doc-gen \
  --container reports/generated_samples/results/results_container.yaml \
  --output reports/test_results.pdf \
  --format pdf
```

## Sample Test Case Details

### SAMPLE_SUCCESS_001 - Successful Execution

**Purpose:** Demonstrate complete successful test execution

**Test Sequence:**
1. Display greeting message (`echo "Hello World"`)
2. Display system date (`date +%Y-%m-%d`)
3. Display current username (`whoami`)

**Expected Behavior:**
- All steps execute successfully
- All verifications pass
- Overall test passes

### SAMPLE_FAILED_FIRST_001 - Failed First Step

**Purpose:** Demonstrate early failure preventing subsequent execution

**Test Sequence:**
1. Attempt invalid operation (`ls /nonexistent_directory_12345`) - FAILS
2. Step not executed
3. Step not executed

**Expected Behavior:**
- Step 1 fails with exit code mismatch
- Steps 2-3 marked as not executed
- Overall test fails

### SAMPLE_FAILED_INTERMEDIATE_001 - Failed Intermediate Step

**Purpose:** Demonstrate mid-sequence failure

**Test Sequence:**
1. First successful step (`echo "Step 1 success"`)
2. Second successful step (`echo "Step 2 success"`)
3. Third step fails (`cat /nonexistent_file_99999.txt`) - FAILS
4. Step not executed

**Expected Behavior:**
- Steps 1-2 pass
- Step 3 fails
- Step 4 not executed
- Overall test fails

### SAMPLE_FAILED_LAST_001 - Failed Last Step

**Purpose:** Demonstrate final step failure with output mismatch

**Test Sequence:**
1. First step passes (`echo "Step 1"`)
2. Second step passes (`echo "Step 2"`)
3. Last step fails verification (`echo "FAILURE"` vs expected "SUCCESS")

**Expected Behavior:**
- Steps 1-2 pass
- Step 3 fails output verification
- Overall test fails

### SAMPLE_MULTI_SEQ_001 - Multiple Sequences

**Purpose:** Demonstrate multi-sequence behavior with mixed results

**Test Sequences:**

**Sequence 1: First Sequence - Success**
1. Echo message - PASS
2. Echo another message - PASS

**Sequence 2: Second Sequence - Fails**
1. First step passes - PASS
2. Second step fails - FAIL (output mismatch)

**Sequence 3: Third Sequence - Not Executed**
1. Step not executed

**Expected Behavior:**
- Sequence 1 fully passes
- Sequence 2 step 2 fails
- Sequence 3 not executed
- Overall test fails

### SAMPLE_COMPLEX_001 - Complex Variable Capture

**Purpose:** Demonstrate advanced features

**Test Sequence:**
1. Generate timestamp and capture it
   - Uses regex to capture output: `^([0-9]+)$`
   - Stores in variable `TIMESTAMP`
2. Display captured timestamp
   - Uses `$TIMESTAMP` variable
3. Conditional verification based on platform
   - Different verification for Darwin vs Linux

**Expected Behavior:**
- Variable captured successfully
- Variable used in subsequent step
- Conditional verification works correctly
- All steps pass

### SAMPLE_HOOK_SCRIPT_START_001 - Script Start Hook

**Purpose:** Demonstrate script_start hook execution

**Hook:** `scripts/hook_success.sh` at script_start lifecycle point

**Test Sequence:**
1. Echo message

**Expected Behavior:**
- Hook executes before any test steps
- Hook succeeds
- Test step executes
- All pass

### SAMPLE_HOOK_BEFORE_SEQ_001 - Before Sequence Hook

**Purpose:** Demonstrate before_sequence hook execution

**Hook:** `scripts/hook_success.sh` at before_sequence lifecycle point

**Test Sequence:**
1. Echo message

**Expected Behavior:**
- Hook executes before sequence starts
- Hook succeeds
- Test step executes
- All pass

## Execution Log Format

All generated execution logs follow this JSON structure:

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo \"Hello World\"",
    "exit_code": 0,
    "output": "Hello World",
    "timestamp": "2024-01-15T10:00:00.000000+00:00"
  }
]
```

**Fields:**
- `test_sequence`: Sequence ID matching test case definition
- `step`: Step number within sequence
- `command`: Executed command string
- `exit_code`: Command exit code (0 = success)
- `output`: Command output/stderr
- `timestamp`: ISO 8601 timestamp with timezone

## Verification Result Format

Individual result YAML files contain:

```yaml
test_case_id: SAMPLE_SUCCESS_001
description: Sample successful execution with all steps passing
requirement: SAMPLE_SUCCESS
item: 1
tc: 1
total_steps: 3
passed_steps: 3
failed_steps: 0
not_executed_steps: 0
overall_pass: true
sequences:
  - sequence_id: 1
    name: "Basic Command Execution"
    all_steps_passed: true
    step_results:
      - Pass:
          step_number: 1
          description: "Display greeting message"
          expected: "Hello World"
          actual_result: "Hello World"
```

## Results Container Format

The results container aggregates all results:

```yaml
title: 'Sample Test Cases Execution Results'
project: 'Test Case Manager - Generated Samples'
test_date: '2024-01-01T00:00:00Z'
test_results:
  - test_case_id: SAMPLE_SUCCESS_001
    # ... (all result fields)
  - test_case_id: SAMPLE_FAILED_FIRST_001
    # ... (all result fields)
metadata:
  environment: 'Test Environment'
  platform: 'Test Case Manager - Generated Samples'
  executor: 'Automated Sample Workflow'
  execution_duration: 0.0
  total_test_cases: 8
  passed_test_cases: 0
  failed_test_cases: 0
```

## Customization

### Adding New Sample Scenarios

To add a new sample scenario type:

1. **Update `generate_all_sample_cases.sh`:**

```bash
# Add new category directory
mkdir -p "$OUTPUT_DIR/new_category"

# Create test case YAML
cat > "$OUTPUT_DIR/new_category/SAMPLE_NEW_001.yml" << 'EOF'
requirement: "SAMPLE_NEW"
item: 1
tc: 1
id: 'SAMPLE_NEW_001'
description: 'New sample scenario'
# ... test case definition
EOF

GENERATED_TEST_CASES+=("$OUTPUT_DIR/new_category/SAMPLE_NEW_001.yml")
```

2. **Create execution log** (manually or via orchestrator)

3. **Update report templates** in `run_all_samples_and_generate_reports.sh` to include new category

### Modifying Report Content

Edit the report generation sections in `run_all_samples_and_generate_reports.sh`:

- **AsciiDoc:** Modify the `cat > "$ASCIIDOC_REPORT"` heredoc
- **Markdown:** Modify the `cat > "$MARKDOWN_REPORT"` heredoc

## Dependencies

### Required:
- Bash 3.2+ (macOS/Linux compatible)
- Cargo and Rust toolchain
- Python 3 (for JSON/YAML processing)

### Optional:
- `asciidoctor` for AsciiDoc to HTML conversion
- `asciidoctor-pdf` for AsciiDoc to PDF conversion
- `pandoc` for Markdown format conversion

## Error Handling

The scripts include comprehensive error handling:

- Exit on any command failure (`set -e`)
- Validation of required files and directories
- Graceful handling of expected test failures (exit code 1)
- Logging of errors and warnings
- Cleanup of temporary resources

## Logging

All scripts use the centralized logging library (`scripts/lib/logger.sh`):

- `log_info`: Informational messages
- `log_warning`: Warning messages
- `log_error`: Error messages
- `pass`: Success messages with ✓
- `fail`: Failure messages with ✗
- `info`: Info messages with ℹ
- `section`: Section headers

## Performance Considerations

### Execution Time

- Sample generation: < 1 second
- Test execution: 1-5 seconds per test case
- Verification: 1-2 seconds for all samples
- Report generation: 1-2 seconds

### Disk Space

- Test cases: ~15KB
- Execution logs: ~500 bytes - 2KB each
- Verification results: ~5-10KB total
- Reports: ~20-30KB each

## Testing

The generated samples serve as integration tests for:

1. **Test Case Schema Validation**
   - All samples validate against schema
   - Cover all major schema features

2. **Orchestrator Functionality**
   - Test execution
   - Log generation
   - Hook execution

3. **Verifier Functionality**
   - Log parsing
   - Result verification
   - Pass/fail determination

4. **Report Generation**
   - JSON output
   - YAML output
   - Documentation formats

## Future Enhancements

Potential improvements:

1. **Additional Scenario Types**
   - Timeout scenarios
   - Resource cleanup failures
   - Network operation failures
   - Database operation scenarios

2. **Report Enhancements**
   - Charts and graphs
   - Trend analysis
   - Coverage metrics
   - Performance metrics

3. **Automation**
   - CI/CD integration
   - Scheduled execution
   - Email notifications
   - Slack/Teams integration

4. **Interactive Reports**
   - HTML with JavaScript
   - Filterable results
   - Drill-down capability
   - Export options

## Conclusion

This implementation provides a comprehensive sample generation system that:

- Covers all major test execution scenarios
- Generates execution logs automatically or manually
- Runs verification on all samples
- Produces professional documentation in multiple formats
- Integrates seamlessly with existing tools
- Serves as both examples and integration tests

The generated samples demonstrate the full capabilities of the Test Case Manager framework and provide a reference implementation for users creating their own test cases.
