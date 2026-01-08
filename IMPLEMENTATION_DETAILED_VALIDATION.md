# Implementation Summary: Detailed Validation Error Messages

## Overview

This implementation enhances the YAML file validation system to capture and display detailed validation error messages from the JSON schema validator. The system now provides comprehensive error information including the specific JSON path where validation failed, the constraint that was violated, what was expected, and what was actually found in the file.

## Changes Made

### 1. New Data Structures (`src/models.rs`)

Added three new types to support detailed validation error reporting:

#### `ValidationErrorDetail`
Stores detailed information about a single validation error:
- `path`: JSON path where the error occurred (e.g., "/test_sequences/0/steps/1/expected")
- `constraint`: Type of constraint that failed (e.g., "type_mismatch", "missing_property")
- `found_value`: The actual value found in the file
- `expected_constraint`: Description of what the schema expected

#### `FileValidationStatus`
Enum representing the validation status of a file:
- `Valid`: File passed validation
- `ParseError { message }`: File failed to parse as YAML
- `ValidationError { errors }`: File failed schema validation with detailed errors

#### `TestCaseFileInfo`
Contains information about a test case file:
- `path`: Path to the file
- `status`: Validation status
- `test_case`: Optional parsed test case data

### 2. Enhanced Validation Module (`src/validation.rs`)

#### New Method: `validate_with_details()`
Returns structured validation error details instead of just failing:
```rust
pub fn validate_with_details(&self, yaml_content: &str) -> Result<Vec<ValidationErrorDetail>>
```

This method:
1. Parses the YAML content
2. Validates against the JSON schema
3. Extracts detailed error information for each validation failure
4. Returns a vector of `ValidationErrorDetail` structures

#### Helper Functions
Added utility functions to extract constraint information:
- `extract_constraint_info()`: Parses error messages to identify constraint types (type_mismatch, missing_property, pattern_mismatch, etc.)
- `extract_instance_value()`: Navigates JSON paths to retrieve the actual value that failed validation

### 3. Enhanced Storage Module (`src/storage.rs`)

#### Updated `load_all_test_cases()`
Enhanced to display detailed validation errors when files are loaded:
- Shows validation error details including path, constraint, and expected values
- Limits error display to top 3 errors per file to avoid overwhelming output
- Continues loading valid files even when some files fail validation

#### New Method: `load_all_with_validation()`
```rust
pub fn load_all_with_validation(&self) -> Result<Vec<TestCaseFileInfo>>
```

Scans directory and returns detailed validation status for all files:
- Attempts to parse each YAML file
- Validates parsed content against schema
- Returns structured information about each file including validation errors

#### New Helper: `load_file_with_validation()`
Internal method that loads a single file and captures validation details.

### 4. Enhanced CLI (`src/main.rs`)

#### Updated `handle_validate()`
Completely rewritten to display detailed validation errors:

**For single file validation (`--file`):**
- Shows validation error count
- Displays each error with:
  - Error number
  - JSON path
  - Constraint type
  - Expected value
  - Found value

**For batch validation (`--all`):**
- Validates all files in directory
- Shows detailed errors for each invalid file
- Provides summary statistics:
  - Total files
  - Valid count
  - Schema violation count
  - Parse error count

#### Updated `handle_list()` 
Enhanced verbose mode to show validation status:
- When using `--verbose`, displays validation status for each file
- Shows valid files with checkmark (✓)
- Shows invalid files with error details (✗)
- Limits error display to top 3 per file

### 5. Updated Exports (`src/lib.rs`)

Added public exports for new types:
- `ValidationErrorDetail`
- `FileValidationStatus`
- `TestCaseFileInfo`

### 6. Enhanced Example (`examples/validate_gsma_schema.rs`)

Updated to demonstrate new functionality:
- Shows basic validation with `validate_chunk()`
- Demonstrates detailed error reporting with `validate_with_details()`
- Shows batch file validation with `load_all_with_validation()`
- Includes example output formatting

### 7. Updated Documentation (`docs/validation.md`)

Comprehensive documentation updates:
- Added section on `validate_with_details()` method
- Documented `ValidationErrorDetail` structure
- Added "File Validation and Listing" section
- Included CLI command examples with sample output
- Added code examples for programmatic usage

## Usage Examples

### CLI Usage

**List files with validation status:**
```bash
cargo run -- list --verbose
```

**Validate all files with detailed errors:**
```bash
cargo run -- validate --all
```

**Validate a single file:**
```bash
cargo run -- validate --file test_case.yaml
```

### Programmatic Usage

**Get detailed validation errors:**
```rust
let validator = SchemaValidator::new()?;
let errors = validator.validate_with_details(yaml_content)?;

for error in errors {
    println!("Error at {}: {}", error.path, error.constraint);
    println!("  Expected: {}", error.expected_constraint);
    println!("  Found: {}", error.found_value);
}
```

**Load files with validation status:**
```rust
let storage = TestCaseStorage::new("data")?;
let file_infos = storage.load_all_with_validation()?;

for file_info in file_infos {
    match &file_info.status {
        FileValidationStatus::Valid => println!("✓ Valid"),
        FileValidationStatus::ValidationError { errors } => {
            for error in errors {
                println!("✗ Path '{}': {}", error.path, error.constraint);
            }
        }
        FileValidationStatus::ParseError { message } => {
            println!("✗ Parse error: {}", message);
        }
    }
}
```

## Error Information Captured

The system now captures and displays:

1. **JSON Path**: Exact location in the document where validation failed
   - Example: `/test_sequences/0/steps/1/expected`

2. **Constraint Type**: The specific validation rule that was violated
   - `type_mismatch`: Wrong data type
   - `missing_property`: Required field not present
   - `pattern_mismatch`: String pattern validation failed
   - `minimum_value`/`maximum_value`: Numeric range violation
   - `oneOf_validation`: Multiple schema match failed

3. **Expected Constraint**: What the schema required
   - Example: "Expected type: integer"
   - Example: "Required property 'test_sequences' is missing"

4. **Found Value**: The actual value in the file
   - Formatted appropriately for the type (strings quoted, numbers plain, etc.)
   - Shows `<missing field>` for missing properties
   - Shows object/array indicators for complex types

## Benefits

1. **Better Debugging**: Developers can quickly identify exactly what's wrong with their YAML files
2. **Clear Guidance**: Error messages explain both what was expected and what was found
3. **Efficient Troubleshooting**: JSON paths make it easy to locate problems in large files
4. **Batch Processing**: Can validate entire directories and see all errors at once
5. **Programmatic Access**: Error details are structured data that can be processed by tools

## Backward Compatibility

All existing validation methods (`validate_chunk()`, `validate_complete()`) continue to work as before. The new functionality is additive and doesn't break existing code.
