# Hooks System

The hooks system provides lifecycle management for test execution, allowing you to run custom scripts at specific points during test execution. Hooks enable setup, teardown, logging, custom validation, and resource management throughout the test lifecycle.

## Table of Contents

- [Eight Hook Types](#eight-hook-types)
- [Execution Points in Test Lifecycle](#execution-points-in-test-lifecycle)
- [Hooks Field Structure in YAML](#hooks-field-structure-in-yaml)
- [Error Handling Modes](#error-handling-modes)
- [Hook Script Requirements](#hook-script-requirements)
- [Environment Variables](#environment-variables)
- [Sourcing vs Execution Behavior](#sourcing-vs-execution-behavior)
- [Use Cases for Each Hook Type](#use-cases-for-each-hook-type)
- [Examples from TC_HOOKS_001.yaml](#examples-from-tc_hooks_001yaml)
- [Best Practices](#best-practices)

## Eight Hook Types

The test execution system provides eight hook types that execute at different points in the test lifecycle:

1. **`script_start`** - Executes once at the very beginning of test script execution
2. **`setup_test`** - Executes once after prerequisites, before any test sequences
3. **`before_sequence`** - Executes before each test sequence starts
4. **`after_sequence`** - Executes after each test sequence completes
5. **`before_step`** - Executes before each test step runs
6. **`after_step`** - Executes after each test step completes
7. **`teardown_test`** - Executes once after all test sequences complete
8. **`script_end`** - Executes once at the very end of test script execution

## Execution Points in Test Lifecycle

The complete test lifecycle with hook execution points:

```
Test Script Start
├── script_start hook          ← Executes once at script start
├── Prerequisites check
├── setup_test hook            ← Executes once before sequences
│
├── Test Sequence 1
│   ├── before_sequence hook   ← Executes before sequence
│   ├── Sequence variables setup
│   ├── Sequence initial conditions
│   │
│   ├── Step 1
│   │   ├── before_step hook   ← Executes before step
│   │   ├── Command execution
│   │   ├── Variable capture
│   │   ├── Verification
│   │   └── after_step hook    ← Executes after step
│   │
│   ├── Step 2
│   │   ├── before_step hook
│   │   ├── Command execution
│   │   ├── Variable capture
│   │   ├── Verification
│   │   └── after_step hook
│   │
│   └── after_sequence hook    ← Executes after sequence
│
├── Test Sequence 2
│   ├── before_sequence hook
│   ├── Steps...
│   └── after_sequence hook
│
├── teardown_test hook         ← Executes once after sequences
└── script_end hook            ← Executes once at script end
```

## Hooks Field Structure in YAML

Hooks are defined in a top-level `hooks` field in your test case YAML. Each hook type is optional and can be configured independently:

```yaml
hooks:
  # Script lifecycle hooks
  script_start:
    command: "scripts/script_start.sh"
    on_error: "fail"
  
  setup_test:
    command: "scripts/setup_test.sh"
    on_error: "fail"
  
  # Sequence lifecycle hooks
  before_sequence:
    command: "scripts/before_sequence.sh"
    on_error: "fail"
  
  after_sequence:
    command: "scripts/after_sequence.sh"
    on_error: "continue"
  
  # Step lifecycle hooks
  before_step:
    command: "scripts/before_step.sh"
    on_error: "fail"
  
  after_step:
    command: "scripts/after_step.sh"
    on_error: "continue"
  
  # Cleanup hooks
  teardown_test:
    command: "scripts/teardown_test.sh"
    on_error: "continue"
  
  script_end:
    command: "scripts/script_end.sh"
    on_error: "continue"
```

### Hook Configuration Fields

Each hook configuration has two fields:

- **`command`** (required): The script or command to execute
  - Can be a relative path: `"scripts/hook.sh"`
  - Can be an absolute path: `"/usr/local/bin/hook.sh"`
  - Can be an inline command: `"echo 'Hook executed'"`

- **`on_error`** (optional): Error handling mode
  - `"fail"` - Stop test execution if hook fails (default)
  - `"continue"` - Log warning but continue test execution

## Error Handling Modes

The `on_error` field controls what happens when a hook fails (returns non-zero exit code):

### `fail` Mode (Default)

When a hook fails, the test execution stops immediately:

```yaml
before_step:
  command: "scripts/before_step.sh"
  on_error: "fail"  # or omit (fail is default)
```

Behavior:
- Hook returns non-zero exit code
- Error message is printed: `"Error: before_step hook failed with exit code X"`
- Test script exits with the hook's exit code
- No subsequent steps or hooks are executed

### `continue` Mode

When a hook fails, a warning is logged but test execution continues:

```yaml
after_step:
  command: "scripts/after_step.sh"
  on_error: "continue"
```

Behavior:
- Hook returns non-zero exit code
- Warning is printed: `"Warning: after_step hook failed with exit code X (continuing)"`
- Test execution continues normally
- Useful for optional logging or cleanup operations

### When to Use Each Mode

**Use `fail` mode for:**
- Setup operations that must succeed (creating directories, setting up databases)
- Critical validation that should stop tests if it fails
- Resource acquisition that tests depend on
- Security checks or permission validation

**Use `continue` mode for:**
- Optional logging or metrics collection
- Best-effort cleanup operations
- Non-critical validation or reporting
- Notification systems that shouldn't block tests

## Hook Script Requirements

### Executable Permissions

**For `.sh` files:** Executable permissions are NOT required because they are sourced into the current shell:

```bash
# .sh files are sourced, no execute permission needed
source "scripts/hook.sh"
```

**For non-`.sh` files:** Executable permissions ARE required because they are executed directly:

```bash
# Non-.sh files are executed, need execute permission
chmod +x scripts/hook.py
./scripts/hook.py
```

### Return Codes

Hooks communicate success or failure through exit codes:

- **Exit code 0**: Success - hook completed successfully
- **Exit code non-zero**: Failure - triggers `on_error` behavior
- **Exit code 127**: Special code used when hook script file is not found

Example hook with explicit return code:

```bash
#!/usr/bin/env bash

# Do some work
if ! some_command; then
    echo "Hook operation failed"
    exit 1  # Signal failure
fi

exit 0  # Signal success
```

### Missing Hook Files

If a hook script file doesn't exist:

1. A warning is printed: `"Warning: Hook script 'path/to/hook.sh' not found"`
2. Hook exit code is set to 127
3. The `on_error` behavior determines what happens next

## Environment Variables

Hooks have access to environment variables that provide context about the test execution:

### Available to All Hooks

- **`TEST_CASE_ID`** - The ID of the test case (e.g., `"TC_HOOKS_001"`)

### Available to Sequence Hooks

These hooks have access to sequence-level variables: `before_sequence`, `after_sequence`, `before_step`, `after_step`

- **`SEQUENCE_ID`** - Numeric ID of the current sequence (e.g., `1`, `2`, `3`)
- **`SEQUENCE_NAME`** - Name of the current sequence (e.g., `"Hook Lifecycle Test"`)

### Available to Step Hooks

These hooks have access to step-level variables: `before_step`, `after_step`

- **`STEP_NUMBER`** - Numeric step number within the sequence (e.g., `1`, `2`, `3`)
- **`STEP_DESC`** - Description of the current step

### Available to after_step Hook Only

The `after_step` hook has additional context from the just-completed step:

- **`EXIT_CODE`** - Exit code from the step command
- **`COMMAND_OUTPUT`** - Complete output (stdout + stderr) from the step command

### Additional Environment Variables

All sequence variables defined in the test case are also available as environment variables:

```yaml
test_sequences:
  - id: 1
    variables:
      TEST_VAR_1: "value_one"
      TEST_VAR_2: "value_two"
```

These become available as `$TEST_VAR_1` and `$TEST_VAR_2` in hook scripts.

### Example: Accessing Variables in Hooks

```bash
#!/usr/bin/env bash

# Available to all hooks
echo "Test Case: $TEST_CASE_ID"

# Available to sequence and step hooks
echo "Sequence: $SEQUENCE_ID - $SEQUENCE_NAME"

# Available to step hooks
echo "Step: $STEP_NUMBER - $STEP_DESC"

# Available only to after_step hook
echo "Exit Code: $EXIT_CODE"
echo "Output: $COMMAND_OUTPUT"

# Custom test variables
echo "Variables: $TEST_VAR_1, $TEST_VAR_2"
```

## Sourcing vs Execution Behavior

The hook system uses different execution methods based on file extension:

### `.sh` Files - Sourced

Files ending in `.sh` are **sourced** into the current shell:

```yaml
before_step:
  command: "scripts/before_step.sh"
```

Generated bash code:
```bash
if [ -f "scripts/before_step.sh" ]; then
    source "scripts/before_step.sh"
    HOOK_EXIT_CODE=$?
else
    echo "Warning: Hook script 'scripts/before_step.sh' not found" >&2
    HOOK_EXIT_CODE=127
fi
```

**Benefits of sourcing:**
- Can modify environment variables that persist after the hook
- Can define functions available to subsequent hooks
- No executable permissions required
- Runs in the same shell process

**Considerations:**
- Changes to shell state (variables, functions) persist
- Errors in the script can affect the main test execution
- Should use clean coding practices

### Non-`.sh` Files - Executed

Files with other extensions are **executed** as separate processes:

```yaml
after_step:
  command: "scripts/validator.py"
```

Generated bash code:
```bash
scripts/validator.py
HOOK_EXIT_CODE=$?
```

**Benefits of execution:**
- Isolated execution environment
- Can use any scripting language (Python, Ruby, etc.)
- Clear separation from test script

**Requirements:**
- Must have executable permissions (`chmod +x`)
- Must include shebang line (e.g., `#!/usr/bin/env python3`)
- Cannot modify parent shell environment

### Inline Commands

Inline commands are executed directly:

```yaml
before_step:
  command: "echo 'Starting step execution'"
```

These are useful for simple operations that don't require separate script files.

## Use Cases for Each Hook Type

### `script_start` - Test Initialization

**When it executes:** Once at the very beginning, before anything else

**Common use cases:**
- Record test start time for duration calculations
- Initialize global tracking files or state
- Set up logging infrastructure
- Print test banner or header information
- Validate environment before test begins

**Example:**
```bash
#!/usr/bin/env bash
TEST_START_TIME=$(date +%s)
echo "$TEST_START_TIME" > /tmp/test_start_time.txt
echo "Test execution started at $(date)"
```

### `setup_test` - Environment Preparation

**When it executes:** Once after prerequisites, before test sequences begin

**Common use cases:**
- Create test workspace directories
- Initialize databases or services
- Set up test fixtures
- Generate test data files
- Configure test environment settings

**Example:**
```bash
#!/usr/bin/env bash
TEST_DIR="/tmp/test_workspace"
mkdir -p "$TEST_DIR"
mkdir -p "$TEST_DIR/logs"
mkdir -p "$TEST_DIR/data"
echo "Test workspace created: $TEST_DIR"
```

### `before_sequence` - Sequence Setup

**When it executes:** Before each test sequence starts

**Common use cases:**
- Log sequence start for traceability
- Create sequence-specific resources
- Initialize sequence-level state
- Set up sequence-specific monitoring
- Record sequence start timestamp

**Example:**
```bash
#!/usr/bin/env bash
SEQUENCE_LOG="/tmp/sequence_${SEQUENCE_ID}.log"
echo "Sequence $SEQUENCE_ID ($SEQUENCE_NAME) started at $(date)" > "$SEQUENCE_LOG"
echo "Starting sequence: $SEQUENCE_NAME"
```

### `after_sequence` - Sequence Cleanup

**When it executes:** After each test sequence completes

**Common use cases:**
- Clean up sequence-specific resources
- Log sequence completion
- Generate sequence-level reports
- Archive sequence data
- Validate sequence-level invariants

**Example:**
```bash
#!/usr/bin/env bash
SEQUENCE_LOG="/tmp/sequence_${SEQUENCE_ID}.log"
echo "Sequence $SEQUENCE_ID completed at $(date)" >> "$SEQUENCE_LOG"
echo "Cleaning up sequence resources"
```

### `before_step` - Step Preparation and Logging

**When it executes:** Before each test step runs

**Common use cases:**
- Log step execution for debugging
- Print step context and variables
- Set up step-specific monitoring
- Record step start time
- Validate pre-conditions for steps

**Example:**
```bash
#!/usr/bin/env bash
echo "--- Step $STEP_NUMBER ---"
echo "Description: $STEP_DESC"
echo "Sequence: $SEQUENCE_NAME"
if [ -n "$TEST_VAR_1" ]; then
    echo "TEST_VAR_1: $TEST_VAR_1"
fi
```

### `after_step` - Custom Validation and Results

**When it executes:** After each test step completes

**Common use cases:**
- Perform custom validation beyond standard verification
- Save step output to files for later analysis
- Log step results and exit codes
- Generate step-level reports
- Validate side effects of step execution
- Check for unexpected warnings or errors in output

**Example:**
```bash
#!/usr/bin/env bash
echo "Step $STEP_NUMBER completed with exit code: $EXIT_CODE"

# Save step output to file
OUTPUT_DIR="/tmp/test_outputs"
mkdir -p "$OUTPUT_DIR"
echo "$COMMAND_OUTPUT" > "$OUTPUT_DIR/step_${SEQUENCE_ID}_${STEP_NUMBER}.txt"

# Custom validation
if [ "$EXIT_CODE" = "0" ]; then
    echo "✓ Step validation: SUCCESS"
else
    echo "✗ Step validation: FAILED"
fi
```

### `teardown_test` - Test Cleanup

**When it executes:** Once after all test sequences complete

**Common use cases:**
- Remove test workspace directories
- Clean up test data
- Tear down services or databases
- Archive test results
- Clean up temporary files

**Example:**
```bash
#!/usr/bin/env bash
echo "Cleaning up test resources"
TEST_DIR="/tmp/test_workspace"
if [ -d "$TEST_DIR" ]; then
    rm -rf "$TEST_DIR"
    echo "Removed workspace: $TEST_DIR"
fi
```

### `script_end` - Test Finalization

**When it executes:** Once at the very end, after everything else

**Common use cases:**
- Calculate total test duration
- Print test summary
- Clean up tracking files
- Send test completion notifications
- Generate final reports

**Example:**
```bash
#!/usr/bin/env bash
TEST_END_TIME=$(date +%s)
TEST_START_TIME=$(cat /tmp/test_start_time.txt)
DURATION=$((TEST_END_TIME - TEST_START_TIME))
echo "Test completed in $DURATION seconds"
rm -f /tmp/test_start_time.txt
```

## Examples from TC_HOOKS_001.yaml

The reference test case `testcases/examples/hooks/TC_HOOKS_001.yaml` demonstrates all eight hook types. Here are key examples:

### Complete Hook Configuration

```yaml
hooks:
  script_start:
    command: "scripts/script_start.sh"
  setup_test:
    command: "scripts/setup_test.sh"
  before_sequence:
    command: "scripts/before_sequence.sh"
  after_sequence:
    command: "scripts/after_sequence.sh"
  before_step:
    command: "scripts/before_step.sh"
  after_step:
    command: "scripts/after_step.sh"
  teardown_test:
    command: "scripts/teardown_test.sh"
  script_end:
    command: "scripts/script_end.sh"
  on_error: "fail"
```

### script_start Hook Example

```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../../scripts/lib/logger.sh" || exit 1

TEST_START_TIME=$(date +%s)
echo "$TEST_START_TIME" > /tmp/tc_hooks_001_start_time.txt

log_info "HOOK: script_start - Test execution started at $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
log_info "HOOK: script_start - Start time stored: $TEST_START_TIME"
```

### setup_test Hook Example

```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../../scripts/lib/logger.sh" || exit 1

TEST_DIR="/tmp/tc_hooks_001_test_workspace"
SEQUENCE_DIR="/tmp/tc_hooks_001_sequences"

log_info "HOOK: setup_test - Creating test workspace directories"

mkdir -p "$TEST_DIR"
mkdir -p "$SEQUENCE_DIR"

echo "$TEST_DIR" > /tmp/tc_hooks_001_workspace_dir.txt
echo "$SEQUENCE_DIR" > /tmp/tc_hooks_001_sequence_dir.txt

log_info "HOOK: setup_test - Created workspace: $TEST_DIR"
log_info "HOOK: setup_test - Created sequence directory: $SEQUENCE_DIR"
```

### before_step Hook Example

```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
STEP_DESC="${TEST_STEP_DESCRIPTION:-no description}"

log_info "HOOK: before_step - Sequence $SEQUENCE_ID, Step $STEP_NUMBER"
log_info "HOOK: before_step - Description: $STEP_DESC"

if [ -n "$TEST_VAR_1" ]; then
    log_verbose "HOOK: before_step - TEST_VAR_1 = $TEST_VAR_1"
fi

if [ -n "$TEST_VAR_2" ]; then
    log_verbose "HOOK: before_step - TEST_VAR_2 = $TEST_VAR_2"
fi

log_info "HOOK: before_step - Step execution about to begin"
```

### after_step Hook Example

```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"

log_info "HOOK: after_step - Sequence $SEQUENCE_ID, Step $STEP_NUMBER completed"
log_info "HOOK: after_step - Exit code: $EXIT_CODE"

if [ "$EXIT_CODE" = "0" ]; then
    log_info "HOOK: after_step - Step validation: SUCCESS"
    pass "Step $STEP_NUMBER passed"
else
    log_warning "HOOK: after_step - Step validation: FAILED"
    fail "Step $STEP_NUMBER failed with exit code $EXIT_CODE"
fi

TEST_DIR=$(cat /tmp/tc_hooks_001_workspace_dir.txt 2>/dev/null || echo "/tmp/tc_hooks_001_test_workspace")
STEP_OUTPUT_FILE="$TEST_DIR/step_${SEQUENCE_ID}_${STEP_NUMBER}_output.txt"

if [ -n "$COMMAND_OUTPUT" ]; then
    echo "$COMMAND_OUTPUT" > "$STEP_OUTPUT_FILE"
    log_verbose "HOOK: after_step - Saved step output to $STEP_OUTPUT_FILE"
fi
```

### teardown_test Hook Example

```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../../scripts/lib/logger.sh" || exit 1

log_info "HOOK: teardown_test - Cleaning up test resources"

TEST_DIR=$(cat /tmp/tc_hooks_001_workspace_dir.txt 2>/dev/null || echo "/tmp/tc_hooks_001_test_workspace")
SEQUENCE_DIR=$(cat /tmp/tc_hooks_001_sequence_dir.txt 2>/dev/null || echo "/tmp/tc_hooks_001_sequences")

if [ -d "$TEST_DIR" ]; then
    log_info "HOOK: teardown_test - Removing workspace directory: $TEST_DIR"
    rm -rf "$TEST_DIR"
fi

if [ -d "$SEQUENCE_DIR" ]; then
    log_info "HOOK: teardown_test - Removing sequence directory: $SEQUENCE_DIR"
    rm -rf "$SEQUENCE_DIR"
fi

log_info "HOOK: teardown_test - Cleanup completed"
```

## Best Practices

### 1. Use the Centralized Logging Library

**All hook scripts should use the logging library** for consistent output formatting:

```bash
#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/path/to/scripts/lib/logger.sh" || exit 1

# Use logging functions
log_info "Hook executing"
log_warning "Warning message"
log_error "Error message"
log_debug "Debug message (only if VERBOSE=1)"
```

See `AGENTS.md` for complete logger documentation.

### 2. Make Hooks Idempotent

Hooks should be safe to run multiple times:

```bash
# Good - check before creating
if [ ! -d "$TEST_DIR" ]; then
    mkdir -p "$TEST_DIR"
fi

# Good - cleanup is safe even if files don't exist
rm -f /tmp/tracking_file.txt
```

### 3. Use Defensive Programming

Hooks should handle missing files and variables gracefully:

```bash
# Use default values for variables
TEST_DIR="${TEST_DIR:-/tmp/default_workspace}"

# Check file existence before reading
if [ -f /tmp/tracking_file.txt ]; then
    VALUE=$(cat /tmp/tracking_file.txt)
else
    VALUE="default"
fi
```

### 4. Choose Appropriate Error Modes

- Use `on_error: fail` for critical operations
- Use `on_error: continue` for optional operations like logging

```yaml
hooks:
  setup_test:
    command: "scripts/setup.sh"
    on_error: "fail"  # Critical - must succeed
  
  after_step:
    command: "scripts/log_step.sh"
    on_error: "continue"  # Optional - don't fail test
```

### 5. Keep Hooks Focused and Simple

Each hook should have a single, clear purpose:

```bash
# Good - focused on one task
#!/usr/bin/env bash
# Create test workspace
mkdir -p /tmp/test_workspace
echo "/tmp/test_workspace" > /tmp/workspace_path.txt
```

### 6. Document Hook Behavior

Add comments explaining what each hook does:

```bash
#!/usr/bin/env bash
# Before Step Hook
# Purpose: Log step execution details and current variables
# Dependencies: Logger library
# Environment: Requires STEP_NUMBER and STEP_DESC

source "$SCRIPT_DIR/lib/logger.sh" || exit 1
log_info "Step $STEP_NUMBER: $STEP_DESC"
```

### 7. Use Consistent Exit Codes

Return meaningful exit codes:

```bash
# Success
exit 0

# General error
exit 1

# Missing dependency
exit 2

# Invalid configuration
exit 3
```

### 8. Make Hooks Reusable

Write generic hooks that can be used across multiple test cases:

```bash
#!/usr/bin/env bash
# Generic workspace creation hook
# Uses TEST_CASE_ID for unique workspace paths

WORKSPACE_DIR="/tmp/workspace_${TEST_CASE_ID}"
mkdir -p "$WORKSPACE_DIR"
echo "$WORKSPACE_DIR" > "/tmp/${TEST_CASE_ID}_workspace.txt"
echo "Created workspace: $WORKSPACE_DIR"
```

### 9. Test Hooks Independently

Test hook scripts independently before integrating:

```bash
# Test hook manually
TEST_CASE_ID="TC001" \
SEQUENCE_ID="1" \
STEP_NUMBER="1" \
./scripts/before_step.sh
```

### 10. Handle Cleanup Robustly

Cleanup hooks should handle partial cleanup gracefully:

```bash
#!/usr/bin/env bash
# Cleanup even if some operations fail
set +e  # Don't exit on error

if [ -d "$TEST_DIR" ]; then
    rm -rf "$TEST_DIR" 2>/dev/null || log_warning "Failed to remove $TEST_DIR"
fi

if [ -f /tmp/tracking_file.txt ]; then
    rm -f /tmp/tracking_file.txt 2>/dev/null
fi

exit 0  # Always succeed for cleanup
```

### 11. Avoid Long-Running Operations

Hooks should be fast to avoid slowing down tests:

```bash
# Bad - long operation in hook
sleep 30
run_expensive_operation

# Good - defer expensive operations
echo "needs_expensive_operation" > /tmp/deferred_ops.txt
# Process deferred operations in teardown_test or script_end
```

### 12. Use Proper Quoting

Always quote variables to handle spaces and special characters:

```bash
# Good - properly quoted
if [ -f "$WORKSPACE_DIR/file.txt" ]; then
    cat "$WORKSPACE_DIR/file.txt"
fi

# Bad - unquoted (breaks with spaces)
if [ -f $WORKSPACE_DIR/file.txt ]; then
    cat $WORKSPACE_DIR/file.txt
fi
```

## Summary

The hooks system provides powerful lifecycle management for test execution:

- **Eight hook types** cover all phases of test execution
- **Two error modes** (`fail` and `continue`) provide flexible error handling
- **Environment variables** give hooks context about the test execution
- **Sourcing vs execution** provides flexibility in how hooks run
- **Real-world examples** in `TC_HOOKS_001.yaml` demonstrate all features
- **Best practices** ensure reliable, maintainable hook scripts

For more information, see:
- Example test case: `testcases/examples/hooks/TC_HOOKS_001.yaml`
- Example hook scripts: `testcases/examples/hooks/scripts/`
- Logger library: `scripts/lib/logger.sh` (documented in `AGENTS.md`)
