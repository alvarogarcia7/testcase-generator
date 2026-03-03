use chrono::{DateTime, Utc};
use testcase_manager::models::{
    TestReportOutput, TestReportResults, TestReportSummary, TestRun, TestRunStatus,
};

#[test]
fn test_test_report_output_single_constructor() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "execution log".to_string(),
    );

    let report = TestReportOutput::single(test_run.clone());

    // Verify constructor behavior
    assert_eq!(report.test_case_count(), 1);
    assert!(report.title.is_none());
    assert!(report.project.is_none());
    assert!(report.test_date.is_none());
    assert!(report.environment.is_none());
    assert!(report.platform.is_none());
    assert!(report.executor.is_none());
    assert!(report.execution_duration.is_none());

    // Verify results variant
    match &report.results {
        TestReportResults::Single(run) => {
            assert_eq!(run.test_case_id, "TC001");
            assert_eq!(run.status, TestRunStatus::Pass);
            assert_eq!(run.duration, 1.5);
        }
        _ => panic!("Expected Single variant"),
    }
}

#[test]
fn test_test_report_output_multiple_constructor() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run1 = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log1".to_string(),
    );
    let test_run2 = TestRun::new(
        "TC002".to_string(),
        TestRunStatus::Fail,
        timestamp,
        2.5,
        "log2".to_string(),
    );
    let test_run3 = TestRun::new(
        "TC003".to_string(),
        TestRunStatus::Skip,
        timestamp,
        0.5,
        "log3".to_string(),
    );

    let report = TestReportOutput::multiple(vec![test_run1, test_run2, test_run3]);

    // Verify constructor behavior
    assert_eq!(report.test_case_count(), 3);
    assert!(report.title.is_none());
    assert!(report.project.is_none());
    assert!(report.test_date.is_none());
    assert!(report.environment.is_none());
    assert!(report.platform.is_none());
    assert!(report.executor.is_none());
    assert!(report.execution_duration.is_none());

    // Verify results variant
    match &report.results {
        TestReportResults::Multiple(runs) => {
            assert_eq!(runs.len(), 3);
            assert_eq!(runs[0].test_case_id, "TC001");
            assert_eq!(runs[1].test_case_id, "TC002");
            assert_eq!(runs[2].test_case_id, "TC003");
        }
        _ => panic!("Expected Multiple variant"),
    }
}

#[test]
fn test_test_report_output_builder_pattern() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run)
        .with_title("Test Execution Report".to_string())
        .with_project("MyProject".to_string())
        .with_test_date("2024-01-15".to_string())
        .with_environment("production".to_string())
        .with_platform("linux-x86_64".to_string())
        .with_executor("CI/CD Pipeline".to_string())
        .with_execution_duration(3.5);

    assert_eq!(report.title, Some("Test Execution Report".to_string()));
    assert_eq!(report.project, Some("MyProject".to_string()));
    assert_eq!(report.test_date, Some("2024-01-15".to_string()));
    assert_eq!(report.environment, Some("production".to_string()));
    assert_eq!(report.platform, Some("linux-x86_64".to_string()));
    assert_eq!(report.executor, Some("CI/CD Pipeline".to_string()));
    assert_eq!(report.execution_duration, Some(3.5));
}

#[test]
fn test_test_report_output_builder_pattern_chaining() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Pass,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
    ];

    // Test that builder methods can be chained in any order
    let report = TestReportOutput::multiple(test_runs)
        .with_executor("Jenkins".to_string())
        .with_platform("darwin-aarch64".to_string())
        .with_title("Regression Tests".to_string())
        .with_environment("staging".to_string())
        .with_test_date("2024-02-20".to_string())
        .with_execution_duration(5.0)
        .with_project("Backend Services".to_string());

    assert_eq!(report.executor, Some("Jenkins".to_string()));
    assert_eq!(report.platform, Some("darwin-aarch64".to_string()));
    assert_eq!(report.title, Some("Regression Tests".to_string()));
    assert_eq!(report.environment, Some("staging".to_string()));
    assert_eq!(report.test_date, Some("2024-02-20".to_string()));
    assert_eq!(report.execution_duration, Some(5.0));
    assert_eq!(report.project, Some("Backend Services".to_string()));
}

#[test]
fn test_test_report_output_test_case_count_single() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run);
    assert_eq!(report.test_case_count(), 1);
}

