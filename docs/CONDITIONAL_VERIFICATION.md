# Conditional Verification Syntax

This document describes the conditional verification feature, which extends the standard verification syntax to support branching logic based on condition evaluation.

## Overview

Conditional verification allows test steps to execute different commands based on whether a verification condition passes or fails. This is useful for:

- **Debug output**: Log detailed information when conditions pass or fail
- **Conditional cleanup**: Perform different cleanup actions based on verification results
- **Complex validation**: Build multi-step verification logic with branching paths
- **Error diagnostics**: Output detailed error information when verifications fail

**Important**: The pass/fail status of a test step is determined solely by the **condition** result, not by the success or failure of action commands executed in the branches. Action commands (in `if_true`, `if_false`, and `always` blocks) are for side effects like logging, debugging, or cleanup operations.

## Syntax Structure

Conditional verification can be used in any of the three verification fields:
- `verification.result` - Verifies the command result/exit code
- `verification.output` - Verifies the command output
- `verification.output_file` - Verifies output written to a log file

### Basic Structure

```yaml
verification:
  result:
    condition: "shell expression that returns exit code 0 (success) or non-zero (failure)"
    if_true:
      - "command to run if condition succeeds"
      - "another command if condition succeeds"
    if_false:
      - "command to run if condition fails"
      - "another command if condition fails"
    always:
      - "command that always runs regardless of condition result"
      - "another command that always runs"
```

### Required and Optional Fields

- **condition** (required): Shell expression that evaluates the verification condition
  - Must return exit code 0 for success (condition passes)
  - Non-zero exit code indicates failure (condition fails)

- **if_true** (optional): List of shell commands to execute when condition passes
  - Commands run sequentially in the order listed
  - Typically used for debug logging or success actions
  
- **if_false** (optional): List of shell commands to execute when condition fails
  - Commands run sequentially in the order listed
  - Typically used for error logging or diagnostic output
  - Can include `exit 1` to halt test execution on verification failure

- **always** (optional): List of shell commands that always execute
  - Runs after `if_true` or `if_false` (whichever was executed)
  - Useful for cleanup actions that should always occur
  - **Note**: If an action command calls `exit`, the always block will not execute

## Backward Compatibility

The conditional verification syntax is fully backward compatible with the simple string format. You can mix both formats within the same test case:

```yaml
verification:
  # Simple string format (backward compatible)
  result: "[[ $EXIT_CODE -eq 0 ]]"
  
  # Conditional format with branches
  output:
    condition: "grep -q 'SUCCESS' <<< \"$COMMAND_OUTPUT\""
    if_true:
      - "echo 'Output contains SUCCESS'"
    if_false:
      - "echo 'Output missing SUCCESS'"
```

## Environment Variables

The following environment variables are available in all verification expressions:

- `$EXIT_CODE` - Exit code of the executed command
- `$COMMAND_OUTPUT` - Full output from the command execution
- `$LOG_FILE` - Path to log file (for output_file verification)
- `$RESULT` - The result value being verified
- Any custom environment variables set in earlier steps

## Examples

### Example 1: Result Verification with Debug Output

```yaml
- step: 1
  description: "Verify command succeeds with exit code 0"
  command: echo "Command succeeded"
  expected:
    success: true
    result: 0
    output: "Command succeeded"
  verification:
    result:
      condition: "[[ $EXIT_CODE -eq 0 ]]"
      if_true:
        - "echo 'DEBUG: Exit code is 0 as expected'"
        - "echo 'DEBUG: Command passed successfully'"
      if_false:
        - "echo 'ERROR: Expected exit code 0 but got $EXIT_CODE'"
        - "exit 1"
    output: "grep -q 'Command succeeded' <<< \"$COMMAND_OUTPUT\""
```

**Output when passing:**
```
DEBUG: Exit code is 0 as expected
DEBUG: Command passed successfully
```

### Example 2: Expected Error Code Verification

```yaml
- step: 2
  description: "Verify command fails with specific exit code"
  command: /bin/sh -c "exit 42"
  expected:
    success: false
    result: 42
    output: ""
  verification:
    result:
      condition: "[[ $EXIT_CODE -eq 42 ]]"
      if_true:
        - "echo 'DEBUG: Got expected error code 42'"
        - "echo 'DEBUG: Negative test case passed'"
      if_false:
        - "echo 'ERROR: Expected exit code 42 but got $EXIT_CODE'"
        - "exit 1"
    output: "true"
```

