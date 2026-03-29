# Sample Test Files

This directory contains sample test files for manual testing of the audit-verifier.

## Files

- `sample_test_case.yaml`: A simple test case YAML file
- `sample_execution_log.json`: A corresponding execution log with matching SHA-256 hashes

## Usage

### Scenario 1: Generate Key On The Fly

Verify the sample payload and generate a new key:

```bash
# From project root
cargo run --bin audit-verifier -- \
  --yaml crates/audit-verifier/tests/sample/sample_test_case.yaml \
  --log crates/audit-verifier/tests/sample/sample_execution_log.json \
  --save-key /tmp/audit-key.pem \
  --key-id "sample-key-generated" \
  --output /tmp/sample-signed.json
```

Verify the signature:

```bash
cargo run --bin verify-audit-signature -- \
  --input /tmp/sample-signed.json \
  --verbose
```

### Scenario 2: Use Existing Key

First, generate a key (if you don't have one):

```bash
# Generate a key using the first scenario above
# Or create one separately
```

Then use the existing key:

```bash
cargo run --bin audit-verifier -- \
  --yaml crates/audit-verifier/tests/sample/sample_test_case.yaml \
  --log crates/audit-verifier/tests/sample/sample_execution_log.json \
  --private-key /tmp/audit-key.pem \
  --key-id "sample-key-existing" \
  --output /tmp/sample-signed-existing.json
```

Verify the signature:

```bash
cargo run --bin verify-audit-signature -- \
  --input /tmp/sample-signed-existing.json \
  --verbose
```

## Hash Information

The execution log contains SHA-256 hashes that match the test case YAML file:

```
Hash: ff72a80406dd1afcecf3a841a4c4663fb4728a2edf5a3a521651650b216ca013
```

You can verify this by running:

```bash
shasum -a 256 crates/audit-verifier/tests/sample/sample_test_case.yaml
```

## Testing Hash Mismatches

To test what happens with hash mismatches, you can:

1. Modify the YAML file
2. Run audit-verifier with the modified YAML and original log
3. The tool will report hash mismatches and exit with code 1

Example:

```bash
# Create a modified YAML
cp crates/audit-verifier/tests/sample/sample_test_case.yaml /tmp/modified.yaml
echo "# Modified" >> /tmp/modified.yaml

# Try to verify (will fail)
cargo run --bin audit-verifier -- \
  --yaml /tmp/modified.yaml \
  --log crates/audit-verifier/tests/sample/sample_execution_log.json \
  --save-key /tmp/key.pem \
  --output /tmp/signed-fail.json

# The tool exits with code 1 but still creates the signed output
# showing verification_passed: false
```