#[test]
fn test_test_report_output_test_case_count_multiple() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.5,
            "log3".to_string(),
        ),
        TestRun::new(
            "TC004".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.5,
            "log4".to_string(),
        ),
        TestRun::new(
            "TC005".to_string(),
            TestRunStatus::Pass,
            timestamp,
            2.5,
            "log5".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    assert_eq!(report.test_case_count(), 5);
}

#[test]
fn test_test_report_output_test_case_count_empty() {
    let report = TestReportOutput::multiple(vec![]);
    assert_eq!(report.test_case_count(), 0);
}

#[test]
fn test_test_report_output_test_runs_single() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run);
    let runs = report.test_runs();

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].test_case_id, "TC001");
}

#[test]
fn test_test_report_output_test_runs_multiple() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.5,
            "log3".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    let runs = report.test_runs();

    assert_eq!(runs.len(), 3);
    assert_eq!(runs[0].test_case_id, "TC001");
    assert_eq!(runs[1].test_case_id, "TC002");
    assert_eq!(runs[2].test_case_id, "TC003");
}

#[test]
fn test_test_report_output_summary_stats_all_passed() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Pass,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.5,
            "log3".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    let summary = report.summary_stats();

    assert_eq!(summary.total, 3);
    assert_eq!(summary.passed, 3);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.total_duration, 4.5);
}

#[test]
fn test_test_report_output_summary_stats_all_failed() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Fail,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    let summary = report.summary_stats();

    assert_eq!(summary.total, 2);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 2);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.total_duration, 3.0);
}

#[test]
fn test_test_report_output_summary_stats_all_skipped() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "log3".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    let summary = report.summary_stats();

    assert_eq!(summary.total, 3);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 3);
    assert_eq!(summary.total_duration, 0.0);
}

#[test]
fn test_test_report_output_summary_stats_mixed() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Pass,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Fail,
            timestamp,
            1.5,
            "log3".to_string(),
        ),
        TestRun::new(
            "TC004".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.5,
            "log4".to_string(),
        ),
        TestRun::new(
            "TC005".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "log5".to_string(),
        ),
        TestRun::new(
            "TC006".to_string(),
            TestRunStatus::Pass,
            timestamp,
            3.0,
            "log6".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    let summary = report.summary_stats();

    assert_eq!(summary.total, 6);
    assert_eq!(summary.passed, 3);
    assert_eq!(summary.failed, 2);
    assert_eq!(summary.skipped, 1);
    assert_eq!(summary.total_duration, 10.0);
}

#[test]
fn test_test_report_output_summary_stats_single_test() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        2.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run);
    let summary = report.summary_stats();

    assert_eq!(summary.total, 1);
    assert_eq!(summary.passed, 1);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.total_duration, 2.5);
}

#[test]
fn test_test_report_output_summary_stats_empty() {
    let report = TestReportOutput::multiple(vec![]);
    let summary = report.summary_stats();

    assert_eq!(summary.total, 0);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.total_duration, 0.0);
}

#[test]
fn test_test_report_output_json_serialization_single() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "execution log".to_string(),
    );

    let report = TestReportOutput::single(test_run)
        .with_title("Test Report".to_string())
        .with_project("MyProject".to_string());

    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("\"title\":\"Test Report\""));
    assert!(json.contains("\"project\":\"MyProject\""));
    assert!(json.contains("\"test_case_id\":\"TC001\""));
    assert!(json.contains("\"status\":\"Pass\""));

    // Test deserialization
    let deserialized: TestReportOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_output_json_serialization_multiple() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs)
        .with_title("Multi-Test Report".to_string())
        .with_environment("staging".to_string());

    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("\"title\":\"Multi-Test Report\""));
    assert!(json.contains("\"environment\":\"staging\""));
    assert!(json.contains("\"test_case_id\":\"TC001\""));
    assert!(json.contains("\"test_case_id\":\"TC002\""));

    // Test deserialization
    let deserialized: TestReportOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_output_yaml_serialization_single() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "execution log".to_string(),
    );

    let report = TestReportOutput::single(test_run)
        .with_title("YAML Test Report".to_string())
        .with_platform("linux".to_string());

    let yaml = serde_yaml::to_string(&report).unwrap();
    assert!(yaml.contains("title: YAML Test Report"));
    assert!(yaml.contains("platform: linux"));
    assert!(yaml.contains("test_case_id: TC001"));
    assert!(yaml.contains("status: Pass"));

    // Test deserialization
    let deserialized: TestReportOutput = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_output_yaml_serialization_multiple() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.5,
            "log3".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs)
        .with_title("Multi-Test YAML Report".to_string())
        .with_executor("CI".to_string());

    let yaml = serde_yaml::to_string(&report).unwrap();
    assert!(yaml.contains("title: Multi-Test YAML Report"));
    assert!(yaml.contains("executor: CI"));
    assert!(yaml.contains("test_case_id: TC001"));
    assert!(yaml.contains("test_case_id: TC002"));
    assert!(yaml.contains("test_case_id: TC003"));

    // Test deserialization
    let deserialized: TestReportOutput = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_output_json_skip_none_fields() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run);

    let json = serde_json::to_string(&report).unwrap();
    assert!(!json.contains("\"title\""));
    assert!(!json.contains("\"project\""));
    assert!(!json.contains("\"test_date\""));
    assert!(!json.contains("\"environment\""));
    assert!(!json.contains("\"platform\""));
    assert!(!json.contains("\"executor\""));
    assert!(!json.contains("\"execution_duration\""));
}

