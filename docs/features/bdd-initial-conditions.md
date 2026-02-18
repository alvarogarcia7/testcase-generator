# BDD Initial Conditions

## Overview

The Test Case Manager supports Behavior-Driven Development (BDD) style initial conditions that are automatically converted to executable commands during test execution. This allows test authors to write initial conditions in a human-readable format that is simultaneously:

- **Readable**: Easy for non-technical stakeholders to understand
- **Executable**: Automatically converted to bash commands during test script generation
- **Maintainable**: Centrally defined patterns can be reused across all test cases

## How It Works

When you write an initial condition (general, top-level, or sequence-level) using BDD syntax, the test executor automatically:

1. Detects if the condition matches a registered BDD pattern
2. Extracts parameters from the condition text
3. Generates the corresponding bash command using the pattern's template
4. Includes the generated command in the test execution script

**Example:**

```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""
```

During test execution, this is automatically converted to:
```bash
mkdir -p "/tmp/test"
```

## Built-in Step Patterns

The system includes 23 built-in BDD step patterns covering common test setup operations:

### File Operations

#### 1. Create File with Content
**Pattern:** `create file "<path>" with content:`

**Parameters:**
- `path`: File path (quoted string)

**Generated Command:** `touch "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "create file \"/tmp/config.txt\" with content:"
    - "append \"server=localhost\" to file \"/tmp/config.txt\""
    - "append \"port=8080\" to file \"/tmp/config.txt\""
```

**Note:** This creates an empty file. Use the "append to file" pattern to add content.

#### 2. Check File Exists
**Pattern:** `file "<path>" should exist`

**Parameters:**
- `path`: File path to check (quoted string)

**Generated Command:** `test -f "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "file \"/etc/passwd\" should exist"
```

#### 3. Check File Contains Text
**Pattern:** `file "<path>" should contain "<text>"`

**Parameters:**
- `path`: File path (quoted string)
- `text`: Text to search for (quoted string)

**Generated Command:** `grep -q "{text}" "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "file \"/tmp/log.txt\" should contain \"SUCCESS\""
```

#### 4. Append to File
**Pattern:** `append "<text>" to file "<path>"`

**Parameters:**
- `text`: Text to append (quoted string)
- `path`: File path (quoted string)

**Generated Command:** `echo "{text}" >> "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "append \"Additional line\" to file \"/tmp/notes.txt\""
```

#### 5. Replace in File
**Pattern:** `replace "<pattern>" with "<replacement>" in file "<path>"`

**Parameters:**
- `pattern`: Text pattern to find (quoted string)
- `replacement`: Replacement text (quoted string)
- `path`: File path (quoted string)

**Generated Command:** `sed -i 's/{pattern}/{replacement}/g' "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "replace \"localhost\" with \"192.168.1.100\" in file \"/etc/hosts\""
```

#### 6. Change File Permissions
**Pattern:** `change permissions of "<path>" to <mode>`

**Parameters:**
- `path`: File path (quoted string)
- `mode`: Octal permission mode (3-4 digits, unquoted)

**Generated Command:** `chmod {mode} "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "change permissions of \"/tmp/script.sh\" to 755"
```

### Directory Operations

#### 7. Create Directory
**Pattern:** `create directory "<path>"`

**Parameters:**
- `path`: Directory path (quoted string)

**Generated Command:** `mkdir -p "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test/nested/dirs\""
```

#### 8. Remove Directory
**Pattern:** `remove directory "<path>"`

**Parameters:**
- `path`: Directory path (quoted string)

**Generated Command:** `rm -rf "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "remove directory \"/tmp/old_test_data\""
```

#### 9. List Directory Contents
**Pattern:** `list contents of directory "<path>"`

**Parameters:**
- `path`: Directory path (quoted string)

**Generated Command:** `ls -la "{path}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "list contents of directory \"/tmp\""
```

### Environment Variables

#### 10. Set Environment Variable
**Pattern:** `set environment variable "<name>" to "<value>"`

**Parameters:**
- `name`: Variable name (quoted string)
- `value`: Variable value (quoted string)

