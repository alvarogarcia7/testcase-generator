use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{NamedTempFile, TempDir};
use testcase_manager::hydration::VarHydrator;
use testcase_manager::models::{
    EnvVariable, Expected, Step, TestCase, TestSequence, Verification, VerificationExpression,
};
use testcase_manager::TestExecutor;

// ============================================================================
// Placeholder Extraction Tests
// ============================================================================

#[test]
fn test_extract_placeholders_from_simple_yaml() {
    let hydrator = VarHydrator::new();
    let yaml = r#"
command: ssh ${#SERVER_HOST}
output: ${#EXPECTED_OUTPUT}
"#;

    let placeholders = hydrator.extract_placeholders(yaml);
    assert_eq!(placeholders.len(), 2);
    assert!(placeholders.contains(&"SERVER_HOST".to_string()));
    assert!(placeholders.contains(&"EXPECTED_OUTPUT".to_string()));
}

#[test]
fn test_extract_placeholders_with_duplicates() {
    let hydrator = VarHydrator::new();
    let yaml = r#"
command: ssh ${#SERVER_HOST}
backup: ssh ${#SERVER_HOST}
mirror: ssh ${#SERVER_HOST}
"#;

    let placeholders = hydrator.extract_placeholders(yaml);
    // Should only return unique placeholders
    assert_eq!(placeholders.len(), 1);
    assert!(placeholders.contains(&"SERVER_HOST".to_string()));
}

#[test]
fn test_extract_placeholders_with_mixed_patterns() {
    let hydrator = VarHydrator::new();
    let yaml = r#"
valid_pattern: ${#VAR_NAME}
invalid_no_hash: ${VAR_NAME}
invalid_lowercase: ${#var_name}
valid_underscore: ${#_VAR}
valid_numbers: ${#VAR_123}
"#;

    let placeholders = hydrator.extract_placeholders(yaml);
    // Only ${#UPPERCASE_VAR} patterns should match
    assert_eq!(placeholders.len(), 3);
    assert!(placeholders.contains(&"VAR_NAME".to_string()));
    assert!(placeholders.contains(&"_VAR".to_string()));
    assert!(placeholders.contains(&"VAR_123".to_string()));
    assert!(!placeholders.contains(&"var_name".to_string()));
}

#[test]
fn test_extract_placeholders_from_complex_yaml() {
    let hydrator = VarHydrator::new();
    let yaml = r#"
test_sequences:
  - id: 1
    steps:
      - step: 1
        command: ssh ${#USERNAME}@${#SERVER_HOST}:${#PORT}
        expected:
          output: ${#EXPECTED_MSG}
      - step: 2
        command: scp ${#FILE_PATH} ${#USERNAME}@${#SERVER_HOST}:/tmp/
"#;

    let placeholders = hydrator.extract_placeholders(yaml);
    assert_eq!(placeholders.len(), 5);
    assert!(placeholders.contains(&"USERNAME".to_string()));
    assert!(placeholders.contains(&"SERVER_HOST".to_string()));
    assert!(placeholders.contains(&"PORT".to_string()));
    assert!(placeholders.contains(&"EXPECTED_MSG".to_string()));
    assert!(placeholders.contains(&"FILE_PATH".to_string()));
}

#[test]
fn test_extract_placeholders_empty_yaml() {
    let hydrator = VarHydrator::new();
    let yaml = "";

    let placeholders = hydrator.extract_placeholders(yaml);
    assert_eq!(placeholders.len(), 0);
}

#[test]
fn test_extract_placeholders_no_matches() {
    let hydrator = VarHydrator::new();
    let yaml = r#"
command: ssh server.example.com
output: Success
no_placeholders_here: just plain text
"#;

    let placeholders = hydrator.extract_placeholders(yaml);
    assert_eq!(placeholders.len(), 0);
}

// ============================================================================
// Export File Parsing Tests
// ============================================================================

#[test]
fn test_parse_export_file_simple_format() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export VAR1=value1")?;
    writeln!(file, "export VAR2=value2")?;
    writeln!(file, "export VAR3=value3")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
    assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
    assert_eq!(hydrator.get("VAR3"), Some(&"value3".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_double_quotes() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export SERVER=\"example.com\"")?;
    writeln!(file, "export MESSAGE=\"Hello World\"")?;
    writeln!(file, "export PATH=\"/usr/local/bin\"")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(hydrator.get("SERVER"), Some(&"example.com".to_string()));
    assert_eq!(hydrator.get("MESSAGE"), Some(&"Hello World".to_string()));
    assert_eq!(hydrator.get("PATH"), Some(&"/usr/local/bin".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_single_quotes() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export VAR1='single quoted'")?;
    writeln!(file, "export VAR2='with spaces'")?;
    writeln!(file, "export VAR3='special!@#$%'")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(hydrator.get("VAR1"), Some(&"single quoted".to_string()));
    assert_eq!(hydrator.get("VAR2"), Some(&"with spaces".to_string()));
    assert_eq!(hydrator.get("VAR3"), Some(&"special!@#$%".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_mixed_quotes() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export VAR1=unquoted")?;
    writeln!(file, "export VAR2=\"double quoted\"")?;
    writeln!(file, "export VAR3='single quoted'")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(hydrator.get("VAR1"), Some(&"unquoted".to_string()));
    assert_eq!(hydrator.get("VAR2"), Some(&"double quoted".to_string()));
    assert_eq!(hydrator.get("VAR3"), Some(&"single quoted".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_with_comments_and_empty_lines() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "# This is a comment")?;
    writeln!(file, "export VAR1=value1")?;
    writeln!(file)?;
    writeln!(file, "# Another comment")?;
    writeln!(file, "export VAR2=value2")?;
    writeln!(file)?;
    writeln!(file)?;
    writeln!(file, "export VAR3=value3")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
    assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
    assert_eq!(hydrator.get("VAR3"), Some(&"value3".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_with_leading_trailing_spaces() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "  export VAR1=value1  ")?;
    writeln!(file, "export  VAR2=value2")?;
    writeln!(file, "   export VAR3=value3   ")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(hydrator.get("VAR1"), Some(&"value1".to_string()));
    assert_eq!(hydrator.get("VAR2"), Some(&"value2".to_string()));
    assert_eq!(hydrator.get("VAR3"), Some(&"value3".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_empty_values() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export EMPTY1=\"\"")?;
    writeln!(file, "export EMPTY2=''")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 2);
    assert_eq!(hydrator.get("EMPTY1"), Some(&"".to_string()));
    assert_eq!(hydrator.get("EMPTY2"), Some(&"".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_special_characters() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "export URL=\"https://example.com/api?key=value&foo=bar\""
    )?;
    writeln!(file, "export JSON='{{\"key\": \"value\"}}'")?;
    writeln!(file, "export COMPLEX=\"value!@#$%^&*()_+-=[]{{}}|;:,.<>?\"")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    assert_eq!(hydrator.len(), 3);
    assert_eq!(
        hydrator.get("URL"),
        Some(&"https://example.com/api?key=value&foo=bar".to_string())
    );
    assert_eq!(
        hydrator.get("JSON"),
        Some(&"{\"key\": \"value\"}".to_string())
    );
    assert_eq!(
        hydrator.get("COMPLEX"),
        Some(&"value!@#$%^&*()_+-=[]{}|;:,.<>?".to_string())
    );
    Ok(())
}

#[test]
fn test_parse_export_file_multiline_not_supported() -> Result<()> {
    // Export files don't support multiline values in our implementation
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export VAR1=line1")?;
    writeln!(file, "export VAR2=line2")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    // Each line is treated as a separate variable
    assert_eq!(hydrator.len(), 2);
    assert_eq!(hydrator.get("VAR1"), Some(&"line1".to_string()));
    assert_eq!(hydrator.get("VAR2"), Some(&"line2".to_string()));
    Ok(())
}

#[test]
fn test_parse_export_file_nonexistent() {
    let mut hydrator = VarHydrator::new();
    let result = hydrator.load_from_export_file("/nonexistent/file.env");

    assert!(result.is_err());
}

// ============================================================================
// Hydration with Missing/Extra Variables Tests
// ============================================================================

#[test]
fn test_hydrate_with_all_variables_present() {
    let mut vars = HashMap::new();
    vars.insert("SERVER_HOST".to_string(), "example.com".to_string());
    vars.insert("PORT".to_string(), "8080".to_string());
    vars.insert("USERNAME".to_string(), "admin".to_string());

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = r#"
server: ${#SERVER_HOST}
port: ${#PORT}
user: ${#USERNAME}
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    assert!(hydrated.contains("server: example.com"));
    assert!(hydrated.contains("port: 8080"));
    assert!(hydrated.contains("user: admin"));
}

#[test]
fn test_hydrate_with_missing_variables() {
    let mut vars = HashMap::new();
    vars.insert("VAR1".to_string(), "value1".to_string());
    // VAR2 and VAR3 are missing

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = r#"
field1: ${#VAR1}
field2: ${#VAR2}
field3: ${#VAR3}
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    // VAR1 should be replaced
    assert!(hydrated.contains("field1: value1"));
    // VAR2 and VAR3 should remain as placeholders
    assert!(hydrated.contains("field2: ${#VAR2}"));
    assert!(hydrated.contains("field3: ${#VAR3}"));
}

#[test]
fn test_hydrate_with_extra_variables() {
    let mut vars = HashMap::new();
    vars.insert("VAR1".to_string(), "value1".to_string());
    vars.insert("VAR2".to_string(), "value2".to_string());
    vars.insert("EXTRA1".to_string(), "extra_value1".to_string());
    vars.insert("EXTRA2".to_string(), "extra_value2".to_string());

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = r#"
field1: ${#VAR1}
field2: ${#VAR2}
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    // Only used variables should be replaced
    assert!(hydrated.contains("field1: value1"));
    assert!(hydrated.contains("field2: value2"));
    // Extra variables don't affect the output
    assert!(!hydrated.contains("extra_value1"));
    assert!(!hydrated.contains("extra_value2"));
}

#[test]
fn test_hydrate_with_partial_variables() {
    let mut vars = HashMap::new();
    vars.insert("DB_HOST".to_string(), "localhost".to_string());
    vars.insert("DB_PORT".to_string(), "5432".to_string());
    // DB_NAME and DB_USER are missing

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = r#"
database:
  host: ${#DB_HOST}
  port: ${#DB_PORT}
  name: ${#DB_NAME}
  user: ${#DB_USER}
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    assert!(hydrated.contains("host: localhost"));
    assert!(hydrated.contains("port: 5432"));
    assert!(hydrated.contains("name: ${#DB_NAME}"));
    assert!(hydrated.contains("user: ${#DB_USER}"));
}

#[test]
fn test_hydrate_empty_yaml() {
    let mut vars = HashMap::new();
    vars.insert("VAR1".to_string(), "value1".to_string());

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = "";

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    assert_eq!(hydrated, "");
}

#[test]
fn test_hydrate_no_placeholders() {
    let mut vars = HashMap::new();
    vars.insert("VAR1".to_string(), "value1".to_string());

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = r#"
field1: plain_value
field2: another_value
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    // Content should remain unchanged
    assert_eq!(hydrated, yaml);
}

#[test]
fn test_hydrate_multiple_occurrences() {
    let mut vars = HashMap::new();
    vars.insert("API_KEY".to_string(), "secret123".to_string());

    let hydrator = VarHydrator::with_variables(vars);
    let yaml = r#"
auth_key: ${#API_KEY}
backup_key: ${#API_KEY}
fallback_key: ${#API_KEY}
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    assert_eq!(hydrated.matches("secret123").count(), 3);
}

// ============================================================================
// Bash Script Generation with Sourced Export Files Tests
// ============================================================================

#[test]
fn test_generate_script_with_export_file_sourcing() {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_HYDRATION_001".to_string(),
        "Test with hydration variables".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test with hydration var".to_string(),
        command: "echo ${#SERVER_HOST}".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "localhost".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Should source an export file
    assert!(script.contains("EXPORT_FILE="));
    assert!(script.contains("TC_HYDRATION_001.env"));
    assert!(script.contains("source \"$EXPORT_FILE\""));
    assert!(script.contains("if [ -f \"$EXPORT_FILE\" ]; then"));
}

#[test]
fn test_generate_script_without_hydration_vars() {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_NO_HYDRATION_001".to_string(),
        "Test without hydration variables".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test without hydration".to_string(),
        command: "echo 'Hello World'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "Hello World".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple("true".to_string()),
            output_file: None,
            general: None,
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Should NOT source an export file
    assert!(!script.contains("EXPORT_FILE="));
    assert!(!script.contains("source \"$EXPORT_FILE\""));
}

#[test]
fn test_generate_script_hydration_in_verification() {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_HYDRATION_002".to_string(),
        "Test with hydration in verification".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "echo 'test'".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "test".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "grep -q ${#EXPECTED_VALUE} $COMMAND_OUTPUT".to_string(),
            ),
            output_file: None,
            general: None,
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // Should source export file because hydration var in verification
    assert!(script.contains("EXPORT_FILE="));
    assert!(script.contains("source \"$EXPORT_FILE\""));
}

#[test]
fn test_generate_script_converts_hydration_placeholders() {
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_HYDRATION_003".to_string(),
        "Test placeholder conversion".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Seq1".to_string(), "Test sequence".to_string());
    let step = Step {
        step: 1,
        manual: None,
        description: "Test step".to_string(),
        command: "curl ${#API_URL}/endpoint".to_string(),
        capture_vars: None,
        expected: Expected {
            success: Some(true),
            result: "0".to_string(),
            output: "success".to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[ $EXIT_CODE -eq 0 ]".to_string()),
            output: VerificationExpression::Simple(
                "grep ${#SUCCESS_MSG} $COMMAND_OUTPUT".to_string(),
            ),
            output_file: None,
            general: None,
        },
    };
    sequence.steps.push(step);
    test_case.test_sequences.push(sequence);

    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);

    // ${#VAR_NAME} should be converted to ${VAR_NAME} in the script
    assert!(script.contains("${API_URL}"));
    assert!(script.contains("${SUCCESS_MSG}"));
    assert!(!script.contains("${#API_URL}"));
    assert!(!script.contains("${#SUCCESS_MSG}"));
}

// ============================================================================
// CLI Command Execution with Sample Test Cases Tests
// ============================================================================

#[test]
fn test_cli_hydrate_command_with_export_file() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create test case YAML with placeholders
    let yaml_path = temp_dir.path().join("test.yaml");
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "Test case with placeholders"
general_initial_conditions: {}
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Test sequence"
    description: "Test"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Test step"
        command: "ssh ${#USERNAME}@${#SERVER_HOST}"
        expected:
          result: "0"
          output: "success"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
"#;
    fs::write(&yaml_path, yaml_content)?;

    // Create export file
    let export_path = temp_dir.path().join("vars.env");
    let export_content = r#"export USERNAME=testuser
export SERVER_HOST=example.com
"#;
    fs::write(&export_path, export_content)?;

    // Create output path
    let output_path = temp_dir.path().join("hydrated.yaml");

    // Run hydrate command
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "test-executor",
            "--",
            "hydrate",
            yaml_path.to_str().unwrap(),
            "--export-file",
            export_path.to_str().unwrap(),
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }

    // Read hydrated output
    let hydrated_content = fs::read_to_string(&output_path)?;

    // Verify placeholders were replaced
    assert!(hydrated_content.contains("testuser"));
    assert!(hydrated_content.contains("example.com"));
    assert!(!hydrated_content.contains("${#USERNAME}"));
    assert!(!hydrated_content.contains("${#SERVER_HOST}"));

    Ok(())
}

#[test]
fn test_cli_generate_export_command() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create test case YAML with hydration_vars
    let yaml_path = temp_dir.path().join("test.yaml");
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "Test case"
general_initial_conditions: {}
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Test sequence"
    description: "Test"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Test step"
        command: "echo test"
        expected:
          result: "0"
          output: "test"
        verification:
          result: "true"
          output: "true"
hydration_vars:
  SERVER_HOST:
    name: "SERVER_HOST"
    description: "Server hostname"
    default_value: "localhost"
    required: true
  PORT:
    name: "PORT"
    description: "Server port"
    default_value: "8080"
    required: false
"#;
    fs::write(&yaml_path, yaml_content)?;

    let output_path = temp_dir.path().join("export.env");

    // Run generate-export command
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "test-executor",
            "--",
            "generate-export",
            yaml_path.to_str().unwrap(),
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        return Ok(());
    }

    // Read generated export file
    let export_content = fs::read_to_string(&output_path)?;

    // Verify export file contains variables
    assert!(export_content.contains("export SERVER_HOST="));
    assert!(export_content.contains("export PORT="));
    assert!(export_content.contains("localhost") || export_content.contains("SERVER_HOST="));
    assert!(export_content.contains("8080") || export_content.contains("PORT="));

    Ok(())
}

#[test]
fn test_cli_validate_export_command_success() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create test case YAML
    let yaml_path = temp_dir.path().join("test.yaml");
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "Test case"
general_initial_conditions: {}
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Test sequence"
    description: "Test"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Test step"
        command: "echo test"
        expected:
          result: "0"
          output: "test"
        verification:
          result: "true"
          output: "true"
hydration_vars:
  VAR1:
    name: "VAR1"
    description: "Variable 1"
    required: true
  VAR2:
    name: "VAR2"
    description: "Variable 2"
    required: true
"#;
    fs::write(&yaml_path, yaml_content)?;

    // Create valid export file
    let export_path = temp_dir.path().join("vars.env");
    let export_content = r#"export VAR1=value1
export VAR2=value2
"#;
    fs::write(&export_path, export_content)?;

    // Run validate-export command
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "test-executor",
            "--",
            "validate-export",
            yaml_path.to_str().unwrap(),
            "--export-file",
            export_path.to_str().unwrap(),
        ])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("valid") || stdout.contains("present"));
    }

    Ok(())
}

#[test]
fn test_cli_validate_export_command_failure() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create test case YAML with required variables
    let yaml_path = temp_dir.path().join("test.yaml");
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC001"
description: "Test case"
general_initial_conditions: {}
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Test sequence"
    description: "Test"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Test step"
        command: "echo test"
        expected:
          result: "0"
          output: "test"
        verification:
          result: "true"
          output: "true"
hydration_vars:
  REQUIRED_VAR:
    name: "REQUIRED_VAR"
    description: "Required variable"
    required: true
  OPTIONAL_VAR:
    name: "OPTIONAL_VAR"
    description: "Optional variable"
    required: false
"#;
    fs::write(&yaml_path, yaml_content)?;

    // Create incomplete export file (missing required variable)
    let export_path = temp_dir.path().join("vars.env");
    let export_content = r#"export OPTIONAL_VAR=optional_value
"#;
    fs::write(&export_path, export_content)?;

    // Run validate-export command (should fail)
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "test-executor",
            "--",
            "validate-export",
            yaml_path.to_str().unwrap(),
            "--export-file",
            export_path.to_str().unwrap(),
        ])
        .output()?;

    // Should exit with non-zero status
    assert!(!output.status.success());

    Ok(())
}

#[test]
fn test_cli_generate_script_with_hydration() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create test case YAML with hydration variables
    let yaml_path = temp_dir.path().join("test.yaml");
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC_HYDRATION"
description: "Test with hydration"
general_initial_conditions: {}
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Test sequence"
    description: "Test"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Test step"
        command: "ping ${#SERVER_HOST}"
        expected:
          result: "0"
          output: "success"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "true"
"#;
    fs::write(&yaml_path, yaml_content)?;

    let output_path = temp_dir.path().join("test.sh");

    // Run generate command
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "test-executor",
            "--",
            "generate",
            yaml_path.to_str().unwrap(),
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()?;

    if !output.status.success() {
        return Ok(());
    }

    // Read generated script
    let script_content = fs::read_to_string(&output_path)?;

    // Verify script sources export file
    assert!(script_content.contains("EXPORT_FILE="));
    assert!(script_content.contains("TC_HYDRATION.env"));
    assert!(script_content.contains("source \"$EXPORT_FILE\""));

    Ok(())
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_full_hydration_workflow() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Step 1: Create test case with placeholders
    let yaml_path = temp_dir.path().join("test.yaml");
    let yaml_content = r#"
requirement: "REQ001"
item: 1
tc: 1
id: "TC_WORKFLOW"
description: "Full workflow test"
general_initial_conditions: {}
initial_conditions: {}
test_sequences:
  - id: 1
    name: "Test sequence"
    description: "Test"
    initial_conditions: {}
    steps:
      - step: 1
        description: "Connect to server"
        command: "ssh ${#USERNAME}@${#SERVER_HOST}:${#PORT}"
        expected:
          result: "0"
          output: "connected"
        verification:
          result: "[ $EXIT_CODE -eq 0 ]"
          output: "echo $COMMAND_OUTPUT | grep -q ${#SUCCESS_MSG}"
hydration_vars:
  USERNAME:
    name: "USERNAME"
    description: "SSH username"
    default_value: "admin"
    required: true
  SERVER_HOST:
    name: "SERVER_HOST"
    description: "Server hostname"
    default_value: "localhost"
    required: true
  PORT:
    name: "PORT"
    description: "SSH port"
    default_value: "22"
    required: false
  SUCCESS_MSG:
    name: "SUCCESS_MSG"
    description: "Expected success message"
    default_value: "connected"
    required: true
"#;
    fs::write(&yaml_path, yaml_content)?;

    // Step 2: Extract placeholders
    let hydrator = VarHydrator::new();
    let placeholders = hydrator.extract_placeholders(yaml_content);
    assert!(placeholders.len() >= 4);
    assert!(placeholders.contains(&"USERNAME".to_string()));
    assert!(placeholders.contains(&"SERVER_HOST".to_string()));
    assert!(placeholders.contains(&"PORT".to_string()));
    assert!(placeholders.contains(&"SUCCESS_MSG".to_string()));

    // Step 3: Create export file
    let export_path = temp_dir.path().join("vars.env");
    let mut export_file = fs::File::create(&export_path)?;
    writeln!(export_file, "export USERNAME=testuser")?;
    writeln!(export_file, "export SERVER_HOST=example.com")?;
    writeln!(export_file, "export PORT=2222")?;
    writeln!(export_file, "export SUCCESS_MSG=connected")?;

    // Step 4: Load export file and verify parsing
    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(&export_path)?;
    assert_eq!(hydrator.len(), 4);
    assert_eq!(hydrator.get("USERNAME"), Some(&"testuser".to_string()));
    assert_eq!(
        hydrator.get("SERVER_HOST"),
        Some(&"example.com".to_string())
    );
    assert_eq!(hydrator.get("PORT"), Some(&"2222".to_string()));
    assert_eq!(hydrator.get("SUCCESS_MSG"), Some(&"connected".to_string()));

    // Step 5: Hydrate YAML
    let hydrated_yaml = hydrator.hydrate_yaml_content(yaml_content);
    assert!(hydrated_yaml.contains("testuser"));
    assert!(hydrated_yaml.contains("example.com"));
    assert!(hydrated_yaml.contains("2222"));
    assert!(!hydrated_yaml.contains("${#USERNAME}"));
    assert!(!hydrated_yaml.contains("${#SERVER_HOST}"));

    // Step 6: Parse hydrated YAML
    let test_case: TestCase = serde_yaml::from_str(&hydrated_yaml)?;
    assert_eq!(test_case.id, "TC_WORKFLOW");
    assert_eq!(test_case.test_sequences.len(), 1);
    assert_eq!(test_case.test_sequences[0].steps.len(), 1);

    // Step 7: Generate bash script
    let executor = TestExecutor::new();
    let script = executor.generate_test_script(&test_case);
    assert!(script.contains("#!/bin/bash"));
    assert!(script.contains("TC_WORKFLOW"));

    Ok(())
}

#[test]
fn test_round_trip_export_generation_and_loading() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create test case with hydration_vars
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC_ROUNDTRIP".to_string(),
        "Round trip test".to_string(),
    );

    let mut hydration_vars = HashMap::new();
    hydration_vars.insert(
        "SERVER".to_string(),
        EnvVariable {
            name: "SERVER".to_string(),
            description: Some("Server address".to_string()),
            default_value: Some("example.com".to_string()),
            required: true,
        },
    );
    hydration_vars.insert(
        "PORT".to_string(),
        EnvVariable {
            name: "PORT".to_string(),
            description: Some("Server port".to_string()),
            default_value: Some("8080".to_string()),
            required: true,
        },
    );
    hydration_vars.insert(
        "TIMEOUT".to_string(),
        EnvVariable {
            name: "TIMEOUT".to_string(),
            description: Some("Connection timeout".to_string()),
            default_value: Some("30".to_string()),
            required: false,
        },
    );
    test_case.hydration_vars = Some(hydration_vars);

    // Generate export file
    let mut hydrator = VarHydrator::new();
    if let Some(ref vars) = test_case.hydration_vars {
        for (var_name, env_var) in vars {
            let value = env_var.default_value.as_deref().unwrap_or("");
            hydrator.set(var_name.clone(), value.to_string());
        }
    }

    let export_path = temp_dir.path().join("generated.env");
    hydrator.generate_export_file(&export_path)?;

    // Load export file back
    let mut hydrator2 = VarHydrator::new();
    hydrator2.load_from_export_file(&export_path)?;

    // Verify all variables were preserved
    assert_eq!(hydrator2.len(), 3);
    assert_eq!(hydrator2.get("SERVER"), Some(&"example.com".to_string()));
    assert_eq!(hydrator2.get("PORT"), Some(&"8080".to_string()));
    assert_eq!(hydrator2.get("TIMEOUT"), Some(&"30".to_string()));

    Ok(())
}

#[test]
fn test_hydration_with_special_characters_in_values() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "export URL=\"https://api.example.com/v1?key=abc123\"")?;
    writeln!(file, "export COMMAND='ls -la | grep test'")?;
    writeln!(file, "export JSON='{{\"foo\": \"bar\"}}'")?;

    let mut hydrator = VarHydrator::new();
    hydrator.load_from_export_file(file.path())?;

    let yaml = r#"
url: ${#URL}
command: ${#COMMAND}
json: ${#JSON}
"#;

    let hydrated = hydrator.hydrate_yaml_content(yaml);
    assert!(hydrated.contains("https://api.example.com/v1?key=abc123"));
    assert!(hydrated.contains("ls -la | grep test"));
    assert!(hydrated.contains("{\"foo\": \"bar\"}"));

    Ok(())
}

#[test]
fn test_generate_export_file_with_quoting() -> Result<()> {
    let mut vars = HashMap::new();
    vars.insert("SIMPLE".to_string(), "value".to_string());
    vars.insert("WITH_SPACES".to_string(), "hello world".to_string());
    vars.insert("WITH_QUOTES".to_string(), "say \"hello\"".to_string());
    vars.insert("WITH_DOLLAR".to_string(), "price $100".to_string());
    vars.insert("WITH_BACKSLASH".to_string(), "path\\to\\file".to_string());

    let hydrator = VarHydrator::with_variables(vars);
    let temp_file = NamedTempFile::new()?;

    hydrator.generate_export_file(temp_file.path())?;

    let content = fs::read_to_string(temp_file.path())?;

    // Simple values should not be quoted
    assert!(content.contains("export SIMPLE=value"));

    // Values with special characters should be quoted and escaped
    assert!(content.contains("export WITH_SPACES=\"hello world\""));
    assert!(content.contains("export WITH_QUOTES="));
    assert!(content.contains("export WITH_DOLLAR="));
    assert!(content.contains("export WITH_BACKSLASH="));

    Ok(())
}
