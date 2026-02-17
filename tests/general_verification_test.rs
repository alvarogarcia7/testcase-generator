use std::collections::BTreeMap;
use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{
    Expected, GeneralVerification, Step, TestCase, TestSequence, Verification,
    VerificationExpression,
};

// ============================================================================
// General Verification Array Parsing Tests
// ============================================================================

#[test]
fn test_general_verification_parsing_single() {
    let yaml = r#"
result: "[[ $? -eq 0 ]]"
output: "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\""
general:
  - name: check_file_exists
    condition: "test -f /tmp/output.txt"
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    assert!(verification.general.is_some());
    let general = verification.general.unwrap();
    assert_eq!(general.len(), 1);
    assert_eq!(general[0].name, "check_file_exists");
    assert_eq!(general[0].condition, "test -f /tmp/output.txt");
}

#[test]
fn test_general_verification_parsing_multiple() {
    let yaml = r#"
result: "[[ $? -eq 0 ]]"
output: "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\""
general:
  - name: check_file_exists
    condition: "test -f /tmp/output.txt"
  - name: check_permissions
    condition: "test -r /tmp/output.txt"
  - name: check_size
    condition: "[[ $(stat -f%z /tmp/output.txt 2>/dev/null || stat -c%s /tmp/output.txt) -gt 0 ]]"
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    assert!(verification.general.is_some());
    let general = verification.general.unwrap();
    assert_eq!(general.len(), 3);
    assert_eq!(general[0].name, "check_file_exists");
    assert_eq!(general[1].name, "check_permissions");
    assert_eq!(general[2].name, "check_size");
}

#[test]
fn test_general_verification_parsing_empty_array() {
    let yaml = r#"
result: "[[ $? -eq 0 ]]"
output: "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\""
general: []
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    assert!(verification.general.is_some());
    let general = verification.general.unwrap();
    assert_eq!(general.len(), 0);
}

#[test]
fn test_general_verification_parsing_none() {
    let yaml = r#"
result: "[[ $? -eq 0 ]]"
output: "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\""
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    assert!(verification.general.is_none());
}

#[test]
fn test_general_verification_parsing_with_complex_conditions() {
    let yaml = r#"
result: "[[ $? -eq 0 ]]"
output: "true"
general:
  - name: validate_json
    condition: "jq empty /tmp/output.json 2>/dev/null"
  - name: check_exit_code_range
    condition: "[[ $EXIT_CODE -ge 0 && $EXIT_CODE -le 255 ]]"
  - name: verify_pattern
    condition: "echo \"$COMMAND_OUTPUT\" | grep -qE '^[A-Z]{3}-[0-9]{4}$'"
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    let general = verification.general.unwrap();
    assert_eq!(general.len(), 3);
    assert_eq!(general[0].name, "validate_json");
    assert!(general[0].condition.contains("jq empty"));
    assert_eq!(general[1].name, "check_exit_code_range");
    assert!(general[1].condition.contains("EXIT_CODE"));
    assert_eq!(general[2].name, "verify_pattern");
    assert!(general[2].condition.contains("grep -qE"));
}

#[test]
fn test_general_verification_with_special_characters_in_name() {
    let yaml = r#"
result: "true"
output: "true"
general:
  - name: "check-file-with-dashes"
    condition: "test -f /tmp/test.txt"
  - name: "check file with spaces"
    condition: "test -d /tmp"
  - name: "check_underscore"
    condition: "true"
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    let general = verification.general.unwrap();
    assert_eq!(general.len(), 3);
    assert_eq!(general[0].name, "check-file-with-dashes");
    assert_eq!(general[1].name, "check file with spaces");
    assert_eq!(general[2].name, "check_underscore");
}

#[test]
fn test_general_verification_serialization_roundtrip() {
    let verification = Verification {
        result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: Some(vec![
            GeneralVerification {
                name: "test1".to_string(),
                condition: "test -f /tmp/file".to_string(),
            },
            GeneralVerification {
                name: "test2".to_string(),
                condition: "[ $EXIT_CODE -eq 0 ]".to_string(),
            },
        ]),
    };

    let yaml = serde_yaml::to_string(&verification).unwrap();
    let deserialized: Verification = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(verification, deserialized);
}

