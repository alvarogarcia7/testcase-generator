# Test Acceptance Suite

Comprehensive documentation for the YAML-based test harness acceptance test suite.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Directory Structure](#directory-structure)
- [Test Scenario Categories](#test-scenario-categories)
- [Running the Full Suite](#running-the-full-suite)
- [Running Individual Stage Validations](#running-individual-stage-validations)
- [Pipeline Validation Process](#pipeline-validation-process)
- [TPDG Integration](#tpdg-integration)
- [Adding New Test Cases](#adding-new-test-cases)
- [Output and Report Formats](#output-and-report-formats)
- [Troubleshooting](#troubleshooting)
- [CI/CD Integration](#cicd-integration)

---

## Overview

The acceptance test suite provides comprehensive end-to-end testing of the YAML-based test harness project, validating the entire workflow from YAML test case definitions through to executable scripts and documentation generation.

### Purpose

The acceptance test suite serves four primary purposes:

1. **Validation**: Verify that the test harness correctly handles various test scenarios
2. **Documentation**: Demonstrate features and best practices through examples
3. **Regression Testing**: Ensure new changes don't break existing functionality
4. **Feature Coverage**: Provide comprehensive coverage of all harness capabilities

### Statistics

- **Total test cases**: 91
- **Test categories**: 8 (success, failure, hooks, manual, variables, dependencies, complex, bash_commands)
- **Pipeline stages**: 7 (validation, generation, execution, verification, container validation, individual documentation, consolidated documentation)
- **Supported platforms**: macOS (BSD, bash 3.2+) and Linux (GNU, bash 3.2+)

---

## Quick Start

### Run the Complete Suite

From the project root, run:

```bash
make acceptance-test
```

This will:
- Build all required binaries (test-executor, verifier, validate-yaml, validate-json)
- Validate TPDG (test-plan-documentation-generator) availability
- Execute all 7 pipeline stages
- Generate comprehensive individual and consolidated reports
- Display final statistics and results

**Exit codes:**
- `0` - All stages completed successfully
- `1` - One or more stages had failures

### Alternative: Direct Script Execution

```bash
cd test-acceptance
./run_acceptance_suite.sh
```

### Common Options

```bash
# Verbose output for debugging
./run_acceptance_suite.sh --verbose

# Include manual tests in execution
./run_acceptance_suite.sh --include-manual

# Skip expensive stages for quick validation
./run_acceptance_suite.sh --skip-execution --skip-documentation

# Regenerate documentation only
./run_acceptance_suite.sh --skip-generation --skip-execution
```

---

## Directory Structure

```
test-acceptance/
├── test_cases/                  # Test case YAML files (91 total)
│   ├── success/                 # Success scenario tests (13)
│   ├── failure/                 # Failure scenario tests (14)
│   ├── hooks/                   # Lifecycle hooks tests (14)
│   ├── manual/                  # Manual test cases (9)
│   ├── variables/               # Variable capture/usage tests (11)
│   ├── dependencies/            # Dependency management tests (8)
│   ├── complex/                 # Complex integration tests (9)
│   ├── bash_commands/           # Bash command tests (13)
│   └── README.md                # Test case documentation
│
├── scripts/                     # Generated executable bash scripts (gitignored)
│   ├── TC_SUCCESS_*.sh
│   ├── TC_FAILURE_*.sh
│   └── ...
│
├── execution_logs/              # JSON execution logs (gitignored)
│   ├── TC_SUCCESS_*.json
│   ├── TC_FAILURE_*.json
│   └── ...
│
├── verification_results/        # Container YAMLs (gitignored)
│   ├── TC_SUCCESS_*_container.yaml
│   ├── TC_FAILURE_*_container.yaml
│   └── ...
│
├── reports/                     # Generated documentation (gitignored)
│   ├── asciidoc/                # AsciiDoc format reports (individual)
│   ├── markdown/                # Markdown format reports (individual)
│   ├── consolidated/            # Consolidated documentation (all tests)
│   │   ├── all_tests_container.yaml
│   │   ├── all_tests.adoc
│   │   └── all_tests.md
│   ├── acceptance_suite_execution.log
│   └── acceptance_suite_summary.txt
│
├── run_acceptance_suite.sh      # Master orchestrator script
├── validate_stage1_yaml.sh      # Stage 1: YAML validation
├── validate_stage2_scripts.sh   # Stage 2: Script generation validation
├── validate_stage3_execution.sh # Stage 3: Execution validation
├── validate_stage4_verification.sh  # Stage 4: Verification validation
├── validate_stage5_tpdg_result_docs.sh  # Stage 5: Result docs validation
├── validate_stage6_tpdg_plan_docs.sh    # Stage 6: Plan docs validation
├── validate_stage7_consolidated_docs.sh # Stage 7: Consolidated docs validation
├── generate_final_report.sh     # Summary report generator
│
├── README.md                    # This file
├── ACCEPTANCE_SUITE.md          # Detailed orchestrator documentation
├── QUICKSTART.md                # Quick start guide
├── WORKFLOW.md                  # Workflow documentation
└── IMPLEMENTATION_SUMMARY.md    # Implementation details
```

---

## Test Scenario Categories

The test suite includes 91 test cases across 8 categories:

### 1. Success Scenarios (`success/`, 13 tests)

Tests that demonstrate successful test execution scenarios:

- **TC_SUCCESS_SIMPLE_001**: Simple single-sequence test (3 steps)
- **TC_SUCCESS_MULTI_SEQ_001**: Multi-sequence test (3 sequences, 2-4 steps each)
- **TC_SUCCESS_VAR_CAPTURE_001**: Variable capture and usage
- **TC_SUCCESS_REGEX_VALIDATION_001**: Output validation with regex
- **TC_SUCCESS_ENV_VARS_001**: Environment variable usage
- **TC_SUCCESS_CMD_CHAIN_001**: Command chaining with &&
- **TC_SUCCESS_STEP_DEPS_001**: Step dependencies using captured variables
- **TC_SUCCESS_LONG_RUNNING_001**: Long-running commands
- **TC_SUCCESS_EMPTY_OUTPUT_001**: Empty output validation
- **TC_SUCCESS_CONDITIONAL_001**: Complex conditional verification logic
- **TC_SUCCESS_COMPLEX_DATA_001**: Complex data processing
- **TC_SUCCESS_FILE_OPS_001**: Advanced file operations
- **TC_SUCCESS_TEXT_PROCESSING_001**: Advanced text processing

### 2. Failure Scenarios (`failure/`, 14 tests)

Tests that demonstrate various failure modes and error handling:

- Command execution failures
- Validation failures
- Expected failures for negative testing
- Error handling and recovery scenarios

### 3. Hooks (`hooks/`, 14 tests)

Tests demonstrating test execution lifecycle hooks:

- `script_start` - Script initialization
- `setup_test` - Test-wide setup
- `before_sequence` - Sequence initialization
- `after_sequence` - Sequence cleanup
- `before_step` - Step preparation
- `after_step` - Step validation
- `teardown_test` - Test-wide cleanup
- `script_end` - Final cleanup

### 4. Manual Tests (`manual/`, 9 tests)

Tests requiring human interaction:

- User input scenarios
- Manual verification steps
- Interactive workflows
- **Note**: Skipped by default; use `--include-manual` to execute

### 5. Variables (`variables/`, 11 tests)

Tests for variable capture and usage:

- Regex-based variable capture
- Command-based variable capture
- Variable substitution
- Cross-step variable dependencies
- Sequence-scoped variables

### 6. Dependencies (`dependencies/`, 8 tests)

Tests for dependency management:

- Step dependencies
- Sequence dependencies
- Variable-based dependencies
- Conditional execution based on dependencies

### 7. Complex Integration (`complex/`, 9 tests)

Complex end-to-end integration tests:

- Multi-sequence workflows
- Complex data transformations
- Integration with external tools
- Advanced verification scenarios

### 8. Bash Commands (`bash_commands/`, 13 tests)

Tests for various bash command scenarios:

- Command chaining
- Pipes and redirections
- Subshells and command substitution
- Script compatibility (BSD/GNU)
- Bash 3.2+ compatibility

---

## Running the Full Suite

### Using Make (Recommended)

From the project root:

```bash
make acceptance-test
```

This target:
1. Builds all required binaries
2. Validates TPDG availability
3. Runs the orchestrator script
4. Captures output to log file
5. Generates summary report
6. Returns appropriate exit code

### Direct Script Execution

From the `test-acceptance/` directory:

```bash
./run_acceptance_suite.sh [OPTIONS]
```

**Available Options:**

| Option | Description |
|--------|-------------|
| `--verbose` | Enable verbose output with detailed logging |
| `--include-manual` | Include manual tests in execution (requires user interaction) |
| `--skip-generation` | Skip bash script generation stage (use existing scripts) |
| `--skip-execution` | Skip test execution stage (use existing logs) |
| `--skip-verification` | Skip verification stage (use existing results) |
| `--skip-documentation` | Skip documentation generation stage |
| `-h, --help` | Show help message |

**Examples:**

```bash
# Full suite with verbose output
./run_acceptance_suite.sh --verbose

# Quick validation (skip execution and docs)
./run_acceptance_suite.sh --skip-execution --skip-documentation

# Regenerate documentation only
./run_acceptance_suite.sh --skip-generation --skip-execution

# Test everything including manual tests
./run_acceptance_suite.sh --include-manual --verbose
```

---

## Running Individual Stage Validations

For targeted validation or debugging, run individual stage scripts:

### Stage 1: YAML Validation

Validates all test case YAMLs against schema:

```bash
./validate_stage1_yaml.sh
```

**What it does:**
- Finds all `*.yaml` files in `test_cases/`
- Validates against `schemas/test-case.schema.json`
- Reports pass/fail for each file

### Stage 2: Script Generation

Generates and validates bash scripts:

```bash
./validate_stage2_scripts.sh
```

**What it does:**
- Generates bash scripts from test case YAMLs
- Validates script syntax
- Ensures scripts are executable

### Stage 3: Test Execution

Executes test scripts and validates output:

```bash
./validate_stage3_execution.sh
```

**What it does:**
- Runs generated test scripts
- Captures JSON execution logs
- Validates JSON structure
- Skips manual tests by default

### Stage 4: Verification

Runs verifier to generate container YAMLs:

```bash
./validate_stage4_verification.sh
```

**What it does:**
- Processes execution logs
- Generates container YAML files with metadata
- Validates verification output

### Stage 5: TPDG Result Documentation

Generates result documentation:

```bash
./validate_stage5_tpdg_result_docs.sh
```

**What it does:**
- Generates AsciiDoc reports from containers
- Generates Markdown reports from containers
- Validates TPDG output

### Stage 6: TPDG Plan Documentation

Generates plan documentation:

```bash
./validate_stage6_tpdg_plan_docs.sh
```

**What it does:**
- Generates test plan documentation
- Creates comprehensive project documentation
- Validates final documentation output

**Note:** Stages 5, 6, and 7 require TPDG to be installed.

---

## Pipeline Validation Process

The acceptance test suite executes a 7-stage pipeline:

### Stage 1: YAML Validation ✓

**Purpose:** Validate all test case YAML files against schema

**Actions:**
- Scans `test_cases/` directory for `*.yaml` files
- Validates each file using `validate-yaml` binary
- Checks conformance to `schemas/test-case.schema.json`
- Tracks pass/fail counts

**Input:** Test case YAML files
**Output:** Validation results (pass/fail)
**Can Skip:** No (required for subsequent stages)

**Example Output:**
```
=== Stage 1: Validating Test Case YAMLs ===
[INFO] Found 91 test case YAML files

✓ TC_SUCCESS_SIMPLE_001.yaml
✓ TC_SUCCESS_MULTI_SEQ_001.yaml
✓ TC_FAILURE_EXPECTED_001.yaml

[INFO] Validation: 91 passed, 0 failed
```

### Stage 2: Script Generation ✓

**Purpose:** Generate executable bash scripts from test case YAMLs

**Actions:**
- Reads validated test case YAML files
- Generates bash scripts using `test-executor generate --json-log`
- Saves scripts to `scripts/` directory
- Makes scripts executable (`chmod +x`)
- Includes JSON logging instrumentation

**Input:** Test case YAML files
**Output:** Executable bash scripts in `scripts/`
**Can Skip:** Yes (`--skip-generation`)

**Generated Script Features:**
- JSON logging for execution tracking
- Step-by-step execution with validation
- Variable capture and substitution
- Error handling and exit codes
- Lifecycle hook integration

### Stage 3: Test Execution ✓

**Purpose:** Execute generated test scripts and capture JSON logs

**Actions:**
- Finds all `*.sh` files in `scripts/` directory
- Identifies manual tests (skips unless `--include-manual`)
- Executes each automated test script
- Captures output to JSON log files in `execution_logs/`
- Validates JSON structure using `validate-json`
- Records exit codes and execution status

**Input:** Bash scripts from Stage 2
**Output:** JSON execution logs in `execution_logs/`
**Can Skip:** Yes (`--skip-execution`)

**Manual Test Handling:**
- Tests with `manual: true` are skipped by default
- Use `--include-manual` to execute them
- Manual tests may require user interaction

**Example Log Structure:**
```json
{
  "test_case_id": "TC_SUCCESS_SIMPLE_001",
  "sequences": [
    {
      "sequence_id": "1",
      "steps": [
        {
          "step_number": 1,
          "command": "echo 'Hello'",
          "output": "Hello",
          "exit_code": 0
        }
      ]
    }
  ]
}
```

### Stage 4: Verification ✓

**Purpose:** Run verifier to generate container YAMLs with metadata

**Actions:**
- Processes each JSON execution log
- Finds corresponding test case YAML
- Runs `verifier` with metadata flags:
  - `--title`: Test-specific title
  - `--project`: "Test Case Manager - Acceptance Suite"
  - `--environment`: "Automated Test Environment - [hostname]"
- Generates container YAML files in `verification_results/`
- Includes test execution results and metadata

**Input:** Execution logs from Stage 3
**Output:** Container YAML files in `verification_results/`
**Can Skip:** Yes (`--skip-verification`)

**Container YAML Structure:**
```yaml
metadata:
  title: "TC_SUCCESS_SIMPLE_001"
  project: "Test Case Manager - Acceptance Suite"
  environment: "Automated Test Environment - hostname"
  test_date: "2024-03-17T14:30:00Z"
test_case:
  # Original test case YAML
results:
  # Execution results and verification
```

### Stage 5: Container Validation ✓

**Purpose:** Validate container YAMLs against schema

**Actions:**
- Finds all `*_container.yaml` files in `verification_results/`
- Validates against `data/testcase_results_container/schema.json`
- Ensures compatibility with TPDG
- Reports validation status for each container

**Input:** Container YAMLs from Stage 4
**Output:** Validation results (pass/fail)
**Can Skip:** Auto-skipped if Stage 4 is skipped

**Why This Matters:**
- Ensures container YAMLs are TPDG-compatible
- Validates schema compliance
- Catches data structure issues early

### Stage 6: Individual Documentation ✓

**Purpose:** Generate individual AsciiDoc and Markdown documentation for each test using TPDG

**Actions:**
- Checks for TPDG binary availability
- Processes each container YAML individually
- Generates AsciiDoc reports in `reports/asciidoc/`
- Generates Markdown reports in `reports/markdown/`
- Includes original test case YAML for context
- Creates comprehensive per-test documentation

**Input:** Container YAMLs from Stage 4
**Output:** 
- Individual AsciiDoc reports (`.adoc`)
- Individual Markdown reports (`.md`)
- Individual HTML reports (if asciidoctor installed)

**Can Skip:** Yes (`--skip-documentation`) or auto-skipped if TPDG not available

**TPDG Command Format:**
```bash
test-plan-documentation-generator \
    --input container.yaml \
    --output report.adoc \
    --format asciidoc \
    --test-case original.yaml
```

### Stage 7: Consolidated Documentation ✓

**Purpose:** Generate unified documentation combining all test results in a single report

**Actions:**
- Runs verifier in `--folder` mode to process all execution logs
- Creates consolidated container YAML (`all_tests_container.yaml`)
- Generates comprehensive AsciiDoc report (`all_tests.adoc`)
- Generates comprehensive Markdown report (`all_tests.md`)
- Outputs to `reports/consolidated/` directory
- Provides high-level overview of entire test suite

**Input:** All execution logs from Stage 3
**Output:** 
- `reports/consolidated/all_tests_container.yaml` - Unified container with all test results
- `reports/consolidated/all_tests.adoc` - Comprehensive AsciiDoc report
- `reports/consolidated/all_tests.md` - Comprehensive Markdown report

**Can Skip:** Auto-skipped if Stage 6 is skipped or TPDG not available

**Verifier Folder Mode Command:**
```bash
verifier --folder \
    --title "Acceptance Test Suite - All Tests" \
    --project "Test Case Manager - Acceptance Suite" \
    --environment "Automated Test Environment - [hostname]" \
    test-acceptance/test_cases/ \
    test-acceptance/execution_logs/
```

**TPDG Consolidated Command:**
```bash
test-plan-documentation-generator \
    --input reports/consolidated/all_tests_container.yaml \
    --output reports/consolidated/all_tests.adoc \
    --format asciidoc
```

---

## Individual vs. Consolidated Documentation

The acceptance suite generates two types of documentation to serve different purposes:

### Individual Documentation (Stage 6)

**Purpose:** Detailed per-test analysis and debugging

**Generated Files:**
- `reports/asciidoc/TC_*_container.adoc` - One file per test
- `reports/markdown/TC_*_container.md` - One file per test

**Use Cases:**
- Debugging individual test failures
- Reviewing specific test execution details
- Understanding single test behavior
- Detailed step-by-step analysis
- Test case development and validation

**Content:**
- Single test case metadata
- Full execution trace for all sequences and steps
- Captured variables and outputs
- Verification results
- Error messages and diagnostics

**When to Use:**
- Investigating why a specific test failed
- Reviewing test case implementation details
- Creating test case documentation
- Sharing individual test results

### Consolidated Documentation (Stage 7)

**Purpose:** High-level suite overview and comprehensive reporting

**Generated Files:**
- `reports/consolidated/all_tests_container.yaml` - Unified container
- `reports/consolidated/all_tests.adoc` - Suite-wide AsciiDoc report
- `reports/consolidated/all_tests.md` - Suite-wide Markdown report

**Use Cases:**
- Suite-wide status overview
- Executive summaries and reporting
- Pass/fail statistics across all tests
- Identifying patterns in test results
- Release validation documentation
- CI/CD pipeline reporting

**Content:**
- Aggregated test suite metadata
- Summary statistics (pass/fail counts by category)
- High-level execution results
- Suite-wide verification status
- Cross-test trends and patterns

**When to Use:**
- Generating suite-level reports for stakeholders
- Analyzing overall test coverage
- Release go/no-go decisions
- CI/CD dashboard integration
- Quality metrics and trends

### Key Differences

| Aspect | Individual (Stage 6) | Consolidated (Stage 7) |
|--------|---------------------|----------------------|
| Granularity | Per-test detail | Suite-wide overview |
| File Count | 91 files (one per test) | 3 files (container + 2 formats) |
| Detail Level | Complete step traces | Summary statistics |
| Use Case | Debugging, development | Reporting, metrics |
| Output Directory | `reports/asciidoc/`, `reports/markdown/` | `reports/consolidated/` |
| Generation | Loop over containers | Single verifier --folder call |
| Target Audience | Developers, QA engineers | Managers, stakeholders |

### Example Scenarios

**Scenario 1: Test Development**
- **Use Individual Documentation**
- You're writing a new test case and need to verify each step executes correctly
- Review `reports/asciidoc/TC_NEW_TEST_001_container.adoc` for detailed execution trace

**Scenario 2: CI/CD Pipeline Report**
- **Use Consolidated Documentation**
- Your CI pipeline completes and needs to generate a summary report
- Use `reports/consolidated/all_tests.md` for the build report artifact

**Scenario 3: Debugging Failure**
- **Use Individual Documentation**
- Test `TC_FAILURE_EXPECTED_003` failed unexpectedly
- Open `reports/markdown/TC_FAILURE_EXPECTED_003_container.md` to see exact command output and error

**Scenario 4: Release Validation**
- **Use Consolidated Documentation**
- Preparing for a release and need overall test status
- Review `reports/consolidated/all_tests.adoc` for comprehensive pass/fail statistics

---

## TPDG Integration

### What is TPDG?

**TPDG** (test-plan-documentation-generator) is a Rust-based tool that generates comprehensive documentation from test case definitions and execution results. It produces AsciiDoc, Markdown, and HTML reports.

### Installation

#### Option 1: Global Installation (Recommended)

```bash
cargo install test-plan-documentation-generator
```

This makes TPDG available system-wide as `test-plan-documentation-generator`.

#### Option 2: Custom Path

If you have a custom build or alternate location:

```bash
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator
```

The acceptance suite will use this path if set.

### Verification

Check if TPDG is installed and accessible:

```bash
which test-plan-documentation-generator
# or
test-plan-documentation-generator --version
```

### Setup Instructions

1. **Install TPDG:**
   ```bash
   cargo install test-plan-documentation-generator
   ```

2. **Verify installation:**
   ```bash
   test-plan-documentation-generator --version
   ```

3. **Run acceptance suite:**
   ```bash
   make acceptance-test
   ```

4. **Check documentation output:**
   ```bash
   ls test-acceptance/reports/asciidoc/
   ls test-acceptance/reports/markdown/
   ```

### TPDG Features Used

- **Multiple Formats**: AsciiDoc, Markdown, HTML
- **Metadata Integration**: Test titles, projects, environments, dates
- **Test Case Context**: Includes original test case YAML
- **Execution Results**: Captures step-by-step results
- **Schema Validation**: Ensures container compatibility

### What Happens Without TPDG?

If TPDG is not installed:
- Stages 1-5 execute normally
- Stage 6 (Documentation) is skipped with a warning
- Summary report notes TPDG unavailability
- No documentation is generated

**Warning Message:**
```
[WARNING] TPDG not found - skipping documentation generation
[INFO] Install TPDG: cargo install test-plan-documentation-generator
```

---

## Adding New Test Cases

### Step-by-Step Guide

1. **Choose appropriate category** based on test purpose:
   - `success/` - Successful execution scenarios
   - `failure/` - Failure and error handling
   - `hooks/` - Lifecycle hook tests
   - `manual/` - Manual interaction tests
   - `variables/` - Variable capture/usage
   - `dependencies/` - Dependency management
   - `complex/` - Complex integration tests
   - `bash_commands/` - Bash command scenarios

2. **Create test case YAML** following the schema:
   ```yaml
   test_case_id: TC_CATEGORY_NAME_001
   title: "Descriptive Test Title"
   description: "Detailed description of what this test validates"
   
   hydration_vars:
     required: []
     optional: []
   
   sequences:
     - sequence_id: "1"
       name: "Test Sequence"
       steps:
         - step_number: 1
           description: "Step description"
           command: "echo 'test'"
           capture_output:
             variable_name: "output_var"
           verification_expression: "output_var == 'test'"
   ```

3. **Follow naming conventions:**
   - File: `TC_CATEGORY_NAME_NNN.yaml`
   - ID: Match filename
   - Category: One of the 8 standard categories
   - Number: Sequential (001, 002, etc.)

4. **Ensure cross-platform compatibility:**
   - Use portable bash 3.2+ syntax
   - Avoid GNU-specific flags
   - Test on both macOS and Linux if possible
   - Use BSD/GNU compatible command options

5. **Add comprehensive verification:**
   - Include `verification_expression` for key steps
   - Use conditional logic (if/then/else) when appropriate
   - Validate captured variables
   - Check exit codes and output

6. **Document the test case:**
   - Add description to YAML
   - Update `test_cases/README.md` if adding new category
   - Include comments for complex scenarios

7. **Validate the test case:**
   ```bash
   cargo run --bin verifier -- test-acceptance/test_cases/category/TC_NEW_TEST_001.yaml
   ```

8. **Generate and test the script:**
   ```bash
   cargo run -- test-acceptance/test_cases/category/TC_NEW_TEST_001.yaml
   ./test-acceptance/scripts/TC_NEW_TEST_001.sh
   ```

9. **Run the acceptance suite:**
   ```bash
   make acceptance-test
   ```

### Schema Reference

Test cases must conform to `schemas/test-case.schema.json`. Key requirements:

- **test_case_id**: Required, unique identifier
- **title**: Required, descriptive title
- **description**: Optional, detailed description
- **sequences**: Required, array of test sequences
- **steps**: Required within sequences
- **command**: Required for each step
- **verification_expression**: Optional but recommended

### Best Practices

- **Start simple**: Begin with a basic test, then add complexity
- **Test one thing**: Each test should validate a specific feature
- **Use descriptive names**: Make test purpose clear from ID and title
- **Add verification**: Always verify command output and behavior
- **Handle errors**: Include expected failure scenarios
- **Document assumptions**: Note any prerequisites or dependencies
- **Keep portable**: Avoid platform-specific features

### Testing Your New Test Case

```bash
# Validate YAML
cargo run --bin verifier -- test-acceptance/test_cases/category/TC_NEW_TEST_001.yaml

# Generate script
cargo run -- test-acceptance/test_cases/category/TC_NEW_TEST_001.yaml

# Verify script syntax
bash -n test-acceptance/scripts/TC_NEW_TEST_001.sh

# Execute test
./test-acceptance/scripts/TC_NEW_TEST_001.sh

# Run full suite
make acceptance-test
```

---

## Output and Report Formats

### Console Output

Real-time console output with color-coded indicators:

```
=== Stage 1: Validating Test Case YAMLs ===
[INFO] Found 91 test case YAML files

✓ TC_SUCCESS_SIMPLE_001.yaml
✓ TC_SUCCESS_MULTI_SEQ_001.yaml
✗ TC_INVALID_TEST_001.yaml

[INFO] Validation: 90 passed, 1 failed

=== Stage 2: Generating Bash Scripts ===
[INFO] Generating scripts for 90 test cases

✓ TC_SUCCESS_SIMPLE_001.sh
✓ TC_SUCCESS_MULTI_SEQ_001.sh

[INFO] Generation: 90 passed, 0 failed
```

**Indicators:**
- ✓ (green) - Success
- ✗ (red) - Failure
- ℹ (blue) - Information
- ⚠ (yellow) - Warning

### Execution Log

Detailed execution log saved to `reports/acceptance_suite_execution.log`:

```
2024-03-17 14:30:00 [INFO] Starting acceptance test suite
2024-03-17 14:30:01 [INFO] Stage 1: Validating YAMLs
2024-03-17 14:30:05 [INFO] Stage 1 complete: 90/90 passed
2024-03-17 14:30:05 [INFO] Stage 2: Generating scripts
...
```

### Summary Report

Comprehensive summary in `reports/acceptance_suite_summary.txt`:

```
=========================================
Acceptance Test Suite Execution Summary
=========================================

Execution Date: 2024-03-17 14:30:00
Total Test Cases: 91

--- Stage 1: YAML Validation ---
Passed:  90
Failed:  1

Failed validations:
test-acceptance/test_cases/invalid/TC_INVALID_TEST_001.yaml

--- Stage 2: Script Generation ---
Passed:  90
Failed:  0

--- Stage 3: Test Execution ---
Passed:  72
Failed:  9
Skipped: 9 (manual tests)

Failed executions:
test-acceptance/scripts/TC_FAILURE_EXPECTED_001.sh (expected)
...

--- Stage 4: Verification ---
Passed:  81
Failed:  0

--- Stage 5: Container Validation ---
Passed:  81
Failed:  0

--- Stage 6: Individual Documentation ---
Passed:  81
Failed:  0

--- Stage 7: Consolidated Documentation ---
Passed:  1
Failed:  0

Generated files:
reports/consolidated/all_tests_container.yaml
reports/consolidated/all_tests.adoc
reports/consolidated/all_tests.md

=========================================
Overall Result:
SUCCESS - All stages completed
=========================================
```

### JSON Execution Logs

Structured JSON logs in `execution_logs/`:

```json
{
  "test_case_id": "TC_SUCCESS_SIMPLE_001",
  "execution_date": "2024-03-17T14:30:00Z",
  "sequences": [
    {
      "sequence_id": "1",
      "sequence_name": "Basic Test",
      "steps": [
        {
          "step_number": 1,
          "description": "Echo test",
          "command": "echo 'Hello World'",
          "output": "Hello World",
          "exit_code": 0,
          "timestamp": "2024-03-17T14:30:01Z"
        }
      ]
    }
  ]
}
```

### Container YAML Files

Container YAMLs in `verification_results/`:

```yaml
metadata:
  title: "TC_SUCCESS_SIMPLE_001"
  project: "Test Case Manager - Acceptance Suite"
  environment: "Automated Test Environment - MacBook-Pro"
  test_date: "2024-03-17T14:30:00Z"
  
test_case:
  test_case_id: "TC_SUCCESS_SIMPLE_001"
  title: "Simple Success Test"
  description: "Basic test demonstrating successful execution"
  sequences:
    # Full test case YAML content
    
results:
  sequences:
    - sequence_id: "1"
      status: "passed"
      steps:
        - step_number: 1
          status: "passed"
          output: "Hello World"
          exit_code: 0
```

### AsciiDoc Reports

Formatted documentation in `reports/asciidoc/`:

```asciidoc
= TC_SUCCESS_SIMPLE_001: Simple Success Test
Test Case Manager - Acceptance Suite
2024-03-17

== Test Information
Project:: Test Case Manager - Acceptance Suite
Environment:: Automated Test Environment - MacBook-Pro
Test Date:: 2024-03-17T14:30:00Z

== Test Description
Basic test demonstrating successful execution

== Test Sequences
=== Sequence 1: Basic Test
Status: PASSED

==== Step 1: Echo test
Command: `echo 'Hello World'`
Output:
----
Hello World
----
Exit Code: 0
Status: PASSED
```

### Markdown Reports

GitHub-compatible Markdown in `reports/markdown/`:

```markdown
# TC_SUCCESS_SIMPLE_001: Simple Success Test

**Project:** Test Case Manager - Acceptance Suite  
**Environment:** Automated Test Environment - MacBook-Pro  
**Test Date:** 2024-03-17T14:30:00Z

## Test Description

Basic test demonstrating successful execution

## Test Results

### Sequence 1: Basic Test

**Status:** PASSED

#### Step 1: Echo test

**Command:** `echo 'Hello World'`

**Output:**
```
Hello World
```

**Exit Code:** 0  
**Status:** PASSED
```

---

## Troubleshooting

### TPDG Not Found

**Problem:** Documentation generation fails with "TPDG not found" error

**Symptoms:**
```
[ERROR] TPDG binary not found
[WARNING] Skipping documentation generation stage
```

**Solutions:**

1. **Install TPDG globally:**
   ```bash
   cargo install test-plan-documentation-generator
   ```

2. **Set custom path:**
   ```bash
   export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator
   ```

3. **Verify installation:**
   ```bash
   which test-plan-documentation-generator
   test-plan-documentation-generator --version
   ```

4. **Check PATH:**
   ```bash
   echo $PATH
   # Ensure cargo bin directory is in PATH
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

### Schema Validation Failures

**Problem:** Test case YAML fails schema validation

**Symptoms:**
```
✗ TC_NEW_TEST_001.yaml
[ERROR] Schema validation failed: missing required field 'test_case_id'
```

**Solutions:**

1. **Check schema requirements:**
   ```bash
   cat schemas/test-case.schema.json
   ```

2. **Common issues:**
   - Missing required fields: `test_case_id`, `title`, `sequences`
   - Invalid field types: strings vs. numbers
   - Incorrect structure: steps not in sequences
   - Typos in field names

3. **Validate manually:**
   ```bash
   cargo run --bin verifier -- test-acceptance/test_cases/category/TC_NEW_TEST_001.yaml
   ```

4. **Use verbose mode:**
   ```bash
   cargo run --bin verifier -- --verbose test-acceptance/test_cases/category/TC_NEW_TEST_001.yaml
   ```

5. **Check examples:**
   Look at existing test cases in `test_cases/success/` for correct structure

### Script Execution Timeouts

**Problem:** Test scripts hang or timeout during execution

**Symptoms:**
```
[WARNING] Script execution exceeded timeout: TC_LONG_TEST_001.sh
```

**Solutions:**

1. **Identify long-running commands:**
   - Check for infinite loops
   - Look for blocking I/O operations
   - Verify external dependencies are available

2. **Add timeouts to commands:**
   ```bash
   # Use timeout command (GNU/BSD compatible)
   timeout 30 long_running_command
   ```

3. **Debug script manually:**
   ```bash
   bash -x test-acceptance/scripts/TC_LONG_TEST_001.sh
   ```

4. **Check for missing dependencies:**
   - Verify all required tools are installed
   - Check environment variables are set
   - Ensure prerequisite steps completed

5. **Review manual tests:**
   - Manual tests may wait for user input
   - Ensure `--include-manual` is intentional
   - Check test is not flagged as manual incorrectly

### Binary Not Found Errors

**Problem:** Required binaries are missing

**Symptoms:**
```
[ERROR] Required binary not found: validate-yaml
[ERROR] Please build binaries before running suite
```

**Solutions:**

1. **Build all binaries:**
   ```bash
   cargo build --bin validate-yaml
   cargo build --bin test-executor
   cargo build --bin verifier
   cargo build --bin validate-json
   ```

2. **Or use make:**
   ```bash
   make build
   ```

3. **Check binary paths:**
   ```bash
   ls -la target/debug/validate-yaml
   ls -la target/debug/test-executor
   ls -la target/debug/verifier
   ls -la target/debug/validate-json
   ```

### Invalid JSON in Execution Log

**Problem:** Execution log contains invalid JSON

**Symptoms:**
```
[ERROR] Invalid JSON in execution log: TC_TEST_001.json
[ERROR] Parse error at line 15: unexpected end of input
```

**Solutions:**

1. **Check generated script:**
   ```bash
   cat test-acceptance/scripts/TC_TEST_001.sh
   ```

2. **Look for JSON corruption:**
   - Unescaped quotes in output
   - Missing commas or braces
   - Invalid UTF-8 characters

3. **Run script manually:**
   ```bash
   ./test-acceptance/scripts/TC_TEST_001.sh > manual_output.json
   cat manual_output.json | jq .
   ```

4. **Validate JSON:**
   ```bash
   cargo run --bin validate-json -- test-acceptance/execution_logs/TC_TEST_001.json
   ```

5. **Check for command output issues:**
   - Binary output may corrupt JSON
   - Special characters need escaping
   - Verify json-escape utility usage

### Container Validation Failures

**Problem:** Container YAML fails schema validation

**Symptoms:**
```
✗ TC_TEST_001_container.yaml
[ERROR] Container schema validation failed
```

**Solutions:**

1. **Check container schema:**
   ```bash
   cat data/testcase_results_container/schema.json
   ```

2. **Validate container manually:**
   ```bash
   cargo run --bin test-plan-documentation-generator-compat -- validate \
     test-acceptance/verification_results/TC_TEST_001_container.yaml
   ```

3. **Review verifier output:**
   - Check metadata fields are present
   - Verify test_case section is complete
   - Ensure results section is structured correctly

4. **Use verbose verification:**
   ```bash
   cargo run --bin verifier -- --verbose \
     --title "Test" \
     --project "Project" \
     --environment "Env" \
     test-acceptance/test_cases/category/TC_TEST_001.yaml \
     test-acceptance/execution_logs/TC_TEST_001.json
   ```

### Permission Denied Errors

**Problem:** Cannot execute generated scripts

**Symptoms:**
```
[ERROR] Permission denied: ./test-acceptance/scripts/TC_TEST_001.sh
```

**Solutions:**

1. **Make scripts executable:**
   ```bash
   chmod +x test-acceptance/scripts/*.sh
   ```

2. **Check file permissions:**
   ```bash
   ls -la test-acceptance/scripts/TC_TEST_001.sh
   ```

3. **Run with bash explicitly:**
   ```bash
   bash test-acceptance/scripts/TC_TEST_001.sh
   ```

### Platform Compatibility Issues

**Problem:** Tests fail on macOS or Linux due to platform differences

**Symptoms:**
```
[ERROR] Command not found: sed -r
[ERROR] Invalid option: --format=long
```

**Solutions:**

1. **Use portable syntax:**
   - Replace `sed -r` with `sed -E` (BSD/GNU compatible)
   - Avoid GNU-specific flags
   - Check AGENTS.md for compatibility guidelines

2. **Test on both platforms:**
   ```bash
   # macOS (BSD)
   uname -s
   # Linux (GNU)
   uname -s
   ```

3. **Use conditional logic:**
   ```bash
   if [[ "$(uname -s)" == "Darwin" ]]; then
     # macOS-specific command
   else
     # Linux-specific command
   fi
   ```

4. **Follow bash 3.2+ requirements:**
   - No associative arrays (`declare -A`)
   - No `[[...]]` bash 4.0+ features
   - POSIX-compliant constructs

---

## CI/CD Integration

### Overview

The acceptance test suite integrates seamlessly with CI/CD pipelines to ensure code quality and prevent regressions.

### Prerequisites for CI/CD

1. Rust toolchain installed
2. Cargo available
3. Build tools (make, bash)
4. TPDG installed (optional but recommended)

### GitHub Actions Integration

```yaml
name: Acceptance Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  acceptance:
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install TPDG
        run: cargo install test-plan-documentation-generator
      
      - name: Build project
        run: make build
      
      - name: Run acceptance tests
        run: make acceptance-test
      
      - name: Upload reports
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: acceptance-reports
          path: test-acceptance/reports/
      
      - name: Upload execution logs
        uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: execution-logs
          path: test-acceptance/execution_logs/
```

### GitLab CI Integration

```yaml
acceptance_tests:
  stage: test
  
  image: rust:latest
  
  before_script:
    - cargo --version
    - cargo install test-plan-documentation-generator
  
  script:
    - make build
    - make acceptance-test
  
  artifacts:
    when: always
    paths:
      - test-acceptance/reports/
    expire_in: 1 week
  
  artifacts:
    when: on_failure
    paths:
      - test-acceptance/execution_logs/
      - test-acceptance/verification_results/
    expire_in: 1 week
```

### Jenkins Integration

```groovy
pipeline {
    agent any
    
    stages {
        stage('Setup') {
            steps {
                sh 'cargo --version'
                sh 'cargo install test-plan-documentation-generator || true'
            }
        }
        
        stage('Build') {
            steps {
                sh 'make build'
            }
        }
        
        stage('Acceptance Tests') {
            steps {
                sh 'make acceptance-test'
            }
        }
    }
    
    post {
        always {
            archiveArtifacts artifacts: 'test-acceptance/reports/**/*', 
                           allowEmptyArchive: true
        }
        
        failure {
            archiveArtifacts artifacts: 'test-acceptance/execution_logs/**/*', 
                           allowEmptyArchive: true
        }
    }
}
```

### CircleCI Integration

```yaml
version: 2.1

jobs:
  acceptance:
    docker:
      - image: rust:latest
    
    steps:
      - checkout
      
      - run:
          name: Install TPDG
          command: cargo install test-plan-documentation-generator
      
      - run:
          name: Build project
          command: make build
      
      - run:
          name: Run acceptance tests
          command: make acceptance-test
      
      - store_artifacts:
          path: test-acceptance/reports
          destination: acceptance-reports
      
      - store_artifacts:
          path: test-acceptance/execution_logs
          destination: execution-logs
          when: on_fail

workflows:
  version: 2
  test:
    jobs:
      - acceptance
```

### Pre-commit Hook Integration

Add to `.git/hooks/pre-commit`:

```bash
#!/usr/bin/env bash
set -e

echo "Running acceptance tests..."
make acceptance-test

if [ $? -ne 0 ]; then
    echo "Acceptance tests failed. Commit aborted."
    exit 1
fi

echo "Acceptance tests passed."
```

### Best Practices for CI/CD

1. **Cache dependencies:**
   - Cache Cargo registry and build artifacts
   - Speeds up subsequent runs
   - Reduces network usage

2. **Fail fast:**
   - Exit on first failure
   - Save CI/CD resources
   - Provide quick feedback

3. **Upload artifacts:**
   - Always upload reports (success or failure)
   - Upload logs on failure for debugging
   - Set appropriate expiration times

4. **Parallel execution:**
   - Run acceptance tests in parallel with unit tests
   - Use matrix builds for multiple platforms
   - Balance resource usage

5. **Conditional documentation:**
   - Skip documentation in PR builds
   - Generate full docs on main branch only
   - Use `--skip-documentation` flag for speed

6. **Status badges:**
   - Add CI status badges to README
   - Show acceptance test status
   - Link to latest reports

### Example: Quick Validation in CI

For faster PR validation:

```bash
# Skip documentation generation
make acceptance-test SKIP_DOCS=1

# Or use script directly
cd test-acceptance
./run_acceptance_suite.sh --skip-documentation
```

### Example: Full Validation for Releases

For release branches:

```bash
# Include everything, including manual tests
cd test-acceptance
./run_acceptance_suite.sh --include-manual --verbose
```

---

## Additional Resources

- **[ACCEPTANCE_SUITE.md](ACCEPTANCE_SUITE.md)** - Detailed orchestrator documentation
- **[test_cases/README.md](test_cases/README.md)** - Test case catalog
- **[QUICKSTART.md](QUICKSTART.md)** - Quick start guide
- **[WORKFLOW.md](WORKFLOW.md)** - Workflow documentation
- **[../AGENTS.md](../AGENTS.md)** - Project commands and guidelines
- **[../docs/report_generation.md](../docs/report_generation.md)** - TPDG documentation
- **[../schemas/test-case.schema.json](../schemas/test-case.schema.json)** - Test case schema
- **[../data/testcase_results_container/schema.json](../data/testcase_results_container/schema.json)** - Container schema

---

## Support

For issues or questions:

1. Check this README and related documentation
2. Review test case examples in `test_cases/`
3. Run with `--verbose` for detailed output
4. Check the summary report in `reports/acceptance_suite_summary.txt`
5. Validate individual components using stage scripts
6. Review schema definitions for structure requirements

---

## Contributing

When contributing to the acceptance test suite:

1. Follow the test case addition guidelines
2. Ensure cross-platform compatibility (macOS/Linux)
3. Use portable bash 3.2+ syntax
4. Add comprehensive verification expressions
5. Document test purpose and features
6. Run full acceptance suite before committing
7. Update this README if adding new categories or features

---

**Last Updated:** 2024-03-17  
**Test Suite Version:** 91 test cases across 8 categories  
**Pipeline Stages:** 7 (validation, generation, execution, verification, container validation, individual documentation, consolidated documentation)
