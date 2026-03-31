use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;
use tempfile::TempDir;
use testcase_manager::{
    CaptureVar, CaptureVarsFormat, Expected, Step, TestCase, TestExecutor, TestSequence,
    TestStepExecutionEntry, Verification, VerificationExpression,
};

// ============================================================================
// Helper Functions
// ============================================================================

fn create_test_case_with_sequence_variables() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "SEQ_VAR_TC_001".to_string(),
        "Test case with sequence variables".to_string(),
    );

    let mut variables = BTreeMap::new();
    variables.insert("BASE_URL".to_string(), "http://localhost".to_string());
    variables.insert("TIMEOUT".to_string(), "30".to_string());

    let mut sequence = TestSequence::new(
        1,
        "VariableSequence".to_string(),
        "Sequence with variables".to_string(),
    );
    sequence.variables = Some(variables);

    let step = Step {
        step: 1,
        manual: None,
        description: "Use sequence variables".to_string(),
        command: "echo ${BASE_URL} ${TIMEOUT}".to_string(),
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

fn create_test_case_with_legacy_capture_vars() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ002".to_string(),
        1,
        2,
        "LEGACY_CAPTURE_TC_001".to_string(),
        "Test case with legacy capture_vars".to_string(),
    );

    let mut sequence =
        TestSequence::new(1, "CaptureSequence".to_string(), "Capture test".to_string());

    let mut capture_map = BTreeMap::new();
    capture_map.insert("MY_VAR".to_string(), "value=(.+)".to_string());

    let step1 = Step {
        step: 1,
        manual: None,
        description: "Capture variable".to_string(),
        command: "echo 'value=hello123'".to_string(),
        capture_vars: Some(CaptureVarsFormat::Legacy(capture_map)),
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
        manual: None,
        description: "Use captured variable".to_string(),
        command: "echo ${MY_VAR}".to_string(),
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
    sequence.steps.push(step2);

    test_case.test_sequences.push(sequence);
    test_case
}

fn create_test_case_with_new_format_command_mode() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ003".to_string(),
        1,
        3,
        "NEW_CAPTURE_CMD_TC_001".to_string(),
        "Test case with new format command-based capture".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "CommandCaptureSequence".to_string(),
        "Command capture test".to_string(),
    );

    let capture_vars = vec![CaptureVar {
        name: "CMD_OUTPUT".to_string(),
        capture: None,
        command: Some("echo 'command_result_123'".to_string()),
    }];

    let step1 = Step {
        step: 1,
        manual: None,
        description: "Capture via command".to_string(),
        command: "echo 'step1'".to_string(),
        capture_vars: Some(CaptureVarsFormat::New(capture_vars)),
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
        manual: None,
        description: "Use command-captured variable".to_string(),
        command: "echo ${CMD_OUTPUT}".to_string(),
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
    sequence.steps.push(step2);

    test_case.test_sequences.push(sequence);
    test_case
}

fn create_test_case_with_manual_and_auto_steps() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ004".to_string(),
        1,
        4,
        "MANUAL_AUTO_TC_001".to_string(),
        "Test case with manual and automated steps".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "MixedSequence".to_string(),
        "Mixed manual and auto steps".to_string(),
    );

    let step1 = Step {
        step: 1,
        manual: None,
        description: "Automated step 1".to_string(),
        command: "echo 'auto1'".to_string(),
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
        description: "Automated step 3".to_string(),
        command: "echo 'auto3'".to_string(),
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

fn create_multi_sequence_test_case() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ005".to_string(),
        1,
        5,
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

    test_case.test_sequences.push(sequence1);
    test_case.test_sequences.push(sequence2);
    test_case
}

