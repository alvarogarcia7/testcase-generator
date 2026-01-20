# Test Case Manager

A comprehensive CLI tool for managing test cases in YAML format with interactive workflows, fuzzy search, git integration, and test verification capabilities.

## Features

- **Interactive Test Case Creation**: Build test cases with guided prompts
- **Database-backed Condition Selection**: Fuzzy search through existing conditions from test database
- **Test Sequence Builder**: Create test sequences with automatic numbering and validation
- **Step Collection Loop**: Add steps to sequences with fuzzy search for existing steps
- **Git Integration**: Commit progress after each step or sequence
- **Schema Validation**: Validate test cases against a JSON schema
- **Fuzzy Search**: Search through test cases, sequences, steps, and conditions
- **TTY Fallback**: Automatic detection of non-TTY environments (e.g., VS Code debug console) with graceful fallback to numbered selection
- **Recovery Mechanism**: Automatically saves progress after each operation and can resume from saved state if interrupted
- **Test Verification**: Batch verification mode that processes test execution logs and generates reports with JUnit XML output for CI/CD integration

## Binaries

This project includes multiple binaries:

- **tcm** (Test Case Manager): Interactive test case creation and management
- **test-verify**: Test verification tool for validating test execution logs against test cases
- **validate-yaml**: YAML validation tool

## Test Case Manager (tcm) Commands

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
```

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
