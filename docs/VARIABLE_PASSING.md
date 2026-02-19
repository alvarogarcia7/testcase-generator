# Variable Passing in Test Cases

## Overview

The Test Case Manager supports variable passing between test steps, allowing you to:

1. **Capture** dynamic values from command output using regex patterns
2. **Store** captured values in sequence-scoped variables
3. **Reference** stored variables in subsequent commands and verifications
4. **Initialize** variables at the sequence level for use across all steps

This enables dynamic, data-driven test flows where outputs from one step can be used as inputs to later steps.

## Core Concepts

### Variable Storage

All variables are stored in a bash associative array called `STEP_VARS`. This array persists throughout the test sequence execution and is accessible to all steps within that sequence.

### Variable Scope

- **Sequence-scoped**: Variables are scoped to their test sequence
- **Step-persistent**: Once captured or set, variables remain available to all subsequent steps in the same sequence
- **Cross-step**: Variables captured in step N can be used in step N+1, N+2, etc.

### Variable Lifecycle

1. **Initialization** - Variables can be declared in the `variables` section of a test sequence
2. **Capture** - Variables can be extracted from command output using regex patterns in `capture_vars`
3. **Substitution** - Variables are automatically substituted in commands and verification expressions
4. **Persistence** - Variables persist for the duration of the test sequence

## Variable Declaration and Initialization

### Sequence-Level Variables

Declare variables at the test sequence level using the `variables` field:

```yaml
test_sequences:
  - id: 1
    name: API Test Sequence
    description: Test API operations with shared configuration
    variables:
      api_host: "api.example.com"
      api_port: "8080"
      api_version: "v1"
      default_timeout: "30"
    initial_conditions:
      Platform:
        - "API server is running"
    steps: []
```

**Characteristics:**
- Variables are initialized before any steps execute
- Available to all steps in the sequence
- Use for configuration values, constants, or default values
- Values must be strings (quoted in YAML)

### Example Use Case

```yaml
test_sequences:
  - id: 1
    name: User Management Test
    description: Test user creation and authentication
    variables:
      base_url: "http://localhost:8080"
      admin_token: "test_admin_token_12345"
      test_environment: "staging"
    steps:
      - step: 1
        description: Create new user
        command: "curl -X POST ${base_url}/users -H 'Authorization: Bearer ${admin_token}'"
        # ... rest of step definition
```

## Variable Capture Syntax

### The capture_vars Field

Use the `capture_vars` field in a test step to extract values from command output:

```yaml
steps:
  - step: 1
    description: Login and capture session token
    command: "curl -X POST http://api.example.com/login -d 'user=admin&pass=secret'"
    capture_vars:
      session_token: "(?<=token=)[a-zA-Z0-9]+"
      user_id: "(?<=user_id=)\\d+"
      expires_at: "(?<=expires=)\\d{10}"
    expected:
      result: "0"
      output: "Login successful"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[[ \"$COMMAND_OUTPUT\" =~ \"Login successful\" ]]"
```

### Structure

```yaml
capture_vars:
  variable_name: "regex_pattern"
  another_variable: "another_regex_pattern"
```

**Key Points:**
- `capture_vars` is an optional field in step definitions
- Keys are variable names (must follow naming conventions)
- Values are Perl-compatible regex patterns (PCRE)
- Multiple variables can be captured from a single command output

## Variable Naming Conventions

### Valid Variable Names

Variable names must follow these rules:

1. **Start with a letter or underscore**: `[a-zA-Z_]`
2. **Contain only alphanumeric characters and underscores**: `[a-zA-Z0-9_]*`
3. **Case-sensitive**: `user_id` and `USER_ID` are different variables

### Recommended Naming Conventions

**Good variable names:**
```yaml
capture_vars:
  user_id: "\\d+"              # Snake case
  session_token: "[A-Z0-9]+"   # Descriptive
  api_response_code: "\\d{3}"  # Clear purpose
  server_timestamp: "\\d{10}"  # Specific
  device_ICCID: "[0-9]{19}"    # Mixed case acceptable
```