fn create_test_case_with_special_characters() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ006".to_string(),
        1,
        6,
        "SPECIAL_CHAR_TC_001".to_string(),
        "Test case with special characters".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "SpecialChars".to_string(),
        "Test with special characters".to_string(),
    );

    sequence.steps.push(Step {
        step: 1,
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
        step: 2,
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
        step: 3,
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
        step: 4,
        manual: None,
        description: "Command with Unicode".to_string(),
        command: "echo '你好世界 🚀'".to_string(),
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
    test_case
}

// ============================================================================
// Variable Tests
// ============================================================================

#[test]
fn test_sequence_variables_in_generated_script() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_sequence_variables();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let script = executor.generate_test_script(&test_case);

    assert!(
        script.contains("BASE_URL="),
        "Script should contain BASE_URL variable initialization"
    );
    assert!(
        script.contains("TIMEOUT="),
        "Script should contain TIMEOUT variable initialization"
    );
    assert!(
        script.contains("CAPTURED_VAR_NAMES"),
        "Script should contain CAPTURED_VAR_NAMES for variable tracking"
    );
    assert!(
        script.contains("http://localhost"),
        "Script should contain BASE_URL value"
    );
    assert!(script.contains("30"), "Script should contain TIMEOUT value");

    Ok(())
}

#[test]
fn test_capture_vars_legacy_format_execution() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_legacy_capture_vars();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);
    assert!(
        result.is_ok(),
        "Test execution should succeed: {:?}",
        result.err()
    );

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

    assert_eq!(entries.len(), 2, "Should have 2 execution entries");
    assert_eq!(entries[0].step, 1);
    assert_eq!(entries[1].step, 2);

    assert!(
        entries[1].output.contains("hello123"),
        "Step 2 output should contain the captured value"
    );

    Ok(())
}

#[test]
fn test_capture_vars_new_format_command_mode() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_new_format_command_mode();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);
    assert!(
        result.is_ok(),
        "Test execution should succeed: {:?}",
        result.err()
    );

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

    assert_eq!(entries.len(), 2, "Should have 2 execution entries");

    assert!(
        entries[1].output.contains("command_result_123"),
        "Step 2 output should contain the command-captured value"
    );

    Ok(())
}

// ============================================================================
// User Interaction Tests
// ============================================================================

#[test]
fn test_manual_steps_excluded_from_json_log() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_manual_and_auto_steps();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);
    assert!(
        result.is_ok(),
        "Test execution should succeed: {:?}",
        result.err()
    );

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;

    assert_eq!(
        entries.len(),
        2,
        "Should only have 2 entries (step 1 and 3, excluding manual step 2)"
    );
    assert_eq!(entries[0].step, 1, "First entry should be step 1");
    assert_eq!(entries[1].step, 3, "Second entry should be step 3");

    let step1_log = temp_dir
        .path()
        .join(format!("{}_seq_{}_step_{}.actual.log", test_case.id, 1, 1));
    let step2_log = temp_dir
        .path()
        .join(format!("{}_seq_{}_step_{}.actual.log", test_case.id, 1, 2));
    let step3_log = temp_dir
        .path()
        .join(format!("{}_seq_{}_step_{}.actual.log", test_case.id, 1, 3));

    assert!(step1_log.exists(), "Step 1 log file should exist");
    assert!(
        !step2_log.exists(),
        "Step 2 (manual) log file should not exist"
    );
    assert!(step3_log.exists(), "Step 3 log file should exist");

    Ok(())
}

#[test]
fn test_manual_step_prompt_in_generated_script() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_manual_and_auto_steps();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let script = executor.generate_test_script(&test_case);

    assert!(
        script.contains("read_verification"),
        "Script should contain read_verification function for manual steps"
    );

    assert!(
        script.contains("Manual verification step"),
        "Script should contain manual step description"
    );

    let manual_step_section = script
        .split("Manual verification step")
        .nth(1)
        .unwrap_or("");
    assert!(
        !manual_step_section.contains("VERIFICATION_RESULT_PASS"),
        "Manual step section should not contain automatic verification result"
    );

    Ok(())
}

// ============================================================================
// JSON Log Archival Tests
// ============================================================================

#[test]
fn test_json_log_source_yaml_sha256() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_sequence_variables();

    let yaml_content = serde_yaml::to_string(&test_case)?;
    let yaml_bytes = yaml_content.as_bytes();

    let executor = TestExecutor::with_output_dir(temp_dir.path());
    let script = executor.generate_test_script_from_yaml(&test_case, yaml_bytes);

    let expected_hash = testcase_execution::compute_yaml_sha256(yaml_bytes);
    assert!(
        script.contains(&format!("SOURCE_YAML_SHA256=\"{}\"", expected_hash)),
        "Script should contain source YAML SHA-256 hash"
    );

    let script_path = temp_dir.path().join("test_script.sh");
    fs::write(&script_path, &script)?;

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

    if !output.status.success() {
        eprintln!("Script execution failed:");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let entries: Vec<Value> = serde_json::from_str(&json_content)?;

    for (i, entry) in entries.iter().enumerate() {
        let source_hash = entry.get("source_yaml_sha256");
        assert!(
            source_hash.is_some(),
            "Entry {} should have source_yaml_sha256 field",
            i
        );
        let hash_value = source_hash.unwrap().as_str().unwrap();
        assert_eq!(
            hash_value, expected_hash,
            "Entry {} should have matching SHA-256 hash",
            i
        );
    }

    Ok(())
}

