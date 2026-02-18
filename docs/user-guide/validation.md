# Validation Module

## Overview

The validation module provides JSON schema validation for YAML structures with detailed error reporting. It includes:

1. **SchemaValidator**: Validates YAML against the schema defined in `testcases/schema.json`
2. **Detailed Error Reporting**: Captures validation errors with JSON path, constraint type, expected values, and actual values
3. **File Loading with Validation**: Scans directories and provides validation status for each file
4. **Watch Mode**: Continuously monitors directories for file changes and automatically validates modified files

## validate-yaml Binary

The `validate-yaml` binary is a command-line tool for validating YAML files against JSON schema definitions. It supports both single-file and multi-file validation, with optional watch mode for continuous monitoring.

> **Quick Reference**: See [VALIDATE_YAML_QUICK_REF.md](VALIDATE_YAML_QUICK_REF.md) for a comprehensive quick reference guide with examples, troubleshooting, and integration tips.

### Command-Line Interface

```
validate-yaml [OPTIONS] <YAML_FILES>... --schema <SCHEMA_FILE>

Arguments:
  <YAML_FILES>...         Path(s) to the YAML payload file(s) to validate

Options:
  -s, --schema <SCHEMA_FILE>   Path to the JSON schema file
  -w, --watch                  Watch mode - monitor YAML files for changes and re-validate (Linux/macOS only)
  -v, --verbose                Enable verbose logging
  -h, --help                   Print help
  -V, --version                Print version
```

### Basic Validation (Without Watch Mode)

**Single File:**
```bash
validate-yaml testcase.yml --schema schema.json
```

**Multiple Files:**
```bash
validate-yaml test1.yml test2.yml test3.yml --schema schema.json
```

**Using Shell Globs:**
```bash
validate-yaml testcases/*.yml --schema schema.json
```

**With Verbose Output:**
```bash
validate-yaml testcase.yml --schema schema.json --verbose
```

### Output Format

**Success:**
```
✓ testcases/test1.yml
✓ testcases/test2.yml

Summary:
  Total files validated: 2
  Passed: 2
  Failed: 0
```

**Validation Errors:**
```
✗ testcases/test_bad.yml
  Schema constraint violations:
    Error #1: Path '/item'
      Constraint: "not a integer"
      Found value: "not_an_integer"

Summary:
  Total files validated: 1
  Passed: 0
  Failed: 1
```

### Exit Codes

- `0`: All validations passed
- `1`: One or more validations failed

## Watch Mode for File Validation

> **Note**: There are two watch mode implementations available. See [WATCH_MODE_COMPARISON.md](WATCH_MODE_COMPARISON.md) for a detailed comparison to help you choose the right one for your needs.

### validate-yaml Binary with --watch Flag

The `validate-yaml` binary includes built-in watch mode support for monitoring YAML files and automatically re-validating them when changes are detected.

#### Platform Support

- **Linux**: Full support (uses `notify` crate with inotify backend)
- **macOS**: Full support (uses `notify` crate with FSEvents backend)
- **Windows**: Watch mode is **disabled** (the `--watch` flag is not available on Windows)

#### Basic Usage

**Single File Validation with Watch Mode:**
```bash
validate-yaml testcase.yml --schema schema.json --watch
```

**Multiple File Validation with Watch Mode:**
```bash
validate-yaml testcase1.yml testcase2.yml testcase3.yml --schema schema.json --watch
```

**Using Glob Patterns:**
```bash
validate-yaml testcases/*.yml --schema schema.json --watch
```

**With Verbose Logging:**
```bash
validate-yaml testcases/*.yml --schema schema.json --watch --verbose
```

#### Watch Mode Features

1. **Initial Validation**: Runs complete validation on all specified files at startup
2. **Real-time Monitoring**: Detects file modifications immediately using native OS file watching
3. **Debounced Event Handling**: Groups rapid file changes (300ms debounce window) to avoid duplicate validations
4. **Smart Re-validation**: 
   - Validates only changed files first
   - Shows immediate results for changed files
   - When all changed files pass, automatically runs full validation on all watched files
