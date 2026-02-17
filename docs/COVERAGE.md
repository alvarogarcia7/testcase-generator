# Code Coverage Testing Guide

This guide covers everything you need to know about code coverage testing in this project, including installation, usage, configuration, and troubleshooting.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Available Make Targets](#available-make-targets)
- [Coverage Threshold Configuration](#coverage-threshold-configuration)
- [Using HTML Reports](#using-html-reports)
- [LCOV Export for CI/CD](#lcov-export-for-cicd)
- [Interpreting Coverage Results](#interpreting-coverage-results)
- [Troubleshooting](#troubleshooting)

## Installation

Code coverage is powered by `cargo-llvm-cov`, a Cargo subcommand that provides LLVM-based code coverage for Rust projects.

### Install cargo-llvm-cov

```bash
cargo install cargo-llvm-cov
```

### Verify Installation

After installation, verify that `cargo-llvm-cov` is available:

```bash
cargo llvm-cov --version
```

### System Requirements

- **Rust**: 1.60.0 or later recommended
- **LLVM**: `cargo-llvm-cov` bundles its own LLVM tools, so no additional installation is required
- **Operating Systems**: Linux, macOS, and Windows are supported

## Quick Start

The fastest way to check coverage is to run:

```bash
make coverage-report
```

This displays a summary of coverage statistics in your terminal without generating additional files.

For a more detailed analysis:

```bash
make coverage-html
```

This generates an interactive HTML report and opens it in your default browser.

## Available Make Targets

The project provides several Make targets for different coverage workflows:

### `make coverage`

**Purpose**: Run tests with coverage analysis and enforce threshold.

**Command**: `cargo llvm-cov --all-features --workspace --fail-under-lines 70`

**When to use**:
- Part of the pre-commit workflow (see AGENTS.md)
- CI/CD pipeline validation
- Ensuring code meets minimum coverage requirements

**Behavior**:
- Runs all tests with coverage tracking enabled
- Fails if line coverage falls below 70%
- Exit code 0 on success, non-zero on failure

**Example**:
```bash
$ make coverage
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running unittests src/lib.rs (target/llvm-cov-target/debug/deps/testcase_manager-...)
...
error: coverage rate is below 70.0% (actual: 65.3%)
make: *** [coverage] Error 1
```

### `make coverage-html`

**Purpose**: Generate an interactive HTML coverage report for local development.

**Command**: `cargo llvm-cov --all-features --workspace --html --open`

**When to use**:
- Investigating which code paths are not covered
- Identifying gaps in test coverage
- Visual exploration of coverage data
- Local development and debugging

**Behavior**:
- Generates HTML report in `target/llvm-cov/html/`
- Automatically opens the report in your default browser
- Shows line-by-line coverage with color coding

**Output location**: `target/llvm-cov/html/index.html`

### `make coverage-report`

**Purpose**: Display a summary of coverage statistics in the terminal.

**Command**: `cargo llvm-cov report --all-features --workspace`

**When to use**:
- Quick coverage check without generating files
- Terminal-based workflows
- Getting an overview before deeper investigation

**Behavior**:
- Shows coverage percentages by file and overall
- Displays line, region, and function coverage
- No files are generated (output only to stdout)

**Example output**:
```
Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed
---------------------------------------------------------------------------------------------------------
src/lib.rs                        45                 3    93.33%          12                 0   100.00%
src/models/mod.rs                 23                 8    65.22%           8                 2    75.00%
...
---------------------------------------------------------------------------------------------------------
TOTAL                            892               156    82.51%         234                18    92.31%
```

### `make coverage-lcov`

**Purpose**: Export coverage data in LCOV format for CI/CD integration.

**Command**: `cargo llvm-cov --all-features --workspace --lcov --output-path target/llvm-cov/lcov.info`

**When to use**:
- CI/CD pipelines (GitLab CI, GitHub Actions, etc.)
- Integration with coverage tracking services (Codecov, Coveralls)
- Automated coverage reporting

**Behavior**:
- Generates `lcov.info` file at `target/llvm-cov/lcov.info`
- LCOV format is widely supported by coverage tools
- Suitable for parsing and automated processing

**Output location**: `target/llvm-cov/lcov.info`

### `make coverage-clean`

**Purpose**: Remove all coverage data and artifacts.

**Command**: `cargo llvm-cov clean --workspace`

**When to use**:
- Coverage data appears stale or corrupted
- Disk space cleanup
- Starting fresh coverage analysis
- Before rebuilding coverage from scratch

**Behavior**:
- Removes all coverage artifacts from `target/llvm-cov/`
- Does not remove compiled test binaries
- Requires re-running tests to regenerate coverage data

## Coverage Threshold Configuration

The project enforces a **minimum line coverage threshold of 70%**. This ensures a baseline level of test coverage across the codebase.

### Current Configuration

The threshold is configured in the `make coverage` target:

```makefile
coverage:
	cargo llvm-cov --all-features --workspace --fail-under-lines 70
```

### Customizing the Threshold

To modify the threshold, edit the `Makefile` and change the `--fail-under-lines` value:

**Example: Increase threshold to 75%**
```makefile
coverage:
	cargo llvm-cov --all-features --workspace --fail-under-lines 75
```

**Example: Increase threshold to 80%**
```makefile
coverage:
	cargo llvm-cov --all-features --workspace --fail-under-lines 80
```

### Other Threshold Options

`cargo-llvm-cov` supports multiple threshold types:

- `--fail-under-lines N`: Line coverage threshold (current: 70%)
- `--fail-under-regions N`: Region coverage threshold
- `--fail-under-functions N`: Function coverage threshold

You can combine multiple thresholds:

```makefile
coverage:
	cargo llvm-cov --all-features --workspace \
		--fail-under-lines 70 \
		--fail-under-functions 80
```

### Coverage Requirements by Context

- **Minimum threshold**: 70% line coverage (enforced)
- **Recommended for new code**: 80%+ line coverage
- **Critical paths**: Aim for 90%+ coverage
- **Pre-commit requirement**: Must pass the 70% threshold

See `AGENTS.md` for the complete pre-commit workflow requirements.

## Using HTML Reports

HTML reports provide the most detailed view of coverage data and are invaluable for understanding test gaps.

### Generating the Report

```bash
make coverage-html
```

The report will automatically open in your browser. If it doesn't, manually open:
```
target/llvm-cov/html/index.html
```

### Navigating the HTML Report

#### Index Page
- **Overview**: Summary statistics for the entire project
- **File list**: All source files with coverage percentages
- **Color coding**:
  - **Green**: Well-covered files (typically >80%)
  - **Yellow**: Moderately covered (typically 60-80%)
  - **Red**: Poorly covered (typically <60%)

#### File View
Click any source file to see line-by-line coverage:
- **Green lines**: Executed by tests
- **Red lines**: Not executed by tests
- **Gray lines**: Non-executable (comments, blank lines)
- **Execution counts**: Numbers on the left show how many times each line was executed

#### Region Coverage
Hover over highlighted regions to see:
- Branch coverage (if/else paths)
- Which branches were taken
- Which branches were not tested

### Best Practices for Using HTML Reports

1. **Start with low-coverage files**: Focus on files with <70% coverage first
2. **Look for critical paths**: Ensure error handling and edge cases are covered
3. **Check branch coverage**: Not just lines, but all conditional paths
4. **Identify dead code**: Lines that never execute might be unnecessary
5. **Use during development**: Generate reports frequently as you add tests

### Example Workflow

1. Generate the report: `make coverage-html`
2. Identify files below 70% coverage
3. Open each file in the HTML report
4. Note uncovered lines (red)
5. Write tests to cover those lines
6. Re-run `make coverage-html` to verify improvement

## LCOV Export for CI/CD

LCOV is a standardized format for coverage data that integrates with many CI/CD systems and coverage services.

### Generating LCOV Output

```bash
make coverage-lcov
```

This creates `target/llvm-cov/lcov.info`.

### GitLab CI Integration

Add to your `.gitlab-ci.yml`:

```yaml
test:coverage:
  stage: test
  script:
    - cargo install cargo-llvm-cov
    - make coverage-lcov
  coverage: '/TOTAL.*\s+(\d+\.\d+)%/'
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: target/llvm-cov/lcov.info
```

For more GitLab CI examples, see `docs/GITLAB_CI_SETUP.md` and `docs/GITLAB_CI_EXAMPLES.md`.

### GitHub Actions Integration

Add to your workflow:

```yaml
- name: Install cargo-llvm-cov
  run: cargo install cargo-llvm-cov

- name: Generate coverage
  run: make coverage-lcov

- name: Upload to Codecov
  uses: codecov/codecov-action@v3
  with:
    files: target/llvm-cov/lcov.info
    fail_ci_if_error: true
```

### Codecov Integration

```bash
# After generating lcov.info
bash <(curl -s https://codecov.io/bash) -f target/llvm-cov/lcov.info
```

### Coveralls Integration

```bash
# After generating lcov.info
coveralls --lcov target/llvm-cov/lcov.info
```

### LCOV Format Details

The `lcov.info` file contains:
- **TN**: Test name
- **SF**: Source file path
- **FN**: Function name
- **FNDA**: Function execution count
- **FNF/FNH**: Functions found/hit
- **DA**: Line execution count
- **LF/LH**: Lines found/hit
- **BRDA**: Branch data
- **BRF/BRH**: Branches found/hit

This format is parseable by most coverage tools and can be processed with scripts.

## Interpreting Coverage Results

Coverage data includes three main metrics: line coverage, region coverage, and function coverage.

### Line Coverage

**Definition**: Percentage of executable lines that were executed during tests.

**Formula**: `(Executed Lines / Total Executable Lines) × 100`

**Example**:
```
Lines: 450/500 (90.0%)
```
This means 450 out of 500 executable lines were run during tests.

**What it tells you**:
- Basic measure of test thoroughness
- Easy to understand and improve
- Most commonly used metric

**Limitations**:
- Doesn't capture branch coverage
- A line might execute but not test all paths

### Region Coverage

**Definition**: Percentage of code regions (branches, conditions) that were executed.

**Example**:
```rust
if condition {        // Region 1
    do_something();   // Region 2
} else {             // Region 3
    do_other();      // Region 4
}
```

If only the `if` branch executes:
- Line coverage: 100% (all lines executed)
- Region coverage: 50% (only regions 1-2 executed)

**What it tells you**:
- Whether all conditional branches were tested
- More accurate than line coverage
- Catches untested error paths

**When it matters**:
- Error handling code
- Complex conditionals
- State machine transitions

### Function Coverage

**Definition**: Percentage of functions that were called during tests.

**Example**:
```
Functions: 45/50 (90.0%)
```
This means 45 out of 50 functions were called at least once.

**What it tells you**:
- Which functions are completely untested
- Dead or unused code
- API surface coverage

**Limitations**:
- Calling a function once doesn't mean it's well-tested
- Doesn't measure quality of testing

### Reading Coverage Reports

#### Terminal Output (make coverage-report)

```
Filename                      Regions    Missed Regions     Cover   Functions  Missed Functions  Executed
---------------------------------------------------------------------------------------------------------
src/executor.rs                   125                 5    96.00%          18                 0   100.00%
src/parser.rs                      89                18    79.78%          12                 2    83.33%
src/validator.rs                   56                23    58.93%           8                 3    62.50%
---------------------------------------------------------------------------------------------------------
TOTAL                             892               156    82.51%         234                18    92.31%
```

**How to read this**:
- **Regions**: Total code regions and how many weren't executed
- **Cover**: Region coverage percentage
- **Functions**: Total functions and how many weren't called
- **Executed**: Function coverage percentage

**Action items**:
- Focus on files with <70% region coverage (e.g., `validator.rs`)
- Check which functions weren't called (3 functions in `validator.rs`)

#### HTML Report

- **Green highlighted**: Code that was executed
- **Red highlighted**: Code that was NOT executed
- **Execution counts**: Numbers show how many times each line ran
  - `0`: Never executed (red)
  - `1+`: Executed at least once (green)
  - High numbers: Executed many times (e.g., in loops)

### Coverage Goals

- **70%**: Minimum required threshold (enforced)
- **80%**: Good coverage for most code
- **90%+**: Excellent coverage for critical paths
- **100%**: Ideal but not always practical or necessary

### What Good Coverage Looks Like

**Well-covered function**:
```rust
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {                           // ✓ Both branches tested
        return Err("Division by zero");   // ✓ Error path tested
    }
    Ok(a / b)                             // ✓ Success path tested
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }
}
```
- Line coverage: 100%
- Region coverage: 100% (both branches tested)
- Function coverage: 100%

**Poorly-covered function** (missing error case):
```rust
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {                           // ✗ Only true branch tested
        return Err("Division by zero");   // ✗ Never executed
    }
    Ok(a / b)                             // ✓ Success path tested
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }
    // Missing: test_divide_by_zero
}
```
- Line coverage: 66% (2 of 3 lines)
- Region coverage: 50% (1 of 2 branches)
- Function coverage: 100% (function was called)

## Troubleshooting

### Common Issues and Solutions

#### Issue: "error: no such subcommand: `llvm-cov`"

**Cause**: `cargo-llvm-cov` is not installed.

**Solution**:
```bash
cargo install cargo-llvm-cov
```

Verify installation:
```bash
cargo llvm-cov --version
```

#### Issue: "error: coverage rate is below 70.0%"

**Cause**: Test coverage is below the required threshold.

**Solution**:
1. Generate HTML report to see gaps:
   ```bash
   make coverage-html
   ```
2. Identify files with low coverage
3. Write additional tests
4. Re-run `make coverage`

#### Issue: Coverage data appears stale or incorrect

**Cause**: Cached coverage data from previous runs.

**Solution**:
```bash
make coverage-clean
make coverage-report
```

This removes old coverage data and regenerates it.

#### Issue: "error: linking with `cc` failed" during coverage

**Cause**: Instrumentation conflicts or corrupted build artifacts.

**Solution**:
```bash
cargo clean
make coverage-clean
make coverage
```

#### Issue: HTML report doesn't open automatically

**Cause**: No default browser set or browser blocked by system.

**Solution**:
Manually open the report:
```bash
open target/llvm-cov/html/index.html          # macOS
xdg-open target/llvm-cov/html/index.html      # Linux
start target/llvm-cov/html/index.html         # Windows
```

#### Issue: Coverage is 0% or very low unexpectedly

**Cause**: Tests aren't actually running or coverage instrumentation failed.

**Solution**:
1. Verify tests run successfully:
   ```bash
   make test
   ```
2. Clean and rebuild:
   ```bash
   cargo clean
   make coverage-clean
   make coverage
   ```
3. Check for compilation errors in test code

#### Issue: "error: package excludes itself from coverage"

**Cause**: Workspace configuration or profile settings exclude certain packages.

**Solution**:
Check `Cargo.toml` for coverage exclusions. Remove or adjust:
```toml
[package.metadata.coverage]
exclude = [...]  # Remove if present
```

#### Issue: Slow coverage generation

**Cause**: Coverage instrumentation adds overhead to test execution.

**Solution**:
- Use `make coverage-report` for quick checks (no HTML generation)
- Only generate HTML when needed: `make coverage-html`
- Consider excluding slow integration tests from coverage:
  ```bash
  cargo llvm-cov --all-features --lib --tests
  ```

#### Issue: LCOV file is empty or malformed

**Cause**: Coverage ran but no tests executed, or generation failed.

**Solution**:
1. Verify tests run:
   ```bash
   make test
   ```
2. Regenerate LCOV:
   ```bash
   make coverage-clean
   make coverage-lcov
   ```
3. Check file contents:
   ```bash
   head -20 target/llvm-cov/lcov.info
   ```

#### Issue: Different coverage numbers between HTML and terminal

**Cause**: Different commands or options between `coverage-html` and `coverage-report`.

**Solution**:
Both commands use the same flags (`--all-features --workspace`), so numbers should match. If they don't:
1. Clean coverage data:
   ```bash
   make coverage-clean
   ```
2. Regenerate both:
   ```bash
   make coverage-report
   make coverage-html
   ```

### Getting Help

If you encounter issues not covered here:

1. **Check cargo-llvm-cov documentation**:
   ```bash
   cargo llvm-cov --help
   ```

2. **Verify your Rust installation**:
   ```bash
   rustc --version
   cargo --version
   ```

3. **Check for known issues**:
   - [cargo-llvm-cov GitHub issues](https://github.com/taiki-e/cargo-llvm-cov/issues)

4. **Debug with verbose output**:
   ```bash
   cargo llvm-cov --all-features --workspace --verbose
   ```

### Platform-Specific Notes

#### macOS
- Works with both Intel and Apple Silicon
- Xcode command line tools must be installed
- If you encounter linker errors, install Xcode CLT:
  ```bash
  xcode-select --install
  ```

#### Linux
- Requires `gcc` or `clang`
- On minimal distributions, install build essentials:
  ```bash
  # Debian/Ubuntu
  sudo apt-get install build-essential
  
  # Fedora/RHEL
  sudo dnf install gcc
  ```

#### Windows
- Requires Visual Studio Build Tools or MSVC
- Use PowerShell or Git Bash for running Make commands
- Consider using WSL2 for a Linux-like environment

## Best Practices

### Pre-Commit Workflow

Before committing any changes, run the full validation suite:

```bash
make build        # Ensure code compiles
make lint         # Fix style issues
make test         # Verify tests pass
make coverage     # Verify coverage meets 70% threshold
```

See `AGENTS.md` for the complete pre-commit requirements.

### Coverage-Driven Development

1. **Write tests first** (TDD) or alongside code
2. **Check coverage early**: Run `make coverage-report` frequently
3. **Use HTML reports**: Identify gaps visually with `make coverage-html`
4. **Focus on critical paths**: Ensure error handling and edge cases are covered
5. **Don't chase 100%**: Aim for meaningful coverage, not perfect coverage

### Maintaining Coverage

- **Review coverage in PRs**: Ensure new code maintains the 70% threshold
- **Monitor trends**: Track coverage over time
- **Update tests with code**: When modifying functions, update their tests
- **Test edge cases**: Not just happy paths

### Coverage Anti-Patterns

**❌ Don't**: Write tests just to increase coverage numbers
**✓ Do**: Write tests that verify correct behavior

**❌ Don't**: Ignore low-coverage files because they're "not important"
**✓ Do**: Evaluate each file's criticality and test accordingly

**❌ Don't**: Aim for 100% coverage everywhere
**✓ Do**: Focus on critical business logic and error paths

**❌ Don't**: Skip coverage checks locally, relying only on CI
**✓ Do**: Run `make coverage` before pushing

## Additional Resources

- **Project Documentation**:
  - `AGENTS.md`: Pre-commit workflow and coverage requirements
  - `docs/GITLAB_CI_SETUP.md`: CI/CD pipeline setup
  - `docs/GITLAB_CI_EXAMPLES.md`: Coverage integration examples

- **External Resources**:
  - [cargo-llvm-cov GitHub](https://github.com/taiki-e/cargo-llvm-cov)
  - [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
  - [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
