# Test Case Validation Analysis Report

**Generated**: 2026-03-19  
**Schema**: schemas/test-case.schema.json  
**Validation Tool**: validate-yaml (Rust binary)  

---

## Executive Summary

✅ **Overall Status**: 98.8% Pass Rate (168/170 files)  
❌ **Failures**: 2 files in tests/sample/ directory  
✅ **Test Acceptance Suite**: 100% Pass Rate (98/98 files in test-acceptance/test_cases/)  

### Key Findings

1. All test case files in `test-acceptance/test_cases/` directory pass validation (98 files, 100% pass rate)
2. All test case files in `testcases/` directory pass validation (68 files, 100% pass rate)
3. Only 2 files fail validation, both in `tests/sample/` directory:
   - **tests/sample/SGP.22_4.4.2.yaml** - Missing `general_initial_conditions` field (1 error)
   - **tests/sample/data.yml** - Not a test case file (7 errors)

---

## Validation Results by Directory

| Directory | Total Files | Passed | Failed | Pass Rate |
|-----------|-------------|--------|--------|-----------|
| test-acceptance/test_cases/ | 98 | 98 | 0 | 100% ✅ |
| testcases/ | 68 | 68 | 0 | 100% ✅ |
| tests/sample/ | 4 | 2 | 2 | 50% ⚠️ |
| data/ | 2 | 2 | 0 | 100% ✅ |
| **TOTAL** | **170** | **168** | **2** | **98.8%** |

---

## Detailed Breakdown by Subdirectory

### test-acceptance/test_cases/ (98 files - ALL PASSING ✅)

| Subdirectory | Files | Status |
|--------------|-------|--------|
| bash_commands/ | 15 | ✅ All passing |
| complex/ | 9 | ✅ All passing |
| dependencies/ | 8 | ✅ All passing |
| failure/ | 12 | ✅ All passing |
| hooks/ | 14 | ✅ All passing |
| manual/ | 9 | ✅ All passing |
| prerequisites/ | 7 | ✅ All passing |
| success/ | 13 | ✅ All passing |
| variables/ | 11 | ✅ All passing |

**Test Categories Covered**:
- Bash command execution (arrays, conditionals, loops, file ops, etc.)
- Complex scenarios (hooks, BDD, performance, security)
- Dependency management (circular, nested, sequence)
- Failure scenarios (command errors, exit codes, output mismatches)
- Hook lifecycle management (before/after sequence/step, setup/teardown)
- Manual test steps and verification
- Prerequisites (automatic, manual, mixed)
- Success scenarios (file ops, variables, conditionals)
- Variable capture and substitution

---

## Failed Files Analysis

### File 1: tests/sample/SGP.22_4.4.2.yaml

**Status**: ❌ FAILED  
**Error Count**: 1  
**Error Type**: Missing Required Fields  

**Missing Field**:
- `general_initial_conditions` (required property at root level)

**Current Structure** (abbreviated):
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

**Fix Required**: Add `general_initial_conditions` field
```yaml
general_initial_conditions:
  - "Prerequisite condition 1"
  - "Prerequisite condition 2"
```

**Complexity**: LOW - Single field addition  
**Priority**: HIGH - This is a legitimate test case that should pass

---

### File 2: tests/sample/data.yml

**Status**: ❌ FAILED  
**Error Count**: 7  
**Error Type**: Missing Required Fields  

**Missing Fields**:
1. `requirement`
2. `item`
3. `tc`
4. `id`
5. `general_initial_conditions`
6. `initial_conditions`
7. `test_sequences`

**Current Content**:
```yaml
date: "2020-03-16"
description: "Global and regional mobile industry indicators..."
product: "GSMA Mobile Economy 2020"
```

**Analysis**: This file is **NOT a test case** - it's metadata about a GSMA Mobile Economy 2020 report. It should not be validated against the test case schema.

**Recommended Fix**: Exclude from validation by adding to exclusion patterns in `scripts/generate_validation_report.py`

**Complexity**: TRIVIAL - Add one line to exclusion list  
**Priority**: MEDIUM - File should be excluded, not fixed

---

## Error Pattern Analysis

### Error Distribution

| Error Category | Count | Percentage |
|----------------|-------|------------|
| Missing Required Fields | 8 | 100% |
| Invalid Field Values | 0 | 0% |
| Schema Constraint Violations | 0 | 0% |
| Type Mismatch | 0 | 0% |
| Malformed YAML | 0 | 0% |
| Additional Properties | 0 | 0% |

### Most Common Missing Fields

| Field Name | Occurrences | Files Affected |
|------------|-------------|----------------|
| general_initial_conditions | 2 | SGP.22_4.4.2.yaml, data.yml |
| requirement | 1 | data.yml |
| item | 1 | data.yml |
| tc | 1 | data.yml |
| id | 1 | data.yml |
| initial_conditions | 1 | data.yml |
| test_sequences | 1 | data.yml |

**Key Insight**: The `data.yml` file accounts for 7 of 8 errors (87.5%). Excluding this non-test-case file would reduce failures to just 1 file.

---

## Schema Compliance Analysis

### Required Root-Level Fields (Per Schema)

All test case YAML files must contain:

1. ✅ `requirement` (string) - Requirement identifier
2. ✅ `item` (number) - Item number
3. ✅ `tc` (number) - Test case number
4. ✅ `id` (string) - Unique test case ID
5. ✅ `description` (string) - Test case description
6. ✅ `general_initial_conditions` (array) - General setup conditions
7. ✅ `initial_conditions` (object) - Specific initial conditions per component
8. ✅ `test_sequences` (array) - Test sequence definitions

