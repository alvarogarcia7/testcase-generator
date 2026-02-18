# Variable Display and Debugging

## Overview

This document describes how captured variables are displayed throughout the Test Case Manager system:

1. **Generated Bash Scripts**: Variables stored as `STEP_VAR_*` with comments
2. **Console Output**: Variable names and values displayed during execution
3. **JSON Execution Logs**: Commands with variable metadata
4. **Debugging**: Techniques for troubleshooting variable capture issues

Understanding how variables are displayed helps with debugging test failures, verifying correct variable capture, and monitoring test execution.

## Related Documentation

- [VARIABLES_CAPTURE_COMMAND.md](VARIABLES_CAPTURE_COMMAND.md) - Complete guide to variable capture syntax and methods
- [VARIABLE_PASSING.md](VARIABLE_PASSING.md) - Variable scoping, substitution, and lifecycle

## Variable Display in Generated Bash Scripts

### Variable Storage Format

When test cases use variable capture, the generated bash scripts store captured values in variables prefixed with `STEP_VAR_`:

```bash
# Variable captured via regex
STEP_VAR_user_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"user_id":([0-9]+).*/\1/p' | head -n 1 || echo "")

# Variable captured via command
STEP_VAR_file_size=$(wc -c /tmp/output.txt | awk '{print $1}' 2>&1 || echo "")
```

**Naming Convention:**
- All captured variables use the prefix `STEP_VAR_`
- Original variable name follows the prefix: `STEP_VAR_{variable_name}`
- Variable names follow bash naming rules: alphanumeric and underscores only

**Examples:**
- `api_token` → `STEP_VAR_api_token`
- `session_id` → `STEP_VAR_session_id`
- `byte_count` → `STEP_VAR_byte_count`

### Comments Indicating Capture Source

The generated scripts include a comment before each capture block:

```bash
# Capture variables from output
STEP_VAR_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"token":"([^"]+)".*/\1/p' | head -n 1 || echo "")
STEP_VAR_user_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"user_id":([0-9]+).*/\1/p' | head -n 1 || echo "")
```

The comment `# Capture variables from output` indicates the start of a variable capture section for a step.

### Regex-Based Captures

Regex-based captures extract values from the step's `COMMAND_OUTPUT`:

```bash
# Capture variables from output
STEP_VAR_access_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"access_token":"([^"]+)".*/\1/p' | head -n 1 || echo "")
STEP_VAR_refresh_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"refresh_token":"([^"]+)".*/\1/p' | head -n 1 || echo "")
STEP_VAR_expires_in=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"expires_in":([0-9]+).*/\1/p' | head -n 1 || echo "")
```

**Characteristics:**
- Uses `sed` with regex pattern matching
- Extracts from `$COMMAND_OUTPUT` variable
- Uses `head -n 1` to capture only the first match
- Falls back to empty string with `|| echo ""`

### Command-Based Captures

Command-based captures execute shell commands directly:

```bash
# Capture variables from output
STEP_VAR_byte_count=$(wc -c /tmp/test_output.txt | awk '{print $1}' 2>&1 || echo "")
STEP_VAR_line_count=$(wc -l /tmp/test_output.txt | awk '{print $1}' 2>&1 || echo "")
STEP_VAR_user_id=$(jq -r '.user_id' /tmp/data.json 2>&1 || echo "")
```

**Characteristics:**
- Executes the command directly in a subshell `$(...)`
- Captures both stdout and stderr with `2>&1`
- Falls back to empty string with `|| echo ""`
- Can access `$COMMAND_OUTPUT` environment variable if needed

### Variable Tracking System

The generated scripts maintain a list of captured variable names:

```bash
# Initialize variable storage for captured variables (bash 3.2+ compatible)
STEP_VAR_NAMES=""
```

After each variable capture, it's added to the tracking list:

```bash
STEP_VAR_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"token":"([^"]+)".*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " token "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES token"
fi
```

This tracking system enables:
- Dynamic variable substitution in commands
- Variable substitution in verification expressions
- Bash 3.2+ compatibility (no associative arrays needed)

### Variable Substitution in Commands

When variables are referenced in commands, the script performs substitution:

```bash
# Store original command for substitution
ORIGINAL_COMMAND="curl http://api.example.com/users/${user_id}/profile"

# Perform variable substitution
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        # Escape special characters for sed
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\\]/\\&/g')
        # Replace ${var_name} pattern
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
        # Replace $var_name pattern (simple form)
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\$$var_name\([^a-zA-Z0-9_]\)/$escaped_value\1/g")
    done
fi

# Execute substituted command
eval "$SUBSTITUTED_COMMAND"
```

