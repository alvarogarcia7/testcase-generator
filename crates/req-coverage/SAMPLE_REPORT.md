# Sample Coverage Report

This directory contains a sample HTML coverage report demonstrating the capabilities of the `req-coverage` tool.

## Files

- **`sample_coverage_report.html`** - A sample HTML report showing requirement coverage analysis
- **`generate_sample_report.sh`** - Script to regenerate the sample report from test data

## Viewing the Report

To view the sample HTML report, open `sample_coverage_report.html` in a web browser:

```bash
# macOS
open crates/req-coverage/sample_coverage_report.html

# Linux
xdg-open crates/req-coverage/sample_coverage_report.html

# Or use any web browser
firefox crates/req-coverage/sample_coverage_report.html
```

## Report Features

The HTML report includes:

- **Dashboard Summary**: Overview of total, fully covered, partially covered, and uncovered requirements
- **Interactive Table**: Click on any requirement ID to expand and see details
- **Detailed Breakdown**: 
  - Requirement text and covered portions
  - Test cases covering each requirement
  - Pass/fail status for each test
  - Coverage errors and warnings

## Regenerating the Sample Report

To regenerate the sample report from the latest test data:

```bash
./crates/req-coverage/generate_sample_report.sh
```

This will:
1. Create a sample requirements file
2. Analyze coverage from test cases in `crates/testcase-manager/tests/integration/req_coverage_testdata/`
3. Generate a JSON coverage report
4. Convert the JSON to an HTML report

## Understanding the Report

### Coverage Types

- **Full Coverage**: Test cases cover the complete requirement text
- **Partial Coverage**: Test cases cover only portions of the requirement text

### Status Indicators

- **Green (Covered - All Passed)**: Fully covered requirement with all tests passing
- **Red (Covered - Some Failed)**: Covered requirement with one or more test failures
- **Yellow (Partially Covered - All Passed)**: Partially covered requirement with all tests passing
- **Orange (Partially Covered - Some Failed)**: Partially covered requirement with some test failures
- **Gray (No Coverage)**: Requirement has no associated test cases

### Test Case Details

Each requirement shows:
- Test case IDs
- Test descriptions
- Covered portions (for partial coverage)
- Pass/fail status
- Coverage errors (if any)

## Using in Your Own Projects

To generate coverage reports for your own test cases:

1. Create a requirements file (YAML or JSON):
   ```yaml
   requirements:
     - id: REQ-001
       text: "Your requirement text here"
       description: "Optional description"
   ```

2. Run the coverage analysis:
   ```bash
   cargo run -p req-coverage -- verify \
       --test-cases-folder /path/to/testcases \
       --test-results-folder /path/to/verification_results \
       --requirements-file requirements.yaml \
       --output coverage_report.json
   ```

3. Generate the HTML report:
   ```bash
   cargo run -p req-coverage -- print \
       --format html \
       --input coverage_report.json \
       --output report_html/
   ```

4. Open `report_html/index.html` in your browser

See the main [README.md](README.md) for complete documentation.
