# Test Comparison Report Tool

## Overview

The `test_comparison_report.py` tool generates a comprehensive JSON report comparing test execution before and after the crate splitting changes. It analyzes cargo test output to provide insights into:

1. **Which tests were executed** before and after the change
2. **Test organization** - after splitting, which crate contains each test
3. **Performance metrics** - total execution time before and after

## Installation

No additional dependencies required beyond the project's Python dependencies (Python 3.14, PyYAML).

## Usage

### Option 1: Run Tests Automatically

The tool can automatically checkout git references, run tests, and generate the comparison report:

```bash
# Compare main branch with split-binaries-into-crates branch
python scripts/test_comparison_report.py --run-tests --output report.json

# Use custom git references
python scripts/test_comparison_report.py \
    --run-tests \
    --before-ref main \
    --after-ref split-binaries-into-crates \
    --output report.json

# Enable verbose output
python scripts/test_comparison_report.py --run-tests --verbose --output report.json
```

### Option 2: Use Pre-saved Test Outputs

If you've already captured test outputs to files, you can generate a report from them:

```bash
# Save test outputs first
git checkout main
cargo test --workspace > before_tests.txt 2>&1

git checkout split-binaries-into-crates
cargo test --workspace > after_tests.txt 2>&1

# Generate report from saved outputs
python scripts/test_comparison_report.py \
    --before before_tests.txt \
    --after after_tests.txt \
    --output report.json
```

### Recommended Workflow

```bash
# 1. Save current branch
CURRENT_BRANCH=$(git branch --show-current)

# 2. Run comparison (this will checkout branches)
python scripts/test_comparison_report.py --run-tests --output comparison_report.json

# 3. Return to original branch
git checkout $CURRENT_BRANCH

# 4. View the report
cat comparison_report.json | jq .
```

## Report Format

The generated JSON report contains:

### Metadata
```json
{
  "metadata": {
    "generated_at": "2024-01-15T10:30:00.123456",
    "before_ref": "main",
    "after_ref": "split-binaries-into-crates",
    "tool_version": "1.0.0"
  }
}
```

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

### Before State
```json
{
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
        "name": "test_example",
        "crate": null,
        "status": "ok",
        "duration_seconds": 0.0
      }
    ]
  }
}
```

### After State (with Crate Organization)
```json
{
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
      "bash-eval": {
        "test_count": 15,
        "tests": [...]
      },
      "testcase-models": {
        "test_count": 25,
        "tests": [...]
      },
      "testcase-manager": {
        "test_count": 45,
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
    "new_tests": [
      "test_new_feature_a",
      "test_new_feature_b"
    ],
    "removed_tests": [
      "test_deprecated_function"
    ],
    "common_tests": [
      "test_existing_feature_1",
      "test_existing_feature_2"
    ]
  }
}
```

## Command-Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--run-tests` | Automatically run cargo tests for comparison | - |
| `--before FILE` | Path to file with test output from before | - |
| `--after FILE` | Path to file with test output from after | - |
| `--output FILE` | Output path for JSON report | `test_comparison_report.json` |
| `--before-ref REF` | Git reference for before state | `main` |
| `--after-ref REF` | Git reference for after state | `split-binaries-into-crates` |
| `--verbose` | Enable verbose output | disabled |

## Examples

### Basic Comparison
```bash
python scripts/test_comparison_report.py --run-tests
```

### Compare Different Branches
```bash
python scripts/test_comparison_report.py \
    --run-tests \
    --before-ref develop \
    --after-ref feature/my-changes \
    --output my_comparison.json
```

### Use Saved Outputs
```bash
# Save outputs first
cargo test --workspace > test_output_old.txt 2>&1

# Make changes, then:
cargo test --workspace > test_output_new.txt 2>&1

# Generate report
python scripts/test_comparison_report.py \
    --before test_output_old.txt \
    --after test_output_new.txt \
    --output comparison.json
```

### Analyze Report with jq

```bash
# View summary
jq '.summary' report.json

# List new tests
jq '.changes.new_tests[]' report.json

# Show tests by crate
jq '.after.tests_by_crate | keys[]' report.json

# Count tests per crate
jq '.after.tests_by_crate | to_entries[] | "\(.key): \(.value.test_count)"' report.json -r

# Find failed tests
jq '.after.tests[] | select(.status == "FAILED") | .name' report.json -r
```

## Output Interpretation

### Performance Changes

- **Negative percentage**: Tests run faster after the change (improvement)
- **Positive percentage**: Tests run slower after the change (regression)

Example:
```
Duration before: 45.32s
Duration after:  42.15s
Difference:      -3.17s (-7.0%)
```
This shows a 7% improvement in test execution time.

### Test Organization

The `tests_by_crate` section shows how tests are distributed across crates after splitting:

```
Tests by crate after splitting:
  bash-eval: 15 tests
  testcase-models: 25 tests
  testcase-manager: 45 tests
  ...
```

This helps verify that tests were properly migrated to their corresponding crates.

### Test Changes

- **New tests**: Tests that appear in the after state but not before
- **Removed tests**: Tests that appear in the before state but not after
- **Common tests**: Tests that appear in both states

## Integration with CI/CD

Add to your CI pipeline to track test organization changes:

```yaml
# .gitlab-ci.yml
test-comparison:
  stage: test
  script:
    - python scripts/test_comparison_report.py --run-tests --output $CI_PROJECT_DIR/test_comparison.json
  artifacts:
    reports:
      test_comparison.json
    paths:
      - test_comparison.json
```

## Troubleshooting

### "Git reference not found"
Ensure the git references exist and are accessible:
```bash
git fetch origin
git branch -a
```

### "Cargo test timeout"
For large test suites, the default 10-minute timeout might be insufficient. Consider:
- Running tests separately and using `--before`/`--after` options
- Filtering tests with cargo test filters

### "Cannot parse test output"
Ensure cargo test is run with standard output format:
```bash
cargo test --workspace -- --nocapture
```

### "No crate information in after state"
This is normal if cargo doesn't output crate names. Tests will be marked with `"crate": null`.

## See Also

- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Workspace Documentation](../AGENTS.md#workspace-structure)
- [Test Commands](../TEST_COMMANDS.md)
