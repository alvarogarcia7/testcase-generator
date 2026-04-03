# Pipeline Batch Processor

## Overview

The Pipeline Batch Processor is a comprehensive testing tool that applies a 5-stage validation pipeline to all test case YAML files in the project. It provides automated end-to-end validation of test cases from YAML schema validation through script generation, execution, verification, and result reporting.

## 5-Stage Pipeline

The batch processor applies the following stages to each test case:

### Stage 1: YAML Test Case Validation
- Validates test case YAML against the schema (`test-case.schema.json`)
- Ensures structural correctness and required fields
- **Success Criteria**: YAML is well-formed and passes schema validation

### Stage 2: Script Generation and Validation
- Generates executable shell script from the test case YAML
- Validates bash syntax using `bash -n`
- Runs shellcheck for quality validation (warnings don't fail)
- **Success Criteria**: Script generates successfully and passes syntax validation

### Stage 3: Script Execution and JSON Validation
- Executes the generated test script with 60-second timeout
- Validates that execution produces JSON output
- Checks that at least one test step was recorded
- **Success Criteria**: Script executes and produces well-formed JSON with recorded steps

### Stage 4: Verification with YAML Output
- Runs verifier against execution JSON and original test case
- Generates YAML verification report
- Validates verification YAML against schema (`verification-result.schema.json`)
- **Success Criteria**: Verification report is generated and validates against schema

### Stage 5: Result Summary Generation
- Generates JSON result summary from verification
- Validates JSON structure and required fields
- Determines overall test pass/fail status
- **Success Criteria**: Result JSON is well-formed and contains required fields

## Usage

### Running the Batch Processor

```bash
# Run with default settings (processes all YAMLs in testcases/ directory)
make test-e2e-pipeline-batch

# Or run directly
./crates/testcase-manager/tests/integration/test_pipeline_batch.sh

# Preserve temporary files for inspection
./crates/testcase-manager/tests/integration/test_pipeline_batch.sh --no-remove

# Process YAMLs from a custom directory
./crates/testcase-manager/tests/integration/test_pipeline_batch.sh --testcases-dir /path/to/testcases
```

### Running the Single Pipeline E2E Test

```bash
# Run the original single test case pipeline validation
make test-e2e-pipeline

# Or run directly
./crates/testcase-manager/tests/integration/test_pipeline_e2e.sh
```

## Output and Reporting

### Summary Report

The batch processor generates a comprehensive summary report including:

#### Stage Results
For each of the 5 stages:
- Total number of test cases processed
- Number of successes
- Number of failures
- Success rate percentage

Example:
```
Stage 1: YAML Test Case Validation
  Total:   123
  Success: 120
  Failure: 3
  Success Rate: 97%
```

#### Test Case Results
Overall test case outcomes:
- Total test cases discovered
- Number that completed full pipeline
- Test verification pass count
- Test verification fail count
- Test pass rate percentage

#### Detailed Failure List
Lists all failed test cases with the stage where they failed:
```
Failed Test Cases:
  - invalid_test.yml (failed at: Stage 1)
  - syntax_error.yml (failed at: Stage 2)
  - timeout_test.yml (failed at: Stage 3)
```

#### Detailed Pass List
Lists all test cases that passed through all stages.

### Output Directory Structure

When run with `--no-remove`, temporary files are preserved in a directory with the following structure:

```
/tmp/tmp.XXXXXXXXXX/
├── logs/              # Stage execution logs for each test case
│   ├── TEST_001_stage1.log
│   ├── TEST_001_stage2.log
│   ├── TEST_001_stage3.log
│   ├── TEST_001_stage4.log
│   └── TEST_001_stage5.log
├── scripts/           # Generated test scripts
│   └── TEST_001_test.sh
├── executions/        # Execution JSON outputs
│   └── TEST_001_execution.json
├── verifications/     # Verification YAML outputs
│   └── TEST_001_verification.yaml
└── results/           # Result JSON outputs
    └── TEST_001_result.json
```

## Prerequisites

The batch processor requires the following tools:
- **bash 3.2+** (compatible with macOS default bash and modern Linux)
- `test-executor` binary (from workspace)
- `validate-yaml` binary (from workspace)
- `verifier` binary (from workspace)
- `jq` (for JSON processing)
- `timeout` command (optional, for execution timeouts - not available on macOS by default)
- `shellcheck` (optional, for enhanced script validation)

## Integration with CI/CD

The pipeline batch processor is integrated into the comprehensive test suite:

```bash
# Run all E2E tests including pipeline tests
make test-e2e-all

# Run full test suite (unit + E2E + pipeline)
make test-all
```

The `test_pipeline_e2e.sh` script is automatically included in `test-e2e-all-no-build` target for comprehensive testing.

## Performance Considerations

- **Timeout**: Each test case execution has a 60-second timeout to prevent hanging (when `timeout` command is available)
  - On systems without `timeout` (e.g., macOS), scripts run without timeout limit
- **Parallel Processing**: Currently sequential; could be parallelized in the future
- **Resource Usage**: Temporary files are created for each test case (cleaned up by default)
- **Expected Duration**: Approximately 5-30 seconds per test case depending on complexity

For 123 test cases, expect approximately 10-60 minutes of total processing time.

## Compatibility Notes

- **Bash 3.2+**: The script is compatible with bash 3.2, which is the default on macOS
- **No mapfile**: Uses bash 3.2 compatible array population methods
- **No associative arrays**: Uses individual variables instead of associative arrays for bash 3.2 compatibility
- **Timeout command**: Gracefully handles absence of `timeout` command on macOS and other systems

## Exit Codes

- `0`: All stages completed successfully
- `1`: One or more prerequisites missing or failures occurred

Note: Test case failures (Stage 4-5) don't cause script exit; the summary report shows pass/fail status for each test.

## Examples

### Example: Processing Small Test Set

```bash
# Process only BDD examples
./crates/testcase-manager/tests/integration/test_pipeline_batch.sh \
  --testcases-dir testcases/bdd_examples \
  --no-remove

# Output preserved in /tmp/tmp.XXXXXXXXXX/
```

### Example: CI/CD Integration

```yaml
# GitLab CI example
test:pipeline:batch:
  stage: test
  script:
    - make build-all
    - ./crates/testcase-manager/tests/integration/test_pipeline_batch.sh
  artifacts:
    when: always
    paths:
      - pipeline_summary.log
```

## Troubleshooting

### Common Issues

**Issue**: "No YAML files found"
- **Solution**: Check that `--testcases-dir` points to correct directory
- Verify YAML files have `.yml` or `.yaml` extension

**Issue**: Stage 1 failures for many files
- **Solution**: Run `make verify-testcases` to validate schemas
- Check schema path is correct: `schemas/test-case.schema.json`

**Issue**: Stage 3 timeouts
- **Solution**: Test cases with long-running commands may timeout
- Consider increasing timeout in the script (default: 60 seconds)

**Issue**: Manual steps blocking execution
- **Solution**: Manual steps are automatically handled in non-interactive mode
- Set `DEBIAN_FRONTEND=noninteractive` environment variable

### Debug Mode

To debug specific test case failures:

```bash
# Run with --no-remove to preserve output
./crates/testcase-manager/tests/integration/test_pipeline_batch.sh --no-remove

# Inspect logs for specific test case
cat /tmp/tmp.XXXXXXXXXX/logs/TEST_001_stage3.log

# Check generated script
cat /tmp/tmp.XXXXXXXXXX/scripts/TEST_001_test.sh
```

## Related Documentation

- [Test Pipeline E2E Test](crates/testcase-manager/tests/integration/test_pipeline_e2e.sh) - Single test case pipeline validation
- [Test Executor Documentation](README.md#test-executor) - Script generation details
- [Verifier Documentation](README.md#verifier) - Verification and reporting
- [YAML Validation](README_VALIDATION.md) - Schema validation details

## Future Enhancements

Potential improvements for the batch processor:

1. **Parallel Execution**: Process multiple test cases concurrently
2. **Selective Processing**: Filter test cases by pattern or metadata
3. **HTML Report Generation**: Create visual dashboard of results
4. **Trend Analysis**: Track success rates over time
5. **Performance Metrics**: Measure execution time per stage
6. **Failure Analysis**: Automatic categorization of failure types
7. **Partial Retry**: Re-run only failed test cases
8. **Watch Mode**: Continuously monitor and reprocess changed files