**Generated Command:** `export {name}={value}`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "set environment variable \"TEST_ENV\" to \"production\""
```

#### 11. Unset Environment Variable
**Pattern:** `unset environment variable "<name>"`

**Parameters:**
- `name`: Variable name (quoted string)

**Generated Command:** `unset {name}`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "unset environment variable \"DEBUG\""
```

### Process Management

#### 12. Check Process Running
**Pattern:** `process "<process_name>" should be running`

**Parameters:**
- `process_name`: Process name to check (quoted string)

**Generated Command:** `pgrep -f "{process_name}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "process \"nginx\" should be running"
```

#### 13. Kill Process
**Pattern:** `kill process "<process_name>"`

**Parameters:**
- `process_name`: Process name to terminate (quoted string)

**Generated Command:** `pkill -f "{process_name}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "kill process \"old_daemon\""
```

### Network Operations

#### 14. Ping Device
**Pattern:** `ping device "<ip>" with <retries> retries`

**Parameters:**
- `ip`: IP address or hostname (quoted string)
- `retries`: Number of ping attempts (numeric, unquoted)

**Generated Command:** `ping -c {retries} "{ip}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "ping device \"192.168.1.1\" with 3 retries"
```

#### 15. Check Port Open
**Pattern:** `port <port> on "<host>" should be open`

**Parameters:**
- `port`: Port number (numeric, unquoted)
- `host`: Host address (quoted string)

**Generated Command:** `nc -z "{host}" {port}`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "port 80 on \"localhost\" should be open"
```

#### 16. HTTP Request
**Pattern:** `send <method> request to "<url>"`

**Parameters:**
- `method`: HTTP method - GET, POST, PUT, or DELETE (unquoted)
- `url`: Target URL (quoted string)

**Generated Command:** `curl -X {method} "{url}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "send GET request to \"http://api.example.com/status\""
```

### Timing Operations

#### 17. Wait for Seconds
**Pattern:** `wait for <seconds> seconds?`

**Parameters:**
- `seconds`: Number of seconds (numeric, unquoted)

**Generated Command:** `sleep {seconds}`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "wait for 5 seconds"
    - "wait for 1 second"
```

**Note:** Pattern supports both singular "second" and plural "seconds".

#### 18. Wait Until File Exists
**Pattern:** `wait until file "<path>" exists with timeout <timeout> seconds`

**Parameters:**
- `path`: File path to wait for (quoted string)
- `timeout`: Maximum wait time in seconds (numeric, unquoted)

**Generated Command:** `timeout {timeout} bash -c 'while [ ! -f "{path}" ]; do sleep 1; done'`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "wait until file \"/tmp/ready.flag\" exists with timeout 30 seconds"
```

### User Management

#### 19. Create User
**Pattern:** `create user "<username>" with uid <uid>`

**Parameters:**
- `username`: Username (quoted string)
- `uid`: User ID number (numeric, unquoted)

**Generated Command:** `useradd -u {uid} "{username}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "create user \"testuser\" with uid 1001"
```

#### 20. Delete User
**Pattern:** `delete user "<username>"`

**Parameters:**
- `username`: Username to delete (quoted string)

**Generated Command:** `userdel "{username}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "delete user \"olduser\""
```

### System Services

#### 21. Restart Service
**Pattern:** `restart service "<service_name>"`

**Parameters:**
- `service_name`: Service name (quoted string)

**Generated Command:** `systemctl restart "{service_name}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "restart service \"nginx\""
```

### Archive Operations

#### 22. Extract Archive
**Pattern:** `extract archive "<archive_path>" to "<destination>"`

**Parameters:**
- `archive_path`: Path to archive file (quoted string)
- `destination`: Destination directory (quoted string)

**Generated Command:** `tar -xzf "{archive_path}" -C "{destination}"`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "extract archive \"/tmp/data.tar.gz\" to \"/opt/app\""
```

#### 23. Create Archive
**Pattern:** `create archive "<archive_path>" from directory "<source_directory>"`

**Parameters:**
- `archive_path`: Output archive file path (quoted string)
- `source_directory`: Source directory to archive (quoted string)