5. **Color-coded Output**:
   - Green checkmark (✓) for passing files
   - Red X (✗) for failing files
   - Bold text for emphasis
   - Yellow highlights for change notifications
6. **Detailed Error Messages**: Shows JSON path, constraint violations, and found values

#### How Watch Mode Works

1. **Startup Phase**:
   - Parses command-line arguments
   - Loads and compiles JSON schema
   - Performs initial validation on all specified YAML files
   - Displays results and summary

2. **Monitoring Phase**:
   - Creates file system watchers for each specified file
   - Canonicalizes file paths for reliable change detection
   - Listens for file modification events

3. **Change Detection**:
   - Receives file system events from OS
   - Filters for modification events only
   - Adds changed files to debounce buffer
   - Waits 300ms after last event before processing

4. **Re-validation Phase**:
   - Displays list of changed files
   - Validates only the changed files
   - Shows results for changed files
   - If all changed files pass:
     - Displays "All changed files passed!" message
     - Runs full validation on all watched files
     - Shows complete results and summary
   - If any changed file fails:
     - Shows summary for changed files only
     - Waits for next change

#### Exit Watch Mode

Press `Ctrl+C` to stop watching and exit.

#### Example Output

```
Watch mode enabled
Monitoring 3 files for changes...

Initial validation:
✓ testcases/test1.yml
✓ testcases/test2.yml
✓ testcases/test3.yml

Summary:
  Total files validated: 3
  Passed: 3
  Failed: 0

Watching for changes...

File changes detected:
  → /path/to/testcases/test2.yml

Validating changed files:
✓ testcases/test2.yml

All changed files passed! Running full validation...

✓ testcases/test1.yml
✓ testcases/test2.yml
✓ testcases/test3.yml

Summary:
  Total files validated: 3
  Passed: 3
  Failed: 0

Watching for changes...
```

#### Windows Limitations

On Windows, the `--watch` flag is not compiled into the binary due to platform-specific limitations. Attempting to use watch mode on Windows will result in a command-line parsing error indicating the flag is not recognized.

**Windows users** should use the standard validation mode without `--watch`:
```bash
validate-yaml testcase.yml --schema schema.json
```

For continuous validation workflows on Windows, consider:
- Setting up a file watcher using external tools (e.g., PowerShell FileSystemWatcher)
- Using WSL (Windows Subsystem for Linux) to run the Linux version with watch mode
- Implementing a scheduled task to periodically run validation

---

### validate-files.sh Script Watch Mode

The `validate-files.sh` script provides an alternative watch mode implementation with different features, useful for directory-wide monitoring with pattern matching.

#### Installation Requirements

**Linux:**
```bash
sudo apt-get install inotify-tools
```

**macOS:**
```bash
brew install fswatch
```

#### Usage

Basic watch mode (monitors `testcases/` directory by default):
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch
```

Watch a custom directory:
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch path/to/dir/
```

With verbose output:
```bash
./scripts/validate-files.sh --pattern '\.ya?ml$' --validator ./scripts/validate-yaml-wrapper.sh --watch --verbose
```

#### Features

- **Initial Validation**: Runs a complete validation of all matching files on startup
- **Live Monitoring**: Detects file modifications, creations, deletions, and moves in real-time
- **Instant Feedback**: Validates changed files immediately and displays results with color-coded output
- **Persistent Cache**: Maintains validation cache across watch sessions for fast re-validation
- **Cache Cleanup**: Automatically removes cache entries for deleted files
- **Pattern Matching**: Only validates files matching the specified regex pattern

#### How It Works

1. On startup, the script performs an initial validation of all files matching the pattern
2. It then starts monitoring the specified directory recursively
3. When a file change is detected:
   - The script checks if the file matches the specified pattern
   - If it matches, validation is triggered immediately
   - Results are displayed with color-coded output (green for pass, red for fail)
   - The cache is updated with the new validation result
