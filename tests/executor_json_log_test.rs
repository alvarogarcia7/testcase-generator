use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use testcase_manager::{TestCase, TestExecutor, TestStepExecutionEntry};

/// Validates that a JSON string conforms to the execution log schema
fn validate_json_schema(json_str: &str) {
    let parsed: Result<Vec<TestStepExecutionEntry>, _> = serde_json::from_str(json_str);
    assert!(
        parsed.is_ok(),
        "JSON must be parseable as Vec<TestStepExecutionEntry>: {:?}",
        parsed.err()
    );

    let entries = parsed.unwrap();
    for entry in &entries {
        assert!(
            entry.test_sequence >= 0,
            "test_sequence must be non-negative"
        );
        assert!(entry.step >= 0, "step must be non-negative");
        assert!(!entry.command.is_empty(), "command must not be empty");
    }

    let json_value: Value = serde_json::from_str(json_str).expect("JSON must be valid");
    assert!(json_value.is_array(), "Root must be an array");

    let array = json_value.as_array().unwrap();
    for item in array {
        assert!(item.is_object(), "Each item must be an object");
        let obj = item.as_object().unwrap();

        assert!(
            obj.contains_key("test_sequence"),
            "Must have test_sequence field"
        );
        assert!(obj.contains_key("step"), "Must have step field");
        assert!(obj.contains_key("command"), "Must have command field");
        assert!(obj.contains_key("exit_code"), "Must have exit_code field");
        assert!(obj.contains_key("output"), "Must have output field");

        assert!(
            obj["test_sequence"].is_number(),
            "test_sequence must be a number"
        );
        assert!(obj["step"].is_number(), "step must be a number");
        assert!(obj["command"].is_string(), "command must be a string");
        assert!(obj["exit_code"].is_number(), "exit_code must be a number");
        assert!(obj["output"].is_string(), "output must be a string");

        if obj.contains_key("timestamp") {
            assert!(
                obj["timestamp"].is_string(),
                "timestamp must be a string if present"
            );
        }
    }
}

/// Validates that the JSON log entries match the test case structure
fn validate_log_matches_testcase(entries: &[TestStepExecutionEntry], test_case: &TestCase) {
    let mut expected_entries = Vec::new();
    for sequence in &test_case.test_sequences {
        for step in &sequence.steps {
            if step.manual != Some(true) {
                expected_entries.push((sequence.id, step.step, step.command.clone()));
            }
        }
    }

    assert_eq!(
        entries.len(),
        expected_entries.len(),
        "Number of log entries must match non-manual steps in test case"
    );

    for (i, entry) in entries.iter().enumerate() {
        let (expected_seq, expected_step, expected_cmd) = &expected_entries[i];
        assert_eq!(
            entry.test_sequence, *expected_seq,
            "Entry {} test_sequence mismatch",
            i
        );
        assert_eq!(entry.step, *expected_step, "Entry {} step mismatch", i);
        assert_eq!(entry.command, *expected_cmd, "Entry {} command mismatch", i);
    }
}

#[test]
fn test_json_log_schema_validation() -> Result<()> {
    let json_str = r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "command": "echo 'test'",
    "exit_code": 0,
    "output": "test",
    "timestamp": "2024-01-15T10:30:00Z"
  },
  {
    "test_sequence": 1,
    "step": 2,
    "command": "true",
    "exit_code": 0,
    "output": ""
  }
]"#;

    validate_json_schema(json_str);
    Ok(())
}

#[test]
fn test_json_log_invalid_schema() {
    let invalid_jsons = [
        r#"{"not": "an array"}"#,
        r#"[{"test_sequence": -1, "step": 1, "command": "test", "exit_code": 0, "output": ""}]"#,
        r#"[{"test_sequence": 1, "step": 1, "exit_code": 0, "output": ""}]"#,
        r#"[{"test_sequence": 1, "step": 1, "command": "", "exit_code": 0, "output": ""}]"#,
    ];

    for (i, json_str) in invalid_jsons.iter().enumerate() {
        let result = std::panic::catch_unwind(|| {
            validate_json_schema(json_str);
        });
        assert!(
            result.is_err(),
            "Invalid JSON {} should have failed validation",
            i
        );
    }
}

#[test]
fn test_executor_generates_valid_json_log() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let test_case = create_simple_test_case();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);

    assert!(result.is_ok(), "Test execution should succeed");

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    validate_json_schema(&json_content);

    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;
    validate_log_matches_testcase(&entries, &test_case);

    Ok(())
}

