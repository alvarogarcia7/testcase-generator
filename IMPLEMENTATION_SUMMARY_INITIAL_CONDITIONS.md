# Initial Conditions Collection Implementation Summary

## Overview

This implementation adds interactive initial conditions collection with device selection, iterative condition entry, schema validation, file saving, and git commit functionality.

## Changes Made

### 1. src/prompts.rs

Added `prompt_initial_conditions` function that:
- Prompts user for device name (e.g., eUICC)
- Iteratively collects condition strings
- Validates user enters at least one condition
- Constructs a YAML mapping structure with device as key and conditions array as value
- Validates the structure against the schema using specialized validation
- Returns the validated structure

**Key Features:**
- Supports default values (can keep or edit)
- Empty string input to finish entering conditions
- Clear prompts with numbered conditions
- Immediate validation feedback

### 2. src/validation.rs

Added `validate_initial_conditions` method to `SchemaValidator`:
- Validates that initial_conditions is an object/mapping
- Validates each device has an array of conditions
- Validates each condition is a string
- Provides clear error messages for validation failures

**Validation Rules:**
```rust
initial_conditions:
  device_name:           // Must be a mapping key
    - "condition 1"      // Must be a string
    - "condition 2"      // Must be a string
```

Added comprehensive tests:
- `test_validate_initial_conditions_valid` - validates correct structure
- `test_validate_initial_conditions_invalid_not_array` - catches non-array values
- `test_validate_initial_conditions_invalid_not_strings` - catches non-string conditions
- `test_validate_initial_conditions_multiple_devices` - validates multiple devices

### 3. src/builder.rs

Updated `add_initial_conditions` method:
- Simplified implementation to use the new `prompt_initial_conditions` function
- Removed editor-based approach for initial conditions
- Maintains validation and structure insertion

### 4. docs/interactive_workflow.md

Updated documentation to reflect:
- New interactive prompt-based workflow for initial conditions
- Device selection and iterative condition entry
- Validation details specific to initial conditions
- Example interactions showing the user experience

### 5. README.md

Updated feature list to clarify:
- Editor integration is for general initial conditions
- Interactive device selection and condition entry for initial conditions

## Workflow Integration

The implementation integrates seamlessly with the existing `handle_create_interactive` workflow:

1. **Metadata Collection** → Commit (optional)
2. **General Initial Conditions** (editor-based) → Commit (optional)
3. **Initial Conditions** (interactive prompts) → Commit (optional)
4. **Save to file**

Each step includes:
- Interactive prompts
- Schema validation
- Optional git commit with descriptive message
- Progress feedback

## Git Integration

The workflow supports git commits before and after collecting initial conditions:

**Before:**
- User can commit metadata before starting initial conditions collection
- This creates a save point before entering the conditions

**After:**
- User can commit initial conditions after successful validation
- Commit message: "Add initial conditions"
- File is saved and staged before commit

## Example Usage

```bash
# Run interactive workflow
testcase-manager create-interactive

# Workflow prompts:
=== Test Case Metadata ===
Requirement: XXX100
Item: 1
TC: 4
ID: test-case-001
Description: Test case description

=== Validating Metadata ===
✓ Metadata is valid

Commit metadata to git? [Y/n]: y
✓ Committed: Add test case metadata

Add initial conditions? [Y/n]: y

=== Initial Conditions ===

Device name (e.g., eUICC): eUICC

Enter conditions for 'eUICC' (enter empty string to finish):
Condition #1: The PROFILE_OPERATIONAL1 is Enabled.
Condition #2: The PROFILE_OPERATIONAL2 is Enabled.
Condition #3: [press Enter]

✓ Valid structure
✓ Initial conditions added

Commit initial conditions to git? [Y/n]: y
✓ Committed: Add initial conditions
```

## Technical Details

### Data Structure

The initial_conditions structure follows the schema:

```yaml
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
  LPA:
    - "LPA Condition 1"
```

### Validation Flow

1. User enters device name
2. User enters conditions iteratively
3. Structure is built as `serde_yaml::Value::Mapping`
4. `validate_initial_conditions` is called
5. If valid, structure is added to builder
6. User can commit or continue

### Error Handling

The implementation provides clear error messages:
- "At least one condition is required" - if user tries to finish without entering conditions
- "Device 'X' must have an array of conditions" - if structure is malformed
- "Condition #N for device 'X' must be a string" - if condition has wrong type
- "initial_conditions must be an object with device names as keys" - if root structure is wrong

## Benefits

1. **User-friendly**: Simple prompts, no editor required for initial conditions
2. **Validated**: All input validated before being added to structure
3. **Traceable**: Git commits create an audit trail
4. **Flexible**: Supports default values and custom device names
5. **Robust**: Comprehensive error handling and validation
6. **Incremental**: Progress saved after each major section

## Testing

Added tests in `src/validation.rs`:
- Valid initial conditions with single device
- Valid initial conditions with multiple devices
- Invalid: device with non-array value
- Invalid: conditions with non-string items

All tests verify proper validation behavior and error messages.
