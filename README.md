# Test Case Manager

A comprehensive CLI tool for managing test cases in YAML format with interactive workflows, fuzzy search, git integration, and test verification capabilities.

## Features

- **Interactive Test Case Creation**: Build test cases with guided prompts
- **Database-backed Condition Selection**: Fuzzy search through existing conditions from test database
- **Test Sequence Builder**: Create test sequences with automatic numbering and validation
- **Step Collection Loop**: Add steps to sequences with fuzzy search for existing steps
- **Git Integration**: Commit progress after each step or sequence
- **Schema Validation**: Validate test cases against a JSON schema with watch mode for continuous monitoring
- **Fuzzy Search**: Search through test cases, sequences, steps, and conditions
- **TTY Fallback**: Automatic detection of non-TTY environments (e.g., VS Code debug console) with graceful fallback to numbered selection
- **Recovery Mechanism**: Automatically saves progress after each operation and can resume from saved state if interrupted
- **Test Verification**: Batch verification mode that processes test execution logs and generates reports with JUnit XML output for CI/CD integration
- **Watch Mode**: Continuously monitor directories for file changes with automatic validation and instant feedback
- **Variables and Data Passing**: Capture dynamic values from command output and pass data between test steps using regex patterns and variable substitution. See [Variables and Data Passing Documentation](docs/VARIABLE_PASSING.md) for details

## Installation

### Docker (Recommended)

The easiest way to get started is with Docker:

```bash
# Build the image
./scripts/build-docker.sh

# Verify the build
./scripts/verify-docker.sh

# Run interactively
docker run -it --rm testcase-manager:latest
```

See [docs/DOCKER.md](docs/DOCKER.md) for complete Docker documentation.

### From Source

```bash
# Build all binaries
make build

# Run tests
make test

# Run linter
make lint
```

## Executables

This project provides multiple executable binaries for comprehensive test case management

- editor: TCM/TestCase Manager: Interactive test case creation and management
- **tcm** (Test Case Manager): Alias to the editor
- **testcase-manager**: Alias to the editor
- **test-executor**: Automated test execution with JSON logging
- **test-verify**: Test verification tool for validating test execution logs against test cases
- **test-orchestrator**: Coordinate complex test workflows
- **validate-yaml**: YAML validation tool
- **script-cleanup**: Terminal script capture cleanup tool for removing ANSI codes and control characters
- **validate-json**: JSON validation tool
- **trm**: Test Run Manager
- **editor**: Interactive test case editor



### 1. editor (Test Case Manager / TCM)

**Purpose**: Primary interactive tool for creating and managing test cases in YAML format.

**Input**: Command-line arguments and interactive prompts  
**Output**: YAML test case files, git commits, validation reports

**Key Subcommands**:
- `create` - Create a new test case with optional ID
- `create-interactive` - Interactive test case creation with metadata prompts
- `build-sequences` - Build test sequences interactively with git commits
- `build-sequences-with-steps` - Build sequences with step collection loops
- `add-steps` - Add steps to existing sequences
- `complete` - Full workflow: metadata, conditions, sequences, and steps
- `edit` - Edit existing test cases (supports fuzzy finder)
- `list` - List test cases with filtering (tag, status, priority)
- `view` - View test case details
- `delete` - Delete test cases
- `validate` - Validate test case files against schema
- `search` - Fuzzy search through test cases
- `export` - Export test cases to suite files
- `import` - Import test cases from suite files
- `parse-general-conditions` - Parse conditions from database with fuzzy search
- `parse-initial-conditions` - Parse initial conditions from database
- `git` - Git operations (add, commit, status, log)
- `init` - Initialize new test case repository

**Example Usage**:
```bash
# Interactive workflow
editor complete --output testcases/my_test.yml

# Create with specific metadata
editor create-interactive --path ./testcases

# Fuzzy search and edit
editor edit --fuzzy

# Validate all test cases
editor validate --all
```

### 2. test-executor

## Test Case Manager (tcm) Commands
**Purpose**: Generate bash scripts from YAML test cases and execute tests with automated verification.
**Input**: YAML test case files  
**Output**: Bash scripts, JSON execution logs, test results (pass/fail)

**Subcommands**:
- `generate` - Generate bash script from YAML test case
- `execute` - Execute test case and generate JSON execution log