#[test]
fn test_executor_json_log_structure_and_content() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let test_case = create_multi_sequence_test_case();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);

    assert!(result.is_ok(), "Test execution should succeed");

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;

    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

    assert_eq!(entries.len(), 4, "Should have 4 log entries");

    assert_eq!(entries[0].test_sequence, 1);
    assert_eq!(entries[0].step, 1);
    assert_eq!(entries[0].command, "echo 'Hello'");
    assert_eq!(entries[0].exit_code, 0);
    assert!(entries[0].output.contains("Hello"));
    assert!(entries[0].timestamp.is_some());

    assert_eq!(entries[1].test_sequence, 1);
    assert_eq!(entries[1].step, 2);
    assert_eq!(entries[1].command, "true");
    assert_eq!(entries[1].exit_code, 0);

    assert_eq!(entries[2].test_sequence, 2);
    assert_eq!(entries[2].step, 1);
    assert_eq!(entries[2].command, "echo 'World'");
    assert_eq!(entries[2].exit_code, 0);
    assert!(entries[2].output.contains("World"));

    assert_eq!(entries[3].test_sequence, 2);
    assert_eq!(entries[3].step, 2);
    assert_eq!(entries[3].command, "echo 'Test Complete'");
    assert_eq!(entries[3].exit_code, 0);
    assert!(entries[3].output.contains("Test Complete"));

    Ok(())
}

#[test]
fn test_executor_json_log_with_manual_steps() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let test_case = create_test_case_with_manual_steps();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);

    assert!(result.is_ok(), "Test execution should succeed");

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

    assert_eq!(entries.len(), 2, "Should only have 2 automated steps");

    assert_eq!(entries[0].test_sequence, 1);
    assert_eq!(entries[0].step, 1);
    assert_eq!(entries[0].command, "echo 'Before Manual'");

    assert_eq!(entries[1].test_sequence, 1);
    assert_eq!(entries[1].step, 3);
    assert_eq!(entries[1].command, "echo 'After Manual'");

    Ok(())
}

#[test]
fn test_executor_with_gsma_yaml_example() -> Result<()> {
    let gsma_file = PathBuf::from("data/gsma_4.4.2.2_TC.yml");
    if !gsma_file.exists() {
        println!("Skipping test: gsma_4.4.2.2_TC.yml not found");
        return Ok(());
    }

    let yaml_content = fs::read_to_string(&gsma_file)?;
    let test_case: TestCase = serde_yaml::from_str(&yaml_content)?;

    let temp_dir = TempDir::new()?;

    let executor = TestExecutor::with_output_dir(temp_dir.path());
    let _result = executor.execute_test_case(&test_case);

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if log_file.exists() {
        let json_content = fs::read_to_string(&log_file)?;
        validate_json_schema(&json_content);

        let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

        let non_manual_steps: Vec<_> = test_case
            .test_sequences
            .iter()
            .flat_map(|seq| &seq.steps)
            .filter(|step| step.manual != Some(true))
            .collect();

        assert_eq!(
            entries.len(),
            non_manual_steps.len(),
            "Log entries should match non-manual steps"
        );

        for entry in &entries {
            assert!(entry.test_sequence > 0, "test_sequence must be positive");
            assert!(entry.step > 0, "step must be positive");
            assert!(!entry.command.is_empty(), "command must not be empty");
        }
    }

    Ok(())
}

#[test]
fn test_executor_with_self_validated_example() -> Result<()> {
    let example_file = PathBuf::from("testcases/self_validated_example.yml");
    if !example_file.exists() {
        println!("Skipping test: self_validated_example.yml not found");
        return Ok(());
    }

    let yaml_content = fs::read_to_string(&example_file)?;
    let test_case: TestCase = serde_yaml::from_str(&yaml_content)?;

    let temp_dir = TempDir::new()?;

    let executor = TestExecutor::with_output_dir(temp_dir.path());
    let result = executor.execute_test_case(&test_case);

    if result.is_err() {
        println!("Note: Test case execution may fail, but we're validating the log format");
    }

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if log_file.exists() {
        let json_content = fs::read_to_string(&log_file)?;
        validate_json_schema(&json_content);

        let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;
        validate_log_matches_testcase(&entries, &test_case);

        for (i, entry) in entries.iter().enumerate() {
            assert!(
                entry.timestamp.is_some(),
                "Entry {} should have a timestamp",
                i
            );
        }

        assert!(!entries.is_empty(), "Should have at least one entry");
    }

    Ok(())
}

