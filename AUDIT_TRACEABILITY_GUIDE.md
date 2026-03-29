# Audit Traceability Guide

## Overview

The audit traceability system provides a comprehensive mechanism to track all stages of test case processing with cryptographic verification. This allows users to maintain an audit trail and verify file integrity throughout the test case lifecycle.

## JSON File Structure

The audit traceability log is stored in JSON format with the following structure:

```json
{
  "date": "2024-01-15T10:30:00Z",
  "witness_key": "production-witness",
  "test_cases": {
    "TC1": {
      "stages": {
        "initial": {
          "path": "testcases/TC1.yaml",
          "sha256": "abc123def456..."
        },
        "05_shell_script": {
          "path": "output/TC1.sh",
          "sha256": "def456abc123..."
        }
      }
    },
    "TC2": {
      "stages": {
        "initial": {
          "path": "testcases/TC2.yaml",
          "sha256": "789abc012def..."
        },
        "05_shell_script": {
          "path": "output/TC2.sh",
          "sha256": "012def789abc..."
        }
      }
    }
  }
}
```

### Fields

- **date**: ISO 8601 timestamp (UTC) of when the audit log was created
- **witness_key**: Identifier for the authority or system that created the log
- **test_cases**: Map of test case IDs to their audit information
  - **stages**: Map of stage names to stage information
    - **path**: File path relative to the project root or absolute path
    - **sha256**: SHA-256 hash of the file contents

## Common Stage Names

The system uses the following conventional stage names:

- **initial**: Original YAML test case file
- **05_shell_script**: Generated bash script from the test case
- **resolved**: Dependency-resolved YAML (optional)
- **execution_log**: Execution log JSON (optional)

Additional custom stage names can be added as needed.

## CLI Usage

### 1. Create a New Audit Log

Create a new empty audit log:

```bash
test-executor audit-log create \
  --output audit-traceability-log.json \
  --witness-key "production-system-2024"
```

**Options:**
- `--output` / `-o`: Path to the output JSON file (default: `audit-traceability-log.json`)
- `--witness-key` / `-w`: Identifier for the witness/authority (default: `default-witness`)

### 2. Generate Script with Automatic Audit Logging

Generate a test script and automatically update the audit log:

```bash
test-executor generate testcases/TC001.yaml \
  --output output/TC001.sh \
  --audit-log audit-traceability-log.json
```

This will:
1. Generate the shell script from the YAML file
2. Add both the YAML and generated script to the audit log
3. Calculate SHA-256 hashes for both files
4. Save the updated audit log

