use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Verification information for a test step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Verification {
    /// Verification result value
    pub result: String,

    /// Verification output
    pub output: String,
}

impl fmt::Display for Verification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "result: {} | output: {}", self.result, self.output)
    }
}

/// Expected outcome for a test step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Expected {
    /// Whether the step should succeed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// Expected result value
    pub result: String,

    /// Expected output
    pub output: String,
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let success_str = match self.success {
            Some(true) => "true",
            Some(false) => "false",
            None => "None",
        };
        write!(
            f,
            "success: {} | result: {} | output: {}",
            success_str, self.result, self.output
        )
    }
}

/// Represents a single step in a test sequence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Step {
    /// Step number
    pub step: i64,

    /// Whether this is a manual step (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,

    /// Description of the step
    pub description: String,

    /// Command to execute
    pub command: String,

    /// Expected outcome
    pub expected: Expected,

    /// Verification information
    #[serde(default = "default_verification_from_expected")]
    pub verification: Verification,
}

/// Default verification based on expected values (for backward compatibility)
fn default_verification_from_expected() -> Verification {
    Verification {
        result: "[[ $? -eq 0 ]]".to_string(),
        output: "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"".to_string(),
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}: {} ({})", self.step, self.description, self.command)
    }
}

/// A sequence of test steps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestSequence {
    /// Sequence identifier
    pub id: i64,

    /// Name of the test sequence
    pub name: String,

    /// Description of the test sequence
    pub description: String,

    /// Initial conditions specific to this sequence
    pub initial_conditions: HashMap<String, Vec<String>>,

    /// List of steps in the sequence
    pub steps: Vec<Step>,
}

impl fmt::Display for TestSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}: {} - {}", self.id, self.name, self.description)
    }
}

/// A complete test case following the GSMA schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestCase {
    /// Requirement identifier
    pub requirement: String,

    /// Item number
    pub item: i64,

    /// TC number
    pub tc: i64,

    /// Unique identifier for the test case
    pub id: String,

    /// Description of the test case
    pub description: String,

    /// General initial conditions
    pub general_initial_conditions: HashMap<String, Vec<String>>,

    /// Initial conditions
    pub initial_conditions: HashMap<String, Vec<String>>,

    /// Test sequences
    pub test_sequences: Vec<TestSequence>,
}

/// Collection of test cases
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestSuite {
    /// Name of the test suite
    pub name: String,

    /// Description of the test suite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Version of the test suite
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// List of test cases
    pub test_cases: Vec<TestCase>,
}

impl TestCase {
    /// Create a new test case with required fields
    pub fn new(requirement: String, item: i64, tc: i64, id: String, description: String) -> Self {
        let general_initial_conditions: HashMap<String, Vec<String>> = HashMap::new();
        Self {
            requirement,
            item,
            tc,
            id,
            description,
            general_initial_conditions,
            initial_conditions: HashMap::new(),
            test_sequences: Vec::new(),
        }
    }
}

impl TestSequence {
    /// Create a new test sequence
    pub fn new(id: i64, name: String, description: String) -> Self {
        let initial_conditions: HashMap<String, Vec<String>> = HashMap::new();
        Self {
            id,
            name,
            description,
            initial_conditions,
            steps: Vec::new(),
        }
    }
}

impl Step {
    /// Create a new step
    pub fn new(
        step: i64,
        description: String,
        command: String,
        result: String,
        output: String,
    ) -> Self {
        Self {
            step,
            manual: None,
            description,
            command,
            expected: Expected {
                success: None,
                result,
                output,
            },
            verification: default_verification_from_expected(),
        }
    }
}

impl TestSuite {
    /// Create a new test suite
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            version: None,
            test_cases: Vec::new(),
        }
    }
}

/// Detailed validation error information
#[derive(Debug, Clone)]
pub struct ValidationErrorDetail {
    /// JSON path where the error occurred (e.g., "/test_sequences/0/steps/1/expected")
    pub path: String,

