# validate-files.sh Integration Test Coverage

Comprehensive test coverage documentation for the `validate-files.sh` integration test suite.

## Overview

The integration test suite (`validate_files_integration.exp`) provides comprehensive validation of the `validate-files.sh` script functionality, ensuring correct behavior across various scenarios including caching, pattern matching, error handling, and parallel validation.

## Test Architecture

### Technology Stack
- **Expect**: TCL-based automation for script interaction
- **Bash**: Test file creation and validation
- **Shell utilities**: grep, find, stat, sha256sum/shasum

### Test Environment
- **Temporary directory**: `test_validate_files_[timestamp]`
- **Cache directory**: `test_validate_files_[timestamp]/.validation-cache`
- **Auto-cleanup**: All test artifacts removed on completion

## Test Cases

### TEST 1: Validator Command Detection - File Type
**Purpose**: Verify that file-based validators are correctly detected

**Steps**:
1. Create executable validator script
2. Create test files matching pattern
3. Run validate-files.sh with file-based validator
4. Verify output contains "Using file" message

**Validates**:
- File validator detection logic
- Script executability checking
- Proper validator invocation

### TEST 2: Validator Command Detection - Command Type
**Purpose**: Verify that command-based validators (like `true`, `false`) are correctly detected

**Steps**:
1. Create test files
2. Run validate-files.sh with `true` command as validator
3. Verify output contains "Using command" message

**Validates**:
- Command availability checking via `command` builtin
- Distinction between file and command validators
- Proper command execution

### TEST 3: Dual-Layer Caching - Mtime Check (Layer 1)
**Purpose**: Verify first layer of caching using file modification time

**Steps**:
1. Create test file and validator
2. First run: validate file (no cache)
3. Second run: use cached result (mtime unchanged)
4. Verify statistics show correct validation/cache counts

**Validates**:
- Cache creation on first validation
- Mtime comparison logic
- Cache hit detection
- Statistics tracking (Validated: 0, Cached: 1)

**Expected Behavior**:
- First run: File validated, cache entry created
- Second run: Cache hit via mtime, no re-validation

### TEST 4: Dual-Layer Caching - Hash Check (Layer 2)
**Purpose**: Verify second layer of caching using content hash when mtime changes

**Steps**:
1. Use cached file from TEST 3
2. Touch file to change mtime (content unchanged)
3. Run validation again
4. Verify cache hit via hash comparison

**Validates**:
- Hash calculation (SHA256)
- Mtime update in cache with unchanged hash
- Layer 2 cache logic
- Verbose output: "Mtime changed", "Hash unchanged"

**Expected Behavior**:
- Mtime detected as changed
- Hash comparison performed
- Cache updated with new mtime
- Validation skipped (content unchanged)

### TEST 5: Cache Invalidation on Content Change
**Purpose**: Verify cache is invalidated when file content changes

**Steps**:
1. Modify file content from previous tests
2. Run validation
3. Verify re-validation occurs

**Validates**:
- Content change detection
- Hash comparison logic
- Cache update with new validation result
- Verbose output: "Hash changed"

**Expected Behavior**:
- Both mtime and hash detected as changed
- File re-validated
- New cache entry written

### TEST 6: Cache Hit Rate Calculation Accuracy
**Purpose**: Verify accurate cache hit rate percentage calculation

**Scenarios Tested**:

#### Scenario A: 0% Cache Hit Rate
- 10 files, first run
- Expected: 10 validated, 0 cached, 0.0% hit rate

#### Scenario B: 100% Cache Hit Rate
- 10 files, second run (all cached)
- Expected: 0 validated, 10 cached, 100.0% hit rate

#### Scenario C: 70% Cache Hit Rate
- 10 files, modify 3 files
- Expected: 3 validated, 7 cached, 70.0% hit rate

**Validates**:
- Floating-point calculation accuracy
- Percentage formatting (X.X%)
- Edge cases (0% and 100%)
- Partial cache scenarios

### TEST 7: Regex Pattern Matching Edge Cases
**Purpose**: Verify correct regex pattern matching for various file patterns

**Test Scenarios**:

