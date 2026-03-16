# Sample Test Case Generation - Complete Guide

## Overview

This implementation provides a complete sample generation system that creates comprehensive test cases covering all major execution scenarios, generates execution logs, runs verification, and produces professional documentation reports in both **AsciiDoc** and **Markdown** formats.

## Quick Start

### 1. Generate Sample Test Cases

```bash
./scripts/generate_all_sample_cases.sh
```

This creates 8 sample test cases with execution logs in `testcases/generated_samples/`

### 2. Generate Reports (Both Formats)

```bash
./scripts/run_all_samples_and_generate_reports.sh --format both
```

This runs the complete workflow and generates reports in:
- `reports/generated_samples/docs/sample_execution_results.adoc` (AsciiDoc)
- `reports/generated_samples/docs/sample_execution_results.md` (Markdown)

## What's Included

### 8 Sample Test Cases

| Sample ID | Category | Description | Expected |
|-----------|----------|-------------|----------|
| SAMPLE_SUCCESS_001 | Successful | All steps pass | PASS |
| SAMPLE_FAILED_FIRST_001 | Failed First | First step fails | FAIL |
| SAMPLE_FAILED_INTERMEDIATE_001 | Failed Intermediate | Step 3 fails | FAIL |
| SAMPLE_FAILED_LAST_001 | Failed Last | Last step fails | FAIL |
| SAMPLE_MULTI_SEQ_001 | Multiple Sequences | Mixed results | FAIL |
| SAMPLE_COMPLEX_001 | Complex Features | Variable capture | PASS |
| SAMPLE_HOOK_SCRIPT_START_001 | Hooks | script_start hook | PASS |
| SAMPLE_HOOK_BEFORE_SEQ_001 | Hooks | before_sequence hook | PASS |

### Execution Logs

Each test case has a corresponding `*_execution_log.json` file containing:
- Command execution details
- Exit codes
- Command output
- Timestamps

### Report Formats

#### AsciiDoc Format
- Professional documentation format
- PDF/HTML generation support
- Section numbering and TOC
- Syntax highlighting ready

#### Markdown Format
- GitHub-flavored Markdown
- Universal compatibility
- Clean formatting
- Tables and code blocks

## File Locations

```
testcases/generated_samples/        # Generated test cases
├── successful/
├── failed_first/
├── failed_intermediate/
├── failed_last/
├── multiple_sequences/
├── complex/
└── hooks/

reports/generated_samples/          # Generated reports
├── verification/                   # Verification results
├── results/                        # Individual result files
├── execution_logs/                 # Archived execution logs
└── docs/                          # Documentation reports
    ├── sample_execution_results.adoc
    └── sample_execution_results.md
```

## Usage Examples

### Generate Samples Only

```bash
./scripts/generate_all_sample_cases.sh --verbose
```

### Generate AsciiDoc Report Only

```bash
./scripts/run_all_samples_and_generate_reports.sh --format asciidoc
```

### Generate Markdown Report Only

```bash
./scripts/run_all_samples_and_generate_reports.sh --format markdown
```

### Regenerate Reports from Existing Data

```bash
./scripts/run_all_samples_and_generate_reports.sh \
  --skip-generation \
  --skip-execution \
  --skip-verification
```

## Verify Samples

```bash
# Verify all samples
cargo run --bin verifier -- \
  --folder testcases/generated_samples \
  --format json \
  --test-case-dir testcases/generated_samples

# Verify single sample
cargo run --bin verifier -- \
  --log testcases/generated_samples/successful/SAMPLE_SUCCESS_001_execution_log.json \
  --test-case SAMPLE_SUCCESS_001 \
  --test-case-dir testcases/generated_samples \
  --format yaml
```

## View Reports

### Markdown Report

```bash
# View in terminal
cat reports/generated_samples/docs/sample_execution_results.md

# Open in default viewer (macOS)
open reports/generated_samples/docs/sample_execution_results.md

# Open in browser (Linux)
xdg-open reports/generated_samples/docs/sample_execution_results.md
```

### AsciiDoc Report

```bash
# Convert to HTML and view
asciidoctor reports/generated_samples/docs/sample_execution_results.adoc
open reports/generated_samples/docs/sample_execution_results.html

# Convert to PDF
asciidoctor-pdf reports/generated_samples/docs/sample_execution_results.adoc
open reports/generated_samples/docs/sample_execution_results.pdf
```

## Documentation

### Comprehensive Guides

1. **[IMPLEMENTATION_SAMPLE_GENERATION.md](IMPLEMENTATION_SAMPLE_GENERATION.md)**
   - Complete implementation details
   - Component architecture
   - File structure documentation
   - Customization guide
   - Integration points

