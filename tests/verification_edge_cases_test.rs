use std::fs;
use testcase_manager::models::TestCase;
use testcase_manager::storage::TestCaseStorage;
use testcase_manager::verification::{MatchStrategy, TestVerifier};

/// Helper function to load a test case YAML file
fn load_test_case(yaml_path: &str) -> TestCase {
    let content = fs::read_to_string(yaml_path)
        .unwrap_or_else(|e| panic!("Failed to read test case file {}: {}", yaml_path, e));
    serde_yaml::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse test case YAML {}: {}", yaml_path, e))
}

/// Helper function to get base path for edge case test files
fn get_edge_case_path(test_id: &str) -> (String, String) {
    let base_path = "testcases/verifier_scenarios/edge_cases";
    let yaml_path = format!("{}/{}.yml", base_path, test_id);
    let json_path = format!("{}/{}_execution_log.json", base_path, test_id);
    (yaml_path, json_path)
}

// ============================================================================
// Edge Case 1: TEST_EDGE_ALL_FAIL_001
// All steps executed but all failed verification
// ============================================================================

#[test]
fn test_edge_all_fail_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_ALL_FAIL_001");
    let test_case = load_test_case(&yaml_path);

    // Create verifier with exact matching strategy
    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    // Parse execution log
    let logs = verifier.parse_log_file(&json_path).unwrap();

    // Verify test case
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 0, failed: 3, not_executed: 0
    assert_eq!(result.test_case_id, "TEST_EDGE_ALL_FAIL_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 0, "Should have 0 passed steps");
    assert_eq!(result.failed_steps, 3, "Should have 3 failed steps");
    assert_eq!(
        result.not_executed_steps, 0,
        "Should have 0 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should not pass"
    );
    assert_eq!(result.sequences[0].step_results.len(), 3);
}

// ============================================================================
// Edge Case 2: TEST_EDGE_ALL_MISSING_001
// Test case defined but no steps in execution log
// ============================================================================

#[test]
fn test_edge_all_missing_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_ALL_MISSING_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 0, failed: 0, not_executed: 3
    assert_eq!(result.test_case_id, "TEST_EDGE_ALL_MISSING_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 0, "Should have 0 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 3,
        "Should have 3 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should not pass"
    );
}

// ============================================================================
// Edge Case 3: TEST_EDGE_ALL_PASS_ONE_MISSING_001
// All executed steps pass but one step missing
// ============================================================================

#[test]
fn test_edge_all_pass_one_missing_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_ALL_PASS_ONE_MISSING_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 4, failed: 0, not_executed: 1
    assert_eq!(result.test_case_id, "TEST_EDGE_ALL_PASS_ONE_MISSING_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 5);
    assert_eq!(result.passed_steps, 4, "Should have 4 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 1,
        "Should have 1 not executed step"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail due to missing step"
    );
}

// ============================================================================
// Edge Case 4: TEST_EDGE_DUPLICATE_STEPS_001
// Execution log has duplicate step entries
// ============================================================================

#[test]
fn test_edge_duplicate_steps_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_DUPLICATE_STEPS_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = true, passed: 3, failed: 0, not_executed: 0
    // Duplicates should not affect pass/fail, only first execution counts
    assert_eq!(result.test_case_id, "TEST_EDGE_DUPLICATE_STEPS_001");
    assert!(result.overall_pass, "Overall pass should be true");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 3, "Should have 3 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 0,
        "Should have 0 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(result.sequences[0].all_steps_passed, "Sequence should pass");
}

// ============================================================================
// Edge Case 5: TEST_EDGE_EXTRA_STEPS_001
// Execution log has steps not in test case
// ============================================================================

#[test]
fn test_edge_extra_steps_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_EXTRA_STEPS_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = true, passed: 3, failed: 0, not_executed: 0
    // Extra steps in log should be ignored
    assert_eq!(result.test_case_id, "TEST_EDGE_EXTRA_STEPS_001");
    assert!(result.overall_pass, "Overall pass should be true");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 3, "Should have 3 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 0,
        "Should have 0 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(result.sequences[0].all_steps_passed, "Sequence should pass");
}

// ============================================================================
// Edge Case 6: TEST_EDGE_LAST_STEP_ONLY_001
// Only last step executed, earlier steps missing
// ============================================================================