### Sequence Variables

Variables declared at the sequence level are initialized before steps execute:

```yaml
test_sequences:
  - id: 1
    variables:
      api_host: "api.example.com"
      api_port: "8080"
```

Generated script:

```bash
# Initialize sequence variables
STEP_VAR_api_host="api.example.com"
if ! echo " $STEP_VAR_NAMES " | grep -q " api_host "; then STEP_VAR_NAMES="$STEP_VAR_NAMES api_host"; fi
STEP_VAR_api_port="8080"
if ! echo " $STEP_VAR_NAMES " | grep -q " api_port "; then STEP_VAR_NAMES="$STEP_VAR_NAMES api_port"; fi
```

### Example: Complete Variable Capture Block

Here's a complete example from a generated script showing both regex and command-based captures:

```bash
# =============================================================================
# Test Sequence 1: User Registration and Token Capture
# =============================================================================

# Initialize sequence variables
STEP_VAR_api_base="http://localhost:8080"
if ! echo " $STEP_VAR_NAMES " | grep -q " api_base "; then STEP_VAR_NAMES="$STEP_VAR_NAMES api_base"; fi

# -----------------------------------------------------------------------------
# Step 1: Register user and capture tokens
# -----------------------------------------------------------------------------

# Execute command
COMMAND_OUTPUT=$(cat << 'EOF'
{"status":"success","user_id":12345,"access_token":"Bearer_abc123xyz","refresh_token":"rt_def456uvw"}
EOF
)
EXIT_CODE=$?

# Capture variables from output
STEP_VAR_user_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"user_id":([0-9]+).*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " user_id "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES user_id"
fi

STEP_VAR_access_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"access_token":"([^"]+)".*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " access_token "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES access_token"
fi

STEP_VAR_refresh_token=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"refresh_token":"([^"]+)".*/\1/p' | head -n 1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " refresh_token "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES refresh_token"
fi

# -----------------------------------------------------------------------------
# Step 2: Verify token and get file size
# -----------------------------------------------------------------------------

# Store original command for substitution
ORIGINAL_COMMAND='echo "Token: ${access_token}" > /tmp/token.txt'

# Perform variable substitution
SUBSTITUTED_COMMAND="$ORIGINAL_COMMAND"
if [ -n "$STEP_VAR_NAMES" ]; then
    for var_name in $STEP_VAR_NAMES; do
        eval "var_value=\$STEP_VAR_$var_name"
        escaped_value=$(printf '%s' "$var_value" | sed 's/[&/\\]/\\&/g')
        SUBSTITUTED_COMMAND=$(echo "$SUBSTITUTED_COMMAND" | sed "s/\${$var_name}/$escaped_value/g")
    done
fi

# Execute substituted command
eval "$SUBSTITUTED_COMMAND"
EXIT_CODE=$?

# Capture variables from output
STEP_VAR_file_size=$(wc -c /tmp/token.txt | awk '{print $1}' 2>&1 || echo "")
if ! echo " $STEP_VAR_NAMES " | grep -q " file_size "; then
    STEP_VAR_NAMES="$STEP_VAR_NAMES file_size"
fi
```

## Console Output During Execution

When executing test cases, the Test Case Manager displays progress information to the console. While variable values are not explicitly printed during normal execution, they can be viewed through:

### 1. Step Command Output

The output from commands that produce variable values is visible in the console:

```
=============================================================================
Test Sequence 1: API Authentication Flow
=============================================================================

-----------------------------------------------------------------------------
Step 1: Login and capture tokens
-----------------------------------------------------------------------------
Command: curl -X POST http://api.example.com/login -d 'user=admin&pass=secret'

Output:
{"status":"success","access_token":"Bearer_abc123xyz","user_id":12345}

Exit Code: 0
✓ Step 1 passed
```

### 2. Verification Output

When general verification conditions reference variables, any errors will show the variable values:

```
-----------------------------------------------------------------------------
Step 2: Verify user ID range
-----------------------------------------------------------------------------

✗ General verification failed: verify_user_id_range
   Condition: [[ $user_id -gt 10000 && $user_id -lt 20000 ]]
   Exit code: 1
```

### 3. Debug Mode

Variables can be displayed by adding debug steps to your test case:

```yaml
- step: 2
  description: "Debug: Display captured variables"
  command: |
    echo "Captured variables:"
    echo "  user_id: ${user_id}"
    echo "  access_token: ${access_token}"
    echo "  refresh_token: ${refresh_token}"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
```

This will produce console output like:

