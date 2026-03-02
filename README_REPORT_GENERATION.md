# Verifier Scenario Report Generation Guide

This guide explains how to generate PDF reports for the 7 verifier test scenarios.

## Overview

The report generation process consists of three steps:
1. **Test Execution** - Generate execution logs (already created for verifier scenarios)
2. **Verification** - Run verifier to analyze execution logs against test cases
3. **Report Generation** - Create PDF reports from verification results

## Prerequisites

```bash
# Install Python dependencies for PDF generation
pip3 install reportlab

# Ensure Rust/Cargo is installed and binaries can be built
cargo build --release --bin verifier
```

## Scenarios Covered

The following 7 test scenarios are included:

1. **TEST_SUCCESS_001** - Successful execution with all steps passing
2. **TEST_FAILED_FIRST_001** - First step failure preventing subsequent execution
3. **TEST_FAILED_INTERMEDIATE_001** - Intermediate step failure with partial execution
4. **TEST_FAILED_LAST_001** - Last step failure with output mismatch
5. **TEST_INTERRUPTED_001** - Interrupted execution with incomplete sequences
6. **TEST_MULTI_SEQ_001** - Multiple sequences with mixed results
7. **TEST_HOOK_SCRIPT_START_001** - Hook failure at script start

## Quick Start

### Option 1: Automated Report Generation (Recommended)

Run the Python script to automatically build binaries, run verifier, and generate PDF reports:

```bash
python3 scripts/generate_verifier_reports.py
```

This will:
- Build the verifier binary
- Run verifier on all 7 execution logs
- Generate PDF reports (or HTML if reportlab not installed)
- Place all reports in `reports/verifier_scenarios/`

### Option 2: Manual Step-by-Step

If you prefer to run each step manually:

```bash
# 1. Create output directory
mkdir -p reports/verifier_scenarios

# 2. Build verifier
cargo build --release --bin verifier

# 3. Run verifier for each scenario
cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/successful/TEST_SUCCESS_001_execution_log.json \
    --test-case TEST_SUCCESS_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_SUCCESS_001_verification.json

cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/failed_first/TEST_FAILED_FIRST_001_execution_log.json \
    --test-case TEST_FAILED_FIRST_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_FAILED_FIRST_001_verification.json

cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/failed_intermediate/TEST_FAILED_INTERMEDIATE_001_execution_log.json \
    --test-case TEST_FAILED_INTERMEDIATE_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_FAILED_INTERMEDIATE_001_verification.json

cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/failed_last/TEST_FAILED_LAST_001_execution_log.json \
    --test-case TEST_FAILED_LAST_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_FAILED_LAST_001_verification.json

cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/interrupted/TEST_INTERRUPTED_001_execution_log.json \
    --test-case TEST_INTERRUPTED_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_INTERRUPTED_001_verification.json

cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/multiple_sequences/TEST_MULTI_SEQ_001_execution_log.json \
    --test-case TEST_MULTI_SEQ_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_MULTI_SEQ_001_verification.json

cargo run --release --bin verifier -- \
    --log testcases/verifier_scenarios/hooks/TEST_HOOK_SCRIPT_START_001_execution_log.json \
    --test-case TEST_HOOK_SCRIPT_START_001 \
    --format json \
    --output reports/verifier_scenarios/TEST_HOOK_SCRIPT_START_001_verification.json

# 4. Generate PDF reports
python3 scripts/generate_verifier_reports.py
```

### Option 3: Shell Script

A convenience shell script is provided:

```bash
./generate_reports.sh
```

This will generate verification JSON files. If you have reportlab installed, you can then run the Python script to create PDFs.

## Output Files

After successful execution, you will find:

### Verification Reports (JSON)
- `reports/verifier_scenarios/TEST_SUCCESS_001_verification.json`
- `reports/verifier_scenarios/TEST_FAILED_FIRST_001_verification.json`
- `reports/verifier_scenarios/TEST_FAILED_INTERMEDIATE_001_verification.json`
- `reports/verifier_scenarios/TEST_FAILED_LAST_001_verification.json`
- `reports/verifier_scenarios/TEST_INTERRUPTED_001_verification.json`
- `reports/verifier_scenarios/TEST_MULTI_SEQ_001_verification.json`
- `reports/verifier_scenarios/TEST_HOOK_SCRIPT_START_001_verification.json`

