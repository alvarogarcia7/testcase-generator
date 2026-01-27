use std::collections::HashMap;
use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{
    Expected, Step, TestCase, TestSequence, TestStepExecutionEntry, Verification,
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
        expected: Expected {
            success,
            result: expected_result.to_string(),
            output: expected_output.to_string(),
        },
        verification: Verification {
            result: "[ $? -eq 0 ]".to_string(),
            output: "[ \"$COMMAND_OUTPUT\" = \"success\" ]".to_string(),
        },
    }
}

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

    assert!(script.contains("COMMAND_OUTPUT=$()"));
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

    assert!(script.contains("COMMAND_OUTPUT=$(MY_VAR=test echo $MY_VAR)"));
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
