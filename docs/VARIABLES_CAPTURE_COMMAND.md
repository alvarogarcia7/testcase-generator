# Variables Capture with Command-Based Extraction

## Overview

This document describes the advanced variable capture feature that allows extracting dynamic values from command output using two methods:

1. **Regex-based capture**: Extract values using regex patterns (from `COMMAND_OUTPUT`)
2. **Command-based capture**: Extract values by executing shell commands

Both methods use the new array format for `capture_vars` and can be combined in the same test step.

## The `capture_vars` Array Format

### Structure

The `capture_vars` field accepts an array of capture variable objects, each with the following structure:

```yaml
capture_vars:
  - name: variable_name
    capture: "regex_pattern"    # Option 1: Regex capture (mutually exclusive with command)
  - name: another_variable
    command: "shell_command"     # Option 2: Command-based capture (mutually exclusive with capture)
```

### Field Reference

Each capture variable object has three fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | Yes | Name of the variable to capture. Must follow bash variable naming rules: start with letter or underscore, contain only alphanumeric and underscores |
| `capture` | String | Conditional | Regex pattern to extract from `COMMAND_OUTPUT`. Mutually exclusive with `command` |
| `command` | String | Conditional | Shell command to execute to capture the value. Mutually exclusive with `capture` |

### Mutual Exclusivity of `capture` and `command`

**Important**: Each capture variable must specify **either** `capture` **or** `command`, but **not both**.

**Valid examples:**
```yaml
capture_vars:
  # Using capture
  - name: token
    capture: '"token":"([^"]+)"'
  
  # Using command
  - name: byte_count
    command: "wc -c /tmp/file.txt | awk '{print $1}'"
```

**Invalid example:**
```yaml
capture_vars:
  # ERROR: Both capture and command specified
  - name: value
    capture: '\d+'
    command: "echo 123"
```

**Invalid example:**
```yaml
capture_vars:
  # ERROR: Neither capture nor command specified
  - name: value
```

## Regex-Based Capture

Regex-based capture extracts values from the step's `COMMAND_OUTPUT` using regex patterns.

### Syntax

```yaml
steps:
  - step: 1
    command: |
      echo '{"status":"success","token":"abc123xyz"}'
    capture_vars:
      - name: token
        capture: '"token":"([^"]+)"'
      - name: status
        capture: '"status":"([^"]+)"'
```

### How It Works

1. The step's command executes and produces output
2. The output is stored in `COMMAND_OUTPUT`
3. Each regex pattern is applied to `COMMAND_OUTPUT` using extended regex
4. The first capture group `(...)` is extracted and stored in the variable

### Example: JSON Field Extraction

```yaml
- step: 1
  description: "Simulate API response and capture tokens using regex"
  command: |
    cat << 'EOF'
    {"status":"success","access_token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9","refresh_token":"rt_7f8e9d6c5b4a","expires_in":3600}
    EOF
  capture_vars:
    - name: access_token
      capture: '"access_token":"([^"]+)"'
    - name: refresh_token
      capture: '"refresh_token":"([^"]+)"'
    - name: expires_in
      capture: '"expires_in":([0-9]+)'
  expected:
    success: true
    result: '0'
    output: 'success'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "grep -q 'success' <<< \"$COMMAND_OUTPUT\""
```

### Example: Structured Log Parsing

```yaml
- step: 2
  description: "Extract multiple fields from structured log output"
  command: |
    cat << 'EOF'
    [2024-01-15T10:30:45Z] REQUEST_ID=req-a1b2c3d4 METHOD=POST PATH=/api/users/create STATUS=201 DURATION=145ms
    EOF
  capture_vars:
    - name: request_id
      capture: 'REQUEST_ID=([a-z0-9-]+)'
    - name: method
      capture: 'METHOD=([A-Z]+)'
    - name: api_path
      capture: 'PATH=([^ ]+)'
    - name: status_code
      capture: 'STATUS=([0-9]+)'
    - name: duration_ms
      capture: 'DURATION=([0-9]+)ms'
  expected:
    success: true
    result: '0'
    output: 'REQUEST_ID'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "grep -q 'REQUEST_ID' <<< \"$COMMAND_OUTPUT\""
```

