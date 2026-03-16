use crate::models::{ActualResult, Expected, Step, TestCase, TestSequence};
use crate::storage::TestCaseStorage;
use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStrategy {
    Exact,
    Regex,
    Contains,
    Precomputed,
}

/// Result of verifying a single step (struct-based, for backward compatibility with tests)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepVerificationResult {
    pub step_number: i64,
    pub passed: bool,
    pub result_match: bool,
    pub output_match: bool,
    pub success_match: bool,
    pub diff: VerificationDiff,
}

/// Result of verifying a single step (enum-based, for batch verification)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum StepVerificationResultEnum {
    /// Step passed verification
    Pass {
        step: i64,
        description: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        requirement: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        item: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tc: Option<i64>,
    },
    /// Step failed verification
    Fail {
        step: i64,
        description: String,
        expected: Expected,
        actual_result: String,
        actual_output: String,
        reason: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        requirement: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        item: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tc: Option<i64>,
    },
    /// Step was not found in execution log
    NotExecuted {
        step: i64,
        description: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        requirement: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        item: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tc: Option<i64>,
    },
}

impl StepVerificationResultEnum {
    pub fn is_pass(&self) -> bool {
        matches!(self, StepVerificationResultEnum::Pass { .. })
    }

    pub fn step_number(&self) -> i64 {
        match self {
            StepVerificationResultEnum::Pass { step, .. } => *step,
            StepVerificationResultEnum::Fail { step, .. } => *step,
            StepVerificationResultEnum::NotExecuted { step, .. } => *step,
        }
    }

    pub fn requirement(&self) -> Option<&String> {
        match self {
            StepVerificationResultEnum::Pass { requirement, .. } => requirement.as_ref(),
            StepVerificationResultEnum::Fail { requirement, .. } => requirement.as_ref(),
            StepVerificationResultEnum::NotExecuted { requirement, .. } => requirement.as_ref(),
        }
    }

    pub fn item(&self) -> Option<i64> {
        match self {
            StepVerificationResultEnum::Pass { item, .. } => *item,
            StepVerificationResultEnum::Fail { item, .. } => *item,
            StepVerificationResultEnum::NotExecuted { item, .. } => *item,
        }
    }

    pub fn tc(&self) -> Option<i64> {
        match self {
            StepVerificationResultEnum::Pass { tc, .. } => *tc,
            StepVerificationResultEnum::Fail { tc, .. } => *tc,
            StepVerificationResultEnum::NotExecuted { tc, .. } => *tc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationDiff {
    pub result_diff: Option<DiffDetail>,
    pub output_diff: Option<DiffDetail>,
    pub success_diff: Option<DiffDetail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiffDetail {
    pub expected: String,
    pub actual: String,
    pub message: String,
}

/// Old-style execution verification result (for backward compatibility)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionVerificationResult {
    pub test_case_id: String,
    pub sequence_id: i64,
    pub overall_passed: bool,
    pub step_results: Vec<StepVerificationResult>,
    pub missing_steps: Vec<i64>,
    pub unexpected_steps: Vec<i64>,
}

/// Represents a parsed test execution log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionLog {
    /// Test case ID
    pub test_case_id: String,

    /// Test sequence ID
    pub sequence_id: i64,

    /// Step number
    pub step_number: i64,

    /// Whether the step succeeded
    pub success: Option<bool>,

    /// Actual result from execution
    pub actual_result: String,

    /// Actual output from execution
    pub actual_output: String,

    /// Timestamp of execution
    pub timestamp: Option<DateTime<Utc>>,

    /// Path to the log file
    pub log_file_path: PathBuf,

    /// Whether result verification passed (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_verification_pass: Option<bool>,

    /// Whether output verification passed (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_verification_pass: Option<bool>,
}

/// Result of verifying a test case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseVerificationResult {
    /// Test case ID
    pub test_case_id: String,

    /// Test case description
    pub description: String,

    /// Test sequence results
    pub sequences: Vec<SequenceVerificationResult>,

    /// Total number of steps
    pub total_steps: usize,

    /// Number of passed steps
    pub passed_steps: usize,

    /// Number of failed steps
    pub failed_steps: usize,

    /// Number of not executed steps
    pub not_executed_steps: usize,

    /// Overall pass/fail status
    pub overall_pass: bool,

    /// Requirement identifier (optional, for reporting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,

    /// Item number (optional, for reporting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<i64>,

    /// TC number (optional, for reporting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<i64>,
}

/// Result of verifying a test sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceVerificationResult {
    /// Sequence ID
    pub sequence_id: i64,

    /// Sequence name
    pub name: String,

    /// Step results
    pub step_results: Vec<StepVerificationResultEnum>,

    /// Whether all steps passed
    pub all_steps_passed: bool,

    /// Requirement identifier (optional, for reporting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<String>,

    /// Item number (optional, for reporting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<i64>,

    /// TC number (optional, for reporting)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc: Option<i64>,
}

/// Aggregated batch verification report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchVerificationReport {
    /// All test case results
    pub test_cases: Vec<TestCaseVerificationResult>,

    /// Total number of test cases
    pub total_test_cases: usize,

    /// Number of test cases that passed
    pub passed_test_cases: usize,

    /// Number of test cases that failed
    pub failed_test_cases: usize,

    /// Total steps across all test cases
    pub total_steps: usize,

    /// Passed steps across all test cases
    pub passed_steps: usize,

    /// Failed steps across all test cases
    pub failed_steps: usize,

    /// Not executed steps across all test cases
    pub not_executed_steps: usize,

    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
}

impl BatchVerificationReport {
    /// Create a new empty report
    pub fn new() -> Self {
        Self {
            test_cases: Vec::new(),
            total_test_cases: 0,
            passed_test_cases: 0,
            failed_test_cases: 0,
            total_steps: 0,
            passed_steps: 0,
            failed_steps: 0,
            not_executed_steps: 0,
            generated_at: Local::now().with_timezone(&Utc),
        }
    }

    /// Add a test case result to the report
    pub fn add_test_case_result(&mut self, result: TestCaseVerificationResult) {
        self.total_test_cases += 1;
        if result.overall_pass {
            self.passed_test_cases += 1;
        } else {
            self.failed_test_cases += 1;
        }

        self.total_steps += result.total_steps;
        self.passed_steps += result.passed_steps;
        self.failed_steps += result.failed_steps;
        self.not_executed_steps += result.not_executed_steps;

        self.test_cases.push(result);
    }

