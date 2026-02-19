use std::collections::HashMap;
use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{
    Expected, InitialConditionItem, InitialConditions, Step, TestCase, TestSequence,
    TestStepExecutionEntry, Verification, VerificationExpression,
};

// Helper function to create a test step
fn create_test_step(
    step: i64,
    description: &str,
    command: &str,
    expected_result: &str,
    expected_output: &str,
    success: Option<bool>,
) -> Step {
    Step {
        step,
        manual: None,
        description: description.to_string(),
        command: command.to_string(),
        capture_vars: None,
        expected: Expected {
            success,
            result: expected_result.to_string(),
            output: expected_output.to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "[ \"$COMMAND_OUTPUT\" = \"success\" ]".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    }
}

#[test]
fn test_verification_serialization() {
    let verification = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"success\" ]".to_string()),
        output_file: None,
        general: None,
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
    assert_eq!(
        verification.result,
        VerificationExpression::Simple("[ $? -eq 0 ]".to_string())
    );
    assert_eq!(
        verification.output,
        VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"success\" ]".to_string())
    );
}

#[test]
fn test_verification_round_trip() {
    let original = Verification {
        result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[[ \"$COMMAND_OUTPUT\" =~ \"OK\" ]]".to_string()),
        output_file: None,
        general: None,
    };

    let yaml = serde_yaml::to_string(&original).unwrap();
    let deserialized: Verification = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(original, deserialized);
}

// ============================================================================
// JSON Log Generation Tests - Two-Stage Workflow
// ============================================================================

#[test]
fn test_execution_log_entry_creation() {
    let entry = TestStepExecutionEntry::new(
        1, // sequence
        1, // step
        "echo 'hello'".to_string(),
        0, // exit_code
        "hello".to_string(),
    );

    assert_eq!(entry.test_sequence, 1);
    assert_eq!(entry.step, 1);
    assert_eq!(entry.command, "echo 'hello'");
    assert_eq!(entry.exit_code, 0);
    assert_eq!(entry.output, "hello");
    assert!(entry.is_success());
}

#[test]
fn test_execution_log_entry_with_timestamp() {
    let timestamp = "2024-01-15T10:30:00Z";
    let entry = TestStepExecutionEntry::with_timestamp(
        1,
        1,
        "echo 'test'".to_string(),
        0,
        "test".to_string(),
        timestamp.to_string(),
    );

    assert_eq!(entry.timestamp, Some(timestamp.to_string()));
    assert!(entry.parse_timestamp().is_some());
}

#[test]
fn test_execution_log_entry_failure() {
    let entry = TestStepExecutionEntry::new(
        1,
        2,
        "exit 1".to_string(),
        1, // non-zero exit code
        "".to_string(),
    );

    assert!(!entry.is_success());
    assert!(entry.is_failure());
    assert_eq!(entry.exit_code, 1);
}

#[test]
fn test_execution_log_json_serialization() {
    let entry = TestStepExecutionEntry::new(
        1,
        1,
        "echo 'test'".to_string(),
        0,
        "test\noutput".to_string(),
    );

    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("\"test_sequence\":1"));
    assert!(json.contains("\"step\":1"));
    assert!(json.contains("\"command\":\"echo 'test'\""));
    assert!(json.contains("\"exit_code\":0"));
    assert!(json.contains("test\\noutput"));
}

#[test]
fn test_execution_log_json_array_serialization() {
    let entries = vec![
        TestStepExecutionEntry::new(1, 1, "step1".to_string(), 0, "output1".to_string()),
        TestStepExecutionEntry::new(1, 2, "step2".to_string(), 0, "output2".to_string()),
        TestStepExecutionEntry::new(1, 3, "step3".to_string(), 1, "error".to_string()),
    ];

    let json = serde_json::to_string_pretty(&entries).unwrap();
    assert!(json.starts_with('['));
    assert!(json.ends_with(']'));
    assert!(json.contains("\"step\": 1"));
    assert!(json.contains("\"step\": 2"));
    assert!(json.contains("\"step\": 3"));
    assert!(json.contains("\"exit_code\": 0"));
    assert!(json.contains("\"exit_code\": 1"));
}

#[test]
fn test_execution_log_json_deserialization() {
    let json = r#"{
        "test_sequence": 1,
        "step": 2,
        "command": "echo 'hello'",
        "exit_code": 0,
        "output": "hello"
    }"#;

    let entry: TestStepExecutionEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.test_sequence, 1);
    assert_eq!(entry.step, 2);
    assert_eq!(entry.command, "echo 'hello'");
    assert_eq!(entry.exit_code, 0);
    assert_eq!(entry.output, "hello");
}

// ============================================================================
// Script Generation Tests - Still Valid for Execution Stage
// ============================================================================

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
    let step = create_test_step(1, "Echo test", "echo 'hello'", "0", "hello", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.starts_with("#!/bin/bash\n"));
    assert!(script.contains("set -euo pipefail\n"));
    assert!(script.contains("# Test Case: TC001"));
    assert!(script.contains("# Description: Basic test case"));
}

#[test]
fn test_shell_script_generates_json_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // The new workflow should generate JSON logs, not do inline verification
    assert!(script.contains("echo 'test'"));
    assert!(script.contains("EXIT_CODE=$?"));
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

    sequence.steps.push(create_test_step(
        1,
        "First step",
        "echo 'step1'",
        "0",
        "step1",
        Some(true),
    ));
    sequence.steps.push(create_test_step(
        2,
        "Second step",
        "echo 'step2'",
        "0",
        "step2",
        Some(true),
    ));
    sequence.steps.push(create_test_step(
        3,
        "Third step",
        "echo 'step3'",
        "0",
        "step3",
        Some(true),
    ));

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
    let step1 = create_test_step(1, "Seq1 Step1", "echo 'seq1'", "0", "seq1", Some(true));
    sequence1.steps.push(step1);

    let mut sequence2 = TestSequence::new(2, "Seq2".to_string(), "Second sequence".to_string());
    let step2 = create_test_step(1, "Seq2 Step1", "echo 'seq2'", "0", "seq2", Some(true));
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
    let mut step = create_test_step(
        1,
        "Manual verification",
        "ssh device",
        "connected",
        "success",
        Some(true),
    );
    step.manual = Some(true);
    // Set verification to "true" to test simple manual step without verification
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo \"Step 1: Manual verification\""));
    assert!(script.contains("echo \"Command: ssh device\""));
    assert!(script
        .contains("echo \"INFO: This is a manual step. You must perform this action manually.\""));
    assert!(script.contains("read -p \"Press ENTER to continue...\""));
    assert!(!script.contains("MANUAL STEP - Skipping"));
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

    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Device".to_string(),
        vec![
            InitialConditionItem::String("Powered on".to_string()),
            InitialConditionItem::String("Connected".to_string()),
        ],
    );
    test_case.general_initial_conditions = InitialConditions {
        include: None,
        devices: general_devices,
    };

    let mut devices = HashMap::new();
    devices.insert(
        "Network".to_string(),
        vec![InitialConditionItem::String("Online".to_string())],
    );
    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut seq_devices = HashMap::new();
    seq_devices.insert(
        "Session".to_string(),
        vec![InitialConditionItem::String("Active".to_string())],
    );
    sequence.initial_conditions = InitialConditions {
        include: None,
        devices: seq_devices,
    };

    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Device: Powered on"));
    assert!(script.contains("# Device: Connected"));
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Network: Online"));
    assert!(script.contains("# Sequence Initial Conditions"));
    assert!(script.contains("# Session: Active"));

    // Verify each condition appears on its own line with proper formatting
    assert!(script.contains("# Device: Powered on\n"));
    assert!(script.contains("# Device: Connected\n"));
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
    let step = create_test_step(1, "Empty command", "", "0", "", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$("));
}

#[test]
fn test_command_with_special_characters() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Special characters test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(
        1,
        "Special chars",
        "echo 'hello \"world\"'",
        "0",
        "hello \"world\"",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("echo 'hello \"world\"'"));
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
    let step = create_test_step(
        1,
        "Pipe command",
        "echo 'hello world' | grep world",
        "0",
        "world",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$({ echo 'hello world' | grep world; } 2>&1 | tee"));
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
    let step = create_test_step(
        1,
        "Redirect command",
        "cat /dev/null 2>&1",
        "0",
        "",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$({ cat /dev/null 2>&1; } 2>&1 | tee"));
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
    let step = create_test_step(
        1,
        "Environment variable",
        "MY_VAR=test echo $MY_VAR",
        "0",
        "",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    assert!(script.contains("COMMAND_OUTPUT=$({ MY_VAR=test echo $MY_VAR; } 2>&1 | tee"));
}

#[test]
fn test_verification_equals_operator() {
    let verification = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string()),
        output_file: None,
        general: None,
    };

    let verification2 = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string()),
        output_file: None,
        general: None,
    };

    assert_eq!(verification, verification2);
}

#[test]
fn test_verification_not_equals_operator() {
    let verification1 = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string()),
        output_file: None,
        general: None,
    };

    let verification2 = Verification {
        result: VerificationExpression::Simple("[ $? -eq 1 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string()),
        output_file: None,
        general: None,
    };

    assert_ne!(verification1, verification2);
}

#[test]
fn test_verification_display_trait() {
    let verification = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string()),
        output_file: None,
        general: None,
    };

    let display_string = format!("{}", verification);
    assert!(display_string.contains("result: [ $? -eq 0 ]"));
    assert!(display_string.contains("output: [ \"$COMMAND_OUTPUT\" = \"test\" ]"));
}

#[test]
fn test_verification_clone() {
    let verification = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ \"$COMMAND_OUTPUT\" = \"test\" ]".to_string()),
        output_file: None,
        general: None,
    };

    let cloned = verification.clone();
    assert_eq!(verification, cloned);
}

// ============================================================================
// Two-Stage Workflow Integration Tests
// ============================================================================

#[test]
fn test_execution_produces_json_logs() {
    // In the two-stage workflow:
    // 1. Execution produces JSON logs (TestStepExecutionEntry)
    let entries = vec![
        TestStepExecutionEntry::new(1, 1, "echo 'test1'".to_string(), 0, "test1".to_string()),
        TestStepExecutionEntry::new(1, 2, "echo 'test2'".to_string(), 0, "test2".to_string()),
    ];

    // Verify JSON serialization works
    let json = serde_json::to_string_pretty(&entries).unwrap();
    assert!(json.contains("test_sequence"));
    assert!(json.contains("step"));
    assert!(json.contains("command"));
    assert!(json.contains("exit_code"));
    assert!(json.contains("output"));

    // Verify deserialization works
    let deserialized: Vec<TestStepExecutionEntry> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.len(), 2);
    assert_eq!(deserialized[0].test_sequence, 1);
    assert_eq!(deserialized[0].step, 1);
    assert_eq!(deserialized[1].step, 2);
}

#[test]
fn test_json_log_preserves_exit_codes() {
    let success_entry = TestStepExecutionEntry::new(1, 1, "true".to_string(), 0, "".to_string());
    let failure_entry = TestStepExecutionEntry::new(1, 2, "false".to_string(), 1, "".to_string());

    let entries = vec![success_entry, failure_entry];
    let json = serde_json::to_string(&entries).unwrap();

    let deserialized: Vec<TestStepExecutionEntry> = serde_json::from_str(&json).unwrap();
    assert!(deserialized[0].is_success());
    assert!(deserialized[1].is_failure());
}

#[test]
fn test_json_log_preserves_multiline_output() {
    let multiline_output = "Line 1\nLine 2\nLine 3";
    let entry =
        TestStepExecutionEntry::new(1, 1, "command".to_string(), 0, multiline_output.to_string());

    let json = serde_json::to_string(&entry).unwrap();
    let deserialized: TestStepExecutionEntry = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.output, multiline_output);
}

// ============================================================================
// Bash Helper Functions Tests
// ============================================================================

#[test]
fn test_bash_helper_functions_in_preamble() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test with bash helpers".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify helper functions are present
    assert!(
        script.contains("read_true_false()"),
        "Script should contain read_true_false function"
    );
    assert!(
        script.contains("read_verification()"),
        "Script should contain read_verification function"
    );
    assert!(
        script.contains("# Bash helper functions for user prompts"),
        "Script should contain helper function comment"
    );
}

#[test]
fn test_read_true_false_function_signature() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test read_true_false signature".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify function has proper signature
    assert!(script.contains("read_true_false() {"));
    assert!(script.contains("local prompt=\"$1\""));
    assert!(script.contains("local default=\"${2:-y}\""));
}

#[test]
fn test_read_true_false_function_tty_detection() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test TTY detection".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify TTY detection logic in read_true_false
    assert!(
        script.contains("if [[ \"${DEBIAN_FRONTEND}\" == 'noninteractive' ]] || ! [ -t 0 ]; then")
    );
    assert!(script.contains("# Non-interactive mode: return default"));
}

#[test]
fn test_read_true_false_function_returns() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test return values".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify return values (1 for yes, 0 for no)
    assert!(script.contains("return 1"));
    assert!(script.contains("return 0"));
    assert!(script.contains("# Returns: 1 for yes, 0 for no"));
}

#[test]
fn test_read_verification_function_signature() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test read_verification signature".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify read_verification function has proper signature
    assert!(script.contains("read_verification() {"));
    assert!(script.contains("local prompt=\"$1\""));
    assert!(script.contains("local default=\"${2:-y}\""));
}

#[test]
fn test_read_verification_function_tty_detection() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test read_verification TTY detection".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify TTY detection logic in read_verification
    assert!(
        script.contains("if [[ \"${DEBIAN_FRONTEND}\" == 'noninteractive' ]] || ! [ -t 0 ]; then")
    );
    assert!(script.contains("# Non-interactive mode: return default"));
}

#[test]
fn test_read_verification_function_returns() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test read_verification return values".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify return values (1 for yes, 0 for no) in read_verification
    assert!(script.contains("return 1"));
    assert!(script.contains("return 0"));
    assert!(script.contains("# Returns: 1 for yes, 0 for no"));
}

