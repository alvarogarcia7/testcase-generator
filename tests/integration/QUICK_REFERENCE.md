# Integration Tests - Quick Reference

## One-Line Commands

```bash
# Check if environment is ready
./tests/integration/check_environment.sh

# Run complete workflow test
make test-e2e

# Run all integration tests
make test-e2e-all

# Run with automatic build
./tests/integration/run_all_tests.sh --build

# Run for CI/CD
./tests/integration/ci_test.sh

# Run single test manually
./tests/integration/e2e_complete_workflow.exp ./target/debug/testcase-manager
```

## Test Files

| File | Purpose | Duration |
|------|---------|----------|
| `e2e_complete_workflow.exp` | Full workflow test | ~30s |
| `e2e_basic_workflow.exp` | Quick smoke test | ~10s |

## What Gets Tested

### Complete Workflow
- ✓ Metadata entry and validation
- ✓ General initial conditions
- ✓ Device-specific initial conditions
- ✓ Test sequence creation
- ✓ Step collection (2 steps)
- ✓ Git commits (7 commits)
- ✓ YAML output structure
- ✓ Schema validation

### Basic Workflow
- ✓ Minimal metadata
- ✓ One initial condition
- ✓ One test sequence (no steps)
- ✓ Git commits (2 commits)
- ✓ Basic YAML output
- ✓ Schema validation

## Test Data

### Metadata (Complete Test)
```
Requirement: SGP.22_v3.0
Item: 4
TC: 2
ID: test_e2e_001
Description: E2E integration test case
```

### Steps (Complete Test)
```
Step 1: Initialize connection to eUICC
  Command: ssh init
  Expected: SW=9000, Connection established

Step 2: Download profile package
  Command: download profile.pkg
  Expected: Package downloaded, 100% complete
```

## Prerequisites

```bash
# Ubuntu/Debian
sudo apt-get install expect

# macOS
brew install expect

# Verify
which expect  # Should return: /usr/bin/expect
```

## Common Issues

| Problem | Solution |
|---------|----------|
| "expect: command not found" | Install expect package |
| "Binary not found" | Run `cargo build` |
| "Timeout waiting for prompt" | Check CLI is working, increase timeout |
| "Permission denied" | Run `chmod +x tests/integration/*.{sh,exp}` |
| Leftover test directories | Run `rm -rf test_e2e_* test_basic_*` |

## Debugging

```bash
# Add to top of .exp file for verbose output
exp_internal 1

# Adjust timeout (default is 60 seconds)
set timeout 120  # 2 minutes
set timeout -1   # Infinite (debugging only)

# Run and keep artifacts
# Comment out: exec rm -rf $test_dir
```

## CI/CD Integration

### GitHub Actions
```yaml
- run: sudo apt-get install -y expect
- run: cargo build
- run: ./tests/integration/ci_test.sh
```

### GitLab CI
```yaml
before_script:
  - apt-get install -y expect
script:
  - cargo build
  - ./tests/integration/ci_test.sh
```

## Expected Output

### Success
```
==========================================
ALL TESTS PASSED ✓
==========================================
```

### Failure
```
ERROR: Timeout waiting for [prompt]
[or]
ERROR: Missing required field: [field]
[or]
ERROR: Expected at least [N] commits, found [M]
```

## File Locations

```
tests/integration/
├── *.exp              # Expect test scripts
├── *.sh               # Shell runners
├── *.md               # Documentation
└── [generated]        # Test artifacts (auto-deleted)
    ├── test_e2e_*/
    └── test_basic_*/
```

## Make Targets

```makefile
test-e2e         # Run complete workflow test
test-e2e-all     # Run all integration tests
test-all         # Run unit + integration tests
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | All tests passed |
| 1 | Test failed or error occurred |

## Validation Checks

Each test validates:
1. ✓ Process exits successfully
2. ✓ Output file exists
3. ✓ YAML is valid
4. ✓ Required fields present
5. ✓ Field values correct
6. ✓ Git commits created
7. ✓ Commit messages correct
8. ✓ Working directory clean
9. ✓ Schema validation passes
10. ✓ Artifacts cleaned up

## Performance

| Test | Time | Operations |
|------|------|------------|
| Environment Check | 1s | System validation |
| Basic Workflow | 10s | Minimal test case |
| Complete Workflow | 30s | Full test case |
| All Tests | 45s | Both + validation |

## Documentation

- `README.md` - User guide
- `TESTING_GUIDE.md` - Comprehensive guide
- `test_scenarios.md` - Coverage matrix
- `IMPLEMENTATION_SUMMARY.md` - Implementation details
- `QUICK_REFERENCE.md` - This file

## Adding New Tests

1. Copy existing test:
   ```bash
   cp e2e_basic_workflow.exp e2e_my_test.exp
   ```

2. Modify workflow and validations

3. Add to `run_all_tests.sh`:
   ```bash
   run_test "My Test" "$SCRIPT_DIR/e2e_my_test.exp"
   ```

4. Update documentation

## Getting Help

1. Check `TESTING_GUIDE.md` for detailed help
2. Run `./tests/integration/check_environment.sh`
3. Review test output carefully
4. Check if CLI prompts changed
5. Open an issue with test output

## Pro Tips

- Run `check_environment.sh` before testing
- Use `make test-e2e` for quick validation
- Use `--build` flag when testing changes
- Keep tests fast (<1 minute)
- Clean up leftover directories regularly
- Review git log in test directories when debugging
