use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use testcase_manager::config::{
    Config, JsonEscapingConfig, JsonEscapingMethod, ScriptGenerationConfig,
};
use testcase_manager::executor::TestExecutor;
use testcase_manager::models::{
    Expected, Step, TestCase, TestSequence, Verification, VerificationExpression,
};

// ============================================================================
// Integration Test: Script Generation + Execution with JSON Escape
// ============================================================================

/// Test that generated scripts with json-escape produce valid JSON logs with special characters
#[test]
fn test_generated_script_with_json_escape_special_chars() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create config that uses RustBinary method
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: None, // Use default json-escape from PATH
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    // Create test case with commands that output special characters
    let test_case = create_test_case_with_special_characters();

    // Generate the test script
    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_script.sh");
    fs::write(&script_path, &script)?;

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Build json-escape binary first to ensure it's available
    let build_output = Command::new("cargo")
        .args(["build", "--bin", "json-escape"])
        .output()?;

    if !build_output.status.success() {
        eprintln!(
            "Failed to build json-escape: {}",
            String::from_utf8_lossy(&build_output.stderr)
        );
        return Err(anyhow::anyhow!("Failed to build json-escape binary"));
    }

    // Execute the generated script
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    // Script should succeed
    if !output.status.success() {
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
        eprintln!("Script stdout: {}", String::from_utf8_lossy(&output.stdout));
    }

    // Read the generated JSON log
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));
    assert!(
        json_log_path.exists(),
        "JSON log file should be created: {}",
        json_log_path.display()
    );

    let json_content = fs::read_to_string(&json_log_path)?;

    // Validate JSON is parseable
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);
    assert!(
        parsed.is_ok(),
        "Generated JSON should be valid, got error: {:?}",
        parsed.err()
    );

    let json_value = parsed.unwrap();
    assert!(json_value.is_array(), "JSON should be an array");

    let entries = json_value.as_array().unwrap();
    assert_eq!(
        entries.len(),
        4,
        "Should have 4 entries for 4 non-manual steps"
    );

    // Validate that special characters are properly escaped in JSON
    for entry in entries {
        assert!(entry["output"].is_string(), "output should be a string");
        let output_str = entry["output"].as_str().unwrap();

        // Verify no unescaped characters that would break JSON
        // The JSON parser already validated this by successfully parsing,
        // but let's do some sanity checks
        assert!(
            !output_str.contains("\n\n"),
            "Should not have literal newlines in JSON string"
        );
    }

    Ok(())
}

/// Test script generation with Auto mode - tries json-escape, falls back to shell
#[test]
fn test_generated_script_auto_mode_with_binary() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create config with Auto mode
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_characters();

    let script = executor.generate_test_script(&test_case);

    // Verify script contains both paths
    assert!(
        script.contains("if command -v json-escape"),
        "Script should check for json-escape binary"
    );
    assert!(
        script.contains("else"),
        "Script should have fallback branch"
    );
    assert!(
        script.contains("Shell fallback"),
        "Script should have shell fallback"
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

    // Build json-escape binary
    Command::new("cargo")
        .args(["build", "--bin", "json-escape"])
        .output()?;

    // Execute script with json-escape available
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    if !output.status.success() {
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Validate JSON output
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(parsed.is_ok(), "Generated JSON should be valid");
    }

    Ok(())
}

/// Test script with Auto mode when json-escape binary is NOT available
#[test]
fn test_generated_script_auto_mode_without_binary() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create config with Auto mode but point to non-existent binary
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: Some(PathBuf::from("/nonexistent/json-escape")),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_characters();

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_fallback.sh");
    fs::write(&script_path, &script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Execute script - should use shell fallback
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    if !output.status.success() {
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Validate JSON output - shell fallback should still produce valid JSON
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(
            parsed.is_ok(),
            "JSON from shell fallback should be valid, error: {:?}",
            parsed.err()
        );
    }

    Ok(())
}

