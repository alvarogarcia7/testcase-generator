use chrono::{Local, Utc};
use std::path::PathBuf;
use testcase_manager::models::{
    Expected, Step, TestCase, TestSequence, Verification, VerificationExpression,
};
use testcase_manager::verification::{
    DiffDetail, MatchStrategy, StepVerificationResult, TestCaseVerificationResult,
    TestExecutionLog, TestVerifier, VerificationDiff,
};

fn create_step(step_num: i64, result: &str, output: &str, success: Option<bool>) -> Step {
    Step {
        step: step_num,
        manual: None,
        description: "Test step".to_string(),
        command: "test command".to_string(),
        capture_vars: None,
        expected: Expected {
            success,
            result: result.to_string(),
            output: output.to_string(),
        },
        verification: Verification {
            result: VerificationExpression::Simple("[[ $? -eq 0 ]]".to_string()),
            output: VerificationExpression::Simple(
                "cat $COMMAND_OUTPUT | grep -q \"${OUTPUT}\"".to_string(),
            ),
            output_file: None,
            general: None,
        },
        reference: None,
    }
}

fn create_execution_log(
    test_case_id: &str,
    sequence_id: i64,
    step_number: i64,
    actual_result: &str,
    actual_output: &str,
    success: Option<bool>,
) -> TestExecutionLog {
    TestExecutionLog {
        test_case_id: test_case_id.to_string(),
        sequence_id,
        step_number,
        success,
        actual_result: actual_result.to_string(),
        actual_output: actual_output.to_string(),
        timestamp: Some(Local::now().with_timezone(&Utc)),
        log_file_path: PathBuf::from("test.json"),
        result_verification_pass: None,
        output_verification_pass: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn create_execution_log_with_precomputed(
    test_case_id: &str,
    sequence_id: i64,
    step_number: i64,
    actual_result: &str,
    actual_output: &str,
    success: Option<bool>,
    result_verification_pass: Option<bool>,
    output_verification_pass: Option<bool>,
) -> TestExecutionLog {
    TestExecutionLog {
        test_case_id: test_case_id.to_string(),
        sequence_id,
        step_number,
        success,
        actual_result: actual_result.to_string(),
        actual_output: actual_output.to_string(),
        timestamp: Some(Local::now().with_timezone(&Utc)),
        log_file_path: PathBuf::from("test.json"),
        result_verification_pass,
        output_verification_pass,
    }
}

// ============================================================================
// Exact Matching Tests
// ============================================================================

#[test]
fn test_exact_match_all_fields_match() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "SW=0x9000", "Success", Some(true));
    let log = create_execution_log("TC001", 1, 1, "SW=0x9000", "Success", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
    assert!(result.success_match);
    assert_eq!(result.step_number, 1);
    assert!(result.diff.result_diff.is_none());
    assert!(result.diff.output_diff.is_none());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_exact_match_result_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "SW=0x9000", "Success", Some(true));
    let log = create_execution_log("TC001", 1, 1, "SW=0x6A82", "Success", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(result.output_match);
    assert!(result.success_match);

    let diff = result.diff.result_diff.as_ref().unwrap();
    assert_eq!(diff.expected, "SW=0x9000");
    assert_eq!(diff.actual, "SW=0x6A82");
    assert!(diff.message.contains("Result mismatch"));
}

#[test]
fn test_exact_match_output_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "SW=0x9000", "Success", Some(true));
    let log = create_execution_log("TC001", 1, 1, "SW=0x9000", "Failed", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(result.result_match);
    assert!(!result.output_match);
    assert!(result.success_match);

    let diff = result.diff.output_diff.as_ref().unwrap();
    assert_eq!(diff.expected, "Success");
    assert_eq!(diff.actual, "Failed");
    assert!(diff.message.contains("Output mismatch"));
}

#[test]
fn test_exact_match_case_sensitive() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "success", "output", None);
    let log = create_execution_log("TC001", 1, 1, "SUCCESS", "OUTPUT", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_exact_match_whitespace_sensitive() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let log = create_execution_log("TC001", 1, 1, " result ", " output ", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_exact_match_empty_strings() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "", "", None);
    let log = create_execution_log("TC001", 1, 1, "", "", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_exact_match_empty_expected_vs_nonempty_actual() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "", "", None);
    let log = create_execution_log("TC001", 1, 1, "something", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

// ============================================================================
// Regex Pattern Matching Tests
// ============================================================================

#[test]
fn test_regex_match_basic_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"SW=0x[0-9A-Fa-f]{4}", r"Suc\w+", None);
    let log = create_execution_log("TC001", 1, 1, "SW=0x9000", "Success", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_complex_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(
        1,
        r"^\d{3}-\d{3}-\d{4}$",
        r"^[A-Z][a-z]+\s[A-Z][a-z]+$",
        None,
    );
    let log = create_execution_log("TC001", 1, 1, "123-456-7890", "John Smith", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_pattern_mismatch() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"^\d{4}$", r"^Success$", None);
    let log = create_execution_log("TC001", 1, 1, "12345", "Failed", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_regex_match_invalid_regex_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Exact);
    let step = create_step(1, "[invalid(regex", "Success", None);
    let log = create_execution_log("TC001", 1, 1, "anything", "Success", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_multiline_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"Line1", r"Out1", None);
    let log = create_execution_log("TC001", 1, 1, "Line1\nLine2", "Out1\nOut2", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_special_characters() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"\$\d+\.\d{2}", r"\[.*\]", None);
    let log = create_execution_log("TC001", 1, 1, "$99.99", "[SUCCESS]", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_unicode_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"你好.*世界", r"🚀.*🎉", None);
    let log = create_execution_log("TC001", 1, 1, "你好 世界", "🚀 Success 🎉", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_anchored_vs_unanchored() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"test", r"^output$", None);
    let log = create_execution_log("TC001", 1, 1, "this is a test string", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

// ============================================================================
// Substring Contains Matching Tests
// ============================================================================

#[test]
fn test_contains_match_basic() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "9000", "Success", None);
    let log = create_execution_log("TC001", 1, 1, "SW=0x9000", "Operation Success", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_substring_not_found() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "9000", "Success", None);
    let log = create_execution_log("TC001", 1, 1, "SW=0x6A82", "Failed", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_contains_match_case_sensitive() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "success", "output", None);
    let log = create_execution_log("TC001", 1, 1, "SUCCESS", "OUTPUT", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_contains_match_empty_expected_matches_all() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "", "", None);
    let log = create_execution_log("TC001", 1, 1, "any result", "any output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_partial_word() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "err", "warn", None);
    let log = create_execution_log(
        "TC001",
        1,
        1,
        "error occurred",
        "warning message",
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_multiline() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "Line2", "Out2", None);
    let log = create_execution_log(
        "TC001",
        1,
        1,
        "Line1\nLine2\nLine3",
        "Out1\nOut2\nOut3",
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_special_characters() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "$99", "[OK]", None);
    let log = create_execution_log("TC001", 1, 1, "Price: $99.99", "Status: [OK]", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_unicode() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "你好", "🚀", None);
    let log = create_execution_log("TC001", 1, 1, "测试 你好 世界", "Start 🚀 End", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

// ============================================================================
// Success Flag Validation Tests
// ============================================================================

#[test]
fn test_success_flag_true_matches_true() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(true));
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_false_matches_false() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(false));
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(false));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_true_mismatch_false() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(true));
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(false));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.success_match);
    assert!(result.result_match);
    assert!(result.output_match);

    let diff = result.diff.success_diff.as_ref().unwrap();
    assert_eq!(diff.expected, "true");
    assert_eq!(diff.actual, "false");
    assert!(diff.message.contains("Success flag mismatch"));
}

