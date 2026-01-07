# Interactive Test Case Creation - Implementation Summary

## Overview

This document describes the implementation of the interactive test case creation workflow with metadata prompts, validation, and git commits.

## Implemented Features

### 1. Interactive Metadata Prompts

**Location**: `src/prompts.rs`

**Implementation**:
- `Prompts::prompt_metadata()` - Prompts for all metadata fields
- `Prompts::input_integer()` - Prompts for integer values with validation
- `TestCaseMetadata` struct - Holds metadata fields
- `TestCaseMetadata::to_yaml()` - Converts to YAML structure
- `TestCaseMetadata::validate()` - Validates against schema

**Fields Prompted**:
1. `requirement` (String) - Requirement identifier
2. `item` (Integer) - Item number  
3. `tc` (Integer) - Test case number
4. `id` (String) - Unique test case ID
5. `description` (String) - Test case description

**Example Usage**:
```rust
let metadata = Prompts::prompt_metadata()?;
let validator = SchemaValidator::new()?;
metadata.validate(&validator)?;
```

### 2. Metadata Validation

**Location**: `src/prompts.rs`, `src/validation.rs`

**Implementation**:
- Uses `SchemaValidator` to validate metadata chunk
- Validates partial YAML structures
- Clear error messages for validation failures

**Validation Process**:
1. Convert metadata to YAML structure
2. Serialize to YAML string
3. Validate against schema using `validate_partial_chunk()`
4. Return detailed error messages if validation fails

### 3. Git Commit Integration

**Location**: `src/builder.rs`, `src/git.rs`

**Implementation**:
- `TestCaseBuilder::commit()` - Commits current structure to git
- Uses `GitManager::commit_progress()` for descriptive commits
- Automatic file writing before commit
- Uses environment variables for author information

**Commit Flow**:
1. Convert structure to YAML string
2. Write to file
3. Stage file with `git.add()`
4. Commit with descriptive message
5. Return commit OID

**Environment Variables**:
- `GIT_AUTHOR_NAME` - Author name (default: "Test Case Manager")
- `GIT_AUTHOR_EMAIL` - Author email (default: "testcase@example.com")

### 4. General Initial Conditions Flow

**Location**: `src/prompts.rs`, `src/builder.rs`

**Implementation**:
- `Prompts::prompt_general_initial_conditions()` - Interactive prompt with editor
- `TestCaseBuilder::add_general_initial_conditions()` - Adds to structure

**Workflow**:
1. **Show Defaults**: Display current defaults if available
2. **Prompt to Keep/Edit**: Ask user to keep defaults or edit
3. **Launch Editor**: Open editor with template if editing
4. **Parse YAML**: Parse edited content
5. **Validate**: Validate against schema
6. **Retry on Failure**: Allow user to retry if validation fails
7. **Add to Structure**: Add validated content to structure

**Template**:
```yaml
# General Initial Conditions
# Example:
# - eUICC:
#     - "Condition 1"
#     - "Condition 2"

- eUICC:
    - ""
```

### 5. Initial Conditions Flow

**Location**: `src/builder.rs`

**Implementation**:
- `TestCaseBuilder::add_initial_conditions()` - Interactive prompt with editor
- Same workflow as general initial conditions
- Different YAML structure (object instead of array)

**Template**:
```yaml
# Initial Conditions
# Example:
# eUICC:
#   - "Condition 1"
#   - "Condition 2"

eUICC:
  - ""
```

### 6. Test Case Builder

**Location**: `src/builder.rs`

**Main Struct**: `TestCaseBuilder`

**Key Methods**:
- `new()` - Create builder with path and validator
- `add_metadata()` - Prompt for and add metadata
- `add_general_initial_conditions()` - Add general initial conditions
- `add_initial_conditions()` - Add initial conditions
- `add_field()` - Add custom field to structure
- `validate()` - Validate entire structure
- `commit()` - Commit to git
- `save()` - Save to file
- `to_yaml_string()` - Convert structure to YAML

**Structure**:
- Uses `IndexMap<String, Value>` to maintain field order
- Integrates with `SchemaValidator` for validation
- Integrates with `GitManager` for version control

### 7. CLI Command

**Location**: `src/cli.rs`, `src/main.rs`

**Command**: `create-interactive`

**Options**:
- `--path` - Custom path for test cases (optional)

**Handler**: `handle_create_interactive()`

**Flow**:
1. Create `TestCaseBuilder`
2. Display welcome message
3. Prompt for and add metadata
4. Optionally commit metadata
5. Prompt for and add general initial conditions
6. Optionally commit general initial conditions
7. Prompt for and add initial conditions
8. Optionally commit initial conditions
9. Save file
10. Display success message

## Files Created/Modified

### New Files

1. **`src/builder.rs`** (297 lines)
   - `TestCaseBuilder` struct and implementation
   - Unit tests for builder functionality

2. **`examples/interactive_workflow.rs`** (117 lines)
   - Example demonstrating the workflow
   - Shows programmatic usage

3. **`docs/interactive_workflow.md`** (489 lines)
   - Comprehensive user documentation
   - Usage examples and troubleshooting

4. **`docs/INTERACTIVE_IMPLEMENTATION.md`** (this file)
   - Technical implementation documentation

### Modified Files

1. **`src/prompts.rs`**
   - Added `prompt_metadata()` method
   - Added `input_integer()` method
   - Added `prompt_general_initial_conditions()` method
   - Added `TestCaseMetadata` struct and implementation
   - Added imports for editor and validator

2. **`src/cli.rs`**
   - Added `CreateInteractive` command variant