#### 7a: Optional Character Matching
- Pattern: `\.ya?ml$`
- Files: `.yaml`, `.yml`, `.hidden.yaml`, nested files
- Expected: 4 matches

#### 7b: Subdirectory Matching
- Pattern: `subdir.*\.ya`
- Files: `subdir1/nested.yaml`, `subdir2/deep.yml`
- Expected: 2 matches

#### 7c: Alternation Pattern
- Pattern: `\.(json|txt)$`
- Files: `.json` and `.txt` files
- Expected: 2 matches

#### 7d: No Matches
- Pattern: `\.nonexistent$`
- Expected: Graceful exit with exit code 0, "No files found" message

**Validates**:
- POSIX extended regex support
- Case sensitivity
- Hidden file matching
- Nested directory traversal
- Empty result handling

### TEST 8: Parallel Test Case Validation
**Purpose**: Verify handling of multiple files in sequence

**Steps**:
1. Create 20 test files
2. Create slow validator (simulates real validation work)
3. First run: validate all 20 files
4. Second run: use cache for all 20 files

**Validates**:
- Sequential processing of multiple files
- Bulk cache operations
- Statistics accuracy with large file counts
- Performance with many files

**Expected Behavior**:
- First run: All files validated
- Second run: All files cached (100% hit rate)
- Correct statistics for 20 files

### TEST 9: Error Propagation - Validation Failures
**Purpose**: Verify correct handling and reporting of validation failures

**Steps**:
1. Create 3 files: 2 valid, 1 invalid
2. Create validator that fails for specific content
3. Run validation
4. Verify exit code is non-zero
5. Verify failed file is reported
6. Run again to verify failed result is cached

**Validates**:
- Non-zero exit code on validation failure
- Failed file list reporting
- Correct pass/fail counts
- Caching of failed validation results
- Error message propagation

**Expected Output**:
```
Total files:     3
Validated:       3
Passed:          2
Failed:          1
Failed files:
  - invalid.check
```

### TEST 10: Error Propagation - Missing Validator
**Purpose**: Verify error handling when validator doesn't exist

**Steps**:
1. Run with non-existent validator path
2. Verify non-zero exit code
3. Verify error message

**Validates**:
- Validator existence checking
- Clear error messaging
- Early exit on configuration error

**Expected Error**: "Validator script not found"

### TEST 11: Error Propagation - Non-Executable Validator
**Purpose**: Verify error handling when validator isn't executable

**Steps**:
1. Create validator without execute permissions
2. Run validation
3. Verify error is detected

**Validates**:
- Execute permission checking
- File-based validator validation
- Clear error messaging

**Expected Error**: "not executable"

### TEST 12: Verbose Mode Output
**Purpose**: Verify verbose mode provides detailed debugging information

**Steps**:
1. Run with `--verbose` flag
2. Verify verbose output includes detailed information

**Validates**:
- Verbose flag parsing
- Debug output generation
- Cache checking details
- File processing information

**Expected Output Includes**:
- `[VERBOSE]` prefix on debug messages
- "Checking cache for: [file]"
- "Processing: [file]"
- Cache state information
- Mtime and hash details

### TEST 13: Custom Cache Directory
**Purpose**: Verify support for custom cache directory location

**Steps**:
1. Specify custom cache directory with `--cache-dir`
2. Run validation
3. Verify cache directory is created
4. Verify cache files are written to custom location

**Validates**:
- `--cache-dir` argument parsing
- Custom directory creation
- Cache file path generation
- Directory existence checking

### TEST 14: Cache Entry Content Validation
**Purpose**: Verify cache entries have correct JSON structure

**Steps**:
1. Run validation to create cache entry
2. Read cache file
3. Verify JSON contains all required fields

**Validates**:
- JSON structure
- Required fields: path, mtime, hash, valid, timestamp
- Field value types
- JSON formatting

**Expected Structure**:
```json
{
  "path": "file/path",
  "mtime": 1234567890,
  "hash": "sha256hash...",
  "valid": true,
  "timestamp": 1234567890
}
```

### TEST 15: Help Option
**Purpose**: Verify help documentation is accessible

**Steps**:
1. Run with `--help` flag
2. Verify usage information is displayed