4. For deleted files, the corresponding cache entries are automatically removed

#### Exit

Press `Ctrl+C` to stop watch mode and exit the script.

## SchemaValidator

### Purpose

Validates partial or complete YAML structures against the GSMA test case schema defined in `testcases/schema.json`.

### Usage

```rust
use testcase_manager::validation::SchemaValidator;

fn main() -> anyhow::Result<()> {
    // Create a validator (loads testcases/schema.json)
    let validator = SchemaValidator::new()?;
    
    // Validate a complete YAML structure
    let yaml_content = r#"
requirement: XXX100
item: 1
tc: 4
id: '4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata'
description: 'Test description'
general_initial_conditions:
  - eUICC:
      - "Some condition"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences:
  - id: 1
    name: "Test Sequence"
    description: "Test description"
    initial_conditions:
      - eUICC:
          - "Condition"
    steps:
      - step: 1
        description: "Step description"
        command: "ssh"
        expected:
          success: true
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
  - id: 2
    name: "Test Sequence 2"
    description: "Test description 2"
    initial_conditions:
      - eUICC:
          - "Condition"
    steps:
      - step: 1
        description: "Step"
        command: "ssh"
        expected:
          success: false
          result: "SW=0x9000"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "ssh"
        expected:
          result: "SW=0x9000"
          output: "Success"
"#;
    
    validator.validate_chunk(yaml_content)?;
    println!("✓ Validation successful!");
    
    Ok(())
}
```

### API Methods

#### `new() -> Result<Self>`

Creates a new `SchemaValidator` by loading and compiling the schema from `testcases/schema.json`.

**Returns:**
- `Ok(SchemaValidator)` on success
- `Err` if the schema file cannot be read or compiled

#### `validate_chunk(&self, yaml_content: &str) -> Result<()>`

Validates a complete or partial YAML structure against the schema.

**Parameters:**
- `yaml_content`: YAML string to validate

**Returns:**
- `Ok(())` if validation succeeds
- `Err` with clear error messages if validation fails

**Error Messages:**
The validator provides detailed error messages for schema violations:
- Missing required properties
- Type mismatches (e.g., string instead of integer)
- Invalid enum values
- Array/string length violations
- Pattern match failures
- Additional properties not allowed

**Example Error Output:**
```
Schema validation failed:
  - Path '/item': Invalid type, expected integer
  - Path 'root': Missing required property 'test_sequences'
```

#### `validate_with_details(&self, yaml_content: &str) -> Result<Vec<ValidationErrorDetail>>`

**NEW** - Validates YAML content and returns structured error details for comprehensive error reporting.

**Parameters:**
- `yaml_content`: YAML string to validate

**Returns:**
- `Ok(Vec<ValidationErrorDetail>)` - Empty vector if valid, otherwise contains detailed error information
- `Err` if the YAML cannot be parsed

**ValidationErrorDetail Structure:**
Each error detail contains:
- `path`: JSON path where the error occurred (e.g., "/test_sequences/0/steps/1/expected")
- `constraint`: Type of constraint that failed (e.g., "type_mismatch", "missing_property")
- `found_value`: The actual value found in the file
- `expected_constraint`: Description of what was expected by the schema

**Example Usage:**
```rust
let validator = SchemaValidator::new()?;
let errors = validator.validate_with_details(yaml_content)?;

if errors.is_empty() {
    println!("✓ Valid");
} else {
    for error in errors {
        println!("Error at {}: {}", error.path, error.constraint);
        println!("  Expected: {}", error.expected_constraint);
        println!("  Found: {}", error.found_value);
    }
}
```

#### `validate_partial_chunk(&self, yaml_content: &str) -> Result<()>`

Validates a partial YAML structure, allowing empty objects.

**Parameters:**
- `yaml_content`: YAML string to validate (can be incomplete)

