use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use testcase_manager::{TestCase, TestExecutor};

/// E2E test for multi-line step descriptions with YAML fold (>) syntax
///
/// This test validates:
/// 1. YAML test case deserialization with multi-line fold syntax descriptions
/// 2. Shell script generation with properly commented multi-line descriptions
/// 3. Bash syntax validation of generated scripts
/// 4. Successful execution of generated scripts
/// 5. JSON execution log generation and validation
#[test]
fn test_fold_syntax_multiline_descriptions_e2e() -> Result<()> {
    // Setup: Get project root and test case path
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_case_path = manifest_dir.join("tests/sample/test_fold_syntax.yaml");

    // Verify test case file exists
    assert!(
        test_case_path.exists(),
        "Test case file not found: {:?}",
        test_case_path
    );

    // Step 1: Load test case from YAML
    let yaml_content = fs::read_to_string(&test_case_path)?;
    let test_case: TestCase = serde_yaml::from_str(&yaml_content)?;

    // Verify test case loaded correctly
    assert_eq!(test_case.id, "TEST_MULTILINE_FOLD");
    assert_eq!(test_case.test_sequences.len(), 1);
    assert_eq!(test_case.test_sequences[0].steps.len(), 2);

    // Verify fold syntax description is present (should be folded into one line or multiple)
    let test_description = &test_case.description;
    assert!(test_description.contains("Test case demonstrating multi-line descriptions"));
    assert!(test_description.contains("YAML fold (>) syntax"));

    let sequence_description = &test_case.test_sequences[0].description;
    assert!(sequence_description.contains("This sequence tests that multi-line descriptions"));
    assert!(sequence_description.contains("fold syntax"));

    let step1_description = &test_case.test_sequences[0].steps[0].description;
    assert!(step1_description.contains("folded multi-line description"));
    assert!(step1_description.contains("second sentence"));
    assert!(step1_description.contains("third sentence"));

    let step2_description = &test_case.test_sequences[0].steps[1].description;
    assert!(step2_description.contains("Execute date command"));
    assert!(step2_description.contains("logging purposes"));
    assert!(step2_description.contains("output format"));

    // Step 2: Generate shell script
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify script has proper structure
    assert!(script.starts_with("#!/bin/bash\n"));
    assert!(script.contains("set -euo pipefail"));
    assert!(script.contains("# Test Case: TEST_MULTILINE_FOLD"));

    // Verify multi-line descriptions are present in comments
    // Fold syntax should combine lines into single comment or multiple comments
    assert!(
        script.contains("folded multi-line description"),
        "Script should contain fold syntax description text"
    );

    // Step 3: Write script to temporary file and validate bash syntax
    let temp_dir = TempDir::new()?;
    let script_path = temp_dir.path().join("test_fold_syntax.sh");
    fs::write(&script_path, &script)?;

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Validate bash syntax
    let bash_check = Command::new("bash").arg("-n").arg(&script_path).output()?;

    assert!(
        bash_check.status.success(),
        "Bash syntax validation failed: {}",
        String::from_utf8_lossy(&bash_check.stderr)
    );

    // Step 4: Execute the generated script
    let execution_output = Command::new("bash")
        .arg(&script_path)
        .current_dir(temp_dir.path())
        .output()?;

    // Verify execution succeeded
    assert!(
        execution_output.status.success(),
        "Script execution failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&execution_output.stdout),
        String::from_utf8_lossy(&execution_output.stderr)
    );

    // Step 5: Verify JSON execution log was created
    let json_log_path = temp_dir
        .path()
        .join("TEST_MULTILINE_FOLD_execution_log.json");
    assert!(
        json_log_path.exists(),
        "JSON execution log not found at {:?}",
        json_log_path
    );

    // Validate JSON log content
    let json_content = fs::read_to_string(&json_log_path)?;
    let log_entries: Vec<serde_json::Value> = serde_json::from_str(&json_content)?;

    // Verify log has 2 entries (2 steps)
    assert_eq!(
        log_entries.len(),
        2,
        "Expected 2 log entries, got {}",
        log_entries.len()
    );

    // Verify first entry (pwd command)
    assert_eq!(log_entries[0]["test_sequence"], 1);
    assert_eq!(log_entries[0]["step"], 1);
    assert_eq!(log_entries[0]["command"], "pwd");
    assert_eq!(log_entries[0]["exit_code"], 0);

    // Verify second entry (date command)
    assert_eq!(log_entries[1]["test_sequence"], 1);
    assert_eq!(log_entries[1]["step"], 2);
    assert!(log_entries[1]["command"].as_str().unwrap().contains("date"));
    assert_eq!(log_entries[1]["exit_code"], 0);

    // Verify output matches date format (YYYY-MM-DD)
    let date_output = log_entries[1]["output"].as_str().unwrap();
    let date_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$")?;
    assert!(
        date_regex.is_match(date_output.trim()),
        "Date output doesn't match expected format YYYY-MM-DD: {}",
        date_output
    );

    Ok(())
}

#[test]
fn test_fold_syntax_script_contains_description_comments() -> Result<()> {
    // Load test case
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_case_path = manifest_dir.join("tests/sample/test_fold_syntax.yaml");
    let yaml_content = fs::read_to_string(&test_case_path)?;
    let test_case: TestCase = serde_yaml::from_str(&yaml_content)?;

    // Generate script
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Verify that folded descriptions appear in the script as comments
    // The fold syntax combines multiple lines into a single paragraph

    // Check test case description appears
    assert!(
        script.contains("# Description: Test case demonstrating multi-line descriptions"),
        "Script should contain test case description"
    );

    // Check sequence description appears
    assert!(
        script.contains("# This sequence tests that multi-line descriptions")
            || script.contains("This sequence tests that multi-line descriptions"),
        "Script should contain sequence description text"
    );

    // Check step descriptions appear
    assert!(
        script.contains("folded multi-line description"),
        "Script should contain step 1 description"
    );

    assert!(
        script.contains("Execute date command"),
        "Script should contain step 2 description"
    );

    Ok(())
}

#[test]
fn test_fold_syntax_preserves_semantic_content() -> Result<()> {
    // Load test case
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_case_path = manifest_dir.join("tests/sample/test_fold_syntax.yaml");
    let yaml_content = fs::read_to_string(&test_case_path)?;
    let test_case: TestCase = serde_yaml::from_str(&yaml_content)?;

    // Verify that fold syntax preserves semantic content
    // (even though formatting may change, the words should be present)

    let step1_desc = &test_case.test_sequences[0].steps[0].description;

    // All words from the folded description should be present
    assert!(step1_desc.contains("folded"));
    assert!(step1_desc.contains("multi-line"));
    assert!(step1_desc.contains("description"));
    assert!(step1_desc.contains("second"));
    assert!(step1_desc.contains("sentence"));
    assert!(step1_desc.contains("continues"));
    assert!(step1_desc.contains("thought"));
    assert!(step1_desc.contains("third"));
    assert!(step1_desc.contains("completes"));

    Ok(())
}
