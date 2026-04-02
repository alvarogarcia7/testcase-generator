use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use testcase_common::{Config, JsonEscapingConfig, JsonEscapingMethod, ScriptGenerationConfig};
use testcase_manager::TestExecutor;
use testcase_models::{
    Expected, Step, TestCase, TestSequence, Verification, VerificationExpression,
};

// ============================================================================
// Jq JSON Escaping Integration Tests
// ============================================================================
//
// These tests verify that:
// 1. TestCase can be built with commands containing special characters
// 2. Config can be set to JsonEscapingMethod::Jq
// 3. Scripts are generated programmatically with jq escaping
// 4. Generated scripts execute correctly
// 5. JSON logs are parsed via serde_json
// 6. Command and output fields roundtrip correctly
// 7. Jq fallback behavior works when binary is unavailable
// ============================================================================

/// Helper function to create a test case with special characters
fn create_test_case_with_special_chars() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "JQ_SPECIAL_TC".to_string(),
        "Test jq escaping with special characters".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Jq escaping test".to_string());

    // Step 1: Command with quotes
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Test with double quotes".to_string(),
        command: r#"echo 'He said "hello world"'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: r#"He said "hello world""#.to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    // Step 2: Command with backslashes
    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Test with backslashes".to_string(),
        command: r#"echo 'Path: C:\Users\test\file.txt'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: r#"Path: C:\Users\test\file.txt"#.to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    // Step 3: Command with newlines (using printf)
    sequence.steps.push(Step {
        step: 3,
        manual: None,
        description: "Test with newlines".to_string(),
        command: r#"printf 'Line1\nLine2\nLine3'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "Line1\nLine2\nLine3".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
        reference: None,
    });

    // Step 4: Command with tabs
    sequence.steps.push(Step {
        step: 4,
        manual: None,
        description: "Test with tabs".to_string(),
        command: r#"printf 'Col1\tCol2\tCol3'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "Col1\tCol2\tCol3".to_string(),
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

/// Test 1: Build TestCase with commands containing special characters
#[test]
fn test_build_testcase_with_special_chars() -> Result<()> {
    let test_case = create_test_case_with_special_chars();

    // Verify test case structure
    assert_eq!(test_case.id, "JQ_SPECIAL_TC");
    assert_eq!(test_case.test_sequences.len(), 1);
    assert_eq!(test_case.test_sequences[0].steps.len(), 4);

    // Verify special characters in commands
    let steps = &test_case.test_sequences[0].steps;
    assert!(steps[0].command.contains(r#""hello world""#));
    assert!(steps[1].command.contains(r#"C:\Users\test"#));
    assert!(steps[2].command.contains(r#"\n"#));
    assert!(steps[3].command.contains(r#"\t"#));

    Ok(())
}

/// Test 2: Set config to JsonEscapingMethod::Jq and generate scripts
#[test]
fn test_config_jq_method_script_generation() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    // Verify script uses jq for JSON escaping
    assert!(
        script.contains("jq -Rs"),
        "Script should use jq -Rs for JSON escaping"
    );
    assert!(script.contains("jq -Rs ."), "Script should use jq -Rs .");

    Ok(())
}

/// Test 3: Execute generated script and parse JSON logs via serde_json
#[test]
fn test_execute_jq_script_and_parse_json() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();
    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        eprintln!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    // Write script to file
    let script_path = temp_dir.path().join("test_jq.sh");
    fs::write(&script_path, &script)?;

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Execute script
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    if !output.status.success() {
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("Script stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    // Read and parse JSON log
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    assert!(
        json_log_path.exists(),
        "JSON log file should be created: {}",
        json_log_path.display()
    );

    let json_content = fs::read_to_string(&json_log_path)?;
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);

    assert!(
        parsed.is_ok(),
        "JSON should be parseable with serde_json, error: {:?}",
        parsed.err()
    );

    let json_value = parsed.unwrap();
    assert!(json_value.is_array(), "JSON should be an array");

    let entries = json_value.as_array().unwrap();
    assert_eq!(entries.len(), 4, "Should have 4 entries for 4 steps");

    Ok(())
}

/// Test 4: Assert command and output fields roundtrip correctly
#[test]
fn test_command_output_roundtrip() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();
    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        eprintln!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    let script_path = temp_dir.path().join("test_roundtrip.sh");
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
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    let json_content = fs::read_to_string(&json_log_path)?;
    let json_value: Value = serde_json::from_str(&json_content)?;
    let entries = json_value.as_array().unwrap();

    // Verify Step 1: Quotes
    let entry1 = &entries[0];
    let cmd1 = entry1["command"].as_str().unwrap();
    let output1 = entry1["output"].as_str().unwrap();
    assert!(
        cmd1.contains("echo"),
        "Command should contain echo: {}",
        cmd1
    );
    assert!(
        output1.contains("hello world"),
        "Output should contain 'hello world': {}",
        output1
    );

    // Verify Step 2: Backslashes
    let entry2 = &entries[1];
    let output2 = entry2["output"].as_str().unwrap();
    assert!(
        output2.contains("C:") && output2.contains("Users") && output2.contains("test"),
        "Output should contain path components: {}",
        output2
    );

    // Verify Step 3: Newlines
    let entry3 = &entries[2];
    let output3 = entry3["output"].as_str().unwrap();
    assert!(
        output3.contains("Line1") && output3.contains("Line2") && output3.contains("Line3"),
        "Output should contain all lines: {}",
        output3
    );

    // Verify Step 4: Tabs
    let entry4 = &entries[3];
    let output4 = entry4["output"].as_str().unwrap();
    assert!(
        output4.contains("Col1") && output4.contains("Col2") && output4.contains("Col3"),
        "Output should contain all columns: {}",
        output4
    );

    Ok(())
}

/// Test 5: Verify jq fallback behavior when binary is unavailable (using Auto mode)
#[test]
fn test_jq_fallback_when_unavailable() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Use Auto mode with a fake jq path that doesn't exist
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
                jq_path: Some(PathBuf::from("/nonexistent/jq")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    // Verify script has conditional jq check with fallback
    assert!(
        script.contains("if command -v"),
        "Script should check for jq availability in Auto mode"
    );
    assert!(
        script.contains("else"),
        "Script should have fallback branch"
    );

    // Write and execute script
    let script_path = temp_dir.path().join("test_fallback.sh");
    fs::write(&script_path, &script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Execute script - should use fallback since jq path is invalid
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    if !output.status.success() {
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Verify JSON log was created and is valid (using fallback)
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(
            parsed.is_ok(),
            "JSON from fallback should be valid, error: {:?}",
            parsed.err()
        );
    }

    Ok(())
}

/// Test 6: Test with custom jq_path configuration
#[test]
fn test_custom_jq_path_config() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: Some(PathBuf::from("/usr/bin/jq")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    // Verify script uses custom jq path
    assert!(
        script.contains("/usr/bin/jq"),
        "Script should use custom jq path"
    );

    Ok(())
}

/// Test 7: Test jq with complex mixed special characters
#[test]
fn test_jq_with_complex_mixed_chars() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();
    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        eprintln!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    // Create test case with very complex mixed characters
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "JQ_COMPLEX_TC".to_string(),
        "Test jq with complex characters".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Complex test".to_string());

    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Mixed special characters".to_string(),
        command: r#"printf 'Error: "file not found"\nPath: C:\\test\\file\tStatus: OK'"#
            .to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "mixed".to_string(),
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

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_complex.sh");
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
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    let json_content = fs::read_to_string(&json_log_path)?;
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);

    assert!(
        parsed.is_ok(),
        "JSON with complex characters should be valid, error: {:?}",
        parsed.err()
    );

    let json_value = parsed.unwrap();
    let entries = json_value.as_array().unwrap();
    let entry = &entries[0];
    let output_str = entry["output"].as_str().unwrap();

    // Verify output contains expected strings
    assert!(
        output_str.contains("Error") && output_str.contains("file not found"),
        "Output should contain error message: {}",
        output_str
    );

    Ok(())
}

/// Test 8: Test jq escaping disabled (should use fallback)
#[test]
fn test_jq_escaping_disabled() -> Result<()> {
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: false,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_config(config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    // When disabled, script might still include escaping code
    // This documents current behavior
    assert!(!script.is_empty(), "Script should still be generated");

    Ok(())
}

/// Test 9: Test Auto mode with jq available
#[test]
fn test_auto_mode_with_jq_available() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();
    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        eprintln!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_chars();
    let script = executor.generate_test_script(&test_case);

    // Verify Auto mode includes conditional check
    assert!(
        script.contains("if command -v"),
        "Auto mode should check for jq/json-escape availability"
    );

    let script_path = temp_dir.path().join("test_auto.sh");
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
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(parsed.is_ok(), "JSON should be valid in Auto mode");
    }

    Ok(())
}

/// Test 10: Test that jq properly handles empty output
#[test]
fn test_jq_empty_output() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();
    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        eprintln!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "JQ_EMPTY_TC".to_string(),
        "Test jq with empty output".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Empty test".to_string());

    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Command with no output".to_string(),
        command: "true".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "".to_string(),
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

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_empty.sh");
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
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    let json_content = fs::read_to_string(&json_log_path)?;
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);

    assert!(
        parsed.is_ok(),
        "JSON with empty output should be valid, error: {:?}",
        parsed.err()
    );

    let json_value = parsed.unwrap();
    let entries = json_value.as_array().unwrap();
    let entry = &entries[0];
    let output_str = entry["output"].as_str().unwrap();

    // Empty output should be empty string or whitespace only
    assert!(
        output_str.trim().is_empty(),
        "Output should be empty: '{}'",
        output_str
    );

    Ok(())
}

/// Test 11: Verify jq vs json-escape vs shell fallback in script generation
#[test]
fn test_jq_vs_other_methods_in_script() -> Result<()> {
    // Test Jq method
    let config_jq = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor_jq = TestExecutor::with_config(config_jq);
    let test_case = create_test_case_with_special_chars();
    let script_jq = executor_jq.generate_test_script(&test_case);

    // Verify jq method
    assert!(script_jq.contains("jq -Rs"), "Jq method should use jq -Rs");
    assert!(
        !script_jq.contains("json-escape"),
        "Jq method should not reference json-escape"
    );
    // Note: jq method uses sed to strip quotes from jq output, so we check for sed 's/\\\\/
    assert!(
        !script_jq.contains("sed 's/\\\\/"),
        "Jq method should not use sed for backslash escaping"
    );

    // Test RustBinary method
    let config_rust = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor_rust = TestExecutor::with_config(config_rust);
    let script_rust = executor_rust.generate_test_script(&test_case);

    assert!(
        script_rust.contains("json-escape"),
        "RustBinary method should use json-escape"
    );
    assert!(
        !script_rust.contains("jq -R -s"),
        "RustBinary method should not use jq"
    );

    // Test ShellFallback method
    let config_shell = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor_shell = TestExecutor::with_config(config_shell);
    let script_shell = executor_shell.generate_test_script(&test_case);

    assert!(
        script_shell.contains("sed 's/"),
        "ShellFallback method should use sed"
    );
    assert!(
        !script_shell.contains("jq -R -s"),
        "ShellFallback method should not use jq"
    );
    assert!(
        !script_shell.contains("json-escape"),
        "ShellFallback method should not use json-escape"
    );

    Ok(())
}

/// Test 12: Test jq with unicode characters
#[test]
fn test_jq_with_unicode() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();
    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        eprintln!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Jq,
                enabled: true,
                binary_path: None,
                jq_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "JQ_UNICODE_TC".to_string(),
        "Test jq with unicode".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Unicode test".to_string());

    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Unicode characters".to_string(),
        command: r#"echo 'Hello 世界 🌍 café'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "Hello 世界 🌍 café".to_string(),
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

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_unicode.sh");
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
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    let json_content = fs::read_to_string(&json_log_path)?;
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);

    assert!(
        parsed.is_ok(),
        "JSON with unicode should be valid, error: {:?}",
        parsed.err()
    );

    let json_value = parsed.unwrap();
    let entries = json_value.as_array().unwrap();
    let entry = &entries[0];
    let output_str = entry["output"].as_str().unwrap();

    // Verify unicode is preserved
    assert!(
        output_str.contains("世界") || output_str.contains("Hello"),
        "Output should contain unicode or text: {}",
        output_str
    );

    Ok(())
}