    /// Generate a summary string
    pub fn summary(&self) -> String {
        format!(
            "Test Cases: {}/{} passed, Steps: {}/{} passed ({} failed, {} not executed)",
            self.passed_test_cases,
            self.total_test_cases,
            self.passed_steps,
            self.total_steps,
            self.failed_steps,
            self.not_executed_steps
        )
    }
}

impl Default for BatchVerificationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata for container reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerReportMetadata {
    /// Optional environment information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Optional platform information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// Optional executor information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor: Option<String>,

    /// Execution duration in seconds
    pub execution_duration: f64,

    /// Total number of test cases
    pub total_test_cases: usize,

    /// Number of test cases that passed
    pub passed_test_cases: usize,

    /// Number of test cases that failed
    pub failed_test_cases: usize,
}

/// Container report for batch verification with enhanced metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerReport {
    /// Report title
    pub title: String,

    /// Project name
    pub project: String,

    /// Test execution date
    pub test_date: DateTime<Utc>,

    /// Test case results
    pub test_results: Vec<TestCaseVerificationResult>,

    /// Report metadata
    pub metadata: ContainerReportMetadata,
}

impl ContainerReport {
    /// Create a new ContainerReport from a BatchVerificationReport
    pub fn from_batch_report(
        batch_report: BatchVerificationReport,
        title: String,
        project: String,
        environment: Option<String>,
        platform: Option<String>,
        executor: Option<String>,
        execution_duration: f64,
    ) -> Self {
        Self {
            title,
            project,
            test_date: batch_report.generated_at,
            test_results: batch_report.test_cases,
            metadata: ContainerReportMetadata {
                environment,
                platform,
                executor,
                execution_duration,
                total_test_cases: batch_report.total_test_cases,
                passed_test_cases: batch_report.passed_test_cases,
                failed_test_cases: batch_report.failed_test_cases,
            },
        }
    }
}

/// JUnit XML test suite representation
#[derive(Debug, Clone)]
pub struct JUnitTestSuite {
    /// Test suite name
    pub name: String,

    /// Number of tests
    pub tests: usize,

    /// Number of failures
    pub failures: usize,

    /// Number of errors
    pub errors: usize,

    /// Number of skipped tests
    pub skipped: usize,

    /// Execution time in seconds
    pub time: f64,

    /// Test cases
    pub test_cases: Vec<JUnitTestCase>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// JUnit XML test case representation
#[derive(Debug, Clone)]
pub struct JUnitTestCase {
    /// Test case name
    pub name: String,

    /// Test class name
    pub classname: String,

    /// Execution time in seconds
    pub time: f64,

    /// Failure information
    pub failure: Option<JUnitFailure>,

    /// Whether test was skipped
    pub skipped: bool,
}

/// JUnit XML failure information
#[derive(Debug, Clone)]
pub struct JUnitFailure {
    /// Failure message
    pub message: String,

    /// Failure type
    pub failure_type: String,

    /// Failure details
    pub text: String,
}

impl JUnitTestSuite {
    /// Generate JUnit XML format
    pub fn to_xml(&self) -> Result<String> {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

        // Write XML declaration
        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .context("Failed to write XML declaration")?;

        // Write testsuite element
        let mut testsuite = BytesStart::new("testsuite");
        testsuite.push_attribute(("name", self.name.as_str()));
        testsuite.push_attribute(("tests", self.tests.to_string().as_str()));
        testsuite.push_attribute(("failures", self.failures.to_string().as_str()));
        testsuite.push_attribute(("errors", self.errors.to_string().as_str()));
        testsuite.push_attribute(("skipped", self.skipped.to_string().as_str()));
        testsuite.push_attribute(("time", format!("{:.3}", self.time).as_str()));
        testsuite.push_attribute((
            "timestamp",
            self.timestamp
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                .as_str(),
        ));

        writer
            .write_event(Event::Start(testsuite))
            .context("Failed to write testsuite start")?;

        // Write test cases
        for test_case in &self.test_cases {
            let mut testcase = BytesStart::new("testcase");
            testcase.push_attribute(("name", test_case.name.as_str()));
            testcase.push_attribute(("classname", test_case.classname.as_str()));
            testcase.push_attribute(("time", format!("{:.3}", test_case.time).as_str()));

            if test_case.skipped {
                writer
                    .write_event(Event::Start(testcase))
                    .context("Failed to write testcase start")?;
                writer
                    .write_event(Event::Empty(BytesStart::new("skipped")))
                    .context("Failed to write skipped element")?;
                writer
                    .write_event(Event::End(BytesEnd::new("testcase")))
                    .context("Failed to write testcase end")?;
            } else if let Some(failure) = &test_case.failure {
                writer
                    .write_event(Event::Start(testcase))
                    .context("Failed to write testcase start")?;

                let mut failure_elem = BytesStart::new("failure");
                failure_elem.push_attribute(("message", failure.message.as_str()));
                failure_elem.push_attribute(("type", failure.failure_type.as_str()));

                writer
                    .write_event(Event::Start(failure_elem))
                    .context("Failed to write failure start")?;
                writer
                    .write_event(Event::Text(BytesText::new(&failure.text)))
                    .context("Failed to write failure text")?;
                writer
                    .write_event(Event::End(BytesEnd::new("failure")))
                    .context("Failed to write failure end")?;

                writer
                    .write_event(Event::End(BytesEnd::new("testcase")))
                    .context("Failed to write testcase end")?;
            } else {
                writer
                    .write_event(Event::Empty(testcase))
                    .context("Failed to write testcase")?;
            }
        }

        writer
            .write_event(Event::End(BytesEnd::new("testsuite")))
            .context("Failed to write testsuite end")?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result).context("Failed to convert XML to string")
    }

