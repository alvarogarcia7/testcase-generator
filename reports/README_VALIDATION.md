# Test Case Validation Reports - Index

This directory contains comprehensive validation reports for all test case YAML files in the repository.

## Report Files Overview

### 📊 Primary Reports

#### 1. [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
**Start here for a fast overview**
- Summary statistics (170 files, 168 passing, 2 failing)
- Quick fix instructions
- Command reference
- Test acceptance suite status (100% passing)

#### 2. [validation_report.txt](validation_report.txt)
**Full detailed validation output**
- Complete validation results for all 170 files
- Detailed error messages for each failure
- Error categorization
- Common error patterns
- Validation commands for failed files

#### 3. [VALIDATION_ANALYSIS.md](VALIDATION_ANALYSIS.md)
**Comprehensive analysis document**
- Executive summary with pass rates by directory
- Detailed breakdown of all 98 test-acceptance files (100% passing)
- Error pattern analysis
- Schema compliance analysis
- Fix recommendations with priority levels
- Test case categories successfully validated

### 🔧 Fix Guides

#### 4. [validation_fix_guide.md](validation_fix_guide.md)
**Step-by-step fix instructions**
- Detailed fixes for each failing file
- Code examples with before/after
- Schema requirements reference
- Verification steps
- Expected outcomes

#### 5. [validation_failures_summary.md](validation_failures_summary.md)
**Failures grouped by directory and error type**
- Failed files organized by directory
- Errors grouped by type
- Systematic fix plan
- Validation script recommendations
- Summary statistics

### 📈 Legacy Reports

#### 6. [validation_error_analysis.md](validation_error_analysis.md)
**Earlier analysis document** (archived for reference)

---

## Quick Navigation

### By Use Case

