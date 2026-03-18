# Sample Generation Quick Start Guide

This guide provides quick commands for generating and using sample test cases with documentation reports in both AsciiDoc and Markdown formats.

## Quick Commands

### 1. Generate All Samples

```bash
./scripts/generate_all_sample_cases.sh
```

**Output:** 8 sample test cases in `testcases/generated_samples/`

### 2. View Generated Samples

```bash
# List all generated samples
find testcases/generated_samples -name "*.yml" -type f

# View a specific sample
cat testcases/generated_samples/successful/SAMPLE_SUCCESS_001.yml
```

### 3. Check Execution Logs

```bash
# List all execution logs
find testcases/generated_samples -name "*_execution_log.json" -type f

# View a specific log
cat testcases/generated_samples/successful/SAMPLE_SUCCESS_001_execution_log.json
```

### 4. Run Verifier on Samples

```bash
# Verify all samples in batch mode
cargo run --bin verifier -- \
  --folder testcases/generated_samples \
  --format json \
  --output reports/generated_samples/batch_verification.json \
  --test-case-dir testcases/generated_samples
```

### 5. Generate Reports (Both Formats)

```bash
# Complete workflow: generate, execute, verify, and report
./scripts/run_all_samples_and_generate_reports.sh --format both
```

**Output:**
- `reports/generated_samples/docs/sample_execution_results.adoc`
- `reports/generated_samples/docs/sample_execution_results.md`

### 6. Generate Only AsciiDoc Report

```bash
./scripts/run_all_samples_and_generate_reports.sh --format asciidoc
```

**Output:**
- `reports/generated_samples/docs/sample_execution_results.adoc`

### 7. Generate Only Markdown Report

```bash
./scripts/run_all_samples_and_generate_reports.sh --format markdown
```

**Output:**
- `reports/generated_samples/docs/sample_execution_results.md`

## Sample Categories

### 1. Successful Execution
**Location:** `testcases/generated_samples/successful/`
- `SAMPLE_SUCCESS_001.yml` - All steps pass

### 2. Failed First Step
**Location:** `testcases/generated_samples/failed_first/`
- `SAMPLE_FAILED_FIRST_001.yml` - First step fails, rest not executed

### 3. Failed Intermediate Step
**Location:** `testcases/generated_samples/failed_intermediate/`
- `SAMPLE_FAILED_INTERMEDIATE_001.yml` - Step 3 fails after 2 successful steps

### 4. Failed Last Step
**Location:** `testcases/generated_samples/failed_last/`
- `SAMPLE_FAILED_LAST_001.yml` - Final step fails output verification

### 5. Multiple Sequences
**Location:** `testcases/generated_samples/multiple_sequences/`
- `SAMPLE_MULTI_SEQ_001.yml` - 3 sequences with mixed results

### 6. Complex (Variable Capture)
**Location:** `testcases/generated_samples/complex/`
- `SAMPLE_COMPLEX_001.yml` - Variable capture and conditional verification

### 7. Hooks
**Location:** `testcases/generated_samples/hooks/`
- `SAMPLE_HOOK_SCRIPT_START_001.yml` - script_start hook
- `SAMPLE_HOOK_BEFORE_SEQ_001.yml` - before_sequence hook

## Report Locations

After running the complete workflow:

```
reports/generated_samples/
├── verification/
│   ├── batch_verification.json      # JSON verification results
│   └── batch_verification.yaml      # YAML verification results
├── results/
│   ├── *_result.yaml                # Individual result files
│   └── results_container.yaml       # Combined results container
├── execution_logs/
│   └── *_execution_log.json         # Archived execution logs
└── docs/
    ├── sample_execution_results.adoc  # AsciiDoc documentation
    └── sample_execution_results.md    # Markdown documentation
```

## View Reports

### View Markdown Report

```bash
# In terminal with Markdown support
cat reports/generated_samples/docs/sample_execution_results.md

# Or open in browser (macOS)
open reports/generated_samples/docs/sample_execution_results.md

# Or open in browser (Linux)
xdg-open reports/generated_samples/docs/sample_execution_results.md
```

### View AsciiDoc Report

```bash
# In terminal
cat reports/generated_samples/docs/sample_execution_results.adoc

# Convert to HTML and view
asciidoctor reports/generated_samples/docs/sample_execution_results.adoc
open reports/generated_samples/docs/sample_execution_results.html

# Convert to PDF
asciidoctor-pdf reports/generated_samples/docs/sample_execution_results.adoc
open reports/generated_samples/docs/sample_execution_results.pdf
```

## Common Workflows

### Workflow 1: Generate Everything from Scratch

```bash
# Clean previous runs
rm -rf testcases/generated_samples reports/generated_samples

# Generate and run complete workflow
./scripts/run_all_samples_and_generate_reports.sh
```

### Workflow 2: Regenerate Reports Only

```bash
# Use existing samples and logs, regenerate reports
./scripts/run_all_samples_and_generate_reports.sh \
  --skip-generation \
  --skip-execution \
  --skip-verification
```

### Workflow 3: Update Samples and Regenerate

```bash
# Regenerate samples
./scripts/generate_all_sample_cases.sh

# Run full workflow
./scripts/run_all_samples_and_generate_reports.sh --skip-generation
```

### Workflow 4: Different Output Locations

