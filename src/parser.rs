use crate::models::{TestCase, TestSequence, TestSuite};

pub struct TestCaseParser;

impl TestCaseParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TestCaseParser {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SearchableCollections {
    pub test_cases: Vec<TestCase>,
    pub test_sequences: Vec<TestSequence>,
    pub test_suites: Vec<TestSuite>,
}

impl SearchableCollections {
    pub fn new() -> Self {
        Self {
            test_cases: Vec::new(),
            test_sequences: Vec::new(),
            test_suites: Vec::new(),
        }
    }
}

impl Default for SearchableCollections {
    fn default() -> Self {
        Self::new()
    }
}
