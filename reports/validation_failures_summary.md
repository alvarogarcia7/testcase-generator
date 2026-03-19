# Test Case Validation Failures Summary

Generated: 2026-03-19 20:57:13  
Total Files Validated: 170  
Passed: 168  
Failed: 2  

## Overview

This document provides a structured breakdown of all test case YAML files that failed schema validation, organized by directory and error type to guide systematic fixes.

---

## Failed Test Cases by Directory

### tests/sample/ (2 failures)

#### tests/sample/SGP.22_4.4.2.yaml
- **Error Type**: Missing Required Fields
- **Error Count**: 1 error
- **Missing Fields**:
  - `general_initial_conditions` (required property)

**Current Structure**:
```yaml
description: Sample test case for demonstration
id: SGP.22_4.4.2
item: 4
requirement: SGP.22
tc: 42
initial_conditions:
  LPA: ["test"]
test_sequences: [...]
```

**Required Fix**: Add `general_initial_conditions` field at root level

---

#### tests/sample/data.yml
- **Error Type**: Missing Required Fields
- **Error Count**: 7 errors
- **Missing Fields**:
  1. `requirement` (required property)
  2. `item` (required property)
  3. `tc` (required property)
  4. `id` (required property)
  5. `general_initial_conditions` (required property)
  6. `initial_conditions` (required property)
  7. `test_sequences` (required property)

**Current Structure**:
```yaml
date: "2020-03-16"
description: "Global and regional mobile industry indicators..."
product: "GSMA Mobile Economy 2020"
```

**Required Fix**: This file is NOT a test case file - it appears to be data/metadata. Should be excluded from test case validation or moved to a different location.

---

## Failures Grouped by Error Type

### 1. Missing Required Fields (8 total errors across 2 files)

#### `general_initial_conditions` missing (2 occurrences)
- tests/sample/SGP.22_4.4.2.yaml
- tests/sample/data.yml

#### `requirement` missing (1 occurrence)
- tests/sample/data.yml

#### `item` missing (1 occurrence)
- tests/sample/data.yml

#### `tc` missing (1 occurrence)
- tests/sample/data.yml

#### `id` missing (1 occurrence)
- tests/sample/data.yml

#### `initial_conditions` missing (1 occurrence)
- tests/sample/data.yml

#### `test_sequences` missing (1 occurrence)
- tests/sample/data.yml

---

## Systematic Fix Plan

### Priority 1: Fix Legitimate Test Cases

**File**: `tests/sample/SGP.22_4.4.2.yaml`
- **Action**: Add missing `general_initial_conditions` field
- **Complexity**: Low (1 field to add)
- **Impact**: Fixes 1 validation error

### Priority 2: Handle Non-Test-Case Files

**File**: `tests/sample/data.yml`
- **Action Option 1**: Exclude from test case validation (add to exclude patterns)
- **Action Option 2**: Move to non-test-case directory
- **Reason**: This file is clearly not a test case - it's metadata about GSMA Mobile Economy 2020 report
- **Impact**: Resolves 7 validation errors

---

## Validation Script Recommendations

### Current Exclusion Patterns (in scripts/generate_validation_report.py)
```python
exclude_patterns = [
    "te.y",  # Malformed test files
    "wrong",  # Intentionally wrong test files
    "sample_test_runs.yaml",  # Test run metadata
    "/expected_test_results/test_case_result/",
    "/expected_test_results/container/",
    "/expected_output_reports/",
    "/testcase_results_container/",
    "container_config.yml",
    "container_data.yml",
    "_report.yaml",
]
```

### Recommended Addition
Add `data.yml` to exclusion patterns since it's not a test case file:
```python
exclude_patterns = [
    # ... existing patterns ...
    "data.yml",  # Non-test-case data files
]
```

---

## Summary Statistics

### By Directory
| Directory | Total Files | Failed | Pass Rate |
|-----------|-------------|--------|-----------|
| test-acceptance/test_cases/ | 98 | 0 | 100% |
| testcases/ | 68 | 0 | 100% |
| tests/sample/ | 4 | 2 | 50% |
| data/ | 2 | 0 | 100% |

### By Error Category
| Error Category | Count | % of Total |
|----------------|-------|------------|
| Missing Required Fields | 8 | 100% |

### Most Common Issues
1. `general_initial_conditions` missing - 2 files
2. Multiple required fields missing - 1 file (data.yml - not a test case)

---

## Next Steps

1. **Fix tests/sample/SGP.22_4.4.2.yaml**: Add `general_initial_conditions` field
2. **Exclude tests/sample/data.yml**: Add to validation exclusion patterns
3. **Re-run validation**: Verify all test cases pass after fixes
4. **Update documentation**: Document the correct structure for `general_initial_conditions`

---

## Validation Commands

### Validate Individual Files
```bash
# Validate SGP.22_4.4.2.yaml
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml

# Validate data.yml (currently fails - not a test case)
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/data.yml
```

### Re-run Full Validation Report
```bash
make validate-testcases-report
cat reports/validation_report.txt
```

---

## Success Metrics

- **Current**: 168/170 files passing (98.8%)
- **After SGP.22_4.4.2.yaml fix**: 169/170 files passing (99.4%)
- **After data.yml exclusion**: 169/169 files passing (100%)

All test-acceptance/test_cases/ files are already passing validation (98/98 = 100%).
