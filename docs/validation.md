# Validation Module

## Overview

The validation module provides JSON schema validation for YAML structures. It includes two validators:

1. **SchemaValidator**: Validates YAML against the schema defined in `data/schema.json`
2. **TestCaseValidator**: Validates test cases against the internal test case schema

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