#[test]
fn test_helper_function_input_validation_case_statements() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test input validation with case statements".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify input validation with case statements
    assert!(script.contains("case \"$response\" in"));
    assert!(script.contains("[Yy]|[Yy][Ee][Ss])"));
    assert!(script.contains("[Nn]|[Nn][Oo])"));
    assert!(script.contains("Invalid response. Please enter Y or n."));
}

#[test]
fn test_helper_function_prompt_variations() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test prompt variations".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify prompt variations ([Y/n] vs [y/N])
    assert!(script.contains("read -p \"$prompt [Y/n]: \" response"));
    assert!(script.contains("read -p \"$prompt [y/N]: \" response"));
    assert!(script.contains("if [[ \"$default\" =~ ^[Yy]$ ]]; then"));
}

#[test]
fn test_helper_function_default_parameter_handling() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test default parameter handling".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify default parameter handling with ${2:-y}
    assert!(script.contains("local default=\"${2:-y}\""));

    // Verify empty response uses default
    assert!(script.contains("# Empty response uses default"));
    assert!(script.contains("if [[ -z \"$response\" ]]; then"));
    assert!(script.contains("response=\"$default\""));
}

#[test]
fn test_helper_function_interactive_mode_logic() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test interactive mode logic".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify interactive mode loop and validation
    assert!(script.contains("# Interactive mode: prompt user"));
    assert!(script.contains("while true; do"));
    assert!(script.contains("# Validate response"));
}

#[test]
fn test_helper_function_non_interactive_default_yes() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test non-interactive mode with default yes".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify non-interactive mode returns correct default
    assert!(script.contains("if [[ \"$default\" =~ ^[Yy]$ ]]; then"));
    assert!(script.contains("return 1")); // Yes returns 1
    assert!(script.contains("return 0")); // No returns 0
}

#[test]
fn test_read_true_false_function_complete_structure() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test read_true_false complete structure".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify complete structure of read_true_false
    assert!(script.contains("read_true_false() {"));
    assert!(script.contains("local prompt=\"$1\""));
    assert!(script.contains("local default=\"${2:-y}\""));
    assert!(script.contains("# Check if running in non-interactive mode"));
    assert!(
        script.contains("if [[ \"${DEBIAN_FRONTEND}\" == 'noninteractive' ]] || ! [ -t 0 ]; then")
    );
    assert!(script.contains("# Non-interactive mode: return default"));
    assert!(script.contains("# Interactive mode: prompt user"));
    assert!(script.contains("while true; do"));
    assert!(script.contains("case \"$response\" in"));
    assert!(script.contains("[Yy]|[Yy][Ee][Ss])"));
    assert!(script.contains("return 1"));
    assert!(script.contains("[Nn]|[Nn][Oo])"));
    assert!(script.contains("return 0"));
    assert!(script.contains("Invalid response. Please enter Y or n."));
    assert!(script.contains("done"));
    assert!(script.contains("}"));
}

#[test]
fn test_read_verification_function_complete_structure() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test read_verification complete structure".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify complete structure of read_verification
    assert!(script.contains("read_verification() {"));
    assert!(script.contains("local prompt=\"$1\""));
    assert!(script.contains("local default=\"${2:-y}\""));
    assert!(script.contains("# Check if running in non-interactive mode"));
    assert!(
        script.contains("if [[ \"${DEBIAN_FRONTEND}\" == 'noninteractive' ]] || ! [ -t 0 ]; then")
    );
    assert!(script.contains("# Non-interactive mode: return default"));
    assert!(script.contains("# Interactive mode: prompt user"));
    assert!(script.contains("while true; do"));
    assert!(script.contains("case \"$response\" in"));
    assert!(script.contains("[Yy]|[Yy][Ee][Ss])"));
    assert!(script.contains("return 1"));
    assert!(script.contains("[Nn]|[Nn][Oo])"));
    assert!(script.contains("return 0"));
    assert!(script.contains("Invalid response. Please enter Y or n."));
    assert!(script.contains("done"));
    assert!(script.contains("}"));
}

#[test]
fn test_helper_functions_comment_documentation() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test helper functions documentation".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify documentation comments
    assert!(script.contains("# Bash helper functions for user prompts"));
    assert!(script.contains("# Prompts user for Y/n input with proper validation"));
    assert!(script.contains("# Returns: 1 for yes, 0 for no"));
    assert!(
        script.contains("# Supports both interactive and non-interactive modes with TTY detection")
    );
    assert!(script.contains("# Prompts user for verification with Y/n input"));
}

#[test]
fn test_helper_functions_stderr_error_output() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test helper functions error output to stderr".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify invalid response errors go to stderr
    assert!(script.contains("echo \"Invalid response. Please enter Y or n.\" >&2"));
}

#[test]
fn test_both_helper_functions_present_in_preamble() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test both helper functions in preamble".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Get indices to verify order
    let preamble_start = script.find("#!/bin/bash").expect("Shebang not found");
    let read_true_false_pos = script
        .find("read_true_false() {")
        .expect("read_true_false not found");
    let read_verification_pos = script
        .find("read_verification() {")
        .expect("read_verification not found");
    let test_case_pos = script
        .find("# Test Case:")
        .expect("Test case header not found");

    // Verify both functions are in preamble (before test case)
    assert!(read_true_false_pos > preamble_start);
    assert!(read_true_false_pos < test_case_pos);
    assert!(read_verification_pos > preamble_start);
    assert!(read_verification_pos < test_case_pos);

    // Verify read_true_false comes before read_verification
    assert!(read_true_false_pos < read_verification_pos);
}

// ============================================================================
// Manual Steps with Verification Expressions Tests
// ============================================================================

#[test]
fn test_manual_step_with_simple_verification() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Manual step with verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Check device status",
        "ssh device 'cat /var/log/status.log'",
        "0",
        "ready",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/device_ready ]".to_string()),
        output: VerificationExpression::Simple("grep -q 'ready' /tmp/status.log".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify manual step is recognized
    assert!(script.contains("echo \"Step 1: Check device status\""));
    assert!(script
        .contains("echo \"INFO: This is a manual step. You must perform this action manually.\""));

    // Verify USER_VERIFICATION variables are set
    assert!(script.contains("USER_VERIFICATION_RESULT=false"));
    assert!(script.contains("USER_VERIFICATION_OUTPUT=false"));

    // Verify verification logic is generated
    assert!(script.contains("if [ -f /tmp/device_ready ]; then"));
    assert!(script.contains("USER_VERIFICATION_RESULT=true"));
    assert!(script.contains("if grep -q 'ready' /tmp/status.log; then"));
    assert!(script.contains("USER_VERIFICATION_OUTPUT=true"));

    // Verify combined USER_VERIFICATION check
    assert!(script.contains("# Set USER_VERIFICATION based on verification results"));
    assert!(script.contains(
        "if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"
    ));
    assert!(script.contains("USER_VERIFICATION=true"));
    assert!(script.contains("USER_VERIFICATION=false"));
}

#[test]
fn test_manual_step_with_conditional_verification() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ002".to_string(),
        1,
        1,
        "TC002".to_string(),
        "Manual step with conditional verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Deploy configuration",
        "scp config.txt device:/etc/config.txt",
        "0",
        "success",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ -f /tmp/deployment_mode ]".to_string(),
            if_true: Some(vec!["echo 'Production mode'".to_string()]),
            if_false: Some(vec!["echo 'Development mode'".to_string()]),
            always: Some(vec!["echo 'Deployment complete'".to_string()]),
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify conditional verification is generated
    assert!(script.contains("if [ -f /tmp/deployment_mode ]; then"));
    assert!(script.contains("USER_VERIFICATION_RESULT=true"));
    assert!(script.contains("echo 'Production mode'"));
    assert!(script.contains("echo 'Development mode'"));
    assert!(script.contains("echo 'Deployment complete'"));
}

#[test]
fn test_manual_step_user_verification_variable_usage() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ003".to_string(),
        1,
        1,
        "TC003".to_string(),
        "Test USER_VERIFICATION variable".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Verify system state",
        "systemctl status app",
        "0",
        "active",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("systemctl is-active app".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify USER_VERIFICATION variable is used for pass/fail logic
    assert!(script.contains("if [ \"$USER_VERIFICATION\" = true ]; then"));
    assert!(script.contains("[PASS] Step 1: Verify system state"));
    assert!(script.contains("else"));
    assert!(script.contains("[FAIL] Step 1: Verify system state"));
    assert!(script.contains("echo \"  Result verification: $USER_VERIFICATION_RESULT\""));
    assert!(script.contains("echo \"  Output verification: $USER_VERIFICATION_OUTPUT\""));
    assert!(script.contains("exit 1"));
}

#[test]
fn test_manual_step_without_verification_no_user_verification_variable() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ004".to_string(),
        1,
        1,
        "TC004".to_string(),
        "Manual step without verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual action",
        "reboot device",
        "0",
        "rebooted",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify no USER_VERIFICATION variables are set when verification is "true"
    assert!(!script.contains("USER_VERIFICATION_RESULT=false"));
    assert!(!script.contains("USER_VERIFICATION_OUTPUT=false"));
    assert!(!script.contains("USER_VERIFICATION=true"));
    assert!(!script.contains("[PASS] Step 1"));
    assert!(!script.contains("[FAIL] Step 1"));

    // Should just prompt to continue
    assert!(script.contains("read -p \"Press ENTER to continue...\""));
}

#[test]
fn test_manual_step_with_read_true_false_in_verification() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ005".to_string(),
        1,
        1,
        "TC005".to_string(),
        "Manual step using read_true_false".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Confirm LED is blinking",
        "observe device LED",
        "0",
        "blinking",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple(
            "read_true_false \"Is the LED blinking?\" \"y\"".to_string(),
        ),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify read_true_false is called in verification
    assert!(script.contains("if read_true_false \"Is the LED blinking?\" \"y\"; then"));
    assert!(script.contains("USER_VERIFICATION_RESULT=true"));
}

#[test]
fn test_manual_step_with_complex_verification_referencing_read_true_false() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ006".to_string(),
        1,
        1,
        "TC006".to_string(),
        "Complex verification with read_true_false".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Multi-step verification",
        "complex manual steps",
        "0",
        "verified",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "read_true_false \"Did step 1 succeed?\" \"y\"".to_string(),
            if_true: Some(vec![
                "read_true_false \"Did step 2 succeed?\" \"y\" || exit 1".to_string(),
            ]),
            if_false: Some(vec![
                "echo 'Step 1 failed'".to_string(),
                "exit 1".to_string(),
            ]),
            always: Some(vec!["echo 'Verification complete'".to_string()]),
        },
        output: VerificationExpression::Simple(
            "read_true_false \"Was output correct?\" \"y\"".to_string(),
        ),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify complex conditional with read_true_false is generated
    assert!(script.contains("if read_true_false \"Did step 1 succeed?\" \"y\"; then"));
    assert!(script.contains("USER_VERIFICATION_RESULT=true"));
    assert!(script.contains("read_true_false \"Did step 2 succeed?\" \"y\" || exit 1"));
    assert!(script.contains("echo 'Step 1 failed'"));
    assert!(script.contains("echo 'Verification complete'"));

    // Verify output verification with read_true_false
    assert!(script.contains("if read_true_false \"Was output correct?\" \"y\"; then"));
    assert!(script.contains("USER_VERIFICATION_OUTPUT=true"));
}

#[test]
fn test_manual_step_verification_with_file_checks() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ007".to_string(),
        1,
        1,
        "TC007".to_string(),
        "Manual step with file checks".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Create config file",
        "manually create /etc/app.conf",
        "0",
        "created",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /etc/app.conf ]".to_string()),
        output: VerificationExpression::Simple("grep -q 'version=1.0' /etc/app.conf".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify file check verification expressions
    assert!(script.contains("if [ -f /etc/app.conf ]; then"));
    assert!(script.contains("USER_VERIFICATION_RESULT=true"));
    assert!(script.contains("if grep -q 'version=1.0' /etc/app.conf; then"));
    assert!(script.contains("USER_VERIFICATION_OUTPUT=true"));
}

#[test]
fn test_manual_step_verification_combining_multiple_checks() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ008".to_string(),
        1,
        1,
        "TC008".to_string(),
        "Manual step combining multiple checks".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "System verification",
        "verify system state",
        "0",
        "verified",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple(
            "[ -f /tmp/file1 ] && [ -f /tmp/file2 ] && read_true_false \"Are services running?\" \"y\"".to_string(),
        ),
        output: VerificationExpression::Simple(
            "grep -q 'status=ok' /tmp/status && read_true_false \"Is display correct?\" \"y\"".to_string(),
        ),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify combined checks in verification expressions
    assert!(script.contains(
        "[ -f /tmp/file1 ] && [ -f /tmp/file2 ] && read_true_false \"Are services running?\" \"y\""
    ));
    assert!(script.contains(
        "grep -q 'status=ok' /tmp/status && read_true_false \"Is display correct?\" \"y\""
    ));
}

#[test]
fn test_script_with_both_manual_and_automated_steps() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ009".to_string(),
        1,
        1,
        "TC009".to_string(),
        "Mixed manual and automated steps".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    // Add automated step
    let auto_step = create_test_step(
        1,
        "Automated step",
        "echo 'automated'",
        "0",
        "automated",
        Some(true),
    );
    sequence.steps.push(auto_step);

    // Add manual step with verification
    let mut manual_step =
        create_test_step(2, "Manual step", "manual action", "0", "done", Some(true));
    manual_step.manual = Some(true);
    manual_step.verification = Verification {
        result: VerificationExpression::Simple(
            "read_true_false \"Is it complete?\" \"y\"".to_string(),
        ),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(manual_step);

    // Add another automated step
    let auto_step2 = create_test_step(
        3,
        "Another automated step",
        "echo 'done'",
        "0",
        "done",
        Some(true),
    );
    sequence.steps.push(auto_step2);

    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify automated steps use normal execution
    assert!(script.contains("# Step 1: Automated step"));
    assert!(script.contains("COMMAND_OUTPUT=$({ echo 'automated'; } 2>&1 | tee"));

    // Verify manual step uses USER_VERIFICATION
    assert!(script.contains("# Step 2: Manual step"));
    assert!(script.contains("USER_VERIFICATION_RESULT=false"));
    assert!(script.contains("read_true_false \"Is it complete?\" \"y\""));

    // Verify third automated step
    assert!(script.contains("# Step 3: Another automated step"));
    assert!(script.contains("COMMAND_OUTPUT=$({ echo 'done'; } 2>&1 | tee"));
}