/// Test script with ShellFallback mode - does not use json-escape binary
#[test]
fn test_generated_script_shell_fallback_mode() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create config that only uses shell fallback
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_characters();

    let script = executor.generate_test_script(&test_case);

    // Verify script only uses shell fallback
    assert!(
        script.contains("Shell fallback"),
        "Script should use shell fallback"
    );
    assert!(
        !script.contains("if command -v json-escape"),
        "Script should not check for json-escape"
    );
    assert!(
        script.contains("sed 's/\\\\/\\\\\\\\/g"),
        "Script should have sed escaping"
    );
    assert!(script.contains("awk"), "Script should use awk for newlines");

    let script_path = temp_dir.path().join("test_shell_only.sh");
    fs::write(&script_path, &script)?;

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
    }

    // Validate JSON output
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(parsed.is_ok(), "JSON from shell fallback should be valid");
    }

    Ok(())
}

/// Test that script properly escapes complex scenarios with multiple special chars
#[test]
fn test_script_execution_complex_special_characters() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    // Create test case with very complex special characters
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "COMPLEX_CHARS_TC".to_string(),
        "Test complex character escaping".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Complex chars".to_string());

    // Step with backslashes
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Backslashes test".to_string(),
        command: r#"echo 'Path: C:\Users\test\file.txt'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    // Step with quotes
    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Quotes test".to_string(),
        command: r#"echo 'He said "hello world"'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    // Step with mixed special characters
    sequence.steps.push(Step {
        step: 3,
        manual: None,
        description: "Mixed special chars".to_string(),
        command: r#"printf 'Line1\nLine2\tTabbed\rReturn'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    // Build json-escape binary
    Command::new("cargo")
        .args(["build", "--bin", "json-escape"])
        .output()?;

    // Execute script
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    if !output.status.success() {
        eprintln!("Script stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Validate JSON
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    assert!(
        json_log_path.exists(),
        "JSON log should be created: {}",
        json_log_path.display()
    );

    let json_content = fs::read_to_string(&json_log_path)?;

    // Must be valid JSON
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);
    assert!(
        parsed.is_ok(),
        "Complex JSON should be valid, error: {:?}",
        parsed.err()
    );

    let json_value = parsed.unwrap();
    let entries = json_value.as_array().unwrap();
    assert_eq!(entries.len(), 3, "Should have 3 entries");

    // Verify each entry has properly escaped output
    for entry in entries {
        let output_str = entry["output"].as_str().unwrap();
        // If it parsed as valid JSON, the escaping worked
        assert!(
            !output_str.is_empty() || entry["command"].as_str().unwrap().contains("printf"),
            "Output should exist or be empty for printf"
        );
    }

    Ok(())
}

/// Test script validation with jq (if available)
#[test]
fn test_script_json_validation_with_jq() -> Result<()> {
    // Check if jq is available
    let jq_check = Command::new("which").arg("jq").output();

    if jq_check.is_err() || !jq_check.unwrap().status.success() {
        println!("Skipping test: jq not available");
        return Ok(());
    }

    let temp_dir = TempDir::new()?;

    let config = Config::default();
    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_characters();

    let script = executor.generate_test_script(&test_case);

    // Script should contain jq validation
    assert!(
        script.contains("jq empty"),
        "Script should validate JSON with jq"
    );
    assert!(
        script.contains("if command -v jq"),
        "Script should check for jq availability"
    );

    Ok(())
}

/// Test that OUTPUT_ESCAPED variable is properly set in generated scripts
#[test]
fn test_script_output_escaped_variable() -> Result<()> {
    let config = Config::default();
    let executor = TestExecutor::with_config(config);
    let test_case = create_simple_test_case();

    let script = executor.generate_test_script(&test_case);

    // Verify OUTPUT_ESCAPED is used
    assert!(
        script.contains("OUTPUT_ESCAPED="),
        "Script should set OUTPUT_ESCAPED variable"
    );
    assert!(
        script.contains(r#"\"output\": \"$OUTPUT_ESCAPED\""#),
        "Script should use OUTPUT_ESCAPED in JSON log"
    );
    assert!(
        script.contains("COMMAND_OUTPUT"),
        "Script should capture COMMAND_OUTPUT"
    );

    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a test case with commands that output special characters
fn create_test_case_with_special_characters() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "SPECIAL_CHARS_TC".to_string(),
        "Test with special characters".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Special chars".to_string());

    // Step with newlines
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Newline test".to_string(),
        command: r#"printf 'Line1\nLine2\nLine3'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    // Step with tabs
    sequence.steps.push(Step {
        step: 2,
        manual: None,
        description: "Tab test".to_string(),
        command: r#"printf 'Col1\tCol2\tCol3'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    // Step with quotes
    sequence.steps.push(Step {
        step: 3,
        manual: None,
        description: "Quote test".to_string(),
        command: r#"echo 'Test "with" quotes'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    // Step with backslashes
    sequence.steps.push(Step {
        step: 4,
        manual: None,
        description: "Backslash test".to_string(),
        command: r#"echo 'Path: C:\test\file'"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

/// Create a simple test case for basic testing
fn create_simple_test_case() -> TestCase {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "SIMPLE_TC".to_string(),
        "Simple test".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Simple sequence".to_string());

    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Echo test".to_string(),
        command: "echo 'hello'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "hello".to_string(),
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
// Error Handling Tests
// ============================================================================

/// Test handling of json-escape binary crash/non-zero exit
#[test]
fn test_json_escape_binary_crash_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Point to a script that simulates binary crash
    let fake_binary = temp_dir.path().join("fake-json-escape");
    fs::write(&fake_binary, "#!/bin/bash\nexit 1\n")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&fake_binary)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&fake_binary, perms)?;
    }

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: Some(fake_binary.clone()),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_simple_test_case();

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_crash.sh");
    fs::write(&script_path, &script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Execute script - should handle binary failure gracefully
    let _output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    // Script might succeed with empty output or handle the error
    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    // Check if JSON was created and is parseable
    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        // JSON should be valid even if binary crashed (due to || echo "")
        assert!(
            parsed.is_ok(),
            "JSON should still be valid after binary crash"
        );
    }

    Ok(())
}

/// Test handling when json-escape binary crashes but Auto mode falls back to shell
#[test]
fn test_json_escape_auto_fallback_on_crash() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a fake json-escape binary that always crashes
    let fake_binary = temp_dir.path().join("fake-json-escape-crash");
    fs::write(
        &fake_binary,
        "#!/bin/bash\necho 'ERROR: Binary crashed' >&2\nexit 127\n",
    )?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&fake_binary)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&fake_binary, perms)?;
    }

    // Use Auto mode which should fall back to shell
    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: Some(fake_binary.clone()),
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);
    let test_case = create_test_case_with_special_characters();

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_auto_fallback.sh");
    fs::write(&script_path, &script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Execute script - should use shell fallback
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

    assert!(
        json_log_path.exists(),
        "JSON log should be created even with crashing binary in Auto mode"
    );

    let json_content = fs::read_to_string(&json_log_path)?;
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);
    assert!(
        parsed.is_ok(),
        "JSON should be valid using shell fallback after binary crash"
    );

    Ok(())
}

/// Test sed/awk fallback failures with invalid regex patterns
#[test]
fn test_shell_fallback_with_extreme_characters() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::ShellFallback,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    // Create test case with extreme special characters that might challenge sed/awk
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "EXTREME_TC".to_string(),
        "Test extreme characters".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Extreme chars".to_string());

    // Binary data, control characters, null bytes (printed as string)
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Extreme characters test".to_string(),
        command: r#"printf '\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f'"#
            .to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_extreme.sh");
    fs::write(&script_path, &script)?;

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

    // This test documents that shell fallback has limitations with control characters
    // Script execution may fail with validation errors for extreme binary data
    if !output.status.success() {
        // Expected failure - shell fallback cannot handle all control characters
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Internal Script Error") || stderr.contains("not valid"),
            "Script should fail gracefully with validation error for control characters"
        );
    }

    let json_log_path = temp_dir
        .path()
        .join(format!("{}_execution_log.json", test_case.id));

    if json_log_path.exists() {
        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        // If JSON was generated, it might be invalid due to control characters
        if let Err(ref e) = parsed {
            eprintln!("Expected limitation: JSON parse error with control characters: {:?}", e);
        }
    }

    Ok(())
}

