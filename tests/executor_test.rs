use std::collections::HashMap;
use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{Expected, Step, TestCase, TestSequence, Verification};

#[test]
fn test_verification_serialization() {
    let verification = Verification {
        result: "[ $? -eq 0 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"success\" ]".to_string(),
    };

    let yaml = serde_yaml::to_string(&verification).unwrap();
    assert!(yaml.contains("result:"));
    assert!(yaml.contains("output:"));
    assert!(yaml.contains("[ $? -eq 0 ]"));
    assert!(yaml.contains("[ \"$COMMAND_OUTPUT\" = \"success\" ]"));
}

#[test]
fn test_verification_deserialization() {
    let yaml = r#"
result: "[ $? -eq 0 ]"
output: "[ \"$COMMAND_OUTPUT\" = \"success\" ]"
"#;

    let verification: Verification = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(verification.result, "[ $? -eq 0 ]");
    assert_eq!(verification.output, "[ \"$COMMAND_OUTPUT\" = \"success\" ]");
}

#[test]
fn test_verification_round_trip() {
    let original = Verification {
        result: "[ $EXIT_CODE -eq 0 ]".to_string(),
        output: "[[ \"$COMMAND_OUTPUT\" =~ \"OK\" ]]".to_string(),
    };

    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: Verification = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

#[test]
fn test_shell_script_basic_structure() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Basic test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Echo test".to_string(),
        command: "echo 'hello'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"hello\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.starts_with("#!/bin/bash\n"));
    assert!(script.contains("set -e\n"));
    assert!(script.contains("# Test Case: TC001"));
    assert!(script.contains("# Description: Basic test case"));
}

#[test]
fn test_shell_script_exit_code_substitution() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test exit code".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "ls /nonexistent".to_string(),
        expected: Expected {
            success: Some(false),
            result: "[ $EXIT_CODE -ne 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $? -ne 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("EXIT_CODE=$?"));
    assert!(script.contains("[ $? -ne 0 ]"));
}

#[test]
fn test_shell_script_command_output_substitution() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test command output".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Echo test".to_string(),
        command: "echo 'test output'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"test output\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"test output\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=\"\""));
    assert!(script.contains("COMMAND_OUTPUT=$(echo 'test output')"));
    assert!(script.contains("[ \"$COMMAND_OUTPUT\" = \"test output\" ]"));
}

#[test]
fn test_shell_script_output_variable_substitution() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test OUTPUT variable".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test with OUTPUT".to_string(),
        command: "echo 'data'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"${OUTPUT}\" =~ \"data\" ]]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"${OUTPUT}\" =~ \"data\" ]]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("[[ \"${OUTPUT}\" =~ \"data\" ]]"));
}

#[test]
fn test_multi_step_script_generation() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Multi-step test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Multi-step sequence".to_string());

    let step1 = Step {
        step: 1,
        manual: None,
        description: "First step".to_string(),
        command: "echo 'step1'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"step1\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"step1\" ]".to_string(),
        },
    };

    let step2 = Step {
        step: 2,
        manual: None,
        description: "Second step".to_string(),
        command: "echo 'step2'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"step2\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"step2\" ]".to_string(),
        },
    };

    let step3 = Step {
        step: 3,
        manual: None,
        description: "Third step".to_string(),
        command: "echo 'step3'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"step3\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"step3\" ]".to_string(),
        },
    };

    sequence.steps.push(step1);
    sequence.steps.push(step2);
    sequence.steps.push(step3);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# Step 1: First step"));
    assert!(script.contains("# Step 2: Second step"));
    assert!(script.contains("# Step 3: Third step"));
    assert!(script.contains("echo 'step1'"));
    assert!(script.contains("echo 'step2'"));
    assert!(script.contains("echo 'step3'"));
}

#[test]
fn test_multi_sequence_script_generation() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Multi-sequence test".to_string(),
    );

    let mut sequence1 = TestSequence::new(1, "Seq1".to_string(), "First sequence".to_string());
    let step1 = Step {
        step: 1,
        manual: None,
        description: "Seq1 Step1".to_string(),
        command: "echo 'seq1'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence1.steps.push(step1);

    let mut sequence2 = TestSequence::new(2, "Seq2".to_string(), "Second sequence".to_string());
    let step2 = Step {
        step: 1,
        manual: None,
        description: "Seq2 Step1".to_string(),
        command: "echo 'seq2'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence2.steps.push(step2);

    test_case.test_sequences.push(sequence1);
    test_case.test_sequences.push(sequence2);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# Test Sequence 1: Seq1"));
    assert!(script.contains("# First sequence"));
    assert!(script.contains("# Test Sequence 2: Seq2"));
    assert!(script.contains("# Second sequence"));
}

#[test]
fn test_verification_expression_evaluation() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("VERIFICATION_RESULT_PASS=false"));
    assert!(script.contains("VERIFICATION_OUTPUT_PASS=false"));
    assert!(script.contains("if [ $EXIT_CODE -eq 0 ]; then"));
    assert!(script.contains("VERIFICATION_RESULT_PASS=true"));
    assert!(script.contains("if [ \"$COMMAND_OUTPUT\" = \"test\" ]; then"));
    assert!(script.contains("VERIFICATION_OUTPUT_PASS=true"));
    assert!(script.contains("if [ \"$VERIFICATION_RESULT_PASS\" = true ] && [ \"$VERIFICATION_OUTPUT_PASS\" = true ]; then"));
}

