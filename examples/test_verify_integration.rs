/// Integration example showing how to use test-verify with real test case files
/// This demonstrates the complete workflow from test case creation to verification
use std::fs;
use testcase_manager::{Step, TestCase, TestCaseStorage, TestSequence, TestVerifier};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Test Verification Integration Example ===\n");

    // Create a temporary directory structure
    let temp_dir = tempfile::tempdir()?;
    let testcases_dir = temp_dir.path().join("testcases");
    let logs_dir = temp_dir.path().join("logs");
    fs::create_dir_all(&testcases_dir)?;
    fs::create_dir_all(&logs_dir)?;

    println!("Created temporary directories:");
    println!("  Test cases: {}", testcases_dir.display());
    println!("  Logs: {}", logs_dir.display());
    println!();

    // Step 1: Create multiple test cases
    println!("Step 1: Creating test cases...");
    let storage = TestCaseStorage::new(&testcases_dir)?;

    // Test Case 1: Profile Download
    let mut tc1 = TestCase::new(
        "GSMA-4.4.2.2".to_string(),
        4,
        2,
        "ProfileDownload_TC".to_string(),
        "Profile Download and Activation Test".to_string(),
    );

    let mut seq1 = TestSequence::new(
        1,
        "Standard Download".to_string(),
        "Download and enable profile".to_string(),
    );

    seq1.steps.push(Step::new(
        1,
        "Download profile from SM-DP+".to_string(),
        "ssh esim download".to_string(),
        "SW=0x9000".to_string(),
        "Profile downloaded successfully".to_string(),
    ));

    seq1.steps.push(Step::new(
        2,
        "Enable downloaded profile".to_string(),
        "ssh esim enable".to_string(),
        "SW=0x9000".to_string(),
        "Profile enabled".to_string(),
    ));

    tc1.test_sequences.push(seq1);
    storage.save_test_case(&tc1)?;
    println!("  ✓ Created: {}", tc1.id);

    // Test Case 2: Profile Management
    let mut tc2 = TestCase::new(
        "GSMA-4.4.3.1".to_string(),
        4,
        3,
        "ProfileManagement_TC".to_string(),
        "Profile Management Operations Test".to_string(),
    );

    let mut seq2 = TestSequence::new(
        1,
        "List and Switch".to_string(),
        "List profiles and switch between them".to_string(),
    );

    seq2.steps.push(Step::new(
        1,
        "List all profiles".to_string(),
        "ssh esim list".to_string(),
        "*".to_string(),                // Wildcard match
        "/Profile.*found/".to_string(), // Regex match
    ));

    seq2.steps.push(Step::new(
        2,
        "Switch to profile 2".to_string(),
        "ssh esim switch 2".to_string(),
        "SW=0x9000".to_string(),
        "Switched to profile 2".to_string(),
    ));

    let mut seq2_2 = TestSequence::new(
        2,
        "Disable Profile".to_string(),
        "Disable the active profile".to_string(),
    );

    seq2_2.steps.push(Step::new(
        1,
        "Disable active profile".to_string(),
        "ssh esim disable".to_string(),
        "SW=0x9000".to_string(),
        "Profile disabled".to_string(),
    ));

    tc2.test_sequences.push(seq2);
    tc2.test_sequences.push(seq2_2);
    storage.save_test_case(&tc2)?;
    println!("  ✓ Created: {}", tc2.id);
    println!();

    // Step 2: Simulate test execution and create logs
    println!("Step 2: Generating test execution logs...");

    // Log 1: Successful execution of TC1
    let log1_path = logs_dir.join("test_run_1.log");
    let log1_content = r#"# Test Run 1: Profile Download Test
[2024-01-15T10:00:00Z] TestCase: ProfileDownload_TC, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Profile downloaded successfully
[2024-01-15T10:00:05Z] TestCase: ProfileDownload_TC, Sequence: 1, Step: 2, Success: true, Result: SW=0x9000, Output: Profile enabled
"#;
    fs::write(&log1_path, log1_content)?;
    println!("  ✓ Created: {}", log1_path.display());

    // Log 2: Partial failure in TC2
    let log2_path = logs_dir.join("test_run_2.log");
    let log2_content = r#"# Test Run 2: Profile Management Test
[2024-01-15T10:05:00Z] TestCase: ProfileManagement_TC, Sequence: 1, Step: 1, Success: true, Result: OK, Output: 3 Profiles found
[2024-01-15T10:05:02Z] TestCase: ProfileManagement_TC, Sequence: 1, Step: 2, Success: false, Result: SW=0x6A82, Output: Profile not found
[2024-01-15T10:05:05Z] TestCase: ProfileManagement_TC, Sequence: 2, Step: 1, Success: true, Result: SW=0x9000, Output: Profile disabled
"#;
    fs::write(&log2_path, log2_content)?;
    println!("  ✓ Created: {}", log2_path.display());

    // Log 3: Mixed results
    let log3_path = logs_dir.join("test_run_3.log");
    let log3_content = r#"# Test Run 3: Mixed results