## Command-Based Variable Extraction

Command-based capture executes a shell command to extract or compute values. This is powerful for:
- Processing files created by the main command
- Performing arithmetic operations
- Using specialized tools like `jq`, `awk`, `wc`
- Complex data transformations

### Syntax

```yaml
steps:
  - step: 1
    command: |
      echo "Hello World" > /tmp/test_output.txt
    capture_vars:
      - name: byte_count
        command: "wc -c /tmp/test_output.txt | awk '{print $1}'"
      - name: line_count
        command: "wc -l /tmp/test_output.txt | awk '{print $1}'"
```

### How It Works

1. The step's main command executes
2. For each command-based capture variable:
   - The specified command executes in a bash shell
   - The command has access to `COMMAND_OUTPUT` environment variable
   - The command's stdout is captured and stored in the variable
   - Leading/trailing whitespace is trimmed

### Environment Variables Available in Commands

Command-based captures have access to:
- `COMMAND_OUTPUT`: Output from the step's main command
- `EXIT_CODE`: Exit code from the step's main command
- All previously captured variables in `STEP_VARS`
- Standard environment variables

### Use Case 1: File Operations

When your command creates or modifies files, use command-based capture to extract metadata:

```yaml
- step: 1
  description: "Create test file and capture byte count using wc -c"
  command: |
    echo "Hello World" > /tmp/test_output.txt
  capture_vars:
    - name: byte_count
      command: "wc -c /tmp/test_output.txt | awk '{print $1}'"
    - name: line_count
      command: "wc -l /tmp/test_output.txt | awk '{print $1}'"
  expected:
    success: true
    result: '0'
    output: ''
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
    general:
      - name: verify_byte_count_value
        condition: "[[ $byte_count -eq 12 ]]"
      - name: verify_line_count_value
        condition: "[[ $line_count -eq 1 ]]"
```

### Use Case 2: JSON Parsing with jq

Extract specific fields from JSON files using `jq`:

```yaml
- step: 2
  description: "Create JSON data and capture fields using jq"
  command: |
    cat > /tmp/test_data.json << 'EOF'
    {
      "user_id": 12345,
      "username": "testuser",
      "email": "test@example.com",
      "active": true,
      "score": 98.5
    }
    EOF
  capture_vars:
    - name: user_id
      command: "jq -r '.user_id' /tmp/test_data.json"
    - name: username
      command: "jq -r '.username' /tmp/test_data.json"
    - name: email
      command: "jq -r '.email' /tmp/test_data.json"
    - name: score
      command: "jq -r '.score' /tmp/test_data.json"
  expected:
    success: true
    result: '0'
    output: ''
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
    general:
      - name: verify_user_id_numeric
        condition: "[[ $user_id =~ ^[0-9]+$ ]]"
      - name: verify_email_format
        condition: "[[ $email =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$ ]]"
```

### Use Case 3: Arithmetic Operations

Perform calculations on captured or computed values:

```yaml
- step: 2
  description: "Analyze metrics with arithmetic comparisons"
  command: |
    cat << 'EOF'
    Network_In: 1024 KB/s
    Network_Out: 512 KB/s
    EOF
  capture_vars:
    - name: network_in
      capture: 'Network_In: ([0-9]+) KB/s'
    - name: network_out
      capture: 'Network_Out: ([0-9]+) KB/s'
    - name: total_network
      command: "echo $((1024 + 512))"
  expected:
    success: true
    result: '0'
    output: 'Network'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "grep -q 'Network' <<< \"$COMMAND_OUTPUT\""
    general:
      - name: verify_total_network_calculated
        condition: "[[ $total_network -eq 1536 ]]"
```

### Use Case 4: Processing Command Output with grep/awk

Extract patterns from the main command's output using `COMMAND_OUTPUT`:

```yaml
- step: 3
  description: "Extract data using grep and awk pipeline"
  command: |
    cat > /tmp/server_log.txt << 'EOF'
    2024-01-15 10:30:45 INFO User=admin Action=LOGIN IP=192.168.1.100
    2024-01-15 10:31:12 INFO User=testuser Action=READ IP=192.168.1.101
    2024-01-15 10:32:03 WARN User=testuser Action=WRITE IP=192.168.1.101 Status=THROTTLED
    2024-01-15 10:33:21 ERROR User=baduser Action=DELETE IP=10.0.0.50 Status=DENIED
    EOF
  capture_vars:
    - name: first_user
      command: "grep 'User=' /tmp/server_log.txt | head -1 | awk -F'User=' '{print $2}' | awk '{print $1}'"
    - name: error_count
      command: "grep -c 'ERROR' /tmp/server_log.txt"
    - name: warn_count
      command: "grep -c 'WARN' /tmp/server_log.txt"
    - name: last_ip
      command: "grep 'IP=' /tmp/server_log.txt | tail -1 | awk -F'IP=' '{print $2}' | awk '{print $1}'"
  expected:
    success: true
    result: '0'
    output: ''
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
    general:
      - name: verify_first_user_is_admin
        condition: "[[ \"$first_user\" = \"admin\" ]]"
      - name: verify_error_count
        condition: "[[ $error_count -eq 1 ]]"
      - name: verify_total_issues
        condition: "[[ $((error_count + warn_count)) -eq 2 ]]"
```

## Mixing Regex and Command-Based Captures

You can combine both capture methods in a single step:

```yaml
- step: 1
  description: "Process file and capture both computed and extracted values"
  command: |
    cat > /tmp/data.txt << 'EOF'
    Transaction: TXN-2024-001
    Amount: $1500.50
    Currency: USD
    Status: COMPLETED
    EOF
    cat /tmp/data.txt
  capture_vars:
    # Regex-based captures from COMMAND_OUTPUT
    - name: transaction_id
      capture: 'Transaction: (TXN-[0-9]{4}-[0-9]+)'
    - name: amount_str
      capture: 'Amount: \$([0-9]+\.[0-9]{2})'
    - name: currency
      capture: 'Currency: ([A-Z]{3})'
    - name: status
      capture: 'Status: ([A-Z]+)'
    # Command-based captures from file
    - name: file_size
      command: "wc -c /tmp/data.txt | awk '{print $1}'"
    - name: line_count
      command: "wc -l /tmp/data.txt | awk '{print $1}'"
  expected:
    success: true
    result: '0'
    output: 'Transaction'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "grep -q 'Transaction' <<< \"$COMMAND_OUTPUT\""
    general:
      - name: verify_transaction_id_format
        condition: "[[ $transaction_id =~ ^TXN-[0-9]{4}-[0-9]+$ ]]"
      - name: verify_file_size_reasonable
        condition: "[[ $file_size -gt 100 && $file_size -lt 1000 ]]"
```

## General Verification Array

The `verification.general` field accepts an array of named bash conditions that can reference captured variables.

### Structure

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: descriptive_verification_name
      condition: "[[ $variable_name -eq 123 ]]"
    - name: another_verification
      condition: "[[ $another_var =~ ^pattern$ ]]"
```

### Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | Yes | Descriptive name for the verification. Used in test reports |
| `condition` | String | Yes | Bash condition to evaluate. Can reference captured variables |

### How It Works

1. After the step's main command executes
2. After all `capture_vars` are processed
3. Each general verification condition is evaluated in a bash subshell
4. Variables from `STEP_VARS` are available in the condition
5. If any condition evaluates to false, the step fails verification

### Example: Numeric Validations

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_byte_count_numeric
      condition: "[[ $byte_count =~ ^[0-9]+$ ]]"
    - name: verify_byte_count_value
      condition: "[[ $byte_count -eq 12 ]]"
    - name: verify_line_count_numeric
      condition: "[[ $line_count =~ ^[0-9]+$ ]]"
    - name: verify_line_count_value
      condition: "[[ $line_count -eq 1 ]]"
```

