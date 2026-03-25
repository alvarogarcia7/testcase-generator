# Test Comparison Tool Documentation

## Overview

The Test Comparison Tool is a Python-based utility that generates comprehensive JSON reports comparing test execution before and after the crate splitting refactor. It helps analyze:

1. **Test Coverage**: Which tests were executed before and after changes
2. **Test Organization**: After splitting, which crate contains each test
3. **Performance Impact**: Total execution time comparison with percentage change

## Location

- **Script**: `scripts/test_comparison_report.py`
- **Documentation**: `scripts/README_TEST_COMPARISON.md`
- **Quick Start**: `TEST_COMPARISON_QUICK_START.md`
- **Example**: `examples/test_comparison_example.sh`
- **Sample Report**: `examples/test_comparison_report_sample.json`

## Quick Start

### Using Make (Recommended)

```bash
# Generate report automatically
make test-comparison-report

# View the report
cat reports/test_comparison_report.json | jq .
```

### Using Python Script Directly

```bash
# Run tests and generate report
python scripts/test_comparison_report.py --run-tests --output report.json

# Use pre-saved test outputs
python scripts/test_comparison_report.py \
    --before before.txt \
    --after after.txt \
    --output report.json
```

## Use Cases

### 1. Verify Crate Splitting Success

After splitting a monolithic crate into multiple crates, verify that:
- All tests still run
- Tests are properly organized in their respective crates
- No tests were lost in the migration

```bash
make test-comparison-report
jq '.after.tests_by_crate' reports/test_comparison_report.json
```

### 2. Performance Impact Analysis

Measure the performance impact of crate splitting:

```bash
make test-comparison-report
jq '.summary | {
  duration_before: .total_duration_before_seconds,
  duration_after: .total_duration_after_seconds,
  change_percent: .duration_percent_change
}' reports/test_comparison_report.json
```

### 3. Test Migration Verification

Ensure tests were correctly migrated:

```bash
# Check for removed tests (should be 0 or expected)
jq '.summary.removed_tests' reports/test_comparison_report.json

# List any removed tests
jq '.changes.removed_tests[]' reports/test_comparison_report.json
```

### 4. CI/CD Integration

Track test organization changes in your CI pipeline:

```yaml
# .gitlab-ci.yml
test-comparison:
  stage: analysis
  script:
    - make test-comparison-report
  artifacts:
    reports:
      test_comparison_report.json
    paths:
      - reports/test_comparison_report.json
  only:
    - merge_requests
```

## Report Structure

### Complete Report Schema

```json
{
  "metadata": {
    "generated_at": "ISO 8601 timestamp",
    "before_ref": "Git reference or file path",
    "after_ref": "Git reference or file path",
    "tool_version": "Tool version"
  },
  "summary": {
    "tests_before": 150,
    "tests_after": 152,
    "new_tests": 5,
    "removed_tests": 3,
    "common_tests": 147,
    "total_duration_before_seconds": 45.32,
    "total_duration_after_seconds": 42.15,
    "duration_difference_seconds": -3.17,
    "duration_percent_change": -7.0
  },
  "before": {
    "git_ref": "main",
    "total_tests": 150,
    "passed": 148,
    "failed": 2,
    "ignored": 0,
    "filtered": 0,
    "total_duration_seconds": 45.32,
    "tests": [
      {
        "name": "test_name",
        "crate": null,
        "status": "ok|FAILED|ignored",
        "duration_seconds": 0.0
      }
    ]
  },
  "after": {
    "git_ref": "split-binaries-into-crates",
    "total_tests": 152,
    "passed": 150,
    "failed": 2,
    "ignored": 0,
    "filtered": 0,
    "total_duration_seconds": 42.15,
    "tests": [...],
    "tests_by_crate": {
      "crate-name": {
        "test_count": 15,
        "tests": [...]
      }
    }
  },
  "changes": {
    "new_tests": ["test1", "test2"],
    "removed_tests": ["old_test"],
    "common_tests": ["test3", "test4"]
  }
}
```

## Command Reference

### Make Targets

| Target | Description |
|--------|-------------|
| `make test-comparison-report` | Run tests on main and split branches, generate report |
| `make test-comparison-from-files BEFORE=file1 AFTER=file2` | Generate report from saved test outputs |

### Python Script Options

```bash
python scripts/test_comparison_report.py [OPTIONS]

Options:
  --run-tests              Run cargo tests for both states
  --before FILE            Path to before test output
  --after FILE             Path to after test output
  --output FILE            Output path for JSON report (default: test_comparison_report.json)
  --before-ref REF         Git reference for before state (default: main)
  --after-ref REF          Git reference for after state (default: split-binaries-into-crates)
  --verbose                Enable verbose output
  --help                   Show help message
```

## Analysis Examples

### Example 1: Count Tests per Crate

```bash
jq '.after.tests_by_crate | to_entries[] | "\(.key): \(.value.test_count)"' \
    reports/test_comparison_report.json -r
```

Output:
```
bash-eval: 15
json-escape: 3
testcase-models: 25
testcase-manager: 45
testcase-validation: 20
verifier: 18
```

### Example 2: Find Failed Tests

```bash
jq '.after.tests[] | select(.status == "FAILED") | .name' \
    reports/test_comparison_report.json -r
```

### Example 3: Compare Test Counts

```bash
jq '{
  before: .summary.tests_before,
  after: .summary.tests_after,
  difference: (.summary.tests_after - .summary.tests_before),
  new: .summary.new_tests,
  removed: .summary.removed_tests
}' reports/test_comparison_report.json
```

### Example 4: Performance Summary

```bash
jq '.summary | {
  duration_before: .total_duration_before_seconds,
  duration_after: .total_duration_after_seconds,
  improvement_seconds: .duration_difference_seconds,
  improvement_percent: .duration_percent_change
}' reports/test_comparison_report.json
```

