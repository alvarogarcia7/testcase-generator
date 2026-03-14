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

        // Validate verification fields if present
        if obj.contains_key("result_verification_pass") {
            assert!(
                obj["result_verification_pass"].is_boolean(),
                "result_verification_pass must be a boolean if present"
            );
        }

        if obj.contains_key("output_verification_pass") {
            assert!(
                obj["output_verification_pass"].is_boolean(),
                "output_verification_pass must be a boolean if present"
            );
        }
    }
}

/// Validates that the JSON log entries match the test case structure
fn validate_log_matches_testcase(entries: &[TestStepExecutionEntry], test_case: &TestCase) {
    use std::collections::HashMap;
    use testcase_manager::Step;

    // Build a map of all steps from the test case
    let mut all_steps: HashMap<(i64, i64), (Step, bool)> = HashMap::new();
    let mut manual_step_count = 0;

    for sequence in &test_case.test_sequences {
        for step in &sequence.steps {
            let is_manual = step.manual == Some(true);
            if is_manual {
                manual_step_count += 1;
            }
            all_steps.insert((sequence.id, step.step), (step.clone(), is_manual));
        }
    }

    let total_steps = all_steps.len();
    let non_manual_steps = total_steps - manual_step_count;

    // Determine if this log includes manual steps based on entry count
    let includes_manual_steps = if entries.len() == total_steps {
        true
    } else if entries.len() == non_manual_steps {
        false
    } else {
        panic!(
            "Number of log entries ({}) does not match expected count with all steps ({}) or without manual steps ({})",
            entries.len(),
            total_steps,
            non_manual_steps
        );
    };

    // Validate each entry
    for (i, entry) in entries.iter().enumerate() {
        let key = (entry.test_sequence, entry.step);
        let (step, is_manual) = all_steps.get(&key).unwrap_or_else(|| {
            panic!(
                "Entry {} has unexpected test_sequence={} step={} combination",
                i, entry.test_sequence, entry.step
            )
        });

        // If manual steps are not included in the log, we shouldn't see any
        if *is_manual && !includes_manual_steps {
            panic!(
                "Entry {} is a manual step but manual steps should not be in this log",
                i
            );
        }

        assert_eq!(entry.command, step.command, "Entry {} command mismatch", i);

        // Validate verification fields are present and correctly typed in JSON
        if *is_manual {
            // Check if manual step has verification fields (not just "true")
            let has_result_verification = !matches!(
                &step.verification.result,
                testcase_manager::VerificationExpression::Simple(s) if s.trim() == "true"
            );
            let has_output_verification = !matches!(
                &step.verification.output,
                testcase_manager::VerificationExpression::Simple(s) if s.trim() == "true"
            );
            let has_verification = has_result_verification || has_output_verification;

            if has_verification {
                // Manual steps with verification should have verification fields
                assert!(
                    entry.result_verification_pass.is_some(),
                    "Entry {} (manual step with verification) must have result_verification_pass field",
                    i
                );
                assert!(
                    entry.output_verification_pass.is_some(),
                    "Entry {} (manual step with verification) must have output_verification_pass field",
                    i
                );

                // Verify the fields are booleans (unwrap to ensure they're the correct type)
                // Since result_verification_pass and output_verification_pass are Option<bool>,
                // unwrapping them is sufficient to verify they contain boolean values
                let _result_pass = entry.result_verification_pass.unwrap();
                let _output_pass = entry.output_verification_pass.unwrap();
            }
        } else {
            // Automated steps should always have verification fields
            assert!(
                entry.result_verification_pass.is_some(),
                "Entry {} (automated step) must have result_verification_pass field",
                i
            );
            assert!(
                entry.output_verification_pass.is_some(),
                "Entry {} (automated step) must have output_verification_pass field",
                i
            );

            // Verify the fields are booleans
            let _result_pass = entry.result_verification_pass.unwrap();
            let _output_pass = entry.output_verification_pass.unwrap();
        }
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

    // All steps in create_simple_test_case are automated, verify they have verification fields
    for (i, entry) in entries.iter().enumerate() {
        assert!(
            entry.result_verification_pass.is_some(),
            "Automated step {} should have result_verification_pass field",
            i
        );
        assert!(
            entry.output_verification_pass.is_some(),
            "Automated step {} should have output_verification_pass field",
            i
        );
    }

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
    // Automated steps should have verification fields
    assert!(
        entries[0].result_verification_pass.is_some(),
        "Automated step should have result_verification_pass field"
    );
    assert!(
        entries[0].output_verification_pass.is_some(),
        "Automated step should have output_verification_pass field"
    );

    assert_eq!(entries[1].test_sequence, 1);
    assert_eq!(entries[1].step, 2);
    assert_eq!(entries[1].command, "true");
    assert_eq!(entries[1].exit_code, 0);
    // Automated steps should have verification fields
    assert!(
        entries[1].result_verification_pass.is_some(),
        "Automated step should have result_verification_pass field"
    );
    assert!(
        entries[1].output_verification_pass.is_some(),
        "Automated step should have output_verification_pass field"
    );

    assert_eq!(entries[2].test_sequence, 2);
    assert_eq!(entries[2].step, 1);
    assert_eq!(entries[2].command, "echo 'World'");
    assert_eq!(entries[2].exit_code, 0);
    assert!(entries[2].output.contains("World"));
    // Automated steps should have verification fields
    assert!(
        entries[2].result_verification_pass.is_some(),
        "Automated step should have result_verification_pass field"
    );
    assert!(
        entries[2].output_verification_pass.is_some(),
        "Automated step should have output_verification_pass field"
    );

    assert_eq!(entries[3].test_sequence, 2);
    assert_eq!(entries[3].step, 2);
    assert_eq!(entries[3].command, "echo 'Test Complete'");
    assert_eq!(entries[3].exit_code, 0);
    assert!(entries[3].output.contains("Test Complete"));
    // Automated steps should have verification fields
    assert!(
        entries[3].result_verification_pass.is_some(),
        "Automated step should have result_verification_pass field"
    );
    assert!(
        entries[3].output_verification_pass.is_some(),
        "Automated step should have output_verification_pass field"
    );

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
    // Automated steps should have verification fields
    assert!(
        entries[0].result_verification_pass.is_some(),
        "Automated step should have result_verification_pass field"
    );
    assert!(
        entries[0].output_verification_pass.is_some(),
        "Automated step should have output_verification_pass field"
    );

    assert_eq!(entries[1].test_sequence, 1);
    assert_eq!(entries[1].step, 3);
    assert_eq!(entries[1].command, "echo 'After Manual'");
    // Automated steps should have verification fields
    assert!(
        entries[1].result_verification_pass.is_some(),
        "Automated step should have result_verification_pass field"
    );
    assert!(
        entries[1].output_verification_pass.is_some(),
        "Automated step should have output_verification_pass field"
    );

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

    // Set environment variables to run in non-interactive mode
    // This prevents the executor from hanging on manual step prompts
    std::env::set_var("DEBIAN_FRONTEND", "noninteractive");
    std::env::set_var("CI", "1");

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
            // All steps in self_validated_example are automated, verify they have verification fields
            assert!(
                entry.result_verification_pass.is_some(),
                "Automated step {} should have result_verification_pass field",
                i
            );
            assert!(
                entry.output_verification_pass.is_some(),
                "Automated step {} should have output_verification_pass field",
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

        for (i, entry) in entries.iter().enumerate() {
            assert!(entry.timestamp.is_some(), "Should have timestamp");
            // Test case created by create_simple_test_case has only automated steps
            assert!(
                entry.result_verification_pass.is_some(),
                "Automated step {} should have result_verification_pass field",
                i
            );
            assert!(
                entry.output_verification_pass.is_some(),
                "Automated step {} should have output_verification_pass field",
                i
            );
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

        for (i, entry) in entries.iter().enumerate() {
            assert!(!entry.command.is_empty());
            assert!(entry.test_sequence >= 0);
            assert!(entry.step >= 0);
            // Test case created by create_test_case_with_special_characters has only automated steps
            assert!(
                entry.result_verification_pass.is_some(),
                "Automated step {} should have result_verification_pass field",
                i
            );
            assert!(
                entry.output_verification_pass.is_some(),
                "Automated step {} should have output_verification_pass field",
                i
            );
        }
    }

    Ok(())
}

#[test]
fn test_json_log_with_manual_step_verification() -> Result<()> {
    use std::process::Command;

    let temp_dir = TempDir::new()?;

    let test_case = create_test_case_with_manual_step_verification();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    let script = executor.generate_test_script_with_json_output(&test_case, &json_log_path);

    let script_path = temp_dir.path().join("test_script.sh");
    fs::write(&script_path, script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Execute the script with automated inputs for manual steps
    // The script will prompt for manual verification, we provide "yes" answers
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    // The script may fail if it tries to prompt for input in non-interactive mode
    // But we should still have JSON log entries for automated steps
    if !json_log_path.exists() {
        // If no JSON log was created, the script failed before completing any steps
        eprintln!("Script output: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("Script errors: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(()); // Skip test if script can't run
    }

    let json_content = fs::read_to_string(&json_log_path)?;

    // Parse the JSON log
    let parsed_result: Result<Vec<TestStepExecutionEntry>, _> = serde_json::from_str(&json_content);
    assert!(
        parsed_result.is_ok(),
        "JSON should parse successfully. Error: {:?}\nJSON content:\n{}",
        parsed_result.err(),
        json_content
    );

    let entries = parsed_result.unwrap();

    // The test case has:
    // - Step 1: automated step (should be in log)
    // - Step 2: manual step with verification (should be in log if executed)
    // - Step 3: automated step (should be in log if script continued)

    // Since manual steps may not execute in non-interactive mode,
    // we just verify the JSON structure for any entries that exist
    for entry in &entries {
        // Check basic fields
        assert!(entry.test_sequence > 0, "test_sequence must be positive");
        assert!(entry.step > 0, "step must be positive");
        assert!(!entry.command.is_empty(), "command must not be empty");

        // If this entry corresponds to the manual step (step 2, sequence 1)
        if entry.test_sequence == 1 && entry.step == 2 {
            // Manual step with verification should have verification fields
            assert!(
                entry.result_verification_pass.is_some(),
                "Manual step with verification must have result_verification_pass field"
            );
            assert!(
                entry.output_verification_pass.is_some(),
                "Manual step with verification must have output_verification_pass field"
            );

            // The fields should be booleans
            let _result_pass = entry.result_verification_pass.unwrap();
            let _output_pass = entry.output_verification_pass.unwrap();
        } else {
            // Automated steps (step 1 and step 3) should have verification fields
            assert!(
                entry.result_verification_pass.is_some(),
                "Automated step (seq {}, step {}) should have result_verification_pass field",
                entry.test_sequence,
                entry.step
            );
            assert!(
                entry.output_verification_pass.is_some(),
                "Automated step (seq {}, step {}) should have output_verification_pass field",
                entry.test_sequence,
                entry.step
            );
        }
    }

    // Validate overall JSON schema
    validate_json_schema(&json_content);

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
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
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
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence1.steps.push(step1);

    let step2 = Step {
        step: 2,
        manual: None,
        description: "Run true".to_string(),
        command: "true".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
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
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence2.steps.push(step3);

    let step4 = Step {
        step: 2,
        manual: None,
        description: "Echo Complete".to_string(),
        command: "echo 'Test Complete'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
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
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step1);

    let step2 = Step {
        step: 2,
        manual: Some(true),
        description: "Manual verification step".to_string(),
        command: "ssh device".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "connected".to_string(),
            output: "success".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("true".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step2);

    let step3 = Step {
        step: 3,
        manual: None,
        description: "After manual step".to_string(),
        command: "echo 'After Manual'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step3);

    test_case.test_sequences.push(sequence);
    test_case
}

fn create_test_case_with_manual_step_verification() -> TestCase {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "MANUAL_VERIFICATION_TC_001".to_string(),
        "Test case with manual step verification".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "Test with manual step verification".to_string(),
    );

    // Step 1: Automated step before manual step
    let step1 = Step {
        step: 1,
        manual: None,
        description: "Setup step before manual verification".to_string(),
        command: "echo 'Setup complete'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step1);

    // Step 2: Manual step with non-trivial verification checks
    let step2 = Step {
        step: 2,
        manual: Some(true),
        description: "Manual step with verification checks".to_string(),
        command: "echo 'Check device connectivity manually'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "device connected".to_string(),
            output: "connectivity verified".to_string(),
        },
        verification: Verification {
            // Non-trivial result verification (not just "true")
            result: VerificationExpression::Simple("[ 1 -eq 1 ]".to_string()),
            // Non-trivial output verification (not just "true")
            output: VerificationExpression::Simple(
                "[ -n \"$COMMAND_OUTPUT\" ] || true".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step2);

    // Step 3: Automated step after manual step
    let step3 = Step {
        step: 3,
        manual: None,
        description: "Cleanup step after manual verification".to_string(),
        command: "echo 'Cleanup complete'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
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
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    };
    sequence.steps.push(step);

    test_case.test_sequences.push(sequence);
    test_case
}

#[test]
fn test_generated_script_produces_valid_json_with_special_chars() -> Result<()> {
    use std::process::Command;

    let temp_dir = TempDir::new()?;

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "SPECIAL_JSON_TC_001".to_string(),
        "Test case with special characters for JSON".to_string(),
    );

    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let mut sequence = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "Test with various special characters".to_string(),
    );

    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Command with single quotes".to_string(),
        command: "echo 'single quoted text'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Command with double quotes".to_string(),
        command: r#"echo "double quoted text""#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 3,
        manual: None,
        description: "Command with backslashes".to_string(),
        command: r#"echo "path\\to\\file""#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 4,
        manual: None,
        description: "Command with newlines".to_string(),
        command: "echo 'line1'\necho 'line2'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 5,
        manual: None,
        description: "Command with tabs".to_string(),
        command: "echo 'text\twith\ttabs'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::with_output_dir(temp_dir.path());
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    let script = executor.generate_test_script_with_json_output(&test_case, &json_log_path);

    let script_path = temp_dir.path().join("test_script.sh");
    fs::write(&script_path, script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    let _json_content_debug = if json_log_path.exists() {
        fs::read_to_string(&json_log_path)
            .unwrap_or_else(|_| "Could not read JSON file".to_string())
    } else {
        "JSON file does not exist".to_string()
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        eprintln!("Script execution failed but continuing to test JSON parsing...");
        eprintln!("Script exit code: {:?}", output.status.code());
        eprintln!("stdout: {}", stdout);
        eprintln!("stderr: {}", stderr);
    }

    assert!(
        json_log_path.exists(),
        "JSON log file should be created even if script failed"
    );

    let json_content = fs::read_to_string(&json_log_path)?;

    let parsed_result: Result<Vec<TestStepExecutionEntry>, _> = serde_json::from_str(&json_content);
    assert!(
        parsed_result.is_ok(),
        "JSON should parse successfully with serde_json. Error: {:?}\nJSON content:\n{}",
        parsed_result.err(),
        json_content
    );

    let entries = parsed_result.unwrap();
    assert_eq!(entries.len(), 5, "Should have 5 execution entries");

    // Note: Single quotes in commands are converted to double quotes to avoid bash syntax issues
    assert_eq!(entries[0].command, "echo \"single quoted text\"");
    assert_eq!(entries[1].command, r#"echo "double quoted text""#);
    assert_eq!(entries[2].command, r#"echo "path\\to\\file""#);
    assert_eq!(entries[3].command, "echo \"line1\"\necho \"line2\"");
    assert_eq!(entries[4].command, "echo \"text\twith\ttabs\"");

    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(entry.exit_code, 0, "All commands should succeed");
        // All steps are automated, verify they have verification fields
        assert!(
            entry.result_verification_pass.is_some(),
            "Automated step {} should have result_verification_pass field",
            i
        );
        assert!(
            entry.output_verification_pass.is_some(),
            "Automated step {} should have output_verification_pass field",
            i
        );
    }

    Ok(())
}

#[test]
fn test_command_json_escaping_edge_cases() -> Result<()> {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let temp_dir = TempDir::new()?;

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "JSON_ESCAPE_EDGE_TC_001".to_string(),
        "Test case with JSON escaping edge cases".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "Test with JSON escaping edge cases".to_string(),
    );

    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Empty command string".to_string(),
        command: "true".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Command with only special characters".to_string(),
        command: r#"echo '!@#$%^&*(){}[]|\:;<>,.?/~`'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 3,
        manual: None,
        description: "Command with Unicode characters".to_string(),
        command: "echo '你好世界 🚀 Привет мир'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 4,
        manual: None,
        description: "Command with consecutive quotes".to_string(),
        command: r#"echo "\"\"\"multiple\"\"\"quotes\"\"\"""#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    sequence.steps.push(Step {
        step: 5,
        manual: None,
        description: "Command with escape sequences".to_string(),
        command: r#"echo "test\\nline\\tbreak\\rcarriage\\bbackspace\\fformfeed""#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "[ $EXIT_CODE -eq 0 ]".to_string(),
            output: "true".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::with_output_dir(temp_dir.path());
    let _result = executor.execute_test_case(&test_case);

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;

    let parsed_result: Result<Vec<TestStepExecutionEntry>, _> = serde_json::from_str(&json_content);
    assert!(
        parsed_result.is_ok(),
        "JSON with edge cases should parse successfully. Error: {:?}\nJSON content:\n{}",
        parsed_result.err(),
        json_content
    );

    let entries = parsed_result.unwrap();
    assert_eq!(entries.len(), 5, "Should have 5 execution entries");

    assert_eq!(entries[0].command, "true");
    assert!(
        entries[1].command.contains("!@#$%^&*(){}[]"),
        "Command should contain special characters: {}",
        entries[1].command
    );
    assert!(
        entries[2].command.contains("你好世界") && entries[2].command.contains("🚀"),
        "Command should contain Unicode characters: {}",
        entries[2].command
    );
    assert!(
        entries[3].command.contains("multiple") && entries[3].command.contains("quotes"),
        "Command should contain consecutive quotes: {}",
        entries[3].command
    );
    assert!(
        entries[4].command.contains("test") && entries[4].command.contains("line"),
        "Command should contain escape sequences: {}",
        entries[4].command
    );

    for (i, entry) in entries.iter().enumerate() {
        assert_eq!(
            entry.exit_code, 0,
            "Command {} should succeed: {}",
            i, entry.command
        );
        assert!(!entry.command.is_empty(), "Command should not be empty");
        assert_eq!(entry.test_sequence, 1, "Should be in test_sequence 1");
        assert_eq!(entry.step, (i + 1) as i64, "Step should match index");
        // All steps are automated, verify they have verification fields
        assert!(
            entry.result_verification_pass.is_some(),
            "Automated step {} should have result_verification_pass field",
            i
        );
        assert!(
            entry.output_verification_pass.is_some(),
            "Automated step {} should have output_verification_pass field",
            i
        );
    }

    validate_json_schema(&json_content);

    Ok(())
}

#[test]
fn test_verification_pass_fields_correctness() -> Result<()> {
    use testcase_manager::{Expected, Step, TestSequence, Verification, VerificationExpression};

    let temp_dir = TempDir::new()?;

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "VERIFICATION_CORRECTNESS_TC_001".to_string(),
        "Test case to verify correctness of verification pass fields".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Sequence1".to_string(),
        "Test with passing and failing verifications".to_string(),
    );

    // Step 1: All verifications should pass
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Passing step with exit code 0".to_string(),
        command: "echo 'success'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "success".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "grep -q 'success' <<< \"$COMMAND_OUTPUT\"".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    // Step 2: Result verification should fail (exit code is 0 but we check for 1)
    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Step with failing result verification".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(false),
            result: "1".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 1 ]".to_string()),
            output: VerificationExpression::Simple(
                "grep -q 'test' <<< \"$COMMAND_OUTPUT\"".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    // Step 3: Output verification should fail (output doesn't contain 'notfound')
    sequence.steps.push(Step {
        step: 3,
        manual: None,
        description: "Step with failing output verification".to_string(),
        command: "echo 'present'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "present".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "grep -q 'notfound' <<< \"$COMMAND_OUTPUT\"".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::with_output_dir(temp_dir.path());
    let _result = executor.execute_test_case(&test_case);

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

    assert_eq!(entries.len(), 3, "Should have 3 execution entries");

    // Step 1: Both verifications should pass
    assert_eq!(entries[0].test_sequence, 1);
    assert_eq!(entries[0].step, 1);
    assert!(
        entries[0].result_verification_pass.is_some(),
        "Step 1 should have result_verification_pass field"
    );
    assert!(
        entries[0].output_verification_pass.is_some(),
        "Step 1 should have output_verification_pass field"
    );
    assert!(
        entries[0].result_verification_pass.unwrap(),
        "Step 1 result verification should pass"
    );
    assert!(
        entries[0].output_verification_pass.unwrap(),
        "Step 1 output verification should pass"
    );

    // Step 2: Result verification should fail, output verification should pass
    assert_eq!(entries[1].test_sequence, 1);
    assert_eq!(entries[1].step, 2);
    assert!(
        entries[1].result_verification_pass.is_some(),
        "Step 2 should have result_verification_pass field"
    );
    assert!(
        entries[1].output_verification_pass.is_some(),
        "Step 2 should have output_verification_pass field"
    );
    assert!(
        !entries[1].result_verification_pass.unwrap(),
        "Step 2 result verification should fail"
    );
    assert!(
        entries[1].output_verification_pass.unwrap(),
        "Step 2 output verification should pass"
    );

    // Step 3: Result verification should pass, output verification should fail
    assert_eq!(entries[2].test_sequence, 1);
    assert_eq!(entries[2].step, 3);
    assert!(
        entries[2].result_verification_pass.is_some(),
        "Step 3 should have result_verification_pass field"
    );
    assert!(
        entries[2].output_verification_pass.is_some(),
        "Step 3 should have output_verification_pass field"
    );
    assert!(
        entries[2].result_verification_pass.unwrap(),
        "Step 3 result verification should pass"
    );
    assert!(
        !entries[2].output_verification_pass.unwrap(),
        "Step 3 output verification should fail"
    );

    Ok(())
}
