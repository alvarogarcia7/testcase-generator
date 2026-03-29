# Audit Verifier

A comprehensive audit logging and verification system with digital signatures for test case operations.

## Overview

This crate provides:

1. **Audit Logging**: Track all test case operations (generate, execute, verify, etc.)
2. **Digital Signatures**: Sign audit logs with P-521 ECDSA keys
3. **Signature Verification**: Verify the integrity and authenticity of audit logs
4. **Hash Verification**: Verify test case YAML hashes against execution logs

## Features

- **Complete Operation Tracking**: Logs all operations with timestamps, user info, file hashes, and metadata
- **P-521 ECDSA Signatures**: Industry-standard elliptic curve digital signatures
- **Tamper Detection**: Any modification to the audit log invalidates the signature
- **File Hash Tracking**: SHA-256 hashes of input and output files
- **Comprehensive Metadata**: Captures user, hostname, working directory, command arguments, duration, and custom metadata

## Components

### Audit Log Structure

```rust
pub struct AuditLog {
    pub version: String,
    pub entries: Vec<AuditLogEntry>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub operation: OperationType,
    pub user: Option<String>,
    pub hostname: Option<String>,
    pub working_directory: Option<PathBuf>,
    pub input_files: Vec<PathBuf>,
    pub output_files: Vec<PathBuf>,
    pub input_file_hashes: Vec<(PathBuf, String)>,
    pub output_file_hashes: Vec<(PathBuf, String)>,
    pub command_args: Vec<String>,
    pub status: OperationStatus,
    pub error_message: Option<String>,
    pub duration_ms: Option<u64>,
    pub metadata: serde_json::Value,
}
```

### Operation Types

- `GenerateScript`: Script generation from YAML
- `ExecuteScript`: Script execution
- `VerifyTest`: Test verification
- `HydrateYaml`: YAML hydration with variables
- `GenerateExport`: Export file generation
- `ValidateExport`: Export file validation
- `ListTestCases`: Test case listing
- `ResolveDependencies`: Dependency resolution
- `ValidateYaml`: YAML validation
- `LoadTestCase`: Test case loading
- `SaveTestCase`: Test case saving
- `Other(String)`: Custom operations

## CLI Tools

### 1. sign-audit-log

Sign an audit log file with a private key.

```bash
# Generate new key and sign
sign-audit-log --log audit.log.json --output signed_audit.json --save-key private.pem

# Sign with existing key
sign-audit-log --log audit.log.json --output signed_audit.json --private-key private.pem --key-id "production-key"
```

**Options:**
- `--log PATH`: Path to audit log JSON file (required)
- `--output PATH`: Output file for signed audit log (required)
- `--private-key PATH`: Path to P-521 private key PEM file (optional, generates new if not provided)
- `--save-key PATH`: Path to save generated private key
- `--key-id ID`: Key identifier (default: "audit-signer")
- `--log-level LEVEL`: Set log level (trace, debug, info, warn, error)
- `-v, --verbose`: Enable verbose output

### 2. verify-audit-log

Verify a signed audit log file.

```bash
# Basic verification
verify-audit-log signed_audit.json

# Detailed verification with report output
verify-audit-log signed_audit.json --detailed --output verification_report.json
```

**Options:**
- `SIGNED_LOG`: Path to signed audit log JSON file (required)
- `-o, --output PATH`: Output file for verification report (JSON format)
- `-d, --detailed`: Show detailed information about each log entry
- `--log-level LEVEL`: Set log level
- `-v, --verbose`: Enable verbose output

**Exit Codes:**
- `0`: Verification successful
- `1`: Verification failed

### 3. audit-verifier

Verify test case YAML hash against execution log entries.

```bash
# Verify and sign
audit-verifier --yaml test.yml --log execution.log.json --output verification.json

# With existing key
audit-verifier --yaml test.yml --log execution.log.json --private-key key.pem --output verification.json
```