### Example: String Pattern Matching

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_username_pattern
      condition: "[[ $username =~ ^[a-z]+$ ]]"
    - name: verify_username_length
      condition: "[[ ${#username} -ge 4 && ${#username} -le 20 ]]"
    - name: verify_email_format
      condition: "[[ $email =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$ ]]"
```

### Example: Range Validations

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_user_id_range
      condition: "[[ $user_id -gt 10000 && $user_id -lt 20000 ]]"
    - name: verify_score_is_numeric
      condition: "[[ $score =~ ^[0-9]+\\.?[0-9]*$ ]]"
    - name: verify_status_is_2xx
      condition: "[[ $status_code -ge 200 && $status_code -lt 300 ]]"
```

### Example: Arithmetic Operations

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_total_issues
      condition: "[[ $((error_count + warn_count)) -eq 2 ]]"
    - name: verify_amount_range
      condition: "[[ $(echo $amount_str | cut -d. -f1) -ge 1000 && $(echo $amount_str | cut -d. -f1) -le 2000 ]]"
    - name: verify_network_balance
      condition: "[[ $network_in -gt $network_out ]]"
```

### Example: Complex Pattern Checks

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_access_token_pattern
      condition: "[[ $access_token =~ ^eyJ[A-Za-z0-9_-]+$ ]]"
    - name: verify_refresh_token_prefix
      condition: "[[ $refresh_token =~ ^rt_ ]]"
    - name: verify_git_commit_hexadecimal
      condition: "[[ $git_commit =~ ^[a-f0-9]{40}$ ]]"
    - name: verify_config_is_yaml
      condition: "[[ $config_path =~ \\.yaml$ ]]"
```

## Complete Example from TC_VAR_CAPTURE_002.yaml

Here's a comprehensive example demonstrating all features:

```yaml
- step: 1
  description: "Create JSON data and capture fields using jq"
  command: |
    cat > /tmp/test_data.json << 'EOF'
    {
      "user_id": 12345,
      "username": "testuser",
      "email": "test@example.com",
      "active": true,
      "score": 98.5
    }
    EOF
  capture_vars:
    - name: user_id
      command: "jq -r '.user_id' /tmp/test_data.json"
    - name: username
      command: "jq -r '.username' /tmp/test_data.json"
    - name: email
      command: "jq -r '.email' /tmp/test_data.json"
    - name: score
      command: "jq -r '.score' /tmp/test_data.json"
  expected:
    success: true
    result: '0'
    output: ''
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "true"
    general:
      - name: verify_user_id_numeric
        condition: "[[ $user_id =~ ^[0-9]+$ ]]"
      - name: verify_user_id_range
        condition: "[[ $user_id -gt 10000 && $user_id -lt 20000 ]]"
      - name: verify_username_pattern
        condition: "[[ $username =~ ^[a-z]+$ ]]"
      - name: verify_username_length
        condition: "[[ ${#username} -ge 4 && ${#username} -le 20 ]]"
      - name: verify_email_format
        condition: "[[ $email =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$ ]]"
      - name: verify_score_is_numeric
        condition: "[[ $score =~ ^[0-9]+\\.?[0-9]*$ ]]"
```

Another example mixing both capture types:

```yaml
- step: 3
  description: "Complex pattern matching with multiple capture groups"
  command: |
    cat << 'EOF'
    [INFO] Server started successfully
    [INFO] Listening on 0.0.0.0:8080
    [INFO] Database connected: postgresql://db.example.com:5432/mydb
    [INFO] Cache initialized: redis://cache.example.com:6379
    [WARN] Configuration file /etc/app/config.yaml uses deprecated settings
    [INFO] Ready to accept connections
    EOF
  capture_vars:
    # Regex-based captures
    - name: listen_port
      capture: 'Listening on [^:]+:([0-9]+)'
    - name: db_host
      capture: 'postgresql://([^:]+):[0-9]+'
    - name: db_port
      capture: 'postgresql://[^:]+:([0-9]+)'
    - name: cache_host
      capture: 'redis://([^:]+):[0-9]+'
    - name: cache_port
      capture: 'redis://[^:]+:([0-9]+)'
    - name: config_path
      capture: 'Configuration file ([^ ]+)'
    # Command-based captures
    - name: info_count
      command: "grep -c '\\[INFO\\]' <<< \"$COMMAND_OUTPUT\""
    - name: warn_count
      command: "grep -c '\\[WARN\\]' <<< \"$COMMAND_OUTPUT\""
  expected:
    success: true
    result: '0'
    output: 'Server started'
  verification:
    result: "[[ $EXIT_CODE -eq 0 ]]"
    output: "grep -q 'Server started' <<< \"$COMMAND_OUTPUT\""
    general:
      - name: verify_listen_port_is_8080
        condition: "[[ $listen_port -eq 8080 ]]"
      - name: verify_listen_port_valid_range
        condition: "[[ $listen_port -ge 1024 && $listen_port -le 65535 ]]"
      - name: verify_db_host_format
        condition: "[[ $db_host =~ \\.example\\.com$ ]]"
      - name: verify_db_port_is_5432
        condition: "[[ $db_port -eq 5432 ]]"
      - name: verify_cache_port_is_6379
        condition: "[[ $cache_port -eq 6379 ]]"
      - name: verify_config_path_absolute
        condition: "[[ $config_path =~ ^/ ]]"
      - name: verify_config_is_yaml
        condition: "[[ $config_path =~ \\.yaml$ ]]"
      - name: verify_info_messages_present
        condition: "[[ $info_count -ge 4 ]]"
      - name: verify_warn_messages_exist
        condition: "[[ $warn_count -ge 1 ]]"
      - name: verify_total_message_count
        condition: "[[ $((info_count + warn_count)) -eq 6 ]]"
```

## Migration Guide: Legacy to New Format

### Legacy BTreeMap Format

The legacy format used a simple map structure:

```yaml
capture_vars:
  variable_name: "regex_pattern"
  another_variable: "another_regex_pattern"
```

**Limitations:**
- Only supported regex-based capture
- No support for command-based capture
- Less explicit structure

### New Array Format

The new format uses an array of objects:

```yaml
capture_vars:
  - name: variable_name
    capture: "regex_pattern"
  - name: another_variable
    command: "shell_command"
```

**Advantages:**
- Supports both regex and command-based capture
- More explicit and self-documenting
- Extensible for future enhancements
- Better error messages

### Migration Examples

**Before (Legacy):**
```yaml
steps:
  - step: 1
    command: "curl http://api.example.com/users/create"
    capture_vars:
      user_id: '(?<="id":)\d+'
      username: '(?<="name":")[^"]+'
      email: '(?<="email":")[^"]+'
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[ -n \"$user_id\" ]"
```

**After (New):**
```yaml
steps:
  - step: 1
    command: "curl http://api.example.com/users/create"
    capture_vars:
      - name: user_id
        capture: '(?<="id":)\d+'
      - name: username
        capture: '(?<="name":")[^"]+'
      - name: email
        capture: '(?<="email":")[^"]+'
    verification:
      result: "[[ $EXIT_CODE -eq 0 ]]"
      output: "true"
      general:
        - name: verify_user_id_captured
          condition: "[[ -n \"$user_id\" ]]"
```

**With Command-Based Captures:**
```yaml
steps:
  - step: 1
    command: |
      curl http://api.example.com/users/create > /tmp/response.json
      cat /tmp/response.json
    capture_vars:
      # Regex from output
      - name: user_id
        capture: '(?<="id":)\d+'
      # Command to process file with jq
      - name: username
        command: "jq -r '.name' /tmp/response.json"
      - name: email
        command: "jq -r '.email' /tmp/response.json"
      # Command for file metadata
      - name: response_size
        command: "wc -c /tmp/response.json | awk '{print $1}'"
    verification:
      result: "[[ $EXIT_CODE -eq 0 ]]"
      output: "true"
      general:
        - name: verify_user_id_numeric
          condition: "[[ $user_id =~ ^[0-9]+$ ]]"
        - name: verify_response_not_empty
          condition: "[[ $response_size -gt 0 ]]"
```

