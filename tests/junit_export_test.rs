use chrono::{DateTime, Utc};
use testcase_manager::{TestRun, TestRunStatus};

/// Validates JUnit XML output against the Maven Surefire XSD schema requirements.
///
/// This function validates the generated XML against the structure and constraints
/// defined in: https://maven.apache.org/surefire/maven-surefire-plugin/xsd/surefire-test-report.xsd
///
/// Key validations performed:
/// - Root element must be <testsuite>
/// - Required attributes: name, tests, failures, skipped, time
/// - All numeric attributes must be valid numbers
/// - Number of <testcase> elements must match the tests attribute
/// - Each <testcase> must have name and time attributes
/// - Count of <failure> elements must match failures attribute
/// - Count of <skipped> elements must match skipped attribute
/// - All <failure> and <error> elements must have message attribute or text content
fn validate_junit_xml_against_xsd(xml: &str) {
    let doc = roxmltree::Document::parse(xml).expect("Failed to parse XML");

    let root = doc.root_element();
    assert_eq!(
        root.tag_name().name(),
        "testsuite",
        "Root element must be <testsuite>"
    );

    assert!(
        root.has_attribute("name"),
        "testsuite must have 'name' attribute"
    );
    assert!(
        root.has_attribute("tests"),
        "testsuite must have 'tests' attribute"
    );
    assert!(
        root.has_attribute("failures"),
        "testsuite must have 'failures' attribute"
    );
    assert!(
        root.has_attribute("skipped"),
        "testsuite must have 'skipped' attribute"
    );
    assert!(
        root.has_attribute("time"),
        "testsuite must have 'time' attribute"
    );

    let tests_count = root
        .attribute("tests")
        .and_then(|v| v.parse::<usize>().ok())
        .expect("tests attribute must be a valid number");

    let failures_count = root
        .attribute("failures")
        .and_then(|v| v.parse::<usize>().ok())
        .expect("failures attribute must be a valid number");

    let skipped_count = root
        .attribute("skipped")
        .and_then(|v| v.parse::<usize>().ok())
        .expect("skipped attribute must be a valid number");

    let time_value = root
        .attribute("time")
        .and_then(|v| v.parse::<f64>().ok())
        .expect("time attribute must be a valid number");

    assert!(time_value >= 0.0, "time must be non-negative");

    let testcases: Vec<_> = root
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "testcase")
        .collect();

    assert_eq!(
        testcases.len(),
        tests_count,
        "Number of testcase elements must match tests attribute"
    );

    let mut actual_failures = 0;
    let mut actual_skipped = 0;

    for testcase in &testcases {
        assert!(
            testcase.has_attribute("name"),
            "testcase must have 'name' attribute"
        );
        assert!(
            testcase.has_attribute("time"),
            "testcase must have 'time' attribute"
        );

        let tc_time = testcase
            .attribute("time")
            .and_then(|v| v.parse::<f64>().ok())
            .expect("testcase time attribute must be a valid number");
        assert!(tc_time >= 0.0, "testcase time must be non-negative");

        let has_failure = testcase
            .children()
            .any(|n| n.is_element() && n.tag_name().name() == "failure");
        let has_skipped = testcase
            .children()
            .any(|n| n.is_element() && n.tag_name().name() == "skipped");

        if has_failure {
            actual_failures += 1;
        }
        if has_skipped {
            actual_skipped += 1;
        }

        for child in testcase.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "failure" => {
                    assert!(
                        child.has_attribute("message") || child.text().is_some(),
                        "failure element must have message attribute or text content"
                    );
                }
                "skipped" => {}
                "error" => {
                    assert!(
                        child.has_attribute("message") || child.text().is_some(),
                        "error element must have message attribute or text content"
                    );
                }
                _ => {}
            }
        }
    }

    assert_eq!(
        actual_failures, failures_count,
        "Number of failure elements must match failures attribute"
    );
    assert_eq!(
        actual_skipped, skipped_count,
        "Number of skipped elements must match skipped attribute"
    );
}

#[test]
fn test_single_test_run_pass() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.500,
        "".to_string(),
    );

    let xml = test_run.to_junit_xml();

    assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("<testsuite"));
    assert!(xml.contains("name=\"TestRun\""));
    assert!(xml.contains("tests=\"1\""));
    assert!(xml.contains("failures=\"0\""));
    assert!(xml.contains("skipped=\"0\""));
    assert!(xml.contains("id=\"TC001\""));

    validate_junit_xml_against_xsd(&xml);
}

#[test]
fn test_single_test_run_fail() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::with_error(
        "TC002".to_string(),
        TestRunStatus::Fail,
        timestamp,
        2.000,
        "Test failed".to_string(),
    );

    let xml = test_run.to_junit_xml();

    assert!(xml.contains("failures=\"1\""));
    assert!(xml.contains("<failure"));
    assert!(xml.contains("Test failed"));

    validate_junit_xml_against_xsd(&xml);
}