#[test]
fn test_json_log_special_characters_roundtrip() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_test_case_with_special_characters();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);
    assert!(
        result.is_ok(),
        "Test execution should succeed: {:?}",
        result.err()
    );

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;

    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;
    assert_eq!(entries.len(), 4, "Should have 4 execution entries");

    assert_eq!(
        entries[0].command, r#"echo "double quoted text""#,
        "Command with double quotes should match"
    );
    assert_eq!(
        entries[1].command, r#"echo "path\\to\\file""#,
        "Command with backslashes should match"
    );
    assert_eq!(
        entries[2].command, "echo \"line1\"\necho \"line2\"",
        "Command with newlines should match"
    );
    assert!(
        entries[3].command.contains("你好世界") && entries[3].command.contains("🚀"),
        "Command with Unicode should match"
    );

    Ok(())
}

#[test]
fn test_json_log_structure_compliance() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_case = create_multi_sequence_test_case();
    let executor = TestExecutor::with_output_dir(temp_dir.path());

    let result = executor.execute_test_case(&test_case);
    assert!(
        result.is_ok(),
        "Test execution should succeed: {:?}",
        result.err()
    );

    let log_file = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(log_file.exists(), "JSON log file should be created");

    let json_content = fs::read_to_string(&log_file)?;
    let json_value: Value = serde_json::from_str(&json_content)?;

    assert!(json_value.is_array(), "Root must be an array");

    let array = json_value.as_array().unwrap();
    assert!(!array.is_empty(), "Array should not be empty");

    for (i, item) in array.iter().enumerate() {
        assert!(item.is_object(), "Entry {} must be an object", i);
        let obj = item.as_object().unwrap();

        assert!(
            obj.contains_key("test_sequence"),
            "Entry {} must have test_sequence field",
            i
        );
        assert!(obj.contains_key("step"), "Entry {} must have step field", i);
        assert!(
            obj.contains_key("command"),
            "Entry {} must have command field",
            i
        );
        assert!(
            obj.contains_key("exit_code"),
            "Entry {} must have exit_code field",
            i
        );
        assert!(
            obj.contains_key("output"),
            "Entry {} must have output field",
            i
        );

        assert!(
            obj["test_sequence"].is_number(),
            "Entry {} test_sequence must be a number",
            i
        );
        assert!(obj["step"].is_number(), "Entry {} step must be a number", i);
        assert!(
            obj["command"].is_string(),
            "Entry {} command must be a string",
            i
        );
        assert!(
            obj["exit_code"].is_number(),
            "Entry {} exit_code must be a number",
            i
        );
        assert!(
            obj["output"].is_string(),
            "Entry {} output must be a string",
            i
        );

        assert!(
            obj.contains_key("result_verification_pass"),
            "Entry {} must have result_verification_pass field",
            i
        );
        assert!(
            obj.contains_key("output_verification_pass"),
            "Entry {} must have output_verification_pass field",
            i
        );
        assert!(
            obj["result_verification_pass"].is_boolean(),
            "Entry {} result_verification_pass must be boolean",
            i
        );
        assert!(
            obj["output_verification_pass"].is_boolean(),
            "Entry {} output_verification_pass must be boolean",
            i
        );
    }

    let entries: Vec<TestStepExecutionEntry> = serde_json::from_str(&json_content)?;
    assert_eq!(entries[0].test_sequence, 1);
    assert_eq!(entries[0].step, 1);
    assert_eq!(entries[1].test_sequence, 1);
    assert_eq!(entries[1].step, 2);
    assert_eq!(entries[2].test_sequence, 2);
    assert_eq!(entries[2].step, 1);

    Ok(())
}