**Generated Command:** `tar -czf "{archive_path}" -C "{source_directory}" .`

**Example:**
```yaml
initial_conditions:
  eUICC:
    - "create archive \"/tmp/backup.tar.gz\" from directory \"/opt/data\""
```

## Parameter Syntax and Quoting Rules

### String Parameters (Quoted)

String parameters **must** be enclosed in double quotes in the BDD pattern:

**Correct:**
```yaml
- "create directory \"/tmp/test\""
- "set environment variable \"PATH\" to \"/usr/local/bin\""
```

**Incorrect:**
```yaml
- "create directory /tmp/test"  # Missing quotes around path
- 'create directory "/tmp/test"'  # Using single quotes for YAML string
```

### Numeric Parameters (Unquoted)

Numeric parameters should **not** be quoted in the BDD pattern:

**Correct:**
```yaml
- "wait for 5 seconds"
- "ping device \"192.168.1.1\" with 3 retries"
- "change permissions of \"/tmp/script.sh\" to 755"
```

**Incorrect:**
```yaml
- "wait for \"5\" seconds"  # Don't quote numbers
- "change permissions of \"/tmp/script.sh\" to \"755\""  # Don't quote numeric mode
```

### Enumerated Values (Unquoted)

HTTP methods and other enumerated values should be unquoted:

**Correct:**
```yaml
- "send GET request to \"http://example.com\""
- "send POST request to \"http://api.example.com/data\""
```

**Incorrect:**
```yaml
- "send \"GET\" request to \"http://example.com\""  # Don't quote method
```

### YAML String Escaping

Since the BDD pattern is itself a YAML string, you need to escape quotes:

**In YAML:**
```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""  # Backslash escapes inner quotes
```

**Alternative using literal style:**
```yaml
initial_conditions:
  eUICC:
    - 'create directory "/tmp/test"'  # Single quotes allow inner double quotes
```

### Special Characters in Parameters

If your parameter values contain special characters, ensure proper escaping:

**Spaces in paths:**
```yaml
- "create directory \"/tmp/my test directory\""
```

**Special bash characters:**
```yaml
- "append \"Line with $var and \\n newline\" to file \"/tmp/log.txt\""
```

## Adding Custom Patterns

You can extend the BDD system with custom patterns by editing the `data/bdd_step_definitions.toml` file.

### TOML File Structure

```toml
[[step]]
name = "my_custom_step"
pattern = "^my pattern with \"([^\"]+)\" and (\\d+)$"
command_template = "my_command {param1} {param2}"
description = "Description of what this step does"
parameters = ["param1", "param2"]
```

### Field Descriptions

- **name**: Unique identifier for the step (lowercase with underscores)
- **pattern**: Regular expression pattern to match the BDD text
  - Must start with `^` and end with `$`
  - Use `([^"]+)` for quoted string parameters
  - Use `(\d+)` for numeric parameters
  - Use `(GET|POST|PUT|DELETE)` for enumerated options
- **command_template**: Bash command template with `{parameter_name}` placeholders
- **description**: Human-readable description of the step's purpose
- **parameters**: Ordered list of parameter names (must match capture groups)

### Pattern Examples

**Quoted string parameter:**
```toml
pattern = "^my action \"([^\"]+)\"$"
```
Matches: `my action "some value"`

**Numeric parameter:**
```toml
pattern = "^repeat (\\d+) times$"
```
Matches: `repeat 5 times`

**Multiple parameters:**
```toml
pattern = "^copy from \"([^\"]+)\" to \"([^\"]+)\"$"
```
Matches: `copy from "/source" to "/dest"`

**Optional text with regex:**
```toml
pattern = "^wait for (\\d+) seconds?$"
```
Matches: `wait for 1 second` OR `wait for 5 seconds`

### Complete Custom Step Example

Add this to `data/bdd_step_definitions.toml`:

```toml
[[step]]
name = "docker_run_container"
pattern = "^run docker container \"([^\"]+)\" with image \"([^\"]+)\"$"
command_template = "docker run --name {container_name} {image}"
description = "Starts a Docker container with the specified name and image"
parameters = ["container_name", "image"]
```

