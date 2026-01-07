# Recovery Mechanism Documentation

## Overview

The recovery mechanism serializes the current TestCase structure and any validation errors to a JSON file (`.recovery.json`) after each operation. On startup, it detects this file and prompts the user whether to resume from the saved state. When resuming, interactive prompts are pre-populated with recovered values, and validation error annotations are displayed inline for fields that previously failed schema validation. The recovery file is automatically deleted upon successful completion.

## Features

### Automatic State Saving

After each operation during the interactive workflow, the current state is saved:

- **Metadata Entry**: After validating and adding test case metadata
- **General Initial Conditions**: After adding general initial conditions
- **Initial Conditions**: After adding initial conditions
- **Test Sequences**: After adding each test sequence
- **Steps**: After adding each step to a sequence

### Error Tracking

When validation fails, the error information is captured and saved:

- Field path where the error occurred
- Complete error message
- Timestamp of when the error occurred

### Recovery Prompt

When a recovery file is detected on startup:

```
╔══════════════════════════════════════════════════════╗
║           Recovery File Detected                    ║
╚══════════════════════════════════════════════════════╝

Found recovery data from: 2024-01-15 14:30:45 UTC
Phase: metadata

Validation Errors Found (2):
  • item: Invalid type (must be integer)
  • tc: Missing required property 'tc'

Resume from saved state? [Y/n]
```

### Pre-populated Fields

When resuming from a saved state, the prompts show recovered values as editable initial text:

```
=== Test Case Metadata ===

⚠ Recovered values shown as editable text (Enter confirms, you can edit/delete)

Requirement: XXX100
Item: 1
TC: 4
ID: test_001
Description: Test description
```

The recovered values appear in the input field and can be:
- Accepted as-is by pressing Enter
- Edited using normal text editing (arrow keys, backspace, etc.)
- Deleted completely and replaced with new values

### Inline Error Display

Validation errors from previous attempts are displayed when prompting for the same field:

```
⚠ Previous validation errors for 'item':
  • Path '/item': Invalid type (expected integer, got string)

Item [not_an_integer]: 
```

## File Format

The recovery file (`.recovery.json`) contains:

```json
{
  "structure": {
    "requirement": "XXX100",
    "item": 1,
    "tc": 4,
    "id": "test_001",
    "description": "Test description"
  },
  "validation_errors": [
    {
      "field_path": "item",
      "error_message": "Path '/item': Invalid type"
    }
  ],
  "current_phase": "metadata",
  "timestamp": "2024-01-15T14:30:45.123Z"
}
```

## Usage Example

### Starting a Workflow

```bash
testcase-manager complete -o output/test.yaml
```

### If Interrupted

The workflow is interrupted (Ctrl+C or error), and progress is saved to `.recovery.json`.

### Resuming the Workflow

Run the same command again:

```bash
testcase-manager complete -o output/test.yaml
```

You'll be prompted:

```
╔══════════════════════════════════════════════════════╗
║           Recovery File Detected                    ║
╚══════════════════════════════════════════════════════╝

Found recovery data from: 2024-01-15 14:30:45 UTC
Phase: test_sequences

Resume from saved state? [Y/n] y

✓ Resuming from saved state
```

The workflow will continue from where it left off, with all previously entered data pre-populated in the prompts.

### Successful Completion

When the workflow completes successfully:

```
✓ Complete test case saved to: output/test.yaml

✓ Recovery file deleted

╔══════════════════════════════════════════════════════╗
║         Test Case Workflow Completed!               ║
╚══════════════════════════════════════════════════════╝
```

## Supported Commands

The recovery mechanism is integrated into these interactive workflows:

- `create-interactive`: Interactive test case creation
- `build-sequences`: Build test sequences with commits
- `add-steps`: Add steps to sequences
- `build-sequences-with-steps`: Build sequences and steps together
- `complete`: Complete workflow with all components

## Recovery File Location

The recovery file is saved in the base directory (same as the test case files):

```
./testcases/.recovery.json
```

It is automatically ignored by git (included in `.gitignore`).

## Technical Details

### RecoveryState Structure

```rust
pub struct RecoveryState {
    pub structure: IndexMap<String, Value>,
    pub validation_errors: Vec<ValidationError>,
    pub current_phase: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

### ValidationError Structure

```rust
pub struct ValidationError {
    pub field_path: String,
    pub error_message: String,
}
```

### RecoveryManager Methods

- `save_state(&self, state: &RecoveryState)`: Save current state to file
- `load_state(&self)`: Load state from file
- `delete_recovery_file(&self)`: Delete the recovery file
- `prompt_for_recovery(&self)`: Prompt user to resume
- `display_field_errors(&self, state: &RecoveryState, field_path: &str)`: Show errors for a specific field
- `extract_validation_errors_from_anyhow(error: &anyhow::Error)`: Parse validation errors from error messages

## Best Practices

1. **Regular Saves**: Save state after each major operation to minimize data loss
2. **Error Context**: Include full error messages in ValidationError for helpful recovery
3. **Field Paths**: Use clear, dot-separated field paths (e.g., `test_sequences.0.steps.2.description`)
4. **Cleanup**: Always delete recovery file on successful completion
5. **User Choice**: Always prompt before resuming to allow starting fresh if needed

## Limitations

- Recovery file must be in the same directory as the test case
- Only one recovery file per directory (overwrites previous)
- Manual edits to recovery file may cause deserialization errors
- Recovery state is lost if the directory is moved or deleted
