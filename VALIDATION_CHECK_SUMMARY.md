# Test Case Validation Check - Quick Summary

## What Was Implemented

A comprehensive validation system for checking all test case YAML files against the JSON schema.

## Command to Run

```bash
make validate-testcases-report
```

## What It Does

1. **Discovers** all test case YAML files in:
   - `testcases/`
   - `test-acceptance/`
   - `tests/sample/`

2. **Validates** each file against `schemas/test-case.schema.json`

3. **Generates** files:
   - `testcase_validation_report.txt` - Detailed report with errors
   - `backlog.md` - Tracking document for fixes
   - `backlog/tasks/TCMS-*.md` - Individual MCP tasks for each failed file (if enabled)

4. **Displays** results with color-coded output using logger.sh

5. **Creates MCP tasks** automatically for each failed file (can be disabled with `USE_MCP=false`)

## Files Currently Failing

Run the command above to see which files are currently failing validation. The results will be in:
- Console output (real-time)
- `testcase_validation_report.txt` (detailed)
- `backlog.md` (checklist format)

## Validation Individual File

To check a specific test case file:

```bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json path/to/file.yaml
```

## Output Files

### testcase_validation_report.txt

Detailed report showing:
- Total files validated
- Number passed/failed
- List of all files with pass/fail status
- Detailed error messages for failures

### backlog.md

Tracking document showing:
- List of failed files (grouped by directory)
- Checklist format for tracking fixes
- Instructions for validation
- Next steps

**Note:** `testcase_validation_report.txt` and `backlog.md` are in `.gitignore` and should NOT be committed. However, the MCP task files in `backlog/tasks/` should be committed as they are part of the project's task tracking system.

## Implementation Files

| File | Purpose |
|------|---------|
| `scripts/validate_all_testcases.sh` | Main validation script |
| `Makefile` (line ~294) | Make target `validate-testcases-report` |
| `docs/VALIDATION_REPORT.md` | Comprehensive documentation |
| `README_VALIDATION.md` | User guide |
| `IMPLEMENTATION_VALIDATION_CHECK.md` | Implementation details |
| `.gitignore` | Excludes generated reports |

## How to Fix Failing Files

1. **Run validation:**
   ```bash
   make validate-testcases-report
   ```

2. **Check backlog.md for list of failed files**

3. **For each failed file, see detailed error:**
   ```bash
   cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json <file>
   ```

4. **Fix the schema violations in the file**

5. **Verify the fix:**
   ```bash
   cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json <file>
   ```

6. **Re-run full validation:**
   ```bash
   make validate-testcases-report
   ```

## Common Errors

### Missing Required Property

**Fix:** Add the missing field to your YAML file

### Invalid Type

**Fix:** Ensure the field type matches the schema (e.g., integer not string)

### oneOf Constraint Violation

**Fix:** Ensure the value matches one of the allowed patterns

## Integration with CI/CD

The script exits with:
- `0` if all tests pass
- `1` if any tests fail

This makes it suitable for CI/CD pipelines.

## Related Commands

```bash
# Build validation tools
make build

# Validate with MCP task creation (default)
make validate-testcases-report

# Validate without MCP task creation
USE_MCP=false make validate-testcases-report

# Quick validation (no detailed report)
make verify-testcases

# Continuous validation (watch mode)
make watch

# Validate single file
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json <file>
```

## MCP Integration

The validation script integrates with the backlog.md MCP server:

- **Automatic Task Creation**: Creates a task for each failed file
- **Task Format**: Follows TCMS task structure with frontmatter
- **Task Status**: All tasks created in "To Do" status
- **Task IDs**: Auto-incremented from highest existing TCMS number
- **Task Location**: `backlog/tasks/TCMS-{id}: Fix validation for {filename}.md`
- **Disable MCP**: Set `USE_MCP=false` to skip task creation

## Documentation

- **User Guide:** `README_VALIDATION.md`
- **Technical Details:** `docs/VALIDATION_REPORT.md`
- **Implementation:** `IMPLEMENTATION_VALIDATION_CHECK.md`

## Next Steps

1. Run `make validate-testcases-report`
2. Review generated `backlog.md` for failed files
3. Fix failed files one by one
4. Re-run validation to verify fixes
5. Do NOT commit the generated report files (they're in .gitignore)

## Summary

**Command:** `make validate-testcases-report`

**Generates:**
- `testcase_validation_report.txt` (detailed results)
- `backlog.md` (fix tracking)

**Purpose:** Validate all test case YAML files and track what needs fixing
