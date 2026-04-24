# Testing Guide for req-coverage

Complete guide to testing the `req-coverage` tool.

## Test Overview

The `req-coverage` tool has three layers of testing:

1. **Unit Tests** - Fast, in-memory tests of individual components
2. **Rust Integration Tests** - In-process tests of library functionality  
3. **Shell Integration Tests** - End-to-end tests of the actual binary

## Quick Start

### Run All Tests

```bash
# Unit tests
cargo test -p req-coverage --lib

# Rust integration tests  
cargo test -p req-coverage --test string_verification_tests
cargo test -p req-coverage --test simple_test

# Shell integration tests (requires jq and binary)
cargo build -p req-coverage
cd crates/req-coverage/tests/integration
./test_runner.sh
```

### Run Specific Tests

```bash
# Specific unit test
cargo test -p req-coverage --lib test_coverage_report_new

# Specific Rust integration test
cargo test -p req-coverage --test string_verification_tests test_full_coverage_with_single_test_case

# All tests in one go
cargo test -p req-coverage && cd crates/req-coverage/tests/integration && ./test_runner.sh
```

## Unit Tests (11 tests)

**Location:** `src/models.rs`

**What they test:**
- Data model serialization/deserialization
- Coverage status colors and display names
- Coverage report statistics computation
- Requirement definition handling

**Run them:**
```bash
cargo test -p req-coverage --lib
```

**Example output:**
```
running 11 tests
test models::tests::test_coverage_type_serialization ... ok
test models::tests::test_coverage_status_colors ... ok
...
test result: ok. 11 passed; 0 failed
```

## Rust Integration Tests (15 tests)

**Location:** `tests/string_verification_tests.rs`, `tests/simple_test.rs`

**What they test:**
- Full coverage detection with requirement definitions
- Partial coverage detection  
- Error reporting for invalid covers strings
- Multiple requirement handling
- Backward compatibility without requirements file
- JSON and YAML format support
- Case sensitivity
- Edge cases (duplicates, overlaps)

**Run them:**
```bash
# All Rust integration tests
cargo test -p req-coverage --test string_verification_tests

# With debug output
RUST_LOG=debug cargo test -p req-coverage --test string_verification_tests -- --nocapture

# Specific test
cargo test -p req-coverage test_full_coverage_with_single_test_case
```

## Shell Integration Tests (10 tests)

**Location:** `tests/integration/test_runner.sh`

**What they test:**
- End-to-end binary execution
- Requirement verification workflow
- HTML report generation
- JSON and YAML requirements formats
- Error handling and reporting
- Backward compatibility
- Real-world usage scenarios

**Prerequisites:**
```bash
# Install jq
brew install jq  # macOS
apt-get install jq  # Linux

# Build the binary
cargo build -p req-coverage
```

**Run them:**
```bash
cd crates/req-coverage/tests/integration
./test_runner.sh
```

**Check results:**
```bash
# View saved test results
ls -la results/

# View a specific JSON report
cat results/test_full_coverage_single.json | jq .

# View HTML report
open results/test_html_output/index.html
```

## Test Results

### Unit Tests
- Run in-process, no output files
- Fast execution (< 1 second)
- Part of `cargo test` workflow

### Rust Integration Tests  
- Run in-process with temporary directories
- Fast execution (< 5 seconds)
- Automatic cleanup
- Part of `cargo test` workflow

### Shell Integration Tests
- Spawn actual binary processes
- Slower execution (10-30 seconds)
- **Results saved to `tests/integration/results/`**
- Includes:
  - `*.json` - Coverage report outputs
  - `*.log` - Command execution logs
  - `test_html_output/` - Generated HTML report

## Debugging Tests

### Unit Test Failures
```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test -p req-coverage --lib test_name

# Run with verbose output
cargo test -p req-coverage --lib test_name -- --nocapture
```

### Rust Integration Test Failures
```bash
# Enable debug logging
RUST_LOG=debug cargo test -p req-coverage --test string_verification_tests test_name -- --nocapture

# Run single test
cargo test -p req-coverage --test string_verification_tests test_name -- --exact
```

### Shell Integration Test Failures
```bash
# Check the log file
cat tests/integration/results/test_name.log

# Check the JSON output
cat tests/integration/results/test_name.json | jq .

# Run single test by commenting out others in test_runner.sh main()
```

## CI/CD Integration

### GitHub Actions Example
```yaml
- name: Run unit tests
  run: cargo test -p req-coverage --lib

- name: Run Rust integration tests  
  run: cargo test -p req-coverage

- name: Run shell integration tests
  run: |
    cargo build -p req-coverage
    cd crates/req-coverage/tests/integration
    ./test_runner.sh
    
- name: Archive test results
  uses: actions/upload-artifact@v3
  if: always()
  with:
    name: test-results
    path: crates/req-coverage/tests/integration/results/
```

### GitLab CI Example
```yaml
test:
  script:
    - cargo test -p req-coverage
    - cargo build -p req-coverage
    - cd crates/req-coverage/tests/integration && ./test_runner.sh
  artifacts:
    paths:
      - crates/req-coverage/tests/integration/results/
    when: always
```

## Test Coverage Summary

| Layer | Count | Speed | Scope |
|-------|-------|-------|-------|
| Unit Tests | 11 | Fast | Data models, utilities |
| Rust Integration | 15 | Fast | Library functionality |
| Shell Integration | 10 | Moderate | Binary, end-to-end |
| **Total** | **36** | - | **Complete coverage** |

## Adding New Tests

### Unit Test
Edit `src/models.rs`, add test in `#[cfg(test)] mod tests { ... }`

### Rust Integration Test  
Add test function to `tests/string_verification_tests.rs`

### Shell Integration Test
1. Add test function to `tests/integration/test_runner.sh`
2. Add call to test in `main()` function
3. Document in `tests/integration/README.md`

## Test Documentation

- **Unit Tests**: Documented in code comments
- **Rust Integration Tests**: See `tests/README.md`
- **Shell Integration Tests**: See `tests/integration/README.md`

## Related Documentation

- [README.md](README.md) - General usage
- [tests/README.md](tests/README.md) - Detailed test documentation
- [tests/integration/README.md](tests/integration/README.md) - Shell test guide
- [REQ_COVERAGE_TESTS_IMPLEMENTATION.md](../../REQ_COVERAGE_TESTS_IMPLEMENTATION.md) - Test implementation details