#[test]
fn test_empty_command() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Empty command test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Empty command".to_string(),
        command: "".to_string(),
        expected: Expected {
            success: Some(true),
            result: "true".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "true".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$()"));
}

#[test]
fn test_special_characters_in_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Special characters test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Special chars".to_string(),
        command: "echo 'hello \"world\"'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ \"world\" ]]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ \"world\" ]]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo 'hello \"world\"'"));
    assert!(script.contains("[[ \"$COMMAND_OUTPUT\" =~ \"world\" ]]"));
}

#[test]
fn test_command_with_quotes_escaping() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Quote escaping test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test quotes".to_string(),
        command: "echo \"test\"".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo \"test\"") || script.contains("echo \\\"test\\\""));
    let fail_section = script.split("else").nth(1).unwrap();
    assert!(fail_section.contains("echo \\\"test\\\""));
}

#[test]
fn test_complex_bash_expression_result() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Complex bash test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Complex expression".to_string(),
        command: "echo 'data'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ] && [ -n \"$COMMAND_OUTPUT\" ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ] && [ -n \"$COMMAND_OUTPUT\" ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("if [ $EXIT_CODE -eq 0 ] && [ -n \"$COMMAND_OUTPUT\" ]; then"));
}

#[test]
fn test_complex_bash_expression_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Complex output test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Complex output check".to_string(),
        command: "echo 'hello world'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ ^hello ]] && [[ \"$COMMAND_OUTPUT\" =~ world$ ]]"
                .to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ ^hello ]] && [[ \"$COMMAND_OUTPUT\" =~ world$ ]]"
                .to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains(
        "if [[ \"$COMMAND_OUTPUT\" =~ ^hello ]] && [[ \"$COMMAND_OUTPUT\" =~ world$ ]]; then"
    ));
}

#[test]
fn test_regex_pattern_matching() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Regex test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Regex match".to_string(),
        command: "echo 'Version 1.2.3'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ [0-9]+\\.[0-9]+\\.[0-9]+ ]]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ [0-9]+\\.[0-9]+\\.[0-9]+ ]]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("[[ \"$COMMAND_OUTPUT\" =~ [0-9]+\\.[0-9]+\\.[0-9]+ ]]"));
}

#[test]
fn test_manual_step_skipped() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Manual step test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: Some(true),
        description: "Manual verification".to_string(),
        command: "ssh device".to_string(),
        expected: Expected {
            success: Some(true),
            result: "connected".to_string(),
            output: "success".to_string(),
        },
        verification: Verification {
            result: "true".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# MANUAL STEP - Skipping automated execution"));
    assert!(script.contains("# Command: ssh device"));
    assert!(script.contains("# Expected result: connected"));
    assert!(script.contains("# Expected output: success"));
    assert!(!script.contains("COMMAND_OUTPUT=$(ssh device)"));
}

#[test]
fn test_initial_conditions_in_script() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Conditions test".to_string(),
    );

    let mut general_conditions = HashMap::new();
    general_conditions.insert(
        "Device".to_string(),
        vec!["Powered on".to_string(), "Connected".to_string()],
    );
    test_case.general_initial_conditions = general_conditions;

    let mut conditions = HashMap::new();
    conditions.insert("Network".to_string(), vec!["Online".to_string()]);
    test_case.initial_conditions = conditions;

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut seq_conditions = HashMap::new();
    seq_conditions.insert("Session".to_string(), vec!["Active".to_string()]);
    sequence.initial_conditions = seq_conditions;

    let step = Step {
        step: 1,
        manual: None,
        description: "Test".to_string(),
        command: "echo 'test'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "true".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "true".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Device: Powered on, Connected"));
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Network: Online"));
    assert!(script.contains("# Sequence Initial Conditions"));
    assert!(script.contains("# Session: Active"));
}

#[test]
fn test_script_success_message() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Success message test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test".to_string(),
        command: "true".to_string(),
        expected: Expected {
            success: Some(true),
            result: "true".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "true".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo \"All test sequences completed successfully\""));
    assert!(script.contains("exit 0"));
}

