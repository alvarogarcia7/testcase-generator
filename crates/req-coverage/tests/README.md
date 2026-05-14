# req-coverage Test Suite

This directory contains comprehensive test coverage for the requirement coverage string-based verification feature.

## Test Types

### 1. Unit Tests (in `src/models.rs`)
Tests for data models, serialization, and core functionality.

### 2. Rust Integration Tests (in `tests/*.rs`)
In-process integration tests using the library interface.

### 3. Shell Integration Tests (in `tests/integration/`)
End-to-end tests running the actual `req-coverage` binary. See [integration/README.md](integration/README.md) for details.

## Test Files

### string_verification_tests.rs

Integration tests for the string-based requirement coverage verification feature. These tests validate:

1. **Full Coverage Tests**
   - `test_full_coverage_with_single_test_case` - Single test case that fully covers a requirement
   - `test_full_coverage_with_multiple_test_cases` - Multiple test cases that together fully cover a requirement

2. **Partial Coverage Tests**
   - `test_partial_coverage_with_multiple_test_cases` - Test cases that only partially cover a requirement

3. **Error Detection Tests**
   - `test_invalid_covers_string_error` - Validates error reporting when covers string is not in requirement text
   - `test_missing_requirement_definition` - Validates error when requirement definition doesn't exist
   - `test_case_sensitive_matching` - Verifies case-sensitive string matching

4. **Backward Compatibility Tests**
   - `test_without_requirements_file` - Ensures tool works without requirements file (legacy mode)

5. **Format Support Tests**
   - `test_json_requirements_file` - Tests JSON format for requirements definitions

6. **Multi-Requirement Tests**
   - `test_multiple_requirements` - Tests handling of multiple requirements with different coverage levels

7. **Test Status Tests**
   - `test_coverage_with_test_failures` - Tests coverage calculation with failing test cases

8. **Edge Cases**
   - `test_duplicate_covers_strings` - Multiple test cases with same covers string
   - `test_overlapping_covers_strings` - Test cases with overlapping coverage text
   - `test_empty_covers_string` - Test case without covers field

## Running Tests

### Quick Start - Use the Shell Script (Recommended)

Run all tests and save results automatically:
```bash
cd crates/req-coverage
./run_integration_tests.sh
```

This script will:
- Run all unit tests
- Run all integration tests
- Save results to `test_results/integration_test_results_YYYYMMDD_HHMMSS.txt`
- Create a symlink to latest results
- Display a summary

View the latest results:
```bash
cat crates/req-coverage/test_results/latest_results.txt
```

### Manual Test Commands

#### Run all tests
```bash
cargo test -p req-coverage
```

#### Run specific test
```bash
cargo test -p req-coverage test_full_coverage_with_single_test_case
```

#### Run with output
```bash
cargo test -p req-coverage -- --nocapture
```

#### Run integration tests only
```bash
cargo test -p req-coverage --test string_verification_tests
```

#### Run unit tests only
```bash
cargo test -p req-coverage --lib
```

## Test Structure

Integration tests follow this pattern:

1. **Setup** - Create temporary directories and files
2. **Arrange** - Create requirement definitions, test cases, and verification results
3. **Act** - Run the coverage analyzer
4. **Assert** - Verify the expected behavior

## Test Coverage

The test suite covers:

- ✅ Full coverage detection
- ✅ Partial coverage detection
- ✅ Error detection and reporting
- ✅ Multiple requirement handling
- ✅ Test pass/fail status tracking
- ✅ YAML and JSON format support
- ✅ Backward compatibility
- ✅ Case-sensitive matching
- ✅ Edge cases (duplicates, overlaps, empty strings)

## Adding New Tests

When adding new tests:

1. Use descriptive test names that explain what is being tested
2. Include comments explaining the test scenario
3. Follow the existing test structure (setup, arrange, act, assert)
4. Clean up resources (tempfile handles this automatically)
5. Update this README with the new test description

## Test Data

Tests use temporary directories created with `tempfile::TempDir` to ensure isolation and automatic cleanup. Test data includes:

- Minimal valid test case YAML files
- Verification container YAML files
- Requirement definition files (YAML and JSON)

## Debugging Tests

To debug a failing test:

```bash
# Run with verbose output
RUST_LOG=debug cargo test -p req-coverage test_name -- --nocapture

# Run a single test
cargo test -p req-coverage test_name -- --exact
```