#[test]
fn test_success_flag_false_mismatch_true() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(false));
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.success_match);

    let diff = result.diff.success_diff.as_ref().unwrap();
    assert_eq!(diff.expected, "false");
    assert_eq!(diff.actual, "true");
}

#[test]
fn test_success_flag_none_matches_true() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_none_matches_false() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(false));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_none_with_other_field_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "expected", "output", None);
    let log = create_execution_log("TC001", 1, 1, "actual", "output", Some(false));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

// ============================================================================
// Multi-Step Verification Tests
// ============================================================================

#[test]
fn test_multi_step_all_pass() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());

    sequence
        .steps
        .push(create_step(1, "Success", "OK", Some(true)));
    sequence
        .steps
        .push(create_step(2, "Complete", "Done", Some(true)));
    sequence
        .steps
        .push(create_step(3, "Finished", "End", Some(true)));

    test_case.test_sequences.push(sequence);

    let logs = vec![
        create_execution_log("TC001", 1, 1, "Success", "OK", Some(true)),
        create_execution_log("TC001", 1, 2, "Complete", "Done", Some(true)),
        create_execution_log("TC001", 1, 3, "Finished", "End", Some(true)),
    ];

    let result = verifier.verify_test_case(&test_case, &logs);

    assert!(result.overall_pass);
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 3);
    assert_eq!(result.failed_steps, 0);
}

#[test]
fn test_multi_step_mixed_pass_fail() {
    let verifier = TestVerifier::with_exact_matching();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());

    sequence
        .steps
        .push(create_step(1, "output1", "output1", Some(true)));
    sequence
        .steps
        .push(create_step(2, "expected", "expected", Some(true)));
    sequence
        .steps
        .push(create_step(3, "output1", "output1", Some(true)));

    test_case.test_sequences.push(sequence);

    let logs = vec![
        create_execution_log("TC001", 1, 1, "output1", "output1", Some(true)),
        create_execution_log("TC001", 1, 2, "actual", "actual", Some(true)),
        create_execution_log("TC001", 1, 3, "output1", "output1", Some(true)),
    ];

    let result = verifier.verify_test_case(&test_case, &logs);

    assert!(!result.overall_pass);
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 2);
    assert_eq!(result.failed_steps, 1);
}

#[test]
fn test_multi_step_all_fail() {
    let verifier = TestVerifier::with_exact_matching();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());

    sequence
        .steps
        .push(create_step(1, "expected1", "out1", Some(true)));
    sequence
        .steps
        .push(create_step(2, "expected2", "out2", Some(true)));
    sequence
        .steps
        .push(create_step(3, "expected3", "out3", Some(true)));

    test_case.test_sequences.push(sequence);

    let logs = vec![
        create_execution_log("TC001", 1, 1, "wrong", "wrong", Some(false)),
        create_execution_log("TC001", 1, 2, "wrong", "wrong", Some(false)),
        create_execution_log("TC001", 1, 3, "wrong", "wrong", Some(false)),
    ];

    let result = verifier.verify_test_case(&test_case, &logs);

    assert!(!result.overall_pass);
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 0);
    assert_eq!(result.failed_steps, 3);
}

#[test]
fn test_multi_step_with_different_strategies() {
    let verifier_result_exact = TestVerifier::new(MatchStrategy::Exact, MatchStrategy::Contains);
    let step = create_step(1, "exact_result", "partial", Some(true));
    let log = create_execution_log(
        "TC001",
        1,
        1,
        "exact_result",
        "partial output string",
        Some(true),
    );

    let result = verifier_result_exact.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_multi_step_empty_sequence() {
    let verifier = TestVerifier::with_exact_matching();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());
    test_case.test_sequences.push(sequence);

    let logs = vec![];

    let result = verifier.verify_test_case(&test_case, &logs);

    assert!(result.overall_pass);
    assert_eq!(result.total_steps, 0);
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[test]
fn test_edge_case_empty_result_and_output() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "", "", Some(true));
    let log = create_execution_log("TC001", 1, 1, "", "", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
    assert!(result.success_match);
}

#[test]
fn test_edge_case_very_long_strings() {
    let verifier = TestVerifier::with_exact_matching();
    let long_string = "A".repeat(10000);
    let step = create_step(1, &long_string, &long_string, None);
    let log = create_execution_log("TC001", 1, 1, &long_string, &long_string, Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_unicode_characters() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "测试 你好 世界 🚀 🎉", "Привет мир 日本語", Some(true));
    let log = create_execution_log(
        "TC001",
        1,
        1,
        "测试 你好 世界 🚀 🎉",
        "Привет мир 日本語",
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_unicode_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "你好", "🚀", None);
    let log = create_execution_log("TC001", 1, 1, "你好世界", "🎉", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_multiline_output() {
    let verifier = TestVerifier::with_exact_matching();
    let multiline = "Line1\nLine2\nLine3\nLine4";
    let step = create_step(1, multiline, multiline, None);
    let log = create_execution_log("TC001", 1, 1, multiline, multiline, Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_multiline_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "Line1\nLine2", "Out1\nOut2", None);
    let log = create_execution_log("TC001", 1, 1, "Line1\nLine3", "Out1\nOut3", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_special_regex_characters_in_exact_match() {
    let verifier = TestVerifier::with_exact_matching();
    let special_chars = r".*+?^${}[]()|\";
    let step = create_step(1, special_chars, special_chars, None);
    let log = create_execution_log("TC001", 1, 1, special_chars, special_chars, Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_whitespace_variations() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "  spaces  ", "\ttabs\t", None);
    let log = create_execution_log("TC001", 1, 1, " spaces ", "\ttabs\t\n", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_newline_types() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "Line1\nLine2", "Out1\nOut2", None);
    let log = create_execution_log("TC001", 1, 1, "Line1\r\nLine2", "Out1\r\nOut2", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_null_bytes() {
    let verifier = TestVerifier::with_exact_matching();
    let with_null = "Before\x00After";
    let step = create_step(1, with_null, with_null, None);
    let log = create_execution_log("TC001", 1, 1, with_null, with_null, Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_missing_step_in_logs() {
    let verifier = TestVerifier::with_exact_matching();
    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());
    sequence
        .steps
        .push(create_step(1, "result", "output", Some(true)));
    sequence
        .steps
        .push(create_step(2, "result", "output", Some(true)));

    test_case.test_sequences.push(sequence);

    // Only provide log for step 1, not step 2
    let logs = vec![create_execution_log(
        "TC001",
        1,
        1,
        "result",
        "output",
        Some(true),
    )];

    let result = verifier.verify_test_case(&test_case, &logs);

    assert!(!result.overall_pass);
    assert_eq!(result.total_steps, 2);
    assert_eq!(result.passed_steps, 1);
    assert_eq!(result.not_executed_steps, 1);
}

#[test]
fn test_edge_case_contains_with_regex_special_chars() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "[test]", "(output)", None);
    let log = create_execution_log(
        "TC001",
        1,
        1,
        "Result: [test] done",
        "Status: (output) ok",
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_regex_dot_matches_newline_when_specified() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"(?s)Line1.*Line3", r"(?s)Out1.*Out3", None);
    let log = create_execution_log(
        "TC001",
        1,
        1,
        "Line1\nLine2\nLine3",
        "Out1\nOut2\nOut3",
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

// ============================================================================
// Diff Detail Tests
// ============================================================================

#[test]
fn test_diff_detail_all_fields() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log("TC001", 1, 1, "actual_result", "actual_output", Some(false));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);

    let result_diff = result.diff.result_diff.as_ref().unwrap();
    assert_eq!(result_diff.expected, "expected_result");
    assert_eq!(result_diff.actual, "actual_result");
    assert!(result_diff.message.contains("Result mismatch"));
    assert!(result_diff.message.contains("Exact"));

    let output_diff = result.diff.output_diff.as_ref().unwrap();
    assert_eq!(output_diff.expected, "expected_output");
    assert_eq!(output_diff.actual, "actual_output");
    assert!(output_diff.message.contains("Output mismatch"));
    assert!(output_diff.message.contains("Exact"));

    let success_diff = result.diff.success_diff.as_ref().unwrap();
    assert_eq!(success_diff.expected, "true");
    assert_eq!(success_diff.actual, "false");
    assert!(success_diff.message.contains("Success flag mismatch"));
}

#[test]
fn test_diff_detail_only_result() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "expected", "output", Some(true));
    let log = create_execution_log("TC001", 1, 1, "actual", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_some());
    assert!(result.diff.output_diff.is_none());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_diff_detail_only_output() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "expected", Some(true));
    let log = create_execution_log("TC001", 1, 1, "result", "actual", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_none());
    assert!(result.diff.output_diff.is_some());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_diff_detail_only_success() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(true));
    let log = create_execution_log("TC001", 1, 1, "result", "output", Some(false));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_none());
    assert!(result.diff.output_diff.is_none());
    assert!(result.diff.success_diff.is_some());
}