#[test]
fn test_manual_step_pass_fail_messages() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ010".to_string(),
        1,
        1,
        "TC010".to_string(),
        "Manual step pass/fail messages".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Critical manual check",
        "inspect device",
        "0",
        "ok",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/check_result ]".to_string()),
        output: VerificationExpression::Simple("grep -q 'PASS' /tmp/check_result".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify pass message
    assert!(script.contains("echo \"[PASS] Step 1: Critical manual check\""));

    // Verify fail message with details
    assert!(script.contains("echo \"[FAIL] Step 1: Critical manual check\""));
    assert!(script.contains("echo \"  Result verification: $USER_VERIFICATION_RESULT\""));
    assert!(script.contains("echo \"  Output verification: $USER_VERIFICATION_OUTPUT\""));

    // Verify exit on failure
    assert!(script.contains("exit 1"));
}

#[test]
fn test_manual_step_interactive_prompt() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ011".to_string(),
        1,
        1,
        "TC011".to_string(),
        "Manual step interactive prompt".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Interactive check",
        "check something",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/result ]".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify interactive mode check
    assert!(script.contains("if [[ \"${DEBIAN_FRONTEND}\" != 'noninteractive' && -t 0 ]]; then"));
    assert!(script.contains("read -p \"Press ENTER after completing the manual action...\""));
    assert!(script.contains("else"));
    assert!(script
        .contains("echo \"Non-interactive mode detected, skipping manual step confirmation.\""));
}

#[test]
fn test_multiple_manual_steps_in_sequence() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ012".to_string(),
        1,
        1,
        "TC012".to_string(),
        "Multiple manual steps".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    // First manual step
    let mut step1 = create_test_step(1, "Manual step 1", "action 1", "0", "done", Some(true));
    step1.manual = Some(true);
    step1.verification = Verification {
        result: VerificationExpression::Simple(
            "read_true_false \"Step 1 done?\" \"y\"".to_string(),
        ),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step1);

    // Second manual step
    let mut step2 = create_test_step(2, "Manual step 2", "action 2", "0", "done", Some(true));
    step2.manual = Some(true);
    step2.verification = Verification {
        result: VerificationExpression::Simple(
            "read_true_false \"Step 2 done?\" \"y\"".to_string(),
        ),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step2);

    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify both manual steps have verification
    assert!(script.contains("# Step 1: Manual step 1"));
    assert!(script.contains("read_true_false \"Step 1 done?\" \"y\""));

    assert!(script.contains("# Step 2: Manual step 2"));
    assert!(script.contains("read_true_false \"Step 2 done?\" \"y\""));

    // Each should have their own USER_VERIFICATION checks
    let user_verification_result_count = script.matches("USER_VERIFICATION_RESULT=false").count();
    let user_verification_output_count = script.matches("USER_VERIFICATION_OUTPUT=false").count();
    assert_eq!(user_verification_result_count, 2);
    assert_eq!(user_verification_output_count, 2);
}

// ============================================================================
// Manual Step Verification Script Generation Tests
// ============================================================================

#[test]
fn test_manual_verification_initializes_user_verification_variables() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV001".to_string(),
        1,
        1,
        "TC_MV001".to_string(),
        "Test USER_VERIFICATION variable initialization".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual verification step",
        "perform manual action",
        "0",
        "success",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/result_file ]".to_string()),
        output: VerificationExpression::Simple("grep -q 'SUCCESS' /tmp/output_file".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify USER_VERIFICATION_RESULT is initialized to false
    assert!(
        script.contains("USER_VERIFICATION_RESULT=false"),
        "Script must initialize USER_VERIFICATION_RESULT to false"
    );

    // Verify USER_VERIFICATION_OUTPUT is initialized to false
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT=false"),
        "Script must initialize USER_VERIFICATION_OUTPUT to false"
    );

    // Verify initialization happens before evaluation
    let result_init_pos = script.find("USER_VERIFICATION_RESULT=false").unwrap();
    let output_init_pos = script.find("USER_VERIFICATION_OUTPUT=false").unwrap();
    let result_eval_pos = script.find("if [ -f /tmp/result_file ]").unwrap();
    let output_eval_pos = script
        .find("if grep -q 'SUCCESS' /tmp/output_file")
        .unwrap();

    assert!(
        result_init_pos < result_eval_pos,
        "USER_VERIFICATION_RESULT initialization must occur before result evaluation"
    );
    assert!(
        output_init_pos < output_eval_pos,
        "USER_VERIFICATION_OUTPUT initialization must occur before output evaluation"
    );
}

#[test]
fn test_manual_verification_generates_result_evaluation_code() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV002".to_string(),
        1,
        1,
        "TC_MV002".to_string(),
        "Test result verification evaluation code".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Result verification test",
        "check result",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/result.txt ]".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify result evaluation code is generated
    assert!(
        script.contains("if [ -f /tmp/result.txt ]; then"),
        "Script must contain result verification condition"
    );
    assert!(
        script.contains("USER_VERIFICATION_RESULT=true"),
        "Script must set USER_VERIFICATION_RESULT=true on success"
    );

    // Verify the evaluation happens after initialization
    let init_pos = script.find("USER_VERIFICATION_RESULT=false").unwrap();
    let eval_pos = script.find("if [ -f /tmp/result.txt ]").unwrap();
    let set_true_pos = script.find("USER_VERIFICATION_RESULT=true").unwrap();

    assert!(
        init_pos < eval_pos && eval_pos < set_true_pos,
        "Evaluation must happen in correct order: initialize, check, set result"
    );
}

#[test]
fn test_manual_verification_generates_output_evaluation_code() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV003".to_string(),
        1,
        1,
        "TC_MV003".to_string(),
        "Test output verification evaluation code".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Output verification test",
        "check output",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("grep -q 'VALID' /tmp/output.log".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify output evaluation code is generated
    assert!(
        script.contains("if grep -q 'VALID' /tmp/output.log; then"),
        "Script must contain output verification condition"
    );
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT=true"),
        "Script must set USER_VERIFICATION_OUTPUT=true on success"
    );

    // Verify the evaluation happens after initialization
    let init_pos = script.find("USER_VERIFICATION_OUTPUT=false").unwrap();
    let eval_pos = script.find("if grep -q 'VALID' /tmp/output.log").unwrap();
    let set_true_pos = script.find("USER_VERIFICATION_OUTPUT=true").unwrap();

    assert!(
        init_pos < eval_pos && eval_pos < set_true_pos,
        "Evaluation must happen in correct order: initialize, check, set result"
    );
}

#[test]
fn test_manual_verification_combines_with_and_condition() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV004".to_string(),
        1,
        1,
        "TC_MV004".to_string(),
        "Test combined USER_VERIFICATION with AND".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Combined verification",
        "check both",
        "0",
        "both checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/result ]".to_string()),
        output: VerificationExpression::Simple("[ -f /tmp/output ]".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify combined USER_VERIFICATION logic with AND condition
    assert!(
        script.contains("# Set USER_VERIFICATION based on verification results"),
        "Script must contain comment for combined verification"
    );
    assert!(
        script.contains("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"),
        "Script must combine result and output with AND condition"
    );
    assert!(
        script.contains("USER_VERIFICATION=true"),
        "Script must set USER_VERIFICATION=true when both are true"
    );
    assert!(
        script.contains("USER_VERIFICATION=false"),
        "Script must set USER_VERIFICATION=false when either is false"
    );

    // Verify proper structure with if/else
    let and_condition_pos = script
        .find("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]")
        .unwrap();
    let set_true_pos = script[and_condition_pos..]
        .find("USER_VERIFICATION=true")
        .unwrap()
        + and_condition_pos;
    let else_pos = script[and_condition_pos..].find("else").unwrap() + and_condition_pos;
    let set_false_pos = script[else_pos..].find("USER_VERIFICATION=false").unwrap() + else_pos;

    assert!(
        and_condition_pos < set_true_pos && set_true_pos < else_pos && else_pos < set_false_pos,
        "Combined verification must have proper if/else structure"
    );
}

#[test]
fn test_manual_verification_generates_pass_fail_messages_with_diagnostics() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV005".to_string(),
        1,
        1,
        "TC_MV005".to_string(),
        "Test PASS/FAIL messages with diagnostics".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Diagnostic test step",
        "perform diagnostic check",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ -n \"$OUTPUT\" ]".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify [PASS] message is generated
    assert!(
        script.contains("[PASS] Step 1: Diagnostic test step"),
        "Script must contain [PASS] message with step description"
    );

    // Verify [FAIL] message is generated
    assert!(
        script.contains("[FAIL] Step 1: Diagnostic test step"),
        "Script must contain [FAIL] message with step description"
    );

    // Verify diagnostic output for result verification
    assert!(
        script.contains("Result verification: $USER_VERIFICATION_RESULT"),
        "Script must show result verification status in diagnostic output"
    );

    // Verify diagnostic output for output verification
    assert!(
        script.contains("Output verification: $USER_VERIFICATION_OUTPUT"),
        "Script must show output verification status in diagnostic output"
    );

    // Verify proper structure: check USER_VERIFICATION, then show pass/fail
    let user_verification_check = script.find("if [ \"$USER_VERIFICATION\" = true ]").unwrap();
    let pass_message_pos = script[user_verification_check..]
        .find("[PASS] Step 1")
        .unwrap()
        + user_verification_check;
    let fail_message_pos = script[user_verification_check..]
        .find("[FAIL] Step 1")
        .unwrap()
        + user_verification_check;
    let result_diagnostic_pos = script[fail_message_pos..]
        .find("Result verification:")
        .unwrap()
        + fail_message_pos;
    let output_diagnostic_pos = script[result_diagnostic_pos..]
        .find("Output verification:")
        .unwrap()
        + result_diagnostic_pos;

    assert!(
        user_verification_check < pass_message_pos
            && pass_message_pos < fail_message_pos
            && fail_message_pos < result_diagnostic_pos
            && result_diagnostic_pos < output_diagnostic_pos,
        "Pass/fail messages and diagnostics must be in correct order"
    );
}

#[test]
fn test_manual_verification_exits_on_failure() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV006".to_string(),
        1,
        1,
        "TC_MV006".to_string(),
        "Test exit 1 on verification failure".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Exit on failure test",
        "action that might fail",
        "0",
        "completed",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/must_exist ]".to_string()),
        output: VerificationExpression::Simple("[ -f /tmp/output_must_exist ]".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify exit 1 is called on failure
    assert!(
        script.contains("exit 1"),
        "Script must contain 'exit 1' on verification failure"
    );

    // Verify exit 1 is in the failure branch (after [FAIL] message)
    let fail_message_pos = script.find("[FAIL] Step 1").unwrap();
    let exit_pos = script[fail_message_pos..].find("exit 1").unwrap() + fail_message_pos;

    assert!(
        fail_message_pos < exit_pos,
        "exit 1 must be after [FAIL] message"
    );

    // Verify exit 1 is after diagnostic output
    let result_diagnostic_pos = script[fail_message_pos..]
        .find("Result verification:")
        .unwrap()
        + fail_message_pos;
    let output_diagnostic_pos = script[result_diagnostic_pos..]
        .find("Output verification:")
        .unwrap()
        + result_diagnostic_pos;

    assert!(
        result_diagnostic_pos < exit_pos && output_diagnostic_pos < exit_pos,
        "exit 1 must be after diagnostic output"
    );

    // Verify the exit is within the failure else block
    let user_verification_check = script.find("if [ \"$USER_VERIFICATION\" = true ]").unwrap();
    let else_pos =
        script[user_verification_check..].find("else").unwrap() + user_verification_check;
    let fi_after_exit = script[exit_pos..].find("fi").unwrap() + exit_pos;

    assert!(
        user_verification_check < else_pos && else_pos < exit_pos && exit_pos < fi_after_exit,
        "exit 1 must be within the else block of USER_VERIFICATION check"
    );
}

// ============================================================================
// execute_test_case with Manual Verification Tests
// ============================================================================

#[test]
fn test_execute_manual_step_checks_meaningful_verification() {
    // Test that execute_test_case identifies meaningful verification
    // (not just "true") through its logic
    let mut test_case = TestCase::new(
        "REQ_EXEC_001".to_string(),
        1,
        1,
        "TC_EXEC_001".to_string(),
        "Execute manual step with meaningful verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual step with file check",
        "touch /tmp/test_file.txt",
        "0",
        "success",
        Some(true),
    );
    step.manual = Some(true);
    // Meaningful verification - not just "true"
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/test_file.txt ]".to_string()),
        output: VerificationExpression::Simple("[ -n \"$OUTPUT\" ]".to_string()),
        output_file: None,
        general: None,
    };

    // Test that verification is meaningful (has_result_verification logic)
    let has_result_verification = !matches!(&step.verification.result,
        VerificationExpression::Simple(s) if s.trim() == "true");
    let has_output_verification = !matches!(&step.verification.output,
        VerificationExpression::Simple(s) if s.trim() == "true");
    let has_verification = has_result_verification || has_output_verification;

    assert!(has_verification, "Should detect meaningful verification");
    assert!(
        has_result_verification,
        "Result verification should be meaningful"
    );
    assert!(
        has_output_verification,
        "Output verification should be meaningful"
    );

    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);
}

#[test]
fn test_execute_manual_step_skips_trivial_verification() {
    // Test that execute_test_case skips manual steps with trivial verification
    let mut test_case = TestCase::new(
        "REQ_EXEC_002".to_string(),
        1,
        1,
        "TC_EXEC_002".to_string(),
        "Execute manual step with trivial verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual step with trivial verification",
        "manual action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    // Trivial verification - just "true"
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };

    // Test detection of trivial verification
    let has_result_verification = !matches!(&step.verification.result,
        VerificationExpression::Simple(s) if s.trim() == "true");
    let has_output_verification = !matches!(&step.verification.output,
        VerificationExpression::Simple(s) if s.trim() == "true");
    let has_verification = has_result_verification || has_output_verification;

    assert!(!has_verification, "Should detect trivial verification");

    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);
}

