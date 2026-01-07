# Interactive Test Case Creation Workflow

This document describes the interactive test case creation workflow that guides users through building test cases step-by-step with validation and git commits.

## Overview

The interactive workflow provides:

1. **Interactive prompts** for test case metadata (requirement, item, tc, id, description)
2. **Validation** of each chunk against the JSON schema
3. **Git commits** after each major section is completed
4. **Editor integration** for complex structures like initial conditions
5. **Default values** that can be kept or edited
6. **Schema validation** for all inputs

## Command Line Usage

### Create Interactive Test Case

```bash
# Start interactive test case creation
testcase-manager create-interactive

# Specify custom path
testcase-manager create-interactive --path ./my-testcases
```

## Workflow Steps

### 1. Metadata Prompts

The workflow begins by prompting for test case metadata:

```
=== Test Case Metadata ===

Requirement: XXX100
Item: 1
TC: 4
ID: 4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata
Description: Test case for ES6.UpdateMetadata operations
```

**Fields:**
- **Requirement** (string): The requirement identifier
- **Item** (integer): The item number
- **TC** (integer): The test case number
- **ID** (string): The unique test case identifier
- **Description** (string): A description of the test case

### 2. Metadata Validation

After entering metadata, the system validates it against the schema:

```
=== Validating Metadata ===
✓ Metadata is valid
```

If validation fails, clear error messages indicate what needs to be fixed:

```
✗ Validation failed:
  - Path '/item': Invalid type, expected integer
```

### 3. Git Commit (Metadata)

After successful validation, you can commit the metadata:

```
Commit metadata to git? [Y/n]: y
✓ Committed: Add test case metadata
```

### 4. General Initial Conditions

Next, the workflow prompts for general initial conditions:

```
=== General Initial Conditions ===

Add general initial conditions? [Y/n]: y
```

#### Option A: Keep Defaults

If defaults are available, they are displayed:

```
Current defaults:
- eUICC:
    - "The profile PROFILE_OPERATIONAL1 is loaded"

Keep these defaults? [Y/n]: y
✓ General initial conditions added
```

#### Option B: Edit in Editor

If you choose to edit, your default editor opens with a template:

```yaml
# General Initial Conditions
# Example:
# - eUICC:
#     - "Condition 1"
#     - "Condition 2"

- eUICC:
    - ""
```

After editing and saving:
- The YAML is parsed and validated against the schema
- If validation fails, you can retry or cancel
- If validation succeeds, the conditions are added to the structure

```
✓ Valid structure
✓ General initial conditions added
```

### 5. Git Commit (General Initial Conditions)

```
Commit general initial conditions to git? [Y/n]: y
✓ Committed: Add general initial conditions
```

### 6. Initial Conditions

Similar to general initial conditions, but for the main initial conditions:

```
=== Initial Conditions ===

Add initial conditions? [Y/n]: y
```

The same flow applies:
1. Show defaults (if available)
2. Prompt to keep or edit
3. Open editor with template if editing
4. Validate against schema
5. Add to structure
6. Offer to commit

Template for initial conditions:

```yaml
# Initial Conditions
# Example:
# eUICC:
#   - "Condition 1"
#   - "Condition 2"

eUICC:
  - ""
```

### 7. Completion

After all steps are complete:

```
╔═══════════════════════════════════════════════╗
║          Test Case Created Successfully       ║
╚═══════════════════════════════════════════════╝

Saved to: ./testcases/4.2.2.2.1_TC_eUICC_ES6.UpdateMetadata.yaml
```

## Programmatic Usage

### Using TestCaseBuilder

```rust
use testcase_manager::{TestCaseBuilder, Prompts};
use anyhow::Result;

fn create_test_case() -> Result<()> {
    let mut builder = TestCaseBuilder::new("./testcases")?;

    // Add metadata
    builder.add_metadata()?;
    builder.commit("Add test case metadata")?;

    // Add general initial conditions
    builder.add_general_initial_conditions(None)?;
    builder.commit("Add general initial conditions")?;

    // Add initial conditions
    builder.add_initial_conditions(None)?;
    builder.commit("Add initial conditions")?;

    // Save the test case
    let file_path = builder.save()?;
    println!("Saved to: {}", file_path.display());

    Ok(())
}
```

### Using Prompts Directly

```rust
use testcase_manager::{Prompts, TestCaseMetadata};
use anyhow::Result;

fn prompt_metadata() -> Result<TestCaseMetadata> {
    // Use the built-in metadata prompts
    let metadata = Prompts::prompt_metadata()?;
    
    // Validate the metadata
    let validator = testcase_manager::SchemaValidator::new()?;
    metadata.validate(&validator)?;
    
    Ok(metadata)
}
```

### Custom Workflows

