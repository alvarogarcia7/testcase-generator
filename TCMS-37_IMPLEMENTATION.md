# TCMS-37: Support Merging Multiple Test Campaigns

## Overview

This document describes the implementation of TCMS-37, which adds support for executing and verifying multiple test campaigns with a Python script to merge results using configurable merge strategies.

## Objective

Enable TCMS to:
1. Execute multiple independent test campaigns (each campaign is a folder)
2. Verify results from all campaigns
3. Merge verification results using configurable strategies

## Implementation

### New Files

#### 1. `scripts/merge_campaigns.py` (460 lines, executable)

Main script for merging verification results from multiple test campaigns.

**Features:**
- Reads verification YAML files from `20_verification/` directories in campaign folders
- Extracts execution timestamps from `10_test_results/execution_logs/*.json` (for timestamp-based strategies)
- Groups test cases by ID across campaigns
- Applies merge strategy to compute final pass/fail status
- Outputs merged results as YAML container conforming to `test-results-container.schema.v1.json`
- Validates output against JSON schema
- Gracefully handles missing timestamps (fallback to file modification time)

**Merge Strategies:**
- `or`: failure OR success = success (any campaign passing result → overall pass)
- `and`: failure AND success = failure (all campaigns must pass)
- `oldest`: use result from campaign with earliest execution timestamp
- `newest`: use result from campaign with latest execution timestamp

**Command-line Interface:**
```bash
merge_campaigns.py --campaigns DIR1 DIR2 ... \
                  --merge-strategy [or|and|oldest|newest] \
                  [--output FILE] \
                  [-v|--verbose]
```

**Output Format:**
```yaml
type: test_results_container
schema: tcms/test-results-container.schema.v1.json
title: Merged Test Results ({strategy} strategy)
project: Test Case Manager - Merged Campaign Results
test_date: ISO8601 timestamp
test_results: [...]  # merged verification results
metadata:
  execution_duration: 0.0
  total_test_cases: N
  passed_test_cases: M
  failed_test_cases: K
```

#### 2. `scripts/test_merge_campaigns.py` (450 lines)

Comprehensive unit test suite for `merge_campaigns.py`.

**Test Coverage:**
- `TestMergeStrategies` (8 tests): Each merge strategy with various pass/fail combinations
- `TestCampaignLoading` (3 tests): Loading verification files from campaign directories
- `TestExecutionTimestamps` (3 tests): Extracting timestamps from execution logs
- `TestIntegrationMergeCampaigns` (6 tests): End-to-end integration tests

**Total: 20 tests, all passing ✓**

### Modified Files

#### `mk/python.mk`

Added two new Makefile targets:

1. **`make test-merge-campaigns`** - Run unit tests for merge_campaigns script
   ```bash
   make test-merge-campaigns
   ```

2. **`make merge-campaigns`** - Merge campaign directories
   ```bash
   make merge-campaigns CAMPAIGNS="dir1 dir2 ..." STRATEGY=[or|and|oldest|newest] [OUTPUT=file.yaml]
   ```

## Usage Examples

### Example 1: Merge two campaigns with OR strategy
```bash
uv run python3.14 scripts/merge_campaigns.py \
  --campaigns campaign1 campaign2 \
  --merge-strategy or \
  --output merged.yaml
```

### Example 2: Merge three campaigns with AND strategy
```bash
make merge-campaigns \
  CAMPAIGNS='dir1 dir2 dir3' \
  STRATEGY=and \
  OUTPUT=result.yaml
```

### Example 3: Use newest (most recent) execution results
```bash
uv run python3.14 scripts/merge_campaigns.py \
  --campaigns c1 c2 c3 \
  --merge-strategy newest \
  --output merged_latest.yaml \
  --verbose
```

## Merge Strategy Examples

### Setup
Two campaigns:
- **Campaign 1**: TC_001=PASS, TC_002=FAIL
- **Campaign 2**: TC_001=FAIL, TC_002=PASS

### OR Strategy (`--merge-strategy or`)
- TC_001: PASS OR FAIL = **PASS** ✓ (any campaign passes)
- TC_002: FAIL OR PASS = **PASS** ✓ (any campaign passes)

### AND Strategy (`--merge-strategy and`)
- TC_001: PASS AND FAIL = **FAIL** ✗ (all must pass)
- TC_002: FAIL AND PASS = **FAIL** ✗ (all must pass)

