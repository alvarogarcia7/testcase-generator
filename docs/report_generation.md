# Report Generation Documentation

## Overview

The Test Case Manager includes a comprehensive report generation system that converts test execution logs into professional documentation reports. The system integrates with `test-plan-doc-gen`, an external Rust-based tool that generates reports in multiple formats (AsciiDoc, Markdown, PDF) from test case YAML files and verification results.

## Table of Contents

1. [Installation and Building test-plan-doc-gen](#installation-and-building-test-plan-doc-gen)
2. [Directory Structure](#directory-structure)
3. [Running Report Generation](#running-report-generation)
4. [Report Output Formats and Locations](#report-output-formats-and-locations)
5. [Verifier Configuration](#verifier-configuration)
6. [Workflow Examples](#workflow-examples)
7. [Customizing Templates](#customizing-templates)
8. [Troubleshooting](#troubleshooting)

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

---

## Verifier Configuration

The verifier binary provides comprehensive configuration options for generating reports with rich metadata. Reports can be configured using either a YAML configuration file, individual CLI flags, or a combination of both.

### Basic Verifier Options

- `--log PATH, -l PATH` - Single-file mode: path to log file
- `--test-case ID, -c ID` - Single-file mode: test case ID to verify against
- `--folder PATH, -f PATH` - Folder discovery mode: path to folder containing log files
- `--format FORMAT, -F FORMAT` - Output format (yaml or json, default: yaml)
- `--output PATH, -o PATH` - Output file path (optional, defaults to stdout)
- `--test-case-dir DIR, -d DIR` - Path to test case storage directory (default: testcases)
- `--log-level LEVEL` - Set log level (trace, debug, info, warn, error, default: info)
- `--verbose, -v` - Enable verbose output (equivalent to --log-level=debug)

### Report Metadata Configuration

The verifier generates container-format output with enhanced metadata and statistics. Metadata can be configured using two methods:

#### Method 1: Configuration File (Recommended)

Use a YAML configuration file to define report metadata:

**Configuration File Format:**
```yaml
# verifier-config.yaml
title: "Test Execution Results"
project: "Test Case Manager - Verification Results"
environment: "Staging"
platform: "Linux x86_64"
executor: "Jenkins v3.2"
```

**Usage:**
```bash
verifier -f logs/ --format yaml --output report.yaml --config verifier-config.yaml
```

**Configuration File Option:**
- `--config PATH` - Path to YAML configuration file (optional)

#### Method 2: Individual CLI Flags

Configure metadata using individual command-line flags:

**Metadata CLI Flags:**
- `--title TEXT` - Report title (default: "Test Execution Results")
- `--project TEXT` - Project name (default: "Test Case Manager - Verification Results")
- `--environment TEXT` - Environment information (e.g., "Staging", "Production")
- `--platform TEXT` - Platform information (e.g., "Linux x86_64")
- `--executor TEXT` - Executor information (e.g., "CI Pipeline v2.1")

**Usage:**
```bash
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases \
  --title "Q1 2024 Test Results" \
  --project "Product XYZ Certification" \
  --environment "Production Test Lab" \
  --platform "Test Platform v2.0" \
  --executor "CI/CD Pipeline"
```

#### Method 3: Hybrid Approach (Configuration File + CLI Overrides)

Combine both methods by using a configuration file as the base and overriding specific values with CLI flags:

**Configuration File (`base-config.yaml`):**
```yaml
title: "Default Test Report"
project: "My Project"
environment: "Development"
platform: "Test Platform v1.0"
executor: "Manual Execution"
```

**Usage with Overrides:**
```bash
verifier -f logs/ --format yaml --output report.yaml \
  --config base-config.yaml \
  --title "Nightly Test Run" \
  --environment "Production"
```

This will use:
- `title`: "Nightly Test Run" (CLI override)
- `project`: "My Project" (from config file)
- `environment`: "Production" (CLI override)
- `platform`: "Test Platform v1.0" (from config file)
- `executor`: "Manual Execution" (from config file)

### Configuration Precedence Rules

When both a configuration file and CLI flags are provided, the following precedence rules apply:

1. **CLI flags have highest priority** - Any metadata specified via CLI flag will override the corresponding value in the configuration file
2. **Configuration file values are used as defaults** - Values from the config file are used when no corresponding CLI flag is provided
3. **Built-in defaults are used as fallback** - If a value is not specified in either the config file or CLI, built-in defaults are used (for `--title` and `--project` only)

**Precedence Order (highest to lowest):**
1. CLI flags (`--title`, `--project`, `--environment`, `--platform`, `--executor`)
2. Configuration file values
3. Built-in defaults (`--title`: "Test Execution Results", `--project`: "Test Case Manager - Verification Results")

**Note:** The `--environment`, `--platform`, and `--executor` fields are optional and will only be included in the output if specified via either method.

### Configuration Examples

#### Using Defaults Only

```bash
# Uses built-in defaults for title and project
verifier -f logs/ --format yaml --output report.yaml
```

**Output includes:**
- `title`: "Test Execution Results"
- `project`: "Test Case Manager - Verification Results"

#### Using Configuration File Only

```bash
verifier -f logs/ --format yaml --output report.yaml --config verifier-config.yaml
```

**Configuration File (`verifier-config.yaml`):**
```yaml
title: "GSMA SGP.22 Compliance Testing Results"
project: "eUICC Test Suite v3.2"
environment: "GSMA Certification Lab - Environment 2"
platform: "eUICC Test Platform v3.2.1"
executor: "Automated Test Framework v2.5.0"
```

**Output includes all values from config file.**

#### Using CLI Flags Only

```bash
verifier -f logs/ --format yaml --output report.yaml \
  --title "Build 123 Results" \
  --project "CI/CD Testing" \
  --environment "Staging" \
  --platform "Linux x86_64" \
  --executor "GitLab Runner"
```

**Output includes all values from CLI flags.**

#### Using Hybrid Approach

```bash
verifier -f logs/ --format yaml --output report.yaml \
  --config base-config.yaml \
  --title "Custom Title" \
  --environment "Production"
```

**Configuration File (`base-config.yaml`):**
```yaml
title: "Default Title"
project: "My Project"
environment: "Development"
platform: "Test Platform"
executor: "Manual"
```

**Output includes:**
- `title`: "Custom Title" (CLI override)
- `project`: "My Project" (from config)
- `environment`: "Production" (CLI override)
- `platform`: "Test Platform" (from config)
- `executor`: "Manual" (from config)

### Step-by-Step Manual Generation

You can also run each step individually:

#### Step 1: Run Verifier on Execution Logs

**Using Configuration File:**
```bash
# Build verifier if needed
cargo build --release --bin verifier

# Run verifier with config file
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/documentation/results/container_report.yaml \
  --test-case-dir testcases \
  --config verifier-config.yaml
```

**Using CLI Flags:**
```bash
# Run verifier with CLI flags
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/documentation/results/container_report.yaml \
  --test-case-dir testcases \
  --title "Test Execution Results Report" \
  --project "Test Case Manager - Verification Results" \
  --environment "Test Environment" \
  --platform "Test Case Manager" \
  --executor "Automated Test Framework"
```

**Output Structure (YAML):**
```yaml
title: 'Test Execution Results Report'
project: 'Test Case Manager - Verification Results'
test_date: '2024-01-15T14:30:00Z'
test_results:
  - test_case_id: "TC_001"
    description: "Example test case"
    requirement: "REQ_001"
    item: 1
    tc: 1
    sequences:
      - sequence_id: 1
        name: "Test Sequence"
        step_results:
          - Pass:
              step: 1
              description: "Execute command"
        all_steps_passed: true
    total_steps: 1
    passed_steps: 1
    failed_steps: 0
    not_executed_steps: 0
    overall_pass: true
metadata:
  environment: 'Test Environment'
  platform: 'Test Case Manager'
  executor: 'Automated Test Framework'
  execution_duration: 45.7
  total_test_cases: 1
  passed_test_cases: 1
  failed_test_cases: 0
```

**Output Structure (JSON):**
```json
{
  "title": "Test Execution Results Report",
  "project": "Test Case Manager - Verification Results",
  "test_date": "2024-01-15T14:30:00Z",
  "test_results": [
    {
      "test_case_id": "TC_001",
      "description": "Example test case",
      "requirement": "REQ_001",
      "item": 1,
      "tc": 1,
      "sequences": [...],
      "total_steps": 1,
      "passed_steps": 1,
      "failed_steps": 0,
      "not_executed_steps": 0,
      "overall_pass": true
    }
  ],
  "metadata": {
    "environment": "Test Environment",
    "platform": "Test Case Manager",
    "executor": "Automated Test Framework",
    "execution_duration": 45.7,
    "total_test_cases": 1,
    "passed_test_cases": 1,
    "failed_test_cases": 0
  }
}
```

#### Step 2: Generate Test Results Report (AsciiDoc)

```bash
# Set binary path
export TEST_PLAN_DOC_GEN=../test-plan-doc-gen/target/release/test-plan-doc-gen

# Generate AsciiDoc report
$TEST_PLAN_DOC_GEN \
  --container reports/documentation/results/container_report.yaml \
  --output reports/documentation/reports/test_results_report.adoc \
  --format asciidoc
```

#### Step 3: Generate Test Plan Reports (Markdown)

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

#### 1. Container YAML/JSON

**Location:** User-specified via `--output` flag

Container-format output with comprehensive test results, metadata, and execution statistics. This is the primary output format used for documentation generation.

**Structure:**
```yaml
title: 'Test Execution Results Report'
project: 'Project Name'
test_date: '2024-01-01T00:00:00Z'
test_results:
  - # Test case 1
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

#### 2. AsciiDoc Report

**Location:** `reports/documentation/reports/test_results_report.adoc`

Professional test results report in AsciiDoc format, generated from the container YAML/JSON. Can be converted to HTML or PDF using AsciiDoctor.

**Example Usage:**
```bash
# Convert AsciiDoc to HTML
asciidoctor reports/documentation/reports/test_results_report.adoc

# Convert AsciiDoc to PDF
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc
```

#### 3. Markdown Test Plans

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
├── results/
│   └── container_report.yaml           # Container with all results
└── reports/
    ├── test_results_report.adoc        # AsciiDoc results report
    ├── TC_001_test_plan.md             # Markdown test plans
    ├── TC_002_test_plan.md
    └── ...
```

### Accessing Reports

```bash
# View container YAML
cat reports/documentation/results/container_report.yaml

# View AsciiDoc report (raw)
less reports/documentation/reports/test_results_report.adoc

# View Markdown test plan
cat reports/documentation/reports/TC_001_test_plan.md

# Convert AsciiDoc to HTML and open in browser
asciidoctor reports/documentation/reports/test_results_report.adoc
open reports/documentation/reports/test_results_report.html
```

---

## Workflow Examples

This section provides complete workflow examples showing different approaches for common reporting scenarios.

### Workflow 1: Basic Test Verification with Configuration File

Use a configuration file to define consistent report metadata across test runs.

**Step 1: Create Configuration File**
```bash
cat > verifier-config.yaml << 'EOF'
title: "Test Execution Results"
project: "Test Case Manager"
environment: "Test Environment"
platform: "Test Platform v1.0"
executor: "Automated Test Framework"
EOF
```

**Step 2: Run Verifier with Configuration**
```bash
# Build verifier
cargo build --release --bin verifier

# Run verification with config file
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container_report.yaml \
  --test-case-dir testcases \
  --config verifier-config.yaml
```

**Step 3: Generate Documentation (Optional)**
```bash
# Generate AsciiDoc report with test-plan-doc-gen
../test-plan-doc-gen/target/release/test-plan-doc-gen \
  --container reports/container_report.yaml \
  --output reports/test_results.adoc \
  --format asciidoc

# Convert to PDF
asciidoctor-pdf reports/test_results.adoc
```

---

### Workflow 2: Direct CLI Flags Approach

Use CLI flags for one-off reports or when metadata varies between runs.

**Step 1: Run Verifier with CLI Flags**
```bash
# Build verifier
cargo build --release --bin verifier

# Run verification with CLI flags
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container_report.yaml \
  --test-case-dir testcases \
  --title "Test Execution Results Report" \
  --project "Test Case Manager - Q1 2024" \
  --environment "Test Environment" \
  --platform "Test Case Manager v1.0" \
  --executor "Automated Test Framework"
```

**Step 2: Generate Documentation (Optional)**
```bash
# Generate AsciiDoc report
../test-plan-doc-gen/target/release/test-plan-doc-gen \
  --container reports/container_report.yaml \
  --output reports/test_results.adoc \
  --format asciidoc

# Convert to HTML
asciidoctor reports/test_results.adoc
```

---

### Workflow 3: CI/CD Pipeline Integration with Base Configuration

Use a base configuration file with CI/CD-specific overrides for automated testing.

**Base Configuration File (`ci-config.yaml`):**
```yaml
project: "Product XYZ - CI/CD Testing"
platform: "CI/CD Test Platform v2.0"
executor: "GitLab Runner"
```

**GitLab CI Example:**
```yaml
# .gitlab-ci.yml
test_verification:
  stage: test
  script:
    # Build verifier
    - cargo build --release --bin verifier
    
    # Run tests and generate execution logs
    # (your test execution commands here)
    
    # Run verifier with base config + environment-specific overrides
    - |
      ./target/release/verifier \
        --folder testcases/verifier_scenarios \
        --format json \
        --output "reports/test_results_build_${CI_PIPELINE_ID}.json" \
        --test-case-dir testcases \
        --config ci-config.yaml \
        --title "Build ${CI_PIPELINE_ID} Test Results" \
        --environment "${CI_ENVIRONMENT_NAME}"
  
  artifacts:
    reports:
      junit: reports/junit.xml
    paths:
      - reports/
    when: always
  
  allow_failure: false
```

**GitHub Actions Example:**
```yaml
# .github/workflows/test.yml
name: Test Verification
on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build verifier
        run: cargo build --release --bin verifier
      
      - name: Run tests
        run: |
          # Your test execution commands
          
      - name: Verify results
        run: |
          ./target/release/verifier \
            --folder testcases/verifier_scenarios \
            --format yaml \
            --output reports/results.yaml \
            --test-case-dir testcases \
            --config ci-config.yaml \
            --title "Build ${{ github.run_number }} Results" \
            --environment "${{ github.ref_name }}" \
            --executor "GitHub Actions - Run ${{ github.run_id }}"
      
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: reports/
```

---

### Workflow 4: Single Test Case Verification

Verify a single test case execution log with minimal configuration.

**Using Configuration File:**
```bash
# Run verifier on single log file with config
./target/release/verifier \
  --log testcases/logs/TC_001_execution_log.json \
  --test-case "TC_001" \
  --format yaml \
  --output reports/TC_001_result.yaml \
  --test-case-dir testcases \
  --config verifier-config.yaml

# View results
cat reports/TC_001_result.yaml
```

**Using CLI Flags:**
```bash
# Run verifier on single log file with CLI flags
./target/release/verifier \
  --log testcases/logs/TC_001_execution_log.json \
  --test-case "TC_001" \
  --format yaml \
  --output reports/TC_001_result.yaml \
  --test-case-dir testcases \
  --title "TC_001 Verification Result" \
  --project "Test Case Validation"

# View results
cat reports/TC_001_result.yaml
```

---

### Workflow 5: Multi-Environment Testing

Test across multiple environments with environment-specific configurations.

**Base Configuration (`base-config.yaml`):**
```yaml
project: "Multi-Environment Validation"
platform: "Test Platform v2.0"
executor: "Automated Testing Framework"
```

**Multi-Environment Script:**
```bash
#!/bin/bash
# multi_env_test.sh - Run tests across environments

ENVIRONMENTS=("dev" "staging" "production")

for ENV in "${ENVIRONMENTS[@]}"; do
  echo "Testing environment: $ENV"
  
  # Run tests (your test execution commands)
  # ./run_tests.sh --environment $ENV
  
  # Run verifier with base config + environment-specific overrides
  ./target/release/verifier \
    --folder "testcases/verifier_scenarios/$ENV" \
    --format yaml \
    --output "reports/${ENV}_test_results.yaml" \
    --test-case-dir testcases \
    --config base-config.yaml \
    --title "$(date +%Y-%m-%d) $ENV Environment Test Results" \
    --environment "$ENV Environment"
  
  # Check exit status
  if [ $? -eq 0 ]; then
    echo "✓ $ENV: All tests passed"
  else
    echo "✗ $ENV: Some tests failed"
  fi
done

# Generate consolidated report
echo "Consolidating results..."
cat reports/*_test_results.yaml > reports/consolidated_results.yaml
```

---

### Workflow 6: Compliance Documentation

Generate audit-ready compliance documentation with comprehensive metadata.

**Compliance Configuration (`compliance-config.yaml`):**
```yaml
project: "Product XYZ - Compliance Testing"
platform: "Certified Test Platform"
executor: "Compliance Testing Framework"
```

**Compliance Report Script:**
```bash
#!/bin/bash
# compliance_report.sh - Generate compliance documentation

# Set compliance metadata
COMPLIANCE_STANDARD="ISO 9001:2015"
AUDIT_DATE=$(date +%Y-%m-%d)
AUDITOR="Quality Assurance Team"
VERSION="v1.2.3"

# Run comprehensive verification with hybrid configuration
./target/release/verifier \
  --folder testcases/compliance_scenarios \
  --format yaml \
  --output "reports/compliance_report_${AUDIT_DATE}.yaml" \
  --test-case-dir testcases/compliance \
  --config compliance-config.yaml \
  --title "Compliance Verification Report - ${COMPLIANCE_STANDARD}" \
  --environment "Compliance Test Lab - Controlled Environment" \
  --executor "Compliance Testing Framework - Auditor: ${AUDITOR}"

# Generate professional documentation
if [ -f "../test-plan-doc-gen/target/release/test-plan-doc-gen" ]; then
  # Generate AsciiDoc
  ../test-plan-doc-gen/target/release/test-plan-doc-gen \
    --container "reports/compliance_report_${AUDIT_DATE}.yaml" \
    --output "reports/compliance_report_${AUDIT_DATE}.adoc" \
    --format asciidoc
  
  # Convert to PDF for archival
  asciidoctor-pdf "reports/compliance_report_${AUDIT_DATE}.adoc"
  
  echo "✓ Compliance report generated: reports/compliance_report_${AUDIT_DATE}.pdf"
else
  echo "⚠ test-plan-doc-gen not available, skipping PDF generation"
fi

# Create audit package
mkdir -p "audit_packages/audit_${AUDIT_DATE}"
cp "reports/compliance_report_${AUDIT_DATE}."* "audit_packages/audit_${AUDIT_DATE}/"
cp -r testcases/compliance_scenarios "audit_packages/audit_${AUDIT_DATE}/logs"

echo "✓ Audit package created: audit_packages/audit_${AUDIT_DATE}/"
```

---

### Workflow 7: Batch Verification with Custom Metadata

Generate comprehensive reports with detailed execution metadata using hybrid configuration.

**Base Configuration (`batch-config.yaml`):**
```yaml
platform: "Test Platform v2.5.0 - Ubuntu 22.04 LTS"
executor: "Test Automation Framework v1.2.3"
```

**Batch Verification Script:**
```bash
#!/bin/bash
# batch_verification.sh

# Ensure all execution logs are in place
ls -la testcases/verifier_scenarios/*.json

# Run verifier with base config + runtime overrides
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/batch_verification_$(date +%Y%m%d_%H%M%S).yaml \
  --test-case-dir testcases \
  --config batch-config.yaml \
  --title "$(date +%Y-%m-%d) Test Execution Results" \
  --project "Product XYZ - Sprint 23 Testing" \
  --environment "QA Environment - Server cluster-qa-01"

# Archive results
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mkdir -p archives/$TIMESTAMP
cp reports/batch_verification_*.yaml archives/$TIMESTAMP/
cp -r testcases/verifier_scenarios/*.json archives/$TIMESTAMP/logs/

# Generate summary report
echo "Test Execution Summary - $TIMESTAMP" > archives/$TIMESTAMP/SUMMARY.txt
echo "======================================" >> archives/$TIMESTAMP/SUMMARY.txt
echo "" >> archives/$TIMESTAMP/SUMMARY.txt
grep -E "total_test_cases|passed_test_cases|failed_test_cases" \
  reports/batch_verification_*.yaml >> archives/$TIMESTAMP/SUMMARY.txt
```

---

### Configuration Method Comparison

| Approach | Best For | Benefits | Use When |
|----------|----------|----------|----------|
| **Config File Only** | Consistent environments | Reusable, version-controlled | Same metadata across runs |
| **CLI Flags Only** | One-off reports | Flexible, no file management | Metadata varies each time |
| **Hybrid (Config + CLI)** | CI/CD pipelines | Base consistency + runtime flexibility | Some values fixed, others dynamic |

### Key Takeaways

1. **Configuration File Method**: Best for maintaining consistency across test runs
   - Define standard metadata in version-controlled config file
   - Easy to maintain and update
   - Good for team collaboration

2. **CLI Flags Method**: Best for dynamic or one-off scenarios
   - No file management required
   - Full control over all metadata
   - Good for quick reports

3. **Hybrid Approach**: Best for CI/CD and automated testing
   - Combine base configuration with runtime overrides
   - Maximum flexibility with consistency
   - Good for complex workflows

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

#### 4. Configuration File Issues

**Problem:**
```
✗ Error: Failed to parse configuration file
Invalid YAML syntax
```

**Solutions:**

**Validate YAML Syntax:**
```bash
# Check YAML syntax using Python
python3 -c "import yaml; yaml.safe_load(open('verifier-config.yaml'))"

# Or use a YAML linter
yamllint verifier-config.yaml
```

**Common Configuration Issues:**
- Incorrect indentation
- Missing quotes around values with special characters
- Invalid YAML structure

**Fix Example:**
```yaml
# Before (invalid)
title: Test Report: Q1 2024  # Colon needs quotes
environment: Dev & Test      # Ampersand needs quotes

# After (valid)
title: "Test Report: Q1 2024"
environment: "Dev & Test"
```

#### 5. Empty or Missing Reports

**Problem:**
```
No result files found to include in container
```

**Solutions:**

**Check Verifier Output:**
```bash
# Verify container file was generated
ls -la reports/documentation/results/

# Check container contents
cat reports/documentation/results/container_report.yaml
```

**Verify Output Structure:**
```bash
# Check that container has test_results
grep -A 5 "test_results:" reports/documentation/results/container_report.yaml
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

#### 9. Metadata Not Appearing in Output

**Problem:**
```
Missing metadata in container output
Container YAML doesn't include environment/platform/executor
```

**Solutions:**

**Verify Configuration:**
```bash
# Check that config file includes the fields
cat verifier-config.yaml

# Or verify CLI flags are being passed correctly
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases \
  --environment "Test Environment" \
  --platform "Test Platform" \
  --executor "Test Executor"
```

**Check Optional Fields:**
```bash
# Remember: environment, platform, executor are optional
# They only appear if specified via config file or CLI

# Verify values in output
grep -E "^(environment|platform|executor):" reports/container.yaml
```

#### 10. CLI Override Not Working

**Problem:**
```
CLI flag values not overriding config file values
```

**Solution:**

**Verify Flag Order:**
```bash
# Ensure flags come after --config
# Correct:
verifier --config base.yaml --title "New Title" -f logs/ -o report.yaml

# Incorrect (may not work):
verifier --title "New Title" --config base.yaml -f logs/ -o report.yaml
```

**Check Flag Syntax:**
```bash
# Ensure values with spaces are quoted
verifier --config base.yaml --title "My Report Title" -f logs/ -o report.yaml

# Verify the override worked
grep "^title:" report.yaml
```

### Debug Mode

Enable verbose logging for detailed troubleshooting:

```bash
# Set verbose mode
export VERBOSE=1

# Run report generation
./scripts/generate_documentation_reports.sh

# Or use verbose flag in verifier
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases \
  --verbose
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

1. **test-plan-doc-gen Dependency:** The report generation system requires `test-plan-doc-gen` to be available as a sibling directory or in PATH for professional report generation (optional).

2. **Container Template Structure:** The container template must follow the exact YAML structure expected by `test-plan-doc-gen`.

3. **Execution Log Format:** Verifier expects execution logs in a specific format with TEST_SEQUENCE, STEP, EXIT_CODE, and TIMESTAMP markers.

4. **File Naming:** Result files are named `{test_case_id}_result.yaml`. Test case IDs must be valid filenames (no special characters like `/`, `\`, `:`, etc.).

5. **Metadata Field Limits:** While title, project, environment, platform, and executor fields accept arbitrary strings, extremely long values (>1000 characters) may cause rendering issues in some documentation tools.

6. **Configuration File Format:** Configuration files must be valid YAML. The verifier does not support JSON configuration files.

---

## Quick Reference

### Basic Verifier Commands

**Minimal (Using Defaults):**
```bash
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases
```

**With Configuration File:**
```bash
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases \
  --config verifier-config.yaml
```

**With CLI Flags:**
```bash
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases \
  --title "Report Title" \
  --project "Project Name" \
  --environment "Environment Info" \
  --platform "Platform Info" \
  --executor "Executor Info"
```

**Hybrid (Config + CLI Overrides):**
```bash
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format yaml \
  --output reports/container.yaml \
  --test-case-dir testcases \
  --config base-config.yaml \
  --title "Custom Title" \
  --environment "Production"
```

**Single File:**
```bash
./target/release/verifier \
  --log testcases/logs/TC_001_execution_log.json \
  --test-case "TC_001" \
  --format yaml \
  --output reports/TC_001_result.yaml \
  --test-case-dir testcases \
  --title "TC_001 Results" \
  --project "Test Validation"
```

**JSON Output:**
```bash
./target/release/verifier \
  --folder testcases/verifier_scenarios \
  --format json \
  --output reports/container.json \
  --test-case-dir testcases \
  --config verifier-config.yaml
```

### CLI Options Summary

| Option | Short | Required | Default | Description |
|--------|-------|----------|---------|-------------|
| `--folder` | `-f` | Yes* | - | Path to folder with logs |
| `--log` | `-l` | Yes* | - | Single log file path |
| `--test-case` | `-c` | Yes** | - | Test case ID (single-file mode) |
| `--format` | `-F` | No | yaml | Output format (yaml/json) |
| `--output` | `-o` | No | stdout | Output file path |
| `--test-case-dir` | `-d` | No | testcases | Test case directory |
| `--config` | - | No | - | Path to YAML configuration file |
| `--title` | - | No | "Test Execution Results" | Report title |
| `--project` | - | No | "Test Case Manager - Verification Results" | Project name |
| `--environment` | - | No | - | Environment info (optional) |
| `--platform` | - | No | - | Platform info (optional) |
| `--executor` | - | No | - | Executor info (optional) |
| `--verbose` | `-v` | No | false | Enable verbose logging |

\* Either `--folder` or `--log` is required  
\** `--test-case` is required when using `--log`

### Configuration Method Cheat Sheet

| Need | Use Method | Example |
|------|-----------|---------|
| Consistent metadata | Config file only | `--config verifier-config.yaml` |
| One-off report | CLI flags only | `--title "..." --project "..." --environment "..."` |
| CI/CD with base config | Hybrid approach | `--config base.yaml --title "Build 123" --environment "staging"` |
| Minimal setup | Defaults only | No config, no flags (uses built-in defaults) |

### Precedence Rules

1. **CLI flags** (highest priority)
2. **Configuration file values**
3. **Built-in defaults** (for title and project only)

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
