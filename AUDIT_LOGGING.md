# Audit Logging System

## Overview

The test harness includes a comprehensive audit logging system that tracks all operations (script generation, execution, validation, etc.) with digital signature support for tamper detection and verification.

## Features

### 1. Complete Operation Tracking

Every operation is logged with:
- **Timestamp**: UTC timestamp when the operation occurred
- **Operation Type**: Type of operation (generate, execute, verify, etc.)
- **User Information**: Username and hostname
- **Working Directory**: Directory where operation was executed
- **Input/Output Files**: All files read and written
- **File Hashes**: SHA-256 hashes of all input and output files
- **Command Arguments**: Complete command-line arguments
- **Status**: Success, Failed, Warning, or Started
- **Duration**: Time taken to complete the operation
- **Error Messages**: Detailed error information if operation failed
- **Custom Metadata**: Additional operation-specific information

### 2. Digital Signatures

Audit logs can be digitally signed using:
- **P-521 ECDSA**: Industry-standard elliptic curve digital signatures
- **SHA-256 Hashing**: Cryptographic hashing for integrity verification
- **Key Management**: Support for key generation, storage, and loading
- **Tamper Detection**: Any modification invalidates the signature

### 3. Verification System

Comprehensive verification includes:
- **Signature Verification**: Validates digital signature authenticity
- **Hash Verification**: Ensures log content hasn't been modified
- **File Integrity**: Verifies input/output file hashes
- **Detailed Reports**: JSON reports with verification results

## Architecture

### Components

```
audit-verifier/
├── src/
│   ├── audit_log.rs          # Audit log data structures
│   ├── audit_signer.rs       # Digital signing and verification
│   ├── signing.rs            # P-521 ECDSA primitives
│   ├── verify_signature.rs   # Signature verification utilities
│   ├── lib.rs                # Public API
│   ├── main.rs               # CLI: audit-verifier
│   └── bin/
│       ├── sign_audit_log.rs      # CLI: sign-audit-log
│       └── verify_audit_log.rs    # CLI: verify-audit-log

testcase-execution/
└── src/
    └── audit.rs              # Integration with test executor

test-executor/
└── src/
    └── main.rs               # Automatic audit logging
```

### Data Flow

```
Operation → AuditLogger → AuditLog → File
                                    ↓
                            sign-audit-log
                                    ↓
                            SignedAuditLog → File
                                    ↓
                           verify-audit-log
                                    ↓
                          VerificationReport
```

## Usage

### Automatic Audit Logging

The test executor automatically logs all operations:

```bash
# Enable audit logging (default)
export AUDIT_LOG_FILE=my_operations.log.json
test-executor generate test.yml --output test.sh

# Disable audit logging
test-executor --no-audit generate test.yml --output test.sh

# Use custom audit log path
test-executor --audit-log custom.log.json execute test.yml
```

### Manual Audit Logging

For custom integrations:

```rust
use testcase_execution::AuditLogger;
use audit_verifier::audit_log::OperationStatus;
use std::path::PathBuf;

// Create logger with file
let logger = AuditLogger::with_file("audit.log.json");

// Log an operation
logger.log_generate_script(
    &PathBuf::from("test.yml"),
    Some(&PathBuf::from("test.sh")),
    OperationStatus::Success,
    None,
)?;

// Save the log
logger.save()?;
```

### Signing Audit Logs

```bash
# Generate new key and sign
sign-audit-log \
  --log operations.log.json \
  --output signed_operations.json \
  --save-key private.pem \
  --key-id "production-key"

# Sign with existing key
sign-audit-log \
  --log operations.log.json \
  --output signed_operations.json \
  --private-key existing_key.pem \
  --key-id "production-key"
```

### Verifying Signed Audit Logs

```bash
# Basic verification
verify-audit-log signed_operations.json

# Detailed verification with report
verify-audit-log signed_operations.json \
  --detailed \
  --output verification_report.json
```

## Complete Workflow Example

### Step 1: Run Operations with Audit Logging

```bash
# Configure audit logging
export AUDIT_LOG_FILE=project_audit.log.json

# Run operations
test-executor generate test1.yml --output test1.sh
test-executor execute test2.yml
test-executor hydrate test3.yml --export-file vars.env --output hydrated.yml
test-executor list testcases/

# Review the audit log
cat project_audit.log.json | jq '.'
```

Example audit log:
```json
{
  "version": "1.0.0",
  "entries": [
    {
      "timestamp": "2024-01-15T10:05:23.456Z",
      "operation": "generate_script",
      "user": "developer",
      "hostname": "build-server",
      "working_directory": "/home/developer/project",
      "input_files": ["/home/developer/project/test1.yml"],
      "output_files": ["/home/developer/project/test1.sh"],
      "input_file_hashes": [
        ["/home/developer/project/test1.yml", "a3f2b8c9..."]
      ],
      "output_file_hashes": [
        ["/home/developer/project/test1.sh", "b8f3c9d4..."]
      ],
      "command_args": ["test-executor", "generate", "test1.yml"],
      "status": "success",
      "error_message": null,
      "duration_ms": 145,
      "metadata": {}
    }
  ],
  "created_at": "2024-01-15T10:00:00Z",
  "last_updated": "2024-01-15T10:15:00Z"
}
```

