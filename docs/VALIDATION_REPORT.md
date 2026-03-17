# Test Case Validation Report

This document describes how to validate all test case YAML files against the JSON schema and generate a validation report.

## Overview

The validation system checks all test case YAML files in the repository to ensure they conform to the JSON schema defined in `schemas/test-case.schema.json`. This helps maintain consistency and catch schema violations early.

## Running Validation

### Using Make Target

The easiest way to run validation is using the Makefile target:

```bash
make validate-testcases-report
```

This command will:
1. Build the project
2. Discover all test case YAML files
3. Validate each file against the schema
4. Generate a detailed report
5. Create a backlog of failed files (if any)
6. Create MCP tasks for each failed file (if enabled)

### Using Script Directly

You can also run the validation script directly:

```bash
./scripts/validate_all_testcases.sh
```

### Custom Configuration

You can customize the validation using environment variables:

```bash
# Use a different schema file
SCHEMA_FILE=path/to/schema.json ./scripts/validate_all_testcases.sh

# Change output file location
OUTPUT_FILE=my_report.txt ./scripts/validate_all_testcases.sh

# Change backlog file location
BACKLOG_FILE=my_backlog.md ./scripts/validate_all_testcases.sh

# Disable MCP task creation
USE_MCP=false ./scripts/validate_all_testcases.sh
```

## Output Files

The validation process generates the following files:

### 1. Validation Report (`testcase_validation_report.txt`)

A detailed text report containing:
- Summary statistics (total files, passed, failed)
- List of all validated files with pass/fail status
- Detailed error messages for failed files

Example:
```
# Test Case Validation Report
Generated: Mon Jan 15 10:30:45 PST 2024

## Summary
- Total files validated: 150
- Passed: 145
- Failed: 5

## Validation Results
✓ testcases/examples/TC_EXAMPLE_001.yaml
✗ testcases/examples/TC_EXAMPLE_002.yaml
  Error details:
    Schema validation failed:
      - Path 'root': Missing required property 'test_sequences'
```

### 2. Backlog File (`backlog.md`)

A Markdown file tracking test cases that need to be fixed. This file is organized by directory and includes:
- Summary of failed files
- Instructions for validation
- Checklist of failed files (organized by directory)
- Next steps for remediation

Example:
```markdown
# Test Case Validation Backlog

**Generated:** Mon Jan 15 10:30:45 PST 2024

**Total Failed Files:** 5

## How to Validate

To validate all test cases, run:
\`\`\`bash
make build
./scripts/validate_all_testcases.sh
\`\`\`

To validate a specific file:
\`\`\`bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json <file_path>
\`\`\`

## Failed Files

### testcases/examples

- [ ] `testcases/examples/TC_EXAMPLE_002.yaml`
- [ ] `testcases/examples/TC_EXAMPLE_005.yaml`

### test-acceptance/test_cases/failure

- [ ] `test-acceptance/test_cases/failure/TC_FAILURE_001.yaml`
```

### 3. MCP Task Files (`backlog/tasks/TCMS-*.md`)

When validation fails and MCP is enabled (default), individual task files are created for each failed file:

**Location:** `backlog/tasks/TCMS-{id}: Fix validation for {filename}.md`

**Task Structure:**
- YAML frontmatter with metadata (id, title, status, assignee, labels, created_date)
- Description with file path
- Validation error section with command to reproduce
- How to fix instructions
- Definition of done checklist

**Example Task:**
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
- Task IDs auto-increment from the highest existing TCMS task number
- All tasks created in "To Do" status
- Tasks integrate with backlog.md MCP server
- Can be disabled by setting `USE_MCP=false`

**Important:** These task files should be committed to the repository as part of the project's task tracking system.

## Validation Coverage

The validation script checks YAML files in the following directories:
- `testcases/` - Main test case directory
- `test-acceptance/` - Acceptance test cases
- `tests/sample/` - Sample test cases

Files are excluded if they match these patterns:
- `*te.y*` - Template files
- `sample_test_runs.yaml` - Test run data
- `*_wrong.*` - Intentionally invalid test files
- `*/incorrect/*` - Files in incorrect directories

## Validating Individual Files

To validate a single test case file:

```bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json path/to/testcase.yaml
```

To see detailed validation output:

```bash
cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json --verbose path/to/testcase.yaml
```

## Common Validation Errors

### Missing Required Properties

**Error:** `Missing required property 'test_sequences'`

**Fix:** Add the missing required property to your test case YAML file.

### Invalid Type

**Error:** `Invalid type (expected integer, got string)`

**Fix:** Ensure the property has the correct type as defined in the schema.

### Schema Constraint Violations

**Error:** `Value does not match any of the allowed schemas (oneOf constraint)`

**Fix:** Review the schema definition and ensure your value matches one of the allowed patterns.

## Integration with CI/CD

The validation script can be integrated into CI/CD pipelines:

```yaml
# Example GitLab CI configuration
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
```

## Exit Codes

The validation script returns the following exit codes:
- `0` - All test cases are valid
- `1` - One or more test cases failed validation

## Updating the Backlog

As you fix test case files:

1. Run validation to identify failures
2. Fix each failing file according to the error messages
3. Mark items as complete in `backlog.md` (change `[ ]` to `[x]`)
4. Re-run validation to verify fixes
5. Commit the fixed files

When all files pass validation, the backlog will automatically show:
```markdown
✅ All test cases are valid! No items in backlog.
```

## Related Commands

- `make verify-testcases` - Quick validation without detailed reports
- `make watch` - Watch mode for continuous validation
- `make build` - Build the validation tools