3. **`src/main.rs`**
   - Added `handle_create_interactive()` function
   - Added match arm for `CreateInteractive` command
   - Added `TestCaseBuilder` import

4. **`src/lib.rs`**
   - Added `builder` module
   - Exported `TestCaseBuilder`
   - Exported `TestCaseMetadata`

5. **`README.md`**
   - Added section for `create-interactive` command
   - Added documentation link

## Dependencies

No new dependencies were added. The implementation uses:
- `dialoguer` - For interactive prompts (already present)
- `edit` - For editor integration (already present)
- `serde_yaml` - For YAML handling (already present)
- `indexmap` - For ordered maps (already present)
- `anyhow` - For error handling (already present)
- `git2` - For git operations (already present)

## Testing

### Unit Tests

**Location**: `src/builder.rs`

**Tests**:
1. `test_builder_creation` - Tests builder instantiation
2. `test_add_field` - Tests adding custom fields
3. `test_to_yaml_string` - Tests YAML serialization
4. `test_save_file` - Tests file saving
5. `test_complete_metadata` - Tests complete metadata workflow

**Run Tests**:
```bash
cargo test
```

### Example Program

**Run Example**:
```bash
cargo run --example interactive_workflow
```

This demonstrates the workflow without requiring user interaction.

## Integration with Existing Code

### Schema Validator

The implementation uses the existing `SchemaValidator` from `src/validation.rs`:
- Validates metadata chunks
- Validates general initial conditions
- Validates initial conditions
- Provides clear error messages

### Git Manager

The implementation uses the existing `GitManager` from `src/git.rs`:
- Uses `commit_progress()` for commits
- Stages files with `add()`
- Handles author information from environment variables

### Editor Integration

The implementation uses the existing `TestCaseEditor` from `src/editor.rs`:
- Opens default editor for complex structures
- Handles editor failures gracefully
- Supports all editor environment variables

## Workflow Diagram

```
┌─────────────────────────────────────┐
│  Start Interactive Workflow         │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Prompt for Metadata                │
│  - requirement                      │
│  - item                             │
│  - tc                               │
│  - id                               │
│  - description                      │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Validate Metadata                  │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Commit Metadata? (optional)        │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Add General Initial Conditions?    │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Show Defaults / Edit in Editor     │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Validate Structure                 │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Commit General Initial Conditions? │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Add Initial Conditions?            │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Show Defaults / Edit in Editor     │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Validate Structure                 │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Commit Initial Conditions?         │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Save File                          │
└─────────────┬───────────────────────┘
              │
              ▼
┌─────────────────────────────────────┐
│  Display Success Message            │
└─────────────────────────────────────┘
```

## API Reference

### TestCaseBuilder

```rust
pub struct TestCaseBuilder {
    base_path: PathBuf,
    validator: SchemaValidator,
    git_manager: Option<GitManager>,
    structure: IndexMap<String, Value>,
}

impl TestCaseBuilder {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self>;
    pub fn add_metadata(&mut self) -> Result<&mut Self>;
    pub fn add_general_initial_conditions(&mut self, defaults: Option<&Value>) -> Result<&mut Self>;
    pub fn add_initial_conditions(&mut self, defaults: Option<&Value>) -> Result<&mut Self>;
    pub fn add_field(&mut self, key: String, value: Value) -> Result<&mut Self>;
    pub fn validate(&self) -> Result<()>;
    pub fn commit(&self, message: &str) -> Result<()>;
    pub fn save(&self) -> Result<PathBuf>;
    pub fn to_yaml_string(&self) -> Result<String>;
    pub fn structure(&self) -> &IndexMap<String, Value>;
    pub fn structure_mut(&mut self) -> &mut IndexMap<String, Value>;
}
```

### TestCaseMetadata

```rust
#[derive(Debug, Clone)]
pub struct TestCaseMetadata {
    pub requirement: String,
    pub item: i64,
    pub tc: i64,
    pub id: String,
    pub description: String,
}

impl TestCaseMetadata {
    pub fn to_yaml(&self) -> IndexMap<String, Value>;
    pub fn validate(&self, validator: &SchemaValidator) -> Result<()>;
}
```

### Prompts

```rust
impl Prompts {
    pub fn prompt_metadata() -> Result<TestCaseMetadata>;
    pub fn input_integer(prompt: &str) -> Result<i64>;
    pub fn prompt_general_initial_conditions(
        defaults: Option<&Value>,
        validator: &SchemaValidator,
    ) -> Result<Value>;
}
```

## Error Handling

All functions return `anyhow::Result<T>` for consistent error handling:

1. **Validation Errors**: Clear messages showing what failed
2. **Editor Errors**: Graceful handling of editor failures
3. **Git Errors**: Informative messages about git issues
4. **YAML Parse Errors**: Detailed syntax error information

## Future Enhancements

Potential improvements:
1. Add test sequence prompts
2. Add preconditions and cleanup prompts
3. Support for test step creation
4. Template system for common patterns
5. Interactive validation error fixing
6. Undo/redo functionality
7. Import defaults from previous test cases

## Conclusion

The interactive test case creation workflow provides a comprehensive, guided experience for creating test cases with:
- ✅ Interactive metadata prompts
- ✅ Schema validation
- ✅ Git commit integration
- ✅ General initial conditions flow
- ✅ Initial conditions flow
- ✅ Editor integration
- ✅ Default value handling
- ✅ Clear error messages
- ✅ Comprehensive documentation
- ✅ Unit tests
- ✅ Example code

All requirements have been fully implemented and tested.
