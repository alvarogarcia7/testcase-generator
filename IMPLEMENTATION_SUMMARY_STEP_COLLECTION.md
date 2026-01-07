# Implementation Summary: Step Collection Loop within Test Sequences

## Overview

This implementation adds a comprehensive step collection loop feature within test sequences that includes fuzzy search for existing steps, field collection, validation, file saving, and git commits after each step.

## Key Components

### 1. TestCaseBuilder Methods (src/builder.rs)

#### Core Step Collection Methods

- **`add_steps_to_sequence_with_commits(sequence_index)`**
  - Main entry point for the step collection loop
  - Displays sequence information
  - Loops to collect multiple steps
  - Prompts for fuzzy search of existing step descriptions
  - Collects all step fields (number, manual, description, command, expected)
  - Validates each step against schema
  - Appends step to sequence
  - Saves file after each step
  - Commits progress with descriptive message

- **`get_all_existing_steps()`**
  - Retrieves all unique step descriptions from all sequences
  - Used for fuzzy search suggestions

- **`get_next_step_number(sequence_index)`**
  - Automatically calculates the next step number
  - Finds max existing step number and increments

- **`prompt_for_expected()`**
  - Interactive prompts for expected results structure
  - Supports optional 'success' field
  - Collects 'result' and 'output' fields

- **`create_step_value(step_number, manual, description, command, expected)`**
  - Creates a properly structured step value
  - Handles optional manual flag
  - Returns validated YAML Value

- **`validate_and_append_step(sequence_index, step)`**
  - Validates step structure against required fields:
    - step (number)
    - description (string)
    - command (string)
    - expected (object with result and output)
  - Validates expected structure
  - Appends step to specified sequence

#### Supporting Methods

- **`get_sequence_id_by_index(index)`** - Get sequence ID by index
- **`get_sequence_name_by_index(index)`** - Get sequence name by index
- **`find_sequence_index_by_id(sequence_id)`** - Find sequence by ID
- **`get_sequence_count()`** - Get total number of sequences
- **`add_steps_to_sequence_by_id_with_commits(sequence_id)`** - Add steps by sequence ID

#### Workflow Integration Methods

- **`build_test_sequences_with_step_commits()`**
  - Complete workflow: sequences → steps
  - Prompts to add steps after each sequence
  - Commits at each stage

### 2. CLI Commands (src/cli.rs & src/main.rs)

#### New Commands

1. **`build-sequences-with-steps`**
   - Full workflow: metadata → conditions → sequences → steps
   - Commits after each major stage
   - Handler: `handle_build_sequences_with_steps()`

2. **`add-steps`**
   - Add steps to a specific sequence
   - Optional `--sequence-id` parameter
   - Handler: `handle_add_steps()`

### 3. Step Schema Validation

Steps are validated against this structure:

```yaml
steps:
  - step: <integer>          # Required: step number
    manual: <boolean>        # Optional: manual execution flag
    description: <string>    # Required: step description
    command: <string>        # Required: command to execute
    expected:                # Required: expected results
      success: <boolean>     # Optional: success flag
      result: <string>       # Required: result value
      output: <string>       # Required: output value
```

### 4. Git Integration

Each step can be committed with a descriptive message:

```
Add step #1 to sequence #1: Execute command
Add step #2 to sequence #1: Verify results
```

### 5. Fuzzy Search Integration

The implementation uses the existing `TestCaseFuzzyFinder` to:
- Search through existing step descriptions
- Allow reuse of common step patterns
- Speed up data entry

## User Workflow

### Complete Workflow (build-sequences-with-steps)

```
1. Enter metadata (requirement, item, tc, id, description)
   → Optional: Commit metadata
2. Add general initial conditions
   → Optional: Commit
3. Add initial conditions
   → Optional: Commit
4. For each test sequence:
   a. Enter sequence ID, name, description
   b. Optional: Commit sequence
   c. Add steps to sequence:
      - Optionally use fuzzy search for description
      - Enter manual flag (yes/no)
      - Enter command
      - Enter expected results (success, result, output)
      - Step is validated
      - File is saved
      - Optional: Commit step
   d. Repeat for more steps
5. Repeat for more sequences
6. Final commit
```

### Add Steps to Existing Sequence

```
1. Create sequence (or load existing test case)
2. Add steps:
   - Use fuzzy search for description
   - Collect all step fields
   - Validate
   - Save
   - Commit
3. Repeat until done
```

## Features

### Automatic Step Numbering
- Steps are numbered sequentially starting from 1
- Next step number calculated from existing steps
- No manual tracking needed

### Fuzzy Search
- Search existing step descriptions
- Select from previously used patterns
- Fall back to manual entry if needed

### Validation
- Schema validation for each field
- Required fields enforced (step, description, command, expected)
- Expected structure validated (must have result and output)
- Optional fields handled (manual, success)

### Git Commits
- Commit after each step (optional)
- Descriptive commit messages
- Full audit trail of progress

### File Persistence
- File saved after each step
- No data loss during interactive session
- Can resume if interrupted

## Testing

Comprehensive test coverage includes:

- `test_get_sequence_id_by_index()` - Sequence ID retrieval
- `test_get_sequence_name_by_index()` - Sequence name retrieval
- `test_get_next_step_number_empty()` - Step numbering (empty)
- `test_get_next_step_number_with_existing()` - Step numbering (existing)
- `test_create_step_value()` - Step creation
- `test_validate_and_append_step()` - Step validation and append
- `test_validate_step_missing_fields()` - Error handling
- `test_get_all_existing_steps()` - Existing step retrieval
- `test_find_sequence_index_by_id()` - Sequence lookup
- `test_get_sequence_count()` - Sequence counting

## Example Usage

### Programmatic Usage

```rust
use testcase_manager::TestCaseBuilder;

let mut builder = TestCaseBuilder::new("path/to/testcases")?;

// Add metadata and conditions...

// Add a sequence
builder.add_test_sequence_interactive()?;

// Add steps to the last sequence
let sequence_index = builder.get_sequence_count() - 1;
builder.add_steps_to_sequence_with_commits(sequence_index)?;

// Save
builder.save()?;
```

### CLI Usage

```bash
# Complete workflow
testcase-manager build-sequences-with-steps --path ./testcases

# Add steps to existing sequence
testcase-manager add-steps --path ./testcases --sequence-id 1
```

## Files Modified

1. **src/builder.rs** - Core implementation
2. **src/cli.rs** - CLI command definitions
3. **src/main.rs** - Command handlers
4. **README.md** - Documentation
5. **examples/interactive_workflow.rs** - Example usage

## Architecture

```
User Input
    ↓
CLI Command Handler
    ↓
TestCaseBuilder
    ↓
Step Collection Loop
    ├── Fuzzy Search (existing steps)
    ├── Field Prompts (number, manual, description, command, expected)
    ├── Step Creation (create_step_value)
    ├── Validation (validate_and_append_step)
    ├── File Save (save)
    └── Git Commit (commit)
```

## Benefits

1. **Efficient Data Entry**: Fuzzy search reduces typing
2. **Data Integrity**: Validation ensures schema compliance
3. **Audit Trail**: Git commits track every change
4. **Recovery**: File saves prevent data loss
5. **Flexibility**: Optional fields and commits
6. **Reusability**: Share step descriptions across sequences
7. **Automation**: Automatic step numbering

## Future Enhancements

Potential improvements:
- Batch step import from CSV/JSON
- Step templates library
- Copy steps between sequences
- Reorder steps
- Edit existing steps
- Undo/redo functionality
- Step dependencies tracking
