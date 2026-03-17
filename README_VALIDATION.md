# Test Case Validation Guide

This guide explains how to validate all test case YAML files in the repository and manage validation failures.

## Quick Start

To validate all test cases and generate a report:

```bash
make validate-testcases-report
```

This will:
1. Build the validation tools
2. Validate all test case YAML files
3. Generate a detailed report in `testcase_validation_report.txt`
4. Create/update `backlog.md` with failed files
5. Create MCP tasks in `backlog/tasks/` for each failed file (if MCP is enabled)

## What Gets Validated

The validation script checks all YAML files in these directories:
- `testcases/` - Main test case files
- `test-acceptance/` - Acceptance test suite
- `tests/sample/` - Sample test cases

Files are validated against the JSON schema defined in `schemas/test-case.schema.json`.

## Understanding the Output

### Console Output

During validation, you'll see real-time progress:

```
Starting validation of all test case files...
Schema file: schemas/test-case.schema.json

Found 150 test case files

Validation Results
✓ testcases/examples/TC_EXAMPLE_001.yaml
✗ testcases/examples/TC_EXAMPLE_002.yaml
  Error details:
    Schema validation failed:
      - Path 'root': Missing required property 'test_sequences'
...

Summary
Total files validated: 150
✓ Passed: 145
✗ Failed: 5
```

### Generated Files

#### 1. `testcase_validation_report.txt`

A detailed report containing:
- Summary statistics
- Full list of validated files with pass/fail status
- Detailed error messages for each failure

**Example:**
```
# Test Case Validation Report
Generated: Mon Jan 15 10:30:45 PST 2024

## Summary
- Total files validated: 150
- Passed: 145
- Failed: 5

## Validation Results

✓ testcases/1.yaml
✗ testcases/examples/TC_EXAMPLE_002.yaml
  Error details:
    Schema validation failed:
      - Path 'root': Missing required property 'test_sequences'
```

#### 2. `backlog.md`

A tracking document for fixing failed test cases:

**When there are failures:**
```markdown
# Test Case Validation Backlog

**Generated:** Mon Jan 15 10:30:45 PST 2024
**Total Failed Files:** 5

## How to Validate

To validate all test cases, run:
`make validate-testcases-report`

To validate a specific file:
`cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json <file_path>`

## Failed Files

### testcases/examples
- [ ] `testcases/examples/TC_EXAMPLE_002.yaml`
- [ ] `testcases/examples/TC_EXAMPLE_005.yaml`

### test-acceptance/test_cases/failure
- [ ] `test-acceptance/test_cases/failure/TC_FAILURE_001.yaml`

## Next Steps
1. Review each failed file
2. Run validation on individual files to see specific errors
3. Fix the schema violations
4. Re-run validation to verify fixes
5. Check off completed items
```

**When all tests pass:**
```markdown
# Test Case Validation Backlog

**Generated:** Mon Jan 15 10:30:45 PST 2024

✅ All test cases are valid! No items in backlog.
```

#### 3. MCP Tasks in `backlog/tasks/`

When validation fails and MCP is enabled (default), the script automatically creates a task in the backlog for each failed file:

**Generated Task Example:**
```markdown
---
id: TCMS-14
title: Fix validation for TC_EXAMPLE_002.yaml
status: To Do
assignee: []
created_date: '2024-01-15'
labels:
  - validation
  - test-case
  - schema
dependencies: []
---

## Description

This test case file failed schema validation and needs to be fixed.

**File:** `testcases/examples/TC_EXAMPLE_002.yaml`

### Validation Error

Run the following command to see the validation error:

\`\`\`bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json testcases/examples/TC_EXAMPLE_002.yaml
\`\`\`

### How to Fix

1. Run the validation command above to see specific error messages
2. Review the schema at `schemas/test-case.schema.json`
3. Fix the YAML file to conform to the schema
4. Re-run validation to verify the fix
5. Once fixed, run full validation: `make validate-testcases-report`

### Definition of Done
- [ ] #1 File passes schema validation
- [ ] #2 No validation errors when running validate-yaml
- [ ] #3 Full validation suite passes
```

**Features:**
- Each failed file gets its own task with a unique TCMS ID
- Tasks are created in `To Do` status
- Task IDs automatically increment from the highest existing task number
- Tasks include the file path and validation command
- Can be disabled by setting `USE_MCP=false`

## Validating Individual Files

To check a specific test case:

```bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json path/to/testcase.yaml
```

For detailed output:

```bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json --verbose path/to/testcase.yaml
```

## Common Validation Errors and Fixes

### Missing Required Property

**Error:**
```
- Path 'root': Missing required property 'test_sequences'
```

**Fix:** Add the missing required field to your YAML file:
```yaml
test_sequences:
  - id: 1
    name: "Test Sequence"
    description: "Test description"
    # ... rest of sequence definition
```

