# Detailed Validation Error Messages - Quick Reference

## What Changed

The YAML validation system now captures and displays **detailed error information** including:
- **JSON Path**: Where the error occurred in the document
- **Constraint Type**: What validation rule failed
- **Expected Value**: What the schema required
- **Found Value**: What was actually in the file

## Quick Start

### CLI Commands

```bash
# List all files with validation status
cargo run -- list --verbose

# Validate all files with detailed errors
cargo run -- validate --all

# Validate a specific file
cargo run -- validate --file test_case.yaml
```

### Example Output

```
✗ Invalid: test_case.yaml
  Validation Errors (2):

    Error #1: Path '/item'
      Constraint: type_mismatch
      Expected: Expected type: integer
      Found: "not_an_integer"

    Error #2: Path '/test_sequences/0/steps'
      Constraint: missing_property
      Expected: Required property 'steps' is missing
      Found: <missing field 'steps'>
```

## API Usage

### Get Detailed Error Information

```rust
use testcase_manager::SchemaValidator;

let validator = SchemaValidator::new()?;
let errors = validator.validate_with_details(yaml_content)?;

if !errors.is_empty() {
    for error in errors {
        eprintln!("Path: {}", error.path);
        eprintln!("  Type: {}", error.constraint);
        eprintln!("  Expected: {}", error.expected_constraint);
        eprintln!("  Found: {}", error.found_value);
    }
}
```

### Load Files with Validation Status

```rust
use testcase_manager::{TestCaseStorage, FileValidationStatus};

let storage = TestCaseStorage::new("data")?;
let file_infos = storage.load_all_with_validation()?;

for file_info in file_infos {
    match &file_info.status {
        FileValidationStatus::Valid => {
            println!("✓ {}", file_info.path.display());
        }
        FileValidationStatus::ValidationError { errors } => {
            println!("✗ {} ({} errors)", 
                file_info.path.display(), errors.len());
            for error in errors {
                println!("  - {}: {}", error.path, error.constraint);
            }
        }
        FileValidationStatus::ParseError { message } => {
            println!("✗ {}: {}", file_info.path.display(), message);
        }
    }
}
```

## Common Error Types

| Constraint | Meaning | Example |
|------------|---------|---------|
| `type_mismatch` | Wrong data type | Expected integer, found string |
| `missing_property` | Required field absent | Field 'test_sequences' is missing |
| `pattern_mismatch` | String pattern failed | ID must match pattern "TC_\\d+" |
| `minimum_value` | Value too small | Must be >= 1 |
| `maximum_value` | Value too large | Must be <= 100 |
| `oneOf_validation` | Schema match failed | Must match one of the allowed schemas |

## Data Structures

### ValidationErrorDetail

```rust
pub struct ValidationErrorDetail {
    pub path: String,              // "/test_sequences/0/steps/1"
    pub constraint: String,         // "type_mismatch"
    pub found_value: String,        // "\"invalid\""
    pub expected_constraint: String // "Expected type: integer"
}
```

### FileValidationStatus

```rust
pub enum FileValidationStatus {
    Valid,
    ParseError { message: String },
    ValidationError { errors: Vec<ValidationErrorDetail> }
}
```

### TestCaseFileInfo

```rust
pub struct TestCaseFileInfo {
    pub path: PathBuf,
    pub status: FileValidationStatus,
    pub test_case: Option<TestCase>
}
```

## New Methods

### SchemaValidator

- `validate_with_details(&self, yaml_content: &str) -> Result<Vec<ValidationErrorDetail>>`
  - Returns structured error details instead of generic error message

### TestCaseStorage

- `load_all_with_validation(&self) -> Result<Vec<TestCaseFileInfo>>`
  - Loads all files and returns validation status for each

## Backward Compatibility

All existing methods work unchanged:
- `validate_chunk()` - Still validates and returns Result
- `validate_complete()` - Still validates complete documents
- `load_all_test_cases()` - Now prints detailed warnings for invalid files

## See Also

- Full documentation: `docs/validation.md`
- Example code: `examples/validate_gsma_schema.rs`
- Implementation details: `IMPLEMENTATION_DETAILED_VALIDATION.md`