#[test]
fn test_diff_detail_none_success_in_message() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let log = create_execution_log("TC001", 1, 1, "wrong", "output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_some());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_diff_detail_contains_strategy_message() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "expected", "output", None);
    let log = create_execution_log("TC001", 1, 1, "actual", "wrong", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);

    let result_diff = result.diff.result_diff.as_ref().unwrap();
    assert!(result_diff.message.contains("Contains"));

    let output_diff = result.diff.output_diff.as_ref().unwrap();
    assert!(output_diff.message.contains("Contains"));
}

#[test]
fn test_diff_detail_regex_strategy_message() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"^\d+$", r"^[A-Z]+$", None);
    let log = create_execution_log("TC001", 1, 1, "abc", "123", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);

    let result_diff = result.diff.result_diff.as_ref().unwrap();
    assert!(result_diff.message.contains("Regex"));

    let output_diff = result.diff.output_diff.as_ref().unwrap();
    assert!(output_diff.message.contains("Regex"));
}

// ============================================================================
// Default and Constructor Tests
// ============================================================================

#[test]
fn test_default_verifier_uses_exact_matching() {
    let verifier = TestVerifier::default();
    let step = create_step(1, "result", "output", None);
    let exact_log = create_execution_log("TC001", 1, 1, "result", "output", Some(true));
    let contains_log = create_execution_log(
        "TC001",
        1,
        1,
        "result and more",
        "output and more",
        Some(true),
    );

    let result_exact = verifier.verify_step_from_log(&step, &exact_log);
    let result_contains = verifier.verify_step_from_log(&step, &contains_log);

    assert!(result_exact.passed);
    assert!(!result_contains.passed);
}

#[test]
fn test_verifier_with_mixed_strategies() {
    let verifier = TestVerifier::new(MatchStrategy::Exact, MatchStrategy::Contains);
    let step = create_step(1, "exact_result", "partial", None);
    let log = create_execution_log("TC001", 1, 1, "exact_result", "partial output", Some(true));

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_multiple_verifiers_independent() {
    let verifier1 = TestVerifier::with_exact_matching();
    let verifier2 = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);

    let step = create_step(1, "result", "output", None);
    let log = create_execution_log("TC001", 1, 1, "result string", "output string", Some(true));

    let result1 = verifier1.verify_step_from_log(&step, &log);
    let result2 = verifier2.verify_step_from_log(&step, &log);

    assert!(!result1.passed);
    assert!(result2.passed);
}

// ============================================================================
// Serialization Tests
// ============================================================================

#[test]
fn test_step_verification_result_serialization() {
    use serde_json;

    let result = StepVerificationResult {
        step_number: 1,
        passed: true,
        result_match: true,
        output_match: true,
        success_match: true,
        diff: VerificationDiff {
            result_diff: None,
            output_diff: None,
            success_diff: None,
        },
    };

    let json = serde_json::to_string(&result).unwrap();
    let deserialized: StepVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(result, deserialized);
}

#[test]
fn test_diff_detail_serialization() {
    use serde_json;

    let diff = DiffDetail {
        expected: "expected".to_string(),
        actual: "actual".to_string(),
        message: "message".to_string(),
    };

    let json = serde_json::to_string(&diff).unwrap();
    let deserialized: DiffDetail = serde_json::from_str(&json).unwrap();

    assert_eq!(diff, deserialized);
}

#[test]
fn test_match_strategy_serialization() {
    use serde_json;

    let strategies = vec![
        MatchStrategy::Exact,
        MatchStrategy::Regex,
        MatchStrategy::Contains,
    ];

    for strategy in strategies {
        let json = serde_json::to_string(&strategy).unwrap();
        let deserialized: MatchStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(strategy, deserialized);
    }
}

// ============================================================================
// Report Generation Tests
// ============================================================================

#[test]
fn test_generate_report_yaml_basic() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let result = TestCaseVerificationResult {
        test_case_id: "TC001".to_string(),
        description: "Basic test case".to_string(),
        sequences: vec![],
        total_steps: 5,
        passed_steps: 4,
        failed_steps: 1,
        not_executed_steps: 0,
        overall_pass: false,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();

    assert!(yaml.contains("test_case_id: TC001"));
    assert!(yaml.contains("description: Basic test case"));
    assert!(yaml.contains("total_steps: 5"));
    assert!(yaml.contains("passed_steps: 4"));
    assert!(yaml.contains("failed_steps: 1"));
    assert!(yaml.contains("overall_pass: false"));
    assert!(yaml.contains("requirement: REQ001"));
}