    /// The specific constraint that failed
    pub constraint: String,

    /// The actual value that was found
    pub found_value: String,

    /// The expected value or constraint
    pub expected_constraint: String,
}

/// Result of validating a file
#[derive(Debug, Clone)]
pub enum FileValidationStatus {
    /// File is valid
    Valid,

    /// File failed to parse as YAML
    ParseError { message: String },

    /// File failed schema validation
    ValidationError { errors: Vec<ValidationErrorDetail> },
}

/// Information about a test case file and its validation status
#[derive(Debug, Clone)]
pub struct TestCaseFileInfo {
    /// Path to the file
    pub path: PathBuf,

    /// Validation status
    pub status: FileValidationStatus,

    /// Test case data if successfully loaded
    pub test_case: Option<TestCase>,
}

/// Test execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestRunStatus {
    /// Test passed
    Pass,
    /// Test failed
    Fail,
    /// Test was skipped
    Skip,
}

impl fmt::Display for TestRunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestRunStatus::Pass => write!(f, "Pass"),
            TestRunStatus::Fail => write!(f, "Fail"),
            TestRunStatus::Skip => write!(f, "Skip"),
        }
    }
}

/// Test run execution result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestRun {
    /// Test case ID
    pub test_case_id: String,

    /// Test execution status
    pub status: TestRunStatus,

    /// Timestamp when the test was executed
    pub timestamp: DateTime<Utc>,

    /// Duration in seconds
    pub duration: f64,

    /// Execution log capturing output and events
    pub execution_log: String,
    /// Optional error message (for failed or skipped tests)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    /// Optional test name/description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl TestRun {
    /// Create a new test run
    pub fn new(
        test_case_id: String,
        status: TestRunStatus,
        timestamp: DateTime<Utc>,
        duration: f64,
        execution_log: String,
    ) -> Self {
        Self {
            test_case_id,
            status,
            timestamp,
            duration,
            error_message: None,
            name: None,
            execution_log,
        }
    }

    /// Create a test run with error message
    pub fn with_error(
        test_case_id: String,
        status: TestRunStatus,
        timestamp: DateTime<Utc>,
        duration: f64,
        error_message: String,
    ) -> Self {
        Self {
            test_case_id,
            status,
            timestamp,
            duration,
            error_message: Some(error_message),
            name: None,
            execution_log: "".to_string(),
        }
    }

    /// Serialize to JUnit XML format
    pub fn to_junit_xml(&self) -> String {
        use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
        use quick_xml::Writer;
        use std::io::Cursor;

        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

        writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
            .unwrap();

        let mut testsuite = BytesStart::new("testsuite");
        testsuite.push_attribute(("name", "TestRun"));
        testsuite.push_attribute(("tests", "1"));
        testsuite.push_attribute(("time", format!("{0:.3}", self.duration).as_str()));

        let (failures, skipped) = match self.status {
            TestRunStatus::Fail => ("1", "0"),
            TestRunStatus::Skip => ("0", "1"),
            TestRunStatus::Pass => ("0", "0"),
        };
        testsuite.push_attribute(("failures", failures));
        testsuite.push_attribute(("skipped", skipped));

        writer.write_event(Event::Start(testsuite)).unwrap();

        let mut testcase = BytesStart::new("testcase");
        testcase.push_attribute(("id", self.test_case_id.as_str()));

        let name = self.name.as_deref().unwrap_or(&self.test_case_id);
        testcase.push_attribute(("name", name));
        testcase.push_attribute(("time", format!("{0:.3}", self.duration).as_str()));
        testcase.push_attribute(("timestamp", self.timestamp.to_rfc3339().as_str()));

        match self.status {
            TestRunStatus::Pass => {
                writer.write_event(Event::Empty(testcase)).unwrap();
            }
            TestRunStatus::Fail => {
                writer.write_event(Event::Start(testcase)).unwrap();

                let mut failure = BytesStart::new("failure");
                failure.push_attribute(("message", "Test failed"));

                if let Some(ref error_msg) = self.error_message {
                    writer.write_event(Event::Start(failure)).unwrap();
                    writer
                        .write_event(Event::Text(BytesText::new(error_msg)))
                        .unwrap();
                    writer
                        .write_event(Event::End(BytesEnd::new("failure")))
                        .unwrap();
                } else {
                    writer.write_event(Event::Empty(failure)).unwrap();
                }

                writer
                    .write_event(Event::End(BytesEnd::new("testcase")))
                    .unwrap();
            }
            TestRunStatus::Skip => {
                writer.write_event(Event::Start(testcase)).unwrap();

                let mut skipped = BytesStart::new("skipped");

                if let Some(ref error_msg) = self.error_message {
                    skipped.push_attribute(("message", error_msg.as_str()));
                    writer.write_event(Event::Empty(skipped)).unwrap();
                } else {
                    skipped.push_attribute(("message", "Test skipped"));
                    writer.write_event(Event::Empty(skipped)).unwrap();
                }

                writer
                    .write_event(Event::End(BytesEnd::new("testcase")))
                    .unwrap();
            }
        }

        writer
            .write_event(Event::End(BytesEnd::new("testsuite")))
            .unwrap();

        let result = writer.into_inner().into_inner();
        String::from_utf8(result).unwrap()
    }
}