**Validates**:
- Help flag parsing
- Usage message content
- Exit code 0 on help
- Documentation completeness

**Expected Output Includes**:
- Usage line
- Options descriptions
- Examples
- Exit code information

### TEST 16: Missing Required Arguments
**Purpose**: Verify error handling for missing required arguments

**Scenarios**:
- Missing `--pattern`: Error message and non-zero exit
- Missing `--validator`: Error message and non-zero exit

**Validates**:
- Argument requirement checking
- Clear error messages
- Usage hint on error
- Non-zero exit codes

## Test Statistics

### Coverage Metrics
- **Total Test Cases**: 16
- **Command Detection**: 2 tests
- **Caching Logic**: 5 tests
- **Pattern Matching**: 4 scenarios in 1 test
- **Error Handling**: 4 tests
- **CLI Options**: 3 tests
- **Performance**: 1 test (parallel validation)

### Feature Coverage
- ✓ File-based validator detection
- ✓ Command-based validator detection
- ✓ Layer 1 caching (mtime)
- ✓ Layer 2 caching (hash)
- ✓ Cache invalidation
- ✓ Cache hit rate calculation (0%, partial, 100%)
- ✓ POSIX extended regex patterns
- ✓ Nested directory matching
- ✓ Multiple file validation
- ✓ Validation failure handling
- ✓ Failed result caching
- ✓ Missing validator errors
- ✓ Non-executable validator errors
- ✓ Verbose mode
- ✓ Custom cache directories
- ✓ Cache entry JSON structure
- ✓ Help documentation
- ✓ Missing argument detection

## Running the Tests

### Quick Start
```bash
# Run the integration test suite
./tests/integration/validate_files_integration.exp

# Or use the runner script
./tests/integration/run_validate_files_test.sh
```

### Requirements
- **expect**: TCL automation tool
- **bash**: Shell scripting
- **sha256sum** or **shasum**: Hash calculation
- **stat**: File metadata
- **find**: File searching

### Installation
```bash
# Ubuntu/Debian
sudo apt-get install expect

# macOS
brew install expect

# RHEL/CentOS
sudo yum install expect
```

## Test Duration

Expected execution time: **~15-30 seconds**

Breakdown:
- Environment setup: ~1s
- Test execution: ~10-25s
- Cleanup: ~1s

## Debugging Tests

### Enable Expect Internal Logging
Add to test file:
```tcl
exp_internal 1
```

### Preserve Test Directory
Comment out cleanup:
```tcl
# exec rm -rf $test_dir
```

Then inspect:
```bash
ls -la test_validate_files_*/
cat test_validate_files_*/.validation-cache/*.json
```

### Adjust Timeout
```tcl
set timeout 120  # Increase if needed
```

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
  script:
    - apt-get update && apt-get install -y expect
    - ./tests/integration/run_validate_files_test.sh
```

## Maintenance

### Adding New Tests
1. Add new test section in `validate_files_integration.exp`
2. Follow existing test pattern structure
3. Update this documentation
4. Increment test count in summary

### Modifying Existing Tests
1. Update test case section
2. Update documentation to match
3. Verify all tests still pass
4. Update test statistics if needed

## Known Limitations

1. **Sequential Validation**: Tests validate that files are processed sequentially, not in parallel (current implementation)
2. **Platform Differences**: Some tests may behave slightly differently on macOS vs Linux (stat command differences handled)
3. **Timing Sensitivity**: Cache invalidation tests use sleep/delays to ensure mtime changes are detectable
4. **Hash Algorithm**: Tests assume SHA256 is available (fallback to shasum on macOS)

## Future Enhancements

Potential areas for additional test coverage:
- [ ] Symlink handling
- [ ] Permission denied scenarios
- [ ] Very large file counts (100+ files)
- [ ] Binary file validation
- [ ] Cache corruption recovery
- [ ] Concurrent execution safety
- [ ] Cache expiration policies
- [ ] Custom hash algorithms

## References

- [validate-files.sh source](../../scripts/validate-files.sh)
- [Expect documentation](https://core.tcl-lang.org/expect/index)
- [Integration testing guide](TESTING_GUIDE.md)