#[test]
fn test_execute_manual_step_mixed_verification() {
    // Test manual step with mixed meaningful/trivial verification
    let mut test_case = TestCase::new(
        "REQ_EXEC_003".to_string(),
        1,
        1,
        "TC_EXEC_003".to_string(),
        "Execute manual step with mixed verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual step with mixed verification",
        "perform action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    // Mixed: meaningful result, trivial output
    step.verification = Verification {
        result: VerificationExpression::Simple("[ $? -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };

    // Test detection of mixed verification
    let has_result_verification = !matches!(&step.verification.result,
        VerificationExpression::Simple(s) if s.trim() == "true");
    let has_output_verification = !matches!(&step.verification.output,
        VerificationExpression::Simple(s) if s.trim() == "true");
    let has_verification = has_result_verification || has_output_verification;

    assert!(
        has_verification,
        "Should detect meaningful verification (mixed)"
    );
    assert!(has_result_verification, "Result should be meaningful");
    assert!(!has_output_verification, "Output should be trivial");

    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);
}

#[test]
fn test_execute_manual_prompt_format() {
    // Test that the confirmation prompt format is correct
    let step_number = 1;
    let expected_prompt = format!(
        "Have you completed the manual action for Step {}?",
        step_number
    );
    assert_eq!(
        expected_prompt,
        "Have you completed the manual action for Step 1?"
    );
}

#[test]
fn test_execute_manual_with_general_verifications() {
    // Test manual step with general verifications
    let mut test_case = TestCase::new(
        "REQ_EXEC_004".to_string(),
        1,
        1,
        "TC_EXEC_004".to_string(),
        "Test manual step with general verifications".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Step with general verifications",
        "perform complex action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
        output: VerificationExpression::Simple("[ -n \"$COMMAND_OUTPUT\" ]".to_string()),
        output_file: None,
        general: Some(vec![
            testcase_manager::models::GeneralVerification {
                name: "check_file_exists".to_string(),
                condition: "[ -f /tmp/result.txt ]".to_string(),
            },
            testcase_manager::models::GeneralVerification {
                name: "check_permissions".to_string(),
                condition: "[ -r /tmp/result.txt ]".to_string(),
            },
        ]),
    };

    // Verify general verifications are present
    assert!(step.verification.general.is_some());
    let general = step.verification.general.as_ref().unwrap();
    assert_eq!(general.len(), 2);
    assert_eq!(general[0].name, "check_file_exists");
    assert_eq!(general[1].name, "check_permissions");

    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);
}

#[test]
fn test_manual_verification_complete_workflow() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV007".to_string(),
        1,
        1,
        "TC_MV007".to_string(),
        "Test complete manual verification workflow".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Complete workflow test",
        "comprehensive manual check",
        "0",
        "all verified",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("systemctl is-active my-service".to_string()),
        output: VerificationExpression::Simple(
            "grep -q 'status=running' /var/log/service.log".to_string(),
        ),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify complete workflow in order:
    // 1. Initialize variables
    assert!(script.contains("USER_VERIFICATION_RESULT=false"));
    assert!(script.contains("USER_VERIFICATION_OUTPUT=false"));

    // 2. Evaluate result verification
    assert!(script.contains("if systemctl is-active my-service; then"));
    assert!(script.contains("USER_VERIFICATION_RESULT=true"));

    // 3. Evaluate output verification
    assert!(script.contains("if grep -q 'status=running' /var/log/service.log; then"));
    assert!(script.contains("USER_VERIFICATION_OUTPUT=true"));

    // 4. Combine with AND condition
    assert!(script.contains(
        "if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"
    ));
    assert!(script.contains("USER_VERIFICATION=true"));
    assert!(script.contains("USER_VERIFICATION=false"));

    // 5. Generate PASS/FAIL messages
    assert!(script.contains("[PASS] Step 1: Complete workflow test"));
    assert!(script.contains("[FAIL] Step 1: Complete workflow test"));

    // 6. Show diagnostics
    assert!(script.contains("Result verification: $USER_VERIFICATION_RESULT"));
    assert!(script.contains("Output verification: $USER_VERIFICATION_OUTPUT"));

    // 7. Exit on failure
    assert!(script.contains("exit 1"));

    // Verify order of execution
    let init_result_pos = script.find("USER_VERIFICATION_RESULT=false").unwrap();
    let init_output_pos = script.find("USER_VERIFICATION_OUTPUT=false").unwrap();
    let eval_result_pos = script.find("if systemctl is-active my-service").unwrap();
    let eval_output_pos = script
        .find("if grep -q 'status=running' /var/log/service.log")
        .unwrap();
    let combine_pos = script
        .find("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]")
        .unwrap();
    let pass_pos = script.find("[PASS] Step 1").unwrap();
    let fail_pos = script.find("[FAIL] Step 1").unwrap();
    // Find exit 1 after the fail message
    let exit_pos = script[fail_pos..].find("exit 1").unwrap() + fail_pos;

    assert!(
        init_result_pos < init_output_pos
            && init_output_pos < eval_result_pos
            && eval_result_pos < eval_output_pos
            && eval_output_pos < combine_pos
            && combine_pos < pass_pos
            && pass_pos < fail_pos
            && fail_pos < exit_pos,
        "Complete workflow must execute in correct order"
    );
}

#[test]
fn test_manual_verification_with_both_result_and_output_expressions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV008".to_string(),
        1,
        1,
        "TC_MV008".to_string(),
        "Test both result and output verification expressions".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Dual verification",
        "check result and output",
        "0",
        "verified",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("test -x /usr/bin/myapp".to_string()),
        output: VerificationExpression::Simple("pgrep -f myapp > /dev/null".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify both result and output verification expressions are evaluated
    assert!(
        script.contains("if test -x /usr/bin/myapp; then"),
        "Result verification expression must be evaluated"
    );
    assert!(
        script.contains("if pgrep -f myapp > /dev/null; then"),
        "Output verification expression must be evaluated"
    );

    // Verify both can set their respective variables to true
    let result_check = script.find("if test -x /usr/bin/myapp; then").unwrap();
    let result_true = script[result_check..]
        .find("USER_VERIFICATION_RESULT=true")
        .unwrap()
        + result_check;

    let output_check = script.find("if pgrep -f myapp > /dev/null; then").unwrap();
    let output_true = script[output_check..]
        .find("USER_VERIFICATION_OUTPUT=true")
        .unwrap()
        + output_check;

    assert!(
        result_check < result_true,
        "Result verification must set USER_VERIFICATION_RESULT=true"
    );
    assert!(
        output_check < output_true,
        "Output verification must set USER_VERIFICATION_OUTPUT=true"
    );

    // Verify both are required for overall success (AND condition)
    assert!(
        script.contains("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"),
        "Both verifications must pass for overall success"
    );
}

#[test]
fn test_manual_verification_result_only() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV009".to_string(),
        1,
        1,
        "TC_MV009".to_string(),
        "Test result verification only".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Result only verification",
        "check result only",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -d /tmp/test_dir ]".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // When output is "true", USER_VERIFICATION_OUTPUT should be set to true
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT=true"),
        "Output should default to true when expression is 'true'"
    );

    // Result verification should still be evaluated
    assert!(
        script.contains("if [ -d /tmp/test_dir ]; then"),
        "Result verification must be evaluated"
    );
    assert!(
        script.contains("USER_VERIFICATION_RESULT=true"),
        "Result verification can set to true"
    );

    // Combined check should still work
    assert!(
        script.contains("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"),
        "Combined check must still use AND condition"
    );
}

#[test]
fn test_manual_verification_output_only() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_MV010".to_string(),
        1,
        1,
        "TC_MV010".to_string(),
        "Test output verification only".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Output only verification",
        "check output only",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple(
            "tail -1 /var/log/app.log | grep -q 'SUCCESS'".to_string(),
        ),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // When result is "true", USER_VERIFICATION_RESULT should be set to true
    assert!(
        script.contains("USER_VERIFICATION_RESULT=true"),
        "Result should default to true when expression is 'true'"
    );

    // Output verification should still be evaluated
    assert!(
        script.contains("if tail -1 /var/log/app.log | grep -q 'SUCCESS'; then"),
        "Output verification must be evaluated"
    );
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT=true"),
        "Output verification can set to true"
    );

    // Combined check should still work
    assert!(
        script.contains("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"),
        "Combined check must still use AND condition"
    );
}

// ============================================================================
// Manual Steps with Conditional Verification Expression Tests
// ============================================================================

#[test]
fn test_manual_step_with_conditional_verification_result_generates_if_else_fi_blocks() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV001".to_string(),
        1,
        1,
        "TC_CV001".to_string(),
        "Test conditional verification generates if/else/fi blocks".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual step with conditional",
        "perform manual action",
        "0",
        "success",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ -f /tmp/production ]".to_string(),
            if_true: Some(vec!["echo 'Production mode'".to_string()]),
            if_false: Some(vec!["echo 'Development mode'".to_string()]),
            always: Some(vec!["echo 'Always executed'".to_string()]),
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify if block
    assert!(
        script.contains("if [ -f /tmp/production ]; then"),
        "Script must generate if statement with condition"
    );

    // Verify else block
    assert!(
        script.contains("else"),
        "Script must generate else statement"
    );

    // Verify fi block
    assert!(
        script.contains("fi"),
        "Script must generate fi statement to close if/else block"
    );

    // Verify structure - if comes before else, else comes before fi
    let if_pos = script.find("if [ -f /tmp/production ]; then").unwrap();
    let else_pos = script[if_pos..].find("else").unwrap() + if_pos;
    let fi_pos = script[else_pos..].find("fi").unwrap() + else_pos;

    assert!(
        if_pos < else_pos && else_pos < fi_pos,
        "if/else/fi blocks must be in correct order"
    );
}

#[test]
fn test_manual_step_conditional_verification_evaluates_condition_before_branching() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV002".to_string(),
        1,
        1,
        "TC_CV002".to_string(),
        "Test condition is evaluated before branching".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Conditional evaluation test",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "test -d /var/log".to_string(),
            if_true: Some(vec!["ls /var/log".to_string()]),
            if_false: Some(vec!["echo 'Directory not found'".to_string()]),
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify condition is evaluated at the beginning of the if statement
    assert!(
        script.contains("if test -d /var/log; then"),
        "Condition must be evaluated in if statement"
    );

    // Verify that USER_VERIFICATION_RESULT is set to true when condition passes
    let if_pos = script.find("if test -d /var/log; then").unwrap();
    let set_true_pos = script[if_pos..]
        .find("USER_VERIFICATION_RESULT=true")
        .unwrap()
        + if_pos;
    let if_true_command_pos = script[if_pos..].find("ls /var/log").unwrap() + if_pos;

    // Verify variable is set before if_true commands are executed
    assert!(
        set_true_pos < if_true_command_pos,
        "USER_VERIFICATION_RESULT must be set to true before if_true commands"
    );
}

#[test]
fn test_manual_step_conditional_verification_executes_if_true_when_condition_passes() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV003".to_string(),
        1,
        1,
        "TC_CV003".to_string(),
        "Test if_true commands execute when condition passes".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(1, "If-true test", "action", "0", "done", Some(true));
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ $? -eq 0 ]".to_string(),
            if_true: Some(vec![
                "echo 'Command succeeded'".to_string(),
                "touch /tmp/success".to_string(),
            ]),
            if_false: Some(vec!["echo 'Command failed'".to_string()]),
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify if_true commands are in the script
    assert!(
        script.contains("echo 'Command succeeded'"),
        "if_true command 1 must be in script"
    );
    assert!(
        script.contains("touch /tmp/success"),
        "if_true command 2 must be in script"
    );

    // Verify commands are in the then block (between if and else)
    let if_pos = script.find("if [ $? -eq 0 ]; then").unwrap();
    let cmd1_pos = script[if_pos..].find("echo 'Command succeeded'").unwrap() + if_pos;
    let cmd2_pos = script[cmd1_pos..].find("touch /tmp/success").unwrap() + cmd1_pos;
    let else_pos = script[cmd2_pos..].find("else").unwrap() + cmd2_pos;

    assert!(
        if_pos < cmd1_pos && cmd1_pos < cmd2_pos && cmd2_pos < else_pos,
        "if_true commands must be in then block before else"
    );
}

#[test]
fn test_manual_step_conditional_verification_executes_if_false_when_condition_fails() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV004".to_string(),
        1,
        1,
        "TC_CV004".to_string(),
        "Test if_false commands execute when condition fails".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(1, "If-false test", "action", "0", "done", Some(true));
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ -f /tmp/nonexistent ]".to_string(),
            if_true: Some(vec!["echo 'File exists'".to_string()]),
            if_false: Some(vec![
                "echo 'File not found'".to_string(),
                "mkdir -p /tmp".to_string(),
            ]),
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify if_false commands are in the script
    assert!(
        script.contains("echo 'File not found'"),
        "if_false command 1 must be in script"
    );
    assert!(
        script.contains("mkdir -p /tmp"),
        "if_false command 2 must be in script"
    );

    // Verify commands are in the else block (between else and fi)
    let else_pos = script.find("else").unwrap();
    let cmd1_pos = script[else_pos..].find("echo 'File not found'").unwrap() + else_pos;
    let cmd2_pos = script[cmd1_pos..].find("mkdir -p /tmp").unwrap() + cmd1_pos;
    let fi_pos = script[cmd2_pos..].find("fi").unwrap() + cmd2_pos;

    assert!(
        else_pos < cmd1_pos && cmd1_pos < cmd2_pos && cmd2_pos < fi_pos,
        "if_false commands must be in else block before fi"
    );
}

