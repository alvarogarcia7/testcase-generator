# Requirement Coverage Tool - Quick Start Guide

This guide will help you get started with the `req-coverage` tool to generate requirement coverage reports.

## Prerequisites

- Rust toolchain installed
- Test case YAML files in the standard format
- Verification container YAML files from test execution

## Installation

Build the `req-coverage` tool from the workspace root:

```bash
cargo build --release -p req-coverage
```

The binary will be available at `target/release/req-coverage`.

## Quick Start

### Step 1: Verify Test Cases and Results Exist

Ensure you have:
- Test case YAML files (e.g., in `./testcases/`)
- Verification container YAML files (e.g., in `./verification_results/`)

Example directory structure:
```
./testcases/
  ├── TC_001.yml
  ├── TC_002.yml
  └── TC_003.yml

./verification_results/
  ├── TC_001_container.yaml
  ├── TC_002_container.yaml
  └── TC_003_container.yaml
```

### Step 2: Generate Coverage Report (JSON)

Run the `verify` command to analyze test cases and generate a JSON coverage report:

```bash
./target/release/req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./verification_results \
  --output req-coverage.json
```

**Expected output:**
```
INFO  req-coverage::coverage > Starting coverage analysis
INFO  req-coverage::coverage > Loaded 3 test cases
INFO  req-coverage::coverage > Loaded 3 verification results
INFO  req-coverage::coverage > Coverage analysis complete
INFO  req-coverage::coverage >   Total requirements: 2
INFO  req-coverage::coverage >   Fully covered: 1
INFO  req-coverage::coverage >   Partially covered: 1
INFO  req-coverage::coverage >   Uncovered: 0
INFO  req-coverage::report   > Coverage report written to: "req-coverage.json"
```

This creates a JSON file with detailed coverage information.

### Step 3: Generate HTML Report

Run the `print` command to convert the JSON to an interactive HTML report:

```bash
./target/release/req-coverage print \
  --format html \
  --input req-coverage.json \
  --output ./coverage-report/
```

**Expected output:**
```
INFO  req-coverage::report > Loading coverage report from: "req-coverage.json"
INFO  req-coverage::report > Generating HTML report to: "./coverage-report/"
INFO  req-coverage::report > HTML report written to: "./coverage-report/index.html"
INFO  req-coverage           > HTML report available at: "./coverage-report/index.html"
```

### Step 4: View the Report

Open the HTML report in your browser:

```bash
# macOS
open ./coverage-report/index.html

# Linux
xdg-open ./coverage-report/index.html

# Windows
start ./coverage-report/index.html
```

## Understanding the Report

### Dashboard Overview

The HTML report displays:
- **Total Requirements**: All unique requirements found in test cases
- **Fully Covered**: Requirements with full coverage and test cases
- **Partially Covered**: Requirements with partial coverage
- **Uncovered**: Requirements with no test cases

### Requirements Table

The interactive table shows:
- **Requirement ID**: Click to expand details
- **Coverage Type**: Full or Partial
- **Status**: Color-coded badge (green/red/yellow/orange/gray)
- **Test Cases**: Number of test cases covering this requirement
- **Pass/Fail**: Ratio of passed to failed tests

### Status Colors

- 🟢 **Green (Covered - All Passed)**: Fully covered, all tests passed
- 🔴 **Red (Covered - Some Failed)**: Fully covered, some tests failed
- 🟡 **Yellow (Partial - All Passed)**: Partially covered, all tests passed
- 🟠 **Orange (Partial - Some Failed)**: Partially covered, some tests failed
- ⚪ **Gray (No Coverage)**: No test coverage

## Common Options

### Verbose Output

Add `--verbose` or `--log-level debug` for detailed logging:

```bash
./target/release/req-coverage verify \
  --verbose \
  --test-cases-folder ./testcases \
  --test-results-folder ./verification_results \
  --output req-coverage.json
```

### Help

View all available options:

```bash
./target/release/req-coverage --help
./target/release/req-coverage verify --help
./target/release/req-coverage print --help
```

## Example Workflow

Complete end-to-end example:

```bash
# 1. Build the tool
cargo build --release -p req-coverage

# 2. Generate coverage report
./target/release/req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./test-acceptance/verification_results \
  --output coverage.json

# 3. Generate HTML report
./target/release/req-coverage print \
  --format html \
  --input coverage.json \
  --output ./public/coverage

# 4. View the report
open ./public/coverage/index.html
```

## Troubleshooting

### No test cases found

**Problem**: "Loaded 0 test cases"

**Solution**: 
- Ensure test case files have `.yaml` or `.yml` extension
- Verify the `--test-cases-folder` path is correct
- Check that YAML files are valid test case format with `requirement` field

### No verification results found

**Problem**: "Loaded 0 verification results"

**Solution**:
- Ensure verification files end with `_container.yaml` or `_container.yml`
- Verify the `--test-results-folder` path is correct
- Check that container files have `test_results` array

### All requirements show as "Not Executed"

**Problem**: Test cases load but show gray status

**Solution**:
- Ensure verification container files have matching `test_case_id` fields
- Check that test case IDs in YAML files match those in verification results
- Verify verification containers have `overall_pass` field

### HTML report not displaying correctly

**Problem**: HTML file opens but looks broken

**Solution**:
- Ensure the entire HTML file was written (check file size > 0)
- Try opening in a different browser
- Check browser console for JavaScript errors

## Next Steps

- Read the [Product Requirements Document](PRD_REQ_COVERAGE.md) for detailed specifications
- See the [README](../crates/req-coverage/README.md) for advanced usage
- Explore the [crate architecture](../crates/README.md) to understand how it fits into the workspace

## CI/CD Integration

Example GitLab CI configuration:

```yaml
coverage-report:
  stage: report
  script:
    - cargo build --release -p req-coverage
    - ./target/release/req-coverage verify --test-cases-folder testcases --test-results-folder verification_results --output coverage.json
    - ./target/release/req-coverage print --format html --input coverage.json --output public/coverage
  artifacts:
    paths:
      - public/coverage
      - coverage.json
    expire_in: 30 days
  only:
    - main
    - merge_requests
```

This will generate coverage reports on every merge request and make them available as pipeline artifacts.
