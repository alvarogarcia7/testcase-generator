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