### Step 2: Sign the Audit Log

```bash
# Sign with new key
sign-audit-log \
  --log project_audit.log.json \
  --output signed_project_audit.json \
  --save-key project_signing_key.pem \
  --key-id "project-2024-q1" \
  --verbose

# Output:
# [INFO] Loading audit log from: project_audit.log.json
# [INFO] Loaded audit log with 4 entries
# [INFO] Generating new P-521 private key...
# [INFO] Saving private key to: project_signing_key.pem
# [INFO] Signing audit log...
# [INFO] Saving signed audit log to: signed_project_audit.json
# [INFO] ✓ Audit log signed successfully
# [INFO]   Log hash: a3f2b8c9d4e5f6a7...
# [INFO]   Signature: 0123456789abcdef...
# [INFO]   Signed at: 2024-01-15T10:30:00Z
```

### Step 3: Archive and Distribute

```bash
# Archive the signed audit log securely
tar -czf project_audit_archive.tar.gz \
  signed_project_audit.json \
  project_signing_key.pem

# For distribution, only share the signed log (not the private key)
cp signed_project_audit.json /shared/audits/
```

### Step 4: Verify at Any Time

```bash
# Basic verification
verify-audit-log signed_project_audit.json

# Output:
# === Audit Log Verification Report ===
#
# Verification Status: ✓ VALID
# Log Hash Verified:   ✓
# Signature Verified:  ✓
#
# Key ID:        project-2024-q1
# Signed At:     2024-01-15T10:30:00Z
# Verified At:   2024-01-15T14:45:00Z
# Total Entries: 4

# Detailed verification
verify-audit-log signed_project_audit.json --detailed

# Generate verification report
verify-audit-log signed_project_audit.json \
  --output verification_report.json

cat verification_report.json
# {
#   "is_valid": true,
#   "log_hash_verified": true,
#   "signature_verified": true,
#   "key_id": "project-2024-q1",
#   "signed_at": "2024-01-15T10:30:00Z",
#   "verified_at": "2024-01-15T14:45:00Z",
#   "total_entries": 4,
#   "errors": []
# }
```

### Step 5: Tamper Detection

If someone modifies the audit log:

```bash
# Attempt to modify the signed audit log
cat signed_project_audit.json | \
  jq '.audit_log.entries[0].status = "failed"' > tampered.json

# Verification fails
verify-audit-log tampered.json

# Output:
# === Audit Log Verification Report ===
#
# Verification Status: ✗ INVALID
# Log Hash Verified:   ✗
# Signature Verified:  ✗
#
# Errors:
#   - Log hash mismatch: computed 'b8f3c9...', stored 'a3f2b8...'
#   - Signature verification failed
```

## Operation Types

| Type | Description | Logged By |
|------|-------------|-----------|
| `GenerateScript` | Script generation from YAML | test-executor |
| `ExecuteScript` | Script execution | test-executor |
| `VerifyTest` | Test verification | test-executor, verifier |
| `HydrateYaml` | YAML variable hydration | test-executor |
| `GenerateExport` | Export file generation | test-executor |
| `ValidateExport` | Export file validation | test-executor |
| `ListTestCases` | Test case listing | test-executor |
| `ResolveDependencies` | Dependency resolution | test-executor |
| `ValidateYaml` | YAML validation | validate-yaml |
| `LoadTestCase` | Test case loading | Various |
| `SaveTestCase` | Test case saving | Various |
| `Other(String)` | Custom operations | Custom tools |

## Security Best Practices

### 1. Private Key Management

**DO:**
- ✓ Store private keys in secure key management systems
- ✓ Use file permissions (chmod 600) to protect key files
- ✓ Rotate keys periodically (e.g., quarterly)
- ✓ Use different keys for different environments (dev, staging, prod)
- ✓ Back up keys securely

**DON'T:**
- ✗ Commit private keys to version control
- ✗ Share private keys via email or chat
- ✗ Use the same key across multiple projects
- ✗ Store keys in world-readable locations

### 2. Audit Log Protection

- Store signed audit logs in tamper-evident storage
- Use append-only logging systems when possible
- Regularly verify signatures on archived logs
- Maintain multiple copies in different locations

### 3. Verification

- Always verify signatures before trusting audit data
- Automate verification in CI/CD pipelines
- Alert on verification failures
- Maintain verification reports for compliance

### 4. Key Rotation