**"I want a quick overview"**
→ [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

**"I need to fix the failing tests"**
→ [validation_fix_guide.md](validation_fix_guide.md)

**"I want detailed statistics"**
→ [VALIDATION_ANALYSIS.md](VALIDATION_ANALYSIS.md)

**"I need the full validation output"**
→ [validation_report.txt](validation_report.txt)

**"I want errors grouped by type"**
→ [validation_failures_summary.md](validation_failures_summary.md)

---

## Key Findings Summary

### Overall Statistics
- **Total Files**: 170
- **Passed**: 168 (98.8%)
- **Failed**: 2 (1.2%)

### By Directory
| Directory | Files | Passed | Failed | Pass Rate |
|-----------|-------|--------|--------|-----------|
| test-acceptance/test_cases/ | 98 | 98 | 0 | 100% ✅ |
| testcases/ | 68 | 68 | 0 | 100% ✅ |
| tests/sample/ | 4 | 2 | 2 | 50% ⚠️ |
| data/ | 2 | 2 | 0 | 100% ✅ |

### Failed Files
1. **tests/sample/SGP.22_4.4.2.yaml** - Missing `general_initial_conditions` field
2. **tests/sample/data.yml** - Not a test case file (should be excluded)

### Test Acceptance Suite ✅
**All 98 files in test-acceptance/test_cases/ pass validation (100%)**

Categories validated:
- bash_commands (15 files)
- complex (9 files)
- dependencies (8 files)
- failure (12 files)
- hooks (14 files)
- manual (9 files)
- prerequisites (7 files)
- success (13 files)
- variables (11 files)

---

## Regenerating Reports

### Generate Full Validation Report
```bash
make validate-testcases-report
```

This will:
1. Discover all test case YAML files in:
   - testcases/
   - tests/sample/
   - data/
   - test-acceptance/test_cases/
2. Validate each file against schemas/test-case.schema.json
3. Generate reports/validation_report.txt
4. Display summary to console

### View Generated Report
```bash
cat reports/validation_report.txt
```

### Build Validation Tool
```bash
cargo build --bin validate-yaml
```

---

## Error Types Found

### Missing Required Fields (8 errors across 2 files)

| Field | Occurrences | Files |
|-------|-------------|-------|
| general_initial_conditions | 2 | SGP.22_4.4.2.yaml, data.yml |
| requirement | 1 | data.yml |
| item | 1 | data.yml |
| tc | 1 | data.yml |
| id | 1 | data.yml |
| initial_conditions | 1 | data.yml |
| test_sequences | 1 | data.yml |

**Note**: 7 of 8 errors are from data.yml (a non-test-case file)

---

## Fix Implementation Guide

### Fix Priority 1: tests/sample/SGP.22_4.4.2.yaml

**Add missing field**:
```yaml
general_initial_conditions:
  - "eUICC in initial state"
  - "Test environment configured"
```

**Validate**:
```bash
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml
```

### Fix Priority 2: tests/sample/data.yml

**Exclude from validation** by editing `scripts/generate_validation_report.py`:
```python
exclude_patterns = [
    # ... existing patterns ...
    "data.yml",  # Non-test-case metadata files
]
```

---

## Expected Results After Fixes

```
================================================================================
                              Summary
================================================================================

Total files validated: 169
Passed: 169
Failed: 0

✓ All test case files passed validation!
```

**Pass Rate**: 100% ✅

---

## Schema Requirements

All test case files must include these root-level fields:

1. `requirement` (string) - Requirement identifier
2. `item` (number) - Item number
3. `tc` (number) - Test case number
4. `id` (string) - Unique test case ID
5. `description` (string) - Test case description
6. `general_initial_conditions` (array) - General setup conditions
7. `initial_conditions` (object) - Component-specific conditions
8. `test_sequences` (array) - Test sequence definitions

Optional fields:
- `test_setup`, `prerequisites`, `hydration`, `hooks`, `dependencies`, `metadata`

---

## Tools and Commands

### Validation Commands
```bash
# Generate full report
make validate-testcases-report

# Validate single file
./target/debug/validate-yaml --schema schemas/test-case.schema.json <file>

# Build validator
cargo build --bin validate-yaml
```

### Report Files Location
```bash
ls -lh reports/
```

### View Reports
```bash
# Quick reference
cat reports/QUICK_REFERENCE.md

# Full report
cat reports/validation_report.txt

# Analysis
cat reports/VALIDATION_ANALYSIS.md

# Fix guide
cat reports/validation_fix_guide.md

# Failures summary
cat reports/validation_failures_summary.md
```

---

## Report Generation Details

**Generator Script**: `scripts/generate_validation_report.py`  
**Python Version**: 3.14 (via uv)  
**Schema**: schemas/test-case.schema.json  
**Validator**: ./target/debug/validate-yaml (Rust binary)  

**Directories Scanned**:
- testcases/
- tests/sample/
- data/
- test-acceptance/test_cases/

**Files Excluded** (not test cases):
- Files with "te.y" in name (malformed test files)
- Files with "wrong" in name (intentionally wrong)
- sample_test_runs.yaml
- Files in /expected_test_results/ directories
- Files in /expected_output_reports/
- Files in /testcase_results_container/
- container_config.yml, container_data.yml
- Files ending with _report.yaml

---

## Additional Resources

- **Schema File**: schemas/test-case.schema.json
- **Validation Script**: scripts/generate_validation_report.py
- **Example Passing Tests**: test-acceptance/test_cases/ (all 98 files)
- **Makefile Target**: make validate-testcases-report

---

## Change Log

### 2026-03-19
- Generated initial validation report (170 files validated)
- Identified 2 failing files in tests/sample/
- Confirmed 100% pass rate for test-acceptance/test_cases/ (98 files)
- Created comprehensive analysis and fix guides
- Updated validation script to include test-acceptance/test_cases/ directory

---

## Questions?

For issues or questions about:
- **Validation failures**: See [validation_fix_guide.md](validation_fix_guide.md)
- **Error patterns**: See [validation_failures_summary.md](validation_failures_summary.md)
- **Statistics**: See [VALIDATION_ANALYSIS.md](VALIDATION_ANALYSIS.md)
- **Quick help**: See [QUICK_REFERENCE.md](QUICK_REFERENCE.md)