    /// Convert from batch verification report
    pub fn from_batch_report(report: &BatchVerificationReport, suite_name: &str) -> Self {
        let mut junit_suite = JUnitTestSuite {
            name: suite_name.to_string(),
            tests: 0,
            failures: 0,
            errors: 0,
            skipped: 0,
            time: 0.0,
            test_cases: Vec::new(),
            timestamp: report.generated_at,
        };

        for tc_result in &report.test_cases {
            for seq_result in &tc_result.sequences {
                for step_result in &seq_result.step_results {
                    junit_suite.tests += 1;

                    let name = format!(
                        "{}.seq{}.step{}",
                        tc_result.test_case_id,
                        seq_result.sequence_id,
                        step_result.step_number()
                    );

                    let classname = format!("{}.{}", tc_result.test_case_id, seq_result.name);

                    let junit_tc = match step_result {
                        StepVerificationResultEnum::Pass { description, .. } => JUnitTestCase {
                            name: format!("{} - {}", name, description),
                            classname,
                            time: 0.0,
                            failure: None,
                            skipped: false,
                        },
                        StepVerificationResultEnum::Fail {
                            description,
                            expected,
                            actual_result,
                            actual_output,
                            reason,
                            ..
                        } => {
                            junit_suite.failures += 1;
                            JUnitTestCase {
                                name: format!("{} - {}", name, description),
                                classname,
                                time: 0.0,
                                failure: Some(JUnitFailure {
                                    message: reason.clone(),
                                    failure_type: "VerificationFailure".to_string(),
                                    text: format!(
                                        "Expected:\n  Result: {}\n  Output: {}\n\nActual:\n  Result: {}\n  Output: {}\n\nReason: {}",
                                        expected.result, expected.output, actual_result, actual_output, reason
                                    ),
                                }),
                                skipped: false,
                            }
                        }
                        StepVerificationResultEnum::NotExecuted { description, .. } => {
                            junit_suite.skipped += 1;
                            JUnitTestCase {
                                name: format!("{} - {}", name, description),
                                classname,
                                time: 0.0,
                                failure: None,
                                skipped: true,
                            }
                        }
                    };

                    junit_suite.test_cases.push(junit_tc);
                }
            }
        }

        junit_suite
    }
}

/// Test verifier for comparing execution logs against test cases
pub struct TestVerifier {
    storage: TestCaseStorage,
    result_strategy: MatchStrategy,
    output_strategy: MatchStrategy,
}

impl Default for TestVerifier {
    fn default() -> Self {
        Self::with_exact_matching()
    }
}

impl TestVerifier {
    pub fn from_storage(storage: TestCaseStorage) -> Self {
        Self {
            storage,
            result_strategy: MatchStrategy::Exact,
            output_strategy: MatchStrategy::Exact,
        }
    }

    pub fn with_strategies(
        storage: TestCaseStorage,
        result_strategy: MatchStrategy,
        output_strategy: MatchStrategy,
    ) -> Self {
        Self {
            storage,
            result_strategy,
            output_strategy,
        }
    }

    /// Get a reference to the test case storage
    pub fn storage(&self) -> &TestCaseStorage {
        &self.storage
    }

    /// Parse a test execution log file
    pub fn parse_log_file<P: AsRef<Path>>(&self, log_path: P) -> Result<Vec<TestExecutionLog>> {
        let log_path = log_path.as_ref();
        let content =
            fs::read_to_string(log_path).context("Failed to read test execution log file")?;

        self.parse_log_content(&content, log_path)
    }

    /// Parse a test execution log file with a specified test case ID
    /// This is useful when the log file doesn't follow the naming convention
    pub fn parse_log_file_with_test_case_id<P: AsRef<Path>>(
        &self,
        log_path: P,
        test_case_id: &str,
    ) -> Result<Vec<TestExecutionLog>> {
        let log_path = log_path.as_ref();
        let content =
            fs::read_to_string(log_path).context("Failed to read test execution log file")?;

        self.parse_log_content_with_test_case_id(&content, log_path, test_case_id)
    }

    /// Parse test execution log content with a specified test case ID
    pub fn parse_log_content_with_test_case_id(
        &self,
        content: &str,
        log_path: &Path,
        test_case_id: &str,
    ) -> Result<Vec<TestExecutionLog>> {
        let mut logs = Vec::new();

        log::debug!(
            "Parsing log file with test_case_id: {} from {} (length: {} bytes)",
            test_case_id,
            log_path.display(),
            content.len()
        );

        // Try to parse as JSON first
        let trimmed_content = content.trim();
        if trimmed_content.starts_with('[') {
            log::debug!("Log file appears to be JSON format (starts with '[')");
            // Attempt to parse as JSON array of TestStepExecutionEntry
            match self.parse_json_log_content_with_test_case_id(content, log_path, test_case_id) {
                Ok(json_logs) => {
                    log::debug!(
                        "Successfully parsed as JSON format: {} log entries",
                        json_logs.len()
                    );
                    return Ok(json_logs);
                }
                Err(e) => {
                    log::debug!(
                        "Failed to parse as JSON, falling back to text format: {}",
                        e
                    );
                }
            }
        } else {
            log::debug!("Log file appears to be text format (does not start with '[')");
        }

        // Fall back to text format parsing with the provided test_case_id
        let log_regex = Regex::new(
            r"(?x)
            (?:\[([^\]]+)\]\s+)?  # Optional timestamp
            TestCase:\s+([^,]+),\s+
            Sequence:\s+(\d+),\s+
            Step:\s+(\d+),\s+
            Success:\s+(true|false|null|none|-),\s+
            Result:\s+([^,]+),\s+
            Output:\s+(.+)
            ",
        )
        .context("Failed to compile log regex")?;