```
-----------------------------------------------------------------------------
Step 2: Debug: Display captured variables
-----------------------------------------------------------------------------
Command: echo "Captured variables:" ...

Output:
Captured variables:
  user_id: 12345
  access_token: Bearer_abc123xyz
  refresh_token: rt_def456uvw

Exit Code: 0
✓ Step 2 passed
```

## JSON Execution Logs with Variable Metadata

### Log File Location

JSON execution logs are created in the output directory:

```
{output_dir}/{test_case_id}_execution_log.json
```

Example:
```
output/TC_VAR_CAPTURE_001_execution_log.json
```

### Log Entry Format

Each step execution is logged as a JSON object:

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "curl -X POST http://api.example.com/login",
    "exit_code": 0,
    "output": "{\"status\":\"success\",\"user_id\":12345,\"access_token\":\"Bearer_abc123xyz\"}",
    "timestamp": "2024-01-15T10:30:00Z"
  },
  {
    "test_sequence": 1,
    "step": 2,
    "command": "curl -H 'Authorization: Bearer Bearer_abc123xyz' http://api.example.com/profile",
    "exit_code": 0,
    "output": "{\"user_id\":12345,\"name\":\"admin\",\"email\":\"admin@example.com\"}",
    "timestamp": "2024-01-15T10:30:01Z"
  }
]
```

### Variable Metadata in Logs

Variables are reflected in the logs through:

**1. Original Commands (Before Substitution)**

The log shows the original command as written in the test case:

```json
{
  "command": "curl -H 'Authorization: Bearer ${access_token}' http://api.example.com/profile"
}
```

**2. Command Output with Captured Values**

The output field contains the raw command output, which includes values that were captured:

```json
{
  "output": "{\"status\":\"success\",\"user_id\":12345,\"access_token\":\"Bearer_abc123xyz\"}"
}
```

**3. Substituted Commands in Subsequent Steps**

Commands in later steps show variable references, and the actual executed command (with substituted values) can be inferred from the output:

```json
{
  "step": 2,
  "command": "echo User ID: ${user_id}",
  "output": "User ID: 12345"
}
```

### Parsing Execution Logs

Example of parsing logs to extract variable information:

```bash
# Extract all commands that reference a specific variable
jq '.[] | select(.command | contains("${user_id}")) | {step, command, output}' execution_log.json

# Find steps where variables were captured (look for capture patterns in output)
jq '.[] | select(.output | contains("user_id")) | {step, output}' execution_log.json

# Track variable usage across steps
jq '.[] | {step, command}' execution_log.json | grep -A1 "user_id"
```

### Log Schema

The execution log follows this JSON schema:

```json
[
  {
    "test_sequence": <integer>,  // Sequence number (1-based)
    "step": <integer>,            // Step number (1-based)
    "command": <string>,          // Command as written in test case
    "exit_code": <integer>,       // Exit code (0 = success)
    "output": <string>,           // Command stdout/stderr output
    "timestamp": <string>         // ISO 8601 timestamp (optional)
  }
]
```

## Debugging Variable Capture Issues

### Common Issues and Solutions

#### 1. Variable Not Captured (Empty Value)

**Symptoms:**
- Variable appears empty in subsequent steps
- Verification fails with "unset variable" or empty string errors
- Substitution produces blank values

**Debug Steps:**

**Step 1: Add a debug command to display the variable value**

```yaml
- step: 2
  description: "Debug: Check if user_id was captured"
  command: |
    echo "user_id value: '${user_id}'"
    echo "user_id length: ${#user_id}"
    if [ -z "${user_id}" ]; then
      echo "ERROR: user_id is empty!"
    fi
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
```

**Step 2: Inspect the generated script**

Look for the capture line in the generated bash script:

```bash
STEP_VAR_user_id=$(echo "$COMMAND_OUTPUT" | sed -n 's/.*"user_id":([0-9]+).*/\1/p' | head -n 1 || echo "")
```

**Step 3: Test the regex pattern manually**

```bash
# Copy the COMMAND_OUTPUT from the step
COMMAND_OUTPUT='{"status":"success","user_id":12345}'