**Returns:**
- `Ok(())` if validation succeeds or object is empty
- `Err` with error messages if validation fails

### Error Formatting

The validator provides human-readable error messages that include:

1. **Path**: The location in the YAML structure where the error occurred
2. **Error Type**: The specific validation rule that failed
3. **Expected Value**: What the schema expected (for type/enum errors)

### Example

See `examples/validate_gsma_schema.rs` for a complete working example.

Run it with:
```bash
cargo run --example validate_gsma_schema
```

## TestCaseValidator

The `TestCaseValidator` validates internal test case models against the built-in test case schema. It's used by the CLI for validating test cases in the standard format.

### Usage

```rust
use testcase_manager::{TestCase, TestCaseValidator};

let validator = TestCaseValidator::new()?;
let test_case = TestCase::new("TC001".to_string(), "Test".to_string());
validator.validate_test_case(&test_case)?;
```

## File Validation and Listing

### Loading Files with Validation Status

The storage module provides `load_all_with_validation()` to scan directories and return detailed validation information for each file.

**Usage:**
```rust
use testcase_manager::{TestCaseStorage, FileValidationStatus};

let storage = TestCaseStorage::new("data")?;
let file_infos = storage.load_all_with_validation()?;

for file_info in file_infos {
    match &file_info.status {
        FileValidationStatus::Valid => {
            println!("✓ {} is valid", file_info.path.display());
        }
        FileValidationStatus::ParseError { message } => {
            println!("✗ {} failed to parse: {}", file_info.path.display(), message);
        }
        FileValidationStatus::ValidationError { errors } => {
            println!("✗ {} has {} validation error(s):", 
                file_info.path.display(), errors.len());
            for error in errors {
                println!("  - Path '{}': {}", error.path, error.constraint);
                println!("    Expected: {}", error.expected_constraint);
                println!("    Found: {}", error.found_value);
            }
        }
    }
}
```

### CLI Commands

#### List with Validation (Verbose Mode)

```bash
# List all files with validation status
cargo run -- list --verbose

# Example output:
# ✓ test_case_1.yaml (Valid)
#   ID: TC001
#   Requirement: REQ001
#
# ✗ test_case_2.yaml (Schema Validation Failed: 2 error(s))
#   Error #1: Path '/item' - type_mismatch
#   Error #2: Path 'root' - missing_property
```

#### Validate All Files

```bash
# Validate all files in the directory with detailed error output
cargo run -- validate --all

# Example output:
# ✓ Valid: test_case_1.yaml
# 
# ✗ Invalid: test_case_2.yaml
#   Validation Errors (2):
#
#     Error #1: Path '/item'
#       Constraint: type_mismatch
#       Expected: Expected type: integer
#       Found: "not_an_integer"
#
# === Validation Summary ===
# Total files: 2
# ✓ Valid: 1
# ✗ Schema violations: 1
# ✗ Parse errors: 0
```

#### Validate Single File

```bash
# Validate a specific file
cargo run -- validate --file test_case.yaml
```

## Schema Structure

The schema in `testcases/schema.json` defines the structure for GSMA test cases with the following top-level properties:

- `requirement`: (string) Requirement identifier
- `item`: (integer) Item number
- `tc`: (integer) Test case number
- `id`: (string) Test case identifier
- `description`: (string) Test case description
- `general_initial_conditions`: (array) General initial conditions
- `initial_conditions`: (object) Initial conditions
- `test_sequences`: (array) Test sequences with steps

Each test sequence contains:
- `id`: (integer) Sequence identifier
- `name`: (string) Sequence name
- `description`: (string) Sequence description
- `initial_conditions`: (array) Sequence-specific initial conditions
- `steps`: (array) Test steps

Each step contains:
- `step`: (integer) Step number
- `description`: (string) Step description
- `command`: (string) Command to execute
- `manual`: (boolean, optional) Whether step is manual
- `expected`: (object) Expected result with `success`, `result`, and `output` fields
