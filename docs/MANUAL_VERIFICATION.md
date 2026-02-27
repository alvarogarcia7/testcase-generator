# Manual Step Verification

This document explains how to write verification expressions for manual test steps, including the use of helper functions, the USER_VERIFICATION variable, and best practices for both interactive and non-interactive execution modes.

## Table of Contents

1. [Overview](#overview)
2. [Helper Functions](#helper-functions)
3. [USER_VERIFICATION Variable](#user_verification-variable)
4. [Verification Expressions](#verification-expressions)
5. [Interactive vs Non-Interactive Behavior](#interactive-vs-non-interactive-behavior)
6. [Examples](#examples)
7. [Best Practices](#best-practices)

---

## Overview

Manual steps in test cases require human intervention and verification. While automated steps execute commands and verify results programmatically, manual steps need a mechanism for users to confirm that expected outcomes occurred.

The manual verification system provides:

- **Automated verification expressions**: Shell expressions that check conditions automatically (e.g., file existence, process status)
- **Helper functions**: `read_true_false()` and `read_verification()` for interactive user prompts
- **USER_VERIFICATION variable**: Combined verification status that determines PASS/FAIL
- **Mode detection**: Automatic switching between interactive and non-interactive modes

### Key Concepts

- **Manual steps** are marked with `manual: true` in YAML
- **Verification expressions** evaluate conditions to confirm expected outcomes
- **USER_VERIFICATION** combines result and output verification into a single pass/fail status
- **Helper functions** prompt users when automatic verification is not possible
- **TTY detection** enables non-interactive operation in CI/CD environments

---

## Helper Functions

Two bash helper functions are automatically included in generated test scripts to support interactive verification prompts.

### read_true_false()

Prompts the user for a yes/no response with proper validation.

**Signature:**
```bash
read_true_false "prompt message" "default"
```

**Parameters:**
- `prompt` - The question or prompt to display to the user
- `default` - Default response: "y" (yes) or "n" (no). Default is "y" if not specified.

**Return Values:**
- Returns `1` for "yes" response
- Returns `0` for "no" response

**Behavior:**
- **Interactive mode**: Displays prompt with `[Y/n]` or `[y/N]` based on default, validates input, loops until valid response
- **Non-interactive mode**: Returns default value immediately without prompting

**Valid Responses:**
- Yes: `Y`, `y`, `Yes`, `yes`, `YES`
- No: `N`, `n`, `No`, `no`, `NO`
- Empty (ENTER): Uses default value

**Example Usage:**
```bash
if read_true_false "Did the LED turn green?"; then
    echo "User confirmed LED is green"
    USER_VERIFICATION_RESULT=true
else
    echo "User reported LED is not green"
    USER_VERIFICATION_RESULT=false
fi
```

### read_verification()

Alias function identical to `read_true_false()`. Both functions have the same implementation and can be used interchangeably.

**Signature:**
```bash
read_verification "prompt message" "default"
```

**Usage:**
```bash
if read_verification "Is the display showing 'Welcome'?" "y"; then
    USER_VERIFICATION_OUTPUT=true
else
    USER_VERIFICATION_OUTPUT=false
fi
```

### Function Implementation

Both functions include:

1. **TTY Detection**: Checks if stdin is connected to a terminal and if `DEBIAN_FRONTEND=noninteractive`
2. **Input Validation**: Loops until user provides valid Y/N response
3. **Default Handling**: Empty input uses the specified default
4. **Case Insensitivity**: Accepts Y/y/Yes/YES and N/n/No/NO
5. **Error Messages**: Displays helpful error message for invalid input

---

## USER_VERIFICATION Variable

The `USER_VERIFICATION` variable is the final combined status that determines whether a manual step passes or fails.

### Component Variables

Three variables work together to determine verification status:

1. **USER_VERIFICATION_RESULT** - Verification of the command result/exit code
2. **USER_VERIFICATION_OUTPUT** - Verification of the command output
3. **USER_VERIFICATION** - Combined verification status (AND of result and output)

### Initialization

At the start of each manual step with verification:

```bash
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false
```

### Evaluation Logic

After verification expressions are evaluated:

```bash
# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi
```

### Pass/Fail Messages

The `USER_VERIFICATION` variable controls the final output message:

```bash
if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 1: Manually verify LED is green"
else
    echo "[FAIL] Step 1: Manually verify LED is green"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi
```

### Verification Scenarios

| Result | Output | USER_VERIFICATION | Outcome |
|--------|--------|-------------------|---------|
| true   | true   | true              | PASS    |
| true   | false  | false             | FAIL    |
| false  | true   | false             | FAIL    |
| false  | false  | false             | FAIL    |

---

## Verification Expressions

Verification expressions are shell commands that evaluate to true (exit code 0) or false (non-zero exit code).

### Simple String Expressions

Basic verification expressions are written as shell test commands:

```yaml
verification:
  result: "[ -f /tmp/led_green ]"
  output: "grep -q 'green' /tmp/led_status.log"
```

**Generated Bash Code:**
```bash
# Manual step verification
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false

# Verify result
if [ -f /tmp/led_green ]; then
    USER_VERIFICATION_RESULT=true
else
    USER_VERIFICATION_RESULT=false
fi

# Verify output
if grep -q 'green' /tmp/led_status.log; then
    USER_VERIFICATION_OUTPUT=true
else
    USER_VERIFICATION_OUTPUT=false
fi
```

### Conditional Verification Expressions

For more complex verification, use conditional verification syntax with branches:

```yaml
verification:
  result:
    condition: "[ -f /tmp/production_mode ]"
    if_true:
      - "echo 'Production mode detected'"
      - "USER_VERIFICATION_RESULT=true"
    if_false:
      - "echo 'Development mode detected'"
      - "USER_VERIFICATION_RESULT=false"
    always:
      - "echo 'Mode check complete'"
  output: "true"
```

**Generated Bash Code:**
```bash
# Conditional verification for result
if [ -f /tmp/production_mode ]; then
    echo 'Production mode detected'
    USER_VERIFICATION_RESULT=true
else
    echo 'Development mode detected'
    USER_VERIFICATION_RESULT=false
fi
echo 'Mode check complete'

# Simple verification for output
USER_VERIFICATION_OUTPUT=true
```

### Common Verification Patterns

#### File Existence
```yaml
verification:
  result: "[ -f /tmp/expected_file ]"
  output: "[ -s /tmp/output_file ]"  # File exists and is not empty
```

#### Pattern Matching in Files
```yaml
verification:
  result: "grep -q 'SUCCESS' /tmp/status.log"
  output: "grep -qE '(green|active)' /tmp/led_status.log"
```

#### Process Running
```yaml
verification:
  result: "pgrep -f 'my_service' > /dev/null"
  output: "ps aux | grep -v grep | grep -q 'my_service'"
```

#### Network Connectivity
```yaml
verification:
  result: "ping -c 1 192.168.1.1 > /dev/null 2>&1"
  output: "netstat -an | grep -q '192.168.1.1:22'"
```

#### Always True/False
```yaml
verification:
  result: "true"   # Always passes
  output: "false"  # Always fails
```

### Verification Expression Best Practices

1. **Use full paths**: Always use absolute paths for files
2. **Redirect stderr**: Use `2>/dev/null` to suppress error messages
3. **Use -q with grep**: Suppress output with `-q` flag for cleaner output
4. **Test expressions**: Verify expressions work in bash before adding to YAML
5. **Keep it simple**: Simple expressions are easier to debug and maintain

---

## Interactive vs Non-Interactive Behavior

The helper functions and manual steps automatically adapt based on the execution environment.

### Interactive Mode

**Triggered when:**
- Script is run from a terminal (TTY)
- `DEBIAN_FRONTEND` is not set to `noninteractive`
- Standard input is connected to a terminal (`[ -t 0 ]` is true)

**Behavior:**
- Displays prompts with `[Y/n]` or `[y/N]`
- Waits for user input
- Validates responses and loops on invalid input
- Allows user to confirm manual actions

**Example Output:**
```
Step 1: Manually verify LED is green
Command: Check LED color
INFO: This is a manual step. You must perform this action manually.
Did the LED turn green? [Y/n]: y
[PASS] Step 1: Manually verify LED is green
```

### Non-Interactive Mode

**Triggered when:**
- `DEBIAN_FRONTEND=noninteractive` environment variable is set
- Standard input is NOT a terminal (e.g., piped input, redirected input)
- Script is run in CI/CD pipeline
- Input is piped: `echo "" | bash script.sh`

**Behavior:**
- Skips user prompts entirely
- Returns default values immediately
- Relies only on automated verification expressions
- Displays "Non-interactive mode detected" message

**Example Output:**
```
Step 1: Manually verify LED is green
Command: Check LED color
INFO: This is a manual step. You must perform this action manually.
Non-interactive mode detected, skipping manual step confirmation.
[PASS] Step 1: Manually verify LED is green
```

### TTY Detection Logic

The detection logic checks two conditions:

```bash
if [[ "${DEBIAN_FRONTEND}" == 'noninteractive' ]] || ! [ -t 0 ]; then
    # Non-interactive mode: use defaults
else
    # Interactive mode: prompt user
fi
```

**Explanation:**
- `${DEBIAN_FRONTEND} == 'noninteractive'` - Debian/Ubuntu package installation convention
- `! [ -t 0 ]` - Tests if file descriptor 0 (stdin) is NOT a terminal

### Testing Both Modes

**Interactive execution:**
```bash
bash test_script.sh
```

**Non-interactive execution:**
```bash
# Using environment variable
DEBIAN_FRONTEND=noninteractive bash test_script.sh

# Using piped input
echo "" | bash test_script.sh

# Using redirected input
bash test_script.sh < /dev/null
```

---

## Examples

### Example 1: Simple File-Based Verification

From the e2e integration test:

**YAML:**
```yaml
- step: 1
  manual: true
  description: Manually verify LED is green
  command: Check LED color
  expected:
    success: true
    result: "0"
    output: green
  verification:
    result: "[ -f /tmp/led_green ]"
    output: "grep -q 'green' /tmp/led_status.log"
```

**Generated Bash:**
```bash
# Step 1: Manually verify LED is green
echo "Step 1: Manually verify LED is green"
echo "Command: Check LED color"
echo "INFO: This is a manual step. You must perform this action manually."
if [[ "${DEBIAN_FRONTEND}" != 'noninteractive' && -t 0 ]]; then
    read -p "Press ENTER to continue..."
else
    echo "Non-interactive mode detected, skipping manual step confirmation."
fi

# Manual step verification
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false

# Verify result
if [ -f /tmp/led_green ]; then
    USER_VERIFICATION_RESULT=true
else
    USER_VERIFICATION_RESULT=false
fi

# Verify output
if grep -q 'green' /tmp/led_status.log; then
    USER_VERIFICATION_OUTPUT=true
else
    USER_VERIFICATION_OUTPUT=false
fi

# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi

if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 1: Manually verify LED is green"
else
    echo "[FAIL] Step 1: Manually verify LED is green"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi
```

**Setup for Testing:**
```bash
# Create verification files for automated pass
touch /tmp/led_green
echo "green" > /tmp/led_status.log

# Execute script - should pass without prompts
bash test_script.sh
```

### Example 2: Always True Verification

From TC_MANUAL_SSH_001.yaml:

**YAML:**
```yaml
- step: 2
  manual: true
  description: "Manually SSH into device and verify login"
  command: ssh admin@192.168.1.100
  expected:
    success: true
    result: 0
    output: "Successfully authenticated"
  verification:
    result: "true"
    output: "Verify that SSH login prompt appears and authentication succeeds"
```

**Note:** When `verification.result` or `verification.output` is the string `"true"`, it's interpreted as a shell command that always succeeds. Similarly, `"false"` would always fail.

### Example 3: Conditional Verification in Manual Step

From the e2e integration test:

**YAML:**
```yaml
- step: 1
  manual: true
  description: Check deployment mode
  command: cat /etc/deployment_mode
  expected:
    success: true
    result: "0"
    output: production
  verification:
    result:
      condition: "[ -f /tmp/production_mode ]"
      if_true:
        - "echo 'MARKER_PRODUCTION: Production mode detected'"
        - "USER_VERIFICATION_RESULT=true"
      if_false:
        - "echo 'MARKER_DEVELOPMENT: Development mode detected'"
        - "USER_VERIFICATION_RESULT=false"
      always:
        - "echo 'MARKER_ALWAYS: Mode check complete'"
    output: "true"
```

**Generated Bash:**
```bash
# Manual step verification
USER_VERIFICATION_RESULT=false
USER_VERIFICATION_OUTPUT=false

# Conditional verification for result
if [ -f /tmp/production_mode ]; then
    echo 'MARKER_PRODUCTION: Production mode detected'
    USER_VERIFICATION_RESULT=true
else
    echo 'MARKER_DEVELOPMENT: Development mode detected'
    USER_VERIFICATION_RESULT=false
fi
echo 'MARKER_ALWAYS: Mode check complete'

# Simple verification for output
USER_VERIFICATION_OUTPUT=true

# Set USER_VERIFICATION based on verification results
if [ "$USER_VERIFICATION_RESULT" = true ] && [ "$USER_VERIFICATION_OUTPUT" = true ]; then
    USER_VERIFICATION=true
else
    USER_VERIFICATION=false
fi

if [ "$USER_VERIFICATION" = true ]; then
    echo "[PASS] Step 1: Check deployment mode"
else
    echo "[FAIL] Step 1: Check deployment mode"
    echo "  Result verification: $USER_VERIFICATION_RESULT"
    echo "  Output verification: $USER_VERIFICATION_OUTPUT"
    exit 1
fi
```

### Example 4: Multiple Verification Checks

**YAML:**
```yaml
- step: 1
  manual: true
  description: Verify system is ready for deployment
  command: Check system status
  expected:
    success: true
    result: "0"
    output: "Ready"
  verification:
    result:
      condition: "[ -f /tmp/system_ready ] && [ -f /tmp/checks_passed ]"
      if_true:
        - "echo 'All readiness checks passed'"
        - "USER_VERIFICATION_RESULT=true"
      if_false:
        - "echo 'System readiness checks failed'"
        - "[ ! -f /tmp/system_ready ] && echo '  Missing: system_ready marker'"
        - "[ ! -f /tmp/checks_passed ] && echo '  Missing: checks_passed marker'"
        - "USER_VERIFICATION_RESULT=false"
    output:
      condition: "pgrep -f 'deployment_service' > /dev/null"
      if_true:
        - "echo 'Deployment service is running'"
        - "USER_VERIFICATION_OUTPUT=true"
      if_false:
        - "echo 'Deployment service is not running'"
        - "USER_VERIFICATION_OUTPUT=false"
```

This example demonstrates:
- Multiple file checks using `&&` operator
- Conditional diagnostic output in `if_false` branch
- Process verification using `pgrep`
- Explicit setting of verification variables in conditional branches

### Example 5: Network-Based Verification

**YAML:**
```yaml
- step: 1
  manual: true
  description: Manually verify device is accessible on network
  command: Ping device and check SSH port
  expected:
    success: true
    result: "0"
    output: "Device accessible"
  verification:
    result: "ping -c 1 192.168.1.100 > /dev/null 2>&1"
    output: "nc -z 192.168.1.100 22 2>/dev/null"
```

**Generated verification:**
```bash
# Verify result - ping device
if ping -c 1 192.168.1.100 > /dev/null 2>&1; then
    USER_VERIFICATION_RESULT=true
else
    USER_VERIFICATION_RESULT=false
fi

# Verify output - check SSH port is open
if nc -z 192.168.1.100 22 2>/dev/null; then
    USER_VERIFICATION_OUTPUT=true
else
    USER_VERIFICATION_OUTPUT=false
fi
```

---

## Best Practices

### 1. Choose Appropriate Verification Methods

**Use automated verification expressions when possible:**
- File existence checks
- Process status checks
- Network connectivity tests
- Log file content verification

**Use `true` for purely manual verification:**
- Visual confirmation (LED colors, display text)
- Physical inspection (cable connections, button states)
- GUI navigation and interaction
- Subjective assessments

### 2. Design for Non-Interactive Execution

**Do:**
- Create marker files that verification expressions can check
- Use process status, file existence, or log content as verification
- Set up test environment to create expected conditions
- Make verification expressions deterministic

**Don't:**
- Rely solely on user prompts for verification
- Assume interactive terminal is available
- Require human judgment in CI/CD pipelines

### 3. Provide Clear Descriptions

```yaml
# Good - Clear actionable description
description: "Press the power button and verify LED changes from red to green"

# Bad - Vague description
description: "Check the device"
```

### 4. Use Meaningful Marker Files

```bash
# Good - Descriptive marker files
touch /tmp/led_green_verified
touch /tmp/ssh_login_successful
touch /tmp/deployment_mode_production

# Bad - Cryptic markers
touch /tmp/m1
touch /tmp/ok
touch /tmp/x
```

### 5. Test Both Pass and Fail Scenarios

```bash
# Test passing scenario
touch /tmp/expected_marker
bash test_script.sh  # Should PASS

# Test failing scenario
rm /tmp/expected_marker
bash test_script.sh  # Should FAIL
```

### 6. Include Diagnostic Information

Use conditional verification to provide helpful diagnostic output:

```yaml
verification:
  result:
    condition: "[ -f /tmp/marker ]"
    if_true:
      - "echo 'Verification marker found'"
    if_false:
      - "echo 'ERROR: Verification marker not found at /tmp/marker'"
      - "ls -la /tmp/ | grep marker || echo 'No marker files found'"
```

### 7. Combine Multiple Checks Appropriately

```yaml
# Use && for all-must-pass scenarios
verification:
  result: "[ -f /tmp/file1 ] && [ -f /tmp/file2 ] && [ -f /tmp/file3 ]"

# Use || for any-can-pass scenarios
verification:
  result: "[ -f /tmp/file1 ] || [ -f /tmp/file2 ] || [ -f /tmp/file3 ]"

# Use conditional verification for complex logic
verification:
  result:
    condition: "[ -f /tmp/file1 ]"
    if_true:
      - "[ -f /tmp/file2 ] && USER_VERIFICATION_RESULT=true || USER_VERIFICATION_RESULT=false"
```

### 8. Document Complex Verification Logic

```yaml
- step: 1
  manual: true
  description: |
    Verify system deployment mode.
    
    This step checks for the presence of /tmp/production_mode file
    which is created by the deployment script when running in
    production mode. In development mode, this file will not exist.
    
    Expected: File should exist in production deployments.
  command: cat /etc/deployment_mode
  expected:
    success: true
    result: "0"
    output: production
  verification:
    result: "[ -f /tmp/production_mode ]"
    output: "grep -q 'production' /etc/deployment_mode"
```

### 9. Handle Missing Dependencies Gracefully

```yaml
verification:
  result: "command -v nc >/dev/null 2>&1 && nc -z 192.168.1.100 22 2>/dev/null || true"
```

This pattern:
- Checks if `nc` command exists
- Runs the check if available
- Falls back to `true` if command not found (allowing manual verification)

### 10. Avoid Side Effects in Verification Expressions

```yaml
# Bad - Creates files as side effect
verification:
  result: "touch /tmp/marker && [ -f /tmp/marker ]"

# Good - Only tests conditions
verification:
  result: "[ -f /tmp/marker ]"
```

Verification expressions should be idempotent and not modify system state.

---

## Summary

### Key Takeaways

1. **Helper Functions**: `read_true_false()` and `read_verification()` provide interactive user prompts with automatic non-interactive mode support

2. **USER_VERIFICATION**: Combined status variable that determines PASS/FAIL based on both result and output verification

3. **Verification Expressions**: Shell commands that evaluate to true/false, supporting both simple strings and conditional logic

4. **Mode Detection**: Automatic detection of interactive vs non-interactive environments using TTY detection

5. **Best Practices**: Design for non-interactive execution, use clear descriptions, provide diagnostic output, and test both pass and fail scenarios

### Related Documentation

- **MANUAL_STEPS_HANDLING.md** - Complete guide to manual step handling across YAML, script generation, and execution
- **CONDITIONAL_VERIFICATION.md** - Detailed explanation of conditional verification syntax with if_true/if_false/always branches
- **TEST_VERIFY_USAGE.md** - General verification patterns and best practices for all step types
- **MANUAL_STEP_FILTERING.md** - How to filter and execute specific manual or automated steps

### Example Test Cases

The following test cases in `testcases/examples/manual_steps/` demonstrate manual verification patterns:

- **TC_MANUAL_SSH_001.yaml** - SSH authentication with simple true verification
- **TC_MANUAL_HARDWARE_002.yaml** - Physical hardware connections
- **TC_MANUAL_MIXED_010.yaml** - Mixed automated and manual workflow
- **TC_MANUAL_UI_003.yaml** - GUI interaction verification
- **TC_MANUAL_NETWORK_005.yaml** - Network configuration verification

### Integration Test

The complete end-to-end integration test demonstrating all manual verification features can be found at:

```
tests/integration/test_manual_verification_e2e.sh
```

This test validates:
- Helper function generation and usage
- USER_VERIFICATION variable initialization and evaluation
- Automatic verification expressions
- Interactive and non-interactive mode behavior
- PASS/FAIL message generation
- Conditional verification in manual steps
