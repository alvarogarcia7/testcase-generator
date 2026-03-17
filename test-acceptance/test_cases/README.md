# Test Acceptance Test Cases

This directory contains comprehensive test cases for acceptance testing of the YAML-based test harness. The test cases are organized into subdirectories by category.

## Directory Structure

```
test-acceptance/test_cases/
├── success/           # Success scenario test cases
└── README.md         # This file
```

## Success Scenario Test Cases

The `success/` directory contains 13 comprehensive test cases demonstrating all major features of the test harness:

### 1. TC_SUCCESS_SIMPLE_001 - Simple Single-Sequence Test
- **Purpose**: Basic command execution with 3 steps
- **Features**: Single sequence, simple commands, basic output validation
- **Steps**: 
  - Print hello world message
  - Display current date with format validation
  - List temporary directory

### 2. TC_SUCCESS_MULTI_SEQ_001 - Multi-Sequence Test
- **Purpose**: Test organization with multiple sequences
- **Features**: 3 sequences with 2-4 steps each
- **Sequences**:
  1. File Creation (3 steps) - Create test files
  2. File Content Processing (4 steps) - Read and process files
  3. File Cleanup (4 steps) - Remove files and verify

### 3. TC_SUCCESS_VAR_CAPTURE_001 - Variable Capture and Usage
- **Purpose**: Capture variables from output and reuse them
- **Features**: Regex capture, variable reuse in subsequent steps
- **Steps**:
  - Generate and capture timestamp
  - Create file with captured timestamp
  - Read back and verify using captured value
  - Cleanup using captured variable

### 4. TC_SUCCESS_REGEX_VALIDATION_001 - Regex Pattern Validation
- **Purpose**: Complex pattern matching in verification
- **Features**: Multiple regex patterns for validation
- **Patterns Validated**:
  - UUID format
  - Email address format
  - IP address format
  - URL format
  - Semantic version format

### 5. TC_SUCCESS_ENV_VARS_001 - Environment Variable Usage
- **Purpose**: Use environment variables in commands
- **Features**: Hydration variables, required/optional vars
- **Variables**:
  - TEST_USER (required)
  - TEST_HOME (required)
  - TEST_PORT (optional)
- **Steps**: Create directories, config files using env vars

### 6. TC_SUCCESS_CMD_CHAIN_001 - Command Chaining with &&
- **Purpose**: Sequential command execution with dependencies
- **Features**: Multiple commands chained with && operator
- **Examples**:
  - Directory creation and file operations
  - File operations with multiple steps
  - Grep and awk pipelines
  - Directory navigation and operations

### 7. TC_SUCCESS_STEP_DEPS_001 - Step Dependencies
- **Purpose**: Steps that depend on variables from previous steps
- **Features**: Complex variable dependencies across steps
- **Flow**:
  - Generate unique ID → capture
  - Create directory using ID
  - Create files in directory
  - Read files and capture content
  - Calculate sizes
  - Cleanup using all captured variables

### 8. TC_SUCCESS_LONG_RUNNING_001 - Long-Running Commands
- **Purpose**: Handle commands that take time to execute
- **Features**: Sleep commands, timing measurements, progress updates
- **Examples**:
  - 2-second delay operations
  - Measure operation duration
  - Progress updates over time
  - Batch processing with timestamps

### 9. TC_SUCCESS_EMPTY_OUTPUT_001 - Empty Output Validation
- **Purpose**: Handle commands with no output
- **Features**: Validation of empty output, file operations without output
- **Steps**:
  - Create files silently (touch)
  - Remove files silently
  - Verify empty directories
  - Read empty files

### 10. TC_SUCCESS_CONDITIONAL_001 - Conditional Verification Logic
- **Purpose**: Complex if/then/else verification
- **Features**: Conditional verification with captured variables
- **Examples**:
  - Score-based pass/fail logic
  - Status checking with conditional actions
  - Multiple variable conditions
  - Resource thresholds with alerts
  - Environment-specific validation