**Output when passing:**
```
DEBUG: Got expected error code 42
DEBUG: Negative test case passed
```

### Example 3: Range-Based Exit Code Checking with Always Block

```yaml
- step: 3
  description: "Verify exit code is within expected range"
  command: /bin/sh -c "exit 5"
  expected:
    success: false
    result: 5
    output: ""
  verification:
    result:
      condition: "[[ $EXIT_CODE -ge 1 && $EXIT_CODE -le 10 ]]"
      if_true:
        - "echo 'DEBUG: Exit code $EXIT_CODE is in expected range [1-10]'"
      if_false:
        - "echo 'ERROR: Exit code $EXIT_CODE is outside expected range'"
        - "exit 1"
      always:
        - "echo 'DEBUG: Verification step completed for exit code $EXIT_CODE'"
    output: "true"
```

**Output when passing:**
```
DEBUG: Exit code 5 is in expected range [1-10]
DEBUG: Verification step completed for exit code 5
```

### Example 4: Output Verification with Pattern Matching

```yaml
- step: 4
  description: "Verify output contains success pattern"
  command: 'echo "Status: SUCCESS - Operation completed"'
  expected:
    result: 0
    output: "Status: SUCCESS - Operation completed"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output:
      condition: "grep -q 'SUCCESS' <<< \"$COMMAND_OUTPUT\""
      if_true:
        - "echo 'INFO: Output contains SUCCESS keyword'"
        - "echo 'INFO: Pattern match successful'"
      if_false:
        - "echo 'ERROR: Output does not contain SUCCESS keyword'"
        - "echo 'ERROR: Expected pattern not found in: $COMMAND_OUTPUT'"
        - "exit 1"
```

### Example 5: Multi-Line Output with Conditional Counting

```yaml
- step: 5
  description: "Verify multi-line output has correct line count"
  command: echo -e "Line 1\nLine 2\nLine 3\nLine 4"
  expected:
    result: 0
    output: "Line 1\nLine 2\nLine 3\nLine 4"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output:
      condition: "[[ $(grep -c 'Line' <<< \"$COMMAND_OUTPUT\") -eq 4 ]]"
      if_true:
        - "echo 'INFO: Found exactly 4 lines as expected'"
        - "LINE_COUNT=$(grep -c 'Line' <<< \"$COMMAND_OUTPUT\")"
        - "echo 'INFO: Line count verified: $LINE_COUNT'"
      if_false:
        - "ACTUAL_COUNT=$(grep -c 'Line' <<< \"$COMMAND_OUTPUT\")"
        - "echo 'ERROR: Expected 4 lines but found $ACTUAL_COUNT'"
        - "exit 1"
      always:
        - "echo 'INFO: Output verification completed'"
```

### Example 6: JSON-Like Output Verification

```yaml
- step: 6
  description: "Verify JSON output has required fields"
  command: 'echo "{\"status\": \"ok\", \"code\": 200}"'
  expected:
    result: 0
    output: '{"status": "ok", "code": 200}'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output:
      condition: "grep -q '\"status\": \"ok\"' <<< \"$COMMAND_OUTPUT\" && grep -q '\"code\": 200' <<< \"$COMMAND_OUTPUT\""
      if_true:
        - "echo 'INFO: JSON output contains expected status and code'"
        - "echo 'INFO: All required fields present'"
      if_false:
        - "echo 'ERROR: JSON output missing expected fields'"
        - "echo 'ERROR: Actual output: $COMMAND_OUTPUT'"
        - "exit 1"
```

### Example 7: Output File Verification with Cleanup

```yaml
- step: 7
  description: "Verify log file contains expected entry"
  command: echo "Test data written to log"
  expected:
    result: 0
    output: "Test data written to log"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "grep -q 'Test data' <<< \"$COMMAND_OUTPUT\""
    output_file:
      condition: "grep -q 'Test data written to log' \"$LOG_FILE\""
      if_true:
        - "echo 'INFO: Log file contains expected entry'"
        - "echo 'INFO: Log verification passed'"
      if_false:
        - "echo 'ERROR: Log file missing expected entry'"
        - "echo 'ERROR: Log file contents:'"
        - "cat \"$LOG_FILE\""
        - "exit 1"
```

### Example 8: Multiple Log Entries with Always Block

