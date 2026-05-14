# TCMS-37: Edge Cases and Comprehensive Testing

## Overview

This document describes the edge cases covered by the comprehensive test suite for TCMS-37 (Multiple Campaign Merging) and explains the behavior when unusual situations occur, such as missing execution logs.

## Comprehensive Test Coverage

The test suite includes **30 unit tests** organized into 5 test classes:

### 1. `TestMergeStrategies` (8 tests)
Tests individual merge strategy functions with various pass/fail combinations.

- `test_or_strategy_all_pass` - OR with all campaigns passing
- `test_or_strategy_all_fail` - OR with all campaigns failing
- `test_or_strategy_mixed` - OR with mixed results (one pass, one fail)
- `test_or_strategy_aggregates_steps` - Verify step counts are aggregated
- `test_and_strategy_all_pass` - AND with all passing
- `test_and_strategy_all_fail` - AND with all failing
- `test_and_strategy_mixed` - AND with mixed results
- `test_newest_strategy` / `test_oldest_strategy` - Timestamp-based strategies

### 2. `TestCampaignLoading` (3 tests)
Tests loading verification files from campaign directories.

- `test_load_verification_from_campaign` - Load single verification file
- `test_load_verification_multiple_files` - Load multiple files from campaign
- `test_load_verification_missing_directory` - Handle missing 20_verification directory

### 3. `TestExecutionTimestamps` (3 tests)
Tests timestamp extraction from execution logs.

- `test_get_execution_timestamp_from_logs` - Extract timestamp from JSON logs
- `test_get_execution_timestamp_multiple_logs` - Find earliest across multiple logs
- `test_get_execution_timestamp_missing_logs` - Handle missing execution logs

### 4. `TestIntegrationMergeCampaigns` (6 tests)
End-to-end integration tests with real directory structures.

- `test_merge_two_campaigns_or_strategy` - OR merge of two campaigns
- `test_merge_two_campaigns_and_strategy` - AND merge of two campaigns
- `test_merge_non_overlapping_test_cases` - Campaigns with different test cases
- `test_merge_container_metadata` - Verify metadata in output
- `test_merge_oldest_strategy_uses_earliest_timestamp` - OLDEST strategy integration

### 5. `TestEdgeCasesNoExecution` (4 tests) - **NEW**
Tests scenarios when campaigns lack execution logs.

#### 5.1: Campaigns Without Execution Logs

**Test:** `test_merge_campaign_without_execution_logs_or_strategy`

**Scenario:** Two campaigns with verification results but NO execution logs directory.

**Expected Behavior:**
```
Campaign 1: 20_verification/TC_001_verification.yaml ✓
            10_test_results/execution_logs/         ✗ (missing)

Campaign 2: 20_verification/TC_001_verification.yaml ✓
            10_test_results/execution_logs/         ✗ (missing)
```

**Result:** Script gracefully handles missing logs. For non-timestamp-based strategies (OR, AND), no timestamp needed.

#### 5.2: OLDEST Strategy Without Execution Logs

**Test:** `test_merge_campaign_without_execution_logs_oldest_strategy`

**Scenario:** Multiple campaigns without execution logs, using OLDEST strategy.

**Expected Behavior:**
```
Campaign 1: Created at 10:05 UTC (newer)
Campaign 2: Created at 10:00 UTC (older)
```

**Result:** When execution logs are missing, fallback uses file modification time (mtime) of verification YAML files. Campaign 2's result is used (oldest by mtime).

**Fallback Chain:**
1. Try to extract timestamp from `10_test_results/execution_logs/*.json`
2. If no logs found, try to find mtime of `10_test_results/execution_logs/` directory
3. If logs directory doesn't exist, use mtime of verification YAML file itself
4. All times converted to UTC timezone-aware datetimes for proper comparison

#### 5.3: Partial Execution Logs

**Test:** `test_merge_campaign_partial_execution_logs`

**Scenario:** Some campaigns have execution logs, others don't.

**Expected Behavior:**
```
Campaign 1: WITH execution logs     → timestamp extracted from log
Campaign 2: WITHOUT execution logs  → fallback to verification file mtime

NEWEST strategy: Uses Campaign 1 or 2 based on which is newer
```

**Result:** Mixed timestamps work correctly because:
- Execution log timestamps are ISO 8601 (timezone-aware)
- File mtime fallback converted to UTC timezone-aware datetime
- All datetimes can be compared consistently

#### 5.4: Completely Empty Campaign

**Test:** `test_campaign_completely_empty`

**Scenario:** Campaign directory exists but has no verification files.

**Expected Behavior:**
```
Campaign 1: 20_verification/TC_001_verification.yaml ✓
Campaign 2: 20_verification/                         ✗ (empty)
```

**Result:** Script warns "No verification results found in campaign2" and continues. Output includes only test cases from Campaign 1.

### 6. `TestEdgeCasesMultipleCampaigns` (6 tests) - **NEW**
Complex multi-campaign scenarios.

#### 6.1: Three-Way Merge with OR Strategy

**Test:** `test_merge_three_campaigns_or`

**Scenario:**
```
Campaign 1: TC_001=PASS, TC_002=FAIL
Campaign 2: TC_001=FAIL, TC_003=PASS
Campaign 3: TC_002=PASS, TC_003=FAIL
```

**Expected Results with OR:**
- TC_001: PASS OR FAIL OR (missing) = **PASS** ✓
- TC_002: FAIL OR (missing) OR PASS = **PASS** ✓
- TC_003: (missing) OR PASS OR FAIL = **PASS** ✓

