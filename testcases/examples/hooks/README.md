# Hooks Example Test Cases

This directory contains comprehensive examples demonstrating the hook system for test case execution lifecycle management.

## Overview

Hooks allow custom scripts to be executed at various points in the test case lifecycle, enabling:
- Resource setup and teardown
- Logging and monitoring
- Custom validation
- Context management
- Error handling and recovery

## Hook Types

### 1. script_start
- **Execution**: Once at the very beginning of the test script
- **Purpose**: Initialize test environment, start timing, create global resources
- **Example**: `scripts/script_start.sh` - Logs test start time

### 2. setup_test
- **Execution**: Once after script_start, before any test sequences
- **Purpose**: Setup test-wide resources, create directories, initialize state
- **Example**: `scripts/setup_test.sh` - Creates workspace and sequence directories

### 3. before_sequence
- **Execution**: Before each test sequence starts
- **Purpose**: Sequence-level initialization, logging, resource allocation
- **Example**: `scripts/before_sequence.sh` - Logs sequence start and creates sequence log
- **Available Variables**: `TEST_SEQUENCE_ID`, `TEST_SEQUENCE_NAME`

### 4. after_sequence
- **Execution**: After each test sequence completes
- **Purpose**: Sequence-level cleanup, resource deallocation, logging
- **Example**: `scripts/after_sequence.sh` - Updates sequence log and cleans up resources
- **Available Variables**: `TEST_SEQUENCE_ID`, `TEST_SEQUENCE_NAME`

### 5. before_step
- **Execution**: Before each test step
- **Purpose**: Step-level initialization, variable logging, pre-execution checks
- **Example**: `scripts/before_step.sh` - Logs step details and current variable values
- **Available Variables**: `TEST_SEQUENCE_ID`, `TEST_STEP_NUMBER`, `TEST_STEP_DESCRIPTION`, sequence variables

### 6. after_step
- **Execution**: After each test step completes
- **Purpose**: Step validation, output capture, result logging
- **Example**: `scripts/after_step.sh` - Validates step result and saves output
- **Available Variables**: `TEST_SEQUENCE_ID`, `TEST_STEP_NUMBER`, `STEP_EXIT_CODE`, `COMMAND_OUTPUT`

### 7. teardown_test
- **Execution**: Once after all test sequences complete, before script_end
- **Purpose**: Test-wide cleanup, remove temporary resources
- **Example**: `scripts/teardown_test.sh` - Removes workspace and sequence directories

### 8. script_end
- **Execution**: Once at the very end of the test script
- **Purpose**: Final logging, duration calculation, cleanup tracking files
- **Example**: `scripts/script_end.sh` - Logs test completion time and total duration

## Error Handling

The `on_error` hook configuration controls behavior when a hook script fails:

- **fail** (default): Test execution stops immediately if a hook fails
- **continue**: Hook errors are logged but test execution continues

Example:
```yaml
hooks:
  script_start:
    command: "scripts/script_start.sh"
  setup_test:
    command: "scripts/setup_test.sh"
  # ... other hooks ...
  on_error: "fail"  # or "continue"
```

## Test Case: TC_HOOKS_001

### Description
Comprehensive integration test demonstrating all eight hook types with practical examples of logging, resource management, and validation.

### Features Demonstrated
1. **Complete Lifecycle Coverage**: All eight hooks are executed in proper sequence
2. **Resource Management**: Temporary directories and files created/cleaned by hooks
3. **Logging Integration**: All hooks use centralized logger library for consistent output
4. **Variable Access**: Hooks can access and log test context (sequences, steps, variables)
5. **Output Validation**: Hooks can validate and save step outputs
6. **Error Handling**: Demonstrates both fail and continue modes
7. **Timing and Duration**: Tracks test start/end times and calculates duration

### Test Sequences

#### Sequence 1: Hook Lifecycle with on_error: fail
- Basic hook execution flow
- Workspace and sequence log validation
- Hook tracking file verification

#### Sequence 2: Hook Integration with Variables and Capture
- Hook access to sequence variables
- Integration with variable capture
- Context-aware logging

#### Sequence 3: Hook Error Handling
- Resilient hook design
- Graceful handling of missing resources
- Logger library consistency

## Hook Script Implementation

All hook scripts follow these best practices:

1. **Shell Compatibility**: Compatible with bash 3.2+ (macOS and Linux)
2. **Logger Library**: Uses centralized `scripts/lib/logger.sh` for consistent output
3. **Error Handling**: Proper error checking and graceful degradation
4. **Variable Access**: Reads environment variables provided by test harness
5. **Resource Tracking**: Uses temporary files for state management
6. **Cleanup**: Removes tracking files when no longer needed

### Example Hook Structure

```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../../scripts/lib/logger.sh" || exit 1

# Access test context variables
SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"

# Perform hook operation
log_info "HOOK: hook_name - Operation description"

# Handle resources gracefully
if [ -f "/tmp/some_file.txt" ]; then
    log_info "HOOK: hook_name - Processing existing resource"
else
    log_verbose "HOOK: hook_name - Resource not found, skipping"
fi
```

## Usage

To execute the hooks example test:

```bash
# Generate test script
make build

# Run the test
./target/debug/yamlscript-test-harness testcases/examples/hooks/TC_HOOKS_001.yaml

# Or generate and inspect the shell script
./target/debug/yamlscript-test-harness testcases/examples/hooks/TC_HOOKS_001.yaml > test_script.sh
bash test_script.sh
```

## Hook Script Paths

Hook script paths in the YAML are relative to the test case YAML file location:

```yaml
hooks:
  script_start:
    command: "scripts/script_start.sh"  # Resolves to testcases/examples/hooks/scripts/script_start.sh
```

Alternatively, use absolute paths:

```yaml
hooks:
  script_start:
    command: "/absolute/path/to/script.sh"
```

Each hook can also specify error handling behavior individually:

```yaml
hooks:
  script_start:
    command: "scripts/script_start.sh"
    on_error: "fail"
  setup_test:
    command: "scripts/setup_test.sh"
    on_error: "continue"
```

## Expected Output

When running TC_HOOKS_001, you should see:
- Hook execution logged with `HOOK:` prefix
- Timestamps for test start and completion
- Sequence and step logging
- Resource creation and cleanup messages
- Total test duration calculation
- All tests passing with hooks executing successfully

## Benefits of Using Hooks

1. **Separation of Concerns**: Test logic separate from setup/teardown
2. **Reusability**: Same hook scripts can be used across multiple tests
3. **Debugging**: Detailed logging at each lifecycle point
4. **Resource Management**: Centralized cleanup prevents resource leaks
5. **Context Awareness**: Hooks have access to test execution context
6. **Flexibility**: Custom behavior at any point in test lifecycle
7. **Error Control**: Choose between strict (fail) or lenient (continue) error handling