#[test]
fn test_generate_report_yaml_with_sequences() {
    use testcase_manager::models::Expected;
    use testcase_manager::verification::{
        SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
        TestVerifier,
    };

    let verifier = TestVerifier::with_exact_matching();

    let sequence = SequenceVerificationResult {
        sequence_id: 1,
        name: "Test Sequence 1".to_string(),
        step_results: vec![
            StepVerificationResultEnum::Pass {
                step: 1,
                description: "Step 1 description".to_string(),
                requirement: Some("REQ001".to_string()),
                item: Some(1),
                tc: Some(1),
            },
            StepVerificationResultEnum::Fail {
                step: 2,
                description: "Step 2 description".to_string(),
                expected: Expected {
                    success: Some(true),
                    result: "expected_result".to_string(),
                    output: "expected_output".to_string(),
                },
                actual_result: "actual_result".to_string(),
                actual_output: "actual_output".to_string(),
                reason: "Result mismatch".to_string(),
                requirement: Some("REQ001".to_string()),
                item: Some(1),
                tc: Some(1),
            },
        ],
        all_steps_passed: false,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    };

    let result = TestCaseVerificationResult {
        test_case_id: "TC001".to_string(),
        description: "Test case with sequences".to_string(),
        sequences: vec![sequence],
        total_steps: 2,
        passed_steps: 1,
        failed_steps: 1,
        not_executed_steps: 0,
        overall_pass: false,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();

    assert!(yaml.contains("test_case_id: TC001"));
    assert!(yaml.contains("sequence_id: 1"));
    assert!(yaml.contains("Test Sequence 1"));
    assert!(yaml.contains("Step 1 description"));
    assert!(yaml.contains("Step 2 description"));
    assert!(yaml.contains("status: pass"));
    assert!(yaml.contains("status: fail"));
    assert!(yaml.contains("expected_result"));
    assert!(yaml.contains("actual_result"));
}

#[test]
fn test_generate_report_yaml_not_executed_steps() {
    use testcase_manager::verification::{
        SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
        TestVerifier,
    };

    let verifier = TestVerifier::with_exact_matching();

    let sequence = SequenceVerificationResult {
        sequence_id: 1,
        name: "Test Sequence".to_string(),
        step_results: vec![
            StepVerificationResultEnum::Pass {
                step: 1,
                description: "Executed step".to_string(),
                requirement: None,
                item: None,
                tc: None,
            },
            StepVerificationResultEnum::NotExecuted {
                step: 2,
                description: "Not executed step".to_string(),
                requirement: None,
                item: None,
                tc: None,
            },
        ],
        all_steps_passed: false,
        requirement: None,
        item: None,
        tc: None,
    };

    let result = TestCaseVerificationResult {
        test_case_id: "TC002".to_string(),
        description: "Test with not executed".to_string(),
        sequences: vec![sequence],
        total_steps: 2,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: None,
        item: None,
        tc: None,
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();

    assert!(yaml.contains("status: not_executed"));
    assert!(yaml.contains("Not executed step"));
    assert!(yaml.contains("not_executed_steps: 1"));
}

#[test]
fn test_generate_report_json_basic() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let result = TestCaseVerificationResult {
        test_case_id: "TC003".to_string(),
        description: "JSON test case".to_string(),
        sequences: vec![],
        total_steps: 3,
        passed_steps: 3,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: Some("REQ003".to_string()),
        item: Some(3),
        tc: Some(3),
    };

    let json = verifier.generate_report_json(&result).unwrap();

    assert!(json.contains("\"test_case_id\": \"TC003\""));
    assert!(json.contains("\"description\": \"JSON test case\""));
    assert!(json.contains("\"total_steps\": 3"));
    assert!(json.contains("\"passed_steps\": 3"));
    assert!(json.contains("\"failed_steps\": 0"));
    assert!(json.contains("\"overall_pass\": true"));
    assert!(json.contains("\"requirement\": \"REQ003\""));

    // Verify it can be deserialized
    let parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.test_case_id, "TC003");
    assert_eq!(parsed.total_steps, 3);
    assert!(parsed.overall_pass);
}

#[test]
fn test_generate_report_json_with_sequences() {
    use testcase_manager::models::Expected;
    use testcase_manager::verification::{
        SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
        TestVerifier,
    };

    let verifier = TestVerifier::with_exact_matching();

    let sequence = SequenceVerificationResult {
        sequence_id: 1,
        name: "JSON Sequence".to_string(),
        step_results: vec![StepVerificationResultEnum::Fail {
            step: 1,
            description: "Failed step".to_string(),
            expected: Expected {
                success: None,
                result: "0x9000".to_string(),
                output: "Success".to_string(),
            },
            actual_result: "0x6A82".to_string(),
            actual_output: "Error".to_string(),
            reason: "Status code mismatch".to_string(),
            requirement: None,
            item: None,
            tc: None,
        }],
        all_steps_passed: false,
        requirement: None,
        item: None,
        tc: None,
    };

    let result = TestCaseVerificationResult {
        test_case_id: "TC004".to_string(),
        description: "JSON with sequences".to_string(),
        sequences: vec![sequence],
        total_steps: 1,
        passed_steps: 0,
        failed_steps: 1,
        not_executed_steps: 0,
        overall_pass: false,
        requirement: None,
        item: None,
        tc: None,
    };

    let json = verifier.generate_report_json(&result).unwrap();

    assert!(json.contains("\"status\": \"fail\""));
    assert!(json.contains("\"expected\""));
    assert!(json.contains("\"0x9000\""));
    assert!(json.contains("\"0x6A82\""));
    assert!(json.contains("\"Status code mismatch\""));

    // Verify deserialization
    let parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.sequences.len(), 1);
}

#[test]
fn test_generate_report_yaml_roundtrip() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let original = TestCaseVerificationResult {
        test_case_id: "TC005".to_string(),
        description: "Roundtrip test".to_string(),
        sequences: vec![],
        total_steps: 10,
        passed_steps: 8,
        failed_steps: 1,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: Some("REQ005".to_string()),
        item: Some(5),
        tc: Some(5),
    };

    let yaml = verifier.generate_report_yaml(&original).unwrap();
    let parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(parsed.test_case_id, original.test_case_id);
    assert_eq!(parsed.description, original.description);
    assert_eq!(parsed.total_steps, original.total_steps);
    assert_eq!(parsed.passed_steps, original.passed_steps);
    assert_eq!(parsed.failed_steps, original.failed_steps);
    assert_eq!(parsed.not_executed_steps, original.not_executed_steps);
    assert_eq!(parsed.overall_pass, original.overall_pass);
}

#[test]
fn test_generate_report_json_roundtrip() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let original = TestCaseVerificationResult {
        test_case_id: "TC006".to_string(),
        description: "JSON roundtrip test".to_string(),
        sequences: vec![],
        total_steps: 7,
        passed_steps: 7,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    let json = verifier.generate_report_json(&original).unwrap();
    let parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed.test_case_id, original.test_case_id);
    assert_eq!(parsed.description, original.description);
    assert_eq!(parsed.total_steps, original.total_steps);
    assert_eq!(parsed.passed_steps, original.passed_steps);
    assert_eq!(parsed.overall_pass, original.overall_pass);
}

#[test]
fn test_report_generation_special_characters() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let result = TestCaseVerificationResult {
        test_case_id: "TC007".to_string(),
        description: "Test with special chars: \"quotes\", 'apostrophes', <tags>, & symbols"
            .to_string(),
        sequences: vec![],
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    // YAML generation should handle special characters
    let yaml = verifier.generate_report_yaml(&result).unwrap();
    assert!(yaml.contains("special chars"));

    // JSON generation should escape properly
    let json = verifier.generate_report_json(&result).unwrap();
    assert!(json.contains("special chars"));

    // Verify both can be deserialized
    let yaml_parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();
    let json_parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(yaml_parsed.description, result.description);
    assert_eq!(json_parsed.description, result.description);
}

#[test]
fn test_report_generation_unicode() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let result = TestCaseVerificationResult {
        test_case_id: "TC008".to_string(),
        description: "Test with unicode: 你好世界 🚀 Привет мир 日本語".to_string(),
        sequences: vec![],
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();
    let json = verifier.generate_report_json(&result).unwrap();

    // Verify unicode is preserved
    let yaml_parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();
    let json_parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(yaml_parsed.description, result.description);
    assert_eq!(json_parsed.description, result.description);
}

#[test]
fn test_report_generation_empty_sequences() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let result = TestCaseVerificationResult {
        test_case_id: "TC009".to_string(),
        description: "Test with no sequences".to_string(),
        sequences: vec![],
        total_steps: 0,
        passed_steps: 0,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();
    let json = verifier.generate_report_json(&result).unwrap();

    assert!(yaml.contains("sequences: []"));
    assert!(json.contains("\"sequences\": []"));
}

#[test]
fn test_report_generation_optional_fields() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    // Test with all optional fields present
    let with_fields = TestCaseVerificationResult {
        test_case_id: "TC010".to_string(),
        description: "With optional fields".to_string(),
        sequences: vec![],
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: Some("REQ010".to_string()),
        item: Some(10),
        tc: Some(10),
    };

    let yaml_with = verifier.generate_report_yaml(&with_fields).unwrap();
    assert!(yaml_with.contains("requirement: REQ010"));
    assert!(yaml_with.contains("item: 10"));
    assert!(yaml_with.contains("tc: 10"));

    // Test with all optional fields absent
    let without_fields = TestCaseVerificationResult {
        test_case_id: "TC011".to_string(),
        description: "Without optional fields".to_string(),
        sequences: vec![],
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    let yaml_without = verifier.generate_report_yaml(&without_fields).unwrap();
    // Optional fields should be omitted when None
    assert!(!yaml_without.contains("requirement:"));
    assert!(!yaml_without.contains("item:"));
    // Note: "tc:" might appear in test_case_id, so we check more specifically
    let lines: Vec<&str> = yaml_without.lines().collect();
    let has_tc_field = lines.iter().any(|line| line.trim().starts_with("tc:"));
    assert!(!has_tc_field);
}