### PDF Reports
- `reports/verifier_scenarios/TEST_SUCCESS_001_report.pdf`
- `reports/verifier_scenarios/TEST_FAILED_FIRST_001_report.pdf`
- `reports/verifier_scenarios/TEST_FAILED_INTERMEDIATE_001_report.pdf`
- `reports/verifier_scenarios/TEST_FAILED_LAST_001_report.pdf`
- `reports/verifier_scenarios/TEST_INTERRUPTED_001_report.pdf`
- `reports/verifier_scenarios/TEST_MULTI_SEQ_001_report.pdf`
- `reports/verifier_scenarios/TEST_HOOK_SCRIPT_START_001_report.pdf`

### HTML Reports (if reportlab not available)
If reportlab is not installed, HTML reports will be generated instead with the same names but `.html` extension.

## Report Contents

Each PDF report contains:

1. **Test Summary**
   - Test case ID and description
   - Overall pass/fail status
   - Step counts (total, passed, failed, not executed)
   - Report generation timestamp

2. **Test Sequences**
   - For each sequence: ID, name, and status
   - Detailed step results table with:
     - Step number
     - Description
     - Status (pass/fail/not_executed)
     - Failure reason (if applicable)

3. **Visual Indicators**
   - Color-coded status (green=pass, red=fail, grey=not executed)
   - Professional formatting with tables and sections

## Execution Logs

Execution logs have been pre-created for all 7 scenarios to simulate specific test outcomes:

- `testcases/verifier_scenarios/successful/TEST_SUCCESS_001_execution_log.json` - All steps succeed
- `testcases/verifier_scenarios/failed_first/TEST_FAILED_FIRST_001_execution_log.json` - First step fails
- `testcases/verifier_scenarios/failed_intermediate/TEST_FAILED_INTERMEDIATE_001_execution_log.json` - Middle step fails
- `testcases/verifier_scenarios/failed_last/TEST_FAILED_LAST_001_execution_log.json` - Last step fails
- `testcases/verifier_scenarios/interrupted/TEST_INTERRUPTED_001_execution_log.json` - Incomplete execution
- `testcases/verifier_scenarios/multiple_sequences/TEST_MULTI_SEQ_001_execution_log.json` - Mixed sequence results
- `testcases/verifier_scenarios/hooks/TEST_HOOK_SCRIPT_START_001_execution_log.json` - Hook error

These logs simulate realistic test execution scenarios and are used by the verifier to validate the test framework's verification capabilities.

## Troubleshooting

### reportlab not installed
```
Warning: reportlab not installed. Will generate HTML reports instead.
To generate PDF reports, install reportlab: pip3 install reportlab
```
**Solution:** Install reportlab with `pip3 install reportlab`

### Verifier binary not found
```
✗ Failed to build binaries. Exiting.
```
**Solution:** Ensure cargo/rust is installed and run `cargo build --release --bin verifier`

### Execution log not found
```
⚠ Skipping TEST_XXX_001: execution log not found
```
**Solution:** Ensure the execution log JSON file exists in the expected location

### Permission errors
If you encounter permission errors when creating the `reports/` directory, you can modify the scripts to use an alternative location like `/tmp/verifier_reports`:

```bash
# In Python script
output_dir = Path('/tmp/verifier_reports')

# In shell script
OUTPUT_DIR="/tmp/verifier_reports"
```

## Integration with CI/CD

To integrate report generation in CI/CD pipelines:

```yaml
verifier_reports:
  stage: test
  script:
    - pip3 install reportlab
    - python3 scripts/generate_verifier_reports.py
  artifacts:
    paths:
      - reports/verifier_scenarios/
    expire_in: 30 days
```

## Additional Resources

- **Verifier Scenarios Documentation:** `testcases/verifier_scenarios/README.md`
- **Main Project Documentation:** `AGENTS.md`
- **Verifier Source Code:** `src/bin/verifier.rs`
- **Verification Module:** `src/verification.rs`