### Invalid Type

**Error:**
```
- Path 'item': Invalid type (expected integer, got string)
```

**Fix:** Ensure the field has the correct type:
```yaml
# Wrong
item: "1"

# Correct
item: 1
```

### oneOf Constraint Violation

**Error:**
```
- Path 'initial_conditions/eUICC[1]': Value does not match any of the allowed schemas (oneOf constraint)
```

**Fix:** The value must match one of the allowed patterns. For initial conditions, items must be either:
- A string: `"condition description"`
- An object with `ref`: `{ ref: "some_ref" }`
- An object with `test_sequence`: `{ test_sequence: { id: 1, step: "2" } }`

## Working Through the Backlog

Recommended workflow for fixing failed test cases:

### Step 1: Run Initial Validation

```bash
make validate-testcases-report
```

Review the generated `backlog.md` to see all failed files.

### Step 2: Fix One File at a Time

For each failed file:

1. **Check the specific error:**
   ```bash
   cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json testcases/examples/TC_EXAMPLE_002.yaml
   ```

2. **Edit the file to fix the error**

3. **Verify the fix:**
   ```bash
   cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json testcases/examples/TC_EXAMPLE_002.yaml
   ```

4. **Mark as complete in backlog.md:**
   ```markdown
   - [x] `testcases/examples/TC_EXAMPLE_002.yaml`
   ```

### Step 3: Re-run Full Validation

After fixing several files, re-run the full validation:

```bash
make validate-testcases-report
```

This will update the backlog with remaining failures.

### Step 4: Commit Your Changes

Once all validations pass:

```bash
git add testcases/
git commit -m "Fix test case schema violations"
```

**Note:** The report files (`testcase_validation_report.txt` and `backlog.md`) are in `.gitignore` and should not be committed.

## Configuration Options

You can customize the validation behavior with environment variables:

### Custom Schema File

```bash
SCHEMA_FILE=path/to/custom-schema.json ./scripts/validate_all_testcases.sh
```

### Custom Output Location

```bash
OUTPUT_FILE=reports/validation.txt ./scripts/validate_all_testcases.sh
```

### Custom Backlog Location

```bash
BACKLOG_FILE=docs/backlog.md ./scripts/validate_all_testcases.sh
```

### Disable MCP Task Creation

```bash
USE_MCP=false ./scripts/validate_all_testcases.sh
```

### Combined Example

```bash
SCHEMA_FILE=schemas/test-case.schema.json \
OUTPUT_FILE=reports/validation.txt \
BACKLOG_FILE=docs/backlog.md \
USE_MCP=true \
./scripts/validate_all_testcases.sh
```

## Integration with Development Workflow

### Pre-Commit Hook

Add validation to your pre-commit workflow:

```bash
#!/bin/bash
# .git/hooks/pre-commit
make validate-testcases-report
```

### CI/CD Integration

Example GitLab CI configuration:

```yaml
validate-testcases:
  stage: validate
  script:
    - make build
    - make validate-testcases-report
  artifacts:
    paths:
      - testcase_validation_report.txt
      - backlog.md
    when: always
  allow_failure: false
```

### Watch Mode

For continuous validation during development:

```bash
make watch
```

This watches for changes and automatically re-validates.

## Exit Codes

The validation script uses standard exit codes:

- `0` - All test cases are valid
- `1` - One or more test cases failed validation

This allows easy integration with CI/CD pipelines and scripts:

```bash
if make validate-testcases-report; then
    echo "All tests valid!"
else
    echo "Validation failures detected"
    cat backlog.md
fi
```

## Related Documentation

- [Validation Report Details](docs/VALIDATION_REPORT.md) - Comprehensive validation documentation
- [AGENTS.md](AGENTS.md) - Development guidelines and commands
- [README.md](README.md) - Main project documentation

## Troubleshooting

### Schema File Not Found

**Error:** `Schema file not found: schemas/test-case.schema.json`

**Solution:** Ensure you're running the command from the repository root directory.

### Build Failures

**Error:** Validation fails to build

**Solution:** Run `make build` first to ensure all tools are compiled:
```bash
make build
make validate-testcases-report
```

### Permission Denied

**Error:** `Permission denied: ./scripts/validate_all_testcases.sh`

**Solution:** The script should already be executable. If not, check file permissions or run via bash:
```bash
bash scripts/validate_all_testcases.sh
```

## Summary

**Quick command to validate all test cases:**
```bash
make validate-testcases-report
```

**Files generated:**
- `testcase_validation_report.txt` - Detailed validation results
- `backlog.md` - Tracking document for fixes

**Key validation command:**
```bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json <file>
```

For more details, see [docs/VALIDATION_REPORT.md](docs/VALIDATION_REPORT.md).
