# Report Generation Documentation

## Overview

The Test Case Manager includes a comprehensive report generation system that converts test execution logs into professional documentation reports. The system uses **test-plan-documentation-generator** (tpdg), a Rust-based tool that generates reports in multiple formats (AsciiDoc, Markdown, HTML) from test case YAML files and verification results.

**Migration from Python to Rust**: The legacy Python-based PDF generation has been completely removed in favor of test-plan-documentation-generator, which provides better performance, maintainability, and native integration with the Rust test framework.

## Quick Reference

**Install Dependencies:**
```bash
cargo install test-plan-documentation-generator
pip3 install pyyaml
```

**Generate Reports:**
```bash
make generate-docs          # Verifier scenarios only
make generate-docs-all      # All test cases
```

**Convert to HTML/PDF:**
```bash
# HTML (requires asciidoctor)
asciidoctor reports/documentation/reports/test_results_report.adoc

# PDF (requires asciidoctor-pdf)
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc
```

**Output Formats:**
- AsciiDoc (.adoc) - Primary format from tpdg
- Markdown (.md) - Primary format from tpdg
- HTML - Converted from AsciiDoc/Markdown
- PDF - Converted from AsciiDoc

## Table of Contents

1. [Installation](#installation)
2. [Dependencies](#dependencies)
3. [Directory Structure](#directory-structure)
4. [Running Report Generation](#running-report-generation)
5. [Report Output Formats and Locations](#report-output-formats-and-locations)
6. [Schema Compatibility](#schema-compatibility)
7. [Customizing Templates](#customizing-templates)
8. [Troubleshooting](#troubleshooting)

---

## Installation

### Method 1: Install from crates.io (Recommended)

The simplest way to install test-plan-documentation-generator is via cargo:

```bash
# Install globally from crates.io
cargo install test-plan-documentation-generator

# Verify installation
which test-plan-documentation-generator
test-plan-documentation-generator --version
```

After installation, the binary will be available in your PATH (typically `~/.cargo/bin/test-plan-documentation-generator`).

### Method 2: Build from Source

If you need to build from source or use a development version:

```bash
# Clone the repository
git clone <test-plan-documentation-generator-repo-url>
cd test-plan-documentation-generator

# Build release binary
cargo build --release

# Install to cargo bin directory
cargo install --path .

# Or use the binary directly
./target/release/test-plan-documentation-generator --version
```

### Method 3: Custom Binary Path

If you prefer not to install globally, you can specify a custom binary path:

```bash
# Set environment variable
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator

# Or use with report generation scripts
./scripts/generate_documentation_reports.sh \
  --test-plan-doc-gen /path/to/test-plan-documentation-generator
```

### Verification

Verify that test-plan-documentation-generator is correctly installed:

```bash
# Check version
test-plan-documentation-generator --version

# Display help
test-plan-documentation-generator --help

# Test basic functionality
test-plan-documentation-generator \
  --test-case testcases/example.yml \
  --output /tmp/test_report.md \
  --format markdown
```

---

## Dependencies

### Required Dependencies

#### 1. test-plan-documentation-generator (tpdg)

**Installation:**
```bash
cargo install test-plan-documentation-generator
```

**Purpose:** Primary report generation tool (Rust-based)

**Formats:** AsciiDoc, Markdown

#### 2. Python 3 with PyYAML

**Installation:**
```bash
# Ubuntu/Debian
sudo apt-get install python3 python3-pip
pip3 install pyyaml

# macOS
brew install python3
pip3 install pyyaml

# Or use virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate
pip install pyyaml
```

**Purpose:** Used by `convert_verification_to_result_yaml.py` script for JSON to YAML conversion

**Note:** This is the only remaining Python dependency. The legacy reportlab dependency has been removed.

### Optional Dependencies (for HTML/PDF conversion)

#### 3. asciidoctor (for HTML conversion)

**Installation:**
```bash
# Ubuntu/Debian
sudo apt-get install asciidoctor

# macOS
brew install asciidoctor
gem install asciidoctor

# Or via RubyGems
gem install asciidoctor
```

**Purpose:** Convert AsciiDoc reports to HTML

**Usage:**
```bash
asciidoctor reports/documentation/reports/test_results_report.adoc
```

#### 4. asciidoctor-pdf (for PDF conversion)

**Installation:**
```bash
# Via RubyGems
gem install asciidoctor-pdf

# Or with bundler
bundle add asciidoctor-pdf
```

**Purpose:** Convert AsciiDoc reports to PDF

**Usage:**
```bash
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc
```

#### 5. pandoc (for Markdown to HTML conversion)

**Installation:**
```bash
# Ubuntu/Debian
sudo apt-get install pandoc

# macOS
brew install pandoc
```

**Purpose:** Convert Markdown test plans to HTML

**Usage:**
```bash
pandoc reports/documentation/reports/TC_001_test_plan.md \
  -o TC_001_test_plan.html \
  --standalone
```

### Dependency Summary

| Dependency | Required | Purpose | Installation |
|------------|----------|---------|--------------|
| test-plan-documentation-generator | Yes | Report generation | `cargo install test-plan-documentation-generator` |
| Python 3 + PyYAML | Yes | JSON to YAML conversion | `pip3 install pyyaml` |
| asciidoctor | No | AsciiDoc to HTML | `gem install asciidoctor` |
| asciidoctor-pdf | No | AsciiDoc to PDF | `gem install asciidoctor-pdf` |
| pandoc | No | Markdown to HTML | `brew install pandoc` or `apt-get install pandoc` |

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

### Quick Start

The simplest way to generate reports is using the Makefile targets:

```bash
# Generate documentation reports for verifier_scenarios only
make generate-docs

# Generate documentation reports for all testcases
make generate-docs-all
```

**What these commands do:**
1. Run the `verifier` tool on execution logs
2. Convert verification JSON to result YAML files
3. Create a combined results container YAML
4. Generate AsciiDoc test results report using test-plan-documentation-generator
5. Generate Markdown test plan reports using test-plan-documentation-generator

**Output location:** `reports/documentation/`

**Note:** HTML and PDF generation requires additional conversion steps (see [Converting to HTML/PDF](#converting-and-viewing-htmlpdf))

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
# Generate AsciiDoc report using test-plan-documentation-generator
test-plan-documentation-generator \
  --container reports/documentation/results/results_container.yaml \
  --output reports/documentation/reports/test_results_report.adoc \
  --format asciidoc

# Or use environment variable for custom binary path
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator
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
    test-plan-documentation-generator \
      --test-case "$test_case" \
      --output "reports/documentation/reports/${basename}_test_plan.md" \
      --format markdown
done
```

#### Step 6 (Optional): Convert to HTML or PDF

```bash
# Convert AsciiDoc to HTML (requires asciidoctor)
asciidoctor reports/documentation/reports/test_results_report.adoc

# Convert AsciiDoc to PDF (requires asciidoctor-pdf)
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc

# Convert Markdown to HTML (requires pandoc)
for md in reports/documentation/reports/*_test_plan.md; do
    pandoc "$md" -o "${md%.md}.html" --standalone
done
```

### Using the Report Generator Library

For custom scripts, use the `report_generator.sh` library which provides helper functions for working with test-plan-documentation-generator:

```bash
#!/usr/bin/env bash
set -e

# Source libraries
source scripts/lib/logger.sh
source scripts/lib/report_generator.sh

# Check if test-plan-documentation-generator is available
if check_test_plan_doc_gen_available; then
    log_info "test-plan-documentation-generator is available"
else
    log_error "test-plan-documentation-generator not found"
    exit 1
fi

# Generate report
invoke_test_plan_doc_gen \
    --test-case "testcases/example.yml" \
    --output "reports/example_report.md" \
    --format markdown

# Validate output
validate_report_output "reports" "example_report.md"
```

**Available Library Functions:**

- `check_test_plan_doc_gen_available` - Check if binary is available in PATH
- `invoke_test_plan_doc_gen` - Call test-plan-documentation-generator with arguments
- `validate_report_output` - Verify report file was created successfully

**Setting Custom Binary Path:**

```bash
# Use environment variable
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator

# Or specify in script
TEST_PLAN_DOC_GEN=/custom/path invoke_test_plan_doc_gen \
    --test-case "testcases/example.yml" \
    --output "reports/example_report.md" \
    --format markdown
```

---

## Report Output Formats and Locations

### Output Formats

The report generation system produces multiple output formats through a multi-stage pipeline:

**Primary Formats (Generated by test-plan-documentation-generator):**
- AsciiDoc (.adoc) - Structured documentation format
- Markdown (.md) - GitHub-compatible documentation

**Secondary Formats (Converted from AsciiDoc):**
- HTML - Converted using asciidoctor (requires asciidoctor installation)
- PDF - Converted using asciidoctor-pdf (requires asciidoctor-pdf installation)

#### 1. Verification JSON
**Location:** `reports/documentation/verification/batch_verification.json`

Raw verification results from the verifier tool in JSON format. Contains detailed pass/fail information for all test cases.

**Generated by:** `verifier` binary

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

Professional test results report in AsciiDoc format, generated from the results container by test-plan-documentation-generator.

**Generated by:** `test-plan-documentation-generator` with `--format asciidoc`

**Conversion to HTML:**
```bash
# Convert AsciiDoc to HTML (requires asciidoctor)
asciidoctor reports/documentation/reports/test_results_report.adoc

# Output: reports/documentation/reports/test_results_report.html
```

**Conversion to PDF:**
```bash
# Convert AsciiDoc to PDF (requires asciidoctor-pdf)
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc

# Output: reports/documentation/reports/test_results_report.pdf
```

**Custom Styling:**
```bash
# HTML with custom CSS
asciidoctor \
  -a stylesheet=custom-style.css \
  reports/documentation/reports/test_results_report.adoc

# PDF with custom theme
asciidoctor-pdf \
  -a pdf-theme=custom-theme.yml \
  reports/documentation/reports/test_results_report.adoc
```

#### 5. Markdown Test Plans
**Location:** `reports/documentation/reports/*_test_plan.md`

Individual test plan documentation in Markdown format, generated from test case YAML files by test-plan-documentation-generator.

**Generated by:** `test-plan-documentation-generator` with `--format markdown`

**Content includes:**
- Test case metadata (requirement, item, tc, id)
- Description and prerequisites
- Initial conditions (general and device-specific)
- Test sequences and steps
- Expected results and verification expressions

**Conversion to HTML:**
```bash
# Convert Markdown to HTML (requires pandoc)
pandoc reports/documentation/reports/TC_001_test_plan.md \
  -o TC_001_test_plan.html \
  --standalone

# With custom CSS
pandoc reports/documentation/reports/TC_001_test_plan.md \
  -o TC_001_test_plan.html \
  --css=custom-style.css \
  --standalone
```

#### 6. HTML Reports (Converted)
**Location:** `reports/documentation/reports/*.html`

HTML versions of AsciiDoc and Markdown reports, generated via asciidoctor or pandoc.

**Not directly generated by test-plan-documentation-generator** - these are created by converting AsciiDoc or Markdown outputs.

**Generation methods:**
```bash
# From AsciiDoc (test results report)
asciidoctor reports/documentation/reports/test_results_report.adoc

# From Markdown (test plans)
pandoc reports/documentation/reports/TC_001_test_plan.md \
  -o TC_001_test_plan.html \
  --standalone
```

#### 7. PDF Reports (Converted)
**Location:** `reports/documentation/reports/*.pdf`

PDF versions of AsciiDoc reports, generated via asciidoctor-pdf.

**Not directly generated by test-plan-documentation-generator** - these are created by converting AsciiDoc outputs.

**Generation method:**
```bash
# From AsciiDoc (test results report)
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc
```

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

#### Viewing Raw Reports

```bash
# View verification JSON (with pretty printing)
cat reports/documentation/verification/batch_verification.json | jq .

# View result YAML
cat reports/documentation/results/TC_001_result.yaml

# View container YAML
cat reports/documentation/results/results_container.yaml

# View AsciiDoc report (raw source)
less reports/documentation/reports/test_results_report.adoc

# View Markdown test plan (raw source)
cat reports/documentation/reports/TC_001_test_plan.md
```

#### Converting and Viewing HTML/PDF

```bash
# Convert AsciiDoc to HTML
asciidoctor reports/documentation/reports/test_results_report.adoc

# Open HTML in browser (macOS)
open reports/documentation/reports/test_results_report.html

# Open HTML in browser (Linux)
xdg-open reports/documentation/reports/test_results_report.html

# Convert AsciiDoc to PDF
asciidoctor-pdf reports/documentation/reports/test_results_report.adoc

# Open PDF in viewer
open reports/documentation/reports/test_results_report.pdf

# Convert Markdown to HTML
pandoc reports/documentation/reports/TC_001_test_plan.md \
  -o TC_001_test_plan.html \
  --standalone

# Open Markdown HTML in browser
open TC_001_test_plan.html
```

#### Batch Conversion

```bash
# Convert all AsciiDoc reports to HTML
for adoc in reports/documentation/reports/*.adoc; do
    asciidoctor "$adoc"
done

# Convert all Markdown test plans to HTML
for md in reports/documentation/reports/*_test_plan.md; do
    basename=$(basename "$md" .md)
    pandoc "$md" -o "reports/documentation/reports/${basename}.html" --standalone
done

# Convert all AsciiDoc reports to PDF
for adoc in reports/documentation/reports/*.adoc; do
    asciidoctor-pdf "$adoc"
done
```

---

## Schema Compatibility

### Overview

The test-plan-documentation-generator requires YAML files to conform to specific schemas for proper report generation. The Test Case Manager includes schema validation tools to ensure compatibility.

### Container YAML Schema

Container YAML files must include:

**Required Fields:**
- `title`: Report title
- `project`: Project name
- `test_date`: Test execution date (ISO 8601 format)
- `test_results`: Array of test result objects
- `metadata`: Execution metadata

**Example:**
```yaml
title: 'Test Execution Results Report'
project: 'My Project'
test_date: '2024-01-15T10:00:00Z'
test_results:
  - test_case_id: "TC_001"
    description: "Test description"
    # ... test results ...
metadata:
  environment: 'Test Environment'
  platform: 'Test Platform'
  executor: 'Test Framework'
  execution_duration: 123.45
  total_test_cases: 10
  passed_test_cases: 8
  failed_test_cases: 2
```

### Test Result Schema

Individual test result YAML files must include:

**Required Fields:**
- `type`: Must be set to `"result"`
- `test_case_id`: Unique test case identifier
- `description`: Test case description
- `sequences`: Array of test sequence results
- `total_steps`: Total number of steps
- `passed_steps`: Number of passed steps
- `failed_steps`: Number of failed steps
- `not_executed_steps`: Number of not executed steps
- `overall_pass`: Boolean indicating overall pass/fail

**Example:**
```yaml
type: result
test_case_id: "TC_001"
description: "Example test case"
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

### Validating Compatibility

The project includes a compatibility checker tool:

```bash
# Build the compatibility checker
cargo build --bin test-plan-documentation-generator-compat

# Validate a single container file
cargo run --bin test-plan-documentation-generator-compat -- \
  validate container.yaml

# Batch validate multiple files
cargo run --bin test-plan-documentation-generator-compat -- \
  batch testcases/expected_output_reports/*.yml

# Test against verifier scenarios
cargo run --bin test-plan-documentation-generator-compat -- \
  test-verifier-scenarios

# Generate compatibility report
cargo run --bin test-plan-documentation-generator-compat -- \
  report --output compatibility_report.md
```

### Common Schema Issues

#### 1. Missing Required Fields

**Problem:**
```yaml
# Missing 'type' field
test_case_id: "TC_001"
description: "Test"
```

**Solution:**
```yaml
type: result  # Add required 'type' field
test_case_id: "TC_001"
description: "Test"
```

#### 2. Invalid Date Format

**Problem:**
```yaml
test_date: '2024-01-15'  # Missing time component
```

**Solution:**
```yaml
test_date: '2024-01-15T10:00:00Z'  # Use ISO 8601 format
```

#### 3. Missing Metadata Fields

**Problem:**
```yaml
metadata:
  environment: 'Test'
  # Missing other required fields
```

**Solution:**
```yaml
metadata:
  environment: 'Test Environment'
  platform: 'Test Platform'
  executor: 'Test Framework'
  execution_duration: 0.0
  total_test_cases: 0
  passed_test_cases: 0
  failed_test_cases: 0
```

### Schema Documentation

For detailed schema specifications, see:
- Test Case Schema: `schemas/test-case.schema.json`
- Execution Log Schema: `schemas/execution-log.schema.json`
- Verification Result Schema: `schemas/verification-result.schema.json`
- Verification Output Schema: `schemas/verification-output.schema.json`
- Schema Documentation: `schemas/README.md`

### Compatibility Testing

Run the compatibility test suite:

```bash
# Test container YAML compatibility
make test-container-compat

# This runs:
# 1. Schema validation
# 2. Compatibility checks with test-plan-doc-gen
# 3. Verifier scenario testing
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

#### 1. Missing test-plan-documentation-generator Binary

**Problem:**
```
✗ Error: test-plan-documentation-generator binary not found
Command not found: test-plan-documentation-generator
```

**Solutions:**

**Option A: Install from crates.io (Recommended)**
```bash
# Install globally
cargo install test-plan-documentation-generator

# Verify installation
which test-plan-documentation-generator
test-plan-documentation-generator --version
```

**Option B: Build from Source**
```bash
# Clone repository
git clone <test-plan-documentation-generator-repo-url>
cd test-plan-documentation-generator

# Build and install
cargo build --release
cargo install --path .
```

**Option C: Specify Custom Path**
```bash
# Set environment variable
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator

# Or use with script
./scripts/generate_documentation_reports.sh \
  --test-plan-doc-gen /path/to/test-plan-documentation-generator
```

**Option D: Use PATH Binary**
```bash
# Add to PATH if installed elsewhere
export PATH=$PATH:/path/to/binary/directory

# Verify
which test-plan-documentation-generator
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

#### 9. HTML/PDF Conversion Issues

**Problem:**
```
asciidoctor: command not found
```

**Solution:**
```bash
# Install asciidoctor (for HTML)
gem install asciidoctor

# Or via package manager
# Ubuntu/Debian
sudo apt-get install asciidoctor

# macOS
brew install asciidoctor
```

**Problem:**
```
asciidoctor-pdf: command not found
```

**Solution:**
```bash
# Install asciidoctor-pdf (for PDF)
gem install asciidoctor-pdf

# Verify installation
which asciidoctor-pdf
asciidoctor-pdf --version
```

#### 10. Report Generation Workflow Issues

**Problem:**
```
No valid container YAML found
```

**Solution:**
```bash
# Check container file structure
cat testcases/expected_output_reports/container_data.yml

# Validate against schema
cargo run --bin test-plan-documentation-generator-compat -- \
  validate testcases/expected_output_reports/container_data.yml

# Use custom container template
./scripts/generate_documentation_reports.sh \
  --container-template /path/to/custom_container.yml
```

**Problem:**
```
test-plan-documentation-generator failed with exit code 1
```

**Solution:**
```bash
# Enable verbose mode for detailed error messages
export VERBOSE=1
./scripts/generate_documentation_reports.sh

# Run test-plan-documentation-generator directly to see errors
test-plan-documentation-generator \
  --container container.yaml \
  --output report.adoc \
  --format asciidoc \
  --verbose

# Check input file format
cat container.yaml | less

# Validate YAML syntax
yamllint container.yaml
```

### Workflow Troubleshooting

#### Complete Report Generation Pipeline

If reports are not generating correctly, follow this diagnostic workflow:

**Step 1: Verify Dependencies**
```bash
# Check all required tools are installed
which test-plan-documentation-generator
which python3
python3 -c "import yaml; print('PyYAML installed')"

# Optional tools
which asciidoctor
which pandoc
```

**Step 2: Verify Input Files**
```bash
# Check execution logs exist
ls -la testcases/verifier_scenarios/*.log

# Check test case files exist
ls -la testcases/*.yml

# Validate test case schema
cargo run --bin validate-yaml -- \
  --schema schemas/test-case.schema.json \
  testcases/TC_001.yml
```

**Step 3: Run Pipeline Step by Step**
```bash
# 1. Run verifier
cargo run --bin verifier -- \
  --folder testcases/verifier_scenarios \
  --format json \
  --output /tmp/verification.json \
  --test-case-dir testcases \
  --verbose

# 2. Convert to YAML
python3 scripts/convert_verification_to_result_yaml.py \
  /tmp/verification.json \
  -o /tmp/results \
  -v

# 3. Create container (manual check)
cat /tmp/results/*_result.yaml

# 4. Generate report
test-plan-documentation-generator \
  --container /tmp/container.yaml \
  --output /tmp/report.adoc \
  --format asciidoc
```

**Step 4: Check Output**
```bash
# Verify report files were created
ls -la reports/documentation/reports/

# Check file content
head -20 reports/documentation/reports/test_results_report.adoc

# Validate AsciiDoc syntax
asciidoctor --safe -o /dev/null \
  reports/documentation/reports/test_results_report.adoc
```

### Getting Help

If you encounter issues not covered here:

1. **Check Installation:** Verify test-plan-documentation-generator is properly installed
   ```bash
   test-plan-documentation-generator --version
   test-plan-documentation-generator --help
   ```

2. **Check Schema Documentation:** `schemas/README.md`

3. **Review Script Help:** 
   ```bash
   ./scripts/generate_documentation_reports.sh --help
   ```

4. **Examine Examples:** `testcases/expected_output_reports/`

5. **Run Tests:** 
   ```bash
   make test
   make test-container-compat
   ```

6. **Check AGENTS.md:** For build, lint, and test commands

7. **Enable Verbose Mode:**
   ```bash
   export VERBOSE=1
   ./scripts/generate_documentation_reports.sh
   ```

8. **Review Compatibility Documentation:** `docs/TEST_PLAN_DOC_GEN_COMPATIBILITY.md`

### Known Limitations

1. **test-plan-documentation-generator Dependency:** The report generation system requires `test-plan-documentation-generator` to be installed via cargo or available in PATH.

2. **Container Template Structure:** The container template must follow the exact YAML structure expected by test-plan-documentation-generator. See schema documentation for details.

3. **Result YAML Format:** Result YAML files must include the `type: result` field to be valid. This field is automatically added by the conversion script.

4. **Execution Log Format:** Verifier expects execution logs in a specific format with TEST_SEQUENCE, STEP, EXIT_CODE, and TIMESTAMP markers.

5. **File Naming:** Result files are named `{test_case_id}_result.yaml`. Test case IDs must be valid filenames (no special characters like `/`, `\`, `:`, etc.).

6. **HTML/PDF Dependencies:** Converting AsciiDoc to HTML or PDF requires asciidoctor or asciidoctor-pdf to be installed separately (via RubyGems).

7. **Markdown to HTML:** Converting Markdown test plans to HTML requires pandoc to be installed separately.

8. **Python Dependency:** PyYAML is required for JSON to YAML conversion. This is the only remaining Python dependency after removal of reportlab.

### Migration from Python-based PDF Generation

If you were previously using Python-based PDF generation:

1. **Removed Dependencies:**
   - `reportlab` - No longer required
   - `scripts/generate_verifier_reports.py` - Removed

2. **New Dependencies:**
   - `test-plan-documentation-generator` - Required (Rust-based)
   - `asciidoctor` or `asciidoctor-pdf` - Optional (for HTML/PDF conversion)

3. **Workflow Changes:**
   - Reports are generated in AsciiDoc/Markdown format by default
   - Use asciidoctor to convert to HTML/PDF if needed
   - No Python code runs for report generation (only for JSON to YAML conversion)

4. **Benefits:**
   - Faster report generation
   - Better maintainability
   - Native Rust integration
   - More output format options
   - No Python packaging issues

---

## Additional Resources

### Documentation

- **Schema Documentation:** `schemas/README.md` - Detailed schema specifications
- **Verifier Usage:** `docs/TEST_VERIFY_USAGE.md` - Test verification tool documentation
- **Test Verification Workflow:** `docs/TEST_VERIFY_WORKFLOW.md` - Verification workflow guide
- **Validation Quick Reference:** `docs/VALIDATE_YAML_QUICK_REF.md` - YAML validation guide
- **Compatibility Documentation:** `docs/TEST_PLAN_DOC_GEN_COMPATIBILITY.md` - Container YAML compatibility
- **GitLab CI Examples:** `docs/GITLAB_CI_EXAMPLES.md` - CI/CD integration examples
- **AGENTS.md:** Build, lint, and test commands

### Schema Files

- **Test Case Schema:** `schemas/test-case.schema.json` - Test case YAML structure
- **Execution Log Schema:** `schemas/execution-log.schema.json` - Execution log JSON structure
- **Verification Result Schema:** `schemas/verification-result.schema.json` - Verification result structure
- **Verification Output Schema:** `schemas/verification-output.schema.json` - Verifier output structure

### Example Files

- **Container Template:** `testcases/expected_output_reports/container_data.yml`
- **Test Case Examples:** `testcases/expected_output_reports/sample_gsma_*.yml`
- **Verifier Scenarios:** `testcases/verifier_scenarios/*.log`

### External Tools

- **test-plan-documentation-generator:** `cargo install test-plan-documentation-generator`
- **asciidoctor:** `gem install asciidoctor` or `brew install asciidoctor`
- **asciidoctor-pdf:** `gem install asciidoctor-pdf`
- **pandoc:** `brew install pandoc` or `apt-get install pandoc`

## Related Makefile Targets

```bash
# Building
make build                     # Build all binaries including verifier

# Report Generation
make generate-docs             # Generate docs for verifier_scenarios
make generate-docs-all         # Generate docs for all testcases

# Validation and Testing
make test                      # Run full test suite
make test-container-compat     # Test container YAML compatibility
make verify-scripts            # Verify shell script syntax

# Linting
make lint                      # Run linter on Rust code
```

## Summary

The test-plan-documentation-generator (tpdg) provides a comprehensive, high-performance solution for generating professional test documentation from YAML test cases and verification results. Key benefits include:

✅ **Performance** - Rust-based implementation is significantly faster than Python  
✅ **Maintainability** - Single codebase for all report generation  
✅ **Multiple Formats** - AsciiDoc, Markdown, and conversion to HTML/PDF  
✅ **Schema Validation** - Built-in compatibility checking  
✅ **CI/CD Integration** - Easy integration with automated pipelines  
✅ **No Python Dependencies** - Only PyYAML for conversion script  

For questions or issues, refer to the [Troubleshooting](#troubleshooting) section or check the compatibility documentation at `docs/TEST_PLAN_DOC_GEN_COMPATIBILITY.md`.
