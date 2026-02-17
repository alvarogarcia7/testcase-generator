# json-escape End-to-End Integration Test

## Overview

`test_json_escape_e2e.sh` is a comprehensive end-to-end integration test for the json-escape feature in the testcase-manager project. This test validates the complete workflow of JSON string escaping in generated test scripts, covering multiple escaping methods, fallback mechanisms, and complex special character handling.

## Purpose

The json-escape feature ensures that command output containing special characters (quotes, newlines, backslashes, tabs, etc.) is properly escaped when written to JSON log files. This test validates:

1. The json-escape binary works correctly
2. Generated test scripts use the correct escaping method
3. Fallback mechanisms work when the binary is unavailable
4. All escaping methods produce valid JSON output
5. Complex special characters are handled correctly

## Test Coverage

### Test 1: Build json-escape binary
- Builds the json-escape binary from source
- Verifies the binary exists and is executable
- **Assertions**: 2

### Test 2: Test json-escape with special characters
- Tests basic quote escaping
- Tests newline escaping
- Tests backslash escaping
- Tests tab escaping
- **Assertions**: 4

### Test 3: Test json-escape validation mode
- Tests validation mode with simple input
- Tests validation mode with special characters
- **Assertions**: 2

### Test 4: Generate and execute test script with RustBinary mode
- Creates test case YAML with commands containing special characters
- Generates test script configured for RustBinary mode
- Verifies script uses json-escape binary
- Executes script and validates JSON output
- Validates JSON structure with jq
- **Assertions**: 8+ (with jq)

### Test 5: Generate and execute test script with ShellFallback mode
- Generates test script configured for ShellFallback mode
- Verifies script uses sed/awk instead of json-escape
- Verifies script does NOT check for json-escape binary
- Executes script and validates JSON output
- **Assertions**: 5+ (with jq)

### Test 6: Generate and execute test script with Auto mode (binary available)
- Generates test script configured for Auto mode
- Verifies script checks for json-escape availability
- Verifies script contains shell fallback
- Executes script with json-escape in PATH
- Validates JSON output
- **Assertions**: 5+ (with jq)

### Test 7: Test Auto mode fallback when binary is not in PATH
- Removes json-escape from PATH
- Executes script with Auto mode (should fall back to shell)
- Validates JSON output is still valid
- **Assertions**: 2+ (with jq)

### Test 8: Test with complex special characters
- Creates test case with mixed special characters
- Tests quotes, newlines, tabs, backslashes in combination
- Tests JSON-like output strings
- Validates all entries are properly escaped
- **Assertions**: 6+ (with jq)

### Test 9: Verify json-escape handles empty input
- Tests json-escape with empty input
- Verifies empty output
- **Assertions**: 1

### Test 10: Verify shell fallback escaping patterns
- Tests shell fallback directly (sed/awk pipeline)
- Validates backslash escaping pattern
- Validates quote escaping pattern
- Validates newline escaping pattern
- **Assertions**: 3

## Total Test Assertions

- **Minimum**: 37 pass/fail assertions
- **With jq available**: 40+ assertions
- **Test categories**: 10

## Usage

### Basic Usage

```bash
cd tests/integration
./test_json_escape_e2e.sh
```

### Keep Temporary Files (for debugging)

```bash
./test_json_escape_e2e.sh --no-remove
```

This keeps the temporary test directory so you can inspect:
- Generated YAML test cases
- Generated shell scripts
- JSON log files
- Build logs
- Execution logs

### Prerequisites

- Rust toolchain (cargo)
- bash 3.2+
- sed (BSD or GNU)
- awk (BSD or GNU)
- jq (optional, for JSON validation)

## Test Data

### Special Characters Tested

| Character | Description | JSON Escape |
|-----------|-------------|-------------|
| `"` | Double quote | `\"` |
| `\` | Backslash | `\\` |
| `\n` | Newline | `\n` |
| `\r` | Carriage return | `\r` |
| `\t` | Tab | `\t` |

### JSON Escaping Methods

| Method | Description | Fallback |
|--------|-------------|----------|
| `RustBinary` | Uses json-escape binary | None |
| `ShellFallback` | Uses sed/awk only | N/A |
| `Auto` | Tries binary, falls back to shell | Yes |

## Generated Test Files

The test creates temporary test case YAML files:

1. **test_rust_binary.yaml**: Basic test case with special characters (4 steps)
2. **test_complex_chars.yaml**: Complex special character test (3 steps)

Each test case includes commands that output:
- Quotes in strings
- Multi-line output
- Backslashes in paths
- Tab-separated values
- Mixed special characters

## Validation

### JSON Validation with jq

If jq is available, the test validates:
- JSON is well-formed and parseable
- Correct number of entries in array
- Each entry has required fields
- Output strings are properly escaped

### Manual Validation

Without jq, the test still validates:
- JSON log files are created
- Scripts execute successfully
- Generated scripts contain expected patterns

## Exit Codes

- `0`: All tests passed
- `1`: One or more tests failed

## Output Format

The test uses the centralized logger library (`scripts/lib/logger.sh`) for consistent output:

```
=== json-escape End-to-End Integration Test ===