#[test]
fn test_edge_last_step_only_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_LAST_STEP_ONLY_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 1, failed: 0, not_executed: 3
    assert_eq!(result.test_case_id, "TEST_EDGE_LAST_STEP_ONLY_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 4);
    assert_eq!(result.passed_steps, 1, "Should have 1 passed step");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 3,
        "Should have 3 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 7: TEST_EDGE_MISSING_FIRST_001
// First step missing while others executed
// ============================================================================

#[test]
fn test_edge_missing_first_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MISSING_FIRST_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 2, failed: 0, not_executed: 1
    assert_eq!(result.test_case_id, "TEST_EDGE_MISSING_FIRST_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 2, "Should have 2 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 1,
        "Should have 1 not executed step"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 8: TEST_EDGE_MISSING_LAST_001
// Last step missing with previous steps executed
// ============================================================================

#[test]
fn test_edge_missing_last_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MISSING_LAST_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 2, failed: 0, not_executed: 1
    assert_eq!(result.test_case_id, "TEST_EDGE_MISSING_LAST_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 2, "Should have 2 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 1,
        "Should have 1 not executed step"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 9: TEST_EDGE_MISSING_MIDDLE_001
// Middle step missing with first and last present
// ============================================================================

#[test]
fn test_edge_missing_middle_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MISSING_MIDDLE_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 2, failed: 0, not_executed: 1
    assert_eq!(result.test_case_id, "TEST_EDGE_MISSING_MIDDLE_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 2, "Should have 2 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 1,
        "Should have 1 not executed step"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 10: TEST_EDGE_MIXED_PASS_FAIL_001
// Alternating pass/fail pattern across steps
// ============================================================================

#[test]
fn test_edge_mixed_pass_fail_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MIXED_PASS_FAIL_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 3, failed: 2, not_executed: 0
    assert_eq!(result.test_case_id, "TEST_EDGE_MIXED_PASS_FAIL_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 5);
    assert_eq!(result.passed_steps, 3, "Should have 3 passed steps");
    assert_eq!(result.failed_steps, 2, "Should have 2 failed steps");
    assert_eq!(
        result.not_executed_steps, 0,
        "Should have 0 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 11: TEST_EDGE_ONE_CORRECT_REST_MISSING_001
// Single step passes, all others missing
// ============================================================================

#[test]
fn test_edge_one_correct_rest_missing_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_ONE_CORRECT_REST_MISSING_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 1, failed: 0, not_executed: 4
    assert_eq!(
        result.test_case_id,
        "TEST_EDGE_ONE_CORRECT_REST_MISSING_001"
    );
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 5);
    assert_eq!(result.passed_steps, 1, "Should have 1 passed step");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 4,
        "Should have 4 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 12: TEST_EDGE_PARTIAL_SEQ1_001
// Multi-sequence with only seq 1 step 1 executed
// ============================================================================

#[test]
fn test_edge_partial_seq1_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_PARTIAL_SEQ1_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 1, failed: 0, not_executed: 3
    assert_eq!(result.test_case_id, "TEST_EDGE_PARTIAL_SEQ1_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 4);
    assert_eq!(result.passed_steps, 1, "Should have 1 passed step");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 3,
        "Should have 3 not executed steps"
    );

    // Verify sequence-level results - 2 sequences
    assert_eq!(result.sequences.len(), 2);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence 1 should fail"
    );
    assert!(
        !result.sequences[1].all_steps_passed,
        "Sequence 2 should fail (not executed)"
    );
}

// ============================================================================
// Edge Case 13: TEST_EDGE_PARTIAL_SEQ2_001
// Multi-sequence with seq 1 complete, seq 2 partial
// ============================================================================

#[test]
fn test_edge_partial_seq2_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_PARTIAL_SEQ2_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 3, failed: 0, not_executed: 1
    assert_eq!(result.test_case_id, "TEST_EDGE_PARTIAL_SEQ2_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 4);
    assert_eq!(result.passed_steps, 3, "Should have 3 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 1,
        "Should have 1 not executed step"
    );

    // Verify sequence-level results - 2 sequences
    assert_eq!(result.sequences.len(), 2);
    assert!(
        result.sequences[0].all_steps_passed,
        "Sequence 1 should pass"
    );
    assert!(
        !result.sequences[1].all_steps_passed,
        "Sequence 2 should fail"
    );
}

// ============================================================================
// Edge Case 14: TEST_EDGE_SPARSE_EXECUTION_001
// Sparse execution pattern with gaps
// ============================================================================

#[test]
fn test_edge_sparse_execution_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_SPARSE_EXECUTION_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = false, passed: 3, failed: 0, not_executed: 3
    assert_eq!(result.test_case_id, "TEST_EDGE_SPARSE_EXECUTION_001");
    assert!(!result.overall_pass, "Overall pass should be false");
    assert_eq!(result.total_steps, 6);
    assert_eq!(result.passed_steps, 3, "Should have 3 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 3,
        "Should have 3 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(
        !result.sequences[0].all_steps_passed,
        "Sequence should fail"
    );
}

// ============================================================================
// Edge Case 15: TEST_EDGE_WRONG_SEQUENCE_001
// Steps executed in wrong order
// ============================================================================

#[test]
fn test_edge_wrong_sequence_001() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_WRONG_SEQUENCE_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    // Expected: overall_pass = true, passed: 3, failed: 0, not_executed: 0
    // Order doesn't matter for verification, only that all steps executed and passed
    assert_eq!(result.test_case_id, "TEST_EDGE_WRONG_SEQUENCE_001");
    assert!(result.overall_pass, "Overall pass should be true");
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 3, "Should have 3 passed steps");
    assert_eq!(result.failed_steps, 0, "Should have 0 failed steps");
    assert_eq!(
        result.not_executed_steps, 0,
        "Should have 0 not executed steps"
    );

    // Verify sequence-level results
    assert_eq!(result.sequences.len(), 1);
    assert!(result.sequences[0].all_steps_passed, "Sequence should pass");
}

