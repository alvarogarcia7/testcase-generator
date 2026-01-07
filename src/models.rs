use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a single step in a test sequence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Step {
    /// Unique identifier for the step
    pub id: String,

    /// Human-readable description of the step
    pub description: String,

    /// Action to perform (e.g., "click", "type", "verify")
    pub action: String,

    /// Target element or location for the action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// Value or input data for the action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Expected result or outcome
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,

    /// Additional metadata or parameters
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_yaml::Value>,
}

/// Priority level for test cases
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of a test case
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Active,
    Deprecated,
    Archived,
}

/// Type of test
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TestType {
    Functional,
    Integration,
    Regression,
    Smoke,
    Performance,
    Security,
    #[serde(rename = "user-acceptance")]
    UserAcceptance,
    Other(String),
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Environment {
    /// Environment name (e.g., "development", "staging", "production")
    pub name: String,

    /// Base URL or endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Environment-specific variables
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, String>,
}

/// Precondition for test execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Precondition {
    /// Description of the precondition
    pub description: String,

    /// Steps to set up the precondition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub setup_steps: Vec<String>,
}

/// Post-execution cleanup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cleanup {
    /// Description of the cleanup
    pub description: String,

    /// Steps to perform cleanup
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cleanup_steps: Vec<String>,
}

/// A sequence of test steps
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestSequence {
    /// Unique identifier for the sequence
    pub id: String,

    /// Name of the test sequence
    pub name: String,

    /// Detailed description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// List of steps in the sequence
    pub steps: Vec<Step>,

    /// Tags for categorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// Additional metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_yaml::Value>,
}

/// A complete test case
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestCase {
    /// Unique identifier for the test case
    pub id: String,

    /// Title of the test case
    pub title: String,

    /// Detailed description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Priority level
    pub priority: Priority,

    /// Current status
    pub status: Status,

    /// Type of test
    #[serde(rename = "type")]
    pub test_type: TestType,

    /// Tags for categorization and filtering
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// Author or creator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Last update timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Test sequences
    pub sequences: Vec<TestSequence>,

    /// Preconditions for the test
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions: Vec<Precondition>,

    /// Cleanup steps after the test
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cleanup: Vec<Cleanup>,

    /// Environment configurations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environments: Vec<Environment>,

    /// Related test case IDs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_tests: Vec<String>,

    /// Additional metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_yaml::Value>,
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

    /// Additional metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_yaml::Value>,
}

impl TestCase {
    /// Create a new test case with default values
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
            priority: Priority::Medium,
            status: Status::Draft,
            test_type: TestType::Functional,
            tags: Vec::new(),
            author: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
            sequences: Vec::new(),
            preconditions: Vec::new(),
            cleanup: Vec::new(),
            environments: Vec::new(),
            related_tests: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = Some(chrono::Utc::now());
    }
}

impl TestSequence {
    /// Create a new test sequence
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            steps: Vec::new(),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

impl Step {
    /// Create a new step
    pub fn new(id: String, description: String, action: String) -> Self {
        Self {
            id,
            description,
            action,
            target: None,
            value: None,
            expected: None,
            metadata: HashMap::new(),
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
            metadata: HashMap::new(),
        }
    }
}
