# Test Campaign Quick Start Guide

## Overview

Test campaigns provide an organized way to execute, track, and collect evidence for test executions.

## Quick Start (5 Minutes)

### 1. Start a Campaign

```bash
./scripts/campaign-start.sh --name "My_First_Campaign"
```

**Output:** Creates `campaigns/My_First_Campaign/` with complete directory structure.

### 2. Run Tests

```bash
# Run all tests
./scripts/campaign-run.sh --campaign campaigns/My_First_Campaign

# Run only specific tests (regex pattern)
./scripts/campaign-run.sh --campaign campaigns/My_First_Campaign \
    --pattern "EXAMPLE.*\.yml"
```

**Output:** 
- Test execution logs in `campaigns/My_First_Campaign/execution_logs/run_1_*/`
- Verification results in `campaigns/My_First_Campaign/verification_results/`

### 3. Collect Evidence

```bash
./scripts/campaign-collect-evidence.sh \
    --campaign campaigns/My_First_Campaign \
    --checksums
```

**Output:** 
- `My_First_Campaign_evidence.tar.gz` - Compressed archive
- `My_First_Campaign_evidence.tar.gz.sha256` - Checksum file
- `My_First_Campaign_evidence_report.txt` - Evidence report

### 4. Stop Campaign

```bash
./scripts/campaign-stop.sh \
    --campaign campaigns/My_First_Campaign \
    --summary "My first campaign completed successfully"
```

**Output:** 
- Updates campaign status to COMPLETED
- Generates `campaigns/My_First_Campaign/reports/CAMPAIGN_SUMMARY.md`

---

## Common Use Cases

### Use Case 1: Run Specific Test Subset

```bash
# Start campaign
./scripts/campaign-start.sh --name "BDD_Tests"

# Run only BDD examples
./scripts/campaign-run.sh --campaign campaigns/BDD_Tests \
    --testcase-dir testcases/bdd_examples

# Stop campaign
./scripts/campaign-stop.sh --campaign campaigns/BDD_Tests
```

### Use Case 2: Multiple Test Runs

```bash
# Start campaign
./scripts/campaign-start.sh --name "Multi_Run_Campaign"

# First run - smoke tests
./scripts/campaign-run.sh --campaign campaigns/Multi_Run_Campaign \
    --pattern "smoke.*"

# Second run - regression tests
./scripts/campaign-run.sh --campaign campaigns/Multi_Run_Campaign \
    --pattern "regression.*"

# Third run - all remaining
./scripts/campaign-run.sh --campaign campaigns/Multi_Run_Campaign \
    --pattern ".*"

# Stop campaign with evidence collection
./scripts/campaign-stop.sh --campaign campaigns/Multi_Run_Campaign \
    --collect-evidence
```

### Use Case 3: Parallel Test Execution

```bash
# Start campaign
./scripts/campaign-start.sh --name "Parallel_Tests"

# Run 4 tests in parallel, continue on errors
./scripts/campaign-run.sh --campaign campaigns/Parallel_Tests \
    --parallel 4 \
    --continue-on-error

# Stop campaign
./scripts/campaign-stop.sh --campaign campaigns/Parallel_Tests
```

### Use Case 4: CI/CD Integration

```bash
# Start campaign with CI build number
./scripts/campaign-start.sh --name "CI_Build_${BUILD_NUMBER}"

# Run all tests
./scripts/campaign-run.sh \
    --campaign "campaigns/CI_Build_${BUILD_NUMBER}" \
    --continue-on-error

# Collect evidence with checksums
./scripts/campaign-collect-evidence.sh \
    --campaign "campaigns/CI_Build_${BUILD_NUMBER}" \
    --checksums

# Stop campaign
./scripts/campaign-stop.sh \
    --campaign "campaigns/CI_Build_${BUILD_NUMBER}" \
    --summary "CI Build ${BUILD_NUMBER} completed"
```

---

## Pattern Matching Examples

The `--pattern` option accepts regex patterns for filtering test cases:

```bash
# All test cases (default)
--pattern ".*"

# Test cases starting with "EXAMPLE"
--pattern "EXAMPLE.*"

# Test cases ending with "_001.yml"
--pattern ".*_001\.yml"

# Test cases containing "gsma"
--pattern ".*gsma.*"

# Specific test case
--pattern "SELF_VALIDATED_EXAMPLE_001\.yml"

# Multiple patterns (run script multiple times or use regex OR)
--pattern "(EXAMPLE|gsma).*"
```

---

## Command Reference

### campaign-start.sh

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `--name` | Campaign name | Yes | - |
| `--description` | Campaign description | No | Auto-generated |
| `--output-dir` | Output directory | No | campaigns/<name> |
| `--testcase-dir` | Test case directory | No | testcases |
| `--verbose` | Verbose output | No | Off |