#[test]
fn test_generate_report_yaml_complex_sequences() {
    use testcase_manager::models::Expected;
    use testcase_manager::verification::{
        SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
        TestVerifier,
    };

    let verifier = TestVerifier::with_exact_matching();

    // Create a complex report with multiple sequences and mixed results
    let sequences = vec![
        SequenceVerificationResult {
            sequence_id: 1,
            name: "Initialization Sequence".to_string(),
            step_results: vec![
                StepVerificationResultEnum::Pass {
                    step: 1,
                    description: "Initialize system".to_string(),
                    requirement: Some("REQ-INIT-001".to_string()),
                    item: Some(1),
                    tc: Some(1),
                },
                StepVerificationResultEnum::Pass {
                    step: 2,
                    description: "Verify configuration".to_string(),
                    requirement: Some("REQ-INIT-002".to_string()),
                    item: Some(1),
                    tc: Some(1),
                },
            ],
            all_steps_passed: true,
            requirement: Some("REQ-INIT".to_string()),
            item: Some(1),
            tc: Some(1),
        },
        SequenceVerificationResult {
            sequence_id: 2,
            name: "Execution Sequence".to_string(),
            step_results: vec![
                StepVerificationResultEnum::Pass {
                    step: 3,
                    description: "Execute command".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
                StepVerificationResultEnum::Fail {
                    step: 4,
                    description: "Validate output".to_string(),
                    expected: Expected {
                        success: Some(true),
                        result: "SUCCESS".to_string(),
                        output: "Operation completed".to_string(),
                    },
                    actual_result: "ERROR".to_string(),
                    actual_output: "Operation failed".to_string(),
                    reason: "Unexpected error occurred during execution".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
                StepVerificationResultEnum::NotExecuted {
                    step: 5,
                    description: "Cleanup resources".to_string(),
                    requirement: None,
                    item: None,
                    tc: None,
                },
            ],
            all_steps_passed: false,
            requirement: None,
            item: None,
            tc: None,
        },
    ];

    let result = TestCaseVerificationResult {
        test_case_id: "TC_COMPLEX".to_string(),
        description: "Complex test with multiple sequences".to_string(),
        sequences,
        total_steps: 5,
        passed_steps: 3,
        failed_steps: 1,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: Some("REQ-COMPLEX".to_string()),
        item: Some(99),
        tc: Some(99),
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();

    // Verify all sequences and steps are present
    assert!(yaml.contains("sequence_id: 1"));
    assert!(yaml.contains("sequence_id: 2"));
    assert!(yaml.contains("Initialization Sequence"));
    assert!(yaml.contains("Execution Sequence"));
    assert!(yaml.contains("Initialize system"));
    assert!(yaml.contains("Validate output"));
    assert!(yaml.contains("Cleanup resources"));
    assert!(yaml.contains("status: pass"));
    assert!(yaml.contains("status: fail"));
    assert!(yaml.contains("status: not_executed"));
}

#[test]
fn test_generate_report_json_complex_sequences() {
    use testcase_manager::models::Expected;
    use testcase_manager::verification::{
        SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
        TestVerifier,
    };

    let verifier = TestVerifier::with_exact_matching();

    let sequences = vec![SequenceVerificationResult {
        sequence_id: 1,
        name: "Test Sequence".to_string(),
        step_results: vec![
            StepVerificationResultEnum::Pass {
                step: 1,
                description: "Step 1".to_string(),
                requirement: None,
                item: None,
                tc: None,
            },
            StepVerificationResultEnum::Fail {
                step: 2,
                description: "Step 2".to_string(),
                expected: Expected {
                    success: None,
                    result: "expected".to_string(),
                    output: "output".to_string(),
                },
                actual_result: "actual".to_string(),
                actual_output: "actual_output".to_string(),
                reason: "mismatch".to_string(),
                requirement: None,
                item: None,
                tc: None,
            },
        ],
        all_steps_passed: false,
        requirement: None,
        item: None,
        tc: None,
    }];

    let result = TestCaseVerificationResult {
        test_case_id: "TC_JSON_COMPLEX".to_string(),
        description: "JSON complex test".to_string(),
        sequences,
        total_steps: 2,
        passed_steps: 1,
        failed_steps: 1,
        not_executed_steps: 0,
        overall_pass: false,
        requirement: None,
        item: None,
        tc: None,
    };

    let json = verifier.generate_report_json(&result).unwrap();

    // Verify JSON contains all expected elements
    assert!(json.contains("\"sequences\""));
    assert!(json.contains("\"sequence_id\": 1"));
    assert!(json.contains("\"status\": \"pass\""));
    assert!(json.contains("\"status\": \"fail\""));
    assert!(json.contains("\"expected\""));
    assert!(json.contains("\"actual_result\""));

    // Verify it can be deserialized
    let parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.sequences.len(), 1);
    assert_eq!(parsed.sequences[0].step_results.len(), 2);
}

#[test]
fn test_generate_report_multiline_descriptions() {
    use testcase_manager::verification::{
        SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
        TestVerifier,
    };

    let verifier = TestVerifier::with_exact_matching();

    let multiline_desc =
        "This is a test description\nthat spans multiple lines\nwith various details";

    let sequences = vec![SequenceVerificationResult {
        sequence_id: 1,
        name: "Multi-line test".to_string(),
        step_results: vec![StepVerificationResultEnum::Pass {
            step: 1,
            description: multiline_desc.to_string(),
            requirement: None,
            item: None,
            tc: None,
        }],
        all_steps_passed: true,
        requirement: None,
        item: None,
        tc: None,
    }];

    let result = TestCaseVerificationResult {
        test_case_id: "TC_MULTILINE".to_string(),
        description: multiline_desc.to_string(),
        sequences,
        total_steps: 1,
        passed_steps: 1,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();
    let json = verifier.generate_report_json(&result).unwrap();

    // Verify multiline strings are properly handled
    let yaml_parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();
    let json_parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(yaml_parsed.description, multiline_desc);
    assert_eq!(json_parsed.description, multiline_desc);
}

#[test]
fn test_generate_report_large_numbers() {
    use testcase_manager::verification::{TestCaseVerificationResult, TestVerifier};

    let verifier = TestVerifier::with_exact_matching();

    let result = TestCaseVerificationResult {
        test_case_id: "TC_LARGE".to_string(),
        description: "Test with large numbers".to_string(),
        sequences: vec![],
        total_steps: 10000,
        passed_steps: 9999,
        failed_steps: 1,
        not_executed_steps: 0,
        overall_pass: false,
        requirement: None,
        item: Some(999999),
        tc: Some(888888),
    };

    let yaml = verifier.generate_report_yaml(&result).unwrap();
    let json = verifier.generate_report_json(&result).unwrap();

    // Verify large numbers are properly serialized
    assert!(yaml.contains("total_steps: 10000"));
    assert!(yaml.contains("passed_steps: 9999"));
    assert!(json.contains("\"total_steps\": 10000"));
    assert!(json.contains("\"passed_steps\": 9999"));

    let yaml_parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();
    let json_parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(yaml_parsed.total_steps, 10000);
    assert_eq!(json_parsed.total_steps, 10000);
}

#[test]
fn test_report_generation_error_handling() {
    use testcase_manager::verification::TestVerifier;

    let verifier = TestVerifier::with_exact_matching();

    // Test that report generation doesn't panic with empty data
    let empty_result = testcase_manager::verification::TestCaseVerificationResult {
        test_case_id: "".to_string(),
        description: "".to_string(),
        sequences: vec![],
        total_steps: 0,
        passed_steps: 0,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    };

    let yaml = verifier.generate_report_yaml(&empty_result);
    let json = verifier.generate_report_json(&empty_result);

    assert!(yaml.is_ok());
    assert!(json.is_ok());
}

#[test]
fn test_step_verification_result_enum_methods() {
    use testcase_manager::verification::StepVerificationResultEnum;

    let pass = StepVerificationResultEnum::Pass {
        step: 1,
        description: "test".to_string(),
        requirement: Some("REQ".to_string()),
        item: Some(1),
        tc: Some(1),
    };

    assert!(pass.is_pass());
    assert_eq!(pass.step_number(), 1);
    assert_eq!(pass.requirement(), Some(&"REQ".to_string()));
    assert_eq!(pass.item(), Some(1));
    assert_eq!(pass.tc(), Some(1));

    let fail = StepVerificationResultEnum::Fail {
        step: 2,
        description: "test".to_string(),
        expected: testcase_manager::models::Expected {
            success: None,
            result: "r".to_string(),
            output: "o".to_string(),
        },
        actual_result: "a".to_string(),
        actual_output: "ao".to_string(),
        reason: "reason".to_string(),
        requirement: None,
        item: None,
        tc: None,
    };

    assert!(!fail.is_pass());
    assert_eq!(fail.step_number(), 2);
    assert_eq!(fail.requirement(), None);

    let not_executed = StepVerificationResultEnum::NotExecuted {
        step: 3,
        description: "test".to_string(),
        requirement: None,
        item: None,
        tc: None,
    };

    assert!(!not_executed.is_pass());
    assert_eq!(not_executed.step_number(), 3);
}

// ============================================================================
// Container Report Integration Tests
// ============================================================================

#[test]
fn test_container_report_from_batch_report_constructor() {
    use chrono::{TimeZone, Utc};
    use testcase_manager::verification::{BatchVerificationReport, ContainerReport};

    let mut batch_report = BatchVerificationReport::new();
    let fixed_time = Utc.with_ymd_and_hms(2024, 3, 15, 14, 30, 0).unwrap();
    batch_report.generated_at = fixed_time;

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC001".to_string(),
        description: "Test Case 1".to_string(),
        sequences: vec![],
        total_steps: 5,
        passed_steps: 4,
        failed_steps: 1,
        not_executed_steps: 0,
        overall_pass: false,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    });

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC002".to_string(),
        description: "Test Case 2".to_string(),
        sequences: vec![],
        total_steps: 3,
        passed_steps: 3,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: Some("REQ002".to_string()),
        item: Some(2),
        tc: Some(2),
    });

    let container_report = ContainerReport::from_batch_report(
        batch_report.clone(),
        "Test Report Title".to_string(),
        "Test Project".to_string(),
        Some("Test Environment".to_string()),
        Some("Test Platform".to_string()),
        Some("Test Executor".to_string()),
        123.45,
    );

    // Verify all fields are populated correctly
    assert_eq!(container_report.title, "Test Report Title");
    assert_eq!(container_report.project, "Test Project");
    assert_eq!(container_report.test_date, fixed_time);
    assert_eq!(container_report.test_results.len(), 2);
    assert_eq!(container_report.test_results[0].test_case_id, "TC001");
    assert_eq!(container_report.test_results[1].test_case_id, "TC002");

    // Verify metadata is populated correctly
    assert_eq!(
        container_report.metadata.environment,
        Some("Test Environment".to_string())
    );
    assert_eq!(
        container_report.metadata.platform,
        Some("Test Platform".to_string())
    );
    assert_eq!(
        container_report.metadata.executor,
        Some("Test Executor".to_string())
    );
    assert_eq!(container_report.metadata.execution_duration, 123.45);

    // Verify metadata aggregation from BatchVerificationReport
    assert_eq!(container_report.metadata.total_test_cases, 2);
    assert_eq!(container_report.metadata.passed_test_cases, 1);
    assert_eq!(container_report.metadata.failed_test_cases, 1);
}