**Avoid:**
```yaml
capture_vars:
  x: "\\d+"           # Too generic
  temp: "[A-Z]+"      # Unclear purpose
  var1: ".*"          # Non-descriptive
  123id: "\\d+"       # Starts with number (invalid)
  user-id: "\\d+"     # Contains hyphen (invalid)
```

### Best Practices

1. **Use descriptive names**: `session_token` instead of `token` or `st`
2. **Use snake_case**: `user_id` instead of `userId` or `UserID`
3. **Be specific**: `api_response_code` instead of `code`
4. **Avoid reserved bash variables**: Don't use `PATH`, `HOME`, `USER`, etc.
5. **Namespace by domain**: `auth_token`, `db_connection_id`, `api_key`

## Supported Regex Patterns

The system uses Perl-compatible regex (PCRE) via `grep -oP` for variable extraction.

### Common Regex Patterns

#### 1. Numeric Values

```yaml
capture_vars:
  # Any integer
  user_id: "\\d+"
  
  # Specific length integer (e.g., 5 digits)
  zip_code: "\\d{5}"
  
  # Integer range (e.g., 3-5 digits)
  error_code: "\\d{3,5}"
  
  # Floating point number
  price: "\\d+\\.\\d{2}"
  
  # Signed integer
  temperature: "-?\\d+"
```

#### 2. Alphanumeric Strings

```yaml
capture_vars:
  # Lowercase letters and numbers
  username: "[a-z0-9]+"
  
  # Mixed case alphanumeric
  session_id: "[a-zA-Z0-9]+"
  
  # Alphanumeric with specific length
  order_id: "[A-Z0-9]{8}"
  
  # Alphanumeric with underscores
  api_key: "[a-zA-Z0-9_]+"
```

#### 3. Using Lookahead and Lookbehind

```yaml
capture_vars:
  # Extract value after "token="
  token: "(?<=token=)[a-zA-Z0-9]+"
  
  # Extract value after "id: "
  request_id: "(?<=id: )\\d+"
  
  # Extract value between quotes after "status":
  status: "(?<=status\":\")[^\"]+(?=\")"
  
  # Extract UUID
  uuid: "(?<==)[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"
```

#### 4. JSON Field Extraction

```yaml
capture_vars:
  # Simple JSON string value
  name: "(?<=\"name\":\")[^\"]*"
  
  # JSON numeric value
  count: "(?<=\"count\":)\\d+"
  
  # JSON boolean
  active: "(?<=\"active\":)(true|false)"
  
  # Nested JSON value
  city: "(?<=\"address\":\\{\"city\":\")[^\"]*"
```

#### 5. Complex Patterns

```yaml
capture_vars:
  # IP address
  ip_address: "\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}"
  
  # Email address
  email: "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
  
  # URL
  callback_url: "https?://[a-zA-Z0-9.-]+(?:/[^\\s]*)?"
  
  # ISO 8601 timestamp
  timestamp: "\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}(?:\\.\\d+)?(?:Z|[+-]\\d{2}:\\d{2})"
  
  # Hex color code
  color: "#[0-9a-fA-F]{6}"
```

### Pattern Tips

1. **Escape backslashes**: Use `\\d` instead of `\d` in YAML strings
2. **Use character classes**: `[a-zA-Z]` is more efficient than `(a|b|c|...)`
3. **Be specific**: Use `\\d{3}` instead of `\\d+` when you know the exact length
4. **Test patterns**: Use `grep -oP 'pattern'` to test your regex before adding to YAML
5. **Capture first match**: The system uses `head -n 1` to extract only the first match

### Regex Escaping in YAML

When writing regex patterns in YAML:

```yaml
# Correct - backslashes are escaped in double-quoted strings
capture_vars:
  digit: "\\d+"
  word: "\\w+"
  
# Alternative - use single quotes (no escaping needed)
capture_vars:
  digit: '\d+'
  word: '\w+'
  
# Alternative - literal block scalar
capture_vars:
  pattern: |
    \d{3}-\d{3}-\d{4}
```

## Variable Substitution Syntax

### Substitution in Commands

Variables can be referenced in commands using two syntax forms:

#### 1. Simple Dollar Syntax

```yaml
command: "curl http://$api_host:$api_port/users/$user_id"
```

**Pattern**: `$variable_name`