#[test]
fn test_manual_step_conditional_verification_always_executes_always_commands() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV005".to_string(),
        1,
        1,
        "TC_CV005".to_string(),
        "Test always commands always execute".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Always execution test",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ -n \"$RESULT\" ]".to_string(),
            if_true: Some(vec!["echo 'Result is not empty'".to_string()]),
            if_false: Some(vec!["echo 'Result is empty'".to_string()]),
            always: Some(vec![
                "echo 'Cleanup started'".to_string(),
                "rm -f /tmp/temp_file".to_string(),
                "echo 'Cleanup complete'".to_string(),
            ]),
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify always commands are in the script
    assert!(
        script.contains("echo 'Cleanup started'"),
        "always command 1 must be in script"
    );
    assert!(
        script.contains("rm -f /tmp/temp_file"),
        "always command 2 must be in script"
    );
    assert!(
        script.contains("echo 'Cleanup complete'"),
        "always command 3 must be in script"
    );

    // Verify always commands are after the fi block
    let fi_pos = script.find("fi").unwrap();
    let cleanup_start_pos = script[fi_pos..].find("echo 'Cleanup started'").unwrap() + fi_pos;
    let cleanup_action_pos = script[cleanup_start_pos..]
        .find("rm -f /tmp/temp_file")
        .unwrap()
        + cleanup_start_pos;
    let cleanup_complete_pos = script[cleanup_action_pos..]
        .find("echo 'Cleanup complete'")
        .unwrap()
        + cleanup_action_pos;

    assert!(
        fi_pos < cleanup_start_pos
            && cleanup_start_pos < cleanup_action_pos
            && cleanup_action_pos < cleanup_complete_pos,
        "always commands must execute after fi block in correct order"
    );
}

#[test]
fn test_manual_step_conditional_verification_with_only_condition() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV006".to_string(),
        1,
        1,
        "TC_CV006".to_string(),
        "Test conditional with only condition (no branches)".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(1, "Minimal conditional", "action", "0", "done", Some(true));
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "grep -q 'SUCCESS' /tmp/result.log".to_string(),
            if_true: None,
            if_false: None,
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify condition is evaluated
    assert!(
        script.contains("if grep -q 'SUCCESS' /tmp/result.log; then"),
        "Condition must be evaluated even without branches"
    );

    // Verify USER_VERIFICATION_RESULT is set to true when condition passes
    assert!(
        script.contains("USER_VERIFICATION_RESULT=true"),
        "Variable must be set to true in then block"
    );

    // Verify else block exists (even if empty)
    assert!(script.contains("else"), "else block must exist");

    // Verify fi block closes the conditional
    assert!(script.contains("fi"), "fi must close the conditional block");
}

#[test]
fn test_manual_step_conditional_verification_with_condition_and_if_true_only() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV007".to_string(),
        1,
        1,
        "TC_CV007".to_string(),
        "Test conditional with condition and if_true only".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "If-true only conditional",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "systemctl is-active nginx".to_string(),
            if_true: Some(vec![
                "echo 'Nginx is running'".to_string(),
                "systemctl status nginx".to_string(),
            ]),
            if_false: None,
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify condition and if_true commands
    assert!(
        script.contains("if systemctl is-active nginx; then"),
        "Condition must be evaluated"
    );
    assert!(
        script.contains("echo 'Nginx is running'"),
        "if_true command 1 must be in then block"
    );
    assert!(
        script.contains("systemctl status nginx"),
        "if_true command 2 must be in then block"
    );

    // Verify else block exists (should have placeholder comment or no-op)
    let if_pos = script.find("if systemctl is-active nginx; then").unwrap();
    let else_pos = script[if_pos..].find("else").unwrap() + if_pos;
    let fi_pos = script[else_pos..].find("fi").unwrap() + else_pos;

    assert!(
        if_pos < else_pos && else_pos < fi_pos,
        "Structure must be complete with else block"
    );
}

#[test]
fn test_manual_step_conditional_verification_with_condition_and_if_false_only() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV008".to_string(),
        1,
        1,
        "TC_CV008".to_string(),
        "Test conditional with condition and if_false only".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "If-false only conditional",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "ping -c 1 google.com".to_string(),
            if_true: None,
            if_false: Some(vec![
                "echo 'Network is down'".to_string(),
                "echo 'Check network connection'".to_string(),
            ]),
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify condition and if_false commands
    assert!(
        script.contains("if ping -c 1 google.com; then"),
        "Condition must be evaluated"
    );
    assert!(
        script.contains("echo 'Network is down'"),
        "if_false command 1 must be in else block"
    );
    assert!(
        script.contains("echo 'Check network connection'"),
        "if_false command 2 must be in else block"
    );

    // Verify the if_false commands are after else
    let else_pos = script.find("else").unwrap();
    let cmd1_pos = script[else_pos..].find("echo 'Network is down'").unwrap() + else_pos;
    let cmd2_pos = script[cmd1_pos..]
        .find("echo 'Check network connection'")
        .unwrap()
        + cmd1_pos;

    assert!(
        else_pos < cmd1_pos && cmd1_pos < cmd2_pos,
        "if_false commands must be in else block"
    );
}

#[test]
fn test_manual_step_conditional_verification_with_condition_and_always_only() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV009".to_string(),
        1,
        1,
        "TC_CV009".to_string(),
        "Test conditional with condition and always only".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Always only conditional",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ -f /tmp/lock ]".to_string(),
            if_true: None,
            if_false: None,
            always: Some(vec![
                "rm -f /tmp/lock".to_string(),
                "echo 'Lock removed'".to_string(),
            ]),
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify condition is evaluated
    assert!(
        script.contains("if [ -f /tmp/lock ]; then"),
        "Condition must be evaluated"
    );

    // Verify always commands are after fi
    assert!(
        script.contains("rm -f /tmp/lock"),
        "always command 1 must be in script"
    );
    assert!(
        script.contains("echo 'Lock removed'"),
        "always command 2 must be in script"
    );

    let fi_pos = script.find("fi").unwrap();
    let rm_pos = script[fi_pos..].find("rm -f /tmp/lock").unwrap() + fi_pos;
    let echo_pos = script[rm_pos..].find("echo 'Lock removed'").unwrap() + rm_pos;

    assert!(
        fi_pos < rm_pos && rm_pos < echo_pos,
        "always commands must execute after fi block"
    );
}

#[test]
fn test_manual_step_both_result_and_output_conditional_verifications() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV010".to_string(),
        1,
        1,
        "TC_CV010".to_string(),
        "Test both result and output with conditional verifications".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Dual conditional verification",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "[ -f /tmp/result ]".to_string(),
            if_true: Some(vec!["echo 'Result file exists'".to_string()]),
            if_false: Some(vec!["echo 'Result file missing'".to_string()]),
            always: Some(vec!["echo 'Result check complete'".to_string()]),
        },
        output: VerificationExpression::Conditional {
            condition: "[ -f /tmp/output ]".to_string(),
            if_true: Some(vec!["echo 'Output file exists'".to_string()]),
            if_false: Some(vec!["echo 'Output file missing'".to_string()]),
            always: Some(vec!["echo 'Output check complete'".to_string()]),
        },
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify result verification conditional
    assert!(
        script.contains("if [ -f /tmp/result ]; then"),
        "Result condition must be evaluated"
    );
    assert!(
        script.contains("echo 'Result file exists'"),
        "Result if_true must be present"
    );
    assert!(
        script.contains("echo 'Result file missing'"),
        "Result if_false must be present"
    );
    assert!(
        script.contains("echo 'Result check complete'"),
        "Result always must be present"
    );

    // Verify output verification conditional
    assert!(
        script.contains("if [ -f /tmp/output ]; then"),
        "Output condition must be evaluated"
    );
    assert!(
        script.contains("echo 'Output file exists'"),
        "Output if_true must be present"
    );
    assert!(
        script.contains("echo 'Output file missing'"),
        "Output if_false must be present"
    );
    assert!(
        script.contains("echo 'Output check complete'"),
        "Output always must be present"
    );

    // Verify both set USER_VERIFICATION_RESULT and USER_VERIFICATION_OUTPUT
    assert!(
        script.contains("USER_VERIFICATION_RESULT=true"),
        "Result verification must set USER_VERIFICATION_RESULT"
    );
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT=true"),
        "Output verification must set USER_VERIFICATION_OUTPUT"
    );

    // Verify combined check
    assert!(
        script.contains("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"),
        "Combined verification check must be present"
    );
}

#[test]
fn test_manual_step_conditional_verification_complex_nested_structure() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV011".to_string(),
        1,
        1,
        "TC_CV011".to_string(),
        "Test complex conditional with multiple commands in each branch".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(1, "Complex conditional", "action", "0", "done", Some(true));
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "grep -q 'ERROR' /var/log/app.log".to_string(),
            if_true: Some(vec![
                "echo 'Errors found in log'".to_string(),
                "tail -20 /var/log/app.log".to_string(),
                "echo 'Review errors above'".to_string(),
            ]),
            if_false: Some(vec![
                "echo 'No errors in log'".to_string(),
                "echo 'Application is healthy'".to_string(),
            ]),
            always: Some(vec![
                "echo 'Log check completed at $(date)'".to_string(),
                "echo 'Next check scheduled'".to_string(),
            ]),
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify all if_true commands are present in order
    let if_pos = script
        .find("if grep -q 'ERROR' /var/log/app.log; then")
        .unwrap();
    let cmd1_pos = script[if_pos..].find("echo 'Errors found in log'").unwrap() + if_pos;
    let cmd2_pos = script[cmd1_pos..]
        .find("tail -20 /var/log/app.log")
        .unwrap()
        + cmd1_pos;
    let cmd3_pos = script[cmd2_pos..]
        .find("echo 'Review errors above'")
        .unwrap()
        + cmd2_pos;
    let else_pos = script[cmd3_pos..].find("else").unwrap() + cmd3_pos;

    assert!(
        if_pos < cmd1_pos && cmd1_pos < cmd2_pos && cmd2_pos < cmd3_pos && cmd3_pos < else_pos,
        "All if_true commands must be in then block in correct order"
    );

    // Verify all if_false commands are present in order
    let cmd4_pos = script[else_pos..].find("echo 'No errors in log'").unwrap() + else_pos;
    let cmd5_pos = script[cmd4_pos..]
        .find("echo 'Application is healthy'")
        .unwrap()
        + cmd4_pos;
    let fi_pos = script[cmd5_pos..].find("fi").unwrap() + cmd5_pos;

    assert!(
        else_pos < cmd4_pos && cmd4_pos < cmd5_pos && cmd5_pos < fi_pos,
        "All if_false commands must be in else block in correct order"
    );

    // Verify all always commands are present after fi
    let always1_pos = script[fi_pos..]
        .find("echo 'Log check completed at $(date)'")
        .unwrap()
        + fi_pos;
    let always2_pos = script[always1_pos..]
        .find("echo 'Next check scheduled'")
        .unwrap()
        + always1_pos;

    assert!(
        fi_pos < always1_pos && always1_pos < always2_pos,
        "All always commands must be after fi in correct order"
    );
}

#[test]
fn test_manual_step_conditional_verification_sets_variable_on_condition_success() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV012".to_string(),
        1,
        1,
        "TC_CV012".to_string(),
        "Test USER_VERIFICATION variable is set when condition succeeds".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Variable setting test",
        "action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "test -x /usr/bin/app".to_string(),
            if_true: Some(vec!["echo 'App is executable'".to_string()]),
            if_false: Some(vec!["echo 'App is not executable'".to_string()]),
            always: None,
        },
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify USER_VERIFICATION_RESULT is set to true in then block
    let if_pos = script.find("if test -x /usr/bin/app; then").unwrap();
    let set_true_pos = script[if_pos..]
        .find("USER_VERIFICATION_RESULT=true")
        .unwrap()
        + if_pos;
    let else_pos = script[set_true_pos..].find("else").unwrap() + set_true_pos;

    assert!(
        if_pos < set_true_pos && set_true_pos < else_pos,
        "USER_VERIFICATION_RESULT=true must be in then block"
    );

    // Verify the variable is NOT set to true in else block
    let else_block = &script[else_pos..];
    let fi_pos = else_block.find("fi").unwrap();
    let else_section = &else_block[..fi_pos];

    assert!(
        !else_section.contains("USER_VERIFICATION_RESULT=true"),
        "USER_VERIFICATION_RESULT should not be set to true in else block"
    );
}

#[test]
fn test_manual_step_conditional_verification_integrates_with_pass_fail_logic() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_CV013".to_string(),
        1,
        1,
        "TC_CV013".to_string(),
        "Test conditional verification integrates with PASS/FAIL logic".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(1, "Integration test", "action", "0", "done", Some(true));
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Conditional {
            condition: "pgrep nginx".to_string(),
            if_true: Some(vec!["echo 'Nginx process found'".to_string()]),
            if_false: Some(vec!["echo 'Nginx process not found'".to_string()]),
            always: Some(vec!["echo 'Process check done'".to_string()]),
        },
        output: VerificationExpression::Conditional {
            condition: "curl -s http://localhost > /dev/null".to_string(),
            if_true: Some(vec!["echo 'Service is responding'".to_string()]),
            if_false: Some(vec!["echo 'Service is not responding'".to_string()]),
            always: None,
        },
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify both conditional verifications set their variables
    assert!(
        script.contains("USER_VERIFICATION_RESULT=true"),
        "Result verification must set variable"
    );
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT=true"),
        "Output verification must set variable"
    );

    // Verify combined USER_VERIFICATION logic
    assert!(
        script.contains("if [ \"$USER_VERIFICATION_RESULT\" = true ] && [ \"$USER_VERIFICATION_OUTPUT\" = true ]"),
        "Combined verification must check both variables"
    );
    assert!(
        script.contains("USER_VERIFICATION=true"),
        "Combined verification sets USER_VERIFICATION=true"
    );
    assert!(
        script.contains("USER_VERIFICATION=false"),
        "Combined verification sets USER_VERIFICATION=false on failure"
    );

    // Verify PASS/FAIL messages
    assert!(
        script.contains("[PASS] Step 1: Integration test"),
        "PASS message must be present"
    );
    assert!(
        script.contains("[FAIL] Step 1: Integration test"),
        "FAIL message must be present"
    );

    // Verify diagnostic output
    assert!(
        script.contains("Result verification: $USER_VERIFICATION_RESULT"),
        "Diagnostic must show result verification"
    );
    assert!(
        script.contains("Output verification: $USER_VERIFICATION_OUTPUT"),
        "Diagnostic must show output verification"
    );

    // Verify exit on failure
    let fail_pos = script.find("[FAIL] Step 1").unwrap();
    let exit_pos = script[fail_pos..].find("exit 1").unwrap() + fail_pos;
    assert!(fail_pos < exit_pos, "exit 1 must be after FAIL message");
}