// ============================================================================
// Comprehensive Edge Case Validation Test
// Tests all 15 edge cases in a single comprehensive test
// ============================================================================

#[test]
fn test_all_edge_cases_comprehensive() {
    let edge_cases = vec![
        ("TEST_EDGE_ALL_FAIL_001", false, 3, 0, 3, 0),
        ("TEST_EDGE_ALL_MISSING_001", false, 3, 0, 0, 3),
        ("TEST_EDGE_ALL_PASS_ONE_MISSING_001", false, 5, 4, 0, 1),
        ("TEST_EDGE_DUPLICATE_STEPS_001", true, 3, 3, 0, 0),
        ("TEST_EDGE_EXTRA_STEPS_001", true, 3, 3, 0, 0),
        ("TEST_EDGE_LAST_STEP_ONLY_001", false, 4, 1, 0, 3),
        ("TEST_EDGE_MISSING_FIRST_001", false, 3, 2, 0, 1),
        ("TEST_EDGE_MISSING_LAST_001", false, 3, 2, 0, 1),
        ("TEST_EDGE_MISSING_MIDDLE_001", false, 3, 2, 0, 1),
        ("TEST_EDGE_MIXED_PASS_FAIL_001", false, 5, 3, 2, 0),
        ("TEST_EDGE_ONE_CORRECT_REST_MISSING_001", false, 5, 1, 0, 4),
        ("TEST_EDGE_PARTIAL_SEQ1_001", false, 4, 1, 0, 3),
        ("TEST_EDGE_PARTIAL_SEQ2_001", false, 4, 3, 0, 1),
        ("TEST_EDGE_SPARSE_EXECUTION_001", false, 6, 3, 0, 3),
        ("TEST_EDGE_WRONG_SEQUENCE_001", true, 3, 3, 0, 0),
    ];

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    for (test_id, expected_pass, total, passed, failed, not_executed) in edge_cases {
        let (yaml_path, json_path) = get_edge_case_path(test_id);
        let test_case = load_test_case(&yaml_path);
        let logs = verifier.parse_log_file(&json_path).unwrap();
        let result = verifier.verify_test_case(&test_case, &logs);

        assert_eq!(
            result.overall_pass, expected_pass,
            "{}: Expected overall_pass = {}, got {}",
            test_id, expected_pass, result.overall_pass
        );
        assert_eq!(
            result.total_steps, total,
            "{}: Expected total_steps = {}, got {}",
            test_id, total, result.total_steps
        );
        assert_eq!(
            result.passed_steps, passed,
            "{}: Expected passed_steps = {}, got {}",
            test_id, passed, result.passed_steps
        );
        assert_eq!(
            result.failed_steps, failed,
            "{}: Expected failed_steps = {}, got {}",
            test_id, failed, result.failed_steps
        );
        assert_eq!(
            result.not_executed_steps, not_executed,
            "{}: Expected not_executed_steps = {}, got {}",
            test_id, not_executed, result.not_executed_steps
        );
    }
}

