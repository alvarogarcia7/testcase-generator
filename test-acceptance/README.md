# Test Acceptance Framework

## Overview

The test-acceptance framework provides a comprehensive acceptance testing environment for the YAML-based test harness. It validates end-to-end functionality by executing complete workflows from test case definition through script generation, execution, verification, and documentation generation.

## Purpose

This acceptance testing framework serves to:

1. **Validate Complete Workflows**: Test the entire pipeline from YAML test case definition to final documentation generation
2. **Verify Feature Integration**: Ensure all features (variables, hooks, prerequisites, conditionals, etc.) work correctly together
3. **Catch Regressions**: Detect breaking changes across the full system
4. **Document Usage Patterns**: Provide real-world examples of test case configurations
5. **Test Edge Cases**: Validate behavior with complex scenarios and error conditions

## Directory Structure

```
test-acceptance/
├── test_cases/          # Organized YAML test case definitions
│   ├── success/         # Test cases designed to pass
│   ├── failure/         # Test cases designed to fail
│   ├── hooks/           # Test cases demonstrating hook functionality
│   ├── manual/          # Test cases with manual prerequisites
│   ├── variables/       # Test cases focusing on variable capture/usage
│   ├── dependencies/    # Test cases with dependency management
│   ├── prerequisites/   # Test cases with prerequisite verification
│   └── complex/         # Complex multi-feature test cases
├── scripts/             # Generated bash test scripts
├── execution_logs/      # JSON execution logs from test runs
├── verification_results/ # Container YAML files for verification
├── reports/             # Generated documentation (AsciiDoc, Markdown, HTML)
└── README.md            # This file
```

## Test Case Categories

### success/
Test cases that are designed to execute successfully, demonstrating correct functionality:
- Basic command execution
- Variable capture and substitution
- Conditional verification
- Hook execution
- Prerequisite satisfaction
- Multi-sequence scenarios

### failure/
Test cases that intentionally fail to validate error handling:
- Command failures
- Failed assertions
- Verification failures
- Expected errors in various scenarios

### hooks/
Test cases demonstrating the lifecycle hook system:
- Script start/end hooks
- Test setup/teardown hooks
- Sequence before/after hooks
- Step before/after hooks
- Hook error handling (fail vs continue)
- Hook context access

### manual/
Test cases requiring manual intervention:
- Manual prerequisites with instructions
- Interactive verification steps
- Environment setup requiring user action

### variables/
Test cases focusing on variable functionality:
- Regex-based variable capture
- Command-based variable capture
- Variable substitution in commands
- Variable scoping (global vs sequence)
- Complex variable transformations

### dependencies/
Test cases demonstrating dependency management:
- Package dependencies
- Tool version requirements
- External service dependencies

### prerequisites/
Test cases with prerequisite verification:
- Automatic prerequisites with verification commands
- Prerequisite failure handling
- Mixed automatic and manual prerequisites

### complex/
Advanced test cases combining multiple features:
- Multi-sequence workflows
- Complex conditional logic
- Extensive variable capture chains
- Full lifecycle hooks with resource management
- Integration scenarios

## Workflow

### 1. Test Case Definition
Create or modify YAML test case files in the appropriate category directory:

```yaml
# test-acceptance/test_cases/success/basic_echo.yaml
test_name: "Basic Echo Test"
test_id: "ACC_ECHO_001"
test_description: "Verify basic command execution"

prerequisites:
  automatic: []
  manual: []

sequences:
  - sequence_id: "1"
    sequence_name: "Echo Test"
    steps:
      - step_number: 1
        description: "Echo a message"
        command: "echo 'Hello World'"
        expected_output: "Hello World"
```

### 2. Script Generation
Generate bash scripts from test case YAML files using the verifier:

```bash
# Generate script for a single test case
cargo run --bin verifier -- \
  test-acceptance/test_cases/success/basic_echo.yaml \
  test-acceptance/scripts/basic_echo.sh

# Generate scripts for all test cases in a category
for testcase in test-acceptance/test_cases/success/*.yaml; do
  script_name=$(basename "$testcase" .yaml).sh
  cargo run --bin verifier -- \
    "$testcase" \
    "test-acceptance/scripts/$script_name"
done
```

### 3. Test Execution
Execute generated bash scripts and capture execution logs:

```bash
# Execute with JSON logging
test-acceptance/scripts/basic_echo.sh \
  > test-acceptance/execution_logs/basic_echo.json 2>&1
```

### 4. Verification
Convert execution logs to container YAML format for verification:

```bash
# Convert JSON log to container YAML
python3 scripts/convert_verification_to_result_yaml.py \
  test-acceptance/test_cases/success/basic_echo.yaml \
  test-acceptance/execution_logs/basic_echo.json \
  test-acceptance/verification_results/basic_echo_container.yaml
```

### 5. Documentation Generation
Generate comprehensive documentation reports:

```bash
# Generate reports using test-plan-documentation-generator
test-plan-documentation-generator generate \
  --test-case test-acceptance/test_cases/success/basic_echo.yaml \
  --container test-acceptance/verification_results/basic_echo_container.yaml \
  --output-dir test-acceptance/reports/basic_echo \
  --formats adoc,markdown,html
```

## Running Acceptance Tests

### Full Pipeline for a Single Test