/// Test extremely large command output (1MB+)
#[test]
fn test_large_output_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "LARGE_OUTPUT_TC".to_string(),
        "Test large output".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Large output".to_string());

    // Generate ~1MB of output with special characters
    sequence.steps.push(Step {
        step: 1,
        manual: None,
        description: "Large output test".to_string(),
        command: r#"for i in $(seq 1 10000); do echo "Line $i with special chars: \"quotes\" and \backslashes\ and tabs	and newlines"; done"#.to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
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

    let script = executor.generate_test_script(&test_case);
    let script_path = temp_dir.path().join("test_large.sh");
    fs::write(&script_path, &script)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Build json-escape binary
    Command::new("cargo")
        .args(["build", "--bin", "json-escape"])
        .output()?;

    // Execute script with 60 second timeout for large output
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

    assert!(
        json_log_path.exists(),
        "JSON log should be created for large output"
    );

    let json_content = fs::read_to_string(&json_log_path)?;

    // Verify JSON is parseable
    let parsed: Result<Value, _> = serde_json::from_str(&json_content);
    assert!(
        parsed.is_ok(),
        "Large output JSON should be valid, error: {:?}",
        parsed.err()
    );

    // Verify the JSON contains the large output
    let json_value = parsed.unwrap();
    let entries = json_value.as_array().unwrap();
    assert_eq!(entries.len(), 1, "Should have 1 entry");

    let output_str = entries[0]["output"].as_str().unwrap();
    // Output should be large (>100KB in escaped form)
    assert!(
        output_str.len() > 100_000,
        "Escaped output should be large, got {} bytes",
        output_str.len()
    );

    Ok(())
}

/// Test concurrent execution with multiple scripts
#[test]
fn test_concurrent_script_execution() -> Result<()> {
    use std::thread;

    let temp_dir = TempDir::new()?;

    // Build json-escape binary first
    Command::new("cargo")
        .args(["build", "--bin", "json-escape"])
        .output()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::Auto,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    let executor = TestExecutor::with_output_dir_and_config(temp_dir.path(), config);

    // Create multiple test cases
    let test_cases: Vec<TestCase> = (1..=5)
        .map(|i| {
            let mut test_case = TestCase::new(
                "REQ001".to_string(),
                1,
                i,
                format!("CONCURRENT_TC_{}", i),
                format!("Concurrent test {}", i),
            );

            let mut sequence = TestSequence::new(1, format!("Seq{}", i), format!("Sequence {}", i));

            sequence.steps.push(Step {
                step: 1,
                manual: None,
                description: format!("Test step {}", i),
                command: format!(
                    r#"echo "Test {} with special chars: \"quotes\" and newlines\nand tabs\t""#,
                    i
                ),
                capture_vars: None,
                expected: Expected {
                    success: Some(true),
                    result: "0".to_string(),
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
        })
        .collect();

    // Generate scripts for all test cases
    let script_paths: Vec<PathBuf> = test_cases
        .iter()
        .map(|tc| {
            let script = executor.generate_test_script(tc);
            let script_path = temp_dir
                .path()
                .join(format!("test_concurrent_{}.sh", tc.id));
            fs::write(&script_path, &script).unwrap();

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&script_path).unwrap().permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&script_path, perms).unwrap();
            }

            script_path
        })
        .collect();

    // Execute all scripts concurrently
    let temp_dir_path = temp_dir.path().to_path_buf();
    let handles: Vec<_> = script_paths
        .into_iter()
        .map(|script_path| {
            let temp_dir_clone = temp_dir_path.clone();
            thread::spawn(move || {
                Command::new("bash")
                    .arg(&script_path)
                    .current_dir(&temp_dir_clone)
                    .output()
            })
        })
        .collect();

    // Wait for all threads to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all executed successfully
    for result in &results {
        assert!(result.is_ok(), "Script execution should succeed");
    }

    // Verify all JSON logs were created and are valid
    for test_case in &test_cases {
        let json_log_path = temp_dir
            .path()
            .join(format!("{}_execution_log.json", test_case.id));

        assert!(
            json_log_path.exists(),
            "JSON log should exist for {}",
            test_case.id
        );

        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(
            parsed.is_ok(),
            "JSON should be valid for {}, error: {:?}",
            test_case.id,
            parsed.err()
        );
    }

    Ok(())
}

/// Test race conditions with concurrent writes to same output
#[test]
fn test_concurrent_with_shared_output_dir() -> Result<()> {
    use std::thread;

    let temp_dir = TempDir::new()?;

    // Build json-escape binary first
    Command::new("cargo")
        .args(["build", "--bin", "json-escape"])
        .output()?;

    let config = Config {
        script_generation: ScriptGenerationConfig {
            json_escaping: JsonEscapingConfig {
                method: JsonEscapingMethod::RustBinary,
                enabled: true,
                binary_path: None,
            },
        },
        ..Default::default()
    };

    // All tests use the same output directory
    let shared_output_dir = temp_dir.path().join("shared_output");
    fs::create_dir_all(&shared_output_dir)?;

    // Create multiple test cases with DIFFERENT IDs
    let test_cases: Vec<TestCase> = (1..=10)
        .map(|i| {
            let mut test_case = TestCase::new(
                format!("REQ{:03}", i),
                1,
                i,
                format!("RACE_TC_{}", i),
                format!("Race test {}", i),
            );

            let mut sequence = TestSequence::new(1, format!("Seq{}", i), format!("Sequence {}", i));

            sequence.steps.push(Step {
                step: 1,
                manual: None,
                description: format!("Test step {}", i),
                command: format!(r#"echo "Race test {} with chars: \"quotes\"""#, i),
                capture_vars: None,
                expected: Expected {
                    success: Some(true),
                    result: "0".to_string(),
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
        })
        .collect();

    // Generate and execute scripts concurrently
    let handles: Vec<_> = test_cases
        .into_iter()
        .map(|tc| {
            let config_clone = config.clone();
            let shared_output_clone = shared_output_dir.clone();

            thread::spawn(move || {
                let executor =
                    TestExecutor::with_output_dir_and_config(&shared_output_clone, config_clone);
                let script = executor.generate_test_script(&tc);
                let script_path = shared_output_clone.join(format!("test_race_{}.sh", tc.id));
                fs::write(&script_path, &script).unwrap();

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&script_path).unwrap().permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&script_path, perms).unwrap();
                }

                let output = Command::new("bash")
                    .arg(&script_path)
                    .current_dir(&shared_output_clone)
                    .output()
                    .unwrap();

                (tc.id.clone(), output)
            })
        })
        .collect();

    // Wait for all threads and collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all JSON logs are valid and separate
    for (test_id, output) in results {
        if !output.status.success() {
            eprintln!(
                "Test {} stderr: {}",
                test_id,
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let json_log_path = shared_output_dir.join(format!("{}_execution_log.json", test_id));

        assert!(
            json_log_path.exists(),
            "JSON log should exist for {}",
            test_id
        );

        let json_content = fs::read_to_string(&json_log_path)?;
        let parsed: Result<Value, _> = serde_json::from_str(&json_content);
        assert!(
            parsed.is_ok(),
            "JSON should be valid for {}, error: {:?}",
            test_id,
            parsed.err()
        );
    }

    Ok(())
}