### OLDEST Strategy (`--merge-strategy oldest`)
- Uses result from campaign with earliest execution timestamp
- Timestamp extracted from `10_test_results/execution_logs/*.json`
- Example: If Campaign 1 executed at 10:00 and Campaign 2 at 10:05
  - TC_001: **PASS** (from Campaign 1, the oldest)
  - TC_002: **FAIL** (from Campaign 1, the oldest)

### NEWEST Strategy (`--merge-strategy newest`)
- Uses result from campaign with latest execution timestamp
- Example: If Campaign 1 executed at 10:00 and Campaign 2 at 10:05
  - TC_001: **FAIL** (from Campaign 2, the newest)
  - TC_002: **PASS** (from Campaign 2, the newest)

## Technical Details

### Campaign Directory Structure

Each campaign directory must have:
```
campaign/
├── 00_test_cases/           # Test case YAML definitions (optional for merge)
├── 05_scripts/              # Generated bash scripts (optional for merge)
├── 10_test_results/
│   └── execution_logs/      # JSON execution logs (needed for oldest/newest)
└── 20_verification/         # Verification YAML files (REQUIRED)
    ├── TC_001_verification.yaml
    ├── TC_002_verification.yaml
    └── ...
```

### Verification Result Format

Input files in `20_verification/` must have:
```yaml
type: test_verification
schema: tcms/test-verification.schema.v1.json
test_case_id: TC_XXXXX
description: Test description
overall_pass: true/false
total_steps: N
passed_steps: M
failed_steps: K
not_executed_steps: L
```

### Timestamp Extraction

For `oldest`/`newest` strategies:
1. Reads JSON files from `10_test_results/execution_logs/`
2. Each JSON file contains an array of execution entries
3. Each entry has a `timestamp` field in ISO 8601 format
4. Finds minimum (oldest) or maximum (newest) timestamp across all entries
5. Fallback: uses file modification time (mtime) if no execution logs found

### Step Aggregation

When merging test cases across campaigns:
- All merge strategies aggregate step counts
- `total_steps = sum of steps from all campaigns with that test`
- `passed_steps = sum of passed steps`
- `failed_steps = sum of failed steps`
- `not_executed_steps = sum of not-executed steps`

### Schema Validation

- Output validates against `schemas/tcms/test-results-container.schema.v1.json`
- Required metadata fields: `execution_duration`, `total_test_cases`, `passed_test_cases`, `failed_test_cases`
- Validation warnings (not errors) if input doesn't match schema

## Testing

### Unit Tests

Run all merge_campaigns tests:
```bash
make test-merge-campaigns
```

Or directly:
```bash
uv run python3.14 scripts/test_merge_campaigns.py -v
```

### Test Results

```
Ran 20 tests in 0.023s
OK
✓ merge_campaigns tests passed
```

### Integration Testing

Successfully tested with actual test-acceptance data:
```bash
uv run python3.14 scripts/merge_campaigns.py \
  --campaigns test-acceptance test-acceptance \
  --merge-strategy or \
  --output /tmp/merged_test.yaml \
  --verbose
```

Result:
```
✓ Output validation passed against test-results-container.schema.v1.json
✓ Merged results written to: /tmp/merged_test.yaml
```

## Validation

- ✓ All 20 unit tests passing
- ✓ All existing project tests still passing (`make test`)
- ✓ Output validates against JSON schema
- ✓ Integration tested with real test data
- ✓ Compatible with existing TCMS infrastructure
- ✓ Follows project patterns and conventions
- ✓ Code committed to git

## Definition of Done

- ✓ All tests are passing. Run `make test`
- ✓ TCMS-37 task requirements fully implemented
- ✓ Code committed with proper message

## Git Commit

```
Commit: 22d64f3
Message: [TCMS-37] Support merging multiple test campaigns with configurable strategies

- Add scripts/merge_campaigns.py: Python script for merging campaigns
- Add scripts/test_merge_campaigns.py: 20 comprehensive unit tests
- Update mk/python.mk: Add make targets
```

## Future Enhancements (Out of scope for TCMS-37)

Potential future improvements:
- Support for weighted merging (e.g., campaign weights)
- Custom merge strategy plugins
- Parallel campaign execution
- Campaign-specific result filtering
- Merge report generation with diffs
- Web UI for merge strategy selection
- Historical trend analysis across campaigns

## References

- **Input Schema**: `schemas/tcms/test-verification.schema.v1.json`
- **Output Schema**: `schemas/tcms/test-results-container.schema.v1.json`
- **Similar Script**: `scripts/convert_verification_to_result_yaml.py`
- **Test Data**: `test-acceptance/20_verification/`
- **Project Architecture**: See `AGENTS.md`
