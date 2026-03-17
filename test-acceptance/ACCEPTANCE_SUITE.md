# Acceptance Test Suite Master Orchestrator

The `run_acceptance_suite.sh` script is a comprehensive master orchestrator that automates the complete acceptance testing workflow from validation to documentation generation.

## Overview

The orchestrator executes six sequential stages:

1. **YAML Validation** - Validates all test case YAMLs against schema
2. **Script Generation** - Generates bash scripts with JSON logging
3. **Test Execution** - Executes automated tests (skip manual by default)
4. **Verification** - Runs verifier on execution logs to create containers
5. **Container Validation** - Validates container YAMLs against schema
6. **Documentation** - Generates AsciiDoc and Markdown reports via TPDG

## Prerequisites

### Required Binaries

Build all required binaries before running the suite:

```bash
# Build all required binaries
cargo build --bin validate-yaml
cargo build --bin test-executor
cargo build --bin verifier
cargo build --bin validate-json
```

### Optional: Test Plan Documentation Generator

For documentation generation (Stage 6), install TPDG:

```bash
# Install globally
cargo install test-plan-documentation-generator

# Or set custom path
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator
```

If TPDG is not available, Stage 6 will be skipped with a warning.

## Usage

### Basic Execution

Run the complete suite with default settings:

```bash
cd test-acceptance
./run_acceptance_suite.sh
```

This will:
- Validate all test case YAMLs
- Generate bash scripts for all test cases
- Execute only automated tests (skip manual tests)
- Verify execution logs
- Validate container YAMLs
- Generate documentation (if TPDG available)

### Command Line Options

```bash
./run_acceptance_suite.sh [OPTIONS]

OPTIONS:
    --verbose               Enable verbose output with detailed logging
    --include-manual        Include manual tests in execution
    --skip-generation       Skip bash script generation stage
    --skip-execution        Skip test execution stage
    --skip-verification     Skip verification stage
    --skip-documentation    Skip documentation generation stage
    -h, --help             Show help message
```

### Common Usage Patterns

**Verbose mode for debugging:**
```bash
./run_acceptance_suite.sh --verbose
```

**Include manual tests:**
```bash
./run_acceptance_suite.sh --include-manual
```

**Skip slow stages for quick validation:**
```bash
./run_acceptance_suite.sh --skip-execution --skip-verification
```

**Regenerate documentation only:**
```bash
./run_acceptance_suite.sh --skip-generation --skip-execution
```

**Validation and generation only:**
```bash
./run_acceptance_suite.sh --skip-execution --skip-verification --skip-documentation
```

## Directory Structure

The orchestrator creates and uses the following directory structure:

```
test-acceptance/
├── test_cases/              # Input: Test case YAML files
│   ├── success/
│   ├── failure/
│   ├── variables/
│   ├── hooks/
│   ├── dependencies/
│   ├── bash_commands/
│   ├── complex/
│   └── manual/
├── scripts/                 # Generated: Executable bash scripts
│   ├── TC_SUCCESS_*.sh
│   ├── TC_FAILURE_*.sh
│   └── ...
├── execution_logs/          # Generated: JSON execution logs
│   ├── TC_SUCCESS_*.json
│   ├── TC_FAILURE_*.json
│   └── ...
├── verification_results/    # Generated: Container YAMLs
│   ├── TC_SUCCESS_*_container.yaml
│   ├── TC_FAILURE_*_container.yaml
│   └── ...
├── reports/                 # Generated: Documentation reports
│   ├── asciidoc/           # AsciiDoc format reports
│   │   ├── TC_SUCCESS_*.adoc
│   │   └── ...
│   ├── markdown/           # Markdown format reports
│   │   ├── TC_SUCCESS_*.md
│   │   └── ...
│   └── acceptance_suite_summary.txt  # Final summary
└── run_acceptance_suite.sh  # Master orchestrator script
```

## Stage Details

### Stage 1: YAML Validation

Validates all test case YAML files against `schemas/test-case.schema.json`.

**Actions:**
- Finds all `*.yaml` files in `test_cases/` directory
- Validates each using `validate-yaml` binary
- Tracks pass/fail counts

**Output:**
- Console: Pass/fail status for each file
- Summary: Total passed and failed

**Skip:** Not skippable (required for all subsequent stages)

### Stage 2: Script Generation

Generates executable bash scripts from test case YAMLs.

**Actions:**
- Reads test case YAML files
- Generates bash scripts using `test-executor generate --json-log`
- Saves scripts to `scripts/` directory
- Makes scripts executable

