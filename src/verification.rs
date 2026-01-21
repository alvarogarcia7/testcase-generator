use crate::models::ActualResult;

use crate::models::Step;
use crate::models::TestCase;
use crate::models::TestSequence;
use crate::storage::TestCaseStorage;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use quick_xml::events::BytesDecl;
use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use quick_xml::Writer;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use crate::models::Expected;
use crate::storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStrategy {
    Exact,
    Regex,
    Contains,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepVerificationResultY {
    pub step_number: i64,
    pub passed: bool,
    pub result_match: bool,
    pub output_match: bool,
    pub success_match: bool,
    pub diff: VerificationDiff,
}

/// Result of verifying a single step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepVerificationResultX {
    /// Step passed verification
    Pass { step: i64, description: String },
    /// Step failed verification
    Fail {
        step: i64,
        description: String,
        expected: Expected,
        actual_result: String,
        actual_output: String,
        reason: String,
    },
    /// Step was not found in execution log
    NotExecuted { step: i64, description: String },
}

impl StepVerificationResultX {
    pub fn is_pass(&self) -> bool {
        matches!(self, StepVerificationResultX::Pass { .. })
    }

    pub fn step_number(&self) -> i64 {
        match self {
            StepVerificationResultX::Pass { step, .. } => *step,
            StepVerificationResultX::Fail { step, .. } => *step,
            StepVerificationResultX::NotExecuted { step, .. } => *step,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionVerificationResult {
    pub test_case_id: String,
    pub sequence_id: i64,
    pub overall_passed: bool,
    pub step_results: Vec<StepVerificationResultX>,
    pub missing_steps: Vec<i64>,
    pub unexpected_steps: Vec<i64>,
}

impl Default for TestVerifier {
    fn default() -> Self {
        Self::with_exact_matching()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Verification;
    use super::*;

    fn create_test_step(step_num: i64, result: &str, output: &str, success: Option<bool>) -> Step {
        Step {
            verification: Verification,
            step: step_num,
            manual: None,
            description: "Test step".to_string(),
            command: "test command".to_string(),
            expected: Expected {
                success,
                result: result.to_string(),
                output: output.to_string(),
            },
        }
    }

    fn create_actual_result(result: &str, output: &str, success: bool) -> ActualResult {
        ActualResult {
            result: result.to_string(),
            output: output.to_string(),
            success,
        }
    }

    #[test]
    fn test_exact_match_success() {
        let verifier = TestVerifier::with_exact_matching();
        let step = create_test_step(1, "SW=0x9000", "Success", Some(true));
        let actual = create_actual_result("SW=0x9000", "Success", true);

        let result = verifier.verify_step(&step, &actual);

        assert!(result.passed);
        assert!(result.result_match);
        assert!(result.output_match);
        assert!(result.success_match);
        assert!(result.diff.result_diff.is_none());
        assert!(result.diff.output_diff.is_none());
        assert!(result.diff.success_diff.is_none());
    }

    #[test]
    fn test_exact_match_failure() {
        let verifier = TestVerifier::with_exact_matching();
        let step = create_test_step(1, "SW=0x9000", "Success", Some(true));
        let actual = create_actual_result("SW=0x6A82", "Failed", false);

        let result = verifier.verify_step(&step, &actual);

        assert!(!result.passed);
        assert!(!result.result_match);
        assert!(!result.output_match);
        assert!(!result.success_match);
        assert!(result.diff.result_diff.is_some());
        assert!(result.diff.output_diff.is_some());
        assert!(result.diff.success_diff.is_some());
    }

    #[test]
    fn test_contains_strategy() {
        let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
        let step = create_test_step(1, "9000", "Success", None);
        let actual = create_actual_result("SW=0x9000", "Operation Success", true);

        let result = verifier.verify_step(&step, &actual);

        assert!(result.passed);
        assert!(result.result_match);
        assert!(result.output_match);
        assert!(result.success_match);
    }

    #[test]
    fn test_regex_strategy() {
        let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
        let step = create_test_step(1, r"SW=0x[0-9A-Fa-f]{4}", r"Suc\w+", None);
        let actual = create_actual_result("SW=0x9000", "Success", true);

        let result = verifier.verify_step(&step, &actual);

        assert!(result.passed);
        assert!(result.result_match);
        assert!(result.output_match);
    }

    #[test]
    fn test_regex_strategy_invalid_regex() {
        let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Exact);
        let step = create_test_step(1, "[invalid(regex", "Success", None);
        let actual = create_actual_result("[invalid(regex", "Success", true);

        let result = verifier.verify_step(&step, &actual);

        assert!(!result.result_match);
    }

    #[test]
    fn test_success_flag_none_always_matches() {
        let verifier = TestVerifier::with_exact_matching();
        let step = create_test_step(1, "result", "output", None);
        let actual_true = create_actual_result("result", "output", true);
        let actual_false = create_actual_result("result", "output", false);

        let result_true = verifier.verify_step(&step, &actual_true);
        let result_false = verifier.verify_step(&step, &actual_false);

        assert!(result_true.success_match);
        assert!(result_false.success_match);
        assert!(result_true.passed);
        assert!(result_false.passed);
    }

    #[test]
    fn test_partial_match() {
        let verifier = TestVerifier::with_exact_matching();
        let step = create_test_step(1, "SW=0x9000", "Wrong", Some(true));
        let actual = create_actual_result("SW=0x9000", "Success", true);

        let result = verifier.verify_step(&step, &actual);

        assert!(!result.passed);
        assert!(result.result_match);
        assert!(!result.output_match);
        assert!(result.success_match);
        assert!(result.diff.result_diff.is_none());
        assert!(result.diff.output_diff.is_some());
        assert!(result.diff.success_diff.is_none());
    }

    #[test]
    fn test_diff_messages() {
        let verifier = TestVerifier::with_exact_matching();
        let step = create_test_step(1, "expected_result", "expected_output", Some(true));
        let actual = create_actual_result("actual_result", "actual_output", false);

        let result = verifier.verify_step(&step, &actual);

        let result_diff = result.diff.result_diff.as_ref().unwrap();
        assert_eq!(result_diff.expected, "expected_result");
        assert_eq!(result_diff.actual, "actual_result");
        assert!(result_diff.message.contains("Result mismatch"));

        let output_diff = result.diff.output_diff.as_ref().unwrap();
        assert_eq!(output_diff.expected, "expected_output");
        assert_eq!(output_diff.actual, "actual_output");
        assert!(output_diff.message.contains("Output mismatch"));

        let success_diff = result.diff.success_diff.as_ref().unwrap();
        assert_eq!(success_diff.expected, "true");
        assert_eq!(success_diff.actual, "false");
        assert!(success_diff.message.contains("Success flag mismatch"));
    }

    #[test]
    fn test_verify_execution_log_with_missing_sequence() {
        let verifier = TestVerifier::with_exact_matching();
        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test case".to_string(),
        );

        let execution_log = TestExecutionLog {
            test_case_id: "TC001".to_string(),
            sequence_id: 999,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            actual_output: "output".to_string(),
            actual_success: true,
            duration_ms: 1000,
            error_message: None,
        };

        let result = verifier.verify_execution_log(&test_case, &execution_log);

        assert!(!result.overall_passed);
        assert!(result.step_results.is_empty());
        assert_eq!(result.test_case_id, "TC001");
        assert_eq!(result.sequence_id, 999);
    }

    #[test]
    fn test_verify_execution_log_all_steps_pass() {
        let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);

        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test case".to_string(),
        );

        let mut sequence = crate::models::TestSequence::new(
            1,
            "Sequence 1".to_string(),
            "Description".to_string(),
        );

        let step1 = create_test_step(1, "Success", "Output", Some(true));
        let step2 = create_test_step(2, "Complete", "Done", Some(true));
        sequence.steps.push(step1);
        sequence.steps.push(step2);
        test_case.test_sequences.push(sequence);

        let execution_log = TestExecutionLog {
            test_case_id: "TC001".to_string(),
            sequence_id: 1,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            actual_output: "Success Output Complete Done".to_string(),
            actual_success: true,
            duration_ms: 1000,
            error_message: None,
        };

        let result = verifier.verify_execution_log(&test_case, &execution_log);

        assert!(result.overall_passed);
        assert_eq!(result.step_results.len(), 2);
        assert!(result.step_results.iter().all(|r| r.passed));
    }