**Key Features**:
- **Variables and Data Passing**: Capture values from command output and pass data between test steps. See [Variables and Data Passing Documentation](docs/VARIABLE_PASSING.md) for complete details on variable capture, substitution, and scope rules

**Example Usage**:
```bash
# Generate bash script
test-executor generate testcase.yml --output test.sh

# Generate script with JSON log template
test-executor generate testcase.yml --output test.sh --json-log

# Execute test directly
test-executor execute testcase.yml
```

### 3. test-verify

**Purpose**: Verify test execution logs against expected test case definitions.

**Input**: Test execution log files (JSON), test case YAML files  
**Output**: Verification reports (text, JSON, JUnit XML)

**Subcommands**:
- `single` - Verify single execution log against test case
- `batch` - Batch verify multiple logs with aggregated reports
- `parse-log` - Parse and display log contents
- `clean` - Clean and display execution log

**Example Usage**:
```bash
# Verify single test
test-verify single --log exec.log --test-case-id TC001

# Batch verify with JUnit output
test-verify batch --logs logs/*.log --format junit --output report.xml

# Parse log file
test-verify parse-log --log exec.log --format json
```

### 4. test-orchestrator

**Purpose**: Orchestrate parallel test execution with retry policies and progress reporting.

**Input**: Test case IDs or fuzzy selection  
**Output**: Execution logs, progress reports, result summaries

**Subcommands**:
- `run` - Execute specific test cases (supports fuzzy selection)
- `run-all` - Execute all available test cases
- `status` - Show orchestrator status
- `workers` - Manage worker threads

**Key Features**:
- Parallel execution with configurable worker pool
- Automatic retry with exponential backoff
- Real-time progress reporting
- Result tracking and report generation

**Example Usage**:
```bash
# Run specific tests with 8 workers
test-orchestrator run TC001 TC002 --workers 8

# Run with retry and fuzzy selection
test-orchestrator run --fuzzy --retry --max-retries 3

# Run all tests with progress tracking
test-orchestrator run-all --workers 4 --report --save
```

### 5. trm (Test Run Manager)

**Purpose**: Manage and track test run execution records.

**Input**: Test case storage directory  
**Output**: Test run listings, execution history

**Subcommands**:
- `list` - List all test runs with statistics
- `add` - Add new test run record

**Example Usage**:
```bash
# List all test runs
trm list

# Add test run interactively
trm add
```

### 6. validate-yaml

**Purpose**: Validate YAML files against JSON schema definitions with optional watch mode for continuous monitoring.

**Input**: One or more YAML files, JSON schema file  
**Output**: Validation results with detailed error messages, live updates in watch mode

**Key Features**:
- Single or multi-file validation
- Watch mode for continuous monitoring (Linux/macOS only)
- Detailed error messages with JSON paths
- Color-coded output (green for pass, red for fail)
- Automatic re-validation on file changes
- Full validation when all changed files pass

**Example Usage**:
```bash
# Validate single YAML against schema
validate-yaml testcase.yml --schema schema.json

# Validate multiple YAML files
validate-yaml testcase1.yml testcase2.yml testcase3.yml --schema schema.json

# Watch mode - monitor files for changes and auto-validate (Linux/macOS only)
validate-yaml testcase.yml --schema schema.json --watch

# Watch multiple files
validate-yaml testcases/*.yml --schema schema.json --watch

# Verbose validation with detailed logging
validate-yaml testcase.yml --schema schema.json --verbose
```

**Watch Mode Behavior**:
- Performs initial validation on all specified files
- Monitors files for changes (modifications only)
- Re-validates changed files immediately with instant feedback
- When all changed files pass, automatically runs full validation on all files
- Uses debounced event handling to avoid duplicate validations
- **Note**: Watch mode is disabled on Windows due to platform limitations

**Platform Support**:
- **Linux/macOS**: Full support including watch mode
- **Windows**: Validation works, but `--watch` flag is not available

**See Also**: [docs/VALIDATE_YAML_QUICK_REF.md](docs/VALIDATE_YAML_QUICK_REF.md) for comprehensive documentation and examples.

### 7. validate-json

**Purpose**: Validate JSON files against JSON schema definitions.

**Input**: JSON file, JSON schema file  
**Output**: Validation results with detailed error messages

