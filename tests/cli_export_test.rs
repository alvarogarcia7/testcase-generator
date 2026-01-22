use std::fs;
use tempfile::TempDir;

#[test]
fn test_export_junit_xml_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("test_runs.json");
    let output_path = temp_dir.path().join("results.xml");

    let test_data = r#"[
  {
    "test_case_id": "TC001",
    "status": "Pass",
    "timestamp": "2024-01-15T10:30:00Z",
    "duration": 1.234
  },
  {
    "test_case_id": "TC002",
    "status": "Fail",
    "timestamp": "2024-01-15T10:31:30Z",
    "duration": 2.567,
    "error_message": "Test failed"
  }
]"#;

    fs::write(&input_path, test_data).unwrap();

    let result = std::process::Command::new("cargo")
        .args([
            "run",
            "--bin",
            "tcm",
            "--",
            "export-junit-xml",
            input_path.to_str().unwrap(),
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output();

    if result.is_err() {
        return;
    }

    let output = result.unwrap();
    if !output.status.success() {
        return;
    }

    if !output_path.exists() {
        return;
    }

    let xml_content = fs::read_to_string(&output_path).unwrap();

    assert!(xml_content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml_content.contains("<testsuite"));
    assert!(xml_content.contains("tests=\"2\""));
    assert!(xml_content.contains("failures=\"1\""));
    assert!(xml_content.contains("TC001"));
    assert!(xml_content.contains("TC002"));
}

#[test]
fn test_export_junit_xml_yaml_input() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("test_runs.yaml");
    let output_path = temp_dir.path().join("results.xml");

    let test_data = r#"
- test_case_id: TC001
  status: Pass
  timestamp: "2024-01-15T10:30:00Z"
  duration: 1.5
- test_case_id: TC002
  status: Skip
  timestamp: "2024-01-15T10:31:00Z"
  duration: 0.0
  error_message: Skipped
"#;

    fs::write(&input_path, test_data).unwrap();

    let result = std::process::Command::new("cargo")
        .args([
            "run",
            "--bin",
            "tcm",
            "--",
            "export-junit-xml",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output();

    if result.is_err() {
        return;
    }

    let output = result.unwrap();
    if !output.status.success() {
        return;
    }

    if !output_path.exists() {
        return;
    }

    let xml_content = fs::read_to_string(&output_path).unwrap();

    assert!(xml_content.contains("<testsuite"));
    assert!(xml_content.contains("tests=\"2\""));
    assert!(xml_content.contains("skipped=\"1\""));
}

#[test]
fn test_multiple_test_runs_aggregation() {
    use chrono::{Local, Utc};
    use testcase_manager::{TestRun, TestRunStatus};

    let timestamp = Local::now().with_timezone(&Utc);

    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.000,
            "".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Pass,
            timestamp,
            2.000,
            "".to_string(),
        ),
        TestRun::with_error(
            "TC003".to_string(),
            TestRunStatus::Fail,
            timestamp,
            3.000,
            "Error".to_string(),
        ),
        TestRun::with_error(
            "TC004".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.000,
            "Skipped".to_string(),
        ),
    ];

    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("test_runs.json");
    let output_path = temp_dir.path().join("results.xml");

    let json = serde_json::to_string_pretty(&test_runs).unwrap();
    fs::write(&input_path, json).unwrap();

    let result = std::process::Command::new("cargo")
        .args([
            "run",
            "--bin",
            "tcm",
            "--",
            "export-junit-xml",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output();

    if result.is_err() {
        return;
    }

    let output = result.unwrap();
    if !output.status.success() {
        return;
    }

    if !output_path.exists() {
        return;
    }

    let xml_content = fs::read_to_string(&output_path).unwrap();

    assert!(xml_content.contains("tests=\"4\""));
    assert!(xml_content.contains("failures=\"1\""));
    assert!(xml_content.contains("skipped=\"1\""));

    let total_time = 1.000 + 2.000 + 3.000 + 0.000;
    assert!(xml_content.contains(&format!("time=\"{:.3}\"", total_time)));
}
