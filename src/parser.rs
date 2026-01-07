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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = TestCaseParser::new();
        let _ = parser;
    }

    #[test]
    fn test_parser_default() {
        let _parser = TestCaseParser;
    }

    #[test]
    fn test_searchable_collections_creation() {
        let collections = SearchableCollections::new();
        assert_eq!(collections.test_cases.len(), 0);
        assert_eq!(collections.test_sequences.len(), 0);
        assert_eq!(collections.test_suites.len(), 0);
    }

    #[test]
    fn test_searchable_collections_default() {
        let collections = SearchableCollections::default();
        assert_eq!(collections.test_cases.len(), 0);
        assert_eq!(collections.test_sequences.len(), 0);
        assert_eq!(collections.test_suites.len(), 0);
    }

    #[test]
    fn test_searchable_collections_add_test_case() {
        let mut collections = SearchableCollections::new();
        let test_case = TestCase::new(
            "REQ001".to_string(),
            1,
            1,
            "TC001".to_string(),
            "Test Case".to_string(),
        );
        collections.test_cases.push(test_case);
        assert_eq!(collections.test_cases.len(), 1);
    }

    #[test]
    fn test_searchable_collections_add_test_sequence() {
        let mut collections = SearchableCollections::new();
        let sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());
        collections.test_sequences.push(sequence);
        assert_eq!(collections.test_sequences.len(), 1);
    }

    #[test]
    fn test_searchable_collections_add_test_suite() {
        let mut collections = SearchableCollections::new();
        let suite = TestSuite::new("Suite 1".to_string());
        collections.test_suites.push(suite);
        assert_eq!(collections.test_suites.len(), 1);
    }
}