**Output:**
- `scripts/TC_*.sh` - Generated bash scripts
- Console: Generation status for each file

**Skip:** `--skip-generation` (use existing scripts)

### Stage 3: Test Execution

Executes generated test scripts and captures JSON logs.

**Actions:**
- Finds all `*.sh` files in `scripts/` directory
- Identifies and skips manual tests (unless `--include-manual`)
- Executes each script
- Captures output to JSON log files
- Validates JSON structure

**Output:**
- `execution_logs/TC_*.json` - Execution logs
- Console: Execution status (pass/fail/skipped)

**Skip:** `--skip-execution` (use existing logs)

**Manual Tests:**
- By default, tests with `manual: true` are skipped
- Use `--include-manual` to execute them
- Manual tests require human interaction

### Stage 4: Verification

Runs verifier to generate container YAMLs with metadata.

**Actions:**
- Processes each execution log JSON file
- Finds corresponding test case YAML
- Runs `verifier` with metadata flags:
  - `--title`: Test-specific title
  - `--project`: "Test Case Manager - Acceptance Suite"
  - `--environment`: Hostname-based environment
- Generates container YAML files

**Output:**
- `verification_results/TC_*_container.yaml` - Container files
- Console: Verification status

**Skip:** `--skip-verification` (use existing containers)

**Metadata Included:**
- Title: Derived from test case filename
- Project: "Test Case Manager - Acceptance Suite"
- Environment: "Automated Test Environment - [hostname]"
- Test date: ISO 8601 timestamp

### Stage 5: Container Validation

Validates container YAMLs against schema.

**Actions:**
- Finds all `*_container.yaml` files
- Validates against `data/testcase_results_container/schema.json`
- Ensures compatibility with TPDG

**Output:**
- Console: Validation status for each container

**Skip:** Auto-skipped if `--skip-verification` is set

### Stage 6: Documentation Generation

Generates AsciiDoc and Markdown reports using TPDG.

**Actions:**
- Checks for TPDG binary availability
- Processes each container YAML
- Generates both AsciiDoc and Markdown formats
- Includes original test case YAML for context

**Output:**
- `reports/asciidoc/TC_*.adoc` - AsciiDoc reports
- `reports/markdown/TC_*.md` - Markdown reports
- Console: Generation status

**Skip:** `--skip-documentation` or auto-skip if TPDG not available

**TPDG Command:**
```bash
test-plan-documentation-generator \
    --input container.yaml \
    --output report.adoc \
    --format asciidoc \
    --test-case original.yaml
```

## Output and Reporting

### Console Output

The script provides real-time console output with:
- Color-coded status indicators (✓ pass, ✗ fail, ℹ info)
- Section headers for each stage
- Progress messages
- Summary statistics

**Example:**
```
=== Stage 1: Validating Test Case YAMLs ===
[INFO] Found 93 test case YAML files

✓ TC_SUCCESS_SIMPLE_001.yaml
✓ TC_SUCCESS_MULTI_SEQ_001.yaml
✗ TC_INVALID_TEST_001.yaml

[INFO] Validation: 92 passed, 1 failed
```

### Summary Report

A comprehensive summary is saved to `reports/acceptance_suite_summary.txt`:

```
=========================================
Acceptance Test Suite Execution Summary
=========================================

Execution Date: 2024-03-17 14:30:00
Total Test Cases: 93

--- Stage 1: YAML Validation ---
Passed:  92
Failed:  1

Failed validations:
test-acceptance/test_cases/invalid/TC_INVALID_TEST_001.yaml

--- Stage 2: Script Generation ---
Passed:  92
Failed:  0

--- Stage 3: Test Execution ---
Passed:  65
Failed:  10
Skipped: 17 (manual tests)

--- Stage 4: Verification ---
Passed:  75
Failed:  0

--- Stage 5: Container Validation ---
Passed:  75
Failed:  0

--- Stage 6: Documentation Generation ---
Passed:  75
Failed:  0

=========================================
Overall Result:
SUCCESS - All stages completed without errors
=========================================
```

## Statistics Tracking

The orchestrator tracks detailed statistics:

- **Per Stage:**
  - Passed count
  - Failed count
  - Skipped count (execution only)

- **Overall:**
  - Total test cases found
  - Total failures across all stages
  - Execution time

## Exit Codes

- `0` - All stages completed successfully
- `1` - One or more stages had failures

## Error Handling

### Validation Failures

If test case YAML validation fails:
- Logged to console
- Recorded in failure tracking
- Subsequent stages continue with valid files

### Execution Failures