```bash
# Generate new key
sign-audit-log \
  --log operations.log.json \
  --output signed_with_new_key.json \
  --save-key new_key.pem \
  --key-id "project-2024-q2"

# Archive old key and signed logs
tar -czf archive_q1.tar.gz \
  signed_with_old_key.json \
  old_key.pem

# Use new key going forward
mv new_key.pem current_key.pem
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `AUDIT_LOG_FILE` | `audit.log.json` | Path to audit log file |
| `AUDIT_LOG_ENABLED` | `true` | Enable/disable audit logging |
| `RUST_LOG` | `warn` | Logging level |

## CLI Tools

### audit-verifier

Verify test case YAML hash against execution logs.

```bash
audit-verifier \
  --yaml test.yml \
  --log execution.log.json \
  --output verification.json \
  --private-key key.pem \
  --key-id "production"
```

### sign-audit-log

Sign an audit log with a private key.

```bash
sign-audit-log \
  --log operations.log.json \
  --output signed.json \
  --private-key key.pem \
  --key-id "production"
```

### verify-audit-log

Verify a signed audit log.

```bash
verify-audit-log signed.json --detailed --output report.json
```

## Integration Examples

### CI/CD Pipeline

```yaml
# .gitlab-ci.yml
audit_and_sign:
  stage: test
  script:
    # Run tests with audit logging
    - export AUDIT_LOG_FILE=ci_audit_${CI_PIPELINE_ID}.log.json
    - test-executor generate test.yml --output test.sh
    - test-executor execute test.yml
    
    # Sign the audit log
    - sign-audit-log \
        --log ci_audit_${CI_PIPELINE_ID}.log.json \
        --output signed_ci_audit_${CI_PIPELINE_ID}.json \
        --private-key ${CI_SIGNING_KEY_PATH} \
        --key-id "ci-${CI_COMMIT_REF_NAME}"
    
    # Upload signed audit log
    - aws s3 cp signed_ci_audit_${CI_PIPELINE_ID}.json \
        s3://audit-logs/ci/${CI_PIPELINE_ID}/
  
  artifacts:
    paths:
      - signed_ci_audit_${CI_PIPELINE_ID}.json

verify_audit:
  stage: verify
  script:
    # Download and verify
    - verify-audit-log signed_ci_audit_${CI_PIPELINE_ID}.json \
        --output verification_report.json
  
  artifacts:
    paths:
      - verification_report.json
```

### Programmatic Usage

```rust
use audit_verifier::audit_log::{AuditLog, AuditLogEntry, OperationType, OperationStatus};
use audit_verifier::audit_signer::SignedAuditLog;
use audit_verifier::signing;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Create audit log
    let mut log = AuditLog::new();
    
    // Add entry
    let entry = AuditLogEntry::builder(OperationType::GenerateScript)
        .with_input_file(PathBuf::from("test.yml"))
        .with_output_file(PathBuf::from("test.sh"))
        .with_metadata("test_id", "TC-001")
        .with_status(OperationStatus::Success)
        .build();
    
    log.entries.push(entry);
    
    // Sign the log
    let key = signing::generate_private_key();
    let signed = SignedAuditLog::sign_log(log, &key, "my-key".to_string())?;
    signed.save_to_file("signed_audit.json")?;
    
    // Verify later
    let loaded = SignedAuditLog::load_from_file("signed_audit.json")?;
    assert!(loaded.verify_signature()?);
    
    Ok(())
}
```

## Troubleshooting

### Issue: Audit log file grows too large

**Solution:** Implement log rotation:

```bash
# Archive and start fresh
mv audit.log.json audit.$(date +%Y%m%d).log.json
sign-audit-log --log audit.$(date +%Y%m%d).log.json \
  --output signed_audit.$(date +%Y%m%d).json \
  --private-key key.pem
```

### Issue: Verification fails unexpectedly

**Solution:** Check for:
1. File encoding issues (ensure UTF-8)
2. Line ending differences (use consistent line endings)
3. JSON formatting differences (use canonical JSON)

### Issue: Performance impact from audit logging

**Solution:**
- Disable during development: `--no-audit`
- Use asynchronous logging for high-volume operations
- Batch operations before writing to disk

## Demo

Run the complete audit logging demonstration:

```bash
make demo-audit-logging
```

Or manually:

```bash
chmod +x examples/audit_logging_demo.sh
./examples/audit_logging_demo.sh
```

## Further Reading

- [Audit Verifier README](crates/audit-verifier/README.md) - Detailed crate documentation
- [P-521 ECDSA](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf) - NIST standard
- [Digital Signatures](https://en.wikipedia.org/wiki/Digital_signature) - Concepts and theory

## Support

For issues or questions:
1. Check the README files in `crates/audit-verifier/`
2. Review the example scripts in `examples/`
3. Run the demo: `make demo-audit-logging`
