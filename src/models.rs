use serde::{Deserialize, Serialize};

/// Expected outcome for a test step
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expected {
    /// Whether the step should succeed (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// Expected result value
    pub result: String,

    /// Expected output
    pub output: String,
}

/// Represents a single step in a test sequence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
}

/// Initial condition for eUICC
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitialCondition {
    #[serde(rename = "eUICC")]
    pub euicc: Vec<String>,
}

/// A sequence of test steps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestSequence {
    /// Sequence identifier
    pub id: i64,

    /// Name of the test sequence
    pub name: String,

    /// Description of the test sequence
    pub description: String,

    /// Initial conditions specific to this sequence
    pub initial_conditions: Vec<InitialCondition>,

    /// List of steps in the sequence
    pub steps: Vec<Step>,
}

/// General initial condition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralInitialCondition {
    #[serde(rename = "eUICC")]
    pub euicc: Vec<String>,
}

/// Top-level initial conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopLevelInitialConditions {
    #[serde(rename = "eUICC")]
    pub euicc: Vec<String>,
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
    pub general_initial_conditions: Vec<GeneralInitialCondition>,

    /// Initial conditions
    pub initial_conditions: TopLevelInitialConditions,

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
        Self {
            requirement,
            item,
            tc,
            id,
            description,
            general_initial_conditions: Vec::new(),
            initial_conditions: TopLevelInitialConditions { euicc: Vec::new() },
            test_sequences: Vec::new(),
        }
    }
}

impl TestSequence {
    /// Create a new test sequence
    pub fn new(id: i64, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            initial_conditions: Vec::new(),
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
    fn test_initial_condition_creation() {
        let condition = InitialCondition {
            euicc: vec!["Condition 1".to_string(), "Condition 2".to_string()],
        };
        assert_eq!(condition.euicc.len(), 2);
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
    fn test_general_initial_condition_creation() {
        let condition = GeneralInitialCondition {
            euicc: vec!["General Condition".to_string()],
        };
        assert_eq!(condition.euicc.len(), 1);
    }

    #[test]
    fn test_top_level_initial_conditions_creation() {
        let conditions = TopLevelInitialConditions {
            euicc: vec!["Condition 1".to_string(), "Condition 2".to_string()],
        };
        assert_eq!(conditions.euicc.len(), 2);
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
        assert_eq!(test_case.initial_conditions.euicc.len(), 0);
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