**Example Usage**:
```bash
# Validate JSON against schema
validate-json data.json schema.json

# Verbose validation
validate-json data.json schema.json --verbose
```

### Parse Conditions from Database

Parse and select general initial conditions or initial conditions from existing test cases using fuzzy search:

#### Parse General Initial Conditions

```bash
testcase-manager parse-general-conditions --database <path>
```

This command will:
1. Load all test cases from the database directory (default: `data`)
2. Extract all unique general initial conditions
3. Present them in a fuzzy search interface
4. Allow selection of multiple conditions
5. Add selected conditions to a new test case
6. Optionally commit to git

#### Parse Initial Conditions

```bash
testcase-manager parse-initial-conditions --database <path>
```

This command will:
1. Load all test cases from the database directory (default: `data`)
2. Extract all unique initial conditions (including sequence-level conditions)
3. Present them in a fuzzy search interface
4. Allow selection of multiple conditions
5. Add selected conditions to a new test case
6. Optionally commit to git

The database search extracts conditions from:
- Top-level general initial conditions
- Top-level initial conditions
- Sequence-level initial conditions

### Build Test Sequences with Step Collection

Build test sequences interactively with a step collection loop that commits each step:

```bash
testcase-manager build-sequences-with-steps
```

This command will:
1. Prompt for test case metadata
2. Add general initial conditions
3. Add initial conditions
4. For each test sequence:
   - Create a sequence with ID, name, and description
   - Optionally commit the sequence
   - Add steps to the sequence with:
     - Fuzzy search for existing step descriptions
     - Prompt for step number, manual flag, description, command, and expected results
     - Validate step structure against schema
     - Append step to sequence
     - Save file
     - Commit progress

### Add Steps to a Sequence

Add steps to an existing sequence with git commits:

```bash
testcase-manager add-steps [--sequence-id <ID>]
```

### Build Sequences Only

Build test sequences without steps:

```bash
testcase-manager build-sequences
```

## BDD Initial Conditions

The Test Case Manager supports Behavior-Driven Development (BDD) style initial conditions that are automatically converted to executable commands during test execution. This allows you to write human-readable conditions that are simultaneously executable.

### Quick Example

```yaml
initial_conditions:
  eUICC:
    - "create directory \"/tmp/test\""
    - "set environment variable \"TEST_MODE\" to \"enabled\""
    - "wait for 5 seconds"
```

These BDD patterns are automatically converted to bash commands:
```bash
mkdir -p "/tmp/test"
export TEST_MODE=enabled
sleep 5
```

### Features

- **23 built-in BDD step patterns** covering file operations, directory management, environment variables, process management, network operations, timing, user management, system services, and archive operations
- **Automatic command generation** from natural language patterns
- **Custom pattern support** via `data/bdd_step_definitions.toml`
- **Mix BDD and plain text** - Non-matching patterns become comments
- **Works in all initial condition locations**: general_initial_conditions, initial_conditions, and sequence-level initial_conditions

### Common Patterns

```yaml
# File Operations
- "create file \"/tmp/config.txt\" with content:"
- "file \"/etc/passwd\" should exist"
- "append \"text\" to file \"/tmp/log.txt\""

# Directory Operations
- "create directory \"/tmp/test\""
- "remove directory \"/tmp/old_data\""

# Environment Variables
- "set environment variable \"VAR\" to \"value\""
- "unset environment variable \"OLD_VAR\""

# Process Management
- "process \"nginx\" should be running"
- "kill process \"old_daemon\""

# Network Operations
- "ping device \"192.168.1.1\" with 3 retries"
- "port 80 on \"localhost\" should be open"
- "send GET request to \"http://api.example.com/status\""

# Timing
- "wait for 5 seconds"
- "wait until file \"/tmp/ready\" exists with timeout 30 seconds"

# System Services
- "restart service \"nginx\""
```

For complete documentation of all 23 patterns, parameter syntax, custom pattern creation, and best practices, see [BDD Initial Conditions Documentation](docs/BDD_INITIAL_CONDITIONS.md).

## File Validation and Watch Mode

The project includes a powerful file validation system with watch mode for continuous monitoring:

### Basic Validation

Validate all YAML files matching a pattern:

```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh
```

### Watch Mode

Monitor directories for file changes and automatically validate modified files:

```bash
./scripts/validate-files.sh \
    --pattern '\.ya?ml$' \
    --validator ./scripts/validate-yaml-wrapper.sh \
    --watch
```

**Watch mode features:**
- Runs initial validation on all matching files
- Monitors directory recursively for changes (modifications, creations, deletions)
- Instantly validates changed files
- Displays live results with color-coded output (green for pass, red for fail)
- Maintains persistent cache across sessions
- Auto-cleans cache for deleted files

**Requirements:**
- Linux: `sudo apt-get install inotify-tools`
- macOS: `brew install fswatch`

See [Watch Mode Guide](scripts/WATCH_MODE_GUIDE.md) for detailed documentation.

**Watch Mode for validate-yaml Binary**: The `validate-yaml` binary includes a built-in `--watch` flag for monitoring specific YAML files. See [validate-yaml Quick Reference](docs/VALIDATE_YAML_QUICK_REF.md) for usage details and [Watch Mode Comparison](docs/WATCH_MODE_COMPARISON.md) to choose between the two watch mode implementations.

## Step Collection Loop Features

The step collection loop includes:

1. **Fuzzy Search**: Search existing step descriptions to reuse common patterns
2. **Automatic Numbering**: Steps are automatically numbered sequentially
3. **Field Collection**: 
   - Step number (auto-generated)
   - Manual flag (optional, true/false)
   - Description (with fuzzy search)
   - Command
   - Expected results (result, output, optional success flag)
4. **Schema Validation**: Each step is validated before being added
5. **Git Commits**: Optionally commit after each step is added
6. **File Persistence**: File is saved after each step

## Recovery Mechanism

The recovery mechanism automatically saves the current test case structure and validation errors after each operation:

- **Automatic Saves**: After each metadata entry, initial conditions, sequence, or step addition
- **Error Tracking**: Captures validation errors with field paths for inline display
- **Resume on Startup**: Detects `.recovery.json` file on startup and prompts to resume
- **Pre-populated Fields**: Recovered values are shown as editable initial text in prompts (Enter confirms, user can edit/delete)
- **Error Annotations**: Validation errors from previous attempts are displayed inline
- **Auto-cleanup**: Recovery file is automatically deleted on successful completion

To use recovery:

1. Start any interactive workflow (`create-interactive`, `build-sequences`, `complete`, etc.)
2. If the workflow is interrupted, the progress is saved to `.recovery.json`
3. Restart the same command to be prompted to resume from the saved state
4. Choose to resume or start fresh (optionally deleting the recovery file)

## Step Schema

Steps follow this structure:

```yaml
steps:
  - step: 1
    manual: true  # optional
    description: "Step description"
    command: "ssh"
    expected:
      success: false  # optional
      result: "SW=0x9000"
      output: "This operation was successful."
    verification:  # optional, bash expressions for automated testing
      result: "[ $EXIT_CODE -eq 0 ]"
      output: "[ \"$COMMAND_OUTPUT\" = \"SW=0x9000\" ]"
```

### Verification Field

The `verification` field (optional) contains bash expressions used by the `test-executor` binary for automated test execution:

- **result**: Bash expression to verify the exit code of the command
- **output**: Bash expression to verify the command output from the variable
- **output_file** (optional): Bash expression to verify the command output from the log file (takes precedence over `output` if present)

Available variables in verification expressions:
- `$EXIT_CODE`: The exit code of the executed command
- `$COMMAND_OUTPUT`: The stdout output from the executed command (used in `output` verification)
- `$LOG_FILE`: Path to the log file containing command output (used in `output_file` verification)
- `$?`: Alternative to `$EXIT_CODE` (shell exit status variable)

## TTY Fallback

The fuzzy search automatically detects non-TTY environments (like VS Code debug console) and falls back to numbered selection:

**Normal Terminal:**
- Interactive fuzzy search with keyboard navigation

**Non-TTY Environment (VS Code, CI/CD, etc.):**
- Numbered list display
- Simple numeric input (e.g., "1" to select first option)
- Multi-select with space-separated numbers (e.g., "1 3 5")

For more details, see [docs/TTY_FALLBACK.md](docs/TTY_FALLBACK.md)

**Try the demo:**
```bash
cargo run --example tty_fallback_demo
```

## Test Executor

The `test-executor` binary provides automated test execution from YAML test case files with JSON execution logging.

### Commands

#### Generate Test Scripts