**Characteristics:**
- Concise and readable
- Only works for variables in `STEP_VARS`
- Bash special variables (`$?`, `$$`, `$0`, etc.) are preserved and not substituted

#### 2. Array Syntax

```yaml
command: "curl http://${STEP_VARS[api_host]}/api/${STEP_VARS[user_id]}/profile"
```

**Pattern**: `${STEP_VARS[variable_name]}`

**Characteristics:**
- Explicit reference to the STEP_VARS array
- More verbose but clearer
- Useful for complex shell commands where variable scope matters

#### 3. Brace Expansion Syntax

```yaml
command: "echo User ${user_id} has session ${session_token}"
```

**Pattern**: `${variable_name}`

**Characteristics:**
- Uses bash brace expansion
- Clean syntax for simple substitutions
- Recommended for readability

### Substitution Examples

```yaml
test_sequences:
  - id: 1
    name: Multi-Step API Test
    variables:
      base_url: "http://api.example.com"
      api_version: "v2"
    steps:
      - step: 1
        description: Create user and capture ID
        command: "curl -X POST $base_url/$api_version/users -d '{\"name\":\"John\"}'"
        capture_vars:
          new_user_id: "(?<=\"id\":)\\d+"
        expected:
          result: "0"
          output: "created"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"created\" ]]"
      
      - step: 2
        description: Retrieve user details
        command: "curl $base_url/$api_version/users/${new_user_id}"
        expected:
          result: "0"
          output: "John"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"John\" ]]"
      
      - step: 3
        description: Update user with explicit array syntax
        command: "curl -X PUT ${STEP_VARS[base_url]}/${STEP_VARS[api_version]}/users/${STEP_VARS[new_user_id]} -d '{\"name\":\"Jane\"}'"
        expected:
          result: "0"
          output: "updated"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"updated\" ]]"
```

### Substitution in Verification Expressions

Variables can be used in both `result` and `output` verification expressions:

```yaml
steps:
  - step: 1
    description: Get expected status code
    command: "echo 'expected_status=200'"
    capture_vars:
      expected_status: "(?<=expected_status=)\\d+"
    expected:
      result: "0"
      output: "200"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[[ \"$COMMAND_OUTPUT\" =~ \"200\" ]]"
  
  - step: 2
    description: Make API call and verify status
    command: "curl -w '%{http_code}' http://api.example.com/status"
    capture_vars:
      actual_status: "\\d{3}$"
    expected:
      result: "0"
      output: "200"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[ \"${actual_status}\" -eq \"${expected_status}\" ]"
```

### Reserved Variables

The following variables are reserved by the test framework and should not be overwritten:

- `EXIT_CODE` - Exit code of the last command
- `COMMAND_OUTPUT` - Output of the last command
- `LOG_FILE` - Path to the current step's log file
- `JSON_LOG` - Path to the JSON execution log
- `TIMESTAMP` - Current timestamp
- `FIRST_ENTRY` - JSON log formatting flag

**Important**: Do not use these names in `capture_vars` or `variables`.

## Variable Scope Rules

### Sequence Scope

Variables are scoped to their test sequence:

```yaml
test_sequences:
  - id: 1
    name: Sequence 1
    variables:
      shared_value: "sequence1"
    steps:
      - step: 1
        command: "echo $shared_value"  # Outputs: sequence1
        capture_vars:
          result1: ".*"
        # ... verification omitted for brevity
  
  - id: 2
    name: Sequence 2
    variables:
      shared_value: "sequence2"  # Different value, different scope
    steps:
      - step: 1
        command: "echo $shared_value"   # Outputs: sequence2
      - step: 2
        command: "echo $result1"        # ERROR: result1 not available from sequence 1
        # ... verification omitted for brevity
```

**Key Points:**
- Variables from sequence 1 are NOT available in sequence 2
- Each sequence has its own isolated variable scope
- Variable names can be reused across different sequences without conflict

### Step Persistence

Within a sequence, captured variables persist across steps:

