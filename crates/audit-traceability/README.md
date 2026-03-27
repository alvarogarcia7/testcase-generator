# Audit Traceability

A library for creating and managing audit traceability logs for test case processing.

## Overview

The audit traceability system tracks all stages of test case processing with cryptographic hashes (SHA-256) to ensure data integrity and provide an audit trail. This allows users to follow the processing of files through various stages and verify that files have not been tampered with.

## Features

- **Stage Tracking**: Track multiple processing stages for each test case
- **Cryptographic Hashing**: SHA-256 hashes for all tracked files
- **Verification**: Verify file integrity against recorded hashes
- **JSON Format**: Human-readable and machine-parsable JSON logs
- **Timestamping**: Automatic timestamping of audit log creation

## Data Structure

The audit traceability log follows this structure:

```json
{
  "date": "2024-01-15T10:30:00Z",
  "witness_key": "production-witness",
  "test_cases": {
    "TC1": {
      "stages": {
        "initial": {
          "path": "testcases/TC1.yaml",
          "sha256": "abc123..."
        },
        "05_shell_script": {
          "path": "output/TC1.sh",
          "sha256": "def456..."
        }
      }
    }
  }
}
```

## Usage

### Creating an Audit Log

```rust
use audit_traceability::{AuditTraceabilityLog, TestCaseAudit, StageInfo};

// Create a new audit log
let mut log = AuditTraceabilityLog::new("witness-key".to_string());

// Create a test case audit entry
let mut audit = TestCaseAudit::new();

// Add stages
let initial_stage = StageInfo::from_file("testcases/TC1.yaml")?;
audit.add_stage("initial", initial_stage);

let script_stage = StageInfo::from_file("output/TC1.sh")?;
audit.add_stage("05_shell_script", script_stage);

// Add test case to log
log.add_test_case("TC1", audit);

// Save to file
log.save_to_file("audit-traceability-log.json")?;
```

### Verifying Files

```rust
use audit_traceability::AuditTraceabilityLog;

// Load the audit log
let log = AuditTraceabilityLog::load_from_file("audit-traceability-log.json")?;

// Verify a specific test case
let result = log.verify_test_case("TC1")?;
if result.all_passed {
    println!("✓ All stages verified successfully");
} else {
    println!("✗ Verification failed");
    for stage_result in result.stage_results {
        println!("{}", stage_result.message);
    }
}

// Verify all test cases
let results = log.verify_all()?;
for result in results {
    result.print_summary();
}
```

## CLI Integration

The audit traceability functionality is integrated into the `test-executor` CLI:

### Create a new audit log

```bash
test-executor audit-log create --output audit-log.json --witness-key my-witness
```

### Add a test case to the audit log

```bash
test-executor audit-log add \
  --log-file audit-log.json \
  --test-case-id TC001 \
  --initial testcases/TC001.yaml \
  --shell-script output/TC001.sh
```

### Generate script and automatically update audit log

```bash
test-executor generate testcases/TC001.yaml \
  --output output/TC001.sh \
  --audit-log audit-log.json
```

### Verify all files in the audit log

```bash
test-executor audit-log verify --log-file audit-log.json
```

### Verify a specific test case

```bash
test-executor audit-log verify --log-file audit-log.json --test-case-id TC001
```

## Stage Names

Common stage names used in the system:

- `initial`: Original YAML test case file
- `05_shell_script`: Generated bash script
- `execution_log`: Execution log JSON (if tracked)
- `resolved`: Dependency-resolved YAML (if tracked)

Custom stage names can be used as needed.

## Security Considerations

- SHA-256 hashes provide cryptographic assurance of file integrity
- Audit logs should be stored securely and backed up
- The witness key can be used to identify the authority or system that created the log
- Timestamps are in UTC format for consistency

## License

See the repository root for license information.
