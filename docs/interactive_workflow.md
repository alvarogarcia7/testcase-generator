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

The workflow prompts for the main initial conditions with interactive device selection and iterative condition entry:

```
=== Initial Conditions ===

Add initial conditions? [Y/n]: y
```

#### Option A: Keep Defaults

If defaults are available, they are displayed:

```
Current defaults:
eUICC:
  - "The PROFILE_OPERATIONAL1 is Enabled."
  - "The PROFILE_OPERATIONAL2 is Enabled."

Keep these defaults? [Y/n]: y
✓ Initial conditions added
```

#### Option B: Interactive Entry

If you choose to create new initial conditions, the workflow prompts for:

1. **Device name**: The device for which conditions apply (e.g., eUICC, LPA)
2. **Conditions**: Iteratively enter condition strings until you enter an empty string

Example interaction:

```
Device name (e.g., eUICC): eUICC

Enter conditions for 'eUICC' (enter empty string to finish):
Condition #1: The PROFILE_OPERATIONAL1 is Enabled.
Condition #2: The PROFILE_OPERATIONAL2 is Enabled.
Condition #3: [press Enter to finish]
```

After entering all conditions:
- The structure is validated against the schema
- Ensures device name is valid
- Ensures conditions are an array of strings
- If validation succeeds, the conditions are added to the structure

```
✓ Valid structure
✓ Initial conditions added
```

Template structure for initial conditions:

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

All inputs are validated against the JSON schema defined in `testcases/schema.json`.

### Metadata Validation

The metadata chunk validates:
- **requirement**: Must be a string
- **item**: Must be an integer
- **tc**: Must be an integer
- **id**: Must be a string
- **description**: Must be a string

### Initial Conditions Validation

Initial conditions have specialized validation to ensure the structure is correct:

1. **Structure validation**: The initial_conditions must be an object (mapping)
2. **Device validation**: Each device name must map to an array
3. **Condition validation**: Each condition must be a string

The validator performs these checks:
```rust
// Example validation
eUICC:                    // Must be a mapping key (device name)
  - "Condition 1"        // Must be a string
  - "Condition 2"        // Must be a string
```

Validation error examples:
```
✗ Device 'eUICC' must have an array of conditions, got: "not an array"
✗ Condition #2 for device 'eUICC' must be a string, got: 123
✗ initial_conditions must be an object with device names as keys
```

The validation occurs before the structure is appended to the test case, ensuring that only valid data is saved.

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

## Test Sequence Builder

The `build-sequences` command provides a comprehensive workflow for creating test sequences with git commits before each sequence.

### Overview

The test sequence builder provides:

1. **Fuzzy search** for sequence names from existing sequences
2. **Editor integration** for copying/editing descriptions
3. **Validation** of sequence metadata structure
4. **Incremental ID assignment** automatically assigns sequential IDs
5. **Git commits** optional commits before adding each sequence
6. **Append to structure** validated sequences are appended to test_sequences array

### Command Line Usage

```bash
# Start test sequence builder
testcase-manager build-sequences

# Specify custom path
testcase-manager build-sequences --path ./my-testcases
```

### Workflow Steps

#### 1. Initial Setup

The workflow begins like `create-interactive`:

```
╔═══════════════════════════════════════════════╗
║   Test Sequence Builder with Git Commits     ║
╚═══════════════════════════════════════════════╝

=== Test Case Metadata ===

Requirement: XXX100
Item: 1
TC: 4
ID: 4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata
Description: Test ES6.UpdateMetadata operations

=== Validating Metadata ===
✓ Metadata is valid

✓ Metadata added to structure

Commit metadata to git? [Y/n]: y
✓ Committed: Add test case metadata
```

#### 2. General Initial Conditions

Same as `create-interactive` workflow:

```
Add general initial conditions? [Y/n]: y

=== General Initial Conditions ===

[Editor opens with template or keep defaults]

✓ General initial conditions added

Commit general initial conditions to git? [Y/n]: y
✓ Committed: Add general initial conditions
```

#### 3. Initial Conditions

Same as `create-interactive` workflow:

```
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

#### 4. Build Test Sequences (Loop)

Now the sequence builder starts:

```
╔═══════════════════════════════════════════════╗
║    Test Sequence Builder with Git Commits    ║
╚═══════════════════════════════════════════════╝