**Options:**
- `--audit-log`: Path to the audit log JSON file (creates if doesn't exist)

### 3. Manually Add a Test Case to Audit Log

Add or update a test case in the audit log:

```bash
test-executor audit-log add \
  --log-file audit-traceability-log.json \
  --test-case-id TC001 \
  --initial testcases/TC001.yaml \
  --shell-script output/TC001.sh
```

**Options:**
- `--log-file` / `-l`: Path to the audit log JSON file
- `--test-case-id`: Test case ID
- `--initial`: Path to the initial YAML file
- `--shell-script`: Path to the generated shell script (optional)

### 4. Verify Files in Audit Log

Verify all test cases in the audit log:

```bash
test-executor audit-log verify \
  --log-file audit-traceability-log.json
```

Output example:
```
=== Audit Verification Results ===

✓ Test Case: TC001 (PASS)
  ✓ Stage 'initial' verified
  ✓ Stage '05_shell_script' verified

✗ Test Case: TC002 (FAIL)
  ✓ Stage 'initial' verified
  ✗ Stage '05_shell_script' hash mismatch (file: output/TC002.sh)

=== Summary ===
Total test cases: 2
Passed: 1
Failed: 1
```

Verify a specific test case:

```bash
test-executor audit-log verify \
  --log-file audit-traceability-log.json \
  --test-case-id TC001
```

**Exit Codes:**
- `0`: All verifications passed
- `1`: One or more verifications failed

## Workflow Examples

### Example 1: Single Test Case

```bash
# Step 1: Create audit log
test-executor audit-log create \
  --output audit-log.json \
  --witness-key "my-system"

# Step 2: Generate script with audit logging
test-executor generate testcases/TC001.yaml \
  --output output/TC001.sh \
  --audit-log audit-log.json

# Step 3: Verify integrity
test-executor audit-log verify --log-file audit-log.json
```

### Example 2: Multiple Test Cases

```bash
# Create audit log
test-executor audit-log create --output audit-log.json

# Process multiple test cases
for tc in testcases/*.yaml; do
  tc_id=$(basename "$tc" .yaml)
  test-executor generate "$tc" \
    --output "output/${tc_id}.sh" \
    --audit-log audit-log.json
done

# Verify all at once
test-executor audit-log verify --log-file audit-log.json
```

### Example 3: Continuous Integration

```bash
#!/bin/bash
set -e

# Generate scripts with audit logging
test-executor generate testcases/TC001.yaml \
  --output output/TC001.sh \
  --audit-log audit-log.json

# Verify before committing
if ! test-executor audit-log verify --log-file audit-log.json; then
  echo "Error: Audit verification failed"
  exit 1
fi

# Commit both generated files and audit log
git add output/TC001.sh audit-log.json
git commit -m "Generated TC001 with audit trail"
```

## Library API Usage

For programmatic access, use the `audit-traceability` crate:

```rust
use audit_traceability::{AuditTraceabilityLog, StageInfo, TestCaseAudit};

// Create audit log
let mut log = AuditTraceabilityLog::new("witness-key".to_string());

// Add test case with stages
let mut audit = TestCaseAudit::new();
let initial = StageInfo::from_file("testcases/TC001.yaml")?;
audit.add_stage("initial", initial);

let script = StageInfo::from_file("output/TC001.sh")?;
audit.add_stage("05_shell_script", script);

log.add_test_case("TC001", audit);

// Save to file
log.save_to_file("audit-log.json")?;

// Load and verify
let log = AuditTraceabilityLog::load_from_file("audit-log.json")?;
let result = log.verify_test_case("TC001")?;

if result.all_passed {
    println!("✓ Verification passed");
} else {
    println!("✗ Verification failed");
}
```

## Best Practices

1. **Version Control**: Commit audit logs alongside generated artifacts to maintain a historical record

2. **Witness Keys**: Use descriptive witness keys that identify the system/environment:
   - `dev-machine-john-2024`
   - `ci-server-prod`
   - `build-server-v2`

3. **Regular Verification**: Verify audit logs before and after critical operations:
   - Before deployment
   - After file transfers
   - During code review

4. **Backup**: Keep backups of audit logs separately from the working directory

5. **Stage Naming**: Use consistent stage names across the project:
   - `initial` for source files
   - `05_shell_script` for generated scripts
   - `resolved` for resolved dependencies
   - `execution_log` for execution logs

6. **Automation**: Integrate audit logging into build scripts and CI/CD pipelines

## Security Considerations

- **SHA-256 Hashes**: Provides cryptographic assurance of file integrity
- **Tamper Detection**: Any modification to tracked files will be detected during verification
- **Non-Repudiation**: Timestamp and witness key provide audit trail
- **Read-Only After Creation**: Generated hashes are based on file content at creation time

## Troubleshooting

### Verification Fails After File Modification

This is expected behavior. If you've intentionally modified a file:

```bash
# Regenerate with updated audit log
test-executor generate testcases/TC001.yaml \
  --output output/TC001.sh \
  --audit-log audit-log.json
```

### File Path Issues

If files have been moved:

```bash
# Remove old entry and add new one
test-executor audit-log add \
  --log-file audit-log.json \
  --test-case-id TC001 \
  --initial new-location/TC001.yaml \
  --shell-script new-location/TC001.sh
```

### Large Audit Logs

For projects with many test cases, consider splitting audit logs by:
- Module/component
- Test suite
- Release version

## Integration with Existing Tools

The audit traceability system integrates seamlessly with:

- **test-executor**: Built-in commands for generation and verification
- **CI/CD**: Exit codes and JSON output for automation
- **Version Control**: JSON format is git-friendly
- **Documentation**: Audit logs serve as processing documentation

## Future Enhancements

Potential future features:
- Digital signatures for audit logs
- Audit log merging and diffing
- Historical trend analysis
- Multi-witness verification
- Audit log encryption