#[test]
fn test_test_report_output_yaml_skip_none_fields() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run);

    let yaml = serde_yaml::to_string(&report).unwrap();
    assert!(!yaml.contains("title:"));
    assert!(!yaml.contains("project:"));
    assert!(!yaml.contains("test_date:"));
    assert!(!yaml.contains("environment:"));
    assert!(!yaml.contains("platform:"));
    assert!(!yaml.contains("executor:"));
    assert!(!yaml.contains("execution_duration:"));
}

#[test]
fn test_test_report_results_single_variant() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let results = TestReportResults::Single(test_run.clone());

    match results {
        TestReportResults::Single(run) => {
            assert_eq!(run.test_case_id, "TC001");
            assert_eq!(run.status, TestRunStatus::Pass);
            assert_eq!(run.duration, 1.5);
        }
        _ => panic!("Expected Single variant"),
    }
}

#[test]
fn test_test_report_results_multiple_variant() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
    ];

    let results = TestReportResults::Multiple(test_runs.clone());

    match results {
        TestReportResults::Multiple(runs) => {
            assert_eq!(runs.len(), 2);
            assert_eq!(runs[0].test_case_id, "TC001");
            assert_eq!(runs[1].test_case_id, "TC002");
        }
        _ => panic!("Expected Multiple variant"),
    }
}

#[test]
fn test_test_report_results_json_untagged_single() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let results = TestReportResults::Single(test_run);
    let json = serde_json::to_string(&results).unwrap();

    // Verify no variant tag in JSON (untagged enum)
    assert!(!json.contains("\"Single\""));
    assert!(!json.contains("\"type\""));
    assert!(json.contains("\"test_case_id\":\"TC001\""));

    let deserialized: TestReportResults = serde_json::from_str(&json).unwrap();
    assert!(matches!(deserialized, TestReportResults::Single(_)));
}

#[test]
fn test_test_report_results_json_untagged_multiple() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
    ];

    let results = TestReportResults::Multiple(test_runs);
    let json = serde_json::to_string(&results).unwrap();

    // Verify no variant tag in JSON (untagged enum)
    assert!(!json.contains("\"Multiple\""));
    assert!(!json.contains("\"type\""));
    assert!(json.starts_with('['));
    assert!(json.contains("\"test_case_id\":\"TC001\""));
    assert!(json.contains("\"test_case_id\":\"TC002\""));

    let deserialized: TestReportResults = serde_json::from_str(&json).unwrap();
    assert!(matches!(deserialized, TestReportResults::Multiple(_)));
}

#[test]
fn test_test_report_results_yaml_untagged_single() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let results = TestReportResults::Single(test_run);
    let yaml = serde_yaml::to_string(&results).unwrap();

    // Verify no variant tag in YAML (untagged enum)
    assert!(!yaml.contains("Single:"));
    assert!(yaml.contains("test_case_id: TC001"));

    let deserialized: TestReportResults = serde_yaml::from_str(&yaml).unwrap();
    assert!(matches!(deserialized, TestReportResults::Single(_)));
}

