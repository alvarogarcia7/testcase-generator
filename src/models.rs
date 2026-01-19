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

/// Status of a test run
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestRunStatus {
    /// Test passed successfully
    Pass,
    /// Test failed
    Fail,
    /// Test was skipped
    Skip,
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

/// Represents a single test run execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestRun {
    /// Identifier of the test case that was run
    pub test_case_id: String,

    /// Timestamp when the test was executed
    pub timestamp: DateTime<Utc>,

    /// Status of the test run
    pub status: TestRunStatus,

    /// Duration of the test run in milliseconds
    pub duration: u64,

    /// Execution log capturing output and events
    pub execution_log: String,

    /// Optional error message if the test failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
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
}
