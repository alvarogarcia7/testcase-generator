use chrono::{Local, Utc};
use std::path::PathBuf;
use testcase_manager::models::{
    Expected, Step, TestCase, TestSequence, Verification, VerificationExpression,
};
use testcase_manager::verification::{
    DiffDetail, MatchStrategy, StepVerificationResult, TestExecutionLog, TestVerifier,
    VerificationDiff,
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
        },
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
