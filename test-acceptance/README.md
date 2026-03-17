# Test Acceptance Directory

This directory contains acceptance test cases for the YAML-based test harness project.

## Quick Start

Run the complete acceptance test suite:

```bash
cd test-acceptance
./run_acceptance_suite.sh
```

See [ACCEPTANCE_SUITE.md](ACCEPTANCE_SUITE.md) for detailed documentation.

## Purpose

The test cases in this directory serve multiple purposes:

1. **Validation**: Verify that the test harness correctly handles various test scenarios
2. **Documentation**: Demonstrate features and best practices through examples
3. **Regression Testing**: Ensure new changes don't break existing functionality
4. **Feature Coverage**: Provide comprehensive coverage of all harness capabilities

## Directory Structure

```
test-acceptance/
├── test_cases/              # Test case YAML files
│   ├── success/            # Success scenario test cases
│   ├── failure/            # Failure scenario test cases
│   ├── variables/          # Variable capture and usage tests
│   ├── hooks/              # Lifecycle hooks tests
│   ├── dependencies/       # Dependency management tests
│   ├── bash_commands/      # Bash command tests
│   ├── complex/            # Complex integration tests
│   ├── manual/             # Manual test cases
│   └── README.md           # Test case documentation
├── scripts/                 # Generated executable bash scripts (gitignored)
├── execution_logs/          # JSON execution logs (gitignored)
├── verification_results/    # Container YAMLs (gitignored)
├── reports/                 # Generated documentation (gitignored)
├── run_acceptance_suite.sh  # Master orchestrator script
├── ACCEPTANCE_SUITE.md      # Orchestrator documentation
├── IMPLEMENTATION_SUMMARY.md  # Implementation details
└── README.md               # This file
```

## Test Categories

### Success Scenarios (`test_cases/success/`)

Contains 13 comprehensive test cases that demonstrate successful test execution scenarios:

- **TC_SUCCESS_SIMPLE_001**: Simple single-sequence test (3 steps)
- **TC_SUCCESS_MULTI_SEQ_001**: Multi-sequence test (3 sequences, 2-4 steps each)
- **TC_SUCCESS_VAR_CAPTURE_001**: Variable capture and usage
- **TC_SUCCESS_REGEX_VALIDATION_001**: Output validation with regex
- **TC_SUCCESS_ENV_VARS_001**: Environment variable usage
- **TC_SUCCESS_CMD_CHAIN_001**: Command chaining with &&
- **TC_SUCCESS_STEP_DEPS_001**: Step dependencies using captured variables
- **TC_SUCCESS_LONG_RUNNING_001**: Long-running commands
- **TC_SUCCESS_EMPTY_OUTPUT_001**: Empty output validation
- **TC_SUCCESS_CONDITIONAL_001**: Complex conditional verification logic
- **TC_SUCCESS_COMPLEX_DATA_001**: Complex data processing
- **TC_SUCCESS_FILE_OPS_001**: Advanced file operations
- **TC_SUCCESS_TEXT_PROCESSING_001**: Advanced text processing

See `test_cases/README.md` for detailed documentation of each test case.

## Automated Test Suite

The master orchestrator script automates the complete testing workflow:

### Run Complete Suite

```bash
cd test-acceptance
./run_acceptance_suite.sh
```

This executes all 6 stages:
1. Validate test case YAMLs
2. Generate bash scripts
3. Execute automated tests
4. Verify execution logs
5. Validate container YAMLs
6. Generate documentation

### Common Options

```bash
# Verbose output for debugging
./run_acceptance_suite.sh --verbose

# Include manual tests
./run_acceptance_suite.sh --include-manual

# Skip expensive stages
./run_acceptance_suite.sh --skip-execution --skip-documentation

# Regenerate documentation only
./run_acceptance_suite.sh --skip-generation --skip-execution
```

See [ACCEPTANCE_SUITE.md](ACCEPTANCE_SUITE.md) for complete documentation.

## Manual Testing (Individual Tests)

### Validate a Test Case

```bash
# Validate YAML schema compliance
cargo run --bin verifier -- test-acceptance/test_cases/success/TC_SUCCESS_SIMPLE_001.yaml
```

### Generate Test Script

```bash
# Generate executable bash script from test case
cargo run -- test-acceptance/test_cases/success/TC_SUCCESS_SIMPLE_001.yaml
```

### Execute Test

```bash
# Run the generated test script
./test-acceptance/test_cases/success/TC_SUCCESS_SIMPLE_001.sh
```

## Feature Coverage

The test cases cover all major features:

- ✅ Single and multi-sequence tests
- ✅ Variable capture (regex and command-based)
- ✅ Step dependencies
- ✅ Environment variables with hydration
- ✅ Command chaining
- ✅ Regex output validation
- ✅ Conditional verification (if/then/else)
- ✅ Empty output handling
- ✅ Long-running operations
- ✅ Complex data processing
- ✅ File operations and metadata
- ✅ Text processing with sed/awk/grep

## Statistics

- **Total test cases**: 13
- **Total YAML lines**: 1,581
- **Total sequences**: 26
- **Total steps**: 78
- **Average steps per sequence**: 3.0

## Cross-Platform Compatibility

All test cases are designed to work on:
- macOS (BSD utilities, bash 3.2+)
- Linux (GNU utilities, bash 3.2+)

Test cases use:
- Portable bash syntax
- BSD/GNU compatible command options
- POSIX-compliant regex patterns

## Documentation

- **test_cases/README.md**: Detailed test case documentation
- **IMPLEMENTATION_SUMMARY.md**: Implementation details and statistics

## Contributing

When adding new test cases:

1. Follow the schema defined in `schemas/test-case.schema.json`
2. Ensure cross-platform compatibility (macOS/Linux)
3. Use portable bash 3.2+ syntax
4. Include comprehensive verification expressions
5. Document the test case purpose and features
6. Test on both macOS and Linux if possible

## Schema Validation

All test cases must conform to the JSON schema:

```bash
# Schema location
schemas/test-case.schema.json

# Validation is performed automatically by the verifier binary
cargo run --bin verifier -- <test-case.yaml>
```

## Test Execution Workflow

1. **Parse**: YAML test case is parsed and validated
2. **Generate**: Bash script is generated from test case
3. **Execute**: Script runs with step-by-step validation
4. **Verify**: Output and exit codes are verified
5. **Report**: Results are logged and reported

## Future Additions

Potential future test categories:

- `failure/` - Failure scenario test cases
- `edge_cases/` - Edge case and boundary condition tests
- `integration/` - Integration tests with external systems
- `performance/` - Performance and load testing scenarios

## Support

For questions or issues with test cases:
1. Check the test case documentation in `test_cases/README.md`
2. Review the implementation summary in `IMPLEMENTATION_SUMMARY.md`
3. Refer to the main project documentation
4. Check the schema definition in `schemas/test-case.schema.json`
