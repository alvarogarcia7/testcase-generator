# Test Case Validation - Quick Reference

## Generate Report

```bash
make validate-testcases-report
```

## Current Status (as of 2026-03-19)

- **Total Files:** 95
- **Passed:** 54 (56.8%)
- **Failed:** 41 (43.2%)
- **Actual Issues:** 11 files need fixing

## Error Categories

### 1. Missing Required Fields (174 errors - 97.8%)
Most common: `initial_conditions`, `general_initial_conditions`, `test_sequences`, `id`

**Fix:** Add missing fields or exclude from validation if not a test case

### 2. Malformed YAML Syntax (3 errors - 1.7%)
**Files:**
- `testcases/conditional_verification_example.yml` (Line 290)
- `testcases/examples/doc_gen_file_operations_001.yml`
- `testcases/examples/doc_gen_integration_001.yml`

**Fix:** Quote strings with special characters (`$`, `:`, `{`, `}`)

### 3. Schema Constraint Violations (1 error - 0.6%)
**File:** `data/steps-in-json.yml`

**Fix:** Replace `result: null` with `result: 0`

## Priority Fixes

### 🔴 High Priority (3 files)
Fix YAML syntax errors - these prevent parsing:
```bash
# Fix quoting in these files:
testcases/conditional_verification_example.yml
testcases/examples/doc_gen_file_operations_001.yml
testcases/examples/doc_gen_integration_001.yml
```

### 🟡 Medium Priority (8 files)
Complete partial test cases or relocate:
```bash
# Missing fields:
testcases/1.yaml
testcases/SGP.22_4.4.2.yaml
testcases/examples/doc_gen_data_validation_001.yml
testcases/examples/doc_gen_network_001.yml
testcases/examples/doc_gen_performance_001.yml
tests/sample/data.yml
tests/sample/SGP.22_4.4.2.yaml

# Schema violation:
data/steps-in-json.yml
```

### 🟢 Low Priority (30 files)
Expected failures - these are not test case files:
- Test result/output files (23 files in `expected_output_reports/`, etc.)
- Container configs (3 files)
- Generated samples (8 files)
- Incorrect scenarios (1 file)

**Fix:** Update validation scripts to exclude these patterns

## Quick Validation Commands

```bash
# Validate a single file
./target/debug/validate-yaml --schema schemas/test-case.schema.json FILE.yml

# Generate fresh report
make validate-testcases-report

# View error analysis
cat reports/validation_error_analysis.md

# Search for specific file in report
grep -A 20 "filename.yml" reports/validation_report.txt
```

## Common YAML Syntax Fixes

### Problem: Special Characters in Strings
```yaml
# ❌ WRONG - unquoted shell variable
command: echo "Value: $VAR"

# ✅ CORRECT - single quotes
command: 'echo "Value: $VAR"'

# ✅ CORRECT - escaped double quotes
command: "echo \"Value: \\$VAR\""
```

### Problem: Null Values Not Allowed
```yaml
# ❌ WRONG
expected:
  result: null

# ✅ CORRECT
expected:
  result: 0
```

### Problem: Missing Required Fields
```yaml
# ❌ WRONG - missing required fields
id: "TC001"
description: "Test case"

# ✅ CORRECT - all required fields
id: "TC001"
description: "Test case"
requirement: "REQ-001"
item: 1
tc: 1
general_initial_conditions: {}
initial_conditions: []
test_sequences: []
```

## File Locations

- **Main Report:** `reports/validation_report.txt`
- **Error Analysis:** `reports/validation_error_analysis.md`
- **Documentation:** `reports/README.md`
- **This Guide:** `reports/QUICK_REFERENCE.md`

## Next Steps

1. ✅ Review `validation_error_analysis.md` for categorized errors
2. ✅ Fix 3 YAML syntax errors (high priority)
3. ✅ Fix 1 schema violation (quick win)
4. ✅ Complete or relocate 7 partial test cases
5. ✅ Update validation scripts to exclude non-test-case files
6. ✅ Re-run validation to verify fixes

## Expected Outcome After Fixes

- **Before:** 54/95 passed (56.8%)
- **After:** ~84/95 passed (88.4%)
  - 54 currently passing
  - 30 excluded (non-test-case files)
  - 11 fixed = 65 test cases
  - 65/~73 actual test case files ≈ 89% success rate
