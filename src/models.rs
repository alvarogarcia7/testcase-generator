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