Then use it in your test cases:

```yaml
initial_conditions:
  eUICC:
    - "run docker container \"my-test-container\" with image \"nginx:latest\""
```

This will generate:
```bash
docker run --name my-test-container nginx:latest
```

### Parameter Order

The order of parameters in the `parameters` array must match the order of capture groups in the regex pattern:

```toml
pattern = "^action \"([^\"]+)\" with (\\d+) and \"([^\"]+)\"$"
parameters = ["first", "second", "third"]
#            capture 1 ↑    ↑ capture 2    ↑ capture 3
```

### Testing Custom Patterns

After adding a custom pattern:

1. **Validate TOML syntax:**
   ```bash
   cargo test bdd_step_registry_load_from_toml
   ```

2. **Test pattern matching:**
   ```bash
   cargo test
   ```

3. **Try in a test case:**
   ```yaml
   requirement: TEST001
   item: 1
   tc: 1
   id: custom_pattern_test
   description: Testing custom BDD pattern
   general_initial_conditions:
     eUICC:
       - "run docker container \"test\" with image \"alpine\""
   ```

4. **Generate test script to verify:**
   ```bash
   test-executor generate testcases/custom_pattern_test.yaml output.sh
   cat output.sh  # Check if command was generated correctly
   ```

## Where BDD Patterns Are Applied

BDD patterns are automatically detected and converted in three locations:

### 1. General Initial Conditions (Top-level)
```yaml
general_initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""
  Platform:
    - "set environment variable \"MODE\" to \"test\""
```

### 2. Initial Conditions (Top-level)
```yaml
initial_conditions:
  eUICC:
    - "wait for 5 seconds"
    - "file \"/tmp/ready\" should exist"
```

### 3. Sequence-level Initial Conditions
```yaml
test_sequences:
  - id: 1
    name: My Sequence
    description: Test sequence
    initial_conditions:
      eUICC:
        - "restart service \"nginx\""
        - "wait for 2 seconds"
    steps: []
```

## Non-BDD Conditions

If a condition doesn't match any BDD pattern, it is treated as a comment in the generated script:

```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""           # BDD - generates: mkdir -p /tmp/test
    - "Ensure the device is powered on"         # Not BDD - generates comment
    - "The network should be configured"        # Not BDD - generates comment
```

Generated script:
```bash
# eUICC: create directory "/tmp/test"
mkdir -p /tmp/test
# eUICC: Ensure the device is powered on
# eUICC: The network should be configured
```

This allows mixing executable BDD patterns with human-readable documentation.

## Best Practices

### 1. Use BDD for Automatable Setup
Use BDD patterns for conditions that can be automated:

**Good:**
```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""
    - "set environment variable \"TEST_MODE\" to \"enabled\""
    - "wait for 3 seconds"
```

### 2. Use Plain Text for Manual/Documentation Steps
Use plain text for conditions requiring manual intervention or context:

**Good:**
```yaml
initial_conditions:
  eUICC:
    - "Device is powered on and connected via USB"
    - "Operator has logged into the admin console"
```

### 3. Combine Both for Comprehensive Setup
Mix BDD and plain text as needed:

**Good:**
```yaml
initial_conditions:
  eUICC:
    - "Device is in factory reset state"              # Manual prerequisite
    - "create directory \"/tmp/logs\""                # Automated setup
    - "set environment variable \"LOG_LEVEL\" to \"debug\""  # Automated setup
    - "Operator has SIM card ejector tool ready"      # Manual prerequisite
```

### 4. Be Specific with Paths and Values
Use absolute paths and explicit values:

**Good:**
```yaml
- "create directory \"/tmp/test/output\""
- "set environment variable \"API_KEY\" to \"test_key_12345\""
```

**Avoid:**
```yaml
- "create directory \"test\""  # Relative path - unclear
- "set environment variable \"KEY\" to \"key\""  # Vague value
```

### 5. Keep Patterns Readable
Even though BDD patterns are executable, they should remain human-readable:

**Good:**
```yaml
- "wait for 5 seconds"
- "restart service \"nginx\""
```

**Avoid overly technical:**
```yaml
- "execute /bin/sleep with parameter 5"
- "systemctl restart nginx"  # Just use the command directly if not BDD
```

## Troubleshooting

### Pattern Not Matching

If your BDD pattern isn't being converted to a command:

1. **Check quotes**: Ensure string parameters are in double quotes
   ```yaml
   # Wrong: create directory /tmp/test
   # Right: create directory "/tmp/test"
   ```

2. **Check spelling**: Pattern matching is exact
   ```yaml
   # Wrong: make directory "/tmp/test"
   # Right: create directory "/tmp/test"
   ```

3. **Check parameter format**: Numbers should be unquoted
   ```yaml
   # Wrong: wait for "5" seconds
   # Right: wait for 5 seconds
   ```

4. **Verify pattern exists**: Check `data/bdd_step_definitions.toml`

### Testing Pattern Matching

Use the test suite to verify patterns:

```bash
# Run BDD parser tests
cargo test bdd_parser

# Run specific test
cargo test test_bdd_step_registry_try_parse_create_directory
```

### Debugging Generated Scripts

Generate a test script to see how conditions are converted:

```bash
test-executor generate testcases/my_test.yaml output.sh
cat output.sh  # Review generated commands
```

Look for:
- BDD patterns converted to bash commands
- Non-BDD conditions as comments
- Proper quoting and escaping

## Examples

### Complete Test Case with BDD Conditions

```yaml
requirement: REQ001
item: 1
tc: 1
id: complete_bdd_example
description: Comprehensive BDD initial conditions example

general_initial_conditions:
  Platform:
    - "System is in a clean state"
    - "create directory \"/tmp/test_data\""
    - "set environment variable \"TEST_ENV\" to \"staging\""

initial_conditions:
  eUICC:
    - "create directory \"/tmp/logs\""
    - "wait for 2 seconds"
    - "file \"/etc/config.conf\" should exist"

test_sequences:
  - id: 1
    name: Setup and Verify
    description: Initial setup with verification
    initial_conditions:
      eUICC:
        - "restart service \"test-daemon\""
        - "wait until file \"/tmp/ready.flag\" exists with timeout 10 seconds"
        - "port 8080 on \"localhost\" should be open"
    steps:
      - step: 1
        description: Execute test command
        command: echo "Test running"
        expected:
          result: "0"
          output: "Test running"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"Test running\" ]"
```

This generates a script with all BDD patterns converted to executable bash commands while preserving plain-text conditions as comments.

## Security Considerations

### Shell Command Injection

BDD patterns generate shell commands by substituting parameters extracted from the natural language statements directly into command templates. While the predefined patterns in `data/bdd_step_definitions.toml` are designed to be safe, you should be aware of the following when creating custom patterns or using BDD conditions:

1. **Parameter Validation**: Parameters extracted from BDD statements are inserted directly into command templates without additional escaping. The regex patterns in the TOML file control what characters are allowed in each parameter.

2. **Safe Pattern Design**: When creating custom patterns, use restrictive regex capture groups:
   - For paths: `([^"]+)` - Allows any character except quotes
   - For numbers: `(\d+)` - Only digits
   - For specific values: `(GET|POST|PUT|DELETE)` - Enumerated options only

3. **Avoid User Input**: BDD conditions should be authored by trusted test developers, not sourced from untrusted user input. The test YAML files should be version-controlled and reviewed.

4. **Command Template Safety**: Built-in command templates use parameter substitution without shell evaluation. For example, `mkdir -p {path}` will create a directory with the exact path provided, including any special characters.

**Example of Safe Pattern:**
```toml
[[step]]
name = "create_directory"
pattern = "^create directory \"([^\"]+)\"$"  # Only allows non-quote characters
command_template = "mkdir -p {path}"
parameters = ["path"]
```

**Avoid patterns that could enable injection:**
```toml
# BAD - Don't do this!
[[step]]
pattern = "^run command (.+)$"  # Too permissive
command_template = "{command}"  # Direct execution
```