=== Add Test Sequence ===

Sequence ID: 1
```

##### 4.1. Sequence Name Selection

If existing sequences are found in the structure, you can fuzzy search them:

```
You can select from existing sequence names or type a new one.

Use fuzzy search to select from existing names? [y/N]: y
```

**Fuzzy Search Interface:**
```
> Test Sequence #01 Nominal: Unset PPR1
  Test Sequence #02 Nominal: Unset PPR2 and update icon
  Test Sequence #03 Error Case: Invalid parameters
  
Select sequence name: nominal
> Test Sequence #01 Nominal: Unset PPR1
  Test Sequence #02 Nominal: Unset PPR2 and update icon
```

Press Enter to select. Or skip fuzzy search and type a new name:

```
Sequence name: Test Sequence #01 Nominal: Unset PPR1
```

##### 4.2. Description Editing

You can edit the description in your editor:

```
Edit description in editor? [y/N]: y
```

The editor opens with a template:

```
# Description for: Test Sequence #01 Nominal: Unset PPR1
# Enter the sequence description below:

This test case verifies that the eUICC correctly processes an ES6.UpdateMetadata command to unset PPR1
when the profile is in the operational state and PPR1 is currently set.
```

Comment lines (starting with #) are automatically removed from the final description.

Or enter it via prompt:

```
Description: This test case verifies that the eUICC correctly processes...
```

##### 4.3. Sequence-Specific Initial Conditions

Each sequence can have its own initial conditions:

```
Add sequence-specific initial conditions? [y/N]: y

=== Initial Conditions ===

Device name (e.g., eUICC): eUICC

Enter conditions for 'eUICC' (enter empty string to finish):
Condition #1: The PROFILE_OPERATIONAL3 is Enabled.
Condition #2: [press Enter]

✓ Valid structure
```

##### 4.4. Sequence Validation

The sequence structure is validated:

```
=== Validating Test Sequence ===
✓ Test sequence validated and added
```

**Validation checks:**
- Sequence must be a mapping (object)
- Must have 'id' field
- Must have 'name' field
- Must have 'steps' field (initialized as empty array)
- Optional: 'description', 'initial_conditions'

##### 4.5. Git Commit

Optionally commit the sequence:

```
Commit this sequence to git? [y/N]: y
✓ Committed: Add test sequence #1
```

##### 4.6. Repeat or Finish

```
Add another test sequence? [y/N]: y
```

If yes, the loop repeats with Sequence ID: 2, and so on.

#### 5. Final Save

After all sequences are added:

```
✓ All test sequences added

╔═══════════════════════════════════════════════╗
║    Test Sequences Built Successfully          ║
╚═══════════════════════════════════════════════╝

Saved to: ./testcases/4.2.2.2.1_TC_eUICC_ES6.UpdateMetadata.yaml

Commit final file? [y/N]: y
✓ Committed: Complete test case with all sequences
```

### Features in Detail

#### Fuzzy Search for Sequence Names

- **Reuse names**: Quickly select from previously entered sequence names
- **Consistency**: Ensures naming consistency across sequences
- **Fast navigation**: skim fuzzy finder with incremental search
- **Fallback**: Can always type a new name if no match

#### Editor Integration

- **Template-based**: Opens with helpful comments and examples
- **Clean output**: Automatically removes comment lines
- **Multi-line support**: Write detailed descriptions with proper formatting
- **Fallback to prompt**: Can skip editor and use simple prompt

#### Automatic ID Assignment

- **Incremental**: IDs start at 1 and increment automatically
- **Smart detection**: Finds the maximum existing ID and adds 1
- **Gap handling**: If IDs are 1, 3, 5, next will be 6
- **No user input**: IDs are assigned automatically

#### Sequence Metadata Validation

Validates the structure before appending:

```rust
// Required fields
id: 1                              // Must be an integer
name: "Test Sequence #01"          // Must be a string
steps: []                          // Must be an array (empty for now)