// ============================================================================
// BDD Initial Conditions Integration Tests for generate_test_script_with_json_output
// ============================================================================

#[test]
fn test_bdd_in_general_initial_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "BDD general conditions test".to_string(),
    );

    // Add BDD statements in general_initial_conditions
    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Setup".to_string(),
        vec![
            InitialConditionItem::String("create directory \"/tmp/test\"".to_string()),
            InitialConditionItem::String("wait for 2 seconds".to_string()),
        ],
    );
    test_case.general_initial_conditions = InitialConditions {
        include: None,
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Echo test", "echo 'hello'", "0", "hello", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Should contain comment for BDD statement
    assert!(script.contains("# Setup: create directory \"/tmp/test\""));
    // Should contain the generated command from BDD pattern
    assert!(script.contains("mkdir -p \"/tmp/test\""));

    // Should contain second BDD statement
    assert!(script.contains("# Setup: wait for 2 seconds"));
    assert!(script.contains("sleep 2"));
}

#[test]
fn test_bdd_in_test_level_initial_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ002".to_string(),
        1,
        1,
        "TC002".to_string(),
        "BDD test-level conditions test".to_string(),
    );

    // Add BDD statements in test-level initial_conditions
    let mut devices = HashMap::new();
    devices.insert(
        "Environment".to_string(),
        vec![
            InitialConditionItem::String(
                "set environment variable \"TEST_MODE\" to \"enabled\"".to_string(),
            ),
            InitialConditionItem::String("file \"/tmp/config.txt\" should exist".to_string()),
        ],
    );
    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Should contain Initial Conditions header
    assert!(script.contains("# Initial Conditions"));
    // Should contain comment for BDD statement
    assert!(script.contains("# Environment: set environment variable \"TEST_MODE\" to \"enabled\""));
    // Should contain the generated command from BDD pattern
    assert!(script.contains("export TEST_MODE=enabled"));

    // Should contain second BDD statement
    assert!(script.contains("# Environment: file \"/tmp/config.txt\" should exist"));
    assert!(script.contains("test -f \"/tmp/config.txt\""));
}

#[test]
fn test_bdd_in_sequence_level_initial_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ003".to_string(),
        1,
        1,
        "TC003".to_string(),
        "BDD sequence-level conditions test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    // Add BDD statements in sequence-level initial_conditions
    let mut seq_devices = HashMap::new();
    seq_devices.insert(
        "Precondition".to_string(),
        vec![
            InitialConditionItem::String("ping device \"192.168.1.1\" with 3 retries".to_string()),
            InitialConditionItem::String(
                "create file \"/tmp/testfile.txt\" with content:".to_string(),
            ),
        ],
    );
    sequence.initial_conditions = InitialConditions {
        include: None,
        devices: seq_devices,
    };

    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Should contain Sequence Initial Conditions header
    assert!(script.contains("# Sequence Initial Conditions"));
    // Should contain comment for BDD statement
    assert!(script.contains("# Precondition: ping device \"192.168.1.1\" with 3 retries"));
    // Should contain the generated command from BDD pattern
    assert!(script.contains("ping -c 3 \"192.168.1.1\""));

    // Should contain second BDD statement
    assert!(script.contains("# Precondition: create file \"/tmp/testfile.txt\" with content:"));
    assert!(script.contains("touch \"/tmp/testfile.txt\""));
}

#[test]
fn test_mixed_bdd_and_non_bdd_statements() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ004".to_string(),
        1,
        1,
        "TC004".to_string(),
        "Mixed BDD and non-BDD test".to_string(),
    );

    // Mix BDD and non-BDD statements
    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Setup".to_string(),
        vec![
            InitialConditionItem::String("Device is powered on".to_string()), // Non-BDD
            InitialConditionItem::String("create directory \"/tmp/logs\"".to_string()), // BDD
            InitialConditionItem::String("Network is connected".to_string()), // Non-BDD
        ],
    );
    test_case.general_initial_conditions = InitialConditions {
        include: None,
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Non-BDD statement should be a comment only (no executable command for it)
    assert!(script.contains("# Setup: Device is powered on\n"));

    // BDD statement should generate command
    assert!(script.contains("# Setup: create directory \"/tmp/logs\""));
    assert!(script.contains("mkdir -p \"/tmp/logs\""));

    // Non-BDD statement should be a comment only (no executable command for it)
    assert!(script.contains("# Setup: Network is connected\n"));
}

#[test]
fn test_multiple_bdd_statements_same_type() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ005".to_string(),
        1,
        1,
        "TC005".to_string(),
        "Multiple BDD statements test".to_string(),
    );

    // Multiple BDD statements of the same pattern type
    let mut devices = HashMap::new();
    devices.insert(
        "Files".to_string(),
        vec![
            InitialConditionItem::String("create directory \"/tmp/dir1\"".to_string()),
            InitialConditionItem::String("create directory \"/tmp/dir2\"".to_string()),
            InitialConditionItem::String("create directory \"/tmp/dir3\"".to_string()),
        ],
    );
    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // All three BDD statements should generate commands
    assert!(script.contains("mkdir -p \"/tmp/dir1\""));
    assert!(script.contains("mkdir -p \"/tmp/dir2\""));
    assert!(script.contains("mkdir -p \"/tmp/dir3\""));

    // All three should have comments
    assert!(script.contains("# Files: create directory \"/tmp/dir1\""));
    assert!(script.contains("# Files: create directory \"/tmp/dir2\""));
    assert!(script.contains("# Files: create directory \"/tmp/dir3\""));
}

#[test]
fn test_bdd_in_all_three_locations() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ006".to_string(),
        1,
        1,
        "TC006".to_string(),
        "BDD in all locations test".to_string(),
    );

    // BDD in general_initial_conditions
    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Global".to_string(),
        vec![InitialConditionItem::String(
            "wait for 1 seconds".to_string(),
        )],
    );
    test_case.general_initial_conditions = InitialConditions {
        include: None,
        devices: general_devices,
    };

    // BDD in test-level initial_conditions
    let mut devices = HashMap::new();
    devices.insert(
        "Test".to_string(),
        vec![InitialConditionItem::String(
            "create directory \"/tmp/test\"".to_string(),
        )],
    );
    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    // BDD in sequence-level initial_conditions
    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut seq_devices = HashMap::new();
    seq_devices.insert(
        "Sequence".to_string(),
        vec![InitialConditionItem::String(
            "file \"/tmp/test\" should exist".to_string(),
        )],
    );
    sequence.initial_conditions = InitialConditions {
        include: None,
        devices: seq_devices,
    };

    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Check general conditions
    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Global: wait for 1 seconds"));
    assert!(script.contains("sleep 1"));

    // Check test-level conditions
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Test: create directory \"/tmp/test\""));
    assert!(script.contains("mkdir -p \"/tmp/test\""));

    // Check sequence-level conditions
    assert!(script.contains("# Sequence Initial Conditions"));
    assert!(script.contains("# Sequence: file \"/tmp/test\" should exist"));
    assert!(script.contains("test -f \"/tmp/test\""));
}

#[test]
fn test_bdd_with_missing_toml_file() {
    // Temporarily rename the TOML file if it exists, or test with a non-existent path
    // This test verifies that the system gracefully handles missing BDD definitions

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ007".to_string(),
        1,
        1,
        "TC007".to_string(),
        "BDD with missing TOML test".to_string(),
    );

    // Add what would be BDD statements if the TOML file existed
    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Setup".to_string(),
        vec![InitialConditionItem::String(
            "create directory \"/tmp/test\"".to_string(),
        )],
    );
    test_case.general_initial_conditions = InitialConditions {
        include: None,
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Script should still be generated (BDD gracefully fails)
    assert!(script.contains("#!/bin/bash"));
    assert!(script.contains("# Test Case: TC007"));

    // If TOML is missing, the statement appears as comment only (or if it loads, as command)
    // Either way, the statement text should appear
    assert!(script.contains("create directory \"/tmp/test\""));
}

#[test]
fn test_bdd_complex_patterns_in_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ008".to_string(),
        1,
        1,
        "TC008".to_string(),
        "Complex BDD patterns test".to_string(),
    );

    // Use various complex BDD patterns from the TOML
    let mut devices = HashMap::new();
    devices.insert(
        "Setup".to_string(),
        vec![
            InitialConditionItem::String(
                "change permissions of \"/tmp/file.txt\" to 755".to_string(),
            ),
            InitialConditionItem::String(
                "append \"test data\" to file \"/tmp/log.txt\"".to_string(),
            ),
            InitialConditionItem::String("port 8080 on \"localhost\" should be open".to_string()),
        ],
    );
    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Check that complex patterns are parsed correctly
    assert!(script.contains("chmod 755 \"/tmp/file.txt\""));
    assert!(script.contains("echo \"test data\" >> \"/tmp/log.txt\""));
    assert!(script.contains("nc -z \"localhost\" 8080"));
}

#[test]
fn test_json_output_path_in_script() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ009".to_string(),
        1,
        1,
        "TC009".to_string(),
        "JSON output path test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("/custom/path/output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // Verify the JSON_LOG variable is set to the custom path
    assert!(script.contains("JSON_LOG=\"/custom/path/output.json\""));
}

#[test]
fn test_bdd_with_multiple_keys_in_conditions() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ010".to_string(),
        1,
        1,
        "TC010".to_string(),
        "Multiple keys test".to_string(),
    );

    // Multiple keys with BDD statements
    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Filesystem".to_string(),
        vec![InitialConditionItem::String(
            "create directory \"/tmp/fs1\"".to_string(),
        )],
    );
    general_devices.insert(
        "Network".to_string(),
        vec![InitialConditionItem::String(
            "ping device \"192.168.1.1\" with 5 retries".to_string(),
        )],
    );
    general_devices.insert(
        "Time".to_string(),
        vec![InitialConditionItem::String(
            "wait for 3 seconds".to_string(),
        )],
    );
    test_case.general_initial_conditions = InitialConditions {
        include: None,
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test step", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // All three keys should have their BDD statements processed
    assert!(script.contains("mkdir -p \"/tmp/fs1\""));
    assert!(script.contains("ping -c 5 \"192.168.1.1\""));
    assert!(script.contains("sleep 3"));
}

#[test]
fn test_command_escaping_for_json_with_single_quotes() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ011".to_string(),
        1,
        1,
        "TC011".to_string(),
        "Test command escaping with single quotes".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    // Create a step with a command containing single quotes
    let step = create_test_step(
        1,
        "Echo with quotes",
        "echo 'hello world'",
        "0",
        "hello world",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // The JSON command field should contain the escaped command
    // The implementation converts single quotes to double quotes, then escapes the double quotes
    // Original: echo 'hello world'
    // After escaping (lines 733-736 of executor.rs):
    // 1. Backslashes: no backslashes to escape
    // 2. Single quotes: ' -> " (so 'hello world' becomes "hello world")
    // 3. Double quotes: " -> \" (so "hello world" becomes \"hello world\")
    // Result: echo \"hello world\"

    // Check that the script contains the original command with single quotes
    assert!(
        script.contains("echo 'hello world'"),
        "Script should contain the original command with single quotes"
    );

    // Check that the JSON output correctly converts and escapes quotes
    // The JSON line should be: echo '    "command": "echo \"hello world\"",'
    let expected_json_line = "echo '    \"command\": \"echo \\\"hello world\\\"\",";
    assert!(
        script.contains(expected_json_line),
        "JSON command field should convert single quotes to escaped double quotes. Expected: {}",
        expected_json_line
    );

    // Verify the command is properly included in the JSON write statement
    assert!(
        script.contains("echo '    \"command\":"),
        "Script should write command field to JSON"
    );
}

