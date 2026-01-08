# Test Case Manager

A comprehensive CLI tool for managing test cases in YAML format with interactive workflows, fuzzy search, and git integration.

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

## Commands

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

## Development

Build and test:

```bash
make build
make test
make lint
```

## License

See LICENSE file for details.