```yaml
test_sequences:
  - id: 1
    name: Persistent Variables Demo
    steps:
      - step: 1
        command: "echo 'token=abc123'"
        capture_vars:
          auth_token: "(?<=token=)[a-z0-9]+"
        # ... verification omitted
      
      - step: 2
        command: "curl -H 'Authorization: Bearer $auth_token' http://api/data"
        # auth_token from step 1 is available here
        # ... verification omitted
      
      - step: 3
        command: "echo Token is still $auth_token"
        # auth_token is still available
        # ... verification omitted
```

### Variable Override

Later steps can override variables set in earlier steps:

```yaml
steps:
  - step: 1
    command: "echo 'status=pending'"
    capture_vars:
      status: "(?<=status=)\\w+"
    # status = "pending"
    # ... verification omitted
  
  - step: 2
    command: "echo 'status=complete'"
    capture_vars:
      status: "(?<=status=)\\w+"
    # status = "complete" (overrides previous value)
    # ... verification omitted
  
  - step: 3
    command: "echo Current status: $status"
    # Outputs: "Current status: complete"
    # ... verification omitted
```

### Initialization Order

Variables are initialized and made available in this order:

1. **Sequence-level `variables`** - Set before any steps execute
2. **Step `capture_vars`** - Set after each step's command execution
3. **Variable substitution** - Performed before command execution and verification

## Best Practices

### 1. Use Descriptive Variable Names

**Good:**
```yaml
capture_vars:
  customer_id: "(?<=customer_id=)\\d+"
  order_reference: "(?<=order_ref=)[A-Z0-9]+"
  transaction_timestamp: "(?<=timestamp=)\\d{10}"
```

**Avoid:**
```yaml
capture_vars:
  id: "\\d+"          # Too generic
  ref: "[A-Z0-9]+"    # Ambiguous
  ts: "\\d{10}"       # Cryptic abbreviation
```

### 2. Validate Variable Extraction

Always verify that variables were captured correctly:

```yaml
steps:
  - step: 1
    description: Extract user ID
    command: "curl http://api/users/create -d 'name=John'"
    capture_vars:
      user_id: "(?<=\"id\":)\\d+"
    expected:
      result: "0"
      output: "created"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[[ \"$COMMAND_OUTPUT\" =~ \"id\" ]]"
  
  - step: 2
    description: Verify user ID was captured (optional validation step)
    command: "echo Captured user_id: $user_id"
    expected:
      result: "0"
      output: "Captured user_id:"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[ -n \"${STEP_VARS[user_id]}\" ]"  # Verify not empty
```

### 3. Use Specific Regex Patterns

Be as specific as possible in regex patterns:

**Good:**
```yaml
capture_vars:
  order_id: "[A-Z]{3}-\\d{6}"        # Specific format: ABC-123456
  status_code: "[2-5]\\d{2}"         # HTTP status codes
  uuid: "[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"
```

**Avoid:**
```yaml
capture_vars:
  order_id: ".*"                      # Too greedy
  status_code: "\\d+"                 # Too permissive
  uuid: ".+"                          # Matches anything
```

### 4. Handle Missing Values

Consider what happens if a variable capture fails:

```yaml
steps:
  - step: 1
    description: Try to extract optional field
    command: "curl http://api/config"
    capture_vars:
      optional_setting: "(?<=setting=)[a-z]*"
    expected:
      result: "0"
      output: "config"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "true"  # Don't fail if setting is missing
  
  - step: 2
    description: Use optional setting with default
    command: "echo ${optional_setting:-default_value}"  # Bash default value syntax
    expected:
      result: "0"
      output: ".*"
    verification:
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "true"
```

### 5. Document Variable Dependencies

Use step descriptions to document variable dependencies:

```yaml
steps:
  - step: 1
    description: "Login and capture auth_token"
    command: "curl -X POST http://api/login -d 'user=admin&pass=secret'"
    capture_vars:
      auth_token: "(?<=token=)[A-Z0-9]+"
    # ... verification omitted
  
  - step: 2
    description: "Get user profile (requires auth_token from step 1)"
    command: "curl -H 'Authorization: Bearer $auth_token' http://api/profile"
    # ... verification omitted
```

### 6. Avoid Overly Complex Patterns

If your regex becomes too complex, consider breaking it into multiple steps:

**Complex (harder to maintain):**
```yaml
capture_vars:
  # Extract nested JSON value with complex pattern
  nested_value: "(?<=\"data\":\\{\"user\":\\{\"profile\":\\{\"id\":)\\d+(?=\\})"
```

