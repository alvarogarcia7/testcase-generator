# Quick Fix Guide for Test Case Validation Failures

## Summary
- **Total Failures**: 2 files
- **Actual Test Cases to Fix**: 1 file
- **Non-Test-Case Files**: 1 file (should be excluded)

---

## Fix #1: tests/sample/SGP.22_4.4.2.yaml

### Issue
Missing required field: `general_initial_conditions`

### Current File Structure
```yaml
description: Sample test case for demonstration
id: SGP.22_4.4.2
item: 4
requirement: SGP.22
tc: 42
initial_conditions:
  LPA: ["test"]
test_sequences:
  - ...
```

### Fix: Add general_initial_conditions
```yaml
description: Sample test case for demonstration
id: SGP.22_4.4.2
item: 4
requirement: SGP.22
tc: 42
general_initial_conditions:  # ADD THIS FIELD
  - "Required condition 1"
  - "Required condition 2"
initial_conditions:
  LPA: ["test"]
test_sequences:
  - ...
```

### Example from Passing Test Cases
Reference: `testcases/gsma_4.4.2.2_TC.yml` (which passes validation)
```yaml
general_initial_conditions:
  - eUICC Memory Reset
  - Prepare eSIM with notification
test_setup:
  - Reset LPA to factory defaults
  - Configure test environment
```

### Validation Command
```bash
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml
```

---

## Fix #2: tests/sample/data.yml

### Issue
This file is **NOT a test case** - it's metadata about a GSMA report.

### Current Content
```yaml
date: "2020-03-16"
description: "Global and regional mobile industry indicators..."
product: "GSMA Mobile Economy 2020"
```

### Recommended Actions

#### Option A: Exclude from Validation (Recommended)
Update `scripts/generate_validation_report.py`:

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
    "data.yml",  # ADD THIS LINE - Non-test-case data files
]
```

#### Option B: Move to Different Directory
```bash
# Move to a non-validated directory
mkdir -p docs/reference_data
mv tests/sample/data.yml docs/reference_data/
```

#### Option C: Rename to Indicate Non-Test-Case
```bash
# Rename with prefix that's excluded
mv tests/sample/data.yml tests/sample/reference_data.yml
# Then add to exclusion: "reference_data.yml"
```

**Recommendation**: Use Option A (add to exclusion patterns) as it's the simplest and clearest.

---

## Schema Requirements Reference

### Required Root-Level Fields (Test Case Schema)

All test case YAML files must have:

1. **`requirement`** (string) - The requirement identifier (e.g., "SGP.22")
2. **`item`** (number) - Item number
3. **`tc`** (number) - Test case number
4. **`id`** (string) - Unique test case ID
5. **`description`** (string) - Test case description
6. **`general_initial_conditions`** (array) - General setup conditions
7. **`initial_conditions`** (object) - Specific initial conditions
8. **`test_sequences`** (array) - Test sequence definitions

### Optional Root-Level Fields

- `test_setup` (array) - Setup steps
- `prerequisites` (object) - Test prerequisites
- `hydration` (object) - Environment variable configuration
- `hooks` (object) - Lifecycle hooks
- `dependencies` (object) - Test dependencies
- `metadata` (object) - Additional metadata

---

## Verification Steps

### Step 1: Fix SGP.22_4.4.2.yaml
```bash
# Edit the file
vim tests/sample/SGP.22_4.4.2.yaml

# Add general_initial_conditions field (see example above)

# Validate
./target/debug/validate-yaml --schema schemas/test-case.schema.json tests/sample/SGP.22_4.4.2.yaml
```

### Step 2: Exclude data.yml
```bash
# Edit validation script
vim scripts/generate_validation_report.py

# Add "data.yml" to exclude_patterns list

# Save and exit
```

### Step 3: Re-run Full Validation
```bash
make validate-testcases-report
```

### Step 4: Verify Success
```bash
# Check the report
cat reports/validation_report.txt

# Look for:
# - Total files validated: 169 (was 170, now excluding data.yml)
# - Passed: 169
# - Failed: 0
```

---

## Expected Outcome

After applying both fixes:

```
================================================================================
                              Summary
================================================================================

Total files validated: 169
Passed: 169
Failed: 0

✓ All test case files passed validation!
```

---

## Additional Resources

### Schema Documentation
- Schema file: `schemas/test-case.schema.json`
- Passing examples: `test-acceptance/test_cases/` (all 98 files pass)
- Complex examples: `testcases/` (all 68 files pass)

### Validation Tools
- Manual validation: `./target/debug/validate-yaml --schema <schema> <file>`
- Full report: `make validate-testcases-report`
- Watch mode: `make watch`

### Common Patterns

**Minimal Valid Test Case**:
```yaml
requirement: "REQ-001"
item: 1
tc: 1
id: "TC-001"
description: "Test description"
general_initial_conditions:
  - "Initial condition"
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Sequence 1"
    description: "Sequence description"
    steps:
      - step: 1
        description: "Step 1"
        command: "echo test"
        expected:
          result: 0
          success: true
```

---

## Questions or Issues?

If validation continues to fail after these fixes:

1. Check the detailed error output: `./target/debug/validate-yaml --schema schemas/test-case.schema.json <file>`
2. Compare with passing examples in `test-acceptance/test_cases/`
3. Review the schema: `schemas/test-case.schema.json`
4. Check exclude patterns in `scripts/generate_validation_report.py`
