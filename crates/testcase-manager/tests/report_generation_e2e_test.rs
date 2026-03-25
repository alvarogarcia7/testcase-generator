use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use testcase_manager::TestCaseStorage;
use testcase_manager::{
    BatchVerificationReport, ContainerReport, ContainerReportConfig, SequenceVerificationResult,
    StepVerificationResultEnum, StorageTestVerifier, TestCaseVerificationResult, TestVerifier,
};
use testcase_models::TestCase;

/// Helper function to get the workspace root path
fn get_workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut workspace_root = PathBuf::from(manifest_dir);
    workspace_root.pop(); // Go up from crate dir
    workspace_root.pop(); // Go up from crates dir
    workspace_root
}

fn load_test_case(filename: &str) -> TestCase {
    let workspace_root = get_workspace_root();
    let path = workspace_root.join("testcases").join(filename);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read test case: {}", path.display()));
    serde_yaml::from_str(&content)
        .unwrap_or_else(|_| panic!("Failed to parse test case: {}", filename))
}

// ============================================================================
// TC Type Tests (Test Cases)
// ============================================================================

#[test]
fn test_e2e_tc_gsma_4_4_2_2_yaml_output() {
    let test_case = load_test_case("gsma_4.4.2.2_TC.yml");

    // Create mock verification result matching the test case structure
    let result = TestCaseVerificationResult {
        test_case_id: test_case.id.clone(),
        description: test_case.description.clone(),
        requirement: Some(test_case.requirement.clone()),
        item: Some(test_case.item),
        tc: Some(test_case.tc),
        sequences: vec![
            SequenceVerificationResult {
                sequence_id: 1,
                name: "Test Sequence #01 Nominal: Unset PPR1".to_string(),
                step_results: vec![
                    StepVerificationResultEnum::Pass {
                        step: 1,
                        description: "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; MTD_STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE))".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                    StepVerificationResultEnum::Pass {
                        step: 2,
                        description: "Fetch 'XX'".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                ],
                all_steps_passed: true,
                requirement: None,
                item: None,
                tc: None,
            },
            SequenceVerificationResult {
                sequence_id: 2,
                name: "Test Sequence #02 Nominal: Unset PPPR2 and update icon".to_string(),
                step_results: vec![
                    StepVerificationResultEnum::Pass {
                        step: 1,
                        description: "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; MTD_STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE))".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                    StepVerificationResultEnum::Pass {
                        step: 2,
                        description: "Fetch 'XX'".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                ],
                all_steps_passed: true,
                requirement: None,
                item: None,
                tc: None,
            },
        ],
        total_steps: 4,
        passed_steps: 4,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    };

    let verifier = TestVerifier::with_exact_matching();
    let generated_yaml = verifier.generate_report_yaml(&result).unwrap();

    // Verify key fields are present
    assert!(generated_yaml.contains("requirement: XXX100"));
    assert!(generated_yaml.contains("item: 1"));
    assert!(generated_yaml.contains("tc: 4"));
    assert!(generated_yaml.contains(&test_case.id));
    assert!(generated_yaml.contains("total_steps: 4"));
    assert!(generated_yaml.contains("passed_steps: 4"));
    assert!(generated_yaml.contains("failed_steps: 0"));
    assert!(generated_yaml.contains("overall_pass: true"));
}

#[test]
fn test_e2e_tc_gsma_4_4_2_2_json_output() {
    let test_case = load_test_case("gsma_4.4.2.2_TC.yml");

    let result = TestCaseVerificationResult {
        test_case_id: test_case.id.clone(),
        description: test_case.description.clone(),
        requirement: Some(test_case.requirement.clone()),
        item: Some(test_case.item),
        tc: Some(test_case.tc),
        sequences: vec![],
        total_steps: 4,
        passed_steps: 4,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    };

    let verifier = TestVerifier::with_exact_matching();
    let generated_json = verifier.generate_report_json(&result).unwrap();

    // Verify JSON structure
    assert!(generated_json.contains("\"test_case_id\""));
    assert!(generated_json.contains("\"requirement\": \"XXX100\""));
    assert!(generated_json.contains("\"item\": 1"));
    assert!(generated_json.contains("\"tc\": 4"));
    assert!(generated_json.contains("\"total_steps\": 4"));
    assert!(generated_json.contains("\"passed_steps\": 4"));
    assert!(generated_json.contains("\"overall_pass\": true"));

    // Verify it's valid JSON
    let parsed: TestCaseVerificationResult = serde_json::from_str(&generated_json).unwrap();
    assert_eq!(parsed.test_case_id, test_case.id);
    assert_eq!(parsed.total_steps, 4);
}

// ============================================================================
// Format Comparison Tests
// ============================================================================

#[test]
fn test_yaml_json_equivalence() {
    let result = TestCaseVerificationResult {
        test_case_id: "TC_FORMAT_TEST".to_string(),
        description: "Test format equivalence".to_string(),
        requirement: Some("REQ_FMT".to_string()),
        item: Some(1),
        tc: Some(1),
        sequences: vec![],
        total_steps: 5,
        passed_steps: 3,
        failed_steps: 2,
        not_executed_steps: 0,
        overall_pass: false,
    };

    let verifier = TestVerifier::with_exact_matching();

    let yaml = verifier.generate_report_yaml(&result).unwrap();
    let json = verifier.generate_report_json(&result).unwrap();

    // Parse both formats and verify they contain the same data
    let yaml_parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();
    let json_parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(yaml_parsed.test_case_id, json_parsed.test_case_id);
    assert_eq!(yaml_parsed.description, json_parsed.description);
    assert_eq!(yaml_parsed.requirement, json_parsed.requirement);
    assert_eq!(yaml_parsed.item, json_parsed.item);
    assert_eq!(yaml_parsed.tc, json_parsed.tc);
    assert_eq!(yaml_parsed.total_steps, json_parsed.total_steps);
    assert_eq!(yaml_parsed.passed_steps, json_parsed.passed_steps);
    assert_eq!(yaml_parsed.failed_steps, json_parsed.failed_steps);
    assert_eq!(yaml_parsed.overall_pass, json_parsed.overall_pass);
}

#[test]
fn test_all_test_case_types_can_be_loaded() {
    // Verify TC test case file can be loaded
    let test_case = load_test_case("gsma_4.4.2.2_TC.yml");
    assert!(!test_case.id.is_empty(), "Test case ID should not be empty");
    assert!(
        !test_case.description.is_empty(),
        "Description should not be empty"
    );
}

// ============================================================================
// Container Format Tests - E2E with verifier binary
// ============================================================================

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

    // Run verifier binary (folder mode) - now always produces container format
    let output = Command::new("cargo")
        .args([
            "run",
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
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_yaml = String::from_utf8_lossy(&output.stdout);

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
    assert!(
        output_yaml.contains("execution_duration:"),
        "Missing execution_duration in metadata"
    );
    assert!(
        output_yaml.contains("total_test_cases:"),
        "Missing total_test_cases in metadata"
    );
    assert!(
        output_yaml.contains("passed_test_cases:"),
        "Missing passed_test_cases in metadata"
    );
    assert!(
        output_yaml.contains("failed_test_cases:"),
        "Missing failed_test_cases in metadata"
    );

    // Verify it can be deserialized to ContainerReport
    let parsed: ContainerReport = serde_yaml::from_str(&output_yaml).expect("Failed to parse YAML");
    assert_eq!(parsed.title, "E2E Test Report");
    assert_eq!(parsed.project, "E2E Test Project");
    assert!(parsed.metadata.environment.is_some());
    assert_eq!(parsed.metadata.environment.unwrap(), "Test Environment");
}

#[test]
fn test_e2e_container_format_test_results_structure() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier = StorageTestVerifier::from_storage(storage);

    // Create comprehensive test results with all step statuses
    let mut report = BatchVerificationReport::new();

    // Test case 1: All steps pass
    report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC001".to_string(),
        description: "Test with all passing steps".to_string(),
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
        sequences: vec![SequenceVerificationResult {
            sequence_id: 1,
            name: "Sequence 1".to_string(),
            step_results: vec![
                StepVerificationResultEnum::Pass {
                    step: 1,
                    description: "Step 1 description".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
                StepVerificationResultEnum::Pass {
                    step: 2,
                    description: "Step 2 description".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
            ],
            all_steps_passed: true,
            requirement: None,
            item: None,
            tc: None,
        }],
        total_steps: 2,
        passed_steps: 2,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    });

    // Test case 2: Mixed pass/fail/not_executed
    report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC002".to_string(),
        description: "Test with mixed results".to_string(),
        requirement: Some("REQ002".to_string()),
        item: Some(2),
        tc: Some(2),
        sequences: vec![SequenceVerificationResult {
            sequence_id: 1,
            name: "Mixed Sequence".to_string(),
            step_results: vec![
                StepVerificationResultEnum::Pass {
                    step: 1,
                    description: "Passing step".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
                StepVerificationResultEnum::Fail {
                    step: 2,
                    description: "Failing step".to_string(),
                    expected: testcase_models::Expected {
                        success: Some(true),
                        result: "0x9000".to_string(),
                        output: "Success".to_string(),
                    },
                    actual_result: "0x6985".to_string(),
                    actual_output: "Error".to_string(),
                    reason: "Status code mismatch".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
                StepVerificationResultEnum::NotExecuted {
                    step: 3,
                    description: "Not executed step".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
            ],
            all_steps_passed: false,
            requirement: None,
            item: None,
            tc: None,
        }],
        total_steps: 3,
        passed_steps: 1,
        failed_steps: 1,
        not_executed_steps: 1,
        overall_pass: false,
    });

    let config = ContainerReportConfig {
        title: "Test Container Report".to_string(),
        project: "Test Project".to_string(),
        environment: Some("Dev Environment".to_string()),
        platform: Some("Test Platform".to_string()),
        executor: Some("Test Executor".to_string()),
    };

    let yaml = verifier
        .generate_report(&[report.clone()], "yaml", config)
        .unwrap();

    // Verify test_results array structure
    assert!(yaml.contains("test_results:"), "Missing test_results array");

    // Verify test case entries contain proper fields
    assert!(
        yaml.contains("test_case_id:"),
        "Missing test_case_id in results"
    );
    assert!(yaml.contains("TC001"), "Missing TC001 in results");
    assert!(yaml.contains("TC002"), "Missing TC002 in results");
    assert!(
        yaml.contains("description:"),
        "Missing description in results"
    );
    assert!(yaml.contains("sequences:"), "Missing sequences in results");

    // Verify sequence structure
    assert!(
        yaml.contains("sequence_id:"),
        "Missing sequence_id in sequences"
    );
    assert!(yaml.contains("name:"), "Missing name in sequences");
    assert!(
        yaml.contains("step_results:"),
        "Missing step_results in sequences"
    );
    assert!(
        yaml.contains("all_steps_passed:"),
        "Missing all_steps_passed in sequences"
    );

    // Verify step result types (Pass, Fail, NotExecuted)
    // YAML uses tags for externally tagged enums: !Pass, !Fail, !NotExecuted
    assert!(
        yaml.contains("!Pass") || yaml.contains("Pass:"),
        "Missing Pass status in step results"
    );
    assert!(
        yaml.contains("!Fail") || yaml.contains("Fail:"),
        "Missing Fail status in step results"
    );
    assert!(
        yaml.contains("!NotExecuted") || yaml.contains("NotExecuted:"),
        "Missing NotExecuted status in step results"
    );

    // Verify Fail step contains expected/actual fields
    assert!(yaml.contains("expected:"), "Missing expected in Fail step");
    assert!(
        yaml.contains("actual_result:"),
        "Missing actual_result in Fail step"
    );
    assert!(
        yaml.contains("actual_output:"),
        "Missing actual_output in Fail step"
    );
    assert!(yaml.contains("reason:"), "Missing reason in Fail step");

    // Verify statistics in each test case result
    assert!(
        yaml.contains("total_steps:"),
        "Missing total_steps in test case"
    );
    assert!(
        yaml.contains("passed_steps:"),
        "Missing passed_steps in test case"
    );
    assert!(
        yaml.contains("failed_steps:"),
        "Missing failed_steps in test case"
    );
    assert!(
        yaml.contains("not_executed_steps:"),
        "Missing not_executed_steps in test case"
    );
    assert!(
        yaml.contains("overall_pass:"),
        "Missing overall_pass in test case"
    );

    // Deserialize and verify structure programmatically
    let parsed: ContainerReport =
        serde_yaml::from_str(&yaml).expect("Failed to parse container YAML");

    // Verify metadata statistics
    assert_eq!(
        parsed.metadata.total_test_cases, 2,
        "Metadata total_test_cases mismatch"
    );
    assert_eq!(
        parsed.metadata.passed_test_cases, 1,
        "Metadata passed_test_cases mismatch"
    );
    assert_eq!(
        parsed.metadata.failed_test_cases, 1,
        "Metadata failed_test_cases mismatch"
    );

    // Verify test_results array
    assert_eq!(parsed.test_results.len(), 2, "Should have 2 test results");

    // Verify first test case
    let tc1 = &parsed.test_results[0];
    assert_eq!(tc1.test_case_id, "TC001");
    assert_eq!(tc1.total_steps, 2);
    assert_eq!(tc1.passed_steps, 2);
    assert_eq!(tc1.failed_steps, 0);
    assert_eq!(tc1.not_executed_steps, 0);
    assert!(tc1.overall_pass);
    assert_eq!(tc1.sequences.len(), 1);
    assert_eq!(tc1.sequences[0].step_results.len(), 2);

    // Verify all steps in first test case are Pass
    for step in &tc1.sequences[0].step_results {
        assert!(
            matches!(step, StepVerificationResultEnum::Pass { .. }),
            "All steps should be Pass"
        );
    }

    // Verify second test case
    let tc2 = &parsed.test_results[1];
    assert_eq!(tc2.test_case_id, "TC002");
    assert_eq!(tc2.total_steps, 3);
    assert_eq!(tc2.passed_steps, 1);
    assert_eq!(tc2.failed_steps, 1);
    assert_eq!(tc2.not_executed_steps, 1);
    assert!(!tc2.overall_pass);
    assert_eq!(tc2.sequences.len(), 1);
    assert_eq!(tc2.sequences[0].step_results.len(), 3);

    // Verify mixed step results in second test case
    let steps = &tc2.sequences[0].step_results;
    assert!(
        matches!(steps[0], StepVerificationResultEnum::Pass { .. }),
        "First step should be Pass"
    );
    assert!(
        matches!(steps[1], StepVerificationResultEnum::Fail { .. }),
        "Second step should be Fail"
    );
    assert!(
        matches!(steps[2], StepVerificationResultEnum::NotExecuted { .. }),
        "Third step should be NotExecuted"
    );

    // Verify Fail step contains all required fields
    if let StepVerificationResultEnum::Fail {
        expected,
        actual_result,
        actual_output,
        reason,
        ..
    } = &steps[1]
    {
        assert_eq!(expected.result, "0x9000");
        assert_eq!(expected.output, "Success");
        assert_eq!(actual_result, "0x6985");
        assert_eq!(actual_output, "Error");
        assert_eq!(reason, "Status code mismatch");
    } else {
        panic!("Second step should be Fail variant");
    }
}

#[test]
fn test_e2e_container_format_metadata_accuracy() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier = StorageTestVerifier::from_storage(storage);

    let mut report = BatchVerificationReport::new();

    // Add 3 test cases: 2 passed, 1 failed
    for i in 1..=3 {
        let overall_pass = i != 2; // TC002 fails
        let failed_steps = if overall_pass { 0 } else { 1 };
        let passed_steps = if overall_pass { 2 } else { 1 };

        report.add_test_case_result(TestCaseVerificationResult {
            test_case_id: format!("TC{:03}", i),
            description: format!("Test case {}", i),
            requirement: Some(format!("REQ{:03}", i)),
            item: Some(i as i64),
            tc: Some(i as i64),
            sequences: vec![],
            total_steps: 2,
            passed_steps,
            failed_steps,
            not_executed_steps: 0,
            overall_pass,
        });
    }

    let config = ContainerReportConfig {
        title: "Metadata Test".to_string(),
        project: "Test Project".to_string(),
        environment: Some("Test Env".to_string()),
        platform: Some("Test Platform".to_string()),
        executor: Some("Test Executor".to_string()),
    };

    let yaml = verifier.generate_report(&[report], "yaml", config).unwrap();

    let parsed: ContainerReport = serde_yaml::from_str(&yaml).expect("Failed to parse YAML");

    // Verify metadata statistics are accurate
    assert_eq!(
        parsed.metadata.total_test_cases, 3,
        "Metadata should report 3 total test cases"
    );
    assert_eq!(
        parsed.metadata.passed_test_cases, 2,
        "Metadata should report 2 passed test cases"
    );
    assert_eq!(
        parsed.metadata.failed_test_cases, 1,
        "Metadata should report 1 failed test case"
    );

    // Verify metadata contains all optional fields
    assert_eq!(
        parsed.metadata.environment,
        Some("Test Env".to_string()),
        "Metadata should contain environment"
    );
    assert_eq!(
        parsed.metadata.platform,
        Some("Test Platform".to_string()),
        "Metadata should contain platform"
    );
    assert_eq!(
        parsed.metadata.executor,
        Some("Test Executor".to_string()),
        "Metadata should contain executor"
    );

    // execution_duration should be 0.0 for single report
    assert_eq!(
        parsed.metadata.execution_duration, 0.0,
        "Execution duration should be 0.0 for single report"
    );
}

#[test]
fn test_e2e_container_format_json_output() {
    let temp_dir = TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier = StorageTestVerifier::from_storage(storage);

    let mut report = BatchVerificationReport::new();
    report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC_JSON".to_string(),
        description: "JSON format test".to_string(),
        requirement: Some("REQ_JSON".to_string()),
        item: Some(1),
        tc: Some(1),
        sequences: vec![],
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    });

    let config = ContainerReportConfig {
        title: "JSON Test".to_string(),
        project: "JSON Project".to_string(),
        environment: None,
        platform: None,
        executor: None,
    };

    let json = verifier.generate_report(&[report], "json", config).unwrap();

    // Verify JSON structure
    assert!(json.contains("\"title\""), "Missing title in JSON");
    assert!(
        json.contains("\"JSON Test\""),
        "Missing title value in JSON"
    );
    assert!(json.contains("\"project\""), "Missing project in JSON");
    assert!(json.contains("\"test_date\""), "Missing test_date in JSON");
    assert!(
        json.contains("\"test_results\""),
        "Missing test_results in JSON"
    );
    assert!(json.contains("\"metadata\""), "Missing metadata in JSON");
    assert!(
        json.contains("\"execution_duration\""),
        "Missing execution_duration in JSON"
    );
    assert!(
        json.contains("\"total_test_cases\""),
        "Missing total_test_cases in JSON"
    );
    assert!(
        json.contains("\"passed_test_cases\""),
        "Missing passed_test_cases in JSON"
    );
    assert!(
        json.contains("\"failed_test_cases\""),
        "Missing failed_test_cases in JSON"
    );

    // Verify it can be deserialized
    let parsed: ContainerReport = serde_json::from_str(&json).expect("Failed to parse JSON");
    assert_eq!(parsed.title, "JSON Test");
    assert_eq!(parsed.project, "JSON Project");
    assert_eq!(parsed.metadata.total_test_cases, 1);
}

#[test]
fn test_e2e_container_format_matches_template_structure() {
    // Verify the template exists - we can't directly deserialize it since it uses
    // a different YAML format (tagged enums), but we can verify the structure
    let workspace_root = get_workspace_root();
    let template_path = workspace_root.join("testcases/expected_output_reports/container_data.yml");
    let template_content =
        fs::read_to_string(&template_path).expect("Failed to read container_data.yml template");

    // Verify template has expected top-level keys
    assert!(
        template_content.contains("title:"),
        "Template should have title"
    );
    assert!(
        template_content.contains("project:"),
        "Template should have project"
    );
    assert!(
        template_content.contains("test_date:"),
        "Template should have test_date"
    );
    assert!(
        template_content.contains("test_results:"),
        "Template should have test_results"
    );
    assert!(
        template_content.contains("metadata:"),
        "Template should have metadata"
    );

    // Create a report with similar structure
    let temp_dir = TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier = StorageTestVerifier::from_storage(storage);

    let mut report = BatchVerificationReport::new();
    report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC_TEST".to_string(),
        description: "Test description".to_string(),
        requirement: Some("REQ_TEST".to_string()),
        item: Some(1),
        tc: Some(1),
        sequences: vec![SequenceVerificationResult {
            sequence_id: 1,
            name: "Test Sequence".to_string(),
            step_results: vec![StepVerificationResultEnum::Pass {
                step: 1,
                description: "Test step".to_string(),
                requirement: None,
                item: None,
                tc: None,
            }],
            all_steps_passed: true,
            requirement: None,
            item: None,
            tc: None,
        }],
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    });

    let config = ContainerReportConfig {
        title: "Generated Report".to_string(),
        project: "Generated Project".to_string(),
        environment: Some("Test Env".to_string()),
        platform: Some("Test Platform".to_string()),
        executor: Some("Test Executor".to_string()),
    };

    let yaml = verifier.generate_report(&[report], "yaml", config).unwrap();

    let generated: ContainerReport =
        serde_yaml::from_str(&yaml).expect("Failed to parse generated YAML");

    // Verify generated report has same structure as template
    // Both should have title, project, test_date, test_results, metadata
    assert!(
        !generated.title.is_empty(),
        "Generated report should have title"
    );
    assert!(
        !generated.project.is_empty(),
        "Generated report should have project"
    );
    assert!(
        !generated.test_results.is_empty(),
        "Generated report should have test_results"
    );

    // Verify metadata structure matches template
    assert!(
        generated.metadata.environment.is_some(),
        "Generated metadata should have environment"
    );
    assert!(
        generated.metadata.platform.is_some(),
        "Generated metadata should have platform"
    );
    assert!(
        generated.metadata.executor.is_some(),
        "Generated metadata should have executor"
    );

    // Verify test_results have expected fields
    let generated_tc = &generated.test_results[0];

    // Verify all expected fields are present
    assert!(
        !generated_tc.test_case_id.is_empty(),
        "Test case should have id"
    );
    assert!(
        !generated_tc.description.is_empty(),
        "Test case should have description"
    );
    assert!(
        generated_tc.requirement.is_some(),
        "Test case should have requirement"
    );
    assert!(
        !generated_tc.sequences.is_empty(),
        "Test case should have sequences"
    );

    // Verify sequence structure
    let gen_seq = &generated_tc.sequences[0];
    assert!(gen_seq.sequence_id > 0, "Sequence should have id");
    assert!(!gen_seq.name.is_empty(), "Sequence should have name");
    assert!(
        !gen_seq.step_results.is_empty(),
        "Sequence should have step_results"
    );
}