**Simpler (easier to maintain):**
```yaml
steps:
  - step: 1
    command: "curl http://api/data | jq -r '.data.user.profile.id'"
    capture_vars:
      nested_value: "\\d+"
    # ... verification omitted
```

### 7. Initialize Constants as Sequence Variables

For values that don't change during the sequence, use `variables`:

```yaml
test_sequences:
  - id: 1
    name: Configuration Test
    variables:
      # Constants for this sequence
      api_endpoint: "http://api.example.com/v2"
      timeout: "30"
      retry_count: "3"
      expected_status: "200"
    steps:
      - step: 1
        command: "curl --max-time $timeout --retry $retry_count $api_endpoint/health"
        # ... verification omitted
```

### 8. Use Meaningful Defaults

When declaring sequence variables, use meaningful defaults:

```yaml
variables:
  api_host: "localhost"        # Not "host" or "server"
  api_port: "8080"             # Not "port" or "8080"
  test_user: "test_admin"      # Not "user" or "admin"
  request_timeout: "30"        # Not "timeout" or "30"
```

## Complete Examples

### Example 1: User Registration Flow

```yaml
test_sequences:
  - id: 1
    name: User Registration and Authentication
    description: Test complete user lifecycle from registration to authenticated request
    variables:
      api_base: "http://localhost:8080"
      api_version: "v1"
    steps:
      - step: 1
        description: Register new user and capture user ID
        command: "curl -X POST ${api_base}/${api_version}/users -H 'Content-Type: application/json' -d '{\"username\":\"testuser\",\"email\":\"test@example.com\",\"password\":\"Test123!\"}'"
        capture_vars:
          new_user_id: "(?<=\"id\":)\\d+"
          registration_token: "(?<=\"token\":\")[a-zA-Z0-9._-]+"
        expected:
          result: "0"
          output: "User created successfully"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"created successfully\" ]]"
      
      - step: 2
        description: Verify user was created with correct ID
        command: "curl ${api_base}/${api_version}/users/${new_user_id}"
        expected:
          result: "0"
          output: "testuser"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"testuser\" ]]"
      
      - step: 3
        description: Login with new user credentials
        command: "curl -X POST ${api_base}/${api_version}/auth/login -d 'username=testuser&password=Test123!'"
        capture_vars:
          auth_token: "(?<=\"access_token\":\")[a-zA-Z0-9._-]+"
          refresh_token: "(?<=\"refresh_token\":\")[a-zA-Z0-9._-]+"
          token_expires_in: "(?<=\"expires_in\":)\\d+"
        expected:
          result: "0"
          output: "Login successful"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"access_token\" ]]"
      
      - step: 4
        description: Make authenticated request using captured token
        command: "curl -H 'Authorization: Bearer ${auth_token}' ${api_base}/${api_version}/users/${new_user_id}/profile"
        expected:
          result: "0"
          output: "test@example.com"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"test@example.com\" ]]"
      
      - step: 5
        description: Update user profile using captured user ID and token
        command: "curl -X PUT -H 'Authorization: Bearer ${auth_token}' -H 'Content-Type: application/json' ${api_base}/${api_version}/users/${new_user_id} -d '{\"email\":\"newemail@example.com\"}'"
        expected:
          result: "0"
          output: "Profile updated"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"updated\" ]]"
      
      - step: 6
        description: Delete test user
        command: "curl -X DELETE -H 'Authorization: Bearer ${auth_token}' ${api_base}/${api_version}/users/${new_user_id}"
        expected:
          result: "0"
          output: "User deleted"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"deleted\" ]]"
```

### Example 2: File Processing Pipeline