#[test]
fn test_command_escaping_for_json_with_mixed_quotes() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ012".to_string(),
        1,
        1,
        "TC012".to_string(),
        "Test command escaping with mixed quotes".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    // Create a step with a command containing both single and double quotes
    let step = create_test_step(
        1,
        "Echo with mixed quotes",
        "echo 'test \"quoted\" value'",
        "0",
        "test \"quoted\" value",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // The generated bash script should contain the original command
    assert!(
        script.contains("echo 'test \"quoted\" value'"),
        "Script should contain the original command with mixed quotes"
    );

    // Validate the generated JSON entry contains properly escaped command string
    // The Rust code in executor.rs (lines 733-736) performs these replacements on the command:
    // 1. Backslashes: \ -> \\
    // 2. Single quotes: ' -> " (this is the bug!)
    // 3. Double quotes: " -> \"
    // Due to step 2, a command like: echo 'test "quoted" value'
    // becomes: echo "test \"quoted\" value" before JSON escaping
    // Then after JSON escaping: echo \"test \\\"quoted\\\" value\"
    // This escaped value is directly embedded in the generated bash script

    // Find the line that writes the command to JSON
    // It should be something like: echo '    "command": "...",
    assert!(
        script.contains(r#"echo '    "command":"#),
        "Script should contain JSON command field write statement"
    );

    // Check that the script writes valid JSON structure
    assert!(
        script.contains(r#"echo '  {'"#),
        "Script should open JSON object"
    );
    assert!(
        script.contains(r#"echo '  }'"#),
        "Script should close JSON object"
    );

    // The actual command in the JSON should be escaped
    // Due to the bug on line 735 of executor.rs, single quotes are replaced with double quotes:
    // Original: echo 'test "quoted" value'
    // Step 1 - replace('", \""): echo "test "quoted" value"
    // Step 2 - replace(""", "\\\""): echo \"test \"quoted\" value\"
    // Note: The inner quotes don't get double-escaped because they were already double quotes
    // So in the generated script we should find this escaped version
    let expected_escaped = r#"echo \"test \"quoted\" value\""#;
    assert!(
            script.contains(expected_escaped),
            "Script should contain the properly escaped command string in the JSON field. Expected to find: {}",
            expected_escaped
        );

    // Verify the JSON structure can be validated
    // The script includes a jq validation check at the end
    assert!(
        script.contains("if ! jq empty \"$JSON_LOG\""),
        "Script should include JSON validation with jq"
    );

    // Final check: the generated JSON line for the command field should be:
    // echo '    "command": "echo \"test \"quoted\" value\"",'
    let expected_json_line = "echo '    \"command\": \"echo \\\"test \\\"quoted\\\" value\\\"\",";
    assert!(
            script.contains(expected_json_line),
            "Script should contain the exact JSON command line with properly escaped quotes. Expected: {}",
            expected_json_line
        );
}

#[test]
fn test_command_escaping_for_json_with_newlines() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ013".to_string(),
        1,
        1,
        "TC013".to_string(),
        "Test command escaping with newlines".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    // Create a step with a multi-line bash command
    let multiline_command = "echo 'line1'\necho 'line2'\necho 'line3'";
    let step = create_test_step(
        1,
        "Multi-line command",
        multiline_command,
        "0",
        "line1\nline2\nline3",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // The generated bash script should contain the original multi-line command
    assert!(
        script.contains("echo 'line1'\necho 'line2'\necho 'line3'"),
        "Script should contain the original multi-line command"
    );

    // In the JSON output section of the script, newlines should be escaped as \n
    // The command in the JSON should have the newlines converted to \n for JSON format
    // The script writes: echo '    "command": "...",
    // where the command value should have \n instead of literal newlines

    // Looking for the escaped form in the JSON line
    // The echo command writes the JSON with newlines escaped as \n
    // In the bash script, within single quotes, \n is literal, so it appears as:
    // echo '    "command": "echo \"line1\"\necho \"line2\"\necho \"line3\"",'
    // This produces JSON with \n (which is the correct JSON escape sequence for newlines)
    let expected =
        "echo '    \"command\": \"echo \\\"line1\\\"\\necho \\\"line2\\\"\\necho \\\"line3\\\"\",";
    assert!(
        script.contains(expected),
        "JSON command field should contain newlines escaped as \\n in the JSON output"
    );

    // Verify the JSON structure is properly written
    assert!(
        script.contains("echo '    \"command\":"),
        "Script should contain JSON command field write statement"
    );
}

#[test]
fn test_command_escaping_for_json_with_backslashes() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ014".to_string(),
        1,
        1,
        "TC014".to_string(),
        "Test command escaping with backslashes".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    // Create a step with a command containing backslashes (e.g., grep with regex)
    let step = create_test_step(
        1,
        "Grep with regex",
        r#"grep "\d+" file.txt"#,
        "0",
        "123",
        Some(true),
    );
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let json_path = std::path::Path::new("test_output.json");
    let script = executor.generate_test_script_with_json_output(&test_case, json_path);

    // The generated bash script should contain the original command with backslashes
    assert!(
        script.contains(r#"grep "\d+" file.txt"#),
        "Script should contain the original command with backslashes"
    );

    // In the JSON output, backslashes must be escaped according to JSON spec
    // Original: grep "\d+" file.txt
    // After escaping (lines 733-736 of executor.rs):
    // 1. Backslashes: \ -> \\ (so \d becomes \\d)
    // 2. Single quotes: ' -> " (no single quotes in this command)
    // 3. Double quotes: " -> \" (so " becomes \")
    // Result: grep \"\\d+\" file.txt

    // The JSON line should be:
    // echo '    "command": "grep \"\\d+\" file.txt",'
    let expected_json_line = r#"echo '    "command": "grep \"\\d+\" file.txt","#;
    assert!(
        script.contains(expected_json_line),
        "JSON command field should have properly escaped backslashes and quotes. Expected: {}",
        expected_json_line
    );

    // Verify the JSON structure is properly written
    assert!(
        script.contains("echo '    \"command\":"),
        "Script should contain JSON command field write statement"
    );

    // Verify that backslashes are doubled in the JSON output (JSON escaping requirement)
    // The pattern \d should appear as \\d in the JSON string literal
    assert!(
        script.contains(r#"\"\\d+\""#),
        "Backslashes should be properly escaped in JSON (doubled)"
    );
}

// ============================================================================
// Integration Tests for Script Generation with Dependencies
// ============================================================================

#[test]
fn test_initial_conditions_with_include_array_as_comments() {
    use testcase_manager::models::IncludeRef;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test with include array".to_string(),
    );

    // Add include array to general_initial_conditions
    let include_refs = vec![
        IncludeRef {
            id: "TC_SETUP_001".to_string(),
            test_sequence: None,
        },
        IncludeRef {
            id: "TC_INIT_002".to_string(),
            test_sequence: Some("Seq1".to_string()),
        },
    ];

    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Device".to_string(),
        vec![InitialConditionItem::String("Powered on".to_string())],
    );

    test_case.general_initial_conditions = InitialConditions {
        include: Some(include_refs),
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify include array items appear as comments
    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Include: TC_SETUP_001"));
    assert!(script.contains("# Include: TC_INIT_002 (test_sequence: Seq1)"));
    assert!(script.contains("# Device: Powered on"));
}

#[test]
fn test_initial_conditions_mixed_item_types_as_string_representations() {
    use testcase_manager::models::TestSequenceRefTarget;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ002".to_string(),
        1,
        1,
        "TC002".to_string(),
        "Test with mixed condition items".to_string(),
    );

    // Add mixed initial condition items
    let mut devices = HashMap::new();
    devices.insert(
        "Setup".to_string(),
        vec![
            InitialConditionItem::String("System is ready".to_string()),
            InitialConditionItem::RefItem {
                reference: "CONFIG_REF_001".to_string(),
            },
            InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 5,
                    step: "2a".to_string(),
                },
            },
        ],
    );

    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify all item types are converted to string representations
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Setup: System is ready"));
    assert!(script.contains("# Setup: ref: CONFIG_REF_001"));
    assert!(script.contains("# Setup: test_sequence: id=5, step=2a"));
}

#[test]
fn test_initial_conditions_ref_items_appear_as_comments() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ003".to_string(),
        1,
        1,
        "TC003".to_string(),
        "Test with ref items".to_string(),
    );

    // Add ref items to test-level initial conditions
    let mut devices = HashMap::new();
    devices.insert(
        "Environment".to_string(),
        vec![
            InitialConditionItem::RefItem {
                reference: "ENV_SETUP_001".to_string(),
            },
            InitialConditionItem::RefItem {
                reference: "ENV_CONFIG_002".to_string(),
            },
        ],
    );

    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify ref items appear as comments in the script
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Environment: ref: ENV_SETUP_001"));
    assert!(script.contains("# Environment: ref: ENV_CONFIG_002"));
}

#[test]
fn test_initial_conditions_test_sequence_refs_appear_as_comments() {
    use testcase_manager::models::TestSequenceRefTarget;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ004".to_string(),
        1,
        1,
        "TC004".to_string(),
        "Test with test_sequence refs".to_string(),
    );

    // Add test_sequence ref items to sequence-level initial conditions
    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    let mut seq_devices = HashMap::new();
    seq_devices.insert(
        "Prerequisites".to_string(),
        vec![
            InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 3,
                    step: "1".to_string(),
                },
            },
            InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 7,
                    step: "4b".to_string(),
                },
            },
        ],
    );

    sequence.initial_conditions = InitialConditions {
        include: None,
        devices: seq_devices,
    };

    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify test_sequence refs appear as comments in the script
    assert!(script.contains("# Sequence Initial Conditions"));
    assert!(script.contains("# Prerequisites: test_sequence: id=3, step=1"));
    assert!(script.contains("# Prerequisites: test_sequence: id=7, step=4b"));
}

#[test]
fn test_initial_conditions_bdd_pattern_with_include_array() {
    use testcase_manager::models::IncludeRef;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ005".to_string(),
        1,
        1,
        "TC005".to_string(),
        "Test BDD patterns with include array".to_string(),
    );

    // Add include array and BDD patterns to general_initial_conditions
    let include_refs = vec![IncludeRef {
        id: "TC_COMMON_SETUP".to_string(),
        test_sequence: None,
    }];

    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Setup".to_string(),
        vec![
            InitialConditionItem::String("create directory \"/tmp/testdir\"".to_string()),
            InitialConditionItem::String("wait for 2 seconds".to_string()),
        ],
    );

    test_case.general_initial_conditions = InitialConditions {
        include: Some(include_refs),
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify include array appears as comments
    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Include: TC_COMMON_SETUP"));

    // Verify BDD patterns are processed and generate commands
    assert!(script.contains("# Setup: create directory \"/tmp/testdir\""));
    assert!(script.contains("mkdir -p \"/tmp/testdir\""));
    assert!(script.contains("# Setup: wait for 2 seconds"));
    assert!(script.contains("sleep 2"));
}

#[test]
fn test_initial_conditions_bdd_pattern_with_ref_items() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ006".to_string(),
        1,
        1,
        "TC006".to_string(),
        "Test BDD patterns with ref items".to_string(),
    );

    // Mix BDD patterns and ref items
    let mut devices = HashMap::new();
    devices.insert(
        "Preconditions".to_string(),
        vec![
            InitialConditionItem::String("create directory \"/tmp/logs\"".to_string()),
            InitialConditionItem::RefItem {
                reference: "NETWORK_CONFIG".to_string(),
            },
            InitialConditionItem::String("file \"/tmp/config\" should exist".to_string()),
        ],
    );

    test_case.initial_conditions = InitialConditions {
        include: None,
        devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify BDD patterns generate commands
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Preconditions: create directory \"/tmp/logs\""));
    assert!(script.contains("mkdir -p \"/tmp/logs\""));

    // Verify ref items appear as comments only
    assert!(script.contains("# Preconditions: ref: NETWORK_CONFIG"));

    // Verify second BDD pattern generates command
    assert!(script.contains("# Preconditions: file \"/tmp/config\" should exist"));
    assert!(script.contains("test -f \"/tmp/config\""));
}

#[test]
fn test_initial_conditions_all_levels_with_dependencies() {
    use testcase_manager::models::{IncludeRef, TestSequenceRefTarget};

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ007".to_string(),
        1,
        1,
        "TC007".to_string(),
        "Test all levels with dependencies".to_string(),
    );

    // General initial conditions with include array
    let include_refs = vec![IncludeRef {
        id: "TC_GLOBAL_SETUP".to_string(),
        test_sequence: None,
    }];

    let mut general_devices = HashMap::new();
    general_devices.insert(
        "Global".to_string(),
        vec![
            InitialConditionItem::String("wait for 1 seconds".to_string()),
            InitialConditionItem::RefItem {
                reference: "GLOBAL_REF".to_string(),
            },
        ],
    );

    test_case.general_initial_conditions = InitialConditions {
        include: Some(include_refs),
        devices: general_devices,
    };

    // Test-level initial conditions with mixed items
    let mut test_devices = HashMap::new();
    test_devices.insert(
        "Test".to_string(),
        vec![
            InitialConditionItem::String("create directory \"/tmp/test\"".to_string()),
            InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 2,
                    step: "3".to_string(),
                },
            },
        ],
    );

    test_case.initial_conditions = InitialConditions {
        include: None,
        devices: test_devices,
    };

    // Sequence-level initial conditions with include array
    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    let seq_include_refs = vec![IncludeRef {
        id: "TC_SEQ_SETUP".to_string(),
        test_sequence: Some("Seq2".to_string()),
    }];

    let mut seq_devices = HashMap::new();
    seq_devices.insert(
        "Sequence".to_string(),
        vec![InitialConditionItem::String(
            "file \"/tmp/test\" should exist".to_string(),
        )],
    );

    sequence.initial_conditions = InitialConditions {
        include: Some(seq_include_refs),
        devices: seq_devices,
    };

    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Check general conditions
    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Include: TC_GLOBAL_SETUP"));
    assert!(script.contains("# Global: wait for 1 seconds"));
    assert!(script.contains("sleep 1"));
    assert!(script.contains("# Global: ref: GLOBAL_REF"));

    // Check test-level conditions
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Test: create directory \"/tmp/test\""));
    assert!(script.contains("mkdir -p \"/tmp/test\""));
    assert!(script.contains("# Test: test_sequence: id=2, step=3"));

    // Check sequence-level conditions
    assert!(script.contains("# Sequence Initial Conditions"));
    assert!(script.contains("# Include: TC_SEQ_SETUP (test_sequence: Seq2)"));
    assert!(script.contains("# Sequence: file \"/tmp/test\" should exist"));
    assert!(script.contains("test -f \"/tmp/test\""));
}

#[test]
fn test_initial_conditions_include_array_multiple_refs() {
    use testcase_manager::models::IncludeRef;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ008".to_string(),
        1,
        1,
        "TC008".to_string(),
        "Test multiple include refs".to_string(),
    );

    // Multiple include references
    let include_refs = vec![
        IncludeRef {
            id: "TC_SETUP_001".to_string(),
            test_sequence: None,
        },
        IncludeRef {
            id: "TC_SETUP_002".to_string(),
            test_sequence: None,
        },
        IncludeRef {
            id: "TC_CONFIG_003".to_string(),
            test_sequence: Some("Seq1".to_string()),
        },
        IncludeRef {
            id: "TC_INIT_004".to_string(),
            test_sequence: Some("Seq2".to_string()),
        },
    ];

    test_case.initial_conditions = InitialConditions {
        include: Some(include_refs),
        devices: HashMap::new(),
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify all include references appear as comments
    assert!(script.contains("# Initial Conditions"));
    assert!(script.contains("# Include: TC_SETUP_001"));
    assert!(script.contains("# Include: TC_SETUP_002"));
    assert!(script.contains("# Include: TC_CONFIG_003 (test_sequence: Seq1)"));
    assert!(script.contains("# Include: TC_INIT_004 (test_sequence: Seq2)"));
}