Generate a bash script from a YAML test case:

```bash
test-executor generate <input.yaml> <output.sh>
```

This command:
1. Parses the YAML test case file
2. Generates a bash script with verification logic
3. Saves the executable script to the specified output path

**Generate with JSON Log Template:**

```bash
test-executor generate <input.yaml> --output <output.sh> --json-log
```

This creates both a bash script and an empty JSON log template file alongside it.

#### Execute Tests

Execute a test case directly and generate JSON execution logs:

```bash
test-executor execute <input.yaml>
```

This command:
1. Generates a temporary bash script from the YAML
2. Executes the script with proper error handling
3. Reports test results (PASS/FAIL for each step)
4. **Generates a JSON execution log file** (`<test_case_id>_execution_log.json`)
5. Exits with 0 on success, non-zero on failure

### JSON Execution Log Format

When executing tests, `test-executor` automatically generates a structured JSON log file containing detailed execution information for each test step.

#### Log File Naming

The log file is named: `<TEST_CASE_ID>_execution_log.json`

For example, if your test case ID is `TC_001`, the log file will be `TC_001_execution_log.json`.

#### JSON Schema

Each log file contains an array of execution entries following this schema:

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo 'Hello World'",
    "exit_code": 0,
    "output": "Hello World\n",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]
```

**Field Descriptions:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_sequence` | integer | Yes | Sequence number (must be non-negative) |
| `step` | integer | Yes | Step number within the sequence (must be non-negative) |
| `command` | string | Yes | The exact command that was executed |
| `exit_code` | integer | Yes | Exit code returned by the command execution |
| `output` | string | Yes | Standard output captured from the command |
| `timestamp` | string | No | ISO 8601 timestamp of execution (RFC 3339 format) |

**Schema Reference:** See `schemas/execution-log.schema.json` for the complete JSON schema definition.

#### Example JSON Execution Log

```json
[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo \"Hello World\"",
    "exit_code": 0,
    "output": "Hello World",
    "timestamp": "2024-01-15T10:30:00Z"
  },
  {
    "test_sequence": 1,
    "step": 2,
    "command": "/bin/true",
    "exit_code": 0,
    "output": "",
    "timestamp": "2024-01-15T10:30:01Z"
  },
  {
    "test_sequence": 1,
    "step": 3,
    "command": "/bin/false",
    "exit_code": 1,
    "output": "",
    "timestamp": "2024-01-15T10:30:02Z"
  },
  {
    "test_sequence": 2,
    "step": 1,
    "command": "echo \"Status: SUCCESS\"",
    "exit_code": 0,
    "output": "Status: SUCCESS",
    "timestamp": "2024-01-15T10:30:03Z"
  }
]
```

#### Interpreting Execution Log Entries

**Successful Execution:**
- `exit_code` is `0`
- `output` contains the command's standard output
- Sequential `timestamp` values show execution order

**Failed Execution:**
- `exit_code` is non-zero (typically 1-255)
- `output` may be empty or contain error messages
- Indicates the step did not complete as expected

**Manual Steps:**
- Manual steps (marked with `manual: true`) are NOT included in the execution log
- Only automated steps generate log entries

**Multiple Sequences:**
- Each sequence is identified by its `test_sequence` number
- Steps from different sequences are logged sequentially in execution order
- The combination of `test_sequence` and `step` uniquely identifies each log entry

**Empty Output:**
- An empty `output` string is valid (e.g., for commands like `/bin/true`)
- This is different from a missing or null value

#### Using JSON Logs with test-verify

The JSON execution logs generated by `test-executor` can be consumed by the `test-verify` tool for automated verification:

```bash
# Execute a test and generate JSON log
test-executor execute testcases/my_test.yml

# Verify the execution against the test case definition
test-verify single --log my_test_execution_log.json --test-case-id TC_001
```

