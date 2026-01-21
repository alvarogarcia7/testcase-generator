use testcase_manager::{
    BatchVerificationReport, JUnitTestSuite, Step, TestCase, TestCaseStorage, TestSequence,
    TestVerifier,
};

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("=== Test Verification Demo ===\n");

    // Create a temporary directory for test cases
    let temp_dir = tempfile::tempdir()?;
    let storage = TestCaseStorage::new(temp_dir.path())?;

    // Create a sample test case
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "DEMO_TC001".to_string(),
        "Demonstration Test Case".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Basic Sequence".to_string(),
        "Demonstrates test verification".to_string(),
    );

    // Add steps
    sequence.steps.push(Step::new(
        1,
        "Initialize system".to_string(),
        "init".to_string(),
        "OK".to_string(),
        "System initialized".to_string(),
    ));

    sequence.steps.push(Step::new(
        2,
        "Execute command".to_string(),
        "execute".to_string(),
        "SW=0x9000".to_string(),
        "Command executed successfully".to_string(),
    ));

    sequence.steps.push(Step::new(
        3,
        "Verify result".to_string(),
        "verify".to_string(),
        "PASS".to_string(),
        "Verification complete".to_string(),
    ));

    test_case.test_sequences.push(sequence);

    // Save test case
    storage.save_test_case(&test_case)?;
    println!("✓ Created test case: {}", test_case.id);
    println!();

    // Create test verifier
    let verifier = TestVerifier::from_storage(storage);

    // Demo 1: Successful verification
    println!("--- Demo 1: All Steps Pass ---");
    let log_content_pass = r#"
[2024-01-15T10:00:00Z] TestCase: DEMO_TC001, Sequence: 1, Step: 1, Success: true, Result: OK, Output: System initialized
[2024-01-15T10:00:01Z] TestCase: DEMO_TC001, Sequence: 1, Step: 2, Success: true, Result: SW=0x9000, Output: Command executed successfully
[2024-01-15T10:00:02Z] TestCase: DEMO_TC001, Sequence: 1, Step: 3, Success: true, Result: PASS, Output: Verification complete
"#;

    let logs = verifier.parse_log_content(log_content_pass, std::path::Path::new("demo.log"))?;
    let result = verifier.verify_test_case(&test_case, &logs);

    println!("Test Case: {}", result.test_case_id);
    println!(
        "Overall Status: {}",
        if result.overall_pass {
            "✓ PASS"
        } else {
            "✗ FAIL"
        }
    );
    println!(
        "Steps: {}/{} passed\n",
        result.passed_steps, result.total_steps
    );

    // Demo 2: Failed verification
    println!("--- Demo 2: Step Failure ---");
    let log_content_fail = r#"
[2024-01-15T10:00:00Z] TestCase: DEMO_TC001, Sequence: 1, Step: 1, Success: true, Result: OK, Output: System initialized
[2024-01-15T10:00:01Z] TestCase: DEMO_TC001, Sequence: 1, Step: 2, Success: false, Result: SW=0x6A82, Output: File not found
[2024-01-15T10:00:02Z] TestCase: DEMO_TC001, Sequence: 1, Step: 3, Success: true, Result: PASS, Output: Verification complete
"#;

    let logs = verifier.parse_log_content(log_content_fail, std::path::Path::new("demo.log"))?;
    let result = verifier.verify_test_case(&test_case, &logs);

    println!("Test Case: {}", result.test_case_id);
    println!(
        "Overall Status: {}",
        if result.overall_pass {
            "✓ PASS"
        } else {
            "✗ FAIL"
        }
    );
    println!(
        "Steps: {}/{} passed, {} failed\n",
        result.passed_steps, result.total_steps, result.failed_steps
    );

    for seq_result in &result.sequences {
        for step_result in &seq_result.step_results {
            if let testcase_manager::StepVerificationResultEnum::Fail {
                step,
                description,
                reason,
                ..
            } = step_result
            {
                println!("  ✗ Step {}: {} - {}", step, description, reason);
            }
        }
    }
    println!();

    // Demo 3: Batch verification with report
    println!("--- Demo 3: Batch Verification Report ---");
    let mut batch_report = BatchVerificationReport::new();
    batch_report.add_test_case_result(result);

    println!("{}", batch_report.summary());
    println!();

    // Demo 4: JUnit XML generation
    println!("--- Demo 4: JUnit XML Output ---");
    let junit = JUnitTestSuite::from_batch_report(&batch_report, "Demo Test Suite");
    let xml = junit.to_xml()?;
    println!("Generated JUnit XML ({} bytes)", xml.len());
    println!(
        "Tests: {}, Failures: {}, Skipped: {}",
        junit.tests, junit.failures, junit.skipped
    );
    println!();

    // Show sample of XML
    let lines: Vec<&str> = xml.lines().take(5).collect();
    println!("Sample XML output:");
    for line in lines {
        println!("  {}", line);
    }
    println!("  ...");
    println!();

    // Demo 5: Wildcard matching
    println!("--- Demo 5: Wildcard and Regex Matching ---");

    // Create test case with wildcards
    let mut test_case_wildcards = TestCase::new(
        "REQ002".to_string(),
        1,
        1,
        "DEMO_TC002".to_string(),
        "Wildcard matching demo".to_string(),
    );

    let mut sequence = TestSequence::new(
        1,
        "Wildcard Test".to_string(),
        "Tests pattern matching".to_string(),
    );

    sequence.steps.push(Step::new(
        1,
        "Wildcard match".to_string(),
        "test".to_string(),
        "SW=*".to_string(),
        "*success*".to_string(),
    ));

    sequence.steps.push(Step::new(
        2,
        "Regex match".to_string(),
        "test".to_string(),
        "/SW=0x[0-9A-F]{4}/".to_string(),
        "/.*completed.*/".to_string(),
    ));

    test_case_wildcards.test_sequences.push(sequence);

    let log_content_pattern = r#"
[2024-01-15T10:00:00Z] TestCase: DEMO_TC002, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Command executed successfully
[2024-01-15T10:00:01Z] TestCase: DEMO_TC002, Sequence: 1, Step: 2, Success: true, Result: SW=0xABCD, Output: Process completed successfully
"#;

    let logs = verifier.parse_log_content(log_content_pattern, std::path::Path::new("demo.log"))?;
    let result = verifier.verify_test_case(&test_case_wildcards, &logs);

    println!("Test Case: {}", result.test_case_id);
    println!(
        "Overall Status: {}",
        if result.overall_pass {
            "✓ PASS"
        } else {
            "✗ FAIL"
        }
    );
    println!(
        "Steps: {}/{} passed (with wildcard and regex matching)\n",
        result.passed_steps, result.total_steps
    );

    println!("=== Demo Complete ===");

    Ok(())
}