    #[test]
    fn test_verify_execution_log_some_steps_fail() {
        let verifier = TestVerifier::with_exact_matching();

        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test case".to_string(),
        );

        let mut sequence = crate::models::TestSequence::new(
            1,
            "Sequence 1".to_string(),
            "Description".to_string(),
        );

        let step1 = create_test_step(1, "Expected", "Output", Some(true));
        let step2 = create_test_step(2, "Another", "Result", Some(true));
        sequence.steps.push(step1);
        sequence.steps.push(step2);
        test_case.test_sequences.push(sequence);

        let execution_log = TestExecutionLog {
            test_case_id: "TC001".to_string(),
            sequence_id: 1,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            actual_output: "Wrong output".to_string(),
            actual_success: true,
            duration_ms: 1000,
            error_message: None,
        };

        let result = verifier.verify_execution_log(&test_case, &execution_log);

        assert!(!result.overall_passed);
        assert_eq!(result.step_results.len(), 2);
        assert!(result.step_results.iter().all(|r| !r.passed));
    }
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
}

/// Result of verifying a test sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceVerificationResult {
    /// Sequence ID
    pub sequence_id: i64,

    /// Sequence name
    pub name: String,

    /// Step results
    pub step_results: Vec<StepVerificationResult>,

    /// Whether all steps passed
    pub all_steps_passed: bool,
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
            generated_at: Utc::now(),
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
                        StepVerificationResult::Pass { description, .. } => JUnitTestCase {
                            name: format!("{} - {}", name, description),
                            classname,
                            time: 0.0,
                            failure: None,
                            skipped: false,
                        },
                        StepVerificationResult::Fail {
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
                        StepVerificationResultX::NotExecuted { description, .. } => {
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

impl TestVerifier {
    pub fn new(result_strategy: MatchStrategy, output_strategy: MatchStrategy, storage: TestCaseStorage) -> Self {
        Self {
            storage,
            result_strategy,
            output_strategy,
        }
    }

    /// Create a new test verifier with the given storage
    pub fn with_exact_matching() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        Self::new(MatchStrategy::Exact, MatchStrategy::Exact, storage)
    }



    pub fn verify_execution_log(
        &self,
        test_case: &TestCase,
        execution_log: &TestExecutionLog,
    ) -> ExecutionVerificationResult {
        let sequence = test_case
            .test_sequences
            .iter()
            .find(|seq| seq.id == execution_log.sequence_id);

        if sequence.is_none() {
            return ExecutionVerificationResult {
                test_case_id: test_case.id.clone(),
                sequence_id: execution_log.sequence_id,
                overall_passed: false,
                step_results: vec![],
                missing_steps: vec![],
                unexpected_steps: vec![],
            };
        }

        let sequence = sequence.unwrap();

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

    /// Parse test execution log content
    pub fn parse_log_content(
        &self,
        content: &str,
        log_path: &Path,
    ) -> Result<Vec<TestExecutionLog>> {
        let mut logs = Vec::new();

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

                let test_case_id = caps.get(2).unwrap().as_str().trim().to_string();
                let sequence_id = caps
                    .get(3)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .context("Failed to parse sequence ID")?;
                let step_number = caps
                    .get(4)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .context("Failed to parse step number")?;

                let success_str = caps.get(5).unwrap().as_str().to_lowercase();
                let success = match success_str.as_str() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                };

                let actual_result = caps.get(6).unwrap().as_str().trim().to_string();
                let actual_output = caps.get(7).unwrap().as_str().trim().to_string();

                logs.push(TestExecutionLog {
                    test_case_id,
                    sequence_id,
                    step_number,
                    success,
                    actual_result,
                    actual_output,
                    timestamp,
                    log_file_path: log_path.to_path_buf(),
                });
            }
        }

        Ok(logs)
    }

    /// Verify a single test case against execution logs
    pub fn verify_test_case(
        &self,
        test_case: &TestCase,
        execution_logs: &[TestExecutionLog],
    ) -> TestCaseVerificationResult {
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

        for sequence in &test_case.test_sequences {
            let step_results = self.verify_sequence(sequence, &log_map);

            let all_steps_passed = step_results.iter().all(|r| r.is_pass());

            for result in &step_results {
                total_steps += 1;
                match result {
                    StepVerificationResult::Pass { .. } => passed_steps += 1,
                    StepVerificationResult::Fail { .. } => failed_steps += 1,
                    StepVerificationResult::NotExecuted { .. } => not_executed_steps += 1,
                }
            }

            sequences.push(SequenceVerificationResult {
                sequence_id: sequence.id,
                name: sequence.name.clone(),
                step_results,
                all_steps_passed,
            });
        }

        let overall_pass = failed_steps == 0 && not_executed_steps == 0;

        TestCaseVerificationResult {
            test_case_id: test_case.id.clone(),
            description: test_case.description.clone(),
            sequences,
            total_steps,
            passed_steps,
            failed_steps,
            not_executed_steps,
            overall_pass,
        }
    }

    /// Verify a single sequence against execution logs
    fn verify_sequence(
        &self,
        sequence: &TestSequence,
        log_map: &HashMap<(i64, i64), &TestExecutionLog>,
    ) -> Vec<StepVerificationResult> {
        let mut results = Vec::new();

        for step in &sequence.steps {
            let result = if let Some(log) = log_map.get(&(sequence.id, step.step)) {
                self.verify_step(step, log)
            } else {
                StepVerificationResult::NotExecuted {
                    step: step.step,
                    description: step.description.clone(),
                }
            };
            results.push(result);
        }

        results
    }

    pub fn verify_step(&self, step: &Step, actual: &ActualResult, log: &TestExecutionLog) -> StepVerificationResultY {
        let expected = &step.expected;

        // Check success field if it's defined
        if let Some(expected_success) = expected.success {
            if let Some(actual_success) = log.success {
                if expected_success != actual_success {
                    return StepVerificationResult::Fail {
                        step: step.step,
                        description: step.description.clone(),
                        expected: expected.clone(),
                        actual_result: log.actual_result.clone(),
                        actual_output: log.actual_output.clone(),
                        reason: format!(
                            "Success mismatch: expected {}, got {}",
                            expected_success, actual_success
                        ),
                    };
                }
            }
        }

        // Check result
        if !self.matches(&expected.result, &log.actual_result) {
            return StepVerificationResult::Fail {
                step: step.step,
                description: step.description.clone(),
                expected: expected.clone(),
                actual_result: log.actual_result.clone(),
                actual_output: log.actual_output.clone(),
                reason: format!(
                    "Result mismatch: expected '{}', got '{}'",
                    expected.result, log.actual_result
                ),
            };
        }

        // Check output
        if !self.matches(&expected.output, &log.actual_output) {
            return StepVerificationResult::Fail {
                step: step.step,
                description: step.description.clone(),
                expected: expected.clone(),
                actual_result: log.actual_result.clone(),
                actual_output: log.actual_output.clone(),
                reason: format!(
                    "Output mismatch: expected '{}', got '{}'",
                    expected.output, log.actual_output
                ),
            };
        }

        StepVerificationResult::Pass {
            step: step.step,
            description: step.description.clone(),
        }

        ////
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

        StepVerificationResultY {
            step_number: step.step,
            passed,
            result_match,
            output_match,
            success_match,
            diff,
        }
    }

    /// Verify a single step against its execution log
    fn verify_step(&self, step: &Step, log: &TestExecutionLog) -> StepVerificationResult {

    }

    // /// Check if actual value matches expected (supports wildcards and regex)
    // fn matches(&self, expected: &str, actual: &str) -> bool {
    //     // Exact match
    //     if expected == actual {
    //         return true;
    //     }
    //
    //     // Wildcard match (simple * wildcard)
    //     if expected.contains('*') {
    //         let pattern = expected.replace('*', ".*");
    //         if let Ok(regex) = Regex::new(&format!("^{}$", pattern)) {
    //             return regex.is_match(actual);
    //         }
    //     }
    //
    //     // If expected is wrapped in /.../, treat as regex
    //     if expected.starts_with('/') && expected.ends_with('/') && expected.len() > 2 {
    //         let pattern = &expected[1..expected.len() - 1];
    //         if let Ok(regex) = Regex::new(pattern) {
    //             return regex.is_match(actual);
    //         }
    //     }
    //
    //     false
    // }

    fn matches(&self, expected: &str, actual: &str, strategy: MatchStrategy) -> bool {
        match strategy {
            MatchStrategy::Exact => expected == actual,
            MatchStrategy::Contains => actual.contains(expected),
            MatchStrategy::Regex => {
                if let Ok(regex) = Regex::new(expected) {
                    regex.is_match(actual)
                } else {
                    false
                }
            }
        }
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
                    };
                    report.add_test_case_result(failed_result);
                }
            }
        }

        Ok(report)
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use tempfile::TempDir;
    use crate::MatchStrategy::{Exact, Regex};

    #[test]
    fn test_parse_log_content() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::new(Exact, Exact, storage);

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

    #[test]
    fn test_verify_step_pass() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::new(Exact, Exact, storage);

        let step = Step::new(
            1,
            "Test Step".to_string(),
            "cmd".to_string(),
            "SW=0x9000".to_string(),
            "Success".to_string(),
        );

        let log = TestExecutionLog {
            test_case_id: "TC001".to_string(),
            sequence_id: 1,
            step_number: 1,
            success: Some(true),
            actual_result: "SW=0x9000".to_string(),
            actual_output: "Success".to_string(),
            timestamp: None,
            log_file_path: PathBuf::from("test.log"),
        };

        let result = verifier.verify_step(&step, &log);
        assert!(result.is_pass());
    }

    #[test]
    fn test_verify_step_fail() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::new(Exact, Exact, storage);

        let step = Step::new(
            1,
            "Test Step".to_string(),
            "cmd".to_string(),
            "SW=0x9000".to_string(),
            "Success".to_string(),
        );

        let log = TestExecutionLog {
            test_case_id: "TC001".to_string(),
            sequence_id: 1,
            step_number: 1,
            success: Some(false),
            actual_result: "SW=0x6A82".to_string(),
            actual_output: "Error".to_string(),
            timestamp: None,
            log_file_path: PathBuf::from("test.log"),
        };

        let result = verifier.verify_step(&step, &log);
        assert!(!result.is_pass());
    }

    #[test]
    fn test_wildcard_matching() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::new(Exact, Exact, storage);

        assert!(verifier.matches("SW=0x9000", "SW=0x9000", Exact));
        assert!(verifier.matches("SW=*", "SW=0x9000", Exact));
        assert!(verifier.matches("*9000", "SW=0x9000", Exact));
        assert!(!verifier.matches("SW=0x9001", "SW=0x9000", Exact));
    }

    #[test]
    fn test_regex_matching() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
        let verifier = TestVerifier::new(Exact, Exact, storage);

        assert!(verifier.matches("/SW=0x[0-9A-F]{4}/", "SW=0x9000", Regex));
        assert!(verifier.matches("/^Success$/", "Success", Regex));
        assert!(!verifier.matches("/^Failed$/", "Success", Regex));
    }

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
            timestamp: Utc::now(),
        };

        let xml = suite.to_xml().unwrap();
        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("tests=\"2\""));
        assert!(xml.contains("failures=\"1\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("<failure"));
    }
}
