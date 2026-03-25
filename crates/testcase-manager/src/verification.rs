use crate::storage::TestCaseStorage;
use anyhow::Result;
use std::path::Path;
use testcase_verification::{
    BatchVerificationReport, ContainerReportConfig, MatchStrategy, TestVerifier,
};

/// Storage-aware TestVerifier that wraps the core TestVerifier from testcase-verification
/// and provides storage integration for testcase-manager
pub struct StorageTestVerifier {
    storage: TestCaseStorage,
    verifier: TestVerifier,
}

impl StorageTestVerifier {
    /// Create a new verifier with storage and default match strategies
    pub fn from_storage(storage: TestCaseStorage) -> Self {
        Self {
            storage,
            verifier: TestVerifier::with_exact_matching(),
        }
    }

    /// Create a new verifier with storage and custom match strategies
    pub fn with_strategies(
        storage: TestCaseStorage,
        result_strategy: MatchStrategy,
        output_strategy: MatchStrategy,
    ) -> Self {
        Self {
            storage,
            verifier: TestVerifier::new(result_strategy, output_strategy),
        }
    }

    /// Get a reference to the test case storage
    pub fn storage(&self) -> &TestCaseStorage {
        &self.storage
    }

    /// Get a reference to the inner verifier
    pub fn verifier(&self) -> &TestVerifier {
        &self.verifier
    }

    /// Process multiple log files and verify against test cases from storage
    pub fn batch_verify<P: AsRef<Path>>(&self, log_paths: &[P]) -> Result<BatchVerificationReport> {
        use std::collections::HashMap;
        use testcase_verification::TestExecutionLog;

        let mut all_logs: HashMap<String, Vec<TestExecutionLog>> = HashMap::new();

        // Parse all log files
        for log_path in log_paths {
            let logs = self.verifier.parse_log_file(log_path)?;
            for log in logs {
                all_logs
                    .entry(log.test_case_id.clone())
                    .or_default()
                    .push(log);
            }
        }

        // Load all test cases from storage
        let test_case_ids: Vec<String> = all_logs.keys().cloned().collect();
        let mut test_cases = Vec::new();

        for test_case_id in test_case_ids {
            match self.storage.load_test_case_by_id(&test_case_id) {
                Ok(test_case) => {
                    test_cases.push(test_case);
                }
                Err(e) => {
                    log::warn!(
                        "Failed to load test case '{}' from storage: {}. Skipping.",
                        test_case_id,
                        e
                    );
                }
            }
        }

        // Use the core verifier with loaded test cases
        self.verifier.batch_verify(log_paths, &test_cases)
    }

    /// Generate a container report from multiple batch reports
    pub fn generate_report(
        &self,
        reports: &[BatchVerificationReport],
        format: &str,
        config: ContainerReportConfig,
    ) -> Result<String> {
        self.verifier.generate_report(reports, format, config)
    }

    /// Parse a test execution log file
    pub fn parse_log_file<P: AsRef<Path>>(
        &self,
        log_path: P,
    ) -> Result<Vec<testcase_verification::TestExecutionLog>> {
        self.verifier.parse_log_file(log_path)
    }

    /// Parse a test execution log file with a specified test case ID
    pub fn parse_log_file_with_test_case_id<P: AsRef<Path>>(
        &self,
        log_path: P,
        test_case_id: &str,
    ) -> Result<Vec<testcase_verification::TestExecutionLog>> {
        self.verifier
            .parse_log_file_with_test_case_id(log_path, test_case_id)
    }

    /// Verify a single test case against execution logs
    pub fn verify_test_case(
        &self,
        test_case: &testcase_models::TestCase,
        execution_logs: &[testcase_verification::TestExecutionLog],
    ) -> testcase_verification::TestCaseVerificationResult {
        self.verifier.verify_test_case(test_case, execution_logs)
    }
}