#[test]
fn test_script_failure_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Failure test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo \"[PASS] Step 1: Test step\""));
    assert!(script.contains("echo \"[FAIL] Step 1: Test step\""));
    assert!(script.contains("echo \"  Command: echo 'test'\""));
    assert!(script.contains("echo \"  Exit code: $EXIT_CODE\""));
    assert!(script.contains("echo \"  Output: $COMMAND_OUTPUT\""));
    assert!(script.contains("echo \"  Result verification: $VERIFICATION_RESULT_PASS\""));
    assert!(script.contains("echo \"  Output verification: $VERIFICATION_OUTPUT_PASS\""));
    assert!(script.contains("exit 1"));
}

#[test]
fn test_command_with_pipes() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Pipe test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Pipe command".to_string(),
        command: "echo 'hello world' | grep world".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ world ]]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ world ]]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$(echo 'hello world' | grep world)"));
}

#[test]
fn test_command_with_redirects() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Redirect test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Redirect command".to_string(),
        command: "cat /dev/null 2>&1".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$(cat /dev/null 2>&1)"));
}

#[test]
fn test_command_with_environment_variables() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Env var test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Environment variable".to_string(),
        command: "MY_VAR=test echo $MY_VAR".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$(MY_VAR=test echo $MY_VAR)"));
}

#[test]
fn test_verification_with_numeric_comparison() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Numeric test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Numeric comparison".to_string(),
        command: "echo 42".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" -eq 42 ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" -eq 42 ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("if [ \"$COMMAND_OUTPUT\" -eq 42 ]; then"));
}

#[test]
fn test_verification_with_string_length_check() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "String length test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "String length check".to_string(),
        command: "echo 'test'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ -n \"$COMMAND_OUTPUT\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ -n \"$COMMAND_OUTPUT\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("if [ -n \"$COMMAND_OUTPUT\" ]; then"));
}

#[test]
fn test_verification_with_file_test() {
    use tempfile::NamedTempFile;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "File test".to_string(),
    );

    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_string_lossy().to_string();
    let command = format!("touch {}", temp_path);
    let result_check = format!("[ $EXIT_CODE -eq 0 ] && [ -f {} ]", temp_path);

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "File existence check".to_string(),
        command: command.clone(),
        expected: Expected {
            success: Some(true),
            result: result_check.clone(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: result_check.clone(),
            output: "true".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    println!("{}", script);
    println!("{}", result_check);

    assert!(script.contains(&format!("if {}; then", result_check)));
}

#[test]
fn test_newline_in_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Newline test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Multi-line output".to_string(),
        command: "echo -e 'line1\\nline2'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ line1 ]] && [[ \"$COMMAND_OUTPUT\" =~ line2 ]]"
                .to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[[ \"$COMMAND_OUTPUT\" =~ line1 ]] && [[ \"$COMMAND_OUTPUT\" =~ line2 ]]"
                .to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo -e 'line1\\nline2'"));
}

#[test]
fn test_verification_comments_in_script() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Verification comments test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test".to_string(),
        command: "echo 'test'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# Verification result expression: [ $EXIT_CODE -eq 0 ]"));
    assert!(script.contains("# Verification output expression: [ \"$COMMAND_OUTPUT\" = \"test\" ]"));
}

#[test]
fn test_edge_case_backticks_in_command() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Backticks test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Backticks command".to_string(),
        command: "echo `date`".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ -n \"$COMMAND_OUTPUT\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ -n \"$COMMAND_OUTPUT\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$(echo `date`)"));
}

#[test]
fn test_edge_case_dollar_sign_in_command() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Dollar sign test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Dollar sign command".to_string(),
        command: "echo $(whoami)".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ -n \"$COMMAND_OUTPUT\" ]".to_string(),
        },
        verification: Verification {
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "[ -n \"$COMMAND_OUTPUT\" ]".to_string(),
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$(echo $(whoami))"));
}

#[test]
fn test_verification_equals_operator() {
    let verification = Verification {
        result: "[ $? -eq 0 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
    };

    let verification2 = Verification {
        result: "[ $? -eq 0 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
    };

    assert_eq!(verification, verification2);
}

#[test]
fn test_verification_not_equals_operator() {
    let verification1 = Verification {
        result: "[ $? -eq 0 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
    };

    let verification2 = Verification {
        result: "[ $? -eq 1 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
    };

    assert_ne!(verification1, verification2);
}

#[test]
fn test_verification_display_trait() {
    let verification = Verification {
        result: "[ $? -eq 0 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
    };

    let display_string = format!("{}", verification);
    assert!(display_string.contains("result: [ $? -eq 0 ]"));
    assert!(display_string.contains("output: [ \"$COMMAND_OUTPUT\" = \"test\" ]"));
}

#[test]
fn test_verification_clone() {
    let verification = Verification {
        result: "[ $? -eq 0 ]".to_string(),
        output: "[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string(),
    };

    let cloned = verification.clone();
    assert_eq!(verification, cloned);
}