#[test]
fn test_container_report_yaml_serialization_structure() {
    use chrono::{TimeZone, Utc};
    use testcase_manager::verification::{BatchVerificationReport, ContainerReport};

    let mut batch_report = BatchVerificationReport::new();
    let fixed_time = Utc.with_ymd_and_hms(2024, 3, 15, 14, 30, 0).unwrap();
    batch_report.generated_at = fixed_time;

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC001".to_string(),
        description: "First test case".to_string(),
        sequences: vec![],
        total_steps: 2,
        passed_steps: 2,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    });

    let container_report = ContainerReport::from_batch_report(
        batch_report,
        "GSMA eUICC Test Suite Results".to_string(),
        "GSMA SGP.22 Compliance Testing".to_string(),
        Some("GSMA Certification Lab - Environment 2".to_string()),
        Some("eUICC Test Platform v3.2.1".to_string()),
        Some("Automated Test Framework v2.5.0".to_string()),
        3845.7,
    );

    let yaml = serde_yaml::to_string(&container_report).unwrap();

    // Verify YAML contains all required sections in expected structure
    assert!(yaml.contains("title:"));
    assert!(yaml.contains("GSMA eUICC Test Suite Results"));
    assert!(yaml.contains("project:"));
    assert!(yaml.contains("GSMA SGP.22 Compliance Testing"));
    assert!(yaml.contains("test_date:"));
    assert!(yaml.contains("2024-03-15"));
    assert!(yaml.contains("test_results:"));
    assert!(yaml.contains("metadata:"));
    assert!(yaml.contains("environment:"));
    assert!(yaml.contains("platform:"));
    assert!(yaml.contains("executor:"));
    assert!(yaml.contains("execution_duration:"));
    assert!(yaml.contains("total_test_cases:"));
    assert!(yaml.contains("passed_test_cases:"));
    assert!(yaml.contains("failed_test_cases:"));

    // Verify sections appear in correct order
    let title_pos = yaml.find("title:").unwrap();
    let project_pos = yaml.find("project:").unwrap();
    let test_date_pos = yaml.find("test_date:").unwrap();
    let test_results_pos = yaml.find("test_results:").unwrap();
    let metadata_pos = yaml.find("metadata:").unwrap();

    assert!(title_pos < project_pos);
    assert!(project_pos < test_date_pos);
    assert!(test_date_pos < test_results_pos);
    assert!(test_results_pos < metadata_pos);
}

#[test]
fn test_container_report_metadata_aggregation() {
    use testcase_manager::verification::{BatchVerificationReport, ContainerReport};

    let mut batch_report = BatchVerificationReport::new();

    // Add multiple test cases with various results
    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC001".to_string(),
        description: "Test 1".to_string(),
        sequences: vec![],
        total_steps: 10,
        passed_steps: 10,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    });

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC002".to_string(),
        description: "Test 2".to_string(),
        sequences: vec![],
        total_steps: 8,
        passed_steps: 5,
        failed_steps: 2,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: None,
        item: None,
        tc: None,
    });

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC003".to_string(),
        description: "Test 3".to_string(),
        sequences: vec![],
        total_steps: 5,
        passed_steps: 5,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
        requirement: None,
        item: None,
        tc: None,
    });

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC004".to_string(),
        description: "Test 4".to_string(),
        sequences: vec![],
        total_steps: 12,
        passed_steps: 9,
        failed_steps: 2,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: None,
        item: None,
        tc: None,
    });

    let container_report = ContainerReport::from_batch_report(
        batch_report,
        "Test Report".to_string(),
        "Test Project".to_string(),
        None,
        None,
        None,
        500.0,
    );

    // Verify metadata aggregation calculations are correct
    assert_eq!(container_report.metadata.total_test_cases, 4);
    assert_eq!(container_report.metadata.passed_test_cases, 2);
    assert_eq!(container_report.metadata.failed_test_cases, 2);
    assert_eq!(container_report.metadata.execution_duration, 500.0);

    // Test cases should be preserved in test_results
    assert_eq!(container_report.test_results.len(), 4);
}