[2024-01-15T10:10:00Z] TestCase: ProfileDownload_TC, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Profile downloaded successfully
[2024-01-15T10:10:05Z] TestCase: ProfileDownload_TC, Sequence: 1, Step: 2, Success: false, Result: SW=0x6A88, Output: Profile already enabled
"#;
    fs::write(&log3_path, log3_content)?;
    println!("  ✓ Created: {}", log3_path.display());
    println!();

    // Step 3: Single verification
    println!("Step 3: Single Test Verification...");
    let verifier = TestVerifier::from_storage(storage);

    let logs = verifier.parse_log_file(&log1_path)?;
    let result = verifier.verify_test_case(&tc1, &logs);

    println!("  Test Case: {}", result.test_case_id);
    println!(
        "  Status: {}",
        if result.overall_pass {
            "✓ PASS"
        } else {
            "✗ FAIL"
        }
    );
    println!(
        "  Steps: {}/{} passed",
        result.passed_steps, result.total_steps
    );
    println!();

    // Step 4: Batch verification
    println!("Step 4: Batch Verification...");
    let log_paths = vec![&log1_path, &log2_path, &log3_path];
    let report = verifier.batch_verify(&log_paths)?;

    println!("{}", report.summary());
    println!();

    // Step 5: Detailed results
    println!("Step 5: Detailed Results...");
    for tc_result in &report.test_cases {
        println!("─────────────────────────────────────────");
        let status_icon = if tc_result.overall_pass { "✓" } else { "✗" };
        println!("{} {}", status_icon, tc_result.test_case_id);
        println!("  Description: {}", tc_result.description);
        println!(
            "  Steps: {}/{} passed",
            tc_result.passed_steps, tc_result.total_steps
        );

        // Show failures
        for seq_result in &tc_result.sequences {
            for step_result in &seq_result.step_results {
                if let testcase_manager::StepVerificationResultEnum::Fail {
                    step,
                    description,
                    reason,
                    expected,
                    actual_result,
                    ..
                } = step_result
                {
                    println!(
                        "  ✗ Sequence {}, Step {}: {}",
                        seq_result.sequence_id, step, description
                    );
                    println!("    Reason: {}", reason);
                    println!("    Expected: {}", expected.result);
                    println!("    Got: {}", actual_result);
                }
            }
        }
    }
    println!("─────────────────────────────────────────");
    println!();

    // Step 6: Generate JUnit XML
    println!("Step 6: Generating JUnit XML...");
    let junit =
        testcase_manager::JUnitTestSuite::from_batch_report(&report, "Integration Test Suite");

    let xml_path = temp_dir.path().join("junit-report.xml");
    let xml_content = junit.to_xml()?;
    fs::write(&xml_path, &xml_content)?;

    println!("  ✓ JUnit XML generated: {}", xml_path.display());
    println!(
        "  Tests: {}, Failures: {}, Skipped: {}",
        junit.tests, junit.failures, junit.skipped
    );
    println!();

    // Step 7: Generate JSON report
    println!("Step 7: Generating JSON report...");
    let json_path = temp_dir.path().join("verification-report.json");
    let json_content = serde_json::to_string_pretty(&report)?;
    fs::write(&json_path, &json_content)?;

    println!("  ✓ JSON report generated: {}", json_path.display());
    println!();

    // Step 8: Summary
    println!("Step 8: Summary...");
    println!("─────────────────────────────────────────");
    println!("Generated files:");
    println!("  Test Cases:");
    println!("    - {}/ProfileDownload_TC.yaml", testcases_dir.display());
    println!(
        "    - {}/ProfileManagement_TC.yaml",
        testcases_dir.display()
    );
    println!("  Execution Logs:");
    println!("    - {}/test_run_1.log", logs_dir.display());
    println!("    - {}/test_run_2.log", logs_dir.display());
    println!("    - {}/test_run_3.log", logs_dir.display());
    println!("  Reports:");
    println!("    - {}", xml_path.display());
    println!("    - {}", json_path.display());
    println!("─────────────────────────────────────────");
    println!();

    println!("=== Integration Example Complete ===");
    println!();
    println!("To use test-verify with these files:");
    println!("  cargo run --bin test-verify batch \\");
    println!("    --logs {}/test_run_*.log \\", logs_dir.display());
    println!("    --test-case-dir {} \\", testcases_dir.display());
    println!("    --format junit \\");
    println!("    --output junit-report.xml");

    Ok(())
}