```rust
use testcase_manager::TestCaseBuilder;
use anyhow::Result;

fn custom_workflow() -> Result<()> {
    let mut builder = TestCaseBuilder::new("./testcases")?;

    // Add metadata manually
    builder.add_field(
        "requirement".to_string(),
        serde_yaml::Value::String("XXX100".to_string())
    )?;
    
    builder.add_field(
        "item".to_string(),
        serde_yaml::Value::Number(1.into())
    )?;

    // Access the structure directly
    let structure = builder.structure_mut();
    // Modify as needed...

    // Validate the entire structure
    builder.validate()?;

    // Save
    builder.save()?;

    Ok(())
}
```

## Validation

All inputs are validated against the JSON schema defined in `data/schema.json`.

### Metadata Validation

The metadata chunk validates:
- **requirement**: Must be a string
- **item**: Must be an integer
- **tc**: Must be an integer
- **id**: Must be a string
- **description**: Must be a string

### Initial Conditions Validation

Initial conditions are validated as part of the overall schema. The validator checks:
- Correct YAML structure
- Required fields are present
- Types match schema expectations
- Array items conform to schema

### Error Messages

Validation errors are clear and actionable:

```
Schema validation failed:
  - Path '/item': Invalid type (expected integer)
  - Path 'root': Missing required property 'tc'
```

## Git Integration

The workflow integrates with Git to track progress:

1. **Automatic Git initialization**: If no Git repository exists, one is created
2. **Incremental commits**: Each major section can be committed separately
3. **Descriptive commit messages**: Commits describe what was added
4. **Commit history**: View progress with `git log`

Example commit history:

```
abc1234 - Add initial conditions (Test Case Manager)
def5678 - Add general initial conditions (Test Case Manager)
9ab0cde - Add test case metadata (Test Case Manager)
```

## Best Practices

1. **Use descriptive IDs**: Choose IDs that clearly identify the test case
2. **Commit frequently**: Commit after each major section to track progress
3. **Review validation errors**: If validation fails, carefully read the error messages
4. **Use editor efficiently**: When editing in the editor, follow YAML syntax strictly
5. **Keep defaults when possible**: If defaults are appropriate, keep them to save time

## Troubleshooting

### Validation Fails

**Problem**: Metadata validation fails with type error

**Solution**: Ensure integers are not quoted in input. For example:
- ❌ Item: "1" (wrong - string)
- ✓ Item: 1 (correct - integer)

### Editor Doesn't Open

**Problem**: Editor fails to open when editing conditions

**Solution**: Set your preferred editor in environment variables:
```bash
export EDITOR=vim
# or
export VISUAL=nano
```

### YAML Parse Error

**Problem**: YAML parsing fails after editing in editor

**Solution**: Check YAML syntax:
- Correct indentation (spaces, not tabs)
- Proper list syntax (- for list items)
- Quoted strings when necessary
- No trailing spaces

### Git Commit Fails

**Problem**: Git commit fails with signature error

**Solution**: Set Git author information:
```bash
export GIT_AUTHOR_NAME="Your Name"
export GIT_AUTHOR_EMAIL="your.email@example.com"
```

## Examples

### Complete Interactive Session

```
╔═══════════════════════════════════════════════╗
║   Interactive Test Case Creation Workflow    ║
╚═══════════════════════════════════════════════╝

=== Test Case Metadata ===

Requirement: XXX100
Item: 1
TC: 4
ID: 4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata
Description: Test ES6.UpdateMetadata command processing

=== Validating Metadata ===
✓ Metadata is valid

✓ Metadata added to structure

Commit metadata to git? [Y/n]: y
✓ Committed: Add test case metadata

Add general initial conditions? [Y/n]: y

=== General Initial Conditions ===

[Editor opens with template]

✓ Valid structure
✓ General initial conditions added

Commit general initial conditions to git? [Y/n]: y
✓ Committed: Add general initial conditions

Add initial conditions? [Y/n]: y

=== Initial Conditions ===

[Editor opens with template]

✓ Valid structure
✓ Initial conditions added

Commit initial conditions to git? [Y/n]: y
✓ Committed: Add initial conditions

╔═══════════════════════════════════════════════╗
║          Test Case Created Successfully       ║
╚═══════════════════════════════════════════════╝

Saved to: ./testcases/4.2.2.2.1_TC_eUICC_ES6.UpdateMetadata.yaml
```

### Example Output YAML

```yaml
requirement: XXX100
item: 1
tc: 4
id: 4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata
description: Test ES6.UpdateMetadata command processing
general_initial_conditions:
  - eUICC:
      - "The profile PROFILE_OPERATIONAL1 with #METADATA_WITH_PPRS_AND_ICON is loaded on the eUICC."
initial_conditions:
  eUICC:
    - "The PROFILE_OPERATIONAL1 is Enabled."
    - "The PROFILE_OPERATIONAL2 is Enabled."
```

## See Also

- [Validation Documentation](validation.md)
- [Schema Reference](../data/schema.json)
- [Git Integration](../README.md#git-operations)