**Options:**
- `--yaml PATH`: Path to test case YAML file (required)
- `--log PATH`: Path to execution log JSON file (required)
- `-o, --output PATH`: Output file for signed verification results
- `-k, --private-key PATH`: Path to P-521 private key PEM file
- `--save-key PATH`: Path to save generated private key
- `--key-id ID`: Key identifier (default: "audit-verifier")
- `--log-level LEVEL`: Set log level
- `-v, --verbose`: Enable verbose output

## Integration with Test Executor

The test executor automatically logs all operations to an audit log:

```bash
# Set audit log file via environment variable
export AUDIT_LOG_FILE=my_audit.log.json

# Or via command line
test-executor --audit-log my_audit.log.json generate test.yml --output test.sh

# Disable audit logging
test-executor --no-audit generate test.yml --output test.sh
```

**Environment Variables:**
- `AUDIT_LOG_FILE`: Path to audit log file (default: `audit.log.json`)
- `AUDIT_LOG_ENABLED`: Enable/disable audit logging (default: `true`)

## Workflow Example

### 1. Run Operations with Audit Logging

```bash
# Enable audit logging
export AUDIT_LOG_FILE=operations_audit.log.json

# Run several operations
test-executor generate test1.yml --output test1.sh
test-executor execute test2.yml
test-executor hydrate test3.yml --export-file vars.env --output hydrated.yml

# Check the audit log
cat operations_audit.log.json
```

### 2. Sign the Audit Log

```bash
# Sign with a new key
sign-audit-log \
  --log operations_audit.log.json \
  --output signed_operations.json \
  --save-key audit_key.pem \
  --key-id "team-signing-key"
```

Output:
```
[INFO] Loading audit log from: operations_audit.log.json
[INFO] Loaded audit log with 3 entries
[INFO] Generating new P-521 private key...
[INFO] Saving private key to: audit_key.pem
[INFO] Signing audit log...
[INFO] Saving signed audit log to: signed_operations.json
[INFO] ✓ Audit log signed successfully
[INFO]   Log hash: a3f2b8c9d4e5...
[INFO]   Signature: 0123456789ab...
[INFO]   Signed at: 2024-01-15T10:30:00Z
```

### 3. Verify the Signed Audit Log

```bash
# Basic verification
verify-audit-log signed_operations.json

# Detailed verification
verify-audit-log signed_operations.json --detailed --output verification_report.json
```

Output:
```
=== Audit Log Verification Report ===

Verification Status: ✓ VALID
Log Hash Verified:   ✓
Signature Verified:  ✓

Key ID:        team-signing-key
Signed At:     2024-01-15T10:30:00Z
Verified At:   2024-01-15T14:45:00Z
Total Entries: 3

=== Audit Log Details ===

Version:      1.0.0
Created At:   2024-01-15T10:00:00Z
Last Updated: 2024-01-15T10:15:00Z

Entries:
  1. [Success] GenerateScript
     Timestamp: 2024-01-15T10:05:00Z
     User: john_doe
     Host: build-server-01
     Input files: 1
     Output files: 1
     Duration: 125ms

  2. [Success] ExecuteScript
     Timestamp: 2024-01-15T10:10:00Z
     User: john_doe
     Host: build-server-01
     Input files: 1
     Duration: 3450ms

  3. [Success] HydrateYaml
     Timestamp: 2024-01-15T10:15:00Z
     User: john_doe
     Host: build-server-01
     Input files: 2
     Output files: 1
     Duration: 89ms
```

### 4. Detect Tampering

If someone modifies the audit log after signing:

```bash
verify-audit-log tampered_operations.json
```

Output:
```
=== Audit Log Verification Report ===

Verification Status: ✗ INVALID
Log Hash Verified:   ✗
Signature Verified:  ✗

Errors:
  - Log hash mismatch: computed 'b8f3c9...', stored 'a3f2b8...'
  - Signature verification failed
```

## Programmatic Usage

### Creating Audit Log Entries