#[test]
fn test_json_log_via_test_executor_binary() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let test_yaml_path = temp_dir.path().join("test_case.yml");
    let test_case = create_simple_test_case();
    let yaml_content = serde_yaml::to_string(&test_case)?;
    fs::write(&test_yaml_path, yaml_content)?;

    let output = Command::new("cargo")
        .args(["run", "--bin", "test-executor", "--", "execute"])
        .arg(&test_yaml_path)
        .current_dir(temp_dir.path())
        .output()?;

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if log_file.exists() {
        let json_content = fs::read_to_string(&log_file)?;
        validate_json_schema(&json_content);

        let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;
        assert!(!entries.is_empty(), "Should have execution entries");

        for entry in &entries {
            assert!(entry.timestamp.is_some(), "Should have timestamp");
        }
    } else if output.status.success() {
        panic!("Expected execution log file was not created");
    }

    Ok(())
}

#[test]
fn test_json_log_format_compliance() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let test_case = create_test_case_with_special_characters();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let _result = executor.execute_test_case(&test_case);

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if log_file.exists() {
        let json_content = fs::read_to_string(&log_file)?;

        let parsed_result: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(
            parsed_result.is_ok(),
            "JSON with special characters should be valid"
        );

        let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

        for entry in &entries {
            assert!(!entry.command.is_empty());
            assert!(entry.test_sequence >= 0);
            assert!(entry.step >= 0);
        }
    }

    Ok(())
}

fn create_simple_test_case() -> TestCase {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "SIMPLE_TC_001".to_string(),
        "Simple test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence1".to_string(), "Test sequence".to_string());

    let step = Step {
        step: 1,
        manual: None,
        description: "Echo test".to_string(),
        command: "echo 'test'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence.steps.push(step);

    test_case.test_sequences.push(sequence);
    test_case
}

fn create_multi_sequence_test_case() -> TestCase {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "MULTI_SEQ_TC_001".to_string(),
        "Multi-sequence test case".to_string(),
    );

    let mut sequence1 = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "First test sequence".to_string(),
    );

    let step1 = Step {
        step: 1,
        manual: None,
        description: "Echo Hello".to_string(),
        command: "echo 'Hello'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence1.steps.push(step1);

    let step2 = Step {
        step: 2,
        manual: None,
        description: "Run true".to_string(),
        command: "true".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence1.steps.push(step2);

    let mut sequence2 = TestSequence::new(
        2,
        "Sequence2".to_string(),
        "Second test sequence".to_string(),
    );

    let step3 = Step {
        step: 1,
        manual: None,
        description: "Echo World".to_string(),
        command: "echo 'World'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence2.steps.push(step3);

    let step4 = Step {
        step: 2,
        manual: None,
        description: "Echo Complete".to_string(),
        command: "echo 'Test Complete'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence2.steps.push(step4);

    test_case.test_sequences.push(sequence1);
    test_case.test_sequences.push(sequence2);
    test_case
}

fn create_test_case_with_manual_steps() -> TestCase {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "MANUAL_STEPS_TC_001".to_string(),
        "Test case with manual steps".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "Test with manual steps".to_string(),
    );

    let step1 = Step {
        step: 1,
        manual: None,
        description: "Before manual step".to_string(),
        command: "echo 'Before Manual'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence.steps.push(step1);

    let step2 = Step {
        step: 2,
        manual: Some(true),
        description: "Manual verification step".to_string(),
        command: "ssh device".to_string(),
        expected: Expected {
            success: Some(true),
            result: "connected".to_string(),
            output: "success".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("true".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence.steps.push(step2);

    let step3 = Step {
        step: 3,
        manual: None,
        description: "After manual step".to_string(),
        command: "echo 'After Manual'".to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence.steps.push(step3);

    test_case.test_sequences.push(sequence);
    test_case
}

fn create_test_case_with_special_characters() -> TestCase {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "SPECIAL_CHARS_TC_001".to_string(),
        "Test case with special characters".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "Test with special chars".to_string(),
    );

    let step = Step {
        step: 1,
        manual: None,
        description: "Echo with quotes".to_string(),
        command: r#"echo 'Test "quotes" and $variables'"#.to_string(),
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
        },
    };
    sequence.steps.push(step);

    test_case.test_sequences.push(sequence);
    test_case
}
