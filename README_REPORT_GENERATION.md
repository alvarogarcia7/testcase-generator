# Verifier Scenario Report Generation Guide

This guide explains how to generate documentation reports for the verifier test scenarios.

## Overview

The report generation process uses the Rust-based **test-plan-documentation-generator** tool to create AsciiDoc and Markdown reports from test cases and verification results.

### Migration Notice

**Python-based PDF generation has been removed.** The legacy `scripts/generate_verifier_reports.py` script and its `reportlab` dependency have been removed in favor of the more maintainable and performant Rust-based solution.

## Prerequisites

```bash
# Ensure Rust/Cargo is installed and binaries can be built
cargo build --release --bin verifier

# The test-plan-documentation-generator tool should be available as a sibling directory
# or specified via the TESTPLAN_DOC_GEN_DIR environment variable
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

```bash
# Generate documentation reports for verifier scenarios
make generate-docs

# Generate documentation reports for all test cases
make generate-docs-all
```

## Report Generation Process

The report generation consists of these steps:

1. **Verification** - Run verifier on execution logs to analyze test results
2. **Conversion** - Convert verification JSON to result YAML files
3. **Report Generation** - Use test-plan-documentation-generator to create AsciiDoc and Markdown reports

## Output

Reports are generated in `reports/verifier_scenarios/` and include:

- **Verification JSON** - Detailed verification results for each scenario
- **AsciiDoc Reports (.adoc)** - Structured documentation format
- **Markdown Reports (.md)** - GitHub-compatible documentation

## Manual Execution

You can also run the report generation manually:

```bash
# Run the verifier and generate reports
./scripts/run_verifier_and_generate_reports.sh

# Or use the full documentation report generator
./scripts/generate_documentation_reports.sh \
    --logs-dir testcases/verifier_scenarios \
    --test-case-dir testcases \
    --output-dir reports/documentation
```

## Benefits of test-plan-documentation-generator

- **Better Performance** - Rust-based implementation is faster than Python
- **Maintainability** - Native integration with the Rust test framework
- **Multiple Formats** - Supports AsciiDoc and Markdown output
- **No External Dependencies** - No need for Python reportlab library
- **Consistent Reports** - Uniform report generation across all test scenarios

## Troubleshooting

### test-plan-documentation-generator not found

```
⚠ test-plan-doc-gen directory not found
```

**Solution:** Clone the test-plan-doc-gen repository as a sibling directory:

```bash
cd ..
git clone <test-plan-doc-gen-repo-url> test-plan-doc-gen
cd testcase-generator
```

Or specify its location:

```bash
export TESTPLAN_DOC_GEN_DIR=/path/to/test-plan-doc-gen
```

## See Also

- [AGENTS.md](AGENTS.md) - Report Generation section
- [README.md](README.md) - Report Generation section
- [scripts/generate_documentation_reports.sh](scripts/generate_documentation_reports.sh) - Main report generation script
- [scripts/run_verifier_and_generate_reports.sh](scripts/run_verifier_and_generate_reports.sh) - Verifier scenario report generation
