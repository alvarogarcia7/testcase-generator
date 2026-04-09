use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn get_workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut workspace_root = PathBuf::from(manifest_dir);
    workspace_root.pop(); // Go up from verifier crate
    workspace_root.pop(); // Go up from crates dir
    workspace_root
}

fn get_schema_path() -> PathBuf {
    get_workspace_root().join("data/testcase_results_container/schema.json")
}

// ============================================================================
// Verifier Config File Tests
// ============================================================================

#[test]
#[ignore]
fn test_e2e_verifier_with_config_file_only() {
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log
    let log_file = logs_folder.join("test_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Use the full container config file
    let config_path = get_workspace_root().join("testcases/verifier_scenarios/full_container_config.yml");

    // Run verifier with config file, no CLI overrides
    let schema_path = get_schema_path();
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
            "--config",
            config_path.to_str().unwrap(),
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output (verifier outputs via log::info!)
    let output_text = if stderr.contains("title:") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // Verify values from config file are used
    assert!(
        output_text.contains("title: Full Featured Test Report"),
        "Expected 'title: Full Featured Test Report' in output"
    );
    assert!(
        output_text.contains("project: Full Featured Test Project"),
        "Expected 'project: Full Featured Test Project' in output"
    );
    assert!(
        output_text.contains("environment: Production"),
        "Expected 'environment: Production' in output"
    );
    assert!(
        output_text.contains("platform: macOS ARM64"),
        "Expected 'platform: macOS ARM64' in output"
    );
    assert!(
        output_text.contains("executor: Jenkins v3.2.1"),
        "Expected 'executor: Jenkins v3.2.1' in output"
    );
}

#[test]
fn test_e2e_verifier_with_cli_flags_only() {
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log
    let log_file = logs_folder.join("test_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Run verifier with CLI flags only, no config file
    let schema_path = get_schema_path();
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
            "--title",
            "CLI Only Test Report",
            "--project",
            "CLI Only Test Project",
            "--environment",
            "CLI Environment",
            "--platform",
            "CLI Platform",
            "--executor",
            "CLI Executor",
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output (verifier outputs via log::info!)
    let output_text = if stderr.contains("title:") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // Verify CLI values are used
    assert!(output_text.contains("title: CLI Only Test Report"));
    assert!(output_text.contains("project: CLI Only Test Project"));
    assert!(output_text.contains("environment: CLI Environment"));
    assert!(output_text.contains("platform: CLI Platform"));
    assert!(output_text.contains("executor: CLI Executor"));
}

#[test]
#[ignore]
fn test_e2e_verifier_with_config_and_cli_overrides() {
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log
    let log_file = logs_folder.join("test_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Use the full container config file
    let config_path = get_workspace_root().join("testcases/verifier_scenarios/full_container_config.yml");

    // Run verifier with config file AND CLI overrides
    let schema_path = get_schema_path();
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
            "--config",
            config_path.to_str().unwrap(),
            "--title",
            "CLI Override Title",
            "--project",
            "CLI Override Project",
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output
    let output_text = if stderr.contains("title:") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // CLI flags should override config file values
    assert!(output_text.contains("title: CLI Override Title"));
    assert!(output_text.contains("project: CLI Override Project"));
    // But environment from config file should still be there
    assert!(output_text.contains("environment: Production"));
}

#[test]
fn test_e2e_verifier_with_defaults_fallback() {
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log
    let log_file = logs_folder.join("test_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Run verifier with no config file and minimal CLI flags
    let schema_path = get_schema_path();
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output
    let output_text = if stderr.contains("title:") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // Verify default values are used
    assert!(output_text.contains("title: Test Execution Results"));
    assert!(output_text.contains("project: Test Case Manager - Verification Results"));
}

#[test]
#[ignore]
fn test_e2e_verifier_with_minimal_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log
    let log_file = logs_folder.join("test_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Use the minimal config file
    let config_path = get_workspace_root().join("testcases/verifier_scenarios/minimal_container_config.yml");

    // Run verifier with minimal config file
    let schema_path = get_schema_path();
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
            "--config",
            config_path.to_str().unwrap(),
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output
    let output_text = if stderr.contains("title:") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // Verify that config values are present
    assert!(output_text.contains("title:"));
    assert!(output_text.contains("project:"));
}

#[test]
#[ignore]
fn test_e2e_verifier_json_format_with_config() {
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log
    let log_file = logs_folder.join("test_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Use the full container config file
    let config_path = get_workspace_root().join("testcases/verifier_scenarios/full_container_config.yml");

    // Run verifier with JSON format
    let schema_path = get_schema_path();
    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "json",
            "--config",
            config_path.to_str().unwrap(),
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output
    let output_text = if stderr.contains("\"type\"") || stderr.contains("\"title\"") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // JSON format should contain these fields
    assert!(output_text.contains("\"type\""));
    assert!(output_text.contains("\"title\""));
}

#[test]
fn test_e2e_container_format_yaml_structure() {
    // Create temp directory for test execution logs
    let temp_dir = TempDir::new().unwrap();
    let logs_folder = temp_dir.path().join("logs");
    fs::create_dir(&logs_folder).unwrap();

    // Create a simple execution log with JSON format (naming convention for auto-discovery)
    let log_file = logs_folder.join("4.2.2.2.1 TC_eUICC_ES6.UpdateMetadata_execution_log.json");
    fs::write(
        &log_file,
        r#"[
  {
    "test_sequence": 1,
    "step": 1,
    "exit_code": 0,
    "output": "Success",
    "timestamp": "2024-01-15T10:30:00Z"
  }
]"#,
    )
    .unwrap();

    // Run verifier binary (folder mode) - always produces container format
    let schema_path = get_schema_path();

    let output = Command::new("cargo")
        .args([
            "run",
            "-p",
            "verifier",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
            "--title",
            "E2E Test Report",
            "--project",
            "E2E Test Project",
            "--environment",
            "Test Environment",
            "--platform",
            "Test Platform",
            "--executor",
            "Test Executor",
            "--schema",
            schema_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute verifier");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check stderr for output (verifier outputs via log::info!)
    let output_yaml = if stderr.contains("title:") {
        stderr.to_string()
    } else {
        stdout.to_string()
    };

    // Verify YAML structure matches container_data.yml template
    assert!(output_yaml.contains("title:"), "Missing title field");
    assert!(
        output_yaml.contains("E2E Test Report"),
        "Missing title value"
    );
    assert!(output_yaml.contains("project:"), "Missing project field");
    assert!(
        output_yaml.contains("E2E Test Project"),
        "Missing project value"
    );
    assert!(
        output_yaml.contains("test_date:"),
        "Missing test_date field"
    );
    assert!(
        output_yaml.contains("test_results:"),
        "Missing test_results field"
    );
    assert!(
        output_yaml.contains("metadata:"),
        "Missing metadata section"
    );

    // Verify metadata fields
    assert!(
        output_yaml.contains("environment:"),
        "Missing environment in metadata"
    );
    assert!(
        output_yaml.contains("Test Environment"),
        "Missing environment value"
    );
    assert!(
        output_yaml.contains("platform:"),
        "Missing platform in metadata"
    );
    assert!(
        output_yaml.contains("Test Platform"),
        "Missing platform value"
    );
    assert!(
        output_yaml.contains("executor:"),
        "Missing executor in metadata"
    );
    assert!(
        output_yaml.contains("Test Executor"),
        "Missing executor value"
    );
}