#[test]
fn test_test_report_results_yaml_untagged_multiple() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.0,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.0,
            "log2".to_string(),
        ),
    ];

    let results = TestReportResults::Multiple(test_runs);
    let yaml = serde_yaml::to_string(&results).unwrap();

    // Verify no variant tag in YAML (untagged enum)
    assert!(!yaml.contains("Multiple:"));
    assert!(yaml.contains("test_case_id: TC001"));
    assert!(yaml.contains("test_case_id: TC002"));

    let deserialized: TestReportResults = serde_yaml::from_str(&yaml).unwrap();
    assert!(matches!(deserialized, TestReportResults::Multiple(_)));
}

#[test]
fn test_test_report_summary_creation() {
    let summary = TestReportSummary {
        total: 10,
        passed: 7,
        failed: 2,
        skipped: 1,
        total_duration: 45.5,
    };

    assert_eq!(summary.total, 10);
    assert_eq!(summary.passed, 7);
    assert_eq!(summary.failed, 2);
    assert_eq!(summary.skipped, 1);
    assert_eq!(summary.total_duration, 45.5);
}

#[test]
fn test_test_report_summary_json_serialization() {
    let summary = TestReportSummary {
        total: 5,
        passed: 3,
        failed: 1,
        skipped: 1,
        total_duration: 12.5,
    };

    let json = serde_json::to_string(&summary).unwrap();
    assert!(json.contains("\"total\":5"));
    assert!(json.contains("\"passed\":3"));
    assert!(json.contains("\"failed\":1"));
    assert!(json.contains("\"skipped\":1"));
    assert!(json.contains("\"total_duration\":12.5"));

    let deserialized: TestReportSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(summary, deserialized);
}

#[test]
fn test_test_report_summary_yaml_serialization() {
    let summary = TestReportSummary {
        total: 8,
        passed: 6,
        failed: 1,
        skipped: 1,
        total_duration: 25.75,
    };

    let yaml = serde_yaml::to_string(&summary).unwrap();
    assert!(yaml.contains("total: 8"));
    assert!(yaml.contains("passed: 6"));
    assert!(yaml.contains("failed: 1"));
    assert!(yaml.contains("skipped: 1"));
    assert!(yaml.contains("total_duration: 25.75"));

    let deserialized: TestReportSummary = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(summary, deserialized);
}

#[test]
fn test_test_report_summary_zero_values() {
    let summary = TestReportSummary {
        total: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        total_duration: 0.0,
    };

    assert_eq!(summary.total, 0);
    assert_eq!(summary.passed, 0);
    assert_eq!(summary.failed, 0);
    assert_eq!(summary.skipped, 0);
    assert_eq!(summary.total_duration, 0.0);

    // Test serialization roundtrip
    let json = serde_json::to_string(&summary).unwrap();
    let deserialized: TestReportSummary = serde_json::from_str(&json).unwrap();
    assert_eq!(summary, deserialized);
}

#[test]
fn test_test_report_output_with_all_metadata_fields() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run)
        .with_title("Complete Test Report".to_string())
        .with_project("Full Project Name".to_string())
        .with_test_date("2024-01-15T10:30:00Z".to_string())
        .with_environment("production-eu-west-1".to_string())
        .with_platform("linux-x86_64-gnu".to_string())
        .with_executor("GitHub Actions Runner v2.314.0".to_string())
        .with_execution_duration(123.456);

    // Verify all fields are set
    assert_eq!(report.title, Some("Complete Test Report".to_string()));
    assert_eq!(report.project, Some("Full Project Name".to_string()));
    assert_eq!(report.test_date, Some("2024-01-15T10:30:00Z".to_string()));
    assert_eq!(report.environment, Some("production-eu-west-1".to_string()));
    assert_eq!(report.platform, Some("linux-x86_64-gnu".to_string()));
    assert_eq!(
        report.executor,
        Some("GitHub Actions Runner v2.314.0".to_string())
    );
    assert_eq!(report.execution_duration, Some(123.456));

    // Test JSON roundtrip
    let json = serde_json::to_string(&report).unwrap();
    let deserialized: TestReportOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(report, deserialized);

    // Test YAML roundtrip
    let yaml = serde_yaml::to_string(&report).unwrap();
    let deserialized: TestReportOutput = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_output_partial_metadata() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        1.5,
        "log".to_string(),
    );

    // Only set some metadata fields
    let report = TestReportOutput::single(test_run)
        .with_title("Partial Report".to_string())
        .with_environment("dev".to_string());

    assert_eq!(report.title, Some("Partial Report".to_string()));
    assert!(report.project.is_none());
    assert!(report.test_date.is_none());
    assert_eq!(report.environment, Some("dev".to_string()));
    assert!(report.platform.is_none());
    assert!(report.executor.is_none());
    assert!(report.execution_duration.is_none());
}