/// Metadata for a test run
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestRunMetadata {
    /// Identifier of the test case that was run
    pub test_case_id: String,

    /// Timestamp when the test was executed
    pub timestamp: DateTime<Utc>,

    /// Duration of the test run in milliseconds
    pub duration: u64,
}

/// Actual result of a test step execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActualResult {
    /// Actual result value
    pub result: String,

    /// Actual output
    pub output: String,

    /// Whether the step succeeded
    pub success: bool,
}

/// Result of executing a single test step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepExecutionResult {
    /// Step number
    pub step_number: i64,

    /// Timestamp when the step was executed
    pub timestamp: String,

    /// Actual result of the step
    pub actual_result: ActualResult,

    /// Duration of the step execution in milliseconds
    pub duration_ms: u64,

    /// Optional error message if the step failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Execution log for a test case
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestExecutionLog {
    /// Test case identifier
    pub test_case_id: String,

    /// Sequence identifier
    pub sequence_id: i64,

    /// Timestamp when the execution started
    pub timestamp: String,

    /// Actual output of the execution
    pub actual_output: String,

    /// Whether the overall execution was successful
    pub actual_success: bool,

    /// Total duration of the execution in milliseconds
    pub duration_ms: u64,

    /// Optional error message if the execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Field-level difference between expected and actual values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDiff {
    /// Name of the field
    pub field_name: String,

    /// Expected value
    pub expected: String,

    /// Actual value
    pub actual: String,

    /// Whether the field matched
    pub matched: bool,
}

impl fmt::Display for FieldDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.matched {
            "\x1b[32m✓\x1b[0m" // Green checkmark
        } else {
            "\x1b[31m✗\x1b[0m" // Red X
        };
        write!(
            f,
            "    {} {}: expected '{}', actual '{}'",
            status, self.field_name, self.expected, self.actual
        )
    }
}

/// Overall status of verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationStatus {
    /// All steps passed
    Passed,

    /// Some steps failed
    Failed,

    /// Some steps were skipped
    PartiallySkipped,

    /// All steps were skipped
    AllSkipped,
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerificationStatus::Passed => write!(f, "\x1b[32mPASSED\x1b[0m"),
            VerificationStatus::Failed => write!(f, "\x1b[31mFAILED\x1b[0m"),
            VerificationStatus::PartiallySkipped => write!(f, "\x1b[33mPARTIALLY SKIPPED\x1b[0m"),
            VerificationStatus::AllSkipped => write!(f, "\x1b[33mALL SKIPPED\x1b[0m"),
        }
    }
}