        for line in content.lines() {
            if let Some(caps) = log_regex.captures(line) {
                let timestamp = caps.get(1).and_then(|m| {
                    DateTime::parse_from_rfc3339(m.as_str())
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                });

                // Safe to unwrap: regex capture groups 3-7 are guaranteed by successful match
                let sequence_id = caps
                    .get(3)
                    .expect("Regex capture group 3 should exist")
                    .as_str()
                    .parse::<i64>()
                    .context("Failed to parse sequence ID")?;
                let step_number = caps
                    .get(4)
                    .expect("Regex capture group 4 should exist")
                    .as_str()
                    .parse::<i64>()
                    .context("Failed to parse step number")?;

                let success_str = caps
                    .get(5)
                    .expect("Regex capture group 5 should exist")
                    .as_str()
                    .to_lowercase();
                let success = match success_str.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };

                let actual_result = caps
                    .get(6)
                    .expect("Regex capture group 6 should exist")
                    .as_str()
                    .trim()
                    .to_string();
                let actual_output = caps
                    .get(7)
                    .expect("Regex capture group 7 should exist")
                    .as_str()
                    .trim()
                    .to_string();

                log::debug!(
                    "Parsed text log entry: TestCase={}, Seq={}, Step={}, Success={:?}, Result={}, Output={}",
                    test_case_id,
                    sequence_id,
                    step_number,
                    success,
                    actual_result,
                    actual_output
                );

                logs.push(TestExecutionLog {
                    test_case_id: test_case_id.to_string(),
                    sequence_id,
                    step_number,
                    success,
                    actual_result,
                    actual_output,
                    timestamp,
                    log_file_path: log_path.to_path_buf(),
                    result_verification_pass: None,
                    output_verification_pass: None,
                });
            }
        }

        log::debug!("Parsed {} text format log entries", logs.len());

        Ok(logs)
    }

    /// Parse test execution log content
    pub fn parse_log_content(
        &self,
        content: &str,
        log_path: &Path,
    ) -> Result<Vec<TestExecutionLog>> {
        let mut logs = Vec::new();

        log::debug!(
            "Parsing log file: {} (length: {} bytes)",
            log_path.display(),
            content.len()
        );

        // Try to parse as JSON first
        let trimmed_content = content.trim();
        if trimmed_content.starts_with('[') {
            log::debug!("Log file appears to be JSON format (starts with '[')");
            // Attempt to parse as JSON array of TestStepExecutionEntry
            match self.parse_json_log_content(content, log_path) {
                Ok(json_logs) => {
                    log::debug!(
                        "Successfully parsed as JSON format: {} log entries",
                        json_logs.len()
                    );
                    return Ok(json_logs);
                }
                Err(e) => {
                    log::debug!(
                        "Failed to parse as JSON, falling back to text format: {}",
                        e
                    );
                }
            }
        } else {
            log::debug!("Log file appears to be text format (does not start with '[')");
        }

        // Fall back to text format parsing
        // Regular expressions for parsing log entries
        // Format: [TIMESTAMP] TestCase: <id>, Sequence: <seq_id>, Step: <step_num>, Success: <true/false>, Result: <result>, Output: <output>
        let log_regex = Regex::new(
            r"(?x)
            (?:\[([^\]]+)\]\s+)?  # Optional timestamp
            TestCase:\s+([^,]+),\s+
            Sequence:\s+(\d+),\s+
            Step:\s+(\d+),\s+
            Success:\s+(true|false|null|none|-),\s+
            Result:\s+([^,]+),\s+
            Output:\s+(.+)
            ",
        )
        .context("Failed to compile log regex")?;

        for line in content.lines() {
            if let Some(caps) = log_regex.captures(line) {
                let timestamp = caps.get(1).and_then(|m| {
                    DateTime::parse_from_rfc3339(m.as_str())
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                });

                // Safe to unwrap: regex capture groups 2-7 are guaranteed by successful match
                let test_case_id = caps
                    .get(2)
                    .expect("Regex capture group 2 should exist")
                    .as_str()
                    .trim()
                    .to_string();
                let sequence_id = caps
                    .get(3)
                    .expect("Regex capture group 3 should exist")
                    .as_str()
                    .parse::<i64>()
                    .context("Failed to parse sequence ID")?;
                let step_number = caps
                    .get(4)
                    .expect("Regex capture group 4 should exist")
                    .as_str()
                    .parse::<i64>()
                    .context("Failed to parse step number")?;

                let success_str = caps
                    .get(5)
                    .expect("Regex capture group 5 should exist")
                    .as_str()
                    .to_lowercase();
                let success = match success_str.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };

                let actual_result = caps
                    .get(6)
                    .expect("Regex capture group 6 should exist")
                    .as_str()
                    .trim()
                    .to_string();
                let actual_output = caps
                    .get(7)
                    .expect("Regex capture group 7 should exist")
                    .as_str()
                    .trim()
                    .to_string();

                log::debug!(
                    "Parsed text log entry: TestCase={}, Seq={}, Step={}, Success={:?}, Result={}, Output={}",
                    test_case_id,
                    sequence_id,
                    step_number,
                    success,
                    actual_result,
                    actual_output
                );

                logs.push(TestExecutionLog {
                    test_case_id,
                    sequence_id,
                    step_number,
                    success,
                    actual_result,
                    actual_output,
                    timestamp,
                    log_file_path: log_path.to_path_buf(),
                    result_verification_pass: None,
                    output_verification_pass: None,
                });
            }
        }

        log::debug!("Parsed {} text format log entries", logs.len());

        Ok(logs)
    }

    /// Parse JSON-formatted execution log content with a specified test case ID
    fn parse_json_log_content_with_test_case_id(
        &self,
        content: &str,
        log_path: &Path,
        test_case_id: &str,
    ) -> Result<Vec<TestExecutionLog>> {
        use crate::models::TestStepExecutionEntry;

        log::debug!(
            "Parsing JSON log content with test_case_id: {}",
            test_case_id
        );

        let entries: Vec<TestStepExecutionEntry> =
            serde_json::from_str(content).context("Failed to parse JSON execution log")?;

        log::debug!("Successfully parsed {} JSON entries", entries.len());

        let mut logs = Vec::new();
        for entry in entries {
            // Derive success from exit_code (0 = success, non-zero = failure)
            let success = Some(entry.exit_code == 0);

            // Derive actual_result from exit_code
            let actual_result = entry.exit_code.to_string();

            // Parse timestamp if present
            let timestamp = entry.timestamp.as_ref().and_then(|ts| {
                DateTime::parse_from_rfc3339(ts)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            });

            log::debug!(
                "Parsed JSON entry: TestCase={}, Seq={}, Step={}, ExitCode={}, Success={:?}, Output={}",
                test_case_id,
                entry.test_sequence,
                entry.step,
                entry.exit_code,
                success,
                entry.output
            );

            logs.push(TestExecutionLog {
                test_case_id: test_case_id.to_string(),
                sequence_id: entry.test_sequence,
                step_number: entry.step,
                success,
                actual_result,
                actual_output: entry.output.clone(),
                timestamp,
                log_file_path: log_path.to_path_buf(),
                result_verification_pass: entry.result_verification_pass,
                output_verification_pass: entry.output_verification_pass,
            });
        }

        Ok(logs)
    }

    /// Parse JSON-formatted execution log content
    fn parse_json_log_content(
        &self,
        content: &str,
        log_path: &Path,
    ) -> Result<Vec<TestExecutionLog>> {
        use crate::models::TestStepExecutionEntry;

        let entries: Vec<TestStepExecutionEntry> =
            serde_json::from_str(content).context("Failed to parse JSON execution log")?;

        // Extract test_case_id from the log file path
        // Expected format: {test_case_id}_execution_log.json
        let test_case_id = log_path
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.strip_suffix("_execution_log"))
            .unwrap_or("UNKNOWN")
            .to_string();

        log::debug!(
            "Parsing JSON log content, extracted test_case_id: {} from path: {}",
            test_case_id,
            log_path.display()
        );
        log::debug!("Successfully parsed {} JSON entries", entries.len());

        let mut logs = Vec::new();
        for entry in entries {
            // Derive success from exit_code (0 = success, non-zero = failure)
            let success = Some(entry.exit_code == 0);

            // Derive actual_result from exit_code
            let actual_result = entry.exit_code.to_string();

            // Parse timestamp if present
            let timestamp = entry.timestamp.as_ref().and_then(|ts| {
                DateTime::parse_from_rfc3339(ts)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            });

            log::debug!(
                "Parsed JSON entry: TestCase={}, Seq={}, Step={}, ExitCode={}, Success={:?}, Output={}",
                test_case_id,
                entry.test_sequence,
                entry.step,
                entry.exit_code,
                success,
                entry.output
            );

            logs.push(TestExecutionLog {
                test_case_id: test_case_id.clone(),
                sequence_id: entry.test_sequence,
                step_number: entry.step,
                success,
                actual_result,
                actual_output: entry.output.clone(),
                timestamp,
                log_file_path: log_path.to_path_buf(),
                result_verification_pass: entry.result_verification_pass,
                output_verification_pass: entry.output_verification_pass,
            });
        }

        Ok(logs)
    }

    /// Verify a single test case against execution logs
    pub fn verify_test_case(
        &self,
        test_case: &TestCase,
        execution_logs: &[TestExecutionLog],
    ) -> TestCaseVerificationResult {
        log::debug!(
            "Verifying test case: {} ({}) with {} execution logs",
            test_case.id,
            test_case.description,
            execution_logs.len()
        );

        let mut sequences = Vec::new();
        let mut total_steps = 0;
        let mut passed_steps = 0;
        let mut failed_steps = 0;
        let mut not_executed_steps = 0;

        // Create a lookup map for execution logs
        let mut log_map: HashMap<(i64, i64), &TestExecutionLog> = HashMap::new();
        for log in execution_logs {
            if log.test_case_id == test_case.id {
                log_map.insert((log.sequence_id, log.step_number), log);
            }
        }

        log::debug!(
            "Built log map with {} entries for test case {}",
            log_map.len(),
            test_case.id
        );

        // Extract requirement, item, and tc from test case
        let requirement = Some(test_case.requirement.clone());
        let item = Some(test_case.item);
        let tc = Some(test_case.tc);

        for sequence in &test_case.test_sequences {
            log::debug!(
                "Verifying sequence {} ({}) with {} steps",
                sequence.id,
                sequence.name,
                sequence.steps.len()
            );

            let step_results =
                self.verify_sequence(sequence, &log_map, requirement.as_ref(), item, tc);

            let all_steps_passed = step_results.iter().all(|r| r.is_pass());

            for result in &step_results {
                total_steps += 1;
                match result {
                    StepVerificationResultEnum::Pass { .. } => passed_steps += 1,
                    StepVerificationResultEnum::Fail { .. } => failed_steps += 1,
                    StepVerificationResultEnum::NotExecuted { .. } => not_executed_steps += 1,
                }
            }

            log::debug!(
                "Sequence {} verification complete: all_passed={}",
                sequence.id,
                all_steps_passed
            );

            sequences.push(SequenceVerificationResult {
                sequence_id: sequence.id,
                name: sequence.name.clone(),
                step_results,
                all_steps_passed,
                requirement: requirement.clone(),
                item,
                tc,
            });
        }

        let overall_pass = failed_steps == 0 && not_executed_steps == 0;

        log::debug!(
            "Test case {} verification complete: passed={}/{}, failed={}, not_executed={}, overall_pass={}",
            test_case.id,
            passed_steps,
            total_steps,
            failed_steps,
            not_executed_steps,
            overall_pass
        );

        TestCaseVerificationResult {
            test_case_id: test_case.id.clone(),
            description: test_case.description.clone(),
            sequences,
            total_steps,
            passed_steps,
            failed_steps,
            not_executed_steps,
            overall_pass,
            requirement: requirement.clone(),
            item,
            tc,
        }
    }

    /// Verify a single sequence against execution logs
    fn verify_sequence(
        &self,
        sequence: &TestSequence,
        log_map: &HashMap<(i64, i64), &TestExecutionLog>,
        requirement: Option<&String>,
        item: Option<i64>,
        tc: Option<i64>,
    ) -> Vec<StepVerificationResultEnum> {
        let mut results = Vec::new();

        for step in &sequence.steps {
            let result = if let Some(log) = log_map.get(&(sequence.id, step.step)) {
                log::debug!(
                    "Found execution log for sequence {} step {}, verifying...",
                    sequence.id,
                    step.step
                );
                self.verify_step_new(step, log, requirement, item, tc)
            } else {
                log::debug!(
                    "No execution log found for sequence {} step {} - marking as NotExecuted",
                    sequence.id,
                    step.step
                );
                StepVerificationResultEnum::NotExecuted {
                    step: step.step,
                    description: step.description.clone(),
                    requirement: requirement.cloned(),
                    item,
                    tc,
                }
            };
            results.push(result);
        }

        results
    }

    /// Verify a single step against its execution log (new API)
    fn verify_step_new(
        &self,
        step: &Step,
        log: &TestExecutionLog,
        requirement: Option<&String>,
        item: Option<i64>,
        tc: Option<i64>,
    ) -> StepVerificationResultEnum {
        let expected = &step.expected;

        // Precomputed mode: check precomputed verification fields
        if self.result_strategy == MatchStrategy::Precomputed
            || self.output_strategy == MatchStrategy::Precomputed
        {
            // Check result verification if in Precomputed mode
            if self.result_strategy == MatchStrategy::Precomputed
                && log.result_verification_pass != Some(true)
            {
                return StepVerificationResultEnum::Fail {
                    step: step.step,
                    description: step.description.clone(),
                    expected: expected.clone(),
                    actual_result: log.actual_result.clone(),
                    actual_output: log.actual_output.clone(),
                    reason: "Result verification failed (precomputed)".to_string(),
                    requirement: requirement.cloned(),
                    item,
                    tc,
                };
            }

            // Check output verification if in Precomputed mode
            if self.output_strategy == MatchStrategy::Precomputed
                && log.output_verification_pass != Some(true)
            {
                return StepVerificationResultEnum::Fail {
                    step: step.step,
                    description: step.description.clone(),
                    expected: expected.clone(),
                    actual_result: log.actual_result.clone(),
                    actual_output: log.actual_output.clone(),
                    reason: "Output verification failed (precomputed)".to_string(),
                    requirement: requirement.cloned(),
                    item,
                    tc,
                };
            }

            // In Precomputed mode, skip success field check and pass
            return StepVerificationResultEnum::Pass {
                step: step.step,
                description: step.description.clone(),
                requirement: requirement.cloned(),
                item,
                tc,
            };
        }

        // Non-Precomputed mode: standard verification logic
        log::debug!(
            "Verifying step {} ({}): Expected[result='{}', output='{}', success={:?}]",
            step.step,
            step.description,
            expected.result,
            expected.output,
            expected.success
        );
        log::debug!(
            "  Actual from log: result='{}', output='{}', success={:?}",
            log.actual_result,
            log.actual_output,
            log.success
        );

        // Check success field if it's defined
        if let Some(expected_success) = expected.success {
            if let Some(actual_success) = log.success {
                log::debug!(
                    "  Checking success: expected={}, actual={}",
                    expected_success,
                    actual_success
                );
                if expected_success != actual_success {
                    log::debug!("  SUCCESS CHECK FAILED: mismatch detected");
                    return StepVerificationResultEnum::Fail {
                        step: step.step,
                        description: step.description.clone(),
                        expected: expected.clone(),
                        actual_result: log.actual_result.clone(),
                        actual_output: log.actual_output.clone(),
                        reason: format!(
                            "Success mismatch: expected {}, got {}",
                            expected_success, actual_success
                        ),
                        requirement: requirement.cloned(),
                        item,
                        tc,
                    };
                }
                log::debug!("  Success check passed");
            }
        }

        // Check result
        log::debug!(
            "  Checking result: expected='{}', actual='{}', strategy={:?}",
            expected.result,
            log.actual_result,
            self.result_strategy
        );
        if !self.matches(&expected.result, &log.actual_result, self.result_strategy) {
            log::debug!(
                "  RESULT CHECK FAILED: no match with strategy {:?}",
                self.result_strategy
            );
            return StepVerificationResultEnum::Fail {
                step: step.step,
                description: step.description.clone(),
                expected: expected.clone(),
                actual_result: log.actual_result.clone(),
                actual_output: log.actual_output.clone(),
                reason: format!(
                    "Result mismatch: expected '{}', got '{}'",
                    expected.result, log.actual_result
                ),
                requirement: requirement.cloned(),
                item,
                tc,
            };
        }
        log::debug!("  Result check passed");

        // Check output
        log::debug!(
            "  Checking output: expected='{}', actual='{}', strategy={:?}",
            expected.output,
            log.actual_output,
            self.output_strategy
        );
        if !self.matches(&expected.output, &log.actual_output, self.output_strategy) {
            log::debug!(
                "  OUTPUT CHECK FAILED: no match with strategy {:?}",
                self.output_strategy
            );
            return StepVerificationResultEnum::Fail {
                step: step.step,
                description: step.description.clone(),
                expected: expected.clone(),
                actual_result: log.actual_result.clone(),
                actual_output: log.actual_output.clone(),
                reason: format!(
                    "Output mismatch: expected '{}', got '{}'",
                    expected.output, log.actual_output
                ),
                requirement: requirement.cloned(),
                item,
                tc,
            };
        }
        log::debug!("  Output check passed");

        log::debug!("  Step {} PASSED all checks", step.step);

        StepVerificationResultEnum::Pass {
            step: step.step,
            description: step.description.clone(),
            requirement: requirement.cloned(),
            item,
            tc,
        }
    }

    fn matches(&self, expected: &str, actual: &str, strategy: MatchStrategy) -> bool {
        let result = match strategy {
            MatchStrategy::Exact => {
                let matched = expected == actual;
                log::debug!(
                    "    Match check (Exact): expected='{}', actual='{}' => {}",
                    expected,
                    actual,
                    matched
                );
                matched
            }
            MatchStrategy::Contains => {
                let matched = actual.contains(expected);
                log::debug!(
                    "    Match check (Contains): expected='{}', actual='{}' => {}",
                    expected,
                    actual,
                    matched
                );
                matched
            }
            MatchStrategy::Regex => {
                if let Ok(regex) = Regex::new(expected) {
                    let matched = regex.is_match(actual);
                    log::debug!(
                        "    Match check (Regex): pattern='{}', actual='{}' => {}",
                        expected,
                        actual,
                        matched
                    );
                    matched
                } else {
                    log::debug!(
                        "    Match check (Regex): INVALID pattern='{}' - compilation failed",
                        expected
                    );
                    false
                }
            }
            MatchStrategy::Precomputed => {
                // Precomputed strategy should not be used with matches()
                // This should be handled separately in verify_step_new
                false
            }
        };
        result
    }

    /// Process multiple log files and verify against test cases
    pub fn batch_verify<P: AsRef<Path>>(&self, log_paths: &[P]) -> Result<BatchVerificationReport> {
        let mut report = BatchVerificationReport::new();
        let mut all_logs: HashMap<String, Vec<TestExecutionLog>> = HashMap::new();

        // Parse all log files
        for log_path in log_paths {
            let logs = self.parse_log_file(log_path)?;
            for log in logs {
                all_logs
                    .entry(log.test_case_id.clone())
                    .or_default()
                    .push(log);
            }
        }

        // Verify each test case
        for (test_case_id, logs) in all_logs.iter() {
            // Try to load test case
            match self.storage.load_test_case_by_id(test_case_id) {
                Ok(test_case) => {
                    let result = self.verify_test_case(&test_case, logs);
                    report.add_test_case_result(result);
                }
                Err(e) => {
                    log::warn!(
                        "Failed to load test case '{}': {}. Skipping verification.",
                        test_case_id,
                        e
                    );
                    // Create a failed result for missing test case
                    let failed_result = TestCaseVerificationResult {
                        test_case_id: test_case_id.clone(),
                        description: format!("Test case not found: {}", e),
                        sequences: Vec::new(),
                        total_steps: logs.len(),
                        passed_steps: 0,
                        failed_steps: logs.len(),
                        not_executed_steps: 0,
                        overall_pass: false,
                        requirement: None,
                        item: None,
                        tc: None,
                    };
                    report.add_test_case_result(failed_result);
                }
            }
        }

        Ok(report)
    }

    // ========================================================================
    // Old-style API methods (for backward compatibility with existing tests)
    // ========================================================================

    /// Create verifier with exact matching (old API)
    pub fn with_exact_matching() -> Self {
        Self::new(MatchStrategy::Exact, MatchStrategy::Exact)
    }

    /// Create verifier with custom match strategies (old API, for tests)
    /// This creates a temporary storage for testing purposes
    #[allow(clippy::new_ret_no_self)]
    pub fn new(result_strategy: MatchStrategy, output_strategy: MatchStrategy) -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        Self::with_strategies(storage, result_strategy, output_strategy)
    }

    /// Verify a single step against an execution log (new two-stage workflow)
    pub fn verify_step_from_log(
        &self,
        step: &Step,
        log: &TestExecutionLog,
    ) -> StepVerificationResult {
        // Handle Precomputed strategy
        let result_match = if self.result_strategy == MatchStrategy::Precomputed {
            log.result_verification_pass == Some(true)
        } else {
            self.matches(
                &step.expected.result,
                &log.actual_result,
                self.result_strategy,
            )
        };

        let output_match = if self.output_strategy == MatchStrategy::Precomputed {
            log.output_verification_pass == Some(true)
        } else {
            self.matches(
                &step.expected.output,
                &log.actual_output,
                self.output_strategy,
            )
        };

        let mut success_match = true;
        let mut success_diff = None;

        // Skip success field check if either strategy is Precomputed
        if self.result_strategy != MatchStrategy::Precomputed
            && self.output_strategy != MatchStrategy::Precomputed
        {
            if let Some(expected_success) = step.expected.success {
                if let Some(actual_success) = log.success {
                    success_match = expected_success == actual_success;
                    if !success_match {
                        success_diff = Some(DiffDetail {
                            expected: expected_success.to_string(),
                            actual: actual_success.to_string(),
                            message: "Success flag mismatch".to_string(),
                        });
                    }
                }
            }
        }

        let passed = result_match && output_match && success_match;

        let result_diff = if !result_match {
            Some(DiffDetail {
                expected: step.expected.result.clone(),
                actual: log.actual_result.clone(),
                message: format!(
                    "Result mismatch ({})",
                    self.strategy_name(self.result_strategy)
                ),
            })
        } else {
            None
        };

        let output_diff = if !output_match {
            Some(DiffDetail {
                expected: step.expected.output.clone(),
                actual: log.actual_output.clone(),
                message: format!(
                    "Output mismatch ({})",
                    self.strategy_name(self.output_strategy)
                ),
            })
        } else {
            None
        };

        StepVerificationResult {
            step_number: step.step,
            passed,
            result_match,
            output_match,
            success_match,
            diff: VerificationDiff {
                result_diff,
                output_diff,
                success_diff,
            },
        }
    }

    fn strategy_name(&self, strategy: MatchStrategy) -> &'static str {
        match strategy {
            MatchStrategy::Exact => "Exact",
            MatchStrategy::Regex => "Regex",
            MatchStrategy::Contains => "Contains",
            MatchStrategy::Precomputed => "Precomputed",
        }
    }

    /// Verify a single step with old-style result (struct-based)
    pub fn verify_step(&self, step: &Step, actual: &ActualResult) -> StepVerificationResult {
        let result_match =
            self.matches(&step.expected.result, &actual.result, self.result_strategy);
        let output_match =
            self.matches(&step.expected.output, &actual.output, self.output_strategy);

        let success_match = match step.expected.success {
            Some(expected_success) => expected_success == actual.success,
            None => true,
        };

        let passed = result_match && output_match && success_match;

        let diff = VerificationDiff {
            result_diff: if !result_match {
                Some(DiffDetail {
                    expected: step.expected.result.clone(),
                    actual: actual.result.clone(),
                    message: format!(
                        "Result mismatch (strategy: {:?}): expected '{}' but got '{}'",
                        self.result_strategy, step.expected.result, actual.result
                    ),
                })
            } else {
                None
            },
            output_diff: if !output_match {
                Some(DiffDetail {
                    expected: step.expected.output.clone(),
                    actual: actual.output.clone(),
                    message: format!(
                        "Output mismatch (strategy: {:?}): expected '{}' but got '{}'",
                        self.output_strategy, step.expected.output, actual.output
                    ),
                })
            } else {
                None
            },
            success_diff: if !success_match {
                Some(DiffDetail {
                    expected: step
                        .expected
                        .success
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "None".to_string()),
                    actual: actual.success.to_string(),
                    message: format!(
                        "Success flag mismatch: expected {:?} but got {}",
                        step.expected.success, actual.success
                    ),
                })
            } else {
                None
            },
        };

        StepVerificationResult {
            step_number: step.step,
            passed,
            result_match,
            output_match,
            success_match,
            diff,
        }
    }

    /// Verify execution log with old-style result
    pub fn verify_execution_log(
        &self,
        test_case: &TestCase,
        execution_log: &crate::models::TestExecutionLog,
    ) -> ExecutionVerificationResult {
        let sequence = test_case
            .test_sequences
            .iter()
            .find(|seq| seq.id == execution_log.sequence_id);

        let Some(sequence) = sequence else {
            return ExecutionVerificationResult {
                test_case_id: test_case.id.clone(),
                sequence_id: execution_log.sequence_id,
                overall_passed: false,
                step_results: vec![],
                missing_steps: vec![],
                unexpected_steps: vec![],
            };
        };

        let mut step_results = Vec::new();
        for step in &sequence.steps {
            let actual_result = ActualResult {
                result: execution_log.actual_output.clone(),
                output: execution_log.actual_output.clone(),
                success: execution_log.actual_success,
            };
            let verification_result = self.verify_step(step, &actual_result);
            step_results.push(verification_result);
        }

        let overall_passed = step_results.iter().all(|r| r.passed);

        ExecutionVerificationResult {
            test_case_id: test_case.id.clone(),
            sequence_id: execution_log.sequence_id,
            overall_passed,
            step_results,
            missing_steps: vec![],
            unexpected_steps: vec![],
        }
    }

    // ========================================================================
    // Report Generation Methods
    // ========================================================================

    /// Generate YAML report for a single test case verification result
    pub fn generate_report_yaml(&self, result: &TestCaseVerificationResult) -> Result<String> {
        serde_yaml::to_string(result).context("Failed to serialize verification result to YAML")
    }

    /// Generate JSON report for a single test case verification result
    pub fn generate_report_json(&self, result: &TestCaseVerificationResult) -> Result<String> {
        serde_json::to_string_pretty(result)
            .context("Failed to serialize verification result to JSON")
    }

    /// Generate container report for batch verification (multiple test cases)
    /// Supports both YAML and JSON formats
    pub fn generate_container_report(
        &self,
        report: &BatchVerificationReport,
        format: &str,
    ) -> Result<String> {
        log::debug!(
            "Generating container report: format={}, test_cases={}, total_steps={}, passed={}, failed={}, not_executed={}",
            format,
            report.total_test_cases,
            report.total_steps,
            report.passed_steps,
            report.failed_steps,
            report.not_executed_steps
        );

        let result = match format.to_lowercase().as_str() {
            "yaml" => {
                log::debug!("Serializing report to YAML format");
                serde_yaml::to_string(report).context("Failed to serialize batch report to YAML")
            }
            "json" => {
                log::debug!("Serializing report to JSON format");
                serde_json::to_string_pretty(report)
                    .context("Failed to serialize batch report to JSON")
            }
            _ => {
                log::debug!("Unsupported format requested: {}", format);
                anyhow::bail!("Unsupported format: {}. Use 'yaml' or 'json'.", format)
            }
        };

        if let Ok(ref content) = result {
            log::debug!("Successfully generated report: {} bytes", content.len());
        } else {
            log::debug!("Failed to generate report");
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_log_content() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::from_storage(storage);

        let log_content = r#"
[2024-01-15T10:30:00Z] TestCase: TC001, Sequence: 1, Step: 1, Success: true, Result: SW=0x9000, Output: Success
[2024-01-15T10:30:01Z] TestCase: TC001, Sequence: 1, Step: 2, Success: true, Result: OK, Output: Done
"#;

        let logs = verifier
            .parse_log_content(log_content, Path::new("test.log"))
            .unwrap();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].test_case_id, "TC001");
        assert_eq!(logs[0].sequence_id, 1);
        assert_eq!(logs[0].step_number, 1);
        assert_eq!(logs[0].success, Some(true));
    }

    // Note: Internal verification tests are covered by tests/verification_test.rs

    #[test]
    fn test_junit_xml_generation() {
        let suite = JUnitTestSuite {
            name: "Test Suite".to_string(),
            tests: 2,
            failures: 1,
            errors: 0,
            skipped: 0,
            time: 1.5,
            test_cases: vec![
                JUnitTestCase {
                    name: "test1".to_string(),
                    classname: "TestClass".to_string(),
                    time: 0.5,
                    failure: None,
                    skipped: false,
                },
                JUnitTestCase {
                    name: "test2".to_string(),
                    classname: "TestClass".to_string(),
                    time: 1.0,
                    failure: Some(JUnitFailure {
                        message: "Test failed".to_string(),
                        failure_type: "AssertionError".to_string(),
                        text: "Expected: true, Got: false".to_string(),
                    }),
                    skipped: false,
                },
            ],
            timestamp: Local::now().with_timezone(&Utc),
        };

        let xml = suite.to_xml().unwrap();
        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("tests=\"2\""));
        assert!(xml.contains("failures=\"1\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("<failure"));
    }

    #[test]
    fn test_generate_report_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::from_storage(storage);

        let result = TestCaseVerificationResult {
            test_case_id: "TC001".to_string(),
            description: "Test Case 1".to_string(),
            sequences: vec![],
            total_steps: 5,
            passed_steps: 4,
            failed_steps: 1,
            not_executed_steps: 0,
            overall_pass: false,
            requirement: Some("REQ001".to_string()),
            item: Some(1),
            tc: Some(1),
        };

        let yaml = verifier.generate_report_yaml(&result).unwrap();
        assert!(yaml.contains("test_case_id: TC001"));
        assert!(yaml.contains("description: Test Case 1"));
        assert!(yaml.contains("total_steps: 5"));
        assert!(yaml.contains("passed_steps: 4"));
        assert!(yaml.contains("failed_steps: 1"));
        assert!(yaml.contains("overall_pass: false"));
        assert!(yaml.contains("requirement: REQ001"));
    }

    #[test]
    fn test_generate_report_json() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::from_storage(storage);

        let result = TestCaseVerificationResult {
            test_case_id: "TC002".to_string(),
            description: "Test Case 2".to_string(),
            sequences: vec![],
            total_steps: 3,
            passed_steps: 3,
            failed_steps: 0,
            not_executed_steps: 0,
            overall_pass: true,
            requirement: Some("REQ002".to_string()),
            item: Some(2),
            tc: Some(2),
        };

        let json = verifier.generate_report_json(&result).unwrap();
        assert!(json.contains("\"test_case_id\": \"TC002\""));
        assert!(json.contains("\"description\": \"Test Case 2\""));
        assert!(json.contains("\"total_steps\": 3"));
        assert!(json.contains("\"passed_steps\": 3"));
        assert!(json.contains("\"failed_steps\": 0"));
        assert!(json.contains("\"overall_pass\": true"));
        assert!(json.contains("\"requirement\": \"REQ002\""));

        // Verify it can be deserialized
        let parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.test_case_id, "TC002");
        assert_eq!(parsed.total_steps, 3);
        assert!(parsed.overall_pass);
    }

    #[test]
    fn test_generate_container_report_json() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::from_storage(storage);

        let mut report = BatchVerificationReport::new();
        report.add_test_case_result(TestCaseVerificationResult {
            test_case_id: "TC001".to_string(),
            description: "Test 1".to_string(),
            sequences: vec![],
            total_steps: 2,
            passed_steps: 2,
            failed_steps: 0,
            not_executed_steps: 0,
            overall_pass: true,
            requirement: None,
            item: None,
            tc: None,
        });
        report.add_test_case_result(TestCaseVerificationResult {
            test_case_id: "TC002".to_string(),
            description: "Test 2".to_string(),
            sequences: vec![],
            total_steps: 3,
            passed_steps: 2,
            failed_steps: 1,
            not_executed_steps: 0,
            overall_pass: false,
            requirement: None,
            item: None,
            tc: None,
        });

        let json = verifier.generate_container_report(&report, "json").unwrap();
        assert!(json.contains("\"test_cases\""));
        assert!(json.contains("\"total_test_cases\": 2"));
        assert!(json.contains("\"passed_test_cases\": 1"));
        assert!(json.contains("\"failed_test_cases\": 1"));
        assert!(json.contains("\"total_steps\": 5"));
        assert!(json.contains("\"passed_steps\": 4"));
        assert!(json.contains("\"failed_steps\": 1"));

        // Verify it can be deserialized
        let parsed: BatchVerificationReport = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.total_test_cases, 2);
        assert_eq!(parsed.passed_test_cases, 1);
        assert_eq!(parsed.failed_test_cases, 1);
    }

    #[test]
    fn test_generate_container_report_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::from_storage(storage);

        let mut report = BatchVerificationReport::new();
        report.add_test_case_result(TestCaseVerificationResult {
            test_case_id: "TC001".to_string(),
            description: "Test 1".to_string(),
            sequences: vec![],
            total_steps: 1,
            passed_steps: 1,
            failed_steps: 0,
            not_executed_steps: 0,
            overall_pass: true,
            requirement: None,
            item: None,
            tc: None,
        });

        let yaml = verifier.generate_container_report(&report, "yaml").unwrap();
        assert!(yaml.contains("test_cases:"));
        assert!(yaml.contains("total_test_cases: 1"));
        assert!(yaml.contains("passed_test_cases: 1"));
        assert!(yaml.contains("failed_test_cases: 0"));

        // Verify it can be deserialized
        let parsed: BatchVerificationReport = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(parsed.total_test_cases, 1);
        assert_eq!(parsed.passed_test_cases, 1);
    }

    #[test]
    fn test_generate_container_report_unsupported_format() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::from_storage(storage);

        let report = BatchVerificationReport::new();

        let result = verifier.generate_container_report(&report, "xml");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported format"));
    }
}