```rust
use audit_verifier::audit_log::{AuditLog, AuditLogEntry, OperationType, OperationStatus};
use std::path::PathBuf;

let mut log = AuditLog::new();

let entry = AuditLogEntry::builder(OperationType::GenerateScript)
    .with_input_file(PathBuf::from("test.yml"))
    .with_output_file(PathBuf::from("test.sh"))
    .with_metadata("test_id", "TC-001")
    .with_metadata("version", "1.0.0")
    .with_status(OperationStatus::Success)
    .build();

log.entries.push(entry);
log.save_to_file("audit.log.json")?;
```

### Signing and Verifying

```rust
use audit_verifier::audit_log::AuditLog;
use audit_verifier::audit_signer::{SignedAuditLog, SignatureVerificationReport};
use audit_verifier::signing;

// Load and sign
let log = AuditLog::load_from_file("audit.log.json")?;
let private_key = signing::generate_private_key();
let signed_log = SignedAuditLog::sign_log(log, &private_key, "my-key".to_string())?;
signed_log.save_to_file("signed_audit.json")?;

// Verify
let loaded = SignedAuditLog::load_from_file("signed_audit.json")?;
let report = SignatureVerificationReport::verify(&loaded);

if report.is_valid {
    println!("✓ Audit log is valid and untampered");
} else {
    eprintln!("✗ Audit log verification failed");
    for error in &report.errors {
        eprintln!("  - {}", error);
    }
}
```

### Integration with Test Execution

```rust
use testcase_execution::AuditLogger;
use audit_verifier::audit_log::OperationStatus;
use std::path::PathBuf;

let logger = AuditLogger::with_file("audit.log.json");

// Log a script generation
logger.log_generate_script(
    &PathBuf::from("test.yml"),
    Some(&PathBuf::from("test.sh")),
    OperationStatus::Success,
    None,
)?;

// Log script execution
logger.log_execute_script(
    &PathBuf::from("test.yml"),
    OperationStatus::Success,
    None,
)?;

// Save the audit log
logger.save()?;
```

## Security Considerations

1. **Private Key Protection**: Keep private keys secure. Never commit them to version control.
2. **Key Rotation**: Regularly rotate signing keys and re-sign audit logs.
3. **Signature Verification**: Always verify signatures before trusting audit log contents.
4. **Hash Verification**: File hashes provide integrity guarantees for input/output files.
5. **Timestamp Trust**: Timestamps are self-reported and should not be solely relied upon for forensic purposes.

## File Format

### Audit Log JSON

```json
{
  "version": "1.0.0",
  "entries": [
    {
      "timestamp": "2024-01-15T10:05:00Z",
      "operation": "generate_script",
      "user": "john_doe",
      "hostname": "build-server-01",
      "working_directory": "/home/john/project",
      "input_files": ["/home/john/project/test.yml"],
      "output_files": ["/home/john/project/test.sh"],
      "input_file_hashes": [
        ["/home/john/project/test.yml", "a3f2b8c9d4e5..."]
      ],
      "output_file_hashes": [
        ["/home/john/project/test.sh", "b8f3c9d4e5a2..."]
      ],
      "command_args": ["test-executor", "generate", "test.yml"],
      "status": "success",
      "error_message": null,
      "duration_ms": 125,
      "metadata": {
        "test_id": "TC-001"
      }
    }
  ],
  "created_at": "2024-01-15T10:00:00Z",
  "last_updated": "2024-01-15T10:05:00Z"
}
```

### Signed Audit Log JSON

```json
{
  "audit_log": { /* AuditLog structure */ },
  "log_hash": "a3f2b8c9d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1",
  "signature": "0123456789abcdef...",
  "public_key": "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----",
  "key_id": "production-key",
  "signed_at": "2024-01-15T10:30:00Z"
}
```

## Testing

Run tests:

```bash
cargo test -p audit-verifier
```

Run with logging:

```bash
RUST_LOG=debug cargo test -p audit-verifier -- --nocapture
```

## License

Same as parent project.