### Backward Compatibility

The system supports **both formats** for backward compatibility:
- Legacy BTreeMap format (map of name to regex pattern)
- New array format (array of objects with name and capture/command)

You can continue using the legacy format, but the new format is recommended for:
- New test cases
- Test cases requiring command-based capture
- Better documentation and maintainability

### Migration Checklist

When migrating from legacy to new format:

1. ✅ Replace map structure with array of objects
2. ✅ Add `name:` field for each variable
3. ✅ Rename pattern value to `capture:` field
4. ✅ Consider adding command-based captures where appropriate
5. ✅ Move complex verification logic to `general` array
6. ✅ Update verification conditions to use modern bash syntax `[[ ]]`
7. ✅ Add descriptive names to general verification conditions

## Best Practices

### 1. Choose the Right Capture Method

**Use regex capture when:**
- Extracting patterns from command output
- Parsing structured text (JSON, logs, etc.)
- Pattern is simple and self-contained

**Use command capture when:**
- Processing files created by the command
- Using specialized tools (jq, xmllint, etc.)
- Performing calculations or transformations
- Counting or aggregating data

### 2. Name Variables Descriptively

```yaml
# Good
capture_vars:
  - name: user_id
  - name: session_token
  - name: file_byte_count

# Avoid
capture_vars:
  - name: id
  - name: token
  - name: count
```

### 3. Use General Verification for Complex Checks

Instead of complex inline verification, use named general verifications:

```yaml
verification:
  result: "[[ $EXIT_CODE -eq 0 ]]"
  output: "true"
  general:
    - name: verify_email_format
      condition: "[[ $email =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$ ]]"
    - name: verify_email_domain
      condition: "[[ $email =~ @example\\.com$ ]]"
```

### 4. Validate Captured Values

Always verify that captured values meet expectations:

```yaml
general:
  - name: verify_value_not_empty
    condition: "[[ -n \"$variable_name\" ]]"
  - name: verify_value_numeric
    condition: "[[ $variable_name =~ ^[0-9]+$ ]]"
  - name: verify_value_range
    condition: "[[ $variable_name -ge 0 && $variable_name -le 100 ]]"
```

### 5. Document Complex Conditions

Use descriptive names that explain what the condition checks:

```yaml
general:
  - name: verify_api_response_time_under_200ms
    condition: "[[ $response_ms -lt 200 ]]"
  - name: verify_database_connection_successful
    condition: "[[ $db_status = \"connected\" ]]"
  - name: verify_total_records_match_sum_of_categories
    condition: "[[ $total_records -eq $((category_a + category_b + category_c)) ]]"
```

## Related Documentation

- [VARIABLE_PASSING.md](VARIABLE_PASSING.md) - Variable scoping, substitution, and lifecycle
- [CONDITIONAL_VERIFICATION.md](CONDITIONAL_VERIFICATION.md) - Advanced verification patterns
- [TEST_VERIFY_USAGE.md](TEST_VERIFY_USAGE.md) - Verification expressions and templates
- [BDD_INITIAL_CONDITIONS.md](BDD_INITIAL_CONDITIONS.md) - Initial condition patterns

## Summary

The new `capture_vars` array format provides:

✅ **Two capture methods**: Regex-based and command-based  
✅ **Explicit structure**: Clear field names and validation  
✅ **Flexibility**: Mix both methods in the same step  
✅ **Named verifications**: Descriptive general verification conditions  
✅ **Backward compatibility**: Legacy format still supported  
✅ **Better maintainability**: Self-documenting test cases

Use command-based capture for file operations, JSON parsing, and arithmetic. Use regex capture for pattern extraction from output. Use general verification array for complex validation logic with descriptive names.
