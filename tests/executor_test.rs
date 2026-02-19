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