### Optional Fields Observed in Passing Tests

- `test_setup` - Setup steps before test execution
- `prerequisites` - Manual and automatic prerequisites
- `hydration` - Environment variable configuration
- `hooks` - Lifecycle hooks (script_start, setup_test, before_sequence, etc.)
- `dependencies` - Inter-test dependencies
- `metadata` - Additional metadata for documentation

---

## Fix Recommendations

### Priority 1: Fix Legitimate Test Case (IMMEDIATE)

**File**: tests/sample/SGP.22_4.4.2.yaml  
**Action**: Add missing `general_initial_conditions` field  
**Estimated Effort**: 2 minutes  
**Impact**: Reduces failures from 2 to 1 (50% reduction)  

**Implementation**:
```yaml
# Add after line 5 (after tc: 42)
general_initial_conditions:
  - "eUICC in initial state"
  - "Test environment configured"
```

**Validation Command**:
```bash
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml
```

---

### Priority 2: Exclude Non-Test-Case File (RECOMMENDED)

**File**: tests/sample/data.yml  
**Action**: Add to validation exclusion patterns  
**Estimated Effort**: 1 minute  
**Impact**: Reduces validation scope by 1 file, eliminates 7 errors  

**Implementation**:
Edit `scripts/generate_validation_report.py` line 71-82:
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
    "data.yml",  # ADD THIS LINE - Non-test-case metadata files
]
```

---

## Success Criteria After Fixes

After implementing both recommendations:

```
================================================================================
                              Summary
================================================================================

Total files validated: 169
Passed: 169
Failed: 0

✓ All test case files passed validation!
```

**Expected Outcome**:
- 100% pass rate
- 169 valid test case files
- 1 file excluded (data.yml)

---

## Validation Commands Reference

### Generate Full Report
```bash
make validate-testcases-report
cat reports/validation_report.txt
```

### Validate Individual Files
```bash
# Validate specific file
./target/debug/validate-yaml --schema schemas/test-case.schema.json <file.yaml>

# Validate failing files
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/data.yml
```

### Build Validation Tool
```bash
cargo build --bin validate-yaml
```

---

## Test Case Categories Successfully Validated

### By Functionality (All Passing ✅)

1. **Bash Command Execution** (15 files)
   - Arrays, conditionals, loops, string operations
   - File operations, process management
   - Math operations, redirection

2. **Complex Scenarios** (9 files)
   - Multi-hook integration
   - BDD with hooks and variables
   - Data-driven iterations
   - Performance timing
   - Security authentication

3. **Dependency Management** (8 files)
   - Simple, nested, complex dependencies
   - Circular dependency detection
   - Missing dependency handling
   - Self-referencing tests

4. **Failure Handling** (12 files)
   - Command not found
   - Exit code mismatches
   - Output mismatches
   - Permission denied
   - Variable undefined

5. **Hook Lifecycle** (14 files)
   - script_start, script_end
   - setup_test, teardown_test
   - before_sequence, after_sequence
   - before_step, after_step
   - Error handling (continue/fail)

6. **Manual Testing** (9 files)
   - Manual verification steps
   - Mixed auto/manual workflows
   - File inspection
   - Result verification

7. **Prerequisites** (7 files)
   - Automatic prerequisites
   - Manual prerequisites
   - Mixed prerequisites
   - Prerequisite failures

8. **Success Scenarios** (13 files)
   - Simple command execution
   - File operations
   - Environment variables
   - Conditional logic
   - Regex validation

9. **Variable Management** (11 files)
   - Single and multiple capture
   - Command-based capture
   - Cross-sequence variables
   - JSON extraction
   - Complex substitution

---

## Repository Statistics

### File Distribution
- **Total YAML Files Discovered**: 170
- **Test Cases (Validated)**: 169 (after excluding data.yml)
- **Non-Test-Case Files (Excluded)**: 1

### Directory Structure Quality
- **test-acceptance/test_cases/**: Excellent (100% pass rate, 98 files)
- **testcases/**: Excellent (100% pass rate, 68 files)
- **tests/sample/**: Needs attention (50% pass rate, 2/4 failing)
- **data/**: Good (100% pass rate, 2 files)

---

## Conclusions

1. **Strong Overall Compliance**: 98.8% of files pass validation
2. **Excellent Test Acceptance Suite**: All 98 files in test-acceptance/test_cases/ pass (100%)
3. **Minimal Issues**: Only 2 files fail, both easily resolved
4. **Clear Error Patterns**: All errors are missing required fields (no complex schema violations)
5. **Quick Fix Path**: Both issues can be resolved in < 5 minutes

### Recommended Next Steps

1. ✅ Add `general_initial_conditions` to tests/sample/SGP.22_4.4.2.yaml
2. ✅ Exclude data.yml from validation (add to exclusion patterns)
3. ✅ Re-run validation report to confirm 100% pass rate
4. ✅ Document `general_initial_conditions` field in schema documentation
5. ✅ Update test case templates to include all required fields

---

## Additional Resources

- **Full Validation Report**: reports/validation_report.txt
- **Fix Guide**: reports/validation_fix_guide.md
- **Failures Summary**: reports/validation_failures_summary.md
- **Schema File**: schemas/test-case.schema.json
- **Validation Tool**: ./target/debug/validate-yaml

---

**Report Generated By**: scripts/generate_validation_report.py  
**Python Version**: 3.14  
**Schema Validator**: Rust validate-yaml binary  
**Report Format**: Markdown
