use testcase_manager::validation::SchemaValidator;

#[test]
fn test_validator_accepts_valid_minimal_complete_document() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
requirement: REQ-001
item: 1
tc: 1
id: TC_001
description: Test description
general_initial_conditions:
  eUICC:
    - "Condition 1"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
test_sequences:
  - id: 1
    name: "Sequence 1"
    description: "Description"
    initial_conditions:
      eUICC:
        - "Seq condition"
    steps:
      - step: 1
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
  - id: 2
    name: "Sequence 2"
    description: "Description"
    initial_conditions:
      eUICC:
        - "Seq condition"
    steps:
      - step: 1
        description: "Step 1"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Step 2"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept valid minimal document: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_extra_test_sequences() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq 1"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
  - id: 2
    name: "Seq 2"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
  - id: 3
    name: "Seq 3"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
  - id: 4
    name: "Seq 4"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept 4 test sequences (tuple allows extras): {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_extra_steps() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
      - step: 3
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
      - step: 4
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
      - step: 5
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept 5 steps (tuple allows extras): {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_extra_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
    - "Condition 3"
    - "Condition 4"
    - "Condition 5"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept 5 conditions (tuple allows extras): {:?}",
        result.err()
    );
}

#[test]
fn test_validator_rejects_wrong_type_item() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
item: "should be integer"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject string for integer field");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Invalid type") || error.contains("integer"),
        "Error: {}",
        error
    );
}

#[test]
fn test_validator_rejects_wrong_type_step() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      - eUICC: ["Cond"]
    steps:
      - step: "should be integer"
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject string for step number");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Invalid type") || error.contains("integer"),
        "Error: {}",
        error
    );
}

#[test]
fn test_validator_rejects_missing_required_command() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      - eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step without command"
        expected:
          success: true
          result: "OK"
          output: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject step missing command");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("command") || error.contains("required"),
        "Error: {}",
        error
    );
}

#[test]
fn test_validator_rejects_missing_expected_output() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      - eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
        expected:
          success: true
          result: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject expected missing output");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("output") || error.contains("required"),
        "Error: {}",
        error
    );
}

#[test]
fn test_validator_accepts_metadata_only() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
requirement: REQ-001
item: 1
tc: 1
id: TC_001
description: Test description
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept metadata-only chunk: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_conditions_only() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
general_initial_conditions:
  eUICC:
    - "Condition 1"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept conditions-only chunk: {:?}",
        result.err()
    );
}

#[test]
#[ignore = "??"]
fn test_validator_rejects_non_array_euicc() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC: "should be array"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject non-array eUICC");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Invalid type") || error.contains("array"),
        "Error: {}",
        error
    );
}

#[test]
#[ignore = "??"]
fn test_validator_rejects_non_string_in_euicc_array() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Valid string"
    - 123
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject non-string in eUICC array");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("Invalid type") || error.contains("string"),
        "Error: {}",
        error
    );
}

#[test]
fn test_validator_accepts_empty_arrays() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC: []
test_sequences: []
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept empty arrays: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_manual_optional() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "Step without manual field"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Step"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept steps without manual field: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_success_optional_in_second_step() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Seq"
    description: "Desc"
    initial_conditions:
      eUICC: ["Cond"]
    steps:
      - step: 1
        description: "First step requires success"
        command: "cmd"
        expected:
          success: true
          result: "OK"
          output: "OK"
      - step: 2
        description: "Second step doesn't require success"
        command: "cmd"
        expected:
          result: "OK"
          output: "OK"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept second step without success field: {:?}",
        result.err()
    );
}
