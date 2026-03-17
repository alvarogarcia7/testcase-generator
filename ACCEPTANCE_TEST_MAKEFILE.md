# Acceptance Test Makefile Integration

This document describes the `make acceptance-test` target implementation for the YAML-based test harness project.

## Overview

The `make acceptance-test` target provides a comprehensive, automated way to run the full acceptance test suite with proper environment validation, binary building, and result reporting.

## Features

The `acceptance-test` target implements all requested functionality:

### 1. Binary Building
- ✅ Builds `test-executor` binary
- ✅ Builds `verifier` binary  
- ✅ Builds `validate-yaml` binary
- ✅ Separate `build-acceptance-binaries` target for modularity
- ✅ Progress messages during build

### 2. TPDG Validation
- ✅ Validates test-plan-documentation-generator (TPDG) availability
- ✅ Checks both PATH and TEST_PLAN_DOC_GEN environment variable
- ✅ Provides helpful error message if TPDG not found
- ✅ Shows clear installation instructions
- ✅ Displays which TPDG binary will be used

### 3. Environment Setup
- ✅ Respects TEST_PLAN_DOC_GEN environment variable
- ✅ Passes environment through to acceptance suite script
- ✅ Creates necessary directories (reports/)
- ✅ Validates all prerequisites before execution

### 4. Output Capture
- ✅ Captures output to both console and log file using `tee`
- ✅ Log file: `test-acceptance/reports/acceptance_suite_execution.log`
- ✅ Real-time console output during execution
- ✅ Complete execution history preserved in log

### 5. Report Generation
- ✅ Generates final acceptance test summary report
- ✅ Summary report: `test-acceptance/reports/acceptance_suite_summary.txt`
- ✅ Report includes all stage results
- ✅ Lists failed tests by stage
- ✅ Shows overall pass/fail statistics

### 6. Summary Statistics
- ✅ Displays statistics on completion
- ✅ Shows results for all 6 stages:
  - YAML validation
  - Script generation
  - Test execution
  - Verification
  - Container validation
  - Documentation generation
- ✅ Reports total passed/failed counts
- ✅ Shows file paths for detailed results

### 7. Exit Code Handling
- ✅ Exit code 0 for all tests passing
- ✅ Exit code 1 if any failures occur
- ✅ Proper error propagation from acceptance suite script
- ✅ Clear success/failure messaging

### 8. CI/CD Integration
- ✅ Added to `pre-commit` target
- ✅ Runs as part of standard pre-commit checks
- ✅ Ensures comprehensive validation before commit
- ✅ Integrated with existing test, clippy, coverage targets

## Usage

### Basic Usage

```bash
make acceptance-test
```

This command:
1. Builds all required binaries
2. Validates TPDG availability
3. Runs the acceptance suite
4. Captures output to log file
5. Displays summary statistics
6. Exits with appropriate code

### Prerequisites

**Required**: test-plan-documentation-generator (TPDG) must be installed

Install globally:
```bash
cargo install test-plan-documentation-generator
```

Or set environment variable:
```bash
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator
make acceptance-test
```

### Output Files

After running `make acceptance-test`, the following files are generated:

- **Execution log**: `test-acceptance/reports/acceptance_suite_execution.log`
  - Complete console output from the test run
  - Includes all stage outputs and error messages
  
- **Summary report**: `test-acceptance/reports/acceptance_suite_summary.txt`
  - Statistical summary of all stages
  - List of failed tests (if any)
  - Overall pass/fail status
  
- **Generated scripts**: `test-acceptance/scripts/*.sh`
  - Executable bash scripts generated from test cases
  
- **Execution logs**: `test-acceptance/execution_logs/*.json`
  - JSON execution logs for each test
  
- **Verification results**: `test-acceptance/verification_results/*_container.yaml`
  - Container YAML files from verification stage
  
- **Documentation**: 
  - `test-acceptance/reports/asciidoc/*.adoc`
  - `test-acceptance/reports/markdown/*.md`

## Implementation Details

### Makefile Targets

#### `acceptance-test`
Main target that orchestrates the entire acceptance test suite.

**Dependencies**: `build-acceptance-binaries`

**Steps**:
1. Display header message
2. Create reports directory
3. Validate TPDG availability (fail with helpful message if missing)
4. Run acceptance suite script with output capture
5. Display success/failure message with file locations
6. Exit with appropriate code

#### `build-acceptance-binaries`
Builds all binaries required for acceptance tests.

**Builds**:
- `test-executor` - Generates bash scripts from test cases
- `verifier` - Verifies execution logs and generates container YAMLs
- `validate-yaml` - Validates YAML files against schemas

### Error Handling

The target provides comprehensive error handling:

1. **Missing TPDG**: Clear error message with installation instructions
2. **Build failures**: Cargo build errors are displayed
3. **Test failures**: Exit code 1 with detailed failure summary
4. **Log capture**: All output preserved for post-mortem analysis

### Success Output Example

```
=========================================
Running Acceptance Test Suite
=========================================

Execution log: test-acceptance/reports/acceptance_suite_execution.log

Using TPDG from PATH: /usr/local/bin/test-plan-documentation-generator

[... test execution output ...]

=========================================
Acceptance Test Suite: SUCCESS
=========================================

Full execution log: test-acceptance/reports/acceptance_suite_execution.log
Summary report: test-acceptance/reports/acceptance_suite_summary.txt
```

### Failure Output Example

```
=========================================
Running Acceptance Test Suite
=========================================

Execution log: test-acceptance/reports/acceptance_suite_execution.log

ERROR: test-plan-documentation-generator (TPDG) not found

TPDG is required for acceptance tests.

Install options:
  1. Install globally:
     cargo install test-plan-documentation-generator

  2. Set TEST_PLAN_DOC_GEN environment variable:
     export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator

make: *** [acceptance-test] Error 1
```

## Integration with Pre-Commit

The `acceptance-test` target is now part of the `pre-commit` target:

```makefile
pre-commit: test clippy coverage acceptance-test README_INSTALL_AUTOMATED.md
```

This ensures that before any commit:
1. All unit tests pass
2. All E2E tests pass
3. Code passes clippy linting
4. Code meets coverage thresholds
5. **Acceptance tests pass**
6. Documentation is generated

## Advanced Usage

### Running with Custom TPDG Path

```bash
TEST_PLAN_DOC_GEN=/custom/path/to/tpdg make acceptance-test
```

### Debugging Test Failures

If acceptance tests fail:

1. Check the execution log:
   ```bash
   cat test-acceptance/reports/acceptance_suite_execution.log
   ```

2. Review the summary report:
   ```bash
   cat test-acceptance/reports/acceptance_suite_summary.txt
   ```

3. Run the acceptance suite directly for more control:
   ```bash
   cd test-acceptance
   ./run_acceptance_suite.sh --verbose
   ```

## CI/CD Pipeline Integration

In CI/CD pipelines, ensure TPDG is installed before running the pre-commit target:

```bash
# Install TPDG
cargo install test-plan-documentation-generator

# Run pre-commit checks (includes acceptance tests)
make pre-commit
```

Or use a custom TPDG path:

```bash
# Build TPDG from source
cd /path/to/test-plan-documentation-generator
cargo build --release

# Run acceptance tests with custom path
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator/target/release/test-plan-documentation-generator
make acceptance-test
```

## Files Modified

The following files were modified to implement the `make acceptance-test` target:

1. **Makefile**:
   - Added `acceptance-test` target
   - Added `build-acceptance-binaries` target
   - Updated `pre-commit` to include `acceptance-test`

2. **AGENTS.md**:
   - Added "Acceptance Tests" to Commands section
   - Added comprehensive "Acceptance Test Suite" section
   - Updated pre-commit requirements

3. **test-acceptance/README.md**:
   - Added make target as recommended option in Quick Start
   - Updated Automated Test Suite section
   - Added output file locations

4. **ACCEPTANCE_TEST_MAKEFILE.md** (this file):
   - Complete implementation documentation

## Testing the Implementation

To verify the implementation works correctly:

1. **With TPDG installed**:
   ```bash
   make acceptance-test
   ```
   Should complete successfully and generate reports.

2. **Without TPDG**:
   ```bash
   # Temporarily hide TPDG
   PATH_BACKUP=$PATH
   export PATH=/usr/bin:/bin
   unset TEST_PLAN_DOC_GEN
   
   make acceptance-test
   # Should fail with helpful error message
   
   # Restore PATH
   export PATH=$PATH_BACKUP
   ```

3. **With custom TPDG path**:
   ```bash
   export TEST_PLAN_DOC_GEN=/custom/path/to/tpdg
   make acceptance-test
   ```
   Should use the specified TPDG binary.

## Benefits

The `make acceptance-test` target provides:

1. **Consistency**: Standardized way to run acceptance tests
2. **Convenience**: Single command for complete validation
3. **Visibility**: Clear output and logging for debugging
4. **Safety**: Validates prerequisites before execution
5. **Integration**: Part of pre-commit workflow for CI/CD
6. **Documentation**: Self-documenting through clear messages
7. **Reliability**: Comprehensive error handling and reporting

## Related Documentation

- **test-acceptance/ACCEPTANCE_SUITE.md**: Detailed orchestrator documentation
- **test-acceptance/README.md**: Test acceptance directory overview
- **AGENTS.md**: Project commands and workflows
- **Makefile**: Build system configuration
