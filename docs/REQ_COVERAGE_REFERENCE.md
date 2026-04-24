# req-coverage - Quick Reference

## Commands

### verify - Generate Coverage Report

```bash
req-coverage verify \
  --test-cases-folder <PATH> \
  --test-results-folder <PATH> \
  --output <FILE>
```

**Required Arguments**:
- `--test-cases-folder`: Directory with test case YAML files
- `--test-results-folder`: Directory with container YAML files
- `--output`: Output JSON file path

**Optional Arguments**:
- `--verbose`: Enable debug logging
- `--log-level <LEVEL>`: Set log level (trace/debug/info/warn/error)

**Example**:
```bash
req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./verification_results \
  --output coverage.json
```

### print - Generate HTML Report

```bash
req-coverage print \
  --format html \
  --input <FILE> \
  --output <DIR>
```

**Required Arguments**:
- `--format`: Output format (currently only `html`)
- `--input`: Path to coverage JSON file
- `--output`: Output directory for HTML

**Optional Arguments**:
- `--verbose`: Enable debug logging
- `--log-level <LEVEL>`: Set log level

**Example**:
```bash
req-coverage print \
  --format html \
  --input coverage.json \
  --output ./coverage-html
```

## Coverage Types

| Type | Description |
|------|-------------|
| `full` | Test case covers entire requirement |
| `partial` | Test case covers specific aspect of requirement |

## Coverage Status

| Status | Color | Meaning |
|--------|-------|---------|
| `covered_pass` | 🟢 Green | Fully covered, all tests passed |
| `covered_fail` | 🔴 Red | Fully covered, some tests failed |
| `partial_covered_pass` | 🟡 Yellow | Partially covered, all tests passed |
| `partial_covered_fail` | 🟠 Orange | Partially covered, some tests failed |
| `uncovered` | ⚪ Gray | No test coverage |

## Test Status

| Status | Color | Meaning |
|--------|-------|---------|
| `pass` | 🟢 Green | Test passed verification |
| `fail` | 🔴 Red | Test failed verification |
| `not_executed` | ⚪ Gray | Test not executed or no results |

## File Formats

### Test Case YAML (Future Enhancement)

```yaml
requirement: REQ-001
requirement_coverage:
  type: full  # or partial
  covers: "Optional description for partial"
```

**Note**: Currently defaults to `full` coverage based on `requirement` field.

### Verification Container YAML

```yaml
title: Test Results
test_results:
  - test_case_id: TC-001
    description: Test description
    overall_pass: true
```

**Required Fields**:
- `test_results`: Array of test case results
- `test_case_id`: Must match test case ID from YAML
- `overall_pass`: Boolean pass/fail status

### Coverage JSON Output

```json
{
  "generated_at": "2024-01-15T10:30:00Z",
  "total_requirements": 10,
  "fully_covered_requirements": 7,
  "partially_covered_requirements": 2,
  "uncovered_requirements": 1,
  "requirements": [
    {
      "requirement_id": "REQ-001",
      "coverage_type": "full",
      "test_cases": [...],
      "status": "covered_pass"
    }
  ]
}
```

## HTML Report Structure

```
coverage-html/
└── index.html          # Self-contained HTML report
```

**Features**:
- Responsive design
- Interactive expandable rows
- Embedded CSS and JavaScript
- No external dependencies

## Common Workflows

### Basic Workflow

```bash
# 1. Build
cargo build --release -p req-coverage

# 2. Generate JSON report
./target/release/req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./results \
  --output coverage.json

# 3. Generate HTML
./target/release/req-coverage print \
  --format html \
  --input coverage.json \
  --output ./coverage-html

# 4. View
open ./coverage-html/index.html
```

### With Verbose Logging

```bash
req-coverage verify \
  --verbose \
  --test-cases-folder ./testcases \
  --test-results-folder ./results \
  --output coverage.json
```

### Custom Log Level

```bash
req-coverage verify \
  --log-level debug \
  --test-cases-folder ./testcases \
  --test-results-folder ./results \
  --output coverage.json
```

### Environment Variable Logging

```bash
RUST_LOG=debug req-coverage verify \
  --test-cases-folder ./testcases \
  --test-results-folder ./results \
  --output coverage.json
```

## Troubleshooting

### Problem: No test cases found

```
Loaded 0 test cases
```

**Solutions**:
- Check path: `--test-cases-folder` exists
- Verify files have `.yaml` or `.yml` extension
- Ensure YAML files have `requirement` field
- Use `--verbose` to see which files are scanned

### Problem: No verification results

```
Loaded 0 verification results
```

**Solutions**:
- Check path: `--test-results-folder` exists
- Verify files end with `_container.yaml` or `_container.yml`
- Ensure YAML has `test_results` array
- Check file permissions

### Problem: All tests show "not executed"

**Solutions**:
- Verify `test_case_id` fields match between test cases and results
- Check that verification containers have `overall_pass` field
- Use `--verbose` to see ID matching

### Problem: HTML report is blank

**Solutions**:
- Check JSON file is valid: `cat coverage.json | jq .`
- Verify output directory exists and is writable
- Check browser console for JavaScript errors
- Try different browser

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (invalid arguments, file not found, etc.) |

## Performance Notes

- **1000+ test cases**: Typically < 10 seconds
- **HTML generation**: Typically < 5 seconds
- **Memory usage**: Scales linearly with test case count

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `RUST_LOG` | Set log level | `RUST_LOG=debug` |

**Note**: `RUST_LOG` overrides `--log-level` flag.

## Dependencies

**Workspace Crates**:
- testcase-models
- testcase-common
- testcase-storage

**External Crates**:
- clap (CLI)
- serde, serde_json, serde_yaml (serialization)
- chrono (timestamps)
- walkdir (file discovery)

## Additional Resources

- [Product Requirements Document](PRD_REQ_COVERAGE.md)
- [Quick Start Guide](REQ_COVERAGE_QUICK_START.md)
- [Crate README](../crates/req-coverage/README.md)
- [Implementation Summary](../REQ_COVERAGE_IMPLEMENTATION.md)