```yaml
- step: 8
  description: "Verify multiple log entries"
  command: 'echo "Entry 1"; echo "Entry 2"; echo "Entry 3"'
  expected:
    result: 0
    output: "Entry 1\nEntry 2\nEntry 3"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "[[ $(grep -c 'Entry' <<< \"$COMMAND_OUTPUT\") -eq 3 ]]"
    output_file:
      condition: "[[ $(grep -c 'Entry' \"$LOG_FILE\") -ge 3 ]]"
      if_true:
        - "echo 'INFO: All entries logged successfully'"
        - "ENTRY_COUNT=$(grep -c 'Entry' \"$LOG_FILE\")"
        - "echo 'INFO: Found $ENTRY_COUNT entries in log file'"
      if_false:
        - "echo 'ERROR: Not all entries found in log file'"
        - "exit 1"
      always:
        - "echo 'INFO: File verification completed'"
```

### Example 9: Mixed Simple and Conditional Verifications

This example shows backward compatibility by mixing both formats:

```yaml
steps:
  - step: 1
    description: "Simple verification (backward compatible)"
    command: echo "Simple test"
    expected:
      success: true
      result: 0
      output: "Simple test"
    verification:
      result: "[[ $EXIT_CODE -eq 0 ]]"
      output: "grep -q 'Simple test' <<< \"$COMMAND_OUTPUT\""
      output_file: "grep -q 'Simple test' \"$LOG_FILE\""
  
  - step: 2
    description: "Conditional result with simple output"
    command: true
    expected:
      success: true
      result: 0
      output: ""
    verification:
      result:
        condition: "[[ $EXIT_CODE -eq 0 ]]"
        if_true:
          - "echo 'DEBUG: Simple true command succeeded'"
        if_false:
          - "echo 'ERROR: Simple true command failed unexpectedly'"
          - "exit 1"
      output: "true"
  
  - step: 3
    description: "All conditional verifications"
    command: /bin/sh -c "echo 'All systems operational' && exit 0"
    expected:
      success: true
      result: 0
      output: "All systems operational"
    verification:
      result:
        condition: "[[ $EXIT_CODE -eq 0 ]]"
        if_true:
          - "echo 'DEBUG: Command completed successfully'"
        if_false:
          - "echo 'ERROR: Command failed with exit code $EXIT_CODE'"
          - "exit 1"
      output:
        condition: "grep -q 'operational' <<< \"$COMMAND_OUTPUT\""
        if_true:
          - "echo 'INFO: Output contains operational status'"
        if_false:
          - "echo 'ERROR: Output missing operational status'"
          - "exit 1"
      output_file:
        condition: "test -f \"$LOG_FILE\" && grep -q 'operational' \"$LOG_FILE\""
        if_true:
          - "echo 'INFO: Log file updated correctly'"
        if_false:
          - "echo 'WARN: Log file verification failed'"
        always:
          - "echo 'INFO: All verifications completed'"
```

## Common Use Cases

### 1. Debugging Test Failures

Use `if_true` and `if_false` branches to output detailed diagnostic information:

```yaml
verification:
  result:
    condition: "[[ $EXIT_CODE -eq 0 ]]"
    if_false:
      - "echo 'DIAGNOSTIC: Command failed with exit code: $EXIT_CODE'"
      - "echo 'DIAGNOSTIC: Command was: $COMMAND'"
      - "echo 'DIAGNOSTIC: Output was: $COMMAND_OUTPUT'"
      - "exit 1"
```

### 2. Conditional Cleanup

Perform different cleanup actions based on verification results:

```yaml
verification:
  output_file:
    condition: "test -f \"$LOG_FILE\" && grep -q 'SUCCESS' \"$LOG_FILE\""
    if_true:
      - "echo 'Keeping log file for successful test'"
    if_false:
      - "echo 'Archiving log file for failed test'"
      - "cp \"$LOG_FILE\" \"$LOG_FILE.failed\""
    always:
      - "echo 'Cleanup complete'"
```

### 3. Range Validation

Verify values fall within expected ranges:

```yaml
verification:
  result:
    condition: "[[ $EXIT_CODE -ge 0 && $EXIT_CODE -le 5 ]]"
    if_true:
      - "echo 'Exit code $EXIT_CODE is within acceptable range'"
    if_false:
      - "echo 'Exit code $EXIT_CODE is outside acceptable range [0-5]'"
      - "exit 1"
```

### 4. Pattern Matching with Fallback

