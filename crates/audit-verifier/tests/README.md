# audit-verifier Tests

This directory contains integration tests for the `audit-verifier` crate.

## Test Structure

```
tests/
├── integration/          # Shell-based integration tests
│   ├── test_simple_workflow.sh         # Simple workflow demonstration
│   ├── test_audit_verifier_e2e.sh      # Comprehensive E2E tests
│   └── test_audit_key_scenarios.sh     # Key management scenarios
├── sample/              # Sample test files (if needed)
├── run_all_tests.sh    # Run all integration tests
└── README.md           # This file
```

## Running the Tests

### Prerequisites

Build the audit-verifier binaries:

```bash
cargo build -p audit-verifier
```

Or build in release mode:

```bash
cargo build -p audit-verifier --release
```

### Run All Integration Tests

From the project root:

```bash
# Run all tests with the test runner
./crates/audit-verifier/tests/run_all_tests.sh

# Or run individual tests
./crates/audit-verifier/tests/integration/test_simple_workflow.sh
./crates/audit-verifier/tests/integration/test_audit_verifier_e2e.sh
./crates/audit-verifier/tests/integration/test_audit_key_scenarios.sh
```

### Keep Temporary Files for Debugging

Use the `--no-remove` flag to keep temporary test files:

```bash
./crates/audit-verifier/tests/integration/test_audit_verifier_e2e.sh --no-remove
./crates/audit-verifier/tests/integration/test_audit_key_scenarios.sh --no-remove
```

This will print the temporary directory path, allowing you to inspect test artifacts after the test completes.

## Test Coverage

### test_simple_workflow.sh

A minimal demonstration test that shows the basic workflow:

1. Create a simple test case YAML
2. Create an execution log with matching hash
3. Run audit-verifier to verify and sign (generates key on the fly)
4. Verify the signature

This test is ideal for:
- Quick validation that the binaries work
- Understanding the basic workflow
- CI/CD smoke testing

### test_audit_verifier_e2e.sh

Comprehensive end-to-end test covering:

1. **Sample Payload Generation**: Creates test case YAML and execution log
2. **Verification with Generated Key**: Tests key generation on the fly
3. **Signature Verification**: Validates cryptographic signatures
4. **Verification with Existing Key**: Tests loading and using pre-existing keys
5. **Negative Tests**: 
   - Hash mismatches
   - Missing hash fields
   - Tampered signatures
   - Tampered data
6. **Error Handling**:
   - Missing files
   - Invalid JSON
   - Nonexistent paths

### test_audit_key_scenarios.sh

Focused test on two key management scenarios:

#### Scenario 1: Generate Key On The Fly
- Generate sample payload (YAML + execution log)
- Run audit-verifier without existing key (generates new key)
- Verify the generated key is saved
- Sign the verification result
- Verify the signature

#### Scenario 2: Load Existing Key
- Pre-generate a key
- Generate sample payload
- Run audit-verifier with existing key
- Sign the verification result
- Verify the signature
- Test key reuse (use same key multiple times)

## Test Output

Both tests use a structured output format:

- **Section headers**: Major test phases
- **Pass indicators**: ✓ for successful checks
- **Fail indicators**: ✗ for failed checks
- **Info messages**: Additional context
- **Summary**: Final test results

Example output:
```
=========================================
audit-verifier End-to-End Integration Test
=========================================

====== Checking Prerequisites ======
✓ audit-verifier binary found
✓ verify-audit-signature binary found

====== Test 1: Generate Sample Test Case YAML ======
✓ Created test case YAML: /tmp/tmp.xxx/test_case.yaml
...
```

## Dependencies

The tests require:

- Bash 3.2+ compatible shell
- `jq` (optional, for enhanced JSON validation)
- `shasum` or `sha256sum` (for hash computation)
- Standard Unix utilities: `mktemp`, `grep`, `awk`

## Notes

- Tests create temporary directories for isolation
- All test artifacts are cleaned up by default (unless `--no-remove` is used)
- Tests verify both success and failure cases
- Signature verification uses P-521 ECDSA cryptography
