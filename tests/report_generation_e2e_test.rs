use std::fs;
use std::path::PathBuf;
use testcase_manager::models::TestCase;
use testcase_manager::verification::{
    SequenceVerificationResult, StepVerificationResultEnum, TestCaseVerificationResult,
    TestVerifier,
};

fn load_test_case(filename: &str) -> TestCase {
    let path = PathBuf::from("testcases").join(filename);
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read test case: {}", path.display()));
    serde_yaml::from_str(&content)
        .unwrap_or_else(|_| panic!("Failed to parse test case: {}", filename))
}

// ============================================================================
// TC Type Tests (Test Cases)
// ============================================================================

#[test]
fn test_e2e_tc_gsma_4_4_2_2_yaml_output() {
    let test_case = load_test_case("gsma_4.4.2.2_TC.yml");

    // Create mock verification result matching the test case structure
    let result = TestCaseVerificationResult {
        test_case_id: test_case.id.clone(),
        description: test_case.description.clone(),
        requirement: Some(test_case.requirement.clone()),
        item: Some(test_case.item),
        tc: Some(test_case.tc),
        sequences: vec![
            SequenceVerificationResult {
                sequence_id: 1,
                name: "Test Sequence #01 Nominal: Unset PPR1".to_string(),
                step_results: vec![
                    StepVerificationResultEnum::Pass {
                        step: 1,
                        description: "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; MTD_STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE))".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                    StepVerificationResultEnum::Pass {
                        step: 2,
                        description: "Fetch 'XX'".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                ],
                all_steps_passed: true,
                requirement: None,
                item: None,
                tc: None,
            },
            SequenceVerificationResult {
                sequence_id: 2,
                name: "Test Sequence #02 Nominal: Unset PPPR2 and update icon".to_string(),
                step_results: vec![
                    StepVerificationResultEnum::Pass {
                        step: 1,
                        description: "MTD_SENDS_SMS_PP([INSTALL_PERSO_RES_ISDP]; MTD_STORE_DATA_SCRIPT(#REMOVE_PPR1, FALSE))".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                    StepVerificationResultEnum::Pass {
                        step: 2,
                        description: "Fetch 'XX'".to_string(),
                        requirement: None,
                        item: None,
                        tc: None,
                    },
                ],
                all_steps_passed: true,
                requirement: None,
                item: None,
                tc: None,
            },
        ],
        total_steps: 4,
        passed_steps: 4,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    };

    let verifier = TestVerifier::with_exact_matching();
    let generated_yaml = verifier.generate_report_yaml(&result).unwrap();

    // Verify key fields are present
    assert!(generated_yaml.contains("requirement: XXX100"));
    assert!(generated_yaml.contains("item: 1"));
    assert!(generated_yaml.contains("tc: 4"));
    assert!(generated_yaml.contains(&test_case.id));
    assert!(generated_yaml.contains("total_steps: 4"));
    assert!(generated_yaml.contains("passed_steps: 4"));
    assert!(generated_yaml.contains("failed_steps: 0"));
    assert!(generated_yaml.contains("overall_pass: true"));
}

#[test]
fn test_e2e_tc_gsma_4_4_2_2_json_output() {
    let test_case = load_test_case("gsma_4.4.2.2_TC.yml");

    let result = TestCaseVerificationResult {
        test_case_id: test_case.id.clone(),
        description: test_case.description.clone(),
        requirement: Some(test_case.requirement.clone()),
        item: Some(test_case.item),
        tc: Some(test_case.tc),
        sequences: vec![],
        total_steps: 4,
        passed_steps: 4,
        failed_steps: 0,
        not_executed_steps: 0,
        overall_pass: true,
    };

    let verifier = TestVerifier::with_exact_matching();
    let generated_json = verifier.generate_report_json(&result).unwrap();

    // Verify JSON structure
    assert!(generated_json.contains("\"test_case_id\""));
    assert!(generated_json.contains("\"requirement\": \"XXX100\""));
    assert!(generated_json.contains("\"item\": 1"));
    assert!(generated_json.contains("\"tc\": 4"));
    assert!(generated_json.contains("\"total_steps\": 4"));
    assert!(generated_json.contains("\"passed_steps\": 4"));
    assert!(generated_json.contains("\"overall_pass\": true"));

    // Verify it's valid JSON
    let parsed: TestCaseVerificationResult = serde_json::from_str(&generated_json).unwrap();
    assert_eq!(parsed.test_case_id, test_case.id);
    assert_eq!(parsed.total_steps, 4);
}



// ============================================================================
// Format Comparison Tests
// ============================================================================

#[test]
fn test_yaml_json_equivalence() {
    let result = TestCaseVerificationResult {
        test_case_id: "TC_FORMAT_TEST".to_string(),
        description: "Test format equivalence".to_string(),
        requirement: Some("REQ_FMT".to_string()),
        item: Some(1),
        tc: Some(1),
        sequences: vec![],
        total_steps: 5,
        passed_steps: 3,
        failed_steps: 2,
        not_executed_steps: 0,
        overall_pass: false,
    };

    let verifier = TestVerifier::with_exact_matching();

    let yaml = verifier.generate_report_yaml(&result).unwrap();
    let json = verifier.generate_report_json(&result).unwrap();

    // Parse both formats and verify they contain the same data
    let yaml_parsed: TestCaseVerificationResult = serde_yaml::from_str(&yaml).unwrap();
    let json_parsed: TestCaseVerificationResult = serde_json::from_str(&json).unwrap();

    assert_eq!(yaml_parsed.test_case_id, json_parsed.test_case_id);
    assert_eq!(yaml_parsed.description, json_parsed.description);
    assert_eq!(yaml_parsed.requirement, json_parsed.requirement);
    assert_eq!(yaml_parsed.item, json_parsed.item);
    assert_eq!(yaml_parsed.tc, json_parsed.tc);
    assert_eq!(yaml_parsed.total_steps, json_parsed.total_steps);
    assert_eq!(yaml_parsed.passed_steps, json_parsed.passed_steps);
    assert_eq!(yaml_parsed.failed_steps, json_parsed.failed_steps);
    assert_eq!(yaml_parsed.overall_pass, json_parsed.overall_pass);
}

#[test]
fn test_all_test_case_types_can_be_loaded() {
    // Verify TC test case file can be loaded
    let test_case = load_test_case("gsma_4.4.2.2_TC.yml");
    assert!(
        !test_case.id.is_empty(),
        "Test case ID should not be empty"
    );
    assert!(
        !test_case.description.is_empty(),
        "Description should not be empty"
    );
}
