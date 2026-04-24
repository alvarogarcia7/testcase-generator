# Integration Test Results

This directory contains the results from running the shell-based integration tests.

## Contents

After running `./test_runner.sh`, this directory will contain:

### JSON Coverage Reports
- `test_full_coverage_single.json` - Full coverage with single test case
- `test_partial_coverage.json` - Partial coverage with multiple tests  
- `test_invalid_covers.json` - Invalid covers string detection
- `test_without_requirements.json` - Backward compatibility test
- `test_json_format.json` - JSON requirements format test
- `test_multiple_requirements.json` - Multiple requirements handling
- `test_with_failures.json` - Coverage with test failures
- `test_case_sensitive.json` - Case-sensitive matching test
- `test_duplicates.json` - Duplicate covers strings test

### Command Output Logs
- `*.log` files - STDOUT/STDERR from each test run
- Useful for debugging test failures

### HTML Report Output
- `test_html_output/` - Generated HTML report directory
  - `index.html` - Main HTML report file
  - Contains requirement text, covered portions, and test case details

## Viewing Results

### JSON Reports
```bash
# View coverage report
cat test_full_coverage_single.json | jq .

# Check coverage statistics
cat test_full_coverage_single.json | jq '.total_requirements, .fully_covered_requirements'

# View requirement details
cat test_full_coverage_single.json | jq '.requirements[0]'
```

### HTML Report
```bash
# Open in browser
open test_html_output/index.html
```

### Logs
```bash
# View test output
cat test_full_coverage_single.log

# Search for errors
grep -i error *.log
```

## Cleanup

These files are auto-generated and can be safely deleted:

```bash
rm -rf *.json *.log test_html_output/
```

They will be regenerated the next time you run `./test_runner.sh`.

## Note

This directory is excluded from version control (see `.gitignore`). Results are generated locally and are specific to each test run.