# Test the capture pattern
echo "$COMMAND_OUTPUT" | sed -n 's/.*"user_id":([0-9]+).*/\1/p'
```

**Step 4: Verify the output actually contains the expected value**

Check the execution log to see the actual command output:

```bash
jq '.[] | select(.step == 1) | .output' execution_log.json
```

**Common Causes:**
- Regex pattern doesn't match the actual output format
- Output uses different quote types (single vs double)
- Extra whitespace or newlines in output
- Output is on stderr instead of stdout
- Command failed before producing output

**Solutions:**
- Adjust the regex pattern to match actual output
- Use command-based capture with `jq` for JSON
- Add `2>&1` to capture stderr in command output
- Verify the command succeeds (exit code 0)

#### 2. Variable Substitution Not Working

**Symptoms:**
- Variable reference appears literally: `${user_id}` instead of the value
- Command fails with "command not found" or similar
- Quotes or escaping issues in substituted values

**Debug Steps:**

**Step 1: Check if variable was captured**

Add a step before the substitution to verify:

```yaml
- step: 1
  description: "Capture user_id"
  command: "echo '{\"user_id\":12345}'"
  capture_vars:
    - name: user_id
      capture: '"user_id":([0-9]+)'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"

- step: 2
  description: "Verify capture before use"
  command: "echo Captured user_id: ${user_id}"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
    general:
      - name: verify_user_id_not_empty
        condition: "[[ -n \"${user_id}\" ]]"
```

**Step 2: Check variable name spelling**

Ensure variable names match exactly (case-sensitive):

```yaml
capture_vars:
  - name: user_id  # Must match reference

# Later:
command: "curl http://api/users/${user_id}"  # Must match capture name
```

**Step 3: Verify variable scope**

Variables only persist within the same sequence:

```yaml
test_sequences:
  - id: 1
    steps:
      - step: 1
        capture_vars:
          - name: token
            capture: "..."
      - step: 2
        command: "echo ${token}"  # ✓ Works - same sequence
  
  - id: 2
    steps:
      - step: 1
        command: "echo ${token}"  # ✗ Fails - different sequence
```

**Solutions:**
- Verify variable was captured in an earlier step
- Check spelling and case of variable names
- Ensure variable and usage are in the same sequence
- Use explicit `${STEP_VARS[var_name]}` syntax if needed

#### 3. Regex Pattern Issues

**Symptoms:**
- Captures wrong value or partial value
- Captures multiple values when only one expected
- Pattern matches but extracts nothing

**Debug Techniques:**

**Test pattern with sample data:**

```bash
# Sample output
OUTPUT='{"user_id":12345,"name":"admin"}'

# Test different patterns
echo "$OUTPUT" | sed -n 's/.*"user_id":([0-9]+).*/\1/p'
echo "$OUTPUT" | sed -n 's/.*"name":"([^"]+)".*/\1/p'

# Test with extended regex
echo "$OUTPUT" | grep -oP '(?<="user_id":)\d+'
echo "$OUTPUT" | grep -oP '(?<="name":")[^"]+'
```

**Common regex issues:**

1. **Greedy matching**: Use non-greedy patterns
   ```yaml
   # Bad: Greedy, might match too much
   capture: '"token":"(.+)"'
   
   # Good: Non-greedy, stops at first quote
   capture: '"token":"([^"]+)"'
   ```

2. **Escaping in YAML**: Remember to escape backslashes
   ```yaml
   # Bad: Single backslash in YAML double-quotes
   capture: "\d+"
   
   # Good: Double backslash in YAML double-quotes
   capture: "\\d+"
   
   # Alternative: Single quotes (no escaping needed)
   capture: '\d+'
   ```

3. **Capture groups**: Ensure pattern has capture group `(...)`
   ```yaml
   # Bad: No capture group
   capture: '"user_id":[0-9]+'
   
   # Good: Has capture group
   capture: '"user_id":([0-9]+)'
   ```

**Solutions:**
- Use online regex testers (regex101.com) with PCRE mode
- Test patterns with actual command output
- Use command-based capture with `jq` for complex JSON
- Use more specific patterns (avoid `.*` when possible)

#### 4. Command-Based Capture Failures

**Symptoms:**
- Command execution fails silently
- Captured value is empty despite command appearing correct
- Error messages not visible

**Debug Steps:**

**Step 1: Test command in isolation**

```bash
# Test the capture command directly
wc -c /tmp/output.txt | awk '{print $1}'
jq -r '.user_id' /tmp/data.json

# Check for errors
echo $?  # Should be 0 for success
```

**Step 2: Add error handling to capture command**

```yaml
capture_vars:
  - name: user_id
    command: "jq -r '.user_id' /tmp/data.json 2>&1 || echo 'ERROR: jq failed'"
```

**Step 3: Verify file/data exists before capture**

```yaml
- step: 1
  description: "Create data file"
  command: "echo '{\"user_id\":12345}' > /tmp/data.json"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"

