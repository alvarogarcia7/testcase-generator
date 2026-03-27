# Audit Logging - Quick Start Guide

## 5-Minute Quick Start

### 1. Run Operations with Audit Logging (Enabled by Default)

```bash
# Operations are automatically logged to audit.log.json
test-executor generate test.yml --output test.sh
test-executor execute test.yml
```

### 2. Sign the Audit Log

```bash
sign-audit-log \
  --log audit.log.json \
  --output signed_audit.json \
  --save-key private.pem
```

### 3. Verify the Signed Audit Log

```bash
verify-audit-log signed_audit.json
```

**Output:**
```
Verification Status: ✓ VALID
Log Hash Verified:   ✓
Signature Verified:  ✓
```

That's it! Your operations are now tracked, signed, and verified.

## Common Commands

### Configure Audit Log Location

```bash
# Via environment variable
export AUDIT_LOG_FILE=my_audit.log.json

# Via command line
test-executor --audit-log my_audit.log.json generate test.yml
```

### Disable Audit Logging

```bash
test-executor --no-audit generate test.yml
```

### View Audit Log

```bash
cat audit.log.json | jq '.'
```

### Sign with Existing Key

```bash
sign-audit-log \
  --log audit.log.json \
  --output signed.json \
  --private-key existing_key.pem \
  --key-id "my-team-key"
```

### Detailed Verification

```bash
verify-audit-log signed.json --detailed
```

### Generate Verification Report

```bash
verify-audit-log signed.json --output report.json
cat report.json | jq '.'
```

## What Gets Logged?

Every operation logs:
- ✓ Timestamp and duration
- ✓ User and hostname  
- ✓ Input/output files with SHA-256 hashes
- ✓ Command arguments
- ✓ Success/failure status
- ✓ Error messages (if any)
- ✓ Custom metadata

## Security Tips

### DO:
- ✓ Keep private keys secure (`chmod 600 private.pem`)
- ✓ Verify signatures before trusting audit data
- ✓ Store signed logs in secure, immutable storage
- ✓ Rotate keys regularly

### DON'T:
- ✗ Commit private keys to git (already in .gitignore)
- ✗ Share private keys via email/chat
- ✗ Modify signed audit logs (invalidates signature)

## Example Workflow

```bash
# Day 1: Run operations
export AUDIT_LOG_FILE=january_audit.log.json
test-executor generate test1.yml --output test1.sh
test-executor execute test2.yml
# ... more operations ...

# End of day: Sign the audit log
sign-audit-log \
  --log january_audit.log.json \
  --output signed_january.json \
  --save-key january_key.pem \
  --key-id "team-january-2024"

# Later: Verify anytime
verify-audit-log signed_january.json --detailed
```

## Troubleshooting

### "Verification failed"
- Check if the audit log was modified after signing
- Ensure you're verifying the signed version (not the original)

### "File not found"
- Verify the audit log file path
- Check AUDIT_LOG_FILE environment variable

### "Permission denied"  
- Check file permissions
- Ensure write access to audit log directory

## Next Steps

- Read [AUDIT_LOGGING.md](AUDIT_LOGGING.md) for complete documentation
- Run demo: `make demo-audit-logging`
- View crate docs: [crates/audit-verifier/README.md](crates/audit-verifier/README.md)

## Help

```bash
# CLI help
sign-audit-log --help
verify-audit-log --help
test-executor --help

# Run demo
make demo-audit-logging
```