If test execution fails:
- Exit code captured
- Logged to console
- Verification continues for successful tests

### Missing Dependencies

If required binaries are missing:
- Clear error messages displayed
- Script exits early
- Build instructions provided

## Debugging

### Verbose Mode

Enable verbose output for detailed debugging:

```bash
./run_acceptance_suite.sh --verbose
```

Verbose mode shows:
- Detailed validation messages
- Command outputs on failure
- File processing steps
- Tool invocation details

### Skip Stages

Skip expensive stages during development:

```bash
# Validate only
./run_acceptance_suite.sh \
    --skip-execution \
    --skip-verification \
    --skip-documentation

# Skip generation (use existing scripts)
./run_acceptance_suite.sh --skip-generation

# Verify existing results only
./run_acceptance_suite.sh \
    --skip-generation \
    --skip-execution
```

### Temporary Files

The script uses temporary files for tracking:
- Located in system temp directory
- Automatically cleaned up on exit
- Contains detailed failure information

## Integration with CI/CD

### GitLab CI Example

```yaml
acceptance_tests:
  stage: test
  script:
    - cargo build --bin validate-yaml
    - cargo build --bin test-executor
    - cargo build --bin verifier
    - cargo build --bin validate-json
    - cargo install test-plan-documentation-generator
    - cd test-acceptance
    - ./run_acceptance_suite.sh --verbose
  artifacts:
    paths:
      - test-acceptance/reports/
    when: always
```

### GitHub Actions Example

```yaml
- name: Run Acceptance Suite
  run: |
    cargo build --bin validate-yaml
    cargo build --bin test-executor
    cargo build --bin verifier
    cargo build --bin validate-json
    cargo install test-plan-documentation-generator
    cd test-acceptance
    ./run_acceptance_suite.sh --verbose
    
- name: Upload Reports
  uses: actions/upload-artifact@v3
  if: always()
  with:
    name: acceptance-reports
    path: test-acceptance/reports/
```

## Best Practices

1. **Run Full Suite Before Commits:**
   ```bash
   ./run_acceptance_suite.sh
   ```

2. **Quick Validation During Development:**
   ```bash
   ./run_acceptance_suite.sh --skip-execution --skip-documentation
   ```

3. **Debug Specific Failures:**
   ```bash
   ./run_acceptance_suite.sh --verbose --skip-documentation
   ```

4. **Generate Fresh Documentation:**
   ```bash
   ./run_acceptance_suite.sh --skip-generation --skip-execution
   ```

5. **Test Manual Workflows:**
   ```bash
   ./run_acceptance_suite.sh --include-manual
   ```

## Troubleshooting

### "Binary not found" errors

**Problem:** Required binaries are missing

**Solution:**
```bash
cargo build --bin validate-yaml
cargo build --bin test-executor
cargo build --bin verifier
cargo build --bin validate-json
```

### "No test cases found"

**Problem:** Script can't find test case YAMLs

**Solution:**
- Ensure you're running from `test-acceptance/` directory
- Check that `test_cases/` directory exists and contains `*.yaml` files

### "Invalid JSON in execution log"

**Problem:** Generated script didn't produce valid JSON

**Solution:**
- Check test script for syntax errors
- Run test script manually and inspect output
- Enable `--verbose` mode for details

### "TPDG not found"

**Problem:** Documentation generator not installed

**Solution:**
```bash
cargo install test-plan-documentation-generator
# Or set custom path:
export TEST_PLAN_DOC_GEN=/path/to/binary
```

### Stage failures

**Problem:** Specific stage consistently fails

**Solution:**
- Use `--verbose` for detailed error messages
- Run stage manually to isolate issue
- Check `reports/acceptance_suite_summary.txt` for details

## Limitations

1. **Sequential Execution:** Tests run sequentially, not in parallel
2. **Manual Tests:** Require human interaction, skipped by default
3. **Resource Intensive:** Full suite with 90+ tests takes time
4. **TPDG Dependency:** Documentation requires external tool

## Future Enhancements

Potential improvements:

- Parallel test execution
- Incremental runs (only changed tests)
- HTML summary report generation
- Test result comparison with baseline
- Performance metrics tracking
- Email notification support
- Slack/Teams integration

## See Also

- [Test Acceptance README](README.md) - Overview of test cases
- [AGENTS.md](../AGENTS.md) - Project commands and guidelines
- [Report Generation](../docs/report_generation.md) - TPDG documentation
- [Test Case Schema](../schemas/test-case.schema.json) - YAML schema
- [Container Schema](../data/testcase_results_container/schema.json) - Container schema