2. **[SAMPLE_GENERATION_QUICK_START.md](SAMPLE_GENERATION_QUICK_START.md)**
   - Quick commands
   - Common workflows
   - Troubleshooting
   - CI/CD integration
   - Tips and best practices

3. **[SAMPLE_GENERATION_RESULTS.md](SAMPLE_GENERATION_RESULTS.md)**
   - What was generated
   - File statistics
   - Sample descriptions
   - Coverage analysis
   - Success metrics

## Scripts

### generate_all_sample_cases.sh
Generates all sample test cases and execution logs.

**Options:**
- `--output-dir DIR`: Output directory (default: testcases/generated_samples)
- `--verbose`: Verbose output
- `--help`: Show help

### run_all_samples_and_generate_reports.sh
Complete workflow: generate, execute, verify, and report.

**Options:**
- `--samples-dir DIR`: Samples directory
- `--reports-dir DIR`: Reports directory
- `--skip-generation`: Skip sample generation
- `--skip-execution`: Skip test execution
- `--skip-verification`: Skip verification
- `--format FORMAT`: Report format (both|asciidoc|markdown)
- `--verbose`: Verbose output
- `--help`: Show help

## Sample Descriptions

### Successful Execution (SAMPLE_SUCCESS_001)
Demonstrates complete successful test execution with all steps passing.
- 3 steps, all pass
- Exit code verification
- Output verification

### Failed First Step (SAMPLE_FAILED_FIRST_001)
First step fails, preventing subsequent step execution.
- Step 1: Fails with exit code 2
- Steps 2-3: Not executed

### Failed Intermediate Step (SAMPLE_FAILED_INTERMEDIATE_001)
Step 3 fails after steps 1-2 succeed.
- Steps 1-2: Pass
- Step 3: Fails
- Step 4: Not executed

### Failed Last Step (SAMPLE_FAILED_LAST_001)
All steps execute, but last step fails output verification.
- Steps 1-2: Pass
- Step 3: Fails output match

### Multiple Sequences (SAMPLE_MULTI_SEQ_001)
Three sequences with mixed results.
- Sequence 1: All pass
- Sequence 2: Step 2 fails
- Sequence 3: Not executed

### Complex Features (SAMPLE_COMPLEX_001)
Variable capture and conditional verification.
- Regex variable capture
- Variable usage in commands
- Platform-specific verification

### Hooks (SAMPLE_HOOK_*)
Hook execution at different lifecycle points.
- script_start hook
- before_sequence hook

## Coverage

✅ Successful execution scenarios
✅ Failed first step scenarios
✅ Failed intermediate step scenarios
✅ Failed last step scenarios
✅ Multiple sequence scenarios
✅ Complex features (variables, conditions)
✅ Hook execution scenarios
✅ Both AsciiDoc and Markdown reports

## Integration

### With Verifier
All samples are verifiable using the verifier binary.

### With Orchestrator
All test case YAMLs are valid for orchestrator execution.

### With Documentation Generator
Result container format compatible with test-plan-doc-gen.

## CI/CD Integration

### GitHub Actions

```yaml
- name: Generate Sample Reports
  run: ./scripts/run_all_samples_and_generate_reports.sh --format both

- name: Upload Reports
  uses: actions/upload-artifact@v2
  with:
    name: sample-reports
    path: reports/generated_samples/docs/
```

### GitLab CI

```yaml
generate-reports:
  script:
    - ./scripts/run_all_samples_and_generate_reports.sh --format both
  artifacts:
    paths:
      - reports/generated_samples/docs/
```

## Troubleshooting

### No samples generated
```bash
./scripts/generate_all_sample_cases.sh --verbose
```

### Verifier errors
```bash
cargo run --bin verifier -- --help
```

### Report not generated
```bash
./scripts/run_all_samples_and_generate_reports.sh --verbose
```

## Requirements

- Bash 3.2+ (macOS/Linux)
- Cargo and Rust toolchain
- Python 3 (for JSON processing)
- Optional: asciidoctor, asciidoctor-pdf

## Summary

This implementation provides:

✅ 8 comprehensive sample test cases
✅ Execution logs for all samples
✅ Automated verification workflow
✅ Professional documentation in AsciiDoc format
✅ Universal documentation in Markdown format
✅ Complete workflow automation
✅ Integration with existing tools
✅ Comprehensive documentation

## Getting Help

For more information:
- See `IMPLEMENTATION_SAMPLE_GENERATION.md` for implementation details
- See `SAMPLE_GENERATION_QUICK_START.md` for quick commands
- See `SAMPLE_GENERATION_RESULTS.md` for what was generated
- See `AGENTS.md` for overall project documentation

## License

This implementation is part of the Test Case Manager project.