// Optional fields
description: "..."                 // String
initial_conditions: [...]          // Array of device conditions
```

#### Git Commits Before Each Sequence

- **Incremental history**: Each sequence is a separate commit
- **Clear messages**: Commits are named "Add test sequence #N"
- **Optional**: Can skip commits if desired
- **Rollback friendly**: Easy to revert to previous sequence

### Example Output

The final YAML file:

```yaml
requirement: XXX100
item: 1
tc: 4
id: '4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata'
description: Test ES6.UpdateMetadata operations
general_initial_conditions:
  - eUICC:
      - "The profile PROFILE_OPERATIONAL1 with #METADATA_WITH_PPRS_AND_ICON is loaded on the eUICC."
initial_conditions:
  eUICC:
    - "The PROFILE_OPERATIONAL1 is Enabled."
    - "The PROFILE_OPERATIONAL2 is Enabled."
test_sequences:
  - id: 1
    name: "Test Sequence #01 Nominal: Unset PPR1"
    description: |
      This test case verifies that the eUICC correctly processes an ES6.UpdateMetadata command to unset PPR1
      when the profile is in the operational state and PPR1 is currently set.
    initial_conditions:
      - eUICC:
          - "The PROFILE_OPERATIONAL3 is Enabled."
    steps: []
  - id: 2
    name: "Test Sequence #02 Nominal: Unset PPR2 and update icon"
    description: |
      The purpose of this test is to verify that the MNO can unset PPR2 and update the icon and
      icon type values from a Profile.
    initial_conditions:
      - eUICC:
          - "The PROFILE_OPERATIONAL3 is Enabled."
    steps: []
```

### Git Commit History

```bash
$ testcase-manager git log --limit 5

a1b2c3d - Complete test case with all sequences (Test Case Manager)
d4e5f6g - Add test sequence #2 (Test Case Manager)
h7i8j9k - Add test sequence #1 (Test Case Manager)
l0m1n2o - Add initial conditions (Test Case Manager)
p3q4r5s - Add general initial conditions (Test Case Manager)
```

### Programmatic Usage

```rust
use testcase_manager::TestCaseBuilder;
use anyhow::Result;

fn build_sequences_programmatically() -> Result<()> {
    let mut builder = TestCaseBuilder::new("./testcases")?;

    // Add metadata
    builder.add_metadata()?;
    builder.commit("Add test case metadata")?;

    // Add initial conditions
    builder.add_general_initial_conditions(None)?;
    builder.commit("Add general initial conditions")?;

    builder.add_initial_conditions(None)?;
    builder.commit("Add initial conditions")?;

    // Build sequences with interactive prompts and commits
    builder.build_test_sequences_with_commits()?;

    // Save
    let file_path = builder.save()?;
    println!("Saved to: {}", file_path.display());

    Ok(())
}
```

### Best Practices

1. **Use fuzzy search**: Reuse sequence names for consistency
2. **Edit descriptions in editor**: Better for multi-line descriptions
3. **Commit each sequence**: Creates clear history for tracking changes
4. **Add sequence-specific conditions**: Only when they differ from test case conditions
5. **Keep naming consistent**: Use patterns like "Test Sequence #NN Description"

### Troubleshooting

#### Fuzzy Search Shows No Items

**Problem**: Fuzzy search shows empty list

**Solution**: This happens when no sequences exist yet. The first sequence must be typed manually.

#### Description Not Saved

**Problem**: Description is empty after editing in editor

**Solution**: Make sure to write content outside comment lines (lines starting with #).

#### Sequence ID Skipped

**Problem**: Sequence IDs jump (1, 2, 4 instead of 1, 2, 3)

**Solution**: This is normal if you manually added a sequence with ID 3. The builder finds the max ID and adds 1.

#### Validation Fails

**Problem**: Sequence validation fails with missing field

**Solution**: The builder automatically adds required fields (id, name, steps). If validation fails, check your manual edits.

### Advantages Over Manual Editing

| Feature | Manual YAML Editing | Test Sequence Builder |
|---------|---------------------|----------------------|
| ID assignment | Manual, error-prone | Automatic, incremental |
| Name reuse | Copy/paste, inconsistent | Fuzzy search, consistent |
| Validation | After save, late feedback | Before append, immediate |
| Git history | Manual commits, unclear | Automatic, descriptive |
| Description editing | Text editor, full YAML | Dedicated editor, clean |
| Error handling | Parse errors, unclear | Structured validation |

## See Also

- [Validation Documentation](validation.md)
- [Schema Reference](../testcases/schema.json)
- [Git Integration](../README.md#git-operations)