=== Test 1: Build json-escape binary ===
[INFO] Building json-escape binary...
✓ json-escape binary built successfully
✓ json-escape binary exists at /path/to/binary

=== Test 2: Test json-escape with special characters ===
[INFO] Testing basic escaping...
✓ Basic quote escaping works
[INFO] Testing newline escaping...
✓ Newline escaping works
...

=== Test Summary ===

Total tests: 37
Passed: 37
Failed: 0

✓ All tests passed!
```

## Debugging

### View Test Script

When using `--no-remove`, inspect the generated test script:

```bash
cat /tmp/tmp.XXXXXX/TEST_RUST_BINARY_test.sh
```

### View JSON Output

Check the generated JSON log:

```bash
cat /tmp/tmp.XXXXXX/TEST_RUST_BINARY_execution_log.json
jq . /tmp/tmp.XXXXXX/TEST_RUST_BINARY_execution_log.json
```

### View Build/Execution Logs

```bash
cat /tmp/tmp.XXXXXX/build.log
cat /tmp/tmp.XXXXXX/generate_rust_binary.log
cat /tmp/tmp.XXXXXX/execute_rust_binary.log
```

## Common Issues

### jq not available

The test will skip JSON validation assertions if jq is not installed. To get full coverage:

```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq

# RHEL/CentOS
sudo yum install jq
```

### Build failures

If the json-escape binary fails to build:

1. Verify Rust toolchain is installed: `cargo --version`
2. Check cargo build output in the test logs
3. Try building manually: `cargo build --bin json-escape`

### Script generation failures

If test-executor fails to generate scripts:

1. Verify test-executor binary exists: `ls target/debug/test-executor`
2. Build if needed: `cargo build --bin test-executor`
3. Check YAML syntax in generated test files
4. Review generation logs in temp directory

## Architecture

### Test Flow

```
1. Build json-escape binary
   ↓
2. Test json-escape directly with special chars
   ↓
3. Test json-escape validation mode
   ↓
4. Generate test script (RustBinary mode)
   ↓
5. Execute script with binary available
   ↓
6. Validate JSON output
   ↓
7. Generate test script (ShellFallback mode)
   ↓
8. Execute script (sed/awk only)
   ↓
9. Validate JSON output
   ↓
10. Generate test script (Auto mode)
    ↓
11. Execute with binary in PATH
    ↓
12. Execute without binary in PATH
    ↓
13. Validate both produce valid JSON
    ↓
14. Test complex special characters
    ↓
15. Test shell fallback directly
```

### Script Generation

The test uses `test-executor` to generate shell scripts from YAML test cases. The generated scripts:

1. Execute commands and capture output
2. Escape output for JSON using configured method
3. Write JSON log with escaped output
4. Validate JSON with jq (if available)

### Escaping Pipeline

#### RustBinary Mode
```
command output → json-escape binary → escaped string → JSON log
```

#### ShellFallback Mode
```
command output → sed (escape chars) → awk (handle newlines) → escaped string → JSON log
```

#### Auto Mode
```
command output → check for json-escape
                 ├─ available: use binary
                 └─ not available: use shell fallback
                 → escaped string → JSON log
```

## Integration with CI/CD

This test can be integrated into CI/CD pipelines:

```yaml
# GitHub Actions example
- name: Run json-escape E2E tests
  run: |
    ./tests/integration/test_json_escape_e2e.sh
```

```yaml
# GitLab CI example
json-escape-e2e:
  script:
    - ./tests/integration/test_json_escape_e2e.sh
  artifacts:
    when: on_failure
    paths:
      - /tmp/tmp.*/
```

## Performance

- **Typical duration**: 10-30 seconds
- **Test cases generated**: 2
- **Scripts executed**: 5+
- **JSON files validated**: 4+
- **Binary builds**: 1

## Cross-Platform Compatibility

The test validates cross-platform compatibility:

- **bash 3.2+**: macOS default shell version
- **sed**: BSD and GNU compatible patterns (uses `-E`, not `-r`)
- **awk**: Portable printf patterns for newline handling
- **Shell constructs**: No bash 4.0+ features (no `declare -A`)

## Related Files

- **Binary**: `src/bin/json-escape.rs`
- **Rust test**: `tests/json_escape_integration_test.rs`
- **Logger library**: `scripts/lib/logger.sh`
- **Test executor**: `src/bin/test-executor.rs`
- **Script generator**: `src/executor.rs`

## Future Enhancements

Potential improvements for this test:

1. Test unicode character escaping
2. Test very large output (>1MB)
3. Test concurrent script execution
4. Test control character escaping (beyond basic \n, \r, \t)
5. Test carriage return only (Mac Classic style)
6. Benchmark performance differences between escaping methods
7. Test in containers with minimal shells
8. Add Windows compatibility testing (Git Bash, WSL)

## Contributing

When modifying this test:

1. Keep test cases focused and independent
2. Use the logger library for output
3. Update assertion counts in this documentation
4. Test on both macOS and Linux if possible
5. Verify with and without jq available
6. Update related documentation (README.md, INDEX.md)

## References

- [Integration Tests README](README.md)
- [Integration Tests Index](INDEX.md)
- [AGENTS.md](../../AGENTS.md) - Shell script compatibility requirements
- [Logger Library](../../scripts/lib/logger.sh)