### campaign-run.sh

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `--campaign` | Campaign directory | Yes | - |
| `--pattern` | Test filename regex | No | .* (all) |
| `--testcase-dir` | Override testcase dir | No | From campaign |
| `--parallel` | Parallel jobs | No | 1 |
| `--continue-on-error` | Continue on errors | No | Off |
| `--skip-verification` | Skip verification | No | Off |
| `--verbose` | Verbose output | No | Off |

### campaign-collect-evidence.sh

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `--campaign` | Campaign directory | Yes | - |
| `--output` | Output file | No | <name>_evidence.tar.gz |
| `--format` | Archive format | No | tar.gz |
| `--checksums` | Generate checksums | No | Off |
| `--verbose` | Verbose output | No | Off |

### campaign-stop.sh

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `--campaign` | Campaign directory | Yes | - |
| `--summary` | Summary text | No | Auto-generated |
| `--generate-reports` | Generate reports | No | Off |
| `--collect-evidence` | Auto-collect evidence | No | Off |
| `--verbose` | Verbose output | No | Off |

---

## Directory Structure

```
campaigns/<name>/
├── testcases/                     # Test case YAML files
│   ├── run_1_<timestamp>/
│   ├── run_2_<timestamp>/
│   └── ...
├── execution_logs/                # JSON execution logs
│   ├── run_1_<timestamp>/
│   ├── run_2_<timestamp>/
│   └── ...
├── verification_results/          # Verification results
│   ├── run_1_<timestamp>_verification.json
│   ├── run_1_<timestamp>_verification.yaml
│   └── ...
├── evidence/                      # Additional evidence
├── reports/                       # Generated reports
│   └── CAMPAIGN_SUMMARY.md
├── metadata/                      # Campaign metadata
│   ├── campaign.yaml
│   ├── state.txt
│   ├── run_counter.txt
│   ├── run_1_<timestamp>.yaml
│   └── ...
└── README.md                      # Campaign README
```

---

## Tips and Best Practices

### Naming Conventions

Use descriptive names with dates or versions:
- ✅ `Sprint_23_Regression_2024-01-15`
- ✅ `Release_v2.0_Validation`
- ✅ `Daily_Smoke_Mon`
- ❌ `test1`
- ❌ `campaign`

### Test Organization

- Run critical/smoke tests first
- Use `--pattern` to group related tests
- Use `--continue-on-error` for complete coverage
- Use `--parallel` for independent tests only

### Evidence Collection

- Collect evidence periodically for long campaigns
- Always use `--checksums` for audit/compliance
- Archive evidence immediately after stopping campaign

### Error Handling

If a test run fails:
1. Check run metadata: `campaigns/<name>/metadata/run_*.yaml`
2. Review execution logs: `campaigns/<name>/execution_logs/run_*/`
3. Check verification results: `campaigns/<name>/verification_results/`
4. Fix issues and run again (new run ID will be created)

---

## Troubleshooting

### "Campaign directory already exists"

```bash
# Option 1: Use different name
./scripts/campaign-start.sh --name "My_Campaign_v2"

# Option 2: Remove existing campaign
rm -rf campaigns/My_Campaign
./scripts/campaign-start.sh --name "My_Campaign"
```

### "Campaign is not active"

Campaign has been stopped. Start a new campaign or modify the state file (not recommended):

```bash
# Start new campaign
./scripts/campaign-start.sh --name "My_Campaign_v2"
```

### "No test cases found matching pattern"

Verify pattern and test files exist:

```bash
# List available test cases
find testcases -name "*.yml" -o -name "*.yaml"

# Test pattern locally
echo "test_case.yml" | grep -E "YOUR_PATTERN"
```

### Permission errors

Make scripts executable:

```bash
chmod +x scripts/campaign-*.sh
```

---

## Make Targets

The campaign management system is integrated into the project's Makefile:

```bash
# Run campaign tests (included in 'make test')
make test-campaigns

# Run comprehensive campaign tests with multiple patterns
make test-campaigns-full

# Run interactive campaign demonstration
make campaign-demo
```

These targets are automatically run as part of `make test` to ensure the campaign management system works correctly.

---

## Next Steps

- Read full documentation: [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md)
- Explore test cases: `testcases/`
- Review example test: `testcases/self_validated_example.yml`
- Check CI/CD integration examples in the full README
- Try the demo: `make campaign-demo`

---

## Support

For detailed documentation, see [CAMPAIGN_MANAGEMENT_README.md](CAMPAIGN_MANAGEMENT_README.md)
