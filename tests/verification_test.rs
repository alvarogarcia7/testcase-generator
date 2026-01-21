use testcase_manager::models::{
    ActualResult, Expected, Step, TestCase, TestExecutionLog, TestSequence,
};
use testcase_manager::verification::{
    DiffDetail, ExecutionVerificationResult, MatchStrategy, StepVerificationResult, TestVerifier,
    VerificationDiff,
};

fn create_step(step_num: i64, result: &str, output: &str, success: Option<bool>) -> Step {
    Step {
        step: step_num,
        manual: None,
        description: "Test step".to_string(),
        command: "test command".to_string(),
        expected: Expected {
            success,
            result: result.to_string(),
            output: output.to_string(),
        },
    }
}

fn create_actual(result: &str, output: &str, success: bool) -> ActualResult {
    ActualResult {
        result: result.to_string(),
        output: output.to_string(),
        success,
    }
}

// ============================================================================
// Exact Matching Tests
// ============================================================================

#[test]
fn test_exact_match_all_fields_match() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "SW=0x9000", "Success", Some(true));
    let actual = create_actual("SW=0x9000", "Success", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("SW=0x6A82", "Success", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("SW=0x9000", "Failed", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("SUCCESS", "OUTPUT", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_exact_match_whitespace_sensitive() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let actual = create_actual(" result ", " output ", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_exact_match_empty_strings() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "", "", None);
    let actual = create_actual("", "", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_exact_match_empty_expected_vs_nonempty_actual() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "", "", None);
    let actual = create_actual("something", "output", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("SW=0x9000", "Success", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("123-456-7890", "John Smith", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_pattern_mismatch() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"^\d{4}$", r"^Success$", None);
    let actual = create_actual("12345", "Failed", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_regex_match_invalid_regex_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Exact);
    let step = create_step(1, "[invalid(regex", "Success", None);
    let actual = create_actual("anything", "Success", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_multiline_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"Line1", r"Out1", None);
    let actual = create_actual("Line1\nLine2", "Out1\nOut2", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_special_characters() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"\$\d+\.\d{2}", r"\[.*\]", None);
    let actual = create_actual("$99.99", "[SUCCESS]", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_unicode_pattern() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"你好.*世界", r"🚀.*🎉", None);
    let actual = create_actual("你好 世界", "🚀 Success 🎉", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_regex_match_anchored_vs_unanchored() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"test", r"^output$", None);
    let actual = create_actual("this is a test string", "output", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("SW=0x9000", "Operation Success", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_substring_not_found() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "9000", "Success", None);
    let actual = create_actual("SW=0x6A82", "Failed", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_contains_match_case_sensitive() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "success", "output", None);
    let actual = create_actual("SUCCESS", "OUTPUT", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_contains_match_empty_expected_matches_all() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "", "", None);
    let actual = create_actual("any result", "any output", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_partial_word() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "err", "warn", None);
    let actual = create_actual("error occurred", "warning message", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_multiline() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "Line2", "Out2", None);
    let actual = create_actual("Line1\nLine2\nLine3", "Out1\nOut2\nOut3", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_special_characters() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "$99", "[OK]", None);
    let actual = create_actual("Price: $99.99", "Status: [OK]", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_contains_match_unicode() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "你好", "🚀", None);
    let actual = create_actual("测试 你好 世界", "Start 🚀 End", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("result", "output", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_false_matches_false() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(false));
    let actual = create_actual("result", "output", false);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_true_mismatch_false() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(true));
    let actual = create_actual("result", "output", false);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("result", "output", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("result", "output", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_none_matches_false() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let actual = create_actual("result", "output", false);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.success_match);
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_success_flag_none_with_other_field_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "expected", "output", None);
    let actual = create_actual("actual", "output", false);

    let result = verifier.verify_step(&step, &actual);

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

    let execution_log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        actual_output: "Success OK Complete Done Finished End".to_string(),
        actual_success: true,
        duration_ms: 1000,
        error_message: None,
    };

    let result = verifier.verify_execution_log(&test_case, &execution_log);

    assert!(result.overall_passed);
    assert_eq!(result.step_results.len(), 3);
    assert!(result.step_results.iter().all(|r| r.passed));
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

    let execution_log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        actual_output: "output1".to_string(),
        actual_success: true,
        duration_ms: 1000,
        error_message: None,
    };

    let result = verifier.verify_execution_log(&test_case, &execution_log);

    assert!(!result.overall_passed);
    assert_eq!(result.step_results.len(), 3);
    assert!(result.step_results[0].passed);
    assert!(!result.step_results[1].passed);
    assert!(result.step_results[2].passed);
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

    let execution_log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        actual_output: "wrong".to_string(),
        actual_success: false,
        duration_ms: 1000,
        error_message: None,
    };

    let result = verifier.verify_execution_log(&test_case, &execution_log);

    assert!(!result.overall_passed);
    assert_eq!(result.step_results.len(), 3);
    assert!(result.step_results.iter().all(|r| !r.passed));
}

#[test]
fn test_multi_step_with_different_strategies() {
    let verifier_result_exact = TestVerifier::new(MatchStrategy::Exact, MatchStrategy::Contains);
    let step = create_step(1, "exact_result", "partial", Some(true));
    let actual = create_actual("exact_result", "partial output string", true);

    let result = verifier_result_exact.verify_step(&step, &actual);

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

    let execution_log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        actual_output: "output".to_string(),
        actual_success: true,
        duration_ms: 1000,
        error_message: None,
    };

    let result = verifier.verify_execution_log(&test_case, &execution_log);

    assert!(result.overall_passed);
    assert_eq!(result.step_results.len(), 0);
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[test]
fn test_edge_case_empty_result_and_output() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "", "", Some(true));
    let actual = create_actual("", "", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual(&long_string, &long_string, true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_unicode_characters() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "测试 你好 世界 🚀 🎉", "Привет мир 日本語", Some(true));
    let actual = create_actual("测试 你好 世界 🚀 🎉", "Привет мир 日本語", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_unicode_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "你好", "🚀", None);
    let actual = create_actual("你好世界", "🎉", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_multiline_output() {
    let verifier = TestVerifier::with_exact_matching();
    let multiline = "Line1\nLine2\nLine3\nLine4";
    let step = create_step(1, multiline, multiline, None);
    let actual = create_actual(multiline, multiline, true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_multiline_mismatch() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "Line1\nLine2", "Out1\nOut2", None);
    let actual = create_actual("Line1\nLine3", "Out1\nOut3", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_special_regex_characters_in_exact_match() {
    let verifier = TestVerifier::with_exact_matching();
    let special_chars = r".*+?^${}[]()|\";
    let step = create_step(1, special_chars, special_chars, None);
    let actual = create_actual(special_chars, special_chars, true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_whitespace_variations() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "  spaces  ", "\ttabs\t", None);
    let actual = create_actual(" spaces ", "\ttabs\t\n", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_newline_types() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "Line1\nLine2", "Out1\nOut2", None);
    let actual = create_actual("Line1\r\nLine2", "Out1\r\nOut2", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(!result.result_match);
    assert!(!result.output_match);
}

#[test]
fn test_edge_case_null_bytes() {
    let verifier = TestVerifier::with_exact_matching();
    let with_null = "Before\x00After";
    let step = create_step(1, with_null, with_null, None);
    let actual = create_actual(with_null, with_null, true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_missing_sequence() {
    let verifier = TestVerifier::with_exact_matching();
    let test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let execution_log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 999,
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        actual_output: "output".to_string(),
        actual_success: true,
        duration_ms: 1000,
        error_message: None,
    };

    let result = verifier.verify_execution_log(&test_case, &execution_log);

    assert!(!result.overall_passed);
    assert!(result.step_results.is_empty());
    assert_eq!(result.test_case_id, "TC001");
    assert_eq!(result.sequence_id, 999);
}

#[test]
fn test_edge_case_contains_with_regex_special_chars() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "[test]", "(output)", None);
    let actual = create_actual("Result: [test] done", "Status: (output) ok", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_edge_case_regex_dot_matches_newline_when_specified() {
    let verifier = TestVerifier::new(MatchStrategy::Regex, MatchStrategy::Regex);
    let step = create_step(1, r"(?s)Line1.*Line3", r"(?s)Out1.*Out3", None);
    let actual = create_actual("Line1\nLine2\nLine3", "Out1\nOut2\nOut3", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("actual_result", "actual_output", false);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("actual", "output", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_some());
    assert!(result.diff.output_diff.is_none());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_diff_detail_only_output() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "expected", Some(true));
    let actual = create_actual("result", "actual", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_none());
    assert!(result.diff.output_diff.is_some());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_diff_detail_only_success() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", Some(true));
    let actual = create_actual("result", "output", false);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_none());
    assert!(result.diff.output_diff.is_none());
    assert!(result.diff.success_diff.is_some());
}

#[test]
fn test_diff_detail_none_success_in_message() {
    let verifier = TestVerifier::with_exact_matching();
    let step = create_step(1, "result", "output", None);
    let actual = create_actual("wrong", "output", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(!result.passed);
    assert!(result.diff.result_diff.is_some());
    assert!(result.diff.success_diff.is_none());
}

#[test]
fn test_diff_detail_contains_strategy_message() {
    let verifier = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);
    let step = create_step(1, "expected", "output", None);
    let actual = create_actual("actual", "wrong", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual = create_actual("abc", "123", true);

    let result = verifier.verify_step(&step, &actual);

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
    let actual_exact = create_actual("result", "output", true);
    let actual_contains = create_actual("result and more", "output and more", true);

    let result_exact = verifier.verify_step(&step, &actual_exact);
    let result_contains = verifier.verify_step(&step, &actual_contains);

    assert!(result_exact.passed);
    assert!(!result_contains.passed);
}

#[test]
fn test_verifier_with_mixed_strategies() {
    let verifier = TestVerifier::new(MatchStrategy::Exact, MatchStrategy::Contains);
    let step = create_step(1, "exact_result", "partial", None);
    let actual = create_actual("exact_result", "partial output", true);

    let result = verifier.verify_step(&step, &actual);

    assert!(result.passed);
    assert!(result.result_match);
    assert!(result.output_match);
}

#[test]
fn test_multiple_verifiers_independent() {
    let verifier1 = TestVerifier::with_exact_matching();
    let verifier2 = TestVerifier::new(MatchStrategy::Contains, MatchStrategy::Contains);

    let step = create_step(1, "result", "output", None);
    let actual = create_actual("result string", "output string", true);

    let result1 = verifier1.verify_step(&step, &actual);
    let result2 = verifier2.verify_step(&step, &actual);

    assert!(!result1.passed);
    assert!(result2.passed);
}

// ============================================================================
// ExecutionVerificationResult Tests
// ============================================================================

#[test]
fn test_execution_verification_result_fields() {
    let verifier = TestVerifier::with_exact_matching();

    let mut test_case = TestCase::new(
        "REQ001".to_string(),
        1,
        1,
        "TC001".to_string(),
        "Test case".to_string(),
    );

    let mut sequence = TestSequence::new(1, "Sequence".to_string(), "Desc".to_string());
    sequence
        .steps
        .push(create_step(1, "res", "res", Some(true)));
    test_case.test_sequences.push(sequence);

    let execution_log = TestExecutionLog {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        timestamp: "2024-01-01T00:00:00Z".to_string(),
        actual_output: "res".to_string(),
        actual_success: true,
        duration_ms: 1000,
        error_message: None,
    };

    let result = verifier.verify_execution_log(&test_case, &execution_log);

    assert_eq!(result.test_case_id, "TC001");
    assert_eq!(result.sequence_id, 1);
    assert!(result.overall_passed);
    assert_eq!(result.step_results.len(), 1);
    assert!(result.missing_steps.is_empty());
    assert!(result.unexpected_steps.is_empty());
}

#[test]
fn test_execution_verification_result_serialization() {
    use serde_json;

    let result = ExecutionVerificationResult {
        test_case_id: "TC001".to_string(),
        sequence_id: 1,
        overall_passed: true,
        step_results: vec![],
        missing_steps: vec![],
        unexpected_steps: vec![],
    };

    let json = serde_json::to_string(&result).unwrap();
    let deserialized: ExecutionVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(result, deserialized);
}

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