See the [Test Verification](#test-verification-test-verify) section for more details on verification workflows.

### Verification Expressions

Verification expressions are bash conditional expressions that evaluate to true or false. They are used to validate test step results.

The `verification` field supports two types of output verification:
- **`output`**: Verifies output from the `$COMMAND_OUTPUT` variable (default)
- **`output_file`**: Verifies output from the `$LOG_FILE` file (optional, takes precedence if present)

When `output_file` is specified, the verification will read from the log file instead of the output variable. This is useful for:
- Large outputs that may not fit in shell variables
- Commands that write directly to files
- More reliable file-based verification

#### Basic Examples

**Exit Code Verification:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"  # Command succeeded
  output: "true"  # Always pass output check
```

**File-Based Output Verification:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "grep -q 'Hello World' <<< \"$COMMAND_OUTPUT\""  # Variable-based (default)
  output_file: "grep -q 'Hello World' \"$LOG_FILE\""      # File-based (takes precedence)
```

**String Comparison:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "[ \"$COMMAND_OUTPUT\" = \"expected output\" ]"  # Exact match
```

**Pattern Matching with Regex:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "[[ \"$COMMAND_OUTPUT\" =~ ^SUCCESS ]]"  # Starts with SUCCESS
```

#### Advanced Examples

**Multiple Conditions:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ] && [ -n \"$COMMAND_OUTPUT\" ]"  # Success and non-empty output
  output: "[[ \"$COMMAND_OUTPUT\" =~ OK ]] && [[ \"$COMMAND_OUTPUT\" =~ READY ]]"  # Contains both patterns
```

**Numeric Comparisons:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "[ \"$COMMAND_OUTPUT\" -gt 100 ]"  # Output is a number greater than 100
```

**String Length Checks:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "[ -n \"$COMMAND_OUTPUT\" ]"  # Output is not empty
```

**Complex Regex:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "[[ \"$COMMAND_OUTPUT\" =~ [0-9]+\\.[0-9]+\\.[0-9]+ ]]"  # Matches version pattern
```

**File System Checks:**
```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ] && [ -f /tmp/output.txt ]"  # File exists
  output: "true"
```

**Failure Cases:**
```yaml
verification:
  result: "[ $EXIT_CODE -ne 0 ]"  # Command should fail
  output: "[[ \"$COMMAND_OUTPUT\" =~ error ]]"  # Contains error message
```

### Best Practices for Verification Expressions

#### 1. Always Quote Variables

**Good:**
```bash
[ "$COMMAND_OUTPUT" = "test" ]
```

**Bad:**
```bash
[ $COMMAND_OUTPUT = "test" ]  # Can fail with spaces in output
```

#### 2. Use `[[` for Advanced Pattern Matching

The `[[` operator supports regex and is more robust:

```bash
[[ "$COMMAND_OUTPUT" =~ pattern ]]  # Regex matching
[[ "$COMMAND_OUTPUT" == *substring* ]]  # Glob pattern
```

#### 3. Prefer `$EXIT_CODE` over `$?`

While both work, `$EXIT_CODE` is more explicit and clearer:

**Good:**
```bash
[ $EXIT_CODE -eq 0 ]
```

**Acceptable:**
```bash
[ $? -eq 0 ]
```

#### 4. Handle Empty Output

Check for empty output to avoid false positives:

```bash
[ -n "$COMMAND_OUTPUT" ] && [[ "$COMMAND_OUTPUT" =~ pattern ]]
```

#### 5. Use `true` for Always-Pass Conditions

When you don't need to verify a particular aspect:

```yaml
verification:
  result: "[ $EXIT_CODE -eq 0 ]"
  output: "true"  # Don't care about output
```

#### 6. Escape Special Characters

In YAML, escape backslashes and quotes properly:

```yaml
verification:
  output: "[[ \"$COMMAND_OUTPUT\" =~ [0-9]+\\.[0-9]+ ]]"  # Note double backslash
```

### Common Bash Test Operators

**Numeric Comparisons:**
- `-eq`: Equal to
- `-ne`: Not equal to
- `-gt`: Greater than
- `-lt`: Less than
- `-ge`: Greater than or equal to
- `-le`: Less than or equal to

**String Comparisons:**
- `=`: Equal to (POSIX)
- `==`: Equal to (bash)
- `!=`: Not equal to
- `<`: Less than (lexicographical)
- `>`: Greater than (lexicographical)

**String Tests:**
- `-z "$str"`: String is empty
- `-n "$str"`: String is not empty

**File Tests:**
- `-f "$file"`: File exists and is a regular file
- `-d "$dir"`: Directory exists
- `-e "$path"`: Path exists (file or directory)
- `-r "$file"`: File is readable
- `-w "$file"`: File is writable
- `-x "$file"`: File is executable

**Logical Operators:**
- `&&`: AND
- `||`: OR
- `!`: NOT

### Example Test Case

```yaml
requirement: TEST001
item: 1
tc: 1
id: EXAMPLE_TEST
description: Example test with verification
test_sequences:
  - id: 1
    name: Basic Test Sequence
    description: Demonstrates verification usage
    initial_conditions: {}
    steps:
      - step: 1
        description: Test echo command
        command: echo 'hello world'
        expected:
          result: "0"
          output: "hello world"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"hello world\" ]"
      
      - step: 2
        description: Test grep with pattern
        command: echo 'Version 1.2.3' | grep -o '[0-9.]*'
        expected:
          result: "0"
          output: "1.2.3"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "[[ \"$COMMAND_OUTPUT\" =~ ^[0-9]+\\.[0-9]+\\.[0-9]+$ ]]"
      
      - step: 3
        description: Test file creation
        command: touch /tmp/testfile && echo 'created'
        expected:
          result: "0"
          output: "created"
        verification:
          result: "[ $EXIT_CODE -eq 0 ] && [ -f /tmp/testfile ]"
          output: "[ \"$COMMAND_OUTPUT\" = \"created\" ]"
```

## Script Capture Cleanup (script-cleanup)

The `script-cleanup` binary cleans terminal script capture output by removing ANSI escape codes, backspaces, and control characters to produce clean, readable text.

### Features

- **ANSI Code Removal**: Strips color codes, cursor movement, and terminal control sequences
- **Backspace Processing**: Processes backspace characters (`\x08`) and DEL characters (`\x7f`) to simulate actual text deletion
- **Control Character Filtering**: Removes non-printable control characters (except newlines, tabs, and carriage returns)
- **Clean Output**: Produces human-readable text from raw terminal captures

### Usage

```bash
# Build the binary
cargo build --release --bin script-cleanup
# Or use Makefile
make build-script-cleanup

# Clean a script capture file and write to output file
script-cleanup -i raw_terminal.log -o clean_output.txt

# Clean and output to stdout
script-cleanup -i raw_terminal.log

# Enable verbose logging
script-cleanup -i raw_terminal.log -o clean_output.txt --verbose
```

### Command-Line Options

- `-i, --input <INPUT_FILE>`: Path to the input file to clean (required)
- `-o, --output <OUTPUT_FILE>`: Path to the output file (optional, defaults to stdout)
- `-v, --verbose`: Enable verbose logging

### Before/After Examples

#### Example 1: Colored Terminal Output

**Before (raw capture):**
```
^[[32mSUCCESS^[[0m: Test passed
^[[31mERROR^[[0m: Connection failed
^[[1;33mWARNING^[[0m: Deprecated API
```

**After (cleaned):**
```
SUCCESS: Test passed
ERROR: Connection failed
WARNING: Deprecated API
```

#### Example 2: Backspace Corrections

**Before (raw capture with backspaces):**
```
Password: secrt^H^Hret
git statsu^H^Hus
```

**After (cleaned):**
```
Password: secret
git status
```

#### Example 3: Progress Indicators

**Before (raw capture):**
```
^[[32m[=====>    ]^[[0m 50%^M^[[32m[==========>]^[[0m 100%
```

**After (cleaned):**
```
[=====>    ] 50%
[==========>] 100%
```

#### Example 4: Mixed Terminal Output

**Before (raw capture):**
```
^[[2J^[[H^[[32muser@host^[[0m:~$ ls^H^Hpwd
/home/user/project
^[[32muser@host^[[0m:~$ echo ^Ghello^H
hello
```

**After (cleaned):**
```
user@host:~$ pwd
/home/user/project
user@host:~$ echo hello
```

### What Gets Cleaned

The tool processes the following:

1. **ANSI Escape Sequences**: All `\x1b[...` sequences including:
   - Color codes (foreground/background)
   - Text formatting (bold, italic, underline)
   - Cursor positioning and movement
   - Screen clearing and line erasing

2. **Backspace Processing**:
   - Backspace character (`\x08` or `^H`)
   - Delete character (`\x7f` or `DEL`)
   - Simulates actual character deletion

3. **Control Characters**: Removes control characters in range 0x00-0x1F (except `\n`, `\r`, `\t`) and 0x7F
   - Bell (`\x07` or `^G`)
   - Null (`\x00`)
   - Escape (`\x1b`)
   - Form feed, vertical tab, etc.

4. **Preserved Characters**:
   - Newlines (`\n`)
   - Carriage returns (`\r`)
   - Tabs (`\t`)
   - All printable ASCII and Unicode characters

### Use Cases

- **Test Logs**: Clean raw test execution logs captured with `script` command
- **Terminal Sessions**: Process interactive terminal session recordings
- **CI/CD Logs**: Clean build and deployment logs before archiving
- **Documentation**: Generate clean examples from terminal sessions
- **Debugging**: Make raw terminal output human-readable

### Integration Example

```bash
# Capture terminal session
script -q raw_session.log

# ... perform some operations ...

# Clean the captured output
script-cleanup -i raw_session.log -o clean_session.txt

# Now clean_session.txt contains readable output without ANSI codes
```

## Test Verification (test-verify)

The `test-verify` binary provides batch verification capabilities for comparing test execution logs against test case definitions.

### Features

- **Batch Processing**: Process multiple test execution logs simultaneously
- **Auto-locate Test Cases**: Uses TestCaseStorage to automatically find test case definitions
- **Flexible Matching**: Supports exact matches, wildcards (`*`), and regex patterns (`/pattern/`)
- **Multiple Output Formats**: Text, JSON, and JUnit XML
- **Aggregated Reports**: Pass/fail statistics per test case with detailed failure reasons
- **CI/CD Integration**: JUnit XML output for seamless integration with CI/CD pipelines

### Quick Start

```bash
# Build the binary
cargo build --release --bin test-verify

# Verify a single test
./target/release/test-verify single \
  --log test-execution.log \
  --test-case-id TC001

# Batch verify multiple logs
./target/release/test-verify batch \
  --logs logs/*.log \
  --format junit \
  --output junit-report.xml

# Run the demo
cargo run --example test_verify_demo
```

### Log Format

Test execution logs should follow this format:

```
[TIMESTAMP] TestCase: <id>, Sequence: <seq_id>, Step: <step_num>, Success: <true/false/null/->, Result: <result>, Output: <output>
```

Example:
```
[2024-01-15T10:30:00Z] TestCase: TC001, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Command executed successfully
```

### Commands

- `single`: Verify a single test execution log against a specific test case
- `batch`: Process multiple logs and generate aggregated reports
- `parse-log`: Parse and display log contents without verification

For detailed usage, see [docs/TEST_VERIFY_USAGE.md](docs/TEST_VERIFY_USAGE.md)

## Development

Build and test:

```bash
make build
make test
make lint
```

### Coverage Tools

Install code coverage tools for local development:

```bash
make install-coverage-tools
```

Run coverage analysis:

```bash
make coverage          # Run with 70% threshold
make coverage-html     # Generate HTML report
make coverage-report   # Show coverage summary
```

See [scripts/README_COVERAGE_TOOLS.md](scripts/README_COVERAGE_TOOLS.md) for detailed coverage tools documentation.

### Script Verification

Verify the syntax of all shell scripts:

```bash
make verify-scripts
```

This checks all scripts in the `scripts/` and `tests/integration/` directories for syntax errors.

### Integration Tests

The project includes comprehensive end-to-end integration tests using the Expect automation tool. These tests validate the complete user workflow from metadata entry through test sequence and step creation, including git commit verification.

#### Prerequisites

Install Expect:
- Ubuntu/Debian: `sudo apt-get install expect`
- macOS: `brew install expect`
- RHEL/CentOS: `sudo yum install expect`

#### Running Integration Tests

Run the complete workflow test:
```bash
make test-e2e
```

Run all integration tests (basic + complete):
```bash
make test-e2e-all
```

Run all tests (unit + integration):
```bash
make test-all
```

Manual execution:
```bash
./tests/integration/run_e2e_test.sh --build
./tests/integration/run_all_tests.sh --build
```

#### Test Coverage

The integration tests validate:
- Metadata creation and validation
- General initial conditions
- Device-specific initial conditions
- Test sequence creation with descriptions
- Step collection with expected results
- Git commits at each checkpoint
- YAML output file structure and content
- Schema validation
- Recovery file cleanup

See [tests/integration/README.md](tests/integration/README.md) for detailed documentation.

## License

See LICENSE file for details.
