use chrono::Utc;
use testcase_manager::{TestRun, TestRunStatus};

fn main() {
    println!("=== JUnit XML Export Example ===\n");

    let timestamp = Utc::now();

    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.234,
            "".to_string(),
        ),
        TestRun::with_error(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.567,
            "Assertion failed: expected true, got false".to_string(),
        ),
        TestRun::with_error(
            "TC003".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.000,
            "Test skipped due to missing dependencies".to_string(),
        ),
    ];

    println!("Generated JUnit XML for {} test runs:\n", test_runs.len());

    for test_run in &test_runs {
        println!(
            "--- Test: {} (Status: {}) ---",
            test_run.test_case_id, test_run.status
        );
        println!("{}\n", test_run.to_junit_xml());
    }

    let json = serde_json::to_string_pretty(&test_runs).unwrap();
    println!("=== JSON Representation ===");
    println!("{}\n", json);

    println!("=== Usage ===");
    println!("To export test runs to JUnit XML:");
    println!("  1. Save test runs to a JSON file");
    println!("  2. Run: tcm export-junit-xml <input.json> -o results.xml");
    println!("  3. Or output to stdout: tcm export-junit-xml <input.json>");
}
