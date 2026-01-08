# Validation Module

## Overview

The validation module provides JSON schema validation for YAML structures with detailed error reporting. It includes:

1. **SchemaValidator**: Validates YAML against the schema defined in `data/schema.json`
2. **Detailed Error Reporting**: Captures validation errors with JSON path, constraint type, expected values, and actual values
3. **File Loading with Validation**: Scans directories and provides validation status for each file

## SchemaValidator

### Purpose

Validates partial or complete YAML structures against the GSMA test case schema defined in `data/schema.json`.

### Usage

```rust
use testcase_manager::validation::SchemaValidator;

fn main() -> anyhow::Result<()> {
    // Create a validator (loads data/schema.json)
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

Creates a new `SchemaValidator` by loading and compiling the schema from `data/schema.json`.

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

The schema in `data/schema.json` defines the structure for GSMA test cases with the following top-level properties:

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