// ============================================================================
// Bash Script Generation for Multiple General Conditions
// ============================================================================

#[test]
fn test_bash_generation_single_general_condition() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test with single general condition".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![GeneralVerification {
                name: "check_file".to_string(),
                condition: "test -f /tmp/output.txt".to_string(),
            }]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# General verifications"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_file=false"));
    assert!(script.contains("if test -f /tmp/output.txt; then"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_file=true"));
    assert!(script.contains("\"$GENERAL_VERIFY_PASS_check_file\" = true"));
}

#[test]
fn test_bash_generation_multiple_general_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC002".to_string(),
        "Test with multiple general conditions".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test' > /tmp/test.txt".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "check_file_exists".to_string(),
                    condition: "test -f /tmp/test.txt".to_string(),
                },
                GeneralVerification {
                    name: "check_file_readable".to_string(),
                    condition: "test -r /tmp/test.txt".to_string(),
                },
                GeneralVerification {
                    name: "check_content_size".to_string(),
                    condition: "[[ $(wc -c < /tmp/test.txt) -gt 0 ]]".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify all three general verification variables are initialized
    assert!(script.contains("GENERAL_VERIFY_PASS_check_file_exists=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_file_readable=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_content_size=false"));

    // Verify all conditions are present
    assert!(script.contains("if test -f /tmp/test.txt; then"));
    assert!(script.contains("if test -r /tmp/test.txt; then"));
    assert!(script.contains("if [[ $(wc -c < /tmp/test.txt) -gt 0 ]]; then"));

    // Verify variables are set to true
    assert!(script.contains("GENERAL_VERIFY_PASS_check_file_exists=true"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_file_readable=true"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_content_size=true"));

    // Verify all are included in the overall verification condition
    assert!(script.contains("\"$GENERAL_VERIFY_PASS_check_file_exists\" = true"));
    assert!(script.contains("\"$GENERAL_VERIFY_PASS_check_file_readable\" = true"));
    assert!(script.contains("\"$GENERAL_VERIFY_PASS_check_content_size\" = true"));
}

#[test]
fn test_bash_generation_sanitizes_variable_names() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC003".to_string(),
        "Test name sanitization".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("true".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "check-with-dashes".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "check with spaces".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "check!@#$%special".to_string(),
                    condition: "true".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Names should be sanitized (spaces and dashes -> underscores, special chars removed)
    assert!(script.contains("GENERAL_VERIFY_PASS_check_with_dashes=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_with_spaces=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_checkspecial=false"));

    // Should not contain invalid bash variable characters
    assert!(!script.contains("GENERAL_VERIFY_PASS_check-with-dashes"));
    assert!(!script.contains("GENERAL_VERIFY_PASS_check with spaces"));
    assert!(!script.contains("GENERAL_VERIFY_PASS_check!@#$%special"));
}

#[test]
fn test_bash_generation_general_conditions_in_error_message() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC004".to_string(),
        "Test error message".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "verify_alpha".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "verify_beta".to_string(),
                    condition: "false".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify error message includes general verification results
    assert!(script.contains(
        "echo \"  GENERAL_VERIFY_PASS_verify_alpha: $GENERAL_VERIFY_PASS_verify_alpha\""
    ));
    assert!(script
        .contains("echo \"  GENERAL_VERIFY_PASS_verify_beta: $GENERAL_VERIFY_PASS_verify_beta\""));
}

#[test]
fn test_bash_generation_no_general_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC005".to_string(),
        "Test without general conditions".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Should not contain general verification section
    assert!(!script.contains("# General verifications"));
    assert!(!script.contains("GENERAL_VERIFY_PASS_"));

    // Verification condition should only check result and output
    let verification_check_count = script
        .matches(
            "\"$VERIFICATION_RESULT_PASS\" = true ] && [ \"$VERIFICATION_OUTPUT_PASS\" = true ]",
        )
        .count();
    assert!(verification_check_count > 0);
}

