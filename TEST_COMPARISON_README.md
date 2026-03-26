# Test Comparison Tool

## Quick Summary

A Python tool that generates JSON reports comparing test execution before and after crate splitting.

**What it does:**
1. Lists which tests were executed before and after the change
2. Shows in which crate each test is located after splitting
3. Compares total execution time before and after

## Quick Start

```bash
# Generate report
make test-comparison-report

# View report
cat reports/test_comparison_report.json | jq .
```

## Report Output

The tool generates `reports/test_comparison_report.json` containing:

```json
{
  "summary": {
    "tests_before": 150,
    "tests_after": 152,
    "new_tests": 5,
    "removed_tests": 3,
    "total_duration_before_seconds": 45.32,
    "total_duration_after_seconds": 42.15,
    "duration_percent_change": -7.0
  },
  "after": {
    "tests_by_crate": {
      "bash-eval": {"test_count": 15, "tests": [...]},
      "testcase-models": {"test_count": 25, "tests": [...]}
    }
  }
}
```

## Usage Options

### 1. Automatic (Recommended)
```bash
make test-comparison-report
```

### 2. From Saved Outputs
```bash
make test-comparison-from-files BEFORE=before.txt AFTER=after.txt
```

### 3. Direct Script
```bash
python scripts/test_comparison_report.py --run-tests --output report.json
```

## View Results

```bash
# Summary
jq '.summary' reports/test_comparison_report.json

# Tests by crate
jq '.after.tests_by_crate | to_entries[] | "\(.key): \(.value.test_count)"' \
    reports/test_comparison_report.json -r
```

## Documentation

- **Quick Start**: [TEST_COMPARISON_QUICK_START.md](TEST_COMPARISON_QUICK_START.md)
- **Full Documentation**: [scripts/README_TEST_COMPARISON.md](scripts/README_TEST_COMPARISON.md)
- **Tool Guide**: [docs/TEST_COMPARISON_TOOL.md](docs/TEST_COMPARISON_TOOL.md)
- **Implementation Details**: [TEST_COMPARISON_IMPLEMENTATION.md](TEST_COMPARISON_IMPLEMENTATION.md)

## Example

```bash
# Run the example
./examples/test_comparison_example.sh

# View sample report
cat examples/test_comparison_report_sample.json | jq .
```

## Files

| File | Purpose |
|------|---------|
| `scripts/test_comparison_report.py` | Main tool |
| `scripts/README_TEST_COMPARISON.md` | Complete documentation |
| `TEST_COMPARISON_QUICK_START.md` | Quick reference |
| `docs/TEST_COMPARISON_TOOL.md` | Comprehensive guide |
| `examples/test_comparison_example.sh` | Interactive example |
| `examples/test_comparison_report_sample.json` | Sample output |

## Make Targets

```bash
make test-comparison-report              # Generate report automatically
make test-comparison-from-files          # From saved test outputs
```

See [AGENTS.md](AGENTS.md) for full command reference.