// ============================================================================
// Duplicate Step Handling Tests
// ============================================================================

#[test]
fn test_duplicate_steps_first_execution_counts() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_DUPLICATE_STEPS_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();

    // Verify that we have 5 log entries (3 unique + 2 duplicates)
    assert_eq!(
        logs.len(),
        5,
        "Should have 5 log entries including duplicates"
    );

    let result = verifier.verify_test_case(&test_case, &logs);

    // Despite having 5 log entries, only 3 unique steps should be verified
    assert_eq!(
        result.sequences[0].step_results.len(),
        3,
        "Should have 3 step results"
    );
    assert!(result.overall_pass, "Should pass despite duplicates");
}

// ============================================================================
// Extra Step Handling Tests
// ============================================================================

#[test]
fn test_extra_steps_are_ignored() {
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_EXTRA_STEPS_001");
    let test_case = load_test_case(&yaml_path);

    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    let logs = verifier.parse_log_file(&json_path).unwrap();

    // Test case has 3 steps, log has 5 entries
    assert!(logs.len() >= 3, "Should have at least 3 log entries");

    let result = verifier.verify_test_case(&test_case, &logs);

    // Only defined steps (3) should be in result
    assert_eq!(result.total_steps, 3, "Should only count defined steps");
    assert_eq!(
        result.sequences[0].step_results.len(),
        3,
        "Should have 3 step results"
    );
}

// ============================================================================
// Missing Step Pattern Tests
// ============================================================================

#[test]
fn test_missing_step_patterns() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    // Test missing first step
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MISSING_FIRST_001");
    let test_case = load_test_case(&yaml_path);
    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);
    assert_eq!(result.not_executed_steps, 1);
    assert_eq!(result.passed_steps, 2);

    // Test missing middle step
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MISSING_MIDDLE_001");
    let test_case = load_test_case(&yaml_path);
    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);
    assert_eq!(result.not_executed_steps, 1);
    assert_eq!(result.passed_steps, 2);

    // Test missing last step
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_MISSING_LAST_001");
    let test_case = load_test_case(&yaml_path);
    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);
    assert_eq!(result.not_executed_steps, 1);
    assert_eq!(result.passed_steps, 2);
}

// ============================================================================
// Multi-Sequence Edge Case Tests
// ============================================================================

#[test]
fn test_multi_sequence_partial_execution() {
    let temp_dir = tempfile::TempDir::new().unwrap();
    let storage = TestCaseStorage::new(temp_dir.path()).unwrap();
    let verifier =
        TestVerifier::with_strategies(storage, MatchStrategy::Exact, MatchStrategy::Exact);

    // Test partial sequence 1
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_PARTIAL_SEQ1_001");
    let test_case = load_test_case(&yaml_path);
    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    assert_eq!(result.sequences.len(), 2, "Should have 2 sequences");
    assert!(!result.sequences[0].all_steps_passed, "Seq 1 incomplete");
    assert!(!result.sequences[1].all_steps_passed, "Seq 2 not executed");

    // Test partial sequence 2
    let (yaml_path, json_path) = get_edge_case_path("TEST_EDGE_PARTIAL_SEQ2_001");
    let test_case = load_test_case(&yaml_path);
    let logs = verifier.parse_log_file(&json_path).unwrap();
    let result = verifier.verify_test_case(&test_case, &logs);

    assert_eq!(result.sequences.len(), 2, "Should have 2 sequences");
    assert!(
        result.sequences[0].all_steps_passed,
        "Seq 1 should be complete"
    );
    assert!(!result.sequences[1].all_steps_passed, "Seq 2 incomplete");
}
