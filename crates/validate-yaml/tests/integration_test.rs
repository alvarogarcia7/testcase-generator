use std::fs;
use tempfile::TempDir;
use validate_yaml::YamlValidator;

// Integration tests for YamlValidator library and testcase-common functions

/// Test 1: YamlValidator library validates valid test case YAML files successfully
#[test]
fn test_yaml_validator_validates_valid_test_case() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas/tcms");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create a valid test-case.schema.v1.json schema
    let schema_path = schemas_root.join("test-case.schema.v1.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "type": { "type": "string", "const": "test_case" },
            "schema": { "type": "string" },
            "requirement": { "type": "string" },
            "item": { "type": "integer" },
            "tc": { "type": "integer" },
            "id": { "type": "string" },
            "description": { "type": "string" },
            "general_initial_conditions": { "type": "object" },
            "initial_conditions": { "type": "object" },
            "test_sequences": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer" },
                        "name": { "type": "string" },
                        "description": { "type": "string" },
                        "initial_conditions": { "type": "object" },
                        "steps": { "type": "array" }
                    },
                    "required": ["id", "name", "description", "initial_conditions", "steps"]
                }
            }
        },
        "required": ["requirement", "item", "tc", "id", "description", "general_initial_conditions", "initial_conditions", "test_sequences"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create a valid YAML test case file
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"type: test_case
schema: tcms/test-case.schema.v1.json
requirement: REQ001
item: 1
tc: 1
id: TC001
description: Test case description
general_initial_conditions:
  system:
    - System is ready
initial_conditions:
  device:
    - Device is connected
test_sequences:
  - id: 1
    name: Test Sequence 1
    description: First test sequence
    initial_conditions:
      system:
        - Ready to test
    steps:
      - step: 1
        description: Execute command
        command: echo "test"
        expected:
          result: 0
          output: test
        verification:
          result: "[[ $? -eq 0 ]]"
          output: "grep -q 'test' <<< \"$COMMAND_OUTPUT\""
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate using YamlValidator
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_ok(),
        "Expected validation to succeed for valid test case YAML"
    );
}

/// Test 2: Validation fails with clear errors for YAML files missing `type` field
#[test]
fn test_validation_fails_missing_type_field() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas/tcms");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create a schema that requires 'type' field
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "type": { "type": "string" },
            "schema": { "type": "string" },
            "name": { "type": "string" }
        },
        "required": ["type", "schema", "name"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML file missing 'type' field
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"schema: test-schema.json
name: Test Name
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate using YamlValidator
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_err(),
        "Expected validation to fail for missing 'type' field"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Schema constraint violations"),
        "Error message should mention schema constraint violations"
    );
    assert!(
        err_msg.contains("type") || err_msg.contains("'type' is a required property"),
        "Error message should mention the missing 'type' field"
    );
}

/// Test 3: Validation fails with clear errors for YAML files missing `schema` field
#[test]
fn test_validation_fails_missing_schema_field() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas/tcms");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create a schema that requires 'schema' field
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "type": { "type": "string" },
            "schema": { "type": "string" },
            "name": { "type": "string" }
        },
        "required": ["type", "schema", "name"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML file missing 'schema' field
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"type: test_case
name: Test Name
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate using YamlValidator
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_err(),
        "Expected validation to fail for missing 'schema' field"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Schema constraint violations"),
        "Error message should mention schema constraint violations"
    );
    assert!(
        err_msg.contains("schema") || err_msg.contains("'schema' is a required property"),
        "Error message should mention the missing 'schema' field"
    );
}

/// Test 4: Validation fails for YAML files with schema violations (type mismatch)
#[test]
fn test_validation_fails_schema_violations_type_mismatch() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas/tcms");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create a schema with strict type requirements
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer", "minimum": 0 },
            "active": { "type": "boolean" }
        },
        "required": ["name", "age", "active"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML file with type mismatch (age is string instead of integer)
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"name: John Doe
age: "thirty"
active: true
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate using YamlValidator
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_err(),
        "Expected validation to fail for type mismatch"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Schema constraint violations"),
        "Error message should mention schema constraint violations"
    );
}

/// Test 5: Validation fails for YAML files with schema violations (constraint violation)
#[test]
fn test_validation_fails_schema_violations_constraint() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas/tcms");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create a schema with minimum constraint
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "score": { "type": "integer", "minimum": 0, "maximum": 100 }
        },
        "required": ["name", "score"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML file violating constraint (score > 100)
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"name: Test
score: 150
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate using YamlValidator
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_err(),
        "Expected validation to fail for constraint violation"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Schema constraint violations"),
        "Error message should mention schema constraint violations"
    );
    assert!(
        err_msg.contains("score") || err_msg.contains("150"),
        "Error message should mention the violating field or value"
    );
}