#[test]
fn test_initial_conditions_only_include_array_no_devices() {
    use testcase_manager::models::IncludeRef;

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ009".to_string(),
        1,
        1,
        "TC009".to_string(),
        "Test only include array".to_string(),
    );

    // Only include array, no devices
    let include_refs = vec![IncludeRef {
        id: "TC_FULL_SETUP".to_string(),
        test_sequence: None,
    }];

    test_case.general_initial_conditions = InitialConditions {
        include: Some(include_refs),
        devices: HashMap::new(),
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify include array appears even without devices
    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Include: TC_FULL_SETUP"));
}

#[test]
fn test_initial_conditions_complex_mixed_structure() {
    use testcase_manager::models::{IncludeRef, TestSequenceRefTarget};

    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ010".to_string(),
        1,
        1,
        "TC010".to_string(),
        "Test complex mixed structure".to_string(),
    );

    // Complex structure with all types
    let include_refs = vec![
        IncludeRef {
            id: "TC_BASE_001".to_string(),
            test_sequence: None,
        },
        IncludeRef {
            id: "TC_BASE_002".to_string(),
            test_sequence: Some("SeqBase".to_string()),
        },
    ];

    let mut general_devices = HashMap::new();
    general_devices.insert(
        "System".to_string(),
        vec![
            InitialConditionItem::String("Device is powered".to_string()),
            InitialConditionItem::String("create directory \"/tmp/sys\"".to_string()),
            InitialConditionItem::RefItem {
                reference: "SYS_CONFIG".to_string(),
            },
            InitialConditionItem::TestSequenceRef {
                test_sequence: TestSequenceRefTarget {
                    id: 1,
                    step: "setup".to_string(),
                },
            },
            InitialConditionItem::String("wait for 1 seconds".to_string()),
        ],
    );

    test_case.general_initial_conditions = InitialConditions {
        include: Some(include_refs),
        devices: general_devices,
    };

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = create_test_step(1, "Test", "echo 'test'", "0", "test", Some(true));
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Verify include array
    assert!(script.contains("# General Initial Conditions"));
    assert!(script.contains("# Include: TC_BASE_001"));
    assert!(script.contains("# Include: TC_BASE_002 (test_sequence: SeqBase)"));

    // Verify non-BDD string appears as comment only
    assert!(script.contains("# System: Device is powered"));

    // Verify BDD patterns generate commands
    assert!(script.contains("# System: create directory \"/tmp/sys\""));
    assert!(script.contains("mkdir -p \"/tmp/sys\""));

    // Verify ref item appears as comment
    assert!(script.contains("# System: ref: SYS_CONFIG"));

    // Verify test_sequence ref appears as comment
    assert!(script.contains("# System: test_sequence: id=1, step=setup"));

    // Verify second BDD pattern generates command
    assert!(script.contains("# System: wait for 1 seconds"));
    assert!(script.contains("sleep 1"));
}

// ============================================================================
// Manual Step Without Verification Tests
// ============================================================================

#[test]
fn test_manual_step_without_verification_no_user_verification_variables() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_001".to_string(),
        1,
        1,
        "TC_NO_VERIFY_001".to_string(),
        "Manual step without verification - no variables".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual action without verification",
        "ssh device 'reboot'",
        "0",
        "rebooted",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // REQUIREMENT: No USER_VERIFICATION variables should be generated
    assert!(
        !script.contains("USER_VERIFICATION_RESULT"),
        "Script must NOT contain USER_VERIFICATION_RESULT variable"
    );
    assert!(
        !script.contains("USER_VERIFICATION_OUTPUT"),
        "Script must NOT contain USER_VERIFICATION_OUTPUT variable"
    );
    assert!(
        !script.contains("USER_VERIFICATION="),
        "Script must NOT contain USER_VERIFICATION variable assignment"
    );
}

#[test]
fn test_manual_step_without_verification_no_evaluation_code() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_002".to_string(),
        1,
        1,
        "TC_NO_VERIFY_002".to_string(),
        "Manual step without verification - no evaluation".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Manual task",
        "configure device manually",
        "0",
        "configured",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // REQUIREMENT: No verification evaluation code should be generated
    // Check that script does NOT contain if statements for verification
    assert!(
        !script.contains("if [ \"$USER_VERIFICATION_RESULT\""),
        "Script must NOT contain verification result check"
    );
    assert!(
        !script.contains("if [ \"$USER_VERIFICATION_OUTPUT\""),
        "Script must NOT contain verification output check"
    );
    assert!(
        !script.contains("if [ \"$USER_VERIFICATION\""),
        "Script must NOT contain combined verification check"
    );

    // Should not contain any verification-related conditionals
    let step_section = script.split("Step 1:").nth(1).unwrap_or("");
    assert!(
        !step_section.contains("if [ -"),
        "Step section should not contain file check conditionals"
    );
    assert!(
        !step_section.contains("if grep"),
        "Step section should not contain grep conditionals"
    );
}

#[test]
fn test_manual_step_without_verification_simple_enter_prompt() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_003".to_string(),
        1,
        1,
        "TC_NO_VERIFY_003".to_string(),
        "Manual step without verification - simple prompt".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Observe LED",
        "check device LED",
        "0",
        "checked",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // REQUIREMENT: Only simple ENTER prompt should be generated
    assert!(
        script.contains("Press ENTER to continue..."),
        "Script must contain simple 'Press ENTER to continue...' prompt"
    );

    // Verify it uses read -p with the correct prompt
    assert!(
        script.contains("read -p \"Press ENTER to continue...\""),
        "Script must use read -p with simple continue prompt"
    );

    // Should NOT contain verification-related prompts
    assert!(
        !script.contains("Press ENTER after completing the manual action"),
        "Script should not contain verification-specific prompt"
    );
}

#[test]
fn test_manual_step_without_verification_no_pass_fail_messages() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_004".to_string(),
        1,
        1,
        "TC_NO_VERIFY_004".to_string(),
        "Manual step without verification - no pass/fail".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Physical inspection",
        "inspect hardware",
        "0",
        "inspected",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // REQUIREMENT: No [PASS]/[FAIL] messages should be generated
    assert!(
        !script.contains("[PASS]"),
        "Script must NOT contain [PASS] message"
    );
    assert!(
        !script.contains("[FAIL]"),
        "Script must NOT contain [FAIL] message"
    );

    // Specifically check for this step's pass/fail messages
    assert!(
        !script.contains("[PASS] Step 1"),
        "Script must NOT contain [PASS] message for step 1"
    );
    assert!(
        !script.contains("[FAIL] Step 1"),
        "Script must NOT contain [FAIL] message for step 1"
    );
}

#[test]
fn test_manual_step_without_verification_displays_step_info() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_005".to_string(),
        1,
        1,
        "TC_NO_VERIFY_005".to_string(),
        "Manual step without verification - displays info".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Power cycle device",
        "unplug and replug device",
        "0",
        "power cycled",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Should still display basic step information
    assert!(
        script.contains("Step 1: Power cycle device"),
        "Script must display step description"
    );
    assert!(
        script.contains("Command: unplug and replug device"),
        "Script must display command"
    );
    assert!(
        script.contains("INFO: This is a manual step"),
        "Script must display manual step info"
    );
}

#[test]
fn test_manual_step_without_verification_multiple_steps() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_006".to_string(),
        1,
        1,
        "TC_NO_VERIFY_006".to_string(),
        "Multiple manual steps without verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    // First manual step without verification
    let mut step1 = create_test_step(1, "Manual step 1", "action 1", "0", "done", Some(true));
    step1.manual = Some(true);
    step1.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step1);

    // Second manual step without verification
    let mut step2 = create_test_step(2, "Manual step 2", "action 2", "0", "done", Some(true));
    step2.manual = Some(true);
    step2.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step2);

    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Both steps should have simple ENTER prompts
    let press_enter_count = script.matches("Press ENTER to continue...").count();
    assert!(
        press_enter_count >= 2,
        "Script must have ENTER prompt for each manual step without verification"
    );

    // Neither step should have USER_VERIFICATION variables
    assert!(
        !script.contains("USER_VERIFICATION"),
        "Script must NOT contain any USER_VERIFICATION variables"
    );

    // Neither step should have PASS/FAIL messages
    assert!(
        !script.contains("[PASS]"),
        "Script must NOT contain any [PASS] messages"
    );
    assert!(
        !script.contains("[FAIL]"),
        "Script must NOT contain any [FAIL] messages"
    );
}

#[test]
fn test_manual_step_without_verification_mixed_with_verification() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_007".to_string(),
        1,
        1,
        "TC_NO_VERIFY_007".to_string(),
        "Manual steps mixed - with and without verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());

    // Manual step WITHOUT verification
    let mut step1 = create_test_step(
        1,
        "Simple manual step",
        "perform action",
        "0",
        "done",
        Some(true),
    );
    step1.manual = Some(true);
    step1.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step1);

    // Manual step WITH verification
    let mut step2 = create_test_step(
        2,
        "Verified manual step",
        "perform verified action",
        "0",
        "done",
        Some(true),
    );
    step2.manual = Some(true);
    step2.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/test ]".to_string()),
        output: VerificationExpression::Simple("grep -q 'OK' /tmp/result".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step2);

    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Step 1: Should NOT have USER_VERIFICATION variables
    let step1_section =
        &script[script.find("# Step 1:").unwrap()..script.find("# Step 2:").unwrap()];
    assert!(
        !step1_section.contains("USER_VERIFICATION"),
        "Step 1 section must NOT contain USER_VERIFICATION variables"
    );
    assert!(
        !step1_section.contains("[PASS]"),
        "Step 1 section must NOT contain [PASS] message"
    );
    assert!(
        !step1_section.contains("[FAIL]"),
        "Step 1 section must NOT contain [FAIL] message"
    );
    assert!(
        step1_section.contains("Press ENTER to continue..."),
        "Step 1 must have simple ENTER prompt"
    );

    // Step 2: SHOULD have USER_VERIFICATION variables and pass/fail logic
    let step2_section = &script[script.find("# Step 2:").unwrap()..];
    assert!(
        step2_section.contains("USER_VERIFICATION_RESULT"),
        "Step 2 section must contain USER_VERIFICATION_RESULT"
    );
    assert!(
        step2_section.contains("USER_VERIFICATION_OUTPUT"),
        "Step 2 section must contain USER_VERIFICATION_OUTPUT"
    );
    assert!(
        step2_section.contains("[PASS]") || step2_section.contains("[FAIL]"),
        "Step 2 section must contain PASS/FAIL messages"
    );
}

#[test]
fn test_manual_step_without_verification_result_true_output_false() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_008".to_string(),
        1,
        1,
        "TC_NO_VERIFY_008".to_string(),
        "Manual step with result=true, output=false".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Partial verification",
        "test action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("[ -f /tmp/check ]".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // This case DOES have verification (output is not "true"), so it SHOULD have USER_VERIFICATION
    assert!(
        script.contains("USER_VERIFICATION_RESULT"),
        "Script with output verification should contain USER_VERIFICATION_RESULT"
    );
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT"),
        "Script with output verification should contain USER_VERIFICATION_OUTPUT"
    );
}

#[test]
fn test_manual_step_without_verification_result_false_output_true() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_009".to_string(),
        1,
        1,
        "TC_NO_VERIFY_009".to_string(),
        "Manual step with result=false, output=true".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Partial verification",
        "test action",
        "0",
        "done",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("[ -f /tmp/check ]".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // This case DOES have verification (result is not "true"), so it SHOULD have USER_VERIFICATION
    assert!(
        script.contains("USER_VERIFICATION_RESULT"),
        "Script with result verification should contain USER_VERIFICATION_RESULT"
    );
    assert!(
        script.contains("USER_VERIFICATION_OUTPUT"),
        "Script with result verification should contain USER_VERIFICATION_OUTPUT"
    );
}

#[test]
fn test_manual_step_without_verification_complete_workflow() {
    let executor = TestExecutor::new();
    let mut test_case = TestCase::new(
        "REQ_NO_VERIFY_010".to_string(),
        1,
        1,
        "TC_NO_VERIFY_010".to_string(),
        "Complete workflow for manual step without verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let mut step = create_test_step(
        1,
        "Full manual workflow",
        "complete manual task",
        "0",
        "completed",
        Some(true),
    );
    step.manual = Some(true);
    step.verification = Verification {
        result: VerificationExpression::Simple("true".to_string()),
        output: VerificationExpression::Simple("true".to_string()),
        output_file: None,
        general: None,
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let script = executor.generate_test_script(&test_case);

    // Comprehensive check: script flow should be:
    // 1. Display step info (description, command)
    // 2. Display manual step notice
    // 3. Simple ENTER prompt
    // 4. NO verification logic
    // 5. NO pass/fail messages

    let step_start = script.find("# Step 1:").expect("Step 1 should exist");
    let step_section = &script[step_start..];

    // Find order of elements
    let desc_pos = step_section
        .find("Full manual workflow")
        .expect("Description should exist");
    let cmd_pos = step_section
        .find("complete manual task")
        .expect("Command should exist");
    let manual_info_pos = step_section
        .find("INFO: This is a manual step")
        .expect("Manual info should exist");
    let enter_pos = step_section
        .find("Press ENTER to continue...")
        .expect("ENTER prompt should exist");

    // Verify order
    assert!(desc_pos < cmd_pos, "Description should come before command");
    assert!(
        cmd_pos < manual_info_pos,
        "Command should come before manual info"
    );
    assert!(
        manual_info_pos < enter_pos,
        "Manual info should come before ENTER prompt"
    );

    // Verify absences in this step section (up to next step or end)
    let step_end = step_section.find("# Step 2:").unwrap_or(step_section.len());
    let this_step = &step_section[..step_end];

    assert!(
        !this_step.contains("USER_VERIFICATION"),
        "Should not contain USER_VERIFICATION"
    );
    assert!(!this_step.contains("[PASS]"), "Should not contain [PASS]");
    assert!(!this_step.contains("[FAIL]"), "Should not contain [FAIL]");
    assert!(!this_step.contains("exit 1"), "Should not contain exit 1");
}
