# Test Comparison Report - Quick Start Guide

## Purpose

Compare test execution before and after the crate splitting changes to understand:
- Which tests were executed
- How tests are organized across crates after splitting
- Performance impact (execution time changes)

## Quick Usage

### Method 1: Automatic (Recommended)

Run tests on both branches and generate report automatically:

```bash
make test-comparison-report
```

This will:
1. Run tests on the `main` branch
2. Run tests on the `split-binaries-into-crates` branch
3. Generate `reports/test_comparison_report.json`

### Method 2: From Saved Outputs

If you already have test outputs saved:

```bash
# Save test outputs
git checkout main
cargo test --workspace > before_tests.txt 2>&1

git checkout split-binaries-into-crates
cargo test --workspace > after_tests.txt 2>&1

# Generate report
make test-comparison-from-files BEFORE=before_tests.txt AFTER=after_tests.txt
```

### Method 3: Direct Python Script

For more control:

```bash
# With automatic test execution
python scripts/test_comparison_report.py --run-tests --output my_report.json

# From saved files
python scripts/test_comparison_report.py \
    --before before.txt \
    --after after.txt \
    --output my_report.json

# Custom git references
python scripts/test_comparison_report.py \
    --run-tests \
    --before-ref develop \
    --after-ref feature-branch \
    --output comparison.json
```

## View the Report

```bash
# Pretty print
cat reports/test_comparison_report.json | jq .

# View summary only
cat reports/test_comparison_report.json | jq .summary

# List tests by crate
cat reports/test_comparison_report.json | jq '.after.tests_by_crate | keys[]'

# Count tests per crate
cat reports/test_comparison_report.json | jq '.after.tests_by_crate | to_entries[] | "\(.key): \(.value.test_count)"' -r
```

## Report Structure

The JSON report contains:

### Summary
```json
{
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
  }
}
```

### Tests by Crate (After Splitting)
```json
{
  "after": {
    "tests_by_crate": {
      "bash-eval": {
        "test_count": 15,
        "tests": [...]
      },
      "testcase-models": {
        "test_count": 25,
        "tests": [...]
      }
    }
  }
}
```

### Changes
```json
{
  "changes": {
    "new_tests": ["test_new_1", "test_new_2"],
    "removed_tests": ["test_old"],
    "common_tests": ["test_1", "test_2", ...]
  }
}
```

## Understanding the Results

### Performance
- **Negative duration_percent_change**: Tests run faster (improvement) ✓
- **Positive duration_percent_change**: Tests run slower (needs attention)

### Test Organization
- `tests_by_crate` shows how tests are distributed across crates
- Helps verify proper test migration during crate splitting

### Test Changes
- **new_tests**: Tests added in the after state
- **removed_tests**: Tests removed in the after state
- **common_tests**: Tests present in both states

## Examples

### Example: View Test Count per Crate
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
...
```

### Example: Find New Tests
```bash
jq '.changes.new_tests[]' reports/test_comparison_report.json -r
```

### Example: Check Performance Impact
```bash
jq '.summary | {
  duration_before: .total_duration_before_seconds,
  duration_after: .total_duration_after_seconds,
  change: .duration_percent_change
}' reports/test_comparison_report.json
```

## Tips

1. **Before running**: Ensure both branches are up to date
   ```bash
   git fetch origin
   ```

2. **Save current work**: The tool will checkout different branches
   ```bash
   git stash save "temp work"
   ```

3. **Large test suites**: Consider using Method 2 (saved outputs) to avoid timeouts

4. **CI Integration**: Add to your CI pipeline to track test organization changes
   ```yaml
   test-comparison:
     script:
       - make test-comparison-report
     artifacts:
       paths:
         - reports/test_comparison_report.json
   ```

## Troubleshooting

### "Git reference not found"
```bash
git fetch origin
git branch -a
```

### "Cargo test timeout"
Use Method 2 with saved outputs instead.

### "Cannot parse test output"
Ensure using standard cargo test output:
```bash
cargo test --workspace -- --nocapture
```

## See Also

- Full Documentation: [scripts/README_TEST_COMPARISON.md](scripts/README_TEST_COMPARISON.md)
- Workspace Structure: [AGENTS.md](AGENTS.md#workspace-structure)
- Test Commands: [TEST_COMMANDS.md](TEST_COMMANDS.md)
