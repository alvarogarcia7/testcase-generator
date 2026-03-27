# Audit Verifier

A tool for verifying test case YAML hashes against execution log entries, with cryptographic signature support using NIST P-521 (secp521r1) elliptic curve.

## Features

- Verify that execution logs contain the correct SHA-256 hash of source YAML files
- Generate or load P-521 private keys for signing
- Sign audit verification results with ECDSA signatures
- Output signed audit trails in JSON format with embedded public keys
- Verify cryptographic signatures on audit outputs

## Binaries

### `audit-verifier`

The main binary that performs hash verification and creates signed audit outputs.

#### Usage

```bash
# Basic usage with generated key
audit-verifier --yaml test.yml --log execution.json --output signed-audit.json

# Use existing private key
audit-verifier --yaml test.yml --log execution.json \
  --private-key key.pem --output signed-audit.json

# Generate and save a new key
audit-verifier --yaml test.yml --log execution.json \
  --save-key my-key.pem --output signed-audit.json

# With custom key identifier
audit-verifier --yaml test.yml --log execution.json \
  --key-id "production-auditor-2024" --output signed-audit.json
```

#### Options

- `-y, --yaml <PATH>`: Path to test case YAML file (required)
- `-l, --log <PATH>`: Path to execution log JSON file (required)
- `-k, --private-key <PATH>`: Path to P-521 private key PEM file (optional, generates new key if not provided)
- `--save-key <PATH>`: Path to save generated private key (only used when no key is provided)
- `--key-id <ID>`: Key identifier to include in signature output (default: "audit-verifier")
- `-o, --output <PATH>`: Output file for signed audit verification results (JSON format)

### `verify-audit-signature`

A standalone binary for external auditors to verify cryptographic signatures on audit outputs.

#### Usage

```bash
# Basic verification
verify-audit-signature --input signed-audit.json

# Verbose output with details
verify-audit-signature --input signed-audit.json --verbose
```

#### Options

- `-i, --input <PATH>`: Path to signed audit verification JSON file (required)
- `-v, --verbose`: Display detailed information about the signature

### `verify-audit-log`

A standalone binary to verify an audit log signature given separate keypair, payload, and signature files.

#### Usage

```bash
# Basic verification
verify-audit-log --keypair key.pem --payload audit.json --signature audit.sig

# Verbose output with details
verify-audit-log --keypair key.pem --payload audit.json --signature audit.sig --verbose
```

#### Options

- `-k, --keypair <PATH>`: Path to P-521 private key PEM file (used to derive public key) (required)
- `-p, --payload <PATH>`: Path to payload file to verify (required)
- `-s, --signature <PATH>`: Path to signature file (hex-encoded) (required)
- `-v, --verbose`: Display detailed information

## Cryptographic Details

### Algorithm

- **Elliptic Curve**: NIST P-521 (secp521r1)
- **Signature Scheme**: ECDSA (Elliptic Curve Digital Signature Algorithm)
- **Hash Function**: SHA-256
- **Key Format**: PKCS#8 PEM encoding

### What Gets Signed

The tool creates a SHA-256 hash of the canonical JSON representation of the verification result, which includes:
- Computed YAML hash
- Total number of log entries
- Number of hash mismatches
- Number of missing hash fields
- Overall verification pass/fail status

The ECDSA signature is computed over this hash.

### Output Format

The signed audit output is a JSON file with the following structure:

```json
{
  "verification_result": {
    "computed_hash": "abc123...",
    "total_entries": 10,
    "hash_mismatches": 0,
    "missing_hash_fields": 0,
    "verification_passed": true
  },
  "execution_log_sha256": "def456...",
  "signature": "hexadecimal-encoded-signature",
  "public_key": "-----BEGIN PUBLIC KEY-----\n...",
  "key_id": "audit-verifier",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Key Management

### Generating a New Key

```bash
# Generate and save a key during audit verification
audit-verifier --yaml test.yml --log execution.json \
  --save-key my-private-key.pem --output audit.json
```

### Key Security

- **Private keys** should be stored securely and protected with appropriate file permissions
- **Public keys** are embedded in the signed output and can be freely distributed
- For production use, consider using a hardware security module (HSM) or key management service

### Key Persistence

If you want to use the same key across multiple audit runs:

1. Generate and save a key once:
   ```bash
   audit-verifier --yaml test.yml --log log.json --save-key authority.pem -o audit.json
   ```

2. Reuse the key for subsequent audits:
   ```bash
   audit-verifier --yaml test2.yml --log log2.json --private-key authority.pem -o audit2.json
   ```

## Verification Workflow

### For Audit Authorities

1. Run the audit verifier with your private key:
   ```bash
   audit-verifier --yaml test.yml --log execution.json \
     --private-key authority-key.pem \
     --key-id "CompanyName-Auditor-2024" \
     --output signed-audit.json
   ```

2. Distribute the `signed-audit.json` to stakeholders

### For External Auditors

1. Receive the signed audit JSON file
2. Verify the signature:
   ```bash
   verify-audit-signature --input signed-audit.json --verbose
   ```

3. Check:
   - Signature is valid ✓
   - Key identifier matches expected authority
   - Timestamp is reasonable
   - Verification result shows no hash mismatches

## Security Considerations

- **Signature Validity**: A valid signature only proves that the verification result was signed by the holder of the private key. It does not prove the original YAML or logs were authentic.
- **Key Compromise**: If a private key is compromised, all signatures created with that key should be considered untrusted.
- **Timestamp Trust**: The timestamp is generated by the signing system and should not be considered cryptographically secure.
- **Chain of Trust**: Organizations should establish a documented process for key generation, storage, and distribution of public keys to stakeholders.

## Examples

### Complete Audit Workflow

```bash
# 1. Run tests and generate execution log
# (This happens in your test execution system)

# 2. Verify hashes and create signed audit
audit-verifier \
  --yaml testcases/example.yml \
  --log logs/execution-2024-01-15.json \
  --private-key keys/auditor-2024.pem \
  --key-id "example-org-auditor" \
  --output audits/audit-2024-01-15.json

# 3. External auditor verifies signature
verify-audit-signature \
  --input audits/audit-2024-01-15.json \
  --verbose
```

## Library Usage

The audit-verifier crate can also be used as a library:

```rust
use audit_verifier::signing;
use audit_verifier::verify_signature;

// Generate a key
let private_key = signing::generate_private_key();

// Sign a message
let signature = signing::sign_message(&private_key, message_hash);

// Verify a signed audit file
let is_valid = verify_signature::verify_signed_audit(path)?;
```