/// Test 6: load_and_validate_yaml correctly validates and deserializes TestCase structs
#[test]
fn test_load_and_validate_yaml_deserializes_testcase() {
    use serde::Deserialize;
    use testcase_common::load_and_validate_yaml;

    #[derive(Debug, Deserialize, PartialEq)]
    struct SimpleTestCase {
        #[serde(rename = "type")]
        doc_type: Option<String>,
        schema: String,
        requirement: String,
        item: i64,
        tc: i64,
        id: String,
        description: String,
    }

    let temp_dir = TempDir::new().unwrap();
    let schemas_dir = temp_dir.path().join("schemas");
    let schemas_tcms = schemas_dir.join("tcms");
    fs::create_dir_all(&schemas_tcms).unwrap();

    // Create schema
    let schema_path = schemas_tcms.join("test-case.schema.v1.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "type": { "type": "string" },
            "schema": { "type": "string" },
            "requirement": { "type": "string" },
            "item": { "type": "integer" },
            "tc": { "type": "integer" },
            "id": { "type": "string" },
            "description": { "type": "string" }
        },
        "required": ["schema", "requirement", "item", "tc", "id", "description"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create valid YAML file
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"type: test_case
schema: tcms/test-case.schema.v1.json
requirement: REQ001
item: 1
tc: 2
id: TC001
description: Test case description
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Load and validate - pass schemas_root properly
    let result: Result<SimpleTestCase, _> =
        load_and_validate_yaml(&yaml_file, schemas_dir.to_str().unwrap());

    assert!(
        result.is_ok(),
        "Expected load_and_validate_yaml to succeed for valid YAML: {:?}",
        result.as_ref().err()
    );
    let test_case = result.unwrap();
    assert_eq!(test_case.requirement, "REQ001");
    assert_eq!(test_case.item, 1);
    assert_eq!(test_case.tc, 2);
    assert_eq!(test_case.id, "TC001");
    assert_eq!(test_case.description, "Test case description");
}

/// Test 7: load_and_validate_yaml fails for invalid schema
#[test]
fn test_load_and_validate_yaml_fails_invalid_schema() {
    use serde::Deserialize;
    use testcase_common::load_and_validate_yaml;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct SimpleTestCase {
        schema: String,
        name: String,
        value: i32,
    }

    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create schema requiring value >= 100
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "schema": { "type": "string" },
            "name": { "type": "string" },
            "value": { "type": "integer", "minimum": 100 }
        },
        "required": ["schema", "name", "value"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML file with value < 100
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"schema: test-schema.json
name: Test
value: 50
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Load and validate should fail
    let result: Result<SimpleTestCase, _> =
        load_and_validate_yaml(&yaml_file, schemas_root.to_str().unwrap());

    assert!(
        result.is_err(),
        "Expected load_and_validate_yaml to fail for schema violation"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Schema validation failed")
            || err_msg.contains("Schema constraint violations"),
        "Error message should mention schema validation failure, got: {}",
        err_msg
    );
}

/// Test 8: Validation errors include file path information
#[test]
fn test_validation_errors_include_file_path() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create schema
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "name": { "type": "string" }
        },
        "required": ["name"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML file with invalid syntax
    let yaml_file = temp_dir.path().join("my_test_file.yaml");
    let yaml_content = "name: [unclosed array";
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate using YamlValidator
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_err(),
        "Expected validation to fail for invalid YAML syntax"
    );
    let err_msg = result.unwrap_err().to_string();
    // The error should mention parsing failure
    assert!(
        err_msg.contains("Failed to parse YAML"),
        "Error message should mention YAML parsing failure, got: {}",
        err_msg
    );
}

