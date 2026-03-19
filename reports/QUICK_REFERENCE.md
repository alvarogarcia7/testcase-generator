# Quick Reference - Test Case Validation Failures

## Summary (TL;DR)

- **Total Files**: 170
- **Passed**: 168 (98.8%)
- **Failed**: 2 (1.2%)
- **Test Acceptance Suite**: ✅ 100% pass (98/98 files)

---

## Failed Files List

### 1. tests/sample/SGP.22_4.4.2.yaml
- **Error**: Missing `general_initial_conditions`
- **Fix**: Add the field (see below)

### 2. tests/sample/data.yml
- **Error**: Not a test case file
- **Fix**: Exclude from validation

---

## Quick Fixes

### Fix #1: Add Missing Field

**File**: tests/sample/SGP.22_4.4.2.yaml

**Add this after line 5**:
```yaml
general_initial_conditions:
  - "eUICC in initial state"
  - "Test environment configured"
```

**Test**:
```bash
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml
```

---

### Fix #2: Exclude Non-Test File

**File**: scripts/generate_validation_report.py  
**Line**: ~81

**Add to exclude_patterns**:
```python
"data.yml",  # Non-test-case metadata files
```

**Full context**:
```python
exclude_patterns = [
    "te.y",
    "wrong",
    "sample_test_runs.yaml",
    "/expected_test_results/test_case_result/",
    "/expected_test_results/container/",
    "/expected_output_reports/",
    "/testcase_results_container/",
    "container_config.yml",
    "container_data.yml",
    "_report.yaml",
    "data.yml",  # ADD THIS LINE
]
```

---

## Re-run Validation

```bash
make validate-testcases-report
cat reports/validation_report.txt
```

**Expected result**: 169/169 files passing (100%)

---

## Files by Directory

| Directory | Passed | Failed | Total |
|-----------|--------|--------|-------|
| test-acceptance/test_cases/ | 98 | 0 | 98 ✅ |
| testcases/ | 68 | 0 | 68 ✅ |
| tests/sample/ | 2 | 2 | 4 ⚠️ |
| data/ | 2 | 0 | 2 ✅ |

---

## Error Breakdown

| Error Type | Count |
|------------|-------|
| Missing `general_initial_conditions` | 2 |
| Missing `requirement` | 1 |
| Missing `item` | 1 |
| Missing `tc` | 1 |
| Missing `id` | 1 |
| Missing `initial_conditions` | 1 |
| Missing `test_sequences` | 1 |

**Note**: 7 of 8 errors are from data.yml (non-test file)

---

## Required Schema Fields

All test cases MUST have:

1. `requirement` - Requirement ID
2. `item` - Item number
3. `tc` - Test case number
4. `id` - Unique ID
5. `description` - Description
6. `general_initial_conditions` - General conditions (array)
7. `initial_conditions` - Component conditions (object)
8. `test_sequences` - Test sequences (array)

---

## Report Files

1. **reports/validation_report.txt** - Full detailed report
2. **reports/VALIDATION_ANALYSIS.md** - Complete analysis
3. **reports/validation_failures_summary.md** - Failures grouped by type
4. **reports/validation_fix_guide.md** - Step-by-step fix guide
5. **reports/QUICK_REFERENCE.md** - This file

---

## Commands

### Validate Single File
```bash
./target/debug/validate-yaml --schema schemas/test-case.schema.json <file>
```

### Generate Report
```bash
make validate-testcases-report
```

### Build Validator
```bash
cargo build --bin validate-yaml
```

---

## Success Metrics

- **Before Fixes**: 168/170 passing (98.8%)
- **After Fix #1**: 169/170 passing (99.4%)
- **After Fix #2**: 169/169 passing (100%) ✅

---

## Test Acceptance Suite Status

✅ **ALL 98 FILES PASSING** ✅

Categories validated successfully:
- bash_commands (15 files)
- complex (9 files)
- dependencies (8 files)
- failure (12 files)
- hooks (14 files)
- manual (9 files)
- prerequisites (7 files)
- success (13 files)
- variables (11 files)

**No action needed for test-acceptance/test_cases/ directory**
