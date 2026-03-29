# Testing Guide for audit-verifier

This document provides a quick reference for testing the audit-verifier crate.

## Quick Start

```bash
# Using Make (recommended)
make test-audit-verifier

# Or directly with the test runner
./crates/audit-verifier/tests/run_all_tests.sh

# Or build and run manually
cargo build -p audit-verifier
./crates/audit-verifier/tests/run_all_tests.sh
```

## Test Suites

### 1. Simple Workflow Test (Recommended Starting Point)

A minimal test demonstrating the basic workflow:

```bash
# Using Make
make test-audit-verifier-simple

# Or directly
./crates/audit-verifier/tests/integration/test_simple_workflow.sh
```

**Duration**: ~2 seconds  
**Purpose**: Quick smoke test and workflow demonstration

### 2. Comprehensive E2E Test

Full end-to-end testing with positive and negative test cases:

```bash
# Using Make
make test-audit-verifier-e2e

# Or directly
./crates/audit-verifier/tests/integration/test_audit_verifier_e2e.sh
```

**Duration**: ~5-10 seconds  
**Purpose**: Comprehensive validation of all features

### 3. Key Management Scenarios Test

Focused testing of key generation and loading scenarios:

```bash
# Using Make
make test-audit-verifier-keys

# Or directly
./crates/audit-verifier/tests/integration/test_audit_key_scenarios.sh
```

**Duration**: ~5-10 seconds  
**Purpose**: Validate both key management approaches

## Manual Testing

Sample files are provided in `tests/sample/` for manual testing:

```bash
# Scenario 1: Generate key on the fly
cargo run --bin audit-verifier -- \
  --yaml crates/audit-verifier/tests/sample/sample_test_case.yaml \
  --log crates/audit-verifier/tests/sample/sample_execution_log.json \
  --save-key /tmp/audit-key.pem \
  --key-id "test-key" \
  --output /tmp/signed.json

# Verify the signature
cargo run --bin verify-audit-signature -- \
  --input /tmp/signed.json \
  --verbose
```

## Debugging Failed Tests

Keep temporary files for inspection:

```bash
./crates/audit-verifier/tests/integration/test_audit_verifier_e2e.sh --no-remove
```

The test will print the temporary directory path. You can then inspect:
- Generated test case YAML files
- Execution log JSON files
- Private key PEM files
- Signed output JSON files

## Test Coverage

The test suite validates:

| Feature | Simple | E2E | Key Scenarios |
|---------|--------|-----|---------------|
| Sample payload generation | ✓ | ✓ | ✓ |
| Key generation on the fly | ✓ | ✓ | ✓ |
| Load existing key | - | ✓ | ✓ |
| Key reuse | - | - | ✓ |
| Signature verification | ✓ | ✓ | ✓ |
| Hash mismatch detection | - | ✓ | - |
| Missing hash field detection | - | ✓ | - |
| Tampered signature detection | - | ✓ | - |
| Tampered data detection | - | ✓ | - |
| Error handling | - | ✓ | - |

## Make Targets

The following Make targets are available from the project root:

- `make test-audit-verifier` - Run all audit-verifier tests
- `make test-audit-verifier-simple` - Run simple workflow test only
- `make test-audit-verifier-e2e` - Run comprehensive E2E test only
- `make test-audit-verifier-keys` - Run key management scenarios test only
- `make build-audit-verifier` - Build audit-verifier binaries
- `make audit-verify` - Run a sample verification (demo)

## CI/CD Integration

For CI/CD pipelines, use the Make target or test runner:

```yaml
# Example GitLab CI
test:audit-verifier:
  script:
    - make test-audit-verifier

# Or without Make
test:audit-verifier-direct:
  script:
    - cargo build -p audit-verifier
    - ./crates/audit-verifier/tests/run_all_tests.sh
```

## Test Requirements

- Bash 3.2+
- Standard Unix utilities (shasum, mktemp, grep, awk)
- Optional: `jq` for enhanced JSON validation

## Documentation

- [Test README](tests/README.md) - Detailed test documentation
- [Sample README](tests/sample/README.md) - Manual testing guide
- [Main README](README.md) - Crate documentation