```yaml
test_sequences:
  - id: 1
    name: Document Processing Pipeline
    description: Upload, process, and verify document processing
    variables:
      upload_endpoint: "http://localhost:3000/api/documents"
      processing_endpoint: "http://localhost:3000/api/process"
      status_endpoint: "http://localhost:3000/api/status"
    steps:
      - step: 1
        description: Upload document and capture document ID
        command: "curl -X POST -F 'file=@/tmp/test_document.pdf' ${upload_endpoint}"
        capture_vars:
          document_id: "(?<=\"document_id\":\")[a-f0-9-]{36}"
          upload_timestamp: "(?<=\"uploaded_at\":\")\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}"
        expected:
          result: "0"
          output: "Upload successful"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"document_id\" ]]"
      
      - step: 2
        description: Start document processing
        command: "curl -X POST ${processing_endpoint} -H 'Content-Type: application/json' -d '{\"document_id\":\"${document_id}\",\"operation\":\"extract_text\"}'"
        capture_vars:
          job_id: "(?<=\"job_id\":\")[a-f0-9-]{36}"
          estimated_duration: "(?<=\"estimated_seconds\":)\\d+"
        expected:
          result: "0"
          output: "Processing started"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"job_id\" ]]"
      
      - step: 3
        description: Check processing status
        command: "curl ${status_endpoint}/${job_id}"
        capture_vars:
          job_status: "(?<=\"status\":\")[a-z_]+"
          progress_percent: "(?<=\"progress\":)\\d+"
        expected:
          result: "0"
          output: "completed"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"completed\" ]]"
      
      - step: 4
        description: Retrieve processed document
        command: "curl ${processing_endpoint}/${job_id}/result"
        capture_vars:
          result_url: "(?<=\"result_url\":\")https?://[^\"]+(?=\")"
          page_count: "(?<=\"pages\":)\\d+"
          word_count: "(?<=\"words\":)\\d+"
        expected:
          result: "0"
          output: "result_url"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"result_url\" ]]"
      
      - step: 5
        description: Verify document metadata
        command: "echo Processed document ${document_id}: ${page_count} pages, ${word_count} words"
        expected:
          result: "0"
          output: "pages"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"${page_count}\" =~ ^[0-9]+$ ]]"
```

### Example 3: Multi-Device Provisioning

```yaml
test_sequences:
  - id: 1
    name: Device Provisioning with Dynamic Configuration
    description: Provision multiple devices and verify configuration
    variables:
      provisioning_server: "provisioning.example.com"
      device_type: "eUICC"
      firmware_version: "2.3.1"
    steps:
      - step: 1
        description: Request provisioning configuration
        command: "curl https://${provisioning_server}/api/config?type=${device_type}"
        capture_vars:
          profile_package_id: "(?<=\"package_id\":\")[A-Z0-9-]+"
          provisioning_url: "(?<=\"url\":\")https?://[^\"]+(?=\")"
          timeout_seconds: "(?<=\"timeout\":)\\d+"
        expected:
          result: "0"
          output: "package_id"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"package_id\" ]]"
      
      - step: 2
        description: Initialize device with captured configuration
        command: "device-cli init --url ${provisioning_url} --package ${profile_package_id} --timeout ${timeout_seconds}"
        capture_vars:
          device_eid: "(?<=EID: )[0-9]{32}"
          iccid: "(?<=ICCID: )[0-9]{19,20}"
        expected:
          result: "0"
          output: "Initialization complete"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"Initialization complete\" ]]"
      
      - step: 3
        description: Download and install profile package
        command: "device-cli download --package ${profile_package_id} --eid ${device_eid}"
        capture_vars:
          installation_id: "(?<=Installation ID: )[A-Z0-9-]+"
          profile_state: "(?<=State: )[A-Z_]+"
        expected:
          result: "0"
          output: "INSTALLED"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"${profile_state}\" == \"INSTALLED\" ]]"
      
      - step: 4
        description: Enable profile and verify activation
        command: "device-cli enable --iccid ${iccid}"
        capture_vars:
          activation_code: "(?<=Activation Code: )[A-Z0-9]+"
          imsi: "(?<=IMSI: )\\d{15}"
        expected:
          result: "0"
          output: "Profile enabled"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"enabled\" ]]"
      
      - step: 5
        description: Report activation to provisioning server
        command: "curl -X POST https://${provisioning_server}/api/activation -H 'Content-Type: application/json' -d '{\"eid\":\"${device_eid}\",\"iccid\":\"${iccid}\",\"installation_id\":\"${installation_id}\",\"activation_code\":\"${activation_code}\"}'"
        capture_vars:
          confirmation_id: "(?<=\"confirmation_id\":\")[A-Z0-9-]+"
        expected:
          result: "0"
          output: "Activation confirmed"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ \"confirmed\" ]]"
```