#[test]
fn test_bash_generation_with_variables_and_general_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC006".to_string(),
        "Test with variables and general conditions".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());

    // Add sequence variables
    let mut variables = BTreeMap::new();
    variables.insert("test_var".to_string(), "test_value".to_string());
    sequence.variables = Some(variables);

    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo ${test_var}".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test_value".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "[[ \"$COMMAND_OUTPUT\" = \"${test_var}\" ]]".to_string(),
            ),
            output_file: None,
            general: Some(vec![GeneralVerification {
                name: "check_variable".to_string(),
                condition: "[[ \"${test_var}\" = \"test_value\" ]]".to_string(),
            }]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Should have variable initialization
    assert!(script.contains("STEP_VAR_test_var="));

    // Should have general verification with variable substitution support
    assert!(script.contains("# General verifications"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_variable=false"));

    // Should include variable substitution code for general verification
    assert!(script.contains("EXPR="));
    assert!(script.contains("STEP_VAR_NAMES"));
}

// ============================================================================
// General Verification Execution with Passing/Failing Conditions
// ============================================================================

#[test]
fn test_general_verification_execution_all_pass() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC007".to_string(),
        "Test all general verifications pass".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "[[ \"$COMMAND_OUTPUT\" = \"test\" ]]".to_string(),
            ),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "always_true_1".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "always_true_2".to_string(),
                    condition: "[[ 1 -eq 1 ]]".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Both general verifications should be present
    assert!(script.contains("GENERAL_VERIFY_PASS_always_true_1=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_always_true_2=false"));
    assert!(script.contains("if true; then"));
    assert!(script.contains("if [[ 1 -eq 1 ]]; then"));
}

#[test]
fn test_general_verification_execution_with_failing_condition() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC008".to_string(),
        "Test with failing general verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "[[ \"$COMMAND_OUTPUT\" = \"test\" ]]".to_string(),
            ),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "will_pass".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "will_fail".to_string(),
                    condition: "false".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify both conditions are in the script
    assert!(script.contains("if true; then"));
    assert!(script.contains("GENERAL_VERIFY_PASS_will_pass=true"));
    assert!(script.contains("if false; then"));
    assert!(script.contains("GENERAL_VERIFY_PASS_will_fail=true"));

    // Verify the combined condition requires both to be true
    assert!(script.contains("\"$GENERAL_VERIFY_PASS_will_pass\" = true"));
    assert!(script.contains("\"$GENERAL_VERIFY_PASS_will_fail\" = true"));
}

#[test]
fn test_general_verification_complex_bash_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC009".to_string(),
        "Test complex bash conditions".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("true".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "check_string_length".to_string(),
                    condition: "[[ ${#COMMAND_OUTPUT} -gt 0 ]]".to_string(),
                },
                GeneralVerification {
                    name: "check_numeric_comparison".to_string(),
                    condition: "[[ $EXIT_CODE -ge 0 && $EXIT_CODE -le 255 ]]".to_string(),
                },
                GeneralVerification {
                    name: "check_pattern_match".to_string(),
                    condition: "[[ \"$COMMAND_OUTPUT\" =~ ^[a-z]+$ ]]".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify complex conditions are preserved
    assert!(script.contains("[[ ${#COMMAND_OUTPUT} -gt 0 ]]"));
    assert!(script.contains("[[ $EXIT_CODE -ge 0 && $EXIT_CODE -le 255 ]]"));
    assert!(script.contains("[[ \"$COMMAND_OUTPUT\" =~ ^[a-z]+$ ]]"));
}

// ============================================================================
// Interaction with Result/Output Verification
// ============================================================================