```bash
# Custom directories
./scripts/run_all_samples_and_generate_reports.sh \
  --samples-dir /tmp/my_samples \
  --reports-dir /tmp/my_reports
```

## Verify Individual Samples

### Verify Successful Sample

```bash
cargo run --bin verifier -- \
  --log testcases/generated_samples/successful/SAMPLE_SUCCESS_001_execution_log.json \
  --test-case SAMPLE_SUCCESS_001 \
  --test-case-dir testcases/generated_samples \
  --format yaml
```

**Expected:** All steps pass, overall result is PASS

### Verify Failed First Sample

```bash
cargo run --bin verifier -- \
  --log testcases/generated_samples/failed_first/SAMPLE_FAILED_FIRST_001_execution_log.json \
  --test-case SAMPLE_FAILED_FIRST_001 \
  --test-case-dir testcases/generated_samples \
  --format yaml
```

**Expected:** Step 1 fails, steps 2-3 not executed, overall result is FAIL

### Verify Multi-Sequence Sample

```bash
cargo run --bin verifier -- \
  --log testcases/generated_samples/multiple_sequences/SAMPLE_MULTI_SEQ_001_execution_log.json \
  --test-case SAMPLE_MULTI_SEQ_001 \
  --test-case-dir testcases/generated_samples \
  --format yaml
```

**Expected:** Sequence 1 passes, sequence 2 fails, sequence 3 not executed

## Inspect Generated Files

### View Test Case Structure

```bash
# Pretty-print YAML
cat testcases/generated_samples/complex/SAMPLE_COMPLEX_001.yml

# Count total steps
grep -c "step:" testcases/generated_samples/complex/SAMPLE_COMPLEX_001.yml
```

### View Execution Log Details

```bash
# Pretty-print JSON
python3 -m json.tool testcases/generated_samples/complex/SAMPLE_COMPLEX_001_execution_log.json

# Count executed steps
jq '. | length' testcases/generated_samples/complex/SAMPLE_COMPLEX_001_execution_log.json
```

### View Verification Results

```bash
# View summary
jq '.summary' reports/generated_samples/verification/batch_verification.json

# Count passed vs failed
jq '.summary | {passed: .passed_test_cases, failed: .failed_test_cases}' \
  reports/generated_samples/verification/batch_verification.json
```

## Troubleshooting

### No Samples Generated

```bash
# Check directory exists
ls -la testcases/generated_samples/

# Regenerate
./scripts/generate_all_sample_cases.sh --verbose
```

### No Execution Logs

```bash
# Check for logs
find testcases/generated_samples -name "*_execution_log.json"

# If missing, execution logs are created manually or by orchestrator
# For this implementation, they are pre-generated
```

### Verifier Errors

```bash
# Check verifier can find test cases
cargo run --bin verifier -- \
  --log testcases/generated_samples/successful/SAMPLE_SUCCESS_001_execution_log.json \
  --test-case SAMPLE_SUCCESS_001 \
  --test-case-dir testcases/generated_samples \
  --format yaml --verbose
```

### Report Not Generated

```bash
# Check Python is available (for JSON processing)
python3 --version

# Check reports directory
ls -la reports/generated_samples/docs/

# Regenerate with verbose output
./scripts/run_all_samples_and_generate_reports.sh --verbose
```

## Integration with CI/CD

### GitHub Actions Example

```yaml
name: Generate Sample Reports

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  generate-reports:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Generate Samples and Reports
        run: |
          ./scripts/run_all_samples_and_generate_reports.sh --format both
      
      - name: Upload Reports
        uses: actions/upload-artifact@v2
        with:
          name: sample-reports
          path: reports/generated_samples/docs/
```

### GitLab CI Example

```yaml
generate-sample-reports:
  stage: test
  script:
    - ./scripts/run_all_samples_and_generate_reports.sh --format both
  artifacts:
    paths:
      - reports/generated_samples/docs/
    expire_in: 1 week
```

## Tips and Best Practices

### 1. Always Use Verbose Mode for Debugging

```bash
./scripts/generate_all_sample_cases.sh --verbose
./scripts/run_all_samples_and_generate_reports.sh --verbose
```

### 2. Archive Reports for Comparison

```bash
# Timestamp reports
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
cp -r reports/generated_samples reports/archive_$TIMESTAMP
```

### 3. Validate Report Quality

```bash
# Check AsciiDoc syntax
asciidoctor --safe-mode secure --verbose \
  reports/generated_samples/docs/sample_execution_results.adoc

# Validate Markdown
npx markdownlint reports/generated_samples/docs/sample_execution_results.md
```

### 4. Share Reports

```bash
# Serve reports locally
cd reports/generated_samples/docs
python3 -m http.server 8000

# Then open http://localhost:8000 in browser
```

## Summary

This quick start guide covers:

✅ Generating all 8 sample test cases covering major scenarios
✅ Running verification on samples
✅ Generating comprehensive reports in both AsciiDoc and Markdown
✅ Viewing and inspecting generated files
✅ Common workflows for different use cases
✅ Troubleshooting common issues
✅ CI/CD integration examples

For more details, see:
- `IMPLEMENTATION_SAMPLE_GENERATION.md` - Comprehensive implementation details
- `AGENTS.md` - Overall project documentation
- `testcases/verifier_scenarios/README.md` - Verifier scenarios documentation