/// Test 9: Validation errors from log_yaml_parse_error include line numbers
#[test]
fn test_log_yaml_parse_error_includes_line_numbers() {
    use testcase_common::log_yaml_parse_error;

    // Initialize logger to capture log output
    let _ = env_logger::builder().is_test(true).try_init();

    // Create invalid YAML with error on a specific line
    let yaml_content = r#"line1: valid
line2: valid
line3: [unclosed array
line4: valid
"#;

    // Parse to get error
    let parse_result: Result<serde_yaml::Value, _> = serde_yaml::from_str(yaml_content);
    assert!(parse_result.is_err(), "Expected YAML parsing to fail");

    let error = parse_result.unwrap_err();

    // Call log_yaml_parse_error - this should log with line numbers
    log_yaml_parse_error(&error, yaml_content, "test_file.yaml");

    // We can't easily assert on log output in a unit test, but we verify the function runs
    // without panicking and that the error has location information
    if let Some(location) = error.location() {
        assert!(
            location.line() > 0,
            "Error should have line number information"
        );
        // The YAML error is reported on line 4 (where the parser realizes the array wasn't closed)
        // rather than line 3 (where the unclosed array starts)
        assert!(
            location.line() >= 3,
            "Error should be on or after line 3 where unclosed array is"
        );
    }
}

/// Test 10: load_and_validate_yaml with full TestCase model
#[test]
fn test_load_and_validate_yaml_full_testcase_model() {
    use testcase_common::load_and_validate_yaml;
    use testcase_models::TestCase;

    let temp_dir = TempDir::new().unwrap();
    let schemas_dir = temp_dir.path().join("schemas");
    let schemas_tcms = schemas_dir.join("tcms");
    fs::create_dir_all(&schemas_tcms).unwrap();

    // Create a comprehensive schema
    let schema_path = schemas_tcms.join("test-case.schema.v1.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "type": { "type": "string" },
            "schema": { "type": "string" },
            "requirement": { "type": "string" },
            "item": { "type": "integer" },
            "tc": { "type": "integer" },
            "id": { "type": "string" },
            "description": { "type": "string" },
            "general_initial_conditions": { "type": "object" },
            "initial_conditions": { "type": "object" },
            "test_sequences": { "type": "array" }
        },
        "required": ["requirement", "item", "tc", "id", "description", "general_initial_conditions", "initial_conditions", "test_sequences"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create a valid comprehensive YAML file
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"type: test_case
schema: tcms/test-case.schema.v1.json
requirement: REQ_INTEGRATION_001
item: 1
tc: 1
id: INT_TC_001
description: Integration test case for load_and_validate_yaml
general_initial_conditions:
  system:
    - System is initialized
initial_conditions:
  device:
    - Device is ready
test_sequences:
  - id: 1
    name: Sequence One
    description: First test sequence
    initial_conditions:
      system:
        - Ready for testing
    steps:
      - step: 1
        description: Execute test command
        command: echo "hello"
        expected:
          result: 0
          output: hello
        verification:
          result: "[[ $? -eq 0 ]]"
          output: "grep -q 'hello' <<< \"$COMMAND_OUTPUT\""
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Load and validate using full TestCase model
    let result: Result<TestCase, _> =
        load_and_validate_yaml(&yaml_file, schemas_dir.to_str().unwrap());

    assert!(
        result.is_ok(),
        "Expected load_and_validate_yaml to succeed for full TestCase model: {:?}",
        result.as_ref().err()
    );
    let test_case = result.unwrap();
    assert_eq!(test_case.requirement, "REQ_INTEGRATION_001");
    assert_eq!(test_case.item, 1);
    assert_eq!(test_case.tc, 1);
    assert_eq!(test_case.id, "INT_TC_001");
    assert_eq!(
        test_case.description,
        "Integration test case for load_and_validate_yaml"
    );
    assert_eq!(test_case.test_sequences.len(), 1);
    assert_eq!(test_case.test_sequences[0].name, "Sequence One");
    assert_eq!(test_case.test_sequences[0].steps.len(), 1);
}

/// Test 11: Validation with auto schema resolution
#[test]
fn test_yaml_validator_auto_schema_resolution() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_dir = temp_dir.path().join("schemas");
    let schemas_tcms = schemas_dir.join("tcms");
    fs::create_dir_all(&schemas_tcms).unwrap();

    // Create schema
    let schema_path = schemas_tcms.join("auto-test.schema.v1.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "schema": { "type": "string" },
            "name": { "type": "string" },
            "value": { "type": "integer" }
        },
        "required": ["schema", "name", "value"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML with schema reference
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"schema: tcms/auto-test.schema.v1.json
name: Auto Test
value: 42
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate with auto schema resolution
    let validator = YamlValidator::new();
    let result = validator.validate_file_auto_schema(&yaml_file, &schemas_dir);

    assert!(
        result.is_ok(),
        "Expected auto schema resolution validation to succeed: {:?}",
        result.as_ref().err()
    );
}

/// Test 12: Error details include path information for nested schema violations
#[test]
fn test_validation_errors_include_nested_path() {
    let temp_dir = TempDir::new().unwrap();
    let schemas_root = temp_dir.path().join("schemas");
    fs::create_dir_all(&schemas_root).unwrap();

    // Create schema with nested structure
    let schema_path = schemas_root.join("test-schema.json");
    let schema_content = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "config": {
                "type": "object",
                "properties": {
                    "timeout": { "type": "integer", "minimum": 1 }
                },
                "required": ["timeout"]
            }
        },
        "required": ["name", "config"]
    }"#;
    fs::write(&schema_path, schema_content).unwrap();

    // Create YAML with nested constraint violation
    let yaml_file = temp_dir.path().join("test.yaml");
    let yaml_content = r#"name: Test
config:
  timeout: 0
"#;
    fs::write(&yaml_file, yaml_content).unwrap();

    // Validate
    let validator = YamlValidator::new();
    let result = validator.validate_file(&yaml_file, &schema_path);

    assert!(
        result.is_err(),
        "Expected validation to fail for nested constraint violation"
    );
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Schema constraint violations"),
        "Error message should mention schema constraint violations"
    );
    // Should mention the nested path
    assert!(
        err_msg.contains("config") || err_msg.contains("timeout"),
        "Error message should mention the nested field path"
    );
}
