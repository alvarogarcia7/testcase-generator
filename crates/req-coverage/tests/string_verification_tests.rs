use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn setup_test_env() -> Result<(TempDir, PathBuf, PathBuf, PathBuf)> {
    let temp_dir = TempDir::new()?;
    let test_cases_dir = temp_dir.path().join("testcases");
    let test_results_dir = temp_dir.path().join("results");
    let requirements_file = temp_dir.path().join("requirements.yaml");

    fs::create_dir(&test_cases_dir)?;
    fs::create_dir(&test_results_dir)?;

    Ok((
        temp_dir,
        test_cases_dir,
        test_results_dir,
        requirements_file,
    ))
}

fn create_test_case_file(
    dir: &PathBuf,
    id: &str,
    requirement: &str,
    covers: Option<&str>,
) -> Result<()> {
    let requirement_coverage = if let Some(covers_text) = covers {
        format!(
            r#"requirement_coverage:
  type: partial
  covers: "{}"
"#,
            covers_text
        )
    } else {
        format!(
            r#"requirement_coverage:
  type: full
"#
        )
    };

    let content = format!(
        r#"type: test_case
schema: tcms/test-case.schema.v1.json
requirement: {}
item: 1
tc: 1
id: {}
description: Test case for {}
{}general_initial_conditions:
  system:
    - Test system ready
initial_conditions:
  system:
    - Ready
test_sequences:
  - id: 1
    name: Test sequence
    description: Test
    initial_conditions:
      system:
        - Ready
    steps:
      - step: 1
        description: Test step
        command: echo test
        expected:
          result: "0"
          output: test
        verification:
          result: '[[ $EXIT_CODE -eq 0 ]]'
          output: 'grep -q "test" <<< "$COMMAND_OUTPUT"'
"#,
        requirement, id, requirement, requirement_coverage
    );

    fs::write(dir.join(format!("{}.yaml", id)), content)?;
    Ok(())
}

fn create_verification_result(dir: &PathBuf, test_case_id: &str, passed: bool) -> Result<()> {
    let content = format!(
        r#"title: Verification Results
project: Test Project
test_date: "2024-01-20"
test_results:
  - test_case_id: {}
    description: Test result
    overall_pass: {}
"#,
        test_case_id, passed
    );

    fs::write(
        dir.join(format!("{}_container.yaml", test_case_id)),
        content,
    )?;
    Ok(())
}

fn create_requirements_file(path: &PathBuf, requirements: &[(&str, &str)]) -> Result<()> {
    let mut content = String::from("requirements:\n");
    for (id, text) in requirements {
        content.push_str(&format!(
            "  - id: {}\n    text: \"{}\"\n    description: \"Test requirement\"\n",
            id, text
        ));
    }
    fs::write(path, content)?;
    Ok(())
}

#[test]
fn test_full_coverage_with_single_test_case() -> Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "authenticate users")])?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    assert_eq!(report.total_requirements, 1);
    assert_eq!(report.fully_covered_requirements, 1);
    assert_eq!(report.partially_covered_requirements, 0);

    let req = &report.requirements[0];
    assert_eq!(req.requirement_id, "REQ-001");
    assert_eq!(req.coverage_type, req_coverage::models::CoverageType::Full);
    assert_eq!(
        req.requirement_text,
        Some("authenticate users".to_string())
    );
    assert_eq!(
        req.covered_portions,
        Some(vec!["authenticate users".to_string()])
    );
    assert!(req.coverage_errors.is_none());

    Ok(())
}

#[test]
fn test_partial_coverage_with_multiple_test_cases() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(
        &requirements_file,
        &[(
            "REQ-001",
            "The system shall authenticate users and deny access",
        )],
    )?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;
    create_test_case_file(&test_cases_dir, "TC-002", "REQ-001", Some("deny access"))?;

    create_verification_result(&test_results_dir, "TC-001", true)?;
    create_verification_result(&test_results_dir, "TC-002", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    assert_eq!(report.total_requirements, 1);
    assert_eq!(report.fully_covered_requirements, 0);
    assert_eq!(report.partially_covered_requirements, 1);

    let req = &report.requirements[0];
    assert_eq!(req.requirement_id, "REQ-001");
    assert_eq!(
        req.coverage_type,
        req_coverage::models::CoverageType::Partial
    );
    assert_eq!(
        req.requirement_text,
        Some("The system shall authenticate users and deny access".to_string())
    );

    let covered = req.covered_portions.as_ref().unwrap();
    assert!(covered.contains(&"authenticate users".to_string()));
    assert!(covered.contains(&"deny access".to_string()));
    assert_eq!(covered.len(), 2);
    assert!(req.coverage_errors.is_none());

    Ok(())
}

#[test]
fn test_full_coverage_with_multiple_test_cases() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(
        &requirements_file,
        &[("REQ-001", "authenticate users and deny access")],
    )?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users "),
    )?;
    create_test_case_file(&test_cases_dir, "TC-002", "REQ-001", Some("and "))?;
    create_test_case_file(&test_cases_dir, "TC-003", "REQ-001", Some("deny access"))?;

    create_verification_result(&test_results_dir, "TC-001", true)?;
    create_verification_result(&test_results_dir, "TC-002", true)?;
    create_verification_result(&test_results_dir, "TC-003", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    assert_eq!(report.total_requirements, 1);
    assert_eq!(report.fully_covered_requirements, 1);

    let req = &report.requirements[0];
    assert_eq!(req.coverage_type, req_coverage::models::CoverageType::Full);
    assert_eq!(req.covered_portions.as_ref().unwrap().len(), 3);

    Ok(())
}