#[test]
fn test_container_report_yaml_deserialization() {
    use testcase_manager::verification::ContainerReport;

    // YAML matching the container_data.yml structure
    let yaml = r#"
title: 'Test Report Title'
project: 'Test Project Name'
test_date: '2024-03-15T14:30:00Z'
test_results:
  - test_case_id: 'TC001'
    description: 'Test case description'
    sequences: []
    total_steps: 5
    passed_steps: 4
    failed_steps: 1
    not_executed_steps: 0
    overall_pass: false
    requirement: 'REQ001'
    item: 1
    tc: 1
metadata:
  environment: 'Test Environment'
  platform: 'Test Platform'
  executor: 'Test Executor'
  execution_duration: 123.45
  total_test_cases: 1
  passed_test_cases: 0
  failed_test_cases: 1
"#;

    let deserialized: ContainerReport = serde_yaml::from_str(yaml).unwrap();

    // Verify deserialization works correctly
    assert_eq!(deserialized.title, "Test Report Title");
    assert_eq!(deserialized.project, "Test Project Name");
    assert_eq!(deserialized.test_results.len(), 1);
    assert_eq!(deserialized.test_results[0].test_case_id, "TC001");
    assert_eq!(
        deserialized.metadata.environment,
        Some("Test Environment".to_string())
    );
    assert_eq!(
        deserialized.metadata.platform,
        Some("Test Platform".to_string())
    );
    assert_eq!(
        deserialized.metadata.executor,
        Some("Test Executor".to_string())
    );
    assert_eq!(deserialized.metadata.execution_duration, 123.45);
    assert_eq!(deserialized.metadata.total_test_cases, 1);
    assert_eq!(deserialized.metadata.passed_test_cases, 0);
    assert_eq!(deserialized.metadata.failed_test_cases, 1);
}

#[test]
fn test_container_report_roundtrip_serialization() {
    use chrono::{TimeZone, Utc};
    use testcase_manager::verification::{BatchVerificationReport, ContainerReport};

    let mut batch_report = BatchVerificationReport::new();
    let fixed_time = Utc.with_ymd_and_hms(2024, 3, 15, 14, 30, 0).unwrap();
    batch_report.generated_at = fixed_time;

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC_ROUNDTRIP".to_string(),
        description: "Roundtrip test case".to_string(),
        sequences: vec![],
        total_steps: 7,
        passed_steps: 5,
        failed_steps: 1,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: Some("REQ_RT".to_string()),
        item: Some(99),
        tc: Some(88),
    });

    let original = ContainerReport::from_batch_report(
        batch_report,
        "Roundtrip Test".to_string(),
        "Roundtrip Project".to_string(),
        Some("Roundtrip Env".to_string()),
        Some("Roundtrip Platform".to_string()),
        Some("Roundtrip Executor".to_string()),
        999.99,
    );

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: ContainerReport = serde_yaml::from_str(&yaml).unwrap();

    // Verify all fields match
    assert_eq!(deserialized.title, original.title);
    assert_eq!(deserialized.project, original.project);
    assert_eq!(deserialized.test_date, original.test_date);
    assert_eq!(deserialized.test_results.len(), original.test_results.len());
    assert_eq!(
        deserialized.test_results[0].test_case_id,
        original.test_results[0].test_case_id
    );
    assert_eq!(
        deserialized.test_results[0].total_steps,
        original.test_results[0].total_steps
    );
    assert_eq!(
        deserialized.metadata.environment,
        original.metadata.environment
    );
    assert_eq!(deserialized.metadata.platform, original.metadata.platform);
    assert_eq!(deserialized.metadata.executor, original.metadata.executor);
    assert_eq!(
        deserialized.metadata.execution_duration,
        original.metadata.execution_duration
    );
    assert_eq!(
        deserialized.metadata.total_test_cases,
        original.metadata.total_test_cases
    );
    assert_eq!(
        deserialized.metadata.passed_test_cases,
        original.metadata.passed_test_cases
    );
    assert_eq!(
        deserialized.metadata.failed_test_cases,
        original.metadata.failed_test_cases
    );
}

#[test]
fn test_container_report_with_sequences_yaml_structure() {
    use testcase_manager::models::Expected;
    use testcase_manager::verification::{
        BatchVerificationReport, ContainerReport, SequenceVerificationResult,
        StepVerificationResultEnum,
    };

    let mut batch_report = BatchVerificationReport::new();

    let sequence = SequenceVerificationResult {
        sequence_id: 1,
        name: "Test Sequence 1".to_string(),
        step_results: vec![
            StepVerificationResultEnum::Pass {
                step: 1,
                description: "Step 1 description".to_string(),
                requirement: Some("REQ001".to_string()),
                item: Some(1),
                tc: Some(1),
            },
            StepVerificationResultEnum::Fail {
                step: 2,
                description: "Step 2 description".to_string(),
                expected: Expected {
                    success: Some(true),
                    result: "0x9000".to_string(),
                    output: "Success".to_string(),
                },
                actual_result: "0x6985".to_string(),
                actual_output: "Command not allowed".to_string(),
                reason: "Status code mismatch".to_string(),
                requirement: Some("REQ001".to_string()),
                item: Some(1),
                tc: Some(1),
            },
            StepVerificationResultEnum::NotExecuted {
                step: 3,
                description: "Step 3 description".to_string(),
                requirement: Some("REQ001".to_string()),
                item: Some(1),
                tc: Some(1),
            },
        ],
        all_steps_passed: false,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    };

    batch_report.add_test_case_result(TestCaseVerificationResult {
        test_case_id: "TC_WITH_SEQUENCES".to_string(),
        description: "Test case with sequences".to_string(),
        sequences: vec![sequence],
        total_steps: 3,
        passed_steps: 1,
        failed_steps: 1,
        not_executed_steps: 1,
        overall_pass: false,
        requirement: Some("REQ001".to_string()),
        item: Some(1),
        tc: Some(1),
    });

    let container_report = ContainerReport::from_batch_report(
        batch_report,
        "Test with Sequences".to_string(),
        "Test Project".to_string(),
        None,
        None,
        None,
        100.0,
    );

    let yaml = serde_yaml::to_string(&container_report).unwrap();

    // Verify sequences and steps are in YAML
    assert!(yaml.contains("sequences:"));
    assert!(yaml.contains("sequence_id: 1"));
    assert!(yaml.contains("Test Sequence 1"));
    assert!(yaml.contains("step_results:"));
    assert!(yaml.contains("status: pass"));
    assert!(yaml.contains("status: fail"));
    assert!(yaml.contains("status: not_executed"));
    assert!(yaml.contains("Step 1 description"));
    assert!(yaml.contains("Step 2 description"));
    assert!(yaml.contains("Step 3 description"));
    assert!(yaml.contains("expected:"));
    assert!(yaml.contains("actual_result:"));
    assert!(yaml.contains("actual_output:"));
    assert!(yaml.contains("reason:"));

    // Verify deserialization preserves structure
    let deserialized: ContainerReport = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(deserialized.test_results.len(), 1);
    assert_eq!(deserialized.test_results[0].sequences.len(), 1);
    assert_eq!(
        deserialized.test_results[0].sequences[0].step_results.len(),
        3
    );
}