### Example 5: List Tests in Specific Crate

```bash
# List all tests in testcase-models crate
jq '.after.tests_by_crate["testcase-models"].tests[].name' \
    reports/test_comparison_report.json -r
```

### Example 6: Generate Markdown Summary

```bash
#!/bin/bash
REPORT="reports/test_comparison_report.json"

echo "# Test Comparison Summary"
echo ""
echo "## Statistics"
echo ""
echo "| Metric | Before | After | Change |"
echo "|--------|--------|-------|--------|"
echo "| Tests | $(jq -r .summary.tests_before $REPORT) | $(jq -r .summary.tests_after $REPORT) | $(jq -r .summary.new_tests $REPORT) new, $(jq -r .summary.removed_tests $REPORT) removed |"
echo "| Duration | $(jq -r .summary.total_duration_before_seconds $REPORT)s | $(jq -r .summary.total_duration_after_seconds $REPORT)s | $(jq -r .summary.duration_percent_change $REPORT)% |"
echo ""
echo "## Tests by Crate"
echo ""
jq -r '.after.tests_by_crate | to_entries[] | "- **\(.key)**: \(.value.test_count) tests"' $REPORT
```

## Integration Examples

### GitHub Actions

```yaml
name: Test Comparison
on: [pull_request]

jobs:
  test-comparison:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0
      
      - name: Setup Python
        uses: actions/setup-python@v4
        with:
          python-version: '3.14'
      
      - name: Generate Comparison Report
        run: make test-comparison-report
      
      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: test-comparison-report
          path: reports/test_comparison_report.json
      
      - name: Comment PR
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const report = JSON.parse(fs.readFileSync('reports/test_comparison_report.json'));
            const summary = report.summary;
            
            const comment = `## Test Comparison Report
            
            **Tests**: ${summary.tests_before} → ${summary.tests_after} (${summary.new_tests} new, ${summary.removed_tests} removed)
            **Duration**: ${summary.total_duration_before_seconds}s → ${summary.total_duration_after_seconds}s (${summary.duration_percent_change}% change)
            
            <details>
            <summary>Tests by Crate</summary>
            
            ${Object.entries(report.after.tests_by_crate).map(([crate, data]) => 
              `- **${crate}**: ${data.test_count} tests`
            ).join('\n')}
            
            </details>`;
            
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });
```

### GitLab CI

```yaml
test-comparison:
  stage: analysis
  script:
    - make test-comparison-report
    - |
      cat > comparison_summary.md << EOF
      ## Test Comparison Report
      
      **Branch**: ${CI_MERGE_REQUEST_SOURCE_BRANCH_NAME}
      
      $(jq -r '.summary | "**Tests**: \(.tests_before) → \(.tests_after) (\(.new_tests) new, \(.removed_tests) removed)"' reports/test_comparison_report.json)
      
      $(jq -r '.summary | "**Duration**: \(.total_duration_before_seconds)s → \(.total_duration_after_seconds)s (\(.duration_percent_change)% change)"' reports/test_comparison_report.json)
      
      ### Tests by Crate
      
      $(jq -r '.after.tests_by_crate | to_entries[] | "- **\(.key)**: \(.value.test_count) tests"' reports/test_comparison_report.json)
      EOF
  artifacts:
    paths:
      - reports/test_comparison_report.json
      - comparison_summary.md
    reports:
      codequality: reports/test_comparison_report.json
  only:
    - merge_requests
```

## Troubleshooting

### Problem: Git checkout fails

**Solution**: Ensure git references exist and are accessible
```bash
git fetch origin
git branch -a | grep -E 'main|split-binaries-into-crates'
```

### Problem: Tests timeout

**Solution**: Use saved test outputs instead
```bash
# Run tests separately
git checkout main
cargo test --workspace > before.txt 2>&1

git checkout split-binaries-into-crates
cargo test --workspace > after.txt 2>&1

# Generate report
make test-comparison-from-files BEFORE=before.txt AFTER=after.txt
```

### Problem: Cannot parse test output

**Solution**: Ensure standard cargo test output format
```bash
cargo test --workspace -- --nocapture > output.txt 2>&1
```

### Problem: Missing crate information

This is normal if cargo doesn't output crate names in test results. Tests will show `"crate": null` in the before state and should show crate names in the after state (workspace structure).

### Problem: jq not found

**Solution**: Install jq for JSON processing
```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq

# Or use Python
python -m json.tool < reports/test_comparison_report.json
```

## Best Practices

1. **Save Current Work**: Stash changes before running with `--run-tests`
   ```bash
   git stash save "temp work"
   make test-comparison-report
   git stash pop
   ```

2. **Large Test Suites**: Use saved outputs to avoid timeouts
   ```bash
   # Run once, save outputs, reuse for analysis
   make test-comparison-from-files BEFORE=saved_before.txt AFTER=saved_after.txt
   ```

3. **CI Integration**: Always generate artifacts for later analysis
   ```yaml
   artifacts:
     paths:
       - reports/test_comparison_report.json
     expire_in: 30 days
   ```

4. **Regular Monitoring**: Run comparison reports periodically to track test organization
   ```bash
   # Add to weekly CI schedule
   schedule:
     - cron: "0 0 * * 0"
   ```

## See Also

- [Test Comparison Quick Start](../TEST_COMPARISON_QUICK_START.md)
- [Test Comparison Script README](../scripts/README_TEST_COMPARISON.md)
- [Example Script](../examples/test_comparison_example.sh)
- [Sample Report](../examples/test_comparison_report_sample.json)
- [Workspace Structure](../AGENTS.md#workspace-structure)
- [Test Commands](../TEST_COMMANDS.md)