#### 6.2: Three-Way Merge with AND Strategy

**Test:** `test_merge_three_campaigns_and`

**Scenario:**
```
Campaign 1: TC_001=PASS, TC_002=PASS
Campaign 2: TC_001=PASS, TC_002=FAIL
Campaign 3: TC_001=PASS, TC_002=PASS
```

**Expected Results with AND:**
- TC_001: PASS AND PASS AND PASS = **PASS** ✓
- TC_002: PASS AND FAIL AND PASS = **FAIL** ✗

#### 6.3: Single Campaign Merge

**Test:** `test_merge_single_campaign`

**Scenario:** Merge a single campaign (edge case).

**Expected Behavior:** Works correctly - results from single campaign passed through as-is.

#### 6.4: Step Count Aggregation

**Test:** `test_merge_with_step_aggregation_across_campaigns`

**Scenario:**
```
Campaign 1: TC_001 with passed=2, failed=1, total=3
Campaign 2: TC_001 with passed=1, failed=2, total=3
```

**Expected Aggregation:**
- `total_steps = 3 + 3 = 6`
- `passed_steps = 2 + 1 = 3`
- `failed_steps = 1 + 2 = 3`
- `not_executed_steps = 0 + 0 = 0`

### 7. `TestEdgeCasesTimestamps` (2 tests) - **NEW**
Timestamp handling edge cases.

#### 7.1: Different Timezone Formats

**Test:** `test_oldest_with_iso_timestamps_different_timezones`

**Scenario:**
```
Campaign 1: 2026-05-13T15:00:00+05:00 (UTC equivalent: 10:00)
Campaign 2: 2026-05-13T10:00:00+00:00 (UTC equivalent: 10:00)
```

**Expected Behavior:** Both timestamps are equivalent in absolute time. Script correctly compares them after converting to UTC.

#### 7.2: Multiple Entries in Single Log

**Test:** `test_newest_with_multiple_entries_in_log`

**Scenario:** One execution log file contains multiple step entries with different timestamps:
```json
[
  {"step": 1, "timestamp": "2026-05-13T10:00:00+00:00"},
  {"step": 2, "timestamp": "2026-05-13T10:00:05+00:00"},
  {"step": 3, "timestamp": "2026-05-13T10:00:02+00:00"}
]
```

**Expected Behavior:** Script finds earliest (10:00:00) across all entries and uses that as campaign timestamp.

## Critical Bug Fix: Timezone-Aware Datetime Handling

### Problem
When mixing execution log timestamps with file modification times:
- ISO 8601 timestamps from logs: `2026-05-13T10:00:00+00:00` (timezone-aware)
- File mtime: `datetime.fromtimestamp()` (timezone-naive by default)

Python raises `TypeError: can't compare offset-naive and offset-aware datetimes` when sorting.

### Solution
All datetime objects created for comparison are converted to timezone-aware UTC:

```python
# Before (naive datetime):
mtime = datetime.fromtimestamp(file.stat().st_mtime)

# After (UTC timezone-aware):
mtime = datetime.fromtimestamp(file.stat().st_mtime, tz=timezone.utc)
```

This ensures consistent comparison across all merge strategies, regardless of timestamp source.

## What Happens: Missing Execution Logs (Key Behavior)

### Scenario: Campaign without execution logs

```
campaign/
├── 00_test_cases/
├── 05_scripts/
├── 10_test_results/
│   └── execution_logs/    ← EMPTY or MISSING
└── 20_verification/
    └── TC_001_verification.yaml
```

### For OR/AND Strategies:
✓ Works fine - no timestamp needed

### For OLDEST/NEWEST Strategies:
1. **Try to load from execution logs** → Fails (no logs)
2. **Fallback to file mtime** → Uses timestamp of verification YAML file
3. **Compare with other campaigns** → All times converted to UTC for fair comparison
4. **Result:** Script selects campaign based on file modification time instead of execution timestamp

### Example Output:
```bash
Warning: No execution logs in /path/to/campaign
→ Will use file modification time for OLDEST/NEWEST strategies
```

## Test Execution

Run all 30 tests:
```bash
make test-merge-campaigns

# Output:
# Ran 30 tests in 0.049s
# OK
# Warning: No verification results found in /tmp/tmpXXX/campaign2
# ✓ merge_campaigns tests passed
```

The warning about empty campaigns is **expected and correct** - it demonstrates the script gracefully handles missing verification files.

## Summary of Robustness

The merge_campaigns script is designed to be **robust and fault-tolerant**:

| Scenario | Behavior |
|----------|----------|
| Missing execution logs | Fallback to file mtime ✓ |
| Missing 20_verification directory | Warn and skip campaign ✓ |
| Empty verification directory | Warn and use remaining campaigns ✓ |
| Mixed log/mtime timestamps | Convert all to UTC for comparison ✓ |
| Single campaign merge | Works correctly ✓ |
| Non-overlapping test cases | Includes all test cases ✓ |
| Different timezone formats | Correctly compares absolute times ✓ |
| Multiple timestamp entries in log | Uses earliest entry ✓ |

## Validation

All edge cases validated by comprehensive test suite:
- ✓ 30 unit tests covering all scenarios
- ✓ All tests passing
- ✓ Edge cases documented and tested
- ✓ Behavior predictable and well-defined
- ✓ Error handling graceful with informative warnings