#[test]
fn test_general_verification_combined_with_result_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC010".to_string(),
        "Test combined verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'success'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "success".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "[[ \"$COMMAND_OUTPUT\" = \"success\" ]]".to_string(),
            ),
            output_file: None,
            general: Some(vec![GeneralVerification {
                name: "additional_check".to_string(),
                condition: "[[ \"$COMMAND_OUTPUT\" =~ ^success$ ]]".to_string(),
            }]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify result verification is present
    assert!(script.contains("# Verification result"));
    assert!(script.contains("VERIFICATION_RESULT_PASS=false"));
    assert!(script.contains("[[ $? -eq 0 ]]"));

    // Verify output verification is present
    assert!(script.contains("# Verification output"));
    assert!(script.contains("VERIFICATION_OUTPUT_PASS=false"));

    // Verify general verification is present
    assert!(script.contains("# General verifications"));
    assert!(script.contains("GENERAL_VERIFY_PASS_additional_check=false"));

    // Verify all three are checked together in the final condition
    assert!(script.contains("\"$VERIFICATION_RESULT_PASS\" = true ] && [ \"$VERIFICATION_OUTPUT_PASS\" = true ] && [ \"$GENERAL_VERIFY_PASS_additional_check\" = true"));
}

#[test]
fn test_general_verification_with_output_file() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC011".to_string(),
        "Test with output_file".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test' > /tmp/test.log".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: Some(VerificationExpression::Simple(
                "cat $LOG_FILE | grep -q 'test'".to_string(),
            )),
            general: Some(vec![GeneralVerification {
                name: "check_log_file".to_string(),
                condition: "test -f $LOG_FILE".to_string(),
            }]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Should use output_file verification instead of output
    assert!(script.contains("cat $LOG_FILE | grep -q 'test'"));

    // General verification should still be present
    assert!(script.contains("GENERAL_VERIFY_PASS_check_log_file=false"));
    assert!(script.contains("test -f $LOG_FILE"));
}

#[test]
fn test_general_verification_execution_order() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC012".to_string(),
        "Test execution order".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "first_check".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "second_check".to_string(),
                    condition: "true".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Find positions of key sections in script
    let result_pos = script.find("# Verification result").unwrap();
    let output_pos = script.find("# Verification output").unwrap();
    let general_pos = script.find("# General verifications").unwrap();

    // Verify order: result -> output -> general
    assert!(result_pos < output_pos);
    assert!(output_pos < general_pos);
}

// ============================================================================
// Error Messages When General Conditions Fail
// ============================================================================

#[test]
fn test_error_message_includes_general_verification_status() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC013".to_string(),
        "Test error messages".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![
                GeneralVerification {
                    name: "verify_alpha".to_string(),
                    condition: "true".to_string(),
                },
                GeneralVerification {
                    name: "verify_beta".to_string(),
                    condition: "false".to_string(),
                },
                GeneralVerification {
                    name: "verify_gamma".to_string(),
                    condition: "[[ -f /nonexistent/file ]]".to_string(),
                },
            ]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify failure message includes all general verification variables
    assert!(script.contains("[FAIL] Step 1: Test step"));
    assert!(script.contains(
        "echo \"  GENERAL_VERIFY_PASS_verify_alpha: $GENERAL_VERIFY_PASS_verify_alpha\""
    ));
    assert!(script
        .contains("echo \"  GENERAL_VERIFY_PASS_verify_beta: $GENERAL_VERIFY_PASS_verify_beta\""));
    assert!(script.contains(
        "echo \"  GENERAL_VERIFY_PASS_verify_gamma: $GENERAL_VERIFY_PASS_verify_gamma\""
    ));
}

#[test]
fn test_error_message_format_with_all_verifications() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC014".to_string(),
        "Test complete error format".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Complete verification test".to_string(),
        command: "echo 'output'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "expected".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "[[ \"$COMMAND_OUTPUT\" = \"expected\" ]]".to_string(),
            ),
            output_file: None,
            general: Some(vec![GeneralVerification {
                name: "extra_check".to_string(),
                condition: "[[ \"$COMMAND_OUTPUT\" = \"expected\" ]]".to_string(),
            }]),
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify error section contains all necessary information
    let fail_section = script.split("[FAIL] Step 1:").nth(1).unwrap();
    assert!(fail_section.contains("Command:"));
    assert!(fail_section.contains("Exit code: $EXIT_CODE"));
    assert!(fail_section.contains("Output: $COMMAND_OUTPUT"));
    assert!(fail_section.contains("Result verification: $VERIFICATION_RESULT_PASS"));
    assert!(fail_section.contains("Output verification: $VERIFICATION_OUTPUT_PASS"));
    assert!(
        fail_section.contains("GENERAL_VERIFY_PASS_extra_check: $GENERAL_VERIFY_PASS_extra_check")
    );
}