#[test]
fn test_test_report_output_complex_integration() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();

    // Create diverse test runs
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            0.5,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.2,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.3,
            "log3".to_string(),
        ),
        TestRun::new(
            "TC004".to_string(),
            TestRunStatus::Pass,
            timestamp,
            0.8,
            "log4".to_string(),
        ),
        TestRun::new(
            "TC005".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "log5".to_string(),
        ),
        TestRun::new(
            "TC006".to_string(),
            TestRunStatus::Fail,
            timestamp,
            1.7,
            "log6".to_string(),
        ),
        TestRun::new(
            "TC007".to_string(),
            TestRunStatus::Pass,
            timestamp,
            3.1,
            "log7".to_string(),
        ),
    ];

    // Build complete report
    let report = TestReportOutput::multiple(test_runs)
        .with_title("Integration Test Suite".to_string())
        .with_project("E-Commerce Platform".to_string())
        .with_test_date("2024-01-15".to_string())
        .with_environment("staging".to_string())
        .with_platform("darwin-aarch64".to_string())
        .with_executor("CircleCI".to_string())
        .with_execution_duration(15.5);

    // Verify test case count
    assert_eq!(report.test_case_count(), 7);

    // Verify summary statistics
    let summary = report.summary_stats();
    assert_eq!(summary.total, 7);
    assert_eq!(summary.passed, 4);
    assert_eq!(summary.failed, 2);
    assert_eq!(summary.skipped, 1);
    assert_eq!(summary.total_duration, 9.6);

    // Verify test runs
    let runs = report.test_runs();
    assert_eq!(runs.len(), 7);

    // Test JSON serialization and deserialization
    let json = serde_json::to_string_pretty(&report).unwrap();
    let deserialized_json: TestReportOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(report, deserialized_json);

    // Test YAML serialization and deserialization
    let yaml = serde_yaml::to_string(&report).unwrap();
    let deserialized_yaml: TestReportOutput = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(report, deserialized_yaml);
}

#[test]
fn test_test_report_output_edge_case_large_duration() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "TC001".to_string(),
        TestRunStatus::Pass,
        timestamp,
        999999.999999,
        "log".to_string(),
    );

    let report = TestReportOutput::single(test_run).with_execution_duration(1234567.89);

    let summary = report.summary_stats();
    assert_eq!(summary.total_duration, 999999.999999);
    assert_eq!(report.execution_duration, Some(1234567.89));

    // Test serialization handles large numbers
    let json = serde_json::to_string(&report).unwrap();
    let deserialized: TestReportOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_output_edge_case_empty_strings() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_run = TestRun::new(
        "".to_string(), // Empty test case ID
        TestRunStatus::Pass,
        timestamp,
        1.0,
        "".to_string(), // Empty execution log
    );

    let report = TestReportOutput::single(test_run)
        .with_title("".to_string())
        .with_project("".to_string())
        .with_environment("".to_string());

    assert_eq!(report.title, Some("".to_string()));
    assert_eq!(report.project, Some("".to_string()));

    // Verify serialization/deserialization works with empty strings
    let json = serde_json::to_string(&report).unwrap();
    let deserialized: TestReportOutput = serde_json::from_str(&json).unwrap();
    assert_eq!(report, deserialized);
}

#[test]
fn test_test_report_summary_calculation_precision() {
    let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
    let test_runs = vec![
        TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            0.1,
            "log1".to_string(),
        ),
        TestRun::new(
            "TC002".to_string(),
            TestRunStatus::Pass,
            timestamp,
            0.2,
            "log2".to_string(),
        ),
        TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Pass,
            timestamp,
            0.3,
            "log3".to_string(),
        ),
    ];

    let report = TestReportOutput::multiple(test_runs);
    let summary = report.summary_stats();

    // Test floating point precision
    assert!((summary.total_duration - 0.6).abs() < 0.0001);
}