/// Result of verifying a single step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepVerificationResult {
    /// Step number
    pub step_number: i64,

    /// Whether the step passed verification
    pub passed: bool,

    /// Whether the step was skipped
    pub skipped: bool,

    /// Field-level differences between expected and actual
    pub field_diffs: Vec<FieldDiff>,

    /// Optional error or failure message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl fmt::Display for StepVerificationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.skipped {
            "\x1b[33mSKIPPED\x1b[0m"
        } else if self.passed {
            "\x1b[32mPASS\x1b[0m"
        } else {
            "\x1b[31mFAIL\x1b[0m"
        };

        writeln!(f, "  Step {}: {}", self.step_number, status)?;

        if let Some(ref msg) = self.message {
            writeln!(f, "    Message: {}", msg)?;
        }

        if !self.field_diffs.is_empty() {
            writeln!(f, "    Field Differences:")?;
            for diff in &self.field_diffs {
                writeln!(f, "{}", diff)?;
            }
        }

        Ok(())
    }
}

/// Complete verification report for a test execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationReport {
    /// Total number of steps
    pub total_steps: usize,

    /// Number of steps that passed
    pub passed_steps: usize,

    /// Number of steps that failed
    pub failed_steps: usize,

    /// Number of steps that were skipped
    pub skipped_steps: usize,

    /// Detailed results for each step
    pub step_results: Vec<StepVerificationResult>,

    /// Overall verification status
    pub overall_status: VerificationStatus,
}