## Troubleshooting

### Variable Not Captured

**Problem**: Variable appears empty in subsequent steps

**Checklist**:
1. Verify the regex pattern matches the command output
2. Check for proper escaping in YAML (use `\\d` not `\d`)
3. Ensure the output contains the expected value
4. Test the regex with `echo "output" | grep -oP 'pattern'`

**Debug approach**:
```yaml
- step: 1
  description: Capture variable (debugging)
  command: "echo 'user_id=12345'"
  capture_vars:
    user_id: "(?<=user_id=)\\d+"
  expected:
    result: "0"
    output: "user_id=12345"  # Verify output format
  verification:
    result: "[ $EXIT_CODE -eq 0 ]"
    output: "[[ \"$COMMAND_OUTPUT\" =~ \"user_id\" ]]"

- step: 2
  description: Verify variable was captured
  command: "echo Captured value: ${user_id}"  # Should show the value
  expected:
    result: "0"
    output: "Captured value:"
  verification:
    result: "[ $EXIT_CODE -eq 0 ]"
    output: "[ -n \"${STEP_VARS[user_id]}\" ]"  # Check not empty
```

### Variable Substitution Not Working

**Problem**: Variable reference appears literally in command

**Causes**:
1. Variable not yet captured when referenced
2. Typo in variable name
3. Variable from different sequence

**Solution**:
```yaml
# Ensure variable is captured before use
- step: 1
  command: "echo 'token=abc123'"
  capture_vars:
    token: "(?<=token=)[a-z0-9]+"
  # ... verification omitted

- step: 2
  # token is now available
  command: "curl -H 'Authorization: Bearer ${token}' http://api/data"
  # ... verification omitted
```

### Regex Pattern Not Matching

**Problem**: Pattern doesn't extract expected value

**Debug steps**:
1. Test pattern in isolation:
   ```bash
   echo "your_output" | grep -oP 'your_pattern'
   ```

2. Check for special characters that need escaping
3. Verify YAML string escaping (use `\\d` not `\d` in double quotes)
4. Consider using single quotes or literal blocks in YAML

**Example**:
```yaml
# Testing pattern locally first
# $ echo 'user_id=12345' | grep -oP '(?<=user_id=)\d+'
# Output: 12345

capture_vars:
  user_id: "(?<=user_id=)\\d+"  # Works in YAML
```

### Variable Scope Issues

**Problem**: Variable not available across sequences

**Explanation**: Variables are sequence-scoped and cannot be shared between sequences.

**Solution**: If you need to share data between sequences, use one of these approaches:
1. Write to a file in sequence 1, read in sequence 2
2. Declare the same value in `variables` for both sequences
3. Combine related steps into a single sequence

## Additional Resources

- [VARIABLE_DISPLAY.md](VARIABLE_DISPLAY.md) - How captured variables are displayed and debugging techniques
- [VARIABLES_CAPTURE_COMMAND.md](VARIABLES_CAPTURE_COMMAND.md) - Advanced variable capture with regex and command-based methods
- [BDD Initial Conditions](BDD_INITIAL_CONDITIONS.md) - Using BDD patterns in initial conditions
- [Test Verify Usage](TEST_VERIFY_USAGE.md) - Verification expressions and templates
- [Quick Start Guide](QUICK_START.md) - Getting started with test cases
- JSON Schema: `schemas/schema.json` - Complete test case schema reference

## Summary

Variable passing enables:
- **Dynamic test flows**: Capture outputs and use them in subsequent steps
- **Reusable patterns**: Define configuration once, use throughout sequence
- **Data-driven testing**: Extract and validate dynamic values from commands
- **Complex workflows**: Build multi-step test scenarios with interdependent steps

Key syntax to remember:
- **Declare**: `variables: { var_name: "value" }`
- **Capture**: `capture_vars: { var_name: "regex_pattern" }`
- **Reference**: `$var_name`, `${var_name}`, or `${STEP_VARS[var_name]}`
- **Scope**: Sequence-level, persists across steps within a sequence