### 11. TC_SUCCESS_COMPLEX_DATA_001 - Complex Data Processing
- **Purpose**: Real-world data extraction and validation
- **Features**: Multi-field captures, complex validations
- **Data Types**:
  - JSON-like API responses (8+ captured fields)
  - Server log entries (7+ captured fields)
  - Performance metrics with calculations

### 12. TC_SUCCESS_FILE_OPS_001 - Advanced File Operations
- **Purpose**: Comprehensive file system operations
- **Features**: File metadata, permissions, content manipulation
- **Sequences**:
  1. File Creation and Metadata - Create files, verify size/permissions
  2. File Content Operations - Append, search, copy files
  3. Cleanup and Verification - Remove files and verify

### 13. TC_SUCCESS_TEXT_PROCESSING_001 - Advanced Text Processing
- **Purpose**: Complex text manipulation with sed, awk, grep
- **Features**: Pattern matching, text transformation, aggregation
- **Sequences**:
  1. Pattern Matching and Extraction - awk field extraction, sed transformation, grep counting
  2. Complex Text Transformations - CSV parsing, log formatting, data aggregation

## Feature Coverage Summary

| Feature | Test Cases |
|---------|-----------|
| Single sequence (3 steps) | TC_SUCCESS_SIMPLE_001 |
| Multi-sequence (3 sequences, 2-4 steps) | TC_SUCCESS_MULTI_SEQ_001 |
| Variable capture (regex) | TC_SUCCESS_VAR_CAPTURE_001, TC_SUCCESS_REGEX_VALIDATION_001 |
| Variable capture (command) | TC_SUCCESS_STEP_DEPS_001, TC_SUCCESS_COMPLEX_DATA_001 |
| Output validation with regex | TC_SUCCESS_REGEX_VALIDATION_001 |
| Environment variable usage | TC_SUCCESS_ENV_VARS_001 |
| Command chaining with && | TC_SUCCESS_CMD_CHAIN_001 |
| Step dependencies | TC_SUCCESS_STEP_DEPS_001 |
| Long-running commands | TC_SUCCESS_LONG_RUNNING_001 |
| Empty output validation | TC_SUCCESS_EMPTY_OUTPUT_001 |
| Complex conditional verification | TC_SUCCESS_CONDITIONAL_001 |
| Complex data processing | TC_SUCCESS_COMPLEX_DATA_001 |
| File operations | TC_SUCCESS_FILE_OPS_001 |
| Text processing | TC_SUCCESS_TEXT_PROCESSING_001 |

## Running Test Cases

To validate the test cases:

```bash
# Validate a single test case
cargo run --bin verifier -- test-acceptance/test_cases/success/TC_SUCCESS_SIMPLE_001.yaml

# Validate all test cases
for file in test-acceptance/test_cases/success/*.yaml; do
  echo "Validating $file"
  cargo run --bin verifier -- "$file"
done
```

## Test Case Schema

All test cases follow the schema defined in `schemas/test-case.schema.json` and include:

- **requirement**: Feature requirement identifier
- **item**: Item number
- **tc**: Test case number
- **id**: Unique test case identifier
- **description**: Test case description
- **general_initial_conditions**: Top-level general conditions
- **initial_conditions**: Device/system specific conditions
- **hydration_vars**: Optional environment variables (in some test cases)
- **test_sequences**: Array of test sequences with steps

Each step includes:
- **step**: Step number
- **description**: Step description
- **command**: Command to execute
- **capture_vars**: Optional variables to capture (array format)
- **expected**: Expected result and output
- **verification**: Verification expressions (result, output, general)

## Notes

- All test cases are designed to be idempotent and can be run multiple times
- Test cases clean up after themselves to avoid side effects
- Cross-platform compatibility (macOS/Linux) is considered in all commands
- All regex patterns use portable POSIX syntax where possible