// ============================================================================
// Container Config E2E Tests
// ============================================================================

#[test]
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
    let config_path =
        get_workspace_root().join("testcases/verifier_scenarios/full_container_config.yml");

    // Run verifier with config file, no CLI overrides
    let output = Command::new("cargo")
        .args([
            "run",
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
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_yaml = String::from_utf8_lossy(&output.stdout);

    // Verify values from config file are used
    assert!(output_yaml.contains("title: Full Featured Test Report"));
    assert!(output_yaml.contains("project: Full Featured Test Project"));
    assert!(output_yaml.contains("environment: Production"));
    assert!(output_yaml.contains("platform: macOS ARM64"));
    assert!(output_yaml.contains("executor: Jenkins v3.2.1"));

    let parsed: ContainerReport = serde_yaml::from_str(&output_yaml).expect("Failed to parse YAML");
    assert_eq!(parsed.title, "Full Featured Test Report");
    assert_eq!(parsed.project, "Full Featured Test Project");
    assert_eq!(parsed.metadata.environment, Some("Production".to_string()));
    assert_eq!(parsed.metadata.platform, Some("macOS ARM64".to_string()));
    assert_eq!(parsed.metadata.executor, Some("Jenkins v3.2.1".to_string()));
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
    let output = Command::new("cargo")
        .args([
            "run",
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
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_yaml = String::from_utf8_lossy(&output.stdout);

    // Verify CLI values are used
    assert!(output_yaml.contains("title: CLI Only Test Report"));
    assert!(output_yaml.contains("project: CLI Only Test Project"));
    assert!(output_yaml.contains("environment: CLI Environment"));
    assert!(output_yaml.contains("platform: CLI Platform"));
    assert!(output_yaml.contains("executor: CLI Executor"));

    let parsed: ContainerReport = serde_yaml::from_str(&output_yaml).expect("Failed to parse YAML");
    assert_eq!(parsed.title, "CLI Only Test Report");
    assert_eq!(parsed.project, "CLI Only Test Project");
    assert_eq!(
        parsed.metadata.environment,
        Some("CLI Environment".to_string())
    );
    assert_eq!(parsed.metadata.platform, Some("CLI Platform".to_string()));
    assert_eq!(parsed.metadata.executor, Some("CLI Executor".to_string()));
}

#[test]
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

    let config_path =
        get_workspace_root().join("testcases/verifier_scenarios/container_config.yml");

    // Run verifier with config file AND CLI overrides
    let output = Command::new("cargo")
        .args([
            "run",
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
            "--environment",
            "CLI Override Environment",
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_yaml = String::from_utf8_lossy(&output.stdout);

    // Verify CLI overrides take precedence
    assert!(output_yaml.contains("title: CLI Override Title"));
    assert!(output_yaml.contains("environment: CLI Override Environment"));

    // Verify non-overridden values come from config file
    assert!(output_yaml.contains("project: Container Config Test Project"));
    assert!(output_yaml.contains("platform: Linux x86_64"));
    assert!(output_yaml.contains("executor: CI Pipeline v1.0"));

    let parsed: ContainerReport = serde_yaml::from_str(&output_yaml).expect("Failed to parse YAML");
    assert_eq!(parsed.title, "CLI Override Title");
    assert_eq!(parsed.project, "Container Config Test Project");
    assert_eq!(
        parsed.metadata.environment,
        Some("CLI Override Environment".to_string())
    );
    assert_eq!(parsed.metadata.platform, Some("Linux x86_64".to_string()));
    assert_eq!(
        parsed.metadata.executor,
        Some("CI Pipeline v1.0".to_string())
    );
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

    // Run verifier with NO config file and NO CLI flags (use defaults)
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "verifier",
            "--",
            "--folder",
            logs_folder.to_str().unwrap(),
            "--test-case-dir",
            "testcases",
            "--format",
            "yaml",
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_yaml = String::from_utf8_lossy(&output.stdout);

    // Verify default values are used
    assert!(output_yaml.contains("title: Test Execution Results"));
    assert!(output_yaml.contains("project: Test Case Manager - Verification Results"));

    let parsed: ContainerReport = serde_yaml::from_str(&output_yaml).expect("Failed to parse YAML");
    assert_eq!(parsed.title, "Test Execution Results");
    assert_eq!(parsed.project, "Test Case Manager - Verification Results");

    // Optional fields should be None when using defaults
    assert!(parsed.metadata.environment.is_none());
    assert!(parsed.metadata.platform.is_none());
    assert!(parsed.metadata.executor.is_none());
}

#[test]
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

    let config_path =
        get_workspace_root().join("testcases/verifier_scenarios/minimal_container_config.yml");

    // Run verifier with minimal config file (only required fields)
    let output = Command::new("cargo")
        .args([
            "run",
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
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_yaml = String::from_utf8_lossy(&output.stdout);

    // Verify required fields from minimal config
    assert!(output_yaml.contains("title: Minimal Test Report"));
    assert!(output_yaml.contains("project: Minimal Test Project"));

    let parsed: ContainerReport = serde_yaml::from_str(&output_yaml).expect("Failed to parse YAML");
    assert_eq!(parsed.title, "Minimal Test Report");
    assert_eq!(parsed.project, "Minimal Test Project");

    // Optional fields should be None in minimal config
    assert!(parsed.metadata.environment.is_none());
    assert!(parsed.metadata.platform.is_none());
    assert!(parsed.metadata.executor.is_none());
}

#[test]
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

    let config_path =
        get_workspace_root().join("testcases/verifier_scenarios/container_config.yml");

    // Run verifier with JSON format and config file
    let output = Command::new("cargo")
        .args([
            "run",
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
        ])
        .output()
        .expect("Failed to execute verifier");

    assert!(
        output.status.success(),
        "Verifier command failed: {}\nStdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let output_json = String::from_utf8_lossy(&output.stdout);

    // Verify JSON structure
    assert!(output_json.contains("\"title\": \"Container Config Test Report\""));
    assert!(output_json.contains("\"project\": \"Container Config Test Project\""));
    assert!(output_json.contains("\"environment\": \"Development\""));
    assert!(output_json.contains("\"platform\": \"Linux x86_64\""));
    assert!(output_json.contains("\"executor\": \"CI Pipeline v1.0\""));

    let parsed: ContainerReport = serde_json::from_str(&output_json).expect("Failed to parse JSON");
    assert_eq!(parsed.title, "Container Config Test Report");
    assert_eq!(parsed.project, "Container Config Test Project");
    assert_eq!(parsed.metadata.environment, Some("Development".to_string()));
    assert_eq!(parsed.metadata.platform, Some("Linux x86_64".to_string()));
    assert_eq!(
        parsed.metadata.executor,
        Some("CI Pipeline v1.0".to_string())
    );
}
