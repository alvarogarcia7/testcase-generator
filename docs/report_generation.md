# Report Generation Documentation

## Overview

The Test Case Manager includes a comprehensive report generation system that converts test execution logs into professional documentation reports. The system integrates with `test-plan-doc-gen`, an external Rust-based tool that generates reports in multiple formats (AsciiDoc, Markdown, PDF) from test case YAML files and verification results.

## Table of Contents

1. [Installation and Building test-plan-doc-gen](#installation-and-building-test-plan-doc-gen)
2. [Directory Structure](#directory-structure)
3. [Running Report Generation](#running-report-generation)
4. [Report Output Formats and Locations](#report-output-formats-and-locations)
5. [Customizing Templates](#customizing-templates)
6. [Troubleshooting](#troubleshooting)

---

## Installation and Building test-plan-doc-gen

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Cargo package manager
- Git

### Installation Steps

#### 1. Clone the test-plan-doc-gen Repository

The `test-plan-doc-gen` tool should be cloned as a sibling directory to the Test Case Manager project:

```bash
# Navigate to the parent directory
cd /path/to/parent-directory

# Clone test-plan-doc-gen (replace with actual repository URL)
git clone <test-plan-doc-gen-repo-url> test-plan-doc-gen

# Verify directory structure
ls -la
# Expected output:
#   testcase-manager/
#   test-plan-doc-gen/
```

#### 2. Build test-plan-doc-gen

```bash
# Navigate to test-plan-doc-gen directory
cd test-plan-doc-gen

# Build release binary
cargo build --release

# Verify binary was created
ls -la target/release/test-plan-doc-gen
```

The binary will be located at: `test-plan-doc-gen/target/release/test-plan-doc-gen`

#### 3. Alternative: Install to System PATH

```bash
# Install test-plan-doc-gen globally
cd test-plan-doc-gen
cargo install --path .

# Verify installation
which test-plan-doc-gen
test-plan-doc-gen --version
```

### Automated Building

The report generation scripts automatically build `test-plan-doc-gen` if the binary is not found:

```bash
# The script will check for and build test-plan-doc-gen automatically
cd testcase-manager
./scripts/generate_documentation_reports.sh
```

---

## Directory Structure

### Templates and Schemas

The project uses the following directory structure for templates, schemas, and reports:

```
testcase-manager/
├── schemas/                                   # JSON Schema files for validation
│   ├── test-case.schema.json                 # Test case YAML schema
│   ├── execution-log.schema.json             # Execution log JSON schema
│   ├── verification-result.schema.json       # Verification result schema
│   ├── verification-output.schema.json       # Verification output schema
│   └── README.md                             # Schema documentation
│
├── testcases/                                # Test case definitions
│   ├── expected_output_reports/              # Template examples
│   │   ├── container_data.yml                # Container template with metadata
│   │   ├── sample_gsma_4.4.2.2_TC.yml       # Test case template
│   │   ├── sample_gsma_4.4.2.3_TC.yml       # Test case template
│   │   ├── sample_gsma_4.4.2.4_AN.yml       # Analysis template
│   │   ├── sample_gsma_4.4.2.5_DM.yml       # Demonstration template
│   │   └── sample_gsma_4.4.2.6_IN.yml       # Inspection template
│   │
│   └── verifier_scenarios/                   # Test execution logs
│       └── *.log                             # Execution log files
│
├── reports/                                  # Generated reports output
│   └── documentation/                        # Documentation reports
│       ├── verification/                     # Verification JSON
│       │   └── batch_verification.json       # Verifier output
│       ├── results/                          # Result YAML files
│       │   ├── *_result.yaml                 # Individual test results
│       │   └── results_container.yaml        # Combined results container
│       └── reports/                          # Final documentation
│           ├── test_results_report.adoc      # AsciiDoc results report
│           └── *_test_plan.md                # Markdown test plans
│
├── scripts/                                  # Automation scripts
│   ├── lib/                                  # Script libraries
│   │   ├── logger.sh                         # Logging library
│   │   └── report_generator.sh               # Report generation library
│   ├── generate_documentation_reports.sh     # Main orchestration script
│   └── convert_verification_to_result_yaml.py # JSON to YAML converter
│
└── ../test-plan-doc-gen/                     # Sibling directory
    └── target/release/test-plan-doc-gen      # Report generator binary
```

### Key Directories

#### `schemas/`
Contains JSON Schema files used to validate test cases, execution logs, and verification results. See `schemas/README.md` for detailed schema documentation.

#### `testcases/expected_output_reports/`
Contains template examples demonstrating the structure for container files and test case definitions used by `test-plan-doc-gen`.

**Key Template: `container_data.yml`**
- Defines metadata structure (title, project, test_date)
- Contains test_results array with verification data
- Includes metadata section with execution statistics

#### `reports/documentation/`
Default output directory for all generated reports. Organized into subdirectories:
- `verification/` - Raw verification JSON from verifier tool
- `results/` - Individual and combined YAML result files
- `reports/` - Final AsciiDoc and Markdown documentation

#### `scripts/lib/`
**`report_generator.sh`** - Library functions for:
- Building test-plan-doc-gen binary
- Finding and invoking test-plan-doc-gen
- Validating report outputs

**`logger.sh`** - Centralized logging library for consistent output formatting

---

## Running Report Generation

### Using the Makefile (Recommended)

The simplest way to generate reports is using the Makefile targets:

```bash
# Generate documentation reports for verifier_scenarios
make generate-docs

# Generate documentation reports for all testcases
make generate-docs-all
```

### Manual Execution

#### Full Pipeline (Recommended)

Run the complete end-to-end report generation pipeline:

```bash
./scripts/generate_documentation_reports.sh
```

This script performs all steps:
1. Run verifier on execution logs
2. Convert verification JSON to result YAML files
3. Build test-plan-doc-gen if needed
4. Generate test results report (AsciiDoc)
5. Generate test plan reports (Markdown)

#### Custom Options

```bash
# Custom directories
./scripts/generate_documentation_reports.sh \
  --logs-dir testcases/custom_logs \
  --test-case-dir testcases/custom_tests \
  --output-dir reports/custom_output \
  --test-plan-doc-gen ../test-plan-doc-gen

# Custom container template
./scripts/generate_documentation_reports.sh \
  --container-template testcases/custom_template.yml
```

#### Help and Options

```bash
./scripts/generate_documentation_reports.sh --help
```

**Available Options:**
- `--logs-dir DIR` - Directory containing execution logs (default: `testcases/verifier_scenarios`)
- `--test-case-dir DIR` - Directory containing test case YAML files (default: `testcases`)
- `--output-dir DIR` - Output directory for reports (default: `reports/documentation`)
- `--test-plan-doc-gen DIR` - Path to test-plan-doc-gen sibling directory (default: `../test-plan-doc-gen`)
- `--container-template` - Path to container template YAML (default: `testcases/expected_output_reports/container_data.yml`)
- `--help` - Show help message

### Step-by-Step Manual Generation

You can also run each step individually:

#### Step 1: Run Verifier on Execution Logs

```bash
# Build verifier if needed
cargo build --release --bin verifier

# Run verifier in folder mode
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format json \
  --output reports/documentation/verification/batch_verification.json \
  --test-case-dir testcases
```

#### Step 2: Convert Verification JSON to Result YAML

```bash
# Convert verification JSON to individual result YAML files
python3 scripts/convert_verification_to_result_yaml.py \
  reports/documentation/verification/batch_verification.json \
  -o reports/documentation/results \
  -v
```

This creates individual `*_result.yaml` files with the structure:

```yaml
type: result
test_case_id: "TC_001"
description: "Test description"
sequences:
  - sequence_id: 1
    name: "Test Sequence"
    step_results:
      - Pass:
          step: 1
          description: "Step description"
total_steps: 1
passed_steps: 1
failed_steps: 0
not_executed_steps: 0
overall_pass: true
```

#### Step 3: Create Results Container

```bash
# Create container YAML with all results
cat > reports/documentation/results/results_container.yaml << 'EOF'
title: 'Test Execution Results Report'
project: 'Test Case Manager - Verification Results'
test_date: '2024-01-01T00:00:00Z'
test_results:
EOF

# Append result files (without 'type: result' line)
for result_file in reports/documentation/results/*_result.yaml; do
    sed '/^type: result/d' "$result_file" | sed 's/^/  /' >> reports/documentation/results/results_container.yaml
done

# Add metadata
cat >> reports/documentation/results/results_container.yaml << 'EOF'
metadata:
  environment: 'Test Environment'
  platform: 'Test Case Manager'
  executor: 'Automated Test Framework'
  execution_duration: 0.0
  total_test_cases: 5
  passed_test_cases: 3
  failed_test_cases: 2
EOF
```

#### Step 4: Generate Test Results Report (AsciiDoc)

```bash
# Set binary path
export TEST_PLAN_DOC_GEN=../test-plan-doc-gen/target/release/test-plan-doc-gen

# Generate AsciiDoc report
$TEST_PLAN_DOC_GEN \
  --container reports/documentation/results/results_container.yaml \
  --output reports/documentation/reports/test_results_report.adoc \
  --format asciidoc
```

#### Step 5: Generate Test Plan Reports (Markdown)

```bash
# Generate test plan for each test case
for test_case in testcases/*.yml; do
    basename=$(basename "$test_case" .yml)
    $TEST_PLAN_DOC_GEN \
      --test-case "$test_case" \
      --output "reports/documentation/reports/${basename}_test_plan.md" \
      --format markdown
done
```

### Using the Report Generator Library

For custom scripts, use the `report_generator.sh` library:

```bash
#!/usr/bin/env bash
set -e

# Source libraries
source scripts/lib/logger.sh
source scripts/lib/report_generator.sh

# Build test-plan-doc-gen
build_test_plan_doc_gen "../test-plan-doc-gen"

# Check if available
if check_test_plan_doc_gen_available "../test-plan-doc-gen"; then
    log_info "test-plan-doc-gen is available"
fi

# Generate report
invoke_test_plan_doc_gen \
    --test-case "testcases/example.yml" \
    --output "reports/example_report.md" \
    --format markdown

# Validate output
validate_report_output "reports" "example_report.md"
```

---

## Report Output Formats and Locations

### Output Formats

The report generation system produces multiple output formats:

#### 1. Verification JSON
**Location:** `reports/documentation/verification/batch_verification.json`

Raw verification results from the verifier tool in JSON format. Contains detailed pass/fail information for all test cases.

**Schema:** `schemas/verification-output.schema.json`

**Example Structure:**
```json
{
  "test_cases": [
    {
      "test_case_id": "TC_001",
      "description": "Example test case",
      "sequences": [...],
      "total_steps": 5,
      "passed_steps": 4,
      "failed_steps": 1,
      "not_executed_steps": 0,
      "overall_pass": false
    }
  ],
  "total_test_cases": 10,
  "passed_test_cases": 8,
  "failed_test_cases": 2
}
```

#### 2. Result YAML Files
**Location:** `reports/documentation/results/*_result.yaml`

Individual YAML files for each test case, containing verification results in a structured format.

**Required Field:** `type: result`

**Example:**
```yaml
type: result
test_case_id: "TC_001"
description: "Example test case"
requirement: "REQ_100"
item: 1
tc: 1
sequences:
  - sequence_id: 1
    name: "Test Sequence"
    step_results:
      - Pass:
          step: 1
          description: "Execute command"
      - Fail:
          step: 2
          description: "Check output"
          expected:
            success: true
            result: "0"
            output: "Success"
          actual_result: "1"
          actual_output: "Error"
          reason: "Exit code mismatch"
    all_steps_passed: false
total_steps: 2
passed_steps: 1
failed_steps: 1
not_executed_steps: 0
overall_pass: false
```

#### 3. Results Container YAML
**Location:** `reports/documentation/results/results_container.yaml`

Combined container file with all test results, metadata, and execution statistics. Used as input for AsciiDoc report generation.

**Structure:**
```yaml
title: 'Test Execution Results Report'
project: 'Project Name'
test_date: '2024-01-01T00:00:00Z'
test_results:
  - # Test case 1 (without 'type: result')
    test_case_id: "TC_001"
    # ... result data ...
  - # Test case 2
    test_case_id: "TC_002"
    # ... result data ...
metadata:
  environment: 'Test Environment'
  platform: 'Test Case Manager'
  executor: 'Automated Test Framework'
  execution_duration: 123.45
  total_test_cases: 10
  passed_test_cases: 8
  failed_test_cases: 2
```

#### 4. AsciiDoc Report
**Location:** `reports/documentation/reports/test_results_report.adoc`

Professional test results report in AsciiDoc format, generated from the results container. Can be converted to HTML or PDF using AsciiDoctor.

**Example Usage:**
```bash
# Convert AsciiDoc to HTML
asciidoctor reports/documentation/reports/test_results_report.adoc

# Convert AsciiDoc to PDF
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc
```

#### 5. Markdown Test Plans
**Location:** `reports/documentation/reports/*_test_plan.md`

Individual test plan documentation in Markdown format, generated from test case YAML files. Each file documents:
- Test case metadata (requirement, item, tc, id)
- Description and prerequisites
- Initial conditions (general and device-specific)
- Test sequences and steps
- Expected results and verification expressions

### Default Output Structure

After running `make generate-docs`, the output structure is:

```
reports/documentation/
├── verification/
│   └── batch_verification.json          # Raw verification JSON
├── results/
│   ├── TC_001_result.yaml              # Individual result files
│   ├── TC_002_result.yaml
│   ├── ...
│   └── results_container.yaml          # Combined container
└── reports/
    ├── test_results_report.adoc        # AsciiDoc results report
    ├── TC_001_test_plan.md             # Markdown test plans
    ├── TC_002_test_plan.md
    └── ...
```

### Accessing Reports

```bash
# View verification JSON
cat reports/documentation/verification/batch_verification.json | jq .

# View result YAML
cat reports/documentation/results/TC_001_result.yaml

# View AsciiDoc report (raw)
less reports/documentation/reports/test_results_report.adoc

# View Markdown test plan
cat reports/documentation/reports/TC_001_test_plan.md

# Convert AsciiDoc to HTML and open in browser
asciidoctor reports/documentation/reports/test_results_report.adoc
open reports/documentation/reports/test_results_report.html
```

---

## Customizing Templates

### Container Template Customization

The container template defines the structure and metadata for test results reports. You can customize it to match your organization's requirements.

#### Default Container Template

Location: `testcases/expected_output_reports/container_data.yml`

```yaml
title: 'GSMA eUICC Test Suite Results - Q1 2024'
project: 'GSMA SGP.22 Compliance Testing'
test_date: '2024-03-15T14:30:00Z'
test_results:
  - # Test results go here
metadata:
  environment: 'GSMA Certification Lab - Environment 2'
  platform: 'eUICC Test Platform v3.2.1'
  executor: 'Automated Test Framework v2.5.0'
  execution_duration: 3845.7
  total_test_cases: 5
  passed_test_cases: 3
  failed_test_cases: 2
```

#### Creating Custom Templates

**1. Basic Template**

```yaml
title: 'My Custom Test Report'
project: 'My Project Name'
test_date: '2024-01-15T10:00:00Z'
test_results:
  # Results will be injected here
metadata:
  environment: 'Development Environment'
  platform: 'Custom Test Platform'
  executor: 'CI/CD Pipeline'
  execution_duration: 0.0
  total_test_cases: 0
  passed_test_cases: 0
  failed_test_cases: 0
```

**2. Enterprise Template with Additional Metadata**

```yaml
title: 'Enterprise Compliance Testing Report'
project: 'Product XYZ - Certification'
version: '1.2.3'
test_date: '2024-01-15T10:00:00Z'
organization:
  name: 'ACME Corporation'
  department: 'Quality Assurance'
  contact: 'qa-team@acme.com'
regulatory_info:
  standard: 'ISO 9001:2015'
  certification_body: 'International Certification Agency'
test_results:
  # Results will be injected here
metadata:
  environment: 'Production-like Test Environment'
  platform: 'Enterprise Test Infrastructure v5.0'
  executor: 'Automated CI/CD Pipeline (Jenkins v2.400)'
  execution_duration: 0.0
  total_test_cases: 0
  passed_test_cases: 0
  failed_test_cases: 0
  build_number: '${BUILD_NUMBER}'
  git_commit: '${GIT_COMMIT}'
```

**3. Using Custom Template**

```bash
# Generate reports with custom template
./scripts/generate_documentation_reports.sh \
  --container-template /path/to/custom_template.yml
```

### Test Case Template Customization

Test case YAML files follow the schema defined in `schemas/test-case.schema.json`. Key customizable elements:

#### Test Case Metadata

```yaml
requirement: "REQ_100"      # Requirement identifier
item: 1                     # Item number
tc: 1                       # Test case number
id: "TC_001"               # Test case ID
description: "Test description"
```

#### Prerequisites

```yaml
prerequisites:
  - type: manual
    description: "Setup test environment manually"
  - type: automatic
    description: "Check SSH connectivity"
    verification_command: "ssh -q user@host exit"
```

#### Initial Conditions (BDD Style)

```yaml
general_initial_conditions:
  system:
    - "create directory \"/tmp/test\""
    - "set environment variable \"DEBUG\" to \"1\""

initial_conditions:
  device:
    - "ping device \"${TARGET_HOST}\" with 3 retries"
    - "wait until port 22 on \"${TARGET_HOST}\" is open with timeout 30 seconds"
```

#### Test Sequences and Steps

```yaml
test_sequences:
  - id: 1
    name: "Basic Functionality Test"
    description: "Verify basic operations"
    steps:
      - step: 1
        description: "Execute command"
        command: "echo 'Hello World'"
        expected:
          success: true
          result: 0
          output: "Hello World"
        verification:
          result: "[[ $EXIT_CODE -eq 0 ]]"
          output: "grep -q 'Hello World' <<< \"$COMMAND_OUTPUT\""
```

### Report Style Customization

While `test-plan-doc-gen` controls the final report styling, you can influence the output by:

#### 1. Using Custom CSS (AsciiDoc)

Create a custom CSS file and apply it during conversion:

```bash
# Generate AsciiDoc
asciidoctor \
  -a stylesheet=custom-style.css \
  reports/documentation/reports/test_results_report.adoc
```

#### 2. Using AsciiDoc Attributes

Add attributes to customize the output:

```bash
# Generate PDF with custom theme
asciidoctor-pdf \
  -a pdf-theme=custom-theme.yml \
  reports/documentation/reports/test_results_report.adoc
```

#### 3. Markdown Styling

For Markdown reports, use a Markdown processor with custom CSS:

```bash
# Convert Markdown to HTML with custom CSS
pandoc \
  reports/documentation/reports/TC_001_test_plan.md \
  -o TC_001_test_plan.html \
  --css=custom-style.css \
  --standalone
```

---

## Troubleshooting

### Common Issues and Solutions

#### 1. Missing test-plan-doc-gen Binary

**Problem:**
```
✗ Error: test-plan-doc-gen binary not found
Please build it first using build_test_plan_doc_gen()
```

**Solutions:**

**Option A: Clone and Build test-plan-doc-gen**
```bash
cd /path/to/parent-directory
git clone <test-plan-doc-gen-repo-url> test-plan-doc-gen
cd test-plan-doc-gen
cargo build --release
```

**Option B: Specify Custom Path**
```bash
./scripts/generate_documentation_reports.sh \
  --test-plan-doc-gen /custom/path/to/test-plan-doc-gen
```

**Option C: Install to System PATH**
```bash
cd test-plan-doc-gen
cargo install --path .
export TEST_PLAN_DOC_GEN=test-plan-doc-gen
```

#### 2. Schema Validation Failures

**Problem:**
```
✗ Error: Schema validation failed
Invalid test case structure
```

**Solutions:**

**Check Schema Compliance:**
```bash
# Validate test case against schema
cargo run --bin validate-yaml -- \
  --schema schemas/test-case.schema.json \
  testcases/problematic_test.yml
```

**Common Schema Issues:**
- Missing required fields (`requirement`, `id`, `description`)
- Invalid step structure (missing `step`, `description`, or `command`)
- Incorrect verification expression format
- Invalid initial conditions structure

**Fix Example:**
```yaml
# Before (invalid)
test_sequences:
  - name: "Test"
    steps:
      - description: "Step"  # Missing 'step' field

# After (valid)
test_sequences:
  - id: 1
    name: "Test"
    steps:
      - step: 1
        description: "Step"
        command: "echo 'test'"
```

#### 3. Verifier Failures

**Problem:**
```
✗ Verifier failed with exit code: 2
Verification output not generated
```

**Solutions:**

**Check Execution Logs:**
```bash
# Verify log files exist
ls -la testcases/verifier_scenarios/

# Check log format
cat testcases/verifier_scenarios/example.log
```

**Expected Log Format:**
```
[TEST_SEQUENCE=1] [STEP=1] [EXIT_CODE=0] [TIMESTAMP=2024-01-15T10:00:00Z]
Output from command
```

**Run Verifier Manually:**
```bash
cargo build --release --bin verifier
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format json \
  --output /tmp/verification.json \
  --test-case-dir testcases \
  --verbose
```

#### 4. Python Conversion Script Errors

**Problem:**
```
✗ Error: PyYAML is required
Install with: pip3 install pyyaml
```

**Solution:**
```bash
# Install PyYAML
pip3 install pyyaml

# Or use a virtual environment
python3 -m venv venv
source venv/bin/activate
pip install pyyaml
```

**Problem:**
```
✗ Error: Failed to parse JSON
```

**Solution:**
```bash
# Validate JSON format
cat reports/documentation/verification/batch_verification.json | jq .

# Check for common issues
# - Trailing commas
# - Missing quotes around keys
# - Invalid escape sequences
```

#### 5. Empty or Missing Reports

**Problem:**
```
No result files found to include in container
```

**Solutions:**

**Check Conversion Output:**
```bash
# Re-run conversion with verbose output
python3 scripts/convert_verification_to_result_yaml.py \
  reports/documentation/verification/batch_verification.json \
  -o reports/documentation/results \
  -v
```

**Verify Result Files:**
```bash
# Check if result files were created
ls -la reports/documentation/results/
find reports/documentation/results -name "*_result.yaml"
```

**Check Container Structure:**
```bash
# Verify container has test_results
grep -A 5 "test_results:" reports/documentation/results/results_container.yaml
```

#### 6. Cargo Build Failures

**Problem:**
```
✗ Failed to build test-plan-doc-gen
cargo: not found
```

**Solutions:**

**Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Update Rust:**
```bash
rustup update
```

**Check Dependencies:**
```bash
cd test-plan-doc-gen
cargo check
```

#### 7. Permission Issues

**Problem:**
```
Permission denied: reports/documentation/
```

**Solutions:**

**Fix Directory Permissions:**
```bash
# Create output directory with proper permissions
mkdir -p reports/documentation/{verification,results,reports}
chmod -R u+w reports/documentation
```

**Run with Proper User:**
```bash
# Check current user
whoami

# Change ownership if needed
sudo chown -R $USER:$USER reports/
```

#### 8. Test Case Discovery Issues

**Problem:**
```
Found 0 test case file(s)
No test case files found
```

**Solutions:**

**Check Test Case Directory:**
```bash
# List test case files
find testcases -name "*.yml" -o -name "*.yaml"

# Specify custom directory
./scripts/generate_documentation_reports.sh \
  --test-case-dir /path/to/test-cases
```

**Verify File Extensions:**
```bash
# The script looks for .yml and .yaml extensions
# Rename if needed
rename 's/\.yaml$/.yml/' testcases/*.yaml
```

### Debug Mode

Enable verbose logging for detailed troubleshooting:

```bash
# Set verbose mode
export VERBOSE=1

# Run report generation
./scripts/generate_documentation_reports.sh

# Or use verbose flag in custom scripts
VERBOSE=1 ./scripts/generate_documentation_reports.sh
```

### Logging and Output

The report generation system uses the centralized logging library (`scripts/lib/logger.sh`):

```bash
# View logs with colors
./scripts/generate_documentation_reports.sh 2>&1 | less -R

# Save logs to file
./scripts/generate_documentation_reports.sh 2>&1 | tee report_generation.log

# Filter for errors only
./scripts/generate_documentation_reports.sh 2>&1 | grep "✗"
```

### Getting Help

If you encounter issues not covered here:

1. **Check Schema Documentation:** `schemas/README.md`
2. **Review Script Help:** `./scripts/generate_documentation_reports.sh --help`
3. **Examine Examples:** `testcases/expected_output_reports/`
4. **Run Tests:** `make test` to ensure system is working correctly
5. **Check AGENTS.md:** For build, lint, and test commands

### Known Limitations

1. **test-plan-doc-gen Dependency:** The report generation system requires `test-plan-doc-gen` to be available as a sibling directory or in PATH.

2. **Container Template Structure:** The container template must follow the exact YAML structure expected by `test-plan-doc-gen`.

3. **Result YAML Format:** Result YAML files must include the `type: result` field to be valid.

4. **Execution Log Format:** Verifier expects execution logs in a specific format with TEST_SEQUENCE, STEP, EXIT_CODE, and TIMESTAMP markers.

5. **File Naming:** Result files are named `{test_case_id}_result.yaml`. Test case IDs must be valid filenames (no special characters like `/`, `\`, `:`, etc.).

---

## Additional Resources

- **Schema Documentation:** `schemas/README.md`
- **Verifier Usage:** `docs/TEST_VERIFY_USAGE.md`
- **Test Verification Workflow:** `docs/TEST_VERIFY_WORKFLOW.md`
- **Validation Quick Reference:** `docs/VALIDATE_YAML_QUICK_REF.md`
- **GitLab CI Examples:** `docs/GITLAB_CI_EXAMPLES.md`
- **Test Case Structure:** `schemas/test-case.schema.json`

## Related Makefile Targets

```bash
make build                  # Build all binaries including verifier
make generate-docs          # Generate docs for verifier_scenarios
make generate-docs-all      # Generate docs for all testcases
make test                   # Run full test suite
make verify-scripts         # Verify shell script syntax
```
