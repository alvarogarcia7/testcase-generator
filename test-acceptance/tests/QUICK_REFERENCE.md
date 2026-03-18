# Acceptance Suite E2E Test - Quick Reference

## Running the Test

```bash
# Using Make (recommended)
make test-e2e-acceptance

# Direct execution
./test-acceptance/tests/test_acceptance_suite_e2e.sh

# With verbose output
VERBOSE=1 ./test-acceptance/tests/test_acceptance_suite_e2e.sh
```

## Test Coverage

| Test # | Test Name | Coverage |
|--------|-----------|----------|
| 1 | Basic Execution | All 6 stages complete successfully |
| 2 | File Creation | Files created at each stage |
| 3 | Final Report Statistics | Report structure and statistics |
| 4 | Skip Generation Flag | `--skip-generation` |
| 5 | Skip Execution Flag | `--skip-execution` |
| 6 | Skip Verification Flag | `--skip-verification` |
| 7 | Skip Documentation Flag | `--skip-documentation` |
| 8 | Verbose Flag | `--verbose` increases detail |
| 9 | Missing TPDG Handling | Graceful handling when TPDG unavailable |
| 10 | Timeout Handling | Timeout for long-running scripts |
| 11 | Cleanup | Temporary files cleaned up |
| 12 | Multiple Skip Flags | Combining multiple `--skip-*` |

## Test Subset

**Total: 10 test cases**
- 5 Success scenarios
- 3 Failure scenarios  
- 2 Hook scenarios

## File Validation Counts

| Stage | Artifact | Min Expected |
|-------|----------|--------------|
| 2 | Generated Scripts | ≥8 |
| 3 | Execution Logs | ≥5 |
| 4 | Container YAMLs | ≥5 |
| 6 | AsciiDoc Files | ≥5 |
| 6 | Markdown Files | ≥5 |

## Exit Codes

- `0` - All tests passed
- `1` - One or more tests failed

## Prerequisites

**Required** (built automatically):
- validate-yaml
- test-executor
- verifier

**Optional** (gracefully handled if missing):
- test-plan-documentation-generator (TPDG)

## Key Features

✓ Isolated test environments  
✓ Automatic cleanup  
✓ Comprehensive validation  
✓ Detailed logging  
✓ Error handling  
✓ Bash 3.2+ compatible  
✓ BSD/GNU compatible  

## Expected Output

```
=== Acceptance Suite E2E Test ===

[INFO] Test workspace: /tmp/tmp.XXXXXX
[INFO] Acceptance suite: .../run_acceptance_suite.sh

=== Test: Basic Execution - All Stages Complete ===
✓ All 6 stages executed
✓ Basic Execution - All Stages Complete

...

=== Test Summary ===
[INFO] Tests run:    12
[INFO] Tests passed: 12
[INFO] Tests failed: 0

✓ All tests passed!
```

## Troubleshooting

### Missing Binaries
```bash
make build-acceptance-binaries
```

### TPDG Not Found
```bash
# Install globally
cargo install test-plan-documentation-generator

# Or set path
export TEST_PLAN_DOC_GEN=/path/to/tpdg
```

### Tests Timeout
- Default timeout: 300 seconds (5 minutes)
- Check system performance
- Verify no interference from background processes

## Documentation

- Full documentation: `test-acceptance/tests/README.md`
- Implementation details: `test-acceptance/tests/IMPLEMENTATION_SUMMARY.md`
- Project docs: `AGENTS.md` (Commands section)

## CI/CD Integration

```yaml
- name: Run Acceptance Suite E2E Tests
  run: make test-e2e-acceptance
```

## Development

### Adding a New Test

1. Define test function:
```bash
test_new_feature() {
    local test_env="$TEST_WORKSPACE/new_feature"
    create_test_environment "$test_env"
    # Test logic
    return 0
}
```

2. Add to main():
```bash
run_test "New Feature Description" test_new_feature
```

### Modifying Test Subset

Edit arrays in `create_test_environment()`:
```bash
local success_tests=(
    "TC_SUCCESS_CMD_CHAIN_001.yaml"
    # Add more...
)
```

## Quick Checks

```bash
# Verify syntax
bash -n test-acceptance/tests/test_acceptance_suite_e2e.sh

# Check test count
grep -c '^test_' test-acceptance/tests/test_acceptance_suite_e2e.sh

# Dry-run Makefile
make -n test-e2e-acceptance
```