#[test]
fn test_no_error_message_for_general_when_not_present() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC015".to_string(),
        "Test without general verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "wrong".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "[[ \"$COMMAND_OUTPUT\" = \"wrong\" ]]".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Error message should not mention general verifications
    assert!(!script.contains("GENERAL_VERIFY_PASS_"));
    assert!(!script.contains("# General verifications"));
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_empty_general_verification_array() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC016".to_string(),
        "Test empty array".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("true".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: Some(vec![]), // Empty array
        },
        reference: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Should not generate general verification code for empty array
    assert!(!script.contains("# General verifications"));
    assert!(!script.contains("GENERAL_VERIFY_PASS_"));
}

#[test]
fn test_general_verification_with_multiline_condition() {
    let yaml = r#"
result: "true"
output: "true"
general:
  - name: complex_check
    condition: |
      test -f /tmp/file &&
      test -r /tmp/file &&
      [[ $(wc -l < /tmp/file) -gt 0 ]]
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    let general = verification.general.unwrap();
    assert_eq!(general.len(), 1);
    assert_eq!(general[0].name, "complex_check");
    assert!(general[0].condition.contains("test -f /tmp/file"));
    assert!(general[0].condition.contains("test -r /tmp/file"));
    assert!(general[0].condition.contains("wc -l"));
}

#[test]
fn test_general_verification_with_quotes_and_special_chars() {
    let yaml = r#"
result: "true"
output: "true"
general:
  - name: quote_test
    condition: "[[ \"$VAR\" = \"test's \\\"quoted\\\" value\" ]]"
  - name: dollar_test
    condition: "[[ \"$VAR\" =~ \\$[0-9]+ ]]"
"#;
    let verification: Verification = serde_yaml::from_str(yaml).unwrap();

    let general = verification.general.unwrap();
    assert_eq!(general.len(), 2);
    assert!(general[0].condition.contains("test's"));
    assert!(general[1].condition.contains("$[0-9]+"));
}

#[test]
fn test_general_verification_sorted_behavior() {
    let gen1 = GeneralVerification {
        name: "zebra".to_string(),
        condition: "true".to_string(),
    };
    let gen2 = GeneralVerification {
        name: "alpha".to_string(),
        condition: "true".to_string(),
    };

    // GeneralVerification implements Ord, so we can sort
    let mut verifications = [gen1.clone(), gen2.clone()];
    verifications.sort();

    assert_eq!(verifications[0].name, "alpha");
    assert_eq!(verifications[1].name, "zebra");
}

#[test]
fn test_multiple_steps_each_with_general_verification() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC017".to_string(),
        "Test multiple steps".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());

    for i in 1..=3 {
        let step = Step {
            step: i,
            manual: None,
            description: format!("Step {}", i),
            command: "echo 'test'".to_string(),
            capture_vars: None,
            expected: Expected {
                success: Some(true),
                result: "0".to_string(),
                output: "test".to_string(),
            },
            verification: Verification {
                result: VerificationExpression::Simple("true".to_string()),
                output: VerificationExpression::Simple("true".to_string()),
                output_file: None,
                general: Some(vec![GeneralVerification {
                    name: format!("check_step_{}", i),
                    condition: "true".to_string(),
                }]),
            },
            reference: None,
        };
        sequence.steps.push(step);
    }

    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Each step should have its own general verification
    assert!(script.contains("GENERAL_VERIFY_PASS_check_step_1=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_step_2=false"));
    assert!(script.contains("GENERAL_VERIFY_PASS_check_step_3=false"));

    // Should have 3 instances of general verification section
    let count = script.matches("# General verifications").count();
    assert_eq!(count, 3);
}

#[test]
fn test_general_verification_display_format() {
    let gen = GeneralVerification {
        name: "test_verification".to_string(),
        condition: "test -f /tmp/file".to_string(),
    };

    let display = format!("{}", gen);
    assert_eq!(display, "test_verification: test -f /tmp/file");
}