- step: 2
  description: "Verify file exists before capture"
  command: "test -f /tmp/data.json && cat /tmp/data.json"
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "user_id"
```

**Common Causes:**
- File doesn't exist yet when capture runs
- Command requires tool not installed (jq, awk, etc.)
- Incorrect file path or permissions
- Command syntax error
- Command produces output on stderr instead of stdout

**Solutions:**
- Verify prerequisites in initial conditions
- Test commands manually before adding to test
- Use `2>&1` to capture stderr
- Add file existence checks before capture
- Use simpler commands when possible

### Examining Generated Scripts

To debug variable issues, examine the generated bash script:

**1. Generate the script:**

```bash
testcase_manager generate testcases/TC_VAR_CAPTURE_001.yaml > generated_script.sh
```

**2. Review variable capture sections:**

```bash
# Find all variable captures
grep -A 2 "STEP_VAR_" generated_script.sh

# Find specific variable
grep "STEP_VAR_user_id" generated_script.sh

# See full capture block
grep -A 10 "# Capture variables from output" generated_script.sh
```

**3. Test script manually with debug mode:**

```bash
# Add debug output to script
bash -x generated_script.sh 2>&1 | tee debug_output.txt

# Or enable verbose mode for specific sections
set -x  # Turn on debug mode
# ... variable capture code ...
set +x  # Turn off debug mode
```

### Verification Condition Debugging

When general verification conditions fail, add explicit checks:

```yaml
verification:
  general:
    # Original condition
    - name: verify_user_id_range
      condition: "[[ $user_id -gt 10000 && $user_id -lt 20000 ]]"
    
    # Debug conditions
    - name: debug_user_id_not_empty
      condition: "[[ -n \"$user_id\" ]]"
    
    - name: debug_user_id_is_numeric
      condition: "[[ $user_id =~ ^[0-9]+$ ]]"
    
    - name: debug_user_id_value
      condition: "[[ $user_id -gt 0 ]]"
```

This helps identify which part of a complex condition is failing.

## Best Practices for Variable Display

### 1. Use Descriptive Variable Names

Good variable names make debugging easier:

```yaml
# Good
capture_vars:
  - name: api_access_token
  - name: user_session_id
  - name: response_timestamp

# Avoid
capture_vars:
  - name: token
  - name: id
  - name: ts
```

### 2. Add Debug Steps When Developing

During test development, add temporary debug steps:

```yaml
- step: 2
  description: "DEBUG: Display captured variables"
  command: |
    echo "=== Captured Variables ==="
    echo "user_id: ${user_id}"
    echo "access_token: ${access_token}"
    echo "=== End Debug ==="
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
```

Remove or disable these once the test is stable.

### 3. Validate Captured Values

Always add verification conditions to validate captured values:

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_user_id_captured
      condition: "[[ -n \"$user_id\" ]]"
    
    - name: verify_user_id_numeric
      condition: "[[ $user_id =~ ^[0-9]+$ ]]"
    
    - name: verify_user_id_valid_range
      condition: "[[ $user_id -gt 1000 ]]"
```

### 4. Use Command-Based Capture for Complex Parsing

When regex becomes complex, use command-based capture:

```yaml
# Instead of complex regex
capture_vars:
  - name: user_id
    capture: '(?<="data":\{(?:[^{}]*\{[^{}]*\})*[^{}]*"user_id":)(\d+)'

# Use jq instead
capture_vars:
  - name: user_id
    command: "jq -r '.data.user_id' <<< \"$COMMAND_OUTPUT\""
```

### 5. Document Variable Dependencies

Use step descriptions to document variable flow:

```yaml
- step: 1
  description: "Login and capture access_token for authentication"
  # ...
  capture_vars:
    - name: access_token
      capture: '"access_token":"([^"]+)"'

- step: 2
  description: "Get user profile (requires access_token from step 1)"
  command: "curl -H 'Authorization: Bearer ${access_token}' http://api/profile"
```

## Summary

Variable display in Test Case Manager:

✅ **Bash Scripts**: Variables stored as `STEP_VAR_*` with clear comments  
✅ **Console**: Command output shows variable capture sources  
✅ **JSON Logs**: Execution history with command and output metadata  
✅ **Debugging**: Multiple techniques to troubleshoot capture issues

Key debugging techniques:
- Add debug steps to display variable values
- Examine generated bash scripts for capture logic
- Test regex patterns manually with sample data
- Verify file existence and command availability
- Use validation conditions to catch empty variables early

See [VARIABLES_CAPTURE_COMMAND.md](VARIABLES_CAPTURE_COMMAND.md) for complete variable capture syntax and examples.