#[test]
fn test_single_test_run_skip() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::with_error(
        "TC003".to_string(),
        TestRunStatus::Skip,
        timestamp,
        0.0,
        "Dependencies missing".to_string(),
    );

    let xml = test_run.to_junit_xml();

    assert!(xml.contains("skipped=\"1\""));
    assert!(xml.contains("<skipped"));
    assert!(xml.contains("Dependencies missing"));

    validate_junit_xml_against_xsd(&xml);
}

#[test]
fn test_xml_escaping() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::with_error(
        "TC004".to_string(),
        TestRunStatus::Fail,
        timestamp,
        1.000,
        "Error: <unexpected & \"special\" characters>".to_string(),
    );

    let xml = test_run.to_junit_xml();
    assert!(xml.contains("Error:"));
}

#[test]
fn test_test_run_serialization() {
    let timestamp = Utc::now();
    let test_run = TestRun::new(
        "TC005".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.000,
        "".to_string(),
    );

    let json = serde_json::to_string(&test_run).unwrap();
    let deserialized: TestRun = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.test_case_id, "TC005");
    assert_eq!(deserialized.status, TestRunStatus::Pass);
    assert_eq!(deserialized.duration, 1.000);
}

#[test]
fn test_test_run_array_serialization() {
    let timestamp = Utc::now();
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
            TestRunStatus::Fail,
            timestamp,
            2.000,
            "".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "".to_string(),
        ),
    ];

    let json = serde_json::to_string(&test_runs).unwrap();
    let deserialized: Vec<TestRun> = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.len(), 3);
    assert_eq!(deserialized[0].status, TestRunStatus::Pass);
    assert_eq!(deserialized[1].status, TestRunStatus::Fail);
    assert_eq!(deserialized[2].status, TestRunStatus::Skip);
}

#[test]
fn test_junit_xml_structure_validation() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.234,
        "".to_string(),
    );

    let xml = test_run.to_junit_xml();

    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

    let testsuite_start = xml.find("<testsuite").unwrap();
    let testsuite_end = xml.find("</testsuite>").unwrap();
    assert!(testsuite_start < testsuite_end);

    let testcase_pos = xml.find("<testcase").unwrap();
    assert!(testsuite_start < testcase_pos);
    assert!(testcase_pos < testsuite_end);

    validate_junit_xml_against_xsd(&xml);
}

#[test]
fn test_multiple_test_runs_different_statuses() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();

    let pass_run = TestRun::new(
        "TC_PASS".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.000,
        "".to_string(),
    );
    let fail_run = TestRun::with_error(
        "TC_FAIL".to_string(),
        TestRunStatus::Fail,
        timestamp,
        2.000,
        "Failed".to_string(),
    );
    let skip_run = TestRun::with_error(
        "TC_SKIP".to_string(),
        TestRunStatus::Skip,
        timestamp,
        0.0,
        "Skipped".to_string(),
    );

    let pass_xml = pass_run.to_junit_xml();
    let fail_xml = fail_run.to_junit_xml();
    let skip_xml = skip_run.to_junit_xml();

    assert!(pass_xml.contains("failures=\"0\"") && pass_xml.contains("skipped=\"0\""));
    assert!(fail_xml.contains("failures=\"1\"") && fail_xml.contains("skipped=\"0\""));
    assert!(skip_xml.contains("failures=\"0\"") && skip_xml.contains("skipped=\"1\""));

    validate_junit_xml_against_xsd(&pass_xml);
    validate_junit_xml_against_xsd(&fail_xml);
    validate_junit_xml_against_xsd(&skip_xml);
}

#[test]
fn test_junit_xml_xsd_compliance() {
    // This test validates that our JUnit XML output complies with the Maven Surefire XSD schema
    // Schema URL: https://maven.apache.org/surefire/maven-surefire-plugin/xsd/surefire-test-report.xsd

    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();

    // Test all three status types for XSD compliance
    let pass_test = TestRun::new(
        "TC_XSD_PASS".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.500,
        "".to_string(),
    );
    let fail_test = TestRun::with_error(
        "TC_XSD_FAIL".to_string(),
        TestRunStatus::Fail,
        timestamp,
        2.000,
        "XSD validation test failure".to_string(),
    );
    let skip_test = TestRun::with_error(
        "TC_XSD_SKIP".to_string(),
        TestRunStatus::Skip,
        timestamp,
        0.0,
        "XSD validation test skip".to_string(),
    );

    // Validate each against the XSD schema requirements
    validate_junit_xml_against_xsd(&pass_test.to_junit_xml());
    validate_junit_xml_against_xsd(&fail_test.to_junit_xml());
    validate_junit_xml_against_xsd(&skip_test.to_junit_xml());

    // Test edge cases
    let zero_duration = TestRun::new(
        "TC_ZERO".to_string(),
        TestRunStatus::Pass,
        timestamp,
        0.0,
        "".to_string(),
    );
    validate_junit_xml_against_xsd(&zero_duration.to_junit_xml());

    let fail_no_message = TestRun::new(
        "TC_FAIL_NO_MSG".to_string(),
        TestRunStatus::Fail,
        timestamp,
        1.000,
        "".to_string(),
    );
    validate_junit_xml_against_xsd(&fail_no_message.to_junit_xml());
}