impl fmt::Display for VerificationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Verification Report")?;
        writeln!(f, "===================")?;
        writeln!(f, "Overall Status: {}", self.overall_status)?;
        writeln!(f)?;
        writeln!(
            f,
            "Summary: {} total, {} passed, {} failed, {} skipped",
            self.total_steps, self.passed_steps, self.failed_steps, self.skipped_steps
        )?;
        writeln!(f)?;

        if !self.step_results.is_empty() {
            writeln!(f, "Step Results:")?;
            for result in &self.step_results {
                write!(f, "{}", result)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_creation() {
        let expected = Expected {
            success: Some(true),
            result: "SW=0x9000".to_string(),
            output: "Success".to_string(),
        };
        assert_eq!(expected.success, Some(true));
        assert_eq!(expected.result, "SW=0x9000");
        assert_eq!(expected.output, "Success");
    }

    #[test]
    fn test_expected_without_success() {
        let expected = Expected {
            success: None,
            result: "SW=0x9000".to_string(),
            output: "Success".to_string(),
        };
        assert_eq!(expected.success, None);
    }

    #[test]
    fn test_step_creation() {
        let step = Step::new(
            1,
            "Test step".to_string(),
            "ssh".to_string(),
            "SW=0x9000".to_string(),
            "Success".to_string(),
        );
        assert_eq!(step.step, 1);
        assert_eq!(step.description, "Test step");
        assert_eq!(step.command, "ssh");
        assert_eq!(step.manual, None);
        assert_eq!(step.expected.result, "SW=0x9000");
    }

    #[test]
    fn test_step_with_manual() {
        let mut step = Step::new(
            1,
            "Manual step".to_string(),
            "ssh".to_string(),
            "result".to_string(),
            "output".to_string(),
        );
        step.manual = Some(true);
        assert_eq!(step.manual, Some(true));
    }

    #[test]
    fn test_test_sequence_creation() {
        let sequence = TestSequence::new(1, "Test Sequence".to_string(), "Description".to_string());
        assert_eq!(sequence.id, 1);
        assert_eq!(sequence.name, "Test Sequence");
        assert_eq!(sequence.description, "Description");
        assert_eq!(sequence.initial_conditions.len(), 0);
        assert_eq!(sequence.steps.len(), 0);
    }

    #[test]
    fn test_test_sequence_with_steps() {
        let mut sequence =
            TestSequence::new(1, "Test Sequence".to_string(), "Description".to_string());
        let step = Step::new(
            1,
            "Step 1".to_string(),
            "ssh".to_string(),
            "result".to_string(),
            "output".to_string(),
        );
        sequence.steps.push(step);
        assert_eq!(sequence.steps.len(), 1);
    }

    #[test]
    fn test_test_case_creation() {
        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            2,
            "TC001".to_string(),
            "Test description".to_string(),
        );
        assert_eq!(test_case.requirement, "REQ001");
        assert_eq!(test_case.item, 1);
        assert_eq!(test_case.tc, 2);
        assert_eq!(test_case.id, "TC001");
        assert_eq!(test_case.description, "Test description");
        assert_eq!(test_case.general_initial_conditions.len(), 0);
        assert!(test_case.initial_conditions.is_empty());
        assert_eq!(test_case.test_sequences.len(), 0);
    }

    #[test]
    fn test_test_case_with_sequences() {
        let mut test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            2,
            "TC001".to_string(),
            "Test description".to_string(),
        );
        let sequence = TestSequence::new(1, "Seq 1".to_string(), "Desc".to_string());
        test_case.test_sequences.push(sequence);
        assert_eq!(test_case.test_sequences.len(), 1);
    }

    #[test]
    fn test_test_suite_creation() {
        let suite = TestSuite::new("Test Suite".to_string());
        assert_eq!(suite.name, "Test Suite");
        assert_eq!(suite.description, None);
        assert_eq!(suite.version, None);
        assert_eq!(suite.test_cases.len(), 0);
    }

    #[test]
    fn test_test_suite_with_description() {
        let mut suite = TestSuite::new("Test Suite".to_string());
        suite.description = Some("Suite description".to_string());
        suite.version = Some("1.0.0".to_string());
        assert_eq!(suite.description, Some("Suite description".to_string()));
        assert_eq!(suite.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_test_suite_with_test_cases() {
        let mut suite = TestSuite::new("Test Suite".to_string());
        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            2,
            "TC001".to_string(),
            "Test".to_string(),
        );
        suite.test_cases.push(test_case);
        assert_eq!(suite.test_cases.len(), 1);
    }

    #[test]
    fn test_serialization_and_deserialization() {
        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            2,
            "TC001".to_string(),
            "Test description".to_string(),
        );

        let yaml = serde_yaml::to_string(&test_case).unwrap();
        let deserialized: TestCase = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(test_case, deserialized);
    }

    #[test]
    fn test_step_serialization_with_optional_fields() {
        let mut step = Step::new(
            1,
            "Step".to_string(),
            "ssh".to_string(),
            "result".to_string(),
            "output".to_string(),
        );
        step.manual = Some(true);
        step.expected.success = Some(false);

        let yaml = serde_yaml::to_string(&step).unwrap();
        assert!(yaml.contains("manual: true"));
        assert!(yaml.contains("success: false"));
    }

    #[test]
    fn test_step_serialization_without_optional_fields() {
        let step = Step::new(
            1,
            "Step".to_string(),
            "ssh".to_string(),
            "result".to_string(),
            "output".to_string(),
        );

        let yaml = serde_yaml::to_string(&step).unwrap();
        assert!(!yaml.contains("manual:"));
        assert!(!yaml.contains("success:"));
    }

    #[test]
    fn test_test_run_creation() {
        let timestamp = Utc::now();
        let test_run = TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.500,
            "".to_string(),
        );

        assert_eq!(test_run.test_case_id, "TC001");
        assert_eq!(test_run.status, TestRunStatus::Pass);
        assert_eq!(test_run.duration, 1.500);
        assert_eq!(test_run.error_message, None);
    }

    #[test]
    fn test_test_run_with_error() {
        let timestamp = Utc::now();
        let test_run = TestRun::with_error(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.500,
            "Test failed due to assertion error".to_string(),
        );

        assert_eq!(test_run.test_case_id, "TC002");
        assert_eq!(test_run.status, TestRunStatus::Fail);
        assert_eq!(test_run.duration, 2.500);
        assert_eq!(
            test_run.error_message,
            Some("Test failed due to assertion error".to_string())
        );
    }

    #[test]
    fn test_junit_xml_pass() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let test_run = TestRun::new(
            "TC001".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.234,
            "".to_string(),
        );

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("name=\"TestRun\""));
        assert!(xml.contains("tests=\"1\""));
        assert!(xml.contains("failures=\"0\""));
        assert!(xml.contains("skipped=\"0\""));
        assert!(xml.contains("time=\"1.234\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("id=\"TC001\""));
        assert!(xml.contains("name=\"TC001\""));
        assert!(xml.contains("timestamp=\"2024-01-15T10:30:00+00:00\""));
        assert!(!xml.contains("<failure"));
        assert!(!xml.contains("<skipped"));
    }

    #[test]
    fn test_junit_xml_fail_with_error_message() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let test_run = TestRun::with_error(
            "TC002".to_string(),
            TestRunStatus::Fail,
            timestamp,
            2.500,
            "Assertion failed: expected true, got false".to_string(),
        );

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("failures=\"1\""));
        assert!(xml.contains("skipped=\"0\""));
        assert!(xml.contains("time=\"2.500\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("id=\"TC002\""));
        assert!(xml.contains("<failure"));
        assert!(xml.contains("message=\"Test failed\""));
        assert!(xml.contains("Assertion failed: expected true, got false"));
        assert!(xml.contains("</failure>"));
        assert!(xml.contains("</testcase>"));
    }

    #[test]
    fn test_junit_xml_fail_without_error_message() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let test_run = TestRun::new(
            "TC003".to_string(),
            TestRunStatus::Fail,
            timestamp,
            0.500,
            "".to_string(),
        );

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("failures=\"1\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("id=\"TC003\""));
        assert!(xml.contains("<failure"));
        assert!(xml.contains("message=\"Test failed\""));
        assert!(!xml.contains("</failure>"));
    }

    #[test]
    fn test_junit_xml_skip_with_message() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let test_run = TestRun::with_error(
            "TC004".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.,
            "Test skipped due to missing dependencies".to_string(),
        );

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("failures=\"0\""));
        assert!(xml.contains("skipped=\"1\""));
        assert!(xml.contains("time=\"0.000\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("id=\"TC004\""));
        assert!(xml.contains("<skipped"));
        assert!(xml.contains("message=\"Test skipped due to missing dependencies\""));
        assert!(xml.contains("</testcase>"));
    }

    #[test]
    fn test_junit_xml_skip_without_message() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let test_run = TestRun::new(
            "TC005".to_string(),
            TestRunStatus::Skip,
            timestamp,
            0.0,
            "".to_string(),
        );

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("skipped=\"1\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("id=\"TC005\""));
        assert!(xml.contains("<skipped"));
        assert!(xml.contains("message=\"Test skipped\""));
    }

    #[test]
    fn test_junit_xml_zero_duration() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let test_run = TestRun::new(
            "TC006".to_string(),
            TestRunStatus::Pass,
            timestamp,
            0.0,
            "".to_string(),
        );

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("time=\"0.000\""));
        assert!(xml.contains("<testcase"));
        assert!(xml.contains("id=\"TC006\""));
    }

    #[test]
    fn test_junit_xml_with_name() {
        let timestamp = "2024-01-15T10:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let mut test_run = TestRun::new(
            "TC007".to_string(),
            TestRunStatus::Pass,
            timestamp,
            1.000,
            "".to_string(),
        );
        test_run.name = Some("User Authentication Test".to_string());

        let xml = test_run.to_junit_xml();

        assert!(xml.contains("id=\"TC007\""));
        assert!(xml.contains("name=\"User Authentication Test\""));
    }

    #[test]
    fn test_test_run_status_display() {
        assert_eq!(format!("{}", TestRunStatus::Pass), "Pass");
        assert_eq!(format!("{}", TestRunStatus::Fail), "Fail");
        assert_eq!(format!("{}", TestRunStatus::Skip), "Skip");
    }
}