```bash
#!/usr/bin/env bash
set -e

TESTCASE="test-acceptance/test_cases/success/basic_echo.yaml"
SCRIPT="test-acceptance/scripts/basic_echo.sh"
LOG="test-acceptance/execution_logs/basic_echo.json"
CONTAINER="test-acceptance/verification_results/basic_echo_container.yaml"
REPORT_DIR="test-acceptance/reports/basic_echo"

# 1. Generate script
cargo run --bin verifier -- "$TESTCASE" "$SCRIPT"

# 2. Execute script
"$SCRIPT" > "$LOG" 2>&1

# 3. Convert to container YAML
python3 scripts/convert_verification_to_result_yaml.py \
  "$TESTCASE" "$LOG" "$CONTAINER"

# 4. Generate documentation
test-plan-documentation-generator generate \
  --test-case "$TESTCASE" \
  --container "$CONTAINER" \
  --output-dir "$REPORT_DIR" \
  --formats adoc,markdown,html

echo "Acceptance test complete. Reports available in $REPORT_DIR"
```

### Batch Execution

```bash
# Run all test cases in a category
for testcase in test-acceptance/test_cases/success/*.yaml; do
  test_name=$(basename "$testcase" .yaml)
  echo "Running acceptance test: $test_name"
  
  # Run the full pipeline (script generation, execution, verification, documentation)
  # ... (see single test example above)
done
```

## Best Practices

### Test Case Design

1. **Single Focus**: Each test case should focus on one primary feature or scenario
2. **Clear Naming**: Use descriptive names that indicate the test's purpose
3. **Unique IDs**: Assign unique test IDs following the pattern `ACC_<CATEGORY>_<NUMBER>`
4. **Documentation**: Include clear descriptions and expected outcomes
5. **Isolation**: Test cases should be independent and not rely on side effects from other tests

### Hook Scripts

1. **Location**: Store hook scripts in a `scripts/` subdirectory within the test case directory
2. **Logging**: Use the centralized logging library (`scripts/lib/logger.sh`)
3. **Error Handling**: Explicitly set `on_error` behavior for each hook
4. **Cleanup**: Ensure proper cleanup in teardown hooks, even on failure
5. **Portability**: Follow bash 3.2+ and BSD/GNU compatibility requirements

### Directory Organization

1. **Category Separation**: Keep test cases organized by category
2. **Naming Conventions**: Use consistent naming across test cases, scripts, and logs
3. **Version Control**: Commit test cases and documentation, exclude generated artifacts
4. **Cleanup**: Periodically clean generated scripts, logs, and reports during development

## Validation

### Schema Validation

Validate test case YAML against the schema:

```bash
# Validate a single test case
cargo run --bin verifier -- \
  test-acceptance/test_cases/success/basic_echo.yaml \
  --validate-only

# Validate all test cases
find test-acceptance/test_cases -name "*.yaml" -type f | while read testcase; do
  echo "Validating: $testcase"
  cargo run --bin verifier -- "$testcase" --validate-only
done
```

### Container Compatibility

Verify container YAML compatibility with test-plan-documentation-generator:

```bash
# Check single container
cargo run --bin test-plan-documentation-generator-compat -- \
  validate test-acceptance/verification_results/basic_echo_container.yaml

# Check all containers
cargo run --bin test-plan-documentation-generator-compat -- \
  batch test-acceptance/verification_results/
```

## Integration with CI/CD

The acceptance test framework can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Acceptance Tests
  run: |
    make build
    ./scripts/run_acceptance_tests.sh
    
- name: Upload Reports
  uses: actions/upload-artifact@v3
  with:
    name: acceptance-test-reports
    path: test-acceptance/reports/
```

## Troubleshooting

### Common Issues

**Generated script fails to execute**
- Verify the test case YAML is valid
- Check that all commands are available in the execution environment
- Review prerequisites and ensure they are satisfied

**Container YAML validation fails**
- Ensure execution log JSON is well-formed
- Verify the conversion script completed successfully
- Check for schema compatibility issues

**Documentation generation fails**
- Verify test-plan-documentation-generator is installed and in PATH
- Check container YAML format compatibility
- Review error messages for schema violations

**Hook scripts not executing**
- Verify hook script paths are correct (relative to test case YAML)
- Ensure hook scripts have execute permissions
- Check hook script syntax with `bash -n script.sh`

## Contributing

When adding new acceptance tests:

1. Choose the appropriate category directory
2. Follow naming conventions: `<feature>_<scenario>_<variant>.yaml`
3. Use unique test IDs: `ACC_<CATEGORY>_<NUMBER>`
4. Include comprehensive test descriptions
5. Add comments explaining complex scenarios
6. Test on both macOS and Linux when possible
7. Update this README if adding new categories or patterns

## Reference Documentation

- **AGENTS.md**: Project overview, commands, and development guidelines
- **docs/report_generation.md**: Report generation with test-plan-documentation-generator
- **docs/TEST_PLAN_DOC_GEN_COMPATIBILITY.md**: Container YAML compatibility checker
- **testcases/verifier_scenarios/**: Example test cases for the verifier

## Examples

See the `test_cases/` subdirectories for examples of:
- Basic test cases (success/)
- Error scenarios (failure/)
- Hook usage patterns (hooks/)
- Variable capture techniques (variables/)
- Complex multi-feature scenarios (complex/)