Verify multiple patterns with detailed feedback:

```yaml
verification:
  output:
    condition: "grep -qE '(SUCCESS|OK|PASS)' <<< \"$COMMAND_OUTPUT\""
    if_true:
      - "MATCHED_PATTERN=$(grep -oE '(SUCCESS|OK|PASS)' <<< \"$COMMAND_OUTPUT\" | head -1)"
      - "echo 'Found success pattern: $MATCHED_PATTERN'"
    if_false:
      - "echo 'No success pattern found in output'"
      - "echo 'Output was: $COMMAND_OUTPUT'"
      - "exit 1"
```

## Important Notes

### Pass/Fail Determination

The pass/fail status of a verification is determined **only** by the condition's exit code:
- **Pass**: Condition returns exit code 0
- **Fail**: Condition returns non-zero exit code

Action commands in `if_true`, `if_false`, and `always` blocks do not affect the verification status unless they explicitly call `exit 1` (or another non-zero exit code).

### Exit Behavior

If an action command calls `exit`, the script terminates immediately:
- Commands after the `exit` in the same block will not execute
- The `always` block will not execute if `exit` is called before it
- This is the expected behavior for halting execution on critical failures

Example:
```yaml
if_false:
  - "echo 'Error occurred'"
  - "exit 1"  # Script terminates here
  - "echo 'This will never execute'"
always:
  - "echo 'This will also never execute'"
```

### Variable Scope

Variables set in action commands persist for subsequent commands in the same step:

```yaml
if_true:
  - "COUNT=$(grep -c 'pattern' <<< \"$COMMAND_OUTPUT\")"
  - "echo 'Found $COUNT matches'"  # Can use $COUNT variable
```

### Shell Expression Evaluation

All conditions and commands are evaluated as shell expressions:
- Use proper quoting for strings with spaces
- Use `<<<` for here-strings to avoid subshell issues
- Use `&&` and `||` for compound conditions
- Use `[[ ]]` for advanced test conditions

## Migration from Simple Format

To migrate from simple string format to conditional format:

**Before:**
```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "grep -q 'SUCCESS' <<< \"$COMMAND_OUTPUT\""
```

**After (with debugging):**
```yaml
verification:
  result:
    condition: "[[ $EXIT_CODE -eq 0 ]]"
    if_false:
      - "echo 'ERROR: Exit code was $EXIT_CODE, expected 0'"
      - "exit 1"
  output:
    condition: "grep -q 'SUCCESS' <<< \"$COMMAND_OUTPUT\""
    if_true:
      - "echo 'DEBUG: Output validation passed'"
    if_false:
      - "echo 'ERROR: Output did not contain SUCCESS'"
      - "echo 'Actual output: $COMMAND_OUTPUT'"
      - "exit 1"
```

## Best Practices

1. **Use `if_false` for Error Details**: When a condition fails, log detailed information about what went wrong

2. **Keep Conditions Simple**: Put complex logic in `if_true`/`if_false` blocks rather than in the condition itself

3. **Use `always` for Cleanup**: Put cleanup actions in `always` blocks to ensure they run regardless of condition result

4. **Add Context to Debug Messages**: Include relevant variable values in debug messages

5. **Call `exit 1` Explicitly**: In `if_false` blocks, explicitly call `exit 1` if you want to halt execution on failure

6. **Document Complex Logic**: Add comments in YAML or step descriptions to explain complex conditional logic

7. **Test Both Branches**: Create test cases that exercise both `if_true` and `if_false` paths

8. **Maintain Backward Compatibility**: Use simple string format for basic verifications, conditional format only when needed

## Related Documentation

- See `VERIFICATION_TEMPLATES.md` for pre-built verification patterns
- See `TEST_VERIFY_USAGE.md` for general verification guidance
- See `QUICK_START.md` for getting started with test case creation
- See the example file `testcases/conditional_verification_example.yml` for comprehensive examples

## Schema Definition

The conditional verification syntax is defined in the JSON schema. A verification expression can be either:

1. **Simple String**: `"[[ $EXIT_CODE -eq 0 ]]"`
2. **Conditional Object**: 
   ```json
   {
     "condition": "string (required)",
     "if_true": ["array", "of", "strings"],
     "if_false": ["array", "of", "strings"],
     "always": ["array", "of", "strings"]
   }
   ```

All three verification fields (`result`, `output`, `output_file`) support both formats using `oneOf` schema composition.