#[test]
fn test_container_report_optional_metadata_fields() {
    use testcase_manager::verification::{BatchVerificationReport, ContainerReport};

    let batch_report = BatchVerificationReport::new();

    // Create report with no optional metadata
    let container_report_no_optional = ContainerReport::from_batch_report(
        batch_report.clone(),
        "Test".to_string(),
        "Project".to_string(),
        None,
        None,
        None,
        0.0,
    );

    assert!(container_report_no_optional.metadata.environment.is_none());
    assert!(container_report_no_optional.metadata.platform.is_none());
    assert!(container_report_no_optional.metadata.executor.is_none());

    let yaml_no_optional = serde_yaml::to_string(&container_report_no_optional).unwrap();

    // Optional fields should be omitted when None
    let lines: Vec<&str> = yaml_no_optional.lines().collect();
    let has_environment_field = lines
        .iter()
        .any(|line| line.trim().starts_with("environment:"));
    let has_platform_field = lines
        .iter()
        .any(|line| line.trim().starts_with("platform:"));
    let has_executor_field = lines
        .iter()
        .any(|line| line.trim().starts_with("executor:"));

    assert!(!has_environment_field);
    assert!(!has_platform_field);
    assert!(!has_executor_field);

    // Create report with all optional metadata
    let container_report_with_optional = ContainerReport::from_batch_report(
        batch_report,
        "Test".to_string(),
        "Project".to_string(),
        Some("Env".to_string()),
        Some("Platform".to_string()),
        Some("Executor".to_string()),
        100.0,
    );

    assert_eq!(
        container_report_with_optional.metadata.environment,
        Some("Env".to_string())
    );
    assert_eq!(
        container_report_with_optional.metadata.platform,
        Some("Platform".to_string())
    );
    assert_eq!(
        container_report_with_optional.metadata.executor,
        Some("Executor".to_string())
    );

    let yaml_with_optional = serde_yaml::to_string(&container_report_with_optional).unwrap();
    assert!(yaml_with_optional.contains("environment: Env"));
    assert!(yaml_with_optional.contains("platform: Platform"));
    assert!(yaml_with_optional.contains("executor: Executor"));
}

#[test]
fn test_container_report_empty_test_results() {
    use testcase_manager::verification::{BatchVerificationReport, ContainerReport};

    let batch_report = BatchVerificationReport::new();

    let container_report = ContainerReport::from_batch_report(
        batch_report,
        "Empty Report".to_string(),
        "Empty Project".to_string(),
        None,
        None,
        None,
        0.0,
    );

    assert_eq!(container_report.test_results.len(), 0);
    assert_eq!(container_report.metadata.total_test_cases, 0);
    assert_eq!(container_report.metadata.passed_test_cases, 0);
    assert_eq!(container_report.metadata.failed_test_cases, 0);

    let yaml = serde_yaml::to_string(&container_report).unwrap();
    assert!(yaml.contains("test_results: []"));
    assert!(yaml.contains("total_test_cases: 0"));

    // Verify deserialization works with empty results
    let deserialized: ContainerReport = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(deserialized.test_results.len(), 0);
    assert_eq!(deserialized.metadata.total_test_cases, 0);
}

// ============================================================================
// Precomputed Match Strategy Tests
// ============================================================================

#[test]
fn test_precomputed_both_pass() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        Some(true),
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
    // In precomputed mode, success_match should be true since we skip success field check
    assert!(result.success_match);
}

#[test]
fn test_precomputed_result_fail() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        Some(false),
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
}

#[test]
fn test_precomputed_output_fail() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        Some(true),
        Some(false),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_precomputed_both_fail() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        Some(false),
        Some(false),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    assert!(!result.passed);
    assert!(!result.result_match);
}

#[test]
fn test_precomputed_missing_result_verification_pass() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        None, // Missing result_verification_pass
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    // Missing field should be treated as failure
    assert!(!result.passed);
    assert!(!result.result_match);
}

#[test]
fn test_precomputed_missing_output_verification_pass() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        Some(true),
        None, // Missing output_verification_pass
    );

    let result = verifier.verify_step_from_log(&step, &log);

    // Missing field should be treated as failure
    assert!(!result.passed);
    assert!(result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_precomputed_both_missing() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(true),
        None, // Missing result_verification_pass
        None, // Missing output_verification_pass
    );

    let result = verifier.verify_step_from_log(&step, &log);

    // Both missing fields should be treated as failure
    assert!(!result.passed);
    assert!(!result.result_match);
}

#[test]
fn test_precomputed_ignores_success_field() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "expected_result", "expected_output", Some(true));
    // success is false, but with precomputed mode this should be ignored
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "actual_output",
        Some(false), // Different from expected success=true
        Some(true),
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    // Should pass despite success mismatch since precomputed mode skips success check
    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
    assert!(result.success_match);
}

#[test]
fn test_precomputed_test_case_integration() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Precomputed test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence 1".to_string(), "Description".to_string());
    sequence
        .steps
        .push(create_step(1, "result1", "output1", Some(true)));
    sequence
        .steps
        .push(create_step(2, "result2", "output2", Some(true)));
    sequence
        .steps
        .push(create_step(3, "result3", "output3", Some(true)));

    test_case.test_sequences.push(sequence);

    let logs = vec![
        create_execution_log_with_precomputed(
            "TC001",
            1,
            1,
            "actual1",
            "actual_out1",
            Some(true),
            Some(true),
            Some(true),
        ),
        create_execution_log_with_precomputed(
            "TC001",
            1,
            2,
            "actual2",
            "actual_out2",
            Some(true),
            Some(false), // result verification fails
            Some(true),
        ),
        create_execution_log_with_precomputed(
            "TC001",
            1,
            3,
            "actual3",
            "actual_out3",
            Some(true),
            Some(true),
            Some(true),
        ),
    ];

    let result = verifier.verify_test_case(&test_case, &logs);

    assert!(!result.overall_pass);
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.passed_steps, 2);
    assert_eq!(result.failed_steps, 1);
}

#[test]
fn test_precomputed_mixed_strategies() {
    // Result uses precomputed, output uses exact
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Exact);
    let step = create_step(1, "expected_result", "expected_output", None);
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual_result",
        "expected_output", // Matches expected
        Some(true),
        Some(true), // Result precomputed pass
        None,       // Output verification not used since strategy is Exact
    );

    let result = verifier.verify_step_from_log(&step, &log);

    // Should pass: result verified via precomputed, output verified via exact match
    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_precomputed_serialization() {
    use serde_json;

    let strategy = MatchStrategy::Precomputed;
    let json = serde_json::to_string(&strategy).unwrap();
    let deserialized: MatchStrategy = serde_json::from_str(&json).unwrap();
    assert_eq!(strategy, deserialized);
}

#[test]
fn test_precomputed_strategy_name() {
    let verifier = TestVerifier::new(MatchStrategy::Precomputed, MatchStrategy::Precomputed);
    let step = create_step(1, "result", "output", None);
    let log = create_execution_log_with_precomputed(
        "TC001",
        1,
        1,
        "actual",
        "actual_out",
        Some(true),
        Some(false),
        Some(true),
    );

    let result = verifier.verify_step_from_log(&step, &log);

    // Verify strategy name appears in diff
    if let Some(diff) = result.diff.result_diff.as_ref() {
        assert!(diff.message.contains("Precomputed"));
    }
}