#[test]
fn test_invalid_covers_string_error() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "authenticate users")])?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("invalid text not in requirement"),
    )?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = &report.requirements[0];
    assert!(req.coverage_errors.is_some());
    let errors = req.coverage_errors.as_ref().unwrap();
    assert!(!errors.is_empty());
    assert!(errors[0].contains("not found in requirement"));

    Ok(())
}

#[test]
fn test_missing_requirement_definition() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "test requirement")])?;

    create_test_case_file(&test_cases_dir, "TC-001", "REQ-999", Some("some text"))?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = report
        .requirements
        .iter()
        .find(|r| r.requirement_id == "REQ-999")
        .unwrap();
    assert!(req.coverage_errors.is_some());
    let errors = req.coverage_errors.as_ref().unwrap();
    assert!(errors[0].contains("Requirement definition not found"));

    Ok(())
}

#[test]
fn test_without_requirements_file() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, _requirements_file) = setup_test_env()?;

    create_test_case_file(&test_cases_dir, "TC-001", "REQ-001", Some("some text"))?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::new(&test_cases_dir)?;
    let report = analyzer.analyze(&test_results_dir)?;

    assert_eq!(report.total_requirements, 1);
    let req = &report.requirements[0];
    assert_eq!(req.requirement_id, "REQ-001");
    assert!(req.requirement_text.is_none());
    assert!(req.covered_portions.is_none());
    assert!(req.coverage_errors.is_none());

    Ok(())
}

#[test]
fn test_json_requirements_file() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, _requirements_file) = setup_test_env()?;

    let json_requirements_file = _temp.path().join("requirements.json");
    let content = r#"{
  "requirements": [
    {
      "id": "REQ-001",
      "text": "authenticate users",
      "description": "Test requirement"
    }
  ]
}"#;
    fs::write(&json_requirements_file, content)?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &json_requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    assert_eq!(report.total_requirements, 1);
    let req = &report.requirements[0];
    assert_eq!(req.requirement_text, Some("authenticate users".to_string()));

    Ok(())
}

#[test]
fn test_multiple_requirements() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(
        &requirements_file,
        &[
            ("REQ-001", "authenticate users"),
            ("REQ-002", "log security events"),
            ("REQ-003", "validate input"),
        ],
    )?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;
    create_test_case_file(&test_cases_dir, "TC-002", "REQ-002", Some("log security"))?;
    create_test_case_file(&test_cases_dir, "TC-003", "REQ-002", Some("events"))?;

    create_verification_result(&test_results_dir, "TC-001", true)?;
    create_verification_result(&test_results_dir, "TC-002", true)?;
    create_verification_result(&test_results_dir, "TC-003", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    assert_eq!(report.total_requirements, 3);
    assert_eq!(report.fully_covered_requirements, 1);
    assert_eq!(report.partially_covered_requirements, 1);
    assert_eq!(report.uncovered_requirements, 1);

    Ok(())
}

#[test]
fn test_coverage_with_test_failures() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "authenticate users")])?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;

    create_verification_result(&test_results_dir, "TC-001", false)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = &report.requirements[0];
    assert_eq!(
        req.status,
        req_coverage::models::CoverageStatus::CoveredFail
    );

    Ok(())
}

#[test]
fn test_duplicate_covers_strings() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "authenticate users")])?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;
    create_test_case_file(
        &test_cases_dir,
        "TC-002",
        "REQ-001",
        Some("authenticate users"),
    )?;

    create_verification_result(&test_results_dir, "TC-001", true)?;
    create_verification_result(&test_results_dir, "TC-002", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = &report.requirements[0];
    assert_eq!(req.covered_portions.as_ref().unwrap().len(), 2);
    assert_eq!(req.test_cases.len(), 2);

    Ok(())
}

#[test]
fn test_overlapping_covers_strings() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(
        &requirements_file,
        &[("REQ-001", "authenticate users and validate")],
    )?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;
    create_test_case_file(&test_cases_dir, "TC-002", "REQ-001", Some("users and"))?;

    create_verification_result(&test_results_dir, "TC-001", true)?;
    create_verification_result(&test_results_dir, "TC-002", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = &report.requirements[0];
    let covered = req.covered_portions.as_ref().unwrap();
    assert!(covered.contains(&"authenticate users".to_string()));
    assert!(covered.contains(&"users and".to_string()));

    Ok(())
}

#[test]
fn test_case_sensitive_matching() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "Authenticate Users")])?;

    create_test_case_file(
        &test_cases_dir,
        "TC-001",
        "REQ-001",
        Some("authenticate users"),
    )?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = &report.requirements[0];
    assert!(req.coverage_errors.is_some());
    let errors = req.coverage_errors.as_ref().unwrap();
    assert!(errors[0].contains("not found in requirement"));

    Ok(())
}

#[test]
fn test_empty_covers_string() -> Result<()> {
    let (_temp, test_cases_dir, test_results_dir, requirements_file) = setup_test_env()?;

    create_requirements_file(&requirements_file, &[("REQ-001", "authenticate users")])?;

    create_test_case_file(&test_cases_dir, "TC-001", "REQ-001", None)?;

    create_verification_result(&test_results_dir, "TC-001", true)?;

    let analyzer = req_coverage::coverage::CoverageAnalyzer::with_requirements(
        &test_cases_dir,
        &requirements_file,
    )?;
    let report = analyzer.analyze(&test_results_dir)?;

    let req = &report.requirements[0];
    assert!(req.covered_portions.is_none() || req.covered_portions.as_ref().unwrap().is_empty());

    Ok(())
}
