# validate-files.sh Test Suite Quick Reference

Quick reference guide for running and understanding the validate-files.sh integration tests.

## Quick Start

```bash
# Run the test suite
./tests/integration/validate_files_integration.exp

# Or use the wrapper script
./tests/integration/run_validate_files_test.sh

# Run all integration tests (including this suite)
./tests/integration/run_all_tests.sh
```

## What Gets Tested

| Category | Tests | What's Validated |
|----------|-------|------------------|
| **Validator Detection** | 2 | File and command-based validators |
| **Caching** | 5 | Mtime, hash, invalidation, cache entries |
| **Pattern Matching** | 4 | Regex patterns, nested dirs, edge cases |
| **Error Handling** | 4 | Failures, missing validators, permissions |
| **Statistics** | 3 | Cache hit rates, counts, accuracy |
| **CLI Options** | 3 | Verbose, help, custom cache dir |

**Total: 16 test cases covering all major functionality**

## Test Execution Time

**Expected duration: ~15-30 seconds**

## Prerequisites

```bash
# Required tools
expect    # TCL automation
bash      # Shell scripting
sha256sum # or shasum on macOS
stat      # File metadata
find      # File searching

# Install expect
sudo apt-get install expect  # Ubuntu/Debian
brew install expect          # macOS
sudo yum install expect      # RHEL/CentOS
```

## Test Output

### Success
```
==========================================
Integration Test Suite: validate-files.sh
==========================================

==> TEST 1: Validator command detection (file type)
✓ File-based validator detected correctly

==> TEST 2: Validator command detection (command type)
✓ Command-based validator detected correctly

[... 14 more tests ...]

==========================================
ALL TESTS PASSED ✓
==========================================

Test Coverage Summary:
  ✓ Validator detection (file and command)
  ✓ Dual-layer caching (mtime + hash)
  ✓ Cache hit rate calculation
  ✓ Regex pattern matching edge cases
  ✓ Parallel validation
  ✓ Error propagation
  ✓ Custom cache directories
  ✓ Verbose mode
  ✓ Help and argument validation
```

### Failure
```
==> TEST X: Test name
✗ FAILED: Description of failure
Output: [error details]
```

## Key Test Scenarios

### Caching Behavior
1. **First run**: File validated, cache created (0% hit rate)
2. **Second run**: Cache hit via mtime (100% hit rate)
3. **After touch**: Cache hit via hash (mtime changed, content same)
4. **After edit**: Re-validation (both mtime and hash changed)

### Pattern Matching Examples
```bash
# Match .yaml and .yml files
Pattern: \.ya?ml$

# Match files in subdirectories
Pattern: subdir.*\.ya

# Match multiple extensions
Pattern: \.(json|txt)$

# No matches (graceful exit)
Pattern: \.nonexistent$
```

### Cache Hit Rate Scenarios
- **0%**: All files validated (first run)
- **100%**: All files cached (second run, no changes)
- **70%**: Partial cache (7 cached, 3 modified out of 10)

## Troubleshooting

### Common Issues

| Error | Cause | Solution |
|-------|-------|----------|
| `expect: command not found` | Expect not installed | Install expect package |
| `Validator script not found` | Path issue | Check validator path |
| `not executable` | Permission issue | Check file is executable |
| Test timeout | Slow system | Normal for complex tests |

### Debug Mode

Add to test file for detailed output:
```tcl
exp_internal 1  # Show internal Expect operations
```

### Preserve Test Directory

Comment out in test file:
```tcl
# exec rm -rf $test_dir
```

Then inspect:
```bash
ls -la test_validate_files_*/
cat test_validate_files_*/.validation-cache/*.json
```

## Test Coverage Details

### Layer 1 Cache (Mtime)
- ✓ Cache creation
- ✓ Mtime comparison
- ✓ Fast cache hits

### Layer 2 Cache (Hash)
- ✓ Hash calculation (SHA256)
- ✓ Content comparison
- ✓ Mtime update with unchanged content

### Statistics Tracking
- ✓ Total files count
- ✓ Validated vs cached counts
- ✓ Pass/fail counts
- ✓ Cache hit rate percentage
- ✓ Failed file list

### Error Propagation
- ✓ Validation failures reported
- ✓ Failed results cached
- ✓ Non-zero exit codes
- ✓ Error messages clear

## Integration with CI/CD

### GitHub Actions
```yaml
- name: Install expect
  run: sudo apt-get install -y expect

- name: Run validate-files.sh tests
  run: ./tests/integration/run_validate_files_test.sh
```

### GitLab CI
```yaml
test-validate-files:
  before_script:
    - apt-get update && apt-get install -y expect
  script:
    - ./tests/integration/run_validate_files_test.sh
```

## File Structure

```
tests/integration/
├── validate_files_integration.exp       # Main test suite
├── run_validate_files_test.sh          # Test runner
├── VALIDATE_FILES_TEST_COVERAGE.md     # Detailed coverage
└── VALIDATE_FILES_QUICK_REF.md         # This file
```

## Related Documentation

- [validate-files.sh source](../../scripts/validate-files.sh) - Script being tested
- [Detailed test coverage](VALIDATE_FILES_TEST_COVERAGE.md) - Full test documentation
- [Integration testing guide](TESTING_GUIDE.md) - General testing guide
- [Integration README](README.md) - All integration tests

## Support

For issues:
1. Check this quick reference
2. Review detailed coverage documentation
3. Enable debug mode (`exp_internal 1`)
4. Check test output carefully
5. Verify prerequisites installed

## Contributing

When modifying tests:
1. Maintain test independence
2. Update documentation
3. Follow existing patterns
4. Verify all tests pass
5. Update test count in summary
