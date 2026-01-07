# Recovery Mechanism Implementation Summary

## Overview

Implemented a comprehensive recovery mechanism that serializes the current TestCase structure and validation errors to a JSON file (`.recovery.json`) after each operation. The system detects this file on startup and prompts the user to resume from the saved state, with pre-populated prompts and inline validation error annotations.

## Files Created

### 1. `src/recovery.rs` (New Module)

Core recovery functionality module containing:

- **RecoveryState struct**: Holds structure, validation errors, phase, and timestamp
- **ValidationError struct**: Captures field path and error message
- **RecoveryManager struct**: Manages recovery file operations

Key methods:
- `save_state()`: Serialize and save recovery state to JSON
- `load_state()`: Load recovery state from JSON file
- `delete_recovery_file()`: Remove recovery file on completion
- `prompt_for_recovery()`: Interactive prompt for resuming
- `display_field_errors()`: Show errors for specific fields
- `extract_validation_errors_from_anyhow()`: Parse errors from validation failures

### 2. `docs/RECOVERY.md` (New Documentation)

Comprehensive documentation covering:
- Feature overview
- Usage examples
- File format specification
- Technical details
- Best practices
- Limitations

## Files Modified

### 1. `src/lib.rs`

Added recovery module exports:
```rust
pub mod recovery;
pub use recovery::{RecoveryManager, RecoveryState};
```

### 2. `src/builder.rs`

Enhanced TestCaseBuilder with recovery capabilities:

**New fields:**
- `recovery_manager: RecoveryManager`

**New methods:**
- `new_with_recovery()`: Create builder with recovery check
- `save_recovery_state()`: Save current state
- `save_recovery_state_with_errors()`: Save state with validation errors
- `delete_recovery_file()`: Delete recovery file
- `recovery_manager()`: Get recovery manager reference
- `validator()`: Get validator reference

**Changes:**
- Updated `new()` to initialize RecoveryManager
- All builders now include recovery_manager field

### 3. `src/prompts.rs`

Added recovery-aware prompt methods:

**New methods:**
- `input_with_recovered_default()`: Prompt with recovered value as editable initial text
- `input_integer_with_default()`: Integer prompt with recovered value as editable initial text
- `prompt_metadata_with_recovery()`: Metadata prompts with recovery values as editable initial text

**TestCaseMetadata enhancements:**
- `from_structure()`: Extract metadata from recovered structure

### 4. `src/main.rs`

Updated all interactive workflow handlers to use recovery:

**Modified handlers:**
- `handle_create_interactive()`: Uses `new_with_recovery()`, saves state, deletes on completion
- `handle_build_sequences()`: Uses `new_with_recovery()`, saves state, deletes on completion
- `handle_add_steps()`: Uses `new_with_recovery()`, saves state, deletes on completion
- `handle_build_sequences_with_steps()`: Uses `new_with_recovery()`, saves state, deletes on completion
- `handle_complete()`: Complete recovery integration with:
  - Recovery-aware metadata prompting
  - State saving after each operation
  - Error saving on validation failures
  - Recovery file deletion on success

**Key changes in handle_complete:**
- Loads recovered metadata with `TestCaseMetadata::from_structure()`
- Uses `prompt_metadata_with_recovery()` with recovered values
- Calls `save_recovery_state()` after successful operations
- Calls `save_recovery_state_with_errors()` on validation failures
- Calls `delete_recovery_file()` on workflow completion

### 5. `.gitignore`

Added recovery file to ignore list:
```
# Recovery files
.recovery.json
```

### 6. `README.md`

Updated features section to include:
```
- **Recovery Mechanism**: Automatically saves progress after each operation and can resume from saved state if interrupted
```

Added recovery mechanism section with usage instructions.

## Key Features Implemented

### 1. Automatic State Saving

After each operation:
- Metadata validation
- General initial conditions
- Initial conditions
- Test sequence addition
- Step addition

### 2. Error Tracking

Captures and persists:
- Field path where validation failed
- Complete error message
- Timestamp of failure

### 3. Recovery Prompt

On startup, detects recovery file and shows:
- Timestamp of saved state
- Current phase
- List of validation errors
- Prompt to resume or start fresh

### 4. Pre-populated Fields

When resuming:
- Shows recovered values as editable initial text in the input field
- User can press Enter to confirm as-is
- User can edit using normal text editing (backspace, delete, arrow keys, etc.)
- User can completely delete and replace with new values
- Clear indication that recovered values are being shown

### 5. Inline Error Display

Shows validation errors from previous attempts:
- Field-specific error messages
- Helps user fix issues on retry

### 6. Automatic Cleanup

Deletes recovery file on:
- Successful workflow completion
- User chooses not to resume (optional)

## Integration Points

### TestCaseBuilder Integration

```rust
// Create with recovery check
let mut builder = TestCaseBuilder::new_with_recovery(path)?;

// Save state after operations
builder.save_recovery_state("metadata")?;

// Save state with errors
builder.save_recovery_state_with_errors("metadata", &error)?;

// Delete on completion
builder.delete_recovery_file()?;
```

### Prompts Integration

```rust
// Use recovered metadata
let recovered_metadata = TestCaseMetadata::from_structure(builder.structure());

// Prompt with editable initial text
let metadata = Prompts::prompt_metadata_with_recovery(recovered_metadata.as_ref())?;
```

## Testing

Comprehensive test coverage in `src/recovery.rs`:
- `test_recovery_state_creation()`
- `test_recovery_state_with_errors()`
- `test_add_and_clear_errors()`
- `test_get_errors_for_field()`
- `test_recovery_manager_save_and_load()`
- `test_recovery_manager_delete()`
- `test_recovery_manager_no_file()`
- `test_parse_validation_error_path()`
- `test_extract_validation_errors_from_anyhow()`
- `test_recovery_state_serialization()`

## File Format

Recovery file (`.recovery.json`) structure:
```json
{
  "structure": {
    "requirement": "XXX100",
    "item": 1,
    "tc": 4,
    "id": "test_001",
    "description": "Test description",
    "test_sequences": [...]
  },
  "validation_errors": [
    {
      "field_path": "item",
      "error_message": "Path '/item': Invalid type"
    }
  ],
  "current_phase": "test_sequences",
  "timestamp": "2024-01-15T14:30:45.123456789Z"
}
```

## Usage Flow

1. User starts interactive workflow
2. Recovery file detected → prompt to resume
3. If resumed, structure loaded and prompts pre-populated
4. After each operation, state saved to recovery file
5. On validation error, errors saved to recovery file
6. On successful completion, recovery file deleted

## Benefits

1. **Data Preservation**: No loss of progress on interruption
2. **Error Context**: Validation errors preserved for debugging
3. **User Experience**: Seamless resume without re-entering data
4. **Reliability**: Automatic saves ensure no manual intervention needed
5. **Flexibility**: User can choose to resume or start fresh
6. **Clean Exit**: Automatic cleanup on success

## Supported Commands

Recovery mechanism integrated into:
- `create-interactive`
- `build-sequences`
- `add-steps`
- `build-sequences-with-steps`
- `complete`

## Technical Implementation Details

- **Serialization**: Uses `serde_json` for JSON serialization
- **Storage**: Single `.recovery.json` file per working directory
- **State Management**: Immutable state with clone-on-save
- **Error Parsing**: Extracts validation errors from anyhow error chains
- **Timestamps**: Uses chrono UTC timestamps for tracking
- **IndexMap**: Preserves field order in recovered structure
