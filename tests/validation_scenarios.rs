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
        error.contains("Invalid type") || error.contains("string") || error.contains("oneOf"),
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

#[test]
fn test_validator_accepts_ref_field_on_test_sequence() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    ref: "sequence_reference_1"
    name: "Sequence with ref"
    description: "Test sequence with reference field"
    initial_conditions:
      eUICC: ["Condition"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
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
        "Should accept ref field on test_sequence: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_ref_field_on_step() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Sequence"
    description: "Test sequence"
    initial_conditions:
      eUICC: ["Condition"]
    steps:
      - step: 1
        ref: "step_reference_1"
        description: "Step with reference"
        command: "cmd"
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
        "Should accept ref field on step: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_arrays_in_general_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
general_initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
    - "Condition 3"
  LPA:
    - "LPA Condition 1"
    - "LPA Condition 2"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept arrays in general_initial_conditions: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_arrays_in_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Condition 1"
    - "Condition 2"
    - "Condition 3"
  SM_DP_PLUS:
    - "SM-DP+ Condition 1"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept arrays in initial_conditions: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_arrays_in_test_sequence_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Sequence"
    description: "Test sequence"
    initial_conditions:
      eUICC:
        - "Condition 1"
        - "Condition 2"
      LPA:
        - "LPA Condition"
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
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
        "Should accept arrays in test_sequence.initial_conditions: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_mixed_initial_condition_items_with_strings_and_ref() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Plain string condition"
    - ref: "reference_to_other_condition"
    - "Another string condition"
    - ref: "another_reference"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept mixed string and ref object items: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_mixed_initial_condition_items_with_test_sequence() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Plain string condition"
    - test_sequence:
        id: 1
        step: "2"
    - "Another string condition"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept mixed string and test_sequence object items: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_mixed_items_in_general_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
general_initial_conditions:
  eUICC:
    - "String condition"
    - ref: "some_reference"
    - test_sequence:
        id: 2
        step: "1"
    - "Another string"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept mixed items in general_initial_conditions: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_mixed_items_in_test_sequence_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Sequence"
    description: "Test"
    initial_conditions:
      eUICC:
        - "String condition"
        - ref: "reference_1"
        - test_sequence:
            id: 3
            step: "5"
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
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
        "Should accept mixed items in test_sequence.initial_conditions: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_rejects_include_without_id_in_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  include:
    - test_sequence: "1"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_err(),
        "Should reject include item without id field"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("id") || error.contains("required"),
        "Error should mention missing id: {}",
        error
    );
}

#[test]
fn test_validator_rejects_include_without_id_in_general_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
general_initial_conditions:
  include:
    - test_sequence: "2"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_err(),
        "Should reject include item without id field"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("id") || error.contains("required"),
        "Error should mention missing id: {}",
        error
    );
}

#[test]
fn test_validator_rejects_include_without_id_in_test_sequence_initial_conditions() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
test_sequences:
  - id: 1
    name: "Sequence"
    description: "Test"
    initial_conditions:
      include:
        - test_sequence: "3"
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
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
        result.is_err(),
        "Should reject include item without id field"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("id") || error.contains("required"),
        "Error should mention missing id: {}",
        error
    );
}

#[test]
fn test_validator_rejects_test_sequence_ref_without_id() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "String condition"
    - test_sequence:
        step: "1"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_err(),
        "Should reject test_sequence without id field"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("id") || error.contains("required") || error.contains("oneOf"),
        "Error should mention missing id or oneOf validation failure: {}",
        error
    );
}

#[test]
fn test_validator_rejects_test_sequence_ref_without_step() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "String condition"
    - test_sequence:
        id: 1
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_err(),
        "Should reject test_sequence without step field"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("step") || error.contains("required") || error.contains("oneOf"),
        "Error should mention missing step or oneOf validation failure: {}",
        error
    );
}

#[test]
fn test_validator_rejects_test_sequence_ref_without_both_fields() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
general_initial_conditions:
  eUICC:
    - "String condition"
    - test_sequence: {}
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_err(),
        "Should reject test_sequence without required fields"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("required")
            || error.contains("oneOf")
            || error.contains("id")
            || error.contains("step"),
        "Error should mention missing required fields: {}",
        error
    );
}

#[test]
fn test_validator_rejects_non_string_non_object_array_item_number() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Valid string"
    - 42
    - "Another string"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject plain number in array");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("type") || error.contains("oneOf") || error.contains("Invalid"),
        "Error should mention type mismatch or oneOf failure: {}",
        error
    );
}

#[test]
fn test_validator_rejects_non_string_non_object_array_item_boolean() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
general_initial_conditions:
  LPA:
    - "Valid string"
    - true
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject boolean in array");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("type") || error.contains("oneOf") || error.contains("Invalid"),
        "Error should mention type mismatch or oneOf failure: {}",
        error
    );
}

#[test]
fn test_validator_rejects_non_string_non_object_array_item_null() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Valid string"
    - null
"#;

    let result = validator.validate_chunk(yaml);
    assert!(result.is_err(), "Should reject null in array");
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("type") || error.contains("oneOf") || error.contains("Invalid"),
        "Error should mention type mismatch or oneOf failure: {}",
        error
    );
}

#[test]
fn test_validator_rejects_invalid_object_structure_in_array() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  eUICC:
    - "Valid string"
    - invalid_field: "value"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_err(),
        "Should reject object without ref or test_sequence fields"
    );
    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("oneOf") || error.contains("required"),
        "Error should mention oneOf validation failure or missing required field: {}",
        error
    );
}

#[test]
fn test_validator_accepts_valid_include_with_id() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
initial_conditions:
  include:
    - id: "TC_001"
    - id: "TC_002"
      test_sequence: "1"
"#;

    let result = validator.validate_chunk(yaml);
    assert!(
        result.is_ok(),
        "Should accept valid include items with id: {:?}",
        result.err()
    );
}

#[test]
fn test_validator_accepts_complete_mixed_structure() {
    let validator = SchemaValidator::new().unwrap();

    let yaml = r#"
requirement: REQ-001
item: 1
tc: 1
id: TC_001
description: Complete test with dependencies
general_initial_conditions:
  include:
    - id: "TC_BASE"
      test_sequence: "1"
  eUICC:
    - "Base condition"
    - ref: "common_setup"
    - test_sequence:
        id: 5
        step: "3"
initial_conditions:
  include:
    - id: "TC_PREREQ"
  eUICC:
    - "Initial string condition"
    - ref: "ref_condition_1"
  LPA:
    - test_sequence:
        id: 2
        step: "1"
    - "LPA condition"
test_sequences:
  - id: 1
    ref: "main_sequence"
    name: "Main Sequence"
    description: "Main test sequence"
    initial_conditions:
      include:
        - id: "TC_SEQ_BASE"
      eUICC:
        - "Sequence condition"
        - ref: "seq_ref"
        - test_sequence:
            id: 10
            step: "2"
    steps:
      - step: 1
        ref: "step_1_ref"
        description: "First step"
        command: "cmd1"
        expected:
          success: true
          result: "OK"
          output: "Success"
      - step: 2
        description: "Second step"
        command: "cmd2"
        expected:
          result: "OK"
          output: "Success"
  - id: 2
    name: "Second Sequence"
    description: "Second test sequence"
    initial_conditions:
      eUICC: ["Condition"]
    steps:
      - step: 1
        description: "Step"
        command: "cmd"
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
        "Should accept complete structure with all dependency features: {:?}",
        result.err()
    );
}
